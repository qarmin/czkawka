use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::io::Write;
use std::mem;
use std::ops::RangeInclusive;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use crossbeam_channel::Sender;
use fun_time::fun_time;
use humansize::{BINARY, format_size};
use log::debug;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use vid_dup_finder_lib::{CreationOptions, Cropdetect, VideoHash, VideoHashBuilder};

use crate::common::WorkContinueStatus;
use crate::common::cache::{extract_loaded_cache, get_similar_videos_cache_file, load_cache_from_file_generalized_by_path, save_cache_to_file_generalized};
use crate::common::consts::VIDEO_FILES_EXTENSIONS;
use crate::common::progress_data::{CurrentStage, ProgressData};
use crate::common::progress_stop_handler::{check_if_stop_received, prepare_thread_handler_common, send_info_and_wait_for_ending_all_threads};
use crate::common_dir_traversal::{DirTraversalBuilder, DirTraversalResult, FileEntry, ToolType, inode, take_1_per_inode};
use crate::common_tool::{CommonData, CommonToolData, DeleteMethod};
use crate::common_traits::{DebugPrint, DeletingItems, PrintResults, ResultEntry};
use crate::flc;

pub const MAX_TOLERANCE: i32 = 20;

pub const DEFAULT_CROP_DETECT: Cropdetect = Cropdetect::Letterbox;

pub const ALLOWED_SKIP_FORWARD_AMOUNT: RangeInclusive<u32> = 0..=300;
pub const DEFAULT_SKIP_FORWARD_AMOUNT: u32 = 15;

pub const ALLOWED_VID_HASH_DURATION: RangeInclusive<u32> = 2..=60;
pub const DEFAULT_VID_HASH_DURATION: u32 = 10;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VideosEntry {
    pub path: PathBuf,
    pub size: u64,
    pub modified_date: u64,
    pub vhash: VideoHash,
    pub error: String,
}

impl ResultEntry for VideosEntry {
    fn get_path(&self) -> &Path {
        &self.path
    }
    fn get_modified_date(&self) -> u64 {
        self.modified_date
    }
    fn get_size(&self) -> u64 {
        self.size
    }
}

impl FileEntry {
    fn into_videos_entry(self) -> VideosEntry {
        VideosEntry {
            size: self.size,
            path: self.path,
            modified_date: self.modified_date,

            vhash: Default::default(),
            error: String::new(),
        }
    }
}

pub struct SimilarVideosParameters {
    pub tolerance: i32,
    pub exclude_videos_with_same_size: bool,
    pub ignore_hard_links: bool,
    pub skip_forward_amount: u32,
    pub duration: u32,
    pub crop_detect: Cropdetect,
}

pub fn crop_detect_from_str_opt(s: &str) -> Option<Cropdetect> {
    match s.to_lowercase().as_str() {
        "none" => Some(Cropdetect::None),
        "letterbox" => Some(Cropdetect::Letterbox),
        "motion" => Some(Cropdetect::Motion),
        _ => None,
    }
}

pub fn crop_detect_from_str(s: &str) -> Cropdetect {
    crop_detect_from_str_opt(s).unwrap_or(DEFAULT_CROP_DETECT)
}
pub fn crop_detect_to_str(crop_detect: Cropdetect) -> String {
    match crop_detect {
        Cropdetect::None => "none".to_string(),
        Cropdetect::Letterbox => "letterbox".to_string(),
        Cropdetect::Motion => "motion".to_string(),
    }
}

impl SimilarVideosParameters {
    pub fn new(tolerance: i32, exclude_videos_with_same_size: bool, ignore_hard_links: bool, skip_forward_amount: u32, duration: u32, crop_detect: Cropdetect) -> Self {
        assert!((0..=MAX_TOLERANCE).contains(&tolerance));
        assert!(ALLOWED_SKIP_FORWARD_AMOUNT.contains(&skip_forward_amount));
        assert!(ALLOWED_VID_HASH_DURATION.contains(&duration));
        Self {
            tolerance,
            exclude_videos_with_same_size,
            ignore_hard_links,
            skip_forward_amount,
            duration,
            crop_detect,
        }
    }
}

