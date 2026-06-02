use std::collections::{BTreeMap, BTreeSet};
use std::mem;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use crossbeam_channel::Sender;
use fun_time::fun_time;
use indexmap::{IndexMap, IndexSet};
use log::debug;
use rayon::prelude::*;
use rusty_chromaprint::{Configuration, match_fingerprints};
use similario_core::SignatureConfig;
use similario_core::compare::{CompareConfig, find_similar};
use similario_core::visual::VideoSignature;

use crate::common::audio_fingerprint::calc_fingerprint_and_duration;
use crate::common::cache::{CACHE_VERSION, CACHE_VIDEO_VERSION, load_and_split_cache_generalized_by_path, save_and_connect_cache_generalized_by_path};
use crate::common::config_cache_path::get_config_cache_path;
use crate::common::dir_traversal::{DirTraversalBuilder, DirTraversalResult, inode, take_1_per_inode};
use crate::common::model::{ToolType, WorkContinueStatus};
use crate::common::progress_data::{CurrentStage, ProgressData};
use crate::common::progress_stop_handler::{check_if_stop_received, prepare_thread_handler_common};
use crate::common::tool_data::{CommonData, CommonToolData};
use crate::common::video_utils::{VIDEO_THUMBNAILS_SUBFOLDER, VideoMetadata, generate_thumbnail};
use crate::flc;
use crate::tools::similar_videos::{SimilarVideos, SimilarVideosParameters, VideoAudioEntry, VideosEntry};

impl SimilarVideos {
    pub fn new(params: SimilarVideosParameters) -> Self {
        Self {
            common_data: CommonToolData::new(ToolType::SimilarVideos),
            information: Default::default(),
            similar_vectors: Vec::new(),
            videos_to_check: Default::default(),
            audio_to_check: Default::default(),
            similar_referenced_vectors: Vec::new(),
            audio_config: Configuration::preset_test1(),
            params,
        }
    }

    fn signature_config(&self) -> SignatureConfig {
        SignatureConfig {
            skip_secs: f64::from(self.params.skip_forward_amount),
            window_count: self.params.window_count as usize,
            window_secs: f64::from(self.params.duration),
            cropdetect: self.params.crop_detect,
            audio_fingerprint: false,
        }
    }

