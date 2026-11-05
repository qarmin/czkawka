//! What a healthy file can tell us about a broken one from the same source.
//!
//! When `moov` is gone, the decoder parameters go with it: an H.264 stream in an MP4 stores
//! its SPS and PPS inside `avcC`, not in the media data, so a truncated recording carries no
//! description of itself. Everything needed to read it again therefore has to come from
//! somewhere else - a reference recording, or parameter sets that happen to be in-band.
//!
//! The parts worth keeping are tiny. Measured on the sample files, the reusable part of `moov`
//! is about 750 bytes against 30 to 73 KB of per-file sample tables.

use std::path::Path;
use std::sync::{Mutex, OnceLock};

use serde::{Deserialize, Serialize};

use super::mp4_boxes::{self, BoxHeader};
use crate::file_fixer::io::{SourceFile, read_u32_be, read_u64_be};

/// The two NAL-based codecs the scanner can read.
///
/// They differ in enough places that nothing can be shared by accident: the header is one byte
/// in H.264 and two in HEVC, the unit type sits in different bits, the numbering is unrelated,
/// and the slice header opens with different fields. Every one of those is a place where
/// treating an HEVC stream as H.264 produces a plausible-looking wrong answer rather than an
/// obvious failure, so the codec is carried explicitly instead of being guessed at each site.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VideoCodec {
    H264,
    Hevc,
}

impl VideoCodec {
    /// ffmpeg's demuxer name for this codec's raw byte stream.
    pub fn demuxer(self) -> &'static str {
        match self {
            Self::H264 => "h264",
            Self::Hevc => "hevc",
        }
    }

    /// Extension for a file holding the elementary stream, which is how ffmpeg is told what to
    /// read when no container says.
    pub fn extension(self) -> &'static str {
        match self {
            Self::H264 => "h264",
            Self::Hevc => "h265",
        }
    }

    /// Sample-entry codes an index uses to name this codec.
    fn sample_entries(self) -> &'static [&'static [u8; 4]] {
        match self {
            Self::H264 => &[b"avc1", b"avc2", b"avc3", b"avc4"],
            Self::Hevc => &[b"hvc1", b"hev1", b"hvc2", b"hev2"],
        }
    }
}

/// Everything the scanner and the rebuild need about a stream's encoding.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Profile {
    pub codec: VideoCodec,
    /// HEVC's video parameter sets, which have no H.264 equivalent and stay empty for it.
    pub video_parameter_sets: Vec<Vec<u8>>,
    pub sequence_parameter_sets: Vec<Vec<u8>>,
    pub picture_parameter_sets: Vec<Vec<u8>>,
    /// Identifiers declared by the picture parameter sets, used to reject slices that claim a
    /// parameter set this stream does not have.
    pub picture_parameter_set_ids: Vec<u32>,
    /// Largest video sample the reference contained, which bounds what a plausible one is.
    pub max_sample_size: u32,
    /// Frames per second, needed because the rebuilt file has no timing of its own.
    pub frame_rate: f64,
}

impl Profile {
    /// A ceiling for a single NAL unit. Twice the reference's largest sample leaves room for a
    /// busier scene while still rejecting the multi-megabyte lengths that random bytes produce.
    pub fn max_nal_size(&self) -> u32 {
        self.max_sample_size.saturating_mul(2).max(1 << 16)
    }

    pub fn accepts_parameter_set_id(&self, id: u32) -> bool {
        self.picture_parameter_set_ids.contains(&id)
    }

    /// Whether two profiles describe the same encoder settings.
    ///
    /// The parameter sets are the configuration; `max_sample_size` and `frame_rate` are
    /// measurements of one particular recording. Treating those as part of the identity makes
    /// every file in a collection look unique - a thousand clips off one phone yielded 791
    /// "distinct" profiles instead of the handful actually present, which then crowded the
    /// real alternatives out of the shortlist.
    pub fn same_configuration(&self, other: &Self) -> bool {
        self.codec == other.codec
            && self.video_parameter_sets == other.video_parameter_sets
            && self.sequence_parameter_sets == other.sequence_parameter_sets
            && self.picture_parameter_sets == other.picture_parameter_sets
    }

    /// The parameter sets a decoder needs, in the order a byte stream must carry them.
    pub fn parameter_sets(&self) -> impl Iterator<Item = &Vec<u8>> {
        self.video_parameter_sets.iter().chain(&self.sequence_parameter_sets).chain(&self.picture_parameter_sets)
    }

    /// Widens `self` to also cover what `other` measured.
    fn widen_to(&mut self, other: &Self) {
        self.max_sample_size = self.max_sample_size.max(other.max_sample_size);
    }
}

/// Default frame rate when the reference does not state one. Wrong timing is recoverable by
/// the user; a file that will not build at all is not.
const FALLBACK_FRAME_RATE: f64 = 30.0;

/// Reads a profile out of a reference file.
///
/// A buried index counts. The reference that actually helps a carved recording is another
/// recording from the same camera, and in a carved collection those are damaged the same way -
/// insisting on a healthy one would mean the user has nothing to point at.
pub fn from_reference(data: &[u8]) -> Option<Profile> {
    mp4_boxes::top_level(data)
        .into_iter()
        .find(|b| b.is(*b"moov"))
        .and_then(|moov| from_index(data, &moov))
        .or_else(|| from_buried(data).into_iter().next())
}

