use std::collections::{BTreeMap, HashSet};
use std::fs::File;
use std::hash::Hasher;
use std::io::prelude::*;
use std::io::{self, BufWriter, Error, ErrorKind};
#[cfg(target_family = "unix")]
use std::os::unix::fs::MetadataExt;
use std::path::Path;
use std::sync::atomic::Ordering;
use std::{fs, mem};

use crossbeam_channel::Receiver;
use fun_time::fun_time;
use futures::channel::mpsc::UnboundedSender;
use humansize::{format_size, BINARY};
use log::debug;
use rayon::prelude::*;
use xxhash_rust::xxh3::Xxh3;

use crate::common::{prepare_thread_handler_common, send_info_and_wait_for_ending_all_threads};
use crate::common_cache::{get_duplicate_cache_file, load_cache_from_file_generalized_by_size, save_cache_to_file_generalized};
use crate::common_dir_traversal::{CheckingMethod, DirTraversalBuilder, DirTraversalResult, FileEntry, ProgressData, ToolType};
use crate::common_messages::Messages;
use crate::common_tool::{CommonData, CommonToolData};
use crate::common_traits::*;

const TEMP_HARDLINK_FILE: &str = "rzeczek.rxrxrxl";

#[derive(PartialEq, Eq, Clone, Debug, Copy, Default)]
pub enum HashType {
    #[default]
    Blake3,
    Crc32,
    Xxh3,
}

impl HashType {
    fn hasher(self: &HashType) -> Box<dyn MyHasher> {
        match self {
            HashType::Blake3 => Box::new(blake3::Hasher::new()),
            HashType::Crc32 => Box::new(crc32fast::Hasher::new()),
            HashType::Xxh3 => Box::new(Xxh3::new()),
        }
    }
}

#[derive(Eq, PartialEq, Clone, Debug, Copy, Default)]
pub enum DeleteMethod {
    #[default]
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
    pub number_of_groups_by_size_name: usize,
    pub number_of_duplicated_files_by_size_name: usize,
    pub lost_space_by_size: u64,
    pub lost_space_by_hash: u64,
}

pub struct DuplicateFinder {
    common_data: CommonToolData,
    information: Info,
    // File Size, File Entry
    files_with_identical_names: BTreeMap<String, Vec<FileEntry>>,
    // File (Size, Name), File Entry
    files_with_identical_size_names: BTreeMap<(u64, String), Vec<FileEntry>>,
    // File Size, File Entry
    files_with_identical_size: BTreeMap<u64, Vec<FileEntry>>,
    // File Size, next grouped by file size, next grouped by hash
    files_with_identical_hashes: BTreeMap<u64, Vec<Vec<FileEntry>>>,
    // File Size, File Entry
    files_with_identical_names_referenced: BTreeMap<String, (FileEntry, Vec<FileEntry>)>,
    // File (Size, Name), File Entry
    files_with_identical_size_names_referenced: BTreeMap<(u64, String), (FileEntry, Vec<FileEntry>)>,
    // File Size, File Entry
    files_with_identical_size_referenced: BTreeMap<u64, (FileEntry, Vec<FileEntry>)>,
    // File Size, next grouped by file size, next grouped by hash
    files_with_identical_hashes_referenced: BTreeMap<u64, Vec<(FileEntry, Vec<FileEntry>)>>,
    check_method: CheckingMethod,
    delete_method: DeleteMethod,
    hash_type: HashType,
    ignore_hard_links: bool,
    dryrun: bool,
    use_prehash_cache: bool,
    minimal_cache_file_size: u64,
    minimal_prehash_cache_file_size: u64,
    case_sensitive_name_comparison: bool,
}

impl DuplicateFinder {
    pub fn new() -> Self {
        Self {
            common_data: CommonToolData::new(ToolType::Duplicate),
            information: Info::default(),
            files_with_identical_names: Default::default(),
            files_with_identical_size: Default::default(),
            files_with_identical_size_names: Default::default(),
            files_with_identical_hashes: Default::default(),
            files_with_identical_names_referenced: Default::default(),
            files_with_identical_size_names_referenced: Default::default(),
            files_with_identical_size_referenced: Default::default(),
            files_with_identical_hashes_referenced: Default::default(),
            check_method: CheckingMethod::None,
            delete_method: DeleteMethod::None,
            ignore_hard_links: true,
            hash_type: HashType::Blake3,
            dryrun: false,
            use_prehash_cache: true,
            minimal_cache_file_size: 1024 * 256, // By default cache only >= 256 KB files
            minimal_prehash_cache_file_size: 0,
            case_sensitive_name_comparison: false,
        }
    }

