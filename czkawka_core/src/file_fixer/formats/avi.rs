//! AVI repair.
//!
//! Recognize AVI, pass a healthy file through untouched, and recover ordinary truncation or
//! mid-file corruption via a tolerant remux and a trim to the last cleanly decoding frame - the
//! same three strategies MP4 support started with, and judged the same way: quality is decided
//! by actually decoding the candidate with ffmpeg, since there is no pure-Rust HEVC/H.264
//! decoder worth trusting for that.
//!
//! A file whose `RIFF` header is destroyed outright has none of those available to it, because
//! ffmpeg cannot open it at all; [`super::avi_rebuild`] writes a new container around the chunk
//! chain that survived instead.

use std::path::Path;

use super::avi_rebuild;
use super::ffmpeg::{self, DecodeReport, Probe};
use crate::file_fixer::candidate::{Candidate, Fidelity, Payload, Score};
use crate::file_fixer::engine::{InputFile, RepairContext, Repairer, Strategy, Support};
use crate::file_fixer::error::{FixerError, Result};
use crate::file_fixer::format::Format;
use crate::file_fixer::io::read_u32_le;
use crate::file_fixer::mode::Capabilities;

pub struct AviRepairer;

impl Repairer for AviRepairer {
    fn format(&self) -> Format {
        Format::Avi
    }

    fn supports(&self, data: &[u8]) -> Support {
        match foreign_riff_form(data) {
            Some(form) => Support::Unsupported(format!("this is a RIFF file of type {form:?}, not AVI")),
            None => Support::Supported,
        }
    }

    /// A file that is RIFF but not AVI produces no candidates, so the engine reports it as an
    /// unsupported format rather than as damage. `supports` above only supplies the wording.
    fn strategies(&self, ctx: &RepairContext<'_>) -> Vec<Box<dyn Strategy>> {
        if foreign_riff_form(ctx.data).is_some() {
            return Vec::new();
        }
        vec![
            Box::new(Verbatim),
            Box::new(TolerantRemux),
            Box::new(TrimToLastGoodFrame),
            Box::new(avi_rebuild::RebuildHeader),
        ]
    }
}

/// The RIFF form type of a file that is *not* an AVI, when that is what it turns out to be.
///
/// A recovery tool names a file by the signature it matched, and `RIFF` is shared by a dozen
/// unrelated formats: a carved collection of 42 `.avi` files held 19 CorelDRAW drawings and a
/// sound bank, all perfectly intact. Running the video strategies on those produced
/// `unrecoverable`, which says the file is damaged beyond help - a false accusation about a
/// healthy file that simply is not a video.
fn foreign_riff_form(data: &[u8]) -> Option<String> {
    if !data.starts_with(b"RIFF") {
        return None;
    }
    let form = data.get(8..12)?;
    if form == b"AVI " || form == b"AVIX" {
        return None;
    }
    Some(String::from_utf8_lossy(form).trim_end().to_string())
}

/// A working copy of the input plus what the container says about it.
struct Source {
    input: InputFile,
    probe: Probe,
}

impl Source {
    /// `None` when ffmpeg sees no media at all.
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

    /// Frames the container claims, used as the denominator in the report.
    fn expected_frames(&self) -> Option<u64> {
        self.probe.declared_frames()
    }

    fn has_video(&self) -> bool {
        self.probe.video().is_some()
    }
}

/// Whether `data` looks like a whole, undamaged AVI container.
///
/// Two checks, both necessary: the file must actually start with the `RIFF`/`AVI ` magic (a
/// file recognized only by its `.avi` extension - the header itself destroyed - must not pass
/// just because a stray zero for the size field trivially satisfies the length check below),
/// and the outer `RIFF` chunk must not declare more bytes than are actually present. RIFF's
/// size field excludes the 8-byte `RIFF`+size header itself, so a whole file satisfies
/// `declared + 8 <= len`.
///
/// Deliberately not an equality check on length: real DVR firmware (measured on two files from
/// the same camera) trails a handful of stray bytes - here, a zero-length `00dc` chunk header
/// plus 8 zero bytes, 16 in total - past the end of the declared RIFF payload without updating
/// the size field. That is harmless footer noise, not damage, and ffmpeg opens such files
/// without complaint; only a file *shorter* than declared has actually lost data.
fn riff_looks_whole(data: &[u8]) -> bool {
    if crate::file_fixer::format::detect_by_magic(data) != Some(Format::Avi) {
        return false;
    }
    let Some(declared) = read_u32_le(data, 4) else {
        return false;
    };
    u64::from(declared) + 8 <= data.len() as u64
}

/// Complaints from the decoder mean the frames it produced are concealment, not content.
pub(super) fn fidelity_of(report: &DecodeReport) -> Fidelity {
    if report.is_clean() { Fidelity::Lossless } else { Fidelity::Reconstructed }
}

