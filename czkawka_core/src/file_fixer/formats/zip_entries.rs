//! Walking a ZIP archive from its local headers, and writing a new directory for what is
//! found.
//!
//! A ZIP is described twice: each member carries a local header immediately before its data,
//! and the archive repeats all of that in a central directory at the end. The duplication is
//! what makes the format recoverable - the central directory is the part a truncation destroys
//! first, and it holds nothing that the local headers do not already say.
//!
//! Better still, every member carries a CRC-32 of its own uncompressed contents. That turns
//! "this file came out of the archive intact" into a checksum rather than a judgement, exactly
//! as FLAC's per-frame CRC does, and it is why entries recovered by scanning can be called
//! original rather than plausible.

use std::ops::Range;

use crate::file_fixer::io::{put_u16_le, put_u32_le, read_u16_le, read_u32_le};

pub const LOCAL_HEADER: [u8; 4] = [0x50, 0x4B, 0x03, 0x04];
pub const CENTRAL_HEADER: [u8; 4] = [0x50, 0x4B, 0x01, 0x02];
pub const END_OF_CENTRAL_DIRECTORY: [u8; 4] = [0x50, 0x4B, 0x05, 0x06];
pub const DATA_DESCRIPTOR: [u8; 4] = [0x50, 0x4B, 0x07, 0x08];
pub const ZIP64_END_OF_CENTRAL_DIRECTORY: [u8; 4] = [0x50, 0x4B, 0x06, 0x06];
pub const ZIP64_LOCATOR: [u8; 4] = [0x50, 0x4B, 0x06, 0x07];

/// Fixed part of a local file header, before the name and extra field.
const LOCAL_HEADER_LEN: usize = 30;
/// Fixed part of a central directory header.
const CENTRAL_HEADER_LEN: usize = 46;

/// Set when the sizes were not known at the time the header was written and follow the data in
/// a descriptor instead. Streaming writers - every "zip on the fly" web endpoint - set it.
const FLAG_STREAMED: u16 = 0x0008;

/// Set when the member's data is encrypted, which puts its contents beyond checking here.
const FLAG_ENCRYPTED: u16 = 0x0001;

/// Value a 32-bit size or offset field carries when the real one is in a zip64 extra field.
const ZIP64_MARKER: u32 = 0xFFFF_FFFF;

pub const METHOD_STORED: u16 = 0;
pub const METHOD_DEFLATE: u16 = 8;

/// Names longer than this are a damaged length field, not a file name.
const MAX_NAME_LEN: usize = 4096;

#[derive(Debug, Clone)]
pub struct Entry {
    /// Offset of the local header.
    pub start: usize,
    pub name: Vec<u8>,
    pub flags: u16,
    pub method: u16,
    /// Modification time and date, kept as stored so a rebuild preserves them.
    pub modified: (u16, u16),
    pub crc32: u32,
    pub compressed_size: u64,
    pub uncompressed_size: u64,
    /// The compressed bytes, clamped to what the file actually holds.
    pub data: Range<usize>,
    /// False when the file ends inside this member's data.
    pub complete: bool,
    /// Offset just past the member, including any trailing data descriptor.
    pub end: usize,
}

impl Entry {
    /// True for the zero-length entries that record a folder.
    pub fn is_directory(&self) -> bool {
        self.name.last() == Some(&b'/') || (self.uncompressed_size == 0 && self.compressed_size == 0)
    }

    pub fn name_lossy(&self) -> String {
        String::from_utf8_lossy(&self.name).into_owned()
    }
}