    #[fun_time(message = "find_duplicates")]
    pub fn find_duplicates(&mut self, stop_receiver: Option<&Receiver<()>>, progress_sender: Option<&UnboundedSender<ProgressData>>) {
        self.optimize_dirs_before_start();
        self.common_data.use_reference_folders = !self.common_data.directories.reference_directories.is_empty();

        match self.check_method {
            CheckingMethod::Name => {
                self.common_data.stopped_search = !self.check_files_name(stop_receiver, progress_sender); // TODO restore this to name
                if self.common_data.stopped_search {
                    return;
                }
            }
            CheckingMethod::SizeName => {
                self.common_data.stopped_search = !self.check_files_size_name(stop_receiver, progress_sender);
                if self.common_data.stopped_search {
                    return;
                }
            }
            CheckingMethod::Size => {
                self.common_data.stopped_search = !self.check_files_size(stop_receiver, progress_sender);
                if self.common_data.stopped_search {
                    return;
                }
            }
            CheckingMethod::Hash => {
                self.common_data.stopped_search = !self.check_files_size(stop_receiver, progress_sender);
                if self.common_data.stopped_search {
                    return;
                }
                self.common_data.stopped_search = !self.check_files_hash(stop_receiver, progress_sender);
                if self.common_data.stopped_search {
                    return;
                }
            }
            _ => panic!(),
        }
        self.delete_files();
        self.debug_print();
    }