/// Content is counted in frames, or in milliseconds for a file that has no video track.
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

/// Nothing localizes a decoder complaint to a frame range, so the whole output is flagged
/// rather than a guessed span.
pub(super) fn reconstructed_span(fidelity: Fidelity, frames: u64) -> Vec<std::ops::Range<u64>> {
    match fidelity {
        Fidelity::Lossless => Vec::new(),
        Fidelity::Reconstructed => std::iter::once(0..frames).collect(),
    }
}

pub(super) fn temp_output() -> Result<tempfile::NamedTempFile> {
    tempfile::Builder::new()
        .prefix("file_fixer-")
        .suffix(".avi")
        .tempfile()
        .map_err(|error| FixerError::write("<temp>", error))
}

/// Builds the candidate, or declines when the mode does not allow reconstructed content.
///
/// A result the decoder complained about is not a subset of the original, it is concealment,
/// so it belongs to the same class as a guessed structure and needs the same permission.
fn candidate_from(name: &str, ctx: &RepairContext<'_>, output: tempfile::NamedTempFile, report: &DecodeReport, expected: Option<u64>, has_video: bool) -> Option<Candidate> {
    let fidelity = fidelity_of(report);
    if fidelity == Fidelity::Reconstructed && !ctx.capabilities.guess {
        ctx.decline(Capabilities {
            guess: true,
            ..Capabilities::NONE
        });
        return None;
    }

    Some(Candidate {
        strategy: name.to_string(),
        data: Payload::TempFile(output.into_temp_path()),
        score: score_of(report, expected, has_video),
        fidelity,
        reconstructed: reconstructed_span(fidelity, content_of(report, has_video)),
    })
}

/// The file plays from end to end; hand it back untouched.
struct Verbatim;

impl Strategy for Verbatim {
    fn name(&self) -> &str {
        "avi_verbatim"
    }

    fn run(&self, ctx: &RepairContext<'_>) -> Result<Option<Candidate>> {
        if !riff_looks_whole(ctx.data) {
            return Ok(None);
        }
        let Some(source) = Source::of(ctx)? else {
            return Ok(None);
        };

        let report = ffmpeg::decode_report(source.path())?;
        if !report.is_clean() {
            return Ok(None);
        }

        Ok(Some(Candidate {
            strategy: self.name().to_string(),
            // Not `source.expected_frames()`: unlike MP4's sample table, an AVI stream
            // header's declared length is not authoritative - measured on a clean re-encode,
            // the index counted 2 more chunks than the decoder ever produced frames for, with
            // no damage involved. `riff_looks_whole` above is the structural completeness
            // check; a clean decode to the end is what "healthy" means, so the total is
            // defined as exactly what was recovered.
            score: score_of(&report, Some(content_of(&report, source.has_video())), source.has_video()),
            data: source.input.into_payload(),
            fidelity: Fidelity::Lossless,
            reconstructed: Vec::new(),
        }))
    }
}

/// Rewrite the container, letting ffmpeg walk past damage instead of stopping at it.
///
/// This is the cheap fix for the common case: the header and index survived, the media data
/// was cut short or has a damaged chunk in the middle, and the rest is perfectly good.
struct TolerantRemux;

impl Strategy for TolerantRemux {
    fn name(&self) -> &str {
        "avi_remux"
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
        if report.frames == 0 {
            return Ok(None);
        }
        Ok(candidate_from(self.name(), ctx, output, &report, source.expected_frames(), source.has_video()))
    }
}

/// How many frame intervals to step back from the failure point, in order of preference. See
/// `mp4_trim` for the reasoning; the same back-off applies here unchanged.
const TRIM_BACK_OFF_FRAMES: [f64; 3] = [1.0, 5.0, 25.0];

/// Cut the file just before the point where decoding stops being reliable.
struct TrimToLastGoodFrame;

impl Strategy for TrimToLastGoodFrame {
    fn name(&self) -> &str {
        "avi_trim"
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
            if report.frames == 0 {
                continue;
            }
            if report.is_clean() {
                return Ok(candidate_from(self.name(), ctx, output, &report, source.expected_frames(), source.has_video()));
            }
            fallback.get_or_insert((output, report));
        }

        Ok(fallback.and_then(|(output, report)| candidate_from(self.name(), ctx, output, &report, source.expected_frames(), source.has_video())))
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;
    use crate::file_fixer::corrupt::{DamageKind, DamageSpec, Fraction, apply_damage};
    use crate::file_fixer::engine::{RepairOptions, RepairResult, repair_file};
    use crate::file_fixer::mode::Mode;
    use crate::file_fixer::report::Outcome;

    fn sample() -> PathBuf {
        PathBuf::from("tmp/a.avi")
    }