/// Profiles from indexes the damaged file still carries, buried ones included.
///
/// A carved file needs no reference at all: its `moov` is sitting inside the oversized `mdat`,
/// and `avcC` describes the encoder settings rather than the particular recording, so it stays
/// the right decoder configuration even when the index turns out to belong to a *different*
/// recording than the media data around it. That happens often in carve output, where the tail
/// of one file gets glued to the body of another, and it is why the sample tables from such an
/// index must never be trusted while its parameter sets still can be.
pub fn from_buried(data: &[u8]) -> Vec<Profile> {
    let mut profiles: Vec<Profile> = Vec::new();
    let found = mp4_boxes::find_indexes(data).into_iter().filter_map(|index| from_index(data, &index));
    merge(&mut profiles, found);
    profiles
}

/// Encoder configurations gathered from the whole batch, for files that carry none themselves.
///
/// Half of a carved collection has no index left at all, and a profile is exactly what such a
/// file needs and cannot supply. Its neighbours can: the recordings came off the same handful
/// of devices, so the configuration is almost always sitting in a *different* file of the same
/// run. Learning them once turns a batch into its own reference library.
///
/// The set stays small on its own - a profile is about 750 bytes and identical configurations
/// collapse - so a directory of thousands of clips yields a few dozen entries.
static LEARNED: OnceLock<Mutex<Vec<Profile>>> = OnceLock::new();

fn learned_store() -> &'static Mutex<Vec<Profile>> {
    LEARNED.get_or_init(|| Mutex::new(Vec::new()))
}

/// Adds profiles to `known`, folding each into an entry with the same configuration.
///
/// Deduplicating on configuration rather than on the whole value is what keeps a shortlist
/// meaningful: the alternatives worth trying are the distinct encoder settings, and a list
/// padded with one entry per source file crowds them out before anything gets probed.
pub fn merge(known: &mut Vec<Profile>, profiles: impl IntoIterator<Item = Profile>) {
    for profile in profiles {
        match known.iter_mut().find(|entry| entry.same_configuration(&profile)) {
            Some(entry) => entry.widen_to(&profile),
            None => known.push(profile),
        }
    }
}

pub fn learn(profiles: impl IntoIterator<Item = Profile>) {
    if let Ok(mut store) = learned_store().lock() {
        merge(&mut store, profiles);
    }
}

/// Drops everything learned so far, from memory only.
pub fn forget_all() {
    if let Ok(mut store) = learned_store().lock() {
        store.clear();
    }
    if let Ok(mut store) = learned_mpeg4_store().lock() {
        store.clear();
    }
}

pub fn learned() -> Vec<Profile> {
    learned_store().lock().map_or_else(|_| Vec::new(), |store| store.clone())
}

/// Every encoder configuration one file carries, mapped rather than read so that harvesting a
/// directory of 4 GB recordings costs no memory.
///
/// Also learns the MPEG-4 Part 2 headers the same file may hold, for the same reason and by the
/// same route: an old phone recording keeps its picture dimensions in `esds`, so a carve that
/// lost `moov` cannot be decoded at all without a sibling to supply them.
pub fn harvest_file(path: &Path) -> Vec<Profile> {
    let source = match SourceFile::open(path) {
        Ok(source) => source,
        Err(error) => {
            log::debug!("cannot learn from {}: {error}", path.display());
            return Vec::new();
        }
    };
    let data = source.data();

    // A file whose index is where it belongs is read almost for free: walking the top level
    // touches a handful of box headers.
    if let Some(moov) = mp4_boxes::top_level(data).into_iter().find(|b| b.is(*b"moov")) {
        if let Some(region) = data.get(moov.payload.clone())
            && let Some(headers) = mpeg4_headers_in(region)
        {
            learn_mpeg4(std::iter::once(headers));
        }
        if let Some(profile) = from_index(data, &moov) {
            return vec![profile];
        }
    }

    // Only a file that has *lost* its index needs searching, and then only near the ends: an
    // index sits at the front or the back of a recording, and scanning the middle of a
    // five-gigabyte file for one turns "learn from this directory" into reading every byte of
    // it. What this misses costs nothing, because a file being *repaired* scans itself in full
    // anyway - learning only has to supply configurations to files that carry none at all.
    let mut profiles = Vec::new();
    for window in windows(data) {
        if let Some(headers) = mpeg4_headers_in(window) {
            learn_mpeg4(std::iter::once(headers));
        }
        merge(&mut profiles, from_buried(window));
    }
    profiles
}

/// The two regions of a file worth searching when learning: its first and last window.
fn windows(data: &[u8]) -> Vec<&[u8]> {
    let head = data.get(..HARVEST_WINDOW.min(data.len())).unwrap_or(data);
    match data.get(data.len().saturating_sub(HARVEST_WINDOW)..) {
        Some(tail) if data.len() > HARVEST_WINDOW => vec![head, tail],
        _ => vec![head],
    }
}

/// How much of each end of a file to search for an index when learning from a batch.
///
/// Sixteen megabytes rather than "as far as it takes": learning gathers a *shared* library of
/// encoder configurations, so an index missed in one file is almost always present in another
/// off the same device. Measured on a 62 GB carved collection, this is the difference between
/// a learning pass that reads 24 GB and one that reads 6.
const HARVEST_WINDOW: usize = 16 << 20;

