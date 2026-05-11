use std::collections::BTreeMap;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::time::Instant;
use std::{mem, thread};

use crossbeam_channel::Sender;
use fun_time::fun_time;
use log::debug;
use rayon::prelude::*;

use crate::common::cache::{CACHE_DUPLICATE_VERSION, load_and_split_cache_generalized_by_size, save_cache_to_file_generalized};
use crate::common::dir_traversal::{DirTraversalBuilder, DirTraversalResult};
use crate::common::model::{CheckingMethod, FileEntry, HashType, ToolType, WorkContinueStatus};
use crate::common::progress_data::{CurrentStage, ProgressData};
use crate::common::progress_stop_handler::{check_if_stop_received, prepare_thread_handler_common};
use crate::common::tool_data::{CommonData, CommonToolData};
use crate::common::traits::ResultEntry;
use crate::tools::duplicate::{
    DuplicateEntry, DuplicateFinder, DuplicateFinderParameters, Info, PREHASHING_BUFFER_SIZE, THREAD_BUFFER, filter_hard_links, hash_calculation, hash_calculation_limit,
};

impl DuplicateFinder {
    pub fn new(params: DuplicateFinderParameters) -> Self {
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
            params,
        }
    }

    #[fun_time(message = "check_files_name", level = "debug")]
    pub(crate) fn check_files_name(&mut self, stop_flag: &Arc<AtomicBool>, progress_sender: Option<&Sender<ProgressData>>) -> WorkContinueStatus {
        let group_by_func = if self.get_params().case_sensitive_name_comparison {
            |fe: &FileEntry| {
                fe.path
                    .file_name()
                    .unwrap_or_else(|| panic!("Found invalid file_name \"{}\" (cannot panic, because it is always normal file)", fe.path.to_string_lossy()))
                    .to_string_lossy()
                    .to_string()
            }
        } else {
            |fe: &FileEntry| {
                fe.path
                    .file_name()
                    .unwrap_or_else(|| panic!("Found invalid file_name \"{}\" (cannot panic, because it is always normal file)", fe.path.to_string_lossy()))
                    .to_string_lossy()
                    .to_lowercase()
            }
        };

        let result = DirTraversalBuilder::new()
            .common_data(&self.common_data)
            .group_by(group_by_func)
            .stop_flag(stop_flag)
            .progress_sender(progress_sender)
            .checking_method(CheckingMethod::Name)
            .build()
            .run();

        match result {
            DirTraversalResult::SuccessFiles { grouped_file_entries, warnings } => {
                self.common_data.text_messages.warnings.extend(warnings);

                // Create new BTreeMap without single size entries(files have not duplicates)
                self.files_with_identical_names = grouped_file_entries
                    .into_iter()
                    .filter_map(|(name, vector)| {
                        if vector.len() > 1 {
                            Some((name, vector.into_iter().map(FileEntry::into_duplicate_entry).collect()))
                        } else {
                            None
                        }
                    })
                    .collect();

                // Reference - only use in size, because later hash will be counted differently
                if self.common_data.use_reference_folders {
                    let vec = self
                        .common_data
                        .directories
                        .filter_reference_folders(mem::take(&mut self.files_with_identical_names).into_values().collect());
                    for (fe, vec_fe) in vec {
                        self.files_with_identical_names_referenced.insert(fe.path.to_string_lossy().to_string(), (fe, vec_fe));
                    }
                }
                self.calculate_name_stats();

                WorkContinueStatus::Continue
            }
            DirTraversalResult::Stopped => WorkContinueStatus::Stop,
        }
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

    #[fun_time(message = "check_files_size_name", level = "debug")]
    pub(crate) fn check_files_size_name(&mut self, stop_flag: &Arc<AtomicBool>, progress_sender: Option<&Sender<ProgressData>>) -> WorkContinueStatus {
        let group_by_func = if self.get_params().case_sensitive_name_comparison {
            |fe: &FileEntry| {
                (
                    fe.size,
                    fe.path
                        .file_name()
                        .unwrap_or_else(|| panic!("Found invalid file_name \"{}\" (cannot panic, because it is always normal file)", fe.path.to_string_lossy()))
                        .to_string_lossy()
                        .to_string(),
                )
            }
        } else {
            |fe: &FileEntry| {
                (
                    fe.size,
                    fe.path
                        .file_name()
                        .unwrap_or_else(|| panic!("Found invalid file_name \"{}\" (cannot panic, because it is always normal file)", fe.path.to_string_lossy()))
                        .to_string_lossy()
                        .to_lowercase(),
                )
            }
        };

        let result = DirTraversalBuilder::new()
            .common_data(&self.common_data)
            .group_by(group_by_func)
            .stop_flag(stop_flag)
            .progress_sender(progress_sender)
            .checking_method(CheckingMethod::SizeName)
            .build()
            .run();

        match result {
            DirTraversalResult::SuccessFiles { grouped_file_entries, warnings } => {
                self.common_data.text_messages.warnings.extend(warnings);

                self.files_with_identical_size_names = grouped_file_entries
                    .into_iter()
                    .filter_map(|(size_name, vector)| {
                        if vector.len() > 1 {
                            Some((size_name, vector.into_iter().map(FileEntry::into_duplicate_entry).collect()))
                        } else {
                            None
                        }
                    })
                    .collect();

                // Reference - only use in size, because later hash will be counted differently
                if self.common_data.use_reference_folders {
                    let vec = self
                        .common_data
                        .directories
                        .filter_reference_folders(mem::take(&mut self.files_with_identical_size_names).into_values().collect());
                    for (fe, vec_fe) in vec {
                        self.files_with_identical_size_names_referenced
                            .insert((fe.size, fe.path.to_string_lossy().to_string()), (fe, vec_fe));
                    }
                }
                self.calculate_size_name_stats();

                WorkContinueStatus::Continue
            }

            DirTraversalResult::Stopped => WorkContinueStatus::Stop,
        }
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

    #[fun_time(message = "check_files_size", level = "debug")]
    pub(crate) fn check_files_size(&mut self, stop_flag: &Arc<AtomicBool>, progress_sender: Option<&Sender<ProgressData>>) -> WorkContinueStatus {
        let result = DirTraversalBuilder::new()
            .common_data(&self.common_data)
            .group_by(|fe| fe.size)
            .stop_flag(stop_flag)
            .progress_sender(progress_sender)
            .checking_method(self.get_params().check_method)
            .build()
            .run();

        match result {
            DirTraversalResult::SuccessFiles { grouped_file_entries, warnings } => {
                self.common_data.text_messages.warnings.extend(warnings);

                let grouped_file_entries: Vec<(u64, Vec<FileEntry>)> = grouped_file_entries.into_iter().collect();
                let rayon_max_len = if self.get_hide_hard_links() { 3 } else { 100 };

                let start_time = Instant::now();
                // We only gather files with more than 1 entry, because only this will be later used
                let initial_size = grouped_file_entries
                    .iter()
                    .map(|(_size, vec)| if vec.len() > 1 { vec.len() as u64 } else { 0 })
                    .sum::<u64>();

                let progress_handler = prepare_thread_handler_common(progress_sender, CurrentStage::DuplicateHidingHardLinks, grouped_file_entries.len(), self.get_test_type(), 0);
                self.files_with_identical_size = grouped_file_entries
                    .into_par_iter()
                    .with_max_len(rayon_max_len)
                    .map(|(size, vec)| {
                        if check_if_stop_received(stop_flag) {
                            return None;
                        }
                        progress_handler.increase_items(1);
                        Some((size, vec))
                    })
                    .while_some()
                    .filter_map(|(size, vec)| {
                        if vec.len() <= 1 {
                            return None;
                        }

                        let vector = if self.get_hide_hard_links() { filter_hard_links(vec) } else { vec };

                        if vector.len() > 1 {
                            Some((size, vector.into_iter().map(FileEntry::into_duplicate_entry).collect()))
                        } else {
                            None
                        }
                    })
                    .collect();

                progress_handler.join_thread();

                if check_if_stop_received(stop_flag) {
                    return WorkContinueStatus::Stop;
                }

                let filtered_size = self.files_with_identical_size.values().map(|v| v.len() as u64).sum::<u64>();
                debug!(
                    "check_file_size - filtered hard links in {:?}, removed {} hardlinks ({} -> {})",
                    start_time.elapsed(),
                    initial_size - filtered_size,
                    initial_size,
                    filtered_size
                );

                self.filter_reference_folders_by_size();
                self.calculate_size_stats();

                debug!(
                    "check_file_size - after calculating size stats/duplicates, found in {} groups, {} files with same size | referenced {} groups, {} files",
                    self.files_with_identical_size.len(),
                    self.files_with_identical_size.values().map(Vec::len).sum::<usize>(),
                    self.files_with_identical_size_referenced.len(),
                    self.files_with_identical_size_referenced.values().map(|(_fe, vec)| vec.len()).sum::<usize>()
                );

                WorkContinueStatus::Continue
            }

            DirTraversalResult::Stopped => WorkContinueStatus::Stop,
        }
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

    #[fun_time(message = "filter_reference_folders_by_size", level = "debug")]
    fn filter_reference_folders_by_size(&mut self) {
        if self.common_data.use_reference_folders && self.get_params().check_method == CheckingMethod::Size {
            let vec = self
                .common_data
                .directories
                .filter_reference_folders(mem::take(&mut self.files_with_identical_size).into_values().collect());
            for (fe, vec_fe) in vec {
                self.files_with_identical_size_referenced.insert(fe.size, (fe, vec_fe));
            }
        }
    }

    #[fun_time(message = "prehash_load_cache_at_start", level = "debug")]
    fn prehash_load_cache_at_start(&mut self) -> (BTreeMap<u64, Vec<DuplicateEntry>>, BTreeMap<u64, Vec<DuplicateEntry>>, BTreeMap<u64, Vec<DuplicateEntry>>) {
        load_and_split_cache_generalized_by_size(
            &get_duplicate_cache_file(self.get_params().hash_type, true),
            self.get_params().use_prehash_cache,
            mem::take(&mut self.files_with_identical_size),
            self,
        )
    }

    #[fun_time(message = "prehash_save_cache_at_exit", level = "debug")]
    fn prehash_save_cache_at_exit(&mut self, loaded_hash_map: BTreeMap<u64, Vec<DuplicateEntry>>, combined: &BTreeMap<u64, BTreeMap<String, Vec<DuplicateEntry>>>) {
        if self.get_params().use_prehash_cache {
            // All results = records already cached + computed results
            let mut save_cache_to_hashmap: BTreeMap<String, DuplicateEntry> = Default::default();

            for (size, vec_file_entry) in loaded_hash_map {
                if size >= self.get_params().minimal_prehash_cache_file_size {
                    for file_entry in vec_file_entry {
                        save_cache_to_hashmap.insert(file_entry.path.to_string_lossy().to_string(), file_entry);
                    }
                }
            }

            for (&size, hash_map) in combined {
                if size >= self.get_params().minimal_prehash_cache_file_size {
                    for vec_file_entry in hash_map.values() {
                        for file_entry in vec_file_entry {
                            save_cache_to_hashmap.insert(file_entry.path.to_string_lossy().to_string(), file_entry.clone());
                        }
                    }
                }
            }

            let messages = save_cache_to_file_generalized(
                &get_duplicate_cache_file(self.get_params().hash_type, true),
                &save_cache_to_hashmap,
                self.common_data.save_also_as_json,
                self.get_params().minimal_prehash_cache_file_size,
            );
            self.get_text_messages_mut().extend_with_another_messages(messages);
        }
    }

    #[fun_time(message = "prehashing", level = "debug")]
    fn prehashing(
        &mut self,
        stop_flag: &Arc<AtomicBool>,
        progress_sender: Option<&Sender<ProgressData>>,
        pre_checked_map: &mut BTreeMap<u64, Vec<DuplicateEntry>>,
    ) -> WorkContinueStatus {
        if self.files_with_identical_size.is_empty() {
            return WorkContinueStatus::Continue;
        }

        let check_type = self.get_params().hash_type;
        let progress_handler = prepare_thread_handler_common(progress_sender, CurrentStage::DuplicatePreHashCacheLoading, 0, self.get_test_type(), 0);

        let (loaded_hash_map, records_already_cached, non_cached_files_to_check) = self.prehash_load_cache_at_start();

        progress_handler.join_thread();
        if check_if_stop_received(stop_flag) {
            return WorkContinueStatus::Stop;
        }
        let progress_handler = prepare_thread_handler_common(
            progress_sender,
            CurrentStage::DuplicatePreHashing,
            non_cached_files_to_check.values().map(Vec::len).sum(),
            self.get_test_type(),
            non_cached_files_to_check
                .iter()
                .map(|(&size, items)| {
                    let bytes_per_file = size.min(2 * PREHASHING_BUFFER_SIZE);
                    items.len() as u64 * bytes_per_file
                })
                .sum::<u64>(),
        );

        // Convert to vector to be able to use with_max_len method from rayon
        let non_cached_files_to_check: Vec<(u64, Vec<DuplicateEntry>)> = non_cached_files_to_check.into_iter().collect();

        debug!("Starting calculating prehash");
        #[expect(clippy::type_complexity)]
        let pre_hash_results: Vec<(u64, BTreeMap<String, Vec<DuplicateEntry>>, Vec<String>)> = non_cached_files_to_check
            .into_par_iter()
            .with_max_len(3) // Vectors and BTreeMaps for really big inputs, leave some jobs to 0 thread, to avoid that I minimized max tasks for each thread to 3, which improved performance
            .map(|(size, vec_file_entry)| {
                let mut hashmap_with_hash: BTreeMap<String, Vec<DuplicateEntry>> = Default::default();
                let mut errors: Vec<String> = Vec::new();

                THREAD_BUFFER.with_borrow_mut(|buffer| {
                    for mut file_entry in vec_file_entry {
                        if check_if_stop_received(stop_flag) {
                            return None;
                        }
                        match hash_calculation_limit(buffer, &file_entry, check_type, PREHASHING_BUFFER_SIZE, progress_handler.size_counter()) {
                            Ok(hash_string) => {
                                file_entry.hash = hash_string.clone();
                                hashmap_with_hash.entry(hash_string).or_default().push(file_entry);
                            }
                            Err(s) => errors.push(s),
                        }
                        progress_handler.increase_items(1);
                    }

                    Some(())
                })?;

                Some((size, hashmap_with_hash, errors))
            })
            .while_some()
            .collect();

        debug!("Completed calculating prehash");

        progress_handler.join_thread();

        // Saving into cache
        let progress_handler = prepare_thread_handler_common(progress_sender, CurrentStage::DuplicatePreHashCacheSaving, 0, self.get_test_type(), 0);

        // Merge cached and freshly-computed entries into (size -> hash -> files) groups,
        // then only pass groups with >1 file to full hashing.  Merging is required so that
        // a cached file and a freshly-computed file sharing the same prehash are recognised
        // as a collision even when neither set alone has more than one entry for that hash.
        // pre_hash_results is consumed here so entries are moved, not cloned.
        let mut combined: BTreeMap<u64, BTreeMap<String, Vec<DuplicateEntry>>> = BTreeMap::new();
        for (size, vec_file_entry) in records_already_cached {
            for file_entry in vec_file_entry {
                combined.entry(size).or_default().entry(file_entry.hash.clone()).or_default().push(file_entry);
            }
        }
        for (size, hash_map, errors) in pre_hash_results {
            if !errors.is_empty() {
                self.common_data.text_messages.warnings.extend(errors);
            }
            for (hash, vec_file_entry) in hash_map {
                combined.entry(size).or_default().entry(hash).or_default().extend(vec_file_entry);
            }
        }

        self.prehash_save_cache_at_exit(loaded_hash_map, &combined);

        for (size, hash_groups) in combined {
            for group in hash_groups.into_values() {
                if group.len() > 1 {
                    pre_checked_map.entry(size).or_default().extend(group);
                }
            }
        }

        progress_handler.join_thread();

        if check_if_stop_received(stop_flag) {
            return WorkContinueStatus::Stop;
        }

        WorkContinueStatus::Continue
    }

    #[fun_time(message = "full_hashing_load_cache_at_start", level = "debug")]
    fn full_hashing_load_cache_at_start(
        &mut self,
        pre_checked_map: BTreeMap<u64, Vec<DuplicateEntry>>,
    ) -> (BTreeMap<u64, Vec<DuplicateEntry>>, BTreeMap<u64, Vec<DuplicateEntry>>, BTreeMap<u64, Vec<DuplicateEntry>>) {
        load_and_split_cache_generalized_by_size(
            &get_duplicate_cache_file(self.get_params().hash_type, false),
            self.common_data.use_cache,
            pre_checked_map,
            self,
        )
    }

    #[fun_time(message = "full_hashing_save_cache_at_exit", level = "debug")]
    fn full_hashing_save_cache_at_exit(
        &mut self,
        records_already_cached: BTreeMap<u64, Vec<DuplicateEntry>>,
        full_hash_results: &mut Vec<(u64, BTreeMap<String, Vec<DuplicateEntry>>, Vec<String>)>,
        loaded_hash_map: BTreeMap<u64, Vec<DuplicateEntry>>,
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
            let mut temp_hashmap: BTreeMap<String, Vec<DuplicateEntry>> = Default::default();
            for file_entry in vec_file_entry {
                temp_hashmap.entry(file_entry.hash.clone()).or_default().push(file_entry);
            }
            full_hash_results.push((size, temp_hashmap, Vec::new()));
        }

        // Must save all results to file, old loaded from file with all currently counted results
        let mut all_results: BTreeMap<String, DuplicateEntry> = Default::default();
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
            &get_duplicate_cache_file(self.get_params().hash_type, false),
            &all_results,
            self.common_data.save_also_as_json,
            self.get_params().minimal_cache_file_size,
        );
        self.get_text_messages_mut().extend_with_another_messages(messages);
    }

    #[fun_time(message = "full_hashing", level = "debug")]
    fn full_hashing(
        &mut self,
        stop_flag: &Arc<AtomicBool>,
        progress_sender: Option<&Sender<ProgressData>>,
        pre_checked_map: BTreeMap<u64, Vec<DuplicateEntry>>,
    ) -> WorkContinueStatus {
        if pre_checked_map.is_empty() {
            return WorkContinueStatus::Continue;
        }

        let progress_handler = prepare_thread_handler_common(progress_sender, CurrentStage::DuplicateCacheLoading, 0, self.get_test_type(), 0);

        let (loaded_hash_map, records_already_cached, non_cached_files_to_check) = self.full_hashing_load_cache_at_start(pre_checked_map);

        progress_handler.join_thread();
        if check_if_stop_received(stop_flag) {
            return WorkContinueStatus::Stop;
        }

        let progress_handler = prepare_thread_handler_common(
            progress_sender,
            CurrentStage::DuplicateFullHashing,
            non_cached_files_to_check.values().map(Vec::len).sum(),
            self.get_test_type(),
            non_cached_files_to_check.iter().map(|(size, items)| (*size) * items.len() as u64).sum::<u64>(),
        );

        let non_cached_files_to_check: Vec<(u64, Vec<DuplicateEntry>)> = non_cached_files_to_check.into_iter().collect();

        let check_type = self.get_params().hash_type;
        debug!(
            "Starting full hashing of {} files",
            non_cached_files_to_check.iter().map(|(_size, v)| v.len() as u64).sum::<u64>()
        );
        let mut full_hash_results: Vec<(u64, BTreeMap<String, Vec<DuplicateEntry>>, Vec<String>)> = non_cached_files_to_check
            .into_par_iter()
            .with_max_len(3)
            .map(|(size, vec_file_entry)| {
                let mut hashmap_with_hash: BTreeMap<String, Vec<DuplicateEntry>> = Default::default();
                let mut errors: Vec<String> = Vec::new();

                THREAD_BUFFER.with_borrow_mut(|buffer| {
                    for mut file_entry in vec_file_entry {
                        if check_if_stop_received(stop_flag) {
                            return None;
                        }

                        match hash_calculation(buffer, &file_entry, check_type, progress_handler.size_counter(), stop_flag) {
                            Ok(hash_string) => {
                                if let Some(hash_string) = hash_string {
                                    file_entry.hash = hash_string.clone();
                                    hashmap_with_hash.entry(hash_string).or_default().push(file_entry);
                                } else {
                                    return None;
                                }
                            }
                            Err(s) => errors.push(s),
                        }
                        progress_handler.increase_items(1);
                    }
                    Some(())
                })?;

                Some((size, hashmap_with_hash, errors))
            })
            .while_some()
            .collect();
        debug!("Finished full hashing");

        // Even if clicked stop, save items to cache and show results

        progress_handler.join_thread();
        let progress_handler = prepare_thread_handler_common(progress_sender, CurrentStage::DuplicateCacheSaving, 0, self.get_test_type(), 0);

        self.full_hashing_save_cache_at_exit(records_already_cached, &mut full_hash_results, loaded_hash_map);

        progress_handler.join_thread();

        for (size, hash_map, mut errors) in full_hash_results {
            self.common_data.text_messages.warnings.append(&mut errors);
            for (_hash, vec_file_entry) in hash_map {
                if vec_file_entry.len() > 1 {
                    self.files_with_identical_hashes.entry(size).or_default().push(vec_file_entry);
                }
            }
        }

        WorkContinueStatus::Continue
    }

    #[fun_time(message = "hash_reference_folders", level = "debug")]
    fn hash_reference_folders(&mut self) {
        // Reference - only use in size, because later hash will be counted differently
        if self.common_data.use_reference_folders {
            let vec = mem::take(&mut self.files_with_identical_hashes)
                .into_values()
                .filter_map(|vec_vec_file_entry| {
                    let mut all_results_with_same_size = Vec::new();
                    for vec_file_entry in vec_vec_file_entry {
                        let (mut files_from_referenced_folders, normal_files): (Vec<_>, Vec<_>) = vec_file_entry
                            .into_iter()
                            .partition(|e| self.common_data.directories.is_in_referenced_directory(e.get_path()));

                        if normal_files.is_empty() {
                            continue;
                        }
                        if let Some(file) = files_from_referenced_folders.pop() {
                            all_results_with_same_size.push((file, normal_files));
                        }
                    }
                    if all_results_with_same_size.is_empty() {
                        None
                    } else {
                        Some(all_results_with_same_size)
                    }
                })
                .collect::<Vec<Vec<(DuplicateEntry, Vec<DuplicateEntry>)>>>();
            #[expect(clippy::indexing_slicing)] // Safe, because here, empty vectors cannot exist
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

    #[fun_time(message = "check_files_hash", level = "debug")]
    pub(crate) fn check_files_hash(&mut self, stop_flag: &Arc<AtomicBool>, progress_sender: Option<&Sender<ProgressData>>) -> WorkContinueStatus {
        assert_eq!(self.get_params().check_method, CheckingMethod::Hash);

        let mut pre_checked_map: BTreeMap<u64, Vec<DuplicateEntry>> = Default::default();
        if self.prehashing(stop_flag, progress_sender, &mut pre_checked_map) == WorkContinueStatus::Stop {
            return WorkContinueStatus::Stop;
        }

        if self.full_hashing(stop_flag, progress_sender, pre_checked_map) == WorkContinueStatus::Stop {
            return WorkContinueStatus::Stop;
        }

        self.hash_reference_folders();

        // Clean unused data
        let files_with_identical_size = mem::take(&mut self.files_with_identical_size);
        thread::spawn(move || drop(files_with_identical_size));

        WorkContinueStatus::Continue
    }
}

pub fn get_duplicate_cache_file(type_of_hash: HashType, is_prehash: bool) -> String {
    let prehash_str = if is_prehash { "_prehash" } else { "" };
    format!("cache_duplicates_{type_of_hash:?}{prehash_str}_{CACHE_DUPLICATE_VERSION}.bin")
}
