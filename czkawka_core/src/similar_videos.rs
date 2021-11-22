use crate::common::Common;
use crate::common_directory::Directories;
use crate::common_extensions::Extensions;
use crate::common_items::ExcludedItems;
use crate::common_messages::Messages;
use crate::common_traits::{DebugPrint, PrintResults, SaveResults};
use crossbeam_channel::Receiver;
use directories_next::ProjectDirs;
use ffmpeg_cmdline_utils::FfmpegErrorKind::FfmpegNotFound;
use humansize::{file_size_opts as options, FileSize};
use rayon::prelude::*;
use std::collections::{BTreeMap, HashMap};
use std::fs::OpenOptions;
use std::fs::{File, Metadata};
use std::io::Write;
use std::io::*;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread::sleep;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::{fs, mem, thread};
use vid_dup_finder_lib::HashCreationErrorKind::DetermineVideo;
use vid_dup_finder_lib::{NormalizedTolerance, VideoHash};

pub const MAX_TOLERANCE: i32 = 20;

#[derive(Debug)]
pub struct ProgressData {
    pub current_stage: u8,
    pub max_stage: u8,
    pub videos_checked: usize,
    pub videos_to_check: usize,
}

#[derive(Clone, Debug)]
pub struct FileEntry {
    pub path: PathBuf,
    pub size: u64,
    pub modified_date: u64,
    pub vhash: VideoHash,
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
    recursive_search: bool,
    minimal_file_size: u64,
    maximal_file_size: u64,
    videos_hashes: BTreeMap<Vec<u8>, Vec<FileEntry>>,
    stopped_search: bool,
    videos_to_check: BTreeMap<String, FileEntry>,
    use_cache: bool,
    tolerance: i32,
}