/// MPEG-4 Part 2 configuration headers gathered from the batch, kept beside the H.264 ones.
static LEARNED_MPEG4: OnceLock<Mutex<Vec<Vec<u8>>>> = OnceLock::new();

fn learned_mpeg4_store() -> &'static Mutex<Vec<Vec<u8>>> {
    LEARNED_MPEG4.get_or_init(|| Mutex::new(Vec::new()))
}

pub fn learn_mpeg4(headers: impl IntoIterator<Item = Vec<u8>>) {
    if let Ok(mut store) = learned_mpeg4_store().lock() {
        for header in headers {
            if !store.contains(&header) {
                store.push(header);
            }
        }
    }
}

pub fn learned_mpeg4() -> Vec<Vec<u8>> {
    learned_mpeg4_store().lock().map_or_else(|_| Vec::new(), |store| store.clone())
}

/// The MPEG-4 Part 2 sequence headers a file carries inside its `esds` box.
///
/// These are the visual object sequence and video object layer headers - the ones that state
/// the picture dimensions. Unlike H.264, which many encoders also emit in-band, MPEG-4 Part 2
/// in a 3GP file keeps them *only* here: measured across seven carved phone recordings, not one
/// contained a single `000001B0` or `000001B5` start code in its media data. Losing `moov`
/// therefore loses the dimensions, and a stream without them will not even open.
///
/// The descriptor nesting inside `esds` is variable-length and fiddly; the sequence header is
/// found by its start code within the box instead, which is unambiguous in a region that small.
/// The box is found by signature rather than by walking to it: `esds` sits inside a sample
/// entry, whose layout varies by codec and which the box walker does not descend into - the
/// same reason `avcC` is located this way.
pub fn mpeg4_headers(data: &[u8]) -> Option<Vec<u8>> {
    let regions = std::iter::once(mp4_boxes::top_level(data).into_iter().find(|b| b.is(*b"moov")))
        .flatten()
        .chain(mp4_boxes::find_indexes(data));

    for moov in regions {
        if let Some(region) = data.get(moov.payload.clone())
            && let Some(headers) = mpeg4_headers_in(region)
        {
            return Some(headers);
        }
    }
    None
}

/// The same search, confined to a region already known to hold an index.
fn mpeg4_headers_in(region: &[u8]) -> Option<Vec<u8>> {
    const VISUAL_OBJECT_SEQUENCE: [u8; 4] = [0, 0, 1, 0xB0];
    /// The headers are a few dozen bytes; anything past this is the next box, not the sequence.
    const MAX_HEADER_LEN: usize = 256;

    let esds = region.windows(4).position(|window| window == *b"esds")?;
    let tail = region.get(esds..)?;
    let start = tail
        .windows(4)
        .position(|window| window == VISUAL_OBJECT_SEQUENCE)
        .filter(|position| *position < MAX_HEADER_LEN)?;
    let end = (start + MAX_HEADER_LEN).min(tail.len());
    tail.get(start..end).filter(|headers| headers.len() >= 8).map(<[u8]>::to_vec)
}

fn from_index(data: &[u8], moov: &BoxHeader) -> Option<Profile> {
    let track = video_track(data, moov)?;
    let config = decoder_config(data, &track)?;

    Some(Profile {
        picture_parameter_set_ids: config.parameter_set_ids(),
        codec: config.codec,
        video_parameter_sets: config.video,
        sequence_parameter_sets: config.sequence,
        picture_parameter_sets: config.picture,
        max_sample_size: largest_sample(data, &track).unwrap_or(1 << 20),
        frame_rate: frame_rate(data, &track).unwrap_or(FALLBACK_FRAME_RATE),
    })
}

/// Builds a profile from parameter sets carried in the media data itself.
///
/// Encoders that also emit SPS and PPS in-band leave a truncated file self-describing, which
/// removes the need for a reference entirely. The sample files here do not do this, but many
/// streams, especially anything derived from a broadcast or RTP source, do.
pub fn from_in_band(data: &[u8], mdat: &BoxHeader) -> Option<Profile> {
    let region = data.get(mdat.payload.clone())?;
    let mut h264 = DecoderConfig::empty(VideoCodec::H264);
    let mut hevc = DecoderConfig::empty(VideoCodec::Hevc);

    let mut offset = 0;
    while offset + 5 <= region.len() {
        let Some(length) = read_u32_be(region, offset).map(|l| l as usize) else {
            break;
        };
        let Some(unit) = region.get(offset + 4..offset + 4 + length) else {
            offset += 1;
            continue;
        };
        h264.take_h264(unit);
        hevc.take_hevc(unit);
        if h264.is_complete() || hevc.is_complete() {
            break;
        }
        offset += 4 + length.max(1);
    }

    // HEVC first: an H.264 reading of an HEVC parameter set occasionally lands on types 7 and 8
    // by coincidence, while the reverse needs a two-byte header that H.264 does not have.
    let config = [hevc, h264].into_iter().find(DecoderConfig::is_complete)?;
    Some(Profile {
        picture_parameter_set_ids: config.parameter_set_ids(),
        codec: config.codec,
        video_parameter_sets: config.video,
        sequence_parameter_sets: config.sequence,
        picture_parameter_sets: config.picture,
        max_sample_size: 1 << 20,
        frame_rate: FALLBACK_FRAME_RATE,
    })
}