pub struct SimilarVideos {
    common_data: CommonToolData,
    information: Info,
    similar_vectors: Vec<Vec<VideosEntry>>,
    similar_referenced_vectors: Vec<(VideosEntry, Vec<VideosEntry>)>,
    videos_hashes: BTreeMap<Vec<u8>, Vec<VideosEntry>>,
    videos_to_check: BTreeMap<String, VideosEntry>,
    params: SimilarVideosParameters,
}

impl CommonData for SimilarVideos {
    fn get_cd(&self) -> &CommonToolData {
        &self.common_data
    }
    fn get_cd_mut(&mut self) -> &mut CommonToolData {
        &mut self.common_data
    }
    fn found_any_broken_files(&self) -> bool {
        self.information.number_of_duplicates > 0
    }
}

#[derive(Default)]
pub struct Info {
    pub number_of_duplicates: usize,
    pub number_of_groups: u64,
}

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

    #[fun_time(message = "find_similar_videos", level = "info")]
    pub fn find_similar_videos(&mut self, stop_flag: &Arc<AtomicBool>, progress_sender: Option<&Sender<ProgressData>>) {
        if !ffmpeg_cmdline_utils::ffmpeg_and_ffprobe_are_callable() {
            self.common_data.text_messages.errors.push(flc!("core_ffmpeg_not_found"));
            #[cfg(target_os = "windows")]
            self.common_data.text_messages.errors.push(flc!("core_ffmpeg_not_found_windows"));
        } else {
            self.prepare_items();
            self.common_data.use_reference_folders = !self.common_data.directories.reference_directories.is_empty();
            if self.check_for_similar_videos(stop_flag, progress_sender) == WorkContinueStatus::Stop {
                self.common_data.stopped_search = true;
                return;
            }
            if self.sort_videos(stop_flag, progress_sender) == WorkContinueStatus::Stop {
                self.common_data.stopped_search = true;
                return;
            }
        }
        if self.delete_files(stop_flag, progress_sender) == WorkContinueStatus::Stop {
            self.common_data.stopped_search = true;
            return;
        };
        self.debug_print();
    }

    #[fun_time(message = "check_for_similar_videos", level = "debug")]
    fn check_for_similar_videos(&mut self, stop_flag: &Arc<AtomicBool>, progress_sender: Option<&Sender<ProgressData>>) -> WorkContinueStatus {
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
                file_entry.error = format!("Failed to hash file {path}: reason {e}");
                return file_entry;
            }
        };

        file_entry.vhash = vhash;

        file_entry
    }

    #[fun_time(message = "sort_videos", level = "debug")]
    fn sort_videos(&mut self, stop_flag: &Arc<AtomicBool>, progress_sender: Option<&Sender<ProgressData>>) -> WorkContinueStatus {
        if self.videos_to_check.is_empty() {
            return WorkContinueStatus::Continue;
        }

        let (loaded_hash_map, records_already_cached, non_cached_files_to_check) = self.load_cache_at_start();

        let (progress_thread_handle, progress_thread_run, items_counter, check_was_stopped, size_counter) = prepare_thread_handler_common(
            progress_sender,
            CurrentStage::SimilarVideosCalculatingHashes,
            non_cached_files_to_check.len(),
            self.get_test_type(),
            non_cached_files_to_check.values().map(|e| e.size).sum(),
        );

        let mut vec_file_entry: Vec<VideosEntry> = non_cached_files_to_check
            .into_par_iter()
            .map(|(_, file_entry)| {
                if check_if_stop_received(stop_flag) {
                    check_was_stopped.store(true, Ordering::Relaxed);
                    return None;
                }

                let size = file_entry.size;
                let res = self.check_video_file_entry(file_entry);

                items_counter.fetch_add(1, Ordering::Relaxed);
                size_counter.fetch_add(size, Ordering::Relaxed);

                Some(res)
            })
            .while_some()
            .collect::<Vec<VideosEntry>>();

        send_info_and_wait_for_ending_all_threads(&progress_thread_run, progress_thread_handle);

        // Just connect loaded results with already calculated hashes
        vec_file_entry.extend(records_already_cached.into_values());

        let mut hashmap_with_file_entries: HashMap<String, VideosEntry> = Default::default();
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
        if check_was_stopped.load(Ordering::Relaxed) {
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
    fn match_groups_of_videos(&mut self, vector_of_hashes: Vec<VideoHash>, hashmap_with_file_entries: &HashMap<String, VideosEntry>) {
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

impl DeletingItems for SimilarVideos {
    #[fun_time(message = "delete_files", level = "debug")]
    fn delete_files(&mut self, stop_flag: &Arc<AtomicBool>, progress_sender: Option<&Sender<ProgressData>>) -> WorkContinueStatus {
        if self.get_cd().delete_method == DeleteMethod::None {
            return WorkContinueStatus::Continue;
        }
        let files_to_delete = self.similar_vectors.clone();
        self.delete_advanced_elements_and_add_to_messages(stop_flag, progress_sender, files_to_delete)
    }
}

impl DebugPrint for SimilarVideos {
    #[allow(clippy::print_stdout)]
    fn debug_print(&self) {
        if !cfg!(debug_assertions) {
            return;
        }

        println!("---------------DEBUG PRINT---------------");
        println!("Included directories - {:?}", self.common_data.directories.included_directories);
        self.debug_print_common();
        println!("-----------------------------------------");
    }
}

impl PrintResults for SimilarVideos {
    fn write_results<T: Write>(&self, writer: &mut T) -> std::io::Result<()> {
        if !self.similar_vectors.is_empty() {
            write!(writer, "{} videos which have similar friends\n\n", self.similar_vectors.len())?;

            for struct_similar in &self.similar_vectors {
                writeln!(writer, "Found {} videos which have similar friends", struct_similar.len())?;
                for file_entry in struct_similar {
                    writeln!(writer, "\"{}\" - {}", file_entry.path.to_string_lossy(), format_size(file_entry.size, BINARY))?;
                }
                writeln!(writer)?;
            }
        } else if !self.similar_referenced_vectors.is_empty() {
            write!(writer, "{} videos which have similar friends\n\n", self.similar_referenced_vectors.len())?;

            for (fe, struct_similar) in &self.similar_referenced_vectors {
                writeln!(writer, "Found {} videos which have similar friends", struct_similar.len())?;
                writeln!(writer)?;
                writeln!(writer, "\"{}\" - {}", fe.path.to_string_lossy(), format_size(fe.size, BINARY))?;
                for file_entry in struct_similar {
                    writeln!(writer, "\"{}\" - {}", file_entry.path.to_string_lossy(), format_size(file_entry.size, BINARY))?;
                }
                writeln!(writer)?;
            }
        } else {
            write!(writer, "Not found any similar videos.")?;
        }

        Ok(())
    }

    fn save_results_to_file_as_json(&self, file_name: &str, pretty_print: bool) -> std::io::Result<()> {
        if self.get_use_reference() {
            self.save_results_to_file_as_json_internal(file_name, &self.similar_referenced_vectors, pretty_print)
        } else {
            self.save_results_to_file_as_json_internal(file_name, &self.similar_vectors, pretty_print)
        }
    }
}

impl SimilarVideos {
    pub fn get_params(&self) -> &SimilarVideosParameters {
        &self.params
    }

    pub const fn get_similar_videos(&self) -> &Vec<Vec<VideosEntry>> {
        &self.similar_vectors
    }

    pub const fn get_information(&self) -> &Info {
        &self.information
    }

    pub fn get_similar_videos_referenced(&self) -> &Vec<(VideosEntry, Vec<VideosEntry>)> {
        &self.similar_referenced_vectors
    }

    pub fn get_number_of_base_duplicated_files(&self) -> usize {
        if self.common_data.use_reference_folders {
            self.similar_referenced_vectors.len()
        } else {
            self.similar_vectors.len()
        }
    }

    pub fn get_use_reference(&self) -> bool {
        self.common_data.use_reference_folders
    }
}
