//! Reading and rewriting a TIFF image file directory.
//!
//! TIFF is a file of offsets: the header points at a directory, the directory's entries point
//! at values and at strips of pixel data, and nothing is required to be in any particular
//! order. That makes it unusually robust to damage in one sense - a strip that survived is
//! still exactly where its entry says it is - and unusually fragile in another, since a single
//! damaged offset can make a reader give up on an image that is entirely present.
//!
//! Repairing therefore means *rewriting* the directory rather than patching it: the entries
//! that still resolve are copied into a fresh file, laid out compactly, with the strip offsets
//! pointing where the strips actually landed. Every byte of pixel data is carried over
//! unchanged; only the bookkeeping around it is new.

use std::ops::Range;

use crate::file_fixer::io::{read_u16_be, read_u16_le, read_u32_be, read_u32_le};

pub const IMAGE_WIDTH: u16 = 256;
pub const IMAGE_LENGTH: u16 = 257;
pub const STRIP_OFFSETS: u16 = 273;
pub const ROWS_PER_STRIP: u16 = 278;
pub const STRIP_BYTE_COUNTS: u16 = 279;
pub const TILE_OFFSETS: u16 = 324;

/// TIFF's own magic number, and the one BigTIFF replaced it with.
pub const CLASSIC_VERSION: u16 = 42;
pub const BIG_TIFF_VERSION: u16 = 43;

pub const HEADER_LEN: usize = 8;
const ENTRY_LEN: usize = 12;

/// More entries than any real directory holds; a larger count is a damaged field.
const MAX_ENTRIES: usize = 512;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Order {
    Little,
    Big,
}

impl Order {
    pub fn u16(self, data: &[u8], at: usize) -> Option<u16> {
        match self {
            Self::Little => read_u16_le(data, at),
            Self::Big => read_u16_be(data, at),
        }
    }

    pub fn u32(self, data: &[u8], at: usize) -> Option<u32> {
        match self {
            Self::Little => read_u32_le(data, at),
            Self::Big => read_u32_be(data, at),
        }
    }

    fn put_u16(self, out: &mut Vec<u8>, value: u16) {
        match self {
            Self::Little => out.extend_from_slice(&value.to_le_bytes()),
            Self::Big => out.extend_from_slice(&value.to_be_bytes()),
        }
    }

    fn put_u32(self, out: &mut Vec<u8>, value: u32) {
        match self {
            Self::Little => out.extend_from_slice(&value.to_le_bytes()),
            Self::Big => out.extend_from_slice(&value.to_be_bytes()),
        }
    }

    fn write_u32(self, out: &mut [u8], at: usize, value: u32) {
        if let Some(slot) = out.get_mut(at..at + 4) {
            match self {
                Self::Little => slot.copy_from_slice(&value.to_le_bytes()),
                Self::Big => slot.copy_from_slice(&value.to_be_bytes()),
            }
        }
    }

    pub fn magic(self) -> [u8; 2] {
        match self {
            Self::Little => *b"II",
            Self::Big => *b"MM",
        }
    }
}

