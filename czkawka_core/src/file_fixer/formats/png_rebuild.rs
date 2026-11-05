//! Rebuilding a PNG whose header is gone.
//!
//! `IHDR` holds the geometry, and it sits in the first thirty-odd bytes of the file - exactly
//! the region a damaged first sector destroys. Without it the pixel data is intact but nothing
//! says how wide a row is.
//!
//! Guessing is not needed, because the decompressed data constrains the answer sharply. Every
//! PNG row is preceded by a filter byte in `0..=4`, so a candidate row stride is only viable
//! when it divides the decompressed length *and* every byte at a multiple of that stride is a
//! valid filter code. Over a few hundred rows that rules out essentially everything wrong.

use std::ops::Range;

use png::{BitDepth, ColorType};

use super::png_chunks::{self, ChunkStatus, SIGNATURE};
use super::png_image;
use crate::file_fixer::candidate::{Candidate, Fidelity, Payload, Score};
use crate::file_fixer::engine::{RepairContext, Strategy};
use crate::file_fixer::error::Result;
use crate::file_fixer::io::read_u32_be;
use crate::file_fixer::mode::Capabilities;

/// Colour layouts to try, most common first. Indexed images are excluded: they need a palette,
/// which lives in another chunk that the same damage usually takes with it.
const LAYOUTS: [(ColorType, u8); 5] = [
    (ColorType::Rgba, 4),
    (ColorType::Rgb, 3),
    (ColorType::Grayscale, 1),
    (ColorType::GrayscaleAlpha, 2),
    (ColorType::Indexed, 1),
];

const DEPTHS: [BitDepth; 5] = [BitDepth::Eight, BitDepth::Sixteen, BitDepth::Four, BitDepth::Two, BitDepth::One];

/// Highest filter code the specification defines.
const MAX_FILTER: u8 = 4;

/// Ceiling on how many geometries are worth building and decoding.
const MAX_CANDIDATES: usize = 16;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Geometry {
    pub width: u32,
    pub height: u32,
    pub color_type: ColorType,
    pub bit_depth: BitDepth,
}

/// Locates the compressed pixel data even when the chunk chain cannot be walked.
///
/// Each `IDAT` is found by its four-character code and confirmed by its own checksum, so a
/// destroyed header costs nothing: the chunks after it describe themselves.
pub fn recover_pixel_chunks(data: &[u8]) -> Vec<Range<usize>> {
    if let Some(from_walk) = walk_pixel_chunks(data) {
        return from_walk;
    }

    let mut chunks = Vec::new();
    let mut offset = 4;
    while offset + 8 <= data.len() {
        let Some(position) = find_code(data, offset, *b"IDAT") else {
            break;
        };
        let start = position - 4;
        match verified_payload(data, start) {
            Some(payload) => {
                offset = payload.end + 4;
                chunks.push(payload);
            }
            None => offset = position + 1,
        }
    }
    chunks
}

fn walk_pixel_chunks(data: &[u8]) -> Option<Vec<Range<usize>>> {
    if !data.starts_with(&SIGNATURE) {
        return None;
    }
    let chunks: Vec<Range<usize>> = png_chunks::walk(data)
        .into_iter()
        .filter(|chunk| chunk.is(*b"IDAT") && chunk.status == ChunkStatus::Valid)
        .map(|chunk| chunk.data)
        .collect();
    (!chunks.is_empty()).then_some(chunks)
}

fn find_code(data: &[u8], from: usize, code: [u8; 4]) -> Option<usize> {
    data.get(from..)?.windows(4).position(|window| window == code).map(|position| from + position)
}

/// Confirms a chunk at `start` by its checksum and returns its payload bounds.
fn verified_payload(data: &[u8], start: usize) -> Option<Range<usize>> {
    let length = read_u32_be(data, start)? as usize;
    let payload = start + 8..start.checked_add(8)?.checked_add(length)?;
    let stored = read_u32_be(data, payload.end)?;
    let covered = data.get(start + 4..payload.end)?;

    (crc32fast::hash(covered) == stored).then_some(payload)
}

/// The geometry an intact `IHDR` states, when one survived.
pub fn declared_geometry(data: &[u8]) -> Option<Geometry> {
    let start = find_code(data, 0, *b"IHDR")?.checked_sub(4)?;
    let payload = verified_payload(data, start)?;

    Some(Geometry {
        width: read_u32_be(data, payload.start)?,
        height: read_u32_be(data, payload.start + 4)?,
        bit_depth: depth_from(*data.get(payload.start + 8)?)?,
        color_type: colour_from(*data.get(payload.start + 9)?)?,
    })
}

