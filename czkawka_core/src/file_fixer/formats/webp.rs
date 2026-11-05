//! WebP repair.
//!
//! WebP is RIFF on the outside and VP8 on the inside, and this repairs the outside. That is
//! not a shortcut: the damage that makes a WebP unopenable is almost always structural - a
//! declared length that no longer matches the file, an animation whose last frame was never
//! finished, a first sector that took the twelve-byte header with it while every image chunk
//! after it survived intact.
//!
//! What is deliberately *not* attempted is salvaging a partially written VP8 bitstream. A
//! still image cut in half has one chunk, and half of it decodes to nothing without a VP8
//! decoder carrying its own probability state; claiming otherwise would mean handing back a
//! file that looks repaired and shows nothing. An animation cut in half is a different matter:
//! the frames before the cut are whole chunks, and those come back.

use super::riff::{self, Chunk, Riff};
use crate::file_fixer::candidate::{Candidate, Fidelity, Payload, Score};
use crate::file_fixer::engine::{RepairContext, Repairer, Strategy};
use crate::file_fixer::error::Result;
use crate::file_fixer::format::Format;
use crate::file_fixer::io::{put_u32_le, read_u16_le};
use crate::file_fixer::mode::Capabilities;

pub struct WebPRepairer;

impl Repairer for WebPRepairer {
    fn format(&self) -> Format {
        Format::WebP
    }

    fn strategies(&self, _ctx: &RepairContext<'_>) -> Vec<Box<dyn Strategy>> {
        vec![Box::new(Verbatim), Box::new(FixSize), Box::new(TruncateToWholeChunks), Box::new(RebuildHeader)]
    }
}

const LOSSY: [u8; 4] = *b"VP8 ";
const LOSSLESS: [u8; 4] = *b"VP8L";
const EXTENDED: [u8; 4] = *b"VP8X";
const ANIMATION_FRAME: [u8; 4] = *b"ANMF";

/// The three-byte start code every lossy keyframe carries after its frame tag.
const VP8_KEYFRAME_SYNC: [u8; 3] = [0x9D, 0x01, 0x2A];
/// The one-byte signature a lossless bitstream starts with.
const VP8L_SIGNATURE: u8 = 0x2F;

/// A parsed file: the chunk chain plus what it adds up to.
struct Survey {
    riff: Riff,
}

impl Survey {
    fn of(data: &[u8]) -> Option<Self> {
        let riff = riff::parse(data)?;
        (riff.form == *b"WEBP").then_some(Self { riff })
    }

    /// Chunks that are entirely present, in file order.
    fn whole_chunks(&self) -> impl Iterator<Item = &Chunk> {
        self.riff.chunks.iter().take_while(|chunk| chunk.complete)
    }

    /// Pictures the file holds: one for a still, one per frame for an animation.
    fn frames(&self) -> usize {
        let animated = self.whole_chunks().filter(|chunk| chunk.is(ANIMATION_FRAME)).count();
        if animated > 0 {
            return animated;
        }
        usize::from(self.whole_chunks().any(|chunk| chunk.is(LOSSY) || chunk.is(LOSSLESS)))
    }

    /// True when the outer length field covers exactly the chunks that are there.
    ///
    /// Equality, not "at least": a length field that stops short makes a decoder ignore every
    /// chunk past it, which for an animation means silently losing frames that are present.
    fn is_consistent(&self, len: usize) -> bool {
        !self.riff.has_incomplete_chunk() && !self.riff.promises_more_than(len) && self.frames() > 0 && declared_end(&self.riff) == self.riff.end
    }
}

/// Where the outer size field says the content ends.
fn declared_end(riff: &Riff) -> usize {
    riff.declared_len as usize + 8
}

/// Dimensions a still image's bitstream states about itself, used to confirm that a chunk
/// found by scanning really is one.
fn still_dimensions(data: &[u8], chunk: &Chunk) -> Option<(u32, u32)> {
    let payload = data.get(chunk.payload.clone())?;
    if chunk.is(LOSSY) {
        if payload.get(3..6)? != VP8_KEYFRAME_SYNC {
            return None;
        }
        let width = u32::from(read_u16_le(payload, 6)? & 0x3FFF);
        let height = u32::from(read_u16_le(payload, 8)? & 0x3FFF);
        return (width > 0 && height > 0).then_some((width, height));
    }
    if chunk.is(LOSSLESS) {
        if *payload.first()? != VP8L_SIGNATURE {
            return None;
        }
        let packed = u32::from_le_bytes([*payload.get(1)?, *payload.get(2)?, *payload.get(3)?, *payload.get(4)?]);
        let width = (packed & 0x3FFF) + 1;
        let height = ((packed >> 14) & 0x3FFF) + 1;
        return Some((width, height));
    }
    None
}

