//! FLAC repair.
//!
//! FLAC is the format this crate can be most honest about. Every frame ends in a CRC-16 over
//! its own bytes and starts with a CRC-8 over its header, so "this frame is the original" is
//! not a judgement call - it is a checksum. The repairs below therefore never have to decide
//! how much decoded output to trust: they keep the frames that verify and drop the ones that
//! do not.
//!
//! Frame headers also repeat the sample rate, channel count and sample width, which means a
//! file whose first sector is gone is not a loss at all. The audio describes itself; only the
//! `STREAMINFO` block has to be written out again, and every sample in it is still the one the
//! encoder produced. That is why the rebuild here is `Lossless` where the equivalent strategy
//! for BMP or WAV is not.
//!
//! What is *not* free is the total sample count and the MD5 of the decoded audio that
//! `STREAMINFO` carries. After frames are dropped neither is true any more, so the count is
//! recomputed and the MD5 is written as zero - the specification's way of saying "not
//! computed", which beats leaving a checksum that now disagrees with the file.

use super::flac_frames::{self, Frame, StreamInfo, Walk};
use crate::file_fixer::candidate::{Candidate, Fidelity, Payload, Score};
use crate::file_fixer::engine::{RepairContext, Repairer, Strategy};
use crate::file_fixer::error::Result;
use crate::file_fixer::format::Format;
use crate::file_fixer::mode::Capabilities;

pub struct FlacRepairer;

impl Repairer for FlacRepairer {
    fn format(&self) -> Format {
        Format::Flac
    }

    fn strategies(&self, _ctx: &RepairContext<'_>) -> Vec<Box<dyn Strategy>> {
        vec![Box::new(Verbatim), Box::new(TruncateToVerifiedFrames), Box::new(ResyncPastDamage), Box::new(RebuildHeader)]
    }
}

/// Gaps worth stepping over before concluding the rest of the file is not audio.
const MAX_GAPS: usize = 64;

/// The stream's own description of itself, plus the frames that verify.
struct Survey {
    info: Option<StreamInfo>,
    audio_start: usize,
    walk: Walk,
}

impl Survey {
    fn of(ctx: &RepairContext<'_>) -> Option<Self> {
        let audio_start = flac_frames::audio_start(ctx.data)?;
        let info = flac_frames::stream_info(ctx.data);
        let max_len = flac_frames::max_frame_len(info.as_ref());

        Some(Self {
            info,
            audio_start,
            walk: flac_frames::walk_frames(ctx, audio_start, max_len),
        })
    }

    /// Samples the stream says it holds, when it said.
    fn declared_samples(&self) -> Option<u64> {
        self.info.map(|info| info.total_samples).filter(|total| *total > 0)
    }

    fn is_whole(&self, len: usize) -> bool {
        self.walk.reaches_end && !self.walk.frames.is_empty() && self.declared_samples().is_none_or(|total| self.walk.samples() >= total) && self.walk.end >= len
    }
}

/// The description to write for a stream that now holds exactly `frames`.
///
/// Frame headers are the authority: they carry the rate, channel count and sample width, and
/// unlike `STREAMINFO` they are checksummed individually, so they survive damage that takes
/// the header out entirely.
fn info_for(frames: &[Frame], walk: &Walk, base: Option<StreamInfo>) -> Option<StreamInfo> {
    let first = frames.first()?;
    let (min_block_size, max_block_size) = walk.block_sizes();
    let (min_frame_size, max_frame_size) = walk.frame_sizes();

    let sample_rate = match first.header.sample_rate {
        0 => base.map_or(ASSUMED_SAMPLE_RATE, |info| info.sample_rate),
        rate => rate,
    };
    let bits_per_sample = match first.header.bits_per_sample {
        0 => base.map_or(16, |info| info.bits_per_sample),
        bits => bits,
    };

    Some(StreamInfo {
        min_block_size,
        max_block_size,
        min_frame_size,
        max_frame_size,
        sample_rate,
        channels: first.header.channels,
        bits_per_sample,
        total_samples: frames.iter().map(Frame::samples).sum(),
    })
}

/// Used only when neither the frames nor a surviving `STREAMINFO` state the rate, which
/// happens when an encoder chose the "see the header" code for a non-standard rate.
const ASSUMED_SAMPLE_RATE: u32 = 44_100;

