//! Chunk-level validation of a PNG, used to decide which bytes may be trusted.
//!
//! Every PNG chunk carries a CRC32, which distinguishes the two damage classes that matter
//! here. A chunk whose data runs past the end of the file was *truncated*: the bytes that
//! survive are the original ones. A chunk that is complete but fails its CRC was *altered*:
//! its bytes are no longer the original ones, and anything the decoder derives from them is
//! invention rather than recovery.
//!
//! Without this distinction a partial decode happily emits rows produced from corrupted
//! input, because inflate does not notice damage until much later in the stream.

use std::ops::Range;

pub const SIGNATURE: [u8; 8] = [0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChunkStatus {
    /// Complete and the checksum matches.
    Valid,
    /// Complete but the checksum does not match: the content was altered.
    Altered,
    /// The declared length runs past the end of the file, so the chunk was cut short. The
    /// bytes that are present are still the original ones.
    Truncated,
}

#[derive(Debug, Clone)]
pub struct Chunk {
    pub kind: [u8; 4],
    /// Offset of the chunk's length field, where the chunk begins.
    pub start: usize,
    pub data: Range<usize>,
    pub status: ChunkStatus,
}

impl Chunk {
    /// PNG marks critical chunks with an uppercase first letter.
    pub fn is_critical(&self) -> bool {
        self.kind.first().is_some_and(u8::is_ascii_uppercase)
    }

    /// Whether altering this chunk could change a pixel. Ancillary chunks cannot, and neither
    /// can `IEND`, which is a pure end marker carrying no payload.
    pub fn affects_content(&self) -> bool {
        self.is_critical() && !self.is(*b"IEND")
    }

    pub fn is(&self, kind: [u8; 4]) -> bool {
        self.kind == kind
    }
}

/// Walks the chunk chain, stopping at the first structure that cannot be parsed at all.
pub fn walk(data: &[u8]) -> Vec<Chunk> {
    let mut chunks = Vec::new();
    let mut offset = SIGNATURE.len();

    while offset + 8 <= data.len() {
        let Some(length) = crate::file_fixer::io::read_u32_be(data, offset) else {
            break;
        };
        let Some(kind) = data.get(offset + 4..offset + 8).and_then(|k| <[u8; 4]>::try_from(k).ok()) else {
            break;
        };

        let data_start = offset + 8;
        // The length field is attacker-controlled; `end` is computed in u64 so a hostile
        // value wraps into an out-of-range comparison instead of a panic or a huge read.
        let end = data_start as u64 + u64::from(length) + 4;
        if end > data.len() as u64 {
            chunks.push(Chunk {
                kind,
                start: offset,
                data: data_start..data.len(),
                status: ChunkStatus::Truncated,
            });
            break;
        }

        let end = end as usize;
        let data_range = data_start..end - 4;
        let status = if checksum_matches(data, offset, &data_range) {
            ChunkStatus::Valid
        } else {
            ChunkStatus::Altered
        };
        chunks.push(Chunk {
            kind,
            start: offset,
            data: data_range,
            status,
        });

        if kind == *b"IEND" {
            break;
        }
        offset = end;
    }
    chunks
}

fn checksum_matches(data: &[u8], chunk_start: usize, data_range: &Range<usize>) -> bool {
    let Some(covered) = data.get(chunk_start + 4..data_range.end) else {
        return false;
    };
    let Some(stored) = crate::file_fixer::io::read_u32_be(data, data_range.end) else {
        return false;
    };
    crc32fast::hash(covered) == stored
}

/// Length of the leading byte range whose content is still the original content.
///
/// Cuts at the first content-bearing chunk that failed its checksum. A truncated tail is kept:
/// its bytes were never altered, only cut short, and the decoder can still use them.
pub fn verified_prefix_len(data: &[u8]) -> usize {
    if !data.starts_with(&SIGNATURE) {
        return 0;
    }

    for chunk in walk(data) {
        if chunk.status == ChunkStatus::Altered && chunk.affects_content() {
            return chunk.start;
        }
    }
    data.len()
}

/// True when at least one content-bearing chunk was altered, meaning a plain decode would
/// silently produce content that never existed in the original.
pub fn has_altered_content(data: &[u8]) -> bool {
    verified_prefix_len(data) < data.len()
}

/// Rewrites a PNG so its pixel data spans several `IDAT` chunks, which is what real encoders
/// emit. The `png` crate always writes a single chunk, and checksum granularity is per chunk,
/// so a one-chunk file has no partially trustworthy state to exercise.
#[cfg(test)]
pub fn split_idat(data: &[u8], part: usize) -> Vec<u8> {
    let mut out = Vec::from(SIGNATURE);
    for chunk in walk(data) {
        let Some(payload) = data.get(chunk.data.clone()) else {
            continue;
        };
        if chunk.is(*b"IDAT") {
            for piece in payload.chunks(part) {
                append_chunk(&mut out, chunk.kind, piece);
            }
        } else {
            append_chunk(&mut out, chunk.kind, payload);
        }
    }
    out
}

#[cfg(test)]
pub fn append_chunk(out: &mut Vec<u8>, kind: [u8; 4], payload: &[u8]) {
    out.extend_from_slice(&(payload.len() as u32).to_be_bytes());
    out.extend_from_slice(&kind);
    out.extend_from_slice(payload);
    let mut covered = kind.to_vec();
    covered.extend_from_slice(payload);
    out.extend_from_slice(&crc32fast::hash(&covered).to_be_bytes());
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use png::{BitDepth, ColorType, Encoder};

    use super::*;

    fn tiny_png(width: u32, height: u32) -> Vec<u8> {
        let pixels = vec![0xA5u8; (width * height * 3) as usize];
        let mut out = Vec::new();
        let mut encoder = Encoder::new(Cursor::new(&mut out), width, height);
        encoder.set_color(ColorType::Rgb);
        encoder.set_depth(BitDepth::Eight);
        let mut writer = encoder.write_header().unwrap();
        writer.write_image_data(&pixels).unwrap();
        writer.finish().unwrap();
        out
    }

    fn first_chunk_of(data: &[u8], kind: [u8; 4]) -> Chunk {
        walk(data).into_iter().find(|c| c.is(kind)).expect("chunk is present")
    }

    #[test]
    fn a_healthy_file_validates_end_to_end() {
        let data = tiny_png(16, 16);
        let chunks = walk(&data);

        assert!(chunks.iter().all(|c| c.status == ChunkStatus::Valid));
        assert!(chunks.iter().any(|c| c.is(*b"IHDR")));
        assert!(chunks.iter().any(|c| c.is(*b"IEND")));
        assert_eq!(verified_prefix_len(&data), data.len());
        assert!(!has_altered_content(&data));
    }

    #[test]
    fn an_altered_idat_byte_is_caught_and_cuts_the_trusted_prefix() {
        let mut data = tiny_png(16, 16);
        let idat = first_chunk_of(&data, *b"IDAT");
        data[idat.data.start] ^= 0xFF;

        let chunk = first_chunk_of(&data, *b"IDAT");
        assert_eq!(chunk.status, ChunkStatus::Altered);
        assert_eq!(verified_prefix_len(&data), idat.start);
        assert!(has_altered_content(&data));
    }

    /// Truncation removes bytes but never rewrites them, so what remains is still trustworthy.
    #[test]
    fn a_truncated_tail_stays_trusted() {
        let data = tiny_png(16, 64);
        let cut = data.len() * 2 / 3;
        let truncated = &data[..cut];

        let last = walk(truncated).pop().expect("at least one chunk parses");
        assert_eq!(last.status, ChunkStatus::Truncated);
        assert_eq!(verified_prefix_len(truncated), truncated.len());
        assert!(!has_altered_content(truncated));
    }

    #[test]
    fn a_damaged_ancillary_chunk_does_not_invalidate_pixels() {
        let mut data = Vec::new();
        data.extend_from_slice(&SIGNATURE);
        append_chunk(&mut data, *b"IHDR", &[0u8; 13]);
        append_chunk(&mut data, *b"teXt", b"note");
        let text_crc = data.len() - 1;
        data[text_crc] ^= 0xFF;
        append_chunk(&mut data, *b"IDAT", b"payload");

        let text = first_chunk_of(&data, *b"teXt");
        assert_eq!(text.status, ChunkStatus::Altered);
        assert!(!text.is_critical());
        assert_eq!(verified_prefix_len(&data), data.len());
    }

    #[test]
    fn a_hostile_length_field_does_not_panic_or_over_read() {
        let mut data = Vec::new();
        data.extend_from_slice(&SIGNATURE);
        data.extend_from_slice(&u32::MAX.to_be_bytes());
        data.extend_from_slice(b"IDAT");
        data.extend_from_slice(&[0u8; 16]);

        let chunks = walk(&data);
        assert_eq!(chunks.len(), 1);
        assert_eq!(chunks[0].status, ChunkStatus::Truncated);
        assert!(chunks[0].data.end <= data.len());
    }

    /// `IEND` is critical but carries no payload, so a bad checksum on it costs no pixels.
    #[test]
    fn a_damaged_end_marker_does_not_invalidate_pixels() {
        let mut data = tiny_png(16, 16);
        let last = data.len() - 1;
        data[last] ^= 0xFF;

        let iend = first_chunk_of(&data, *b"IEND");
        assert_eq!(iend.status, ChunkStatus::Altered);
        assert!(iend.is_critical());
        assert!(!iend.affects_content());
        assert!(!has_altered_content(&data));
    }

    #[test]
    fn data_without_a_signature_is_trusted_nowhere() {
        assert_eq!(verified_prefix_len(&[0u8; 64]), 0);
        assert_eq!(verified_prefix_len(&[]), 0);
    }

    #[test]
    fn every_truncated_input_length_is_handled() {
        let data = tiny_png(8, 8);
        for len in 0..data.len() {
            let prefix = &data[..len];
            let verified = verified_prefix_len(prefix);
            assert!(verified <= prefix.len());
        }
    }
}