/// Reads the local header at `at`, and works out where the member's data ends.
pub fn read_entry(data: &[u8], at: usize) -> Option<Entry> {
    if data.get(at..at + 4)? != LOCAL_HEADER {
        return None;
    }
    let flags = read_u16_le(data, at + 6)?;
    let method = read_u16_le(data, at + 8)?;
    let crc32 = read_u32_le(data, at + 14)?;
    let compressed = read_u32_le(data, at + 18)?;
    let uncompressed = read_u32_le(data, at + 22)?;
    let name_len = usize::from(read_u16_le(data, at + 26)?);
    let extra_len = usize::from(read_u16_le(data, at + 28)?);
    if name_len > MAX_NAME_LEN {
        return None;
    }

    let name = data.get(at + LOCAL_HEADER_LEN..at + LOCAL_HEADER_LEN + name_len)?.to_vec();
    let extra_start = at + LOCAL_HEADER_LEN + name_len;
    let extra = data.get(extra_start..extra_start + extra_len).unwrap_or_default();
    let data_start = extra_start + extra_len;

    let (mut compressed_size, mut uncompressed_size) = zip64_sizes(extra, compressed, uncompressed);
    let mut end = data_start + usize::try_from(compressed_size).unwrap_or(usize::MAX);
    let mut crc32 = crc32;

    // A streamed entry left its sizes at zero; they are in a descriptor after the data, and
    // the only way to find that is to look for what follows the member.
    if flags & FLAG_STREAMED != 0 && compressed_size == 0 {
        if let Some(found) = find_descriptor(data, data_start) {
            crc32 = found.crc32;
            compressed_size = found.compressed_size;
            uncompressed_size = found.uncompressed_size;
            end = found.end;
        } else {
            end = data.len();
            compressed_size = (data.len() - data_start.min(data.len())) as u64;
        }
    } else if flags & FLAG_STREAMED != 0 && data.get(end..end + 4) == Some(&DATA_DESCRIPTOR) {
        end += 16;
    }

    let complete = data_start as u64 + compressed_size <= data.len() as u64;
    let data_end = usize::try_from(data_start as u64 + compressed_size).unwrap_or(data.len()).min(data.len());

    Some(Entry {
        start: at,
        name,
        flags,
        method,
        modified: (read_u16_le(data, at + 10)?, read_u16_le(data, at + 12)?),
        crc32,
        compressed_size,
        uncompressed_size,
        data: data_start.min(data.len())..data_end,
        complete,
        end: end.min(data.len()),
    })
}

struct Descriptor {
    crc32: u32,
    compressed_size: u64,
    uncompressed_size: u64,
    end: usize,
}

/// Finds the descriptor that closes a streamed member, confirmed by the size it records.
///
/// The signature is optional in the specification but written by essentially every producer,
/// and the size it states has to match the distance back to the start of the data - which is
/// what rules out the same four bytes appearing inside the compressed data.
fn find_descriptor(data: &[u8], data_start: usize) -> Option<Descriptor> {
    let mut offset = data_start;
    while offset + 16 <= data.len() {
        let found = data.get(offset..)?.windows(4).position(|w| w == DATA_DESCRIPTOR)? + offset;
        let compressed_size = u64::from(read_u32_le(data, found + 8)?);
        if found as u64 == data_start as u64 + compressed_size {
            return Some(Descriptor {
                crc32: read_u32_le(data, found + 4)?,
                compressed_size,
                uncompressed_size: u64::from(read_u32_le(data, found + 12)?),
                end: found + 16,
            });
        }
        offset = found + 1;
    }
    None
}