    fn skip() -> bool {
        !ffmpeg::is_available() || !sample().exists()
    }

    fn damaged_file(source: &Path, kind: DamageKind) -> tempfile::TempPath {
        let original = std::fs::read(source).expect("the source is readable");
        let damaged = apply_damage(&original, &DamageSpec::new(kind, 1));

        let mut file = tempfile::Builder::new().suffix(".avi").tempfile().expect("temp file");
        std::io::Write::write_all(&mut file, &damaged.data).expect("write");
        file.into_temp_path()
    }

    fn truncated(source: &Path, keep: f64) -> tempfile::TempPath {
        damaged_file(
            source,
            DamageKind::TruncateTail {
                keep: Fraction::new(keep).unwrap(),
            },
        )
    }

    fn repair(path: &Path, mode: Mode) -> RepairResult {
        let options = RepairOptions {
            mode,
            output_dir: Some(std::env::temp_dir().join("file_fixer-avi-tests")),
            force: true,
            ..RepairOptions::default()
        };
        let report = repair_file(path, &options);
        RepairResult {
            report,
            winner: None,
            rejected: Vec::new(),
        }
    }

    #[test]
    fn a_healthy_file_is_recognized_and_left_alone() {
        if skip() {
            return;
        }
        let report = repair(&sample(), Mode::Balanced).report;

        assert_eq!(report.outcome, Outcome::Healthy, "{:?}", report.error);
        assert_eq!(report.winning_strategy.as_deref(), Some("avi_verbatim"));
        assert!(report.output_path.is_none(), "a healthy file must not be rewritten");
    }

    #[test]
    fn a_truncated_recording_keeps_the_frames_that_survived() {
        if skip() {
            return;
        }
        for keep in [0.9, 0.6, 0.3] {
            let damaged = truncated(&sample(), keep);
            let report = repair(&damaged, Mode::Balanced).report;

            assert_ne!(report.outcome, Outcome::Healthy, "keep {keep}");
            assert!(report.output_path.is_some(), "keep {keep}: {:?}", report.error);
        }
    }

    /// More surviving bytes must never yield fewer frames.
    #[test]
    fn recovery_grows_with_the_amount_of_surviving_data() {
        if skip() {
            return;
        }
        let mut previous = 0;
        for keep in [0.3, 0.6, 0.9] {
            let damaged = truncated(&sample(), keep);
            let recovered = repair(&damaged, Mode::Balanced).report.content_recovered.unwrap_or(0);
            assert!(recovered >= previous, "keep {keep} gave {recovered} after {previous}");
            previous = recovered;
        }
    }

    /// Found on a real DVR file: the first bytes zeroed, RIFF header and index gone, but raw
    /// stream data still recognizable to ffmpeg's own heuristic sniffing further into the file.
    fn beheaded(source: &Path, bytes: usize) -> tempfile::TempPath {
        let mut data = std::fs::read(source).expect("the source is readable");
        let zeroed = bytes.min(data.len());
        data.get_mut(..zeroed).expect("the file is longer than that").fill(0);

        let mut file = tempfile::Builder::new().suffix(".avi").tempfile().expect("temp file");
        std::io::Write::write_all(&mut file, &data).expect("write");
        file.into_temp_path()
    }

    /// A file whose header is gone must never be called healthy, whatever ffmpeg's sniffing
    /// makes of the bytes further in.
    #[test]
    fn a_zeroed_header_is_never_reported_healthy() {
        if skip() {
            return;
        }
        let damaged = beheaded(&sample(), 4096);
        let report = repair(&damaged, Mode::Balanced).report;
        assert_ne!(report.outcome, Outcome::Healthy, "{:?}", report.error);
    }

    /// The real case this exists for: the first 2 KB of a recording overwritten, every stream
    /// chunk behind them intact. The container is rebuilt around them and the picture comes
    /// back in full.
    #[test]
    fn a_recording_whose_header_was_destroyed_is_rebuilt_around_its_chunks() {
        if skip() {
            return;
        }
        let whole = repair(&sample(), Mode::Balanced).report;
        let damaged = beheaded(&sample(), 2048);
        let report = repair(&damaged, Mode::Aggressive).report;

        assert_eq!(report.outcome, Outcome::FullyRepaired, "{:?}", report.error);
        assert_eq!(report.winning_strategy.as_deref(), Some("avi_rebuild_header"));
        assert_eq!(report.content_recovered, whole.content_recovered, "every frame of the original must come back");
        assert!(
            report.notes.iter().any(|note| note.contains("picture only")),
            "dropping the audio has to be said out loud: {:?}",
            report.notes
        );
    }

