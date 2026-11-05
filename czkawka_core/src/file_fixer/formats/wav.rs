//! WAV repair.
//!
//! A WAV is a header and a wall of samples, and the two fail in different ways. The samples
//! are raw: any byte that survived is exactly the sample that was recorded, so there is never
//! anything to reconstruct in them - which also means the format can prove nothing about them,
//! and a zeroed run in the middle is indistinguishable from a moment of silence (see the
//! README). The header, by contrast, is where practically every real WAV complaint lives: a
//! recorder that lost power never went back to fill in the two length fields it left at zero,
//! and players then refuse a file whose audio is entirely intact.
//!
//! So the ladder is short and mostly capability-free. Correcting a length field discards
//! nothing and is allowed even in safe mode; the only thing that can be discarded is a final
//! sample frame the recorder did not finish writing, and that alone needs `truncate`. When the
//! first sector is gone with the whole header, the `fmt ` chunk is often still there further
//! in - in which case the rebuild is exact - and when it is not, the sample rate and channel
//! count have to be guessed, which changes how the audio *sounds* and is reported as
//! reconstructed.

use super::riff::{self, Chunk, Riff};
use crate::file_fixer::candidate::{Candidate, Fidelity, Payload, Score};
use crate::file_fixer::engine::{RepairContext, Repairer, Strategy};
use crate::file_fixer::error::Result;
use crate::file_fixer::format::Format;
use crate::file_fixer::io::{put_u32_le, read_u16_le, read_u32_le};
use crate::file_fixer::mode::Capabilities;

pub struct WavRepairer;

impl Repairer for WavRepairer {
    fn format(&self) -> Format {
        Format::Wav
    }

    fn strategies(&self, _ctx: &RepairContext<'_>) -> Vec<Box<dyn Strategy>> {
        vec![Box::new(Verbatim), Box::new(FixSizes), Box::new(TruncateToWholeFrames), Box::new(RebuildHeader)]
    }
}

/// What the `fmt ` chunk says about the samples.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WaveFormat {
    pub tag: u16,
    pub channels: u16,
    pub sample_rate: u32,
    /// Bytes per sample frame, which is also the unit truncation has to respect.
    pub block_align: u16,
    pub bits_per_sample: u16,
}

/// Sample rates outside this range are not a recording, they are a damaged field.
const SAMPLE_RATE_RANGE: std::ops::RangeInclusive<u32> = 1000..=768_000;
const MAX_CHANNELS: u16 = 64;

impl WaveFormat {
    fn parse(data: &[u8], payload: &std::ops::Range<usize>) -> Option<Self> {
        if payload.len() < 16 {
            return None;
        }
        let tag = read_u16_le(data, payload.start)?;
        let channels = read_u16_le(data, payload.start + 2)?;
        let sample_rate = read_u32_le(data, payload.start + 4)?;
        let stored_align = read_u16_le(data, payload.start + 12)?;
        let bits_per_sample = read_u16_le(data, payload.start + 14)?;

        if !(1..=MAX_CHANNELS).contains(&channels) || !SAMPLE_RATE_RANGE.contains(&sample_rate) {
            return None;
        }
        // A zeroed block alignment is recoverable from the channel count and sample width,
        // which is worth doing: without it there is no unit to count frames in.
        let block_align = match stored_align {
            0 => channels.checked_mul(bits_per_sample.div_ceil(8))?,
            align => align,
        };
        if block_align == 0 {
            return None;
        }

        Some(Self {
            tag,
            channels,
            sample_rate,
            block_align,
            bits_per_sample,
        })
    }

    fn frames(&self, bytes: usize) -> u64 {
        bytes as u64 / u64::from(self.block_align)
    }

    /// The 16-byte body of a `fmt ` chunk describing this format.
    fn to_chunk_payload(self) -> Vec<u8> {
        let byte_rate = self.sample_rate * u32::from(self.block_align);
        let mut out = Vec::with_capacity(16);
        out.extend_from_slice(&self.tag.to_le_bytes());
        out.extend_from_slice(&self.channels.to_le_bytes());
        out.extend_from_slice(&self.sample_rate.to_le_bytes());
        out.extend_from_slice(&byte_rate.to_le_bytes());
        out.extend_from_slice(&self.block_align.to_le_bytes());
        out.extend_from_slice(&self.bits_per_sample.to_le_bytes());
        out
    }
}

/// Sixteen-bit stereo at CD rate: what a headerless recording is assumed to be when nothing
/// in the file or the references says otherwise.
const ASSUMED_FORMAT: WaveFormat = WaveFormat {
    tag: 1,
    channels: 2,
    sample_rate: 44_100,
    block_align: 4,
    bits_per_sample: 16,
};

