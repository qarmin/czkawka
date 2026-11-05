//! Candidate repairs and the rule that picks a winner among them.

use std::cmp::Reverse;
use std::io;
use std::ops::Range;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

/// What a strategy managed to salvage, measured in whatever unit the format counts in
/// (image rows, video frames). Comparison is lexicographic with *amount of content first*:
/// the tool exists to extract as much as the data allows, so a candidate that recovers more
/// beats a tidier one that recovers less. Cleanliness only breaks ties.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Score {
    pub content_recovered: u64,
    pub content_total: Option<u64>,
    pub decoder_errors: u32,
    pub decodes_cleanly: bool,
    pub streams_complete: bool,
}

impl Score {
    pub fn recovered_fraction(&self) -> Option<f64> {
        match self.content_total {
            Some(0) | None => None,
            Some(total) => Some(self.content_recovered as f64 / total as f64),
        }
    }

    /// True when nothing was lost: every unit of content is present and decoded without complaint.
    pub fn is_perfect(&self) -> bool {
        self.decodes_cleanly && self.decoder_errors == 0 && self.streams_complete && self.content_total.is_some_and(|total| self.content_recovered >= total)
    }

    /// Ordering key. `decoder_errors` is negated so that "more is worse" becomes "bigger is better"
    /// like every other component, letting the tuple be compared directly.
    ///
    /// Perfection is the last component, so it decides nothing except an otherwise exact tie -
    /// two candidates holding the same content, equally cleanly, one of which can prove that is
    /// all the content there was. Measured on a Motion JPEG recording with its header destroyed:
    /// a tolerant remux and a rebuilt container both returned all 239 frames, the remux could
    /// not say how many there should have been, and preferring it reported a complete recovery
    /// as `partial` and handed over the file whose frame count was wrong.
    fn rank(&self) -> (u64, i64, bool, bool, bool) {
        (
            self.content_recovered,
            -i64::from(self.decoder_errors),
            self.decodes_cleanly,
            self.streams_complete,
            self.is_perfect(),
        )
    }
}

/// How a candidate's content relates to the original bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Fidelity {
    /// Every byte of content came from the input unchanged: a subset, never an invention.
    Lossless,
    /// Content was re-encoded or synthesized; `reconstructed` says which parts to distrust.
    Reconstructed,
}

/// Where a candidate's bytes live.
///
/// Images are small enough to hold in memory, but a repaired video is produced by an external
/// tool writing to disk and can be larger than RAM. Reading such a result back into a `Vec`
/// would undo the whole reason the input is memory-mapped rather than read.
#[derive(Debug)]
pub enum Payload {
    Bytes(Vec<u8>),
    /// Deleted when dropped, so candidates that lose cost nothing.
    TempFile(tempfile::TempPath),
    /// The repair is the input itself. Kept as a reference so that confirming a healthy
    /// 50 GB recording costs no memory and no copy unless the result is actually written.
    SameAsInput(PathBuf),
}

