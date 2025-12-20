use std::collections::{BTreeMap, BTreeSet};
use std::mem;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use crossbeam_channel::Sender;
use ffprobe::ffprobe;
use fun_time::fun_time;
use indexmap::IndexMap;
use log::debug;
use rayon::prelude::*;
use vid_dup_finder_lib::{CreationOptions, Cropdetect, VideoHash, VideoHashBuilder};

use crate::common::cache::{CACHE_VIDEO_VERSION, extract_loaded_cache, load_cache_from_file_generalized_by_path, save_cache_to_file_generalized};
use crate::common::consts::VIDEO_FILES_EXTENSIONS;
use crate::common::dir_traversal::{DirTraversalBuilder, DirTraversalResult, inode, take_1_per_inode};
use crate::common::model::{ToolType, WorkContinueStatus};
use crate::common::progress_data::{CurrentStage, ProgressData};
use crate::common::progress_stop_handler::{check_if_stop_received, prepare_thread_handler_common};
use crate::common::tool_data::{CommonData, CommonToolData};
use crate::common::traits::ResultEntry;
use crate::tools::similar_videos::{SimilarVideos, SimilarVideosParameters, VideosEntry};

impl SimilarVideos {
    pub fn new(params: SimilarVideosParameters) -> Self {
        Self {
            common_data: CommonToolData::new(ToolType::SimilarVideos),
            information: Default::default(),
            similar_vectors: vec![],
            videos_hashes: Default::default(),
            videos_to_check: Default::default(),
            similar_referenced_vectors: vec![],
            params,
        }
    }

    #[fun_time(message = "check_for_similar_videos", level = "debug")]
    pub(crate) fn check_for_similar_videos(&mut self, stop_flag: &Arc<AtomicBool>, progress_sender: Option<&Sender<ProgressData>>) -> WorkContinueStatus {
        self.common_data.extensions.set_and_validate_allowed_extensions(VIDEO_FILES_EXTENSIONS);
        if !self.common_data.extensions.set_any_extensions() {
            return WorkContinueStatus::Continue;
        }

        let result = DirTraversalBuilder::new()
            .group_by(inode)
            .stop_flag(stop_flag)
            .progress_sender(progress_sender)
            .common_data(&self.common_data)
            .build()
            .run();

        match result {
            DirTraversalResult::SuccessFiles { grouped_file_entries, warnings } => {
                self.videos_to_check = grouped_file_entries
                    .into_par_iter()
                    .flat_map(if self.get_params().ignore_hard_links { |(_, fes)| fes } else { take_1_per_inode })
                    .map(|fe| (fe.path.to_string_lossy().to_string(), fe.into_videos_entry()))
                    .collect();
                self.common_data.text_messages.warnings.extend(warnings);
                debug!("check_files - Found {} video files.", self.videos_to_check.len());
                WorkContinueStatus::Continue
            }

            DirTraversalResult::Stopped => WorkContinueStatus::Stop,
        }
    }

