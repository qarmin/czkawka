use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fs::{File, Metadata};
use std::io::Write;
use std::io::*;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread::sleep;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::{fs, mem, thread};

use crossbeam_channel::Receiver;
use ffmpeg_cmdline_utils::FfmpegErrorKind::FfmpegNotFound;
use humansize::{file_size_opts as options, FileSize};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use vid_dup_finder_lib::HashCreationErrorKind::DetermineVideo;
use vid_dup_finder_lib::{NormalizedTolerance, VideoHash};

use crate::common::{open_cache_folder, Common, LOOP_DURATION};
use crate::common_directory::Directories;
use crate::common_extensions::Extensions;
use crate::common_items::ExcludedItems;
use crate::common_messages::Messages;
use crate::common_traits::{DebugPrint, PrintResults, SaveResults};
use crate::flc;
use crate::localizer_core::generate_translation_hashmap;
use crate::similar_images::VIDEO_FILES_EXTENSIONS;

pub const MAX_TOLERANCE: i32 = 20;

#[derive(Debug)]
pub struct ProgressData {
    pub current_stage: u8,
    pub max_stage: u8,
    pub videos_checked: usize,
    pub videos_to_check: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FileEntry {
    pub path: PathBuf,
    pub size: u64,
    pub modified_date: u64,
    pub vhash: VideoHash,
    pub error: String,
}

/// Distance metric to use with the BK-tree.
struct Hamming;

impl bk_tree::Metric<Vec<u8>> for Hamming {
    fn distance(&self, a: &Vec<u8>, b: &Vec<u8>) -> u32 {
        hamming::distance_fast(a, b).unwrap() as u32
    }

    fn threshold_distance(&self, a: &Vec<u8>, b: &Vec<u8>, _threshold: u32) -> Option<u32> {
        Some(self.distance(a, b))
    }
}

/// Struct to store most basics info about all folder
pub struct SimilarVideos {
    information: Info,
    text_messages: Messages,
    directories: Directories,
    excluded_items: ExcludedItems,
    allowed_extensions: Extensions,
    similar_vectors: Vec<Vec<FileEntry>>,
    similar_referenced_vectors: Vec<(FileEntry, Vec<FileEntry>)>,
    recursive_search: bool,
    minimal_file_size: u64,
    maximal_file_size: u64,
    videos_hashes: BTreeMap<Vec<u8>, Vec<FileEntry>>,
    stopped_search: bool,
    videos_to_check: BTreeMap<String, FileEntry>,
    use_cache: bool,
    tolerance: i32,
    delete_outdated_cache: bool,
    exclude_videos_with_same_size: bool,
    use_reference_folders: bool,
    save_also_as_json: bool,
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

/// Method implementation for EmptyFolder
impl SimilarVideos {
    /// New function providing basics values
    pub fn new() -> Self {
        Self {
            information: Default::default(),
            text_messages: Messages::new(),
            directories: Directories::new(),
            excluded_items: Default::default(),
            allowed_extensions: Extensions::new(),
            similar_vectors: vec![],
            recursive_search: true,
            minimal_file_size: 1024 * 16,
            maximal_file_size: u64::MAX,
            videos_hashes: Default::default(),
            stopped_search: false,
            videos_to_check: Default::default(),
            use_cache: true,
            tolerance: 10,
            delete_outdated_cache: false,
            exclude_videos_with_same_size: false,
            use_reference_folders: false,
            similar_referenced_vectors: vec![],
            save_also_as_json: true,
        }
    }

    pub fn set_exclude_videos_with_same_size(&mut self, exclude_videos_with_same_size: bool) {
        self.exclude_videos_with_same_size = exclude_videos_with_same_size;
    }

    pub fn set_delete_outdated_cache(&mut self, delete_outdated_cache: bool) {
        self.delete_outdated_cache = delete_outdated_cache;
    }

    pub fn set_tolerance(&mut self, tolerance: i32) {
        assert!((0..=MAX_TOLERANCE).contains(&tolerance));
        self.tolerance = tolerance
    }
    pub fn set_save_also_as_json(&mut self, save_also_as_json: bool) {
        self.save_also_as_json = save_also_as_json;
    }

    pub fn get_stopped_search(&self) -> bool {
        self.stopped_search
    }

    pub const fn get_text_messages(&self) -> &Messages {
        &self.text_messages
    }

    pub fn set_allowed_extensions(&mut self, allowed_extensions: String) {
        self.allowed_extensions.set_allowed_extensions(allowed_extensions, &mut self.text_messages);
    }

