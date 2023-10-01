use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fs::{DirEntry, File, Metadata};
use std::io::{Write, *};
use std::mem;
use std::path::{Path, PathBuf};
use std::sync::atomic::Ordering;

use crossbeam_channel::Receiver;
use ffmpeg_cmdline_utils::FfmpegErrorKind::FfmpegNotFound;
use futures::channel::mpsc::UnboundedSender;
use humansize::{format_size, BINARY};
use log::{debug, info};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use vid_dup_finder_lib::HashCreationErrorKind::DetermineVideo;
use vid_dup_finder_lib::{NormalizedTolerance, VideoHash};

use crate::common::{check_folder_children, open_cache_folder, prepare_thread_handler_common, send_info_and_wait_for_ending_all_threads, VIDEO_FILES_EXTENSIONS};
use crate::common_dir_traversal::{common_get_entry_data_metadata, common_read_dir, get_lowercase_name, get_modified_time, CheckingMethod, ProgressData, ToolType};

use crate::common_messages::Messages;
use crate::common_tool::{CommonData, CommonToolData};
use crate::common_traits::{DebugPrint, PrintResults, ResultEntry, SaveResults};
use crate::flc;
use crate::localizer_core::generate_translation_hashmap;

pub const MAX_TOLERANCE: i32 = 20;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FileEntry {
    pub path: PathBuf,
    pub size: u64,
    pub modified_date: u64,
    pub vhash: VideoHash,
    pub error: String,
}
impl ResultEntry for FileEntry {
    fn get_path(&self) -> &Path {
        &self.path
    }
}

/// Distance metric to use with the BK-tree.
struct Hamming;

impl bk_tree::Metric<Vec<u8>> for Hamming {
    #[inline]
    fn distance(&self, a: &Vec<u8>, b: &Vec<u8>) -> u32 {
        hamming::distance_fast(a, b).unwrap() as u32
    }

    #[inline]
    fn threshold_distance(&self, a: &Vec<u8>, b: &Vec<u8>, _threshold: u32) -> Option<u32> {
        Some(self.distance(a, b))
    }
}

/// Struct to store most basics info about all folder
pub struct SimilarVideos {
    common_data: CommonToolData,
    information: Info,
    similar_vectors: Vec<Vec<FileEntry>>,
    similar_referenced_vectors: Vec<(FileEntry, Vec<FileEntry>)>,
    videos_hashes: BTreeMap<Vec<u8>, Vec<FileEntry>>,
    videos_to_check: BTreeMap<String, FileEntry>,
    tolerance: i32,
    exclude_videos_with_same_size: bool,
}

impl CommonData for SimilarVideos {
    fn get_cd(&self) -> &CommonToolData {
        &self.common_data
    }
    fn get_cd_mut(&mut self) -> &mut CommonToolData {
        &mut self.common_data
    }
}

/// Info struck with helpful information's about results
#[derive(Default)]
pub struct Info {
    pub number_of_duplicates: usize,
    pub number_of_groups: u64,
}

impl Info {
    pub fn new() -> Self {
        Default::default()
    }
}

/// Method implementation for `EmptyFolder`
impl SimilarVideos {
    /// New function providing basics values

    pub fn new() -> Self {
        Self {
            common_data: CommonToolData::new(ToolType::SimilarVideos),
            information: Default::default(),
            similar_vectors: vec![],
            videos_hashes: Default::default(),
            videos_to_check: Default::default(),
            tolerance: 10,
            exclude_videos_with_same_size: false,
            similar_referenced_vectors: vec![],
        }
    }

    pub fn set_exclude_videos_with_same_size(&mut self, exclude_videos_with_same_size: bool) {
        self.exclude_videos_with_same_size = exclude_videos_with_same_size;
    }

    pub fn set_tolerance(&mut self, tolerance: i32) {
        assert!((0..=MAX_TOLERANCE).contains(&tolerance));
        self.tolerance = tolerance;
    }

    pub const fn get_similar_videos(&self) -> &Vec<Vec<FileEntry>> {
        &self.similar_vectors
    }

    pub const fn get_information(&self) -> &Info {
        &self.information
    }

