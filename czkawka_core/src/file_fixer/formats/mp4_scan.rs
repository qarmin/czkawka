//! Finding video NAL units in media data that has lost its index.
//!
//! Samples in an MP4 are length-prefixed, which makes video self-delimiting: a four-byte
//! length followed by a valid NAL header is a strong signal. Audio is not - AAC in an MP4 is
//! stored raw, with no sync word - so the interleaved audio chunks can only be stepped over,
//! by searching forward for the next position that looks like video.
//!
//! The signal alone is not enough. A random four bytes will occasionally read as a plausible
//! length, and accepting one swallows hundreds of kilobytes of real data. Three constraints
//! from the reference profile fix that, and each was added because a measured run needed it:
//!
//! - per-type length limits, because an access-unit delimiter is two bytes and never 94 KB;
//! - the slice header, whose first fields must say "first macroblock, sane slice type, a
//!   parameter set this stream actually has";
//! - one-step consistency: what follows a unit must be another unit, or an audio-sized gap
//!   and then a unit.
//!
//! Both H.264 and HEVC are read, and nothing about them is shared by accident: the header is
//! one byte against two, the type sits in different bits, the numbering is unrelated, and the
//! slice header opens with different fields. The codec comes from the profile rather than
//! being sniffed per unit, because every one of those differences turns a stream read as the
//! wrong codec into plausible-looking nonsense rather than an obvious failure.
//!
//! Measured on `tmp/a.mp4`: 799 of 830 sample starts found, 16 false positives.

use std::ops::Range;

use super::mp4_profile::{Profile, VideoCodec};
use crate::file_fixer::io::read_u32_be;

/// How far to search for the next video unit when stepping over an audio chunk.
///
/// Measured on the sample, the gaps between consecutive video samples run 268 to 1135 bytes.
/// Four kilobytes leaves room for a longer audio run; sixteen was tried and made the false
/// positive rate worse.
const MAX_AUDIO_GAP: usize = 4096;

/// Minimal exponential-Golomb reader, enough for the first fields of a slice header.
pub struct BitReader<'a> {
    data: &'a [u8],
    position: usize,
}

impl<'a> BitReader<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data, position: 0 }
    }

    pub fn bit(&mut self) -> Option<u32> {
        let byte = *self.data.get(self.position / 8)?;
        let bit = u32::from((byte >> (7 - self.position % 8)) & 1);
        self.position += 1;
        Some(bit)
    }

    /// `ue(v)`: leading zeros give the width, the value follows.
    pub fn unsigned_exp_golomb(&mut self) -> Option<u32> {
        let mut leading = 0u32;
        while self.bit()? == 0 {
            leading += 1;
            if leading > 31 {
                return None;
            }
        }
        let mut value = 0u32;
        for _ in 0..leading {
            value = (value << 1) | self.bit()?;
        }
        Some((1u32 << leading) - 1 + value)
    }
}

/// An access-unit delimiter carries one or two bytes of payload; anything larger is a false
/// match. Three covers HEVC, whose header is a byte longer than H.264's.
const MAX_DELIMITER_SIZE: u32 = 3;
const MAX_SUPPLEMENTAL_SIZE: u32 = 1 << 16;
const MAX_SLICE_TYPE: u32 = 9;

/// What a NAL unit is for, once its codec-specific type number has been read.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnitKind {
    Slice { keyframe: bool },
    SupplementalInfo,
    Delimiter,
}

impl UnitKind {
    pub fn is_slice(self) -> bool {
        matches!(self, Self::Slice { .. })
    }

