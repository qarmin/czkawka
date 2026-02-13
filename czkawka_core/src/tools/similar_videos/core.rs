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
use crate::tools::similar_videos::{SimilarVideos, SimilarVideosParameters, VideosEntry};

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
                self.videos_to_check = grouped_file_entries
                    .into_par_iter()
                    .flat_map(if self.get_hide_hard_links() { |(_, fes)| fes } else { take_1_per_inode })
                    .map(|fe| (fe.path.to_string_lossy().to_string(), fe.into_videos_entry()))
                    .collect();
                self.common_data.text_messages.warnings.extend(warnings);
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

    #[fun_time(message = "create_thumbnails", level = "debug")]
    fn create_thumbnails(&mut self, progress_sender: Option<&Sender<ProgressData>>, stop_flag: &Arc<AtomicBool>) -> WorkContinueStatus {
        if !self.params.generate_thumbnails {
            return WorkContinueStatus::Continue;
        }

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
                    ) {
                        Ok(thumbnail_path) => {
                            file_entry.thumbnail_path = Some(thumbnail_path);
                        }
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