    #[fun_time(message = "check_files_name")]
    fn check_files_name(&mut self, stop_receiver: Option<&Receiver<()>>, progress_sender: Option<&UnboundedSender<ProgressData>>) -> bool {
        let group_by_func = if self.case_sensitive_name_comparison {
            |fe: &FileEntry| fe.path.file_name().unwrap().to_string_lossy().to_string()
        } else {
            |fe: &FileEntry| fe.path.file_name().unwrap().to_string_lossy().to_lowercase()
        };

        let result = DirTraversalBuilder::new()
            .root_dirs(self.common_data.directories.included_directories.clone())
            .group_by(group_by_func)
            .stop_receiver(stop_receiver)
            .progress_sender(progress_sender)
            .checking_method(CheckingMethod::Name)
            .directories(self.common_data.directories.clone())
            .allowed_extensions(self.common_data.allowed_extensions.clone())
            .excluded_items(self.common_data.excluded_items.clone())
            .recursive_search(self.common_data.recursive_search)
            .minimal_file_size(self.common_data.minimal_file_size)
            .maximal_file_size(self.common_data.maximal_file_size)
            .build()
            .run();
        debug!("check_files_name - after finding file sizes");
        let res = match result {
            DirTraversalResult::SuccessFiles { grouped_file_entries, warnings } => {
                self.files_with_identical_names = grouped_file_entries;
                self.common_data.text_messages.warnings.extend(warnings);

                // Create new BTreeMap without single size entries(files have not duplicates)
                let mut new_map: BTreeMap<String, Vec<FileEntry>> = Default::default();

                for (name, vector) in &self.files_with_identical_names {
                    if vector.len() > 1 {
                        new_map.insert(name.clone(), vector.clone());
                    }
                }
                self.files_with_identical_names = new_map;

                // Reference - only use in size, because later hash will be counted differently
                if self.common_data.use_reference_folders {
                    let vec = mem::take(&mut self.files_with_identical_names)
                        .into_iter()
                        .filter_map(|(_name, vec_file_entry)| {
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
                    for (fe, vec_fe) in vec {
                        self.files_with_identical_names_referenced.insert(fe.path.to_string_lossy().to_string(), (fe, vec_fe));
                    }
                }
                self.calculate_name_stats();

                true
            }
            DirTraversalResult::SuccessFolders { .. } => {
                unreachable!()
            }
            DirTraversalResult::Stopped => false,
        };
        res
    }

    fn calculate_name_stats(&mut self) {
        if self.common_data.use_reference_folders {
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
    }

    #[fun_time(message = "check_files_size_name")]
    fn check_files_size_name(&mut self, stop_receiver: Option<&Receiver<()>>, progress_sender: Option<&UnboundedSender<ProgressData>>) -> bool {
        let group_by_func = if self.case_sensitive_name_comparison {
            |fe: &FileEntry| (fe.size, fe.path.file_name().unwrap().to_string_lossy().to_string())
        } else {
            |fe: &FileEntry| (fe.size, fe.path.file_name().unwrap().to_string_lossy().to_lowercase())
        };

        let result = DirTraversalBuilder::new()
            .root_dirs(self.common_data.directories.included_directories.clone())
            .group_by(group_by_func)
            .stop_receiver(stop_receiver)
            .progress_sender(progress_sender)
            .checking_method(CheckingMethod::Name)
            .directories(self.common_data.directories.clone())
            .allowed_extensions(self.common_data.allowed_extensions.clone())
            .excluded_items(self.common_data.excluded_items.clone())
            .recursive_search(self.common_data.recursive_search)
            .minimal_file_size(self.common_data.minimal_file_size)
            .maximal_file_size(self.common_data.maximal_file_size)
            .build()
            .run();
        debug!("check_files_size_name - after finding file sizes");
        let res = match result {
            DirTraversalResult::SuccessFiles { grouped_file_entries, warnings } => {
                self.files_with_identical_size_names = grouped_file_entries;
                self.common_data.text_messages.warnings.extend(warnings);

                // Create new BTreeMap without single size entries(files have not duplicates)
                let mut new_map: BTreeMap<(u64, String), Vec<FileEntry>> = Default::default();

                for (name_size, vector) in &self.files_with_identical_size_names {
                    if vector.len() > 1 {
                        new_map.insert(name_size.clone(), vector.clone());
                    }
                }
                self.files_with_identical_size_names = new_map;

                // Reference - only use in size, because later hash will be counted differently
                if self.common_data.use_reference_folders {
                    let vec = mem::take(&mut self.files_with_identical_size_names)
                        .into_iter()
                        .filter_map(|(_size, vec_file_entry)| {
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
                    for (fe, vec_fe) in vec {
                        self.files_with_identical_size_names_referenced
                            .insert((fe.size, fe.path.to_string_lossy().to_string()), (fe, vec_fe));
                    }
                }
                self.calculate_size_name_stats();

                true
            }
            DirTraversalResult::SuccessFolders { .. } => {
                unreachable!()
            }
            DirTraversalResult::Stopped => false,
        };
        res
    }

    fn calculate_size_name_stats(&mut self) {
        if self.common_data.use_reference_folders {
            for ((size, _name), (_fe, vector)) in &self.files_with_identical_size_names_referenced {
                self.information.number_of_duplicated_files_by_size_name += vector.len();
                self.information.number_of_groups_by_size_name += 1;
                self.information.lost_space_by_size += (vector.len() as u64) * size;
            }
        } else {
            for ((size, _name), vector) in &self.files_with_identical_size_names {
                self.information.number_of_duplicated_files_by_size_name += vector.len() - 1;
                self.information.number_of_groups_by_size_name += 1;
                self.information.lost_space_by_size += (vector.len() as u64 - 1) * size;
            }
        }
    }

    #[fun_time(message = "check_files_size")]
    fn check_files_size(&mut self, stop_receiver: Option<&Receiver<()>>, progress_sender: Option<&UnboundedSender<ProgressData>>) -> bool {
        let max_stage = match self.check_method {
            CheckingMethod::Size => 0,
            CheckingMethod::Hash => 2,
            _ => panic!(),
        };
        let result = DirTraversalBuilder::new()
            .root_dirs(self.common_data.directories.included_directories.clone())
            .group_by(|fe| fe.size)
            .stop_receiver(stop_receiver)
            .progress_sender(progress_sender)
            .checking_method(self.check_method)
            .max_stage(max_stage)
            .directories(self.common_data.directories.clone())
            .allowed_extensions(self.common_data.allowed_extensions.clone())
            .excluded_items(self.common_data.excluded_items.clone())
            .recursive_search(self.common_data.recursive_search)
            .minimal_file_size(self.common_data.minimal_file_size)
            .maximal_file_size(self.common_data.maximal_file_size)
            .build()
            .run();
        debug!("check_file_size - after finding file sizes");
        let res = match result {
            DirTraversalResult::SuccessFiles { grouped_file_entries, warnings } => {
                self.files_with_identical_size = grouped_file_entries;
                self.common_data.text_messages.warnings.extend(warnings);

                // Create new BTreeMap without single size entries(files have not duplicates)
                let old_map: BTreeMap<u64, Vec<FileEntry>> = mem::take(&mut self.files_with_identical_size);

                for (size, vec) in old_map {
                    if vec.len() <= 1 {
                        continue;
                    }

                    let vector = if self.ignore_hard_links { filter_hard_links(&vec) } else { vec };

                    if vector.len() > 1 {
                        self.files_with_identical_size.insert(size, vector);
                    }
                }

                self.filter_reference_folders_by_size();
                self.calculate_size_stats();

                true
            }
            DirTraversalResult::SuccessFolders { .. } => {
                unreachable!()
            }
            DirTraversalResult::Stopped => false,
        };
        debug!(
            "check_file_size - after calculating size stats/duplicates, found in {} groups, {} files with same size | referenced {} groups, {} files",
            self.files_with_identical_size.len(),
            self.files_with_identical_size.values().map(Vec::len).sum::<usize>(),
            self.files_with_identical_size_referenced.len(),
            self.files_with_identical_size_referenced.values().map(|(_fe, vec)| vec.len()).sum::<usize>()
        );
        res
    }

    fn calculate_size_stats(&mut self) {
        if self.common_data.use_reference_folders {
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
    }

    fn filter_reference_folders_by_size(&mut self) {
        if self.common_data.use_reference_folders && self.check_method == CheckingMethod::Size {
            let vec = mem::take(&mut self.files_with_identical_size)
                .into_iter()
                .filter_map(|(_size, vec_file_entry)| {
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
            for (fe, vec_fe) in vec {
                self.files_with_identical_size_referenced.insert(fe.size, (fe, vec_fe));
            }
        }
    }

    #[fun_time(message = "prehash_load_cache_at_start")]
    fn prehash_load_cache_at_start(&mut self) -> (BTreeMap<u64, Vec<FileEntry>>, BTreeMap<u64, Vec<FileEntry>>, BTreeMap<u64, Vec<FileEntry>>) {
        // Cache algorithm
        // - Load data from cache
        // - Convert from BT<u64,Vec<FileEntry>> to BT<String,FileEntry>
        // - Save to proper values
        let loaded_hash_map;
        let mut records_already_cached: BTreeMap<u64, Vec<FileEntry>> = Default::default();
        let mut non_cached_files_to_check: BTreeMap<u64, Vec<FileEntry>> = Default::default();

        if self.use_prehash_cache {
            let (messages, loaded_items) = load_cache_from_file_generalized_by_size::<FileEntry>(
                &get_duplicate_cache_file(&self.hash_type, true),
                self.get_delete_outdated_cache(),
                &self.files_with_identical_size,
            );
            self.get_text_messages_mut().extend_with_another_messages(messages);
            loaded_hash_map = loaded_items.unwrap_or_default();

            debug!("prehash_load_cache_at_start - started diff between loaded and prechecked files");
            for (size, mut vec_file_entry) in mem::take(&mut self.files_with_identical_size) {
                if let Some(cached_vec_file_entry) = loaded_hash_map.get(&size) {
                    // TODO maybe hashset is not needed when using < 4 elements
                    let cached_path_entries = cached_vec_file_entry.iter().map(|e| &e.path).collect::<HashSet<_>>();
                    for file_entry in vec_file_entry {
                        if cached_path_entries.contains(&file_entry.path) {
                            records_already_cached.entry(size).or_default().push(file_entry);
                        } else {
                            non_cached_files_to_check.entry(size).or_default().push(file_entry);
                        }
                    }
                } else {
                    non_cached_files_to_check.entry(size).or_default().append(&mut vec_file_entry);
                }
            }

            debug!(
                "prehash_load_cache_at_start - completed diff between loaded and prechecked files, {}({}) - non cached, {}({}) - already cached",
                non_cached_files_to_check.values().map(Vec::len).sum::<usize>(),
                format_size(non_cached_files_to_check.values().map(|v| v.iter().map(|e| e.size).sum::<u64>()).sum::<u64>(), BINARY),
                records_already_cached.values().map(Vec::len).sum::<usize>(),
                format_size(records_already_cached.values().map(|v| v.iter().map(|e| e.size).sum::<u64>()).sum::<u64>(), BINARY),
            );
        } else {
            loaded_hash_map = Default::default();
            mem::swap(&mut self.files_with_identical_size, &mut non_cached_files_to_check);
        }
        (loaded_hash_map, records_already_cached, non_cached_files_to_check)
    }

    #[fun_time(message = "prehash_save_cache_at_exit")]
    fn prehash_save_cache_at_exit(&mut self, loaded_hash_map: BTreeMap<u64, Vec<FileEntry>>, pre_hash_results: &Vec<(u64, BTreeMap<String, Vec<FileEntry>>, Vec<String>)>) {
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

            for (size, hash_map, _errors) in pre_hash_results {
                if *size >= self.minimal_prehash_cache_file_size {
                    for vec_file_entry in hash_map.values() {
                        for file_entry in vec_file_entry {
                            save_cache_to_hashmap.insert(file_entry.path.to_string_lossy().to_string(), file_entry.clone());
                        }
                    }
                }
            }

            let messages = save_cache_to_file_generalized(
                &get_duplicate_cache_file(&self.hash_type, true),
                &save_cache_to_hashmap,
                self.common_data.save_also_as_json,
                self.minimal_prehash_cache_file_size,
            );
            self.get_text_messages_mut().extend_with_another_messages(messages);
        }
    }

    #[fun_time(message = "prehashing")]
    fn prehashing(
        &mut self,
        stop_receiver: Option<&Receiver<()>>,
        progress_sender: Option<&UnboundedSender<ProgressData>>,
        pre_checked_map: &mut BTreeMap<u64, Vec<FileEntry>>,
    ) -> Option<()> {
        let check_type = self.hash_type;
        let (progress_thread_handle, progress_thread_run, atomic_counter, check_was_stopped) = prepare_thread_handler_common(
            progress_sender,
            1,
            2,
            self.files_with_identical_size.values().map(Vec::len).sum(),
            self.check_method,
            self.common_data.tool_type,
        );

        let (loaded_hash_map, records_already_cached, non_cached_files_to_check) = self.prehash_load_cache_at_start();

        #[allow(clippy::type_complexity)]
        let pre_hash_results: Vec<(u64, BTreeMap<String, Vec<FileEntry>>, Vec<String>)> = non_cached_files_to_check
            .par_iter()
            .map(|(size, vec_file_entry)| {
                let mut hashmap_with_hash: BTreeMap<String, Vec<FileEntry>> = Default::default();
                let mut errors: Vec<String> = Vec::new();
                let mut buffer = [0u8; 1024 * 2];

                atomic_counter.fetch_add(vec_file_entry.len(), Ordering::Relaxed);
                if stop_receiver.is_some() && stop_receiver.unwrap().try_recv().is_ok() {
                    check_was_stopped.store(true, Ordering::Relaxed);
                    return None;
                }
                for file_entry in vec_file_entry {
                    match hash_calculation(&mut buffer, file_entry, &check_type, 0) {
                        Ok(hash_string) => {
                            hashmap_with_hash.entry(hash_string.clone()).or_default().push(file_entry.clone());
                        }
                        Err(s) => errors.push(s),
                    }
                }
                Some((*size, hashmap_with_hash, errors))
            })
            .while_some()
            .collect();

        send_info_and_wait_for_ending_all_threads(&progress_thread_run, progress_thread_handle);

        // Check if user aborted search(only from GUI)
        if check_was_stopped.load(Ordering::Relaxed) {
            return None;
        }

        // Add data from cache
        for (size, vec_file_entry) in &records_already_cached {
            pre_checked_map.entry(*size).or_default().append(&mut vec_file_entry.clone());
        }

        // Check results
        for (size, hash_map, errors) in &pre_hash_results {
            self.common_data.text_messages.warnings.append(&mut errors.clone());
            for vec_file_entry in hash_map.values() {
                if vec_file_entry.len() > 1 {
                    pre_checked_map.entry(*size).or_default().append(&mut vec_file_entry.clone());
                }
            }
        }

        self.prehash_save_cache_at_exit(loaded_hash_map, &pre_hash_results);

        Some(())
    }

    #[fun_time(message = "full_hashing_load_cache_at_start")]
    fn full_hashing_load_cache_at_start(
        &mut self,
        mut pre_checked_map: BTreeMap<u64, Vec<FileEntry>>,
    ) -> (BTreeMap<u64, Vec<FileEntry>>, BTreeMap<u64, Vec<FileEntry>>, BTreeMap<u64, Vec<FileEntry>>) {
        let loaded_hash_map;
        let mut records_already_cached: BTreeMap<u64, Vec<FileEntry>> = Default::default();
        let mut non_cached_files_to_check: BTreeMap<u64, Vec<FileEntry>> = Default::default();

        if self.common_data.use_cache {
            debug!("full_hashing_load_cache_at_start - using cache");
            let (messages, loaded_items) =
                load_cache_from_file_generalized_by_size::<FileEntry>(&get_duplicate_cache_file(&self.hash_type, false), self.get_delete_outdated_cache(), &pre_checked_map);
            self.get_text_messages_mut().extend_with_another_messages(messages);
            loaded_hash_map = loaded_items.unwrap_or_default();

            debug!("full_hashing_load_cache_at_start - started diff between loaded and prechecked files");
            for (size, mut vec_file_entry) in pre_checked_map {
                if let Some(cached_vec_file_entry) = loaded_hash_map.get(&size) {
                    // TODO maybe hashset is not needed when using < 4 elements
                    let cached_path_entries = cached_vec_file_entry.iter().map(|e| &e.path).collect::<HashSet<_>>();
                    for file_entry in vec_file_entry {
                        if cached_path_entries.contains(&file_entry.path) {
                            records_already_cached.entry(size).or_default().push(file_entry);
                        } else {
                            non_cached_files_to_check.entry(size).or_default().push(file_entry);
                        }
                    }
                } else {
                    non_cached_files_to_check.entry(size).or_default().append(&mut vec_file_entry);
                }
            }

            debug!(
                "full_hashing_load_cache_at_start - completed diff between loaded and prechecked files - {}({}) non cached, {}({}) already cached",
                non_cached_files_to_check.len(),
                format_size(non_cached_files_to_check.values().map(|v| v.iter().map(|e| e.size).sum::<u64>()).sum::<u64>(), BINARY),
                records_already_cached.len(),
                format_size(records_already_cached.values().map(|v| v.iter().map(|e| e.size).sum::<u64>()).sum::<u64>(), BINARY),
            );
        } else {
            debug!("full_hashing_load_cache_at_start - not using cache");
            loaded_hash_map = Default::default();
            mem::swap(&mut pre_checked_map, &mut non_cached_files_to_check);
        }
        (loaded_hash_map, records_already_cached, non_cached_files_to_check)
    }

    #[fun_time(message = "full_hashing_save_cache_at_exit")]
    fn full_hashing_save_cache_at_exit(
        &mut self,
        records_already_cached: BTreeMap<u64, Vec<FileEntry>>,
        full_hash_results: &mut Vec<(u64, BTreeMap<String, Vec<FileEntry>>, Vec<String>)>,
        loaded_hash_map: BTreeMap<u64, Vec<FileEntry>>,
    ) {
        if !self.common_data.use_cache {
            return;
        }
        'main: for (size, vec_file_entry) in records_already_cached {
            // Check if size already exists, if exists we must to change it outside because cannot have mut and non mut reference to full_hash_results
            for (full_size, full_hashmap, _errors) in &mut (*full_hash_results) {
                if size == *full_size {
                    for file_entry in vec_file_entry {
                        full_hashmap.entry(file_entry.hash.clone()).or_default().push(file_entry);
                    }
                    continue 'main;
                }
            }
            // Size doesn't exists add results to files
            let mut temp_hashmap: BTreeMap<String, Vec<FileEntry>> = Default::default();
            for file_entry in vec_file_entry {
                temp_hashmap.entry(file_entry.hash.clone()).or_default().push(file_entry);
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
        for (_size, hashmap, _errors) in full_hash_results {
            for vec_file_entry in hashmap.values() {
                for file_entry in vec_file_entry {
                    all_results.insert(file_entry.path.to_string_lossy().to_string(), file_entry.clone());
                }
            }
        }

        let messages = save_cache_to_file_generalized(
            &get_duplicate_cache_file(&self.hash_type, false),
            &all_results,
            self.common_data.save_also_as_json,
            self.minimal_cache_file_size,
        );
        self.get_text_messages_mut().extend_with_another_messages(messages);
    }

    #[fun_time(message = "full_hashing")]
    fn full_hashing(
        &mut self,
        stop_receiver: Option<&Receiver<()>>,
        progress_sender: Option<&UnboundedSender<ProgressData>>,
        pre_checked_map: BTreeMap<u64, Vec<FileEntry>>,
    ) -> Option<()> {
        let check_type = self.hash_type;

        let (progress_thread_handle, progress_thread_run, atomic_counter, check_was_stopped) = prepare_thread_handler_common(
            progress_sender,
            2,
            2,
            pre_checked_map.values().map(Vec::len).sum(),
            self.check_method,
            self.common_data.tool_type,
        );

        ///////////////////////////////////////////////////////////////////////////// HASHING START
        {
            let (loaded_hash_map, records_already_cached, non_cached_files_to_check) = self.full_hashing_load_cache_at_start(pre_checked_map);

            let mut full_hash_results: Vec<(u64, BTreeMap<String, Vec<FileEntry>>, Vec<String>)> = non_cached_files_to_check
                .into_par_iter()
                .map(|(size, vec_file_entry)| {
                    let mut hashmap_with_hash: BTreeMap<String, Vec<FileEntry>> = Default::default();
                    let mut errors: Vec<String> = Vec::new();
                    let mut buffer = [0u8; 1024 * 16];

                    atomic_counter.fetch_add(vec_file_entry.len(), Ordering::Relaxed);
                    for mut file_entry in vec_file_entry {
                        if stop_receiver.is_some() && stop_receiver.unwrap().try_recv().is_ok() {
                            check_was_stopped.store(true, Ordering::Relaxed);
                            return None;
                        }

                        match hash_calculation(&mut buffer, &file_entry, &check_type, u64::MAX) {
                            Ok(hash_string) => {
                                file_entry.hash = hash_string.clone();
                                hashmap_with_hash.entry(hash_string.clone()).or_default().push(file_entry);
                            }
                            Err(s) => errors.push(s),
                        }
                    }
                    Some((size, hashmap_with_hash, errors))
                })
                .while_some()
                .collect();

            self.full_hashing_save_cache_at_exit(records_already_cached, &mut full_hash_results, loaded_hash_map);

            send_info_and_wait_for_ending_all_threads(&progress_thread_run, progress_thread_handle);

            // Break if stop was clicked after saving to cache
            if check_was_stopped.load(Ordering::Relaxed) {
                return None;
            }

            for (size, hash_map, mut errors) in full_hash_results {
                self.common_data.text_messages.warnings.append(&mut errors);
                for (_hash, vec_file_entry) in hash_map {
                    if vec_file_entry.len() > 1 {
                        self.files_with_identical_hashes.entry(size).or_default().push(vec_file_entry);
                    }
                }
            }
        }

        Some(())
    }

    #[fun_time(message = "hash_reference_folders")]
    fn hash_reference_folders(&mut self) {
        // Reference - only use in size, because later hash will be counted differently
        if self.common_data.use_reference_folders {
            let vec = mem::take(&mut self.files_with_identical_hashes)
                .into_iter()
                .filter_map(|(_size, vec_vec_file_entry)| {
                    let mut all_results_with_same_size = Vec::new();
                    for vec_file_entry in vec_vec_file_entry {
                        let (mut files_from_referenced_folders, normal_files): (Vec<_>, Vec<_>) = vec_file_entry
                            .into_iter()
                            .partition(|e| self.common_data.directories.is_in_referenced_directory(e.get_path()));

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

        if self.common_data.use_reference_folders {
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
    }

    /// The slowest checking type, which must be applied after checking for size

    #[fun_time(message = "check_files_hash")]
    fn check_files_hash(&mut self, stop_receiver: Option<&Receiver<()>>, progress_sender: Option<&UnboundedSender<ProgressData>>) -> bool {
        assert_eq!(self.check_method, CheckingMethod::Hash);

        let mut pre_checked_map: BTreeMap<u64, Vec<FileEntry>> = Default::default();
        let ret = self.prehashing(stop_receiver, progress_sender, &mut pre_checked_map);
        if ret.is_none() {
            return false;
        }

        let ret = self.full_hashing(stop_receiver, progress_sender, pre_checked_map);
        if ret.is_none() {
            return false;
        }

        self.hash_reference_folders();

        // Clean unused data
        self.files_with_identical_size = Default::default();

        true
    }

    /// Function to delete files, from filed before `BTreeMap`
    /// Using another function to delete files to avoid duplicates data
    #[fun_time(message = "delete_files")]
    fn delete_files(&mut self) {
        if self.delete_method == DeleteMethod::None {
            return;
        }

        match self.check_method {
            CheckingMethod::Name => {
                for vector in self.files_with_identical_names.values() {
                    let _tuple: (u64, usize, usize) = delete_files(vector, &self.delete_method, &mut self.common_data.text_messages, self.dryrun);
                }
            }
            CheckingMethod::SizeName => {
                for vector in self.files_with_identical_size_names.values() {
                    let _tuple: (u64, usize, usize) = delete_files(vector, &self.delete_method, &mut self.common_data.text_messages, self.dryrun);
                }
            }
            CheckingMethod::Hash => {
                for vector_vectors in self.files_with_identical_hashes.values() {
                    for vector in vector_vectors {
                        let _tuple: (u64, usize, usize) = delete_files(vector, &self.delete_method, &mut self.common_data.text_messages, self.dryrun);
                    }
                }
            }
            CheckingMethod::Size => {
                for vector in self.files_with_identical_size.values() {
                    let _tuple: (u64, usize, usize) = delete_files(vector, &self.delete_method, &mut self.common_data.text_messages, self.dryrun);
                }
            }
            _ => panic!(),
        }
    }
}

impl DuplicateFinder {
    pub fn set_case_sensitive_name_comparison(&mut self, case_sensitive_name_comparison: bool) {
        self.case_sensitive_name_comparison = case_sensitive_name_comparison;
    }

    pub const fn get_check_method(&self) -> &CheckingMethod {
        &self.check_method
    }

    pub fn set_minimal_cache_file_size(&mut self, minimal_cache_file_size: u64) {
        self.minimal_cache_file_size = minimal_cache_file_size;
    }

    pub fn set_minimal_prehash_cache_file_size(&mut self, minimal_prehash_cache_file_size: u64) {
        self.minimal_prehash_cache_file_size = minimal_prehash_cache_file_size;
    }

    pub const fn get_files_sorted_by_names(&self) -> &BTreeMap<String, Vec<FileEntry>> {
        &self.files_with_identical_names
    }

    pub fn set_use_prehash_cache(&mut self, use_prehash_cache: bool) {
        self.use_prehash_cache = use_prehash_cache;
    }

    pub const fn get_files_sorted_by_size(&self) -> &BTreeMap<u64, Vec<FileEntry>> {
        &self.files_with_identical_size
    }

    pub const fn get_files_sorted_by_size_name(&self) -> &BTreeMap<(u64, String), Vec<FileEntry>> {
        &self.files_with_identical_size_names
    }

    pub const fn get_files_sorted_by_hash(&self) -> &BTreeMap<u64, Vec<Vec<FileEntry>>> {
        &self.files_with_identical_hashes
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

    pub fn get_use_reference(&self) -> bool {
        self.common_data.use_reference_folders
    }

    pub fn get_files_with_identical_hashes_referenced(&self) -> &BTreeMap<u64, Vec<(FileEntry, Vec<FileEntry>)>> {
        &self.files_with_identical_hashes_referenced
    }

    pub fn get_files_with_identical_name_referenced(&self) -> &BTreeMap<String, (FileEntry, Vec<FileEntry>)> {
        &self.files_with_identical_names_referenced
    }

    pub fn get_files_with_identical_size_referenced(&self) -> &BTreeMap<u64, (FileEntry, Vec<FileEntry>)> {
        &self.files_with_identical_size_referenced
    }

    pub fn get_files_with_identical_size_names_referenced(&self) -> &BTreeMap<(u64, String), (FileEntry, Vec<FileEntry>)> {
        &self.files_with_identical_size_names_referenced
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
        println!("Checking Method - {:?}", self.check_method);
        println!("Delete Method - {:?}", self.delete_method);
        self.debug_print_common();
        println!("-----------------------------------------");
    }
}

impl SaveResults for DuplicateFinder {
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
            CheckingMethod::SizeName => {
                if !self.files_with_identical_names.is_empty() {
                    writeln!(
                        writer,
                        "-------------------------------------------------Files with same size and names-------------------------------------------------"
                    )
                    .unwrap();
                    writeln!(
                        writer,
                        "Found {} files in {} groups with same size and name(may have different content)",
                        self.information.number_of_duplicated_files_by_size_name, self.information.number_of_groups_by_size_name,
                    )
                    .unwrap();
                    for ((size, name), vector) in self.files_with_identical_size_names.iter().rev() {
                        writeln!(writer, "Name - {}, {} - {} files ", name, format_size(*size, BINARY), vector.len()).unwrap();
                        for j in vector {
                            writeln!(writer, "{}", j.path.display()).unwrap();
                        }
                        writeln!(writer).unwrap();
                    }
                } else {
                    write!(writer, "Not found any files with same size and names.").unwrap();
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
            _ => panic!(),
        }

        true
    }
}

impl PrintResults for DuplicateFinder {
    /// Print information's about duplicated entries
    /// Only needed for CLI
    fn print_results(&self) {
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
            CheckingMethod::SizeName => {
                for i in &self.files_with_identical_size_names {
                    number_of_files += i.1.len() as u64;
                    number_of_groups += 1;
                }
                println!("Found {number_of_files} files in {number_of_groups} groups with same size and name(may have different content)",);
                for ((size, name), vector) in &self.files_with_identical_size_names {
                    println!("Name - {}, {} - {} files ", name, format_size(*size, BINARY), vector.len());
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
            _ => panic!(),
        }
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
                    fs::remove_file(&file.path).map(|()| None)
                }
            }
            DeleteMethod::HardLink => {
                let src = &vector[q_index].path;
                if dryrun {
                    Ok(Some(format!("Replace file {} with hard link to {}", file.path.display(), src.display())))
                } else {
                    make_hard_link(src, &file.path).map(|()| None)
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

impl MyHasher for Xxh3 {
    fn update(&mut self, bytes: &[u8]) {
        self.write(bytes);
    }
    fn finalize(&self) -> String {
        self.finish().to_string()
    }
}

impl CommonData for DuplicateFinder {
    fn get_cd(&self) -> &CommonToolData {
        &self.common_data
    }
    fn get_cd_mut(&mut self) -> &mut CommonToolData {
        &mut self.common_data
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
    use std::path::PathBuf;

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
