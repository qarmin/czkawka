//! Walking the box tree of an ISO base media file.
//!
//! Every length here comes from the file being repaired, so none of it is trusted: sizes are
//! computed in `u64` and clamped against the real length before any range is formed. A box
//! whose declared size runs past the end of the file is reported as truncated rather than
//! dropped, because for a cut-short recording that box is exactly the interesting one.

use std::ops::Range;

use crate::file_fixer::io::{read_u32_be, read_u64_be};

/// Boxes that contain other boxes rather than payload data.
const CONTAINERS: [[u8; 4]; 11] = [*b"moov", *b"trak", *b"mdia", *b"minf", *b"stbl", *b"edts", *b"dinf", *b"mvex", *b"moof", *b"traf", *b"udta"];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoxStatus {
    Complete,
    /// The declared size runs past the end of the data.
    Truncated,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoxHeader {
    pub kind: [u8; 4],
    /// Offset of the size field, where the box begins.
    pub start: usize,
    /// Payload bounds, clamped to what the file actually contains.
    pub payload: Range<usize>,
    /// End of the box as declared, which may lie past the end of the file.
    pub declared_end: u64,
    pub status: BoxStatus,
}

impl BoxHeader {
    pub fn is(&self, kind: [u8; 4]) -> bool {
        self.kind == kind
    }

    pub fn is_container(&self) -> bool {
        CONTAINERS.contains(&self.kind)
    }

    pub fn kind_str(&self) -> String {
        String::from_utf8_lossy(&self.kind).to_string()
    }
}

/// Lists the boxes directly inside `range`, without descending.
pub fn walk(data: &[u8], range: Range<usize>) -> Vec<BoxHeader> {
    let mut boxes = Vec::new();
    let limit = range.end.min(data.len());
    let mut offset = range.start;

    while offset + 8 <= limit {
        let Some(header) = read_header(data, offset, limit) else {
            break;
        };
        let advance = header.payload.end.max(offset + 8);
        let truncated = header.status == BoxStatus::Truncated;
        boxes.push(header);

        if truncated || advance <= offset {
            break;
        }
        offset = advance;
    }
    boxes
}

fn read_header(data: &[u8], offset: usize, limit: usize) -> Option<BoxHeader> {
    let size32 = read_u32_be(data, offset)?;
    let kind: [u8; 4] = data.get(offset + 4..offset + 8)?.try_into().ok()?;

    let (declared_size, header_len) = match size32 {
        // Zero means "to the end of the enclosing box", used for a final mdat.
        0 => ((limit - offset) as u64, 8usize),
        1 => (read_u64_be(data, offset + 8)?, 16usize),
        other => (u64::from(other), 8usize),
    };

    if declared_size < header_len as u64 {
        return None;
    }
    let declared_end = offset as u64 + declared_size;
    let payload_start = offset + header_len;
    if payload_start > limit {
        return None;
    }

    let status = if declared_end > limit as u64 { BoxStatus::Truncated } else { BoxStatus::Complete };
    let payload_end = declared_end.min(limit as u64) as usize;

    Some(BoxHeader {
        kind,
        start: offset,
        payload: payload_start..payload_end.max(payload_start),
        declared_end,
        status,
    })
}

pub fn top_level(data: &[u8]) -> Vec<BoxHeader> {
    walk(data, 0..data.len())
}

/// Follows a path of box types from the top level, descending through containers.
pub fn find_path(data: &[u8], path: &[[u8; 4]]) -> Option<BoxHeader> {
    let mut range = 0..data.len();
    let mut found = None;

    for &kind in path {
        let header = walk(data, range).into_iter().find(|b| b.is(kind))?;
        range = header.payload.clone();
        found = Some(header);
    }
    found
}

/// Every box of a given type at the top level, in file order.
pub fn find_all(data: &[u8], kind: [u8; 4]) -> Vec<BoxHeader> {
    top_level(data).into_iter().filter(|b| b.is(kind)).collect()
}

/// Locates the media data, falling back to a signature search when the box chain is broken.
///
/// Walking from offset zero needs every preceding box header to be intact, so damage to the
/// first few bytes hides a perfectly good `mdat`. The four-character code is distinctive enough
/// to find directly, and the size field four bytes before it confirms the match.
pub fn find_media_data(data: &[u8]) -> Option<BoxHeader> {
    if let Some(found) = top_level(data).into_iter().find(|b| b.is(*b"mdat")) {
        return Some(found);
    }

    let position = data
        .windows(4)
        .enumerate()
        .skip(4)
        .find(|(index, window)| *window == b"mdat" && read_header(data, index - 4, data.len()).is_some())
        .map(|(index, _)| index - 4)?;

    read_header(data, position, data.len())
}

/// Every index in the file, including one an oversized `mdat` has swallowed.
///
/// A carving recovery tool writes `ftyp` followed by a single `mdat` claiming everything left,
/// which hides a perfectly good `moov` behind it: [`top_level`] walks straight past into the
/// media data and stops. Nothing is lost in such a file, the index is simply not where a
/// parser looks.
///
/// Four characters are far too weak a signal on their own in megabytes of compressed video, so
/// a hit only counts when the length in front of it fits the file and `mvhd` follows it -
/// `moov` is required by the specification to begin with that box.
pub fn find_indexes(data: &[u8]) -> Vec<BoxHeader> {
    signature_positions(data, *b"moov")
        .filter_map(|position| read_header(data, position.checked_sub(4)?, data.len()))
        .filter(|header| header.status == BoxStatus::Complete && header.payload.len() >= 8)
        .filter(|header| data.get(header.payload.start + 4..header.payload.start + 8) == Some(b"mvhd"))
        .collect()
}

fn signature_positions(data: &[u8], needle: [u8; 4]) -> impl Iterator<Item = usize> + '_ {
    data.windows(4).enumerate().filter(move |(_, window)| *window == needle).map(|(index, _)| index)
}