/// Info struck with helpful information's about results
#[derive(Default)]
pub struct Info {
    pub number_of_removed_files: usize,
    pub number_of_failed_to_remove_files: usize,
    pub gained_space: u64,
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
        }
    }

    pub fn set_tolerance(&mut self, tolerance: i32) {
        assert!((0..=MAX_TOLERANCE).contains(&tolerance));
        self.tolerance = tolerance
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

    /// Public function used by CLI to search for empty folders
    pub fn find_similar_videos(&mut self, stop_receiver: Option<&Receiver<()>>, progress_sender: Option<&futures::channel::mpsc::UnboundedSender<ProgressData>>) {
        if !check_if_ffmpeg_is_installed() {
            self.text_messages.errors.push("Cannot find proper installation of FFmpeg.".to_string());
        } else {
            self.directories.optimize_directories(true, &mut self.text_messages);
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

        // Add root folders for finding
        for id in &self.directories.included_directories {
            folders_to_check.push(id.clone());
        }

        //// PROGRESS THREAD START
        const LOOP_DURATION: u32 = 200; //in ms
        let progress_thread_run = Arc::new(AtomicBool::new(true));

        let atomic_file_counter = Arc::new(AtomicUsize::new(0));

        let progress_thread_handle;
        if let Some(progress_sender) = progress_sender {
            let progress_send = progress_sender.clone();
            let progress_thread_run = progress_thread_run.clone();
            let atomic_file_counter = atomic_file_counter.clone();
            progress_thread_handle = thread::spawn(move || loop {
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
            });
        } else {
            progress_thread_handle = thread::spawn(|| {});
        }
        //// PROGRESS THREAD END

        while !folders_to_check.is_empty() {
            if stop_receiver.is_some() && stop_receiver.unwrap().try_recv().is_ok() {
                // End thread which send info to gui
                progress_thread_run.store(false, Ordering::Relaxed);
                progress_thread_handle.join().unwrap();
                return false;
            }
            let current_folder = folders_to_check.pop().unwrap();

            // Read current dir, if permission are denied just go to next
            let read_dir = match fs::read_dir(&current_folder) {
                Ok(t) => t,
                Err(e) => {
                    self.text_messages.warnings.push(format!("Cannot open dir {}, reason {}", current_folder.display(), e));
                    continue;
                } // Permissions denied
            };

            // Check every sub folder/file/link etc.
            'dir: for entry in read_dir {
                let entry_data = match entry {
                    Ok(t) => t,
                    Err(e) => {
                        self.text_messages.warnings.push(format!("Cannot read entry in dir {}, reason {}", current_folder.display(), e));
                        continue;
                    } //Permissions denied
                };
                let metadata: Metadata = match entry_data.metadata() {
                    Ok(t) => t,
                    Err(e) => {
                        self.text_messages.warnings.push(format!("Cannot read metadata in dir {}, reason {}", current_folder.display(), e));
                        continue;
                    } //Permissions denied
                };
                if metadata.is_dir() {
                    if !self.recursive_search {
                        continue;
                    }

                    let next_folder = current_folder.join(entry_data.file_name());
                    if self.directories.is_excluded(&next_folder) {
                        continue 'dir;
                    }

                    if self.excluded_items.is_excluded(&next_folder) {
                        continue 'dir;
                    }

                    folders_to_check.push(next_folder);
                } else if metadata.is_file() {
                    atomic_file_counter.fetch_add(1, Ordering::Relaxed);

                    let file_name_lowercase: String = match entry_data.file_name().into_string() {
                        Ok(t) => t,
                        Err(_inspected) => {
                            println!("File {:?} has not valid UTF-8 name", entry_data);
                            continue 'dir;
                        }
                    }
                    .to_lowercase();

                    if !self.allowed_extensions.file_extensions.is_empty() {
                        let allowed = self.allowed_extensions.file_extensions.iter().any(|e| file_name_lowercase.ends_with((".".to_string() + e.to_lowercase().as_str()).as_str()));
                        if !allowed {
                            // Not an allowed extension, ignore it.
                            continue 'dir;
                        }
                    }

                    // Checking allowed video extensions
                    let allowed_video_extensions = [".mp4", ".mpv", ".flv", ".mp4a", ".webm", ".mpg", ".mp2", ".mpeg", ".m4p", ".m4v", ".avi", ".wmv", ".qt", ".mov", ".swf", ".mkv"];
                    if !allowed_video_extensions.iter().any(|e| file_name_lowercase.ends_with(e)) {
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
                                        self.text_messages.warnings.push(format!("File {} seems to be modified before Unix Epoch.", current_file_name.display()));
                                        0
                                    }
                                },
                                Err(e) => {
                                    self.text_messages.warnings.push(format!("Unable to get modification date from file {}, reason {}", current_file_name.display(), e));
                                    0
                                } // Permissions Denied
                            },
                            vhash: Default::default(),
                        };

                        self.videos_to_check.insert(current_file_name.to_string_lossy().to_string(), fe);
                    }
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
            loaded_hash_map = match load_hashes_from_file(&mut self.text_messages) {
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

        Common::print_time(hash_map_modification, SystemTime::now(), "sort_videos - reading data from cache and preparing them".to_string());
        let hash_map_modification = SystemTime::now();

        //// PROGRESS THREAD START
        const LOOP_DURATION: u32 = 200; //in ms
        let progress_thread_run = Arc::new(AtomicBool::new(true));

        let atomic_file_counter = Arc::new(AtomicUsize::new(0));

        let progress_thread_handle;
        if let Some(progress_sender) = progress_sender {
            let progress_send = progress_sender.clone();
            let progress_thread_run = progress_thread_run.clone();
            let atomic_file_counter = atomic_file_counter.clone();
            let videos_to_check = non_cached_files_to_check.len();
            progress_thread_handle = thread::spawn(move || loop {
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
            });
        } else {
            progress_thread_handle = thread::spawn(|| {});
        }
        //// PROGRESS THREAD END
        let old_vec_file_entry: Vec<std::result::Result<FileEntry, String>> = non_cached_files_to_check
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
                    Err(e) => return Some(Err(format!("Failed to hash file, {}", e))),
                };

                file_entry.vhash = vhash;

                Some(Ok(file_entry))
            })
            .while_some()
            .collect::<Vec<std::result::Result<FileEntry, String>>>();

        // End thread which send info to gui
        progress_thread_run.store(false, Ordering::Relaxed);
        progress_thread_handle.join().unwrap();

        let mut vec_file_entry = Vec::new();
        for result in old_vec_file_entry {
            match result {
                Ok(t) => vec_file_entry.push(t),
                Err(e) => {
                    self.text_messages.errors.push(e);
                }
            }
        }

        Common::print_time(hash_map_modification, SystemTime::now(), "sort_videos - reading data from files in parallel".to_string());
        let hash_map_modification = SystemTime::now();

        // Just connect loaded results with already calculated hashes
        for (_name, file_entry) in records_already_cached {
            vec_file_entry.push(file_entry.clone());
        }

        let mut hashmap_with_file_entries: HashMap<String, FileEntry> = Default::default();
        let mut vector_of_hashes: Vec<VideoHash> = Vec::new();
        for i in &vec_file_entry {
            hashmap_with_file_entries.insert(i.vhash.src_path().to_string_lossy().to_string(), i.clone());
            vector_of_hashes.push(i.vhash.clone());
        }

        if self.use_cache {
            // Must save all results to file, old loaded from file with all currently counted results
            let mut all_results: BTreeMap<String, FileEntry> = loaded_hash_map;
            for file_entry in vec_file_entry {
                all_results.insert(file_entry.path.to_string_lossy().to_string(), file_entry);
            }
            save_hashes_to_file(&all_results, &mut self.text_messages);
        }

        Common::print_time(hash_map_modification, SystemTime::now(), "sort_videos - saving data to files".to_string());
        let hash_map_modification = SystemTime::now();

        let match_group = vid_dup_finder_lib::search(vector_of_hashes, NormalizedTolerance::new(self.tolerance as f64 / 100.0f64));

        let mut collected_similar_videos: Vec<Vec<FileEntry>> = Default::default();
        for i in match_group {
            let mut temp_vector: Vec<FileEntry> = Vec::new();
            for j in i.duplicates() {
                temp_vector.push(hashmap_with_file_entries.get(&j.to_string_lossy().to_string()).unwrap().clone());
            }
            assert!(temp_vector.len() > 1);
            collected_similar_videos.push(temp_vector);
        }

        self.similar_vectors = collected_similar_videos;

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

