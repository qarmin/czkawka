use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use crossbeam_channel::Sender;

use crate::common::progress_data::ProgressData;
use crate::tools::broken_files::BrokenEntry;
use crate::tools::file_repair::{AssessParameters, ProfileStore, Repairability, assess_files, is_repair_supported};

/// Whether `entry` needs a (new) trial repair under the current settings and camera profiles.
pub(super) fn needs_assessment(entry: &BrokenEntry, params: &AssessParameters, known_profiles: u32) -> bool {
    if !entry.has_errors() || !is_repair_supported(&entry.path) {
        return false;
    }
    match entry.repairability {
        None => true,
        Some(Repairability::Repairable(_)) => false,
        Some(Repairability::TooLarge) => params.max_file_size == 0 || entry.size <= params.max_file_size,
        Some(Repairability::NotRepairable | Repairability::TimedOut) => entry.profiles_when_assessed != known_profiles,
    }
}

/// Runs trial repairs on broken entries that lack a current verdict. Returns `false` when stopped.
pub(super) fn assess_broken_entries(entries: &mut [BrokenEntry], params: &AssessParameters, stop_flag: &Arc<AtomicBool>, progress_sender: Option<&Sender<ProgressData>>) -> bool {
    let known_profiles = u32::try_from(ProfileStore::ensure_loaded()).unwrap_or(u32::MAX);
    let stale: Vec<&mut BrokenEntry> = entries.iter_mut().filter(|entry| needs_assessment(entry, params, known_profiles)).collect();
    let files: Vec<_> = stale.iter().map(|entry| (entry.path.clone(), entry.size)).collect();

    let Some(verdicts) = assess_files(&files, params, stop_flag, progress_sender) else {
        return false;
    };
    for (entry, verdict) in stale.into_iter().zip(verdicts) {
        entry.repairability = Some(verdict);
        entry.profiles_when_assessed = known_profiles;
    }
    true
}
