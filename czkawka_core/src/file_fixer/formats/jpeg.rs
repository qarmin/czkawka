//! JPEG repair.
//!
//! The valuable output for a damaged JPEG is a **lossless canvas crop**: the height field in
//! the frame header is rewritten to the number of whole MCU rows that decoded, the scan is
//! cut at that row's boundary, and an `EOI` is appended. Nothing is re-encoded, so every
//! surviving pixel keeps its exact original value and no grey band is passed off as content.
//!
//! Finding that boundary needs a real pass over the entropy stream, which lives in
//! [`super::jpeg_scan`].
//!
//! One caveat on "lossless": the entropy-coded coefficients are carried over bit for bit, but
//! on a chroma-subsampled image the **last pixel row** of the crop can still decode slightly
//! differently. Upsampling interpolates each chroma sample with the row below it, and at the
//! new bottom edge that row no longer exists, so the decoder replicates instead. The stored
//! data is unchanged; only the decoder's edge handling differs.

use super::jpeg_assemble::assemble;
use super::jpeg_integrity::{Analysis, Integrity, find_end_marker};
use super::jpeg_markers::{self, FrameKind};
use crate::file_fixer::candidate::{Candidate, Fidelity, Payload, Score};
use crate::file_fixer::engine::{RepairContext, Repairer, Strategy, Support};
use crate::file_fixer::error::Result;
use crate::file_fixer::format::Format;
use crate::file_fixer::mode::Capabilities;

pub struct JpegRepairer;

impl Repairer for JpegRepairer {
    fn format(&self) -> Format {
        Format::Jpeg
    }

    fn supports(&self, data: &[u8]) -> Support {
        let layout = jpeg_markers::parse(data);
        let Some(frame) = layout.frame.as_ref() else {
            return Support::Supported;
        };
        match frame.kind {
            FrameKind::Progressive => Support::Unsupported("progressive JPEG is not handled".into()),
            FrameKind::Unsupported => Support::Unsupported("lossless, hierarchical or arithmetic-coded JPEG is not handled".into()),
            FrameKind::Baseline if frame.precision != 8 => Support::Unsupported(format!("{}-bit precision is not handled", frame.precision)),
            FrameKind::Baseline => Support::Supported,
        }
    }

    fn strategies(&self, _ctx: &RepairContext<'_>) -> Vec<Box<dyn Strategy>> {
        vec![
            Box::new(Verbatim),
            Box::new(AppendEndMarker),
            Box::new(CropToWholeMcuRows),
            Box::new(CropUnverified),
            Box::new(super::jpeg_rebuild::RebuildHeader),
        ]
    }
}

fn full_score(height: u16) -> Score {
    Score {
        content_recovered: u64::from(height),
        content_total: Some(u64::from(height)),
        decoder_errors: 0,
        decodes_cleanly: true,
        streams_complete: true,
    }
}

/// The file already decodes end to end; hand the original bytes back untouched.
struct Verbatim;

impl Strategy for Verbatim {
    fn name(&self) -> &str {
        "jpeg_verbatim"
    }

    fn run(&self, ctx: &RepairContext<'_>) -> Result<Option<Candidate>> {
        let Some(height) = intact_height(ctx.data) else {
            return Ok(None);
        };

        Ok(Some(Candidate {
            strategy: self.name().to_string(),
            data: Payload::Bytes(ctx.data.to_vec()),
            score: full_score(height),
            fidelity: Fidelity::Lossless,
            reconstructed: Vec::new(),
        }))
    }
}

/// The image height when the file needs no repair at all, otherwise `None`.
///
/// Deliberately *not* backed by a decoder for variants the scanner declines. `zune-jpeg`
/// accepts a damaged progressive file without complaint - measured on files from a decoder's
/// own crash-test suite - so "it decoded" is not evidence of health. Those variants are
/// reported as unsupported instead, which says what is actually true.
fn intact_height(data: &[u8]) -> Option<u16> {
    let analysis = Analysis::of(data)?;
    let height = analysis.frame()?.height;
    (analysis.integrity == Integrity::Whole && find_end_marker(data).is_some()).then_some(height)
}

/// Every MCU is present but the file lost its end marker. Appending one costs no content.
struct AppendEndMarker;

impl Strategy for AppendEndMarker {
    fn name(&self) -> &str {
        "jpeg_append_eoi"
    }

