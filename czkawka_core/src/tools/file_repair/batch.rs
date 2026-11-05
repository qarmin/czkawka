use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use crossbeam_channel::Sender;
use rayon::ThreadPool;
use rayon::prelude::*;

use crate::common::progress_data::{ProgressData, ToolStage};
use crate::common::progress_stop_handler::prepare_thread_handler_common;
use crate::tools::file_repair::{AssessParameters, FixOutcome, FixParameters, Repairability, assess_file, cancel_scope, fix_file};

/// Repairs hold whole files in memory and spawn ffmpeg, so only a couple may run at once.
const MAX_PARALLEL_REPAIRS: usize = 2;

fn repair_pool() -> Result<ThreadPool, String> {
    rayon::ThreadPoolBuilder::new()
        .num_threads(MAX_PARALLEL_REPAIRS)
        .build()
        .map_err(|error| format!("cannot start repair threads: {error}"))
}

/// Fixes every file, in input order. Files not reached before `stop` was set come back as `Cancelled`.
pub fn fix_files(files: &[(PathBuf, u64)], params: &FixParameters, stop: &Arc<AtomicBool>, progress_sender: Option<&Sender<ProgressData>>) -> Vec<(PathBuf, FixOutcome)> {
    let _cancel = cancel_scope(stop);
    let progress = prepare_thread_handler_common(progress_sender, ToolStage::RepairingFiles, files.len(), files.iter().map(|(_, size)| size).sum());
    let fix_all = |pool: &ThreadPool| {
        pool.install(|| {
            files
                .par_iter()
                .map(|(path, size)| {
                    let outcome = fix_file(path, params, stop);
                    progress.increase_items(1);
                    progress.increase_size(*size);
                    (path.clone(), outcome)
                })
                .collect()
        })
    };
    let results = match repair_pool() {
        Ok(pool) => fix_all(&pool),
        Err(error) => files.iter().map(|(path, _)| (path.clone(), FixOutcome::Failed(error.clone()))).collect(),
    };
    progress.join_thread();
    results
}

/// Assesses every file, in input order; `None` when `stop` interrupted the run.
pub fn assess_files(files: &[(PathBuf, u64)], params: &AssessParameters, stop: &Arc<AtomicBool>, progress_sender: Option<&Sender<ProgressData>>) -> Option<Vec<Repairability>> {
    let _cancel = cancel_scope(stop);
    let progress = prepare_thread_handler_common(
        progress_sender,
        ToolStage::BrokenFilesAssessingRepair,
        files.len(),
        files.iter().map(|(_, size)| size).sum(),
    );
    let pool = repair_pool().ok()?;
    let results: Option<Vec<Repairability>> = pool.install(|| {
        files
            .par_iter()
            .map(|(path, size)| {
                if stop.load(Ordering::Relaxed) {
                    return None;
                }
                let result = assess_file(path, *size, params, stop);
                progress.increase_items(1);
                progress.increase_size(*size);
                result
            })
            .collect()
    });
    progress.join_thread();
    results
}
