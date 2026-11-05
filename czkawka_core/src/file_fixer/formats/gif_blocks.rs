//! Walking a GIF's block stream, and the one thing a GIF can prove about its own content.
//!
//! A GIF is a chain of self-delimiting blocks: every one states its own length, either as a
//! fixed-size descriptor or as a chain of sub-blocks ending in a zero byte. Damage therefore
//! shows up as a chain that runs off the end of the file or a byte where an introducer should
//! be, both of which are exact - unlike JPEG, there is no way to drift silently.
//!
//! Inside an image block the evidence is stronger still. LZW codes are *not* self-
//! synchronizing: a corrupted code very quickly refers to a dictionary entry that does not
//! exist yet, which is detectable, and a healthy frame decodes to exactly width x height
//! pixels - no more, no less. That count is this format's checksum, and it is what lets a
//! recovered frame be called original rather than plausible.
//!
//! Only the *length* of each dictionary entry is tracked here, never the pixels themselves.
//! Lengths obey the same recurrence as the strings do (a new entry is the previous string plus
//! one byte), so the pixel count comes out exact while the decoder stays allocation-free in
//! the size of the image.

use std::ops::Range;

use crate::file_fixer::io::read_u16_le;

pub const IMAGE_SEPARATOR: u8 = 0x2C;
pub const EXTENSION_INTRODUCER: u8 = 0x21;
pub const TRAILER: u8 = 0x3B;

/// Bytes of header before any block: signature, version and logical screen descriptor.
pub const SCREEN_HEADER_LEN: usize = 13;

/// Largest LZW code the format allows; the dictionary never grows past it.
const MAX_CODE: u16 = 1 << 12;

/// The logical screen descriptor: the canvas every frame is drawn onto.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Screen {
    pub width: u16,
    pub height: u16,
    pub packed: u8,
    pub background: u8,
    pub aspect: u8,
}

impl Screen {
    /// Size of the global colour table, zero when the file does not carry one.
    pub fn global_table_bytes(self) -> usize {
        color_table_bytes(self.packed)
    }

    /// Offset of the first block.
    pub fn blocks_start(self) -> usize {
        SCREEN_HEADER_LEN + self.global_table_bytes()
    }
}

fn color_table_bytes(packed: u8) -> usize {
    if packed & 0x80 == 0 {
        return 0;
    }
    3 * (1usize << ((packed & 0x07) + 1))
}

pub fn has_signature(data: &[u8]) -> bool {
    data.starts_with(b"GIF87a") || data.starts_with(b"GIF89a")
}

pub fn parse_screen(data: &[u8]) -> Option<Screen> {
    if !has_signature(data) {
        return None;
    }
    Some(Screen {
        width: read_u16_le(data, 6)?,
        height: read_u16_le(data, 8)?,
        packed: *data.get(10)?,
        background: *data.get(11)?,
        aspect: *data.get(12)?,
    })
}

/// What a frame's compressed data turned out to contain.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Frame {
    pub left: u16,
    pub top: u16,
    pub width: u16,
    pub height: u16,
    /// Pixels the LZW stream actually produced.
    pub pixels: u64,
    /// True when the stream ended on its end-of-information code rather than running out or
    /// hitting an impossible dictionary reference.
    pub ended_cleanly: bool,
    /// Bytes of compressed data the decoder got through before it stopped. Everything before
    /// this point expanded into real pixels, which is what makes a partial frame salvageable.
    pub consumed: usize,
}

impl Frame {
    pub fn expected_pixels(&self) -> u64 {
        u64::from(self.width) * u64::from(self.height)
    }

    /// A frame is only trustworthy when its codes decoded cleanly to exactly the number of
    /// pixels its descriptor promised.
    pub fn is_whole(&self) -> bool {
        self.ended_cleanly && self.pixels == self.expected_pixels()
    }

