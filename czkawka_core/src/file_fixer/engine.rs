//! Candidate generation, scoring and selection.

use std::panic::{self, AssertUnwindSafe};
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};
use std::time::{Duration, Instant};

use rayon::prelude::*;

use crate::file_fixer::candidate::{Candidate, DEFAULT_LOSSLESS_THRESHOLD, Payload, select_best};
use crate::file_fixer::error::{FixerError, Result};
use crate::file_fixer::format::{Detection, Format, detect};
use crate::file_fixer::io::{DEFAULT_ALLOCATION_LIMIT, SourceFile};
use crate::file_fixer::mode::{Capabilities, CapabilityOverrides, Mode};
use crate::file_fixer::output::{keep_candidates, strategy_was_passthrough, write_output};
use crate::file_fixer::report::{self, Outcome, RepairReport};

/// Bounds the search. Candidate count is the product of (references x strategies x variants),
/// which grows fast, and a single full decode of a 4K clip is measured in tens of seconds.
#[derive(Debug, Clone, Copy)]
pub struct Budget {
    pub deadline: Option<Duration>,
    pub max_candidates: Option<usize>,
    /// When false, the search stops as soon as a candidate recovers everything cleanly.
    pub exhaustive: bool,
}

impl Default for Budget {
    fn default() -> Self {
        Self {
            deadline: Some(Duration::from_secs(60)),
            max_candidates: Some(64),
            exhaustive: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RepairOptions {
    pub mode: Mode,
    pub overrides: CapabilityOverrides,
    pub budget: Budget,
    pub lossless_threshold: f64,
    pub allocation_limit: u64,
    /// Healthy files of the same provenance, used to supply structures the damaged file lost:
    /// MP4 decoder parameters, JPEG quantization tables, a TIFF directory, a BMP geometry, a
    /// WAV sample rate.
    pub references: Vec<PathBuf>,
    pub output_dir: Option<PathBuf>,
    pub keep_candidates: Option<PathBuf>,
    pub force: bool,
}

impl Default for RepairOptions {
    fn default() -> Self {
        Self {
            mode: Mode::default(),
            overrides: CapabilityOverrides::default(),
            budget: Budget::default(),
            lossless_threshold: DEFAULT_LOSSLESS_THRESHOLD,
            allocation_limit: DEFAULT_ALLOCATION_LIMIT,
            references: Vec::new(),
            output_dir: None,
            keep_candidates: None,
            force: false,
        }
    }
}

impl RepairOptions {
    pub fn capabilities(&self) -> Capabilities {
        self.overrides.apply_to(self.mode.capabilities())
    }
}

/// Everything a strategy is allowed to look at.
pub struct RepairContext<'a> {
    pub path: &'a Path,
    pub data: &'a [u8],
    pub capabilities: Capabilities,
    pub allocation_limit: u64,
    pub references: &'a [PathBuf],
    /// True when `data` is the entire contents of `path`, so external tools may read it
    /// directly instead of being handed a copy.
    data_is_whole_file: bool,
    deadline: Option<Instant>,
    /// Set for any reason to stop, including a perfect candidate having been found.
    stop: &'a AtomicBool,
    /// Set only when the time or count budget ran out, which is what the report cares about:
    /// stopping early because the search succeeded is not the same as running out of room.
    exhausted: &'a AtomicBool,
    /// Capabilities that would have admitted a candidate the mode turned away, packed by
    /// [`Capabilities::bits`].
    declined: &'a AtomicU8,
    /// Things about the result the user has to be told even though the repair succeeded.
    notes: &'a Mutex<Vec<String>>,
}

impl RepairContext<'_> {
    /// Records that a repair existed but the mode forbade it.
    ///
    /// Without this an `unrecoverable` is ambiguous between "nothing could be done" and "the
    /// tool found something and was not allowed to hand it over", and only the second is worth
    /// the user's time to retry.
    pub fn decline(&self, required: Capabilities) {
        let missing = required.missing_from(self.capabilities);
        if !missing.is_empty() {
            self.declined.fetch_or(missing.bits(), Ordering::Relaxed);
        }
    }

    /// Records something the user needs to know about the file that no score can express.
    ///
    /// The case this exists for: a carved recording whose media turns out to hold runs from
    /// more than one original. The repair genuinely recovers all of them and scores well for
    /// it, and the output is still not one recording - watching it, the picture changes to an
    /// unrelated scene partway through. Nothing in a score says that, so it is said here.
    pub fn note(&self, text: String) {
        if let Ok(mut notes) = self.notes.lock()
            && !notes.contains(&text)
        {
            notes.push(text);
        }
    }

    /// Strategies doing long scans should poll this and bail out early.
    pub fn should_stop(&self) -> bool {
        if self.stop.load(Ordering::Relaxed) {
            return true;
        }
        if crate::file_fixer::cancel::is_cancelled() {
            self.stop.store(true, Ordering::Relaxed);
            return true;
        }
        if self.deadline.is_some_and(|deadline| Instant::now() >= deadline) {
            self.exhausted.store(true, Ordering::Relaxed);
            self.stop.store(true, Ordering::Relaxed);
            return true;
        }
        false
    }

    /// A path an external tool can read the input from.
    ///
    /// Returns the original file whenever the data came from it, so handing a 50 GB recording
    /// to ffmpeg costs nothing. Only detached byte buffers, which is what the library API and
    /// the tests pass, are spilled to a temporary file.
    pub fn input_file(&self) -> Result<InputFile> {
        if self.data_is_whole_file {
            return Ok(InputFile::Existing(self.path.to_path_buf()));
        }

        let extension = self.path.extension().map_or_else(String::new, |e| format!(".{}", e.to_string_lossy()));
        let mut file = tempfile::Builder::new().suffix(&extension).tempfile().map_err(|e| FixerError::write(self.path, e))?;
        std::io::Write::write_all(&mut file, self.data).map_err(|e| FixerError::write(self.path, e))?;
        Ok(InputFile::Temporary(file.into_temp_path()))
    }
}

/// Where an external tool should read the input from.
pub enum InputFile {
    Existing(PathBuf),
    /// Deleted when dropped.
    Temporary(tempfile::TempPath),
}

impl InputFile {
    pub fn path(&self) -> &Path {
        match self {
            Self::Existing(path) => path,
            Self::Temporary(path) => path,
        }
    }

