use crossbeam_channel::Receiver;
use humansize::{file_size_opts as options, FileSize};
use std::collections::BTreeMap;
#[cfg(target_family = "unix")]
use std::collections::HashSet;
use std::fs::{File, Metadata, OpenOptions};
use std::io::prelude::*;
use std::io::{self, Error, ErrorKind};
#[cfg(target_family = "unix")]
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::{fs, mem, thread};

use crate::common::Common;
use crate::common_directory::Directories;
use crate::common_extensions::Extensions;
use crate::common_items::ExcludedItems;
use crate::common_messages::Messages;
use crate::common_traits::*;
use directories_next::ProjectDirs;
use rayon::prelude::*;
use std::hash::Hasher;
use std::io::{BufReader, BufWriter};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread::sleep;

const HASH_MB_LIMIT_BYTES: u64 = 1024 * 1024; // 1MB

const CACHE_FILE_NAME: &str = "cache_duplicates.txt";

#[derive(Debug)]
pub struct ProgressData {
    pub checking_method: CheckingMethod,
    pub current_stage: u8,
    pub max_stage: u8,
    pub files_checked: usize,
    pub files_to_check: usize,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum CheckingMethod {
    None,
    Name,
    Size,
    Hash,
    HashMb,
}

impl MyHasher for blake3::Hasher {
    fn update(&mut self, bytes: &[u8]) {
        self.update(bytes);
    }
    fn finalize(&self) -> String {
        self.finalize().to_hex().to_string()
    }
}

impl MyHasher for crc32fast::Hasher {
    fn update(&mut self, bytes: &[u8]) {
        self.write(bytes);
    }
    fn finalize(&self) -> String {
        self.finish().to_string()
    }
}

impl MyHasher for xxhash_rust::xxh3::Xxh3 {
    fn update(&mut self, bytes: &[u8]) {
        self.write(bytes);
    }
    fn finalize(&self) -> String {
        self.finish().to_string()
    }
}

#[derive(PartialEq, Eq, Clone, Debug, Copy)]
pub enum HashType {
    Blake3,
    Crc32,
    Xxh3,
}

impl HashType {
    fn hasher(self: &HashType) -> Box<dyn MyHasher> {
        match self {
            HashType::Blake3 => Box::new(blake3::Hasher::new()),
            HashType::Crc32 => Box::new(crc32fast::Hasher::new()),
            HashType::Xxh3 => Box::new(xxhash_rust::xxh3::Xxh3::new()),
        }
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum DeleteMethod {
    None,
    AllExceptNewest,
    AllExceptOldest,
    OneOldest,
    OneNewest,
    HardLink,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct FileEntry {
    pub path: PathBuf,
    pub size: u64,
    pub modified_date: u64,
    pub hash: String,
}

/// Info struck with helpful information's about results
#[derive(Default)]
pub struct Info {
    pub number_of_groups_by_size: usize,
    pub number_of_duplicated_files_by_size: usize,
    pub number_of_groups_by_hash: usize,
    pub number_of_duplicated_files_by_hash: usize,
    pub number_of_groups_by_name: usize,
    pub number_of_duplicated_files_by_name: usize,
    pub lost_space_by_size: u64,
    pub lost_space_by_hash: u64,
    pub bytes_read_when_hashing: u64,
    pub number_of_removed_files: usize,
    pub number_of_failed_to_remove_files: usize,
    pub gained_space: u64,
}

impl Info {
    pub fn new() -> Self {
        Default::default()
    }
}

/// Struct with required information's to work
pub struct DuplicateFinder {
    text_messages: Messages,
    information: Info,
    files_with_identical_names: BTreeMap<String, Vec<FileEntry>>,    // File Size, File Entry
    files_with_identical_size: BTreeMap<u64, Vec<FileEntry>>,        // File Size, File Entry
    files_with_identical_hashes: BTreeMap<u64, Vec<Vec<FileEntry>>>, // File Size, File Entry
    directories: Directories,
    allowed_extensions: Extensions,
    excluded_items: ExcludedItems,
    recursive_search: bool,
    minimal_file_size: u64,
    maximal_file_size: u64,
    check_method: CheckingMethod,
    delete_method: DeleteMethod,
    hash_type: HashType,
    ignore_hard_links: bool,
    dryrun: bool,
    stopped_search: bool,
    use_cache: bool,
    minimal_cache_file_size: u64,
}

impl DuplicateFinder {
    pub fn new() -> Self {
        Self {
            text_messages: Messages::new(),
            information: Info::new(),
            files_with_identical_names: Default::default(),
            files_with_identical_size: Default::default(),
            files_with_identical_hashes: Default::default(),
            recursive_search: true,
            allowed_extensions: Extensions::new(),
            check_method: CheckingMethod::None,
            delete_method: DeleteMethod::None,
            minimal_file_size: 8192,
            maximal_file_size: u64::MAX,
            directories: Directories::new(),
            excluded_items: ExcludedItems::new(),
            stopped_search: false,
            ignore_hard_links: true,
            hash_type: HashType::Blake3,
            dryrun: false,
            use_cache: true,
            minimal_cache_file_size: 2 * 1024 * 1024, // By default cache only >= 1MB files
        }
    }

    pub fn find_duplicates(&mut self, stop_receiver: Option<&Receiver<()>>, progress_sender: Option<&futures::channel::mpsc::UnboundedSender<ProgressData>>) {
        self.directories.optimize_directories(self.recursive_search, &mut self.text_messages);

        match self.check_method {
            CheckingMethod::Name => {
                if !self.check_files_name(stop_receiver, progress_sender) {
                    self.stopped_search = true;
                    return;
                }
            }
            CheckingMethod::Size => {
                if !self.check_files_size(stop_receiver, progress_sender) {
                    self.stopped_search = true;
                    return;
                }
            }
            CheckingMethod::HashMb | CheckingMethod::Hash => {
                if !self.check_files_size(stop_receiver, progress_sender) {
                    self.stopped_search = true;
                    return;
                }
                if !self.check_files_hash(stop_receiver, progress_sender) {
                    self.stopped_search = true;
                    return;
                }
            }
            CheckingMethod::None => {
                panic!();
            }
        }
        self.delete_files();
        self.debug_print();
    }

    pub const fn get_check_method(&self) -> &CheckingMethod {
        &self.check_method
    }

    pub fn get_stopped_search(&self) -> bool {
        self.stopped_search
    }

    pub fn set_minimal_cache_file_size(&mut self, minimal_cache_file_size: u64) {
        self.minimal_cache_file_size = minimal_cache_file_size;
    }

    pub const fn get_files_sorted_by_names(&self) -> &BTreeMap<String, Vec<FileEntry>> {
        &self.files_with_identical_names
    }

    pub fn set_use_cache(&mut self, use_cache: bool) {
        self.use_cache = use_cache;
    }

    pub const fn get_files_sorted_by_size(&self) -> &BTreeMap<u64, Vec<FileEntry>> {
        &self.files_with_identical_size
    }

    pub const fn get_files_sorted_by_hash(&self) -> &BTreeMap<u64, Vec<Vec<FileEntry>>> {
        &self.files_with_identical_hashes
    }
    pub fn set_maximal_file_size(&mut self, maximal_file_size: u64) {
        self.maximal_file_size = match maximal_file_size {
            0 => 1,
            t => t,
        };
    }

    pub const fn get_text_messages(&self) -> &Messages {
        &self.text_messages
    }

    pub const fn get_information(&self) -> &Info {
        &self.information
    }

    pub fn set_hash_type(&mut self, hash_type: HashType) {
        self.hash_type = hash_type;
    }

    pub fn set_ignore_hard_links(&mut self, ignore_hard_links: bool) {
        self.ignore_hard_links = ignore_hard_links;
    }

    pub fn set_dryrun(&mut self, dryrun: bool) {
        self.dryrun = dryrun;
    }

    pub fn set_check_method(&mut self, check_method: CheckingMethod) {
        self.check_method = check_method;
    }

    pub fn set_delete_method(&mut self, delete_method: DeleteMethod) {
        self.delete_method = delete_method;
    }

    pub fn set_minimal_file_size(&mut self, minimal_file_size: u64) {
        self.minimal_file_size = match minimal_file_size {
            0 => 1,
            t => t,
        };
    }

    pub fn set_recursive_search(&mut self, recursive_search: bool) {
        self.recursive_search = recursive_search;
    }

    pub fn set_included_directory(&mut self, included_directory: Vec<PathBuf>) -> bool {
        self.directories.set_included_directory(included_directory, &mut self.text_messages)
    }

    pub fn set_excluded_directory(&mut self, excluded_directory: Vec<PathBuf>) {
        self.directories.set_excluded_directory(excluded_directory, &mut self.text_messages);
    }
    pub fn set_allowed_extensions(&mut self, allowed_extensions: String) {
        self.allowed_extensions.set_allowed_extensions(allowed_extensions, &mut self.text_messages);
    }

    pub fn set_excluded_items(&mut self, excluded_items: Vec<String>) {
        self.excluded_items.set_excluded_items(excluded_items, &mut self.text_messages);
    }

    fn check_files_name(&mut self, stop_receiver: Option<&Receiver<()>>, progress_sender: Option<&futures::channel::mpsc::UnboundedSender<ProgressData>>) -> bool {
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
                        checking_method: CheckingMethod::Name,
                        current_stage: 0,
                        max_stage: 0,
                        files_checked: atomic_file_counter.load(Ordering::Relaxed) as usize,
                        files_to_check: 0,
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
                        continue 'dir;
                    } //Permissions denied
                };
                let metadata: Metadata = match entry_data.metadata() {
                    Ok(t) => t,
                    Err(e) => {
                        self.text_messages.warnings.push(format!("Cannot read metadata in dir {}, reason {}", current_folder.display(), e));
                        continue 'dir;
                    } //Permissions denied
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

                    folders_to_check.push(next_folder);
                } else if metadata.is_file() {
                    atomic_file_counter.fetch_add(1, Ordering::Relaxed);
                    // let mut have_valid_extension: bool;
                    let file_name_lowercase: String = match entry_data.file_name().into_string() {
                        Ok(t) => t,
                        Err(_inspected) => {
                            println!("File {:?} has not valid UTF-8 name", entry_data);
                            continue 'dir;
                        }
                    }
                    .to_lowercase();

                    // Checking allowed extensions
                    if !self.allowed_extensions.file_extensions.is_empty() {
                        let allowed = self.allowed_extensions.file_extensions.iter().any(|e| file_name_lowercase.ends_with((".".to_string() + e.to_lowercase().as_str()).as_str()));
                        if !allowed {
                            // Not an allowed extension, ignore it.
                            continue 'dir;
                        }
                    }
                    // Checking files
                    if (self.minimal_file_size..=self.maximal_file_size).contains(&metadata.len()) {
                        let current_file_name = current_folder.join(entry_data.file_name());
                        if self.excluded_items.is_excluded(&current_file_name) {
                            continue 'dir;
                        }

                        // Creating new file entry
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
                            hash: "".to_string(),
                        };

                        // Adding files to BTreeMap
                        self.files_with_identical_names.entry(entry_data.file_name().to_string_lossy().to_string()).or_insert_with(Vec::new);
                        self.files_with_identical_names.get_mut(&entry_data.file_name().to_string_lossy().to_string()).unwrap().push(fe);
                    }
                }
            }
        }

