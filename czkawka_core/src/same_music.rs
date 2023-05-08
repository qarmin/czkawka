use std::cmp::max;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::{mem, panic};

use anyhow::Context;
use crossbeam_channel::Receiver;
use futures::channel::mpsc::UnboundedSender;
use lofty::TaggedFileExt;
use lofty::{read_from, AudioFile, ItemKey};
use rayon::prelude::*;
use rusty_chromaprint::{match_fingerprints, Configuration, Fingerprinter};
use serde::{Deserialize, Serialize};
use symphonia::core::audio::SampleBuffer;
use symphonia::core::codecs::{DecoderOptions, CODEC_TYPE_NULL};
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;

use crate::common::{create_crash_message, prepare_thread_handler_common, send_info_and_wait_for_ending_all_threads, AUDIO_FILES_EXTENSIONS};
use crate::common::{filter_reference_folders_generic, open_cache_folder};
use crate::common_dir_traversal::{CheckingMethod, DirTraversalBuilder, DirTraversalResult, FileEntry, ProgressData, ToolType};
use crate::common_directory::Directories;
use crate::common_extensions::Extensions;
use crate::common_items::ExcludedItems;
use crate::common_messages::Messages;
use crate::common_traits::*;

#[derive(Eq, PartialEq, Clone, Debug, Copy)]
pub enum DeleteMethod {
    None,
    Delete,
}