/// Rebuilds the file from the chunks that are entirely present.
fn assemble(data: &[u8], chunks: &[&Chunk]) -> Option<Vec<u8>> {
    let mut body = Vec::new();
    for chunk in chunks {
        body.extend_from_slice(data.get(chunk.start..chunk.end())?);
    }
    let mut out = riff::header(*b"WEBP", body.len());
    out.extend_from_slice(&body);
    Some(out)
}

fn score_of(frames: usize, total: Option<usize>) -> Score {
    Score {
        content_recovered: frames as u64,
        content_total: total.map(|total| total as u64),
        decoder_errors: 0,
        decodes_cleanly: true,
        streams_complete: total.is_some_and(|total| total <= frames),
    }
}

/// Every chunk is present and the header agrees with them.
struct Verbatim;

impl Strategy for Verbatim {
    fn name(&self) -> &str {
        "webp_verbatim"
    }

    fn run(&self, ctx: &RepairContext<'_>) -> Result<Option<Candidate>> {
        let Some(survey) = Survey::of(ctx.data) else {
            return Ok(None);
        };
        if !survey.is_consistent(ctx.data.len()) {
            return Ok(None);
        }
        let frames = survey.frames();

        Ok(Some(Candidate {
            strategy: self.name().to_string(),
            data: Payload::Bytes(ctx.data.to_vec()),
            score: score_of(frames, Some(frames)),
            fidelity: Fidelity::Lossless,
            reconstructed: Vec::new(),
        }))
    }
}

/// The chunks are all there and the outer length field disagrees with them. Correcting it
/// discards nothing, so this works in safe mode.
struct FixSize;

impl Strategy for FixSize {
    fn name(&self) -> &str {
        "webp_fix_size"
    }

    fn run(&self, ctx: &RepairContext<'_>) -> Result<Option<Candidate>> {
        let Some(survey) = Survey::of(ctx.data) else {
            return Ok(None);
        };
        if survey.riff.has_incomplete_chunk() || survey.is_consistent(ctx.data.len()) || survey.frames() == 0 {
            return Ok(None);
        }

        let mut out = ctx.data.to_vec();
        put_u32_le(&mut out, 4, u32::try_from(survey.riff.end.saturating_sub(8)).unwrap_or(u32::MAX));
        let frames = survey.frames();

        Ok(Some(Candidate {
            strategy: self.name().to_string(),
            data: Payload::Bytes(out),
            score: score_of(frames, Some(frames)),
            fidelity: Fidelity::Lossless,
            reconstructed: Vec::new(),
        }))
    }
}

/// The file stops inside a chunk. Keep the chunks that are whole.
///
/// For an animation that is most of the frames. For a still it is nothing at all, because the
/// single image chunk is the one that was cut - and saying so is better than writing a file
/// that opens to an empty canvas.
struct TruncateToWholeChunks;

impl Strategy for TruncateToWholeChunks {
    fn name(&self) -> &str {
        "webp_truncate"
    }

    fn requires(&self) -> Capabilities {
        Capabilities {
            truncate: true,
            ..Capabilities::NONE
        }
    }

    fn run(&self, ctx: &RepairContext<'_>) -> Result<Option<Candidate>> {
        let Some(survey) = Survey::of(ctx.data) else {
            return Ok(None);
        };
        if !survey.riff.has_incomplete_chunk() {
            return Ok(None);
        }

        let kept: Vec<&Chunk> = survey.whole_chunks().collect();
        let frames = survey.frames();
        if frames == 0 {
            return Ok(None);
        }
        let Some(out) = assemble(ctx.data, &kept) else {
            return Ok(None);
        };

        Ok(Some(Candidate {
            strategy: self.name().to_string(),
            data: Payload::Bytes(out),
            // Nothing says how many frames the animation was meant to have; the count died
            // with the bytes that are missing.
            score: score_of(frames, None),
            fidelity: Fidelity::Lossless,
            reconstructed: Vec::new(),
        }))
    }
}

/// Offsets past this point are not searched for a surviving image chunk.
const SCAN_LIMIT: usize = 1 << 20;

/// The twelve-byte header is gone. The image chunks after it are not.
///
/// A chunk found by scanning is confirmed by its own bitstream: a lossy keyframe carries a
/// three-byte start code and a lossless one a signature byte, both followed by dimensions that
/// have to be non-zero. That is enough to tell a real chunk from four coincidental bytes.
struct RebuildHeader;

impl Strategy for RebuildHeader {
    fn name(&self) -> &str {
        "webp_rebuild_header"
    }

    fn requires(&self) -> Capabilities {
        Capabilities {
            guess: true,
            ..Capabilities::NONE
        }
    }