/// How many video samples an index declares.
///
/// This is the denominator a relinked file is scored against. It is only meaningful when the
/// index really does describe the media around it, which is exactly the case where relinking
/// succeeds - a mismatched index makes the decoder complain long before the count matters.
pub fn declared_video_samples(data: &[u8], moov: &BoxHeader) -> Option<u64> {
    let track = video_track(data, moov)?;
    let stbl = sample_table(data, &track)?;
    let stsz = child(data, &stbl, *b"stsz")?;
    read_u32_be(data, stsz.payload.start + 8).map(u64::from)
}

/// Whether an index really describes the media around it.
///
/// A carved file frequently carries an index belonging to a *different* recording: the tool
/// that recovered it glued the tail of one file to the body of another, and the sample table
/// that parses so cleanly addresses bytes that are not there. Testing that by building the
/// candidate and decoding it costs a full copy of the file - gigabytes of writing per index,
/// on a collection where most indexes do not match - so it is worth asking the bytes first.
///
/// The question has an exact answer for H.264: a sample is a run of length-prefixed NAL units
/// that tiles the sample size *to the byte*. Checking a handful of samples at the first chunk
/// settles it; random data does not survive four such checks in a row.
///
/// `None` means "no opinion" - a codec whose samples are not NAL units, or an index this
/// cannot read - and the caller should fall back to deciding by decoding.
pub fn index_describes_media(data: &[u8], moov: &BoxHeader) -> Option<bool> {
    const PROBED_SAMPLES: usize = 4;

    let track = video_track(data, moov)?;
    let stbl = sample_table(data, &track)?;
    let codec = codec_of_entry(first_sample_entry(data, &stbl)?)?;
    let length_size = nal_length_size(data, &stbl, codec).unwrap_or(4);

    let stco = child(data, &stbl, *b"stco");
    let co64 = child(data, &stbl, *b"co64");
    let first_chunk = match (&stco, &co64) {
        (Some(stco), _) => u64::from(read_u32_be(data, stco.payload.start + 8)?),
        (_, Some(co64)) => read_u64_be(data, co64.payload.start + 8)?,
        _ => return None,
    };

    // Only the samples inside the *first chunk* are contiguous with it. Interleaving puts the
    // audio chunk immediately after, so probing past the chunk boundary walks into AAC and
    // reports a perfectly good index as a mismatch - which is what a first version of this did
    // to the project's own test file, where chunk zero holds exactly two samples.
    let in_first_chunk = samples_in_first_chunk(data, &stbl)?;
    let sizes = first_sample_sizes(data, &stbl, PROBED_SAMPLES.min(in_first_chunk))?;
    if sizes.is_empty() {
        return None;
    }
    Some(nal_chain_tiles(data, usize::try_from(first_chunk).ok()?, &sizes, length_size, codec))
}

/// A track's chunk-offset table: where it lives in the file and what it says.
pub struct ChunkOffsets {
    /// Offset of the first entry, so the table can be rewritten in place.
    pub entries_at: usize,
    /// Four for `stco`, eight for `co64`.
    pub entry_len: usize,
    pub offsets: Vec<u64>,
}

impl ChunkOffsets {
    /// The table rewritten with every offset moved by `shift`, or `None` when one of them would
    /// land outside the file or outside what its own field can hold.
    pub fn shifted(&self, shift: i64, len: usize) -> Option<Vec<u8>> {
        let mut out = Vec::with_capacity(self.offsets.len() * self.entry_len);
        for offset in &self.offsets {
            let moved = offset.checked_add_signed(shift)?;
            if moved >= len as u64 {
                return None;
            }
            match self.entry_len {
                4 => out.extend_from_slice(&u32::try_from(moved).ok()?.to_be_bytes()),
                8 => out.extend_from_slice(&moved.to_be_bytes()),
                _ => return None,
            }
        }
        Some(out)
    }
}

/// Every chunk-offset table in the index, one per track.
pub fn chunk_offset_tables(data: &[u8], moov: &BoxHeader) -> Vec<ChunkOffsets> {
    mp4_boxes::walk(data, moov.payload.clone())
        .into_iter()
        .filter(|found| found.is(*b"trak"))
        .filter_map(|trak| sample_table(data, &trak))
        .filter_map(|stbl| chunk_offsets(data, &stbl))
        .collect()
}

fn chunk_offsets(data: &[u8], stbl: &BoxHeader) -> Option<ChunkOffsets> {
    let (table, entry_len) = match (child(data, stbl, *b"stco"), child(data, stbl, *b"co64")) {
        (Some(stco), _) => (stco, 4),
        (_, Some(co64)) => (co64, 8),
        _ => return None,
    };
    let entries_at = table.payload.start + 8;
    let count = read_u32_be(data, table.payload.start + 4)? as usize;

    let offsets = (0..count)
        .map(|index| {
            let at = entries_at + index * entry_len;
            match entry_len {
                4 => read_u32_be(data, at).map(u64::from),
                _ => read_u64_be(data, at),
            }
        })
        .collect::<Option<Vec<u64>>>()?;

    Some(ChunkOffsets { entries_at, entry_len, offsets })
}