    /// Turns the input into a candidate payload for strategies that conclude the file is
    /// already correct. Ownership of a temporary file moves across, so it outlives this value.
    pub fn into_payload(self) -> Payload {
        match self {
            Self::Existing(path) => Payload::SameAsInput(path),
            Self::Temporary(path) => Payload::TempFile(path),
        }
    }
}

/// One way of producing a repaired file. Cheap and likely strategies come first so that the
/// budget is spent on the most promising options.
pub trait Strategy: Send + Sync {
    fn name(&self) -> &str;

    /// Capabilities this strategy needs before it may run.
    fn requires(&self) -> Capabilities {
        Capabilities::NONE
    }

    /// `Ok(None)` means the strategy does not apply to this file, which is not an error.
    fn run(&self, ctx: &RepairContext<'_>) -> Result<Option<Candidate>>;
}

/// Whether a repairer can say anything trustworthy about a file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Support {
    Supported,
    /// Recognized as this format, but a variant the tool neither verifies nor repairs. Kept
    /// separate from failure so that a perfectly good file is never reported as damaged.
    Unsupported(String),
}

pub trait Repairer: Send + Sync {
    fn format(&self) -> Format;

    /// Checked before any strategy runs, so an unhandled variant is reported as such rather
    /// than as an unexplained failure.
    fn supports(&self, _data: &[u8]) -> Support {
        Support::Supported
    }

