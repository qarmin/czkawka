//! MP4 repair.
//!
//! Strategies are ordered by how invasive they are, cheapest first, so a lightly damaged file
//! short-circuits the search before the expensive options run.
//!
//! Unlike the image formats, quality is judged by actually decoding the result: a candidate
//! that yields more frames with fewer decoder complaints is the better repair. That decoding
//! is what ffmpeg is here for.
//!
//! Whether a result is *faithful* is decided the same way. Copying packets with `-c copy`
//! never re-encodes, so a candidate assembled only from packets the decoder accepted without
//! complaint contains original content. Once the decoder starts complaining, the frames it
//! produces are concealment rather than data, and the candidate says so.
//!
//! A caveat this used to carry - that cutting an inter-coded stream leaves a boundary of one to
//! three frames decoding differently, because what they reference now lies past the cut - turns
//! out not to apply. `-t` on a stream copy keeps packets by *decode* timestamp, and a frame may
//! only predict from pictures earlier in decode order, so what a trim keeps is closed under
//! reference however far presentation order and decode order have drifted apart. Measured by
//! hashing every decoded frame against the healthy original on three streams, HEVC B-pyramids
//! included: 671 of 671, 479 of 479 and 46 of 47 identical. The single exception is the damage
//! rather than the cut - there the surviving packets already form a complete, reference-closed
//! set and the last picture still decodes differently, so no choice of cut point recovers it.
//! See `mp4_trim::tests::a_trim_damages_no_frame_it_keeps`.

use std::path::Path;

use super::ffmpeg::{self, DecodeReport, Probe};
use super::mp4_fragments::CutAtLastFragment;
use super::mp4_rebuild::RebuildIndex;
use super::mp4_relink::RelinkBuriedIndex;
use super::mp4_trim::TrimToLastGoodFrame;
use super::{mp4_boxes, mp4_elementary};
use crate::file_fixer::candidate::{Candidate, Fidelity, Payload, Score};
use crate::file_fixer::engine::{InputFile, RepairContext, Repairer, Strategy};
use crate::file_fixer::error::{FixerError, Result};
use crate::file_fixer::format::Format;
use crate::file_fixer::mode::Capabilities;

pub struct Mp4Repairer;

impl Repairer for Mp4Repairer {
    fn format(&self) -> Format {
        Format::Mp4
    }

    fn strategies(&self, _ctx: &RepairContext<'_>) -> Vec<Box<dyn Strategy>> {
        vec![
            Box::new(Verbatim),
            Box::new(TolerantRemux),
            Box::new(RelinkBuriedIndex),
            // Before the trim: a fragment boundary is structural, so it needs no decoder to
            // find and the result is a prefix of the input rather than a re-muxed copy.
            Box::new(CutAtLastFragment),
            Box::new(TrimToLastGoodFrame),
            Box::new(RebuildIndex),
            // The two codecs a NAL scan cannot reach. Both are cheap to decline - one looks for
            // a JPEG marker, the other for a start code - so they cost nothing on a file that
            // holds neither.
            Box::new(mp4_elementary::CarveMotionJpeg),
            Box::new(mp4_elementary::CarveMpeg4Part2),
            Box::new(mp4_elementary::CarveH263),
        ]
    }
}

/// A working copy of the input plus what the container says about it.
pub(super) struct Source {
    input: InputFile,
    probe: Probe,
}