impl Payload {
    pub fn len(&self) -> u64 {
        match self {
            Self::Bytes(bytes) => bytes.len() as u64,
            Self::TempFile(path) => std::fs::metadata(path).map_or(0, |m| m.len()),
            Self::SameAsInput(path) => std::fs::metadata(path).map_or(0, |m| m.len()),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// In-memory contents, or `None` when the payload lives on disk.
    pub fn as_bytes(&self) -> Option<&[u8]> {
        match self {
            Self::Bytes(bytes) => Some(bytes),
            Self::TempFile(_) | Self::SameAsInput(_) => None,
        }
    }

    pub fn write_to(&self, destination: &Path) -> io::Result<()> {
        match self {
            Self::Bytes(bytes) => std::fs::write(destination, bytes),
            // Rename first: within one filesystem it is atomic and avoids copying gigabytes.
            Self::TempFile(source) => match std::fs::rename(source, destination) {
                Ok(()) => Ok(()),
                Err(_) => std::fs::copy(source, destination).map(|_| ()),
            },
            // Never renamed: this path is the user's original file.
            Self::SameAsInput(source) => std::fs::copy(source, destination).map(|_| ()),
        }
    }
}

#[derive(Debug)]
pub struct Candidate {
    /// Identifies the strategy that produced this, and appears in the report.
    pub strategy: String,
    pub data: Payload,
    pub score: Score,
    pub fidelity: Fidelity,
    /// Ranges of the *output* that are not faithful to the original, in the format's content
    /// unit (rows, frames). Empty for a lossless candidate.
    pub reconstructed: Vec<Range<u64>>,
}

impl Candidate {
    pub fn is_lossless(&self) -> bool {
        matches!(self.fidelity, Fidelity::Lossless)
    }
}

/// Fraction of the leading candidate's content that a lossless candidate must reach to be
/// preferred over it.
pub const DEFAULT_LOSSLESS_THRESHOLD: f64 = 0.9;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectionReason {
    /// Won on the ranking outright.
    HighestScore,
    /// Lost on raw content but was preferred for being byte-faithful, per the threshold rule.
    LosslessPreferred,
}

#[derive(Debug, Clone, Copy)]
pub struct Selection {
    pub index: usize,
    pub reason: SelectionReason,
}

/// Picks the winning candidate.
///
/// This is deliberately not an [`Ord`] implementation. The lossless preference compares each
/// candidate against the *current leader* rather than against its neighbour, which is not a
/// transitive relation and would make a sort produce order-dependent nonsense.
///
/// Candidates arrive in the order their strategies were planned, cheapest and most faithful
/// first, so ties are resolved towards the earlier candidate.
pub fn select_best(candidates: &[Candidate], lossless_threshold: f64) -> Option<Selection> {
    let leader = candidates.iter().enumerate().max_by_key(|(i, c)| (c.score.rank(), Reverse(*i))).map(|(i, _)| i)?;

    let leader_content = candidates.get(leader).map_or(0, |c| c.score.content_recovered);
    if candidates.get(leader).is_some_and(Candidate::is_lossless) {
        return Some(Selection {
            index: leader,
            reason: SelectionReason::HighestScore,
        });
    }

    let required = leader_content as f64 * lossless_threshold;
    let best_lossless = candidates
        .iter()
        .enumerate()
        .filter(|(_, c)| c.is_lossless() && c.score.content_recovered as f64 >= required)
        .max_by_key(|(i, c)| (c.score.rank(), Reverse(*i)))
        .map(|(i, _)| i);

    Some(match best_lossless {
        Some(index) => Selection {
            index,
            reason: SelectionReason::LosslessPreferred,
        },
        None => Selection {
            index: leader,
            reason: SelectionReason::HighestScore,
        },
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn candidate(strategy: &str, recovered: u64, errors: u32, fidelity: Fidelity) -> Candidate {
        Candidate {
            strategy: strategy.to_string(),
            data: Payload::Bytes(Vec::new()),
            score: Score {
                content_recovered: recovered,
                content_total: Some(100),
                decoder_errors: errors,
                decodes_cleanly: errors == 0,
                streams_complete: true,
            },
            fidelity,
            reconstructed: Vec::new(),
        }
    }

    #[test]
    fn a_payload_reports_its_size_whichever_form_it_takes() {
        assert_eq!(Payload::Bytes(vec![0; 7]).len(), 7);
        assert!(Payload::Bytes(Vec::new()).is_empty());
        assert_eq!(Payload::Bytes(vec![1, 2]).as_bytes(), Some(&[1u8, 2][..]));
    }

    #[test]
    fn more_content_wins_over_fewer_errors() {
        let candidates = vec![
            candidate("clean_but_short", 60, 0, Fidelity::Reconstructed),
            candidate("long_but_messy", 95, 12, Fidelity::Reconstructed),
        ];
        let selection = select_best(&candidates, DEFAULT_LOSSLESS_THRESHOLD).unwrap();

        assert_eq!(selection.index, 1);
        assert_eq!(selection.reason, SelectionReason::HighestScore);
    }

    #[test]
    fn errors_break_ties_at_equal_content() {
        let candidates = vec![candidate("messy", 80, 5, Fidelity::Reconstructed), candidate("clean", 80, 1, Fidelity::Reconstructed)];
        assert_eq!(select_best(&candidates, DEFAULT_LOSSLESS_THRESHOLD).unwrap().index, 1);
    }

    #[test]
    fn a_lossless_candidate_close_behind_is_preferred() {
        let candidates = vec![
            candidate("reconstructed", 100, 3, Fidelity::Reconstructed),
            candidate("lossless_crop", 92, 0, Fidelity::Lossless),
        ];
        let selection = select_best(&candidates, DEFAULT_LOSSLESS_THRESHOLD).unwrap();

        assert_eq!(selection.index, 1);
        assert_eq!(selection.reason, SelectionReason::LosslessPreferred);
    }

    #[test]
    fn a_lossless_candidate_far_behind_does_not_win() {
        let candidates = vec![
            candidate("reconstructed", 100, 3, Fidelity::Reconstructed),
            candidate("lossless_crop", 40, 0, Fidelity::Lossless),
        ];
        let selection = select_best(&candidates, DEFAULT_LOSSLESS_THRESHOLD).unwrap();

        assert_eq!(selection.index, 0);
        assert_eq!(selection.reason, SelectionReason::HighestScore);
    }

    #[test]
    fn a_leading_lossless_candidate_reports_a_plain_win() {
        let candidates = vec![
            candidate("lossless_crop", 90, 0, Fidelity::Lossless),
            candidate("reconstructed", 70, 4, Fidelity::Reconstructed),
        ];
        let selection = select_best(&candidates, DEFAULT_LOSSLESS_THRESHOLD).unwrap();

        assert_eq!(selection.index, 0);
        assert_eq!(selection.reason, SelectionReason::HighestScore);
    }

    #[test]
    fn threshold_of_one_demands_an_exact_content_match() {
        let candidates = vec![
            candidate("reconstructed", 100, 3, Fidelity::Reconstructed),
            candidate("lossless_crop", 99, 0, Fidelity::Lossless),
        ];
        assert_eq!(select_best(&candidates, 1.0).unwrap().index, 0);
    }

    /// Strategies are planned cheapest-first, so an exact tie must not silently promote a
    /// later, more invasive strategy.
    #[test]
    fn a_tie_goes_to_the_earlier_candidate() {
        let candidates = vec![candidate("cheap", 100, 0, Fidelity::Lossless), candidate("expensive", 100, 0, Fidelity::Lossless)];
        assert_eq!(select_best(&candidates, DEFAULT_LOSSLESS_THRESHOLD).unwrap().index, 0);
    }

    #[test]
    fn a_tie_among_lossless_fallbacks_also_favours_the_earlier_one() {
        let candidates = vec![
            candidate("reconstructed", 100, 3, Fidelity::Reconstructed),
            candidate("lossless_a", 95, 0, Fidelity::Lossless),
            candidate("lossless_b", 95, 0, Fidelity::Lossless),
        ];
        let selection = select_best(&candidates, DEFAULT_LOSSLESS_THRESHOLD).unwrap();
        assert_eq!(selection.index, 1);
        assert_eq!(selection.reason, SelectionReason::LosslessPreferred);
    }

    /// An exact tie in content and cleanliness goes to the candidate that can prove it recovered
    /// everything, rather than to the one that merely happens to have been planned first.
    #[test]
    fn a_candidate_that_proves_completeness_wins_an_otherwise_exact_tie() {
        let mut unknown_total = candidate("remux", 239, 0, Fidelity::Lossless);
        unknown_total.score.content_total = None;
        let mut complete = candidate("rebuild", 239, 0, Fidelity::Lossless);
        complete.score.content_total = Some(239);

        let candidates = vec![unknown_total, complete];
        let selection = select_best(&candidates, DEFAULT_LOSSLESS_THRESHOLD).unwrap();
        assert_eq!(selection.index, 1);
    }

    #[test]
    fn no_candidates_means_no_selection() {
        assert!(select_best(&[], DEFAULT_LOSSLESS_THRESHOLD).is_none());
    }

    #[test]
    fn perfection_requires_every_unit_and_no_complaints() {
        let mut score = Score {
            content_recovered: 100,
            content_total: Some(100),
            decoder_errors: 0,
            decodes_cleanly: true,
            streams_complete: true,
        };
        assert!(score.is_perfect());

        score.decoder_errors = 1;
        assert!(!score.is_perfect());

        score.decoder_errors = 0;
        score.content_recovered = 99;
        assert!(!score.is_perfect());

        score.content_recovered = 100;
        score.content_total = None;
        assert!(!score.is_perfect());
    }

    #[test]
    fn recovered_fraction_guards_against_a_zero_total() {
        let score = Score {
            content_recovered: 0,
            content_total: Some(0),
            decoder_errors: 0,
            decodes_cleanly: true,
            streams_complete: true,
        };
        assert!(score.recovered_fraction().is_none());
    }
}
