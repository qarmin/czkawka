//! Walking a FLAC stream: metadata blocks, frame headers, and the two checksums that make
//! this format the easiest one here to be honest about.
//!
//! Every frame carries a CRC-8 over its header and a CRC-16 over the whole frame. A frame that
//! passes both is the original, byte for byte - no other format in this crate can say that
//! about its content. Frames are not length-prefixed, so the end of one is found by the only
//! means available: assume the frame ends where the next sync code begins, and accept that
//! guess only if the CRC-16 written there agrees. A wrong boundary fails with probability
//! 65535 in 65536, so the search settles on the right one.
//!
//! Frame headers are also self-describing - sample rate, channel count and sample width are
//! repeated in every single frame - which is why a FLAC whose first sector is gone can be
//! rebuilt exactly rather than guessed at.

use std::ops::Range;

use crate::file_fixer::engine::RepairContext;
use crate::file_fixer::io::{read_u16_be, read_u32_be};

pub const MAGIC: [u8; 4] = *b"fLaC";

/// Metadata block type for `STREAMINFO`, which every stream must begin with.
pub const STREAMINFO: u8 = 0;
pub const STREAMINFO_LEN: usize = 34;

/// Shortest a frame can be: a six-byte header with its CRC-8, one subframe, and the CRC-16.
const MIN_FRAME_LEN: usize = 10;

/// Ceiling on a frame's length when the stream does not state one. Real frames are a few
/// kilobytes; this only exists so a damaged file cannot make the search unbounded.
const DEFAULT_MAX_FRAME_LEN: usize = 1 << 20;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MetadataBlock {
    pub kind: u8,
    pub last: bool,
    /// Offset of the four-byte block header.
    pub start: usize,
    pub payload: Range<usize>,
    pub complete: bool,
}

impl MetadataBlock {
    pub fn end(&self) -> usize {
        self.payload.end
    }
}

/// Walks the metadata blocks that follow the `fLaC` marker.
pub fn metadata_blocks(data: &[u8]) -> Vec<MetadataBlock> {
    let mut blocks = Vec::new();
    let mut offset = MAGIC.len();

    while let Some(&header) = data.get(offset) {
        let Some(length) = read_u24_be(data, offset + 1) else {
            break;
        };
        let payload_start = offset + 4;
        let declared_end = payload_start as u64 + u64::from(length);
        let complete = declared_end <= data.len() as u64;
        let payload_end = usize::try_from(declared_end).unwrap_or(data.len()).min(data.len());

        let block = MetadataBlock {
            kind: header & 0x7F,
            last: header & 0x80 != 0,
            start: offset,
            payload: payload_start..payload_end,
            complete,
        };
        offset = payload_end;
        let last = block.last;
        blocks.push(block);
        if last || !complete {
            break;
        }
    }
    blocks
}