impl Source {
    /// `None` when ffmpeg sees no media at all.
    ///
    /// Audio-only files are accepted: `ffprobe` reports success on almost anything, so the
    /// presence of *a* stream is the real test, and a file whose index is gone has none.
    pub(super) fn of(ctx: &RepairContext<'_>) -> Result<Option<Self>> {
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

    pub(super) fn path(&self) -> &Path {
        self.input.path()
    }

    /// Frames the container claims, used as the denominator in the report.
    pub(super) fn expected_frames(&self) -> Option<u64> {
        self.probe.declared_frames()
    }

    pub(super) fn has_video(&self) -> bool {
        self.probe.video().is_some()
    }
}

/// Complaints from the decoder mean the frames it produced are concealment, not content.
pub(super) fn fidelity_of(report: &DecodeReport) -> Fidelity {
    if report.is_clean() { Fidelity::Lossless } else { Fidelity::Reconstructed }
}

/// Content is counted in frames, or in milliseconds for a file that has no video track. Either
/// way it answers the same question: how much of the recording came back.
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

/// A temporary output in the same container family as `path`.
///
/// The container has to follow the input, because the ISO-BMFF family does not accept the same
/// codecs throughout: MP4 has no tag for H.263 or AMR, both of which are ordinary in a 3GP file
/// off an older phone. Writing every candidate as `.mp4` made ffmpeg refuse the remux *and* the
/// trim with "Could not find tag for codec h263 in stream #0" - so a 3GP whose only problem was
/// a missing last frame came back `unrecoverable`. Measured on a carved collection: 67 files in
/// exactly that state.
pub(super) fn temp_output_like(path: &std::path::Path) -> Result<tempfile::NamedTempFile> {
    let suffix = match path.extension().and_then(|extension| extension.to_str()).map(str::to_ascii_lowercase).as_deref() {
        Some("3gp" | "3g2") => ".3gp",
        Some("mov" | "qt") => ".mov",
        _ => ".mp4",
    };
    tempfile::Builder::new()
        .prefix("file_fixer-")
        .suffix(suffix)
        .tempfile()
        .map_err(|error| FixerError::write("<temp>", error))
}

pub(super) fn temp_output() -> Result<tempfile::NamedTempFile> {
    tempfile::Builder::new()
        .prefix("file_fixer-")
        .suffix(".mp4")
        .tempfile()
        .map_err(|error| FixerError::write("<temp>", error))
}

/// Builds the candidate, or declines when the mode does not allow reconstructed content.
///
/// A result the decoder complained about is not a subset of the original, it is concealment,
/// so it belongs to the same class as a guessed structure and needs the same permission.
pub(super) fn candidate_from(
    name: &str,
    ctx: &RepairContext<'_>,
    output: tempfile::NamedTempFile,
    report: &DecodeReport,
    expected: Option<u64>,
    has_video: bool,
) -> Option<Candidate> {
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
        "mp4_verbatim"
    }

    fn run(&self, ctx: &RepairContext<'_>) -> Result<Option<Candidate>> {
        if mp4_boxes::structure(ctx.data).any_truncated {
            return Ok(None);
        }
        let Some(source) = Source::of(ctx)? else {
            return Ok(None);
        };

        let report = ffmpeg::decode_report(source.path())?;
        let expected = source.expected_frames();
        if !report.is_clean() || expected.is_some_and(|frames| report.frames < frames) {
            return Ok(None);
        }

        Ok(Some(Candidate {
            strategy: self.name().to_string(),
            score: score_of(&report, expected.or(Some(content_of(&report, source.has_video()))), source.has_video()),
            data: source.input.into_payload(),
            fidelity: Fidelity::Lossless,
            reconstructed: Vec::new(),
        }))
    }
}

/// Rewrite the container, letting ffmpeg walk past damage instead of stopping at it.
///
/// This is the cheap fix for the common case: the index survived, the media data was cut
/// short, and the samples that are present are perfectly good.
struct TolerantRemux;

impl Strategy for TolerantRemux {
    fn name(&self) -> &str {
        "mp4_remux"
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
        let output = temp_output_like(ctx.path)?;
        ffmpeg::remux(source.path(), output.path(), true)?;

        let report = ffmpeg::decode_report(output.path())?;
        if report.frames == 0 {
            return Ok(None);
        }
        Ok(candidate_from(self.name(), ctx, output, &report, source.expected_frames(), source.has_video()))
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
        PathBuf::from("tmp/a.mp4")
    }

    fn skip() -> bool {
        !ffmpeg::is_available() || !sample().exists()
    }

    /// Damaged copies are written to disk because every video strategy works through ffmpeg,
    /// which reads files rather than buffers.
    fn damaged_file(source: &Path, kind: DamageKind) -> tempfile::TempPath {
        let original = std::fs::read(source).expect("the source is readable");
        let damaged = apply_damage(&original, &DamageSpec::new(kind, 1));

        let mut file = tempfile::Builder::new().suffix(".mp4").tempfile().expect("temp file");
        std::io::Write::write_all(&mut file, &damaged.data).expect("write");
        file.into_temp_path()
    }

    /// A copy with the index moved to the front, which is what streaming-oriented encoders
    /// produce. Where `moov` sits decides everything about surviving a truncation, so both
    /// layouts need covering.
    fn faststart_copy() -> tempfile::TempPath {
        let output = tempfile::Builder::new().suffix(".mp4").tempfile().expect("temp file");
        ffmpeg::remux(&sample(), output.path(), false).expect("remuxing works");
        output.into_temp_path()
    }