    fn run(&self, ctx: &RepairContext<'_>) -> Result<Option<Candidate>> {
        let Some(analysis) = Analysis::of(ctx.data) else {
            return Ok(None);
        };
        if analysis.integrity != Integrity::Whole || find_end_marker(ctx.data).is_some() {
            return Ok(None);
        }
        let (Some(frame), Some(cut)) = (analysis.frame(), analysis.cut()) else {
            return Ok(None);
        };

        Ok(Some(Candidate {
            strategy: self.name().to_string(),
            data: Payload::Bytes(assemble(ctx.data, &analysis, frame.height, cut)?),
            score: full_score(frame.height),
            fidelity: Fidelity::Lossless,
            reconstructed: Vec::new(),
        }))
    }
}

/// Keep the MCU rows that decoded and shrink the canvas to match.
///
/// The alternative, leaving the declared height alone, produces a file whose lower part is
/// whatever the decoder invents for missing data - usually grey, but decoder-dependent, and
/// indistinguishable from real content once written to disk.
struct CropToWholeMcuRows;

impl Strategy for CropToWholeMcuRows {
    fn name(&self) -> &str {
        "jpeg_crop"
    }

    fn requires(&self) -> Capabilities {
        Capabilities {
            truncate: true,
            ..Capabilities::NONE
        }
    }

    fn run(&self, ctx: &RepairContext<'_>) -> Result<Option<Candidate>> {
        let Some(analysis) = Analysis::of(ctx.data) else {
            return Ok(None);
        };
        if analysis.integrity != Integrity::Truncated {
            return Ok(None);
        }
        crop(self.name(), ctx, &analysis, Fidelity::Lossless)
    }
}

/// Salvage a file whose entropy stream was altered rather than merely cut short.
///
/// Split from the plain crop because the guarantee is different: nothing here can prove which
/// rows came from original bytes. Huffman codes keep decoding after an altered byte, they just
/// stop meaning what they meant, so the surviving rows are plausible rather than certain.
struct CropUnverified;

impl Strategy for CropUnverified {
    fn name(&self) -> &str {
        "jpeg_crop_unverified"
    }

    fn requires(&self) -> Capabilities {
        Capabilities {
            truncate: true,
            guess: true,
            ..Capabilities::NONE
        }
    }

    fn run(&self, ctx: &RepairContext<'_>) -> Result<Option<Candidate>> {
        let Some(analysis) = Analysis::of(ctx.data) else {
            return Ok(None);
        };
        if analysis.integrity.is_provable() {
            return Ok(None);
        }
        crop(self.name(), ctx, &analysis, Fidelity::Reconstructed)
    }
}

