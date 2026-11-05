//! TIFF repair.
//!
//! TIFF stores its pixels in independently compressed strips and describes them in a directory
//! of offsets. That layout decides everything about what can be recovered: a strip that is
//! present is decodable on its own, whatever the compression, because no strip depends on the
//! one before it. So the truncation case is clean - keep the strips that are entirely there,
//! say how many rows that is, and the result is byte-faithful.
//!
//! The complication is that the directory is often at the *end* of the file (every writer is
//! free to put it where it likes, and many append it last), so the same truncation that costs
//! the last strips can take the directory with it. Then the strips are all still there and
//! nothing points at them; the repair is to find the directory - or, when it is gone too, to
//! scan for one - and write the file out again with offsets that lead where the data actually
//! is. Nothing about the pixels changes in any of this.
//!
//! Tiled images and BigTIFF are recognized and declined: both need a different traversal, and
//! reading a tiled image as if it were striped would produce a confidently wrong result.

use super::tiff_ifd::{self, Builder, Ifd, Order};
use crate::file_fixer::candidate::{Candidate, Fidelity, Payload, Score};
use crate::file_fixer::engine::{RepairContext, Repairer, Strategy, Support};
use crate::file_fixer::error::Result;
use crate::file_fixer::format::Format;
use crate::file_fixer::mode::Capabilities;

pub struct TiffRepairer;

impl Repairer for TiffRepairer {
    fn format(&self) -> Format {
        Format::Tiff
    }

    fn supports(&self, data: &[u8]) -> Support {
        match declined_variant(data) {
            Some(reason) => Support::Unsupported(reason),
            None => Support::Supported,
        }
    }

    /// A declined variant produces no candidates at all; `supports` above only supplies the
    /// reason text the engine prints once the list comes back empty.
    fn strategies(&self, ctx: &RepairContext<'_>) -> Vec<Box<dyn Strategy>> {
        if declined_variant(ctx.data).is_some() {
            return Vec::new();
        }
        vec![
            Box::new(Verbatim),
            Box::new(RewriteDirectory),
            Box::new(TruncateToWholeStrips),
            Box::new(RebuildHeader),
            Box::new(DirectoryFromReference),
        ]
    }
}

fn declined_variant(data: &[u8]) -> Option<String> {
    let (order, version, first) = tiff_ifd::parse_header(data)?;
    if version == tiff_ifd::BIG_TIFF_VERSION {
        return Some("BigTIFF is not handled".to_string());
    }
    if version != tiff_ifd::CLASSIC_VERSION {
        return None;
    }
    let ifd = tiff_ifd::read_ifd(data, order, first)?;
    ifd.entry(tiff_ifd::TILE_OFFSETS).map(|_| "tiled TIFF is not handled".to_string())
}

/// Offsets past this point are not searched for a directory.
const SCAN_LIMIT: usize = 8 << 20;

/// The directory a repair should work from, and how it was found.
struct Located {
    order: Order,
    ifd: Ifd,
}

impl Located {
    fn of(data: &[u8]) -> Option<Self> {
        let (order, version, first) = tiff_ifd::parse_header(data)?;
        if version != tiff_ifd::CLASSIC_VERSION {
            return None;
        }
        let ifd = tiff_ifd::read_ifd(data, order, first)?;
        Some(Self { order, ifd })
    }

    /// Looks for a directory anywhere in the file.
    ///
    /// The byte order the header claimed is tried first even when the rest of the header is
    /// unusable - two bytes at the very start of a file often survive damage that took the
    /// offset four bytes later.
    fn by_scan(data: &[u8]) -> Option<Self> {
        let preferred = tiff_ifd::parse_header(data).map(|(order, _, _)| order);
        let orders = preferred.into_iter().chain([Order::Little, Order::Big]);

        for order in orders {
            if let Some(ifd) = tiff_ifd::find_ifd(data, order, SCAN_LIMIT) {
                return Some(Self { order, ifd });
            }
        }
        None
    }

    fn rows(&self, data: &[u8]) -> u64 {
        self.ifd.number(data, self.order, tiff_ifd::IMAGE_LENGTH).unwrap_or_default()
    }

    /// Rows in one strip. Absent means the whole image is a single strip.
    fn rows_per_strip(&self, data: &[u8]) -> u64 {
        match self.ifd.number(data, self.order, tiff_ifd::ROWS_PER_STRIP) {
            Some(rows) if rows > 0 => rows,
            _ => self.rows(data).max(1),
        }
    }