/// Real sizes from a zip64 extra field, when the 32-bit ones are placeholders.
fn zip64_sizes(extra: &[u8], compressed: u32, uncompressed: u32) -> (u64, u64) {
    if compressed != ZIP64_MARKER && uncompressed != ZIP64_MARKER {
        return (u64::from(compressed), u64::from(uncompressed));
    }

    let mut offset = 0;
    while offset + 4 <= extra.len() {
        let Some(id) = read_u16_le(extra, offset) else {
            break;
        };
        let Some(len) = read_u16_le(extra, offset + 2).map(usize::from) else {
            break;
        };
        if id == 0x0001 {
            let read = |at: usize| -> Option<u64> {
                let bytes: [u8; 8] = extra.get(offset + 4 + at..offset + 12 + at)?.try_into().ok()?;
                Some(u64::from_le_bytes(bytes))
            };
            // The fields appear in a fixed order, present only when their 32-bit counterpart
            // was a placeholder.
            let mut at = 0;
            let mut sizes = (u64::from(compressed), u64::from(uncompressed));
            if uncompressed == ZIP64_MARKER {
                sizes.1 = read(at).unwrap_or(sizes.1);
                at += 8;
            }
            if compressed == ZIP64_MARKER {
                sizes.0 = read(at).unwrap_or(sizes.0);
            }
            return sizes;
        }
        offset += 4 + len;
    }
    (u64::from(compressed), u64::from(uncompressed))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EndOfDirectory {
    pub start: usize,
    pub entries: u16,
    pub directory_offset: u32,
    pub directory_size: u32,
}

/// Finds the end-of-directory record, which sits at the very end of an undamaged archive.
pub fn find_end_of_directory(data: &[u8]) -> Option<EndOfDirectory> {
    // The record is 22 bytes plus a comment of up to 64 KB, so only the tail is worth reading.
    let window = data.len().saturating_sub(22 + u16::MAX as usize);
    let start = data.get(window..)?.windows(4).rposition(|w| w == END_OF_CENTRAL_DIRECTORY)? + window;

    Some(EndOfDirectory {
        start,
        entries: read_u16_le(data, start + 10)?,
        directory_size: read_u32_le(data, start + 12)?,
        directory_offset: read_u32_le(data, start + 16)?,
    })
}

/// Walks the members from the front of the file.
///
/// Where one member ends the next begins, so the walk is a chain like any other - and when a
/// link is broken, the next local header is found by scanning, because a ZIP member is
/// self-describing and does not need the one before it.
pub fn scan_entries(data: &[u8], mut should_stop: impl FnMut() -> bool) -> Vec<Entry> {
    let mut entries = Vec::new();
    let mut offset = 0;

    while offset + LOCAL_HEADER_LEN <= data.len() {
        if should_stop() {
            break;
        }
        if data.get(offset..offset + 4) == Some(&CENTRAL_HEADER) {
            break;
        }

        match read_entry(data, offset) {
            Some(entry) => {
                let next = entry.end.max(entry.data.end);
                let complete = entry.complete;
                entries.push(entry);
                if !complete {
                    break;
                }
                offset = next.max(offset + 1);
            }
            None => match find_local_header(data, offset + 1) {
                Some(found) => offset = found,
                None => break,
            },
        }
    }
    entries
}

pub fn find_local_header(data: &[u8], from: usize) -> Option<usize> {
    Some(data.get(from..)?.windows(4).position(|w| w == LOCAL_HEADER)? + from)
}

/// What checking a member's contents against its stored checksum concluded.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Verdict {
    /// The contents decompressed and matched the stored CRC.
    Intact,
    /// They decompressed and did not match, or would not decompress at all.
    Corrupt,
    /// The compression method is one this tool cannot decode, or the member is too large to
    /// check inside the allocation limit. Present, but unproven.
    Unchecked,
    /// The member's data is not all in the file.
    Incomplete,
}

/// Checks a member against the checksum stored with it.
pub fn verify(data: &[u8], entry: &Entry, limit: u64) -> Verdict {
    if !entry.complete {
        return Verdict::Incomplete;
    }
    let Some(payload) = data.get(entry.data.clone()) else {
        return Verdict::Incomplete;
    };
    if entry.is_directory() && entry.uncompressed_size == 0 {
        return Verdict::Intact;
    }
    // Encrypted contents cannot be checked without the password, and the stored checksum is of
    // the plaintext. Calling that corrupt would throw away members that are perfectly fine.
    if entry.flags & FLAG_ENCRYPTED != 0 || entry.uncompressed_size > limit {
        return Verdict::Unchecked;
    }

    match entry.method {
        METHOD_STORED => {
            if payload.len() as u64 != entry.uncompressed_size {
                return Verdict::Corrupt;
            }
            checksum_matches(payload, entry.crc32)
        }
        METHOD_DEFLATE => match inflate(payload, limit) {
            (output, true) if output.len() as u64 == entry.uncompressed_size => checksum_matches(&output, entry.crc32),
            _ => Verdict::Corrupt,
        },
        _ => Verdict::Unchecked,
    }
}

fn checksum_matches(bytes: &[u8], expected: u32) -> Verdict {
    if crc32fast::hash(bytes) == expected { Verdict::Intact } else { Verdict::Corrupt }
}

/// Decompresses as far as the data allows, returning what came out and whether it finished.
pub fn inflate(payload: &[u8], limit: u64) -> (Vec<u8>, bool) {
    let limit = usize::try_from(limit).unwrap_or(usize::MAX);
    let result = super::deflate::inflate(payload, limit, false);
    (result.data, result.complete)
}