    pub const fn get_similar_videos(&self) -> &Vec<Vec<FileEntry>> {
        &self.similar_vectors
    }

    pub const fn get_information(&self) -> &Info {
        &self.information
    }

    pub fn set_use_cache(&mut self, use_cache: bool) {
        self.use_cache = use_cache;
    }

    pub fn set_recursive_search(&mut self, recursive_search: bool) {
        self.recursive_search = recursive_search;
    }

    pub fn set_minimal_file_size(&mut self, minimal_file_size: u64) {
        self.minimal_file_size = match minimal_file_size {
            0 => 1,
            t => t,
        };
    }
    pub fn set_maximal_file_size(&mut self, maximal_file_size: u64) {
        self.maximal_file_size = match maximal_file_size {
            0 => 1,
            t => t,
        };
    }
    pub fn get_similar_videos_referenced(&self) -> &Vec<(FileEntry, Vec<FileEntry>)> {
        &self.similar_referenced_vectors
    }

    pub fn get_number_of_base_duplicated_files(&self) -> usize {
        if self.use_reference_folders {
            self.similar_referenced_vectors.len()
        } else {
            self.similar_vectors.len()
        }
    }

    pub fn get_use_reference(&self) -> bool {
        self.use_reference_folders
    }

    /// Public function used by CLI to search for empty folders
    pub fn find_similar_videos(&mut self, stop_receiver: Option<&Receiver<()>>, progress_sender: Option<&futures::channel::mpsc::UnboundedSender<ProgressData>>) {
        if !check_if_ffmpeg_is_installed() {
            self.text_messages.errors.push(flc!("core_ffmpeg_not_found"));
            self.text_messages.errors.push(flc!(
                "core_ffmpeg_missing_in_snap",
                generate_translation_hashmap(vec![("url", "https://github.com/snapcrafters/ffmpeg/issues/73".to_string())])
            ));
        } else {
            self.directories.optimize_directories(true, &mut self.text_messages);
            self.use_reference_folders = !self.directories.reference_directories.is_empty();
            if !self.check_for_similar_videos(stop_receiver, progress_sender) {
                self.stopped_search = true;
                return;
            }
            if !self.sort_videos(stop_receiver, progress_sender) {
                self.stopped_search = true;
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
    /// Parameter initial_checking for second check before deleting to be sure that checked folder is still empty
    fn check_for_similar_videos(&mut self, stop_receiver: Option<&Receiver<()>>, progress_sender: Option<&futures::channel::mpsc::UnboundedSender<ProgressData>>) -> bool {
        let start_time: SystemTime = SystemTime::now();
        let mut folders_to_check: Vec<PathBuf> = Vec::with_capacity(1024 * 2); // This should be small enough too not see to big difference and big enough to store most of paths without needing to resize vector

        if !self.allowed_extensions.using_custom_extensions() {
            self.allowed_extensions.extend_allowed_extensions(&VIDEO_FILES_EXTENSIONS);
        }

        // Add root folders for finding
        for id in &self.directories.included_directories {
            folders_to_check.push(id.clone());
        }

        //// PROGRESS THREAD START
        let progress_thread_run = Arc::new(AtomicBool::new(true));

        let atomic_file_counter = Arc::new(AtomicUsize::new(0));

        let progress_thread_handle = if let Some(progress_sender) = progress_sender {
            let progress_send = progress_sender.clone();
            let progress_thread_run = progress_thread_run.clone();
            let atomic_file_counter = atomic_file_counter.clone();
            thread::spawn(move || loop {
                progress_send
                    .unbounded_send(ProgressData {
                        current_stage: 0,
                        max_stage: 1,
                        videos_checked: atomic_file_counter.load(Ordering::Relaxed) as usize,
                        videos_to_check: 0,
                    })
                    .unwrap();
                if !progress_thread_run.load(Ordering::Relaxed) {
                    break;
                }
                sleep(Duration::from_millis(LOOP_DURATION as u64));
            })
        } else {
            thread::spawn(|| {})
        };
        //// PROGRESS THREAD END

        while !folders_to_check.is_empty() {
            if stop_receiver.is_some() && stop_receiver.unwrap().try_recv().is_ok() {
                // End thread which send info to gui
                progress_thread_run.store(false, Ordering::Relaxed);
                progress_thread_handle.join().unwrap();
                return false;
            }

            let segments: Vec<_> = folders_to_check
                .par_iter()
                .map(|current_folder| {
                    let mut dir_result = vec![];
                    let mut warnings = vec![];
                    let mut fe_result = vec![];
                    // Read current dir childrens
                    let read_dir = match fs::read_dir(&current_folder) {
                        Ok(t) => t,
                        Err(e) => {
                            warnings.push(flc!(
                                "core_cannot_open_dir",
                                generate_translation_hashmap(vec![("dir", current_folder.display().to_string()), ("reason", e.to_string())])
                            ));
                            return (dir_result, warnings, fe_result);
                        }
                    };

                    // Check every sub folder/file/link etc.
                    'dir: for entry in read_dir {
                        let entry_data = match entry {
                            Ok(t) => t,
                            Err(e) => {
                                warnings.push(flc!(
                                    "core_cannot_read_entry_dir",
                                    generate_translation_hashmap(vec![("dir", current_folder.display().to_string()), ("reason", e.to_string())])
                                ));
                                continue 'dir;
                            }
                        };
                        let metadata: Metadata = match entry_data.metadata() {
                            Ok(t) => t,
                            Err(e) => {
                                warnings.push(flc!(
                                    "core_cannot_read_metadata_dir",
                                    generate_translation_hashmap(vec![("dir", current_folder.display().to_string()), ("reason", e.to_string())])
                                ));
                                continue 'dir;
                            }
                        };
                        if metadata.is_dir() {
                            if !self.recursive_search {
                                continue 'dir;
                            }

                            let next_folder = current_folder.join(entry_data.file_name());
                            if self.directories.is_excluded(&next_folder) {
                                continue 'dir;
                            }

                            if self.excluded_items.is_excluded(&next_folder) {
                                continue 'dir;
                            }

                            dir_result.push(next_folder);
                        } else if metadata.is_file() {
                            atomic_file_counter.fetch_add(1, Ordering::Relaxed);

                            let file_name_lowercase: String = match entry_data.file_name().into_string() {
                                Ok(t) => t,
                                Err(_inspected) => {
                                    warnings.push(flc!(
                                        "core_file_not_utf8_name",
                                        generate_translation_hashmap(vec![("name", entry_data.path().display().to_string())])
                                    ));
                                    continue 'dir;
                                }
                            }
                            .to_lowercase();

                            if !self.allowed_extensions.matches_filename(&file_name_lowercase) {
                                continue 'dir;
                            }

                            // Checking files
                            if (self.minimal_file_size..=self.maximal_file_size).contains(&metadata.len()) {
                                let current_file_name = current_folder.join(entry_data.file_name());
                                if self.excluded_items.is_excluded(&current_file_name) {
                                    continue 'dir;
                                }

                                let fe: FileEntry = FileEntry {
                                    path: current_file_name.clone(),
                                    size: metadata.len(),
                                    modified_date: match metadata.modified() {
                                        Ok(t) => match t.duration_since(UNIX_EPOCH) {
                                            Ok(d) => d.as_secs(),
                                            Err(_inspected) => {
                                                warnings.push(flc!(
                                                    "core_file_modified_before_epoch",
                                                    generate_translation_hashmap(vec![("name", current_file_name.display().to_string())])
                                                ));
                                                0
                                            }
                                        },
                                        Err(e) => {
                                            warnings.push(flc!(
                                                "core_file_no_modification_date",
                                                generate_translation_hashmap(vec![("name", current_file_name.display().to_string()), ("reason", e.to_string())])
                                            ));
                                            0
                                        }
                                    },
                                    vhash: Default::default(),
                                    error: "".to_string(),
                                };

                                fe_result.push((current_file_name.to_string_lossy().to_string(), fe));
                            }
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
                self.text_messages.warnings.extend(warnings);
                for (name, fe) in fe_result {
                    self.videos_to_check.insert(name, fe);
                }
            }
        }

        // End thread which send info to gui
        progress_thread_run.store(false, Ordering::Relaxed);
        progress_thread_handle.join().unwrap();
        Common::print_time(start_time, SystemTime::now(), "check_for_similar_videos".to_string());
        true
    }

    fn sort_videos(&mut self, stop_receiver: Option<&Receiver<()>>, progress_sender: Option<&futures::channel::mpsc::UnboundedSender<ProgressData>>) -> bool {
        let hash_map_modification = SystemTime::now();

        let loaded_hash_map;

        let mut records_already_cached: BTreeMap<String, FileEntry> = Default::default();
        let mut non_cached_files_to_check: BTreeMap<String, FileEntry> = Default::default();

        if self.use_cache {
            loaded_hash_map = match load_hashes_from_file(&mut self.text_messages, self.delete_outdated_cache) {
                Some(t) => t,
                None => Default::default(),
            };

            for (name, file_entry) in &self.videos_to_check {
                #[allow(clippy::if_same_then_else)]
                if !loaded_hash_map.contains_key(name) {
                    // If loaded data doesn't contains current videos info
                    non_cached_files_to_check.insert(name.clone(), file_entry.clone());
                } else if file_entry.size != loaded_hash_map.get(name).unwrap().size || file_entry.modified_date != loaded_hash_map.get(name).unwrap().modified_date {
                    // When size or modification date of video changed, then it is clear that is different video
                    non_cached_files_to_check.insert(name.clone(), file_entry.clone());
                } else {
                    // Checking may be omitted when already there is entry with same size and modification date
                    records_already_cached.insert(name.clone(), loaded_hash_map.get(name).unwrap().clone());
                }
            }
        } else {
            loaded_hash_map = Default::default();
            mem::swap(&mut self.videos_to_check, &mut non_cached_files_to_check);
        }

        Common::print_time(
            hash_map_modification,
            SystemTime::now(),
            "sort_videos - reading data from cache and preparing them".to_string(),
        );
        let hash_map_modification = SystemTime::now();

        //// PROGRESS THREAD START
        let progress_thread_run = Arc::new(AtomicBool::new(true));

        let atomic_file_counter = Arc::new(AtomicUsize::new(0));

        let progress_thread_handle = if let Some(progress_sender) = progress_sender {
            let progress_send = progress_sender.clone();
            let progress_thread_run = progress_thread_run.clone();
            let atomic_file_counter = atomic_file_counter.clone();
            let videos_to_check = non_cached_files_to_check.len();
            thread::spawn(move || loop {
                progress_send
                    .unbounded_send(ProgressData {
                        current_stage: 1,
                        max_stage: 1,
                        videos_checked: atomic_file_counter.load(Ordering::Relaxed) as usize,
                        videos_to_check,
                    })
                    .unwrap();
                if !progress_thread_run.load(Ordering::Relaxed) {
                    break;
                }
                sleep(Duration::from_millis(LOOP_DURATION as u64));
            })
        } else {
            thread::spawn(|| {})
        };
        //// PROGRESS THREAD END
        let mut vec_file_entry: Vec<FileEntry> = non_cached_files_to_check
            .par_iter()
            .map(|file_entry| {
                atomic_file_counter.fetch_add(1, Ordering::Relaxed);
                if stop_receiver.is_some() && stop_receiver.unwrap().try_recv().is_ok() {
                    // This will not break
                    return None;
                }
                let mut file_entry = file_entry.1.clone();

                let vhash = match VideoHash::from_path(&file_entry.path) {
                    Ok(t) => t,
                    Err(e) => {
                        return {
                            file_entry.error = format!("Failed to hash file, reason {}", e);
                            Some(file_entry)
                        }
                    }
                };

                file_entry.vhash = vhash;

                Some(file_entry)
            })
            .while_some()
            .collect::<Vec<FileEntry>>();

        // End thread which send info to gui
        progress_thread_run.store(false, Ordering::Relaxed);
        progress_thread_handle.join().unwrap();

        Common::print_time(hash_map_modification, SystemTime::now(), "sort_videos - reading data from files in parallel".to_string());
        let hash_map_modification = SystemTime::now();

        // Just connect loaded results with already calculated hashes
        for (_name, file_entry) in records_already_cached {
            vec_file_entry.push(file_entry.clone());
        }

        let mut hashmap_with_file_entries: HashMap<String, FileEntry> = Default::default();
        let mut vector_of_hashes: Vec<VideoHash> = Vec::new();
        for file_entry in &vec_file_entry {
            // 0 means that images was not hashed correctly, e.g. could be improperly
            if file_entry.error.is_empty() {
                hashmap_with_file_entries.insert(file_entry.vhash.src_path().to_string_lossy().to_string(), file_entry.clone());
                vector_of_hashes.push(file_entry.vhash.clone());
            } else {
                self.text_messages.warnings.push(file_entry.error.clone());
            }
        }

        if self.use_cache {
            // Must save all results to file, old loaded from file with all currently counted results
            let mut all_results: BTreeMap<String, FileEntry> = loaded_hash_map;
            for file_entry in vec_file_entry {
                all_results.insert(file_entry.path.to_string_lossy().to_string(), file_entry);
            }
            save_hashes_to_file(&all_results, &mut self.text_messages, self.save_also_as_json);
        }

        Common::print_time(hash_map_modification, SystemTime::now(), "sort_videos - saving data to files".to_string());
        let hash_map_modification = SystemTime::now();

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

        if self.use_reference_folders {
            let mut similars_vector = Default::default();
            mem::swap(&mut self.similar_vectors, &mut similars_vector);
            let reference_directories = self.directories.reference_directories.clone();
            self.similar_referenced_vectors = similars_vector
                .into_iter()
                .filter_map(|vec_file_entry| {
                    let mut files_from_referenced_folders = Vec::new();
                    let mut normal_files = Vec::new();
                    for file_entry in vec_file_entry {
                        if reference_directories.iter().any(|e| file_entry.path.starts_with(&e)) {
                            files_from_referenced_folders.push(file_entry);
                        } else {
                            normal_files.push(file_entry);
                        }
                    }

                    if files_from_referenced_folders.is_empty() || normal_files.is_empty() {
                        None
                    } else {
                        Some((files_from_referenced_folders.pop().unwrap(), normal_files))
                    }
                })
                .collect::<Vec<(FileEntry, Vec<FileEntry>)>>();
        }

        if self.use_reference_folders {
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

        Common::print_time(hash_map_modification, SystemTime::now(), "sort_videos - selecting data from BtreeMap".to_string());

        // Clean unused data
        self.videos_hashes = Default::default();
        self.videos_to_check = Default::default();

        true
    }

    /// Set included dir which needs to be relative, exists etc.
    pub fn set_included_directory(&mut self, included_directory: Vec<PathBuf>) {
        self.directories.set_included_directory(included_directory, &mut self.text_messages);
    }

    pub fn set_reference_directory(&mut self, reference_directory: Vec<PathBuf>) {
        self.directories.set_reference_directory(reference_directory);
    }

    pub fn set_excluded_directory(&mut self, excluded_directory: Vec<PathBuf>) {
        self.directories.set_excluded_directory(excluded_directory, &mut self.text_messages);
    }

    pub fn set_excluded_items(&mut self, excluded_items: Vec<String>) {
        self.excluded_items.set_excluded_items(excluded_items, &mut self.text_messages);
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
        println!("Included directories - {:?}", self.directories.included_directories);
        println!("-----------------------------------------");
    }
}

impl SaveResults for SimilarVideos {
    fn save_results_to_file(&mut self, file_name: &str) -> bool {
        let start_time: SystemTime = SystemTime::now();
        let file_name: String = match file_name {
            "" => "results.txt".to_string(),
            k => k.to_string(),
        };

        let file_handler = match File::create(&file_name) {
            Ok(t) => t,
            Err(e) => {
                self.text_messages.errors.push(format!("Failed to create file {}, reason {}", file_name, e));
                return false;
            }
        };
        let mut writer = BufWriter::new(file_handler);

        if let Err(e) = writeln!(
            writer,
            "Results of searching {:?} with excluded directories {:?} and excluded items {:?}",
            self.directories.included_directories, self.directories.excluded_directories, self.excluded_items.items
        ) {
            self.text_messages.errors.push(format!("Failed to save results to file {}, reason {}", file_name, e));
            return false;
        }

        if !self.similar_vectors.is_empty() {
            write!(writer, "{} videos which have similar friends\n\n", self.similar_vectors.len()).unwrap();

            for struct_similar in self.similar_vectors.iter() {
                writeln!(writer, "Found {} videos which have similar friends", self.similar_vectors.len()).unwrap();
                for file_entry in struct_similar {
                    writeln!(writer, "{} - {}", file_entry.path.display(), file_entry.size.file_size(options::BINARY).unwrap(),).unwrap();
                }
                writeln!(writer).unwrap();
            }
        } else {
            write!(writer, "Not found any similar videos.").unwrap();
        }

        Common::print_time(start_time, SystemTime::now(), "save_results_to_file".to_string());
        true
    }
}

impl PrintResults for SimilarVideos {
    fn print_results(&self) {
        if !self.similar_vectors.is_empty() {
            println!("Found {} videos which have similar friends", self.similar_vectors.len());

            for vec_file_entry in &self.similar_vectors {
                for file_entry in vec_file_entry {
                    println!("{} - {}", file_entry.path.display(), file_entry.size.file_size(options::BINARY).unwrap());
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
    let vid = "999999999999999999.txt";
    if let Err(DetermineVideo {
        src_path: _a,
        error: FfmpegNotFound,
    }) = VideoHash::from_path(&vid)
    {
        return false;
    }
    true
}