    pub fn is_keyframe(self) -> bool {
        self == Self::Slice { keyframe: true }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VideoUnit {
    /// The unit's payload, excluding its length prefix.
    pub payload: Range<usize>,
    pub kind: UnitKind,
}

impl VideoUnit {
    pub fn is_slice(&self) -> bool {
        self.kind.is_slice()
    }

    pub fn is_keyframe(&self) -> bool {
        self.kind.is_keyframe()
    }

    /// Whether the unit is needed to decode. Supplemental info and delimiters are not, and
    /// dropping them removes a whole class of complaint from a rebuilt stream.
    pub fn is_required(&self) -> bool {
        self.is_slice()
    }
}

#[derive(Debug, Clone)]
pub struct ScanReport {
    pub units: Vec<VideoUnit>,
    /// How many times the scan had to search forward past an audio chunk.
    pub resynchronizations: u32,
    /// Offset the scan stopped at, which is the end of the data when it read everything.
    pub stopped_at: usize,
}

impl ScanReport {
    pub fn slices(&self) -> usize {
        self.units.iter().filter(|unit| unit.is_slice()).count()
    }

    pub fn keyframes(&self) -> usize {
        self.units.iter().filter(|unit| unit.is_keyframe()).count()
    }
}

/// Bytes of NAL header this codec puts in front of a unit's payload.
fn header_len(codec: VideoCodec) -> usize {
    match codec {
        VideoCodec::H264 => 1,
        VideoCodec::Hevc => 2,
    }
}

/// What the unit starting at `at` is, or `None` when those bytes are not a header the scanner
/// collects. Parameter sets deliberately answer `None`: a rebuilt stream carries the profile's
/// own, and accepting in-band ones would emit them twice.
fn classify(data: &[u8], at: usize, codec: VideoCodec) -> Option<UnitKind> {
    let header = *data.get(at)?;
    // The forbidden zero bit is exactly that; a set one means this is not a NAL header.
    if header & 0x80 != 0 {
        return None;
    }
    match codec {
        VideoCodec::H264 => match header & 0x1F {
            1 => Some(UnitKind::Slice { keyframe: false }),
            5 => Some(UnitKind::Slice { keyframe: true }),
            6 => Some(UnitKind::SupplementalInfo),
            9 => Some(UnitKind::Delimiter),
            _ => None,
        },
        VideoCodec::Hevc if is_base_layer(data, at) => match (header >> 1) & 0x3F {
            0..=9 => Some(UnitKind::Slice { keyframe: false }),
            16..=21 => Some(UnitKind::Slice { keyframe: true }),
            35 => Some(UnitKind::Delimiter),
            39 | 40 => Some(UnitKind::SupplementalInfo),
            _ => None,
        },
        VideoCodec::Hevc => None,
    }
}

/// Whether an HEVC header names layer zero with a legal temporal id.
///
/// The layer id spans the low bit of the first header byte and the top five of the second, and
/// the temporal id in the remaining three bits is stored plus one, so it is never zero. A
/// camera does not write scalable streams, so insisting on the base layer costs nothing and
/// leaves only seven of 65,536 two-byte values plausible.
fn is_base_layer(data: &[u8], at: usize) -> bool {
    data.get(at).is_some_and(|header| header & 0x01 == 0) && data.get(at + 1).is_some_and(|byte| (1..=7).contains(byte))
}

/// Whether these bytes can be the header of any legal NAL unit, parameter sets included.
///
/// Wider than [`classify`] on purpose: a real sample begins with a parameter set often enough
/// that a check for "does this index address NAL units at all" must accept one.
pub fn header_is_plausible(data: &[u8], at: usize, codec: VideoCodec) -> bool {
    let Some(&header) = data.get(at) else {
        return false;
    };
    if header & 0x80 != 0 {
        return false;
    }
    match codec {
        VideoCodec::H264 => (1..=12).contains(&(header & 0x1F)),
        VideoCodec::Hevc => matches!((header >> 1) & 0x3F, 0..=9 | 16..=21 | 32..=40) && is_base_layer(data, at),
    }
}

/// Reads the length and header at `offset`, returning the payload length when it is plausible.
fn unit_at(data: &[u8], offset: usize, end: usize, profile: &Profile) -> Option<(u32, UnitKind)> {
    if offset + 8 > end {
        return None;
    }
    let length = read_u32_be(data, offset)?;
    if length < 2 || offset as u64 + 4 + u64::from(length) > end as u64 {
        return None;
    }

    let kind = classify(data, offset + 4, profile.codec)?;
    match kind {
        UnitKind::Delimiter if length <= MAX_DELIMITER_SIZE => Some((length, kind)),
        UnitKind::SupplementalInfo if length <= MAX_SUPPLEMENTAL_SIZE => Some((length, kind)),
        UnitKind::Slice { .. } if length <= profile.max_nal_size() && slice_header_is_sane(data, offset, profile) => Some((length, kind)),
        _ => None,
    }
}

/// Checks the fields that open a slice header.
///
/// The question is the same in both codecs - is this the first slice of a picture, does it name
/// a parameter set this stream actually carries - but the fields that answer it are not. H.264
/// opens with `first_mb_in_slice`, `slice_type` and `pic_parameter_set_id`, all `ue(v)`; HEVC
/// opens with a single flag, a second flag present only on a random-access picture, and then
/// the parameter set id. Together these reject almost everything that merely looked like a
/// length.
fn slice_header_is_sane(data: &[u8], offset: usize, profile: &Profile) -> bool {
    let header_at = offset + 4;
    let body = header_at + header_len(profile.codec);
    let Some(payload) = data.get(body..(body + 8).min(data.len())) else {
        return false;
    };
    let mut reader = BitReader::new(payload);

    match profile.codec {
        VideoCodec::H264 => {
            if reader.unsigned_exp_golomb() != Some(0) {
                return false;
            }
            if !reader.unsigned_exp_golomb().is_some_and(|kind| kind <= MAX_SLICE_TYPE) {
                return false;
            }
        }
        VideoCodec::Hevc => {
            if reader.bit() != Some(1) {
                return false;
            }
            let Some(&header) = data.get(header_at) else {
                return false;
            };
            // A random-access picture carries one extra flag before the parameter set id.
            if (16..=23).contains(&((header >> 1) & 0x3F)) && reader.bit().is_none() {
                return false;
            }
        }
    }
    reader.unsigned_exp_golomb().is_some_and(|id| profile.accepts_parameter_set_id(id))
}

/// A unit is only accepted when what follows it also makes sense.
fn accept_at(data: &[u8], offset: usize, end: usize, profile: &Profile) -> Option<(u32, UnitKind)> {
    let (length, kind) = unit_at(data, offset, end, profile)?;
    let next = offset + 4 + length as usize;

    if next >= end || unit_at(data, next, end, profile).is_some() {
        return Some((length, kind));
    }
    // Otherwise an audio chunk may sit in between.
    (1..MAX_AUDIO_GAP)
        .any(|gap| next + gap < end && unit_at(data, next + gap, end, profile).is_some())
        .then_some((length, kind))
}

/// Could the bytes at `at` open a slice NAL unit?
///
/// The cheap test that makes scanning a whole recording for the next slice affordable: most
/// positions are rejected by one comparison, and only the survivors pay for a header check.
/// H.264 admits eight byte values out of 256; HEVC admits sixteen, narrowed again by a second
/// byte that must carry a base-layer temporal id.
fn could_open_a_slice(data: &[u8], at: usize, codec: VideoCodec) -> bool {
    classify(data, at, codec).is_some_and(UnitKind::is_slice)
}

/// Finds where video resumes after a gap, widening the search only once it has to.
///
/// The wide stage additionally insists on landing on a slice. Supplemental info and delimiters
/// are short and weakly constrained, so over a megabyte of audio one of them matches by
/// accident regularly; a slice has to pass the header check as well, which does not.
///
/// It also searches to the end of the media rather than giving up at a fixed distance. A carve
/// is a region of disk, not a file: it can hold a still image, a stretch of another recording,
/// or a hole where a fragment went missing, and any of those is wider than an interleaving gap.
/// Measured over three carves from a PhotoRec collection, lifting the old one-megabyte limit
/// took the recovery from 505, 1,864 and 846 frames to 3,001, 9,245 and 3,911 - four to six
/// times as much real video, confirmed by looking at the frames - while the decoder's
/// complaints grew from 1, 79 and 428 to 75, 360 and 2,011.
fn resynchronize(data: &[u8], offset: usize, end: usize, profile: &Profile) -> Option<usize> {
    let close = (1..MAX_AUDIO_GAP)
        .map(|gap| offset + gap)
        .find(|&candidate| candidate < end && accept_at(data, candidate, end, profile).is_some());
    if close.is_some() {
        return close;
    }

    let mut candidate = offset + MAX_AUDIO_GAP;
    while candidate + 5 < end {
        if could_open_a_slice(data, candidate + 4, profile.codec) && accept_at(data, candidate, end, profile).is_some_and(|(_, kind)| kind.is_slice()) {
            return Some(candidate);
        }
        candidate += 1;
    }
    None
}

/// Walks the media data, collecting the video units it can identify.
pub fn scan(data: &[u8], mdat: Range<usize>, profile: &Profile) -> ScanReport {
    let end = mdat.end.min(data.len());
    let mut units = Vec::new();
    let mut offset = mdat.start;
    let mut resynchronizations = 0u32;

    while offset < end {
        if let Some((length, kind)) = accept_at(data, offset, end, profile) {
            units.push(VideoUnit {
                payload: offset + 4..offset + 4 + length as usize,
                kind,
            });
            offset += 4 + length as usize;
            continue;
        }

        let Some(next) = resynchronize(data, offset, end, profile) else {
            break;
        };
        offset = next;
        resynchronizations += 1;
    }

    ScanReport {
        units,
        resynchronizations,
        stopped_at: offset,
    }
}

/// Writes the recovered units as an Annex-B elementary stream.
///
/// Producing a byte stream rather than a rebuilt `moov` is deliberate: ffmpeg can turn this
/// into a correct container, and hand-writing sample tables would add a large surface of
/// fiddly, easy-to-get-subtly-wrong code for no gain.
///
/// Streamed into the writer rather than returned as a `Vec`, because the stream is as large as
/// the media it came from. Measured on a real carved collection: building it in memory cost
/// 10.5 GB of resident memory across the worker threads, which is most of the way to an
/// out-of-memory kill on a 30 GB machine for no benefit at all - the bytes are written to a
/// temporary file the moment they exist.
pub fn write_annex_b<W: std::io::Write>(data: &[u8], report: &ScanReport, profile: &Profile, writer: &mut W) -> std::io::Result<u64> {
    const START_CODE: [u8; 4] = [0, 0, 0, 1];
    let mut written = 0u64;

    // In the order a decoder needs them, which for HEVC means the video parameter set first.
    for set in profile.parameter_sets() {
        writer.write_all(&START_CODE)?;
        writer.write_all(set)?;
        written += (START_CODE.len() + set.len()) as u64;
    }
    for unit in report.units.iter().filter(|unit| unit.is_required()) {
        if let Some(payload) = data.get(unit.payload.clone()) {
            writer.write_all(&START_CODE)?;
            writer.write_all(payload)?;
            written += (START_CODE.len() + payload.len()) as u64;
        }
    }
    writer.flush()?;
    Ok(written)
}

#[cfg(test)]
mod tests {
    use super::super::{mp4_boxes, mp4_profile};
    use super::*;

    fn sample() -> Vec<u8> {
        std::fs::read("tmp/a.mp4").unwrap_or_default()
    }

    /// Panics rather than skipping when the sample is present: a test that silently does
    /// nothing is worse than no test.
    fn profile_and_mdat(data: &[u8]) -> (Profile, Range<usize>) {
        let profile = mp4_profile::from_reference(data).expect("the sample yields a profile");
        let mdat = mp4_boxes::top_level(data).into_iter().find(|b| b.is(*b"mdat")).expect("the sample has media data");
        (profile, mdat.payload)
    }

    /// A slice NAL the scanner will accept: length prefix, a type-1 header, and a slice header
    /// whose first three fields read as first-macroblock zero, slice type zero, parameter set
    /// zero (`111` in exponential-Golomb, then padding).
    fn slice(payload_len: usize) -> Vec<u8> {
        let mut unit = (payload_len as u32).to_be_bytes().to_vec();
        unit.push(0x01);
        unit.push(0xE0);
        unit.extend(std::iter::repeat_n(0x33, payload_len.saturating_sub(2)));
        unit
    }

    /// The HEVC counterpart: a two-byte header for a trailing picture, then a slice header
    /// opening with `first_slice_segment_in_pic_flag` set and parameter set zero.
    fn hevc_slice(payload_len: usize, keyframe: bool) -> Vec<u8> {
        let kind: u8 = if keyframe { 19 } else { 1 };
        let mut unit = (payload_len as u32).to_be_bytes().to_vec();
        unit.push(kind << 1);
        unit.push(0x01);
        // first_slice_segment_in_pic_flag, the extra flag a random-access picture carries, then
        // `1` for parameter set zero - left-aligned in the byte that follows the header.
        unit.push(if keyframe { 0xE0 } else { 0xC0 });
        unit.extend(std::iter::repeat_n(0x33, payload_len.saturating_sub(3)));
        unit
    }

    fn test_profile() -> Profile {
        Profile {
            codec: VideoCodec::H264,
            video_parameter_sets: Vec::new(),
            sequence_parameter_sets: vec![vec![0x67, 0x42, 0x00, 0x1E]],
            picture_parameter_sets: vec![vec![0x68, 0xCE, 0x38, 0x80]],
            picture_parameter_set_ids: vec![0],
            max_sample_size: 1 << 16,
            frame_rate: 30.0,
        }
    }

    fn hevc_profile() -> Profile {
        Profile {
            codec: VideoCodec::Hevc,
            video_parameter_sets: vec![vec![0x40, 0x01, 0x0C, 0x01]],
            sequence_parameter_sets: vec![vec![0x42, 0x01, 0x01]],
            picture_parameter_sets: vec![vec![0x44, 0x01, 0xC0]],
            picture_parameter_set_ids: vec![0],
            max_sample_size: 1 << 16,
            frame_rate: 30.0,
        }
    }

    /// A carve is a region of disk: it can hold a still image, a stretch of another recording
    /// or a hole where a fragment went missing, and any of those is far wider than the gap
    /// between two interleaved chunks. Giving up at a fixed distance left four fifths of some
    /// real recordings unread.
    #[test]
    fn the_scan_resumes_after_a_gap_far_larger_than_an_audio_chunk() {
        let mut data = Vec::new();
        for _ in 0..6 {
            data.extend_from_slice(&slice(500));
        }
        let before = data.len();
        // Two megabytes of something that is not video - well past any interleaving gap.
        data.extend(std::iter::repeat_n(0x7Fu8, 2 << 20));
        for _ in 0..6 {
            data.extend_from_slice(&slice(500));
        }

        let report = scan(&data, 0..data.len(), &test_profile());
        // Eleven, not twelve: a unit is only accepted when something after it confirms it, and
        // the last one before the gap has nothing behind it but the gap. Conceding one frame at
        // a discontinuity is the price of not accepting compressed audio as video.
        assert_eq!(report.slices(), 11, "both runs must be found");
        assert!(report.resynchronizations >= 1);
        assert!(report.units.iter().any(|unit| unit.payload.start > before), "the units past the gap are the point");
    }

    #[test]
    fn hevc_units_are_read_with_their_own_header_and_numbering() {
        let mut data = Vec::new();
        data.extend_from_slice(&hevc_slice(400, true));
        for _ in 0..5 {
            data.extend_from_slice(&hevc_slice(300, false));
        }
        let report = scan(&data, 0..data.len(), &hevc_profile());

        assert_eq!(report.slices(), 6);
        assert_eq!(report.keyframes(), 1);
        assert_eq!(report.stopped_at, data.len());
    }

    /// The failure this exists to prevent: an HEVC stream read as H.264 finds units of the
    /// wrong type at the wrong boundaries and produces a rebuild full of nonsense rather than
    /// an obvious refusal.
    #[test]
    fn an_hevc_stream_is_not_read_as_h264() {
        let mut data = Vec::new();
        for _ in 0..8 {
            data.extend_from_slice(&hevc_slice(300, false));
        }
        assert_eq!(scan(&data, 0..data.len(), &hevc_profile()).slices(), 8);
        assert_eq!(
            scan(&data, 0..data.len(), &test_profile()).slices(),
            0,
            "the wrong codec must find nothing rather than something wrong"
        );
    }

    #[test]
    fn the_prefilter_admits_exactly_the_bytes_that_can_open_a_slice() {
        let admitted: Vec<u8> = (0..=255u8).filter(|byte| could_open_a_slice(&[*byte, 0x01], 0, VideoCodec::H264)).collect();
        assert_eq!(admitted, vec![0x01, 0x05, 0x21, 0x25, 0x41, 0x45, 0x61, 0x65]);

        let hevc: Vec<u8> = (0..=255u8).filter(|byte| could_open_a_slice(&[*byte, 0x01], 0, VideoCodec::Hevc)).collect();
        assert_eq!(hevc.len(), 16, "ten trailing types and six random-access ones");
        assert!(hevc.contains(&0x02) && hevc.contains(&0x26));
        // A zero temporal id cannot occur, and rejecting it removes seven eighths of the noise.
        assert!(!could_open_a_slice(&[0x02, 0x00], 0, VideoCodec::Hevc));
        assert!(!could_open_a_slice(&[0x02, 0x40], 0, VideoCodec::Hevc), "layer one");
    }

    #[test]
    fn exp_golomb_reads_the_documented_values() {
        assert_eq!(BitReader::new(&[0b1000_0000]).unsigned_exp_golomb(), Some(0));
        assert_eq!(BitReader::new(&[0b0100_0000]).unsigned_exp_golomb(), Some(1));
        assert_eq!(BitReader::new(&[0b0110_0000]).unsigned_exp_golomb(), Some(2));
        assert_eq!(BitReader::new(&[0b0010_0000]).unsigned_exp_golomb(), Some(3));
        assert_eq!(BitReader::new(&[]).unsigned_exp_golomb(), None);
        // Thirty-two leading zeros cannot introduce a valid value.
        assert_eq!(BitReader::new(&[0, 0, 0, 0, 0]).unsigned_exp_golomb(), None);
    }

    #[test]
    fn the_sample_scans_to_almost_every_sample_start() {
        let data = sample();
        if data.is_empty() {
            return;
        }
        let (profile, mdat) = profile_and_mdat(&data);
        let report = scan(&data, mdat.clone(), &profile);

        // The container declares 830 video samples; the scan is expected to find most of them.
        assert!(report.slices() > 700, "only {} slices found", report.slices());
        assert!(report.keyframes() > 0);
        assert!(report.resynchronizations > 500, "audio is interleaved every sample");
        assert!(report.stopped_at > mdat.end - mdat.len() / 100, "stopped at {} of {}", report.stopped_at, mdat.end);
    }

    #[test]
    fn a_truncated_file_scans_proportionally_less() {
        let data = sample();
        if data.is_empty() {
            return;
        }
        let (profile, _) = profile_and_mdat(&data);
        let mut previous = 0;

        for keep in [30usize, 60, 90] {
            let cut = data.len() * keep / 100;
            let broken = &data[..cut];
            let Some(mdat) = mp4_boxes::top_level(broken).into_iter().find(|b| b.is(*b"mdat")) else {
                continue;
            };
            let report = scan(broken, mdat.payload, &profile);

            assert!(report.slices() >= previous, "{keep}% found fewer slices than {previous}");
            previous = report.slices();
        }
        assert!(previous > 700);
    }

    #[test]
    fn the_annex_b_stream_starts_with_the_parameter_sets() {
        let data = sample();
        if data.is_empty() {
            return;
        }
        let (profile, mdat) = profile_and_mdat(&data);
        let report = scan(&data, mdat, &profile);
        let mut stream = Vec::new();
        let written = write_annex_b(&data, &report, &profile, &mut stream).expect("a Vec never fails");

        assert_eq!(written, stream.len() as u64, "the count must match what was written");
        assert_eq!(stream.get(..4), Some(&[0, 0, 0, 1][..]));
        let first = profile.sequence_parameter_sets.first().expect("one SPS");
        assert_eq!(stream.get(4..4 + first.len()), Some(first.as_slice()));
        // Only slices are carried over, so the stream is shorter than every unit combined.
        assert!(stream.len() > 1000);
    }

    /// HEVC needs its video parameter set before the sequence one, or the stream will not open.
    #[test]
    fn an_hevc_stream_leads_with_the_video_parameter_set() {
        let profile = hevc_profile();
        let mut stream = Vec::new();
        let report = ScanReport {
            units: Vec::new(),
            resynchronizations: 0,
            stopped_at: 0,
        };
        write_annex_b(&[], &report, &profile, &mut stream).expect("a Vec never fails");

        assert_eq!(stream.get(4..8), Some(&[0x40, 0x01, 0x0C, 0x01][..]), "the VPS first");
        assert_eq!(stream.get(12..15), Some(&[0x42, 0x01, 0x01][..]), "then the SPS");
    }

    #[test]
    fn scanning_hostile_bytes_terminates_without_panicking() {
        for codec in [VideoCodec::H264, VideoCodec::Hevc] {
            let profile = Profile {
                codec,
                video_parameter_sets: Vec::new(),
                sequence_parameter_sets: Vec::new(),
                picture_parameter_sets: Vec::new(),
                picture_parameter_set_ids: vec![0],
                max_sample_size: 1 << 16,
                frame_rate: 30.0,
            };
            for pattern in [0x00u8, 0xFF, 0xA5] {
                let data = vec![pattern; 4096];
                let report = scan(&data, 0..data.len(), &profile);
                assert!(report.stopped_at <= data.len());
            }
        }
    }

    #[test]
    fn an_empty_region_yields_nothing() {
        let report = scan(&[], 0..0, &test_profile());
        assert!(report.units.is_empty());
        assert_eq!(report.resynchronizations, 0);
    }
}