/// A parsed WAV: the container, the format, and where the samples really are.
struct Survey {
    riff: Riff,
    format: WaveFormat,
    data_chunk: Chunk,
    /// Sample bytes actually present, which is the declared length when that is believable and
    /// everything to the end of the file when it is not.
    available: usize,
}

impl Survey {
    fn of(data: &[u8]) -> Option<Self> {
        let riff = riff::parse(data)?;
        if riff.form != *b"WAVE" {
            return None;
        }
        let format_chunk = riff.chunk(*b"fmt ")?;
        let format = WaveFormat::parse(data, &format_chunk.payload)?;
        let data_chunk = riff.chunk(*b"data")?.clone();

        Some(Self {
            available: available_bytes(data, &data_chunk),
            riff,
            format,
            data_chunk,
        })
    }

    fn samples_start(&self) -> usize {
        self.data_chunk.payload.start
    }

    /// Sample bytes that form whole frames. A frame the recorder did not finish is not a
    /// sample, and keeping it would leave a player reading a fraction of one.
    fn kept(&self) -> usize {
        let align = usize::from(self.format.block_align);
        self.available / align * align
    }

    /// Frames the recording was meant to hold.
    ///
    /// A length field that promises *more* than the file holds is evidence of loss and becomes
    /// the denominator. A field left at zero is not evidence of anything - an interrupted
    /// writer never wrote it - so the honest total is what is there, and the result is a full
    /// repair rather than a partial one.
    fn total_frames(&self) -> u64 {
        let declared = self.data_chunk.declared_len as usize;
        if declared > self.available {
            return self.format.frames(declared);
        }
        self.format.frames(self.kept())
    }

    /// True when the header already describes exactly what the file holds.
    fn is_consistent(&self, len: usize) -> bool {
        self.data_chunk.complete && self.data_chunk.declared_len > 0 && !self.riff.promises_more_than(len) && self.kept() == self.available
    }
}

/// How many sample bytes follow the `data` header.
///
/// The declared length wins when it is believable. A zero - what an interrupted recorder
/// leaves behind - or a length larger than the file means the only usable answer is "the rest
/// of the file", which is also why a WAV with metadata chunks *after* its samples cannot be
/// told from one whose samples run to the end once that field is gone.
fn available_bytes(data: &[u8], chunk: &Chunk) -> usize {
    let to_end = data.len().saturating_sub(chunk.payload.start);
    if chunk.declared_len == 0 || !chunk.complete {
        return to_end;
    }
    chunk.payload.len().min(to_end)
}

/// Rewrites the file so both length fields describe the samples that are there.
///
/// Everything before the samples is copied byte for byte, so any metadata chunk that precedes
/// them survives untouched; only the two numbers are patched.
fn with_corrected_lengths(data: &[u8], survey: &Survey, kept: usize) -> Option<Vec<u8>> {
    let samples_start = survey.samples_start();
    let ends_at_data = survey.data_chunk.end() >= data.len() || survey.data_chunk.declared_len == 0;

    let mut out = data.get(..samples_start + kept)?.to_vec();
    if !ends_at_data {
        // Chunks after the samples (metadata, cue points) are content too.
        if let Some(tail) = data.get(survey.data_chunk.end()..) {
            out.extend_from_slice(tail);
        }
    }

    let riff_len = u32::try_from(out.len().saturating_sub(8)).unwrap_or(u32::MAX);
    put_u32_le(&mut out, survey.data_chunk.start + 4, u32::try_from(kept).unwrap_or(u32::MAX));
    put_u32_le(&mut out, 4, riff_len);
    Some(out)
}

fn score_of(frames: u64, total: Option<u64>) -> Score {
    Score {
        content_recovered: frames,
        content_total: total,
        decoder_errors: 0,
        decodes_cleanly: true,
        streams_complete: total.is_some_and(|total| total <= frames),
    }
}

/// The header describes the file and every sample it promises is present.
struct Verbatim;

impl Strategy for Verbatim {
    fn name(&self) -> &str {
        "wav_verbatim"
    }

    fn run(&self, ctx: &RepairContext<'_>) -> Result<Option<Candidate>> {
        let Some(survey) = Survey::of(ctx.data) else {
            return Ok(None);
        };
        if !survey.is_consistent(ctx.data.len()) {
            return Ok(None);
        }
        let frames = survey.format.frames(survey.kept());

        Ok(Some(Candidate {
            strategy: self.name().to_string(),
            data: Payload::Bytes(ctx.data.to_vec()),
            score: score_of(frames, Some(frames)),
            fidelity: Fidelity::Lossless,
            reconstructed: Vec::new(),
        }))
    }
}