    #[fun_time(message = "load_cache_at_start", level = "debug")]
    fn load_cache_at_start(&mut self) -> (BTreeMap<String, VideosEntry>, BTreeMap<String, VideosEntry>, BTreeMap<String, VideosEntry>) {
        let loaded_hash_map;
        let mut records_already_cached: BTreeMap<String, VideosEntry> = Default::default();
        let mut non_cached_files_to_check: BTreeMap<String, VideosEntry> = Default::default();

        if self.common_data.use_cache {
            let (messages, loaded_items) = load_cache_from_file_generalized_by_path::<VideosEntry>(
                &get_similar_videos_cache_file(self.params.skip_forward_amount, self.params.duration, self.params.crop_detect),
                self.get_delete_outdated_cache(),
                &self.videos_to_check,
            );
            self.get_text_messages_mut().extend_with_another_messages(messages);
            loaded_hash_map = loaded_items.unwrap_or_default();

            extract_loaded_cache(
                &loaded_hash_map,
                mem::take(&mut self.videos_to_check),
                &mut records_already_cached,
                &mut non_cached_files_to_check,
            );
        } else {
            loaded_hash_map = Default::default();
            mem::swap(&mut self.videos_to_check, &mut non_cached_files_to_check);
        }
        (loaded_hash_map, records_already_cached, non_cached_files_to_check)
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
        match ffprobe(file_entry.path.clone()) {
            Ok(info) => {
                if let Some(stream) = info.streams.into_iter().find(|s| s.codec_type.as_deref() == Some("video")) {
                    if let Some(codec_name) = stream.codec_name {
                        file_entry.codec = Some(codec_name);
                    }

                    if let Some(bit_rate_str) = stream.bit_rate.or(info.format.bit_rate)
                        && let Ok(b) = bit_rate_str.parse::<u64>()
                    {
                        file_entry.bitrate = Some(b);
                    }

                    if let Some(w) = stream.width
                        && w >= 0
                    {
                        file_entry.width = Some(w as u32);
                    }
                    if let Some(h) = stream.height
                        && h >= 0
                    {
                        file_entry.height = Some(h as u32);
                    }

                    let fps_opt = if !stream.avg_frame_rate.is_empty() && stream.avg_frame_rate != "0/0" {
                        Some(stream.avg_frame_rate)
                    } else if !stream.r_frame_rate.is_empty() && stream.r_frame_rate != "0/0" {
                        Some(stream.r_frame_rate)
                    } else {
                        None
                    };

                    if let Some(fps_str) = fps_opt {
                        let fps_val = if fps_str.contains('/') {
                            let mut parts = fps_str.splitn(2, '/');
                            if let (Some(n), Some(d)) = (parts.next(), parts.next()) {
                                if let (Ok(nv), Ok(dv)) = (n.parse::<f64>(), d.parse::<f64>()) {
                                    if dv != 0.0 { Some(nv / dv) } else { None }
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        } else {
                            fps_str.parse::<f64>().ok()
                        };

                        if let Some(fps_v) = fps_val {
                            file_entry.fps = Some(fps_v);
                        }
                    }
                }
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

        let mut vec_file_entry: Vec<VideosEntry> = non_cached_files_to_check
            .into_par_iter()
            .map(|(_, file_entry)| {
                if check_if_stop_received(stop_flag) {
                    return None;
                }

                // Currently size is not too much relevant
                // let size = file_entry.size;
                let res = self.check_video_file_entry(file_entry);
                let res = Self::read_video_properties(res);

                progress_handler.increase_items(1);
                // progress_handler.increase_size(size);

                Some(res)
            })
            .while_some()
            .collect::<Vec<VideosEntry>>();

        progress_handler.join_thread();

        // Just connect loaded results with already calculated hashes
        vec_file_entry.extend(records_already_cached.into_values());

        let mut hashmap_with_file_entries: IndexMap<String, VideosEntry> = Default::default();
        let mut vector_of_hashes: Vec<VideoHash> = Vec::new();
        for file_entry in &vec_file_entry {
            // 0 means that images was not hashed correctly, e.g. could be improperly
            if file_entry.error.is_empty() {
                hashmap_with_file_entries.insert(file_entry.vhash.src_path().to_string_lossy().to_string(), file_entry.clone());
                vector_of_hashes.push(file_entry.vhash.clone());
            } else {
                self.common_data.text_messages.warnings.push(file_entry.error.clone());
            }
        }

        self.save_cache(vec_file_entry, loaded_hash_map);

        // Break if stop was clicked after saving to cache
        if check_if_stop_received(stop_flag) {
            return WorkContinueStatus::Stop;
        }

        self.match_groups_of_videos(vector_of_hashes, &hashmap_with_file_entries);
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

    #[fun_time(message = "save_cache", level = "debug")]
    fn save_cache(&mut self, vec_file_entry: Vec<VideosEntry>, loaded_hash_map: BTreeMap<String, VideosEntry>) {
        if self.common_data.use_cache {
            // Must save all results to file, old loaded from file with all currently counted results
            let mut all_results: BTreeMap<String, VideosEntry> = loaded_hash_map;
            for file_entry in vec_file_entry {
                all_results.insert(file_entry.path.to_string_lossy().to_string(), file_entry);
            }

            let messages = save_cache_to_file_generalized(
                &get_similar_videos_cache_file(self.params.skip_forward_amount, self.params.duration, self.params.crop_detect),
                &all_results,
                self.common_data.save_also_as_json,
                0,
            );
            self.get_text_messages_mut().extend_with_another_messages(messages);
        }
    }

    #[fun_time(message = "match_groups_of_videos", level = "debug")]
    fn match_groups_of_videos(&mut self, vector_of_hashes: Vec<VideoHash>, hashmap_with_file_entries: &IndexMap<String, VideosEntry>) {
        // Tolerance in library is a value between 0 and 1
        // Tolerance in this app is a value between 0 and 20
        // Default tolerance in library is 0.30
        // We need to allow to set value in range 0 - 0.5
        let match_group = vid_dup_finder_lib::search(vector_of_hashes, self.get_params().tolerance as f64 / 40.0f64);

        let mut collected_similar_videos: Vec<Vec<VideosEntry>> = Default::default();
        for i in match_group {
            let mut temp_vector: Vec<VideosEntry> = Vec::new();
            let mut bt_size: BTreeSet<u64> = Default::default();
            for j in i.duplicates() {
                let file_entry = &hashmap_with_file_entries[&j.to_string_lossy().to_string()];
                if self.get_params().exclude_videos_with_same_size {
                    if bt_size.insert(file_entry.size) {
                        temp_vector.push(file_entry.clone());
                    }
                } else {
                    temp_vector.push(file_entry.clone());
                }
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
