use std::collections::BTreeMap;
use std::fs::File;
use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::{mem, panic};

use anyhow::Context;
use crossbeam_channel::Sender;
use fun_time::fun_time;
use indexmap::IndexSet;
use lofty::file::{AudioFile, TaggedFileExt};
use lofty::prelude::*;
use lofty::read_from;
use log::{debug, error};
use rayon::prelude::*;
use rusty_chromaprint::{Configuration, Fingerprinter, match_fingerprints};
use symphonia::core::audio::SampleBuffer;
use symphonia::core::codecs::{CODEC_TYPE_NULL, DecoderOptions};
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;

use crate::common::cache::{CACHE_VERSION, load_and_split_cache_generalized_by_path, save_and_connect_cache_generalized_by_path};
use crate::common::create_crash_message;
use crate::common::dir_traversal::{DirTraversalBuilder, DirTraversalResult};
use crate::common::model::{ToolType, WorkContinueStatus};
use crate::common::progress_data::{CurrentStage, ProgressData};
use crate::common::progress_stop_handler::{check_if_stop_received, prepare_thread_handler_common};
use crate::common::tool_data::{CommonData, CommonToolData};
use crate::common::traits::ResultEntry;
use crate::tools::same_music::{GroupedFilesToCheck, Info, MusicEntry, MusicSimilarity, SameMusic, SameMusicParameters};

impl SameMusic {
    pub fn new(params: SameMusicParameters) -> Self {
        Self {
            common_data: CommonToolData::new(ToolType::SameMusic),
            information: Info::default(),
            music_entries: Vec::with_capacity(2048),
            duplicated_music_entries: Vec::new(),
            music_to_check: Default::default(),
            duplicated_music_entries_referenced: Vec::new(),
            hash_preset_config: Configuration::preset_test1(), // TODO allow to change this and move to parameters
            params,
        }
    }

    #[fun_time(message = "check_files", level = "debug")]
    pub(crate) fn check_files(&mut self, stop_flag: &Arc<AtomicBool>, progress_sender: Option<&Sender<ProgressData>>) -> WorkContinueStatus {
        let result = DirTraversalBuilder::new()
            .group_by(|_fe| ())
            .stop_flag(stop_flag)
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
                WorkContinueStatus::Continue
            }

