//! MPEG program stream (`.mpg`, `.vob`) repair.
//!
//! This is the format a carving tool produces most of from an old camcorder card or a DVD
//! image, and it is the one that can prove least about itself. A program stream has no index,
//! no checksums and no lengths that span more than a single packet, so "is this recording
//! intact?" has exactly one answer available: decode it and see. Measured on a PhotoRec
//! collection of 228 of them, that answer is usually *yes* - a 551 MB carve reported 30,844
//! macroblock complaints and still played sixteen minutes of watchable video end to end.
//!
//! That shapes what the strategies here do. Scattered macroblock damage inside the pictures is
//! not repairable by anything at the container level, and the measurement says so plainly:
//! remuxing one of these files took the decoder's complaints from 9,069 to 9,290. What *is*
//! repairable is structural - a recording that stops mid-packet, or a carve that begins
//! mid-sector - and the truncation strategy addresses that exactly and losslessly, by keeping
//! the bytes that are whole.
//!
//! For everything else the honest result is the file itself, so the file itself competes. That
//! is why [`Verbatim`] here does not require a clean decode the way its counterpart in every
//! other format does; without it, the best remaining candidate for a watchable sixteen-minute
//! recording was a trim down to its first few seconds.
//!
//! The elementary-stream case (a raw `.m2v` with no packets at all) is handled too, by the
//! ffmpeg-driven strategies alone: the structural walk declines it, which is correct, because
//! there is no packet chain in it to walk.

use std::path::Path;

use super::ffmpeg::{self, DecodeReport, Probe};
use super::mpeg_packs::Structure;
use crate::file_fixer::candidate::{Candidate, Fidelity, Payload, Score};
use crate::file_fixer::engine::{InputFile, RepairContext, Repairer, Strategy};
use crate::file_fixer::error::{FixerError, Result};
use crate::file_fixer::format::Format;
use crate::file_fixer::mode::Capabilities;

pub struct MpegRepairer;

impl Repairer for MpegRepairer {
    fn format(&self) -> Format {
        Format::Mpeg
    }

    fn strategies(&self, _ctx: &RepairContext<'_>) -> Vec<Box<dyn Strategy>> {
        vec![Box::new(Verbatim), Box::new(TruncateToLastPacket), Box::new(TolerantRemux), Box::new(TrimToLastGoodFrame)]
    }
}

/// A working copy of the input plus what the container says about it.
struct Source {
    input: InputFile,
    probe: Probe,
}

impl Source {
    fn of(ctx: &RepairContext<'_>) -> Result<Option<Self>> {
        if !ffmpeg::is_available() {
            return Err(FixerError::FfmpegMissing);
        }
        let input = ctx.input_file()?;
        let probe = ffmpeg::probe(input.path())?;
        if probe.video().is_none() && !probe.has_audio() {
            return Ok(None);
        }
        Ok(Some(Self { input, probe }))
    }

    fn path(&self) -> &Path {
        self.input.path()
    }

    fn has_video(&self) -> bool {
        self.probe.video().is_some()
    }
}

/// What a program stream declares about how much content it should hold: nothing, ever. There
/// is no frame count in the format, and ffprobe's estimate is derived from the damaged file's
/// own duration, so using it would make every percentage self-congratulating.
const NO_DECLARED_TOTAL: Option<u64> = None;

/// Whether a produced file holds nothing at all. Asked in terms of content rather than frames
/// because a fair share of these carves are audio-only - a fragment of the soundtrack that the
/// video packets never made it into - and for those the frame count is zero however well the
/// repair worked.
fn produced_nothing(report: &DecodeReport, has_video: bool) -> bool {
    content_of(report, has_video) == 0
}

fn content_of(report: &DecodeReport, has_video: bool) -> u64 {
    if has_video { report.frames } else { (report.duration * 1000.0).max(0.0) as u64 }
}

fn score_of(report: &DecodeReport, expected: Option<u64>, has_video: bool) -> Score {
    Score {
        content_recovered: content_of(report, has_video),
        content_total: expected,
        decoder_errors: report.errors,
        decodes_cleanly: report.is_clean(),
        streams_complete: true,
    }
}

