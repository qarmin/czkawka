//! Finding the stream chunks of an AVI whose header is gone.
//!
//! An AVI's media data is a chain of `##dc`-style chunks, each carrying its own four-character
//! code and its own little-endian length. That makes it self-delimiting in exactly the way
//! MP4's length-prefixed NAL units are (see [`super::mp4_scan`]), and it is a far stronger
//! signal: the code is four bytes of structure rather than one NAL header, so a chain of a few
//! chunks is essentially impossible to hit by accident. When the `RIFF` header and the index
//! are destroyed, walking that chain recovers every frame boundary the camera recorded, exactly
//! where it recorded it - nothing has to be guessed about the content itself.
//!
//! Two departures from the specification are needed to read real files. Chunks with an odd
//! length are supposed to be followed by a pad byte, and the DVR firmware this was written
//! against does not write one, so both spacings are accepted. And the data may be grouped into
//! `LIST`/`rec ` blocks or interrupted by `JUNK`, so those are stepped over rather than treated
//! as the end of the chain.

use std::ops::Range;

use crate::file_fixer::io::read_u32_le;

/// Chunks that must chain together before a position is believed to hold stream data.
///
/// Four four-character codes each followed by a length that lands exactly on the next one is
/// not something random bytes do; one is, which is why the start of the stream is never taken
/// from a single match.
const MIN_CHAIN: usize = 4;

/// Bytes of a payload examined to classify the unit it opens with.
const HEADER_WINDOW: usize = 16;

/// Chunk kinds that carry stream data, as the two bytes that follow the stream number.
const STREAM_KINDS: [&[u8; 2]; 5] = [b"dc", b"db", b"wb", b"tx", b"pc"];

/// Video codecs a rebuilt header can describe.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Codec {
    Hevc,
    H264,
    Mpeg4,
    Jpeg,
}

impl Codec {
    /// The four-character code an AVI stream header names this codec by.
    pub fn fourcc(self) -> [u8; 4] {
        match self {
            Self::Hevc => *b"H265",
            Self::H264 => *b"H264",
            Self::Mpeg4 => *b"XVID",
            Self::Jpeg => *b"MJPG",
        }
    }