    /// Strips that lie entirely inside the file, counting only the unbroken run from the
    /// first: a hole in the middle would leave a picture with rows from the wrong places.
    fn present_strips(&self, data: &[u8]) -> usize {
        self.ifd
            .strips(data, self.order)
            .into_iter()
            .take_while(|(offset, count)| *count > 0 && offset + count <= data.len() as u64)
            .count()
    }

    fn strip_count(&self, data: &[u8]) -> usize {
        self.ifd.strips(data, self.order).len()
    }

    /// True when every strip is present and the page chain ends properly, which is what a
    /// pass-through requires - and, negated, what makes rewriting the directory worth doing.
    fn is_whole(&self, data: &[u8]) -> bool {
        let strips = self.strip_count(data);
        if strips == 0 || self.present_strips(data) != strips {
            return false;
        }
        tiff_ifd::read_chain(data, self.order, self.ifd.start).last().is_some_and(|last| last.next == 0)
    }
}

/// Writes a fresh file holding the first `keep` strips, with a directory that describes them.
fn rebuild(data: &[u8], located: &Located, keep: usize) -> Option<(Vec<u8>, u64)> {
    let rows_per_strip = located.rows_per_strip(data);
    let declared_rows = located.rows(data);

    let mut builder = Builder::new(located.order);
    for (offset, count) in located.ifd.strips(data, located.order).into_iter().take(keep) {
        let start = usize::try_from(offset).ok()?;
        let end = usize::try_from(offset + count).ok()?;
        builder.add_strip(data.get(start..end)?);
    }
    if builder.strip_count() == 0 {
        return None;
    }

    let rows = (builder.strip_count() as u64 * rows_per_strip).min(declared_rows.max(1));
    let out = builder.finish(data, &located.ifd, &[(tiff_ifd::IMAGE_LENGTH, vec![rows], 4)]);
    Some((out, rows))
}

fn score_of(rows: u64, total: u64) -> Score {
    Score {
        content_recovered: rows,
        content_total: Some(total),
        decoder_errors: 0,
        decodes_cleanly: true,
        streams_complete: rows >= total,
    }
}

/// The header points at a directory, and every strip it names is inside the file.
struct Verbatim;

impl Strategy for Verbatim {
    fn name(&self) -> &str {
        "tiff_verbatim"
    }

    fn run(&self, ctx: &RepairContext<'_>) -> Result<Option<Candidate>> {
        let Some(located) = Located::of(ctx.data) else {
            return Ok(None);
        };
        // A directory chain that breaks part way means later pages are damaged even though
        // this one is fine, and that is a rewrite rather than a pass-through.
        if !located.is_whole(ctx.data) {
            return Ok(None);
        }
        let rows = located.rows(ctx.data);

        Ok(Some(Candidate {
            strategy: self.name().to_string(),
            data: Payload::Bytes(ctx.data.to_vec()),
            score: score_of(rows, rows),
            fidelity: Fidelity::Lossless,
            reconstructed: Vec::new(),
        }))
    }
}

/// Every strip is present but something around them is wrong: a broken pointer to the next
/// page, a value that leads outside the file, a version field that no reader accepts. Writing
/// the file out again with correct offsets discards no pixel, so this needs no capability.
struct RewriteDirectory;

impl Strategy for RewriteDirectory {
    fn name(&self) -> &str {
        "tiff_rewrite_directory"
    }

    fn run(&self, ctx: &RepairContext<'_>) -> Result<Option<Candidate>> {
        let Some(located) = Located::of(ctx.data) else {
            return Ok(None);
        };
        let strips = located.strip_count(ctx.data);
        if strips == 0 || located.present_strips(ctx.data) != strips || located.is_whole(ctx.data) {
            return Ok(None);
        }
        let Some((out, rows)) = rebuild(ctx.data, &located, strips) else {
            return Ok(None);
        };

        Ok(Some(Candidate {
            strategy: self.name().to_string(),
            data: Payload::Bytes(out),
            score: score_of(rows, located.rows(ctx.data).max(rows)),
            fidelity: Fidelity::Lossless,
            reconstructed: Vec::new(),
        }))
    }
}