    /// Complete rows the decoder produced, which is what survives when a frame is cut short.
    pub fn rows_decoded(&self) -> u64 {
        if self.width == 0 {
            return 0;
        }
        (self.pixels / u64::from(self.width)).min(u64::from(self.height))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Image(Frame),
    Extension { label: u8 },
    Trailer,
}

#[derive(Debug, Clone)]
pub struct Block {
    pub range: Range<usize>,
    pub kind: Kind,
}

impl Block {
    pub fn frame(&self) -> Option<&Frame> {
        match &self.kind {
            Kind::Image(frame) => Some(frame),
            Kind::Extension { .. } | Kind::Trailer => None,
        }
    }
}

/// Why the walk stopped.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stop {
    /// The stream ended properly on its trailer byte.
    Trailer,
    /// A block ran past the end of the file. `at` is where that block began.
    Truncated { at: usize },
    /// The byte at `at` introduces nothing the format defines.
    Corrupt { at: usize },
}

#[derive(Debug, Clone)]
pub struct Walk {
    pub blocks: Vec<Block>,
    /// First offset past the last complete block.
    pub end: usize,
    pub stop: Stop,
}

impl Walk {
    pub fn frames(&self) -> usize {
        self.blocks.iter().filter(|block| block.frame().is_some()).count()
    }

    pub fn whole_frames(&self) -> usize {
        self.blocks.iter().filter(|block| block.frame().is_some_and(Frame::is_whole)).count()
    }

    /// True when every block parsed, every frame decoded to its full size, and the stream
    /// ended on its trailer.
    pub fn is_whole(&self) -> bool {
        self.stop == Stop::Trailer && self.frames() == self.whole_frames()
    }

    /// End offset of the last frame that decoded completely, together with how many frames
    /// that leaves. Everything before it - including the extensions that configure it - is
    /// kept, which is why this is an offset into the original file rather than a rebuild.
    pub fn last_whole_frame_end(&self) -> Option<(usize, usize)> {
        let mut kept = 0;
        let mut end = None;
        for block in &self.blocks {
            match block.frame() {
                Some(frame) if frame.is_whole() => {
                    kept += 1;
                    end = Some(block.range.end);
                }
                Some(_) => break,
                None => {}
            }
        }
        end.map(|end| (end, kept))
    }
}

/// Walks the block chain from `start`, measuring every frame it meets.
pub fn walk(data: &[u8], start: usize) -> Walk {
    let mut blocks = Vec::new();
    let mut offset = start;

    loop {
        let Some(&introducer) = data.get(offset) else {
            return Walk {
                blocks,
                end: offset,
                stop: Stop::Truncated { at: offset },
            };
        };

        match introducer {
            TRAILER => {
                blocks.push(Block {
                    range: offset..offset + 1,
                    kind: Kind::Trailer,
                });
                return Walk {
                    blocks,
                    end: offset + 1,
                    stop: Stop::Trailer,
                };
            }
            EXTENSION_INTRODUCER => {
                let Some(&label) = data.get(offset + 1) else {
                    return Walk {
                        blocks,
                        end: offset,
                        stop: Stop::Truncated { at: offset },
                    };
                };
                let Some(end) = sub_block_chain_end(data, offset + 2) else {
                    return Walk {
                        blocks,
                        end: offset,
                        stop: Stop::Truncated { at: offset },
                    };
                };
                blocks.push(Block {
                    range: offset..end,
                    kind: Kind::Extension { label },
                });
                offset = end;
            }
            IMAGE_SEPARATOR => {
                let Some((frame, end)) = read_image(data, offset) else {
                    return Walk {
                        blocks,
                        end: offset,
                        stop: Stop::Truncated { at: offset },
                    };
                };
                blocks.push(Block {
                    range: offset..end,
                    kind: Kind::Image(frame),
                });
                offset = end;
            }
            _ => {
                return Walk {
                    blocks,
                    end: offset,
                    stop: Stop::Corrupt { at: offset },
                };
            }
        }
    }
}

/// An image block's fixed part: everything before the compressed data.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Descriptor {
    pub left: u16,
    pub top: u16,
    pub width: u16,
    pub height: u16,
    pub packed: u8,
    pub min_code_size: u8,
    /// Offset of the first sub-block length byte.
    pub data_start: usize,
}