/// Builds a cropped candidate from the MCU rows that decoded.
fn crop(name: &str, ctx: &RepairContext<'_>, analysis: &Analysis, fidelity: Fidelity) -> Result<Option<Candidate>> {
    let (Some(frame), Some(cut)) = (analysis.frame(), analysis.cut()) else {
        return Ok(None);
    };

    // Rows of coded units, not MCUs: a single-component scan uses plain 8x8 blocks.
    let rows_per_unit_row = analysis.rows_per_unit_row();
    let recovered = u64::from(analysis.report.complete_rows()) * u64::from(rows_per_unit_row);
    let height = u16::try_from(recovered.min(u64::from(frame.height))).unwrap_or(frame.height);
    if height == 0 {
        return Ok(None);
    }

    // Nothing localizes the damage, so the whole output is flagged rather than a guessed range.
    let reconstructed = match fidelity {
        Fidelity::Lossless => Vec::new(),
        Fidelity::Reconstructed => std::iter::once(0..u64::from(height)).collect(),
    };

    Ok(Some(Candidate {
        strategy: name.to_string(),
        data: Payload::Bytes(assemble(ctx.data, analysis, height, cut)?),
        score: Score {
            content_recovered: u64::from(height),
            content_total: Some(u64::from(frame.height)),
            decoder_errors: u32::from(fidelity == Fidelity::Reconstructed),
            decodes_cleanly: true,
            streams_complete: true,
        },
        fidelity,
        reconstructed,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::file_fixer::corrupt::{ByteSpan, DamageKind, DamageSpec, Fraction, apply_damage};
    use crate::file_fixer::engine::{RepairOptions, RepairResult, Support, repair_data};
    use crate::file_fixer::mode::Mode;
    use crate::file_fixer::report::Outcome;

    fn sample() -> Vec<u8> {
        std::fs::read("tmp/Rome.jpg").unwrap_or_default()
    }

    fn bytes(candidate: &Candidate) -> Vec<u8> {
        candidate.data.as_bytes().expect("image payloads stay in memory").to_vec()
    }

    fn repair(data: &[u8], mode: Mode) -> RepairResult {
        let options = RepairOptions { mode, ..RepairOptions::default() };
        repair_data(std::path::Path::new("sample.jpg"), data, &options)
    }

    /// Decoded through an independent decoder, so a bug in our own scanner cannot mask itself.
    fn decode(data: &[u8]) -> Option<(usize, usize, Vec<u8>)> {
        let mut decoder = zune_jpeg::JpegDecoder::new(zune_jpeg::zune_core::bytestream::ZCursor::new(data));
        let pixels = decoder.decode().ok()?;
        let (width, height) = decoder.dimensions()?;
        Some((width, height, pixels))
    }

    fn truncated(percent: usize) -> Vec<u8> {
        let original = sample();
        let keep = Fraction::new(percent as f64 / 100.0).unwrap();
        apply_damage(&original, &DamageSpec::new(DamageKind::TruncateTail { keep }, 1)).data
    }

    /// Found in a carved collection: three perfectly good photographs carried no `DHT` segment
    /// at all, relying on the tables the specification publishes, and were reported
    /// `unrecoverable` - a file with no damage told it was beyond repair.
    ///
    /// The fixture is built from the sample rather than shipped, and it is only meaningful if
    /// the sample's own tables *are* the standard ones; `sample_with_standard_tables` checks
    /// that by scanning the stripped file, which desynchronizes within a few blocks otherwise.
    #[test]
    fn a_file_relying_on_the_standard_huffman_tables_is_not_called_damaged() {
        let Some(original) = sample_with_standard_tables() else {
            return;
        };
        let stripped = super::super::jpeg_tables::without_huffman_tables(&original);
        assert!(stripped.len() < original.len(), "a DHT segment must have been removed");

        assert_eq!(
            Analysis::of(&stripped).map(|analysis| analysis.integrity),
            Some(Integrity::Whole),
            "the standard tables must read the scan to its end"
        );
        assert_eq!(repair(&stripped, Mode::Balanced).report.outcome, Outcome::FullyRepaired);
    }

    /// A JPEG encoded with the specification's tables, so that removing `DHT` leaves a file
    /// that is still decodable. `None` when ffmpeg is not available to make one.
    fn sample_with_standard_tables() -> Option<Vec<u8>> {
        let target = std::env::temp_dir().join("file_fixer-standard-tables.jpg");
        let made = std::process::Command::new("ffmpeg")
            .args(["-v", "error", "-y", "-f", "lavfi", "-i", "testsrc2=size=160x120", "-frames:v", "1", "-huffman", "default"])
            .arg(&target)
            .output()
            .ok()?;
        made.status.success().then(|| std::fs::read(&target).ok())?
    }

    #[test]
    fn a_healthy_file_is_returned_untouched() {
        let original = sample();
        if original.is_empty() {
            return;
        }
        let result = repair(&original, Mode::Balanced);

        assert_eq!(result.report.outcome, Outcome::FullyRepaired);
        assert_eq!(result.report.winning_strategy.as_deref(), Some("jpeg_verbatim"));
        assert_eq!(bytes(&result.winner.unwrap()), original);
    }

    #[test]
    fn a_truncated_file_is_cropped_to_whole_mcu_rows() {
        if sample().is_empty() {
            return;
        }
        for percent in [90usize, 70, 50, 30] {
            let damaged = truncated(percent);
            let winner = repair(&damaged, Mode::Balanced).winner.unwrap_or_else(|| panic!("{percent}% should still yield rows"));

            let (width, height, _) = decode(&bytes(&winner)).unwrap_or_else(|| panic!("{percent}% must decode"));
            assert_eq!(width, 201);
            assert!(height > 0 && height < 203, "{percent}% gave {height} rows");
            // A JPEG can only describe whole MCU rows, and this file's MCUs are 16 px tall.
            assert_eq!(height % 16, 0, "{percent}% gave {height}, not a whole MCU row count");
        }
    }

    /// The promise of the lossless modes: recovered pixels are the original pixels.
    ///
    /// The final row is excluded and checked separately - see the module documentation on
    /// chroma upsampling at the new bottom edge.
    #[test]
    fn recovered_pixels_are_bit_identical_to_the_original() {
        let original = sample();
        if original.is_empty() {
            return;
        }
        let (width, _, reference) = decode(&original).expect("the sample decodes");
        let stride = width * 3;

        for percent in [90usize, 70, 50] {
            let damaged = truncated(percent);
            let winner = repair(&damaged, Mode::Balanced).winner.expect("rows survive");
            let (_, height, pixels) = decode(&bytes(&winner)).expect("the crop decodes");

            assert_eq!(pixels.len(), stride * height, "{percent}%");
            let compared = stride * (height - 1);
            assert_eq!(pixels[..compared], reference[..compared], "{percent}% altered real content");
        }
    }

    /// Pins down how far the upsampling edge effect reaches. If a change ever makes more than
    /// the single boundary row differ, that is a real regression and not an edge effect.
    #[test]
    fn only_the_final_row_is_affected_by_the_upsampling_edge() {
        let original = sample();
        if original.is_empty() {
            return;
        }
        let (width, _, reference) = decode(&original).expect("the sample decodes");
        let stride = width * 3;

        for percent in [90usize, 70, 50] {
            let damaged = truncated(percent);
            let winner = repair(&damaged, Mode::Balanced).winner.expect("rows survive");
            let (_, height, pixels) = decode(&bytes(&winner)).expect("the crop decodes");

            let differing = (0..height)
                .filter(|&y| pixels[y * stride..(y + 1) * stride] != reference[y * stride..(y + 1) * stride])
                .collect::<Vec<_>>();
            assert!(
                differing.is_empty() || differing == vec![height - 1],
                "{percent}% differs on rows {differing:?} of {height}"
            );
        }
    }

    #[test]
    fn safe_mode_refuses_to_shrink_the_canvas() {
        if sample().is_empty() {
            return;
        }
        let result = repair(&truncated(50), Mode::Safe);
        assert_eq!(result.report.outcome, Outcome::Unrecoverable);
        assert!(result.winner.is_none());
    }

    #[test]
    fn a_missing_end_marker_alone_costs_no_content() {
        let original = sample();
        if original.is_empty() {
            return;
        }
        let clipped = &original[..original.len() - 2];
        assert_eq!(&original[original.len() - 2..], &[0xFF, jpeg_markers::EOI]);

        let result = repair(clipped, Mode::Safe);
        let winner = result.winner.expect("no content is missing");

        assert_eq!(result.report.winning_strategy.as_deref(), Some("jpeg_append_eoi"));
        assert_eq!(decode(&bytes(&winner)).map(|(w, h, _)| (w, h)), Some((201, 203)));
    }

    fn zeroed_mid_file() -> Vec<u8> {
        apply_damage(
            &sample(),
            &DamageSpec::new(
                DamageKind::ZeroRange {
                    at: Fraction::HALF,
                    len: ByteSpan::Bytes(4096),
                },
                1,
            ),
        )
        .data
    }

    #[test]
    fn a_desynchronized_file_yields_nothing_in_the_lossless_modes() {
        if sample().is_empty() {
            return;
        }
        assert!(repair(&zeroed_mid_file(), Mode::Balanced).winner.is_none());
    }

    #[test]
    fn aggressive_mode_salvages_a_desynchronized_file_but_flags_it() {
        if sample().is_empty() {
            return;
        }
        let winner = repair(&zeroed_mid_file(), Mode::Aggressive).winner.expect("aggressive mode should still salvage something");

        assert_eq!(winner.strategy, "jpeg_crop_unverified");
        assert!(!winner.is_lossless());
        assert!(!winner.reconstructed.is_empty());
        assert!(decode(&bytes(&winner)).is_some());
    }

    /// Variants the entropy scanner does not understand must be reported as unsupported, not
    /// as damage. `zune-jpeg` decodes a *damaged* progressive file without complaint - measured
    /// on files from a decoder's own crash-test suite - so a decoder cannot stand in as proof
    /// of health here.
    #[test]
    fn unhandled_variants_are_reported_as_unsupported() {
        let original = sample();
        if original.is_empty() {
            return;
        }
        let layout = jpeg_markers::parse(&original);
        // `height_offset` is the segment payload plus one; the marker byte sits four before it.
        let frame_offset = layout.frame.expect("a frame").height_offset - 4;
        assert_eq!(original[frame_offset], 0xC0, "the fixture must start out baseline");

        for (marker, expected) in [(0xC2u8, "progressive"), (0xC3, "lossless")] {
            let mut variant = original.clone();
            variant[frame_offset] = marker;

            let support = JpegRepairer.supports(&variant);
            let Support::Unsupported(reason) = support else {
                panic!("marker {marker:#x} should be unsupported");
            };
            assert!(reason.contains(expected), "{reason}");

            let result = repair(&variant, Mode::Aggressive);
            assert_eq!(result.report.outcome, Outcome::Unsupported);
            assert!(result.winner.is_none());
        }
    }

    #[test]
    fn a_destroyed_header_yields_nothing_rather_than_garbage() {
        let original = sample();
        if original.is_empty() {
            return;
        }
        let damaged = apply_damage(&original, &DamageSpec::new(DamageKind::HeaderNoise { len: ByteSpan::Bytes(700) }, 5));
        assert!(repair(&damaged.data, Mode::Balanced).winner.is_none());
    }

    #[test]
    fn every_truncation_length_is_handled_without_panicking() {
        let original = sample();
        if original.is_empty() {
            return;
        }
        for len in (0..original.len()).step_by(97) {
            let _ = repair(&original[..len], Mode::Aggressive);
        }
    }
}