/// The pixel data stops part way. Keep the strips that are complete and state how many rows
/// that leaves.
struct TruncateToWholeStrips;

impl Strategy for TruncateToWholeStrips {
    fn name(&self) -> &str {
        "tiff_truncate"
    }

    fn requires(&self) -> Capabilities {
        Capabilities {
            truncate: true,
            ..Capabilities::NONE
        }
    }

    fn run(&self, ctx: &RepairContext<'_>) -> Result<Option<Candidate>> {
        let Some(located) = Located::of(ctx.data).or_else(|| Located::by_scan(ctx.data)) else {
            return Ok(None);
        };
        let present = located.present_strips(ctx.data);
        if present == 0 || present == located.strip_count(ctx.data) {
            return Ok(None);
        }
        let Some((out, rows)) = rebuild(ctx.data, &located, present) else {
            return Ok(None);
        };

        Ok(Some(Candidate {
            strategy: self.name().to_string(),
            data: Payload::Bytes(out),
            score: score_of(rows, located.rows(ctx.data).max(rows)),
            fidelity: Fidelity::Lossless,
            reconstructed: Vec::new(),
        }))
    }
}

/// The header is gone, or points nowhere. Find the directory by its shape.
///
/// This is the common shape of a damaged TIFF rather than an exotic one: writers routinely put
/// the directory at the end of the file, where a single lost sector takes the only description
/// of an image whose pixels are entirely intact.
struct RebuildHeader;

impl Strategy for RebuildHeader {
    fn name(&self) -> &str {
        "tiff_rebuild_header"
    }

    fn requires(&self) -> Capabilities {
        Capabilities {
            guess: true,
            ..Capabilities::NONE
        }
    }

    fn run(&self, ctx: &RepairContext<'_>) -> Result<Option<Candidate>> {
        if Located::of(ctx.data).is_some() {
            return Ok(None);
        }
        let Some(located) = Located::by_scan(ctx.data) else {
            return Ok(None);
        };
        let present = located.present_strips(ctx.data);
        if present == 0 {
            return Ok(None);
        }
        let Some((out, rows)) = rebuild(ctx.data, &located, present) else {
            return Ok(None);
        };

        let declared = located.rows(ctx.data).max(rows);
        Ok(Some(Candidate {
            strategy: self.name().to_string(),
            data: Payload::Bytes(out),
            score: score_of(rows, declared),
            // The directory that was found is the original one, and the strips it names are
            // the original pixels - only the eight-byte header leading to it is new.
            fidelity: Fidelity::Lossless,
            reconstructed: Vec::new(),
        }))
    }
}

/// The directory was at the end of the file and went with the bytes that are missing.
///
/// This is the most common way a real TIFF breaks - libtiff and ImageMagick both append the
/// directory last - and on its own it is fatal: the pixels are all still there and nothing in
/// the file says how wide a row is. A healthy file from the same scanner or camera does say,
/// and for an uncompressed image that is enough, because then every row is the same length and
/// the strips follow one another without a table.
///
/// Compressed images are not attempted. Their strip lengths were only ever written down in the
/// directory that is gone, and no property of the data recovers them.
struct DirectoryFromReference;

impl Strategy for DirectoryFromReference {
    fn name(&self) -> &str {
        "tiff_directory_from_reference"
    }

    fn requires(&self) -> Capabilities {
        Capabilities {
            guess: true,
            ..Capabilities::NONE
        }
    }

