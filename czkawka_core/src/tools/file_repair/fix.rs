use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use crate::common::fs_ops::remove_single_file;
use crate::file_fixer::{Budget, Outcome, RepairOptions, RepairReport, repair_file};
use crate::tools::file_repair::{FixOutcome, FixParameters, ProfileStore};

/// Repairs `path` into a new file; the original is touched only when `replace_original` is set and the repair succeeded.
pub fn fix_file(path: &Path, params: &FixParameters, stop: &Arc<AtomicBool>) -> FixOutcome {
    if stop.load(Ordering::Relaxed) {
        return FixOutcome::Cancelled;
    }
    ProfileStore::ensure_loaded();
    let options = RepairOptions {
        mode: params.preset.into(),
        references: params.references.clone(),
        output_dir: params.output_dir.clone(),
        budget: Budget {
            deadline: Some(params.time_budget),
            ..Budget::default()
        },
        ..RepairOptions::default()
    };

    let report = repair_file(path, &options);
    if stop.load(Ordering::Relaxed) {
        discard_output(&report);
        return FixOutcome::Cancelled;
    }
    let Some(output) = report.output_path.clone() else {
        return FixOutcome::Failed(failure_reason(&report));
    };
    if let Some(error) = &report.error {
        return FixOutcome::Failed(error.clone());
    }

    let recovered_percent = report.recovered_percent();
    if !params.replace_original {
        return FixOutcome::Fixed {
            output,
            recovered_percent,
            replaced_original: false,
        };
    }
    match replace_original(path, &output, |original| remove_single_file(original, true)) {
        Ok(()) => FixOutcome::Fixed {
            output: path.to_path_buf(),
            recovered_percent,
            replaced_original: true,
        },
        Err(error) => FixOutcome::Failed(format!("repaired into {}, but the original was not replaced: {error}", output.display())),
    }
}

fn discard_output(report: &RepairReport) {
    if let Some(output) = &report.output_path
        && let Err(error) = std::fs::remove_file(output)
    {
        log::warn!("cannot remove the output of a cancelled repair {}: {error}", output.display());
    }
}

fn failure_reason(report: &RepairReport) -> String {
    if let Some(error) = &report.error {
        return error.clone();
    }
    match report.outcome {
        Outcome::Healthy => "the file is not damaged".to_string(),
        Outcome::Unsupported => "this variant of the format cannot be repaired".to_string(),
        Outcome::Unrecoverable | Outcome::FullyRepaired | Outcome::PartiallyRecovered => {
            report.notes.first().cloned().unwrap_or_else(|| "no repair improved on the file".to_string())
        }
    }
}

pub(super) fn replace_original(original: &Path, repaired: &Path, remove_original: impl FnOnce(&Path) -> Result<(), String>) -> Result<(), String> {
    remove_original(original)?;
    std::fs::rename(repaired, original).map_err(|error| format!("cannot move {} to {}: {error}", repaired.display(), original.display()))
}