fn depth_from(value: u8) -> Option<BitDepth> {
    DEPTHS.into_iter().find(|depth| *depth as u8 == value)
}

fn colour_from(value: u8) -> Option<ColorType> {
    LAYOUTS.into_iter().map(|(colour, _)| colour).find(|c| *c as u8 == value)
}

/// Decompresses the pixel data, keeping whatever came out when it fails partway.
pub fn inflate(data: &[u8], chunks: &[Range<usize>], limit: usize) -> Vec<u8> {
    let mut stream = Vec::new();
    for chunk in chunks {
        if let Some(payload) = data.get(chunk.clone()) {
            stream.extend_from_slice(payload);
        }
    }

    // Not `decompress_to_vec_zlib_with_limit`: on failure it hands back its working buffer
    // rather than the bytes it produced, so a partial decode arrives padded with zeros that
    // would be read here as blank pixel rows. See [`super::deflate`].
    super::deflate::inflate(&stream, limit, true).data
}

/// Geometries consistent with the decompressed data, most plausible first.
///
/// A stride must divide the data exactly and put a valid filter code at the start of every row.
/// What survives is then matched against each colour layout to recover a width. Strides are
/// tried from the smallest up, so the answer checked against the most rows comes first.
pub fn candidate_geometries(inflated: &[u8]) -> Vec<Geometry> {
    let total = inflated.len();
    if total == 0 {
        return Vec::new();
    }

    let mut candidates = Vec::new();
    for stride in divisors(total) {
        if stride < 2 || !filter_codes_are_valid(inflated, stride) {
            continue;
        }
        let height = (total / stride) as u32;
        let row_bytes = stride - 1;
        // A single row makes the filter check meaningless: one byte in `0..=4` proves nothing.
        if height < 2 || height > u32::from(u16::MAX) {
            continue;
        }

        for (color_type, channels) in LAYOUTS {
            for bit_depth in DEPTHS {
                if let Some(width) = width_for(row_bytes, channels, bit_depth) {
                    candidates.push(Geometry {
                        width,
                        height,
                        color_type,
                        bit_depth,
                    });
                }
            }
        }
        if candidates.len() >= MAX_CANDIDATES {
            break;
        }
    }
    candidates.truncate(MAX_CANDIDATES);
    candidates
}

fn divisors(total: usize) -> Vec<usize> {
    let mut small = Vec::new();
    let mut large = Vec::new();
    let mut value = 1usize;

    while value * value <= total {
        if total.is_multiple_of(value) {
            small.push(value);
            if value != total / value {
                large.push(total / value);
            }
        }
        value += 1;
    }
    // Smallest stride first, because the filter check gets stronger the more rows it covers:
    // a stride that survives over hundreds of rows is almost certainly the real one, while the
    // largest strides pass trivially by covering a single row.
    large.reverse();
    small.extend(large);
    small
}

fn filter_codes_are_valid(inflated: &[u8], stride: usize) -> bool {
    inflated.iter().step_by(stride).all(|&filter| filter <= MAX_FILTER)
}

fn width_for(row_bytes: usize, channels: u8, bit_depth: BitDepth) -> Option<u32> {
    let bits = usize::from(channels) * usize::from(bit_depth as u8);
    let width = row_bytes.checked_mul(8)? / bits.max(1);
    (width > 0 && (width * bits).div_ceil(8) == row_bytes).then(|| u32::try_from(width).ok())?
}

/// Writes a PNG with the given geometry around the original compressed data.
///
/// The pixel data is copied across untouched rather than recompressed, so nothing about the
/// image changes; only the description of it is new.
pub fn assemble(data: &[u8], chunks: &[Range<usize>], geometry: Geometry) -> Vec<u8> {
    let mut out = Vec::from(SIGNATURE);

    let mut header = Vec::with_capacity(13);
    header.extend_from_slice(&geometry.width.to_be_bytes());
    header.extend_from_slice(&geometry.height.to_be_bytes());
    header.push(geometry.bit_depth as u8);
    header.push(geometry.color_type as u8);
    header.extend_from_slice(&[0, 0, 0]); // compression, filter and interlace methods
    write_chunk(&mut out, *b"IHDR", &header);

    for chunk in chunks {
        if let Some(payload) = data.get(chunk.clone()) {
            write_chunk(&mut out, *b"IDAT", payload);
        }
    }
    write_chunk(&mut out, *b"IEND", &[]);
    out
}