/// How far the media an index describes has moved, when it has moved by a constant.
///
/// This is the other half of [`index_describes_media`]: that one answers whether the sample
/// table addresses the bytes around it, and on a carved collection it answers no for most of
/// them. Sometimes that is because the index belongs to an entirely different recording, and
/// nothing can be done. But often the recording *is* here and simply starts somewhere else -
/// the carve began a few sectors early or late - and then every offset in the file is wrong by
/// the same amount, and correcting them gives back the whole recording with its audio and its
/// timing, which a stream scan can never do.
///
/// The shift is found by looking for where the first chunk's samples really are: a run of
/// length-prefixed NAL units that tiles the declared sizes exactly. A handful of other chunks
/// then have to look like NAL units at the same displacement, which is what separates the real
/// answer from a coincidence.
pub fn chunk_offset_shift(data: &[u8], moov: &BoxHeader) -> Option<i64> {
    /// Chunks checked at the candidate displacement before it is believed.
    const CONFIRMING_CHUNKS: usize = 8;

    let track = video_track(data, moov)?;
    let stbl = sample_table(data, &track)?;
    let codec = codec_of_entry(first_sample_entry(data, &stbl)?)?;
    let length_size = nal_length_size(data, &stbl, codec).unwrap_or(4);

    let in_first_chunk = samples_in_first_chunk(data, &stbl)?;
    let sizes = first_sample_sizes(data, &stbl, in_first_chunk.min(4))?;
    let table = chunk_offsets(data, &stbl)?;
    let declared = table.offsets.first().copied()?;

    let mut at = 0usize;
    while at + length_size < data.len() {
        if nal_chain_tiles(data, at, &sizes, length_size, codec) {
            let shift = at as i64 - declared as i64;
            if shift != 0 && confirms(data, &table, shift, length_size, codec, CONFIRMING_CHUNKS) {
                return Some(shift);
            }
        }
        at += 1;
    }
    None
}

/// Whether other chunks also look like NAL units once moved by `shift`.
fn confirms(data: &[u8], table: &ChunkOffsets, shift: i64, length_size: usize, codec: VideoCodec, wanted: usize) -> bool {
    let step = (table.offsets.len() / wanted.max(1)).max(1);
    let checked: Vec<u64> = table.offsets.iter().copied().step_by(step).take(wanted).collect();
    if checked.is_empty() {
        return false;
    }

    checked.iter().all(|offset| {
        offset
            .checked_add_signed(shift)
            .and_then(|moved| usize::try_from(moved).ok())
            .is_some_and(|moved| read_u32_be(data, moved).is_some_and(|length| length > 0) && super::mp4_scan::header_is_plausible(data, moved + length_size, codec))
    })
}

/// Which codec a sample-entry code names, when it is one the NAL reader understands.
fn codec_of_entry(entry: [u8; 4]) -> Option<VideoCodec> {
    [VideoCodec::H264, VideoCodec::Hevc].into_iter().find(|codec| codec.sample_entries().contains(&&entry))
}

/// The four-character code of the first entry in the sample description.
fn first_sample_entry(data: &[u8], stbl: &BoxHeader) -> Option<[u8; 4]> {
    let stsd = child(data, stbl, *b"stsd")?;
    // stsd payload: version+flags, entry count, then the entries, each `size` + `type`.
    data.get(stsd.payload.start + 12..stsd.payload.start + 16)?.try_into().ok()
}

/// Bytes of length prefix in front of each NAL unit, from the codec's configuration record.
///
/// The field says the same thing in both records and sits in a different place: byte four of
/// `avcC`, byte twenty-one of `hvcC`, after the profile and tier description HEVC carries.
fn nal_length_size(data: &[u8], stbl: &BoxHeader, codec: VideoCodec) -> Option<usize> {
    let (code, at) = match codec {
        VideoCodec::H264 => (*b"avcC", 4),
        VideoCodec::Hevc => (*b"hvcC", 21),
    };
    let stsd = child(data, stbl, *b"stsd")?;
    let region = data.get(stsd.payload)?;
    let record = region.get(find_record(region, code)?..)?;
    Some(usize::from(*record.get(at)? & 0x03) + 1)
}

/// How many samples the first chunk holds, from the sample-to-chunk table's first entry.
fn samples_in_first_chunk(data: &[u8], stbl: &BoxHeader) -> Option<usize> {
    let stsc = child(data, stbl, *b"stsc")?;
    let start = stsc.payload.start;
    if read_u32_be(data, start + 4)? == 0 {
        return None;
    }
    // Entries are (first_chunk, samples_per_chunk, description); the first one covers chunk 1.
    let per_chunk = read_u32_be(data, start + 12)? as usize;
    (per_chunk > 0).then_some(per_chunk)
}

fn first_sample_sizes(data: &[u8], stbl: &BoxHeader, wanted: usize) -> Option<Vec<u32>> {
    let stsz = child(data, stbl, *b"stsz")?;
    let start = stsz.payload.start;
    let uniform = read_u32_be(data, start + 4)?;
    let count = read_u32_be(data, start + 8)? as usize;
    if uniform > 0 {
        return Some(vec![uniform; wanted.min(count)]);
    }
    (0..wanted.min(count)).map(|index| read_u32_be(data, start + 12 + index * 4)).collect()
}