    /// Rebuilding the container is reconstruction, so a mode that may not invent structures
    /// must decline it - and say that a more permissive mode would have worked.
    #[test]
    fn a_destroyed_header_is_not_rebuilt_without_permission_to_guess() {
        if skip() {
            return;
        }
        let damaged = beheaded(&sample(), 2048);
        let report = repair(&damaged, Mode::Balanced).report;

        assert_eq!(report.outcome, Outcome::Unrecoverable);
        assert!(
            report.declined_capabilities.iter().any(|name| name == "guess"),
            "the user has to learn a repair existed: {:?}",
            report.declined_capabilities
        );
    }

    #[test]
    fn safe_mode_will_not_shorten_a_recording() {
        if skip() {
            return;
        }
        let damaged = truncated(&sample(), 0.6);
        let report = repair(&damaged, Mode::Safe).report;

        assert_eq!(report.outcome, Outcome::Unrecoverable);
        assert!(report.output_path.is_none());
    }

    /// A file recognized as AVI by its extension but holding no real media must not crash the
    /// search, and must never be reported as healthy - the honest answer is that it's damaged.
    #[test]
    fn a_non_media_avi_file_does_not_crash_and_is_not_reported_healthy() {
        if !ffmpeg::is_available() {
            return;
        }
        let mut file = tempfile::Builder::new().suffix(".avi").tempfile().expect("temp file");
        std::io::Write::write_all(&mut file, b"not actually a RIFF file, just junk bytes").expect("write");

        let report = repair(file.path(), Mode::Aggressive).report;
        assert_ne!(report.outcome, Outcome::Healthy);
    }

    /// Found in a PhotoRec corpus: 19 of 42 `.avi` files were CorelDRAW drawings and one was a
    /// sound bank, all intact. The extension comes from the `RIFF` signature the recovery tool
    /// matched, and calling such a file unrecoverable accuses a healthy file of being damaged.
    #[test]
    fn a_riff_file_that_is_not_avi_is_declined_rather_than_called_damaged() {
        let mut data = b"RIFF".to_vec();
        data.extend_from_slice(&100u32.to_le_bytes());
        data.extend_from_slice(b"CMX1cont");
        data.extend_from_slice(&[0u8; 128]);

        assert_eq!(foreign_riff_form(&data).as_deref(), Some("CMX1"));
        assert!(matches!(AviRepairer.supports(&data), Support::Unsupported(_)));

        let report = repair_data_at("drawing.avi", &data);
        assert_eq!(report.outcome, Outcome::Unsupported);
        assert!(report.error.is_some_and(|reason| reason.contains("CMX1")), "the reason names it");
    }

    #[test]
    fn a_real_avi_is_not_mistaken_for_a_foreign_riff_file() {
        let mut data = b"RIFF".to_vec();
        data.extend_from_slice(&8u32.to_le_bytes());
        data.extend_from_slice(b"AVI LIST");
        assert_eq!(foreign_riff_form(&data), None);
        assert!(matches!(AviRepairer.supports(&data), Support::Supported));
        assert_eq!(foreign_riff_form(b"not riff at all"), None);
    }

    fn repair_data_at(name: &str, data: &[u8]) -> crate::file_fixer::report::RepairReport {
        let options = RepairOptions {
            mode: Mode::Aggressive,
            ..RepairOptions::default()
        };
        crate::file_fixer::engine::repair_data(Path::new(name), data, &options).report
    }

    #[test]
    fn riff_length_matching_notices_a_shortened_file() {
        let mut data = b"RIFF".to_vec();
        data.extend_from_slice(&100u32.to_le_bytes());
        data.extend_from_slice(b"AVI LIST");
        assert!(!riff_looks_whole(&data), "declared 100 bytes but only 8 follow");

        let mut whole = b"RIFF".to_vec();
        whole.extend_from_slice(&8u32.to_le_bytes());
        whole.extend_from_slice(b"AVI LIST");
        assert!(riff_looks_whole(&whole));
    }

    /// Real DVR firmware trails stray bytes past the declared RIFF size without updating it;
    /// that must not be mistaken for truncation.
    #[test]
    fn riff_length_matching_tolerates_trailing_footer_noise() {
        let mut data = b"RIFF".to_vec();
        data.extend_from_slice(&8u32.to_le_bytes());
        data.extend_from_slice(b"AVI LIST");
        data.extend_from_slice(&[0u8; 16]);
        assert!(riff_looks_whole(&data), "extra trailing bytes are not truncation");
    }

    /// The exact bug found on a real file: a `.avi` recognized only by extension, with its
    /// first bytes zeroed, must not pass just because a zero "declared size" trivially
    /// satisfies `declared + 8 <= len`.
    #[test]
    fn a_destroyed_header_is_never_mistaken_for_whole() {
        let data = vec![0u8; 4096];
        assert!(!riff_looks_whole(&data));
    }
}