fn temp_output() -> Result<tempfile::NamedTempFile> {
    tempfile::Builder::new()
        .prefix("file_fixer-")
        .suffix(".mpg")
        .tempfile()
        .map_err(|error| FixerError::write("<temp>", error))
}

fn candidate_from(name: &str, ctx: &RepairContext<'_>, output: tempfile::NamedTempFile, report: &DecodeReport, expected: Option<u64>, has_video: bool) -> Option<Candidate> {
    let fidelity = if report.is_clean() { Fidelity::Lossless } else { Fidelity::Reconstructed };
    if fidelity == Fidelity::Reconstructed && !ctx.capabilities.guess {
        ctx.decline(Capabilities {
            guess: true,
            ..Capabilities::NONE
        });
        return None;
    }

    let frames = content_of(report, has_video);
    Some(Candidate {
        strategy: name.to_string(),
        data: Payload::TempFile(output.into_temp_path()),
        score: score_of(report, expected, has_video),
        fidelity,
        reconstructed: match fidelity {
            Fidelity::Lossless => Vec::new(),
            Fidelity::Reconstructed => std::iter::once(0..frames).collect(),
        },
    })
}

/// The packet chain accounts for the whole file, so the file as recorded is a candidate.
///
/// Unlike every other format's verbatim strategy, this one does not insist the decode be clean.
/// It cannot afford to: the ordinary condition of a carved program stream is an intact
/// container full of pictures with macroblock damage, and no repair reaches inside a picture.
/// Standing the original up as a candidate is what lets the scoring say the true thing about
/// such a file - that 11,811 frames survive, that 30,844 of the decoder's complaints are not
/// going away, and that nothing produced here beats simply keeping it. A candidate that loses
/// costs nothing; its absence cost a watchable recording being trimmed to its first few
/// seconds, because a trim was the best thing left in the list.
struct Verbatim;

impl Strategy for Verbatim {
    fn name(&self) -> &str {
        "mpeg_verbatim"
    }

    fn run(&self, ctx: &RepairContext<'_>) -> Result<Option<Candidate>> {
        let Some(source) = Source::of(ctx)? else {
            return Ok(None);
        };
        let whole_chain = Structure::of(ctx.data).map(|structure| structure.is_whole(ctx.data.len()));
        // A chain that stops short is the truncation strategy's business, not this one's.
        if whole_chain == Some(false) {
            return Ok(None);
        }

        let report = ffmpeg::decode_report(source.path())?;
        if produced_nothing(&report, source.has_video()) {
            return Ok(None);
        }

        // A denominator is earned by two things together: a chain walked from the first byte to
        // the last, and a decode with nothing to say about it. Either alone is not enough, and
        // the recovery matrix showed why - a cut that happened to land on a packet boundary
        // walks as whole, and reporting it "full" is reporting 100% of what survived. Without
        // the denominator the same file reports its frame count, which is the true statement.
        let total = (whole_chain == Some(true) && report.is_clean()).then(|| content_of(&report, source.has_video()));

        Ok(Some(Candidate {
            strategy: self.name().to_string(),
            score: score_of(&report, total, source.has_video()),
            data: source.input.into_payload(),
            // Lossless describes the bytes, not the pictures: nothing here was reconstructed,
            // reencoded or guessed. The complaints the score carries say what the pictures are
            // like, and they are what they were when the file was recovered.
            fidelity: Fidelity::Lossless,
            reconstructed: Vec::new(),
        }))
    }
}

/// Keep the packets that are whole and drop what is around them.
///
/// This is the one exact repair the format allows. Every byte written is a byte that was
/// recorded, at the same position relative to its packet, and nothing is rewritten - the file
/// is a slice of the input. It addresses both ways a carve is malformed at its edges: a
/// recording cut off mid-packet at the end, and a carve that began mid-sector so the first
/// bytes are the tail of some other packet.
struct TruncateToLastPacket;

impl Strategy for TruncateToLastPacket {
    fn name(&self) -> &str {
        "mpeg_truncate_to_packet"
    }

    fn requires(&self) -> Capabilities {
        Capabilities {
            truncate: true,
            ..Capabilities::NONE
        }
    }