/// True when length-prefixed NAL units tile every one of `sizes` exactly, starting at `at`.
fn nal_chain_tiles(data: &[u8], at: usize, sizes: &[u32], length_size: usize, codec: VideoCodec) -> bool {
    let mut position = at;
    for size in sizes {
        let mut consumed = 0usize;
        while consumed < *size as usize {
            let head = position + consumed;
            let Some(prefix) = data.get(head..head + length_size) else {
                return false;
            };
            let length = prefix.iter().fold(0usize, |value, byte| (value << 8) | usize::from(*byte));
            if length == 0 || !super::mp4_scan::header_is_plausible(data, head + length_size, codec) {
                return false;
            }
            consumed += length_size + length;
        }
        if consumed != *size as usize {
            return false;
        }
        position += consumed;
    }
    true
}

fn video_track(data: &[u8], moov: &BoxHeader) -> Option<BoxHeader> {
    mp4_boxes::walk(data, moov.payload.clone())
        .into_iter()
        .filter(|b| b.is(*b"trak"))
        .find(|trak| handler_of(data, trak) == Some(*b"vide"))
}

fn handler_of(data: &[u8], trak: &BoxHeader) -> Option<[u8; 4]> {
    let mdia = child(data, trak, *b"mdia")?;
    let hdlr = child(data, &mdia, *b"hdlr")?;
    // hdlr payload: version+flags, pre_defined, then the four-character handler type.
    data.get(hdlr.payload.start + 8..hdlr.payload.start + 12)?.try_into().ok()
}

fn child(data: &[u8], parent: &BoxHeader, kind: [u8; 4]) -> Option<BoxHeader> {
    mp4_boxes::walk(data, parent.payload.clone()).into_iter().find(|b| b.is(kind))
}

fn sample_table(data: &[u8], trak: &BoxHeader) -> Option<BoxHeader> {
    let mdia = child(data, trak, *b"mdia")?;
    let minf = child(data, &mdia, *b"minf")?;
    child(data, &minf, *b"stbl")
}

/// The parameter sets a codec's configuration record carries, whichever record that is.
struct DecoderConfig {
    codec: VideoCodec,
    video: Vec<Vec<u8>>,
    sequence: Vec<Vec<u8>>,
    picture: Vec<Vec<u8>>,
}

impl DecoderConfig {
    fn empty(codec: VideoCodec) -> Self {
        Self {
            codec,
            video: Vec::new(),
            sequence: Vec::new(),
            picture: Vec::new(),
        }
    }

    /// HEVC needs no `video` here: ffmpeg reconstructs a missing VPS from the SPS, and some
    /// cameras genuinely omit it from the configuration record.
    fn is_complete(&self) -> bool {
        !self.sequence.is_empty() && !self.picture.is_empty()
    }

    fn parameter_set_ids(&self) -> Vec<u32> {
        self.picture.iter().filter_map(|set| parameter_set_id(set, self.codec)).collect()
    }

    fn take_h264(&mut self, unit: &[u8]) {
        match unit.first().map(|header| header & 0x1F) {
            Some(7) => self.sequence.push(unit.to_vec()),
            Some(8) => self.picture.push(unit.to_vec()),
            _ => {}
        }
    }

    fn take_hevc(&mut self, unit: &[u8]) {
        if unit.first().is_none_or(|header| header & 0x81 != 0) {
            return;
        }
        match unit.first().map(|header| (header >> 1) & 0x3F) {
            Some(32) => self.video.push(unit.to_vec()),
            Some(33) => self.sequence.push(unit.to_vec()),
            Some(34) => self.picture.push(unit.to_vec()),
            _ => {}
        }
    }
}

/// Pulls the parameter sets out of whichever configuration record the track carries.
///
/// The record is searched for inside the sample description rather than walked to precisely:
/// it sits inside a sample entry whose layout varies by codec, and the four-character code is
/// unambiguous within that bounded region.
fn decoder_config(data: &[u8], trak: &BoxHeader) -> Option<DecoderConfig> {
    let stbl = sample_table(data, trak)?;
    let stsd = child(data, &stbl, *b"stsd")?;
    let region = data.get(stsd.payload)?;

    if let Some(position) = find_record(region, *b"avcC") {
        let config = avc_config(region.get(position..)?);
        if config.as_ref().is_some_and(DecoderConfig::is_complete) {
            return config;
        }
    }
    let position = find_record(region, *b"hvcC")?;
    hevc_config(region.get(position..)?).filter(DecoderConfig::is_complete)
}

fn find_record(region: &[u8], code: [u8; 4]) -> Option<usize> {
    region.windows(4).position(|window| window == code).map(|at| at + 4)
}

/// `avcC`: version, profile, compatibility, level, lengthSizeMinusOne, then the two arrays.
fn avc_config(record: &[u8]) -> Option<DecoderConfig> {
    let mut config = DecoderConfig::empty(VideoCodec::H264);
    let mut offset = 6;
    config.sequence = read_parameter_sets(record, &mut offset, usize::from(*record.get(5)? & 0x1F))?;

    let pps_count = usize::from(*record.get(offset)?);
    offset += 1;
    config.picture = read_parameter_sets(record, &mut offset, pps_count)?;
    Some(config)
}