/// The samples are all there and the length fields are wrong. Correcting a number discards
/// nothing, so this is a safe-mode repair - and it is the one that fixes the overwhelmingly
/// common real complaint, a recording whose writer never came back to fill in its sizes.
struct FixSizes;

impl Strategy for FixSizes {
    fn name(&self) -> &str {
        "wav_fix_sizes"
    }

    fn run(&self, ctx: &RepairContext<'_>) -> Result<Option<Candidate>> {
        let Some(survey) = Survey::of(ctx.data) else {
            return Ok(None);
        };
        let kept = survey.kept();
        if survey.is_consistent(ctx.data.len()) || kept == 0 || kept != survey.available {
            return Ok(None);
        }
        let Some(out) = with_corrected_lengths(ctx.data, &survey, kept) else {
            return Ok(None);
        };

        Ok(Some(Candidate {
            strategy: self.name().to_string(),
            data: Payload::Bytes(out),
            score: score_of(survey.format.frames(kept), Some(survey.total_frames())),
            fidelity: Fidelity::Lossless,
            reconstructed: Vec::new(),
        }))
    }
}

/// The recording stops in the middle of a sample frame. Everything up to the last whole frame
/// is kept; the fraction of a frame that is left cannot be played and is dropped, which is the
/// only thing here that needs permission to discard anything.
struct TruncateToWholeFrames;

impl Strategy for TruncateToWholeFrames {
    fn name(&self) -> &str {
        "wav_truncate"
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
        let kept = survey.kept();
        if kept == 0 || kept == survey.available {
            return Ok(None);
        }
        let Some(out) = with_corrected_lengths(ctx.data, &survey, kept) else {
            return Ok(None);
        };

        Ok(Some(Candidate {
            strategy: self.name().to_string(),
            data: Payload::Bytes(out),
            score: score_of(survey.format.frames(kept), Some(survey.total_frames())),
            fidelity: Fidelity::Lossless,
            reconstructed: Vec::new(),
        }))
    }
}

/// The header is gone. Look for the chunks that describe the recording further into the file.
///
/// A `fmt ` chunk that survived settles everything: it states the rate, the channel count and
/// the sample width, so the rebuilt file is exact and the samples are untouched. Without one,
/// the samples are still real but nothing says how fast to play them - a guess there does not
/// corrupt a byte, it changes the pitch and the duration, which is why the result is reported
/// as reconstructed rather than recovered.
struct RebuildHeader;

impl Strategy for RebuildHeader {
    fn name(&self) -> &str {
        "wav_rebuild_header"
    }

    fn requires(&self) -> Capabilities {
        Capabilities {
            guess: true,
            ..Capabilities::NONE
        }
    }

    fn run(&self, ctx: &RepairContext<'_>) -> Result<Option<Candidate>> {
        if Survey::of(ctx.data).is_some() {
            return Ok(None);
        }

        let found_format = riff::find_chunk(ctx.data, *b"fmt ", 0).and_then(|chunk| WaveFormat::parse(ctx.data, &chunk.payload).map(|format| (chunk, format)));
        let format = found_format.as_ref().map_or_else(|| format_from_reference(ctx).unwrap_or(ASSUMED_FORMAT), |(_, f)| *f);

        let samples_start = match riff::find_chunk(ctx.data, *b"data", 0) {
            Some(chunk) => chunk.payload.start,
            // Nothing named the samples, so they begin where the format chunk ends - or, with
            // no header at all, past the region the damage took.
            None => found_format.as_ref().map_or(riff::HEADER_LEN, |(chunk, _)| chunk.end()),
        };

        let Some(samples) = ctx.data.get(samples_start..) else {
            return Ok(None);
        };
        let align = usize::from(format.block_align);
        let kept = samples.len() / align * align;
        // Below a fraction of a second, whatever was found is far more likely to be the tail
        // of a chunk header than a recording. A tiny WAV whose header is destroyed is given up
        // on rather than turned into a burst of noise.
        if kept < MIN_REBUILT_SAMPLE_BYTES {
            return Ok(None);
        }

        let mut body = Vec::new();
        riff::append_chunk(&mut body, *b"fmt ", &format.to_chunk_payload());
        riff::append_chunk(&mut body, *b"data", samples.get(..kept).unwrap_or_default());
        let mut out = riff::header(*b"WAVE", body.len());
        out.extend_from_slice(&body);

        let frames = format.frames(kept);
        let exact = found_format.is_some();
        Ok(Some(Candidate {
            strategy: self.name().to_string(),
            data: Payload::Bytes(out),
            score: score_of(frames, None),
            fidelity: if exact { Fidelity::Lossless } else { Fidelity::Reconstructed },
            reconstructed: if exact { Vec::new() } else { std::iter::once(0..frames).collect() },
        }))
    }
}