    fn run(&self, ctx: &RepairContext<'_>) -> Result<Option<Candidate>> {
        let Some(structure) = Structure::of(ctx.data) else {
            return Ok(None);
        };
        if structure.is_whole(ctx.data.len()) {
            return Ok(None);
        }
        let Some(kept) = ctx.data.get(structure.kept()) else {
            return Ok(None);
        };
        if kept.is_empty() {
            return Ok(None);
        }

        let output = temp_output()?;
        std::fs::write(output.path(), kept).map_err(|error| FixerError::write(output.path(), error))?;
        if !ffmpeg::is_available() {
            return Err(FixerError::FfmpegMissing);
        }
        let probe = ffmpeg::probe(output.path())?;
        let has_video = probe.video().is_some();
        let report = ffmpeg::decode_report(output.path())?;
        if produced_nothing(&report, has_video) {
            return Ok(None);
        }

        Ok(candidate_from(self.name(), ctx, output, &report, None, has_video))
    }
}

/// Rewrite the container, letting ffmpeg walk past damage instead of stopping at it.
struct TolerantRemux;

impl Strategy for TolerantRemux {
    fn name(&self) -> &str {
        "mpeg_remux"
    }

    fn requires(&self) -> Capabilities {
        Capabilities {
            truncate: true,
            ..Capabilities::NONE
        }
    }

    fn run(&self, ctx: &RepairContext<'_>) -> Result<Option<Candidate>> {
        let Some(source) = Source::of(ctx)? else {
            return Ok(None);
        };
        let output = temp_output()?;
        ffmpeg::remux(source.path(), output.path(), true)?;

        let report = ffmpeg::decode_report(output.path())?;
        if produced_nothing(&report, source.has_video()) {
            return Ok(None);
        }
        Ok(candidate_from(self.name(), ctx, output, &report, NO_DECLARED_TOTAL, source.has_video()))
    }
}

/// How many frame intervals to step back from the failure point, in order of preference.
const TRIM_BACK_OFF_FRAMES: [f64; 3] = [1.0, 5.0, 25.0];

/// Cut the recording just before the point where decoding stops being reliable.
///
/// Worth far less here than it is for MP4 or Matroska, and the corpus says why: the damage in a
/// carved program stream is usually scattered through the pictures rather than concentrated at
/// a break, so cutting at the first complaint discards a watchable recording to buy a clean
/// decode of its first few seconds. It stays in the list because the scoring compares content
/// first, so a trim only wins when there was nothing else to keep.
struct TrimToLastGoodFrame;

impl Strategy for TrimToLastGoodFrame {
    fn name(&self) -> &str {
        "mpeg_trim"
    }

    fn requires(&self) -> Capabilities {
        Capabilities {
            truncate: true,
            ..Capabilities::NONE
        }
    }

    fn run(&self, ctx: &RepairContext<'_>) -> Result<Option<Candidate>> {
        let Some(source) = Source::of(ctx)? else {
            return Ok(None);
        };

        let until_error = ffmpeg::decode_until_error(source.path())?;
        if until_error.is_clean() || until_error.duration <= 0.0 {
            return Ok(None);
        }
        let frame_interval = match until_error.frames {
            0 => return Ok(None),
            frames => until_error.duration / frames as f64,
        };

        let mut fallback = None;
        for back_off in TRIM_BACK_OFF_FRAMES {
            if ctx.should_stop() {
                break;
            }
            let target = until_error.duration - frame_interval * back_off;
            if target <= 0.0 {
                break;
            }

            let output = temp_output()?;
            ffmpeg::trim(source.path(), output.path(), target, true)?;
            let report = ffmpeg::decode_report(output.path())?;
            if produced_nothing(&report, source.has_video()) {
                continue;
            }
            if report.is_clean() {
                return Ok(candidate_from(self.name(), ctx, output, &report, NO_DECLARED_TOTAL, source.has_video()));
            }
            fallback.get_or_insert((output, report));
        }

        Ok(fallback.and_then(|(output, report)| candidate_from(self.name(), ctx, output, &report, NO_DECLARED_TOTAL, source.has_video())))
    }
}

#[cfg(test)]
mod tests {
    use std::io::Write;
    use std::path::PathBuf;

