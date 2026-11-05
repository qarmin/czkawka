//! Walking a RIFF container.
//!
//! WAV and WebP are both flat chains of `id` + little-endian length + payload, padded to an
//! even offset, wrapped in an outer `RIFF` chunk that states the total. That makes damage
//! unusually legible: a chunk either fits in the file or it does not, and the outer size field
//! says whether bytes are missing even when every surviving chunk looks fine.
//!
//! AVI is RIFF too but nests its streams inside `LIST` chunks and is handled by ffmpeg in
//! [`super::avi`], so nothing here tries to descend.

use std::ops::Range;

use crate::file_fixer::io::read_u32_le;

/// Bytes before the first chunk: `RIFF`, the size field, and the form type.
pub const HEADER_LEN: usize = 12;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Chunk {
    pub id: [u8; 4],
    /// Offset of the four-character code.
    pub start: usize,
    /// Payload as the header declares it, clamped to what the file actually holds.
    pub payload: Range<usize>,
    /// What the size field said, before clamping.
    pub declared_len: u32,
    /// False when the file ends inside this chunk.
    pub complete: bool,
}

impl Chunk {
    pub fn is(&self, id: [u8; 4]) -> bool {
        self.id == id
    }

    /// First offset past the chunk, including RIFF's pad byte for an odd length.
    pub fn end(&self) -> usize {
        self.payload.end + usize::from(self.payload.len() % 2 == 1)
    }
}

#[derive(Debug, Clone)]
pub struct Riff {
    pub form: [u8; 4],
    pub declared_len: u32,
    pub chunks: Vec<Chunk>,
    /// First offset past the last complete chunk.
    pub end: usize,
}

impl Riff {
    pub fn chunk(&self, id: [u8; 4]) -> Option<&Chunk> {
        self.chunks.iter().find(|chunk| chunk.is(id))
    }

    /// True when the file ends inside a chunk, so content is missing rather than merely
    /// mis-described.
    pub fn has_incomplete_chunk(&self) -> bool {
        self.chunks.iter().any(|chunk| !chunk.complete)
    }

    /// True when the outer size field promises more bytes than the file holds. RIFF's size
    /// excludes its own eight-byte header, so a whole file satisfies `declared + 8 <= len`.
    ///
    /// Trailing bytes *past* the declared size are not treated as damage: writers pad, and
    /// tools append metadata without updating the field.
    pub fn promises_more_than(&self, len: usize) -> bool {
        u64::from(self.declared_len) + 8 > len as u64
    }
}

/// Reads the chunk chain. `None` when the file does not begin with a RIFF header at all.
pub fn parse(data: &[u8]) -> Option<Riff> {
    if !data.starts_with(b"RIFF") {
        return None;
    }
    let declared_len = read_u32_le(data, 4)?;
    let form: [u8; 4] = data.get(8..12)?.try_into().ok()?;
    let chunks = chunks_in(data, HEADER_LEN..data.len());
    let end = chunks.iter().take_while(|chunk| chunk.complete).last().map_or(HEADER_LEN, Chunk::end);

    Some(Riff { form, declared_len, chunks, end })
}

/// Reads a flat chunk chain inside `range`.
///
/// The payload of a `LIST` is a chain in exactly the same shape as the file's top level, minus
/// the four bytes of form type that open it, which is how a nested container is descended into.
pub fn chunks_in(data: &[u8], range: Range<usize>) -> Vec<Chunk> {
    let limit = range.end.min(data.len());
    let mut chunks = Vec::new();
    let mut offset = range.start;

    while offset + 8 <= limit {
        let Some(id) = data.get(offset..offset + 4).and_then(|id| id.try_into().ok()) else {
            break;
        };
        let Some(chunk_len) = read_u32_le(data, offset + 4) else {
            break;
        };
        let payload_start = offset + 8;
        let declared_end = payload_start as u64 + u64::from(chunk_len);
        let complete = declared_end <= limit as u64;
        let payload_end = usize::try_from(declared_end).unwrap_or(limit).min(limit);

        let chunk = Chunk {
            id,
            start: offset,
            payload: payload_start..payload_end,
            declared_len: chunk_len,
            complete,
        };
        offset = chunk.end();
        chunks.push(chunk);
        if !complete {
            break;
        }
    }
    chunks
}

/// Finds a chunk by its four-character code without trusting the chain that should lead to it.
///
/// Used when the header is destroyed: the chunks after the damage still carry their own codes
/// and lengths, so one that describes a payload fitting inside the file is almost certainly
/// the real thing rather than four bytes of coincidence.
pub fn find_chunk(data: &[u8], id: [u8; 4], from: usize) -> Option<Chunk> {
    let mut offset = from;
    while offset + 8 <= data.len() {
        let found = data.get(offset..)?.windows(4).position(|window| window == id)? + offset;
        let payload_start = found + 8;
        if let Some(chunk_len) = read_u32_le(data, found + 4) {
            let declared_end = payload_start as u64 + u64::from(chunk_len);
            if payload_start <= data.len() {
                let complete = declared_end <= data.len() as u64;
                let payload_end = usize::try_from(declared_end).unwrap_or(data.len()).min(data.len());
                return Some(Chunk {
                    id,
                    start: found,
                    payload: payload_start..payload_end,
                    declared_len: chunk_len,
                    complete,
                });
            }
        }
        offset = found + 1;
    }
    None
}

