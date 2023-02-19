use std::collections::BTreeMap;
#[cfg(target_family = "unix")]
use std::collections::HashSet;
use std::fs::File;
use std::hash::Hasher;
use std::io::prelude::*;
use std::io::{self, Error, ErrorKind};
use std::io::{BufReader, BufWriter};
#[cfg(target_family = "unix")]
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread::sleep;
use std::time::{Duration, SystemTime};
use std::{fs, mem, thread};

use crossbeam_channel::Receiver;
use humansize::format_size;
use humansize::BINARY;
use rayon::prelude::*;

use crate::common::{open_cache_folder, Common, LOOP_DURATION};
use crate::common_dir_traversal::{CheckingMethod, DirTraversalBuilder, DirTraversalResult, FileEntry, ProgressData};
use crate::common_directory::Directories;
use crate::common_extensions::Extensions;
use crate::common_items::ExcludedItems;
use crate::common_messages::Messages;
use crate::common_traits::*;
use crate::flc;
use crate::localizer_core::generate_translation_hashmap;

const TEMP_HARDLINK_FILE: &str = "rzeczek.rxrxrxl";

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

#[derive(Eq, PartialEq, Clone, Debug, Copy)]
pub enum DeleteMethod {
    None,
    AllExceptNewest,
    AllExceptOldest,
    OneOldest,
    OneNewest,
    HardLink,
}

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
}

impl Info {
    #[must_use]
    pub fn new() -> Self {
        Default::default()
    }
}

pub struct DuplicateFinder {
    text_messages: Messages,
    information: Info,
    files_with_identical_names: BTreeMap<String, Vec<FileEntry>>,                            // File Size, File Entry
    files_with_identical_size: BTreeMap<u64, Vec<FileEntry>>,                                // File Size, File Entry
    files_with_identical_hashes: BTreeMap<u64, Vec<Vec<FileEntry>>>,                         // File Size, next grouped by file size, next grouped by hash
    files_with_identical_names_referenced: BTreeMap<String, (FileEntry, Vec<FileEntry>)>,    // File Size, File Entry
    files_with_identical_size_referenced: BTreeMap<u64, (FileEntry, Vec<FileEntry>)>,        // File Size, File Entry
    files_with_identical_hashes_referenced: BTreeMap<u64, Vec<(FileEntry, Vec<FileEntry>)>>, // File Size, next grouped by file size, next grouped by hash
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
    use_prehash_cache: bool,
    minimal_cache_file_size: u64,
    minimal_prehash_cache_file_size: u64,
    delete_outdated_cache: bool,
    use_reference_folders: bool,
    case_sensitive_name_comparison: bool,
}

impl DuplicateFinder {
    #[must_use]
    pub fn new() -> Self {
        Self {
            text_messages: Messages::new(),
            information: Info::new(),
            files_with_identical_names: Default::default(),
            files_with_identical_size: Default::default(),
            files_with_identical_hashes: Default::default(),
            files_with_identical_names_referenced: Default::default(),
            files_with_identical_size_referenced: Default::default(),
            files_with_identical_hashes_referenced: Default::default(),
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
            use_prehash_cache: true,
            minimal_cache_file_size: 1024 * 1024 / 4, // By default cache only >= 256 KB files
            minimal_prehash_cache_file_size: 0,
            delete_outdated_cache: true,
            use_reference_folders: false,
            case_sensitive_name_comparison: false,
        }
    }