fn write_chunk(out: &mut Vec<u8>, kind: [u8; 4], payload: &[u8]) {
    out.extend_from_slice(&(payload.len() as u32).to_be_bytes());
    out.extend_from_slice(&kind);
    out.extend_from_slice(payload);

    let mut covered = Vec::with_capacity(4 + payload.len());
    covered.extend_from_slice(&kind);
    covered.extend_from_slice(payload);
    out.extend_from_slice(&crc32fast::hash(&covered).to_be_bytes());
}

/// The file's own header survived but the signature in front of it did not.
///
/// Nothing is inferred here: the geometry is what the file states, confirmed by its checksum,
/// so this is a structural repair and every mode may do it.
pub struct RestoreSignature;

impl Strategy for RestoreSignature {
    fn name(&self) -> &str {
        "png_restore_signature"
    }

    fn run(&self, ctx: &RepairContext<'_>) -> Result<Option<Candidate>> {
        if ctx.data.starts_with(&SIGNATURE) {
            return Ok(None);
        }
        let Some(geometry) = declared_geometry(ctx.data) else {
            return Ok(None);
        };
        let chunks = recover_pixel_chunks(ctx.data);
        if chunks.is_empty() {
            return Ok(None);
        }

        let rebuilt = assemble(ctx.data, &chunks, geometry);
        let Some(rows) = decoded_rows(ctx, &rebuilt) else {
            return Ok(None);
        };

        Ok(Some(Candidate {
            strategy: self.name().to_string(),
            data: Payload::Bytes(rebuilt),
            score: score_for(rows, geometry.height),
            fidelity: Fidelity::Lossless,
            reconstructed: Vec::new(),
        }))
    }
}

/// The header is gone; work the geometry out from the pixel data instead.
pub struct RebuildHeader;

impl Strategy for RebuildHeader {
    fn name(&self) -> &str {
        "png_rebuild_header"
    }

    fn requires(&self) -> Capabilities {
        Capabilities {
            guess: true,
            ..Capabilities::NONE
        }
    }

    fn run(&self, ctx: &RepairContext<'_>) -> Result<Option<Candidate>> {
        if declared_geometry(ctx.data).is_some() {
            return Ok(None);
        }
        let chunks = recover_pixel_chunks(ctx.data);
        if chunks.is_empty() {
            return Ok(None);
        }

        let limit = usize::try_from(ctx.allocation_limit).unwrap_or(usize::MAX);
        let inflated = inflate(ctx.data, &chunks, limit);
        let mut best: Option<(Vec<u8>, u32, u32)> = None;

        for geometry in candidate_geometries(&inflated) {
            if ctx.should_stop() {
                break;
            }
            let rebuilt = assemble(ctx.data, &chunks, geometry);
            let Some(rows) = decoded_rows(ctx, &rebuilt) else {
                continue;
            };
            if best.as_ref().is_none_or(|(_, best_rows, _)| rows > *best_rows) {
                best = Some((rebuilt, rows, geometry.height));
            }
        }

        Ok(best.map(|(rebuilt, rows, height)| Candidate {
            strategy: self.name().to_string(),
            data: Payload::Bytes(rebuilt),
            score: score_for(rows, height),
            fidelity: Fidelity::Reconstructed,
            // The pixels are the original ones, but only if the geometry was guessed right,
            // and nothing here can prove that beyond the file decoding cleanly.
            reconstructed: std::iter::once(0..u64::from(rows)).collect(),
        }))
    }
}

fn decoded_rows(ctx: &RepairContext<'_>, rebuilt: &[u8]) -> Option<u32> {
    let decoded = png_image::decode_rows(ctx, rebuilt, true)?;
    (decoded.rows_recovered > 0).then_some(decoded.rows_recovered)
}