/// Rebuilds the file from its metadata and a set of verified frames.
///
/// The metadata blocks are copied byte for byte - tags, cover art and seek tables are content
/// too - with only the `STREAMINFO` payload rewritten to describe what is really there.
fn assemble(data: &[u8], audio_start: usize, info: &StreamInfo, frames: &[Frame]) -> Option<Vec<u8>> {
    let mut out = data.get(..audio_start)?.to_vec();
    let block = flac_frames::metadata_blocks(data)
        .into_iter()
        .find(|block| block.kind == flac_frames::STREAMINFO && block.complete);

    match block.filter(|block| block.payload.len() == flac_frames::STREAMINFO_LEN) {
        Some(block) => out.get_mut(block.payload)?.copy_from_slice(&flac_frames::encode_stream_info(info)),
        // No usable description survived, so a fresh one replaces the whole prefix.
        None => out = fresh_metadata(info),
    }

    append_frames(&mut out, data, frames);
    Some(out)
}

/// A minimal metadata section: the marker and a single `STREAMINFO` block.
fn fresh_metadata(info: &StreamInfo) -> Vec<u8> {
    let payload = flac_frames::encode_stream_info(info);
    let mut out = Vec::with_capacity(flac_frames::MAGIC.len() + 4 + payload.len());
    out.extend_from_slice(&flac_frames::MAGIC);
    out.extend_from_slice(&flac_frames::metadata_header(flac_frames::STREAMINFO, true, payload.len()));
    out.extend_from_slice(&payload);
    out
}

fn append_frames(out: &mut Vec<u8>, data: &[u8], frames: &[Frame]) {
    for frame in frames {
        if let Some(bytes) = data.get(frame.range.clone()) {
            out.extend_from_slice(bytes);
        }
    }
}

fn score_of(samples: u64, total: Option<u64>, complete: bool) -> Score {
    Score {
        content_recovered: samples,
        content_total: total.or(Some(samples)),
        decoder_errors: 0,
        decodes_cleanly: true,
        streams_complete: complete,
    }
}

/// Every frame verified and they run to the end of the file.
struct Verbatim;

impl Strategy for Verbatim {
    fn name(&self) -> &str {
        "flac_verbatim"
    }

    fn run(&self, ctx: &RepairContext<'_>) -> Result<Option<Candidate>> {
        let Some(survey) = Survey::of(ctx) else {
            return Ok(None);
        };
        if !survey.is_whole(ctx.data.len()) {
            return Ok(None);
        }

        let samples = survey.walk.samples();
        Ok(Some(Candidate {
            strategy: self.name().to_string(),
            data: Payload::Bytes(ctx.data.to_vec()),
            score: score_of(samples, Some(samples), true),
            fidelity: Fidelity::Lossless,
            reconstructed: Vec::new(),
        }))
    }
}

/// Keep the frames whose checksums hold and cut there.
///
/// This is the whole repair for a truncated download: the frames before the cut are provably
/// the original ones, and the only thing that needs correcting is the sample count the header
/// promised.
struct TruncateToVerifiedFrames;

impl Strategy for TruncateToVerifiedFrames {
    fn name(&self) -> &str {
        "flac_truncate"
    }

    fn requires(&self) -> Capabilities {
        Capabilities {
            truncate: true,
            ..Capabilities::NONE
        }
    }

    fn run(&self, ctx: &RepairContext<'_>) -> Result<Option<Candidate>> {
        let Some(survey) = Survey::of(ctx) else {
            return Ok(None);
        };
        if survey.is_whole(ctx.data.len()) || survey.walk.frames.is_empty() {
            return Ok(None);
        }
        let Some(info) = info_for(&survey.walk.frames, &survey.walk, survey.info) else {
            return Ok(None);
        };
        let Some(out) = assemble(ctx.data, survey.audio_start, &info, &survey.walk.frames) else {
            return Ok(None);
        };

        let samples = survey.walk.samples();
        Ok(Some(Candidate {
            strategy: self.name().to_string(),
            data: Payload::Bytes(out),
            score: score_of(samples, survey.declared_samples(), false),
            fidelity: Fidelity::Lossless,
            reconstructed: Vec::new(),
        }))
    }
}