    fn run(&self, ctx: &RepairContext<'_>) -> Result<Option<Candidate>> {
        if Located::of(ctx.data).is_some() || Located::by_scan(ctx.data).is_some() {
            return Ok(None);
        }

        for path in ctx.references {
            let Ok(reference) = std::fs::read(path) else {
                continue;
            };
            let Some(located) = Located::of(&reference) else {
                continue;
            };
            let Some(layout) = uncompressed_layout(&reference, &located) else {
                continue;
            };

            let first_strip = located
                .ifd
                .strips(&reference, located.order)
                .first()
                .map_or(tiff_ifd::HEADER_LEN as u64, |(offset, _)| *offset) as usize;
            let Some(pixels) = ctx.data.get(first_strip..) else {
                continue;
            };
            let rows = pixels.len() as u64 / layout.row_bytes;
            if rows == 0 {
                continue;
            }

            let mut builder = Builder::new(located.order);
            let mut row = 0;
            while row < rows {
                let height = layout.rows_per_strip.min(rows - row);
                let start = usize::try_from(row * layout.row_bytes).unwrap_or(usize::MAX);
                let end = usize::try_from((row + height) * layout.row_bytes).unwrap_or(usize::MAX);
                let Some(strip) = pixels.get(start..end) else {
                    break;
                };
                builder.add_strip(strip);
                row += height;
            }
            if builder.strip_count() == 0 {
                continue;
            }

            let out = builder.finish(
                &reference,
                &located.ifd,
                &[(tiff_ifd::IMAGE_LENGTH, vec![rows], 4), (tiff_ifd::ROWS_PER_STRIP, vec![layout.rows_per_strip], 4)],
            );
            log::debug!("tiff: {} supplied a {}-byte row, {rows} row(s) recovered", path.display(), layout.row_bytes);

            return Ok(Some(Candidate {
                strategy: self.name().to_string(),
                data: Payload::Bytes(out),
                // The reference's geometry cannot be checked against the damaged file, so
                // every row is only as trustworthy as the claim that the two match. Its height
                // is the best available answer to "how much was there", for the same reason.
                score: score_of(rows, located.rows(&reference).max(rows)),
                fidelity: Fidelity::Reconstructed,
                reconstructed: std::iter::once(0..rows).collect(),
            }));
        }
        Ok(None)
    }
}

/// Row geometry of an uncompressed image, or `None` when the file is compressed and its strip
/// lengths therefore unknowable from the pixels alone.
struct Layout {
    row_bytes: u64,
    rows_per_strip: u64,
}

