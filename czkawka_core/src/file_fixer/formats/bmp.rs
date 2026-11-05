//! BMP repair.
//!
//! BMP is the one format here with neither compression nor a checksum, which cuts both ways.
//! Every row sits at a computable offset, so whatever bytes survive are *exactly* the original
//! pixels - there is no decoder state to desynchronize, and even the truncation path is
//! byte-faithful. In exchange the format can prove nothing about its own content: a zeroed run
//! inside the pixel array is indistinguishable from a run of black pixels, so that damage is
//! reported healthy rather than guessed at (see the README).
//!
//! Two things damage actually does to a BMP: it makes the header disagree with the pixels that
//! follow (wrong sizes, a data offset pointing nowhere), or it cuts the pixel array short. Both
//! are repaired by rewriting the header to describe what is really there.
//!
//! The direction matters when rows are missing. A positive height means bottom-up storage,
//! which is the overwhelmingly common case, so the rows a truncated file kept are the *bottom*
//! of the picture and the missing ones are its top.

use crate::file_fixer::candidate::{Candidate, Fidelity, Payload, Score};
use crate::file_fixer::engine::{RepairContext, Repairer, Strategy, Support};
use crate::file_fixer::error::Result;
use crate::file_fixer::format::Format;
use crate::file_fixer::io::{put_u16_le, put_u32_le, read_i32_le, read_u16_le, read_u32_le};
use crate::file_fixer::mode::Capabilities;

pub struct BmpRepairer;

impl Repairer for BmpRepairer {
    fn format(&self) -> Format {
        Format::Bmp
    }

    fn supports(&self, data: &[u8]) -> Support {
        match parse(data) {
            Some(geometry) if !geometry.is_uncompressed() => Support::Unsupported(format!("BMP compression method {} is not handled", geometry.compression)),
            _ => Support::Supported,
        }
    }

    /// A compressed BMP produces no candidates at all. `supports` above only supplies the
    /// reason text: emptiness here is what the engine actually consults, and returning
    /// `Unsupported` while still handing back strategies would let one of them win anyway.
    fn strategies(&self, ctx: &RepairContext<'_>) -> Vec<Box<dyn Strategy>> {
        if parse(ctx.data).is_some_and(|geometry| !geometry.is_uncompressed()) {
            return Vec::new();
        }
        vec![Box::new(Verbatim), Box::new(FixHeader), Box::new(TruncateToCompleteRows), Box::new(RebuildHeader)]
    }
}

/// Where the pixel array starts in a file written with the usual 40-byte info header and no
/// palette. Nearly every 24- and 32-bit writer emits exactly this, which is what makes it a
/// usable assumption once the header itself is gone.
const CANONICAL_PIXEL_START: usize = FILE_HEADER_LEN + 40;

const FILE_HEADER_LEN: usize = 14;

/// Header sizes the specification defines. Anything else means this is not a BMP header, and
/// accepting it would let the geometry fields be read out of arbitrary bytes.
const DIB_SIZES: [u32; 7] = [12, 40, 52, 56, 64, 108, 124];

const BITS_PER_PIXEL: [u16; 6] = [1, 4, 8, 16, 24, 32];

/// Guards against a damaged header claiming a picture larger than any real one.
const MAX_DIMENSION: u32 = 1 << 18;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Geometry {
    pub width: u32,
    /// Signed exactly as stored: negative means the rows are top-down.
    pub height: i32,
    pub bits_per_pixel: u16,
    pub compression: u32,
    pub dib_size: u32,
    pub declared_file_size: u32,
    /// Where the pixel array begins, taken from the header when it is plausible and computed
    /// from the header sizes when it is not.
    pub pixel_start: usize,
}

impl Geometry {
    /// Row length in bytes, including the padding to a four-byte boundary that BMP requires.
    pub fn stride(&self) -> u64 {
        (u64::from(self.width) * u64::from(self.bits_per_pixel)).div_ceil(32) * 4
    }

    pub fn rows(&self) -> u32 {
        self.height.unsigned_abs()
    }

    pub fn is_top_down(&self) -> bool {
        self.height < 0
    }

    /// BI_RGB and BI_BITFIELDS store raw rows; the run-length and embedded-image methods do
    /// not, and nothing else here would be true of them.
    pub fn is_uncompressed(&self) -> bool {
        matches!(self.compression, 0 | 3 | 6 | 11)
    }

    fn pixel_bytes(&self) -> u64 {
        self.stride() * u64::from(self.rows())
    }

