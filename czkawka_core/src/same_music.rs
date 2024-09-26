use std::collections::{BTreeMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::{mem, panic};

use anyhow::Context;
use crossbeam_channel::{Receiver, Sender};
use fun_time::fun_time;
use humansize::{format_size, BINARY};
use lofty::file::{AudioFile, TaggedFileExt};
use lofty::prelude::*;
use lofty::read_from;
use log::debug;
use rayon::prelude::*;
use rusty_chromaprint::{match_fingerprints, Configuration, Fingerprinter};
use serde::{Deserialize, Serialize};
use symphonia::core::audio::SampleBuffer;
use symphonia::core::codecs::{DecoderOptions, CODEC_TYPE_NULL};
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;

use crate::common::{
    check_if_stop_received, create_crash_message, delete_files_custom, filter_reference_folders_generic, prepare_thread_handler_common, send_info_and_wait_for_ending_all_threads,
    AUDIO_FILES_EXTENSIONS,
};
use crate::common_cache::{get_similar_music_cache_file, load_cache_from_file_generalized_by_path, save_cache_to_file_generalized};
use crate::common_dir_traversal::{CheckingMethod, DirTraversalBuilder, DirTraversalResult, FileEntry, ToolType};
use crate::common_tool::{CommonData, CommonToolData, DeleteMethod};
use crate::common_traits::*;
use crate::progress_data::{CurrentStage, ProgressData};

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
    fn get_modified_date(&self) -> u64 {
        self.modified_date
    }
    fn get_size(&self) -> u64 {
        self.size
    }
}