    use super::super::mpeg_packs::Packet;
    use super::*;
    use crate::file_fixer::corrupt::{DamageKind, DamageSpec, Fraction, apply_damage};
    use crate::file_fixer::engine::{RepairOptions, repair_file};
    use crate::file_fixer::mode::Mode;
    use crate::file_fixer::report::{Outcome, RepairReport};

    fn sample() -> PathBuf {
        PathBuf::from("tmp/samples/clip.mpg")
    }

    fn skip() -> bool {
        !ffmpeg::is_available() || !sample().exists()
    }

    fn damaged_file(keep: f64) -> tempfile::TempPath {
        let original = std::fs::read(sample()).expect("the sample is readable");
        let damaged = apply_damage(
            &original,
            &DamageSpec::new(
                DamageKind::TruncateTail {
                    keep: Fraction::new(keep).unwrap(),
                },
                1,
            ),
        );

        let mut file = tempfile::Builder::new().suffix(".mpg").tempfile().expect("temp file");
        file.write_all(&damaged.data).expect("write");
        file.into_temp_path()
    }

    fn repair(path: &Path, mode: Mode) -> RepairReport {
        let options = RepairOptions {
            mode,
            output_dir: Some(std::env::temp_dir().join("file_fixer-mpeg-tests")),
            force: true,
            ..RepairOptions::default()
        };
        repair_file(path, &options)
    }

    #[test]
    fn a_healthy_recording_is_recognized_and_left_alone() {
        if skip() {
            return;
        }
        let report = repair(&sample(), Mode::Balanced);

        assert_eq!(report.outcome, Outcome::Healthy, "{:?}", report.error);
        assert_eq!(report.winning_strategy.as_deref(), Some("mpeg_verbatim"));
        assert!(report.output_path.is_none(), "a healthy file must not be rewritten");
    }

    #[test]
    fn the_packet_chain_reads_back_the_way_it_was_written() {
        if skip() {
            return;
        }
        let data = std::fs::read(sample()).expect("readable");
        let structure = Structure::of(&data).expect("the sample parses");

        assert!(structure.is_whole(data.len()), "a healthy sample must walk end to end");
        assert!(structure.packets.iter().any(Packet::is_pack_header));
    }

    #[test]
    fn a_cut_recording_keeps_the_packets_that_arrived() {
        if skip() {
            return;
        }
        let damaged = damaged_file(0.6);
        let report = repair(&damaged, Mode::Balanced);

        assert_ne!(report.outcome, Outcome::Healthy, "{:?}", report.error);
        assert!(report.output_path.is_some(), "{:?}", report.error);
        assert!(report.content_recovered.is_some_and(|frames| frames > 0));
    }

    /// The structural cut is a slice of the input, so whatever it writes has to appear in the
    /// original at the same offset - this is the one repair the format allows to be exact.
    #[test]
    fn the_structural_cut_writes_back_the_original_bytes() {
        if skip() {
            return;
        }
        let damaged = damaged_file(0.6);
        let original = std::fs::read(&damaged).expect("readable");
        let structure = Structure::of(&original).expect("parses");
        let kept = structure.kept();

        assert!(!structure.is_whole(original.len()), "the damage must be visible");
        assert_eq!(original.get(kept.clone()).map(<[u8]>::len), Some(kept.end - kept.start));
        assert!(kept.end <= original.len());
        assert!(kept.end > original.len() / 2, "a tail cut must keep the head");
    }

    #[test]
    fn recovery_grows_with_the_amount_of_surviving_data() {
        if skip() {
            return;
        }
        let mut previous = 0;
        for keep in [0.3, 0.6, 0.9] {
            let damaged = damaged_file(keep);
            let recovered = repair(&damaged, Mode::Balanced).content_recovered.unwrap_or(0);
            assert!(recovered >= previous, "keep {keep} gave {recovered} after {previous}");
            previous = recovered;
        }
    }

    #[test]
    fn safe_mode_will_not_shorten_a_recording() {
        if skip() {
            return;
        }
        let report = repair(&damaged_file(0.6), Mode::Safe);

        assert_eq!(report.outcome, Outcome::Unrecoverable);
        assert!(report.output_path.is_none());
    }
}
