//! GIF repair.
//!
//! What a GIF can prove about itself is unusually good for a format with no checksum: frames
//! are length-delimited by sub-block chains, and LZW codes are not self-synchronizing, so a
//! frame either decodes to exactly the pixel count its descriptor promised or it does not
//! decode at all. Frames that pass that test are original content; nothing here has to guess
//! whether they are.
//!
//! That gives a clean ladder for the damage that actually happens to GIFs. A download cut
//! short loses the trailer (repaired by adding one byte, with nothing discarded), or ends in
//! the middle of a frame (the frames before it are still whole, and the partial frame's rows
//! are still real, which only aggressive mode may keep since a viewer will fill the rest with
//! whatever the previous frame left there). Damage to the first sector destroys the palette
//! and canvas size, and the frames after it describe themselves well enough to be found by
//! scanning - but their colours are gone, so that result is honestly reconstructed.
//!
//! See [`super::gif_blocks`] for the block walk and the pixel-count measurement.

use super::gif_blocks::{self, Frame, Screen, Stop, Walk};
use crate::file_fixer::candidate::{Candidate, Fidelity, Payload, Score};
use crate::file_fixer::engine::{RepairContext, Repairer, Strategy};
use crate::file_fixer::error::Result;
use crate::file_fixer::format::Format;
use crate::file_fixer::mode::Capabilities;

pub struct GifRepairer;

impl Repairer for GifRepairer {
    fn format(&self) -> Format {
        Format::Gif
    }

    fn strategies(&self, _ctx: &RepairContext<'_>) -> Vec<Box<dyn Strategy>> {
        vec![
            Box::new(Verbatim),
            Box::new(AddTrailer),
            Box::new(TruncateToLastWholeFrame),
            Box::new(KeepPartialFrame),
            Box::new(RebuildHeader),
        ]
    }
}

/// The screen descriptor plus the block chain it introduces.
fn survey(data: &[u8]) -> Option<(Screen, Walk)> {
    let screen = gif_blocks::parse_screen(data)?;
    let walk = gif_blocks::walk(data, screen.blocks_start());
    Some((screen, walk))
}

fn score_of(frames: usize, total: Option<usize>, complete: bool) -> Score {
    Score {
        content_recovered: frames as u64,
        content_total: total.map(|total| total as u64),
        decoder_errors: 0,
        decodes_cleanly: true,
        streams_complete: complete,
    }
}

/// Every block parsed, every frame decoded to its full size, the stream ended on its trailer.
struct Verbatim;

impl Strategy for Verbatim {
    fn name(&self) -> &str {
        "gif_verbatim"
    }

    fn run(&self, ctx: &RepairContext<'_>) -> Result<Option<Candidate>> {
        let Some((_, walk)) = survey(ctx.data) else {
            return Ok(None);
        };
        if !walk.is_whole() || walk.frames() == 0 {
            return Ok(None);
        }

        Ok(Some(Candidate {
            strategy: self.name().to_string(),
            data: Payload::Bytes(ctx.data.to_vec()),
            score: score_of(walk.frames(), Some(walk.frames()), true),
            fidelity: Fidelity::Lossless,
            reconstructed: Vec::new(),
        }))
    }
}

/// The file holds whole frames and simply stops: the one byte that says "this is the end"
/// never arrived. Adding it discards nothing, so this works in safe mode.
struct AddTrailer;

impl Strategy for AddTrailer {
    fn name(&self) -> &str {
        "gif_add_trailer"
    }

    fn run(&self, ctx: &RepairContext<'_>) -> Result<Option<Candidate>> {
        let Some((_, walk)) = survey(ctx.data) else {
            return Ok(None);
        };
        // Anything left over is an incomplete block, and dropping it is a truncation that
        // belongs to the strategy below rather than to this one.
        if walk.stop == Stop::Trailer || walk.end < ctx.data.len() || walk.frames() == 0 {
            return Ok(None);
        }
        if walk.frames() != walk.whole_frames() {
            return Ok(None);
        }

        let mut out = ctx.data.to_vec();
        out.push(gif_blocks::TRAILER);
        Ok(Some(Candidate {
            strategy: self.name().to_string(),
            data: Payload::Bytes(out),
            score: score_of(walk.frames(), Some(walk.frames()), true),
            fidelity: Fidelity::Lossless,
            reconstructed: Vec::new(),
        }))
    }
}