/// A member as it will be written into a rebuilt archive.
pub struct Rebuilt {
    pub name: Vec<u8>,
    pub method: u16,
    pub modified: (u16, u16),
    pub crc32: u32,
    pub compressed_size: u64,
    pub uncompressed_size: u64,
    /// Offset of the local header in the new archive.
    pub offset: u64,
}

/// Writes a local header for `entry`, with the sizes filled in.
///
/// A streamed member's header said nothing about its contents; the sizes found behind it are
/// written in and the flag that pointed at the descriptor is cleared, which turns an archive
/// that only a streaming reader could open into an ordinary one.
pub fn write_local_header(out: &mut Vec<u8>, entry: &Entry) {
    let header_start = out.len();
    out.extend_from_slice(&LOCAL_HEADER);
    out.extend_from_slice(&20u16.to_le_bytes());
    out.extend_from_slice(&(entry.flags & !FLAG_STREAMED).to_le_bytes());
    out.extend_from_slice(&entry.method.to_le_bytes());
    out.extend_from_slice(&entry.modified.0.to_le_bytes());
    out.extend_from_slice(&entry.modified.1.to_le_bytes());
    out.extend_from_slice(&entry.crc32.to_le_bytes());
    out.extend_from_slice(&clamp32(entry.compressed_size).to_le_bytes());
    out.extend_from_slice(&clamp32(entry.uncompressed_size).to_le_bytes());
    out.extend_from_slice(&u16::try_from(entry.name.len()).unwrap_or(0).to_le_bytes());
    out.extend_from_slice(&0u16.to_le_bytes());
    out.extend_from_slice(&entry.name);
    debug_assert_eq!(out.len(), header_start + LOCAL_HEADER_LEN + entry.name.len());
}

fn clamp32(value: u64) -> u32 {
    u32::try_from(value).unwrap_or(ZIP64_MARKER)
}

/// Writes the central directory and the record that closes the archive.
pub fn write_directory(out: &mut Vec<u8>, members: &[Rebuilt]) {
    let directory_offset = out.len() as u64;

    for member in members {
        let needs_zip64 = member.compressed_size > u64::from(ZIP64_MARKER) || member.uncompressed_size > u64::from(ZIP64_MARKER) || member.offset > u64::from(ZIP64_MARKER);
        let extra = if needs_zip64 {
            let mut extra = Vec::with_capacity(28);
            extra.extend_from_slice(&1u16.to_le_bytes());
            extra.extend_from_slice(&24u16.to_le_bytes());
            extra.extend_from_slice(&member.uncompressed_size.to_le_bytes());
            extra.extend_from_slice(&member.compressed_size.to_le_bytes());
            extra.extend_from_slice(&member.offset.to_le_bytes());
            extra
        } else {
            Vec::new()
        };

        out.extend_from_slice(&CENTRAL_HEADER);
        out.extend_from_slice(&20u16.to_le_bytes());
        out.extend_from_slice(&if needs_zip64 { 45u16 } else { 20u16 }.to_le_bytes());
        out.extend_from_slice(&0u16.to_le_bytes());
        out.extend_from_slice(&member.method.to_le_bytes());
        out.extend_from_slice(&member.modified.0.to_le_bytes());
        out.extend_from_slice(&member.modified.1.to_le_bytes());
        out.extend_from_slice(&member.crc32.to_le_bytes());
        out.extend_from_slice(&clamp32(member.compressed_size).to_le_bytes());
        out.extend_from_slice(&clamp32(member.uncompressed_size).to_le_bytes());
        out.extend_from_slice(&u16::try_from(member.name.len()).unwrap_or(0).to_le_bytes());
        out.extend_from_slice(&u16::try_from(extra.len()).unwrap_or(0).to_le_bytes());
        out.extend_from_slice(&0u16.to_le_bytes());
        out.extend_from_slice(&0u16.to_le_bytes());
        out.extend_from_slice(&0u16.to_le_bytes());
        out.extend_from_slice(&0u32.to_le_bytes());
        out.extend_from_slice(&clamp32(member.offset).to_le_bytes());
        out.extend_from_slice(&member.name);
        out.extend_from_slice(&extra);
        debug_assert!(out.len() >= CENTRAL_HEADER_LEN);
    }

    let directory_size = out.len() as u64 - directory_offset;
    let needs_zip64 = members.len() > usize::from(u16::MAX) || directory_offset > u64::from(ZIP64_MARKER) || directory_size > u64::from(ZIP64_MARKER);

    if needs_zip64 {
        let record = out.len() as u64;
        out.extend_from_slice(&ZIP64_END_OF_CENTRAL_DIRECTORY);
        out.extend_from_slice(&44u64.to_le_bytes());
        out.extend_from_slice(&45u16.to_le_bytes());
        out.extend_from_slice(&45u16.to_le_bytes());
        out.extend_from_slice(&0u32.to_le_bytes());
        out.extend_from_slice(&0u32.to_le_bytes());
        out.extend_from_slice(&(members.len() as u64).to_le_bytes());
        out.extend_from_slice(&(members.len() as u64).to_le_bytes());
        out.extend_from_slice(&directory_size.to_le_bytes());
        out.extend_from_slice(&directory_offset.to_le_bytes());

        out.extend_from_slice(&ZIP64_LOCATOR);
        out.extend_from_slice(&0u32.to_le_bytes());
        out.extend_from_slice(&record.to_le_bytes());
        out.extend_from_slice(&1u32.to_le_bytes());
    }

    let count = u16::try_from(members.len()).unwrap_or(u16::MAX);
    out.extend_from_slice(&END_OF_CENTRAL_DIRECTORY);
    out.extend_from_slice(&0u16.to_le_bytes());
    out.extend_from_slice(&0u16.to_le_bytes());
    out.extend_from_slice(&count.to_le_bytes());
    out.extend_from_slice(&count.to_le_bytes());
    out.extend_from_slice(&clamp32(directory_size).to_le_bytes());
    out.extend_from_slice(&clamp32(directory_offset).to_le_bytes());
    out.extend_from_slice(&0u16.to_le_bytes());
}