    fn repair(path: &Path, mode: Mode) -> RepairResult {
        repair_with_references(path, mode, Vec::new())
    }

    fn repair_with_references(path: &Path, mode: Mode, references: Vec<PathBuf>) -> RepairResult {
        let options = RepairOptions {
            mode,
            references,
            output_dir: Some(std::env::temp_dir().join("file_fixer-mp4-tests")),
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

        assert_eq!(report.outcome, Outcome::Healthy);
        assert_eq!(report.winning_strategy.as_deref(), Some("mp4_verbatim"));
        assert!(report.output_path.is_none(), "a healthy file must not be rewritten");
    }

    /// This file keeps `moov` at the end, so truncation takes the whole index with it. The
    /// media data survives but nothing describes it, and ffmpeg cannot open the file at all.
    fn index_less_file() -> tempfile::TempPath {
        let damaged = damaged_file(
            &sample(),
            DamageKind::TruncateTail {
                keep: Fraction::new(0.6).unwrap(),
            },
        );
        let data = std::fs::read(&damaged).expect("readable");
        assert!(!mp4_boxes::structure(&data).has_moov, "the fixture must have lost its index");
        assert!(ffmpeg::probe(&damaged).is_err(), "ffmpeg is expected to reject a file with no index");
        damaged
    }

    fn truncated_faststart(source: &Path, keep: f64) -> tempfile::TempPath {
        damaged_file(
            source,
            DamageKind::TruncateTail {
                keep: Fraction::new(keep).unwrap(),
            },
        )
    }

    #[test]
    fn a_truncated_recording_keeps_the_frames_that_survived() {
        if skip() {
            return;
        }
        let faststart = faststart_copy();
        for keep in [0.9, 0.6, 0.3] {
            let damaged = truncated_faststart(&faststart, keep);
            let report = repair(&damaged, Mode::Balanced).report;

            assert_eq!(report.outcome, Outcome::PartiallyRecovered, "keep {keep}");
            let recovered = report.content_recovered.expect("frames were recovered");
            assert!(recovered > 0 && recovered < 830, "keep {keep} recovered {recovered}");
            assert!(report.output_path.is_some(), "keep {keep}");
        }
    }

    /// More surviving bytes must never yield fewer frames.
    #[test]
    fn recovery_grows_with_the_amount_of_surviving_data() {
        if skip() {
            return;
        }
        let faststart = faststart_copy();
        let mut previous = 0;

        for keep in [0.3, 0.6, 0.9] {
            let damaged = truncated_faststart(&faststart, keep);
            let recovered = repair(&damaged, Mode::Balanced).report.content_recovered.unwrap_or(0);
            assert!(recovered >= previous, "keep {keep} gave {recovered} after {previous}");
            previous = recovered;
        }
    }

    #[test]
    fn safe_mode_will_not_shorten_a_recording() {
        if skip() {
            return;
        }
        let damaged = truncated_faststart(&faststart_copy(), 0.6);
        let report = repair(&damaged, Mode::Safe).report;

        assert_eq!(report.outcome, Outcome::Unrecoverable);
        assert!(report.output_path.is_none());
    }

    /// Losing the tail of this file means losing `moov`, which is where every sample offset
    /// lives. Until reconstruction exists, that must fail loudly rather than produce a file.
    #[test]
    fn losing_the_index_without_a_reference_is_unrecoverable() {
        if skip() {
            return;
        }
        let report = repair(&index_less_file(), Mode::Aggressive).report;

        assert_eq!(report.outcome, Outcome::Unrecoverable);
        assert!(report.output_path.is_none());
    }

    /// The headline case: a healthy recording from the same source supplies the parameter sets
    /// that the lost index took with it, and the media data is scanned back into a playable
    /// file.
    #[test]
    fn a_reference_makes_an_index_less_file_recoverable() {
        if skip() {
            return;
        }
        let report = repair_with_references(&index_less_file(), Mode::Aggressive, vec![sample()]).report;

        assert_eq!(report.outcome, Outcome::PartiallyRecovered);
        assert_eq!(report.winning_strategy.as_deref(), Some("mp4_rebuild_index"));
        let recovered = report.content_recovered.expect("frames were recovered");
        assert!(recovered > 200, "only {recovered} frames recovered");

        // The scan cannot prove that what it accepted was real, so this never claims fidelity.
        assert!(!report.reconstructed.is_empty());
        let output = report.output_path.expect("a file was written");
        assert!(ffmpeg::probe(&output).is_ok_and(|probe| probe.video().is_some()));
    }

    /// HEVC is what every camera bought in the last decade records, and until the scanner read
    /// two-byte NAL headers an index-less HEVC recording was reported unrecoverable while the
    /// same damage to an H.264 one came back in full.
    #[test]
    fn an_index_less_hevc_recording_is_rebuilt_like_an_h264_one() {
        let sample = PathBuf::from("tmp/samples/clip_hevc.mp4");
        if !ffmpeg::is_available() || !sample.exists() {
            return;
        }
        let whole = ffmpeg::probe(&sample).expect("the sample probes");
        let damaged = truncated_faststart(&sample, 0.6);
        assert!(
            !mp4_boxes::structure(&std::fs::read(&damaged).expect("readable")).has_moov,
            "the fixture must have lost its index"
        );

        let report = repair_with_references(&damaged, Mode::Aggressive, vec![sample]).report;
        assert_eq!(report.winning_strategy.as_deref(), Some("mp4_rebuild_index"), "{report:?}");

        let recovered = report.content_recovered.expect("frames were recovered");
        let declared = whole.declared_frames().expect("the sample declares its frames");
        assert!(recovered > declared / 3, "only {recovered} of {declared} frames recovered");
        let output = report.output_path.expect("a file was written");
        let probe = ffmpeg::probe(&output).expect("the result opens");
        assert_eq!(probe.video().map(|stream| stream.codec.as_str()), Some("hevc"));
    }

    /// Rebuilding invents the index and takes its timing from the reference, so the lossless
    /// modes must not do it.
    #[test]
    fn rebuilding_an_index_needs_the_guess_capability() {
        if skip() {
            return;
        }
        let damaged = index_less_file();
        for mode in [Mode::Safe, Mode::Balanced] {
            let report = repair_with_references(&damaged, mode, vec![sample()]).report;
            assert_eq!(report.outcome, Outcome::Unrecoverable, "{mode}");
        }
    }

    /// What a photo-recovery tool writes: `ftyp`, then one `mdat` whose length claims the rest
    /// of the file and so swallows the index sitting behind the media data.
    fn carved_file() -> tempfile::TempPath {
        let data = std::fs::read(sample()).expect("the source is readable");
        let mdat = mp4_boxes::find_all(&data, *b"mdat").pop().expect("the sample has media data");

        let mut carved = data;
        let claimed = (carved.len() - mdat.start) as u32;
        carved
            .get_mut(mdat.start..mdat.start + 4)
            .expect("the header is present")
            .copy_from_slice(&claimed.to_be_bytes());

        let mut file = tempfile::Builder::new().suffix(".mp4").tempfile().expect("temp file");
        std::io::Write::write_all(&mut file, &carved).expect("write");
        file.into_temp_path()
    }

    /// The damage is one wrong number, and it is enough to make the file unopenable.
    #[test]
    fn a_carved_file_hides_a_perfectly_good_index() {
        if skip() {
            return;
        }
        let carved = carved_file();
        let data = std::fs::read(&carved).expect("readable");

        assert!(!mp4_boxes::structure(&data).has_moov, "the index must be out of reach");
        assert!(ffmpeg::probe(&carved).is_err(), "ffmpeg is expected to refuse it");
        assert_eq!(mp4_boxes::find_indexes(&data).len(), 1, "yet the index is still there");
    }

    /// The headline repair: nothing is missing, so freeing the index gives the whole recording
    /// back - audio, timing and all - without re-encoding or guessing anything.
    #[test]
    fn a_carved_file_is_relinked_without_needing_aggressive_mode() {
        if skip() {
            return;
        }
        let report = repair(&carved_file(), Mode::Balanced).report;

        assert_eq!(report.winning_strategy.as_deref(), Some("mp4_relink"), "{:?}", report.error);
        assert_eq!(report.outcome, Outcome::FullyRepaired);
        assert!(report.reconstructed.is_empty(), "no part of this is invented");

        let output = report.output_path.expect("a file was written");
        let probe = ffmpeg::probe(&output).expect("the result opens");
        assert!(probe.video().is_some());
        assert!(probe.has_audio(), "relinking keeps the audio the scan cannot recover");
    }

    /// A copy with padding inserted after `ftyp`, so every chunk offset in the index now points
    /// that many bytes too early - what a carve that began at a different byte leaves behind.
    fn shifted_file(bytes: usize) -> tempfile::TempPath {
        let data = std::fs::read(sample()).expect("the source is readable");
        let ftyp = mp4_boxes::top_level(&data)
            .into_iter()
            .find(|found| found.is(*b"ftyp"))
            .expect("the sample has a file type box");

        let mut padding = (bytes as u32).to_be_bytes().to_vec();
        padding.extend_from_slice(b"free");
        padding.resize(bytes, 0);

        let mut shifted = data.get(..ftyp.payload.end).expect("a prefix").to_vec();
        shifted.extend_from_slice(&padding);
        shifted.extend_from_slice(data.get(ftyp.payload.end..).expect("the rest"));

        let mut file = tempfile::Builder::new().suffix(".mp4").tempfile().expect("temp file");
        std::io::Write::write_all(&mut file, &shifted).expect("write");
        file.into_temp_path()
    }

    /// The sample table is perfectly intact and perfectly wrong: every offset is off by the
    /// same amount. Correcting the numbers gives the whole recording back - audio, timing and
    /// all - where a stream scan would have produced video only, and worse video at that.
    #[test]
    fn an_index_whose_offsets_are_all_shifted_is_corrected_rather_than_abandoned() {
        if skip() {
            return;
        }
        let shifted = shifted_file(4096);
        assert!(
            ffmpeg::decode_report(&shifted).is_ok_and(|report| !report.is_clean()),
            "the fixture has to be broken for this to mean anything"
        );

        let report = repair(&shifted, Mode::Balanced).report;
        assert_eq!(report.winning_strategy.as_deref(), Some("mp4_relink"), "{report:?}");
        assert_eq!(report.outcome, Outcome::FullyRepaired);
        assert!(report.reconstructed.is_empty(), "no part of this is invented");

        let output = report.output_path.expect("a file was written");
        let probe = ffmpeg::probe(&output).expect("the result opens");
        assert!(probe.has_audio(), "correcting offsets keeps what a scan cannot recover");

        // The strong claim: not merely that it decodes, but that it decodes to the same pictures.
        let repaired = ffmpeg::frame_hashes(&output).expect("the result hashes");
        let original = ffmpeg::frame_hashes(&sample()).expect("the original hashes");
        assert_eq!(repaired, original, "every frame must come back exactly");
    }

    /// Reporting `unrecoverable` for a file the tool could repair, but was not permitted to,
    /// reads as "this recording is beyond help" and gets it deleted.
    #[test]
    fn a_repair_the_mode_forbids_is_advertised() {
        if skip() {
            return;
        }
        let report = repair_with_references(&index_less_file(), Mode::Balanced, vec![sample()]).report;

        assert_eq!(report.outcome, Outcome::Unrecoverable);
        assert_eq!(report.declined_capabilities, vec!["guess".to_string()]);
        assert!(report.summary_line().contains("--mode aggressive"), "{}", report.summary_line());
    }

    #[test]
    fn a_file_that_is_not_media_at_all_is_reported_unsupported() {
        if !ffmpeg::is_available() {
            return;
        }
        let report = repair(Path::new("Cargo.toml"), Mode::Aggressive).report;
        assert_eq!(report.outcome, Outcome::Unsupported);
    }

    /// Found by running the tool over a few hundred real files: an audio-only MP4 has no video
    /// stream, and requiring one made every such healthy file look unrepairable.
    #[test]
    fn an_audio_only_file_is_recognized_as_healthy() {
        if !ffmpeg::is_available() {
            return;
        }
        let output = tempfile::Builder::new().suffix(".m4a").tempfile().expect("temp file");
        let made = std::process::Command::new("ffmpeg")
            .args(["-v", "error", "-y", "-f", "lavfi", "-i", "sine=frequency=440:duration=1", "-c:a", "aac"])
            .arg(output.path())
            .status()
            .is_ok_and(|status| status.success());
        if !made {
            return;
        }

        let report = repair(output.path(), Mode::Balanced).report;
        assert_eq!(report.outcome, Outcome::Healthy, "{:?}", report.error);
        assert_eq!(report.winning_strategy.as_deref(), Some("mp4_verbatim"));
    }
}