/// Step over the damage and keep the frames on the far side of it.
///
/// Every frame kept is still checksum-verified, so nothing here is invented - but the result
/// has a hole in it where the damaged frames were, and the audio after the hole plays earlier
/// than it should. That is a real change to the recording even though no byte of it was
/// altered, which is why it needs permission beyond plain truncation.
struct ResyncPastDamage;

impl Strategy for ResyncPastDamage {
    fn name(&self) -> &str {
        "flac_resync"
    }

    fn requires(&self) -> Capabilities {
        Capabilities {
            truncate: true,
            guess: true,
            ..Capabilities::NONE
        }
    }

    fn run(&self, ctx: &RepairContext<'_>) -> Result<Option<Candidate>> {
        let Some(survey) = Survey::of(ctx) else {
            return Ok(None);
        };
        if survey.is_whole(ctx.data.len()) {
            return Ok(None);
        }

        let max_len = flac_frames::max_frame_len(survey.info.as_ref());
        let frames = collect_past_gaps(ctx, survey.walk.frames.clone(), survey.walk.end, max_len);
        if frames.len() <= survey.walk.frames.len() {
            return Ok(None);
        }

        let walk = Walk {
            end: frames.last().map_or(survey.walk.end, |frame| frame.range.end),
            reaches_end: false,
            frames,
        };
        let Some(info) = info_for(&walk.frames, &walk, survey.info) else {
            return Ok(None);
        };
        let Some(out) = assemble(ctx.data, survey.audio_start, &info, &walk.frames) else {
            return Ok(None);
        };

        Ok(Some(Candidate {
            strategy: self.name().to_string(),
            data: Payload::Bytes(out),
            score: score_of(walk.samples(), survey.declared_samples(), false),
            fidelity: Fidelity::Lossless,
            reconstructed: Vec::new(),
        }))
    }
}

/// Keeps walking after a failure by looking for the next frame that verifies.
fn collect_past_gaps(ctx: &RepairContext<'_>, mut frames: Vec<Frame>, from: usize, max_len: usize) -> Vec<Frame> {
    let mut offset = from;
    for _ in 0..MAX_GAPS {
        if offset >= ctx.data.len() || ctx.should_stop() {
            break;
        }
        let Some(frame) = flac_frames::find_frame(ctx, offset + 1, max_len) else {
            break;
        };
        let resumed = flac_frames::walk_frames(ctx, frame.range.start, max_len);
        if resumed.frames.is_empty() {
            break;
        }
        offset = resumed.end;
        frames.extend(resumed.frames);
    }
    frames
}

/// The marker and the description are gone; the audio is not.
///
/// Every frame header restates the rate, the channel count and the sample width, and carries
/// its own checksum, so scanning for the first frame that verifies recovers not just the audio
/// but everything needed to describe it. The rebuilt `STREAMINFO` is new bytes, but not one
/// sample is invented - which is why this is the only header reconstruction in the crate that
/// is reported as byte-faithful.
struct RebuildHeader;

impl Strategy for RebuildHeader {
    fn name(&self) -> &str {
        "flac_rebuild_header"
    }

    fn requires(&self) -> Capabilities {
        Capabilities {
            guess: true,
            ..Capabilities::NONE
        }
    }