    /// Rows the file actually carries, which is fewer than `rows()` when it was cut short.
    fn rows_present(&self, len: usize) -> u32 {
        let available = (len as u64).saturating_sub(self.pixel_start as u64);
        let stride = self.stride();
        if stride == 0 {
            return 0;
        }
        u32::try_from(available / stride).unwrap_or(u32::MAX).min(self.rows())
    }
}

/// Reads the geometry out of a file whose header is still intact enough to believe.
pub fn parse(data: &[u8]) -> Option<Geometry> {
    if !data.starts_with(b"BM") {
        return None;
    }
    let declared_file_size = read_u32_le(data, 2)?;
    let declared_start = read_u32_le(data, 10)?;
    let dib_size = read_u32_le(data, 14)?;
    if !DIB_SIZES.contains(&dib_size) {
        return None;
    }

    // The 12-byte core header predates every later field and packs its geometry into halves.
    let (width, height, bits_per_pixel, compression) = if dib_size == 12 {
        (u32::from(read_u16_le(data, 18)?), i32::from(read_u16_le(data, 20)? as i16), read_u16_le(data, 24)?, 0)
    } else {
        (
            u32::try_from(read_i32_le(data, 18)?).ok()?,
            read_i32_le(data, 22)?,
            read_u16_le(data, 28)?,
            read_u32_le(data, 30)?,
        )
    };

    if width == 0 || width > MAX_DIMENSION || height == 0 || height.unsigned_abs() > MAX_DIMENSION {
        return None;
    }
    if !BITS_PER_PIXEL.contains(&bits_per_pixel) {
        return None;
    }

    let computed_start = computed_pixel_start(data, dib_size, bits_per_pixel, compression);
    let declared_start = declared_start as usize;
    let pixel_start = if declared_start >= FILE_HEADER_LEN + dib_size as usize && declared_start < data.len() {
        declared_start
    } else {
        computed_start
    };

    Some(Geometry {
        width,
        height,
        bits_per_pixel,
        compression,
        dib_size,
        declared_file_size,
        pixel_start,
    })
}

/// Where the pixel array must begin given the header sizes, used when the stored offset is
/// not believable. A palette (at most 8 bits per pixel) or the three bit-field masks sit
/// between the header and the rows.
fn computed_pixel_start(data: &[u8], dib_size: u32, bits_per_pixel: u16, compression: u32) -> usize {
    let entry_size = if dib_size == 12 { 3 } else { 4 };
    let extra = if bits_per_pixel <= 8 {
        let declared = read_u32_le(data, 46).unwrap_or(0);
        let entries = if dib_size == 12 || declared == 0 || declared > 256 {
            1u32 << bits_per_pixel
        } else {
            declared
        };
        entries as usize * entry_size
    } else if compression == 3 {
        12
    } else {
        0
    };
    FILE_HEADER_LEN + dib_size as usize + extra
}

/// True when the header and the bytes present agree with each other.
///
/// A declared file size *smaller* than the file is tolerated the way AVI tolerates footer
/// noise: some writers pad. A larger one means bytes are missing.
fn looks_whole(data: &[u8], geometry: &Geometry) -> bool {
    let end = geometry.pixel_start as u64 + geometry.pixel_bytes();
    end <= data.len() as u64 && u64::from(geometry.declared_file_size) <= data.len() as u64
}

/// True when the header, whatever its size fields say, still points a viewer at the right
/// bytes.
///
/// The file-size field is deliberately not required to match. Real writers leave it at zero
/// and real files carry appended junk; no decoder reads it, and treating that as damage would
/// mean handing back a "repaired" copy of a picture that was never broken.
fn header_is_usable(data: &[u8], geometry: &Geometry) -> bool {
    let expected_start = computed_pixel_start(data, geometry.dib_size, geometry.bits_per_pixel, geometry.compression);
    let stored_image_size = read_u32_le(data, 34).unwrap_or(0);
    let image_size_agrees = geometry.dib_size < 40 || stored_image_size == 0 || u64::from(stored_image_size) == geometry.pixel_bytes();

    read_u32_le(data, 10).is_some_and(|declared| declared as usize == expected_start) && u64::from(geometry.declared_file_size) <= data.len() as u64 && image_size_agrees
}