/// Patches an already-written local header's sizes, used when a member is rewritten as stored
/// data whose length is only known after it has been produced.
pub fn patch_local_sizes(out: &mut [u8], header_offset: usize, crc32: u32, compressed: u64, uncompressed: u64) {
    put_u32_le(out, header_offset + 14, crc32);
    put_u32_le(out, header_offset + 18, clamp32(compressed));
    put_u32_le(out, header_offset + 22, clamp32(uncompressed));
    put_u16_le(out, header_offset + 8, METHOD_STORED);
}

#[cfg(test)]
/// Builds a real archive: `members` of stored (uncompressed) data, a central directory and
/// an end record. Stored entries keep the fixture honest - the CRC checks below are the
/// same ones a deflated member goes through.
pub fn sample_zip(members: &[(&str, &[u8])]) -> Vec<u8> {
    let mut out = Vec::new();
    let mut rebuilt = Vec::new();

    for (name, contents) in members {
        let entry = Entry {
            start: 0,
            name: name.as_bytes().to_vec(),
            flags: 0,
            method: METHOD_STORED,
            modified: (0x1234, 0x5678),
            crc32: crc32fast::hash(contents),
            compressed_size: contents.len() as u64,
            uncompressed_size: contents.len() as u64,
            data: 0..0,
            complete: true,
            end: 0,
        };
        let offset = out.len() as u64;
        write_local_header(&mut out, &entry);
        out.extend_from_slice(contents);
        rebuilt.push(Rebuilt {
            name: entry.name,
            method: entry.method,
            modified: entry.modified,
            crc32: entry.crc32,
            compressed_size: entry.compressed_size,
            uncompressed_size: entry.uncompressed_size,
            offset,
        });
    }

    write_directory(&mut out, &rebuilt);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn never() -> impl FnMut() -> bool {
        || false
    }

    #[test]
    fn a_built_archive_walks_back_to_its_members() {
        let data = sample_zip(&[("a.txt", b"hello"), ("b.bin", &[7u8; 100])]);
        let entries = scan_entries(&data, never());

        assert_eq!(entries.len(), 2);
        assert_eq!(entries.first().map(Entry::name_lossy).as_deref(), Some("a.txt"));
        assert_eq!(entries.get(1).map(|e| e.uncompressed_size), Some(100));
        for entry in &entries {
            assert!(entry.complete);
            assert_eq!(verify(&data, entry, 1 << 20), Verdict::Intact);
        }
    }

    #[test]
    fn the_end_record_states_what_the_directory_holds() {
        let data = sample_zip(&[("a.txt", b"hello"), ("b.txt", b"world")]);
        let end = find_end_of_directory(&data).expect("found");

        assert_eq!(end.entries, 2);
        assert_eq!(data.get(end.directory_offset as usize..end.directory_offset as usize + 4), Some(&CENTRAL_HEADER[..]));
    }

    /// The checksum is the whole point: altered contents must not pass as recovered.
    #[test]
    fn altered_contents_fail_their_checksum() {
        let mut data = sample_zip(&[("a.txt", b"the quick brown fox")]);
        let entries = scan_entries(&data, never());
        let entry = entries.first().expect("one entry").clone();
        if let Some(byte) = data.get_mut(entry.data.start) {
            *byte ^= 0xFF;
        }

        assert_eq!(verify(&data, &entry, 1 << 20), Verdict::Corrupt);
    }

    #[test]
    fn a_member_the_file_ends_inside_is_reported_incomplete() {
        let mut data = sample_zip(&[("a.txt", b"hello"), ("big.bin", &[3u8; 5000])]);
        data.truncate(data.len() - 3000);

        let entries = scan_entries(&data, never());
        assert_eq!(entries.len(), 2);
        assert_eq!(entries.first().map(|entry| verify(&data, entry, 1 << 20)), Some(Verdict::Intact));
        assert_eq!(entries.get(1).map(|entry| verify(&data, entry, 1 << 20)), Some(Verdict::Incomplete));
    }

    #[test]
    fn a_deflated_member_is_checked_by_decompressing_it() {
        let contents: Vec<u8> = (0..5000u32).map(|index| (index % 7) as u8).collect();
        let compressed = miniz_oxide::deflate::compress_to_vec(&contents, 6);

        let entry = Entry {
            start: 0,
            name: b"x.bin".to_vec(),
            flags: 0,
            method: METHOD_DEFLATE,
            modified: (0, 0),
            crc32: crc32fast::hash(&contents),
            compressed_size: compressed.len() as u64,
            uncompressed_size: contents.len() as u64,
            data: 0..compressed.len(),
            complete: true,
            end: compressed.len(),
        };
        assert_eq!(verify(&compressed, &entry, 1 << 20), Verdict::Intact);

        let mut damaged = compressed.clone();
        if let Some(byte) = damaged.get_mut(compressed.len() / 2) {
            *byte ^= 0xFF;
        }
        assert_ne!(verify(&damaged, &entry, 1 << 20), Verdict::Intact);
    }

    /// A stream cut short still yields the bytes that decompressed before it stopped.
    #[test]
    fn a_truncated_deflate_stream_yields_what_it_managed() {
        let contents: Vec<u8> = (0..20_000u32).map(|index| (index % 251) as u8).collect();
        let mut compressed = miniz_oxide::deflate::compress_to_vec(&contents, 6);
        compressed.truncate(compressed.len() / 2);

        let (output, finished) = inflate(&compressed, 1 << 20);
        assert!(!finished);
        assert!(!output.is_empty() && output.len() < contents.len());
        assert_eq!(output, contents.get(..output.len()).expect("a prefix"));
    }

    #[test]
    fn a_member_too_large_to_check_is_reported_unproven_rather_than_broken() {
        let data = sample_zip(&[("a.txt", b"hello")]);
        let entry = scan_entries(&data, never()).first().cloned().expect("one entry");
        assert_eq!(verify(&data, &entry, 2), Verdict::Unchecked);
    }

    #[test]
    fn nothing_that_is_not_a_local_header_is_read_as_one() {
        assert!(read_entry(&[], 0).is_none());
        assert!(read_entry(&[0x50, 0x4B, 0x03, 0x04], 0).is_none());
        assert!(read_entry(&[0u8; 128], 0).is_none());
    }
}