    pub fn find_duplicates(&mut self, stop_receiver: Option<&Receiver<()>>, progress_sender: Option<&futures::channel::mpsc::UnboundedSender<ProgressData>>) {
        self.directories.optimize_directories(self.recursive_search, &mut self.text_messages);
        self.use_reference_folders = !self.directories.reference_directories.is_empty();

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
            CheckingMethod::Hash => {
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

    pub fn set_delete_outdated_cache(&mut self, delete_outdated_cache: bool) {
        self.delete_outdated_cache = delete_outdated_cache;
    }

    pub fn set_case_sensitive_name_comparison(&mut self, case_sensitive_name_comparison: bool) {
        self.case_sensitive_name_comparison = case_sensitive_name_comparison;
    }

    #[must_use]
    pub const fn get_check_method(&self) -> &CheckingMethod {
        &self.check_method
    }

    #[must_use]
    pub fn get_stopped_search(&self) -> bool {
        self.stopped_search
    }

    pub fn set_minimal_cache_file_size(&mut self, minimal_cache_file_size: u64) {
        self.minimal_cache_file_size = minimal_cache_file_size;
    }

    pub fn set_minimal_prehash_cache_file_size(&mut self, minimal_prehash_cache_file_size: u64) {
        self.minimal_prehash_cache_file_size = minimal_prehash_cache_file_size;
    }

    #[must_use]
    pub const fn get_files_sorted_by_names(&self) -> &BTreeMap<String, Vec<FileEntry>> {
        &self.files_with_identical_names
    }

    pub fn set_use_cache(&mut self, use_cache: bool) {
        self.use_cache = use_cache;
    }

    pub fn set_use_prehash_cache(&mut self, use_prehash_cache: bool) {
        self.use_prehash_cache = use_prehash_cache;
    }

    #[must_use]
    pub const fn get_files_sorted_by_size(&self) -> &BTreeMap<u64, Vec<FileEntry>> {
        &self.files_with_identical_size
    }

    #[must_use]
    pub const fn get_files_sorted_by_hash(&self) -> &BTreeMap<u64, Vec<Vec<FileEntry>>> {
        &self.files_with_identical_hashes
    }
    pub fn set_maximal_file_size(&mut self, maximal_file_size: u64) {
        self.maximal_file_size = match maximal_file_size {
            0 => 1,
            t => t,
        };
    }

    #[must_use]
    pub const fn get_text_messages(&self) -> &Messages {
        &self.text_messages
    }

    #[must_use]
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

    #[must_use]
    pub fn get_use_reference(&self) -> bool {
        self.use_reference_folders
    }

    pub fn set_recursive_search(&mut self, recursive_search: bool) {
        self.recursive_search = recursive_search;
    }

    #[cfg(target_family = "unix")]
    pub fn set_exclude_other_filesystems(&mut self, exclude_other_filesystems: bool) {
        self.directories.set_exclude_other_filesystems(exclude_other_filesystems);
    }
    #[cfg(not(target_family = "unix"))]
    pub fn set_exclude_other_filesystems(&mut self, _exclude_other_filesystems: bool) {}

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
    pub fn set_allowed_extensions(&mut self, allowed_extensions: String) {
        self.allowed_extensions.set_allowed_extensions(allowed_extensions, &mut self.text_messages);
    }

    #[must_use]
    pub fn get_files_with_identical_hashes_referenced(&self) -> &BTreeMap<u64, Vec<(FileEntry, Vec<FileEntry>)>> {
        &self.files_with_identical_hashes_referenced
    }

    #[must_use]
    pub fn get_files_with_identical_name_referenced(&self) -> &BTreeMap<String, (FileEntry, Vec<FileEntry>)> {
        &self.files_with_identical_names_referenced
    }

    #[must_use]
    pub fn get_files_with_identical_size_referenced(&self) -> &BTreeMap<u64, (FileEntry, Vec<FileEntry>)> {
        &self.files_with_identical_size_referenced
    }

    fn check_files_name(&mut self, stop_receiver: Option<&Receiver<()>>, progress_sender: Option<&futures::channel::mpsc::UnboundedSender<ProgressData>>) -> bool {
        let group_by_func = if self.case_sensitive_name_comparison {
            |fe: &FileEntry| fe.path.file_name().unwrap().to_string_lossy().to_string()
        } else {
            |fe: &FileEntry| fe.path.file_name().unwrap().to_string_lossy().to_lowercase()
        };

        let result = DirTraversalBuilder::new()
            .root_dirs(self.directories.included_directories.clone())
            .group_by(group_by_func)
            .stop_receiver(stop_receiver)
            .progress_sender(progress_sender)
            .checking_method(CheckingMethod::Name)
            .directories(self.directories.clone())
            .allowed_extensions(self.allowed_extensions.clone())
            .excluded_items(self.excluded_items.clone())
            .recursive_search(self.recursive_search)
            .minimal_file_size(self.minimal_file_size)
            .maximal_file_size(self.maximal_file_size)
            .build()
            .run();
        match result {
            DirTraversalResult::SuccessFiles {
                start_time,
                grouped_file_entries,
                warnings,
            } => {
                self.files_with_identical_names = grouped_file_entries;
                self.text_messages.warnings.extend(warnings);

                // Create new BTreeMap without single size entries(files have not duplicates)
                let mut new_map: BTreeMap<String, Vec<FileEntry>> = Default::default();

                for (name, vector) in &self.files_with_identical_names {
                    if vector.len() > 1 {
                        new_map.insert(name.clone(), vector.clone());
                    }
                }
                self.files_with_identical_names = new_map;

                // Reference - only use in size, because later hash will be counted differently
                if self.use_reference_folders {
                    let mut btree_map = Default::default();
                    mem::swap(&mut self.files_with_identical_names, &mut btree_map);
                    let reference_directories = self.directories.reference_directories.clone();
                    let vec = btree_map
                        .into_iter()
                        .filter_map(|(_size, vec_file_entry)| {
                            let mut files_from_referenced_folders = Vec::new();
                            let mut normal_files = Vec::new();
                            for file_entry in vec_file_entry {
                                if reference_directories.iter().any(|e| file_entry.path.starts_with(e)) {
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
                    for (fe, vec_fe) in vec {
                        self.files_with_identical_names_referenced.insert(fe.path.to_string_lossy().to_string(), (fe, vec_fe));
                    }
                }

                if self.use_reference_folders {
                    for (_fe, vector) in self.files_with_identical_names_referenced.values() {
                        self.information.number_of_duplicated_files_by_name += vector.len();
                        self.information.number_of_groups_by_name += 1;
                    }
                } else {
                    for vector in self.files_with_identical_names.values() {
                        self.information.number_of_duplicated_files_by_name += vector.len() - 1;
                        self.information.number_of_groups_by_name += 1;
                    }
                }

                Common::print_time(start_time, SystemTime::now(), "check_files_name");
                true
            }
            DirTraversalResult::SuccessFolders { .. } => {
                unreachable!()
            }
            DirTraversalResult::Stopped => false,
        }
    }

    /// Read file length and puts it to different boxes(each for different lengths)
    /// If in box is only 1 result, then it is removed
    fn check_files_size(&mut self, stop_receiver: Option<&Receiver<()>>, progress_sender: Option<&futures::channel::mpsc::UnboundedSender<ProgressData>>) -> bool {
        let max_stage = match self.check_method {
            CheckingMethod::Size => 0,
            CheckingMethod::Hash => 2,
            _ => panic!(),
        };
        let result = DirTraversalBuilder::new()
            .root_dirs(self.directories.included_directories.clone())
            .group_by(|fe| fe.size)
            .stop_receiver(stop_receiver)
            .progress_sender(progress_sender)
            .checking_method(self.check_method)
            .max_stage(max_stage)
            .directories(self.directories.clone())
            .allowed_extensions(self.allowed_extensions.clone())
            .excluded_items(self.excluded_items.clone())
            .recursive_search(self.recursive_search)
            .minimal_file_size(self.minimal_file_size)
            .maximal_file_size(self.maximal_file_size)
            .build()
            .run();
        match result {
            DirTraversalResult::SuccessFiles {
                start_time,
                grouped_file_entries,
                warnings,
            } => {
                self.files_with_identical_size = grouped_file_entries;
                self.text_messages.warnings.extend(warnings);

                // Create new BTreeMap without single size entries(files have not duplicates)
                let mut old_map: BTreeMap<u64, Vec<FileEntry>> = Default::default();
                mem::swap(&mut old_map, &mut self.files_with_identical_size);

                for (size, vec) in old_map {
                    if vec.len() <= 1 {
                        continue;
                    }

                    let vector = if self.ignore_hard_links { filter_hard_links(&vec) } else { vec };

                    if vector.len() > 1 {
                        self.files_with_identical_size.insert(size, vector);
                    }
                }

                // Reference - only use in size, because later hash will be counted differently
                if self.use_reference_folders && self.check_method == CheckingMethod::Size {
                    let mut btree_map = Default::default();
                    mem::swap(&mut self.files_with_identical_size, &mut btree_map);
                    let reference_directories = self.directories.reference_directories.clone();
                    let vec = btree_map
                        .into_iter()
                        .filter_map(|(_size, vec_file_entry)| {
                            let mut files_from_referenced_folders = Vec::new();
                            let mut normal_files = Vec::new();
                            for file_entry in vec_file_entry {
                                if reference_directories.iter().any(|e| file_entry.path.starts_with(e)) {
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
                    for (fe, vec_fe) in vec {
                        self.files_with_identical_size_referenced.insert(fe.size, (fe, vec_fe));
                    }
                }

                if self.use_reference_folders {
                    for (size, (_fe, vector)) in &self.files_with_identical_size_referenced {
                        self.information.number_of_duplicated_files_by_size += vector.len();
                        self.information.number_of_groups_by_size += 1;
                        self.information.lost_space_by_size += (vector.len() as u64) * size;
                    }
                } else {
                    for (size, vector) in &self.files_with_identical_size {
                        self.information.number_of_duplicated_files_by_size += vector.len() - 1;
                        self.information.number_of_groups_by_size += 1;
                        self.information.lost_space_by_size += (vector.len() as u64 - 1) * size;
                    }
                }

                Common::print_time(start_time, SystemTime::now(), "check_files_size");
                true
            }
            DirTraversalResult::SuccessFolders { .. } => {
                unreachable!()
            }
            DirTraversalResult::Stopped => false,
        }
    }

    /// The slowest checking type, which must be applied after checking for size
    fn check_files_hash(&mut self, stop_receiver: Option<&Receiver<()>>, progress_sender: Option<&futures::channel::mpsc::UnboundedSender<ProgressData>>) -> bool {
        assert_eq!(self.check_method, CheckingMethod::Hash);

        let check_type = Arc::new(self.hash_type);

        let start_time: SystemTime = SystemTime::now();
        let check_was_stopped = AtomicBool::new(false); // Used for breaking from GUI and ending check thread
        let mut pre_checked_map: BTreeMap<u64, Vec<FileEntry>> = Default::default();

        //// PROGRESS THREAD START
        let progress_thread_run = Arc::new(AtomicBool::new(true));

        let atomic_file_counter = Arc::new(AtomicUsize::new(0));

        let progress_thread_handle = if let Some(progress_sender) = progress_sender {
            let progress_send = progress_sender.clone();
            let progress_thread_run = progress_thread_run.clone();
            let atomic_file_counter = atomic_file_counter.clone();
            let files_to_check = self.files_with_identical_size.values().map(Vec::len).sum();
            let checking_method = self.check_method;
            thread::spawn(move || loop {
                progress_send
                    .unbounded_send(ProgressData {
                        checking_method,
                        current_stage: 1,
                        max_stage: 2,
                        entries_checked: atomic_file_counter.load(Ordering::Relaxed),
                        entries_to_check: files_to_check,
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

        ///////////////////////////////////////////////////////////////////////////// PREHASHING START
        {
            let loaded_hash_map;
            let mut records_already_cached: BTreeMap<u64, Vec<FileEntry>> = Default::default();
            let mut non_cached_files_to_check: BTreeMap<u64, Vec<FileEntry>> = Default::default();

            // Cache algorithm
            // - Load data from cache
            // - Convert from BT<u64,Vec<FileEntry>> to BT<String,FileEntry>
            // - Save to proper values
            if self.use_prehash_cache {
                loaded_hash_map = match load_hashes_from_file(&mut self.text_messages, self.delete_outdated_cache, &self.hash_type, true) {
                    Some(t) => t,
                    None => Default::default(),
                };

                let mut loaded_hash_map2: BTreeMap<String, FileEntry> = Default::default();
                for vec_file_entry in loaded_hash_map.values() {
                    for file_entry in vec_file_entry {
                        loaded_hash_map2.insert(file_entry.path.to_string_lossy().to_string(), file_entry.clone());
                    }
                }

                #[allow(clippy::if_same_then_else)]
                for vec_file_entry in self.files_with_identical_size.values() {
                    for file_entry in vec_file_entry {
                        let name = file_entry.path.to_string_lossy().to_string();
                        if !loaded_hash_map2.contains_key(&name) {
                            // If loaded data doesn't contains current image info
                            non_cached_files_to_check.entry(file_entry.size).or_insert_with(Vec::new).push(file_entry.clone());
                        } else if file_entry.size != loaded_hash_map2.get(&name).unwrap().size || file_entry.modified_date != loaded_hash_map2.get(&name).unwrap().modified_date {
                            // When size or modification date of image changed, then it is clear that is different image
                            non_cached_files_to_check.entry(file_entry.size).or_insert_with(Vec::new).push(file_entry.clone());
                        } else {
                            // Checking may be omitted when already there is entry with same size and modification date
                            records_already_cached.entry(file_entry.size).or_insert_with(Vec::new).push(file_entry.clone());
                        }
                    }
                }
            } else {
                loaded_hash_map = Default::default();
                mem::swap(&mut self.files_with_identical_size, &mut non_cached_files_to_check);
            }

            #[allow(clippy::type_complexity)]
            let pre_hash_results: Vec<(u64, BTreeMap<String, Vec<FileEntry>>, Vec<String>)> = non_cached_files_to_check
                .par_iter()
                .map(|(size, vec_file_entry)| {
                    let mut hashmap_with_hash: BTreeMap<String, Vec<FileEntry>> = Default::default();
                    let mut errors: Vec<String> = Vec::new();
                    let mut buffer = [0u8; 1024 * 2];

                    atomic_file_counter.fetch_add(vec_file_entry.len(), Ordering::Relaxed);
                    for file_entry in vec_file_entry {
                        if stop_receiver.is_some() && stop_receiver.unwrap().try_recv().is_ok() {
                            check_was_stopped.store(true, Ordering::Relaxed);
                            return None;
                        }
                        match hash_calculation(&mut buffer, file_entry, &check_type, 0) {
                            Ok(hash_string) => {
                                hashmap_with_hash.entry(hash_string.clone()).or_insert_with(Vec::new).push(file_entry.clone());
                            }
                            Err(s) => errors.push(s),
                        }
                    }
                    Some((*size, hashmap_with_hash, errors))
                })
                .while_some()
                .collect();

            // End thread which send info to gui
            progress_thread_run.store(false, Ordering::Relaxed);
            progress_thread_handle.join().unwrap();

            // Check if user aborted search(only from GUI)
            if check_was_stopped.load(Ordering::Relaxed) {
                return false;
            }

            // Add data from cache
            for (size, vec_file_entry) in &records_already_cached {
                pre_checked_map.entry(*size).or_insert_with(Vec::new).append(&mut vec_file_entry.clone());
            }

            // Check results
            for (size, hash_map, errors) in &pre_hash_results {
                self.text_messages.warnings.append(&mut errors.clone());
                for vec_file_entry in hash_map.values() {
                    if vec_file_entry.len() > 1 {
                        pre_checked_map.entry(*size).or_insert_with(Vec::new).append(&mut vec_file_entry.clone());
                    }
                }
            }

            if self.use_prehash_cache {
                // All results = records already cached + computed results
                let mut save_cache_to_hashmap: BTreeMap<String, FileEntry> = Default::default();

                for (size, vec_file_entry) in loaded_hash_map {
                    if size >= self.minimal_prehash_cache_file_size {
                        for file_entry in vec_file_entry {
                            save_cache_to_hashmap.insert(file_entry.path.to_string_lossy().to_string(), file_entry.clone());
                        }
                    }
                }

                for (size, hash_map, _errors) in &pre_hash_results {
                    if *size >= self.minimal_prehash_cache_file_size {
                        for vec_file_entry in hash_map.values() {
                            for file_entry in vec_file_entry {
                                save_cache_to_hashmap.insert(file_entry.path.to_string_lossy().to_string(), file_entry.clone());
                            }
                        }
                    }
                }

                save_hashes_to_file(&save_cache_to_hashmap, &mut self.text_messages, &self.hash_type, true, self.minimal_prehash_cache_file_size);
            }
        }

        ///////////////////////////////////////////////////////////////////////////// PREHASHING END

        Common::print_time(start_time, SystemTime::now(), "check_files_hash - prehash");
        let start_time: SystemTime = SystemTime::now();

        /////////////////////////

        //// PROGRESS THREAD START
        let progress_thread_run = Arc::new(AtomicBool::new(true));

        let atomic_file_counter = Arc::new(AtomicUsize::new(0));

        let progress_thread_handle = if let Some(progress_sender) = progress_sender {
            let progress_send = progress_sender.clone();
            let progress_thread_run = progress_thread_run.clone();
            let atomic_file_counter = atomic_file_counter.clone();
            let files_to_check = pre_checked_map.values().map(Vec::len).sum();
            let checking_method = self.check_method;
            thread::spawn(move || loop {
                progress_send
                    .unbounded_send(ProgressData {
                        checking_method,
                        current_stage: 2,
                        max_stage: 2,
                        entries_checked: atomic_file_counter.load(Ordering::Relaxed),
                        entries_to_check: files_to_check,
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

        ///////////////////////////////////////////////////////////////////////////// HASHING START
        {
            #[allow(clippy::type_complexity)]
            let mut full_hash_results: Vec<(u64, BTreeMap<String, Vec<FileEntry>>, Vec<String>)>;

            let loaded_hash_map;

            let mut records_already_cached: BTreeMap<u64, Vec<FileEntry>> = Default::default();
            let mut non_cached_files_to_check: BTreeMap<u64, Vec<FileEntry>> = Default::default();

            if self.use_cache {
                loaded_hash_map = match load_hashes_from_file(&mut self.text_messages, self.delete_outdated_cache, &self.hash_type, false) {
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
                                    records_already_cached.entry(file_entry.size).or_insert_with(Vec::new).push(loaded_file_entry.clone());
                                    found = true;
                                    break;
                                }
                            }

                            if !found {
                                non_cached_files_to_check.entry(file_entry.size).or_insert_with(Vec::new).push(file_entry);
                            }
                        }
                    }
                }
            } else {
                loaded_hash_map = Default::default();
                mem::swap(&mut pre_checked_map, &mut non_cached_files_to_check);
            }

            full_hash_results = non_cached_files_to_check
                .into_par_iter()
                .map(|(size, vec_file_entry)| {
                    let mut hashmap_with_hash: BTreeMap<String, Vec<FileEntry>> = Default::default();
                    let mut errors: Vec<String> = Vec::new();
                    let mut buffer = [0u8; 1024 * 16];

                    atomic_file_counter.fetch_add(vec_file_entry.len(), Ordering::Relaxed);
                    for mut file_entry in vec_file_entry {
                        if stop_receiver.is_some() && stop_receiver.unwrap().try_recv().is_ok() {
                            check_was_stopped.store(true, Ordering::Relaxed);
                            return None;
                        }

                        match hash_calculation(&mut buffer, &file_entry, &check_type, u64::MAX) {
                            Ok(hash_string) => {
                                file_entry.hash = hash_string.clone();
                                hashmap_with_hash.entry(hash_string.clone()).or_insert_with(Vec::new).push(file_entry);
                            }
                            Err(s) => errors.push(s),
                        }
                    }
                    Some((size, hashmap_with_hash, errors))
                })
                .while_some()
                .collect();

            if self.use_cache {
                'main: for (size, vec_file_entry) in records_already_cached {
                    // Check if size already exists, if exists we must to change it outside because cannot have mut and non mut reference to full_hash_results
                    for (full_size, full_hashmap, _errors) in &mut full_hash_results {
                        if size == *full_size {
                            for file_entry in vec_file_entry {
                                full_hashmap.entry(file_entry.hash.clone()).or_insert_with(Vec::new).push(file_entry);
                            }
                            continue 'main;
                        }
                    }
                    // Size doesn't exists add results to files
                    let mut temp_hashmap: BTreeMap<String, Vec<FileEntry>> = Default::default();
                    for file_entry in vec_file_entry {
                        temp_hashmap.entry(file_entry.hash.clone()).or_insert_with(Vec::new).push(file_entry);
                    }
                    full_hash_results.push((size, temp_hashmap, Vec::new()));
                }

                // Must save all results to file, old loaded from file with all currently counted results
                let mut all_results: BTreeMap<String, FileEntry> = Default::default();
                for (_size, vec_file_entry) in loaded_hash_map {
                    for file_entry in vec_file_entry {
                        all_results.insert(file_entry.path.to_string_lossy().to_string(), file_entry);
                    }
                }
                for (_size, hashmap, _errors) in &full_hash_results {
                    for vec_file_entry in hashmap.values() {
                        for file_entry in vec_file_entry {
                            all_results.insert(file_entry.path.to_string_lossy().to_string(), file_entry.clone());
                        }
                    }
                }
                save_hashes_to_file(&all_results, &mut self.text_messages, &self.hash_type, false, self.minimal_cache_file_size);
            }

            // End thread which send info to gui
            progress_thread_run.store(false, Ordering::Relaxed);
            progress_thread_handle.join().unwrap();

            // Break if stop was clicked after saving to cache
            if check_was_stopped.load(Ordering::Relaxed) {
                return false;
            }

            for (size, hash_map, mut errors) in full_hash_results {
                self.text_messages.warnings.append(&mut errors);
                for (_hash, vec_file_entry) in hash_map {
                    if vec_file_entry.len() > 1 {
                        self.files_with_identical_hashes.entry(size).or_insert_with(Vec::new).push(vec_file_entry);
                    }
                }
            }
        }

        ///////////////////////////////////////////////////////////////////////////// HASHING END

        // Reference - only use in size, because later hash will be counted differently
        if self.use_reference_folders {
            let mut btree_map = Default::default();
            mem::swap(&mut self.files_with_identical_hashes, &mut btree_map);
            let reference_directories = self.directories.reference_directories.clone();
            let vec = btree_map
                .into_iter()
                .filter_map(|(_size, vec_vec_file_entry)| {
                    let mut all_results_with_same_size = Vec::new();
                    for vec_file_entry in vec_vec_file_entry {
                        let mut files_from_referenced_folders = Vec::new();
                        let mut normal_files = Vec::new();
                        for file_entry in vec_file_entry {
                            if reference_directories.iter().any(|e| file_entry.path.starts_with(e)) {
                                files_from_referenced_folders.push(file_entry);
                            } else {
                                normal_files.push(file_entry);
                            }
                        }

                        if files_from_referenced_folders.is_empty() || normal_files.is_empty() {
                            continue;
                        }
                        all_results_with_same_size.push((files_from_referenced_folders.pop().unwrap(), normal_files));
                    }
                    if all_results_with_same_size.is_empty() {
                        None
                    } else {
                        Some(all_results_with_same_size)
                    }
                })
                .collect::<Vec<Vec<(FileEntry, Vec<FileEntry>)>>>();
            for vec_of_vec in vec {
                self.files_with_identical_hashes_referenced.insert(vec_of_vec[0].0.size, vec_of_vec);
            }
        }

        if self.use_reference_folders {
            for (size, vector_vectors) in &self.files_with_identical_hashes_referenced {
                for (_fe, vector) in vector_vectors {
                    self.information.number_of_duplicated_files_by_hash += vector.len();
                    self.information.number_of_groups_by_hash += 1;
                    self.information.lost_space_by_hash += (vector.len() as u64) * size;
                }
            }
        } else {
            for (size, vector_vectors) in &self.files_with_identical_hashes {
                for vector in vector_vectors {
                    self.information.number_of_duplicated_files_by_hash += vector.len() - 1;
                    self.information.number_of_groups_by_hash += 1;
                    self.information.lost_space_by_hash += (vector.len() as u64 - 1) * size;
                }
            }
        }

        Common::print_time(start_time, SystemTime::now(), "check_files_hash - full hash");

        // Clean unused data
        self.files_with_identical_size = Default::default();

        true
    }

    /// Function to delete files, from filed before `BTreeMap`
    /// Using another function to delete files to avoid duplicates data
    fn delete_files(&mut self) {
        let start_time: SystemTime = SystemTime::now();
        if self.delete_method == DeleteMethod::None {
            return;
        }

        match self.check_method {
            CheckingMethod::Name => {
                for vector in self.files_with_identical_names.values() {
                    let _tuple: (u64, usize, usize) = delete_files(vector, &self.delete_method, &mut self.text_messages, self.dryrun);
                }
            }
            CheckingMethod::Hash => {
                for vector_vectors in self.files_with_identical_hashes.values() {
                    for vector in vector_vectors.iter() {
                        let _tuple: (u64, usize, usize) = delete_files(vector, &self.delete_method, &mut self.text_messages, self.dryrun);
                    }
                }
            }
            CheckingMethod::Size => {
                for vector in self.files_with_identical_size.values() {
                    let _tuple: (u64, usize, usize) = delete_files(vector, &self.delete_method, &mut self.text_messages, self.dryrun);
                }
            }
            CheckingMethod::None => {
                //Just do nothing
                panic!("Checking method should never be none.");
            }
        }

        Common::print_time(start_time, SystemTime::now(), "delete_files");
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
        println!(
            "Lost space by size - {} ({} bytes)",
            format_size(self.information.lost_space_by_size, BINARY),
            self.information.lost_space_by_size
        );
        println!(
            "Lost space by hash - {} ({} bytes)",
            format_size(self.information.lost_space_by_hash, BINARY),
            self.information.lost_space_by_hash
        );

        println!("### Other");

        println!("Files list size - {}", self.files_with_identical_size.len());
        println!("Hashed Files list size - {}", self.files_with_identical_hashes.len());
        println!("Excluded items - {:?}", self.excluded_items.items);
        println!("Included directories - {:?}", self.directories.included_directories);
        println!("Excluded directories - {:?}", self.directories.excluded_directories);
        println!("Recursive search - {}", self.recursive_search);
        #[cfg(target_family = "unix")]
        println!("Skip other filesystems - {}", self.directories.exclude_other_filesystems());
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
                self.text_messages.errors.push(format!("Failed to create file {file_name}, reason {e}"));
                return false;
            }
        };
        let mut writer = BufWriter::new(file_handler);

        if let Err(e) = writeln!(
            writer,
            "Results of searching {:?} with excluded directories {:?} and excluded items {:?}",
            self.directories.included_directories, self.directories.excluded_directories, self.excluded_items.items
        ) {
            self.text_messages.errors.push(format!("Failed to save results to file {file_name}, reason {e}"));
            return false;
        }
        match self.check_method {
            CheckingMethod::Name => {
                if !self.files_with_identical_names.is_empty() {
                    writeln!(
                        writer,
                        "-------------------------------------------------Files with same names-------------------------------------------------"
                    )
                    .unwrap();
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
                    writeln!(
                        writer,
                        "-------------------------------------------------Files with same size-------------------------------------------------"
                    )
                    .unwrap();
                    writeln!(
                        writer,
                        "Found {} duplicated files which in {} groups which takes {}.",
                        self.information.number_of_duplicated_files_by_size,
                        self.information.number_of_groups_by_size,
                        format_size(self.information.lost_space_by_size, BINARY)
                    )
                    .unwrap();
                    for (size, vector) in self.files_with_identical_size.iter().rev() {
                        write!(writer, "\n---- Size {} ({}) - {} files \n", format_size(*size, BINARY), size, vector.len()).unwrap();
                        for file_entry in vector {
                            writeln!(writer, "{}", file_entry.path.display()).unwrap();
                        }
                    }
                } else {
                    write!(writer, "Not found any duplicates.").unwrap();
                }
            }
            CheckingMethod::Hash => {
                if !self.files_with_identical_hashes.is_empty() {
                    writeln!(
                        writer,
                        "-------------------------------------------------Files with same hashes-------------------------------------------------"
                    )
                    .unwrap();
                    writeln!(
                        writer,
                        "Found {} duplicated files which in {} groups which takes {}.",
                        self.information.number_of_duplicated_files_by_hash,
                        self.information.number_of_groups_by_hash,
                        format_size(self.information.lost_space_by_hash, BINARY)
                    )
                    .unwrap();
                    for (size, vectors_vector) in self.files_with_identical_hashes.iter().rev() {
                        for vector in vectors_vector {
                            writeln!(writer, "\n---- Size {} ({}) - {} files", format_size(*size, BINARY), size, vector.len()).unwrap();
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
        Common::print_time(start_time, SystemTime::now(), "save_results_to_file");
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
                println!("Found {number_of_files} files in {number_of_groups} groups with same name(may have different content)",);
                for (name, vector) in &self.files_with_identical_names {
                    println!("Name - {} - {} files ", name, vector.len());
                    for j in vector {
                        println!("{}", j.path.display());
                    }
                    println!();
                }
            }
            CheckingMethod::Hash => {
                for vector in self.files_with_identical_hashes.values() {
                    for j in vector {
                        number_of_files += j.len() as u64;
                        number_of_groups += 1;
                    }
                }
                println!(
                    "Found {} duplicated files in {} groups with same content which took {}:",
                    number_of_files,
                    number_of_groups,
                    format_size(self.information.lost_space_by_size, BINARY)
                );
                for (size, vector) in self.files_with_identical_hashes.iter().rev() {
                    for j in vector {
                        println!("Size - {} ({}) - {} files ", format_size(*size, BINARY), size, j.len());
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
                    format_size(self.information.lost_space_by_size, BINARY)
                );
                for (size, vector) in &self.files_with_identical_size {
                    println!("Size - {} ({}) - {} files ", format_size(*size, BINARY), size, vector.len());
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
        Common::print_time(start_time, SystemTime::now(), "print_entries");
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
    let q_index = q_index.map_or(0, |t| t.0);
    let n = match delete_method {
        DeleteMethod::OneNewest | DeleteMethod::OneOldest => 1,
        DeleteMethod::AllExceptNewest | DeleteMethod::AllExceptOldest | DeleteMethod::None | DeleteMethod::HardLink => usize::MAX,
    };
    for (index, file) in vector.iter().enumerate() {
        if q_index == index {
            continue;
        }
        if removed_files + failed_to_remove_files >= n {
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
    let temp = dst_dir.join(TEMP_HARDLINK_FILE);
    fs::rename(dst, temp.as_path())?;
    let result = fs::hard_link(src, dst);
    if result.is_err() {
        fs::rename(temp.as_path(), dst)?;
    }
    fs::remove_file(temp)?;
    result
}

pub fn save_hashes_to_file(hashmap: &BTreeMap<String, FileEntry>, text_messages: &mut Messages, type_of_hash: &HashType, is_prehash: bool, minimal_cache_file_size: u64) {
    if let Some(((file_handler, cache_file), (_json_file, _json_name))) = open_cache_folder(&get_file_hash_name(type_of_hash, is_prehash), true, false, &mut text_messages.warnings)
    {
        let mut writer = BufWriter::new(file_handler.unwrap()); // Unwrap cannot fail

        let mut how_much = 0;
        for file_entry in hashmap.values() {
            if file_entry.size >= minimal_cache_file_size {
                let string: String = format!("{}//{}//{}//{}", file_entry.path.display(), file_entry.size, file_entry.modified_date, file_entry.hash);

                if let Err(e) = writeln!(writer, "{string}") {
                    text_messages
                        .warnings
                        .push(format!("Failed to save some data to cache file {}, reason {}", cache_file.display(), e));
                    return;
                }
                how_much += 1;
            }
        }

        text_messages
            .messages
            .push(flc!("core_saving_to_cache", generate_translation_hashmap(vec![("number", how_much.to_string())])));
    }
}

pub fn load_hashes_from_file(text_messages: &mut Messages, delete_outdated_cache: bool, type_of_hash: &HashType, is_prehash: bool) -> Option<BTreeMap<u64, Vec<FileEntry>>> {
    if let Some(((file_handler, cache_file), (_json_file, _json_name))) =
        open_cache_folder(&get_file_hash_name(type_of_hash, is_prehash), false, false, &mut text_messages.warnings)
    {
        // Unwrap could fail when failed to open cache file, but json would exists
        let file_handler = match file_handler {
            Some(t) => t,
            _ => return Default::default(),
        };
        let reader = BufReader::new(file_handler);

        let mut hashmap_loaded_entries: BTreeMap<u64, Vec<FileEntry>> = Default::default();

        // Read the file line by line using the lines() iterator from std::io::BufRead.
        for (index, line) in reader.lines().enumerate() {
            let line = match line {
                Ok(t) => t,
                Err(e) => {
                    text_messages
                        .warnings
                        .push(format!("Failed to load line number {} from cache file {}, reason {}", index + 1, cache_file.display(), e));
                    return None;
                }
            };
            let uuu = line.split("//").collect::<Vec<&str>>();
            if uuu.len() != 4 {
                text_messages.warnings.push(format!(
                    "Found invalid data(too much or too low amount of data) in line {} - ({}) in cache file {}",
                    index + 1,
                    line,
                    cache_file.display()
                ));
                continue;
            }
            // Don't load cache data if destination file not exists
            if !delete_outdated_cache || Path::new(uuu[0]).exists() {
                let file_entry = FileEntry {
                    path: PathBuf::from(uuu[0]),
                    size: match uuu[1].parse::<u64>() {
                        Ok(t) => t,
                        Err(e) => {
                            text_messages.warnings.push(format!(
                                "Found invalid size value in line {} - ({}) in cache file {}, reason {}",
                                index + 1,
                                line,
                                cache_file.display(),
                                e
                            ));
                            continue;
                        }
                    },
                    modified_date: match uuu[2].parse::<u64>() {
                        Ok(t) => t,
                        Err(e) => {
                            text_messages.warnings.push(format!(
                                "Found invalid modified date value in line {} - ({}) in cache file {}, reason {}",
                                index + 1,
                                line,
                                cache_file.display(),
                                e
                            ));
                            continue;
                        }
                    },
                    hash: uuu[3].to_string(),
                    symlink_info: None,
                };
                hashmap_loaded_entries.entry(file_entry.size).or_insert_with(Vec::new).push(file_entry);
            }
        }

        text_messages.messages.push(flc!(
            "core_loading_from_cache",
            generate_translation_hashmap(vec![("number", hashmap_loaded_entries.values().map(std::vec::Vec::len).sum::<usize>().to_string())])
        ));

        return Some(hashmap_loaded_entries);
    }
    None
}

pub trait MyHasher {
    fn update(&mut self, bytes: &[u8]);
    fn finalize(&self) -> String;
}

fn hash_calculation(buffer: &mut [u8], file_entry: &FileEntry, hash_type: &HashType, limit: u64) -> Result<String, String> {
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
    Ok(hasher.finalize())
}

fn get_file_hash_name(type_of_hash: &HashType, is_prehash: bool) -> String {
    let prehash_str = if is_prehash { "_prehash" } else { "" };
    format!("cache_duplicates_{type_of_hash:?}{prehash_str}.txt")
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

#[cfg(test)]
mod tests {
    use std::fs::{read_dir, File, Metadata};
    use std::io;
    #[cfg(target_family = "windows")]
    use std::os::fs::MetadataExt;
    #[cfg(target_family = "unix")]
    use std::os::unix::fs::MetadataExt;

    use super::*;

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
        actual.sort_unstable();
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
        assert!(!r.is_empty());
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