/// Rebuilds the file from the header bytes that survived plus the rows that are present,
/// patching every size field to describe what is actually there.
///
/// The original header region is copied rather than regenerated: it carries the palette, the
/// bit-field masks and the colour-space block, none of which can be derived from anything else.
fn assemble(data: &[u8], geometry: &Geometry, rows: u32) -> Option<Vec<u8>> {
    let pixels = data.get(geometry.pixel_start..)?;
    let kept = usize::try_from(geometry.stride() * u64::from(rows)).ok()?.min(pixels.len());

    let mut out = data.get(..geometry.pixel_start)?.to_vec();
    let header_len = out.len();
    put_u32_le(&mut out, 2, u32::try_from(header_len + kept).unwrap_or(u32::MAX));
    put_u32_le(&mut out, 10, u32::try_from(header_len).unwrap_or(u32::MAX));

    let height = i64::from(rows) * if geometry.is_top_down() { -1 } else { 1 };
    if geometry.dib_size == 12 {
        put_u16_le(&mut out, 20, u16::try_from(rows).unwrap_or(u16::MAX));
    } else {
        put_u32_le(&mut out, 22, i32::try_from(height).unwrap_or(i32::MAX) as u32);
        put_u32_le(&mut out, 34, u32::try_from(kept).unwrap_or(u32::MAX));
    }

    out.extend_from_slice(pixels.get(..kept)?);
    Some(out)
}

fn complete_score(rows: u32) -> Score {
    Score {
        content_recovered: u64::from(rows),
        content_total: Some(u64::from(rows)),
        decoder_errors: 0,
        decodes_cleanly: true,
        streams_complete: true,
    }
}

/// The header describes the file and every row is present; hand the bytes back untouched.
struct Verbatim;

impl Strategy for Verbatim {
    fn name(&self) -> &str {
        "bmp_verbatim"
    }

    fn run(&self, ctx: &RepairContext<'_>) -> Result<Option<Candidate>> {
        let Some(geometry) = parse(ctx.data) else {
            return Ok(None);
        };
        if !geometry.is_uncompressed() || !looks_whole(ctx.data, &geometry) || !header_is_usable(ctx.data, &geometry) {
            return Ok(None);
        }

        Ok(Some(Candidate {
            strategy: self.name().to_string(),
            data: Payload::Bytes(ctx.data.to_vec()),
            score: complete_score(geometry.rows()),
            fidelity: Fidelity::Lossless,
            reconstructed: Vec::new(),
        }))
    }
}

/// Every row is there but the header points a viewer somewhere else: a scrambled data offset,
/// a stated image size that contradicts the geometry, a file size claiming bytes that are not
/// present. Nothing is discarded, so this needs no capability and works in safe mode.
struct FixHeader;

impl Strategy for FixHeader {
    fn name(&self) -> &str {
        "bmp_fix_header"
    }

    fn run(&self, ctx: &RepairContext<'_>) -> Result<Option<Candidate>> {
        let Some(geometry) = parse(ctx.data) else {
            return Ok(None);
        };
        if !geometry.is_uncompressed() || !looks_whole(ctx.data, &geometry) || header_is_usable(ctx.data, &geometry) {
            return Ok(None);
        }
        let Some(fixed) = assemble(ctx.data, &geometry, geometry.rows()) else {
            return Ok(None);
        };

        Ok(Some(Candidate {
            strategy: self.name().to_string(),
            data: Payload::Bytes(fixed),
            score: complete_score(geometry.rows()),
            fidelity: Fidelity::Lossless,
            reconstructed: Vec::new(),
        }))
    }
}

/// The pixel array stops early. Keep the rows that are complete and say so in the height.
///
/// A partially written final row is dropped: its remaining bytes are original, but a viewer
/// would read past the end of the file to show it.
struct TruncateToCompleteRows;

impl Strategy for TruncateToCompleteRows {
    fn name(&self) -> &str {
        "bmp_truncate"
    }

    fn requires(&self) -> Capabilities {
        Capabilities {
            truncate: true,
            ..Capabilities::NONE
        }
    }

    fn run(&self, ctx: &RepairContext<'_>) -> Result<Option<Candidate>> {
        let Some(geometry) = parse(ctx.data) else {
            return Ok(None);
        };
        if !geometry.is_uncompressed() {
            return Ok(None);
        }
        let rows = geometry.rows_present(ctx.data.len());
        if rows == 0 || rows == geometry.rows() {
            return Ok(None);
        }
        let Some(fixed) = assemble(ctx.data, &geometry, rows) else {
            return Ok(None);
        };

        Ok(Some(Candidate {
            strategy: self.name().to_string(),
            data: Payload::Bytes(fixed),
            score: Score {
                content_recovered: u64::from(rows),
                content_total: Some(u64::from(geometry.rows())),
                decoder_errors: 0,
                decodes_cleanly: true,
                streams_complete: false,
            },
            fidelity: Fidelity::Lossless,
            reconstructed: Vec::new(),
        }))
    }
}