/// Writes a RIFF header for a payload of `payload_len` bytes.
pub fn header(form: [u8; 4], payload_len: usize) -> Vec<u8> {
    let mut out = Vec::with_capacity(HEADER_LEN + payload_len);
    out.extend_from_slice(b"RIFF");
    out.extend_from_slice(&u32::try_from(payload_len + 4).unwrap_or(u32::MAX).to_le_bytes());
    out.extend_from_slice(&form);
    out
}

/// Appends a chunk, with the pad byte an odd length requires.
pub fn append_chunk(out: &mut Vec<u8>, id: [u8; 4], payload: &[u8]) {
    out.extend_from_slice(&id);
    out.extend_from_slice(&u32::try_from(payload.len()).unwrap_or(u32::MAX).to_le_bytes());
    out.extend_from_slice(payload);
    if payload.len() % 2 == 1 {
        out.push(0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build(form: [u8; 4], chunks: &[([u8; 4], &[u8])]) -> Vec<u8> {
        let mut body = Vec::new();
        for (id, payload) in chunks {
            append_chunk(&mut body, *id, payload);
        }
        let mut out = header(form, body.len());
        out.extend_from_slice(&body);
        out
    }

    #[test]
    fn a_whole_file_walks_to_its_end() {
        let data = build(*b"WAVE", &[(*b"fmt ", &[0u8; 16]), (*b"data", &[1u8; 100])]);
        let riff = parse(&data).expect("parses");

        assert_eq!(riff.form, *b"WAVE");
        assert_eq!(riff.chunks.len(), 2);
        assert!(!riff.has_incomplete_chunk());
        assert!(!riff.promises_more_than(data.len()));
        assert_eq!(riff.end, data.len());
        assert_eq!(riff.chunk(*b"data").map(|c| c.payload.len()), Some(100));
    }

    /// An odd-length chunk is followed by a pad byte that is not part of its payload.
    #[test]
    fn an_odd_length_chunk_is_padded() {
        let data = build(*b"WAVE", &[(*b"fmt ", &[0u8; 7]), (*b"data", &[1u8; 4])]);
        let riff = parse(&data).expect("parses");

        assert_eq!(riff.chunks.len(), 2);
        assert_eq!(riff.chunk(*b"fmt ").map(|c| c.payload.len()), Some(7));
        assert_eq!(riff.chunk(*b"data").map(|c| c.payload.len()), Some(4));
    }

    #[test]
    fn a_file_cut_inside_a_chunk_reports_it_incomplete() {
        let mut data = build(*b"WAVE", &[(*b"fmt ", &[0u8; 16]), (*b"data", &[1u8; 100])]);
        data.truncate(data.len() - 40);
        let riff = parse(&data).expect("parses");

        assert!(riff.has_incomplete_chunk());
        assert!(riff.promises_more_than(data.len()));
        let chunk = riff.chunk(*b"data").expect("the data chunk is still described");
        assert_eq!(chunk.declared_len, 100);
        assert_eq!(chunk.payload.len(), 60, "only what is present");
    }

    #[test]
    fn trailing_bytes_past_the_declared_size_are_not_damage() {
        let mut data = build(*b"WAVE", &[(*b"data", &[1u8; 8])]);
        data.extend_from_slice(&[0u8; 16]);
        let riff = parse(&data).expect("parses");

        assert!(!riff.promises_more_than(data.len()));
        assert!(!riff.has_incomplete_chunk());
    }

    /// A `LIST` is a chain inside a chain, which is the shape AVI keeps its stream headers in.
    #[test]
    fn a_nested_list_is_walked_by_the_same_reader() {
        let mut inner = b"strl".to_vec();
        append_chunk(&mut inner, *b"strh", &[7u8; 56]);
        append_chunk(&mut inner, *b"strf", &[9u8; 40]);
        let data = build(*b"AVI ", &[(*b"LIST", &inner)]);

        let list = parse(&data).expect("parses").chunk(*b"LIST").expect("a list").clone();
        let nested = chunks_in(&data, list.payload.start + 4..list.payload.end);

        assert_eq!(nested.len(), 2);
        assert_eq!(nested.first().map(|chunk| chunk.id), Some(*b"strh"));
        assert_eq!(nested.get(1).map(|chunk| chunk.payload.len()), Some(40));
    }

    #[test]
    fn a_chunk_can_be_found_without_a_walk() {
        let mut data = vec![0xAAu8; 64];
        data.extend_from_slice(b"data");
        data.extend_from_slice(&32u32.to_le_bytes());
        data.extend_from_slice(&[7u8; 32]);

        let chunk = find_chunk(&data, *b"data", 0).expect("found");
        assert_eq!(chunk.start, 64);
        assert_eq!(chunk.payload.len(), 32);
        assert!(chunk.complete);
        assert!(find_chunk(&data, *b"fmt ", 0).is_none());
    }

    #[test]
    fn something_that_is_not_riff_is_refused() {
        assert!(parse(b"").is_none());
        assert!(parse(b"RIFF").is_none());
        assert!(parse(&vec![0x42u8; 256]).is_none());
    }
}