impl FileEntry {
    fn into_music_entry(self) -> MusicEntry {
        MusicEntry {
            size: self.size,
            path: self.path,
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

struct GroupedFilesToCheck {
    pub base_files: Vec<MusicEntry>,
    pub files_to_compare: Vec<MusicEntry>,
}

#[derive(Default)]
pub struct Info {
    pub number_of_duplicates: usize,
    pub number_of_groups: u64,
}

pub struct SameMusicParameters {
    pub music_similarity: MusicSimilarity,
    pub approximate_comparison: bool,
    pub check_type: CheckingMethod,
    pub minimum_segment_duration: f32,
    pub maximum_difference: f64,
    pub compare_fingerprints_only_with_similar_titles: bool,
}

impl SameMusicParameters {
    pub fn new(
        music_similarity: MusicSimilarity,
        approximate_comparison: bool,
        check_type: CheckingMethod,
        minimum_segment_duration: f32,
        maximum_difference: f64,
        compare_fingerprints_only_with_similar_titles: bool,
    ) -> Self {
        assert!(!music_similarity.is_empty());
        assert!([CheckingMethod::AudioTags, CheckingMethod::AudioContent].contains(&check_type));
        Self {
            music_similarity,
            approximate_comparison,
            check_type,
            minimum_segment_duration,
            maximum_difference,
            compare_fingerprints_only_with_similar_titles,
        }
    }
}

pub struct SameMusic {
    common_data: CommonToolData,
    information: Info,
    music_to_check: BTreeMap<String, MusicEntry>,
    music_entries: Vec<MusicEntry>,
    duplicated_music_entries: Vec<Vec<MusicEntry>>,
    duplicated_music_entries_referenced: Vec<(MusicEntry, Vec<MusicEntry>)>,
    hash_preset_config: Configuration,
    params: SameMusicParameters,
}

impl SameMusic {
    pub fn new(params: SameMusicParameters) -> Self {
        Self {
            common_data: CommonToolData::new(ToolType::SameMusic),
            information: Info::default(),
            music_entries: Vec::with_capacity(2048),
            duplicated_music_entries: vec![],
            music_to_check: Default::default(),
            duplicated_music_entries_referenced: vec![],
            hash_preset_config: Configuration::preset_test1(), // TODO allow to change this and move to parameters
            params,
        }
    }

    #[fun_time(message = "find_same_music", level = "info")]
    pub fn find_same_music(&mut self, stop_receiver: Option<&Receiver<()>>, progress_sender: Option<&Sender<ProgressData>>) {
        self.prepare_items();
        self.common_data.use_reference_folders = !self.common_data.directories.reference_directories.is_empty();
        if !self.check_files(stop_receiver, progress_sender) {
            self.common_data.stopped_search = true;
            return;
        }
        match self.params.check_type {
            CheckingMethod::AudioTags => {
                if !self.read_tags(stop_receiver, progress_sender) {
                    self.common_data.stopped_search = true;
                    return;
                }
                if !self.check_for_duplicate_tags(stop_receiver, progress_sender) {
                    self.common_data.stopped_search = true;
                    return;
                }
            }
            CheckingMethod::AudioContent => {
                if !self.read_tags(stop_receiver, progress_sender) {
                    self.common_data.stopped_search = true;
                    return;
                }
                if !self.calculate_fingerprint(stop_receiver, progress_sender) {
                    self.common_data.stopped_search = true;
                    return;
                }
                if !self.check_for_duplicate_fingerprints(stop_receiver, progress_sender) {
                    self.common_data.stopped_search = true;
                    return;
                }
            }
            _ => panic!(),
        }
        self.delete_files();
        self.debug_print();
    }

    #[fun_time(message = "check_files", level = "debug")]
    fn check_files(&mut self, stop_receiver: Option<&Receiver<()>>, progress_sender: Option<&Sender<ProgressData>>) -> bool {
        self.common_data.extensions.set_and_validate_allowed_extensions(AUDIO_FILES_EXTENSIONS);
        if !self.common_data.extensions.set_any_extensions() {
            return true;
        }

        let result = DirTraversalBuilder::new()
            .group_by(|_fe| ())
            .stop_receiver(stop_receiver)
            .progress_sender(progress_sender)
            .common_data(&self.common_data)
            .checking_method(self.params.check_type)
            .build()
            .run();

        match result {
            DirTraversalResult::SuccessFiles { grouped_file_entries, warnings } => {
                self.music_to_check = grouped_file_entries
                    .into_values()
                    .flatten()
                    .map(|fe| (fe.path.to_string_lossy().to_string(), fe.into_music_entry()))
                    .collect();
                self.common_data.text_messages.warnings.extend(warnings);
                debug!("check_files - Found {} music files.", self.music_to_check.len());
                true
            }

            DirTraversalResult::Stopped => false,
        }
    }

    #[fun_time(message = "load_cache", level = "debug")]
    fn load_cache(&mut self, checking_tags: bool) -> (BTreeMap<String, MusicEntry>, BTreeMap<String, MusicEntry>, BTreeMap<String, MusicEntry>) {
        let loaded_hash_map;

        let mut records_already_cached: BTreeMap<String, MusicEntry> = Default::default();
        let mut non_cached_files_to_check: BTreeMap<String, MusicEntry> = Default::default();

        if self.common_data.use_cache {
            let (messages, loaded_items) =
                load_cache_from_file_generalized_by_path::<MusicEntry>(&get_similar_music_cache_file(checking_tags), self.get_delete_outdated_cache(), &self.music_to_check);
            self.get_text_messages_mut().extend_with_another_messages(messages);
            loaded_hash_map = loaded_items.unwrap_or_default();

            debug!("load_cache - Starting to check for differences");
            for (name, file_entry) in mem::take(&mut self.music_to_check) {
                if let Some(cached_file_entry) = loaded_hash_map.get(&name) {
                    records_already_cached.insert(name, cached_file_entry.clone());
                } else {
                    non_cached_files_to_check.insert(name, file_entry);
                }
            }
            debug!(
                "load_cache - completed diff between loaded and prechecked files, {}({}) - non cached, {}({}) - already cached",
                non_cached_files_to_check.len(),
                format_size(non_cached_files_to_check.values().map(|e| e.size).sum::<u64>(), BINARY),
                records_already_cached.len(),
                format_size(records_already_cached.values().map(|e| e.size).sum::<u64>(), BINARY),
            );
        } else {
            loaded_hash_map = Default::default();
            mem::swap(&mut self.music_to_check, &mut non_cached_files_to_check);
        }
        (loaded_hash_map, records_already_cached, non_cached_files_to_check)
    }

    #[fun_time(message = "save_cache", level = "debug")]
    fn save_cache(&mut self, vec_file_entry: Vec<MusicEntry>, loaded_hash_map: BTreeMap<String, MusicEntry>, checking_tags: bool) {
        if !self.common_data.use_cache {
            return;
        }
        // Must save all results to file, old loaded from file with all currently counted results
        let mut all_results: BTreeMap<String, MusicEntry> = loaded_hash_map;

        for file_entry in vec_file_entry {
            all_results.insert(file_entry.path.to_string_lossy().to_string(), file_entry);
        }

        let messages = save_cache_to_file_generalized(&get_similar_music_cache_file(checking_tags), &all_results, self.common_data.save_also_as_json, 0);
        self.get_text_messages_mut().extend_with_another_messages(messages);
    }

    #[fun_time(message = "calculate_fingerprint", level = "debug")]
    fn calculate_fingerprint(&mut self, stop_receiver: Option<&Receiver<()>>, progress_sender: Option<&Sender<ProgressData>>) -> bool {
        // We only calculate fingerprints, for files with similar titles
        // This saves a lot of time, because we don't need to calculate and later compare fingerprints for files with different titles

        if self.params.compare_fingerprints_only_with_similar_titles {
            let grouped_by_title: BTreeMap<String, Vec<MusicEntry>> = Self::get_entries_grouped_by_title(mem::take(&mut self.music_entries));
            self.music_to_check = grouped_by_title
                .into_iter()
                .filter_map(|(_title, entries)| if entries.len() >= 2 { Some(entries) } else { None })
                .flatten()
                .map(|e| (e.path.to_string_lossy().to_string(), e))
                .collect();
        } else {
            self.music_to_check = mem::take(&mut self.music_entries).into_iter().map(|e| (e.path.to_string_lossy().to_string(), e)).collect();
        }

        let (progress_thread_handle, progress_thread_run, _atomic_counter, _check_was_stopped) =
            prepare_thread_handler_common(progress_sender, CurrentStage::SameMusicCacheLoadingFingerprints, 0, self.get_test_type());

        let (loaded_hash_map, records_already_cached, non_cached_files_to_check) = self.load_cache(false);

        send_info_and_wait_for_ending_all_threads(&progress_thread_run, progress_thread_handle);
        if check_if_stop_received(stop_receiver) {
            return false;
        }

        let (progress_thread_handle, progress_thread_run, atomic_counter, check_was_stopped) = prepare_thread_handler_common(
            progress_sender,
            CurrentStage::SameMusicCalculatingFingerprints,
            non_cached_files_to_check.len(),
            self.get_test_type(),
        );
        let configuration = &self.hash_preset_config;

        debug!("calculate_fingerprint - starting fingerprinting");
        let mut vec_file_entry = non_cached_files_to_check
            .into_par_iter()
            .map(|(path, mut music_entry)| {
                atomic_counter.fetch_add(1, Ordering::Relaxed);
                if check_if_stop_received(stop_receiver) {
                    check_was_stopped.store(true, Ordering::Relaxed);
                    return None;
                }

                let Ok(fingerprint) = calc_fingerprint_helper(path, configuration) else {
                    return Some(None);
                };
                music_entry.fingerprint = fingerprint;

                Some(Some(music_entry))
            })
            .while_some()
            .flatten()
            .collect::<Vec<_>>();
        debug!("calculate_fingerprint - ended fingerprinting");

        send_info_and_wait_for_ending_all_threads(&progress_thread_run, progress_thread_handle);
        let (progress_thread_handle, progress_thread_run, _atomic_counter, _check_was_stopped) =
            prepare_thread_handler_common(progress_sender, CurrentStage::SameMusicCacheSavingFingerprints, 0, self.get_test_type());

        // Just connect loaded results with already calculated
        vec_file_entry.extend(records_already_cached.into_values());

        self.music_entries = vec_file_entry.clone();

        self.save_cache(vec_file_entry, loaded_hash_map, false);

        // Break if stop was clicked after saving to cache
        send_info_and_wait_for_ending_all_threads(&progress_thread_run, progress_thread_handle);
        if check_was_stopped.load(Ordering::Relaxed) || check_if_stop_received(stop_receiver) {
            return false;
        }
        true
    }

    #[fun_time(message = "read_tags", level = "debug")]
    fn read_tags(&mut self, stop_receiver: Option<&Receiver<()>>, progress_sender: Option<&Sender<ProgressData>>) -> bool {
        let (progress_thread_handle, progress_thread_run, _atomic_counter, _check_was_stopped) =
            prepare_thread_handler_common(progress_sender, CurrentStage::SameMusicCacheLoadingTags, 0, self.get_test_type());

        let (loaded_hash_map, records_already_cached, non_cached_files_to_check) = self.load_cache(true);

        send_info_and_wait_for_ending_all_threads(&progress_thread_run, progress_thread_handle);
        if check_if_stop_received(stop_receiver) {
            return false;
        }

        let (progress_thread_handle, progress_thread_run, atomic_counter, check_was_stopped) =
            prepare_thread_handler_common(progress_sender, CurrentStage::SameMusicReadingTags, non_cached_files_to_check.len(), self.get_test_type());

        debug!("read_tags - starting reading tags");
        // Clean for duplicate files
        let mut vec_file_entry = non_cached_files_to_check
            .into_par_iter()
            .map(|(path, mut music_entry)| {
                atomic_counter.fetch_add(1, Ordering::Relaxed);
                if check_if_stop_received(stop_receiver) {
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
            .filter_map(|e| e)
            .collect::<Vec<_>>();
        debug!("read_tags - ended reading tags");

        send_info_and_wait_for_ending_all_threads(&progress_thread_run, progress_thread_handle);
        let (progress_thread_handle, progress_thread_run, _atomic_counter, _check_was_stopped) =
            prepare_thread_handler_common(progress_sender, CurrentStage::SameMusicCacheSavingTags, 0, self.get_test_type());

        // Just connect loaded results with already calculated
        vec_file_entry.extend(records_already_cached.into_values());

        self.music_entries = vec_file_entry.clone();

        self.save_cache(vec_file_entry, loaded_hash_map, true);

        // Break if stop was clicked after saving to cache
        send_info_and_wait_for_ending_all_threads(&progress_thread_run, progress_thread_handle);
        if check_was_stopped.load(Ordering::Relaxed) {
            return false;
        }

        true
    }

    #[fun_time(message = "check_for_duplicate_tags", level = "debug")]
    fn check_for_duplicate_tags(&mut self, stop_receiver: Option<&Receiver<()>>, progress_sender: Option<&Sender<ProgressData>>) -> bool {
        if self.music_entries.is_empty() {
            return true;
        }
        let (progress_thread_handle, progress_thread_run, atomic_counter, _check_was_stopped) =
            prepare_thread_handler_common(progress_sender, CurrentStage::SameMusicComparingTags, self.music_entries.len(), self.get_test_type());

        let mut old_duplicates: Vec<Vec<MusicEntry>> = vec![self.music_entries.clone()];
        let mut new_duplicates: Vec<Vec<MusicEntry>> = Vec::new();

        if (self.params.music_similarity & MusicSimilarity::TRACK_TITLE) == MusicSimilarity::TRACK_TITLE {
            if check_if_stop_received(stop_receiver) {
                send_info_and_wait_for_ending_all_threads(&progress_thread_run, progress_thread_handle);
                return false;
            }

            old_duplicates = self.check_music_item(old_duplicates, &atomic_counter, |fe| &fe.track_title, self.params.approximate_comparison);
        }
        if (self.params.music_similarity & MusicSimilarity::TRACK_ARTIST) == MusicSimilarity::TRACK_ARTIST {
            if check_if_stop_received(stop_receiver) {
                send_info_and_wait_for_ending_all_threads(&progress_thread_run, progress_thread_handle);
                return false;
            }

            old_duplicates = self.check_music_item(old_duplicates, &atomic_counter, |fe| &fe.track_artist, self.params.approximate_comparison);
        }
        if (self.params.music_similarity & MusicSimilarity::YEAR) == MusicSimilarity::YEAR {
            if check_if_stop_received(stop_receiver) {
                send_info_and_wait_for_ending_all_threads(&progress_thread_run, progress_thread_handle);
                return false;
            }

            old_duplicates = self.check_music_item(old_duplicates, &atomic_counter, |fe| &fe.year, false);
        }
        if (self.params.music_similarity & MusicSimilarity::LENGTH) == MusicSimilarity::LENGTH {
            if check_if_stop_received(stop_receiver) {
                send_info_and_wait_for_ending_all_threads(&progress_thread_run, progress_thread_handle);
                return false;
            }

            old_duplicates = self.check_music_item(old_duplicates, &atomic_counter, |fe| &fe.length, false);
        }
        if (self.params.music_similarity & MusicSimilarity::GENRE) == MusicSimilarity::GENRE {
            if check_if_stop_received(stop_receiver) {
                send_info_and_wait_for_ending_all_threads(&progress_thread_run, progress_thread_handle);
                return false;
            }

            old_duplicates = self.check_music_item(old_duplicates, &atomic_counter, |fe| &fe.genre, false);
        }
        if (self.params.music_similarity & MusicSimilarity::BITRATE) == MusicSimilarity::BITRATE {
            if check_if_stop_received(stop_receiver) {
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
                            hash_map.entry(thing.clone()).or_default().push(file_entry);
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

        if self.common_data.use_reference_folders {
            self.duplicated_music_entries_referenced = filter_reference_folders_generic(mem::take(&mut self.duplicated_music_entries), &self.common_data.directories);
        }

        if self.common_data.use_reference_folders {
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

    fn split_fingerprints_to_base_and_files_to_compare(&self, music_data: Vec<MusicEntry>) -> (Vec<MusicEntry>, Vec<MusicEntry>) {
        if self.common_data.use_reference_folders {
            music_data.into_iter().partition(|f| self.common_data.directories.is_in_referenced_directory(f.get_path()))
        } else {
            (music_data.clone(), music_data)
        }
    }

    fn get_entries_grouped_by_title(music_data: Vec<MusicEntry>) -> BTreeMap<String, Vec<MusicEntry>> {
        let mut entries_grouped_by_title: BTreeMap<String, Vec<MusicEntry>> = BTreeMap::new();
        for entry in music_data {
            let simplified_track_title = get_simplified_name(&entry.track_title);
            entries_grouped_by_title.entry(simplified_track_title).or_default().push(entry);
        }
        entries_grouped_by_title
    }

    fn split_fingerprints_to_check(&mut self) -> Vec<GroupedFilesToCheck> {
        if self.params.compare_fingerprints_only_with_similar_titles {
            let entries_grouped_by_title: BTreeMap<String, Vec<MusicEntry>> = Self::get_entries_grouped_by_title(mem::take(&mut self.music_entries));

            entries_grouped_by_title
                .into_iter()
                .filter_map(|(_title, entries)| {
                    let (base_files, files_to_compare) = self.split_fingerprints_to_base_and_files_to_compare(entries);

                    if base_files.is_empty() || files_to_compare.is_empty() {
                        return None;
                    }

                    Some(GroupedFilesToCheck { base_files, files_to_compare })
                })
                .collect()
        } else {
            let entries = mem::take(&mut self.music_entries);
            let (base_files, files_to_compare) = self.split_fingerprints_to_base_and_files_to_compare(entries);

            vec![GroupedFilesToCheck { base_files, files_to_compare }]
        }
    }

    #[fun_time(message = "compare_fingerprints", level = "debug")]
    fn compare_fingerprints(
        &mut self,
        stop_receiver: Option<&Receiver<()>>,
        atomic_counter: &Arc<AtomicUsize>,
        base_files: Vec<MusicEntry>,
        files_to_compare: &[MusicEntry],
    ) -> Option<Vec<Vec<MusicEntry>>> {
        let mut used_paths: HashSet<String> = Default::default();

        let configuration = &self.hash_preset_config;
        let minimum_segment_duration = self.params.minimum_segment_duration;
        let maximum_difference = self.params.maximum_difference;

        let mut duplicated_music_entries = Vec::new();

        for f_entry in base_files {
            atomic_counter.fetch_add(1, Ordering::Relaxed);
            if check_if_stop_received(stop_receiver) {
                return None;
            }

            let f_string = f_entry.path.to_string_lossy().to_string();
            if used_paths.contains(&f_string) {
                continue;
            }

            let temp_collected_similar_items = files_to_compare
                .par_iter()
                .map(|e_entry| {
                    let e_string = e_entry.path.to_string_lossy().to_string();
                    if used_paths.contains(&e_string) || e_string == f_string {
                        return Ok(None);
                    }
                    let mut segments = match match_fingerprints(&f_entry.fingerprint, &e_entry.fingerprint, configuration) {
                        Ok(segments) => segments,
                        Err(e) => return Err(format!("Error while comparing fingerprints: {e}")),
                    };
                    segments.retain(|s| s.duration(configuration) > minimum_segment_duration && s.score < maximum_difference);
                    if segments.is_empty() {
                        Ok(None)
                    } else {
                        Ok(Some((e_string, e_entry)))
                    }
                })
                .collect::<Vec<_>>();

            let mut collected_similar_items = Vec::with_capacity(temp_collected_similar_items.len());
            for result in temp_collected_similar_items {
                match result {
                    Ok(Some(data)) => {
                        collected_similar_items.push(data);
                    }
                    Ok(None) => (),
                    Err(e) => {
                        self.common_data.text_messages.errors.push(e);
                    }
                }
            }

            collected_similar_items.retain(|(path, _entry)| !used_paths.contains(path));
            if !collected_similar_items.is_empty() {
                let mut music_entries = Vec::new();
                for (path, entry) in collected_similar_items {
                    used_paths.insert(path);
                    music_entries.push(entry.clone());
                }
                used_paths.insert(f_string);
                music_entries.push(f_entry);
                duplicated_music_entries.push(music_entries);
            }
        }
        Some(duplicated_music_entries)
    }

    #[fun_time(message = "check_for_duplicate_fingerprints", level = "debug")]
    fn check_for_duplicate_fingerprints(&mut self, stop_receiver: Option<&Receiver<()>>, progress_sender: Option<&Sender<ProgressData>>) -> bool {
        let grouped_files_to_check = self.split_fingerprints_to_check();
        let base_files_number = grouped_files_to_check.iter().map(|g| g.base_files.len()).sum::<usize>();

        let (progress_thread_handle, progress_thread_run, atomic_counter, _check_was_stopped) =
            prepare_thread_handler_common(progress_sender, CurrentStage::SameMusicComparingFingerprints, base_files_number, self.get_test_type());

        let mut duplicated_music_entries = Vec::new();
        for group in grouped_files_to_check {
            let GroupedFilesToCheck { base_files, files_to_compare } = group;
            let Some(temp_music_entries) = self.compare_fingerprints(stop_receiver, &atomic_counter, base_files, &files_to_compare) else {
                send_info_and_wait_for_ending_all_threads(&progress_thread_run, progress_thread_handle);
                return false;
            };
            duplicated_music_entries.extend(temp_music_entries);
        }

        send_info_and_wait_for_ending_all_threads(&progress_thread_run, progress_thread_handle);

        self.duplicated_music_entries = duplicated_music_entries;

        if self.common_data.use_reference_folders {
            self.duplicated_music_entries_referenced = filter_reference_folders_generic(mem::take(&mut self.duplicated_music_entries), &self.common_data.directories);
        }

        if self.common_data.use_reference_folders {
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

    #[fun_time(message = "check_music_item", level = "debug")]
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
                    thing = get_simplified_name(&thing);
                }
                if !thing.is_empty() {
                    hash_map.entry(thing).or_default().push(file_entry);
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

    #[fun_time(message = "delete_files", level = "debug")]
    fn delete_files(&mut self) {
        if self.common_data.delete_method == DeleteMethod::None {
            return;
        }

        let vec_files = self.duplicated_music_entries.iter().collect::<Vec<_>>();
        delete_files_custom(&vec_files, &self.common_data.delete_method, &mut self.common_data.text_messages, self.common_data.dry_run);
    }
}

impl SameMusic {
    pub const fn get_duplicated_music_entries(&self) -> &Vec<Vec<MusicEntry>> {
        &self.duplicated_music_entries
    }

    pub fn get_params(&self) -> &SameMusicParameters {
        &self.params
    }

    pub const fn get_information(&self) -> &Info {
        &self.information
    }

    pub fn get_similar_music_referenced(&self) -> &Vec<(MusicEntry, Vec<MusicEntry>)> {
        &self.duplicated_music_entries_referenced
    }

    pub fn get_number_of_base_duplicated_files(&self) -> usize {
        if self.common_data.use_reference_folders {
            self.duplicated_music_entries_referenced.len()
        } else {
            self.duplicated_music_entries.len()
        }
    }

    pub fn get_use_reference(&self) -> bool {
        self.common_data.use_reference_folders
    }
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
        let Ok(packet) = format.next_packet() else {
            break;
        };

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
    let Ok(mut file) = File::open(path) else {
        return false;
    };

    let Ok(possible_tagged_file) = panic::catch_unwind(move || {
        match read_from(&mut file) {
            Ok(t) => Some(t),
            Err(_inspected) => {
                // println!("Failed to open {}", path);
                None
            }
        }
    }) else {
        let message = create_crash_message("Lofty", path, "https://github.com/image-rs/image/issues");
        println!("{message}");
        return false;
    };

    let Some(tagged_file) = possible_tagged_file else { return true };

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
    }

    if let Ok(old_length_number) = length.parse::<u32>() {
        let length_number = old_length_number / 60;
        let minutes = length_number / 1000;
        let seconds = (length_number % 1000) * 6 / 100;
        if minutes != 0 || seconds != 0 {
            length = format!("{minutes}:{seconds:02}");
        } else if old_length_number > 0 {
            // That means, that audio have length smaller that second but not zero
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

impl DebugPrint for SameMusic {
    #[fun_time(message = "debug_print", level = "debug")]
    fn debug_print(&self) {
        if !cfg!(debug_assertions) {
            return;
        }
        println!("---------------DEBUG PRINT---------------");
        println!("Found files music - {}", self.music_entries.len());
        println!("Found duplicated files music - {}", self.duplicated_music_entries.len());
        self.debug_print_common();
        println!("-----------------------------------------");
    }
}

impl PrintResults for SameMusic {
    fn write_results<T: Write>(&self, writer: &mut T) -> std::io::Result<()> {
        if !self.duplicated_music_entries.is_empty() {
            writeln!(writer, "{} music files which have similar friends\n\n.", self.duplicated_music_entries.len())?;

            for vec_file_entry in &self.duplicated_music_entries {
                writeln!(writer, "Found {} music files which have similar friends", vec_file_entry.len())?;
                for file_entry in vec_file_entry {
                    writeln!(
                        writer,
                        "TT: {}  -  TA: {}  -  Y: {}  -  L: {}  -  G: {}  -  B: {}  -  P: {:?}",
                        file_entry.track_title, file_entry.track_artist, file_entry.year, file_entry.length, file_entry.genre, file_entry.bitrate, file_entry.path
                    )?;
                }
                writeln!(writer)?;
            }
        } else if !self.duplicated_music_entries_referenced.is_empty() {
            writeln!(writer, "{} music files which have similar friends\n\n.", self.duplicated_music_entries_referenced.len())?;
            for (file_entry, vec_file_entry) in &self.duplicated_music_entries_referenced {
                writeln!(writer, "Found {} music files which have similar friends", vec_file_entry.len())?;
                writeln!(writer)?;
                writeln!(
                    writer,
                    "TT: {}  -  TA: {}  -  Y: {}  -  L: {}  -  G: {}  -  B: {}  -  P: {:?}",
                    file_entry.track_title, file_entry.track_artist, file_entry.year, file_entry.length, file_entry.genre, file_entry.bitrate, file_entry.path
                )?;
                for file_entry in vec_file_entry {
                    writeln!(
                        writer,
                        "TT: {}  -  TA: {}  -  Y: {}  -  L: {}  -  G: {}  -  B: {}  -  P: {:?}",
                        file_entry.track_title, file_entry.track_artist, file_entry.year, file_entry.length, file_entry.genre, file_entry.bitrate, file_entry.path
                    )?;
                }
                writeln!(writer)?;
            }
        } else {
            write!(writer, "Not found any similar music files.")?;
        }

        Ok(())
    }

    fn save_results_to_file_as_json(&self, file_name: &str, pretty_print: bool) -> std::io::Result<()> {
        if self.get_use_reference() {
            self.save_results_to_file_as_json_internal(file_name, &self.duplicated_music_entries_referenced, pretty_print)
        } else {
            self.save_results_to_file_as_json_internal(file_name, &self.duplicated_music_entries, pretty_print)
        }
    }
}

fn get_simplified_name(what: &str) -> String {
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
    new_what
}

impl CommonData for SameMusic {
    fn get_cd(&self) -> &CommonToolData {
        &self.common_data
    }
    fn get_cd_mut(&mut self) -> &mut CommonToolData {
        &mut self.common_data
    }
    fn get_check_method(&self) -> CheckingMethod {
        self.get_params().check_type
    }
}

#[cfg(test)]
mod tests {
    use crate::same_music::get_simplified_name;

    #[test]
    fn test_strings() {
        let what = "roman ( ziemniak ) ".to_string();
        let res = get_simplified_name(&what);
        assert_eq!(res, "roman");

        let what = "  HH)    ".to_string();
        let res = get_simplified_name(&what);
        assert_eq!(res, "HH");

        let what = "  fsf.f.  ".to_string();
        let res = get_simplified_name(&what);
        assert_eq!(res, "fsf f");

        let what = "Kekistan (feat. roman) [Mix on Mix]".to_string();
        let res = get_simplified_name(&what);
        assert_eq!(res, "Kekistan");
    }
}