fn score_for(rows: u32, height: u32) -> Score {
    Score {
        content_recovered: u64::from(rows),
        content_total: Some(u64::from(height)),
        decoder_errors: 0,
        decodes_cleanly: true,
        streams_complete: true,
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use png::Encoder;

    use super::*;

    fn sample_png(width: u32, height: u32, color: ColorType, channels: u32) -> Vec<u8> {
        let pixels: Vec<u8> = (0..width * height * channels).map(|i| (i % 251) as u8).collect();
        let mut out = Vec::new();
        let mut encoder = Encoder::new(Cursor::new(&mut out), width, height);
        encoder.set_color(color);
        encoder.set_depth(BitDepth::Eight);
        let mut writer = encoder.write_header().unwrap();
        writer.write_image_data(&pixels).unwrap();
        writer.finish().unwrap();
        out
    }

    fn wreck_header(data: &[u8]) -> Vec<u8> {
        let mut damaged = data.to_vec();
        damaged[..33].fill(0xAB);
        damaged
    }

    #[test]
    fn divisors_are_complete_and_ordered_from_the_smallest() {
        assert_eq!(divisors(12), vec![1, 2, 3, 4, 6, 12]);
        assert_eq!(divisors(16), vec![1, 2, 4, 8, 16]);
        assert_eq!(divisors(1), vec![1]);
    }

    #[test]
    fn a_width_only_fits_when_the_row_length_matches_exactly() {
        assert_eq!(width_for(40, 4, BitDepth::Eight), Some(10));
        assert_eq!(width_for(30, 3, BitDepth::Eight), Some(10));
        assert_eq!(width_for(2, 1, BitDepth::One), Some(16));
        assert_eq!(width_for(0, 4, BitDepth::Eight), None);
    }

    #[test]
    fn pixel_chunks_survive_a_destroyed_header() {
        let original = sample_png(64, 40, ColorType::Rgba, 4);
        let intact = recover_pixel_chunks(&original);
        assert!(!intact.is_empty());

        let damaged = wreck_header(&original);
        assert!(!damaged.starts_with(&SIGNATURE));
        assert_eq!(recover_pixel_chunks(&damaged), intact);
    }

    #[test]
    fn the_true_geometry_is_among_the_candidates() {
        for (width, height, color, channels) in [(64u32, 40u32, ColorType::Rgba, 4u32), (31, 17, ColorType::Rgb, 3), (16, 9, ColorType::Grayscale, 1)] {
            let original = sample_png(width, height, color, channels);
            let damaged = wreck_header(&original);
            let chunks = recover_pixel_chunks(&damaged);
            let inflated = inflate(&damaged, &chunks, 1 << 26);

            let candidates = candidate_geometries(&inflated);
            let expected = Geometry {
                width,
                height,
                color_type: color,
                bit_depth: BitDepth::Eight,
            };
            assert!(candidates.contains(&expected), "{width}x{height} {color:?} missing from {candidates:?}");
        }
    }

    #[test]
    fn an_assembled_file_decodes_to_the_original_pixels() {
        let original = sample_png(64, 40, ColorType::Rgba, 4);
        let damaged = wreck_header(&original);
        let chunks = recover_pixel_chunks(&damaged);
        let geometry = Geometry {
            width: 64,
            height: 40,
            color_type: ColorType::Rgba,
            bit_depth: BitDepth::Eight,
        };

        let rebuilt = assemble(&damaged, &chunks, geometry);
        let decode = |data: &[u8]| {
            let mut reader = png::Decoder::new(Cursor::new(data)).read_info().unwrap();
            let mut buffer = vec![0; reader.output_buffer_size().expect("png buffer size fits in memory")];
            let info = reader.next_frame(&mut buffer).unwrap();
            buffer.truncate(info.buffer_size());
            buffer
        };
        assert_eq!(decode(&rebuilt), decode(&original));
    }

    #[test]
    fn an_intact_header_is_read_rather_than_derived() {
        let original = sample_png(31, 17, ColorType::Rgb, 3);
        let geometry = declared_geometry(&original).expect("the header is intact");

        assert_eq!(geometry.width, 31);
        assert_eq!(geometry.height, 17);
        assert_eq!(geometry.color_type, ColorType::Rgb);
        assert!(declared_geometry(&wreck_header(&original)).is_none());
    }

    #[test]
    fn nothing_is_recovered_from_data_that_is_not_a_png() {
        assert!(recover_pixel_chunks(&[0u8; 512]).is_empty());
        assert!(candidate_geometries(&[]).is_empty());
        assert!(declared_geometry(&[0u8; 512]).is_none());
    }
}