/// `hvcC`: twenty-two bytes of fixed fields, then one array per NAL unit type it carries.
///
/// The arrays are what differ from `avcC` - a count of arrays, each tagged with the unit type
/// it holds, rather than two hard-coded lists - and the type tag is the only thing that says
/// whether an array is video, sequence or picture parameter sets.
fn hevc_config(record: &[u8]) -> Option<DecoderConfig> {
    const ARRAYS_AT: usize = 22;

    let mut config = DecoderConfig::empty(VideoCodec::Hevc);
    let arrays = usize::from(*record.get(ARRAYS_AT)?);
    let mut offset = ARRAYS_AT + 1;

    for _ in 0..arrays {
        let kind = *record.get(offset)? & 0x3F;
        let count = usize::from(u16::from_be_bytes(record.get(offset + 1..offset + 3)?.try_into().ok()?));
        offset += 3;
        let sets = read_parameter_sets(record, &mut offset, count)?;
        match kind {
            32 => config.video = sets,
            33 => config.sequence = sets,
            34 => config.picture = sets,
            _ => {}
        }
    }
    Some(config)
}

fn read_parameter_sets(record: &[u8], offset: &mut usize, count: usize) -> Option<Vec<Vec<u8>>> {
    let mut sets = Vec::with_capacity(count);
    for _ in 0..count {
        let length = usize::from(u16::from_be_bytes(record.get(*offset..*offset + 2)?.try_into().ok()?));
        sets.push(record.get(*offset + 2..*offset + 2 + length)?.to_vec());
        *offset += 2 + length;
    }
    Some(sets)
}

/// The identifier a picture parameter set declares, which is its first `ue(v)` field.
///
/// The field sits behind the NAL header, and that header is one byte in H.264 and two in HEVC -
/// reading an HEVC set as H.264 decodes the wrong bits into a plausible small number, which the
/// scanner would then happily accept from every slice in the stream.
fn parameter_set_id(set: &[u8], codec: VideoCodec) -> Option<u32> {
    let header_len = match codec {
        VideoCodec::H264 => 1,
        VideoCodec::Hevc => 2,
    };
    super::mp4_scan::BitReader::new(set.get(header_len..)?).unsigned_exp_golomb()
}

fn largest_sample(data: &[u8], trak: &BoxHeader) -> Option<u32> {
    let stbl = sample_table(data, trak)?;
    let stsz = child(data, &stbl, *b"stsz")?;
    let start = stsz.payload.start;

    let uniform = read_u32_be(data, start + 4)?;
    if uniform > 0 {
        return Some(uniform);
    }
    let count = read_u32_be(data, start + 8)? as usize;
    (0..count.min(1 << 22)).filter_map(|index| read_u32_be(data, start + 12 + index * 4)).max()
}

