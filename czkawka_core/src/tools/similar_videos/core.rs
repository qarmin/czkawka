use std::collections::{BTreeMap, BTreeSet};
use std::mem;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use crossbeam_channel::Sender;
use fun_time::fun_time;
use indexmap::IndexMap;
use log::debug;
use rayon::prelude::*;
use vid_dup_finder_lib::{CreationOptions, Cropdetect, VideoHash, VideoHashBuilder};

use crate::common::cache::{CACHE_VIDEO_VERSION, load_and_split_cache_generalized_by_path, save_and_connect_cache_generalized_by_path};
use crate::common::config_cache_path::get_config_cache_path;
use crate::common::dir_traversal::{DirTraversalBuilder, DirTraversalResult, inode, take_1_per_inode};
use crate::common::model::{ToolType, WorkContinueStatus};
use crate::common::progress_data::{CurrentStage, ProgressData};
use crate::common::progress_stop_handler::{check_if_stop_received, prepare_thread_handler_common};
use crate::common::tool_data::{CommonData, CommonToolData};
use crate::common::traits::ResultEntry;
use crate::common::video_utils::{VIDEO_THUMBNAILS_SUBFOLDER, VideoMetadata, generate_thumbnail};
use crate::tools::similar_videos::{
    AudioCacheEntry, AudioSearchPreset, PerceptualCacheEntry, PerceptualSearchPreset, SimilarVideos, SimilarVideosEngine, SimilarVideosParameters, VideosEntry,
    get_audio_cache_file, get_perceptual_cache_file,
};

impl SimilarVideos {
    pub fn new(params: SimilarVideosParameters) -> Self {
        Self {
            common_data: CommonToolData::new(ToolType::SimilarVideos),
            information: Default::default(),
            similar_vectors: Vec::new(),
            videos_hashes: Default::default(),
            videos_to_check: Default::default(),
            similar_referenced_vectors: Vec::new(),
            params,
        }
    }

    #[fun_time(message = "check_for_similar_videos", level = "debug")]
    pub(crate) fn check_for_similar_videos(&mut self, stop_flag: &Arc<AtomicBool>, progress_sender: Option<&Sender<ProgressData>>) -> WorkContinueStatus {
        let result = DirTraversalBuilder::new()
            .group_by(inode)
            .stop_flag(stop_flag)
            .progress_sender(progress_sender)
            .common_data(&self.common_data)
            .build()
            .run();

        match result {
            DirTraversalResult::SuccessFiles { grouped_file_entries, warnings } => {
                self.common_data.text_messages.warnings.extend(warnings);

                let progress_handler = prepare_thread_handler_common(
                    progress_sender,
                    CurrentStage::SimilarVideosHidingHardLinks,
                    grouped_file_entries.len(),
                    self.get_test_type(),
                    0,
                );
                let hide_hard_links = self.get_hide_hard_links();
                self.videos_to_check = grouped_file_entries
                    .into_par_iter()
                    .map(|(inode, fes)| {
                        if check_if_stop_received(stop_flag) {
                            return None;
                        }
                        progress_handler.increase_items(1);
                        Some((inode, fes))
                    })
                    .while_some()
                    .flat_map(if hide_hard_links { |(_, fes)| fes } else { take_1_per_inode })
                    .map(|fe| (fe.path.to_string_lossy().to_string(), fe.into_videos_entry()))
                    .collect();

                progress_handler.join_thread();

                if check_if_stop_received(stop_flag) {
                    return WorkContinueStatus::Stop;
                }

                debug!("check_files - Found {} video files.", self.videos_to_check.len());
                WorkContinueStatus::Continue
            }

            DirTraversalResult::Stopped => WorkContinueStatus::Stop,
        }
    }

    fn check_video_file_entry(&self, mut file_entry: VideosEntry) -> VideosEntry {
        let creation_options = CreationOptions {
            skip_forward_amount: self.params.skip_forward_amount as f64,
            duration: self.params.duration as f64,
            cropdetect: self.params.crop_detect,
        };
        let vhash = match VideoHashBuilder::from_options(creation_options).hash(file_entry.path.clone()) {
            Ok(t) => t,
            Err(e) => {
                let path = file_entry.path.to_string_lossy();
                file_entry.error = format!("Failed to hash file \"{path}\": reason {e}");
                return file_entry;
            }
        };

        file_entry.vhash = vhash;

        file_entry
    }

