//! Walking the EBML tree a Matroska file is built from.
//!
//! Every element is an identifier, a length, and a payload, with both numbers written as
//! variable-length integers whose first byte says how long they are. Nesting is expressed
//! purely by lengths, which makes the format easy to walk and unforgiving of damage: an
//! element whose length runs past the end of the file swallows everything after it, and a
//! player that trusts the length simply stops.
//!
//! The length may also be written as "unknown", all value bits set, which a live recorder uses
//! because it does not know how long the recording will be when it writes the header. A
//! recording interrupted before that number was filled in is exactly the case this walk exists
//! to repair.

use std::ops::Range;

pub const EBML_HEADER: u32 = 0x1A45_DFA3;
pub const SEGMENT: u32 = 0x1853_8067;
pub const CLUSTER: u32 = 0x1F43_B675;
pub const TRACKS: u32 = 0x1654_AE6B;
pub const INFO: u32 = 0x1549_A966;
pub const CUES: u32 = 0x1C53_BB6B;
pub const SEEK_HEAD: u32 = 0x114D_9B74;

/// Identifiers are at most four bytes, lengths at most eight.
const MAX_ID_LEN: usize = 4;
const MAX_SIZE_LEN: usize = 8;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Element {
    pub id: u32,
    /// Offset of the identifier.
    pub start: usize,
    /// Bytes of identifier plus length, so the payload begins at `start + header_len`.
    pub header_len: usize,
    /// Width of the length field, which a rewrite has to keep to avoid moving anything.
    pub size_len: usize,
    /// `None` when the length is the "unknown" pattern.
    pub declared_size: Option<u64>,
    /// Payload bounds, clamped to what the file holds.
    pub payload: Range<usize>,
    /// False when the file ends inside the payload, or the length was never written.
    pub complete: bool,
}

impl Element {
    pub fn end(&self) -> usize {
        self.payload.end
    }

    pub fn is(&self, id: u32) -> bool {
        self.id == id
    }
}

/// Reads an element identifier, keeping its marker bits the way Matroska writes them.
pub fn read_id(data: &[u8], at: usize) -> Option<(u32, usize)> {
    let first = *data.get(at)?;
    let len = match first.leading_zeros() {
        0 => 1,
        1 => 2,
        2 => 3,
        3 => 4,
        _ => return None,
    };

    let mut value = 0u32;
    for index in 0..len {
        value = (value << 8) | u32::from(*data.get(at + index)?);
    }
    Some((value, len))
}

/// Reads a length, returning it, its width, and whether it is the "unknown" pattern.
pub fn read_size(data: &[u8], at: usize) -> Option<(Option<u64>, usize)> {
    let first = *data.get(at)?;
    if first == 0 {
        return None;
    }
    let len = first.leading_zeros() as usize + 1;
    if len > MAX_SIZE_LEN {
        return None;
    }

    // Masked in wider arithmetic: at the maximum width the marker takes the whole first byte,
    // and shifting a `u8` by eight is an overflow rather than a zero.
    let mut value = u64::from(first) & (0xFFu64 >> len);
    for index in 1..len {
        value = (value << 8) | u64::from(*data.get(at + index)?);
    }

    // All value bits set means the writer did not know the length.
    let unknown_pattern = (1u64 << (7 * len)) - 1;
    Some((if value == unknown_pattern { None } else { Some(value) }, len))
}

/// Reads the element at `at`. `None` when nothing there can be one.
pub fn read_element(data: &[u8], at: usize) -> Option<Element> {
    let (id, id_len) = read_id(data, at)?;
    if id_len > MAX_ID_LEN {
        return None;
    }
    let (declared_size, size_len) = read_size(data, at + id_len)?;
    let header_len = id_len + size_len;
    let payload_start = at + header_len;

    let (payload_end, complete) = match declared_size {
        Some(size) => {
            let end = payload_start as u64 + size;
            (usize::try_from(end).unwrap_or(data.len()).min(data.len()), end <= data.len() as u64)
        }
        // An unknown length runs to the end of whatever contains it, which for a walk means
        // the end of the file - and it is never "complete", because nothing said where it ends.
        None => (data.len(), false),
    };

    Some(Element {
        id,
        start: at,
        header_len,
        size_len,
        declared_size,
        payload: payload_start.min(data.len())..payload_end,
        complete,
    })
}

/// Walks the elements directly inside `range`, stopping at the first one that cannot be read.
pub fn walk(data: &[u8], range: Range<usize>) -> Vec<Element> {
    let mut elements = Vec::new();
    let mut offset = range.start;

    while offset < range.end.min(data.len()) {
        let Some(element) = read_element(data, offset) else {
            break;
        };
        let next = element.end().max(offset + element.header_len);
        let complete = element.complete;
        elements.push(element);
        if !complete || next <= offset {
            break;
        }
        offset = next;
    }
    elements
}

