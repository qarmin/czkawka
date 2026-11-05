//! Rebuilding a JPEG whose header is gone.
//!
//! Unlike PNG, a JPEG's geometry cannot be worked out from its own data. The quantization and
//! Huffman tables live in the header, and without them the entropy-coded scan is not merely
//! undescribed but undecodable - there is nothing to derive them from.
//!
//! So the header comes from elsewhere, exactly as the decoder parameters for a video do: a
//! healthy file from the same source. Cameras and phones emit a byte-identical header for every
//! photo taken in the same mode, which makes the splice point the reference's own scan offset.

use super::jpeg_integrity::Analysis;
use super::jpeg_markers;
use crate::file_fixer::candidate::{Candidate, Fidelity, Payload, Score};
use crate::file_fixer::engine::{RepairContext, Strategy};
use crate::file_fixer::error::Result;
use crate::file_fixer::mode::Capabilities;

/// Takes the header from a healthy file of the same provenance and keeps our own scan data.
pub struct RebuildHeader;

impl Strategy for RebuildHeader {
    fn name(&self) -> &str {
        "jpeg_rebuild_header"
    }

    fn requires(&self) -> Capabilities {
        Capabilities {
            truncate: true,
            guess: true,
            ..Capabilities::NONE
        }
    }

    fn run(&self, ctx: &RepairContext<'_>) -> Result<Option<Candidate>> {
        if jpeg_markers::parse(ctx.data).is_scannable() {
            return Ok(None);
        }

        let mut best: Option<Candidate> = None;
        for reference in ctx.references {
            if ctx.should_stop() {
                break;
            }
            let Ok(reference_data) = std::fs::read(reference) else {
                log::warn!("cannot read reference {}", reference.display());
                continue;
            };
            let Some(candidate) = rebuild_from(self.name(), ctx, &reference_data)? else {
                continue;
            };
            if best.as_ref().is_none_or(|current| candidate.score.content_recovered > current.score.content_recovered) {
                best = Some(candidate);
            }
        }
        Ok(best)
    }
}

fn rebuild_from(name: &str, ctx: &RepairContext<'_>, reference: &[u8]) -> Result<Option<Candidate>> {
    let layout = jpeg_markers::parse(reference);
    if !layout.is_scannable() {
        return Ok(None);
    }
    let Some(splice_at) = layout.scans.first().map(|scan| scan.data_offset) else {
        return Ok(None);
    };

    // The reference's header replaces ours; our scan data picks up where its header ended,
    // which lines up only because the same encoder writes the same header every time.
    let (Some(header), Some(entropy)) = (reference.get(..splice_at), ctx.data.get(splice_at..)) else {
        return Ok(None);
    };
    let mut spliced = Vec::with_capacity(header.len() + entropy.len());
    spliced.extend_from_slice(header);
    spliced.extend_from_slice(entropy);

    let Some(analysis) = Analysis::of(&spliced) else {
        return Ok(None);
    };
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

    let rebuilt = super::jpeg_assemble::assemble(&spliced, &analysis, height, cut)?;
    if !decodes(&rebuilt) {
        return Ok(None);
    }

    Ok(Some(Candidate {
        strategy: name.to_string(),
        data: Payload::Bytes(rebuilt),
        score: Score {
            content_recovered: u64::from(height),
            content_total: Some(u64::from(frame.height)),
            decoder_errors: 0,
            decodes_cleanly: true,
            streams_complete: true,
        },
        // The scan data is ours, but the tables describing it are another file's. If that file
        // was encoded differently the colours are wrong in a way nothing here can detect.
        fidelity: Fidelity::Reconstructed,
        reconstructed: std::iter::once(0..u64::from(height)).collect(),
    }))
}

/// Confirms the result is a real image rather than a header wrapped around noise.
fn decodes(data: &[u8]) -> bool {
    zune_jpeg::JpegDecoder::new(zune_jpeg::zune_core::bytestream::ZCursor::new(data))
        .decode()
        .is_ok_and(|pixels| !pixels.is_empty())
}

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};

    use super::*;
    use crate::file_fixer::corrupt::{ByteSpan, DamageKind, DamageSpec, apply_damage};
    use crate::file_fixer::engine::{RepairOptions, repair_data};
    use crate::file_fixer::mode::Mode;
    use crate::file_fixer::report::Outcome;

    fn sample() -> Vec<u8> {
        std::fs::read("tmp/Rome.jpg").unwrap_or_default()
    }

    /// The corpus damage class: enough noise at the front to take the whole header with it.
    fn wrecked_header() -> Vec<u8> {
        apply_damage(&sample(), &DamageSpec::new(DamageKind::HeaderNoise { len: ByteSpan::Bytes(32) }, 1)).data
    }

    fn repair(data: &[u8], mode: Mode, references: Vec<PathBuf>) -> crate::file_fixer::engine::RepairResult {
        let options = RepairOptions {
            mode,
            references,
            ..RepairOptions::default()
        };
        repair_data(Path::new("sample.jpg"), data, &options)
    }

    #[test]
    fn a_wrecked_header_is_unreadable_on_its_own() {
        if sample().is_empty() {
            return;
        }
        assert!(!jpeg_markers::parse(&wrecked_header()).is_scannable());
        assert!(repair(&wrecked_header(), Mode::Aggressive, Vec::new()).winner.is_none());
    }

    #[test]
    fn a_reference_restores_the_header_and_the_image() {
        if sample().is_empty() {
            return;
        }
        let result = repair(&wrecked_header(), Mode::Aggressive, vec![PathBuf::from("tmp/Rome.jpg")]);
        let winner = result.winner.expect("the reference supplies the missing tables");

        assert_eq!(result.report.winning_strategy.as_deref(), Some("jpeg_rebuild_header"));
        assert_eq!(result.report.outcome, Outcome::FullyRepaired);
        assert!(!winner.is_lossless(), "the tables came from another file");
        assert!(!winner.reconstructed.is_empty());

        let pixels = zune_jpeg::JpegDecoder::new(zune_jpeg::zune_core::bytestream::ZCursor::new(winner.data.as_bytes().expect("in memory")))
            .decode()
            .expect("the rebuilt file decodes");
        assert_eq!(pixels.len(), 201 * 203 * 3);
    }

    /// Splicing in another file's tables is a guess about provenance, so the lossless modes
    /// must not do it.
    #[test]
    fn rebuilding_a_header_needs_the_guess_capability() {
        if sample().is_empty() {
            return;
        }
        for mode in [Mode::Safe, Mode::Balanced] {
            let result = repair(&wrecked_header(), mode, vec![PathBuf::from("tmp/Rome.jpg")]);
            assert!(result.winner.is_none(), "{mode}");
        }
    }

    #[test]
    fn a_healthy_file_is_left_to_the_cheaper_strategies() {
        if sample().is_empty() {
            return;
        }
        let result = repair(&sample(), Mode::Aggressive, vec![PathBuf::from("tmp/Rome.jpg")]);
        assert_eq!(result.report.winning_strategy.as_deref(), Some("jpeg_verbatim"));
    }
}