    fn run(&self, ctx: &RepairContext<'_>) -> Result<Option<Candidate>> {
        if Survey::of(ctx).is_some_and(|survey| !survey.walk.frames.is_empty()) {
            return Ok(None);
        }

        let max_len = flac_frames::max_frame_len(None);
        let Some(first) = flac_frames::find_frame(ctx, 0, max_len) else {
            return Ok(None);
        };
        let walked = flac_frames::walk_frames(ctx, first.range.start, max_len);
        let frames = collect_past_gaps(ctx, walked.frames, walked.end, max_len);
        if frames.is_empty() {
            return Ok(None);
        }

        let walk = Walk {
            end: frames.last().map_or(walked.end, |frame| frame.range.end),
            reaches_end: false,
            frames,
        };
        let Some(info) = info_for(&walk.frames, &walk, None) else {
            return Ok(None);
        };

        let mut out = fresh_metadata(&info);
        append_frames(&mut out, ctx.data, &walk.frames);
        let samples = walk.samples();

        // The rate is only in doubt when no frame stated one, which is the rare case of an
        // encoder deferring to the header for a non-standard rate.
        let rate_was_stated = walk.frames.first().is_some_and(|frame| frame.header.sample_rate != 0);
        Ok(Some(Candidate {
            strategy: self.name().to_string(),
            data: Payload::Bytes(out),
            score: score_of(samples, None, false),
            fidelity: if rate_was_stated { Fidelity::Lossless } else { Fidelity::Reconstructed },
            reconstructed: if rate_was_stated { Vec::new() } else { std::iter::once(0..samples).collect() },
        }))
    }
}

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};
    use std::sync::OnceLock;

    use super::*;
    use crate::file_fixer::corrupt::{ByteSpan, DamageKind, DamageSpec, Fraction, apply_damage};
    use crate::file_fixer::engine::{RepairOptions, RepairResult, repair_data};
    use crate::file_fixer::mode::Mode;
    use crate::file_fixer::report::Outcome;

    /// A real FLAC, encoded by ffmpeg from the project's sample audio.
    ///
    /// There is no hand-written fixture here on purpose: a FLAC frame's boundary is found by
    /// its CRC-16, so a synthetic stream would have to be a real encoder to test anything at
    /// all. Every test below is skipped when the sample cannot be produced.
    fn sample() -> Option<&'static Vec<u8>> {
        static SAMPLE: OnceLock<Option<Vec<u8>>> = OnceLock::new();
        SAMPLE
            .get_or_init(|| {
                let source = PathBuf::from("tmp/samples/tone.flac");
                if !source.exists() {
                    return None;
                }
                std::fs::read(source).ok()
            })
            .as_ref()
    }

    fn repair(data: &[u8], mode: Mode) -> RepairResult {
        let options = RepairOptions { mode, ..RepairOptions::default() };
        repair_data(Path::new("music.flac"), data, &options)
    }

    fn bytes(result: &RepairResult) -> Vec<u8> {
        result
            .winner
            .as_ref()
            .and_then(|winner| winner.data.as_bytes())
            .expect("audio payloads stay in memory")
            .to_vec()
    }

    fn verified_samples(data: &[u8]) -> u64 {
        let options = RepairOptions::default();
        let result = repair_data(Path::new("probe.flac"), data, &options);
        result.report.content_recovered.unwrap_or(0)
    }

    #[test]
    fn a_healthy_stream_verifies_frame_by_frame_and_is_left_alone() {
        let Some(original) = sample() else {
            return;
        };
        let info = flac_frames::stream_info(original).expect("the sample has a description");
        let result = repair(original, Mode::Balanced);

        assert_eq!(result.report.outcome, Outcome::FullyRepaired, "{:?}", result.report);
        assert_eq!(result.report.winning_strategy.as_deref(), Some("flac_verbatim"));
        assert_eq!(result.report.content_recovered, Some(info.total_samples), "every sample the header promised");
        assert_eq!(bytes(&result), *original);
    }

    #[test]
    fn a_truncated_stream_keeps_exactly_the_frames_that_checksum() {
        let Some(original) = sample() else {
            return;
        };
        let damaged = apply_damage(
            original,
            &DamageSpec::new(
                DamageKind::TruncateTail {
                    keep: Fraction::new(0.6).unwrap(),
                },
                1,
            ),
        );

        let result = repair(&damaged.data, Mode::Balanced);
        assert_eq!(result.report.winning_strategy.as_deref(), Some("flac_truncate"));
        assert_eq!(result.report.outcome, Outcome::PartiallyRecovered);
        assert!(result.winner.as_ref().is_some_and(|winner| winner.is_lossless()));

        let fixed = bytes(&result);
        let info = flac_frames::stream_info(&fixed).expect("the result describes itself");
        assert_eq!(
            info.total_samples,
            result.report.content_recovered.unwrap_or(0),
            "the rewritten header must agree with the audio kept"
        );
        assert!(info.total_samples > 0);

        // The result must itself be a whole stream, and hold nothing the original did not.
        let second = repair(&fixed, Mode::Balanced);
        assert_eq!(second.report.outcome, Outcome::FullyRepaired);
        assert!(fixed.len() <= damaged.data.len(), "a repair never invents bytes for FLAC");
    }

    /// The point of the per-frame checksum: the recovered audio is provably the original
    /// audio, byte for byte, not merely something that decodes.
    #[test]
    fn recovered_frames_are_the_original_bytes() {
        let Some(original) = sample() else {
            return;
        };
        let damaged = apply_damage(
            original,
            &DamageSpec::new(
                DamageKind::TruncateTail {
                    keep: Fraction::new(0.5).unwrap(),
                },
                1,
            ),
        );

        let fixed = bytes(&repair(&damaged.data, Mode::Balanced));
        let audio_start = flac_frames::audio_start(&fixed).expect("parses");
        let original_start = flac_frames::audio_start(original).expect("parses");

        let recovered_audio = fixed.get(audio_start..).expect("audio");
        let original_audio = original.get(original_start..).expect("audio");
        assert!(!recovered_audio.is_empty());
        assert_eq!(
            recovered_audio,
            original_audio.get(..recovered_audio.len()).expect("a prefix"),
            "every recovered byte must be the original one"
        );
    }

    /// Damage in the middle costs the frames it landed in and nothing else: the frames after
    /// it still checksum, so aggressive mode steps over the hole and keeps them.
    #[test]
    fn frames_past_a_damaged_region_are_recovered_by_resynchronizing() {
        let Some(original) = sample() else {
            return;
        };
        let damaged = apply_damage(
            original,
            &DamageSpec::new(
                DamageKind::RandomRange {
                    at: Fraction::new(0.3).unwrap(),
                    len: ByteSpan::Bytes(4096),
                },
                7,
            ),
        );

        let lossless = verified_samples(&damaged.data);
        let result = repair(&damaged.data, Mode::Aggressive);
        let winner = result.winner.as_ref().expect("most of the file is fine");

        assert_eq!(winner.strategy, "flac_resync");
        assert!(
            winner.score.content_recovered > lossless,
            "resynchronizing must recover more than stopping at the damage: {} vs {lossless}",
            winner.score.content_recovered
        );
        assert!(winner.is_lossless(), "every frame kept was checksum-verified");
        assert_eq!(repair(&bytes(&result), Mode::Balanced).report.outcome, Outcome::FullyRepaired);
    }

    /// Frame headers restate everything the file header held, so a destroyed first sector
    /// costs metadata and nothing else.
    #[test]
    fn a_destroyed_header_is_rebuilt_from_the_frames_themselves() {
        let Some(original) = sample() else {
            return;
        };
        let damaged = apply_damage(original, &DamageSpec::new(DamageKind::HeaderNoise { len: ByteSpan::Bytes(2048) }, 3));

        assert!(repair(&damaged.data, Mode::Balanced).winner.is_none());

        let result = repair(&damaged.data, Mode::Aggressive);
        let winner = result.winner.as_ref().expect("the audio survived");
        assert_eq!(winner.strategy, "flac_rebuild_header");
        assert!(winner.is_lossless(), "the frames say what the header said");

        let fixed = bytes(&result);
        let info = flac_frames::stream_info(&fixed).expect("the rebuilt file describes itself");
        let source = flac_frames::stream_info(original).expect("the sample describes itself");
        assert_eq!(info.sample_rate, source.sample_rate);
        assert_eq!(info.channels, source.channels);
        assert_eq!(info.bits_per_sample, source.bits_per_sample);
        assert_eq!(repair(&fixed, Mode::Balanced).report.outcome, Outcome::FullyRepaired);
    }

    #[test]
    fn safe_mode_will_not_shorten_a_stream() {
        let Some(original) = sample() else {
            return;
        };
        let damaged = apply_damage(
            original,
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

    /// More surviving bytes must never yield fewer samples.
    #[test]
    fn recovery_grows_with_the_amount_of_surviving_data() {
        let Some(original) = sample() else {
            return;
        };
        let mut previous = 0;
        for keep in [0.2, 0.4, 0.6, 0.8] {
            let damaged = apply_damage(
                original,
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
    fn short_and_hostile_inputs_never_panic() {
        let Some(original) = sample() else {
            return;
        };
        for len in [0, 4, 8, 42, 100, 5000] {
            let mut data = original.clone();
            data.truncate(len.min(original.len()));
            let _ = repair(&data, Mode::Aggressive);
        }
    }
}