    fn strategies(&self, ctx: &RepairContext<'_>) -> Vec<Box<dyn Strategy>>;
}

fn capabilities_satisfy(available: Capabilities, required: Capabilities) -> bool {
    (!required.truncate || available.truncate) && (!required.reencode || available.reencode) && (!required.guess || available.guess) && (!required.fill_gaps || available.fill_gaps)
}

pub struct RepairResult {
    pub report: RepairReport,
    pub winner: Option<Candidate>,
    /// The candidates that lost, retained only when [`RepairOptions::keep_candidates`] asks
    /// for them. Holding several copies of a video in memory by default would be wasteful.
    pub rejected: Vec<Candidate>,
}

/// Runs every applicable strategy and picks a winner, without touching the filesystem.
pub fn repair_data(path: &Path, data: &[u8], options: &RepairOptions) -> RepairResult {
    repair_source(path, data, options, false)
}

fn repair_source(path: &Path, data: &[u8], options: &RepairOptions, data_is_whole_file: bool) -> RepairResult {
    let started = Instant::now();
    let capabilities = options.capabilities();
    let deadline = options.budget.deadline.map(|d| started + d);
    let stop = AtomicBool::new(false);
    let exhausted = AtomicBool::new(false);
    let declined = AtomicU8::new(0);
    let notes = Mutex::new(Vec::new());

    let detections = detect(path, data, capabilities.guess);
    let Some(primary) = detections.first().copied() else {
        // Not a format this tool knows. That is not the same as damage, and saying so would
        // be a lie about, for instance, a perfectly healthy Matroska file.
        let mut report = RepairReport::failed(path.to_path_buf(), options.mode, FixerError::UnknownFormat { path: path.to_path_buf() }.to_string());
        report.outcome = Outcome::Unsupported;
        return RepairResult {
            report,
            winner: None,
            rejected: Vec::new(),
        };
    };

    let ctx = RepairContext {
        path,
        data,
        capabilities,
        allocation_limit: options.allocation_limit,
        references: &options.references,
        data_is_whole_file,
        deadline,
        stop: &stop,
        exhausted: &exhausted,
        declined: &declined,
        notes: &notes,
    };

    log::debug!("{}: {} byte(s), detected {} via {:?}", path.display(), data.len(), primary.format, primary.source);

    let (candidates, capped) = collect_candidates(&detections, &ctx, options);
    let budget_exhausted = exhausted.load(Ordering::Relaxed) || capped;
    let declined = Capabilities::from_bits(declined.load(Ordering::Relaxed));
    let unsupported = candidates.is_empty().then(|| unsupported_reason(&detections, data)).flatten();

    finish(
        path,
        primary,
        candidates,
        options,
        started,
        budget_exhausted,
        unsupported,
        declined,
        notes.into_inner().unwrap_or_default(),
    )
}

/// Returns the candidates and whether the strategy list had to be cut short by the count cap.
fn collect_candidates(detections: &[Detection], ctx: &RepairContext<'_>, options: &RepairOptions) -> (Vec<Candidate>, bool) {
    let mut planned: Vec<Box<dyn Strategy>> = Vec::new();
    for detection in detections {
        let repairer = crate::file_fixer::formats::repairer_for(detection.format);
        for strategy in repairer.strategies(ctx) {
            if capabilities_satisfy(ctx.capabilities, strategy.requires()) {
                planned.push(strategy);
            } else {
                ctx.decline(strategy.requires());
            }
        }
    }

    let capped = options.budget.max_candidates.is_some_and(|max| planned.len() > max);
    if let Some(max) = options.budget.max_candidates {
        planned.truncate(max);
    }

    let candidates = planned.par_iter().filter_map(|strategy| run_strategy(strategy.as_ref(), ctx, options)).collect();
    (candidates, capped)
}

/// The reason no repairer could help, when every detected format declines the variant.
fn unsupported_reason(detections: &[Detection], data: &[u8]) -> Option<String> {
    detections
        .iter()
        .map(|detection| crate::file_fixer::formats::repairer_for(detection.format).supports(data))
        .try_fold(None, |found: Option<String>, support| match support {
            Support::Supported => Err(()),
            Support::Unsupported(reason) => Ok(found.or(Some(reason))),
        })
        .unwrap_or(None)
}

/// Runs one strategy, isolating it from the rest of the search.
///
/// Third-party decoders are fed deliberately hostile bytes here. A panic inside one is a bug
/// worth reporting upstream, but it must not take down a batch run over ten thousand files.
fn run_strategy(strategy: &dyn Strategy, ctx: &RepairContext<'_>, options: &RepairOptions) -> Option<Candidate> {
    if ctx.should_stop() {
        ctx.stop.store(true, Ordering::Relaxed);
        return None;
    }

    let started = Instant::now();
    let outcome = panic::catch_unwind(AssertUnwindSafe(|| strategy.run(ctx)));
    let candidate = match outcome {
        Ok(Ok(candidate)) => candidate,
        Ok(Err(error)) => {
            log::debug!("strategy {} did not apply: {error}", strategy.name());
            None
        }
        Err(_) => {
            log::warn!("strategy {} panicked on {}; skipping it", strategy.name(), ctx.path.display());
            None
        }
    };

    match &candidate {
        Some(found) => log::debug!(
            "{}: {} recovered {} unit(s) with {} decoder error(s) in {} ms",
            ctx.path.display(),
            strategy.name(),
            found.score.content_recovered,
            found.score.decoder_errors,
            started.elapsed().as_millis()
        ),
        None => log::debug!("{}: {} produced nothing in {} ms", ctx.path.display(), strategy.name(), started.elapsed().as_millis()),
    }

    if !options.budget.exhaustive && candidate.as_ref().is_some_and(|c| c.score.is_perfect()) {
        ctx.stop.store(true, Ordering::Relaxed);
    }
    candidate
}

fn finish(
    path: &Path,
    detection: Detection,
    candidates: Vec<Candidate>,
    options: &RepairOptions,
    started: Instant,
    budget_exhausted: bool,
    unsupported: Option<String>,
    declined: Capabilities,
    notes: Vec<String>,
) -> RepairResult {
    let selection = select_best(&candidates, options.lossless_threshold);
    let winner_index = selection.map(|s| s.index);

    let mut report = RepairReport {
        path: path.to_path_buf(),
        format: Some(detection.format),
        detected_by: Some(detection.source),
        mode: options.mode,
        outcome: Outcome::Unrecoverable,
        output_path: None,
        winning_strategy: None,
        lossless_preferred: selection.is_some_and(|s| report::selection_reason_is_lossless(s.reason)),
        reconstructed: Vec::new(),
        content_recovered: None,
        content_total: None,
        candidates: report::candidate_reports(&candidates, winner_index),
        budget_exhausted,
        declined_capabilities: declined.names().into_iter().map(str::to_string).collect(),
        notes,
        elapsed_ms: started.elapsed().as_millis() as u64,
        error: None,
    };

    let Some(index) = winner_index else {
        if let Some(reason) = unsupported {
            report.outcome = Outcome::Unsupported;
            report.error = Some(reason);
        }
        return RepairResult {
            report,
            winner: None,
            rejected: Vec::new(),
        };
    };
    let mut candidates = candidates;
    let winner = candidates.remove(index);

    report.outcome = report::outcome_for(&winner.score);
    report.winning_strategy = Some(winner.strategy.clone());
    report.reconstructed = winner.reconstructed.clone();
    report.content_recovered = Some(winner.score.content_recovered);
    report.content_total = winner.score.content_total;

    RepairResult {
        report,
        winner: Some(winner),
        rejected: if options.keep_candidates.is_some() { candidates } else { Vec::new() },
    }
}

/// Repairs a file on disk. The input is opened read-only and is never modified.
pub fn repair_file(path: &Path, options: &RepairOptions) -> RepairReport {
    let source = match SourceFile::open(path) {
        Ok(source) => source,
        Err(error) => return RepairReport::failed(path.to_path_buf(), options.mode, error.to_string()),
    };

    let mut result = repair_source(path, source.data(), options, true);

    // A file whose best candidate is the file itself gets reported, not rewritten - a
    // byte-identical copy with a new name is noise, and in a batch run it would double the size
    // of an archive. When that candidate is also perfect the file was simply healthy; when it
    // is not, the damage is real but nothing we can produce improves on the original, which is
    // the usual verdict on an MPEG carve whose pictures are damaged inside an intact container.
    if strategy_was_passthrough(&result) {
        if result.report.outcome == Outcome::FullyRepaired {
            result.report.outcome = Outcome::Healthy;
        } else {
            // Otherwise "partial, 100%, nothing written" is a riddle. Say the thing plainly:
            // the damage is real, every strategy ran, and none of them improved on the file.
            result
                .report
                .notes
                .push("the damage is inside the content; no repair improved on the file itself".to_string());
        }
        return result.report;
    }

    if crate::file_fixer::cancel::is_cancelled() {
        // A child killed mid-run yields truncated candidates that must never be written out.
        result.report.error = Some("cancelled".to_string());
        return result.report;
    }

    if let Some(directory) = &options.keep_candidates {
        keep_candidates(path, &result, directory);
    }
    if let Some(winner) = &result.winner {
        match write_output(path, &winner.data, options) {
            Ok(output_path) => result.report.output_path = Some(output_path),
            // The repair itself succeeded; only handing it over did not. Reporting this as
            // `unrecoverable` blamed the file for what is really a refusal to overwrite, and
            // in a batch run that reads as "this recording is beyond help".
            Err(error) => result.report.error = Some(format!("repaired, but not written: {error}")),
        }
    }
    result.report
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::file_fixer::candidate::{Fidelity, Score};

    fn perfect_score() -> Score {
        Score {
            content_recovered: 10,
            content_total: Some(10),
            decoder_errors: 0,
            decodes_cleanly: true,
            streams_complete: true,
        }
    }

    #[test]
    fn capability_gating_blocks_strategies_that_need_more_than_the_mode_grants() {
        let safe = Mode::Safe.capabilities();
        let needs_truncate = Capabilities {
            truncate: true,
            ..Capabilities::NONE
        };

        assert!(!capabilities_satisfy(safe, needs_truncate));
        assert!(capabilities_satisfy(Mode::Balanced.capabilities(), needs_truncate));
        assert!(capabilities_satisfy(safe, Capabilities::NONE));
    }

    /// An unknown format is not damage, and reporting it as such would libel a perfectly
    /// healthy file in a format this tool simply does not implement.
    #[test]
    fn an_unrecognizable_file_reports_unsupported_rather_than_damage() {
        let data = vec![0x42u8; 128];
        let result = repair_data(Path::new("mystery.bin"), &data, &RepairOptions::default());

        assert_eq!(result.report.outcome, Outcome::Unsupported);
        assert!(result.winner.is_none());
        assert!(result.report.error.is_some_and(|e| e.contains("format")));
    }

    #[test]
    fn a_perfect_candidate_is_recognized_as_such() {
        assert!(perfect_score().is_perfect());
    }

    #[test]
    fn default_budget_is_bounded_in_both_dimensions() {
        let budget = Budget::default();
        assert!(budget.deadline.is_some());
        assert!(budget.max_candidates.is_some());
        assert!(!budget.exhaustive);
    }

    #[test]
    fn passthrough_detection_keys_off_the_strategy_name() {
        let result = RepairResult {
            report: RepairReport::failed(PathBuf::from("x"), Mode::Safe, String::new()),
            winner: Some(Candidate {
                strategy: "png_verbatim".to_string(),
                data: Payload::Bytes(Vec::new()),
                score: perfect_score(),
                fidelity: Fidelity::Lossless,
                reconstructed: Vec::new(),
            }),
            rejected: Vec::new(),
        };
        assert!(strategy_was_passthrough(&result));
    }

    /// A file that is damaged and that nothing improves on must not be copied under a new name.
    /// Measured on a PhotoRec collection, 214 MPEG carves land here: intact containers whose
    /// pictures have macroblock damage no container repair reaches. Writing a byte-identical
    /// 8 GB of copies to say "we could not help" is worse than saying it.
    #[test]
    fn a_damaged_file_no_repair_improves_on_is_reported_rather_than_copied() {
        let sample = std::path::Path::new("tmp/samples/clip.mpg");
        if !crate::file_fixer::formats::ffmpeg::is_available() || !sample.exists() {
            return;
        }
        let output_dir = std::env::temp_dir().join("file_fixer-passthrough-test");
        let report = repair_file(
            sample,
            &RepairOptions {
                mode: Mode::Aggressive,
                output_dir: Some(output_dir),
                force: true,
                ..RepairOptions::default()
            },
        );

        assert_eq!(report.outcome, Outcome::Healthy, "{:?}", report.error);
        assert!(report.output_path.is_none(), "a passthrough must not be written");
    }
}