/// Reads an image block's descriptor at its separator byte, rejecting anything that could not
/// be one. Used both by the walk and by the scan that looks for frames in a headerless file,
/// which is why the sanity checks live here rather than at the call sites.
pub fn read_descriptor(data: &[u8], offset: usize) -> Option<Descriptor> {
    if data.get(offset).copied() != Some(IMAGE_SEPARATOR) {
        return None;
    }
    let width = read_u16_le(data, offset + 5)?;
    let height = read_u16_le(data, offset + 7)?;
    if width == 0 || height == 0 {
        return None;
    }
    let packed = *data.get(offset + 9)?;
    let data_start = offset + 10 + color_table_bytes(packed);
    let min_code_size = *data.get(data_start)?;
    if !(2..=11).contains(&min_code_size) {
        return None;
    }

    Some(Descriptor {
        left: read_u16_le(data, offset + 1)?,
        top: read_u16_le(data, offset + 3)?,
        width,
        height,
        packed,
        min_code_size,
        data_start,
    })
}

fn measure_frame(data: &[u8], descriptor: &Descriptor) -> Frame {
    let compressed = collect_sub_blocks(data, descriptor.data_start + 1);
    let measured = measure_lzw(&compressed, descriptor.min_code_size);
    Frame {
        left: descriptor.left,
        top: descriptor.top,
        width: descriptor.width,
        height: descriptor.height,
        pixels: measured.pixels,
        ended_cleanly: measured.ended_cleanly,
        consumed: measured.consumed,
    }
}

/// Reads a whole image block starting at its separator byte. `None` means the file ends
/// inside it.
pub fn read_image(data: &[u8], offset: usize) -> Option<(Frame, usize)> {
    let descriptor = read_descriptor(data, offset)?;
    let end = sub_block_chain_end(data, descriptor.data_start + 1)?;
    Some((measure_frame(data, &descriptor), end))
}

/// An image block the file ends inside: what its codes still decoded to, and where the
/// compressed data can be closed off.
#[derive(Debug, Clone, Copy)]
pub struct PartialImage {
    pub frame: Frame,
    /// First offset past the last complete sub-block.
    pub cut: usize,
}

pub fn read_partial_image(data: &[u8], offset: usize) -> Option<PartialImage> {
    let descriptor = read_descriptor(data, offset)?;
    let frame = measure_frame(data, &descriptor);
    Some(PartialImage {
        cut: chain_cut_after(data, descriptor.data_start + 1, frame.consumed),
        frame,
    })
}

/// End of the sub-block that holds the `payload_bytes`-th byte of a chain's payload.
///
/// Cutting here keeps every byte the decoder made sense of, while ending on a sub-block
/// boundary so a zero byte written afterwards terminates the chain the way a writer would.
pub fn chain_cut_after(data: &[u8], start: usize, payload_bytes: usize) -> usize {
    let mut offset = start;
    let mut seen = 0usize;
    while let Some(&length) = data.get(offset) {
        if length == 0 {
            return offset + 1;
        }
        let next = offset + 1 + length as usize;
        if next > data.len() {
            return offset;
        }
        seen += length as usize;
        offset = next;
        if seen >= payload_bytes {
            return offset;
        }
    }
    offset
}

/// First offset past a sub-block chain, or `None` when the chain runs off the end.
pub fn sub_block_chain_end(data: &[u8], start: usize) -> Option<usize> {
    let mut offset = start;
    loop {
        let length = *data.get(offset)? as usize;
        if length == 0 {
            return Some(offset + 1);
        }
        offset += 1 + length;
        if offset > data.len() {
            return None;
        }
    }
}

/// First offset past the last *complete* sub-block of a chain that runs off the end.
///
/// This is where a truncated frame can be closed off: everything before it is whole compressed
/// data, and a zero byte written here terminates the chain the way the writer would have.
pub fn sub_block_chain_available_end(data: &[u8], start: usize) -> usize {
    let mut offset = start;
    while let Some(&length) = data.get(offset) {
        if length == 0 {
            return offset + 1;
        }
        let next = offset + 1 + length as usize;
        if next > data.len() {
            return offset;
        }
        offset = next;
    }
    offset
}