            DirTraversalResult::Stopped => WorkContinueStatus::Stop,
        }
    }

    #[fun_time(message = "load_cache", level = "debug")]
    fn load_cache(&mut self, checking_tags: bool) -> (BTreeMap<String, MusicEntry>, BTreeMap<String, MusicEntry>, BTreeMap<String, MusicEntry>) {
        load_and_split_cache_generalized_by_path(&get_similar_music_cache_file(checking_tags), mem::take(&mut self.music_to_check), self)
    }

    #[fun_time(message = "save_cache", level = "debug")]
    fn save_cache(&mut self, vec_file_entry: &[MusicEntry], loaded_hash_map: BTreeMap<String, MusicEntry>, checking_tags: bool) {
        save_and_connect_cache_generalized_by_path(&get_similar_music_cache_file(checking_tags), vec_file_entry, loaded_hash_map, self);
    }

    #[fun_time(message = "calculate_fingerprint", level = "debug")]
    pub(crate) fn calculate_fingerprint(&mut self, stop_flag: &Arc<AtomicBool>, progress_sender: Option<&Sender<ProgressData>>) -> WorkContinueStatus {
        if self.music_entries.is_empty() {
            return WorkContinueStatus::Continue;
        }

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

        let progress_handler = prepare_thread_handler_common(progress_sender, CurrentStage::SameMusicCacheLoadingFingerprints, 0, self.get_test_type(), 0);

        let (loaded_hash_map, records_already_cached, non_cached_files_to_check) = self.load_cache(false);

        progress_handler.join_thread();
        if check_if_stop_received(stop_flag) {
            return WorkContinueStatus::Stop;
        }

        let progress_handler = prepare_thread_handler_common(
            progress_sender,
            CurrentStage::SameMusicCalculatingFingerprints,
            non_cached_files_to_check.len(),
            self.get_test_type(),
            non_cached_files_to_check.values().map(|e| e.size).sum::<u64>(),
        );
        let configuration = &self.hash_preset_config;

        let non_cached_files_to_check = non_cached_files_to_check.into_iter().collect::<Vec<_>>();

        debug!("calculate_fingerprint - starting fingerprinting");
        let mut vec_file_entry = non_cached_files_to_check
            .into_par_iter()
            .with_max_len(2)
            .map(|(path, mut music_entry)| {
                if check_if_stop_received(stop_flag) {
                    return None;
                }

                let res = calc_fingerprint_helper(path, configuration);
                progress_handler.increase_size(music_entry.size);
                progress_handler.increase_items(1);

                let Ok(fingerprint) = res else {
                    return Some(None);
                };

                music_entry.fingerprint = fingerprint;

                Some(Some(music_entry))
            })
            .while_some()
            .flatten()
            .collect::<Vec<_>>();
        debug!("calculate_fingerprint - ended fingerprinting");

        progress_handler.join_thread();

        let progress_handler = prepare_thread_handler_common(progress_sender, CurrentStage::SameMusicCacheSavingFingerprints, 0, self.get_test_type(), 0);

        vec_file_entry.extend(records_already_cached.into_values());

        self.save_cache(&vec_file_entry, loaded_hash_map, false);

        self.music_entries = vec_file_entry;

        progress_handler.join_thread();
        if check_if_stop_received(stop_flag) {
            return WorkContinueStatus::Stop;
        }
        WorkContinueStatus::Continue
    }

    #[fun_time(message = "read_tags", level = "debug")]
    pub(crate) fn read_tags(&mut self, stop_flag: &Arc<AtomicBool>, progress_sender: Option<&Sender<ProgressData>>) -> WorkContinueStatus {
        if self.music_to_check.is_empty() {
            return WorkContinueStatus::Continue;
        }

        let progress_handler = prepare_thread_handler_common(progress_sender, CurrentStage::SameMusicCacheLoadingTags, 0, self.get_test_type(), 0);

        let (loaded_hash_map, records_already_cached, non_cached_files_to_check) = self.load_cache(true);

        progress_handler.join_thread();
        if check_if_stop_received(stop_flag) {
            return WorkContinueStatus::Stop;
        }

        let progress_handler = prepare_thread_handler_common(
            progress_sender,
            CurrentStage::SameMusicReadingTags,
            non_cached_files_to_check.len(),
            self.get_test_type(),
            0,
        );

        debug!("read_tags - starting reading tags");
        // Clean for duplicate files
        let mut vec_file_entry = non_cached_files_to_check
            .into_par_iter()
            .map(|(path, music_entry)| {
                if check_if_stop_received(stop_flag) {
                    return None;
                }

                let res = read_single_file_tags(&path, music_entry);
                progress_handler.increase_items(1);
                Some(res)
            })
            .while_some()
            .flatten()
            .collect::<Vec<_>>();
        debug!("read_tags - ended reading tags");

        progress_handler.join_thread();
        let progress_handler = prepare_thread_handler_common(progress_sender, CurrentStage::SameMusicCacheSavingTags, 0, self.get_test_type(), 0);

        vec_file_entry.extend(records_already_cached.into_values());

        self.save_cache(&vec_file_entry, loaded_hash_map, true);

        self.music_entries = vec_file_entry;

        progress_handler.join_thread();
        if check_if_stop_received(stop_flag) {
            return WorkContinueStatus::Stop;
        }

        WorkContinueStatus::Continue
    }

    #[fun_time(message = "check_for_duplicate_tags", level = "debug")]
    pub(crate) fn check_for_duplicate_tags(&mut self, stop_flag: &Arc<AtomicBool>, progress_sender: Option<&Sender<ProgressData>>) -> WorkContinueStatus {
        if self.music_entries.is_empty() {
            return WorkContinueStatus::Continue;
        }
        let progress_handler = prepare_thread_handler_common(progress_sender, CurrentStage::SameMusicComparingTags, self.music_entries.len(), self.get_test_type(), 0);

        let mut old_duplicates: Vec<Vec<MusicEntry>> = vec![self.music_entries.clone()];
        let mut new_duplicates: Vec<Vec<MusicEntry>> = Vec::new();

        if (self.params.music_similarity & MusicSimilarity::TRACK_TITLE) == MusicSimilarity::TRACK_TITLE {
            if check_if_stop_received(stop_flag) {
                progress_handler.join_thread();
                return WorkContinueStatus::Stop;
            }

            old_duplicates = self.check_music_item(
                old_duplicates,
                progress_handler.items_counter(),
                |fe| fe.track_title.clone(),
                self.params.approximate_comparison,
            );
        }
        if (self.params.music_similarity & MusicSimilarity::TRACK_ARTIST) == MusicSimilarity::TRACK_ARTIST {
            if check_if_stop_received(stop_flag) {
                progress_handler.join_thread();
                return WorkContinueStatus::Stop;
            }

            old_duplicates = self.check_music_item(
                old_duplicates,
                progress_handler.items_counter(),
                |fe| fe.track_artist.clone(),
                self.params.approximate_comparison,
            );
        }
        if (self.params.music_similarity & MusicSimilarity::YEAR) == MusicSimilarity::YEAR {
            if check_if_stop_received(stop_flag) {
                progress_handler.join_thread();
                return WorkContinueStatus::Stop;
            }

            old_duplicates = self.check_music_item(old_duplicates, progress_handler.items_counter(), |fe| fe.year.clone(), false);
        }
        if (self.params.music_similarity & MusicSimilarity::LENGTH) == MusicSimilarity::LENGTH {
            if check_if_stop_received(stop_flag) {
                progress_handler.join_thread();
                return WorkContinueStatus::Stop;
            }

            old_duplicates = self.check_music_item(old_duplicates, progress_handler.items_counter(), |fe| format_audio_duration(fe.length), false);
        }
        if (self.params.music_similarity & MusicSimilarity::GENRE) == MusicSimilarity::GENRE {
            if check_if_stop_received(stop_flag) {
                progress_handler.join_thread();
                return WorkContinueStatus::Stop;
            }

            old_duplicates = self.check_music_item(old_duplicates, progress_handler.items_counter(), |fe| fe.genre.clone(), false);
        }
        if (self.params.music_similarity & MusicSimilarity::BITRATE) == MusicSimilarity::BITRATE {
            if check_if_stop_received(stop_flag) {
                progress_handler.join_thread();
                return WorkContinueStatus::Stop;
            }
            let old_duplicates_len = old_duplicates.len();
            for vec_file_entry in old_duplicates {
                let mut hash_map: BTreeMap<String, Vec<MusicEntry>> = Default::default();
                for file_entry in vec_file_entry {
                    if file_entry.bitrate != 0 {
                        let thing = file_entry.bitrate.to_string();

                        hash_map.entry(thing).or_default().push(file_entry);
                    }
                }
                for (_title, vec_file_entry) in hash_map {
                    if vec_file_entry.len() > 1 {
                        new_duplicates.push(vec_file_entry);
                    }
                }
            }
            progress_handler.increase_items(old_duplicates_len);
            old_duplicates = new_duplicates;
        }

        progress_handler.join_thread();

        self.duplicated_music_entries = old_duplicates;

        if self.common_data.use_reference_folders {
            self.duplicated_music_entries_referenced = self.common_data.directories.filter_reference_folders(mem::take(&mut self.duplicated_music_entries));
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

        WorkContinueStatus::Continue
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
            // TODO maybe add as option to check for empty titles?
            if simplified_track_title.is_empty() {
                continue;
            }
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

                    // When there is 0 files in base files or files to compare there will be no comparison, so removing it from the list
                    // Also when there is only one file in base files and files to compare and they are the same file, there will be no comparison

                    #[expect(clippy::indexing_slicing)] // Validated that base_files/files_to_compare are not empty
                    if base_files.is_empty()
                        || files_to_compare.is_empty()
                        || (base_files.len() == 1 && files_to_compare.len() == 1 && (base_files[0].path == files_to_compare[0].path))
                    {
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

    fn compare_fingerprints(
        &mut self,
        stop_flag: &Arc<AtomicBool>,
        items_counter: &Arc<AtomicUsize>,
        base_files: Vec<MusicEntry>,
        files_to_compare: &[MusicEntry],
    ) -> Option<Vec<Vec<MusicEntry>>> {
        let mut used_paths: IndexSet<String> = Default::default();

        let configuration = &self.hash_preset_config;
        let minimum_segment_duration = self.params.minimum_segment_duration;
        let maximum_difference = self.params.maximum_difference;

        let mut duplicated_music_entries = Vec::new();

        for f_entry in base_files {
            items_counter.fetch_add(1, Ordering::Relaxed);
            if check_if_stop_received(stop_flag) {
                return None;
            }

            let f_string = f_entry.path.to_string_lossy().to_string();
            if used_paths.contains(&f_string) {
                continue;
            }

            let (mut collected_similar_items, errors): (Vec<_>, Vec<_>) = files_to_compare
                .par_iter()
                .map(|e_entry| {
                    let e_string = e_entry.path.to_string_lossy().to_string();
                    if used_paths.contains(&e_string) || e_string == f_string {
                        return None;
                    }
                    let mut segments = match match_fingerprints(&f_entry.fingerprint, &e_entry.fingerprint, configuration) {
                        Ok(segments) => segments,
                        Err(e) => return Some(Err(format!("Error while comparing fingerprints: {e}"))),
                    };
                    segments.retain(|s| s.duration(configuration) > minimum_segment_duration && s.score < maximum_difference);
                    if segments.is_empty() { None } else { Some(Ok((e_string, e_entry))) }
                })
                .flatten()
                .partition_map(|res| match res {
                    Ok(entry) => itertools::Either::Left(entry),
                    Err(err) => itertools::Either::Right(err),
                });

            self.common_data.text_messages.errors.extend(errors);

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
    pub(crate) fn check_for_duplicate_fingerprints(&mut self, stop_flag: &Arc<AtomicBool>, progress_sender: Option<&Sender<ProgressData>>) -> WorkContinueStatus {
        if self.music_entries.is_empty() {
            return WorkContinueStatus::Continue;
        }

        let grouped_files_to_check = self.split_fingerprints_to_check();
        let base_files_number = grouped_files_to_check.iter().map(|g| g.base_files.len()).sum::<usize>();

        let progress_handler = prepare_thread_handler_common(progress_sender, CurrentStage::SameMusicComparingFingerprints, base_files_number, self.get_test_type(), 0);

        let mut duplicated_music_entries = Vec::new();
        for group in grouped_files_to_check {
            let GroupedFilesToCheck { base_files, files_to_compare } = group;
            let Some(temp_music_entries) = self.compare_fingerprints(stop_flag, progress_handler.items_counter(), base_files, &files_to_compare) else {
                progress_handler.join_thread();
                return WorkContinueStatus::Stop;
            };
            duplicated_music_entries.extend(temp_music_entries);
        }

        progress_handler.join_thread();

        self.duplicated_music_entries = duplicated_music_entries;

        if self.common_data.use_reference_folders {
            self.duplicated_music_entries_referenced = self.common_data.directories.filter_reference_folders(mem::take(&mut self.duplicated_music_entries));
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

        WorkContinueStatus::Continue
    }

    #[fun_time(message = "check_music_item", level = "debug")]
    fn check_music_item(
        &self,
        old_duplicates: Vec<Vec<MusicEntry>>,
        items_counter: &Arc<AtomicUsize>,
        get_item: fn(&MusicEntry) -> String,
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
        items_counter.fetch_add(old_duplicates_len, Ordering::Relaxed);

        new_duplicates
    }
}

// TODO this should be taken from rusty-chromaprint repo, not reimplemented here
fn calc_fingerprint_helper<P: AsRef<Path>>(path: P, config: &Configuration) -> anyhow::Result<Vec<u32>> {
    let path = path.as_ref().to_path_buf();
    panic::catch_unwind(|| {
        let path = &path;

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
    })
    .unwrap_or_else(|_| {
        let message = create_crash_message("Symphonia", &path.to_string_lossy(), "https://github.com/pdeljanov/Symphonia");
        error!("{message}");
        Err(anyhow::anyhow!("{message}"))
    })
}

fn read_single_file_tags(path: &str, mut music_entry: MusicEntry) -> Option<MusicEntry> {
    let Ok(mut file) = File::open(path) else {
        return None;
    };

    let Ok(possible_tagged_file) = panic::catch_unwind(move || read_from(&mut file).ok()) else {
        let message = create_crash_message("Lofty", path, "https://github.com/Serial-ATA/lofty-rs");
        error!("{message}");
        return None;
    };

    let Some(tagged_file) = possible_tagged_file else { return Some(music_entry) };

    let properties = tagged_file.properties();

    let mut track_title = String::new();
    let mut track_artist = String::new();
    let mut year = String::new();
    let mut genre = String::new();

    let bitrate = properties.audio_bitrate().unwrap_or(0);

    if let Some(tag) = tagged_file.primary_tag() {
        track_title = tag.get_string(&ItemKey::TrackTitle).unwrap_or_default().to_string();
        track_artist = tag.get_string(&ItemKey::TrackArtist).unwrap_or_default().to_string();
        year = tag.get_string(&ItemKey::Year).unwrap_or_default().to_string();
        genre = tag.get_string(&ItemKey::Genre).unwrap_or_default().to_string();
    }

    for tag in tagged_file.tags() {
        if track_title.is_empty()
            && let Some(tag_value) = tag.get_string(&ItemKey::TrackTitle)
        {
            track_title = tag_value.to_string();
        }
        if track_artist.is_empty()
            && let Some(tag_value) = tag.get_string(&ItemKey::TrackArtist)
        {
            track_artist = tag_value.to_string();
        }
        if year.is_empty()
            && let Some(tag_value) = tag.get_string(&ItemKey::Year)
        {
            year = tag_value.to_string();
        }
        if genre.is_empty()
            && let Some(tag_value) = tag.get_string(&ItemKey::Genre)
        {
            genre = tag_value.to_string();
        }
    }

    let length_milliseconds = properties.duration().as_millis();
    let length_in_seconds = if length_milliseconds == 0 {
        0
    } else {
        let secs = properties.duration().as_secs() as u32;
        if secs == 0 { 1 } else { secs }
    };

    music_entry.track_title = track_title;
    music_entry.track_artist = track_artist;
    music_entry.year = year;
    music_entry.length = length_in_seconds;
    music_entry.genre = genre;
    music_entry.bitrate = bitrate;

    Some(music_entry)
}

pub fn format_audio_duration(duration: u32) -> String {
    let hours = duration / 3600;
    let minutes = (duration % 3600) / 60;
    let seconds = duration % 60;
    if hours > 0 {
        format!("{hours}:{minutes:02}:{seconds:02}")
    } else {
        format!("{minutes}:{seconds:02}")
    }
}

fn get_simplified_name_internal(what: &str, ignore_numbers: bool) -> String {
    let mut new_what = String::with_capacity(what.len());
    let mut tab_number = 0;
    let mut space_before = true;
    for character in what.chars().map(|e| if e.is_whitespace() { ' ' } else { e }) {
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
                    if ch.is_ascii_alphabetic() || (!ignore_numbers && ch.is_numeric()) {
                        space_before = false;
                        new_what.push(ch);
                    } else {
                        let new_items = deunicode::deunicode_char(character).map_or_else(|| vec![character; 1], |e| e.trim().to_string().chars().collect::<Vec<_>>());

                        // If is equal, then we're trying to deunicode e.g. dot, comma etc.
                        // We just ignore char, because it is mostly useless, but we add space instead it if it wasn't added already
                        if new_items.first() == Some(&character) {
                            if !space_before {
                                new_what.push(' ');
                                space_before = true;
                            }
                        } else {
                            new_what.extend(new_items.into_iter());
                            space_before = false;
                        }
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
fn get_simplified_name(what: &str) -> String {
    let new_what = get_simplified_name_internal(what, true);
    if !new_what.is_empty() {
        return new_what;
    }
    let new_what = get_simplified_name_internal(what, false);
    if !new_what.is_empty() {
        return new_what;
    }
    let simplified_unicode = deunicode::deunicode(what).trim().to_string();
    if !simplified_unicode.is_empty() {
        return simplified_unicode;
    }
    // If everything failed, we return original string
    // this is more useful than returning empty string, which is ignored by other functions
    what.trim().to_string()
}

pub fn get_similar_music_cache_file(checking_tags: bool) -> String {
    if checking_tags {
        format!("cache_same_music_tags_{CACHE_VERSION}.bin")
    } else {
        format!("cache_same_music_fingerprints_{CACHE_VERSION}.bin")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_simplified_names() {
        let cases = [
            ("roman ( ziemniak ) ", "roman"),
            ("  HH)    ", "HH"),
            ("  fsf.f.  ", "fsf f"),
            ("  śśśśćććć  ", "sssscccc"),
            ("rr\t", "rr"),
            ("Kekistan (feat. roman) [Mix on Mix]", "Kekistan"),
            ("23", "23"),
            ("23 (random)", "23"),
            ("(23)", "(23)"),
        ];

        for (input, expected) in cases {
            let res = get_simplified_name(input);
            assert_eq!(res, expected, "Input: {input}, Expected: {expected}, Got: {res}");
        }
    }
}