        // End thread which send info to gui
        progress_thread_run.store(false, Ordering::Relaxed);
        progress_thread_handle.join().unwrap();

        // Create new BTreeMap without single size entries(files have not duplicates)
        let mut new_map: BTreeMap<String, Vec<FileEntry>> = Default::default();

        for (name, vector) in &self.files_with_identical_names {
            if vector.len() > 1 {
                self.information.number_of_duplicated_files_by_name += vector.len() - 1;
                self.information.number_of_groups_by_name += 1;
                new_map.insert(name.clone(), vector.clone());
            }
        }
        self.files_with_identical_names = new_map;

        Common::print_time(start_time, SystemTime::now(), "check_files_name".to_string());
        true
    }

    /// Read file length and puts it to different boxes(each for different lengths)
    /// If in box is only 1 result, then it is removed
    fn check_files_size(&mut self, stop_receiver: Option<&Receiver<()>>, progress_sender: Option<&futures::channel::mpsc::UnboundedSender<ProgressData>>) -> bool {
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
            let checking_method = self.check_method.clone();
            let max_stage = match self.check_method {
                CheckingMethod::Size => 0,
                CheckingMethod::HashMb | CheckingMethod::Hash => 2,
                _ => 255,
            };
            progress_thread_handle = thread::spawn(move || loop {
                progress_send
                    .unbounded_send(ProgressData {
                        checking_method: checking_method.clone(),
                        current_stage: 0,
                        max_stage,
                        files_checked: atomic_file_counter.load(Ordering::Relaxed) as usize,
                        files_to_check: 0,
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
                        continue 'dir;
                    } //Permissions denied
                };
                let metadata: Metadata = match entry_data.metadata() {
                    Ok(t) => t,
                    Err(e) => {
                        self.text_messages.warnings.push(format!("Cannot read metadata in dir {}, reason {}", current_folder.display(), e));
                        continue 'dir;
                    } //Permissions denied
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

                    folders_to_check.push(next_folder);
                } else if metadata.is_file() {
                    atomic_file_counter.fetch_add(1, Ordering::Relaxed);
                    // let mut have_valid_extension: bool;
                    let file_name_lowercase: String = match entry_data.file_name().into_string() {
                        Ok(t) => t,
                        Err(_inspected) => {
                            println!("File {:?} has not valid UTF-8 name", entry_data);
                            continue 'dir;
                        }
                    }
                    .to_lowercase();

                    // Checking allowed extensions
                    if !self.allowed_extensions.file_extensions.is_empty() {
                        let allowed = self.allowed_extensions.file_extensions.iter().any(|e| file_name_lowercase.ends_with((".".to_string() + e.to_lowercase().as_str()).as_str()));
                        if !allowed {
                            // Not an allowed extension, ignore it.

                            continue 'dir;
                        }
                    }
                    // Checking files
                    if (self.minimal_file_size..=self.maximal_file_size).contains(&metadata.len()) {
                        let current_file_name = current_folder.join(entry_data.file_name());
                        if self.excluded_items.is_excluded(&current_file_name) {
                            continue 'dir;
                        }

                        // Creating new file entry
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
                            hash: "".to_string(),
                        };

                        // Adding files to BTreeMap
                        self.files_with_identical_size.entry(metadata.len()).or_insert_with(Vec::new);
                        self.files_with_identical_size.get_mut(&metadata.len()).unwrap().push(fe);
                    }
                }
            }
        }
        // End thread which send info to gui
        progress_thread_run.store(false, Ordering::Relaxed);
        progress_thread_handle.join().unwrap();

        // Create new BTreeMap without single size entries(files have not duplicates)
        let mut new_map: BTreeMap<u64, Vec<FileEntry>> = Default::default();

        for (size, vec) in &self.files_with_identical_size {
            if vec.len() <= 1 {
                continue;
            }

            let vector;
            if self.ignore_hard_links {
                vector = filter_hard_links(vec);
            } else {
                vector = vec.clone();
            }

            if vector.len() > 1 {
                self.information.number_of_duplicated_files_by_size += vector.len() - 1;
                self.information.number_of_groups_by_size += 1;
                self.information.lost_space_by_size += (vector.len() as u64 - 1) * size;
                new_map.insert(*size, vector);
            }
        }
        self.files_with_identical_size = new_map;

        Common::print_time(start_time, SystemTime::now(), "check_files_size".to_string());
        true
    }

    /// The slowest checking type, which must be applied after checking for size
    fn check_files_hash(&mut self, stop_receiver: Option<&Receiver<()>>, progress_sender: Option<&futures::channel::mpsc::UnboundedSender<ProgressData>>) -> bool {
        let check_type = Arc::new(self.hash_type);

        let start_time: SystemTime = SystemTime::now();
        let check_was_breaked = AtomicBool::new(false); // Used for breaking from GUI and ending check thread
        let mut pre_checked_map: BTreeMap<u64, Vec<FileEntry>> = Default::default();

        //// PROGRESS THREAD START
        const LOOP_DURATION: u32 = 200; //in ms
        let progress_thread_run = Arc::new(AtomicBool::new(true));

        let atomic_file_counter = Arc::new(AtomicUsize::new(0));

        let progress_thread_handle;
        if let Some(progress_sender) = progress_sender {
            let progress_send = progress_sender.clone();
            let progress_thread_run = progress_thread_run.clone();
            let atomic_file_counter = atomic_file_counter.clone();
            let files_to_check = self.files_with_identical_size.iter().map(|e| e.1.len()).sum();
            let checking_method = self.check_method.clone();
            progress_thread_handle = thread::spawn(move || loop {
                progress_send
                    .unbounded_send(ProgressData {
                        checking_method: checking_method.clone(),
                        current_stage: 1,
                        max_stage: 2,
                        files_checked: atomic_file_counter.load(Ordering::Relaxed) as usize,
                        files_to_check,
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

        #[allow(clippy::type_complexity)]
        let pre_hash_results: Vec<(u64, BTreeMap<String, Vec<FileEntry>>, Vec<String>, u64)> = self
            .files_with_identical_size
            .par_iter()
            .map(|(size, vec_file_entry)| {
                let mut hashmap_with_hash: BTreeMap<String, Vec<FileEntry>> = Default::default();
                let mut errors: Vec<String> = Vec::new();
                let mut bytes_read: u64 = 0;
                let mut buffer = [0u8; 1024 * 2];

                atomic_file_counter.fetch_add(vec_file_entry.len(), Ordering::Relaxed);
                for file_entry in vec_file_entry {
                    if stop_receiver.is_some() && stop_receiver.unwrap().try_recv().is_ok() {
                        check_was_breaked.store(true, Ordering::Relaxed);
                        return None;
                    }
                    match hash_calculation(&mut buffer, file_entry, &check_type, 0) {
                        Ok((hash_string, bytes)) => {
                            bytes_read += bytes;
                            hashmap_with_hash.entry(hash_string.clone()).or_insert_with(Vec::new);
                            hashmap_with_hash.get_mut(hash_string.as_str()).unwrap().push(file_entry.clone());
                        }
                        Err(s) => errors.push(s),
                    }
                }
                Some((*size, hashmap_with_hash, errors, bytes_read))
            })
            .while_some()
            .collect();

        // End thread which send info to gui
        progress_thread_run.store(false, Ordering::Relaxed);
        progress_thread_handle.join().unwrap();

        // Check if user aborted search(only from GUI)
        if check_was_breaked.load(Ordering::Relaxed) {
            return false;
        }

        // Check results
        for (size, hash_map, mut errors, bytes_read) in pre_hash_results {
            self.information.bytes_read_when_hashing += bytes_read;
            self.text_messages.warnings.append(&mut errors);
            for (_hash, mut vec_file_entry) in hash_map {
                if vec_file_entry.len() > 1 {
                    pre_checked_map.entry(size).or_insert_with(Vec::new);
                    pre_checked_map.get_mut(&size).unwrap().append(&mut vec_file_entry);
                }
            }
        }

        Common::print_time(start_time, SystemTime::now(), "check_files_hash - prehash".to_string());
        let start_time: SystemTime = SystemTime::now();

        /////////////////////////

        //// PROGRESS THREAD START
        // const LOOP_DURATION: u32 = 200; //in ms
        let progress_thread_run = Arc::new(AtomicBool::new(true));

        let atomic_file_counter = Arc::new(AtomicUsize::new(0));

        let progress_thread_handle;
        if let Some(progress_sender) = progress_sender {
            let progress_send = progress_sender.clone();
            let progress_thread_run = progress_thread_run.clone();
            let atomic_file_counter = atomic_file_counter.clone();
            let files_to_check = pre_checked_map.iter().map(|e| e.1.len()).sum();
            let checking_method = self.check_method.clone();
            progress_thread_handle = thread::spawn(move || loop {
                progress_send
                    .unbounded_send(ProgressData {
                        checking_method: checking_method.clone(),
                        current_stage: 2,
                        max_stage: 2,
                        files_checked: atomic_file_counter.load(Ordering::Relaxed) as usize,
                        files_to_check,
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

        #[allow(clippy::type_complexity)]
        let mut full_hash_results: Vec<(u64, BTreeMap<String, Vec<FileEntry>>, Vec<String>, u64)>;

        match self.check_method {
            CheckingMethod::HashMb => {
                full_hash_results = pre_checked_map
                    .par_iter()
                    .map(|(size, vec_file_entry)| {
                        let mut hashmap_with_hash: BTreeMap<String, Vec<FileEntry>> = Default::default();
                        let mut errors: Vec<String> = Vec::new();
                        let mut bytes_read: u64 = 0;
                        let mut buffer = [0u8; 1024 * 128];
                        atomic_file_counter.fetch_add(vec_file_entry.len(), Ordering::Relaxed);
                        for file_entry in vec_file_entry {
                            if stop_receiver.is_some() && stop_receiver.unwrap().try_recv().is_ok() {
                                check_was_breaked.store(true, Ordering::Relaxed);
                                return None;
                            }

                            match hash_calculation(&mut buffer, file_entry, &check_type, HASH_MB_LIMIT_BYTES) {
                                Ok((hash_string, bytes)) => {
                                    bytes_read += bytes;
                                    hashmap_with_hash.entry(hash_string.to_string()).or_insert_with(Vec::new);
                                    hashmap_with_hash.get_mut(hash_string.as_str()).unwrap().push(file_entry.to_owned());
                                }
                                Err(s) => errors.push(s),
                            }
                        }
                        Some((*size, hashmap_with_hash, errors, bytes_read))
                    })
                    .while_some()
                    .collect();
            }
            CheckingMethod::Hash => {
                let loaded_hash_map;

                let mut records_already_cached: BTreeMap<u64, Vec<FileEntry>> = Default::default();
                let mut non_cached_files_to_check: BTreeMap<u64, Vec<FileEntry>> = Default::default();

                if self.use_cache {
                    loaded_hash_map = match load_hashes_from_file(&mut self.text_messages, &self.hash_type) {
                        Some(t) => t,
                        None => Default::default(),
                    };

                    for (size, vec_file_entry) in pre_checked_map {
                        #[allow(clippy::collapsible_if)]
                        if !loaded_hash_map.contains_key(&size) {
                            // If loaded data doesn't contains current info
                            non_cached_files_to_check.insert(size, vec_file_entry);
                        } else {
                            let loaded_vec_file_entry = loaded_hash_map.get(&size).unwrap();

                            for file_entry in vec_file_entry {
                                let mut found: bool = false;
                                for loaded_file_entry in loaded_vec_file_entry {
                                    if file_entry.path == loaded_file_entry.path && file_entry.modified_date == loaded_file_entry.modified_date {
                                        records_already_cached.entry(file_entry.size).or_insert_with(Vec::new);
                                        records_already_cached.get_mut(&file_entry.size).unwrap().push(loaded_file_entry.clone());
                                        found = true;
                                        break;
                                    }
                                }

                                if !found {
                                    non_cached_files_to_check.entry(file_entry.size).or_insert_with(Vec::new);
                                    non_cached_files_to_check.get_mut(&file_entry.size).unwrap().push(file_entry);
                                }
                            }
                        }
                    }
                } else {
                    loaded_hash_map = Default::default();
                    mem::swap(&mut pre_checked_map, &mut non_cached_files_to_check);
                }

                full_hash_results = non_cached_files_to_check
                    .par_iter()
                    .map(|(size, vec_file_entry)| {
                        let mut hashmap_with_hash: BTreeMap<String, Vec<FileEntry>> = Default::default();
                        let mut errors: Vec<String> = Vec::new();
                        let mut bytes_read: u64 = 0;
                        let mut buffer = [0u8; 1024 * 128];

                        atomic_file_counter.fetch_add(vec_file_entry.len(), Ordering::Relaxed);
                        for file_entry in vec_file_entry {
                            if stop_receiver.is_some() && stop_receiver.unwrap().try_recv().is_ok() {
                                check_was_breaked.store(true, Ordering::Relaxed);
                                return None;
                            }

                            match hash_calculation(&mut buffer, file_entry, &check_type, u64::MAX) {
                                Ok((hash_string, bytes)) => {
                                    bytes_read += bytes;
                                    let mut file_entry = file_entry.clone();
                                    file_entry.hash = hash_string.clone();
                                    hashmap_with_hash.entry(hash_string.clone()).or_insert_with(Vec::new);
                                    hashmap_with_hash.get_mut(hash_string.as_str()).unwrap().push(file_entry);
                                }
                                Err(s) => errors.push(s),
                            }
                        }
                        Some((*size, hashmap_with_hash, errors, bytes_read))
                    })
                    .while_some()
                    .collect();

                if self.use_cache {
                    'main: for (size, vec_file_entry) in records_already_cached {
                        // Check if size already exists, if exists we must to change it outside because cannot have mut and non mut reference to full_hash_results
                        for (full_size, full_hashmap, _errors, _bytes_read) in &mut full_hash_results {
                            if size == *full_size {
                                for file_entry in vec_file_entry {
                                    full_hashmap.entry(file_entry.hash.clone()).or_insert_with(Vec::new);
                                    full_hashmap.get_mut(&file_entry.hash).unwrap().push(file_entry);
                                }
                                continue 'main;
                            }
                        }
                        // Size doesn't exists add results to files
                        let mut temp_hashmap: BTreeMap<String, Vec<FileEntry>> = Default::default();
                        for file_entry in vec_file_entry {
                            temp_hashmap.entry(file_entry.hash.clone()).or_insert_with(Vec::new);
                            temp_hashmap.get_mut(&file_entry.hash).unwrap().push(file_entry);
                        }
                        full_hash_results.push((size, temp_hashmap, Vec::new(), 0));
                    }

                    // Must save all results to file, old loaded from file with all currently counted results
                    let mut all_results: BTreeMap<String, FileEntry> = Default::default();
                    for (_size, vec_file_entry) in loaded_hash_map {
                        for file_entry in vec_file_entry {
                            all_results.insert(file_entry.path.to_string_lossy().to_string(), file_entry);
                        }
                    }
                    for (_size, hashmap, _errors, _bytes_read) in &full_hash_results {
                        for vec_file_entry in hashmap.values() {
                            for file_entry in vec_file_entry {
                                all_results.insert(file_entry.path.to_string_lossy().to_string(), file_entry.clone());
                            }
                        }
                    }
                    save_hashes_to_file(&all_results, &mut self.text_messages, &self.hash_type, self.minimal_cache_file_size);
                }
            }
            _ => panic!("What"),
        }

        // End thread which send info to gui
        progress_thread_run.store(false, Ordering::Relaxed);
        progress_thread_handle.join().unwrap();

        // Check if user aborted search(only from GUI)
        if check_was_breaked.load(Ordering::Relaxed) {
            return false;
        }

        for (size, hash_map, mut errors, bytes_read) in full_hash_results {
            self.information.bytes_read_when_hashing += bytes_read;
            self.text_messages.warnings.append(&mut errors);
            for (_hash, vec_file_entry) in hash_map {
                if vec_file_entry.len() > 1 {
                    self.files_with_identical_hashes.entry(size).or_insert_with(Vec::new);
                    self.files_with_identical_hashes.get_mut(&size).unwrap().push(vec_file_entry);
                }
            }
        }

        /////////////////////////

        for (size, vector_vectors) in &self.files_with_identical_hashes {
            for vector in vector_vectors {
                self.information.number_of_duplicated_files_by_hash += vector.len() - 1;
                self.information.number_of_groups_by_hash += 1;
                self.information.lost_space_by_hash += (vector.len() as u64 - 1) * size;
            }
        }

        Common::print_time(start_time, SystemTime::now(), "check_files_hash - full hash".to_string());

        // Clean unused data
        self.files_with_identical_size = Default::default();

        true
    }

    /// Function to delete files, from filed before BTreeMap
    /// Using another function to delete files to avoid duplicates data
    fn delete_files(&mut self) {
        let start_time: SystemTime = SystemTime::now();
        if self.delete_method == DeleteMethod::None {
            return;
        }

        match self.check_method {
            CheckingMethod::Name => {
                for vector in self.files_with_identical_names.values() {
                    let tuple: (u64, usize, usize) = delete_files(vector, &self.delete_method, &mut self.text_messages, self.dryrun);
                    self.information.gained_space += tuple.0;
                    self.information.number_of_removed_files += tuple.1;
                    self.information.number_of_failed_to_remove_files += tuple.2;
                }
            }
            CheckingMethod::Hash | CheckingMethod::HashMb => {
                for vector_vectors in self.files_with_identical_hashes.values() {
                    for vector in vector_vectors.iter() {
                        let tuple: (u64, usize, usize) = delete_files(vector, &self.delete_method, &mut self.text_messages, self.dryrun);
                        self.information.gained_space += tuple.0;
                        self.information.number_of_removed_files += tuple.1;
                        self.information.number_of_failed_to_remove_files += tuple.2;
                    }
                }
            }
            CheckingMethod::Size => {
                for vector in self.files_with_identical_size.values() {
                    let tuple: (u64, usize, usize) = delete_files(vector, &self.delete_method, &mut self.text_messages, self.dryrun);
                    self.information.gained_space += tuple.0;
                    self.information.number_of_removed_files += tuple.1;
                    self.information.number_of_failed_to_remove_files += tuple.2;
                }
            }
            CheckingMethod::None => {
                //Just do nothing
                panic!("Checking method should never be none.");
            }
        }

        Common::print_time(start_time, SystemTime::now(), "delete_files".to_string());
    }
}
impl Default for DuplicateFinder {
    fn default() -> Self {
        Self::new()
    }
}

impl DebugPrint for DuplicateFinder {
    #[allow(dead_code)]
    #[allow(unreachable_code)]
    /// Debugging printing - only available on debug build
    fn debug_print(&self) {
        #[cfg(not(debug_assertions))]
        {
            return;
        }
        println!("---------------DEBUG PRINT---------------");
        println!("### Information's");

        println!("Errors size - {}", self.text_messages.errors.len());
        println!("Warnings size - {}", self.text_messages.warnings.len());
        println!("Messages size - {}", self.text_messages.messages.len());
        println!(
            "Number of duplicated files by size(in groups) - {} ({})",
            self.information.number_of_duplicated_files_by_size, self.information.number_of_groups_by_size
        );
        println!(
            "Number of duplicated files by hash(in groups) - {} ({})",
            self.information.number_of_duplicated_files_by_hash, self.information.number_of_groups_by_hash
        );
        println!(
            "Number of duplicated files by name(in groups) - {} ({})",
            self.information.number_of_duplicated_files_by_name, self.information.number_of_groups_by_name
        );
        println!("Lost space by size - {} ({} bytes)", self.information.lost_space_by_size.file_size(options::BINARY).unwrap(), self.information.lost_space_by_size);
        println!("Lost space by hash - {} ({} bytes)", self.information.lost_space_by_hash.file_size(options::BINARY).unwrap(), self.information.lost_space_by_hash);
        println!(
            "Gained space by removing duplicated entries - {} ({} bytes)",
            self.information.gained_space.file_size(options::BINARY).unwrap(),
            self.information.gained_space
        );
        println!(
            "Bytes read when hashing - {} ({} bytes)",
            self.information.bytes_read_when_hashing.file_size(options::BINARY).unwrap(),
            self.information.bytes_read_when_hashing
        );
        println!("Number of removed files - {}", self.information.number_of_removed_files);
        println!("Number of failed to remove files - {}", self.information.number_of_failed_to_remove_files);

        println!("### Other");

        println!("Files list size - {}", self.files_with_identical_size.len());
        println!("Hashed Files list size - {}", self.files_with_identical_hashes.len());
        println!("Allowed extensions - {:?}", self.allowed_extensions.file_extensions);
        println!("Excluded items - {:?}", self.excluded_items.items);
        println!("Included directories - {:?}", self.directories.included_directories);
        println!("Excluded directories - {:?}", self.directories.excluded_directories);
        println!("Recursive search - {}", self.recursive_search.to_string());
        println!("Minimum file size - {:?}", self.minimal_file_size);
        println!("Checking Method - {:?}", self.check_method);
        println!("Delete Method - {:?}", self.delete_method);
        println!("-----------------------------------------");
    }
}
impl SaveResults for DuplicateFinder {
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
        match self.check_method {
            CheckingMethod::Name => {
                if !self.files_with_identical_names.is_empty() {
                    writeln!(writer, "-------------------------------------------------Files with same names-------------------------------------------------").unwrap();
                    writeln!(
                        writer,
                        "Found {} files in {} groups with same name(may have different content)",
                        self.information.number_of_duplicated_files_by_name, self.information.number_of_groups_by_name,
                    )
                    .unwrap();
                    for (name, vector) in self.files_with_identical_names.iter().rev() {
                        writeln!(writer, "Name - {} - {} files ", name, vector.len()).unwrap();
                        for j in vector {
                            writeln!(writer, "{}", j.path.display()).unwrap();
                        }
                        writeln!(writer).unwrap();
                    }
                } else {
                    write!(writer, "Not found any files with same names.").unwrap();
                }
            }
            CheckingMethod::Size => {
                if !self.files_with_identical_size.is_empty() {
                    writeln!(writer, "-------------------------------------------------Files with same size-------------------------------------------------").unwrap();
                    writeln!(
                        writer,
                        "Found {} duplicated files which in {} groups which takes {}.",
                        self.information.number_of_duplicated_files_by_size,
                        self.information.number_of_groups_by_size,
                        self.information.lost_space_by_size.file_size(options::BINARY).unwrap()
                    )
                    .unwrap();
                    for (size, vector) in self.files_with_identical_size.iter().rev() {
                        write!(writer, "\n---- Size {} ({}) - {} files \n", size.file_size(options::BINARY).unwrap(), size, vector.len()).unwrap();
                        for file_entry in vector {
                            writeln!(writer, "{}", file_entry.path.display()).unwrap();
                        }
                    }
                } else {
                    write!(writer, "Not found any duplicates.").unwrap();
                }
            }
            CheckingMethod::Hash | CheckingMethod::HashMb => {
                if !self.files_with_identical_hashes.is_empty() {
                    writeln!(writer, "-------------------------------------------------Files with same hashes-------------------------------------------------").unwrap();
                    writeln!(
                        writer,
                        "Found {} duplicated files which in {} groups which takes {}.",
                        self.information.number_of_duplicated_files_by_hash,
                        self.information.number_of_groups_by_hash,
                        self.information.lost_space_by_hash.file_size(options::BINARY).unwrap()
                    )
                    .unwrap();
                    for (size, vectors_vector) in self.files_with_identical_hashes.iter().rev() {
                        for vector in vectors_vector {
                            writeln!(writer, "\n---- Size {} ({}) - {} files", size.file_size(options::BINARY).unwrap(), size, vector.len()).unwrap();
                            for file_entry in vector {
                                writeln!(writer, "{}", file_entry.path.display()).unwrap();
                            }
                        }
                    }
                } else {
                    write!(writer, "Not found any duplicates.").unwrap();
                }
            }
            CheckingMethod::None => {
                panic!();
            }
        }
        Common::print_time(start_time, SystemTime::now(), "save_results_to_file".to_string());
        true
    }
}
impl PrintResults for DuplicateFinder {
    /// Print information's about duplicated entries
    /// Only needed for CLI
    fn print_results(&self) {
        let start_time: SystemTime = SystemTime::now();
        let mut number_of_files: u64 = 0;
        let mut number_of_groups: u64 = 0;

        match self.check_method {
            CheckingMethod::Name => {
                for i in &self.files_with_identical_names {
                    number_of_files += i.1.len() as u64;
                    number_of_groups += 1;
                }
                println!("Found {} files in {} groups with same name(may have different content)", number_of_files, number_of_groups,);
                for (name, vector) in &self.files_with_identical_names {
                    println!("Name - {} - {} files ", name, vector.len());
                    for j in vector {
                        println!("{}", j.path.display());
                    }
                    println!();
                }
            }
            CheckingMethod::Hash | CheckingMethod::HashMb => {
                for (_size, vector) in self.files_with_identical_hashes.iter() {
                    for j in vector {
                        number_of_files += j.len() as u64;
                        number_of_groups += 1;
                    }
                }
                println!(
                    "Found {} duplicated files in {} groups with same content which took {}:",
                    number_of_files,
                    number_of_groups,
                    self.information.lost_space_by_size.file_size(options::BINARY).unwrap()
                );
                for (size, vector) in self.files_with_identical_hashes.iter().rev() {
                    for j in vector {
                        println!("Size - {} ({}) - {} files ", size.file_size(options::BINARY).unwrap(), size, j.len());
                        for k in j {
                            println!("{}", k.path.display());
                        }
                        println!("----");
                    }
                    println!();
                }
            }
            CheckingMethod::Size => {
                for i in &self.files_with_identical_size {
                    number_of_files += i.1.len() as u64;
                    number_of_groups += 1;
                }
                println!(
                    "Found {} files in {} groups with same size(may have different content) which took {}:",
                    number_of_files,
                    number_of_groups,
                    self.information.lost_space_by_size.file_size(options::BINARY).unwrap()
                );
                for (size, vector) in &self.files_with_identical_size {
                    println!("Size - {} ({}) - {} files ", size.file_size(options::BINARY).unwrap(), size, vector.len());
                    for j in vector {
                        println!("{}", j.path.display());
                    }
                    println!();
                }
            }
            CheckingMethod::None => {
                panic!("Checking Method shouldn't be ever set to None");
            }
        }
        Common::print_time(start_time, SystemTime::now(), "print_entries".to_string());
    }
}

/// Functions to remove slice(vector) of files with provided method
/// Returns size of removed elements, number of deleted and failed to delete files and modified warning list
fn delete_files(vector: &[FileEntry], delete_method: &DeleteMethod, text_messages: &mut Messages, dryrun: bool) -> (u64, usize, usize) {
    assert!(vector.len() > 1, "Vector length must be bigger than 1(This should be done in previous steps).");
    let mut gained_space: u64 = 0;
    let mut removed_files: usize = 0;
    let mut failed_to_remove_files: usize = 0;
    let mut values = vector.iter().enumerate();
    let q_index = match delete_method {
        DeleteMethod::OneOldest | DeleteMethod::AllExceptNewest => values.max_by(|(_, l), (_, r)| l.modified_date.cmp(&r.modified_date)),
        DeleteMethod::OneNewest | DeleteMethod::AllExceptOldest | DeleteMethod::HardLink => values.min_by(|(_, l), (_, r)| l.modified_date.cmp(&r.modified_date)),
        DeleteMethod::None => values.next(),
    };
    let q_index = q_index.map(|t| t.0).unwrap_or(0);
    let n = match delete_method {
        DeleteMethod::OneNewest | DeleteMethod::OneOldest => 1,
        DeleteMethod::AllExceptNewest | DeleteMethod::AllExceptOldest | DeleteMethod::None | DeleteMethod::HardLink => usize::MAX,
    };
    for (index, file) in vector.iter().enumerate() {
        if q_index == index {
            continue;
        } else if removed_files + failed_to_remove_files >= n {
            break;
        }

        let r = match delete_method {
            DeleteMethod::OneOldest | DeleteMethod::OneNewest | DeleteMethod::AllExceptOldest | DeleteMethod::AllExceptNewest => {
                if dryrun {
                    Ok(Some(format!("Delete {}", file.path.display())))
                } else {
                    fs::remove_file(&file.path).map(|_| None)
                }
            }
            DeleteMethod::HardLink => {
                let src = &vector[q_index].path;
                if dryrun {
                    Ok(Some(format!("Replace file {} with hard link to {}", file.path.display(), src.display())))
                } else {
                    make_hard_link(src, &file.path).map(|_| None)
                }
            }
            DeleteMethod::None => Ok(None),
        };

        match r {
            Err(e) => {
                failed_to_remove_files += 1;
                text_messages.warnings.push(format!("Failed to remove {} ({})", file.path.display(), e));
            }
            Ok(Some(msg)) => {
                text_messages.messages.push(msg);
                removed_files += 1;
                gained_space += file.size;
            }
            Ok(None) => {
                removed_files += 1;
                gained_space += file.size;
            }
        }
    }
    (gained_space, removed_files, failed_to_remove_files)
}

#[cfg(target_family = "windows")]
fn filter_hard_links(vec_file_entry: &[FileEntry]) -> Vec<FileEntry> {
    vec_file_entry.to_vec()
}

#[cfg(target_family = "unix")]
fn filter_hard_links(vec_file_entry: &[FileEntry]) -> Vec<FileEntry> {
    let mut inodes: HashSet<u64> = HashSet::with_capacity(vec_file_entry.len());
    let mut identical: Vec<FileEntry> = Vec::with_capacity(vec_file_entry.len());
    for f in vec_file_entry {
        if let Ok(meta) = fs::metadata(&f.path) {
            if !inodes.insert(meta.ino()) {
                continue;
            }
        }
        identical.push(f.clone());
    }
    identical
}

pub fn make_hard_link(src: &Path, dst: &Path) -> io::Result<()> {
    let dst_dir = dst.parent().ok_or_else(|| Error::new(ErrorKind::Other, "No parent"))?;
    let temp = tempfile::Builder::new().tempfile_in(dst_dir)?;
    fs::rename(dst, temp.path())?;
    let result = fs::hard_link(src, dst);
    if result.is_err() {
        fs::rename(temp.path(), dst)?;
    }
    result
}

fn save_hashes_to_file(hashmap: &BTreeMap<String, FileEntry>, text_messages: &mut Messages, type_of_hash: &HashType, minimal_cache_file_size: u64) {
    if let Some(proj_dirs) = ProjectDirs::from("pl", "Qarmin", "Czkawka") {
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
        let cache_file = cache_dir.join(CACHE_FILE_NAME.replace(".", format!("_{:?}.", type_of_hash).as_str()));
        let file_handler = match OpenOptions::new().truncate(true).write(true).create(true).open(&cache_file) {
            Ok(t) => t,
            Err(e) => {
                text_messages.messages.push(format!("Cannot create or open cache file {}, reason {}", cache_file.display(), e));
                return;
            }
        };
        let mut writer = BufWriter::new(file_handler);

        for file_entry in hashmap.values() {
            // Only cache bigger than 5MB files
            if file_entry.size >= minimal_cache_file_size {
                let string: String = format!("{}//{}//{}//{}", file_entry.path.display(), file_entry.size, file_entry.modified_date, file_entry.hash);

                if let Err(e) = writeln!(writer, "{}", string) {
                    text_messages.messages.push(format!("Failed to save some data to cache file {}, reason {}", cache_file.display(), e));
                    return;
                };
            }
        }
    }
}

pub trait MyHasher {
    fn update(&mut self, bytes: &[u8]);
    fn finalize(&self) -> String;
}

fn hash_calculation(buffer: &mut [u8], file_entry: &FileEntry, hash_type: &HashType, limit: u64) -> Result<(String, u64), String> {
    let mut file_handler = match File::open(&file_entry.path) {
        Ok(t) => t,
        Err(e) => return Err(format!("Unable to check hash of file {}, reason {}", file_entry.path.display(), e)),
    };
    let hasher = &mut *hash_type.hasher();
    let mut current_file_read_bytes: u64 = 0;
    loop {
        let n = match file_handler.read(buffer) {
            Ok(0) => break,
            Ok(t) => t,
            Err(e) => return Err(format!("Error happened when checking hash of file {}, reason {}", file_entry.path.display(), e)),
        };

        current_file_read_bytes += n as u64;
        hasher.update(&buffer[..n]);

        if current_file_read_bytes >= limit {
            break;
        }
    }
    Ok((hasher.finalize(), current_file_read_bytes))
}

fn load_hashes_from_file(text_messages: &mut Messages, type_of_hash: &HashType) -> Option<BTreeMap<u64, Vec<FileEntry>>> {
    if let Some(proj_dirs) = ProjectDirs::from("pl", "Qarmin", "Czkawka") {
        let cache_dir = PathBuf::from(proj_dirs.cache_dir());
        let cache_file = cache_dir.join(CACHE_FILE_NAME.replace(".", format!("_{:?}.", type_of_hash).as_str()));
        let file_handler = match OpenOptions::new().read(true).open(&cache_file) {
            Ok(t) => t,
            Err(_inspected) => {
                // text_messages.messages.push(format!("Cannot find or open cache file {}", cache_file.display())); // This shouldn't be write to output
                return None;
            }
        };

        let reader = BufReader::new(file_handler);

        let mut hashmap_loaded_entries: BTreeMap<u64, Vec<FileEntry>> = Default::default();

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
            if uuu.len() != 4 {
                text_messages
                    .warnings
                    .push(format!("Found invalid data(too much or too low amount of data) in line {} - ({}) in cache file {}", index + 1, line, cache_file.display()));
                continue;
            }
            // Don't load cache data if destination file not exists
            if Path::new(uuu[0]).exists() {
                let file_entry = FileEntry {
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
                    hash: uuu[3].to_string(),
                };
                hashmap_loaded_entries.entry(file_entry.size).or_insert_with(Vec::new);
                hashmap_loaded_entries.get_mut(&file_entry.size).unwrap().push(file_entry);
            }
        }

        return Some(hashmap_loaded_entries);
    }

    text_messages.messages.push("Cannot find or open system config dir to save cache file".to_string());
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{read_dir, File};
    use std::io;
    #[cfg(target_family = "windows")]
    use std::os::fs::MetadataExt;
    #[cfg(target_family = "unix")]
    use std::os::unix::fs::MetadataExt;

    #[cfg(target_family = "unix")]
    fn assert_inode(before: &Metadata, after: &Metadata) {
        assert_eq!(before.ino(), after.ino());
    }
    #[cfg(target_family = "windows")]
    fn assert_inode(_: &Metadata, _: &Metadata) {}

    #[test]
    fn test_make_hard_link() -> io::Result<()> {
        let dir = tempfile::Builder::new().tempdir()?;
        let (src, dst) = (dir.path().join("a"), dir.path().join("b"));
        File::create(&src)?;
        let metadata = fs::metadata(&src)?;
        File::create(&dst)?;

        make_hard_link(&src, &dst)?;

        assert_inode(&metadata, &fs::metadata(&dst)?);
        assert_eq!(metadata.permissions(), fs::metadata(&dst)?.permissions());
        assert_eq!(metadata.modified()?, fs::metadata(&dst)?.modified()?);
        assert_inode(&metadata, &fs::metadata(&src)?);
        assert_eq!(metadata.permissions(), fs::metadata(&src)?.permissions());
        assert_eq!(metadata.modified()?, fs::metadata(&src)?.modified()?);

        let mut actual = read_dir(&dir)?.map(|e| e.unwrap().path()).collect::<Vec<PathBuf>>();
        actual.sort();
        assert_eq!(vec![src, dst], actual);
        Ok(())
    }

    #[test]
    fn test_make_hard_link_fails() -> io::Result<()> {
        let dir = tempfile::Builder::new().tempdir()?;
        let (src, dst) = (dir.path().join("a"), dir.path().join("b"));
        File::create(&dst)?;
        let metadata = fs::metadata(&dst)?;

        assert!(make_hard_link(&src, &dst).is_err());

        assert_inode(&metadata, &fs::metadata(&dst)?);
        assert_eq!(metadata.permissions(), fs::metadata(&dst)?.permissions());
        assert_eq!(metadata.modified()?, fs::metadata(&dst)?.modified()?);

        assert_eq!(vec![dst], read_dir(&dir)?.map(|e| e.unwrap().path()).collect::<Vec<PathBuf>>());
        Ok(())
    }

    #[test]
    fn test_filter_hard_links_empty() {
        let expected: Vec<FileEntry> = Default::default();
        assert_eq!(expected, filter_hard_links(&[]));
    }

    #[cfg(target_family = "unix")]
    #[test]
    fn test_filter_hard_links() -> io::Result<()> {
        let dir = tempfile::Builder::new().tempdir()?;
        let (src, dst) = (dir.path().join("a"), dir.path().join("b"));
        File::create(&src)?;
        fs::hard_link(src.clone(), dst.clone())?;
        let e1 = FileEntry { path: src, ..Default::default() };
        let e2 = FileEntry { path: dst, ..Default::default() };
        let actual = filter_hard_links(&[e1.clone(), e2]);
        assert_eq!(vec![e1], actual);
        Ok(())
    }

    #[test]
    fn test_filter_hard_links_regular_files() -> io::Result<()> {
        let dir = tempfile::Builder::new().tempdir()?;
        let (src, dst) = (dir.path().join("a"), dir.path().join("b"));
        File::create(&src)?;
        File::create(&dst)?;
        let e1 = FileEntry { path: src, ..Default::default() };
        let e2 = FileEntry { path: dst, ..Default::default() };
        let actual = filter_hard_links(&[e1.clone(), e2.clone()]);
        assert_eq!(vec![e1, e2], actual);
        Ok(())
    }

    #[test]
    fn test_hash_calculation() -> io::Result<()> {
        let dir = tempfile::Builder::new().tempdir()?;
        let mut buf = [0u8; 1 << 10];
        let src = dir.path().join("a");
        let mut file = File::create(&src)?;
        file.write_all(b"aa")?;
        let e = FileEntry { path: src, ..Default::default() };
        let r = hash_calculation(&mut buf, &e, &HashType::Blake3, 0).unwrap();
        assert_eq!(2, r.1);
        assert!(!r.0.is_empty());
        Ok(())
    }

    #[test]
    fn test_hash_calculation_limit() -> io::Result<()> {
        let dir = tempfile::Builder::new().tempdir()?;
        let mut buf = [0u8; 1];
        let src = dir.path().join("a");
        let mut file = File::create(&src)?;
        file.write_all(b"aa")?;
        let e = FileEntry { path: src, ..Default::default() };
        let r1 = hash_calculation(&mut buf, &e, &HashType::Blake3, 1).unwrap();
        let r2 = hash_calculation(&mut buf, &e, &HashType::Blake3, 2).unwrap();
        let r3 = hash_calculation(&mut buf, &e, &HashType::Blake3, u64::MAX).unwrap();
        assert_ne!(r1, r2);
        assert_eq!(r2, r3);
        Ok(())
    }

    #[test]
    fn test_hash_calculation_invalid_file() -> io::Result<()> {
        let dir = tempfile::Builder::new().tempdir()?;
        let mut buf = [0u8; 1 << 10];
        let src = dir.path().join("a");
        let e = FileEntry { path: src, ..Default::default() };
        let r = hash_calculation(&mut buf, &e, &HashType::Blake3, 0).unwrap_err();
        assert!(!r.is_empty());
        Ok(())
    }
}