/// The header is gone and with it the only statement of how wide a row is.
///
/// The pixels themselves are untouched, so the question is purely geometric, and a picture
/// constrains it: at the right width consecutive rows resemble each other, at any other width
/// the rows are arbitrary slices through the image and neighbouring bytes stop agreeing.
/// Trying every width that divides the surviving payload exactly and keeping the one with the
/// smoothest vertical transition is the same idea as PNG's filter-byte sieve, only statistical
/// - which is why the result is reported as reconstructed even though every byte is original.
struct RebuildHeader;

impl Strategy for RebuildHeader {
    fn name(&self) -> &str {
        "bmp_rebuild_header"
    }

    fn requires(&self) -> Capabilities {
        Capabilities {
            guess: true,
            ..Capabilities::NONE
        }
    }

    fn run(&self, ctx: &RepairContext<'_>) -> Result<Option<Candidate>> {
        if parse(ctx.data).is_some_and(|geometry| looks_whole(ctx.data, &geometry)) {
            return Ok(None);
        }
        let Some(guess) = guess_geometry(ctx) else {
            return Ok(None);
        };

        let pixels = ctx.data.get(guess.pixel_start..).unwrap_or_default();
        let kept = usize::try_from(guess.stride() * u64::from(guess.rows())).unwrap_or(0);
        let Some(pixels) = pixels.get(..kept) else {
            return Ok(None);
        };

        let mut out = canonical_header(&guess, kept);
        out.extend_from_slice(pixels);
        Ok(Some(Candidate {
            strategy: self.name().to_string(),
            data: Payload::Bytes(out),
            score: Score {
                content_recovered: u64::from(guess.rows()),
                // Deliberately unknown: the original height was in the header that is gone, so
                // claiming a denominator here would turn a guess into a "100% recovered".
                content_total: None,
                decoder_errors: 0,
                decodes_cleanly: true,
                streams_complete: false,
            },
            fidelity: Fidelity::Reconstructed,
            reconstructed: std::iter::once(0..u64::from(guess.rows())).collect(),
        }))
    }
}

/// A fresh 14 + 40 byte header for a rebuilt file. Only used for layouts that need no palette,
/// which is checked before this is reached.
fn canonical_header(geometry: &Geometry, pixel_bytes: usize) -> Vec<u8> {
    let mut out = vec![0u8; CANONICAL_PIXEL_START];
    if let Some(magic) = out.get_mut(..2) {
        magic.copy_from_slice(b"BM");
    }
    put_u32_le(&mut out, 2, u32::try_from(CANONICAL_PIXEL_START + pixel_bytes).unwrap_or(u32::MAX));
    put_u32_le(&mut out, 10, CANONICAL_PIXEL_START as u32);
    put_u32_le(&mut out, 14, 40);
    put_u32_le(&mut out, 18, geometry.width);
    put_u32_le(&mut out, 22, geometry.height as u32);
    put_u16_le(&mut out, 26, 1);
    put_u16_le(&mut out, 28, geometry.bits_per_pixel);
    put_u32_le(&mut out, 34, u32::try_from(pixel_bytes).unwrap_or(u32::MAX));
    out
}

/// Layouts worth trying when the header is gone. Palette-indexed files are excluded: the
/// palette lives in the destroyed region, and without it the recovered bytes are indices into
/// nothing.
const REBUILD_DEPTHS: [u16; 2] = [24, 32];

/// Below this many rows the vertical-continuity measurement is noise.
const MIN_REBUILD_ROWS: u32 = 8;

/// How much better a later candidate must measure to displace the leader.
const EVIDENCE_MARGIN: f64 = 1.02;

/// Rows compared when scoring a candidate layout, and bytes compared per row. Both are capped
/// hard: there are tens of thousands of layouts to score, and the measurement only has to
/// separate "rows line up" from "rows do not", which a sample settles.
const SCORED_ROW_PAIRS: u32 = 16;
const SCORED_ROW_BYTES: usize = 128;

