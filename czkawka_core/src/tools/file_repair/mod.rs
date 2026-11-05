//! Thin adapter between czkawka and the self-contained `file_fixer` module.

mod assess;
mod batch;
mod fix;
mod references;
#[cfg(test)]
mod tests;

use std::path::PathBuf;
use std::time::Duration;

pub use assess::{assess_file, cancel_scope, is_repair_supported};
pub use batch::{assess_files, fix_files};
pub use fix::fix_file;
pub use references::{ProfileStore, collect_references};
use serde::{Deserialize, Serialize};

use crate::file_fixer::Mode;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum RepairPreset {
    Safe,
    Balanced,
    Aggressive,
}

impl From<RepairPreset> for Mode {
    fn from(preset: RepairPreset) -> Self {
        match preset {
            RepairPreset::Safe => Self::Safe,
            RepairPreset::Balanced => Self::Balanced,
            RepairPreset::Aggressive => Self::Aggressive,
        }
    }
}

impl RepairPreset {
    pub const ALL: [Self; 3] = [Self::Safe, Self::Balanced, Self::Aggressive];
}

/// What a trial repair concluded about one file.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Repairability {
    /// The least permissive preset that still produces an improvement.
    Repairable(RepairPreset),
    NotRepairable,
    TimedOut,
    TooLarge,
}

#[derive(Debug, Clone, Copy)]
pub struct AssessParameters {
    /// Files above this size are not tried; 0 means no limit.
    pub max_file_size: u64,
    pub time_budget: Duration,
}

impl Default for AssessParameters {
    fn default() -> Self {
        Self {
            max_file_size: 4 * 1024 * 1024 * 1024,
            time_budget: Duration::from_secs(20),
        }
    }
}

#[derive(Debug, Clone)]
pub struct FixParameters {
    pub preset: RepairPreset,
    pub references: Vec<PathBuf>,
    /// `None` puts the repaired copy next to the original.
    pub output_dir: Option<PathBuf>,
    /// Moves the original to the trash and gives the repaired file its name.
    pub replace_original: bool,
    pub time_budget: Duration,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FixOutcome {
    Fixed {
        output: PathBuf,
        recovered_percent: Option<f64>,
        replaced_original: bool,
    },
    Failed(String),
    Cancelled,
}