/// Where the audio begins: past the last metadata block.
pub fn audio_start(data: &[u8]) -> Option<usize> {
    if !data.starts_with(&MAGIC) {
        return None;
    }
    let blocks = metadata_blocks(data);
    let last = blocks.last()?;
    (last.last && last.complete).then(|| last.end())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct StreamInfo {
    pub min_block_size: u16,
    pub max_block_size: u16,
    pub min_frame_size: u32,
    pub max_frame_size: u32,
    pub sample_rate: u32,
    pub channels: u8,
    pub bits_per_sample: u8,
    /// Zero means the writer did not know, which is legal and common for a live capture.
    pub total_samples: u64,
}

pub fn stream_info(data: &[u8]) -> Option<StreamInfo> {
    let block = metadata_blocks(data).into_iter().find(|block| block.kind == STREAMINFO && block.complete)?;
    parse_stream_info(data.get(block.payload)?)
}

pub fn parse_stream_info(payload: &[u8]) -> Option<StreamInfo> {
    if payload.len() < STREAMINFO_LEN {
        return None;
    }
    let packed = read_u32_be(payload, 10)?;
    let sample_rate = packed >> 12;
    let channels = ((packed >> 9) & 0x07) as u8 + 1;
    let bits_per_sample = ((packed >> 4) & 0x1F) as u8 + 1;
    let total_samples = (u64::from(packed & 0x0F) << 32) | u64::from(read_u32_be(payload, 14)?);

    if sample_rate == 0 {
        return None;
    }
    Some(StreamInfo {
        min_block_size: read_u16_be(payload, 0)?,
        max_block_size: read_u16_be(payload, 2)?,
        min_frame_size: read_u24_be(payload, 4)?,
        max_frame_size: read_u24_be(payload, 7)?,
        sample_rate,
        channels,
        bits_per_sample,
        total_samples,
    })
}

/// Encodes a `STREAMINFO` payload. The MD5 of the decoded audio is left at zero, which the
/// specification defines as "not computed" - the alternative would be to invent a checksum for
/// content that no longer matches the original file.
pub fn encode_stream_info(info: &StreamInfo) -> Vec<u8> {
    let mut out = Vec::with_capacity(STREAMINFO_LEN);
    out.extend_from_slice(&info.min_block_size.to_be_bytes());
    out.extend_from_slice(&info.max_block_size.to_be_bytes());
    out.extend_from_slice(&info.min_frame_size.to_be_bytes()[1..]);
    out.extend_from_slice(&info.max_frame_size.to_be_bytes()[1..]);

    let packed = (info.sample_rate << 12)
        | (u32::from(info.channels.saturating_sub(1) & 0x07) << 9)
        | (u32::from(info.bits_per_sample.saturating_sub(1) & 0x1F) << 4)
        | ((info.total_samples >> 32) & 0x0F) as u32;
    out.extend_from_slice(&packed.to_be_bytes());
    out.extend_from_slice(&(info.total_samples as u32).to_be_bytes());
    out.extend_from_slice(&[0u8; 16]);
    out
}

/// A metadata block header for `kind` with `length` bytes of payload.
pub fn metadata_header(kind: u8, last: bool, length: usize) -> [u8; 4] {
    let length = u32::try_from(length).unwrap_or(0);
    [kind & 0x7F | if last { 0x80 } else { 0 }, (length >> 16) as u8, (length >> 8) as u8, length as u8]
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FrameHeader {
    /// Samples per channel in this frame.
    pub block_size: u32,
    /// Zero when the frame defers to `STREAMINFO`.
    pub sample_rate: u32,
    pub channels: u8,
    /// Zero when the frame defers to `STREAMINFO`.
    pub bits_per_sample: u8,
    /// Frame number, or first sample number for a variable-block-size stream.
    pub number: u64,
    /// Header length in bytes, including the CRC-8.
    pub len: usize,
}

/// Reads and checksum-verifies a frame header. `None` means this offset does not begin one.
pub fn parse_frame_header(data: &[u8], at: usize) -> Option<FrameHeader> {
    if *data.get(at)? != 0xFF {
        return None;
    }
    let second = *data.get(at + 1)?;
    // Sync is fourteen bits; the next bit is reserved and must be zero.
    if second & 0xFE != 0xF8 {
        return None;
    }
    let third = *data.get(at + 2)?;
    let fourth = *data.get(at + 3)?;
    if fourth & 0x01 != 0 {
        return None;
    }

    let block_code = third >> 4;
    let rate_code = third & 0x0F;
    let channel_code = fourth >> 4;
    let size_code = (fourth >> 1) & 0x07;
    if block_code == 0 || rate_code == 0x0F || channel_code > 10 || size_code == 3 {
        return None;
    }

    let (number, mut offset) = read_utf8_number(data, at + 4)?;
    let block_size = match block_code {
        6 => u32::from(*data.get(offset)?) + 1,
        7 => u32::from(read_u16_be(data, offset)?) + 1,
        code => fixed_block_size(code)?,
    };
    offset += match block_code {
        6 => 1,
        7 => 2,
        _ => 0,
    };

    let sample_rate = match rate_code {
        12 => u32::from(*data.get(offset)?) * 1000,
        13 => u32::from(read_u16_be(data, offset)?),
        14 => u32::from(read_u16_be(data, offset)?) * 10,
        code => coded_sample_rate(code),
    };
    offset += match rate_code {
        12 => 1,
        13 | 14 => 2,
        _ => 0,
    };

    let stored_crc = *data.get(offset)?;
    if crc8(data.get(at..offset)?) != stored_crc {
        return None;
    }

    Some(FrameHeader {
        block_size,
        sample_rate,
        channels: if channel_code < 8 { channel_code + 1 } else { 2 },
        bits_per_sample: coded_bits_per_sample(size_code),
        number,
        len: offset + 1 - at,
    })
}

fn fixed_block_size(code: u8) -> Option<u32> {
    Some(match code {
        1 => 192,
        2..=5 => 576 << (code - 2),
        8..=15 => 256 << (code - 8),
        _ => return None,
    })
}

fn coded_sample_rate(code: u8) -> u32 {
    match code {
        1 => 88_200,
        2 => 176_400,
        3 => 192_000,
        4 => 8_000,
        5 => 16_000,
        6 => 22_050,
        7 => 24_000,
        8 => 32_000,
        9 => 44_100,
        10 => 48_000,
        11 => 96_000,
        // Zero means "as stated in STREAMINFO", which the caller resolves.
        _ => 0,
    }
}

fn coded_bits_per_sample(code: u8) -> u8 {
    match code {
        1 => 8,
        2 => 12,
        4 => 16,
        5 => 20,
        6 => 24,
        7 => 32,
        _ => 0,
    }
}

/// Reads the UTF-8-shaped number a frame header carries, returning it and the offset past it.
fn read_utf8_number(data: &[u8], at: usize) -> Option<(u64, usize)> {
    let first = *data.get(at)?;
    let extra = match first {
        0x00..=0x7F => return Some((u64::from(first), at + 1)),
        0xC0..=0xDF => 1,
        0xE0..=0xEF => 2,
        0xF0..=0xF7 => 3,
        0xF8..=0xFB => 4,
        0xFC..=0xFD => 5,
        0xFE => 6,
        _ => return None,
    };

    let mut value = u64::from(first & (0x3F >> extra));
    for index in 0..extra {
        let byte = *data.get(at + 1 + index)?;
        if byte & 0xC0 != 0x80 {
            return None;
        }
        value = (value << 6) | u64::from(byte & 0x3F);
    }
    Some((value, at + 1 + extra))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Frame {
    pub range: Range<usize>,
    pub header: FrameHeader,
}

impl Frame {
    pub fn samples(&self) -> u64 {
        u64::from(self.header.block_size)
    }
}

/// Reads the frame beginning at `at`, confirmed by its own CRC-16.
///
/// The end is wherever the checksum says it is: for every position that could hold the frame's
/// last two bytes, and that is followed by something that looks like the next frame (or by the
/// end of the file), the CRC over everything before it is compared against the value stored
/// there. Only a real boundary matches.
pub fn read_frame(data: &[u8], at: usize, max_len: usize) -> Option<Frame> {
    let header = parse_frame_header(data, at)?;
    let limit = (at + max_len.max(MIN_FRAME_LEN)).min(data.len());

    let mut crc = 0u16;
    let mut position = at;
    while position < limit {
        // `position` is a candidate first byte of the two-byte checksum.
        if position >= at + MIN_FRAME_LEN {
            let end = position + 2;
            let boundary = end == data.len() || parse_frame_header(data, end).is_some();
            if boundary && read_u16_be(data, position) == Some(crc) {
                return Some(Frame { range: at..end, header });
            }
        }
        crc = crc16_update(crc, *data.get(position)?);
        position += 1;
    }
    None
}

#[derive(Debug, Clone)]
pub struct Walk {
    pub frames: Vec<Frame>,
    /// First offset past the last verified frame.
    pub end: usize,
    /// True when the verified frames run to the end of the file.
    pub reaches_end: bool,
}

impl Walk {
    pub fn samples(&self) -> u64 {
        self.frames.iter().map(Frame::samples).sum()
    }

    pub fn block_sizes(&self) -> (u16, u16) {
        let sizes = self.frames.iter().map(|frame| frame.header.block_size);
        let min = sizes.clone().min().unwrap_or(0);
        let max = sizes.max().unwrap_or(0);
        (u16::try_from(min).unwrap_or(u16::MAX), u16::try_from(max).unwrap_or(u16::MAX))
    }

    pub fn frame_sizes(&self) -> (u32, u32) {
        let sizes = self.frames.iter().map(|frame| frame.range.len() as u32);
        (sizes.clone().min().unwrap_or(0), sizes.max().unwrap_or(0))
    }
}

/// Reads every frame that verifies, starting at `start`, stopping at the first one that does
/// not.
pub fn walk_frames(ctx: &RepairContext<'_>, start: usize, max_frame_len: usize) -> Walk {
    let mut frames = Vec::new();
    let mut offset = start;

    while offset < ctx.data.len() {
        if ctx.should_stop() {
            break;
        }
        let Some(frame) = read_frame(ctx.data, offset, max_frame_len) else {
            break;
        };
        offset = frame.range.end;
        frames.push(frame);
    }

    Walk {
        frames,
        end: offset,
        reaches_end: offset >= ctx.data.len(),
    }
}

/// Offsets that are worth trying when looking for the first frame after damage.
///
/// Scanning is cheap because a frame header is checksummed: a false sync survives the byte
/// pattern check about once every few hundred bytes, and then fails its CRC-8 immediately.
pub fn find_frame(ctx: &RepairContext<'_>, from: usize, max_frame_len: usize) -> Option<Frame> {
    let mut offset = from;
    while offset < ctx.data.len() {
        if ctx.should_stop() {
            return None;
        }
        let candidate = ctx.data.get(offset..)?.iter().position(|&byte| byte == 0xFF)?;
        let at = offset + candidate;
        if parse_frame_header(ctx.data, at).is_some()
            && let Some(frame) = read_frame(ctx.data, at, max_frame_len)
        {
            return Some(frame);
        }
        offset = at + 1;
    }
    None
}

/// The longest frame worth looking for, taken from the stream's own statement when it made one.
pub fn max_frame_len(info: Option<&StreamInfo>) -> usize {
    match info.map(|info| info.max_frame_size) {
        Some(max) if max > 0 => (max as usize).saturating_mul(2).max(MIN_FRAME_LEN),
        _ => DEFAULT_MAX_FRAME_LEN,
    }
}

fn read_u24_be(data: &[u8], offset: usize) -> Option<u32> {
    let bytes = data.get(offset..offset + 3)?;
    Some(u32::from_be_bytes([0, *bytes.first()?, *bytes.get(1)?, *bytes.get(2)?]))
}

/// CRC-8 with polynomial x^8 + x^2 + x + 1, which FLAC uses over each frame header.
pub fn crc8(data: &[u8]) -> u8 {
    let mut crc = 0u8;
    for &byte in data {
        crc ^= byte;
        for _ in 0..8 {
            crc = if crc & 0x80 != 0 { (crc << 1) ^ 0x07 } else { crc << 1 };
        }
    }
    crc
}

/// CRC-16 with polynomial x^16 + x^15 + x^2 + 1, which FLAC uses over each whole frame.
pub fn crc16(data: &[u8]) -> u16 {
    let mut crc = 0u16;
    for &byte in data {
        crc = crc16_update(crc, byte);
    }
    crc
}

fn crc16_update(crc: u16, byte: u8) -> u16 {
    let index = ((crc >> 8) as u8 ^ byte) as usize;
    (crc << 8) ^ CRC16_TABLE.get(index).copied().unwrap_or(0)
}

const CRC16_TABLE: [u16; 256] = build_crc16_table();

#[expect(clippy::indexing_slicing, reason = "a const fn cannot use get_mut, and the index is bounded by the loop")]
const fn build_crc16_table() -> [u16; 256] {
    let mut table = [0u16; 256];
    let mut index = 0usize;
    while index < 256 {
        let mut value = (index as u16) << 8;
        let mut bit = 0;
        while bit < 8 {
            value = if value & 0x8000 != 0 { (value << 1) ^ 0x8005 } else { value << 1 };
            bit += 1;
        }
        table[index] = value;
        index += 1;
    }
    table
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The reference vector from the specification's own description of the header checksum.
    #[test]
    fn the_header_checksum_matches_a_known_value() {
        assert_eq!(crc8(&[]), 0);
        assert_eq!(crc8(b"123456789"), 0xF4);
    }

    #[test]
    fn the_frame_checksum_matches_a_known_value() {
        assert_eq!(crc16(&[]), 0);
        assert_eq!(crc16(b"123456789"), 0xFEE8);
    }

    #[test]
    fn the_table_agrees_with_the_bitwise_definition() {
        let mut bitwise = 0u16;
        for &byte in b"a longer string to push the table through every branch" {
            bitwise ^= u16::from(byte) << 8;
            for _ in 0..8 {
                bitwise = if bitwise & 0x8000 != 0 { (bitwise << 1) ^ 0x8005 } else { bitwise << 1 };
            }
        }
        assert_eq!(crc16(b"a longer string to push the table through every branch"), bitwise);
    }

    #[test]
    fn block_sizes_follow_the_coding_table() {
        assert_eq!(fixed_block_size(1), Some(192));
        assert_eq!(fixed_block_size(2), Some(576));
        assert_eq!(fixed_block_size(5), Some(4608));
        assert_eq!(fixed_block_size(8), Some(256));
        assert_eq!(fixed_block_size(12), Some(4096));
        assert_eq!(fixed_block_size(15), Some(32768));
        assert_eq!(fixed_block_size(0), None);
    }

    #[test]
    fn the_frame_number_decodes_like_utf8() {
        assert_eq!(read_utf8_number(&[0x00], 0), Some((0, 1)));
        assert_eq!(read_utf8_number(&[0x7F], 0), Some((127, 1)));
        assert_eq!(read_utf8_number(&[0xC2, 0x80], 0), Some((128, 2)));
        assert_eq!(read_utf8_number(&[0xE0, 0xA0, 0x80], 0), Some((2048, 3)));
        assert_eq!(read_utf8_number(&[0xC2, 0x00], 0), None, "a bad continuation byte");
        assert_eq!(read_utf8_number(&[0x80], 0), None, "a continuation byte first");
    }

    #[test]
    fn stream_info_round_trips() {
        let info = StreamInfo {
            min_block_size: 4096,
            max_block_size: 4096,
            min_frame_size: 14,
            max_frame_size: 9000,
            sample_rate: 44_100,
            channels: 2,
            bits_per_sample: 16,
            total_samples: 1_234_567,
        };
        let encoded = encode_stream_info(&info);

        assert_eq!(encoded.len(), STREAMINFO_LEN);
        assert_eq!(parse_stream_info(&encoded), Some(info));
    }

    #[test]
    fn a_metadata_header_states_its_own_length() {
        assert_eq!(metadata_header(STREAMINFO, false, 34), [0, 0, 0, 34]);
        assert_eq!(metadata_header(STREAMINFO, true, 34), [0x80, 0, 0, 34]);
        assert_eq!(metadata_header(4, true, 0x01_0203), [0x84, 1, 2, 3]);
    }

    #[test]
    fn nothing_that_is_not_a_frame_header_is_accepted() {
        assert!(parse_frame_header(&[0xFF, 0xF8, 0x00, 0x00], 0).is_none(), "block code 0");
        assert!(parse_frame_header(&[0x00; 16], 0).is_none());
        assert!(parse_frame_header(&[0xFF, 0xFF, 0xC0, 0x00], 0).is_none(), "no sync");
        assert!(parse_frame_header(&[], 0).is_none());
    }
}