/// Overwrites a length field in place, keeping its width so nothing after it moves.
///
/// Returns false when the value will not fit in that width, which is only possible if the
/// caller is trying to make a length *larger* - shrinking always fits.
pub fn write_size(data: &mut [u8], at: usize, size_len: usize, value: u64) -> bool {
    if size_len == 0 || size_len > MAX_SIZE_LEN {
        return false;
    }
    let capacity = (1u64 << (7 * size_len)) - 1;
    if value >= capacity {
        return false;
    }

    let Some(field) = data.get_mut(at..at + size_len) else {
        return false;
    };
    for (index, byte) in field.iter_mut().enumerate() {
        let shift = 8 * (size_len - 1 - index);
        *byte = ((value >> shift) & 0xFF) as u8;
    }
    if let Some(first) = field.first_mut() {
        *first |= 0x80 >> (size_len - 1);
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Builds an element: identifier, an eight-byte length, and the payload.
    #[cfg(test)]
    pub fn element(id: u32, payload: &[u8]) -> Vec<u8> {
        let mut out = Vec::new();
        let id_bytes = id.to_be_bytes();
        let skip = id_bytes.iter().take_while(|byte| **byte == 0).count();
        out.extend_from_slice(id_bytes.get(skip..).unwrap_or_default());

        let mut size = [0u8; 8];
        let value = payload.len() as u64;
        for (index, byte) in size.iter_mut().enumerate() {
            *byte = ((value >> (8 * (7 - index))) & 0xFF) as u8;
        }
        if let Some(first) = size.first_mut() {
            *first |= 0x01;
        }
        out.extend_from_slice(&size);
        out.extend_from_slice(payload);
        out
    }

    #[test]
    fn identifiers_keep_their_marker_bits() {
        assert_eq!(read_id(&[0x1A, 0x45, 0xDF, 0xA3], 0), Some((EBML_HEADER, 4)));
        assert_eq!(read_id(&[0x18, 0x53, 0x80, 0x67], 0), Some((SEGMENT, 4)));
        assert_eq!(read_id(&[0xA3], 0), Some((0xA3, 1)), "a one-byte identifier");
        assert_eq!(read_id(&[0x00], 0), None);
        assert_eq!(read_id(&[], 0), None);
    }

    #[test]
    fn lengths_decode_at_every_width() {
        assert_eq!(read_size(&[0x81], 0), Some((Some(1), 1)));
        assert_eq!(read_size(&[0xFE], 0), Some((Some(126), 1)));
        assert_eq!(read_size(&[0x40, 0x7F], 0), Some((Some(127), 2)));
        assert_eq!(read_size(&[0x01, 0, 0, 0, 0, 0, 0x01, 0x00], 0), Some((Some(256), 8)));
        assert_eq!(read_size(&[0x00], 0), None);
    }

    /// A recorder that does not know how long the recording will be writes all ones.
    #[test]
    fn an_unknown_length_is_recognized() {
        assert_eq!(read_size(&[0xFF], 0), Some((None, 1)));
        assert_eq!(read_size(&[0x01, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF], 0), Some((None, 8)));
    }

    #[test]
    fn a_length_can_be_rewritten_in_place() {
        let mut data = vec![0x01, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF];
        assert!(write_size(&mut data, 0, 8, 4096));
        assert_eq!(read_size(&data, 0), Some((Some(4096), 8)));

        let mut narrow = vec![0x81];
        assert!(write_size(&mut narrow, 0, 1, 7));
        assert_eq!(read_size(&narrow, 0), Some((Some(7), 1)));
        assert!(!write_size(&mut narrow, 0, 1, 1000), "too large for one byte");
    }

    #[test]
    fn a_tree_walks_back_to_the_elements_it_was_built_from() {
        let mut segment = Vec::new();
        segment.extend_from_slice(&element(TRACKS, &[1, 2, 3]));
        segment.extend_from_slice(&element(CLUSTER, &[4u8; 100]));
        segment.extend_from_slice(&element(CLUSTER, &[5u8; 100]));

        let mut file = element(EBML_HEADER, &[0u8; 8]);
        file.extend_from_slice(&element(SEGMENT, &segment));

        let top = walk(&file, 0..file.len());
        assert_eq!(top.len(), 2);
        assert!(top.first().is_some_and(|e| e.is(EBML_HEADER)));

        let segment_element = top.get(1).expect("the segment");
        assert!(segment_element.complete);
        let children = walk(&file, segment_element.payload.clone());
        assert_eq!(children.len(), 3);
        assert_eq!(children.iter().filter(|e| e.is(CLUSTER)).count(), 2);
    }

    #[test]
    fn an_element_the_file_ends_inside_is_reported_incomplete() {
        let mut file = element(EBML_HEADER, &[0u8; 8]);
        file.extend_from_slice(&element(CLUSTER, &[4u8; 200]));
        file.truncate(file.len() - 50);

        let top = walk(&file, 0..file.len());
        assert_eq!(top.len(), 2);
        assert!(top.get(1).is_some_and(|e| !e.complete));
        assert_eq!(top.get(1).map(|e| e.payload.end), Some(file.len()));
    }

    #[test]
    fn short_input_never_panics() {
        for len in 0..24usize {
            let data = vec![0x1Au8; len];
            let _ = read_element(&data, 0);
            let _ = walk(&data, 0..len);
        }
    }
}