fn guess_geometry(ctx: &RepairContext<'_>) -> Option<Geometry> {
    if let Some(from_reference) = geometry_from_reference(ctx) {
        return Some(from_reference);
    }

    let mut best: Option<(f64, Geometry)> = None;
    for layout in layout_candidates(ctx.data.len()) {
        if ctx.should_stop() {
            break;
        }
        let Some(pixels) = ctx.data.get(layout.start..) else {
            continue;
        };
        let Some(evidence) = alignment_evidence(pixels, layout.stride) else {
            continue;
        };
        // Candidates arrive tallest first, and the measurement is a sample, so two layouts of
        // the same row length differ by noise alone. Displacing the leader takes a real
        // margin; without one the winner would be whichever sampled rows happened to agree,
        // and the picture would lose rows for no reason.
        if best.as_ref().is_some_and(|(previous, _)| evidence <= previous * EVIDENCE_MARGIN) {
            continue;
        }
        let Some(geometry) = layout.to_geometry() else {
            continue;
        };
        best = Some((evidence, geometry));
    }
    best.map(|(_, geometry)| geometry)
}

/// One way the surviving bytes could be laid out: where the rows start, how long each is, and
/// how many there are.
#[derive(Debug, Clone, Copy)]
struct Layout {
    start: usize,
    stride: usize,
    rows: u32,
}

impl Layout {
    /// The widest picture this row length can hold at a depth whose padding it matches
    /// exactly. Narrower widths with the same stride describe the same bytes with more
    /// padding, and a writer that chose the stride chose it for this width.
    fn to_geometry(self) -> Option<Geometry> {
        for bits_per_pixel in REBUILD_DEPTHS {
            let width = (self.stride as u64 * 8) / u64::from(bits_per_pixel);
            let width = u32::try_from(width).ok()?;
            if width == 0 {
                continue;
            }
            let implied = (u64::from(width) * u64::from(bits_per_pixel)).div_ceil(32) * 4;
            if implied == self.stride as u64 {
                return Some(Geometry {
                    width,
                    height: i32::try_from(self.rows).ok()?,
                    bits_per_pixel,
                    compression: 0,
                    dib_size: 40,
                    declared_file_size: 0,
                    pixel_start: self.start,
                });
            }
        }
        None
    }
}

/// Every layout whose rows fill the file exactly, leaving a header-sized gap at the front.
///
/// The pixel array runs to the end of the file, so a candidate row length and row count fix
/// where the rows must begin - which is what makes the search two-dimensional rather than a
/// guess at one fixed header size. Real headers range from 26 bytes to a colour-managed one
/// with a full palette, and the only thing that rules a layout out cheaply is a start that
/// falls outside that range.
fn layout_candidates(len: usize) -> Vec<Layout> {
    let mut layouts = Vec::new();
    if len <= MAX_HEADER_LEN {
        return layouts;
    }

    let mut stride = MIN_REBUILD_STRIDE;
    while stride <= MAX_REBUILD_STRIDE && stride < len {
        let largest = (len - MIN_HEADER_LEN) / stride;
        let smallest = (len.saturating_sub(MAX_HEADER_LEN)).div_ceil(stride);
        // Tallest first: the candidates differing only in row count all describe the same
        // rows, and the tallest is the one that keeps every row the file still holds.
        for rows in (smallest..=largest).rev() {
            if rows < MIN_REBUILD_ROWS as usize {
                continue;
            }
            let Some(start) = len.checked_sub(stride * rows) else {
                continue;
            };
            if start < MIN_HEADER_LEN {
                continue;
            }
            layouts.push(Layout {
                start,
                stride,
                rows: u32::try_from(rows).unwrap_or(u32::MAX),
            });
        }
        stride += 4;
    }
    layouts
}

/// Smallest header a BMP can have: the file header plus the twelve-byte core header.
const MIN_HEADER_LEN: usize = FILE_HEADER_LEN + 12;
/// Largest one worth considering: a colour-managed header plus a full 256-entry palette.
const MAX_HEADER_LEN: usize = FILE_HEADER_LEN + 124 + 256 * 4;

/// A healthy sibling shot by the same device has the same geometry, which beats any heuristic.
/// Accepted only when its pixel array is exactly the size of the surviving payload, so a
/// reference of the wrong dimensions cannot quietly reshape the picture.
fn geometry_from_reference(ctx: &RepairContext<'_>) -> Option<Geometry> {
    for path in ctx.references {
        let Ok(reference) = std::fs::read(path) else {
            continue;
        };
        let Some(geometry) = parse(&reference) else {
            continue;
        };
        if !geometry.is_uncompressed() || geometry.bits_per_pixel <= 8 {
            continue;
        }
        let payload = (ctx.data.len() as u64).saturating_sub(geometry.pixel_start as u64);
        if payload == geometry.pixel_bytes() {
            return Some(geometry);
        }
    }
    None
}

const MIN_REBUILD_STRIDE: usize = 32;
/// A row of 16384 pixels at 32 bits each, past any picture a consumer device produces.
const MAX_REBUILD_STRIDE: usize = 16384 * 4;