fn uncompressed_layout(data: &[u8], located: &Located) -> Option<Layout> {
    const COMPRESSION: u16 = 259;
    const BITS_PER_SAMPLE: u16 = 258;
    const SAMPLES_PER_PIXEL: u16 = 277;

    let compression = located.ifd.number(data, located.order, COMPRESSION).unwrap_or(1);
    if compression != 1 {
        return None;
    }
    let width = located.ifd.number(data, located.order, tiff_ifd::IMAGE_WIDTH)?;
    let samples = located.ifd.number(data, located.order, SAMPLES_PER_PIXEL).unwrap_or(1).max(1);
    let bits: u64 = located
        .ifd
        .entry(BITS_PER_SAMPLE)
        .map(|entry| entry.numbers(data, located.order).iter().sum())
        .filter(|total: &u64| *total > 0)
        .unwrap_or(8 * samples);

    let row_bytes = (width * bits).div_ceil(8);
    (row_bytes > 0).then(|| Layout {
        row_bytes,
        rows_per_strip: located.rows_per_strip(data).max(1),
    })
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::tiff_ifd::sample_tiff;
    use super::*;
    use crate::file_fixer::corrupt::{ByteSpan, DamageKind, DamageSpec, Fraction, apply_damage};
    use crate::file_fixer::engine::{RepairOptions, RepairResult, repair_data};
    use crate::file_fixer::mode::Mode;
    use crate::file_fixer::report::Outcome;

    fn repair(data: &[u8], mode: Mode) -> RepairResult {
        let options = RepairOptions { mode, ..RepairOptions::default() };
        repair_data(Path::new("scan.tiff"), data, &options)
    }

    fn bytes(result: &RepairResult) -> Vec<u8> {
        result
            .winner
            .as_ref()
            .and_then(|winner| winner.data.as_bytes())
            .expect("image payloads stay in memory")
            .to_vec()
    }

    /// The pixel bytes a file holds, strip by strip, for comparing a repair against its source.
    fn strip_bytes(data: &[u8]) -> Vec<u8> {
        let located = Located::of(data).expect("the file parses");
        let mut out = Vec::new();
        for (offset, count) in located.ifd.strips(data, located.order) {
            if let Some(bytes) = data.get(offset as usize..(offset + count) as usize) {
                out.extend_from_slice(bytes);
            }
        }
        out
    }

    #[test]
    fn a_healthy_file_is_left_alone() {
        let original = sample_tiff(Order::Little, 64, 40, 8);
        let result = repair(&original, Mode::Balanced);

        assert_eq!(result.report.outcome, Outcome::FullyRepaired);
        assert_eq!(result.report.winning_strategy.as_deref(), Some("tiff_verbatim"));
        assert_eq!(result.report.content_recovered, Some(40));
        assert_eq!(bytes(&result), original);
    }

    #[test]
    fn a_big_endian_file_is_handled_the_same_way() {
        let original = sample_tiff(Order::Big, 48, 32, 8);
        let result = repair(&original, Mode::Balanced);

        assert_eq!(result.report.outcome, Outcome::FullyRepaired);
        assert_eq!(bytes(&result), original);
    }

    /// Strips are independently coded, so the ones that survive a cut are whole rows of real
    /// pixels - no decoder state carries across a strip boundary to be desynchronized.
    #[test]
    fn a_truncated_scan_keeps_the_strips_that_are_complete() {
        let original = sample_tiff(Order::Little, 64, 200, 10);
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

        let fixed = bytes(&result);
        let located = Located::of(&fixed).expect("the repaired file parses");
        let rows = located.rows(&fixed);
        assert!(rows > 0 && rows < 200, "{rows}");
        assert_eq!(located.present_strips(&fixed), located.strip_count(&fixed));

        let recovered = strip_bytes(&fixed);
        assert_eq!(
            recovered,
            strip_bytes(&original).get(..recovered.len()).expect("a prefix of the original").to_vec(),
            "every recovered pixel must be the original one"
        );
        assert_eq!(recovered.len() as u64, rows * 64);
    }

    /// The directory sits at the end of the file, so the same cut that loses strips usually
    /// loses the only thing pointing at the ones that remain.
    #[test]
    fn a_file_whose_directory_was_cut_off_is_found_by_scanning() {
        let original = sample_tiff(Order::Little, 64, 100, 10);
        let mut damaged = original.clone();
        // Destroy the header's pointer, leaving the directory itself intact further in.
        if let Some(pointer) = damaged.get_mut(4..8) {
            pointer.copy_from_slice(&0u32.to_le_bytes());
        }

        assert!(Located::of(&damaged).is_none(), "the header leads nowhere now");
        assert!(repair(&damaged, Mode::Balanced).winner.is_none());

        let result = repair(&damaged, Mode::Aggressive);
        let winner = result.winner.as_ref().expect("the directory is still in the file");
        assert_eq!(winner.strategy, "tiff_rebuild_header");
        assert!(winner.is_lossless(), "the directory found is the original one");

        let fixed = bytes(&result);
        assert_eq!(Located::of(&fixed).expect("parses").rows(&fixed), 100);
        assert_eq!(strip_bytes(&fixed), strip_bytes(&original));
    }

    #[test]
    fn a_destroyed_header_is_rebuilt_around_the_surviving_directory() {
        let original = sample_tiff(Order::Little, 64, 100, 10);
        let damaged = apply_damage(&original, &DamageSpec::new(DamageKind::HeaderNoise { len: ByteSpan::Bytes(8) }, 9));

        let result = repair(&damaged.data, Mode::Aggressive);
        let winner = result.winner.as_ref().expect("the pixels and directory survived");
        assert_eq!(winner.strategy, "tiff_rebuild_header");

        let fixed = bytes(&result);
        let located = Located::of(&fixed).expect("the rebuilt file parses");
        assert_eq!(located.rows(&fixed), 100);
        assert_eq!(located.present_strips(&fixed), located.strip_count(&fixed));
    }

    /// A page chain that breaks is not a pass-through, but the page that reads is still whole.
    #[test]
    fn a_broken_link_to_the_next_page_is_repaired_without_losing_a_row() {
        let mut data = sample_tiff(Order::Little, 32, 32, 8);
        let located = Located::of(&data).expect("parses");
        let next_offset = located.ifd.start + 2 + located.ifd.entries.len() * 12;
        if let Some(slot) = data.get_mut(next_offset..next_offset + 4) {
            slot.copy_from_slice(&0xDEAD_BEEFu32.to_le_bytes());
        }

        let result = repair(&data, Mode::Safe);
        assert_eq!(result.report.winning_strategy.as_deref(), Some("tiff_rewrite_directory"));
        assert_eq!(result.report.outcome, Outcome::FullyRepaired);
        assert_eq!(strip_bytes(&bytes(&result)), strip_bytes(&data));
    }

    /// The common real failure: the directory lived at the end of the file and the truncation
    /// took it. Nothing in what remains says how wide a row is - but a healthy file from the
    /// same source does, and for an uncompressed image that is the whole answer.
    #[test]
    fn a_scan_whose_directory_is_gone_is_recovered_from_a_reference() {
        let original = sample_tiff(Order::Little, 64, 200, 10);
        let with_trailing_directory = move_directory_to_the_end(&original);
        let damaged = apply_damage(
            &with_trailing_directory,
            &DamageSpec::new(
                DamageKind::TruncateTail {
                    keep: Fraction::new(0.5).unwrap(),
                },
                1,
            ),
        );

        assert!(Located::of(&damaged.data).is_none(), "the directory is gone");
        assert!(repair(&damaged.data, Mode::Aggressive).winner.is_none());

        let reference = tempfile::Builder::new().suffix(".tif").tempfile().expect("temp file");
        std::fs::write(reference.path(), &original).expect("write");
        let options = RepairOptions {
            mode: Mode::Aggressive,
            references: vec![reference.path().to_path_buf()],
            ..RepairOptions::default()
        };

        let result = repair_data(Path::new("scan.tiff"), &damaged.data, &options);
        let winner = result.winner.as_ref().expect("the pixels are still there");
        assert_eq!(winner.strategy, "tiff_directory_from_reference");
        assert!(!winner.is_lossless(), "the geometry came from another file");

        let fixed = bytes(&result);
        let located = Located::of(&fixed).expect("the rebuilt file parses");
        let rows = located.rows(&fixed);
        assert!(rows > 0 && rows < 200, "{rows}");
        assert_eq!(
            strip_bytes(&fixed),
            strip_bytes(&original).get(..(rows * 64) as usize).expect("a prefix").to_vec(),
            "every recovered pixel must be the original one"
        );
    }

    /// Rewrites a file so its directory sits after the pixel data, which is what libtiff and
    /// ImageMagick both emit and what makes a truncation fatal.
    fn move_directory_to_the_end(data: &[u8]) -> Vec<u8> {
        let located = Located::of(data).expect("parses");
        let directory_len = 2 + located.ifd.entries.len() * 12 + 4;
        let directory = data.get(located.ifd.start..located.ifd.start + directory_len).expect("the directory");

        let mut out = data.to_vec();
        let moved = u32::try_from(out.len()).expect("small enough");
        out.extend_from_slice(directory);
        if let Some(pointer) = out.get_mut(4..8) {
            pointer.copy_from_slice(&moved.to_le_bytes());
        }
        // Blank the original copy so only the moved one can be found.
        if let Some(old) = out.get_mut(located.ifd.start..located.ifd.start + directory_len) {
            old.fill(0);
        }
        out
    }

    #[test]
    fn safe_mode_will_not_shorten_a_scan() {
        let original = sample_tiff(Order::Little, 64, 200, 10);
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
        assert!(result.winner.is_none());
        assert!(result.report.declined_capabilities.contains(&"truncate".to_string()));
    }

    #[test]
    fn recovery_grows_with_the_amount_of_surviving_data() {
        let original = sample_tiff(Order::Little, 64, 200, 10);
        let mut previous = 0;
        for keep in [0.3, 0.5, 0.7, 0.9] {
            let damaged = apply_damage(
                &original,
                &DamageSpec::new(
                    DamageKind::TruncateTail {
                        keep: Fraction::new(keep).unwrap(),
                    },
                    1,
                ),
            );
            let recovered = repair(&damaged.data, Mode::Aggressive).report.content_recovered.unwrap_or(0);
            assert!(recovered >= previous, "keep {keep} gave {recovered} after {previous}");
            previous = recovered;
        }
    }

    #[test]
    fn big_tiff_is_declined_rather_than_misread() {
        let mut data = sample_tiff(Order::Little, 32, 32, 8);
        if let Some(version) = data.get_mut(2..4) {
            version.copy_from_slice(&tiff_ifd::BIG_TIFF_VERSION.to_le_bytes());
        }

        assert!(matches!(TiffRepairer.supports(&data), Support::Unsupported(_)));
        assert_eq!(repair(&data, Mode::Aggressive).report.outcome, Outcome::Unsupported);
    }

    #[test]
    fn short_and_hostile_inputs_never_panic() {
        let original = sample_tiff(Order::Little, 16, 16, 4);
        for len in 0..original.len().min(300) {
            let mut data = original.clone();
            data.truncate(len);
            let _ = repair(&data, Mode::Aggressive);
        }
    }
}