    /// Extension for a file holding this codec's elementary stream, which is how ffprobe is
    /// told what demuxer to read a headerless byte stream with.
    pub fn extension(self) -> &'static str {
        match self {
            Self::Hevc => "h265",
            Self::H264 => "h264",
            Self::Mpeg4 => "m4v",
            Self::Jpeg => "mjpeg",
        }
    }

    /// Whether a four-character code out of some other file names this codec.
    ///
    /// One codec has many spellings - the same HEVC stream is `H265`, `HEVC` or `hvc1`
    /// depending on who wrote the file - and getting this wrong means rejecting the reference
    /// that would have supplied the frame rate.
    pub fn names(self, fourcc: [u8; 4]) -> bool {
        let spelling = fourcc.map(|byte| byte.to_ascii_uppercase());
        let known: &[&[u8; 4]] = match self {
            Self::Hevc => &[b"H265", b"HEVC", b"HVC1", b"HEV1"],
            Self::H264 => &[b"H264", b"AVC1", b"X264", b"DAVC"],
            Self::Mpeg4 => &[b"XVID", b"DIVX", b"DX50", b"FMP4", b"MP4V"],
            Self::Jpeg => &[b"MJPG", b"JPEG", b"MJPA"],
        };
        known.contains(&&spelling)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Chunk {
    pub id: [u8; 4],
    /// Offset of the four-character code.
    pub start: usize,
    /// Payload, excluding the eight-byte chunk header.
    pub payload: Range<usize>,
    /// Whether the frame can be decoded without reference to an earlier one.
    pub keyframe: bool,
}

impl Chunk {
    pub fn is_video(&self) -> bool {
        matches!(self.id.get(2..4), Some(b"dc" | b"db"))
    }
}

#[derive(Debug, Clone)]
pub struct ScanReport {
    pub chunks: Vec<Chunk>,
    /// What the payloads turned out to hold, absent when no chunk opened with something
    /// recognizable.
    pub codec: Option<Codec>,
    /// Offset the first chunk was found at.
    pub start: usize,
    /// Offset the walk stopped at, which is the end of the file when it read everything.
    pub stopped_at: usize,
    /// True when the chain ran out at the end of the file or at the index behind it, rather
    /// than at damage.
    pub ends_cleanly: bool,
    /// How many times the walk had to search forward for the chain again.
    pub resynchronizations: u32,
}

impl ScanReport {
    pub fn video_chunks(&self) -> usize {
        self.chunks.iter().filter(|chunk| chunk.is_video()).count()
    }

    pub fn keyframes(&self) -> usize {
        self.chunks.iter().filter(|chunk| chunk.keyframe).count()
    }

    /// True when the chain accounted for all the media from its start to the end of the file.
    ///
    /// This is what separates "the header was lost and everything else is here" from "the media
    /// data is damaged too", and it decides whether the rebuild may call itself complete.
    pub fn covers_everything(&self) -> bool {
        self.resynchronizations == 0 && self.ends_cleanly
    }
}

/// The payload length of a stream chunk at `offset`, when one is really there.
///
/// A zero length is a real chunk, not a sign of damage: a writer holding a constant frame rate
/// emits an empty one for a frame that repeats the last, and a third of the video chunks in an
/// ffmpeg-written sample are exactly that. Refusing them broke the chain a thousand times over.
fn chunk_len_at(data: &[u8], offset: usize) -> Option<usize> {
    let id = data.get(offset..offset + 4)?;
    if !id.first().is_some_and(u8::is_ascii_digit) || !id.get(1).is_some_and(u8::is_ascii_digit) {
        return None;
    }
    if !STREAM_KINDS.iter().any(|kind| id.get(2..4) == Some(kind.as_slice())) {
        return None;
    }
    let len = read_u32_le(data, offset + 4)? as usize;
    (offset as u64 + 8 + len as u64 <= data.len() as u64).then_some(len)
}

/// Where whatever follows a chunk ending at `end` begins.
///
/// RIFF pads an odd-length chunk to an even offset and this DVR firmware does not, so both have
/// to be tolerated. There is no ambiguity in practice: a writer that pads puts a zero byte
/// there, and zero is not an ASCII digit, so a chunk is found at an odd `end` only in the files
/// that leave the pad byte out.
///
/// Deliberately not conditional on another *chunk* following: the last chunk of a recording is
/// followed by the index, and insisting on a chunk left the walk stranded on the pad byte in
/// front of it, one step short of a clean finish.
fn next_after(data: &[u8], end: usize) -> usize {
    if end % 2 == 1 && chunk_len_at(data, end).is_none() {
        return end + 1;
    }
    end
}

/// Whether `offset` opens a chain of chunks long enough to be believed.
///
/// Groupings and fillers are stepped over here exactly as the walk steps over them, because a
/// position the walk would read has to be a position this accepts - a `JUNK` chunk four frames
/// in must not make the first four frames of a recording look like a coincidence.
fn starts_a_chain(data: &[u8], offset: usize) -> bool {
    let mut at = offset;
    let mut found = 0;

    while found < MIN_CHAIN {
        if let Some(len) = chunk_len_at(data, at) {
            at = next_after(data, at + 8 + len);
            found += 1;
        } else if let Some(step) = container_at(data, at) {
            at += step;
        } else {
            // A chain that runs off the end of the file is the last one in it, not a false match.
            return found > 0 && at >= data.len();
        }
    }
    true
}

/// Finds the next position holding stream data, from `from` onwards.
///
/// The prefilter is what makes searching a whole recording affordable: only a position whose
/// first two bytes are digits and whose next two spell a known chunk kind pays for a chain
/// check, and that is a few positions in a million rather than all of them.
fn find_chain(data: &[u8], from: usize) -> Option<usize> {
    (from..data.len().saturating_sub(8)).find(|&offset| chunk_len_at(data, offset).is_some() && starts_a_chain(data, offset))
}

/// A `LIST`/`rec ` grouping or a `JUNK` filler between chunks, and how far past it to continue.
fn container_at(data: &[u8], offset: usize) -> Option<usize> {
    let id = data.get(offset..offset + 4)?;
    let len = read_u32_le(data, offset + 4)? as usize;
    match id {
        b"LIST" => Some(12),
        b"JUNK" => Some(8 + len + len % 2),
        _ => None,
    }
}

/// Whether the chunk here is the index that follows the media rather than more of it.
///
/// Stopping at `idx1` is not only tidiness. Each of its entries begins with the code of the
/// chunk it describes, so an index read as media is a run of thousands of things that look
/// exactly like the start of a chunk - the one place in an AVI where the chain-of-four test
/// could plausibly be fooled.
fn is_index(data: &[u8], offset: usize) -> bool {
    data.get(offset..offset + 4) == Some(b"idx1")
}

/// Bytes that may trail the last chunk without meaning anything was lost.
///
/// Measured on two files from the same camera: the firmware writes a zero-length `00dc` and
/// eight zero bytes past the end of the media and never mentions them in any size field.
const TRAILER_SLACK: usize = 64;

/// What the payload opens with, when that is enough to name the codec.
///
/// Only a parameter set or a keyframe answers: those carry a fixed, recognizable header, while
/// an inter-coded slice looks much the same in H.264 and HEVC. Scanning forward until a chunk
/// does answer costs nothing, because the first chunk of a recording is always a keyframe.
pub fn codec_of(payload: &[u8]) -> Option<Codec> {
    if payload.starts_with(&[0xFF, 0xD8]) {
        return Some(Codec::Jpeg);
    }
    let unit = annex_b_unit(payload)?;
    let first = *unit.first()?;
    let second = unit.get(1).copied().unwrap_or(0);

    // HEVC's header is two bytes, the second carrying a temporal id that is never zero.
    if first & 0x81 == 0 && second & 0x07 != 0 {
        let kind = (first >> 1) & 0x3F;
        if (32..=34).contains(&kind) || (16..=21).contains(&kind) {
            return Some(Codec::Hevc);
        }
    }
    if first & 0x80 == 0 && matches!(first & 0x1F, 5 | 7 | 8) {
        return Some(Codec::H264);
    }
    matches!(first, 0xB0 | 0xB6).then_some(Codec::Mpeg4)
}

/// The bytes after the first Annex-B start code near the beginning of the payload.
fn annex_b_unit(payload: &[u8]) -> Option<&[u8]> {
    let window = payload.get(..HEADER_WINDOW.min(payload.len()))?;
    let start = window.windows(3).position(|bytes| bytes == [0, 0, 1])? + 3;
    payload.get(start..)
}

/// Whether the chunk holds a frame that can be decoded on its own.
fn opens_a_keyframe(payload: &[u8], codec: Codec) -> bool {
    if codec == Codec::Jpeg {
        return true;
    }
    let Some(unit) = annex_b_unit(payload) else {
        return false;
    };
    let Some(&first) = unit.first() else {
        return false;
    };
    match codec {
        // A parameter set counts: cameras emit them immediately before the keyframe they
        // describe, so the chunk that carries one opens a decodable frame.
        Codec::Hevc => {
            let kind = (first >> 1) & 0x3F;
            (32..=34).contains(&kind) || (16..=21).contains(&kind)
        }
        Codec::H264 => matches!(first & 0x1F, 5 | 7 | 8),
        Codec::Mpeg4 => first == 0xB0 || (first == 0xB6 && unit.get(1).is_some_and(|byte| byte & 0xC0 == 0)),
        Codec::Jpeg => true,
    }
}

/// Walks the chunk chain from wherever in `data` it begins.
pub fn scan(data: &[u8]) -> ScanReport {
    let Some(start) = find_chain(data, 0) else {
        return ScanReport {
            chunks: Vec::new(),
            codec: None,
            start: 0,
            stopped_at: 0,
            ends_cleanly: false,
            resynchronizations: 0,
        };
    };

    let mut chunks: Vec<Chunk> = Vec::new();
    let mut codec = None;
    let mut offset = start;
    let mut resynchronizations = 0u32;
    let mut ends_cleanly = true;

    while offset < data.len() {
        if let Some(len) = chunk_len_at(data, offset)
            && let Some(id) = data.get(offset..offset + 4).and_then(|id| id.try_into().ok())
        {
            let payload = offset + 8..offset + 8 + len;
            if codec.is_none() {
                codec = data.get(payload.clone()).and_then(codec_of);
            }
            chunks.push(Chunk {
                id,
                start: offset,
                payload,
                keyframe: false,
            });
            offset = next_after(data, offset + 8 + len);
            continue;
        }
        if is_index(data, offset) {
            break;
        }
        if let Some(step) = container_at(data, offset) {
            offset += step;
            continue;
        }
        let Some(next) = find_chain(data, offset + 1) else {
            ends_cleanly = data.len() - offset <= TRAILER_SLACK;
            break;
        };
        offset = next;
        resynchronizations += 1;
    }

    // Keyframes could only be flagged once the codec was known, and it is known from the first
    // chunk that carries a parameter set - which is usually, but not always, the first chunk.
    if let Some(codec) = codec {
        for chunk in &mut chunks {
            chunk.keyframe = data.get(chunk.payload.clone()).is_some_and(|payload| opens_a_keyframe(payload, codec));
        }
    }

    ScanReport {
        chunks,
        codec,
        start,
        stopped_at: offset,
        ends_cleanly,
        resynchronizations,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A chunk as this DVR firmware writes them: no pad byte after an odd length.
    fn chunk(id: [u8; 4], payload: &[u8]) -> Vec<u8> {
        let mut out = id.to_vec();
        out.extend_from_slice(&(payload.len() as u32).to_le_bytes());
        out.extend_from_slice(payload);
        out
    }

    fn hevc_keyframe() -> Vec<u8> {
        let mut payload = vec![0, 0, 0, 1, 0x40, 0x01, 0x0C, 0x01];
        payload.extend_from_slice(&[0xAA; 40]);
        payload
    }

    fn hevc_inter() -> Vec<u8> {
        let mut payload = vec![0, 0, 1, 0x02, 0x01, 0xD0];
        payload.extend_from_slice(&[0x5C; 30]);
        payload
    }

    fn recording(frames: usize) -> Vec<u8> {
        let mut data = vec![0u8; 2048];
        data.extend_from_slice(&chunk(*b"00dc", &hevc_keyframe()));
        for _ in 1..frames {
            data.extend_from_slice(&chunk(*b"00dc", &hevc_inter()));
        }
        data
    }

    #[test]
    fn a_chain_is_found_behind_a_destroyed_header() {
        let data = recording(10);
        let report = scan(&data);

        assert_eq!(report.start, 2048);
        assert_eq!(report.chunks.len(), 10);
        assert_eq!(report.codec, Some(Codec::Hevc));
        assert_eq!(report.keyframes(), 1);
        assert_eq!(report.video_chunks(), 10);
        assert_eq!(report.stopped_at, data.len());
        assert!(report.covers_everything());
    }

    /// The firmware these files come from does not pad an odd-length chunk, and a reader that
    /// insists on the pad byte loses the whole recording after the first such chunk.
    #[test]
    fn an_unpadded_odd_length_chunk_does_not_end_the_chain() {
        let mut data = vec![0u8; 16];
        data.extend_from_slice(&chunk(*b"00dc", &hevc_keyframe()));
        for _ in 0..5 {
            data.extend_from_slice(&chunk(*b"00dc", &[0x33; 41]));
        }
        assert_eq!(scan(&data).chunks.len(), 6);
    }

    #[test]
    fn a_padded_odd_length_chunk_is_read_the_same_way() {
        let mut data = vec![0u8; 16];
        data.extend_from_slice(&chunk(*b"00dc", &hevc_keyframe()));
        for _ in 0..5 {
            data.extend_from_slice(&chunk(*b"00dc", &[0x33; 41]));
            data.push(0);
        }
        assert_eq!(scan(&data).chunks.len(), 6);
    }

    #[test]
    fn junk_and_rec_groupings_between_chunks_are_stepped_over() {
        let mut data = vec![0u8; 16];
        data.extend_from_slice(b"LIST");
        data.extend_from_slice(&0u32.to_le_bytes());
        data.extend_from_slice(b"rec ");
        data.extend_from_slice(&chunk(*b"00dc", &hevc_keyframe()));
        data.extend_from_slice(&chunk(*b"JUNK", &[0u8; 24]));
        for _ in 0..4 {
            data.extend_from_slice(&chunk(*b"00dc", &hevc_inter()));
        }
        let report = scan(&data);

        assert_eq!(report.chunks.len(), 5);
        assert_eq!(report.resynchronizations, 0);
    }

    /// A writer holding a constant frame rate emits an empty chunk for a frame that repeats the
    /// last one. Refusing those breaks the chain wherever the picture stops moving.
    #[test]
    fn empty_chunks_are_frames_rather_than_damage() {
        let mut data = vec![0u8; 16];
        data.extend_from_slice(&chunk(*b"00dc", &hevc_keyframe()));
        for index in 0..8 {
            let payload = if index % 2 == 0 { hevc_inter() } else { Vec::new() };
            data.extend_from_slice(&chunk(*b"00dc", &payload));
        }
        let report = scan(&data);

        assert_eq!(report.chunks.len(), 9);
        assert_eq!(report.resynchronizations, 0);
        assert!(report.covers_everything());
    }

    /// The index follows the last chunk, and after an odd-length one there is a pad byte in
    /// between. Stopping on that byte left a recording one step short of a clean finish and
    /// reported a complete recovery as partial.
    #[test]
    fn the_index_behind_the_last_chunk_ends_the_walk_cleanly() {
        let mut data = vec![0u8; 16];
        data.extend_from_slice(&chunk(*b"00dc", &hevc_keyframe()));
        for _ in 0..4 {
            data.extend_from_slice(&chunk(*b"00dc", &[0x33; 41]));
            data.push(0);
        }
        data.extend_from_slice(b"idx1");
        data.extend_from_slice(&(5u32 * 16).to_le_bytes());
        for _ in 0..5 {
            data.extend_from_slice(b"00dc");
            data.extend_from_slice(&[0u8; 12]);
        }
        let report = scan(&data);

        assert_eq!(report.chunks.len(), 5, "the index is not more frames");
        assert_eq!(report.resynchronizations, 0);
        assert!(report.ends_cleanly);
        assert!(report.covers_everything());
    }

    #[test]
    fn a_hole_in_the_middle_is_resynchronized_past() {
        let mut data = recording(6);
        let before = data.len();
        data.extend(std::iter::repeat_n(0x7Fu8, 100_000));
        data.extend_from_slice(&chunk(*b"00dc", &hevc_keyframe()));
        for _ in 0..5 {
            data.extend_from_slice(&chunk(*b"00dc", &hevc_inter()));
        }
        let report = scan(&data);

        assert_eq!(report.chunks.len(), 12);
        assert_eq!(report.resynchronizations, 1);
        assert!(!report.covers_everything(), "a hole is not full coverage");
        assert!(report.chunks.iter().any(|chunk| chunk.start > before));
    }

    #[test]
    fn a_single_plausible_code_is_not_mistaken_for_a_stream() {
        let mut data = vec![0x5Au8; 4096];
        data.get_mut(1000..1004).expect("the buffer is long enough").copy_from_slice(b"00dc");
        data.get_mut(1004..1008).expect("the buffer is long enough").copy_from_slice(&64u32.to_le_bytes());

        assert!(scan(&data).chunks.is_empty(), "one match is a coincidence");
    }

    #[test]
    fn the_codec_is_read_from_the_unit_a_chunk_opens_with() {
        assert_eq!(codec_of(&hevc_keyframe()), Some(Codec::Hevc));
        assert_eq!(codec_of(&[0, 0, 0, 1, 0x67, 0x42, 0x00, 0x1E]), Some(Codec::H264));
        assert_eq!(codec_of(&[0, 0, 1, 0xB6, 0x10, 0x20]), Some(Codec::Mpeg4));
        assert_eq!(codec_of(&[0xFF, 0xD8, 0xFF, 0xE0]), Some(Codec::Jpeg));
        assert_eq!(codec_of(&[0x11; 32]), None);
        assert_eq!(codec_of(&[]), None);
    }

    #[test]
    fn keyframes_are_told_apart_from_inter_coded_frames() {
        assert!(opens_a_keyframe(&hevc_keyframe(), Codec::Hevc));
        assert!(!opens_a_keyframe(&hevc_inter(), Codec::Hevc));
        assert!(opens_a_keyframe(&[0, 0, 0, 1, 0x65, 0x88], Codec::H264));
        assert!(!opens_a_keyframe(&[0, 0, 0, 1, 0x41, 0x9A], Codec::H264));
        assert!(opens_a_keyframe(&[0, 0, 1, 0xB6, 0x10], Codec::Mpeg4));
        assert!(!opens_a_keyframe(&[0, 0, 1, 0xB6, 0x50], Codec::Mpeg4));
    }

    #[test]
    fn a_codec_answers_to_every_spelling_a_writer_might_use() {
        assert!(Codec::Hevc.names(*b"H265"));
        assert!(Codec::Hevc.names(*b"hvc1"));
        assert!(!Codec::Hevc.names(*b"H264"));
        assert!(Codec::H264.names(*b"avc1"));
        assert!(Codec::Mpeg4.names(*b"DX50"));
        assert!(!Codec::Jpeg.names(*b"    "));
    }

    #[test]
    fn scanning_hostile_bytes_terminates_without_panicking() {
        for pattern in [0x00u8, 0xFF, 0x30] {
            let report = scan(&vec![pattern; 8192]);
            assert!(report.stopped_at <= 8192);
        }
        assert!(scan(&[]).chunks.is_empty());
        assert!(scan(b"00dc").chunks.is_empty());
    }
}