    fn read_video_properties(mut file_entry: VideosEntry) -> VideosEntry {
        match VideoMetadata::from_path(&file_entry.path) {
            Ok(metadata) => {
                file_entry.fps = metadata.fps;
                file_entry.codec = metadata.codec;
                file_entry.bitrate = metadata.bitrate;
                file_entry.width = metadata.width;
                file_entry.height = metadata.height;
                file_entry.duration = metadata.duration;
            }
            Err(e) => {
                let path = file_entry.path.to_string_lossy();
                file_entry.error = format!("Failed to read properties for file \"{path}\": reason {e}");
            }
        }

        file_entry
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Dispatcher
    // ─────────────────────────────────────────────────────────────────────────

    #[fun_time(message = "sort_videos", level = "debug")]
    pub(crate) fn sort_videos(&mut self, stop_flag: &Arc<AtomicBool>, progress_sender: Option<&Sender<ProgressData>>) -> WorkContinueStatus {
        if self.videos_to_check.is_empty() {
            return WorkContinueStatus::Continue;
        }

        match self.params.engine.clone() {
            SimilarVideosEngine::VidDupFinder => self.sort_videos_vid_dup_finder(stop_flag, progress_sender),
            SimilarVideosEngine::Perceptual(preset) => self.sort_videos_perceptual(preset, stop_flag, progress_sender),
            SimilarVideosEngine::Audio(preset) => self.sort_videos_audio(preset, stop_flag, progress_sender),
        }
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Original vid_dup_finder engine (renamed from sort_videos)
    // ─────────────────────────────────────────────────────────────────────────

    #[fun_time(message = "sort_videos_vid_dup_finder", level = "debug")]
    fn sort_videos_vid_dup_finder(&mut self, stop_flag: &Arc<AtomicBool>, progress_sender: Option<&Sender<ProgressData>>) -> WorkContinueStatus {
        let (loaded_hash_map, records_already_cached, non_cached_files_to_check) = self.load_cache_at_start();

        let progress_handler = prepare_thread_handler_common(
            progress_sender,
            CurrentStage::SimilarVideosCalculatingHashes,
            non_cached_files_to_check.len(),
            self.get_test_type(),
            0,
        );

        let non_cached_files_to_check: Vec<_> = non_cached_files_to_check.into_iter().map(|f| f.1).collect();
        let mut vec_file_entry: Vec<VideosEntry> = non_cached_files_to_check
            .into_par_iter()
            .with_max_len(2)
            .map(|file_entry| {
                if check_if_stop_received(stop_flag) {
                    return None;
                }

                let res = self.check_video_file_entry(file_entry);
                let res = Self::read_video_properties(res);

                progress_handler.increase_items(1);

                Some(res)
            })
            .while_some()
            .collect::<Vec<VideosEntry>>();

        progress_handler.join_thread();

        // Just connect loaded results with already calculated hashes
        vec_file_entry.extend(records_already_cached.into_values());

        self.save_cache(&vec_file_entry, loaded_hash_map);

        let mut hashmap_with_file_entries: IndexMap<String, VideosEntry> = Default::default();
        let mut vector_of_hashes: Vec<VideoHash> = Vec::new();
        for file_entry in vec_file_entry {
            if file_entry.error.is_empty() {
                vector_of_hashes.push(file_entry.vhash.clone());
                hashmap_with_file_entries.insert(file_entry.vhash.src_path().to_string_lossy().to_string(), file_entry);
            } else {
                self.common_data.text_messages.warnings.push(file_entry.error);
            }
        }

        // Break if stop was clicked after saving to cache
        if check_if_stop_received(stop_flag) {
            return WorkContinueStatus::Stop;
        }

        self.match_groups_of_videos(vector_of_hashes, &hashmap_with_file_entries);

        if self.create_thumbnails(progress_sender, stop_flag) == WorkContinueStatus::Stop {
            return WorkContinueStatus::Stop;
        }

        self.remove_from_reference_folders();

        if self.common_data.use_reference_folders {
            for (_fe, vector) in &self.similar_referenced_vectors {
                self.information.number_of_duplicates += vector.len();
                self.information.number_of_groups += 1;
            }
        } else {
            for vector in &self.similar_vectors {
                self.information.number_of_duplicates += vector.len() - 1;
                self.information.number_of_groups += 1;
            }
        }

        // Clean unused data
        self.videos_hashes = Default::default();
        self.videos_to_check = Default::default();

        WorkContinueStatus::Continue
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Perceptual engine (similarrio_videoo pHash + sliding window)
    // ─────────────────────────────────────────────────────────────────────────

    #[fun_time(message = "sort_videos_perceptual", level = "debug")]
    fn sort_videos_perceptual(&mut self, preset: PerceptualSearchPreset, stop_flag: &Arc<AtomicBool>, progress_sender: Option<&Sender<ProgressData>>) -> WorkContinueStatus {
        // Convert videos_to_check to PerceptualCacheEntry placeholders for cache lookup.
        // The fingerprint field is empty (Default) and used only as a placeholder;
        // real fingerprints come from the cache file or are computed below.
        let perceptual_to_check: BTreeMap<String, PerceptualCacheEntry> = mem::take(&mut self.videos_to_check)
            .into_iter()
            .map(|(key, ve)| {
                (
                    key,
                    PerceptualCacheEntry {
                        path: ve.path,
                        size: ve.size,
                        modified_date: ve.modified_date,
                        fingerprint: Default::default(),
                    },
                )
            })
            .collect();

        let (loaded_hash_map, records_already_cached, non_cached_files_to_check) =
            load_and_split_cache_generalized_by_path(&get_perceptual_cache_file(preset), perceptual_to_check, self);

        let (hash_config, min_matched_frames, max_duration_ratio) = preset.to_hash_config_and_compare_params();

        let progress_handler = prepare_thread_handler_common(
            progress_sender,
            CurrentStage::SimilarVideosCalculatingHashes,
            non_cached_files_to_check.len(),
            self.get_test_type(),
            0,
        );

        // Hash new (non-cached) files in parallel.
        let new_entries: Vec<PerceptualCacheEntry> = non_cached_files_to_check
            .into_values()
            .collect::<Vec<_>>()
            .into_par_iter()
            .with_max_len(2)
            .filter_map(|placeholder| {
                if check_if_stop_received(stop_flag) {
                    return None;
                }
                let fp = match similarrio_videoo::hash_video_cancellable(&placeholder.path, &hash_config, stop_flag) {
                    Ok(fp) => fp,
                    Err(similarrio_videoo::VideoHashError::Cancelled) => return None,
                    Err(e) => {
                        debug!("Perceptual hash failed for {}: {e}", placeholder.path.display());
                        return None;
                    }
                };
                progress_handler.increase_items(1);
                Some(PerceptualCacheEntry {
                    path: placeholder.path,
                    size: placeholder.size,
                    modified_date: placeholder.modified_date,
                    fingerprint: fp,
                })
            })
            .collect();

        progress_handler.join_thread();

        // Save cache (merge new results with the previously loaded map).
        save_and_connect_cache_generalized_by_path(&get_perceptual_cache_file(preset), &new_entries, loaded_hash_map, self);

        if check_if_stop_received(stop_flag) {
            return WorkContinueStatus::Stop;
        }

        // Combine freshly computed + already-cached entries.
        let all_entries: Vec<PerceptualCacheEntry> = new_entries.into_iter().chain(records_already_cached.into_values()).collect();

        if all_entries.len() < 2 {
            return WorkContinueStatus::Continue;
        }

        // Convert tolerance (0–20) to similarity threshold (0.90–1.00).
        // tolerance=0  → threshold=1.00 (exact match only)
        // tolerance=15 → threshold=0.925 (≈ similarrio default 0.93)
        // tolerance=20 → threshold=0.90
        let threshold = 1.0 - (self.params.tolerance as f64) * 0.005;

        let fingerprints: Vec<similarrio_videoo::VideoFingerprint> = all_entries.iter().map(|e| e.fingerprint.clone()).collect();
        let groups = similarrio_videoo::find_similar_group_indices(&fingerprints, threshold, min_matched_frames, max_duration_ratio);

        let exclude_same_size = self.params.exclude_videos_with_same_size;
        let exclude_same_resolution = self.params.exclude_videos_with_same_resolution;

        let mut similar_vectors: Vec<Vec<VideosEntry>> = Vec::new();
        for group in groups {
            let mut temp_vector: Vec<VideosEntry> = Vec::new();
            let mut bt_size: BTreeSet<u64> = Default::default();
            let mut bt_resolution: BTreeSet<(u32, u32)> = Default::default();

            for idx in group {
                let entry = &all_entries[idx];
                let w = entry.fingerprint.width;
                let h = entry.fingerprint.height;

                if exclude_same_size && !bt_size.insert(entry.size) {
                    continue;
                }
                if exclude_same_resolution && !bt_resolution.insert((w, h)) {
                    continue;
                }
                temp_vector.push(perceptual_cache_entry_to_videos_entry(entry));
            }

            if temp_vector.len() > 1 {
                similar_vectors.push(temp_vector);
            }
        }

        self.similar_vectors = similar_vectors;
        self.videos_to_check = Default::default();

        if self.create_thumbnails(progress_sender, stop_flag) == WorkContinueStatus::Stop {
            return WorkContinueStatus::Stop;
        }

        self.remove_from_reference_folders();

        if self.common_data.use_reference_folders {
            for (_fe, vector) in &self.similar_referenced_vectors {
                self.information.number_of_duplicates += vector.len();
                self.information.number_of_groups += 1;
            }
        } else {
            for vector in &self.similar_vectors {
                self.information.number_of_duplicates += vector.len() - 1;
                self.information.number_of_groups += 1;
            }
        }

        WorkContinueStatus::Continue
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Audio engine (Chromaprint via similarrio_videoo)
    // ─────────────────────────────────────────────────────────────────────────

    #[fun_time(message = "sort_videos_audio", level = "debug")]
    fn sort_videos_audio(&mut self, preset: AudioSearchPreset, stop_flag: &Arc<AtomicBool>, progress_sender: Option<&Sender<ProgressData>>) -> WorkContinueStatus {
        // Convert videos_to_check to AudioCacheEntry placeholders.
        let audio_to_check: BTreeMap<String, AudioCacheEntry> = mem::take(&mut self.videos_to_check)
            .into_iter()
            .map(|(key, ve)| {
                (
                    key,
                    AudioCacheEntry {
                        path: ve.path,
                        size: ve.size,
                        modified_date: ve.modified_date,
                        fingerprint: Default::default(),
                    },
                )
            })
            .collect();

        let (loaded_hash_map, records_already_cached, non_cached_files_to_check) =
            load_and_split_cache_generalized_by_path(&get_audio_cache_file(preset), audio_to_check, self);

        let audio_config = preset.to_audio_config();

        let progress_handler = prepare_thread_handler_common(
            progress_sender,
            CurrentStage::SimilarVideosCalculatingHashes,
            non_cached_files_to_check.len(),
            self.get_test_type(),
            0,
        );

        // Extract audio fingerprints for non-cached files in parallel.
        let new_entries: Vec<AudioCacheEntry> = non_cached_files_to_check
            .into_values()
            .collect::<Vec<_>>()
            .into_par_iter()
            .with_max_len(2)
            .filter_map(|placeholder| {
                if check_if_stop_received(stop_flag) {
                    return None;
                }
                let fp = match similarrio_videoo::extract_audio_fingerprint(&placeholder.path, &audio_config) {
                    Ok(fp) => fp,
                    Err(e) => {
                        debug!("Audio fingerprint failed for {}: {e}", placeholder.path.display());
                        return None;
                    }
                };
                progress_handler.increase_items(1);
                Some(AudioCacheEntry {
                    path: placeholder.path,
                    size: placeholder.size,
                    modified_date: placeholder.modified_date,
                    fingerprint: fp,
                })
            })
            .collect();

        progress_handler.join_thread();

        // Save cache.
        save_and_connect_cache_generalized_by_path(&get_audio_cache_file(preset), &new_entries, loaded_hash_map, self);

        if check_if_stop_received(stop_flag) {
            return WorkContinueStatus::Stop;
        }

        // Combine freshly computed + already-cached entries.
        let all_entries: Vec<AudioCacheEntry> = new_entries.into_iter().chain(records_already_cached.into_values()).collect();

        if all_entries.len() < 2 {
            return WorkContinueStatus::Continue;
        }

        // All-pairs audio comparison (parallel).
        const AUDIO_MIN_SEGMENT_SECS: f64 = 5.0;
        const AUDIO_MAX_DIFFERENCE: f64 = 0.6;

        let n = all_entries.len();
        let pairs: Vec<(usize, usize)> = (0..n).flat_map(|i| ((i + 1)..n).map(move |j| (i, j))).collect();

        let similar_pairs: Vec<(usize, usize)> = pairs
            .into_par_iter()
            .filter(|&(i, j)| {
                if check_if_stop_received(stop_flag) {
                    return false;
                }
                let matches = similarrio_videoo::find_audio_matches(&all_entries[i].fingerprint, &all_entries[j].fingerprint, AUDIO_MIN_SEGMENT_SECS, AUDIO_MAX_DIFFERENCE);
                !matches.is_empty()
            })
            .collect();

        if check_if_stop_received(stop_flag) {
            return WorkContinueStatus::Stop;
        }

        // Union-Find to build transitive groups from similar pairs.
        let groups = audio_union_find_groups(n, &similar_pairs);

        let exclude_same_size = self.params.exclude_videos_with_same_size;
        let exclude_same_resolution = self.params.exclude_videos_with_same_resolution;

        let mut similar_vectors: Vec<Vec<VideosEntry>> = Vec::new();
        for group in groups {
            let mut temp_vector: Vec<VideosEntry> = Vec::new();
            let mut bt_size: BTreeSet<u64> = Default::default();
            let mut bt_resolution: BTreeSet<(u32, u32)> = Default::default();

            for idx in group {
                let entry = &all_entries[idx];
                if exclude_same_size && !bt_size.insert(entry.size) {
                    continue;
                }
                // Audio fingerprints don't carry resolution, so skip resolution filter.
                let _ = (exclude_same_resolution, &mut bt_resolution);

                temp_vector.push(audio_cache_entry_to_videos_entry(entry));
            }

            if temp_vector.len() > 1 {
                similar_vectors.push(temp_vector);
            }
        }

        self.similar_vectors = similar_vectors;
        self.videos_to_check = Default::default();

        if self.create_thumbnails(progress_sender, stop_flag) == WorkContinueStatus::Stop {
            return WorkContinueStatus::Stop;
        }

        self.remove_from_reference_folders();

        if self.common_data.use_reference_folders {
            for (_fe, vector) in &self.similar_referenced_vectors {
                self.information.number_of_duplicates += vector.len();
                self.information.number_of_groups += 1;
            }
        } else {
            for vector in &self.similar_vectors {
                self.information.number_of_duplicates += vector.len() - 1;
                self.information.number_of_groups += 1;
            }
        }

        WorkContinueStatus::Continue
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Shared helpers
    // ─────────────────────────────────────────────────────────────────────────

    #[fun_time(message = "create_thumbnails", level = "debug")]
    fn create_thumbnails(&mut self, progress_sender: Option<&Sender<ProgressData>>, stop_flag: &Arc<AtomicBool>) -> WorkContinueStatus {
        let progress_handler = prepare_thread_handler_common(
            progress_sender,
            CurrentStage::SimilarVideosCreatingThumbnails,
            self.similar_vectors.iter().map(|e| e.len()).sum::<usize>(),
            self.get_test_type(),
            0,
        );

        let Some(config_cache_path) = get_config_cache_path() else {
            return WorkContinueStatus::Continue;
        };

        let thumbnails_dir = config_cache_path.cache_folder.join(VIDEO_THUMBNAILS_SUBFOLDER);
        if let Err(e) = std::fs::create_dir_all(&thumbnails_dir) {
            debug!("Failed to create thumbnails directory: {e}");
            return WorkContinueStatus::Continue;
        }
        let thumbnail_video_percentage_from_start = self.params.thumbnail_video_percentage_from_start;
        let generate_grid_instead_of_single = self.params.generate_thumbnail_grid_instead_of_single;
        let thumbnail_grid_tiles_per_side = self.params.thumbnail_grid_tiles_per_side;
        let errors = self
            .similar_vectors
            .par_iter_mut()
            .with_max_len(2)
            .map(|vec_file_entry| {
                let mut errs = Vec::new();
                for file_entry in vec_file_entry {
                    if check_if_stop_received(stop_flag) {
                        return errs;
                    }

                    match generate_thumbnail(
                        stop_flag,
                        &file_entry.path,
                        file_entry.size,
                        file_entry.modified_date,
                        file_entry.duration,
                        &thumbnails_dir,
                        thumbnail_video_percentage_from_start,
                        generate_grid_instead_of_single,
                        thumbnail_grid_tiles_per_side,
                        self.params.generate_thumbnails,
                    ) {
                        Ok(Some(thumbnail_path)) => {
                            file_entry.thumbnail_path = Some(thumbnail_path);
                        }
                        Ok(None) => {}
                        Err(e) => errs.push(e),
                    }

                    progress_handler.increase_items(1);
                }

                errs
            })
            .flatten()
            .collect::<Vec<String>>();

        self.common_data.text_messages.warnings.extend(errors);

        progress_handler.join_thread();
        if check_if_stop_received(stop_flag) {
            return WorkContinueStatus::Stop;
        }

        WorkContinueStatus::Continue
    }

    #[fun_time(message = "save_cache", level = "debug")]
    fn save_cache(&mut self, vec_file_entry: &[VideosEntry], loaded_hash_map: BTreeMap<String, VideosEntry>) {
        save_and_connect_cache_generalized_by_path(
            &get_similar_videos_cache_file(self.params.skip_forward_amount, self.params.duration, self.params.crop_detect),
            vec_file_entry,
            loaded_hash_map,
            self,
        );
    }

    #[fun_time(message = "load_cache_at_start", level = "debug")]
    fn load_cache_at_start(&mut self) -> (BTreeMap<String, VideosEntry>, BTreeMap<String, VideosEntry>, BTreeMap<String, VideosEntry>) {
        load_and_split_cache_generalized_by_path(
            &get_similar_videos_cache_file(self.params.skip_forward_amount, self.params.duration, self.params.crop_detect),
            mem::take(&mut self.videos_to_check),
            self,
        )
    }

    #[fun_time(message = "match_groups_of_videos", level = "debug")]
    fn match_groups_of_videos(&mut self, vector_of_hashes: Vec<VideoHash>, hashmap_with_file_entries: &IndexMap<String, VideosEntry>) {
        // Tolerance in library is a value between 0 and 1
        // Tolerance in this app is a value between 0 and 20
        // Default tolerance in library is 0.30
        // We need to allow to set value in range 0 - 0.5
        let match_group = vid_dup_finder_lib::search(vector_of_hashes, self.get_params().tolerance as f64 / 40.0f64);

        let exclude_same_size = self.get_params().exclude_videos_with_same_size;
        let exclude_same_resolution = self.get_params().exclude_videos_with_same_resolution;
        let mut collected_similar_videos: Vec<Vec<VideosEntry>> = Default::default();
        for i in match_group {
            let mut temp_vector: Vec<VideosEntry> = Vec::new();
            let mut bt_size: BTreeSet<u64> = Default::default();
            let mut bt_resolution: BTreeSet<(u32, u32)> = Default::default();
            for j in i.duplicates() {
                let file_entry = &hashmap_with_file_entries[&j.to_string_lossy().to_string()];
                if exclude_same_size && !bt_size.insert(file_entry.size) {
                    continue;
                }
                if exclude_same_resolution
                    && let (Some(w), Some(h)) = (file_entry.width, file_entry.height)
                    && !bt_resolution.insert((w, h))
                {
                    continue;
                }
                temp_vector.push(file_entry.clone());
            }
            if temp_vector.len() > 1 {
                collected_similar_videos.push(temp_vector);
            }
        }

        self.similar_vectors = collected_similar_videos;
    }

    #[fun_time(message = "remove_from_reference_folders", level = "debug")]
    fn remove_from_reference_folders(&mut self) {
        if self.common_data.use_reference_folders {
            self.similar_referenced_vectors = mem::take(&mut self.similar_vectors)
                .into_iter()
                .filter_map(|vec_file_entry| {
                    let (mut files_from_referenced_folders, normal_files): (Vec<_>, Vec<_>) = vec_file_entry
                        .into_iter()
                        .partition(|e| self.common_data.directories.is_in_referenced_directory(e.get_path()));

                    if normal_files.is_empty() {
                        None
                    } else {
                        files_from_referenced_folders.pop().map(|file| (file, normal_files))
                    }
                })
                .collect::<Vec<(VideosEntry, Vec<VideosEntry>)>>();
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Free functions
// ─────────────────────────────────────────────────────────────────────────────

pub fn get_similar_videos_cache_file(skip_forward_amount: u32, duration: u32, crop_detect: Cropdetect) -> String {
    let crop_detect_str = match crop_detect {
        Cropdetect::None => "none",
        Cropdetect::Letterbox => "letterbox",
        Cropdetect::Motion => "motion",
    };
    format!("cache_similar_videos_{CACHE_VIDEO_VERSION}__skip_{skip_forward_amount}__dur_{duration}__cd_{crop_detect_str}.bin")
}

pub fn format_bitrate_opt(bitrate: Option<u64>) -> String {
    match bitrate {
        Some(b) => {
            if b >= 1_000_000 {
                format!("{:.1} Mbps", b as f64 / 1_000_000.0)
            } else if b >= 1000 {
                format!("{:.0} kbps", b as f64 / 1000.0)
            } else {
                format!("{b} bps")
            }
        }
        None => String::from(""),
    }
}

pub fn format_duration_opt(duration: Option<f64>) -> String {
    duration
        .map(|d| {
            let hours = (d / 3600.0) as u32;
            let minutes = ((d % 3600.0) / 60.0) as u32;
            let seconds = (d % 60.0) as u32;
            if hours > 0 {
                format!("{hours:02}:{minutes:02}:{seconds:02}")
            } else {
                format!("{minutes:02}:{seconds:02}")
            }
        })
        .unwrap_or_default()
}

/// Convert a `PerceptualCacheEntry` to a `VideosEntry` for display.
///
/// Uses data from the `VideoFingerprint` (fps, width, height, duration).
/// `codec` and `bitrate` are not available from the perceptual fingerprint
/// and are left as `None`.
fn perceptual_cache_entry_to_videos_entry(entry: &PerceptualCacheEntry) -> VideosEntry {
    VideosEntry {
        path: entry.path.clone(),
        size: entry.size,
        modified_date: entry.modified_date,
        vhash: Default::default(),
        error: String::new(),
        fps: if entry.fingerprint.fps > 0.0 { Some(entry.fingerprint.fps) } else { None },
        codec: None,
        bitrate: None,
        width: if entry.fingerprint.width > 0 { Some(entry.fingerprint.width) } else { None },
        height: if entry.fingerprint.height > 0 { Some(entry.fingerprint.height) } else { None },
        duration: if entry.fingerprint.duration_secs > 0.0 { Some(entry.fingerprint.duration_secs) } else { None },
        thumbnail_path: None,
    }
}

/// Convert an `AudioCacheEntry` to a `VideosEntry` for display.
///
/// The audio fingerprint only carries `duration_secs`; other video-specific
/// fields (fps, resolution, codec, bitrate) are left as `None`.
fn audio_cache_entry_to_videos_entry(entry: &AudioCacheEntry) -> VideosEntry {
    VideosEntry {
        path: entry.path.clone(),
        size: entry.size,
        modified_date: entry.modified_date,
        vhash: Default::default(),
        error: String::new(),
        fps: None,
        codec: None,
        bitrate: None,
        width: None,
        height: None,
        duration: if entry.fingerprint.duration_secs > 0.0 { Some(entry.fingerprint.duration_secs) } else { None },
        thumbnail_path: None,
    }
}

/// Simple Union-Find (path-halving, rank-based) for building transitive groups
/// from a list of similar pairs.
///
/// Returns groups (each with ≥ 2 elements), sorted largest first.
fn audio_union_find_groups(n: usize, similar_pairs: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut parent: Vec<usize> = (0..n).collect();
    let mut rank: Vec<u8> = vec![0; n];

    fn find(parent: &mut Vec<usize>, mut x: usize) -> usize {
        while parent[x] != x {
            parent[x] = parent[parent[x]]; // path halving
            x = parent[x];
        }
        x
    }

    for &(a, b) in similar_pairs {
        let ra = find(&mut parent, a);
        let rb = find(&mut parent, b);
        if ra == rb {
            continue;
        }
        match rank[ra].cmp(&rank[rb]) {
            std::cmp::Ordering::Less => parent[ra] = rb,
            std::cmp::Ordering::Greater => parent[rb] = ra,
            std::cmp::Ordering::Equal => {
                parent[rb] = ra;
                rank[ra] += 1;
            }
        }
    }

    let mut buckets: BTreeMap<usize, Vec<usize>> = Default::default();
    for i in 0..n {
        let root = find(&mut parent, i);
        buckets.entry(root).or_default().push(i);
    }

    let mut groups: Vec<Vec<usize>> = buckets.into_values().filter(|g| g.len() >= 2).collect();
    groups.sort_by(|a, b| b.len().cmp(&a.len()));
    groups
}