/// Concatenates a sub-block chain's payloads, stopping at whatever is present.
pub fn collect_sub_blocks(data: &[u8], start: usize) -> Vec<u8> {
    let mut out = Vec::new();
    let mut offset = start;
    while let Some(&length) = data.get(offset) {
        if length == 0 {
            break;
        }
        let payload = offset + 1..offset + 1 + length as usize;
        match data.get(payload.clone()) {
            Some(bytes) => out.extend_from_slice(bytes),
            None => {
                // A truncated final sub-block: whatever arrived is still real compressed data.
                if let Some(bytes) = data.get(offset + 1..) {
                    out.extend_from_slice(bytes);
                }
                break;
            }
        }
        offset = payload.end;
    }
    out
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Measured {
    pub pixels: u64,
    pub ended_cleanly: bool,
    /// Whole bytes of input the decoder consumed before it stopped.
    pub consumed: usize,
}

/// Counts the pixels an LZW stream expands to, without expanding it.
///
/// Every dictionary entry is one byte longer than the entry that spawned it, so tracking
/// lengths alone reproduces the pixel count exactly. A code that refers to an entry beyond the
/// dictionary is impossible in a healthy stream and ends the measurement - that is the check
/// that makes a frame's correctness provable.
pub fn measure_lzw(data: &[u8], min_code_size: u8) -> Measured {
    let clear_code = 1u16 << min_code_size;
    let end_code = clear_code + 1;

    let mut lengths: Vec<u32> = vec![1; usize::from(clear_code) + 2];
    let mut code_size = min_code_size + 1;
    let mut previous: Option<u16> = None;
    let mut pixels = 0u64;
    let mut consumed = 0usize;
    let mut bits = BitReader::new(data);

    loop {
        let Some(code) = bits.read(code_size) else {
            return Measured {
                pixels,
                ended_cleanly: false,
                consumed,
            };
        };

        if code == clear_code {
            lengths.truncate(usize::from(clear_code) + 2);
            code_size = min_code_size + 1;
            previous = None;
            consumed = bits.whole_bytes_read();
            continue;
        }
        if code == end_code {
            return Measured {
                pixels,
                ended_cleanly: true,
                consumed: bits.whole_bytes_read(),
            };
        }

        let next_free = lengths.len();
        let length = match lengths.get(usize::from(code)) {
            Some(length) => *length,
            // The one legal forward reference: a code for the entry about to be created.
            None if usize::from(code) == next_free => match previous.and_then(|p| lengths.get(usize::from(p))) {
                Some(length) => length + 1,
                None => {
                    return Measured {
                        pixels,
                        ended_cleanly: false,
                        consumed,
                    };
                }
            },
            None => {
                return Measured {
                    pixels,
                    ended_cleanly: false,
                    consumed,
                };
            }
        };
        pixels += u64::from(length);
        consumed = bits.whole_bytes_read();

        if let Some(previous) = previous
            && lengths.len() < usize::from(MAX_CODE)
        {
            let grown = lengths.get(usize::from(previous)).copied().unwrap_or(1) + 1;
            lengths.push(grown);
            if lengths.len() == 1usize << code_size && code_size < 12 {
                code_size += 1;
            }
        }
        previous = Some(code);
    }
}

/// Least-significant-bit-first reader, which is the order GIF packs its codes in.
struct BitReader<'a> {
    data: &'a [u8],
    bit: usize,
}

impl<'a> BitReader<'a> {
    fn new(data: &'a [u8]) -> Self {
        Self { data, bit: 0 }
    }

    /// Bytes fully behind the read position, which is where a cut can safely fall.
    fn whole_bytes_read(&self) -> usize {
        self.bit / 8
    }

    fn read(&mut self, bits: u8) -> Option<u16> {
        if self.bit + usize::from(bits) > self.data.len() * 8 {
            return None;
        }
        let mut value = 0u32;
        for index in 0..usize::from(bits) {
            let position = self.bit + index;
            let byte = *self.data.get(position / 8)?;
            value |= u32::from((byte >> (position % 8)) & 1) << index;
        }
        self.bit += usize::from(bits);
        u16::try_from(value).ok()
    }
}