/// How much better the bytes line up at `stride` than at a row length four bytes to either
/// side of it. Larger is stronger evidence; one means the candidate explains nothing.
///
/// The raw discontinuity on its own is not enough, and a test with a smooth synthetic picture
/// showed why: in an image with little vertical detail *every* row length scores low, and the
/// winner is then decided by noise - a layout of ten enormous rows beat the real one. What
/// distinguishes the true row length is not that it scores well but that its neighbours score
/// *worse*: at stride + 4 the rows shear by four bytes and a real picture falls apart, while a
/// coincidence stays exactly as smooth as it was.
fn alignment_evidence(pixels: &[u8], stride: usize) -> Option<f64> {
    let aligned = vertical_discontinuity(pixels, stride)?;
    let before = vertical_discontinuity(pixels, stride.checked_sub(4)?)?;
    let after = vertical_discontinuity(pixels, stride + 4)?;

    // Smoothed, so that a flat region (every difference zero) cannot divide by zero or turn a
    // one-byte advantage into unbounded confidence.
    Some((before + after + 2.0) / (2.0 * aligned + 2.0))
}

/// Mean absolute difference between vertically adjacent bytes, or `None` when the data cannot
/// hold two rows of this length.
fn vertical_discontinuity(pixels: &[u8], stride: usize) -> Option<f64> {
    if stride == 0 {
        return None;
    }
    let rows = u32::try_from(pixels.len() / stride).unwrap_or(u32::MAX);
    if rows < 2 {
        return None;
    }

    let step = (rows / SCORED_ROW_PAIRS).max(1);
    let width = stride.min(SCORED_ROW_BYTES);
    let mut total = 0u64;
    let mut compared = 0u64;

    let mut row = 0;
    while row + 1 < rows {
        let above = row as usize * stride;
        let below = above + stride;
        if let (Some(a), Some(b)) = (pixels.get(above..above + width), pixels.get(below..below + width)) {
            total += a.iter().zip(b).map(|(x, y)| u64::from(x.abs_diff(*y))).sum::<u64>();
            compared += a.len() as u64;
        }
        row += step;
    }

    (compared > 0).then(|| total as f64 / compared as f64)
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;
    use crate::file_fixer::corrupt::{ByteSpan, DamageKind, DamageSpec, Fraction, apply_damage};
    use crate::file_fixer::engine::{RepairOptions, RepairResult, repair_data};
    use crate::file_fixer::mode::Mode;
    use crate::file_fixer::report::Outcome;

    /// A smooth 24-bit picture: neighbouring rows resemble each other, which is what the
    /// geometry search relies on and what any real photograph also looks like.
    fn sample_bmp(width: u32, height: u32) -> Vec<u8> {
        let stride = (width as usize * 3).div_ceil(4) * 4;
        let mut pixels = vec![0u8; stride * height as usize];
        for y in 0..height as usize {
            for x in 0..width as usize {
                let offset = y * stride + x * 3;
                let shade = ((x * 255 / width.max(1) as usize) + (y * 64 / height.max(1) as usize)) as u8;
                if let Some(pixel) = pixels.get_mut(offset..offset + 3) {
                    pixel.copy_from_slice(&[shade, shade.wrapping_add(17), shade / 2]);
                }
            }
        }

        let geometry = Geometry {
            width,
            height: height as i32,
            bits_per_pixel: 24,
            compression: 0,
            dib_size: 40,
            declared_file_size: 0,
            pixel_start: CANONICAL_PIXEL_START,
        };
        let mut out = canonical_header(&geometry, pixels.len());
        out.extend_from_slice(&pixels);
        out
    }

    fn repair(data: &[u8], mode: Mode) -> RepairResult {
        let options = RepairOptions { mode, ..RepairOptions::default() };
        repair_data(Path::new("sample.bmp"), data, &options)
    }

    fn bytes(result: &RepairResult) -> Vec<u8> {
        result
            .winner
            .as_ref()
            .and_then(|winner| winner.data.as_bytes())
            .expect("image payloads stay in memory")
            .to_vec()
    }

    #[test]
    fn a_healthy_file_is_returned_untouched() {
        let original = sample_bmp(64, 48);
        let result = repair(&original, Mode::Balanced);

        assert_eq!(result.report.outcome, Outcome::FullyRepaired);
        assert_eq!(result.report.winning_strategy.as_deref(), Some("bmp_verbatim"));
        assert_eq!(bytes(&result), original);
    }

    #[test]
    fn the_geometry_is_read_back_exactly_as_written() {
        let geometry = parse(&sample_bmp(37, 21)).expect("the sample parses");
        assert_eq!(geometry.width, 37);
        assert_eq!(geometry.height, 21);
        assert_eq!(geometry.bits_per_pixel, 24);
        assert_eq!(geometry.stride(), 112, "37 * 3 rounded up to a multiple of four");
        assert_eq!(geometry.pixel_start, CANONICAL_PIXEL_START);
        assert!(!geometry.is_top_down());
    }

    /// The most common real complaint about a BMP: the picture is all there, the header points
    /// a viewer at the wrong place, and the file will not open. Nothing is discarded, so this
    /// is repaired even in safe mode.
    #[test]
    fn a_scrambled_data_offset_is_repaired_without_losing_a_row_even_in_safe_mode() {
        let mut data = sample_bmp(32, 32);
        put_u32_le(&mut data, 10, 0xDEAD);

        let result = repair(&data, Mode::Safe);
        assert_eq!(result.report.winning_strategy.as_deref(), Some("bmp_fix_header"));
        assert_eq!(result.report.outcome, Outcome::FullyRepaired);

        let fixed = bytes(&result);
        assert_eq!(read_u32_le(&fixed, 10), Some(CANONICAL_PIXEL_START as u32));
        assert_eq!(read_u32_le(&fixed, 2), Some(fixed.len() as u32));
        assert_eq!(fixed.get(CANONICAL_PIXEL_START..), data.get(CANONICAL_PIXEL_START..));
    }

    /// Found by running the tool over real images: writers leave the file-size field at zero
    /// and decoders do not care. Calling that damage would hand back a "repaired" copy of a
    /// picture nobody had a problem with.
    #[test]
    fn a_file_size_field_no_decoder_reads_is_not_treated_as_damage() {
        let mut data = sample_bmp(32, 32);
        put_u32_le(&mut data, 2, 0);

        let result = repair(&data, Mode::Balanced);
        assert_eq!(result.report.winning_strategy.as_deref(), Some("bmp_verbatim"));
        assert_eq!(bytes(&result), data);
    }

    /// Bottom-up storage is what makes this worth stating: the rows that survive a cut are the
    /// bottom of the picture, and they must come back byte for byte.
    #[test]
    fn a_truncated_file_keeps_the_rows_that_are_complete() {
        let original = sample_bmp(64, 200);
        let damaged = apply_damage(
            &original,
            &DamageSpec::new(
                DamageKind::TruncateTail {
                    keep: Fraction::new(0.5).unwrap(),
                },
                1,
            ),
        );

        let result = repair(&damaged.data, Mode::Balanced);
        assert_eq!(result.report.outcome, Outcome::PartiallyRecovered);
        assert_eq!(result.report.winning_strategy.as_deref(), Some("bmp_truncate"));

        let fixed = bytes(&result);
        let geometry = parse(&fixed).expect("the repaired file parses");
        assert_eq!(geometry.width, 64);
        assert!(geometry.rows() > 0 && geometry.rows() < 200, "{}", geometry.rows());
        assert_eq!(fixed.len() as u64, geometry.pixel_start as u64 + geometry.pixel_bytes());

        let kept = (geometry.stride() * u64::from(geometry.rows())) as usize;
        assert_eq!(
            fixed.get(geometry.pixel_start..),
            original.get(CANONICAL_PIXEL_START..CANONICAL_PIXEL_START + kept),
            "recovered rows must be the original bytes"
        );
        assert!(result.winner.is_some_and(|winner| winner.is_lossless()));
    }

    #[test]
    fn safe_mode_refuses_to_shorten_a_picture() {
        let original = sample_bmp(64, 200);
        let damaged = apply_damage(
            &original,
            &DamageSpec::new(
                DamageKind::TruncateTail {
                    keep: Fraction::new(0.5).unwrap(),
                },
                1,
            ),
        );

        let result = repair(&damaged.data, Mode::Safe);
        assert_eq!(result.report.outcome, Outcome::Unrecoverable);
        assert!(
            result.report.declined_capabilities.contains(&"truncate".to_string()),
            "the user must be told a repair exists"
        );
    }

    /// Header noise destroys the only statement of how wide a row is. The bytes that follow
    /// are untouched, and at the right width the picture is continuous again.
    #[test]
    fn a_destroyed_header_is_rebuilt_from_the_shape_of_the_pixels() {
        let original = sample_bmp(160, 120);
        let damaged = apply_damage(
            &original,
            &DamageSpec::new(
                DamageKind::HeaderNoise {
                    len: ByteSpan::Bytes(CANONICAL_PIXEL_START as u64),
                },
                3,
            ),
        );

        let result = repair(&damaged.data, Mode::Aggressive);
        let winner = result.winner.as_ref().expect("the pixels are still there");
        assert_eq!(winner.strategy, "bmp_rebuild_header");
        assert!(!winner.is_lossless(), "a guessed geometry is not a faithful repair");

        let geometry = parse(&bytes(&result)).expect("the rebuilt file parses");
        assert_eq!(geometry.width, 160);
        assert_eq!(geometry.rows(), 120);
    }

    #[test]
    fn a_reference_of_the_same_shape_settles_the_geometry_outright() {
        let original = sample_bmp(200, 100);
        let reference = tempfile::Builder::new().suffix(".bmp").tempfile().expect("temp file");
        std::fs::write(reference.path(), sample_bmp(200, 100)).expect("write");

        let mut data = original.clone();
        if let Some(header) = data.get_mut(..CANONICAL_PIXEL_START) {
            header.fill(0);
        }

        let options = RepairOptions {
            mode: Mode::Aggressive,
            references: vec![reference.path().to_path_buf()],
            ..RepairOptions::default()
        };
        let result = repair_data(Path::new("sample.bmp"), &data, &options);
        let geometry = parse(&bytes(&result)).expect("the rebuilt file parses");

        assert_eq!((geometry.width, geometry.rows()), (200, 100));
        assert_eq!(bytes(&result).get(CANONICAL_PIXEL_START..), original.get(CANONICAL_PIXEL_START..));
    }

    /// Safe and balanced modes may not guess, and must say so rather than inventing a width.
    #[test]
    fn a_destroyed_header_yields_nothing_without_permission_to_guess() {
        let original = sample_bmp(160, 120);
        let damaged = apply_damage(&original, &DamageSpec::new(DamageKind::HeaderNoise { len: ByteSpan::Bytes(64) }, 3));

        let result = repair(&damaged.data, Mode::Balanced);
        assert!(result.winner.is_none());
        assert!(result.report.declined_capabilities.contains(&"guess".to_string()));
    }

    /// Nothing in the format can tell a zeroed run from black pixels, so the honest answer is
    /// that the file is intact - not a fabricated repair.
    #[test]
    fn a_zeroed_run_inside_the_pixels_is_not_detectable_and_is_not_pretended_otherwise() {
        let original = sample_bmp(64, 64);
        let damaged = apply_damage(
            &original,
            &DamageSpec::new(
                DamageKind::ZeroRange {
                    at: Fraction::HALF,
                    len: ByteSpan::Bytes(512),
                },
                4,
            ),
        );

        let result = repair(&damaged.data, Mode::Balanced);
        assert_eq!(result.report.winning_strategy.as_deref(), Some("bmp_verbatim"));
        assert_eq!(bytes(&result), damaged.data);
    }

    /// Run-length encoded rows are a different decoding problem entirely; claiming a repair
    /// from the raw bytes would be an invention.
    #[test]
    fn a_run_length_encoded_file_is_declined_rather_than_misread() {
        let mut data = sample_bmp(32, 32);
        put_u16_le(&mut data, 28, 8);
        put_u32_le(&mut data, 30, 1);

        assert!(matches!(BmpRepairer.supports(&data), Support::Unsupported(_)));
        assert_eq!(repair(&data, Mode::Aggressive).report.outcome, Outcome::Unsupported);
    }

    #[test]
    fn a_top_down_picture_keeps_its_direction_through_a_repair() {
        let mut data = sample_bmp(16, 16);
        put_u32_le(&mut data, 22, (-16i32) as u32);
        put_u32_le(&mut data, 2, 0);

        let geometry = parse(&data).expect("parses");
        assert!(geometry.is_top_down());

        let fixed = assemble(&data, &geometry, 8).expect("assembles");
        let repaired = parse(&fixed).expect("the result parses");
        assert_eq!(repaired.height, -8);
    }

    #[test]
    fn a_garbage_header_is_not_mistaken_for_a_picture() {
        assert!(parse(&[]).is_none());
        assert!(parse(b"BM").is_none());
        assert!(parse(&vec![0x42u8; 4096]).is_none());
        assert!(parse(&sample_bmp(1, 1)).is_some());
    }

    #[test]
    fn short_and_hostile_inputs_never_panic() {
        for len in 0..80usize {
            let mut data = sample_bmp(8, 8);
            data.truncate(len);
            let _ = repair(&data, Mode::Aggressive);
        }
    }
}