/// Cut after the last frame that decoded completely and close the stream there.
struct TruncateToLastWholeFrame;

impl Strategy for TruncateToLastWholeFrame {
    fn name(&self) -> &str {
        "gif_truncate"
    }

    fn requires(&self) -> Capabilities {
        Capabilities {
            truncate: true,
            ..Capabilities::NONE
        }
    }

    fn run(&self, ctx: &RepairContext<'_>) -> Result<Option<Candidate>> {
        let Some((_, walk)) = survey(ctx.data) else {
            return Ok(None);
        };
        if walk.is_whole() {
            return Ok(None);
        }
        let Some((end, kept)) = walk.last_whole_frame_end() else {
            return Ok(None);
        };
        if end >= ctx.data.len() && walk.stop == Stop::Trailer {
            return Ok(None);
        }
        let Some(prefix) = ctx.data.get(..end) else {
            return Ok(None);
        };

        let mut out = prefix.to_vec();
        out.push(gif_blocks::TRAILER);
        Ok(Some(Candidate {
            strategy: self.name().to_string(),
            data: Payload::Bytes(out),
            // How many frames the file was meant to hold died with the bytes that are gone.
            score: score_of(kept, None, false),
            fidelity: Fidelity::Lossless,
            reconstructed: Vec::new(),
        }))
    }
}

/// Keep the rows of the frame the file stops inside.
///
/// The compressed data that arrived is original, and the codes that decoded from it are real
/// pixels. What makes this reconstructed is the rest of the frame: the viewer paints it with
/// whatever was on the canvas before, which for a single-frame GIF means the background. That
/// is an invention, so it needs permission - and it is the only thing that recovers anything
/// at all from a still image that was cut in half, which is the common case.
struct KeepPartialFrame;

impl Strategy for KeepPartialFrame {
    fn name(&self) -> &str {
        "gif_keep_partial_frame"
    }

    fn requires(&self) -> Capabilities {
        Capabilities {
            truncate: true,
            guess: true,
            ..Capabilities::NONE
        }
    }

    fn run(&self, ctx: &RepairContext<'_>) -> Result<Option<Candidate>> {
        let Some((_, walk)) = survey(ctx.data) else {
            return Ok(None);
        };
        let Some(damaged) = first_damaged_frame(ctx.data, &walk) else {
            return Ok(None);
        };
        let Some(partial) = gif_blocks::read_partial_image(ctx.data, damaged.offset) else {
            return Ok(None);
        };
        if partial.frame.rows_decoded() == 0 {
            return Ok(None);
        }

        let Some(prefix) = ctx.data.get(..partial.cut) else {
            return Ok(None);
        };
        let mut out = prefix.to_vec();
        // Terminate the sub-block chain the writer never got to finish.
        out.push(0);

        // Blocks past the damaged frame parsed and measured fine, so they are kept rather
        // than thrown away with it - damage in the middle of an animation costs one frame,
        // not everything after it.
        if let Some(tail) = damaged.block_end.and_then(|end| ctx.data.get(end..walk.end)) {
            out.extend_from_slice(tail);
        }
        if out.last() != Some(&gif_blocks::TRAILER) {
            out.push(gif_blocks::TRAILER);
        }

        let kept = walk.whole_frames() + 1;
        Ok(Some(Candidate {
            strategy: self.name().to_string(),
            data: Payload::Bytes(out),
            score: score_of(kept, None, false),
            fidelity: Fidelity::Reconstructed,
            reconstructed: std::iter::once(damaged.frames_before as u64..damaged.frames_before as u64 + 1).collect(),
        }))
    }
}

