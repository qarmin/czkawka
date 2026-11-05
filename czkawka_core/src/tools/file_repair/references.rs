use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Once};

use crossbeam_channel::Sender;
use rayon::prelude::*;

use crate::common::config_cache_path::get_config_cache_path;
use crate::common::progress_data::{ProgressData, ToolStage};
use crate::common::progress_stop_handler::prepare_thread_handler_common;
use crate::file_fixer::format::{Format, detect_by_extension};
use crate::file_fixer::formats::mp4_profile::{self, harvest_file};
use crate::file_fixer::formats::mp4_profile_store;

/// Camera profiles remembered between runs, kept next to the other caches.
pub struct ProfileStore;

impl ProfileStore {
    pub fn path() -> Option<PathBuf> {
        get_config_cache_path().map(|paths| paths.cache_folder.join("video_profiles.json"))
    }

    /// Loads the remembered profiles into memory once per process; returns how many are known.
    pub fn ensure_loaded() -> usize {
        static LOADED: Once = Once::new();
        LOADED.call_once(|| {
            if let Some(path) = Self::path() {
                mp4_profile_store::load(&path);
            }
        });
        Self::count()
    }

    pub fn count() -> usize {
        mp4_profile::learned().len()
    }

    pub fn clear() -> Result<(), String> {
        let path = Self::path().ok_or_else(|| "cache folder is not available".to_string())?;
        match std::fs::remove_file(&path) {
            Ok(()) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => return Err(format!("cannot remove {}: {error}", path.display())),
        }
        mp4_profile::forget_all();
        Ok(())
    }
}

/// Learns decoder settings from the healthy videos under `directories` and saves them.
///
/// Returns the number of stored profiles, or `None` when stopped before anything was saved.
pub fn collect_references(directories: &[PathBuf], stop: &Arc<AtomicBool>, progress_sender: Option<&Sender<ProgressData>>) -> Result<Option<usize>, String> {
    let store_path = ProfileStore::path().ok_or_else(|| "cache folder is not available".to_string())?;
    ProfileStore::ensure_loaded();

    let files = list_video_files(directories, stop);
    let total_size = files.iter().map(|path| std::fs::metadata(path).map_or(0, |meta| meta.len())).sum();
    let progress = prepare_thread_handler_common(progress_sender, ToolStage::CollectingVideoReferences, files.len(), total_size);
    let harvested: Vec<_> = files
        .par_iter()
        .take_any_while(|_| !stop.load(Ordering::Relaxed))
        .flat_map_iter(|path| {
            let profiles = harvest_file(path);
            progress.increase_items(1);
            progress.increase_size(std::fs::metadata(path).map_or(0, |meta| meta.len()));
            profiles
        })
        .collect();
    progress.join_thread();
    if stop.load(Ordering::Relaxed) {
        return Ok(None);
    }

    mp4_profile::learn(harvested);
    mp4_profile_store::save(&store_path)
        .map(Some)
        .map_err(|error| format!("cannot save {}: {error}", store_path.display()))
}

fn list_video_files(directories: &[PathBuf], stop: &AtomicBool) -> Vec<PathBuf> {
    let mut pending: Vec<PathBuf> = directories.to_vec();
    let mut files = Vec::new();
    while let Some(directory) = pending.pop() {
        if stop.load(Ordering::Relaxed) {
            break;
        }
        let Ok(entries) = std::fs::read_dir(&directory) else {
            continue;
        };
        for entry in entries.flatten() {
            let Ok(file_type) = entry.file_type() else {
                continue;
            };
            let path = entry.path();
            if file_type.is_dir() {
                pending.push(path);
            } else if file_type.is_file() && is_profile_source(&path) {
                files.push(path);
            }
        }
    }
    files
}

fn is_profile_source(path: &Path) -> bool {
    detect_by_extension(path) == Some(Format::Mp4)
}