fn frame_rate(data: &[u8], trak: &BoxHeader) -> Option<f64> {
    let mdia = child(data, trak, *b"mdia")?;
    let mdhd = child(data, &mdia, *b"mdhd")?;
    let timescale = read_u32_be(data, mdhd.payload.start + 12)?;

    let stbl = sample_table(data, trak)?;
    let stts = child(data, &stbl, *b"stts")?;
    let delta = read_u32_be(data, stts.payload.start + 12)?;

    (timescale > 0 && delta > 0).then(|| f64::from(timescale) / f64::from(delta))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Vec<u8> {
        std::fs::read("tmp/a.mp4").unwrap_or_default()
    }

    /// An index is only worth building a candidate from if it describes the bytes that are
    /// actually here. Asking costs a few kilobytes of reading; finding out by decoding costs a
    /// full copy of the file.
    #[test]
    fn an_index_that_belongs_to_this_file_is_recognized() {
        let data = sample();
        if data.is_empty() {
            return;
        }
        let index = mp4_boxes::find_indexes(&data).pop().expect("the sample has an index");
        assert_eq!(index_describes_media(&data, &index), Some(true));
    }

    /// The mistake this check exists to avoid, and the one it fell into first: only the samples
    /// inside the first chunk are contiguous with it, because the audio chunk follows
    /// immediately. Probing past that boundary walks into AAC and condemns a perfect index.
    #[test]
    fn only_the_samples_inside_the_first_chunk_are_probed() {
        let data = sample();
        if data.is_empty() {
            return;
        }
        let index = mp4_boxes::find_indexes(&data).pop().expect("an index");
        let track = video_track(&data, &index).expect("a video track");
        let stbl = sample_table(&data, &track).expect("a sample table");

        let in_chunk = samples_in_first_chunk(&data, &stbl).expect("a sample-to-chunk table");
        assert!(in_chunk >= 1);
        let sizes = first_sample_sizes(&data, &stbl, in_chunk).expect("sample sizes");
        assert!(nal_chain_tiles(&data, 48, &sizes, 4, VideoCodec::H264), "the first chunk's samples tile exactly");

        // One sample further is the audio chunk, and the tiling has to fail there.
        let too_many = first_sample_sizes(&data, &stbl, in_chunk + 1).expect("sample sizes");
        assert!(!nal_chain_tiles(&data, 48, &too_many, 4, VideoCodec::H264));
    }

    /// An index describing a recording that is not in this file - what a carve glues together -
    /// has to be refused before anything is built from it.
    #[test]
    fn an_index_describing_other_media_is_refused() {
        let mut data = sample();
        if data.is_empty() {
            return;
        }
        let index = mp4_boxes::find_indexes(&data).pop().expect("an index");
        // Overwrite the media the index points at, leaving the index itself untouched.
        if let Some(media) = data.get_mut(48..4096) {
            media.fill(0x5A);
        }
        assert_eq!(index_describes_media(&data, &index), Some(false));
    }

    #[test]
    fn a_reference_yields_the_parameter_sets_and_limits() {
        let data = sample();
        if data.is_empty() {
            return;
        }
        let profile = from_reference(&data).expect("the sample has a video track");

        assert_eq!(profile.sequence_parameter_sets.len(), 1);
        assert_eq!(profile.picture_parameter_sets.len(), 1);
        assert_eq!(profile.picture_parameter_set_ids, vec![0]);
        assert_eq!(profile.max_sample_size, 175_737);
        assert!((profile.frame_rate - 30.0).abs() < 0.01, "{}", profile.frame_rate);
    }

    #[test]
    fn the_nal_ceiling_leaves_headroom_over_the_reference() {
        let profile = Profile {
            codec: VideoCodec::H264,
            video_parameter_sets: Vec::new(),
            sequence_parameter_sets: Vec::new(),
            picture_parameter_sets: Vec::new(),
            picture_parameter_set_ids: vec![0],
            max_sample_size: 100_000,
            frame_rate: 30.0,
        };
        assert_eq!(profile.max_nal_size(), 200_000);
        assert!(profile.accepts_parameter_set_id(0));
        assert!(!profile.accepts_parameter_set_id(7));
    }

    #[test]
    fn a_tiny_reference_still_gets_a_usable_ceiling() {
        let profile = Profile {
            codec: VideoCodec::H264,
            video_parameter_sets: Vec::new(),
            sequence_parameter_sets: Vec::new(),
            picture_parameter_sets: Vec::new(),
            picture_parameter_set_ids: Vec::new(),
            max_sample_size: 10,
            frame_rate: 30.0,
        };
        assert_eq!(profile.max_nal_size(), 1 << 16);
    }

    #[test]
    fn nothing_is_extracted_from_data_that_is_not_a_container() {
        assert!(from_reference(&[0u8; 128]).is_none());
    }

    fn profile(sps: &[u8], max_sample_size: u32) -> Profile {
        Profile {
            codec: VideoCodec::H264,
            video_parameter_sets: Vec::new(),
            sequence_parameter_sets: vec![sps.to_vec()],
            picture_parameter_sets: vec![vec![0x68, 0xEE]],
            picture_parameter_set_ids: vec![0],
            max_sample_size,
            frame_rate: 30.0,
        }
    }

    /// A thousand clips off one phone are one configuration, not a thousand. Folding them is
    /// what leaves room in the shortlist for the configurations that genuinely differ.
    #[test]
    fn profiles_fold_together_when_the_encoder_settings_match() {
        let mut known = Vec::new();
        merge(&mut known, [profile(b"sps-a", 1000), profile(b"sps-a", 5000)]);

        assert_eq!(known.len(), 1);
        assert_eq!(known[0].max_sample_size, 5000, "the wider measurement must win");

        merge(&mut known, [profile(b"sps-b", 10)]);
        assert_eq!(known.len(), 2);
    }

    /// Folding must widen the ceiling rather than narrow it, or the merged profile rejects
    /// samples that one of its sources accepted.
    #[test]
    fn folding_never_narrows_the_nal_ceiling() {
        let mut known = vec![profile(b"sps-a", 5000)];
        merge(&mut known, [profile(b"sps-a", 10)]);
        assert_eq!(known[0].max_sample_size, 5000);
    }

    #[test]
    fn differing_parameter_sets_are_different_configurations() {
        assert!(profile(b"sps-a", 1).same_configuration(&profile(b"sps-a", 999)));
        assert!(!profile(b"sps-a", 1).same_configuration(&profile(b"sps-b", 1)));
    }

    /// A reference is usually just as damaged as the file being repaired, because they came
    /// out of the same recovery run. Only its parameter sets are wanted, and those survive.
    #[test]
    fn a_reference_may_itself_have_a_buried_index() {
        let data = sample();
        if data.is_empty() {
            return;
        }
        let moov = mp4_boxes::find_all(&data, *b"moov").pop().expect("the sample has an index");
        let expected = from_reference(&data).expect("the healthy sample yields a profile");

        // Swallow the index the way a carving tool does, by overstating the `mdat` length.
        let mut carved = data.clone();
        let mdat = mp4_boxes::find_all(&data, *b"mdat").pop().expect("the sample has media data");
        let claimed = (carved.len() - mdat.start) as u32;
        carved[mdat.start..mdat.start + 4].copy_from_slice(&claimed.to_be_bytes());

        assert!(!mp4_boxes::structure(&carved).has_moov, "the index must be hidden");
        assert!(mp4_boxes::find_indexes(&carved).iter().any(|b| b.start == moov.start));
        assert_eq!(from_reference(&carved).as_ref(), Some(&expected));
    }

    #[test]
    fn truncating_the_reference_never_panics() {
        let data = sample();
        if data.is_empty() {
            return;
        }
        for len in (0..data.len()).step_by(data.len() / 60 + 1) {
            let _ = from_reference(&data[..len]);
        }
    }

    /// These files carry no in-band parameter sets, which is exactly why a reference is needed.
    #[test]
    fn the_sample_has_no_in_band_parameter_sets() {
        let data = sample();
        if data.is_empty() {
            return;
        }
        let mdat = mp4_boxes::top_level(&data).into_iter().find(|b| b.is(*b"mdat")).expect("the sample has media data");
        assert!(from_in_band(&data, &mdat).is_none());
    }
}