    pub fn get_similar_videos_referenced(&self) -> &Vec<(FileEntry, Vec<FileEntry>)> {
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

    /// Public function used by CLI to search for empty folders
    pub fn find_similar_videos(&mut self, stop_receiver: Option<&Receiver<()>>, progress_sender: Option<&UnboundedSender<ProgressData>>) {
        info!("Starting finding similar videos");
        if !check_if_ffmpeg_is_installed() {
            self.common_data.text_messages.errors.push(flc!("core_ffmpeg_not_found"));
            #[cfg(target_os = "windows")]
            self.common_data.text_messages.errors.push(flc!("core_ffmpeg_not_found_windows"));
            #[cfg(target_os = "linux")]
            self.common_data.text_messages.errors.push(flc!(
                "core_ffmpeg_missing_in_snap",
                generate_translation_hashmap(vec![("url", "https://github.com/snapcrafters/ffmpeg/issues/73".to_string())])
            ));
        } else {
            self.optimize_dirs_before_start();
            self.common_data.use_reference_folders = !self.common_data.directories.reference_directories.is_empty();
            if !self.check_for_similar_videos(stop_receiver, progress_sender) {
                self.common_data.stopped_search = true;
                return;
            }
            if !self.sort_videos(stop_receiver, progress_sender) {
                self.common_data.stopped_search = true;
                return;
            }
            // if self.delete_folders {
            //     self.delete_empty_folders();
            // }
        }
        self.debug_print();
    }

    // pub fn set_delete_folder(&mut self, delete_folder: bool) {
    //     self.delete_folders = delete_folder;
    // }

    /// Function to check if folder are empty.
    /// Parameter `initial_checking` for second check before deleting to be sure that checked folder is still empty
    fn check_for_similar_videos(&mut self, stop_receiver: Option<&Receiver<()>>, progress_sender: Option<&UnboundedSender<ProgressData>>) -> bool {
        debug!("check_for_similar_videos - start");
        let mut folders_to_check: Vec<PathBuf> = Vec::with_capacity(1024 * 2); // This should be small enough too not see to big difference and big enough to store most of paths without needing to resize vector

        if !self.common_data.allowed_extensions.using_custom_extensions() {
            self.common_data.allowed_extensions.extend_allowed_extensions(VIDEO_FILES_EXTENSIONS);
        } else {
            self.common_data.allowed_extensions.validate_allowed_extensions(VIDEO_FILES_EXTENSIONS);
            if !self.common_data.allowed_extensions.using_custom_extensions() {
                return true;
            }
        }

        // Add root folders for finding
        for id in &self.common_data.directories.included_directories {
            folders_to_check.push(id.clone());
        }

        let (progress_thread_handle, progress_thread_run, atomic_counter, _check_was_stopped) =
            prepare_thread_handler_common(progress_sender, 0, 1, 0, CheckingMethod::None, self.common_data.tool_type);

        while !folders_to_check.is_empty() {
            if stop_receiver.is_some() && stop_receiver.unwrap().try_recv().is_ok() {
                send_info_and_wait_for_ending_all_threads(&progress_thread_run, progress_thread_handle);
                return false;
            }

            let segments: Vec<_> = folders_to_check
                .par_iter()
                .map(|current_folder| {
                    let mut dir_result = vec![];
                    let mut warnings = vec![];
                    let mut fe_result = vec![];

                    let Some(read_dir) = common_read_dir(current_folder, &mut warnings) else {
                        return (dir_result, warnings, fe_result);
                    };

                    // Check every sub folder/file/link etc.
                    for entry in read_dir {
                        let Some((entry_data, metadata)) = common_get_entry_data_metadata(&entry, &mut warnings, current_folder) else {
                            continue;
                        };

                        if metadata.is_dir() {
                            check_folder_children(
                                &mut dir_result,
                                &mut warnings,
                                current_folder,
                                entry_data,
                                self.common_data.recursive_search,
                                &self.common_data.directories,
                                &self.common_data.excluded_items,
                            );
                        } else if metadata.is_file() {
                            atomic_counter.fetch_add(1, Ordering::Relaxed);
                            self.add_video_file_entry(&metadata, entry_data, &mut fe_result, &mut warnings, current_folder);
                        }
                    }
                    (dir_result, warnings, fe_result)
                })
                .collect();

            // Advance the frontier
            folders_to_check.clear();

            // Process collected data
            for (segment, warnings, fe_result) in segments {
                folders_to_check.extend(segment);
                self.common_data.text_messages.warnings.extend(warnings);
                for (name, fe) in fe_result {
                    self.videos_to_check.insert(name, fe);
                }
            }
        }

        send_info_and_wait_for_ending_all_threads(&progress_thread_run, progress_thread_handle);

        debug!("check_for_similar_videos - end");
        true
    }

    fn add_video_file_entry(&self, metadata: &Metadata, entry_data: &DirEntry, fe_result: &mut Vec<(String, FileEntry)>, warnings: &mut Vec<String>, current_folder: &Path) {
        let Some(file_name_lowercase) = get_lowercase_name(entry_data, warnings) else {
            return;
        };

        if !self.common_data.allowed_extensions.matches_filename(&file_name_lowercase) {
            return;
        }

        // Checking files
        if (self.common_data.minimal_file_size..=self.common_data.maximal_file_size).contains(&metadata.len()) {
            let current_file_name = current_folder.join(entry_data.file_name());
            if self.common_data.excluded_items.is_excluded(&current_file_name) {
                return;
            }
            let current_file_name_str = current_file_name.to_string_lossy().to_string();

            let fe: FileEntry = FileEntry {
                path: current_file_name.clone(),
                size: metadata.len(),
                modified_date: get_modified_time(metadata, warnings, &current_file_name, false),
                vhash: Default::default(),
                error: String::new(),
            };

            fe_result.push((current_file_name_str, fe));
        }
    }

    fn load_cache_at_start(&mut self) -> (BTreeMap<String, FileEntry>, BTreeMap<String, FileEntry>, BTreeMap<String, FileEntry>) {
        debug!("load_cache_at_start - start, use cache: {}", self.common_data.use_cache);
        let loaded_hash_map;
        let mut records_already_cached: BTreeMap<String, FileEntry> = Default::default();
        let mut non_cached_files_to_check: BTreeMap<String, FileEntry> = Default::default();

        if self.common_data.use_cache {
            loaded_hash_map = match load_hashes_from_file(&mut self.common_data.text_messages, self.common_data.delete_outdated_cache) {
                Some(t) => t,
                None => Default::default(),
            };

            for (name, file_entry) in &self.videos_to_check {
                if !loaded_hash_map.contains_key(name) {
                    // If loaded data doesn't contains current videos info
                    non_cached_files_to_check.insert(name.clone(), file_entry.clone());
                } else {
                    let loaded_item = loaded_hash_map.get(name).unwrap();
                    if file_entry.size != loaded_item.size || file_entry.modified_date != loaded_item.modified_date {
                        // When size or modification date of video changed, then it is clear that is different video
                        non_cached_files_to_check.insert(name.clone(), file_entry.clone());
                    } else {
                        // Checking may be omitted when already there is entry with same size and modification date
                        records_already_cached.insert(name.clone(), loaded_item.clone());
                    }
                }
            }
        } else {
            loaded_hash_map = Default::default();
            mem::swap(&mut self.videos_to_check, &mut non_cached_files_to_check);
        }
        debug!("load_cache_at_start - end");
        (loaded_hash_map, records_already_cached, non_cached_files_to_check)
    }

    fn sort_videos(&mut self, stop_receiver: Option<&Receiver<()>>, progress_sender: Option<&UnboundedSender<ProgressData>>) -> bool {
        debug!("sort_videos - start");
        let (loaded_hash_map, records_already_cached, non_cached_files_to_check) = self.load_cache_at_start();

        let (progress_thread_handle, progress_thread_run, atomic_counter, check_was_stopped) =
            prepare_thread_handler_common(progress_sender, 1, 1, non_cached_files_to_check.len(), CheckingMethod::None, self.common_data.tool_type);

        let mut vec_file_entry: Vec<FileEntry> = non_cached_files_to_check
            .par_iter()
            .map(|file_entry| {
                atomic_counter.fetch_add(1, Ordering::Relaxed);
                if stop_receiver.is_some() && stop_receiver.unwrap().try_recv().is_ok() {
                    check_was_stopped.store(true, Ordering::Relaxed);
                    return None;
                }
                let mut file_entry = file_entry.1.clone();

                let vhash = match VideoHash::from_path(&file_entry.path) {
                    Ok(t) => t,
                    Err(e) => {
                        return {
                            file_entry.error = format!("Failed to hash file, reason {e}");
                            Some(file_entry)
                        };
                    }
                };

                file_entry.vhash = vhash;

                Some(file_entry)
            })
            .while_some()
            .collect::<Vec<FileEntry>>();

        send_info_and_wait_for_ending_all_threads(&progress_thread_run, progress_thread_handle);

        // Just connect loaded results with already calculated hashes
        vec_file_entry.extend(records_already_cached.into_values());

        let mut hashmap_with_file_entries: HashMap<String, FileEntry> = Default::default();
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
            return false;
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

        debug!("sort_videos - end");
        true
    }
    fn save_cache(&mut self, vec_file_entry: Vec<FileEntry>, loaded_hash_map: BTreeMap<String, FileEntry>) {
        debug!("save_cache - start, use cache: {}", self.common_data.use_cache);
        if self.common_data.use_cache {
            // Must save all results to file, old loaded from file with all currently counted results
            let mut all_results: BTreeMap<String, FileEntry> = loaded_hash_map;
            for file_entry in vec_file_entry {
                all_results.insert(file_entry.path.to_string_lossy().to_string(), file_entry);
            }
            save_hashes_to_file(&all_results, &mut self.common_data.text_messages, self.common_data.save_also_as_json);
        }
        debug!("save_cache - end");
    }

    fn match_groups_of_videos(&mut self, vector_of_hashes: Vec<VideoHash>, hashmap_with_file_entries: &HashMap<String, FileEntry>) {
        debug!("match_groups_of_videos - start");
        let match_group = vid_dup_finder_lib::search(vector_of_hashes, NormalizedTolerance::new(self.tolerance as f64 / 100.0f64));
        let mut collected_similar_videos: Vec<Vec<FileEntry>> = Default::default();
        for i in match_group {
            let mut temp_vector: Vec<FileEntry> = Vec::new();
            let mut bt_size: BTreeSet<u64> = Default::default();
            for j in i.duplicates() {
                let file_entry = hashmap_with_file_entries.get(&j.to_string_lossy().to_string()).unwrap();
                if self.exclude_videos_with_same_size {
                    if !bt_size.contains(&file_entry.size) {
                        bt_size.insert(file_entry.size);
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
        debug!("match_groups_of_videos - end");
    }

    fn remove_from_reference_folders(&mut self) {
        debug!("remove_from_reference_folders - start, use reference folders: {}", self.common_data.use_reference_folders);
        if self.common_data.use_reference_folders {
            self.similar_referenced_vectors = mem::take(&mut self.similar_vectors)
                .into_iter()
                .filter_map(|vec_file_entry| {
                    let (mut files_from_referenced_folders, normal_files): (Vec<_>, Vec<_>) = vec_file_entry
                        .into_iter()
                        .partition(|e| self.common_data.directories.is_in_referenced_directory(e.get_path()));

                    if files_from_referenced_folders.is_empty() || normal_files.is_empty() {
                        None
                    } else {
                        Some((files_from_referenced_folders.pop().unwrap(), normal_files))
                    }
                })
                .collect::<Vec<(FileEntry, Vec<FileEntry>)>>();
        }
        debug!("remove_from_reference_folders - end");
    }
}

impl Default for SimilarVideos {
    fn default() -> Self {
        Self::new()
    }
}

impl DebugPrint for SimilarVideos {
    #[allow(dead_code)]
    #[allow(unreachable_code)]
    fn debug_print(&self) {
        #[cfg(not(debug_assertions))]
        {
            return;
        }

        println!("---------------DEBUG PRINT---------------");
        println!("Included directories - {:?}", self.common_data.directories.included_directories);
        println!("-----------------------------------------");
    }
}

impl SaveResults for SimilarVideos {
    fn save_results_to_file(&mut self, file_name: &str) -> bool {
        let file_name: String = match file_name {
            "" => "results.txt".to_string(),
            k => k.to_string(),
        };

        let file_handler = match File::create(&file_name) {
            Ok(t) => t,
            Err(e) => {
                self.common_data.text_messages.errors.push(format!("Failed to create file {file_name}, reason {e}"));
                return false;
            }
        };
        let mut writer = BufWriter::new(file_handler);

        if let Err(e) = writeln!(
            writer,
            "Results of searching {:?} with excluded directories {:?} and excluded items {:?}",
            self.common_data.directories.included_directories, self.common_data.directories.excluded_directories, self.common_data.excluded_items.items
        ) {
            self.common_data
                .text_messages
                .errors
                .push(format!("Failed to save results to file {file_name}, reason {e}"));
            return false;
        }

        if !self.similar_vectors.is_empty() {
            write!(writer, "{} videos which have similar friends\n\n", self.similar_vectors.len()).unwrap();

            for struct_similar in &self.similar_vectors {
                writeln!(writer, "Found {} videos which have similar friends", struct_similar.len()).unwrap();
                for file_entry in struct_similar {
                    writeln!(writer, "{} - {}", file_entry.path.display(), format_size(file_entry.size, BINARY)).unwrap();
                }
                writeln!(writer).unwrap();
            }
        } else {
            write!(writer, "Not found any similar videos.").unwrap();
        }

        true
    }
}

impl PrintResults for SimilarVideos {
    fn print_results(&self) {
        if !self.similar_vectors.is_empty() {
            println!("Found {} videos which have similar friends", self.similar_vectors.len());

            for vec_file_entry in &self.similar_vectors {
                for file_entry in vec_file_entry {
                    println!("{} - {}", file_entry.path.display(), format_size(file_entry.size, BINARY));
                }
                println!();
            }
        }
    }
}

pub fn save_hashes_to_file(hashmap: &BTreeMap<String, FileEntry>, text_messages: &mut Messages, save_also_as_json: bool) {
    if let Some(((file_handler, cache_file), (file_handler_json, cache_file_json))) = open_cache_folder(&get_cache_file(), true, save_also_as_json, &mut text_messages.warnings) {
        {
            let writer = BufWriter::new(file_handler.unwrap()); // Unwrap because cannot fail here
            if let Err(e) = bincode::serialize_into(writer, hashmap) {
                text_messages
                    .warnings
                    .push(format!("Cannot write data to cache file {}, reason {}", cache_file.display(), e));
                return;
            }
        }
        if save_also_as_json {
            if let Some(file_handler_json) = file_handler_json {
                let writer = BufWriter::new(file_handler_json);
                if let Err(e) = serde_json::to_writer(writer, hashmap) {
                    text_messages
                        .warnings
                        .push(format!("Cannot write data to cache file {}, reason {}", cache_file_json.display(), e));
                    return;
                }
            }
        }

        text_messages.messages.push(format!("Properly saved to file {} cache entries.", hashmap.len()));
    }
}

pub fn load_hashes_from_file(text_messages: &mut Messages, delete_outdated_cache: bool) -> Option<BTreeMap<String, FileEntry>> {
    if let Some(((file_handler, cache_file), (file_handler_json, cache_file_json))) = open_cache_folder(&get_cache_file(), false, true, &mut text_messages.warnings) {
        let mut hashmap_loaded_entries: BTreeMap<String, FileEntry>;
        if let Some(file_handler) = file_handler {
            let reader = BufReader::new(file_handler);
            hashmap_loaded_entries = match bincode::deserialize_from(reader) {
                Ok(t) => t,
                Err(e) => {
                    text_messages
                        .warnings
                        .push(format!("Failed to load data from cache file {}, reason {}", cache_file.display(), e));
                    return None;
                }
            };
        } else {
            let reader = BufReader::new(file_handler_json.unwrap()); // Unwrap cannot fail, because at least one file must be valid
            hashmap_loaded_entries = match serde_json::from_reader(reader) {
                Ok(t) => t,
                Err(e) => {
                    text_messages
                        .warnings
                        .push(format!("Failed to load data from cache file {}, reason {}", cache_file_json.display(), e));
                    return None;
                }
            };
        }

        // Don't load cache data if destination file not exists
        if delete_outdated_cache {
            hashmap_loaded_entries.retain(|src_path, _file_entry| Path::new(src_path).exists());
        }

        text_messages.messages.push(format!("Properly loaded {} cache entries.", hashmap_loaded_entries.len()));

        return Some(hashmap_loaded_entries);
    }
    None
}

fn get_cache_file() -> String {
    "cache_similar_videos.bin".to_string()
}

pub fn check_if_ffmpeg_is_installed() -> bool {
    let vid = "9999czekoczekoczekolada999.txt";
    if let Err(DetermineVideo {
        src_path: _a,
        error: FfmpegNotFound,
    }) = VideoHash::from_path(vid)
    {
        return false;
    }
    true
}