fn save_hashes_to_file(hashmap: &BTreeMap<String, FileEntry>, text_messages: &mut Messages) {
    if let Some(proj_dirs) = ProjectDirs::from("pl", "Qarmin", "Czkawka") {
        // Lin: /home/username/.cache/czkawka
        // Win: C:\Users\Username\AppData\Local\Qarmin\Czkawka\cache
        // Mac: /Users/Username/Library/Caches/pl.Qarmin.Czkawka

        // Saves data
        // path//file_size//modified_date//num_frames//duration//hash1//hash2 etc.
        // number of hashes is equal to HASH_QWORDS(19 at this moment)

        let cache_dir = PathBuf::from(proj_dirs.cache_dir());
        if cache_dir.exists() {
            if !cache_dir.is_dir() {
                text_messages.messages.push(format!("Config dir {} is a file!", cache_dir.display()));
                return;
            }
        } else if let Err(e) = fs::create_dir_all(&cache_dir) {
            text_messages.messages.push(format!("Cannot create config dir {}, reason {}", cache_dir.display(), e));
            return;
        }
        let cache_file = cache_dir.join("cache_similar_videos.txt");
        let file_handler = match OpenOptions::new().truncate(true).write(true).create(true).open(&cache_file) {
            Ok(t) => t,
            Err(e) => {
                text_messages.messages.push(format!("Cannot create or open cache file {}, reason {}", cache_file.display(), e));
                return;
            }
        };
        let mut writer = BufWriter::new(file_handler);

        for file_entry in hashmap.values() {
            let mut string: String = String::with_capacity(256);

            string += format!("{}//{}//{}//{}//{}", file_entry.path.display(), file_entry.size, file_entry.modified_date, file_entry.vhash.num_frames(), file_entry.vhash.duration()).as_str();

            for i in file_entry.vhash.hash() {
                string.push_str("//");
                string.push_str(i.to_string().as_str());
            }

            if let Err(e) = writeln!(writer, "{}", string) {
                text_messages.messages.push(format!("Failed to save some data to cache file {}, reason {}", cache_file.display(), e));
                return;
            };
        }
    }
}
fn load_hashes_from_file(text_messages: &mut Messages) -> Option<BTreeMap<String, FileEntry>> {
    if let Some(proj_dirs) = ProjectDirs::from("pl", "Qarmin", "Czkawka") {
        let cache_dir = PathBuf::from(proj_dirs.cache_dir());
        let cache_file = cache_dir.join("cache_similar_videos.txt");
        let file_handler = match OpenOptions::new().read(true).open(&cache_file) {
            Ok(t) => t,
            Err(_inspected) => {
                // text_messages.messages.push(format!("Cannot find or open cache file {}", cache_file.display())); // This shouldn't be write to output
                return None;
            }
        };

        let reader = BufReader::new(file_handler);

        let mut hashmap_loaded_entries: BTreeMap<String, FileEntry> = Default::default();

        // Read the file line by line using the lines() iterator from std::io::BufRead.
        for (index, line) in reader.lines().enumerate() {
            let line = match line {
                Ok(t) => t,
                Err(e) => {
                    text_messages.warnings.push(format!("Failed to load line number {} from cache file {}, reason {}", index + 1, cache_file.display(), e));
                    return None;
                }
            };
            let uuu = line.split("//").collect::<Vec<&str>>();
            let hash_size = 19;
            // Hash size + other things
            if uuu.len() != (hash_size + 5) {
                text_messages.warnings.push(format!(
                    "Found invalid data in line {} - ({}) in cache file {}, expected {} values, found {}",
                    index + 1,
                    line,
                    cache_file.display(),
                    hash_size + 5,
                    uuu.len(),
                ));
                continue;
            };
            // Don't load cache data if destination file not exists
            if Path::new(uuu[0]).exists() {
                let mut hash: [u64; 19] = [0; 19];
                for i in 0..hash_size {
                    hash[i] = match uuu[5 + i as usize].parse::<u64>() {
                        Ok(t) => t,
                        Err(e) => {
                            text_messages
                                .warnings
                                .push(format!("Found invalid hash value in line {} - ({}) in cache file {}, reason {}", index + 1, line, cache_file.display(), e));
                            continue;
                        }
                    };
                }

                hashmap_loaded_entries.insert(
                    uuu[0].to_string(),
                    FileEntry {
                        path: PathBuf::from(uuu[0]),
                        size: match uuu[1].parse::<u64>() {
                            Ok(t) => t,
                            Err(e) => {
                                text_messages
                                    .warnings
                                    .push(format!("Found invalid size value in line {} - ({}) in cache file {}, reason {}", index + 1, line, cache_file.display(), e));
                                continue;
                            }
                        },
                        modified_date: match uuu[2].parse::<u64>() {
                            Ok(t) => t,
                            Err(e) => {
                                text_messages
                                    .warnings
                                    .push(format!("Found invalid modified date value in line {} - ({}) in cache file {}, reason {}", index + 1, line, cache_file.display(), e));
                                continue;
                            }
                        },
                        vhash: VideoHash::with_start_data(uuu[4].parse::<u32>().unwrap_or(0), uuu[0], hash, uuu[3].parse::<u32>().unwrap_or(10)),
                    },
                );
            }
        }

        return Some(hashmap_loaded_entries);
    }

    text_messages.messages.push("Cannot find or open system config dir to save cache file".to_string());
    None
}

pub fn check_if_ffmpeg_is_installed() -> bool {
    let vid = "999999999999999999.txt";
    if let Err(DetermineVideo { src_path: _a, error: FfmpegNotFound }) = VideoHash::from_path(&vid) {
        return false;
    }
    true
}