/// Roughly 23 milliseconds of CD audio: the least that is worth calling a recovered recording.
const MIN_REBUILT_SAMPLE_BYTES: usize = 4096;

/// A recording of the same provenance states the rate and channel count this one lost.
fn format_from_reference(ctx: &RepairContext<'_>) -> Option<WaveFormat> {
    for path in ctx.references {
        let Ok(reference) = std::fs::read(path) else {
            continue;
        };
        if let Some(survey) = Survey::of(&reference) {
            return Some(survey.format);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;
    use crate::file_fixer::corrupt::{ByteSpan, DamageKind, DamageSpec, Fraction, apply_damage};
    use crate::file_fixer::engine::{RepairOptions, RepairResult, repair_data};
    use crate::file_fixer::mode::Mode;
    use crate::file_fixer::report::Outcome;

    /// A recording of `frames` sample frames: 16-bit stereo, a tone so that damaged samples
    /// are distinguishable from real ones by eye in a failure.
    fn sample_wav(frames: usize) -> Vec<u8> {
        let mut samples = Vec::with_capacity(frames * 4);
        for index in 0..frames {
            let value = ((index % 400) as i32 * 80 - 16000) as i16;
            samples.extend_from_slice(&value.to_le_bytes());
            samples.extend_from_slice(&(-value).to_le_bytes());
        }

        let mut body = Vec::new();
        riff::append_chunk(&mut body, *b"fmt ", &ASSUMED_FORMAT.to_chunk_payload());
        riff::append_chunk(&mut body, *b"data", &samples);
        let mut out = riff::header(*b"WAVE", body.len());
        out.extend_from_slice(&body);
        out
    }

    fn repair(data: &[u8], mode: Mode) -> RepairResult {
        let options = RepairOptions { mode, ..RepairOptions::default() };
        repair_data(Path::new("recording.wav"), data, &options)
    }

    fn bytes(result: &RepairResult) -> Vec<u8> {
        result
            .winner
            .as_ref()
            .and_then(|winner| winner.data.as_bytes())
            .expect("audio payloads stay in memory")
            .to_vec()
    }

    fn samples_of(data: &[u8]) -> Vec<u8> {
        let survey = Survey::of(data).expect("the file parses");
        data.get(survey.samples_start()..survey.samples_start() + survey.kept())
            .expect("the samples are present")
            .to_vec()
    }

    #[test]
    fn a_healthy_recording_is_left_alone() {
        let original = sample_wav(1000);
        let result = repair(&original, Mode::Balanced);

        assert_eq!(result.report.outcome, Outcome::FullyRepaired);
        assert_eq!(result.report.winning_strategy.as_deref(), Some("wav_verbatim"));
        assert_eq!(result.report.content_recovered, Some(1000));
        assert_eq!(bytes(&result), original);
    }

    /// The most common real WAV complaint by a wide margin: the recorder stopped before
    /// writing its length fields, and every player refuses a file whose audio is perfect.
    #[test]
    fn length_fields_left_at_zero_are_filled_in_even_in_safe_mode() {
        let mut data = sample_wav(1000);
        let survey = Survey::of(&data).expect("parses");
        let data_size_offset = survey.data_chunk.start + 4;
        put_u32_le(&mut data, 4, 0);
        put_u32_le(&mut data, data_size_offset, 0);

        let result = repair(&data, Mode::Safe);
        assert_eq!(result.report.winning_strategy.as_deref(), Some("wav_fix_sizes"));
        assert_eq!(result.report.outcome, Outcome::FullyRepaired);

        let fixed = bytes(&result);
        assert_eq!(read_u32_le(&fixed, 4), Some(fixed.len() as u32 - 8));
        assert_eq!(read_u32_le(&fixed, data_size_offset), Some(4000));
        assert_eq!(samples_of(&fixed), samples_of(&sample_wav(1000)), "not one sample moved");
    }

    #[test]
    fn a_cut_recording_keeps_every_whole_frame_that_survived() {
        let original = sample_wav(4000);
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
        assert_eq!(result.report.outcome, Outcome::PartiallyRecovered);

        let fixed = bytes(&result);
        let survey = Survey::of(&fixed).expect("the repaired file parses");
        assert!(survey.is_consistent(fixed.len()), "the result must describe itself");

        let recovered = samples_of(&fixed);
        assert!(!recovered.is_empty());
        assert_eq!(
            recovered,
            samples_of(&original).get(..recovered.len()).expect("a prefix of the original").to_vec(),
            "every recovered sample must be the original one"
        );
        assert_eq!(result.report.content_total, Some(4000), "the original length is known");
    }

    /// Sample frames are the unit: half a frame is not audio, and keeping it would leave a
    /// player reading a sample that is half missing.
    #[test]
    fn a_half_written_sample_frame_is_dropped_and_that_needs_permission() {
        let mut data = sample_wav(1000);
        data.truncate(data.len() - 2);
        let survey = Survey::of(&data).expect("parses");
        assert_eq!(survey.available % usize::from(survey.format.block_align), 2);

        let result = repair(&data, Mode::Balanced);
        assert_eq!(result.report.winning_strategy.as_deref(), Some("wav_truncate"));
        assert_eq!(result.report.content_recovered, Some(999));

        let safe = repair(&data, Mode::Safe);
        assert!(safe.winner.is_none());
        assert!(safe.report.declined_capabilities.contains(&"truncate".to_string()));
    }

    /// The samples describe themselves as nothing: only the header says how to play them. A
    /// surviving `fmt ` chunk makes the rebuild exact.
    #[test]
    fn a_destroyed_header_is_rebuilt_exactly_when_the_format_chunk_survived() {
        let original = sample_wav(2000);
        let damaged = apply_damage(&original, &DamageSpec::new(DamageKind::HeaderNoise { len: ByteSpan::Bytes(8) }, 3));

        let result = repair(&damaged.data, Mode::Aggressive);
        let winner = result.winner.as_ref().expect("the samples are all there");
        assert_eq!(winner.strategy, "wav_rebuild_header");
        assert!(winner.is_lossless(), "a surviving fmt chunk leaves nothing to guess");

        let fixed = bytes(&result);
        let survey = Survey::of(&fixed).expect("the rebuilt file parses");
        assert_eq!(survey.format, ASSUMED_FORMAT);
        assert_eq!(samples_of(&fixed), samples_of(&original));
    }

    /// Without a format chunk the rate and channel count are a guess: no sample is altered,
    /// but the recording plays at the wrong speed, and the report has to say so.
    #[test]
    fn a_recording_with_no_format_chunk_left_is_rebuilt_but_reported_as_guessed() {
        let original = sample_wav(2000);
        let survey = Survey::of(&original).expect("parses");
        let mut damaged = original;
        if let Some(head) = damaged.get_mut(..survey.samples_start()) {
            head.fill(0x00);
        }

        let result = repair(&damaged, Mode::Aggressive);
        let winner = result.winner.as_ref().expect("the samples are still there");
        assert_eq!(winner.strategy, "wav_rebuild_header");
        assert!(!winner.is_lossless(), "the playback rate is invented");
        assert!(!winner.reconstructed.is_empty());
    }

    /// Nothing in the format can tell a zeroed run from silence, so the honest answer is that
    /// the file is intact.
    #[test]
    fn a_zeroed_run_of_samples_is_silence_as_far_as_the_format_can_tell() {
        let original = sample_wav(4000);
        let damaged = apply_damage(
            &original,
            &DamageSpec::new(
                DamageKind::ZeroRange {
                    at: Fraction::HALF,
                    len: ByteSpan::Bytes(2048),
                },
                5,
            ),
        );

        let result = repair(&damaged.data, Mode::Balanced);
        assert_eq!(result.report.winning_strategy.as_deref(), Some("wav_verbatim"));
        assert_eq!(bytes(&result), damaged.data);
    }

    /// A file cut before its samples even begin has no audio to recover, and must not be
    /// dressed up as a recording of nothing.
    #[test]
    fn a_file_cut_before_its_samples_recovers_nothing() {
        let original = sample_wav(1000);
        let mut data = original;
        data.truncate(20);

        let result = repair(&data, Mode::Aggressive);
        assert!(result.winner.is_none() || result.report.content_recovered == Some(0));
    }

    #[test]
    fn an_implausible_format_chunk_is_not_believed() {
        let mut data = sample_wav(100);
        let survey = Survey::of(&data).expect("parses");
        let rate_offset = survey.riff.chunk(*b"fmt ").expect("fmt").payload.start + 4;
        put_u32_le(&mut data, rate_offset, 5_000_000);

        assert!(Survey::of(&data).is_none(), "five megahertz is not a sample rate");
    }

    #[test]
    fn short_and_hostile_inputs_never_panic() {
        let original = sample_wav(50);
        for len in 0..original.len() {
            let mut data = original.clone();
            data.truncate(len);
            let _ = repair(&data, Mode::Aggressive);
        }
    }
}