/// Encodes `pixels` as an LZW stream that never compresses: every pixel gets its own code,
/// with a clear code emitted often enough that the dictionary never grows and the code width
/// never changes. That is a legal stream, and it lets the tests build real GIFs without
/// carrying a compressor that nothing in the repair path needs.
#[cfg(test)]
pub fn encode_uncompressed(pixels: &[u8], min_code_size: u8) -> Vec<u8> {
    let clear_code = 1u16 << min_code_size;
    let mut writer = BitWriter::default();
    writer.write(clear_code, min_code_size + 1);
    let mut since_clear = 0;
    for &pixel in pixels {
        writer.write(u16::from(pixel), min_code_size + 1);
        since_clear += 1;
        if since_clear == usize::from(clear_code) - 2 {
            writer.write(clear_code, min_code_size + 1);
            since_clear = 0;
        }
    }
    writer.write(clear_code + 1, min_code_size + 1);
    writer.finish()
}

#[cfg(test)]
#[derive(Default)]
struct BitWriter {
    out: Vec<u8>,
    current: u32,
    bits: u8,
}

#[cfg(test)]
impl BitWriter {
    fn write(&mut self, code: u16, size: u8) {
        self.current |= u32::from(code) << self.bits;
        self.bits += size;
        while self.bits >= 8 {
            self.out.push((self.current & 0xFF) as u8);
            self.current >>= 8;
            self.bits -= 8;
        }
    }

    fn finish(mut self) -> Vec<u8> {
        if self.bits > 0 {
            self.out.push((self.current & 0xFF) as u8);
        }
        self.out
    }
}

/// Wraps bytes in the length-prefixed sub-block chain every GIF payload is carried in.
#[cfg(test)]
pub fn sub_blocks(data: &[u8]) -> Vec<u8> {
    let mut out = Vec::new();
    for chunk in data.chunks(255) {
        out.push(chunk.len() as u8);
        out.extend_from_slice(chunk);
    }
    out.push(0);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_stream_of_single_pixel_codes_measures_exactly() {
        let pixels: Vec<u8> = (0..300).map(|i| (i % 4) as u8).collect();
        let encoded = encode_uncompressed(&pixels, 2);
        let measured = measure_lzw(&encoded, 2);

        assert_eq!(measured.pixels, 300);
        assert!(measured.ended_cleanly);
    }

    #[test]
    fn a_stream_that_stops_early_is_reported_as_such() {
        let pixels: Vec<u8> = (0..300).map(|i| (i % 4) as u8).collect();
        let mut encoded = encode_uncompressed(&pixels, 2);
        encoded.truncate(encoded.len() / 2);
        let measured = measure_lzw(&encoded, 2);

        assert!(!measured.ended_cleanly);
        assert!(measured.pixels > 0 && measured.pixels < 300, "{}", measured.pixels);
    }

    /// The property the whole approach rests on: a code that cannot exist yet is refused
    /// instead of being expanded into plausible-looking pixels.
    #[test]
    fn an_impossible_dictionary_reference_ends_the_measurement() {
        // min code size 2: codes 0..=3 are pixels, 4 clears, 5 ends, 6 is the first entry the
        // dictionary can create. A 7 before anything has been added cannot be legal.
        let mut writer = BitWriter::default();
        writer.write(4, 3);
        writer.write(7, 3);
        let measured = measure_lzw(&writer.finish(), 2);

        assert!(!measured.ended_cleanly);
        assert_eq!(measured.pixels, 0);
    }

    #[test]
    fn a_sub_block_chain_ending_off_the_end_is_refused() {
        assert_eq!(sub_block_chain_end(&[0], 0), Some(1));
        assert_eq!(sub_block_chain_end(&[2, 0xAA, 0xBB, 0], 0), Some(4));
        assert_eq!(sub_block_chain_end(&[5, 0xAA], 0), None);
        assert_eq!(sub_block_chain_end(&[], 0), None);
    }

    #[test]
    fn a_truncated_sub_block_still_yields_the_bytes_that_arrived() {
        assert_eq!(collect_sub_blocks(&[3, 1, 2, 3, 0], 0), vec![1, 2, 3]);
        assert_eq!(collect_sub_blocks(&[9, 1, 2, 3], 0), vec![1, 2, 3]);
    }

    #[test]
    fn a_colour_table_size_follows_the_packed_field() {
        assert_eq!(color_table_bytes(0x00), 0);
        assert_eq!(color_table_bytes(0x80), 6, "a two-entry table");
        assert_eq!(color_table_bytes(0x87), 768, "the full 256-entry table");
    }
}