    fn run(&self, ctx: &RepairContext<'_>) -> Result<Option<Candidate>> {
        if Survey::of(ctx.data).is_some_and(|survey| survey.frames() > 0) {
            return Ok(None);
        }
        let Some(found) = find_image_chunk(ctx) else {
            return Ok(None);
        };

        // Everything from the image chunk on is a valid chunk chain in its own right.
        let mut body = Vec::new();
        if let Some(bytes) = ctx.data.get(found.start..) {
            body.extend_from_slice(bytes);
        }
        let mut out = riff::header(*b"WEBP", body.len());
        out.extend_from_slice(&body);

        let frames = Survey::of(&out).map_or(1, |survey| survey.frames().max(1));
        Ok(Some(Candidate {
            strategy: self.name().to_string(),
            data: Payload::Bytes(out),
            score: score_of(frames, None),
            // The pixels are original; what was rebuilt is the container around them. The
            // extended header, if the file had one, is gone with its canvas size and flags.
            fidelity: Fidelity::Reconstructed,
            reconstructed: std::iter::once(0..frames as u64).collect(),
        }))
    }
}

fn find_image_chunk(ctx: &RepairContext<'_>) -> Option<Chunk> {
    let limit = SCAN_LIMIT.min(ctx.data.len());
    let mut best: Option<Chunk> = None;

    for id in [LOSSY, LOSSLESS] {
        let mut from = 0;
        while from < limit {
            if ctx.should_stop() {
                return best;
            }
            let Some(chunk) = riff::find_chunk(ctx.data, id, from) else {
                break;
            };
            if chunk.start >= limit {
                break;
            }
            // An image chunk the file ends inside is not recoverable content: half a VP8
            // bitstream decodes to nothing, so wrapping it in a fresh header would produce a
            // file that opens to an empty canvas.
            if chunk.complete && still_dimensions(ctx.data, &chunk).is_some() {
                // An extended file puts VP8X first; keeping it preserves the canvas and the
                // alpha flag, which the image chunk alone does not carry.
                let extended = riff::find_chunk(ctx.data, EXTENDED, 0).filter(|x| x.start < chunk.start);
                let found = extended.unwrap_or(chunk);
                if best.as_ref().is_none_or(|previous| found.start < previous.start) {
                    best = Some(found);
                }
                break;
            }
            from = chunk.start + 1;
        }
    }
    best
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;
    use crate::file_fixer::corrupt::{ByteSpan, DamageKind, DamageSpec, Fraction, apply_damage};
    use crate::file_fixer::engine::{RepairOptions, RepairResult, repair_data};
    use crate::file_fixer::mode::Mode;
    use crate::file_fixer::report::Outcome;

    /// A lossy still, built by hand: the bitstream body is not a real VP8 frame, but the
    /// keyframe start code and the dimensions are, which is exactly what the repairs read.
    fn sample_still(width: u16, height: u16, payload_len: usize) -> Vec<u8> {
        let mut payload = vec![0u8; 10 + payload_len];
        if let Some(tag) = payload.get_mut(0..3) {
            tag.copy_from_slice(&[0x30, 0x01, 0x00]);
        }
        if let Some(sync) = payload.get_mut(3..6) {
            sync.copy_from_slice(&VP8_KEYFRAME_SYNC);
        }
        if let Some(size) = payload.get_mut(6..10) {
            size.copy_from_slice(&[(width & 0xFF) as u8, (width >> 8) as u8, (height & 0xFF) as u8, (height >> 8) as u8]);
        }
        for (index, byte) in payload.iter_mut().enumerate().skip(10) {
            *byte = (index % 251) as u8;
        }

        let mut body = Vec::new();
        riff::append_chunk(&mut body, LOSSY, &payload);
        let mut out = riff::header(*b"WEBP", body.len());
        out.extend_from_slice(&body);
        out
    }

    /// An animation: the extended header, the animation parameters, and `frames` frame chunks.
    fn sample_animation(frames: usize) -> Vec<u8> {
        let mut body = Vec::new();
        let mut extended = vec![0u8; 10];
        if let Some(flags) = extended.first_mut() {
            *flags = 0x02;
        }
        riff::append_chunk(&mut body, EXTENDED, &extended);
        riff::append_chunk(&mut body, *b"ANIM", &[0, 0, 0, 0, 0, 0]);

        for index in 0..frames {
            let still = sample_still(16, 16, 64);
            let mut frame = vec![0u8; 16];
            if let Some(byte) = frame.get_mut(15) {
                *byte = index as u8;
            }
            frame.extend_from_slice(still.get(riff::HEADER_LEN..).unwrap_or_default());
            riff::append_chunk(&mut body, ANIMATION_FRAME, &frame);
        }

        let mut out = riff::header(*b"WEBP", body.len());
        out.extend_from_slice(&body);
        out
    }

    fn repair(data: &[u8], mode: Mode) -> RepairResult {
        let options = RepairOptions { mode, ..RepairOptions::default() };
        repair_data(Path::new("image.webp"), data, &options)
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
    fn a_healthy_still_is_left_alone() {
        let original = sample_still(64, 48, 512);
        let result = repair(&original, Mode::Balanced);

        assert_eq!(result.report.outcome, Outcome::FullyRepaired);
        assert_eq!(result.report.winning_strategy.as_deref(), Some("webp_verbatim"));
        assert_eq!(bytes(&result), original);
    }

    #[test]
    fn the_bitstream_states_its_own_dimensions() {
        let data = sample_still(320, 240, 64);
        let survey = Survey::of(&data).expect("parses");
        let chunk = survey.riff.chunk(LOSSY).expect("the image chunk");
        assert_eq!(still_dimensions(&data, chunk), Some((320, 240)));
    }

    #[test]
    fn a_wrong_outer_length_is_corrected_in_safe_mode() {
        let mut data = sample_still(64, 48, 512);
        put_u32_le(&mut data, 4, 4);

        let result = repair(&data, Mode::Safe);
        assert_eq!(result.report.winning_strategy.as_deref(), Some("webp_fix_size"));
        assert_eq!(result.report.outcome, Outcome::FullyRepaired);

        let fixed = bytes(&result);
        assert_eq!(fixed.len(), data.len(), "not a byte was dropped");
        assert!(Survey::of(&fixed).expect("parses").is_consistent(fixed.len()));
    }

    /// An animation cut short keeps the frames that are whole chunks.
    #[test]
    fn a_truncated_animation_keeps_the_frames_that_arrived() {
        let original = sample_animation(8);
        let damaged = apply_damage(
            &original,
            &DamageSpec::new(
                DamageKind::TruncateTail {
                    keep: Fraction::new(0.6).unwrap(),
                },
                1,
            ),
        );

        let result = repair(&damaged.data, Mode::Balanced);
        assert_eq!(result.report.winning_strategy.as_deref(), Some("webp_truncate"));

        let fixed = bytes(&result);
        let survey = Survey::of(&fixed).expect("the repaired file parses");
        assert!(survey.is_consistent(fixed.len()), "the result must describe itself");
        assert!(survey.frames() > 0 && survey.frames() < 8, "{}", survey.frames());
        assert_eq!(
            fixed.get(riff::HEADER_LEN..),
            damaged.data.get(riff::HEADER_LEN..fixed.len()),
            "every kept byte is the original one"
        );
    }

    /// Half a VP8 bitstream is not half a picture: without the decoder's probability state it
    /// is nothing, and a file that opens to an empty canvas is worse than an honest failure.
    #[test]
    fn a_still_cut_in_half_is_reported_unrecoverable_rather_than_faked() {
        let original = sample_still(64, 48, 4096);
        let damaged = apply_damage(
            &original,
            &DamageSpec::new(
                DamageKind::TruncateTail {
                    keep: Fraction::new(0.5).unwrap(),
                },
                1,
            ),
        );

        let result = repair(&damaged.data, Mode::Aggressive);
        assert_eq!(result.report.outcome, Outcome::Unrecoverable);
        assert!(result.winner.is_none());
    }

    #[test]
    fn a_destroyed_header_is_rebuilt_around_the_surviving_image_chunk() {
        let original = sample_still(64, 48, 2048);
        let damaged = apply_damage(&original, &DamageSpec::new(DamageKind::HeaderNoise { len: ByteSpan::Bytes(8) }, 3));

        assert!(repair(&damaged.data, Mode::Balanced).winner.is_none());

        let result = repair(&damaged.data, Mode::Aggressive);
        let winner = result.winner.as_ref().expect("the image chunk survived");
        assert_eq!(winner.strategy, "webp_rebuild_header");

        let fixed = bytes(&result);
        let survey = Survey::of(&fixed).expect("the rebuilt file parses");
        assert_eq!(survey.frames(), 1);
        let chunk = survey.riff.chunk(LOSSY).expect("the image chunk");
        assert_eq!(still_dimensions(&fixed, chunk), Some((64, 48)));
    }

    #[test]
    fn a_riff_file_that_is_not_webp_is_not_claimed() {
        let mut data = sample_still(16, 16, 64);
        if let Some(form) = data.get_mut(8..12) {
            form.copy_from_slice(b"AVI ");
        }
        assert!(Survey::of(&data).is_none());
    }

    #[test]
    fn short_and_hostile_inputs_never_panic() {
        let original = sample_still(16, 16, 128);
        for len in 0..original.len() {
            let mut data = original.clone();
            data.truncate(len);
            let _ = repair(&data, Mode::Aggressive);
        }
    }
}
