use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Instant;

use crate::file_fixer::cancel::CancelScope;
use crate::file_fixer::format::detect_by_extension;
use crate::file_fixer::io::SourceFile;
use crate::file_fixer::output::strategy_was_passthrough;
use crate::file_fixer::{Budget, RepairOptions, repair_data};
use crate::tools::file_repair::{AssessParameters, ProfileStore, RepairPreset, Repairability};

pub fn is_repair_supported(path: &Path) -> bool {
    detect_by_extension(path).is_some()
}

/// Makes `stop` abort running repairs, including their child processes, until the guard is dropped.
pub fn cancel_scope(stop: &Arc<AtomicBool>) -> impl Drop + use<> {
    CancelScope::install(Arc::clone(stop))
}

/// Tries the presets from the safest up and reports the first that improves on the file.
///
/// Returns `None` when `stop` was set, because a result computed against killed child processes is meaningless.
pub fn assess_file(path: &Path, size: u64, params: &AssessParameters, stop: &Arc<AtomicBool>) -> Option<Repairability> {
    if !is_repair_supported(path) {
        return Some(Repairability::NotRepairable);
    }
    if params.max_file_size > 0 && size > params.max_file_size {
        return Some(Repairability::TooLarge);
    }

    ProfileStore::ensure_loaded();
    let source = match SourceFile::open(path) {
        Ok(source) => source,
        Err(error) => {
            log::debug!("cannot assess {}: {error}", path.display());
            return Some(Repairability::NotRepairable);
        }
    };

    let deadline = Instant::now() + params.time_budget;
    let mut timed_out = false;
    for preset in RepairPreset::ALL {
        let remaining = deadline.saturating_duration_since(Instant::now());
        if remaining.is_zero() {
            timed_out = true;
            break;
        }
        let options = RepairOptions {
            mode: preset.into(),
            budget: Budget {
                deadline: Some(remaining),
                ..Budget::default()
            },
            ..RepairOptions::default()
        };
        let result = repair_data(path, source.data(), &options);
        if stop.load(Ordering::Relaxed) {
            return None;
        }
        if result.winner.is_some() && !strategy_was_passthrough(&result) {
            return Some(Repairability::Repairable(preset));
        }
        timed_out |= result.report.budget_exhausted;
    }
    Some(if timed_out { Repairability::TimedOut } else { Repairability::NotRepairable })
}