/// The first frame whose pixels are not all there, and what surrounds it.
struct DamagedFrame {
    offset: usize,
    /// End of the block, when the block itself parsed and only its codes are bad. `None` when
    /// the file ends inside it and there is nothing after.
    block_end: Option<usize>,
    frames_before: usize,
}

fn first_damaged_frame(data: &[u8], walk: &Walk) -> Option<DamagedFrame> {
    let mut frames_before = 0;
    for block in &walk.blocks {
        match block.frame() {
            Some(frame) if !frame.is_whole() => {
                return Some(DamagedFrame {
                    offset: block.range.start,
                    block_end: Some(block.range.end),
                    frames_before,
                });
            }
            Some(_) => frames_before += 1,
            None => {}
        }
    }

    // Nothing in the walk was damaged, so the damage is in whatever the file ends inside.
    match walk.stop {
        Stop::Truncated { at } if data.get(at).copied() == Some(gif_blocks::IMAGE_SEPARATOR) => Some(DamagedFrame {
            offset: at,
            block_end: None,
            frames_before,
        }),
        Stop::Truncated { .. } | Stop::Corrupt { .. } | Stop::Trailer => None,
    }
}

/// Offsets past this point are not searched for a surviving frame. Damage that pushes the
/// first one further than this is beyond what scanning can help with.
const SCAN_LIMIT: usize = 1 << 20;

/// Descriptors worth measuring before giving up. Each measurement expands a whole LZW stream,
/// so an unbounded scan over a hostile file would be arbitrarily expensive.
const MAX_SCANNED_FRAMES: usize = 64;

/// The signature and screen descriptor are gone, and with them the canvas size and the palette.
///
/// The frames after the damage describe themselves - position, size, code width - and the
/// pixel-count check confirms which candidate offsets are really frames rather than bytes that
/// happen to start with `0x2C`. What cannot be recovered is the global colour table: unless a
/// frame carries its own, the recovered image is the original shape in invented colours, which
/// is why this is never called lossless.
struct RebuildHeader;

impl Strategy for RebuildHeader {
    fn name(&self) -> &str {
        "gif_rebuild_header"
    }

    fn requires(&self) -> Capabilities {
        Capabilities {
            guess: true,
            ..Capabilities::NONE
        }
    }

    fn run(&self, ctx: &RepairContext<'_>) -> Result<Option<Candidate>> {
        if survey(ctx.data).is_some_and(|(_, walk)| walk.frames() > 0) {
            return Ok(None);
        }
        let Some((offset, frame)) = find_frame(ctx) else {
            return Ok(None);
        };
        let Some(blocks) = ctx.data.get(offset..) else {
            return Ok(None);
        };

        let walk = gif_blocks::walk(ctx.data, offset);
        let kept = walk.whole_frames().max(1);
        let end = walk.last_whole_frame_end().map_or(ctx.data.len(), |(end, _)| end.max(offset));

        let mut out = header_for(&frame);
        out.extend_from_slice(blocks.get(..end - offset).unwrap_or(blocks));
        out.push(gif_blocks::TRAILER);

        Ok(Some(Candidate {
            strategy: self.name().to_string(),
            data: Payload::Bytes(out),
            score: score_of(kept, None, false),
            fidelity: Fidelity::Reconstructed,
            reconstructed: std::iter::once(0..kept as u64).collect(),
        }))
    }
}

/// The first offset holding something that really is a frame: a plausible descriptor whose
/// codes expand to at least one full row of the size it claims.
fn find_frame(ctx: &RepairContext<'_>) -> Option<(usize, Frame)> {
    let window = ctx.data.get(..SCAN_LIMIT.min(ctx.data.len()))?;
    let mut measured = 0;

    for offset in 0..window.len() {
        if window.get(offset).copied() != Some(gif_blocks::IMAGE_SEPARATOR) {
            continue;
        }
        let Some(descriptor) = gif_blocks::read_descriptor(ctx.data, offset) else {
            continue;
        };
        measured += 1;
        if measured > MAX_SCANNED_FRAMES || ctx.should_stop() {
            return None;
        }

        let partial = gif_blocks::read_partial_image(ctx.data, offset)?;
        if partial.frame.rows_decoded() > 0 {
            log::debug!(
                "gif: frame found at {offset} ({}x{}, min code size {})",
                descriptor.width,
                descriptor.height,
                descriptor.min_code_size
            );
            return Some((offset, partial.frame));
        }
    }
    None
}