    fn compare_config(&self) -> CompareConfig {
        // Map czkawka tolerance (0..=20) to similario_core tolerance (0.0..=0.5).
        // Tolerance 0 → very strict; 20 → very loose.
        let tolerance = (self.params.tolerance as f32) / 40.0;
        CompareConfig {
            tolerance,
            duration_tolerance_pct: self.params.duration_tolerance_pct,
            min_matching_windows: self.params.min_matching_windows as f32,
            subclip_min_match: self.params.subclip_min_match as f32,
            ..CompareConfig::default()
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
                let check_audio = self.params.check_audio_content;
                let file_entries: Vec<_> = grouped_file_entries
                    .into_par_iter()
                    .map(|(inode, fes)| {
                        if check_if_stop_received(stop_flag) {
                            return None;
                        }
                        progress_handler.increase_items(1);
                        Some((inode, fes))
                    })
                    .while_some()
                    .flat_map(if hide_hard_links { take_1_per_inode } else { |(_, fes)| fes })
                    .collect();

                if check_audio {
                    self.audio_to_check = file_entries
                        .into_iter()
                        .map(|fe| (fe.path.to_string_lossy().to_string(), fe.into_video_audio_entry()))
                        .collect();
                } else {
                    self.videos_to_check = file_entries.into_iter().map(|fe| (fe.path.to_string_lossy().to_string(), fe.into_videos_entry())).collect();
                }

                progress_handler.join_thread();

                if check_if_stop_received(stop_flag) {
                    return WorkContinueStatus::Stop;
                }

                debug!("check_files - Found {} video files.", self.videos_to_check.len() + self.audio_to_check.len());
                WorkContinueStatus::Continue
            }

            DirTraversalResult::Stopped => WorkContinueStatus::Stop,
        }
    }

    fn check_video_file_entry(&self, mut file_entry: VideosEntry, stop_flag: &AtomicBool) -> VideosEntry {
        let sig_config = self.signature_config();
        match VideoSignature::from_path(&file_entry.path, &sig_config, stop_flag) {
            Ok(sig) => {
                file_entry.signature = Some(sig);
            }
            Err(e) => {
                let path = file_entry.path.to_string_lossy();
                file_entry.error = format!("Failed to hash file \"{path}\": reason {e}");
            }
        }
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

    #[fun_time(message = "sort_videos", level = "debug")]
    pub(crate) fn sort_videos(&mut self, stop_flag: &Arc<AtomicBool>, progress_sender: Option<&Sender<ProgressData>>) -> WorkContinueStatus {
        if self.videos_to_check.is_empty() {
            return WorkContinueStatus::Continue;
        }

        let (loaded_hash_map, records_already_cached, non_cached_files_to_check) = self.load_cache_at_start();

        let progress_handler = prepare_thread_handler_common(
            progress_sender,
            CurrentStage::SimilarVideosCalculatingHashes,
            non_cached_files_to_check.len(),
            self.get_test_type(),
            0, // non_cached_files_to_check.values().map(|e| e.size).sum(), // Looks, that at least for now, there is no big difference between checking big and small files, so at least for now, only tracking number of files is enough
        );

        let non_cached_files_to_check: Vec<_> = non_cached_files_to_check.into_iter().map(|f| f.1).collect();
        let mut vec_file_entry: Vec<VideosEntry> = non_cached_files_to_check
            .into_par_iter()
            .with_max_len(2)
            .map(|file_entry| {
                if check_if_stop_received(stop_flag) {
                    return None;
                }

                let res = self.check_video_file_entry(file_entry, stop_flag);
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
        let mut signatures: Vec<VideoSignature> = Vec::new();
        for file_entry in vec_file_entry {
            if file_entry.error.is_empty() {
                if let Some(sig) = file_entry.signature.clone() {
                    let key = sig.path.to_string_lossy().to_string();
                    signatures.push(sig);
                    hashmap_with_file_entries.insert(key, file_entry);
                }
            } else {
                self.common_data.text_messages.warnings.push(file_entry.error);
            }
        }

        // Break if stop was clicked after saving to cache
        if check_if_stop_received(stop_flag) {
            return WorkContinueStatus::Stop;
        }

        self.match_groups_of_videos(&signatures, &hashmap_with_file_entries);

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
        self.videos_to_check = Default::default();

        WorkContinueStatus::Continue
    }

    #[fun_time(message = "create_thumbnails", level = "debug")]
    pub(crate) fn create_thumbnails(&mut self, progress_sender: Option<&Sender<ProgressData>>, stop_flag: &Arc<AtomicBool>) -> WorkContinueStatus {
        let stage = if self.params.check_audio_content {
            CurrentStage::SimilarVideosAudioCreatingThumbnails
        } else {
            CurrentStage::SimilarVideosCreatingThumbnails
        };
        let progress_handler = prepare_thread_handler_common(progress_sender, stage, self.similar_vectors.iter().map(|e| e.len()).sum::<usize>(), self.get_test_type(), 0);

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
            &get_similar_videos_cache_file(self.params.skip_forward_amount, self.params.duration, self.params.crop_detect, self.params.window_count),
            vec_file_entry,
            loaded_hash_map,
            self,
        );
    }

    #[fun_time(message = "load_cache_at_start", level = "debug")]
    fn load_cache_at_start(&mut self) -> (BTreeMap<String, VideosEntry>, BTreeMap<String, VideosEntry>, BTreeMap<String, VideosEntry>) {
        load_and_split_cache_generalized_by_path(
            &get_similar_videos_cache_file(self.params.skip_forward_amount, self.params.duration, self.params.crop_detect, self.params.window_count),
            mem::take(&mut self.videos_to_check),
            self,
        )
    }

    #[fun_time(message = "match_groups_of_videos", level = "debug")]
    fn match_groups_of_videos(&mut self, signatures: &[VideoSignature], hashmap_with_file_entries: &IndexMap<String, VideosEntry>) {
        let cmp_config = self.compare_config();
        let groups = find_similar(signatures, &cmp_config);

        let exclude_same_size = self.get_params().exclude_videos_with_same_size;
        let exclude_same_resolution = self.get_params().exclude_videos_with_same_resolution;
        let mut collected_similar_videos: Vec<Vec<VideosEntry>> = Default::default();
        for group in groups {
            let mut temp_vector: Vec<VideosEntry> = Vec::new();
            let mut bt_size: BTreeSet<u64> = Default::default();
            let mut bt_resolution: BTreeSet<(u32, u32)> = Default::default();
            for path in &group.files {
                let key = path.to_string_lossy().to_string();
                let Some(file_entry) = hashmap_with_file_entries.get(&key) else {
                    continue;
                };
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
                temp_vector.sort_unstable_by(|a, b| a.modified_date.cmp(&b.modified_date).then(a.path.cmp(&b.path)));
                collected_similar_videos.push(temp_vector);
            }
        }

        self.similar_vectors = collected_similar_videos;
    }

    #[fun_time(message = "remove_from_reference_folders", level = "debug")]
    fn remove_from_reference_folders(&mut self) {
        if self.common_data.use_reference_folders {
            self.similar_referenced_vectors = self.common_data.directories.filter_reference_folders(mem::take(&mut self.similar_vectors));
        }
    }
    #[fun_time(message = "calculate_audio_fingerprints", level = "debug")]
    pub(crate) fn calculate_audio_fingerprints(&mut self, stop_flag: &Arc<AtomicBool>, progress_sender: Option<&Sender<ProgressData>>) -> WorkContinueStatus {
        if self.audio_to_check.is_empty() {
            return WorkContinueStatus::Continue;
        }

        let progress_handler = prepare_thread_handler_common(progress_sender, CurrentStage::SimilarVideosAudioCacheLoading, 0, self.get_test_type(), 0);

        let (loaded_hash_map, records_already_cached, non_cached_files_to_check) =
            load_and_split_cache_generalized_by_path(&get_similar_videos_audio_cache_file(), mem::take(&mut self.audio_to_check), self);

        progress_handler.join_thread();
        if check_if_stop_received(stop_flag) {
            return WorkContinueStatus::Stop;
        }

        let progress_handler = prepare_thread_handler_common(
            progress_sender,
            CurrentStage::SimilarVideosAudioCalculatingFingerprints,
            non_cached_files_to_check.len(),
            self.get_test_type(),
            non_cached_files_to_check.values().map(|e| e.size).sum::<u64>(),
        );
        let configuration = &self.audio_config;

        let non_cached_vec: Vec<_> = non_cached_files_to_check.into_iter().collect();
        let mut vec_audio_entries: Vec<VideoAudioEntry> = non_cached_vec
            .into_par_iter()
            .with_max_len(2)
            .map(|(path, mut audio_entry)| {
                if check_if_stop_received(stop_flag) {
                    return None;
                }

                let size = audio_entry.size;
                let res = calc_fingerprint_and_duration(&path, configuration, stop_flag);
                progress_handler.increase_size(size);
                progress_handler.increase_items(1);

                match res {
                    Err(_e) => {
                        // error!("Can't calculate audio fingerprint: {}", e);
                        Some(None)
                    }
                    Ok(None) => None,
                    Ok(Some((fingerprint, duration_seconds))) => {
                        audio_entry.fingerprint = fingerprint;
                        audio_entry.audio_duration_seconds = duration_seconds;
                        Some(Some(audio_entry))
                    }
                }
            })
            .while_some()
            .flatten()
            .collect();

        progress_handler.join_thread();

        let progress_handler = prepare_thread_handler_common(progress_sender, CurrentStage::SimilarVideosAudioCacheSaving, 0, self.get_test_type(), 0);

        vec_audio_entries.extend(records_already_cached.into_values());

        save_and_connect_cache_generalized_by_path(&get_similar_videos_audio_cache_file(), &vec_audio_entries, loaded_hash_map, self);

        self.audio_to_check = vec_audio_entries.into_iter().map(|e| (e.path.to_string_lossy().to_string(), e)).collect();

        progress_handler.join_thread();
        if check_if_stop_received(stop_flag) {
            return WorkContinueStatus::Stop;
        }
        WorkContinueStatus::Continue
    }

    #[fun_time(message = "compare_audio_fingerprints", level = "debug")]
    pub(crate) fn compare_audio_fingerprints(&mut self, stop_flag: &Arc<AtomicBool>, progress_sender: Option<&Sender<ProgressData>>) -> WorkContinueStatus {
        if self.audio_to_check.is_empty() {
            return WorkContinueStatus::Continue;
        }

        let audio_min_duration_seconds = self.params.audio_min_duration_seconds;
        let entries: Vec<VideoAudioEntry> = mem::take(&mut self.audio_to_check)
            .into_values()
            .filter(|e| e.audio_duration_seconds >= audio_min_duration_seconds)
            .collect();

        let progress_handler = prepare_thread_handler_common(
            progress_sender,
            CurrentStage::SimilarVideosAudioComparingFingerprints,
            entries.len(),
            self.get_test_type(),
            0,
        );

        let audio_similarity_percent = self.params.audio_similarity_percent;
        let audio_length_ratio = self.params.audio_length_ratio;
        let maximum_difference = self.params.maximum_difference;
        let configuration = &self.audio_config;

        let mut similar_vectors: Vec<Vec<VideosEntry>> = Vec::new();
        let mut used_paths: IndexSet<String> = Default::default();

        let lookup: BTreeMap<String, &VideoAudioEntry> = entries.iter().map(|e| (e.path.to_string_lossy().to_string(), e)).collect();

        for f_entry in &entries {
            if check_if_stop_received(stop_flag) {
                progress_handler.join_thread();
                return WorkContinueStatus::Stop;
            }

            progress_handler.increase_items(1);
            let f_string = f_entry.path.to_string_lossy().to_string();
            if used_paths.contains(&f_string) {
                continue;
            }

            let f_duration = f64::from(f_entry.audio_duration_seconds);

            let (mut similar_entries, errors): (Vec<_>, Vec<_>) = entries
                .par_iter()
                .map(|e_entry| {
                    let e_string = e_entry.path.to_string_lossy().to_string();
                    if used_paths.contains(&e_string) || e_string == f_string {
                        return None;
                    }

                    let e_duration = f64::from(e_entry.audio_duration_seconds);
                    let shorter = f_duration.min(e_duration);
                    let longer = f_duration.max(e_duration);
                    if longer == 0.0 || shorter / longer < audio_length_ratio {
                        return None;
                    }

                    let mut segments = match match_fingerprints(&f_entry.fingerprint, &e_entry.fingerprint, configuration) {
                        Ok(s) => s,
                        Err(e) => return Some(Err(flc!("core_error_comparing_fingerprints", reason = e.to_string()))),
                    };
                    segments.retain(|s| s.score < maximum_difference);
                    let matched_duration: f32 = segments.iter().map(|s| s.duration(configuration)).sum();
                    let threshold = shorter as f32 * (audio_similarity_percent / 100.0) as f32;
                    if matched_duration >= threshold { Some(Ok(e_string)) } else { None }
                })
                .flatten()
                .partition_map(|res| match res {
                    Ok(path) => itertools::Either::Left(path),
                    Err(err) => itertools::Either::Right(err),
                });

            self.common_data.text_messages.errors.extend(errors);

            similar_entries.retain(|path| !used_paths.contains(path));
            if !similar_entries.is_empty() {
                let mut result_group: Vec<VideosEntry> = similar_entries
                    .iter()
                    .filter_map(|path| {
                        used_paths.insert(path.clone());
                        lookup.get(path).map(|ae| audio_entry_to_videos_entry(ae))
                    })
                    .collect();
                used_paths.insert(f_string);
                result_group.push(audio_entry_to_videos_entry(f_entry));
                similar_vectors.push(result_group);
            }
        }

        progress_handler.join_thread();

        let exclude_same_size = self.params.exclude_videos_with_same_size;
        let exclude_same_resolution = self.params.exclude_videos_with_same_resolution;
        if exclude_same_size || exclude_same_resolution {
            similar_vectors = similar_vectors
                .into_par_iter()
                .map(|group| {
                    let enriched: Vec<VideosEntry> = if exclude_same_resolution {
                        group
                            .into_par_iter()
                            .map(|mut entry| {
                                if (entry.width.is_none() || entry.height.is_none())
                                    && let Ok(meta) = VideoMetadata::from_path(&entry.path)
                                {
                                    entry.width = meta.width;
                                    entry.height = meta.height;
                                }
                                entry
                            })
                            .collect()
                    } else {
                        group
                    };
                    let mut bt_size: BTreeSet<u64> = Default::default();
                    let mut bt_resolution: BTreeSet<(u32, u32)> = Default::default();
                    let mut filtered_group: Vec<VideosEntry> = Vec::new();
                    for entry in enriched {
                        if exclude_same_size && !bt_size.insert(entry.size) {
                            continue;
                        }
                        if exclude_same_resolution
                            && let (Some(w), Some(h)) = (entry.width, entry.height)
                            && !bt_resolution.insert((w, h))
                        {
                            continue;
                        }
                        filtered_group.push(entry);
                    }
                    filtered_group
                })
                .filter(|g| g.len() > 1)
                .collect();
        }

        self.similar_vectors = similar_vectors;

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
}

fn audio_entry_to_videos_entry(ae: &VideoAudioEntry) -> VideosEntry {
    VideosEntry {
        path: ae.path.clone(),
        size: ae.size,
        modified_date: ae.modified_date,
        signature: None,
        error: String::new(),
        fps: None,
        codec: None,
        bitrate: None,
        width: None,
        height: None,
        duration: Some(f64::from(ae.audio_duration_seconds)),
        thumbnail_path: None,
    }
}

pub fn get_similar_videos_audio_cache_file() -> String {
    format!("cache_similar_videos_audio_{CACHE_VERSION}.bin")
}

pub fn get_similar_videos_cache_file(skip_forward_amount: u32, duration: u32, crop_detect: bool, window_count: u32) -> String {
    let cd = if crop_detect { "on" } else { "off" };
    format!("cache_similar_videos_{CACHE_VIDEO_VERSION}__skip_{skip_forward_amount}__dur_{duration}__cd_{cd}__wc_{window_count}.bin")
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