/// The top-level box that contains `offset`, or `None` when a box starts there already.
///
/// The header to shorten when freeing a buried index is not at a fixed address: `ftyp` is 24
/// bytes in some files and 32 in others, and writing over a guessed offset silently corrupts a
/// healthy file rather than repairing a broken one.
pub fn enclosing(data: &[u8], offset: usize) -> Option<BoxHeader> {
    // `payload.end` is already clamped to the file, and a buried index is inside the file by
    // definition, so the declared end never needs consulting here.
    top_level(data).into_iter().find(|header| header.start < offset && offset < header.payload.end)
}

/// What the top level of the file looks like, which decides which repair applies.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Structure {
    pub has_ftyp: bool,
    pub has_moov: bool,
    /// A fragmented file carries its sample tables in `moof` boxes instead.
    pub has_moof: bool,
    /// True when the media data box was cut short.
    pub mdat_truncated: bool,
    /// True when any top-level box declares a size the file cannot back.
    pub any_truncated: bool,
}

pub fn structure(data: &[u8]) -> Structure {
    let boxes = top_level(data);
    Structure {
        has_ftyp: boxes.iter().any(|b| b.is(*b"ftyp")),
        has_moov: boxes.iter().any(|b| b.is(*b"moov")),
        has_moof: boxes.iter().any(|b| b.is(*b"moof")),
        mdat_truncated: boxes.iter().any(|b| b.is(*b"mdat") && b.status == BoxStatus::Truncated),
        any_truncated: boxes.iter().any(|b| b.status == BoxStatus::Truncated),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Vec<u8> {
        std::fs::read("tmp/a.mp4").unwrap_or_default()
    }

    fn synthetic() -> Vec<u8> {
        let mut data = Vec::new();
        push_box(&mut data, *b"ftyp", b"isomiso2avc1mp41");
        push_box(&mut data, *b"mdat", &[0u8; 64]);
        push_box(&mut data, *b"moov", &[0u8; 32]);
        data
    }

    fn push_box(out: &mut Vec<u8>, kind: [u8; 4], payload: &[u8]) {
        out.extend_from_slice(&((payload.len() + 8) as u32).to_be_bytes());
        out.extend_from_slice(&kind);
        out.extend_from_slice(payload);
    }

    #[test]
    fn a_synthetic_file_walks_in_order() {
        let data = synthetic();
        let boxes = top_level(&data);

        assert_eq!(boxes.len(), 3);
        assert_eq!(boxes[0].kind_str(), "ftyp");
        assert_eq!(boxes[1].kind_str(), "mdat");
        assert_eq!(boxes[2].kind_str(), "moov");
        assert!(boxes.iter().all(|b| b.status == BoxStatus::Complete));
    }

    #[test]
    fn structure_reports_what_the_top_level_holds() {
        let structure = structure(&synthetic());
        assert!(structure.has_ftyp);
        assert!(structure.has_moov);
        assert!(!structure.has_moof);
        assert!(!structure.any_truncated);
    }

    #[test]
    fn a_size_running_past_the_end_is_reported_as_truncated() {
        let mut data = synthetic();
        data.truncate(data.len() - 20);

        let boxes = top_level(&data);
        let last = boxes.last().expect("at least one box parses");
        assert_eq!(last.status, BoxStatus::Truncated);
        assert!(last.payload.end <= data.len());
        assert!(last.declared_end > data.len() as u64);
        assert!(structure(&data).any_truncated);
    }

    #[test]
    fn a_hostile_size_field_does_not_over_read() {
        let mut data = Vec::new();
        data.extend_from_slice(&u32::MAX.to_be_bytes());
        data.extend_from_slice(b"mdat");
        data.extend_from_slice(&[0u8; 32]);

        let boxes = top_level(&data);
        assert_eq!(boxes.len(), 1);
        assert_eq!(boxes[0].status, BoxStatus::Truncated);
        assert!(boxes[0].payload.end <= data.len());
    }

    #[test]
    fn a_size_smaller_than_its_own_header_stops_the_walk() {
        let mut data = Vec::new();
        data.extend_from_slice(&3u32.to_be_bytes());
        data.extend_from_slice(b"mdat");
        data.extend_from_slice(&[0u8; 32]);

        assert!(top_level(&data).is_empty());
    }

    #[test]
    fn every_truncation_length_is_walked_without_panicking() {
        let data = sample();
        if data.is_empty() {
            return;
        }
        for len in (0..data.len()).step_by(data.len() / 200 + 1) {
            let boxes = top_level(&data[..len]);
            assert!(boxes.iter().all(|b| b.payload.end <= len));
        }
    }

    #[test]
    fn the_real_sample_has_its_index_at_the_end() {
        let data = sample();
        if data.is_empty() {
            return;
        }
        let boxes = top_level(&data);
        let kinds: Vec<String> = boxes.iter().map(BoxHeader::kind_str).collect();

        assert_eq!(kinds, vec!["ftyp", "free", "mdat", "moov"]);
        assert!(structure(&data).has_moov);
        assert!(!structure(&data).any_truncated);
    }

    #[test]
    fn descending_reaches_a_nested_box() {
        let data = sample();
        if data.is_empty() {
            return;
        }
        let stsd = find_path(&data, &[*b"moov", *b"trak", *b"mdia", *b"minf", *b"stbl", *b"stsd"]);
        let stsd = stsd.expect("a video track has a sample description");

        assert!(stsd.payload.end > stsd.payload.start);
        assert_eq!(stsd.status, BoxStatus::Complete);
    }

    #[test]
    fn media_data_is_found_even_when_the_leading_bytes_are_destroyed() {
        let data = sample();
        if data.is_empty() {
            return;
        }
        let expected = find_media_data(&data).expect("the sample has media data");

        let mut damaged = data;
        damaged[..32].fill(0xAB);
        assert!(
            top_level(&damaged).iter().all(|b| !b.is(*b"mdat")),
            "the box chain must be broken for this test to mean anything"
        );

        let found = find_media_data(&damaged).expect("the signature search finds it");
        assert_eq!(found.start, expected.start);
        assert_eq!(found.payload, expected.payload);
    }

    #[test]
    fn a_signature_search_on_data_without_media_finds_nothing() {
        assert!(find_media_data(&[0u8; 256]).is_none());
        assert!(find_media_data(b"mdat").is_none());
    }

    #[test]
    fn truncating_the_index_away_leaves_the_media_data_findable() {
        let data = sample();
        if data.is_empty() {
            return;
        }
        let moov = find_all(&data, *b"moov").pop().expect("the sample has an index");
        let without_index = &data[..moov.start];

        let structure = structure(without_index);
        assert!(structure.has_ftyp);
        assert!(!structure.has_moov);
        assert!(structure.mdat_truncated || !structure.any_truncated);
    }
}
