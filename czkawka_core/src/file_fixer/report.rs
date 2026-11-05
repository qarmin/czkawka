//! What the tool tells the user about a repair.
//!
//! A candidate may contain reconstructed content, so a report that only said "repaired" would
//! be actively misleading: the user would get a file that looks complete and is partly
//! invented. Every report therefore names the ranges that are not faithful to the original.

use std::ops::Range;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::file_fixer::candidate::{Candidate, Fidelity, Score, SelectionReason};
use crate::file_fixer::format::{DetectionSource, Format};
use crate::file_fixer::mode::{Capabilities, Mode};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Outcome {
    /// The input decoded without complaint; nothing was written.
    Healthy,
    /// Every unit of content was recovered.
    FullyRepaired,
    /// Some content was recovered, some is gone.
    PartiallyRecovered,
    /// No candidate cleared the bar; nothing was written.
    Unrecoverable,
    /// The format was recognized but this variant is beyond what the tool can verify or
    /// repair. Saying "healthy" would be a guess and "unrecoverable" would imply damage.
    Unsupported,
}

impl Outcome {
    /// Exit code for the CLI, so a batch run can be scripted against.
    pub fn exit_code(self) -> i32 {
        match self {
            Self::Healthy | Self::FullyRepaired => 0,
            Self::PartiallyRecovered => 1,
            Self::Unrecoverable => 2,
            Self::Unsupported => 3,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Healthy => "healthy",
            Self::FullyRepaired => "repaired",
            Self::PartiallyRecovered => "partial",
            Self::Unrecoverable => "unrecoverable",
            Self::Unsupported => "unsupported",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CandidateReport {
    pub strategy: String,
    pub score: Score,
    pub fidelity: Fidelity,
    pub selected: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepairReport {
    pub path: PathBuf,
    pub format: Option<Format>,
    pub detected_by: Option<DetectionSource>,
    pub mode: Mode,
    pub outcome: Outcome,
    pub output_path: Option<PathBuf>,
    pub winning_strategy: Option<String>,
    /// Set when a byte-faithful candidate was chosen over one that recovered more content.
    pub lossless_preferred: bool,
    /// Content ranges of the output that were reconstructed rather than recovered. The unit
    /// is the format's own: image rows, video frames.
    pub reconstructed: Vec<Range<u64>>,
    pub content_recovered: Option<u64>,
    pub content_total: Option<u64>,
    pub candidates: Vec<CandidateReport>,
    /// True when the search stopped on the time or count budget, so a better candidate may exist.
    pub budget_exhausted: bool,
    /// Capabilities that would have admitted a repair the mode turned away. Non-empty means a
    /// result exists and a more permissive run would produce it.
    #[serde(default)]
    pub declined_capabilities: Vec<String>,
    /// Things about the result the user needs that no score expresses, such as a file that is
    /// genuinely damaged and that nothing the tool can produce improves on.
    #[serde(default)]
    pub notes: Vec<String>,
    pub elapsed_ms: u64,
    /// Populated when the file could not be processed at all.
    pub error: Option<String>,
}

impl RepairReport {
    pub fn failed(path: PathBuf, mode: Mode, error: String) -> Self {
        Self {
            path,
            format: None,
            detected_by: None,
            mode,
            outcome: Outcome::Unrecoverable,
            output_path: None,
            winning_strategy: None,
            lossless_preferred: false,
            reconstructed: Vec::new(),
            content_recovered: None,
            content_total: None,
            candidates: Vec::new(),
            budget_exhausted: false,
            declined_capabilities: Vec::new(),
            notes: Vec::new(),
            elapsed_ms: 0,
            error: Some(error),
        }
    }

    pub fn recovered_percent(&self) -> Option<f64> {
        match (self.content_recovered, self.content_total) {
            (Some(recovered), Some(total)) if total > 0 => Some(recovered as f64 / total as f64 * 100.0),
            _ => None,
        }
    }

    /// One line per file, the default CLI output.
    pub fn summary_line(&self) -> String {
        let format = self.format.map_or_else(|| "?".to_string(), |f| f.to_string());
        let path = self.path.display();

        if let Some(error) = &self.error {
            return format!("{path}: {format} {} ({error})", self.outcome.as_str());
        }

        let recovered = match self.recovered_percent() {
            Some(percent) => format!(" {percent:.0}%"),
            None => String::new(),
        };
        let strategy = self.winning_strategy.as_ref().map_or_else(String::new, |s| format!(" via {s}"));
        let reconstructed = if self.reconstructed.is_empty() {
            String::new()
        } else {
            format!(" [{} reconstructed range(s)]", self.reconstructed.len())
        };

        let notes = self.notes.iter().map(|note| format!(" - {note}")).collect::<String>();

        format!("{path}: {format} {}{recovered}{strategy}{reconstructed}{}{notes}", self.outcome.as_str(), self.retry_hint())
    }

    /// Advice when the mode, not the data, is what stopped the repair.
    ///
    /// An `unrecoverable` that really means "a result exists, you did not permit it" is the
    /// most misleading thing this tool can print: the user concludes the file is beyond help
    /// and deletes it.
    pub fn retry_hint(&self) -> String {
        // A file that is already whole always has strategies it was not allowed to run - every
        // format has at least one reconstruction strategy gated behind `guess`. Offering a
        // "retry" for it reads as "your healthy file is damaged", which is the exact libel the
        // healthy outcome exists to avoid.
        let nothing_to_gain = matches!(self.outcome, Outcome::Healthy | Outcome::FullyRepaired);
        if self.declined_capabilities.is_empty() || self.output_path.is_some() || nothing_to_gain {
            return String::new();
        }
        let wanted = Capabilities::from_names(&self.declined_capabilities);
        match Mode::smallest_granting(wanted) {
            Some(mode) if mode != self.mode => format!(" (a repair exists, retry with --mode {mode})"),
            _ => format!(" (a repair exists, retry with --allow {})", self.declined_capabilities.join(" --allow ")),
        }
    }
}

/// Builds the per-candidate section of a report.
pub fn candidate_reports(candidates: &[Candidate], winner: Option<usize>) -> Vec<CandidateReport> {
    candidates
        .iter()
        .enumerate()
        .map(|(index, candidate)| CandidateReport {
            strategy: candidate.strategy.clone(),
            score: candidate.score,
            fidelity: candidate.fidelity,
            selected: winner == Some(index),
        })
        .collect()
}

pub fn outcome_for(score: &Score) -> Outcome {
    if score.is_perfect() {
        Outcome::FullyRepaired
    } else if score.content_recovered > 0 {
        Outcome::PartiallyRecovered
    } else {
        Outcome::Unrecoverable
    }
}

pub fn selection_reason_is_lossless(reason: SelectionReason) -> bool {
    matches!(reason, SelectionReason::LosslessPreferred)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn score(recovered: u64, total: u64, errors: u32) -> Score {
        Score {
            content_recovered: recovered,
            content_total: Some(total),
            decoder_errors: errors,
            decodes_cleanly: errors == 0,
            streams_complete: true,
        }
    }

    fn declined(mode: Mode, capabilities: &[&str]) -> RepairReport {
        let mut report = RepairReport::failed(PathBuf::from("clip.mp4"), mode, String::new());
        report.error = None;
        report.format = Some(Format::Mp4);
        report.declined_capabilities = capabilities.iter().map(|name| (*name).to_string()).collect();
        report
    }

    /// The worst thing this tool can print is an `unrecoverable` that really means "a result
    /// exists and you did not permit it", because the user concludes the file is beyond help.
    #[test]
    fn a_repair_the_mode_forbade_is_advertised_not_hidden() {
        let report = declined(Mode::Balanced, &["guess"]);
        assert!(report.summary_line().contains("retry with --mode aggressive"), "{}", report.summary_line());
    }

    #[test]
    fn nothing_is_suggested_when_the_mode_already_grants_everything_asked_for() {
        assert_eq!(declined(Mode::Aggressive, &[]).retry_hint(), "");
    }

    /// Already holding the output means the repair happened; advice would be noise.
    #[test]
    fn no_advice_once_a_file_has_been_written() {
        let mut report = declined(Mode::Balanced, &["guess"]);
        report.output_path = Some(PathBuf::from("clip.fixedffile.mp4"));
        assert_eq!(report.retry_hint(), "");
    }

    /// Every format keeps a reconstruction strategy behind `guess`, so a perfectly healthy
    /// file always has something the mode turned away. Advertising that as "a repair exists"
    /// tells the user their intact file is damaged.
    #[test]
    fn no_advice_for_a_file_that_was_already_whole() {
        let mut report = declined(Mode::Balanced, &["guess"]);
        report.outcome = Outcome::Healthy;
        assert_eq!(report.retry_hint(), "");

        report.outcome = Outcome::FullyRepaired;
        assert_eq!(report.retry_hint(), "");

        report.outcome = Outcome::PartiallyRecovered;
        assert!(!report.retry_hint().is_empty(), "a partial result can still be improved");
    }

    /// A capability no preset bundles on its own has to be named directly.
    #[test]
    fn a_capability_without_a_matching_preset_is_offered_as_a_flag() {
        let report = declined(Mode::Aggressive, &["guess"]);
        assert!(report.retry_hint().contains("--allow guess"), "{}", report.retry_hint());
    }

    #[test]
    fn exit_codes_distinguish_the_three_outcomes() {
        assert_eq!(Outcome::Healthy.exit_code(), 0);
        assert_eq!(Outcome::FullyRepaired.exit_code(), 0);
        assert_eq!(Outcome::PartiallyRecovered.exit_code(), 1);
        assert_eq!(Outcome::Unrecoverable.exit_code(), 2);
        assert_eq!(Outcome::Unsupported.exit_code(), 3);
    }

    #[test]
    fn outcome_follows_how_much_was_recovered() {
        assert_eq!(outcome_for(&score(100, 100, 0)), Outcome::FullyRepaired);
        assert_eq!(outcome_for(&score(60, 100, 0)), Outcome::PartiallyRecovered);
        assert_eq!(outcome_for(&score(100, 100, 3)), Outcome::PartiallyRecovered);
        assert_eq!(outcome_for(&score(0, 100, 9)), Outcome::Unrecoverable);
    }

    #[test]
    fn a_failed_report_keeps_the_reason() {
        let report = RepairReport::failed("/tmp/x.png".into(), Mode::Balanced, "empty file".to_string());

        assert_eq!(report.outcome, Outcome::Unrecoverable);
        assert!(report.summary_line().contains("empty file"));
        assert!(report.output_path.is_none());
    }

    #[test]
    fn the_summary_mentions_reconstructed_ranges() {
        let mut report = RepairReport::failed("/tmp/x.png".into(), Mode::Aggressive, String::new());
        report.error = None;
        report.format = Some(Format::Png);
        report.outcome = Outcome::PartiallyRecovered;
        report.content_recovered = Some(70);
        report.content_total = Some(100);
        report.winning_strategy = Some("png_resync".to_string());
        report.reconstructed = std::iter::once(70..100).collect();

        let line = report.summary_line();
        assert!(line.contains("70%"), "{line}");
        assert!(line.contains("png_resync"), "{line}");
        assert!(line.contains("reconstructed"), "{line}");
    }

    #[test]
    fn percentage_is_absent_when_the_total_is_unknown() {
        let mut report = RepairReport::failed("/tmp/x.mp4".into(), Mode::Balanced, String::new());
        report.error = None;
        report.content_recovered = Some(10);
        report.content_total = None;

        assert!(report.recovered_percent().is_none());
    }
}
