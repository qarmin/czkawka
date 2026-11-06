use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use crossbeam_channel::Sender;
use rayon::prelude::*;

use crate::common::model::HashType;
use crate::common::progress_data::{CacheLoadPhase, DuplicateStage, ProgressData, ToolStage};
use crate::common::progress_stop_handler::{check_if_stop_received, prepare_thread_handler_common};
use crate::helpers::messages::Messages;
use crate::tools::duplicate::{DuplicateEntry, PREHASHING_BUFFER_SIZE, THREAD_BUFFER, hash_calculation, hash_calculation_limit};
use crate::tools::folder_compare::cache::{CacheSettings, HashCache};

pub(super) type ContentHash = [u8; 32];

#[derive(Default)]
pub(super) struct HashingOutcome {
    /// Only files sharing size and content with at least one other candidate; the rest are unique.
    pub shared_hashes: HashMap<PathBuf, ContentHash>,
    pub warnings: Vec<String>,
    pub messages: Messages,
    pub stopped: bool,
}

pub(super) fn hash_text(text: &str) -> ContentHash {
    *blake3::hash(text.as_bytes()).as_bytes()
}

fn empty_file_hash() -> ContentHash {
    hash_text("")
}

pub(super) fn hash_matching_files(
    files: Vec<(PathBuf, u64)>,
    hash_type: HashType,
    cache_settings: CacheSettings,
    stop_flag: &Arc<AtomicBool>,
    progress_sender: Option<&Sender<ProgressData>>,
) -> HashingOutcome {
    let mut outcome = HashingOutcome::default();
    let (empty_files, non_empty_files): (Vec<_>, Vec<_>) = group_shared(files, |(_, size)| *size).into_iter().flatten().partition(|(_, size)| *size == 0);
    outcome.shared_hashes.extend(empty_files.into_iter().map(|(path, _)| (path, empty_file_hash())));

    let Some(prehashed) = hash_files(non_empty_files, hash_type, Some(PREHASHING_BUFFER_SIZE), stop_flag, progress_sender, &mut outcome.warnings) else {
        outcome.stopped = true;
        return outcome;
    };
    let (small, big): (Vec<_>, Vec<_>) = group_shared(prehashed, |(_, size, prehash)| (*size, prehash.clone()))
        .into_iter()
        .flatten()
        .partition(|(_, size, _)| *size <= 2 * PREHASHING_BUFFER_SIZE);
    outcome.shared_hashes.extend(small.into_iter().map(|(path, _, prehash)| (path, hash_text(&prehash))));

    let big = big.into_iter().map(|(path, size, _)| (path, size)).collect();
    let Some(full_hashed) = full_hash_with_cache(big, hash_type, cache_settings, stop_flag, progress_sender, &mut outcome) else {
        outcome.stopped = true;
        return outcome;
    };
    let shared = group_shared(full_hashed, |(_, size, hash)| (*size, hash.clone()));
    outcome.shared_hashes.extend(shared.into_iter().flatten().map(|(path, _, hash)| (path, hash_text(&hash))));
    outcome
}

/// `None` when the search was stopped.
fn full_hash_with_cache(
    files: Vec<(PathBuf, u64)>,
    hash_type: HashType,
    cache_settings: CacheSettings,
    stop_flag: &Arc<AtomicBool>,
    progress_sender: Option<&Sender<ProgressData>>,
    outcome: &mut HashingOutcome,
) -> Option<Vec<(PathBuf, u64, String)>> {
    let cache = if cache_settings.enabled && !files.is_empty() {
        let progress_handler = prepare_thread_handler_common(progress_sender, ToolStage::Duplicate(DuplicateStage::LoadingHashCache(CacheLoadPhase::Loading)), 0, 0);
        let cache = HashCache::load(cache_settings, hash_type, &files, &mut outcome.messages);
        progress_handler.join_thread();
        cache
    } else {
        HashCache::load(CacheSettings { enabled: false, ..cache_settings }, hash_type, &files, &mut outcome.messages)
    };

    let (cached, uncached): (Vec<_>, Vec<_>) = files.into_iter().partition(|(path, _)| cache.cached_hash(path).is_some());
    let mut hashed: Vec<(PathBuf, u64, String)> = cached
        .into_iter()
        .filter_map(|(path, size)| cache.cached_hash(&path).map(|hash| (hash.to_string(), path, size)))
        .map(|(hash, path, size)| (path, size, hash))
        .collect();

    let computed = hash_files(uncached, hash_type, None, stop_flag, progress_sender, &mut outcome.warnings)?;
    if cache_settings.enabled && !computed.is_empty() {
        let progress_handler = prepare_thread_handler_common(progress_sender, ToolStage::Duplicate(DuplicateStage::SavingHashCache), 0, 0);
        cache.save(&computed, &mut outcome.messages);
        progress_handler.join_thread();
    }

    hashed.extend(computed);
    Some(hashed)
}

fn group_shared<T, K: std::hash::Hash + Eq>(items: Vec<T>, key: impl Fn(&T) -> K) -> Vec<Vec<T>> {
    let mut groups: HashMap<K, Vec<T>> = HashMap::new();
    for item in items {
        groups.entry(key(&item)).or_default().push(item);
    }
    groups.into_values().filter(|group| group.len() > 1).collect()
}

/// Hashes the head and tail (`limit`) or the whole file; `None` when the search was stopped.
fn hash_files(
    files: Vec<(PathBuf, u64)>,
    hash_type: HashType,
    limit: Option<u64>,
    stop_flag: &Arc<AtomicBool>,
    progress_sender: Option<&Sender<ProgressData>>,
    warnings: &mut Vec<String>,
) -> Option<Vec<(PathBuf, u64, String)>> {
    if files.is_empty() {
        return Some(Vec::new());
    }
    let stage = if limit.is_some() { DuplicateStage::PreHashing } else { DuplicateStage::FullHashing };
    let bytes_to_check = files.iter().map(|(_, size)| limit.map_or(*size, |limit| (*size).min(2 * limit))).sum();
    let progress_handler = prepare_thread_handler_common(progress_sender, ToolStage::Duplicate(stage), files.len(), bytes_to_check);

    let results: Vec<Option<Result<(PathBuf, u64, String), String>>> = files
        .into_par_iter()
        .map(|(path, size)| {
            if check_if_stop_received(stop_flag) {
                return None;
            }
            let entry = DuplicateEntry {
                path,
                size,
                ..DuplicateEntry::default()
            };
            let hash = THREAD_BUFFER.with_borrow_mut(|buffer| match limit {
                Some(limit) => hash_calculation_limit(buffer, &entry, hash_type, limit, progress_handler.size_counter()).map(Some),
                None => hash_calculation(buffer, &entry, hash_type, progress_handler.size_counter(), stop_flag),
            });
            progress_handler.increase_items(1);
            match hash {
                Ok(Some(hash)) => Some(Ok((entry.path, size, hash))),
                Ok(None) => None,
                Err(warning) => Some(Err(warning)),
            }
        })
        .collect();
    progress_handler.join_thread();

    if check_if_stop_received(stop_flag) {
        return None;
    }
    let mut hashed = Vec::with_capacity(results.len());
    for result in results.into_iter().flatten() {
        match result {
            Ok(file) => hashed.push(file),
            Err(warning) => warnings.push(warning),
        }
    }
    Some(hashed)
}