bitflags! {
    #[derive(PartialEq, Copy, Clone, Debug)]
    pub struct MusicSimilarity : u32 {
        const NONE = 0;

        const TRACK_TITLE = 0b1;
        const TRACK_ARTIST = 0b10;
        const YEAR = 0b100;
        const LENGTH = 0b1000;
        const GENRE = 0b10000;
        const BITRATE = 0b10_0000;
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MusicEntry {
    pub size: u64,

    pub path: PathBuf,
    pub modified_date: u64,
    pub fingerprint: Vec<u32>,

    pub track_title: String,
    pub track_artist: String,
    pub year: String,
    pub length: String,
    pub genre: String,
    pub bitrate: u32,
}

impl ResultEntry for MusicEntry {
    fn get_path(&self) -> &Path {
        &self.path
    }
}

impl FileEntry {
    fn to_music_entry(&self) -> MusicEntry {
        MusicEntry {
            size: self.size,
            path: self.path.clone(),
            modified_date: self.modified_date,

            fingerprint: vec![],
            track_title: String::new(),
            track_artist: String::new(),
            year: String::new(),
            length: String::new(),
            genre: String::new(),
            bitrate: 0,
        }
    }
}

/// Info struck with helpful information's about results
#[derive(Default)]
pub struct Info {
    pub number_of_duplicates: usize,
    pub number_of_groups: u64,
}

impl Info {
    #[must_use]
    pub fn new() -> Self {
        Default::default()
    }
}

/// Struct with required information's to work
pub struct SameMusic {
    tool_type: ToolType,
    text_messages: Messages,
    information: Info,
    music_to_check: HashMap<String, MusicEntry>,
    music_entries: Vec<MusicEntry>,
    duplicated_music_entries: Vec<Vec<MusicEntry>>,
    duplicated_music_entries_referenced: Vec<(MusicEntry, Vec<MusicEntry>)>,
    directories: Directories,
    allowed_extensions: Extensions,
    excluded_items: ExcludedItems,
    minimal_file_size: u64,
    maximal_file_size: u64,
    recursive_search: bool,
    delete_method: DeleteMethod,
    music_similarity: MusicSimilarity,
    stopped_search: bool,
    approximate_comparison: bool,
    use_cache: bool,
    delete_outdated_cache: bool, // TODO add this to GUI
    use_reference_folders: bool,
    save_also_as_json: bool,
    check_type: CheckingMethod,
    hash_preset_config: Configuration,
    minimum_segment_duration: f32,
    maximum_difference: f64,
}

impl SameMusic {
    #[must_use]
    pub fn new() -> Self {
        Self {
            tool_type: ToolType::SameMusic,
            text_messages: Messages::new(),
            information: Info::new(),
            recursive_search: true,
            directories: Directories::new(),
            allowed_extensions: Extensions::new(),
            excluded_items: ExcludedItems::new(),
            music_entries: Vec::with_capacity(2048),
            delete_method: DeleteMethod::None,
            music_similarity: MusicSimilarity::NONE,
            stopped_search: false,
            minimal_file_size: 8192,
            maximal_file_size: u64::MAX,
            duplicated_music_entries: vec![],
            music_to_check: Default::default(),
            approximate_comparison: true,
            use_cache: false,
            delete_outdated_cache: true,
            use_reference_folders: false,
            duplicated_music_entries_referenced: vec![],
            save_also_as_json: false,
            check_type: CheckingMethod::AudioContent,
            hash_preset_config: Configuration::preset_test1(), // TODO allow to change this
            minimum_segment_duration: 10.0,
            maximum_difference: 2.0,
        }
    }

    pub fn find_same_music(&mut self, stop_receiver: Option<&Receiver<()>>, progress_sender: Option<&UnboundedSender<ProgressData>>) {
        self.directories.optimize_directories(self.recursive_search, &mut self.text_messages);
        self.use_reference_folders = !self.directories.reference_directories.is_empty();
        if !self.check_files(stop_receiver, progress_sender) {
            self.stopped_search = true;
            return;
        }
        match self.check_type {
            CheckingMethod::AudioTags => {
                if !self.read_tags(stop_receiver, progress_sender) {
                    self.stopped_search = true;
                    return;
                }
                if !self.check_for_duplicate_tags(stop_receiver, progress_sender) {
                    self.stopped_search = true;
                    return;
                }
            }
            CheckingMethod::AudioContent => {
                if !self.calculate_fingerprint(stop_receiver, progress_sender) {
                    self.stopped_search = true;
                    return;
                }
                if !self.check_for_duplicate_fingerprints(stop_receiver, progress_sender) {
                    self.stopped_search = true;
                    return;
                }
                if !self.read_tags_to_files_similar_by_content(stop_receiver, progress_sender) {
                    self.stopped_search = true;
                    return;
                }
            }
            _ => panic!(),
        }
        self.delete_files();
        self.debug_print();
    }

    #[must_use]
    pub fn get_stopped_search(&self) -> bool {
        self.stopped_search
    }

    #[must_use]
    pub const fn get_duplicated_music_entries(&self) -> &Vec<Vec<MusicEntry>> {
        &self.duplicated_music_entries
    }
    #[must_use]
    pub const fn get_music_similarity(&self) -> &MusicSimilarity {
        &self.music_similarity
    }

    #[must_use]
    pub const fn get_text_messages(&self) -> &Messages {
        &self.text_messages
    }

    #[must_use]
    pub const fn get_information(&self) -> &Info {
        &self.information
    }

    pub fn set_delete_method(&mut self, delete_method: DeleteMethod) {
        self.delete_method = delete_method;
    }

    pub fn set_save_also_as_json(&mut self, save_also_as_json: bool) {
        self.save_also_as_json = save_also_as_json;
    }

    pub fn set_use_cache(&mut self, use_cache: bool) {
        self.use_cache = use_cache;
    }

    pub fn set_approximate_comparison(&mut self, approximate_comparison: bool) {
        self.approximate_comparison = approximate_comparison;
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

    /// Set included dir which needs to be relative, exists etc.
    pub fn set_included_directory(&mut self, included_directory: Vec<PathBuf>) {
        self.directories.set_included_directory(included_directory, &mut self.text_messages);
    }

    pub fn set_maximum_difference(&mut self, maximum_difference: f64) {
        self.maximum_difference = maximum_difference;
    }
    pub fn set_minimum_segment_duration(&mut self, minimum_segment_duration: f32) {
        self.minimum_segment_duration = minimum_segment_duration;
    }

    pub fn set_reference_directory(&mut self, reference_directory: Vec<PathBuf>) {
        self.directories.set_reference_directory(reference_directory);
    }

    pub fn set_check_type(&mut self, check_type: CheckingMethod) {
        assert!([CheckingMethod::AudioTags, CheckingMethod::AudioContent].contains(&check_type));
        self.check_type = check_type;
    }

    #[must_use]
    pub fn get_check_type(&self) -> CheckingMethod {
        self.check_type
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

    pub fn set_music_similarity(&mut self, music_similarity: MusicSimilarity) {
        self.music_similarity = music_similarity;
    }

    pub fn set_maximal_file_size(&mut self, maximal_file_size: u64) {
        self.maximal_file_size = match maximal_file_size {
            0 => 1,
            t => t,
        };
    }

    #[must_use]
    pub fn get_similar_music_referenced(&self) -> &Vec<(MusicEntry, Vec<MusicEntry>)> {
        &self.duplicated_music_entries_referenced
    }

    #[must_use]
    pub fn get_number_of_base_duplicated_files(&self) -> usize {
        if self.use_reference_folders {
            self.duplicated_music_entries_referenced.len()
        } else {
            self.duplicated_music_entries.len()
        }
    }

    #[must_use]
    pub fn get_use_reference(&self) -> bool {
        self.use_reference_folders
    }

    fn check_files(&mut self, stop_receiver: Option<&Receiver<()>>, progress_sender: Option<&UnboundedSender<ProgressData>>) -> bool {
        if !self.allowed_extensions.using_custom_extensions() {
            self.allowed_extensions.extend_allowed_extensions(AUDIO_FILES_EXTENSIONS);
        } else {
            self.allowed_extensions.validate_allowed_extensions(AUDIO_FILES_EXTENSIONS);
            if !self.allowed_extensions.using_custom_extensions() {
                return true;
            }
        }

        let result = DirTraversalBuilder::new()
            .root_dirs(self.directories.included_directories.clone())
            .group_by(|_fe| ())
            .stop_receiver(stop_receiver)
            .progress_sender(progress_sender)
            .minimal_file_size(self.minimal_file_size)
            .maximal_file_size(self.maximal_file_size)
            .directories(self.directories.clone())
            .allowed_extensions(self.allowed_extensions.clone())
            .excluded_items(self.excluded_items.clone())
            .recursive_search(self.recursive_search)
            .max_stage(2)
            .build()
            .run();
        match result {
            DirTraversalResult::SuccessFiles { grouped_file_entries, warnings } => {
                if let Some(music_to_check) = grouped_file_entries.get(&()) {
                    for fe in music_to_check {
                        self.music_to_check.insert(fe.path.to_string_lossy().to_string(), fe.to_music_entry());
                    }
                }
                self.text_messages.warnings.extend(warnings);

                true
            }
            DirTraversalResult::SuccessFolders { .. } => {
                unreachable!()
            }
            DirTraversalResult::Stopped => false,
        }
    }

    fn load_cache(&mut self, checking_tags: bool) -> (HashMap<String, MusicEntry>, HashMap<String, MusicEntry>, HashMap<String, MusicEntry>) {
        let loaded_hash_map;

        let mut records_already_cached: HashMap<String, MusicEntry> = Default::default();
        let mut non_cached_files_to_check: HashMap<String, MusicEntry> = Default::default();

        if self.use_cache {
            loaded_hash_map = match load_cache_from_file(&mut self.text_messages, self.delete_outdated_cache, checking_tags) {
                Some(t) => t,
                None => Default::default(),
            };

            for (name, file_entry) in &self.music_to_check {
                if !loaded_hash_map.contains_key(name) {
                    // If loaded data doesn't contains current image info
                    non_cached_files_to_check.insert(name.clone(), file_entry.clone());
                } else {
                    let loaded_item = loaded_hash_map.get(name).unwrap();
                    if file_entry.size != loaded_item.size || file_entry.modified_date != loaded_item.modified_date {
                        // When size or modification date of image changed, then it is clear that is different image
                        non_cached_files_to_check.insert(name.clone(), file_entry.clone());
                    } else {
                        // Checking may be omitted when already there is entry with same size and modification date
                        records_already_cached.insert(name.clone(), loaded_item.clone());
                    }
                }
            }
        } else {
            loaded_hash_map = Default::default();
            mem::swap(&mut self.music_to_check, &mut non_cached_files_to_check);
        }
        (loaded_hash_map, records_already_cached, non_cached_files_to_check)
    }

    fn save_cache(&mut self, vec_file_entry: Vec<MusicEntry>, loaded_hash_map: HashMap<String, MusicEntry>, checking_tags: bool) {
        if !self.use_cache {
            return;
        }
        // Must save all results to file, old loaded from file with all currently counted results
        let mut all_results: HashMap<String, MusicEntry> = loaded_hash_map;

        for file_entry in vec_file_entry {
            all_results.insert(file_entry.path.to_string_lossy().to_string(), file_entry);
        }
        save_cache_to_file(&all_results, &mut self.text_messages, self.save_also_as_json, checking_tags);
    }

    fn calculate_fingerprint(&mut self, stop_receiver: Option<&Receiver<()>>, progress_sender: Option<&UnboundedSender<ProgressData>>) -> bool {
        let (loaded_hash_map, records_already_cached, non_cached_files_to_check) = self.load_cache(false);

        let (progress_thread_handle, progress_thread_run, atomic_counter, check_was_stopped) =
            prepare_thread_handler_common(progress_sender, 1, 3, non_cached_files_to_check.len(), self.check_type, self.tool_type);
        let configuration = &self.hash_preset_config;

        // Clean for duplicate files
        let mut vec_file_entry = non_cached_files_to_check
            .into_par_iter()
            .map(|(path, mut music_entry)| {
                atomic_counter.fetch_add(1, Ordering::Relaxed);
                if stop_receiver.is_some() && stop_receiver.unwrap().try_recv().is_ok() {
                    check_was_stopped.store(true, Ordering::Relaxed);
                    return None;
                }

                let Ok(fingerprint)  = calc_fingerprint_helper(path, configuration) else  {
                        return Some(None);
                };
                music_entry.fingerprint = fingerprint;

                Some(Some(music_entry))
            })
            .while_some()
            .filter(Option::is_some)
            .map(Option::unwrap)
            .collect::<Vec<_>>();

        send_info_and_wait_for_ending_all_threads(&progress_thread_run, progress_thread_handle);

        // Just connect loaded results with already calculated
        vec_file_entry.extend(records_already_cached.into_values());

        self.music_entries = vec_file_entry.clone();

        self.save_cache(vec_file_entry, loaded_hash_map, false);

        // Break if stop was clicked after saving to cache
        if check_was_stopped.load(Ordering::Relaxed) {
            return false;
        }

        true
    }

    fn read_tags(&mut self, stop_receiver: Option<&Receiver<()>>, progress_sender: Option<&UnboundedSender<ProgressData>>) -> bool {
        let (loaded_hash_map, records_already_cached, non_cached_files_to_check) = self.load_cache(true);

        let (progress_thread_handle, progress_thread_run, atomic_counter, check_was_stopped) =
            prepare_thread_handler_common(progress_sender, 1, 2, non_cached_files_to_check.len(), self.check_type, self.tool_type);

        // Clean for duplicate files
        let mut vec_file_entry = non_cached_files_to_check
            .into_par_iter()
            .map(|(path, mut music_entry)| {
                atomic_counter.fetch_add(1, Ordering::Relaxed);
                if stop_receiver.is_some() && stop_receiver.unwrap().try_recv().is_ok() {
                    check_was_stopped.store(true, Ordering::Relaxed);
                    return None;
                }
                if read_single_file_tag(&path, &mut music_entry) {
                    Some(Some(music_entry))
                } else {
                    Some(None)
                }
            })
            .while_some()
            .filter(Option::is_some)
            .map(Option::unwrap)
            .collect::<Vec<_>>();

        send_info_and_wait_for_ending_all_threads(&progress_thread_run, progress_thread_handle);

        // Just connect loaded results with already calculated
        vec_file_entry.extend(records_already_cached.into_values());

        self.music_entries = vec_file_entry.clone();

        self.save_cache(vec_file_entry, loaded_hash_map, true);

        // Break if stop was clicked after saving to cache
        if check_was_stopped.load(Ordering::Relaxed) {
            return false;
        }

        true
    }

    fn check_for_duplicate_tags(&mut self, stop_receiver: Option<&Receiver<()>>, progress_sender: Option<&UnboundedSender<ProgressData>>) -> bool {
        let (progress_thread_handle, progress_thread_run, atomic_counter, _check_was_stopped) =
            prepare_thread_handler_common(progress_sender, 2, 2, self.music_to_check.len(), self.check_type, self.tool_type);

        let mut old_duplicates: Vec<Vec<MusicEntry>> = vec![self.music_entries.clone()];
        let mut new_duplicates: Vec<Vec<MusicEntry>> = Vec::new();

        if (self.music_similarity & MusicSimilarity::TRACK_TITLE) == MusicSimilarity::TRACK_TITLE {
            if stop_receiver.is_some() && stop_receiver.unwrap().try_recv().is_ok() {
                send_info_and_wait_for_ending_all_threads(&progress_thread_run, progress_thread_handle);
                return false;
            }

            old_duplicates = self.check_music_item(old_duplicates, &atomic_counter, |fe| &fe.track_title, self.approximate_comparison);
        }
        if (self.music_similarity & MusicSimilarity::TRACK_ARTIST) == MusicSimilarity::TRACK_ARTIST {
            if stop_receiver.is_some() && stop_receiver.unwrap().try_recv().is_ok() {
                send_info_and_wait_for_ending_all_threads(&progress_thread_run, progress_thread_handle);
                return false;
            }

            old_duplicates = self.check_music_item(old_duplicates, &atomic_counter, |fe| &fe.track_artist, self.approximate_comparison);
        }
        if (self.music_similarity & MusicSimilarity::YEAR) == MusicSimilarity::YEAR {
            if stop_receiver.is_some() && stop_receiver.unwrap().try_recv().is_ok() {
                send_info_and_wait_for_ending_all_threads(&progress_thread_run, progress_thread_handle);
                return false;
            }

            old_duplicates = self.check_music_item(old_duplicates, &atomic_counter, |fe| &fe.year, false);
        }
        if (self.music_similarity & MusicSimilarity::LENGTH) == MusicSimilarity::LENGTH {
            if stop_receiver.is_some() && stop_receiver.unwrap().try_recv().is_ok() {
                send_info_and_wait_for_ending_all_threads(&progress_thread_run, progress_thread_handle);
                return false;
            }

            old_duplicates = self.check_music_item(old_duplicates, &atomic_counter, |fe| &fe.length, false);
        }
        if (self.music_similarity & MusicSimilarity::GENRE) == MusicSimilarity::GENRE {
            if stop_receiver.is_some() && stop_receiver.unwrap().try_recv().is_ok() {
                send_info_and_wait_for_ending_all_threads(&progress_thread_run, progress_thread_handle);
                return false;
            }

            old_duplicates = self.check_music_item(old_duplicates, &atomic_counter, |fe| &fe.genre, false);
        }
        if (self.music_similarity & MusicSimilarity::BITRATE) == MusicSimilarity::BITRATE {
            if stop_receiver.is_some() && stop_receiver.unwrap().try_recv().is_ok() {
                send_info_and_wait_for_ending_all_threads(&progress_thread_run, progress_thread_handle);
                return false;
            }
            let old_duplicates_len = old_duplicates.len();
            for vec_file_entry in old_duplicates {
                let mut hash_map: BTreeMap<String, Vec<MusicEntry>> = Default::default();
                for file_entry in vec_file_entry {
                    if file_entry.bitrate != 0 {
                        let thing = file_entry.bitrate.to_string();
                        if !thing.is_empty() {
                            hash_map.entry(thing.clone()).or_insert_with(Vec::new).push(file_entry);
                        }
                    }
                }
                for (_title, vec_file_entry) in hash_map {
                    if vec_file_entry.len() > 1 {
                        new_duplicates.push(vec_file_entry);
                    }
                }
            }
            atomic_counter.fetch_add(old_duplicates_len, Ordering::Relaxed);
            old_duplicates = new_duplicates;
        }

        send_info_and_wait_for_ending_all_threads(&progress_thread_run, progress_thread_handle);

        self.duplicated_music_entries = old_duplicates;

        if self.use_reference_folders {
            self.duplicated_music_entries_referenced = filter_reference_folders_generic(mem::take(&mut self.duplicated_music_entries), &self.directories);
        }

        if self.use_reference_folders {
            for (_fe, vector) in &self.duplicated_music_entries_referenced {
                self.information.number_of_duplicates += vector.len();
                self.information.number_of_groups += 1;
            }
        } else {
            for vector in &self.duplicated_music_entries {
                self.information.number_of_duplicates += vector.len() - 1;
                self.information.number_of_groups += 1;
            }
        }

        // Clear unused data
        self.music_entries.clear();

        true
    }
    fn read_tags_to_files_similar_by_content(&mut self, stop_receiver: Option<&Receiver<()>>, progress_sender: Option<&UnboundedSender<ProgressData>>) -> bool {
        let groups_to_check = max(self.duplicated_music_entries.len(), self.duplicated_music_entries_referenced.len());
        let (progress_thread_handle, progress_thread_run, atomic_counter, check_was_stopped) =
            prepare_thread_handler_common(progress_sender, 3, 3, groups_to_check, self.check_type, self.tool_type);

        // TODO is ther a way to just run iterator and not collect any info?
        if !self.duplicated_music_entries.is_empty() {
            let _: Vec<_> = self
                .duplicated_music_entries
                .par_iter_mut()
                .map(|vec_me| {
                    atomic_counter.fetch_add(1, Ordering::Relaxed);
                    if stop_receiver.is_some() && stop_receiver.unwrap().try_recv().is_ok() {
                        check_was_stopped.store(true, Ordering::Relaxed);
                        return None;
                    }
                    for me in vec_me {
                        let me_path = me.path.to_string_lossy().to_string();
                        read_single_file_tag(&me_path, me);
                    }
                    Some(())
                })
                .while_some()
                .collect();
        } else {
            let _: Vec<_> = self
                .duplicated_music_entries_referenced
                .par_iter_mut()
                .map(|(me_o, vec_me)| {
                    atomic_counter.fetch_add(1, Ordering::Relaxed);
                    if stop_receiver.is_some() && stop_receiver.unwrap().try_recv().is_ok() {
                        check_was_stopped.store(true, Ordering::Relaxed);
                        return None;
                    }
                    let me_o_path = me_o.path.to_string_lossy().to_string();
                    read_single_file_tag(&me_o_path, me_o);
                    for me in vec_me {
                        let me_path = me.path.to_string_lossy().to_string();
                        read_single_file_tag(&me_path, me);
                    }
                    Some(())
                })
                .while_some()
                .collect();
        }

        send_info_and_wait_for_ending_all_threads(&progress_thread_run, progress_thread_handle);

        !check_was_stopped.load(Ordering::Relaxed)
    }

    fn split_fingerprints_to_check(&mut self) -> (Vec<MusicEntry>, Vec<MusicEntry>) {
        let base_files: Vec<MusicEntry>;
        let files_to_compare: Vec<MusicEntry>;

        if self.use_reference_folders {
            (base_files, files_to_compare) = mem::take(&mut self.music_entries)
                .into_iter()
                .partition(|f| self.directories.is_in_referenced_directory(f.get_path()));
        } else {
            base_files = self.music_entries.clone();
            files_to_compare = mem::take(&mut self.music_entries);
        }

        (base_files, files_to_compare)
    }

    fn compare_fingerprints(
        &mut self,
        stop_receiver: Option<&Receiver<()>>,
        atomic_counter: &Arc<AtomicUsize>,
        base_files: Vec<MusicEntry>,
        files_to_compare: &[MusicEntry],
    ) -> Option<Vec<Vec<MusicEntry>>> {
        let mut used_paths: HashSet<String> = Default::default();

        let configuration = &self.hash_preset_config;
        let minimum_segment_duration = self.minimum_segment_duration;
        let maximum_difference = self.maximum_difference;

        let mut duplicated_music_entries = Vec::new();

        for f_entry in base_files {
            atomic_counter.fetch_add(1, Ordering::Relaxed);
            if stop_receiver.is_some() && stop_receiver.unwrap().try_recv().is_ok() {
                return None;
            }

            let f_string = f_entry.path.to_string_lossy().to_string();
            if used_paths.contains(&f_string) {
                continue;
            }

            let mut collected_similar_items = files_to_compare
                .par_iter()
                .filter_map(|e_entry| {
                    let e_string = e_entry.path.to_string_lossy().to_string();
                    if used_paths.contains(&e_string) || e_string == f_string {
                        return None;
                    }
                    let mut segments = match_fingerprints(&f_entry.fingerprint, &e_entry.fingerprint, configuration).unwrap();
                    segments.retain(|s| s.duration(configuration) > minimum_segment_duration && s.score < maximum_difference);
                    if segments.is_empty() {
                        None
                    } else {
                        Some((e_string, e_entry))
                    }
                })
                .collect::<Vec<_>>();

            collected_similar_items.retain(|(path, _entry)| !used_paths.contains(path));
            if !collected_similar_items.is_empty() {
                let mut music_entries = Vec::new();
                for (path, entry) in collected_similar_items {
                    used_paths.insert(path);
                    music_entries.push(entry.clone());
                }
                used_paths.insert(f_string);
                music_entries.push(f_entry.clone());
                duplicated_music_entries.push(music_entries);
            }
        }
        Some(duplicated_music_entries)
    }

    fn check_for_duplicate_fingerprints(&mut self, stop_receiver: Option<&Receiver<()>>, progress_sender: Option<&UnboundedSender<ProgressData>>) -> bool {
        let (base_files, files_to_compare) = self.split_fingerprints_to_check();
        let (progress_thread_handle, progress_thread_run, atomic_counter, _check_was_stopped) =
            prepare_thread_handler_common(progress_sender, 2, 3, base_files.len(), self.check_type, self.tool_type);

        let Some(duplicated_music_entries) = self.compare_fingerprints(stop_receiver, &atomic_counter, base_files, &files_to_compare) else {
            send_info_and_wait_for_ending_all_threads(&progress_thread_run, progress_thread_handle);
            return false;
        };

        send_info_and_wait_for_ending_all_threads(&progress_thread_run, progress_thread_handle);

        self.duplicated_music_entries = duplicated_music_entries;

        if self.use_reference_folders {
            self.duplicated_music_entries_referenced = filter_reference_folders_generic(mem::take(&mut self.duplicated_music_entries), &self.directories);
        }

        if self.use_reference_folders {
            for (_fe, vector) in &self.duplicated_music_entries_referenced {
                self.information.number_of_duplicates += vector.len();
                self.information.number_of_groups += 1;
            }
        } else {
            for vector in &self.duplicated_music_entries {
                self.information.number_of_duplicates += vector.len() - 1;
                self.information.number_of_groups += 1;
            }
        }

        // Clear unused data
        self.music_entries.clear();

        true
    }

    fn check_music_item(
        &self,
        old_duplicates: Vec<Vec<MusicEntry>>,
        atomic_counter: &Arc<AtomicUsize>,
        get_item: fn(&MusicEntry) -> &str,
        approximate_comparison: bool,
    ) -> Vec<Vec<MusicEntry>> {
        let mut new_duplicates: Vec<_> = Default::default();
        let old_duplicates_len = old_duplicates.len();
        for vec_file_entry in old_duplicates {
            let mut hash_map: BTreeMap<String, Vec<MusicEntry>> = Default::default();
            for file_entry in vec_file_entry {
                let mut thing = get_item(&file_entry).trim().to_lowercase();
                if approximate_comparison {
                    get_approximate_conversion(&mut thing);
                }
                if !thing.is_empty() {
                    hash_map.entry(thing).or_insert_with(Vec::new).push(file_entry);
                }
            }
            for (_title, vec_file_entry) in hash_map {
                if vec_file_entry.len() > 1 {
                    new_duplicates.push(vec_file_entry);
                }
            }
        }
        atomic_counter.fetch_add(old_duplicates_len, Ordering::Relaxed);

        new_duplicates
    }

    pub fn set_minimal_file_size(&mut self, minimal_file_size: u64) {
        self.minimal_file_size = match minimal_file_size {
            0 => 1,
            t => t,
        };
    }

    /// Function to delete files, from filed Vector
    fn delete_files(&mut self) {

        // TODO
        // match self.delete_method {
        //     DeleteMethod::Delete => {
        //         for file_entry in &self.music_entries {
        //             if fs::remove_file(file_entry.path.clone()).is_err() {
        //                 self.text_messages.warnings.push(file_entry.path.display().to_string());
        //             }
        //         }
        //     }
        //     DeleteMethod::None => {
        //         //Just do nothing
        //     }
        // }
    }
}

fn save_cache_to_file(hashmap: &HashMap<String, MusicEntry>, text_messages: &mut Messages, save_also_as_json: bool, checking_tags: bool) {
    if let Some(((file_handler, cache_file), (file_handler_json, cache_file_json))) =
        open_cache_folder(get_cache_file(checking_tags), true, save_also_as_json, &mut text_messages.warnings)
    {
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

fn load_cache_from_file(text_messages: &mut Messages, delete_outdated_cache: bool, checking_tags: bool) -> Option<HashMap<String, MusicEntry>> {
    if let Some(((file_handler, cache_file), (file_handler_json, cache_file_json))) = open_cache_folder(get_cache_file(checking_tags), false, true, &mut text_messages.warnings) {
        let mut hashmap_loaded_entries: HashMap<String, MusicEntry>;
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

// TODO this should be taken from rusty-chromaprint repo, not reimplemented here
fn calc_fingerprint_helper(path: impl AsRef<Path>, config: &Configuration) -> anyhow::Result<Vec<u32>> {
    let path = path.as_ref();
    let src = File::open(path).context("failed to open file")?;
    let mss = MediaSourceStream::new(Box::new(src), Default::default());

    let mut hint = Hint::new();
    if let Some(ext) = path.extension().and_then(std::ffi::OsStr::to_str) {
        hint.with_extension(ext);
    }

    let meta_opts: MetadataOptions = Default::default();
    let fmt_opts: FormatOptions = Default::default();

    let probed = symphonia::default::get_probe().format(&hint, mss, &fmt_opts, &meta_opts).context("unsupported format")?;

    let mut format = probed.format;

    let track = format
        .tracks()
        .iter()
        .find(|t| t.codec_params.codec != CODEC_TYPE_NULL)
        .context("no supported audio tracks")?;

    let dec_opts: DecoderOptions = Default::default();

    let mut decoder = symphonia::default::get_codecs().make(&track.codec_params, &dec_opts).context("unsupported codec")?;

    let track_id = track.id;

    let mut printer = Fingerprinter::new(config);
    let sample_rate = track.codec_params.sample_rate.context("missing sample rate")?;
    let channels = track.codec_params.channels.context("missing audio channels")?.count() as u32;
    printer.start(sample_rate, channels).context("initializing fingerprinter")?;

    let mut sample_buf = None;

    loop {
        let Ok(packet) = format.next_packet() else  { break };

        if packet.track_id() != track_id {
            continue;
        }

        match decoder.decode(&packet) {
            Ok(audio_buf) => {
                if sample_buf.is_none() {
                    let spec = *audio_buf.spec();
                    let duration = audio_buf.capacity() as u64;
                    sample_buf = Some(SampleBuffer::<i16>::new(duration, spec));
                }

                if let Some(buf) = &mut sample_buf {
                    buf.copy_interleaved_ref(audio_buf);
                    printer.consume(buf.samples());
                }
            }
            Err(symphonia::core::errors::Error::DecodeError(_)) => (),
            Err(_) => break,
        }
    }

    printer.finish();
    Ok(printer.fingerprint().to_vec())
}

fn read_single_file_tag(path: &str, music_entry: &mut MusicEntry) -> bool {
    let Ok(mut file) = File::open(path) else { return false; };

    let result = panic::catch_unwind(move || {
        match read_from(&mut file) {
            Ok(t) => Some(t),
            Err(_inspected) => {
                // println!("Failed to open {}", path);
                None
            }
        }
    });

    let tagged_file = if let Ok(t) = result {
        match t {
            Some(r) => r,
            None => {
                return true;
            }
        }
    } else {
        let message = create_crash_message("Lofty", path, "https://github.com/image-rs/image/issues");
        println!("{message}");
        return false;
    };

    let properties = tagged_file.properties();

    let mut track_title = String::new();
    let mut track_artist = String::new();
    let mut year = String::new();
    let mut genre = String::new();

    let bitrate = properties.audio_bitrate().unwrap_or(0);
    let mut length = properties.duration().as_millis().to_string();

    if let Some(tag) = tagged_file.primary_tag() {
        track_title = tag.get_string(&ItemKey::TrackTitle).unwrap_or("").to_string();
        track_artist = tag.get_string(&ItemKey::TrackArtist).unwrap_or("").to_string();
        year = tag.get_string(&ItemKey::Year).unwrap_or("").to_string();
        genre = tag.get_string(&ItemKey::Genre).unwrap_or("").to_string();
    }

    for tag in tagged_file.tags() {
        if track_title.is_empty() {
            if let Some(tag_value) = tag.get_string(&ItemKey::TrackTitle) {
                track_title = tag_value.to_string();
            }
        }
        if track_artist.is_empty() {
            if let Some(tag_value) = tag.get_string(&ItemKey::TrackArtist) {
                track_artist = tag_value.to_string();
            }
        }
        if year.is_empty() {
            if let Some(tag_value) = tag.get_string(&ItemKey::Year) {
                year = tag_value.to_string();
            }
        }
        if genre.is_empty() {
            if let Some(tag_value) = tag.get_string(&ItemKey::Genre) {
                genre = tag_value.to_string();
            }
        }
        // println!("{:?}", tag.items());
    }

    if let Ok(old_length_number) = length.parse::<u32>() {
        let length_number = old_length_number / 60;
        let minutes = length_number / 1000;
        let seconds = (length_number % 1000) * 6 / 100;
        if minutes != 0 || seconds != 0 {
            length = format!("{minutes}:{seconds:02}");
        } else if old_length_number > 0 {
            // That means, that audio have length smaller that second, but length is properly read
            length = "0:01".to_string();
        } else {
            length = String::new();
        }
    } else {
        length = String::new();
    }

    music_entry.track_title = track_title;
    music_entry.track_artist = track_artist;
    music_entry.year = year;
    music_entry.length = length;
    music_entry.genre = genre;
    music_entry.bitrate = bitrate;

    true
}

// Using different cache folders, because loading cache just for finding duplicated tags would be really slow
fn get_cache_file(checking_tags: bool) -> &'static str {
    if checking_tags {
        "cache_same_music_tags.bin"
    } else {
        "cache_same_music_fingerprints.bin"
    }
}

impl Default for SameMusic {
    fn default() -> Self {
        Self::new()
    }
}

impl DebugPrint for SameMusic {
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

        println!("### Other");

        println!("Excluded items - {:?}", self.excluded_items.items);
        println!("Minimum file size - {:?}", self.minimal_file_size);
        println!("Found files music - {}", self.music_entries.len());
        println!("Found duplicated files music - {}", self.duplicated_music_entries.len());
        println!("Included directories - {:?}", self.directories.included_directories);
        println!("Excluded directories - {:?}", self.directories.excluded_directories);
        println!("Recursive search - {}", self.recursive_search);
        #[cfg(target_family = "unix")]
        println!("Skip other filesystems - {}", self.directories.exclude_other_filesystems());
        println!("Delete Method - {:?}", self.delete_method);
        println!("-----------------------------------------");
    }
}

impl SaveResults for SameMusic {
    fn save_results_to_file(&mut self, file_name: &str) -> bool {
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

        if !self.music_entries.is_empty() {
            writeln!(writer, "Found {} same music files.", self.information.number_of_duplicates).unwrap();
            for file_entry in &self.music_entries {
                writeln!(writer, "{}", file_entry.path.display()).unwrap();
            }
        } else {
            write!(writer, "Not found any empty files.").unwrap();
        }

        true
    }
}

impl PrintResults for SameMusic {
    /// Print information's about duplicated entries
    /// Only needed for CLI
    fn print_results(&self) {
        println!("Found {} similar music files.\n", self.duplicated_music_entries.len());
        for vec_file_entry in &self.duplicated_music_entries {
            for file_entry in vec_file_entry {
                println!(
                    "TT: {}  -  TA: {}  -  Y: {}  -  L: {}  -  G: {}  -  B: {}  -  P: {}",
                    file_entry.track_title,
                    file_entry.track_artist,
                    file_entry.year,
                    file_entry.length,
                    file_entry.genre,
                    file_entry.bitrate,
                    file_entry.path.display()
                );
            }
            println!();
        }
    }
}

fn get_approximate_conversion(what: &mut String) {
    let mut new_what = String::with_capacity(what.len());
    let mut tab_number = 0;
    let mut space_before = true;
    for character in what.chars() {
        match character {
            '(' | '[' => {
                tab_number += 1;
            }
            ')' | ']' => {
                if tab_number == 0 {
                    // Nothing to do, not even save it to output
                } else {
                    tab_number -= 1;
                }
            }
            ' ' => {
                if !space_before {
                    new_what.push(' ');
                    space_before = true;
                }
            }
            ch => {
                if tab_number == 0 {
                    // Ignore all non alphabetic ascii characters like " or .
                    if !ch.is_ascii() || ch.is_ascii_alphabetic() {
                        space_before = false;
                        new_what.push(ch);
                    } else if !space_before {
                        new_what.push(' ');
                        space_before = true;
                    }
                }
            }
        }
    }

    if new_what.ends_with(' ') {
        new_what.pop();
    }
    *what = new_what;
}

#[cfg(test)]
mod tests {
    use crate::same_music::get_approximate_conversion;

    #[test]
    fn test_strings() {
        let mut what = "roman ( ziemniak ) ".to_string();
        get_approximate_conversion(&mut what);
        assert_eq!(what, "roman");

        let mut what = "  HH)    ".to_string();
        get_approximate_conversion(&mut what);
        assert_eq!(what, "HH");

        let mut what = "  fsf.f.  ".to_string();
        get_approximate_conversion(&mut what);
        assert_eq!(what, "fsf f");

        let mut what = "Kekistan (feat. roman) [Mix on Mix]".to_string();
        get_approximate_conversion(&mut what);
        assert_eq!(what, "Kekistan");
    }
}