/// A replacement signature, screen descriptor and greyscale palette.
///
/// The canvas is sized to hold the frame that was found. The palette is a ramp, which is the
/// honest choice: any other invented table would look equally arbitrary while pretending to
/// be the original.
fn header_for(frame: &Frame) -> Vec<u8> {
    let width = frame.left.saturating_add(frame.width);
    let height = frame.top.saturating_add(frame.height);

    let mut out = Vec::with_capacity(gif_blocks::SCREEN_HEADER_LEN + 768);
    out.extend_from_slice(b"GIF89a");
    out.extend_from_slice(&width.to_le_bytes());
    out.extend_from_slice(&height.to_le_bytes());
    // Global table present, 8 bits per colour, 256 entries.
    out.extend_from_slice(&[0xF7, 0x00, 0x00]);
    for entry in 0..=255u8 {
        out.extend_from_slice(&[entry, entry, entry]);
    }
    out
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;
    use crate::file_fixer::corrupt::{ByteSpan, DamageKind, DamageSpec, Fraction, apply_damage};
    use crate::file_fixer::engine::{RepairOptions, RepairResult, repair_data};
    use crate::file_fixer::mode::Mode;
    use crate::file_fixer::report::Outcome;

    /// A GIF with `frames` frames of the given size, built from scratch: a 256-entry greyscale
    /// palette, one graphic control extension per frame, and pixel data that varies down the
    /// image so a partial decode is recognizable.
    fn sample_gif(width: u16, height: u16, frames: usize) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend_from_slice(b"GIF89a");
        out.extend_from_slice(&width.to_le_bytes());
        out.extend_from_slice(&height.to_le_bytes());
        out.extend_from_slice(&[0xF7, 0x00, 0x00]);
        for entry in 0..=255u8 {
            out.extend_from_slice(&[entry, entry / 2, 255 - entry]);
        }

        for frame in 0..frames {
            out.extend_from_slice(&[0x21, 0xF9, 0x04, 0x00, 0x0A, 0x00, 0x00, 0x00]);
            out.push(gif_blocks::IMAGE_SEPARATOR);
            out.extend_from_slice(&0u16.to_le_bytes());
            out.extend_from_slice(&0u16.to_le_bytes());
            out.extend_from_slice(&width.to_le_bytes());
            out.extend_from_slice(&height.to_le_bytes());
            out.push(0x00);
            out.push(8);

            let pixels: Vec<u8> = (0..usize::from(width) * usize::from(height))
                .map(|index| ((index / usize::from(width) + frame * 7) % 256) as u8)
                .collect();
            out.extend_from_slice(&gif_blocks::sub_blocks(&gif_blocks::encode_uncompressed(&pixels, 8)));
        }

        out.push(gif_blocks::TRAILER);
        out
    }

    fn repair(data: &[u8], mode: Mode) -> RepairResult {
        let options = RepairOptions { mode, ..RepairOptions::default() };
        repair_data(Path::new("sample.gif"), data, &options)
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
    fn a_healthy_file_walks_end_to_end_and_is_left_alone() {
        let original = sample_gif(32, 24, 3);
        let (screen, walk) = survey(&original).expect("the sample parses");

        assert_eq!((screen.width, screen.height), (32, 24));
        assert_eq!(walk.frames(), 3);
        assert!(walk.is_whole());

        let result = repair(&original, Mode::Balanced);
        assert_eq!(result.report.outcome, Outcome::FullyRepaired);
        assert_eq!(result.report.winning_strategy.as_deref(), Some("gif_verbatim"));
        assert_eq!(bytes(&result), original);
    }

    /// Every frame decoded to exactly the pixel count it promised - the only guarantee the
    /// format offers, and the one the repair ladder is built on.
    #[test]
    fn every_frame_of_a_healthy_file_measures_its_declared_size() {
        let original = sample_gif(40, 30, 2);
        let (_, walk) = survey(&original).expect("parses");

        for block in &walk.blocks {
            if let Some(frame) = block.frame() {
                assert_eq!(frame.pixels, 1200, "40 x 30 pixels");
                assert!(frame.is_whole());
            }
        }
    }

    /// The classic broken GIF: the transfer stopped one byte short of the end.
    #[test]
    fn a_missing_trailer_is_added_back_without_discarding_anything() {
        let mut data = sample_gif(32, 24, 2);
        data.pop();

        let result = repair(&data, Mode::Safe);
        assert_eq!(result.report.winning_strategy.as_deref(), Some("gif_add_trailer"));
        assert_eq!(result.report.outcome, Outcome::FullyRepaired);

        let fixed = bytes(&result);
        assert_eq!(fixed.get(..data.len()), Some(&data[..]), "nothing before the end moved");
        assert!(survey(&fixed).expect("parses").1.is_whole());
    }

    #[test]
    fn a_file_cut_inside_a_later_frame_keeps_the_frames_before_it() {
        let original = sample_gif(64, 64, 4);
        let damaged = apply_damage(
            &original,
            &DamageSpec::new(
                DamageKind::TruncateTail {
                    keep: Fraction::new(0.7).unwrap(),
                },
                1,
            ),
        );

        let result = repair(&damaged.data, Mode::Balanced);
        assert_eq!(result.report.winning_strategy.as_deref(), Some("gif_truncate"));
        assert_eq!(result.report.outcome, Outcome::PartiallyRecovered);

        let fixed = bytes(&result);
        let (_, walk) = survey(&fixed).expect("the repaired file parses");
        assert!(walk.is_whole(), "the result must itself be a whole GIF");
        assert!(walk.frames() > 0 && walk.frames() < 4, "{}", walk.frames());
        assert_eq!(fixed.get(..fixed.len() - 1), damaged.data.get(..fixed.len() - 1), "every kept byte is the original one");
    }

    /// A still image cut in half has no whole frame at all, so the lossless ladder recovers
    /// nothing and only aggressive mode salvages the rows that did arrive.
    #[test]
    fn a_single_frame_cut_in_half_is_only_salvaged_with_permission() {
        let original = sample_gif(64, 64, 1);
        let damaged = apply_damage(
            &original,
            &DamageSpec::new(
                DamageKind::TruncateTail {
                    keep: Fraction::new(0.5).unwrap(),
                },
                1,
            ),
        );

        assert!(repair(&damaged.data, Mode::Balanced).winner.is_none());

        let result = repair(&damaged.data, Mode::Aggressive);
        let winner = result.winner.as_ref().expect("rows did arrive");
        assert_eq!(winner.strategy, "gif_keep_partial_frame");
        assert!(!winner.is_lossless());
        assert_eq!(winner.reconstructed, vec![0..1]);

        let (_, walk) = survey(&bytes(&result)).expect("the repaired file parses");
        assert_eq!(walk.stop, Stop::Trailer);
        let frame = walk.blocks.iter().find_map(gif_blocks::Block::frame).expect("a frame");
        assert!(frame.rows_decoded() > 0, "some rows must have decoded");
        assert!(frame.rows_decoded() < 64, "not all of them");
    }

    /// Damage in the middle of an animation costs the frame it landed in, not everything
    /// after it: the later frames still measure whole and are carried over unchanged.
    ///
    /// The damage is written by hand rather than taken from the corpus, and with `0xFF` rather
    /// than zeros, because the sample's deliberately incompressible stream absorbs zeros: a
    /// run of zero bytes is a run of valid one-pixel codes, so the frame still measures its
    /// full size. Real LZW data does not have that property - a real GIF with a zeroed run is
    /// caught - but the fixture would quietly test nothing.
    #[test]
    fn frames_after_a_damaged_one_are_kept_rather_than_thrown_away() {
        let mut data = sample_gif(32, 32, 5);
        let second_frame = survey(&data).expect("parses").1.blocks[3].range.clone();
        let at = second_frame.start + 200;
        if let Some(range) = data.get_mut(at..at + 64) {
            range.fill(0xFF);
        }

        let (_, walk) = survey(&data).expect("parses");
        assert_eq!(walk.whole_frames(), 4, "exactly one frame must have been lost");

        let result = repair(&data, Mode::Aggressive);
        let winner = result.winner.as_ref().expect("most frames survived");
        assert_eq!(winner.strategy, "gif_keep_partial_frame");
        assert_eq!(winner.score.content_recovered, 5, "four whole frames plus the partial one");
        assert_eq!(winner.reconstructed, vec![1..2], "the second frame is the damaged one");

        let (_, repaired) = survey(&bytes(&result)).expect("the repaired file parses");
        assert_eq!(repaired.stop, Stop::Trailer);
        assert_eq!(repaired.whole_frames(), 4, "every frame that was whole must still be whole");
    }

    /// More surviving bytes must never yield fewer frames.
    #[test]
    fn recovery_grows_with_the_amount_of_surviving_data() {
        let original = sample_gif(32, 32, 8);
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
            let recovered = repair(&damaged.data, Mode::Balanced).report.content_recovered.unwrap_or(0);
            assert!(recovered >= previous, "keep {keep} gave {recovered} after {previous}");
            previous = recovered;
        }
    }

    #[test]
    fn a_destroyed_header_is_rebuilt_from_a_frame_that_describes_itself() {
        let original = sample_gif(48, 36, 2);
        let damaged = apply_damage(&original, &DamageSpec::new(DamageKind::HeaderNoise { len: ByteSpan::Bytes(64) }, 7));

        assert!(repair(&damaged.data, Mode::Balanced).winner.is_none());

        let result = repair(&damaged.data, Mode::Aggressive);
        let winner = result.winner.as_ref().expect("the frames survived");
        assert_eq!(winner.strategy, "gif_rebuild_header");
        assert!(!winner.is_lossless(), "the palette is invented");

        let fixed = bytes(&result);
        let (screen, walk) = survey(&fixed).expect("the rebuilt file parses");
        assert_eq!((screen.width, screen.height), (48, 36));
        assert!(walk.frames() >= 1);
        assert!(walk.is_whole());
    }

    /// A frame's pixel count is the format's only checksum; damage inside the codes must not
    /// pass as content.
    #[test]
    fn corrupted_codes_are_not_mistaken_for_pixels() {
        let mut data = sample_gif(64, 64, 2);
        let middle = data.len() / 2;
        if let Some(range) = data.get_mut(middle..middle + 256) {
            range.fill(0xFF);
        }

        let (_, walk) = survey(&data).expect("parses");
        assert!(!walk.is_whole(), "a damaged frame must not measure as whole");

        let result = repair(&data, Mode::Balanced);
        assert_ne!(result.report.outcome, Outcome::Healthy);
        if let Some(winner) = &result.winner {
            assert!(winner.score.content_recovered < 2, "the damaged frame must not count");
        }
    }

    #[test]
    fn safe_mode_will_not_shorten_an_animation() {
        let original = sample_gif(32, 32, 6);
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
    fn short_and_hostile_inputs_never_panic() {
        let original = sample_gif(16, 16, 2);
        for len in 0..original.len().min(200) {
            let mut data = original.clone();
            data.truncate(len);
            let _ = repair(&data, Mode::Aggressive);
        }
        let _ = repair(b"GIF89a\xff\xff\xff\xff\xf7\x00\x00", Mode::Aggressive);
    }
}