/// Bytes one value of each field type occupies. Unknown types are treated as one byte, which
/// is what readers do and keeps a file with a private type from being declared damaged.
fn type_size(kind: u16) -> u64 {
    match kind {
        3 | 8 => 2,
        4 | 9 | 11 => 4,
        5 | 10 | 12 => 8,
        _ => 1,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Entry {
    pub tag: u16,
    pub kind: u16,
    pub count: u32,
    /// The four bytes the entry itself carries: the value when it fits, an offset otherwise.
    pub value: [u8; 4],
}

impl Entry {
    pub fn byte_len(&self) -> u64 {
        type_size(self.kind) * u64::from(self.count)
    }

    pub fn is_inline(&self) -> bool {
        self.byte_len() <= 4
    }

    /// Where the value lives when it does not fit in the entry.
    pub fn data(&self, order: Order) -> Option<Range<usize>> {
        if self.is_inline() {
            return None;
        }
        let offset = order.u32(&self.value, 0)? as usize;
        Some(offset..offset + usize::try_from(self.byte_len()).ok()?)
    }

    /// The entry's values as integers, following the offset when there is one.
    pub fn numbers(&self, data: &[u8], order: Order) -> Vec<u64> {
        let size = type_size(self.kind) as usize;
        let source: &[u8] = match self.data(order) {
            Some(range) => data.get(range).unwrap_or_default(),
            None => &self.value,
        };

        (0..self.count as usize)
            .filter_map(|index| {
                let at = index * size;
                match size {
                    2 => order.u16(source, at).map(u64::from),
                    4 => order.u32(source, at).map(u64::from),
                    1 => source.get(at).map(|byte| u64::from(*byte)),
                    _ => None,
                }
            })
            .collect()
    }

    /// The single number an entry like `ImageLength` carries.
    pub fn number(&self, data: &[u8], order: Order) -> Option<u64> {
        self.numbers(data, order).first().copied()
    }
}

#[derive(Debug, Clone)]
pub struct Ifd {
    pub start: usize,
    pub entries: Vec<Entry>,
    pub next: u32,
}

impl Ifd {
    pub fn entry(&self, tag: u16) -> Option<&Entry> {
        self.entries.iter().find(|entry| entry.tag == tag)
    }

    pub fn number(&self, data: &[u8], order: Order, tag: u16) -> Option<u64> {
        self.entry(tag)?.number(data, order)
    }

    /// The strips this directory describes, as (offset, byte count) pairs.
    pub fn strips(&self, data: &[u8], order: Order) -> Vec<(u64, u64)> {
        let Some(offsets) = self.entry(STRIP_OFFSETS) else {
            return Vec::new();
        };
        let counts = self.entry(STRIP_BYTE_COUNTS).map(|entry| entry.numbers(data, order)).unwrap_or_default();

        offsets
            .numbers(data, order)
            .into_iter()
            .enumerate()
            .map(|(index, offset)| (offset, counts.get(index).copied().unwrap_or(0)))
            .collect()
    }
}

/// Reads the eight-byte header: byte order, version, and where the first directory is.
pub fn parse_header(data: &[u8]) -> Option<(Order, u16, usize)> {
    let order = match data.get(..2)? {
        b"II" => Order::Little,
        b"MM" => Order::Big,
        _ => return None,
    };
    let version = order.u16(data, 2)?;
    let first = order.u32(data, 4)? as usize;
    Some((order, version, first))
}

/// Reads the directory at `at`, rejecting anything that cannot be one.
pub fn read_ifd(data: &[u8], order: Order, at: usize) -> Option<Ifd> {
    let count = usize::from(order.u16(data, at)?);
    if count == 0 || count > MAX_ENTRIES {
        return None;
    }
    if at + 2 + count * ENTRY_LEN + 4 > data.len() {
        return None;
    }

    let mut entries = Vec::with_capacity(count);
    let mut previous_tag = None;
    for index in 0..count {
        let offset = at + 2 + index * ENTRY_LEN;
        let tag = order.u16(data, offset)?;
        // The specification requires ascending tags, and a directory read out of arbitrary
        // bytes almost never satisfies that - which is what makes scanning for one viable.
        if previous_tag.is_some_and(|previous| tag <= previous) {
            return None;
        }
        previous_tag = Some(tag);

        entries.push(Entry {
            tag,
            kind: order.u16(data, offset + 2)?,
            count: order.u32(data, offset + 4)?,
            value: data.get(offset + 8..offset + 12)?.try_into().ok()?,
        });
    }

    Some(Ifd {
        start: at,
        entries,
        next: order.u32(data, at + 2 + count * ENTRY_LEN)?,
    })
}

/// Walks the directory chain, stopping at the first one that does not read.
pub fn read_chain(data: &[u8], order: Order, first: usize) -> Vec<Ifd> {
    let mut directories = Vec::new();
    let mut at = first;
    let mut seen = Vec::new();

    while at != 0 && !seen.contains(&at) {
        seen.push(at);
        let Some(ifd) = read_ifd(data, order, at) else {
            break;
        };
        at = ifd.next as usize;
        directories.push(ifd);
    }
    directories
}

/// Looks for a directory when the header that pointed at one is gone.
///
/// Directories are strongly shaped - an entry count that is not absurd, ascending tags, field
/// types the specification defines - so a scan finds the real one rather than a coincidence.
pub fn find_ifd(data: &[u8], order: Order, limit: usize) -> Option<Ifd> {
    let end = limit.min(data.len());
    // Directories are word-aligned in every writer's output, which halves the search.
    (HEADER_LEN..end)
        .step_by(2)
        .filter_map(|at| read_ifd(data, order, at))
        .find(|ifd| ifd.entry(IMAGE_WIDTH).is_some() && ifd.entry(STRIP_OFFSETS).is_some())
}

/// Builds a fresh TIFF holding one directory.
///
/// Layout is header, strip data, out-of-line values, directory - chosen so that every offset
/// is known by the time the thing referring to it is written.
pub struct Builder {
    order: Order,
    out: Vec<u8>,
    strips: Vec<Vec<u8>>,
}

impl Builder {
    pub fn new(order: Order) -> Self {
        let mut out = Vec::new();
        out.extend_from_slice(&order.magic());
        order.put_u16(&mut out, CLASSIC_VERSION);
        // Patched once the directory's position is known.
        order.put_u32(&mut out, 0);
        Self { order, out, strips: Vec::new() }
    }

    /// Takes one strip of pixel data, which is copied when the file is written out.
    pub fn add_strip(&mut self, bytes: &[u8]) {
        self.strips.push(bytes.to_vec());
    }

    pub fn strip_count(&self) -> usize {
        self.strips.len()
    }

    /// Writes the whole file, with `overrides` replacing or adding entries by tag.
    ///
    /// The directory goes *first*, right after the header, with the values and the pixel data
    /// behind it. That is deliberate: a directory at the end of the file - which many writers
    /// emit - is the single thing a truncation is most likely to destroy, taking with it the
    /// only description of pixels that all survived. A repaired file should not be one cut
    /// away from the same problem.
    ///
    /// `source` entries whose out-of-line values are missing from the damaged file are dropped
    /// rather than carried over as offsets into nothing.
    pub fn finish(mut self, data: &[u8], source: &Ifd, overrides: &[(u16, Vec<u64>, u16)]) -> Vec<u8> {
        let mut fields: Vec<(u16, u16, u32, Vec<u8>)> = Vec::new();

        for entry in &source.entries {
            if overrides.iter().any(|(tag, _, _)| *tag == entry.tag) {
                continue;
            }
            if matches!(entry.tag, STRIP_OFFSETS | STRIP_BYTE_COUNTS) {
                continue;
            }
            match entry.data(self.order) {
                None => fields.push((entry.tag, entry.kind, entry.count, entry.value.to_vec())),
                Some(range) => {
                    if let Some(bytes) = data.get(range) {
                        fields.push((entry.tag, entry.kind, entry.count, bytes.to_vec()));
                    }
                }
            }
        }
        for (tag, values, kind) in overrides {
            fields.push((*tag, *kind, u32::try_from(values.len()).unwrap_or(0), pack(self.order, values, *kind)));
        }

        // The strip tables are written last so they can be sized, then sorted into place with
        // everything else - the specification requires the entries to be in tag order.
        let count = u32::try_from(self.strips.len()).unwrap_or(0);
        fields.push((STRIP_OFFSETS, 4, count, Vec::new()));
        fields.push((STRIP_BYTE_COUNTS, 4, count, Vec::new()));
        fields.sort_by_key(|(tag, _, _, _)| *tag);

        // Lay the file out before writing any of it: the directory refers forward to values
        // and strips, so their offsets have to be known first.
        let directory_len = 2 + fields.len() * ENTRY_LEN + 4;
        let mut cursor = even(HEADER_LEN + directory_len);
        let strip_table_len = self.strips.len() * 4;

        let mut value_offsets: Vec<Option<u32>> = Vec::with_capacity(fields.len());
        for (tag, _, _, bytes) in &fields {
            let length = match *tag {
                STRIP_OFFSETS | STRIP_BYTE_COUNTS => strip_table_len,
                _ => bytes.len(),
            };
            if length <= 4 {
                value_offsets.push(None);
                continue;
            }
            value_offsets.push(Some(u32::try_from(cursor).unwrap_or(u32::MAX)));
            cursor = even(cursor + length);
        }

        let mut strip_offsets = Vec::with_capacity(self.strips.len());
        for strip in &self.strips {
            strip_offsets.push(u64::try_from(cursor).unwrap_or(0));
            cursor = even(cursor + strip.len());
        }

        let offsets = pack(self.order, &strip_offsets, 4);
        let counts = pack(self.order, &self.strips.iter().map(|strip| strip.len() as u64).collect::<Vec<_>>(), 4);
        for (tag, _, _, bytes) in &mut fields {
            match *tag {
                STRIP_OFFSETS => *bytes = offsets.clone(),
                STRIP_BYTE_COUNTS => *bytes = counts.clone(),
                _ => {}
            }
        }

        self.order.write_u32(&mut self.out, 4, HEADER_LEN as u32);
        self.order.put_u16(&mut self.out, u16::try_from(fields.len()).unwrap_or(0));
        for (index, (tag, kind, count, bytes)) in fields.iter().enumerate() {
            self.order.put_u16(&mut self.out, *tag);
            self.order.put_u16(&mut self.out, *kind);
            self.order.put_u32(&mut self.out, *count);
            match value_offsets.get(index).copied().flatten() {
                Some(offset) => self.order.put_u32(&mut self.out, offset),
                None => {
                    let mut inline = [0u8; 4];
                    for (slot, byte) in inline.iter_mut().zip(bytes) {
                        *slot = *byte;
                    }
                    self.out.extend_from_slice(&inline);
                }
            }
        }
        self.order.put_u32(&mut self.out, 0);

        for (index, (_, _, _, bytes)) in fields.iter().enumerate() {
            if value_offsets.get(index).copied().flatten().is_none() {
                continue;
            }
            self.align();
            self.out.extend_from_slice(bytes);
        }
        let strips = std::mem::take(&mut self.strips);
        for strip in &strips {
            self.align();
            self.out.extend_from_slice(strip);
        }
        self.out
    }

    /// TIFF requires every offset to be even.
    fn align(&mut self) {
        if self.out.len() % 2 == 1 {
            self.out.push(0);
        }
    }
}

fn even(offset: usize) -> usize {
    offset + offset % 2
}

fn pack(order: Order, values: &[u64], kind: u16) -> Vec<u8> {
    let mut out = Vec::with_capacity(values.len() * type_size(kind) as usize);
    for value in values {
        match type_size(kind) {
            2 => order.put_u16(&mut out, u16::try_from(*value).unwrap_or(u16::MAX)),
            _ => order.put_u32(&mut out, u32::try_from(*value).unwrap_or(u32::MAX)),
        }
    }
    out
}

/// A minimal but real TIFF: one grayscale image split into strips of `rows_per_strip` rows.
///
/// Built through [`Builder`], which is also what the repairs use, so a test that reads this
/// back is exercising the writer as well as the reader.
#[cfg(test)]
pub fn sample_tiff(order: Order, width: u32, height: u32, rows_per_strip: u32) -> Vec<u8> {
    let mut builder = Builder::new(order);
    let mut row = 0;
    while row < height {
        let rows = rows_per_strip.min(height - row);
        let bytes: Vec<u8> = (0..rows * width).map(|index| ((index / width + row) % 256) as u8).collect();
        builder.add_strip(&bytes);
        row += rows;
    }

    let source = Ifd {
        start: 0,
        entries: Vec::new(),
        next: 0,
    };
    builder.finish(
        &[],
        &source,
        &[
            (IMAGE_WIDTH, vec![u64::from(width)], 4),
            (IMAGE_LENGTH, vec![u64::from(height)], 4),
            (258, vec![8], 3),
            (259, vec![1], 3),
            (262, vec![1], 3),
            (ROWS_PER_STRIP, vec![u64::from(rows_per_strip)], 4),
            (277, vec![1], 3),
        ],
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn read_first(data: &[u8]) -> (Order, Ifd) {
        let (order, version, first) = parse_header(data).expect("the header parses");
        assert_eq!(version, CLASSIC_VERSION);
        (order, read_ifd(data, order, first).expect("the directory parses"))
    }

    #[test]
    fn a_built_file_reads_back_the_way_it_was_written() {
        for order in [Order::Little, Order::Big] {
            let data = sample_tiff(order, 64, 40, 10);
            let (read_order, ifd) = read_first(&data);

            assert_eq!(read_order, order);
            assert_eq!(ifd.number(&data, order, IMAGE_WIDTH), Some(64));
            assert_eq!(ifd.number(&data, order, IMAGE_LENGTH), Some(40));
            assert_eq!(ifd.strips(&data, order).len(), 4);
            assert_eq!(ifd.next, 0);

            for (offset, count) in ifd.strips(&data, order) {
                assert_eq!(count, 640, "ten rows of sixty-four bytes");
                assert!(offset + count <= data.len() as u64, "every strip is inside the file");
            }
        }
    }

    #[test]
    fn entry_values_follow_their_offset_only_when_they_have_to() {
        let data = sample_tiff(Order::Little, 32, 32, 8);
        let (order, ifd) = read_first(&data);

        let width = ifd.entry(IMAGE_WIDTH).expect("width");
        assert!(width.is_inline(), "one long fits in the entry");
        assert_eq!(width.data(order), None);

        let offsets = ifd.entry(STRIP_OFFSETS).expect("strip offsets");
        assert!(!offsets.is_inline(), "four longs do not");
        assert!(offsets.data(order).is_some());
    }

    #[test]
    fn a_directory_is_found_without_a_header() {
        let data = sample_tiff(Order::Little, 32, 32, 8);
        let (_, expected) = read_first(&data);
        let found = find_ifd(&data, Order::Little, data.len()).expect("found");
        assert_eq!(found.start, expected.start);
    }

    #[test]
    fn arbitrary_bytes_are_not_mistaken_for_a_directory() {
        let noise: Vec<u8> = (0..4096u32).map(|index| (index % 251) as u8).collect();
        assert!(find_ifd(&noise, Order::Little, noise.len()).is_none());
        assert!(find_ifd(&noise, Order::Big, noise.len()).is_none());
    }

    #[test]
    fn descending_tags_are_refused() {
        let mut data = sample_tiff(Order::Little, 16, 16, 4);
        let (order, ifd) = read_first(&data);
        // Overwrite the first entry's tag with one larger than the second's.
        let at = ifd.start + 2;
        if let Some(slot) = data.get_mut(at..at + 2) {
            slot.copy_from_slice(&u16::MAX.to_le_bytes());
        }
        assert!(read_ifd(&data, order, ifd.start).is_none());
    }

    #[test]
    fn a_field_type_this_reader_does_not_know_is_tolerated() {
        let entry = Entry {
            tag: 1,
            kind: 777,
            count: 4,
            value: [1, 2, 3, 4],
        };
        assert_eq!(entry.byte_len(), 4, "unknown types are counted as bytes");
        assert!(entry.is_inline());
    }

    #[test]
    fn short_input_never_panics() {
        for len in 0..40usize {
            let data = vec![0x49u8; len];
            let _ = parse_header(&data);
            let _ = read_ifd(&data, Order::Little, 8);
            let _ = find_ifd(&data, Order::Little, len);
        }
    }
}
