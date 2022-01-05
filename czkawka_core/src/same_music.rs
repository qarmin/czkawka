use std::collections::BTreeMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread::sleep;
use std::time::{Duration, SystemTime};
use std::{mem, thread};

use audiotags::Tag;
use crossbeam_channel::Receiver;
use rayon::prelude::*;

use crate::common::Common;
use crate::common_dir_traversal::{CheckingMethod, DirTraversalBuilder, DirTraversalResult, FileEntry, ProgressData};
use crate::common_directory::Directories;
use crate::common_extensions::Extensions;
use crate::common_items::ExcludedItems;
use crate::common_messages::Messages;
use crate::common_traits::*;
use crate::similar_images::AUDIO_FILES_EXTENSIONS;

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum DeleteMethod {
    None,
    Delete,
}

bitflags! {
    pub struct MusicSimilarity : u32 {
        const NONE = 0;

        const TITLE = 0b1;
        const ARTIST = 0b10;

        const ALBUM_TITLE = 0b100;
        const ALBUM_ARTIST = 0b1000;

        const YEAR = 0b10000;
        // const Time = 0b100000;
    }
}

#[derive(Clone, Debug)]
pub struct MusicEntry {
    pub size: u64,

    pub path: PathBuf,
    pub modified_date: u64,

    pub title: String,
    pub artist: String,

    pub album_title: String,
    pub album_artist: String,

    pub year: i32,
    // pub time: u32,
}

impl FileEntry {
    fn into_music_entry(self) -> MusicEntry {
        MusicEntry {
            size: self.size,
            path: self.path,
            modified_date: self.modified_date,

            title: "".to_string(),

            artist: "".to_string(),
            album_title: "".to_string(),
            album_artist: "".to_string(),
            year: 0,
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
    pub fn new() -> Self {
        Default::default()
    }
}

/// Struct with required information's to work
pub struct SameMusic {
    text_messages: Messages,
    information: Info,
    music_to_check: Vec<FileEntry>,
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
    use_reference_folders: bool,
}

impl SameMusic {
    pub fn new() -> Self {
        Self {
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
            music_to_check: Vec::with_capacity(2048),
            approximate_comparison: true,
            use_reference_folders: false,
            duplicated_music_entries_referenced: vec![],
        }
    }

    pub fn find_same_music(&mut self, stop_receiver: Option<&Receiver<()>>, progress_sender: Option<&futures::channel::mpsc::UnboundedSender<ProgressData>>) {
        self.directories.optimize_directories(self.recursive_search, &mut self.text_messages);
        self.use_reference_folders = !self.directories.reference_directories.is_empty();
        if !self.check_files(stop_receiver, progress_sender) {
            self.stopped_search = true;
            return;
        }
        if !self.check_records_multithreaded(stop_receiver, progress_sender) {
            self.stopped_search = true;
            return;
        }
        if !self.check_for_duplicates(stop_receiver, progress_sender) {
            self.stopped_search = true;
            return;
        }
        self.delete_files();
        self.debug_print();
    }

    pub fn get_stopped_search(&self) -> bool {
        self.stopped_search
    }

    pub const fn get_duplicated_music_entries(&self) -> &Vec<Vec<MusicEntry>> {
        &self.duplicated_music_entries
    }
    pub const fn get_music_similarity(&self) -> &MusicSimilarity {
        &self.music_similarity
    }

    pub const fn get_text_messages(&self) -> &Messages {
        &self.text_messages
    }

    pub const fn get_information(&self) -> &Info {
        &self.information
    }

    pub fn set_delete_method(&mut self, delete_method: DeleteMethod) {
        self.delete_method = delete_method;
    }

    pub fn set_approximate_comparison(&mut self, approximate_comparison: bool) {
        self.approximate_comparison = approximate_comparison;
    }

    pub fn set_recursive_search(&mut self, recursive_search: bool) {
        self.recursive_search = recursive_search;
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

    pub fn get_similar_music_referenced(&self) -> &Vec<(MusicEntry, Vec<MusicEntry>)> {
        &self.duplicated_music_entries_referenced
    }

    pub fn get_number_of_base_duplicated_files(&self) -> usize {
        if self.use_reference_folders {
            self.duplicated_music_entries_referenced.len()
        } else {
            self.duplicated_music_entries.len()
        }
    }

    pub fn get_use_reference(&self) -> bool {
        self.use_reference_folders
    }

    fn check_files(&mut self, stop_receiver: Option<&Receiver<()>>, progress_sender: Option<&futures::channel::mpsc::UnboundedSender<ProgressData>>) -> bool {
        if !self.allowed_extensions.using_custom_extensions() {
            self.allowed_extensions.extend_allowed_extensions(&AUDIO_FILES_EXTENSIONS);
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
            DirTraversalResult::SuccessFiles {
                start_time,
                grouped_file_entries,
                warnings,
            } => {
                if let Some(music_to_check) = grouped_file_entries.get(&()) {
                    self.music_to_check = music_to_check.clone();
                }
                self.text_messages.warnings.extend(warnings);
                Common::print_time(start_time, SystemTime::now(), "check_files".to_string());
                true
            }
            DirTraversalResult::SuccessFolders { .. } => {
                unreachable!()
            }
            DirTraversalResult::Stopped => false,
        }
    }

    fn check_records_multithreaded(&mut self, stop_receiver: Option<&Receiver<()>>, progress_sender: Option<&futures::channel::mpsc::UnboundedSender<ProgressData>>) -> bool {
        let start_time: SystemTime = SystemTime::now();

        let check_was_breaked = AtomicBool::new(false); // Used for breaking from GUI and ending check thread

        //// PROGRESS THREAD START
        const LOOP_DURATION: u32 = 200; //in ms
        let progress_thread_run = Arc::new(AtomicBool::new(true));

        let atomic_file_counter = Arc::new(AtomicUsize::new(0));

        let progress_thread_handle = if let Some(progress_sender) = progress_sender {
            let progress_send = progress_sender.clone();
            let progress_thread_run = progress_thread_run.clone();
            let atomic_file_counter = atomic_file_counter.clone();
            let music_to_check = self.music_to_check.len();
            thread::spawn(move || loop {
                progress_send
                    .unbounded_send(ProgressData {
                        checking_method: CheckingMethod::None,
                        current_stage: 1,
                        max_stage: 2,
                        entries_checked: atomic_file_counter.load(Ordering::Relaxed) as usize,
                        entries_to_check: music_to_check,
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

        // Clean for duplicate files
        let music_to_check = mem::take(&mut self.music_to_check);

        let vec_file_entry = music_to_check
            .into_par_iter()
            .map(|file_entry| {
                atomic_file_counter.fetch_add(1, Ordering::Relaxed);
                if stop_receiver.is_some() && stop_receiver.unwrap().try_recv().is_ok() {
                    check_was_breaked.store(true, Ordering::Relaxed);
                    return None;
                }
                let mut file_entry = file_entry.into_music_entry();

                let tag = match Tag::new().read_from_path(&file_entry.path) {
                    Ok(t) => t,
                    Err(_inspected) => return Some(None), // Data not in utf-8, etc., TODO this should be probably added to warnings, errors
                };

                file_entry.title = match tag.title() {
                    Some(t) => t.to_string(),
                    None => "".to_string(),
                };
                file_entry.artist = match tag.artist() {
                    Some(t) => t.to_string(),
                    None => "".to_string(),
                };
                file_entry.album_title = match tag.album_title() {
                    Some(t) => t.to_string(),
                    None => "".to_string(),
                };
                file_entry.album_artist = match tag.album_artist() {
                    Some(t) => t.to_string(),
                    None => "".to_string(),
                };
                file_entry.year = tag.year().unwrap_or(0);

                Some(Some(file_entry))
            })
            .while_some()
            .filter(|file_entry| file_entry.is_some())
            .map(|file_entry| file_entry.unwrap())
            .collect::<Vec<_>>();

        // End thread which send info to gui
        progress_thread_run.store(false, Ordering::Relaxed);
        progress_thread_handle.join().unwrap();

        // Check if user aborted search(only from GUI)
        if check_was_breaked.load(Ordering::Relaxed) {
            return false;
        }

        // Adding files to Vector
        self.music_entries = vec_file_entry;

        Common::print_time(start_time, SystemTime::now(), "check_records_multithreaded".to_string());

        true
    }
    fn check_for_duplicates(&mut self, stop_receiver: Option<&Receiver<()>>, progress_sender: Option<&futures::channel::mpsc::UnboundedSender<ProgressData>>) -> bool {
        if MusicSimilarity::NONE == self.music_similarity {
            panic!("This can't be none");
        }
        let start_time: SystemTime = SystemTime::now();

        //// PROGRESS THREAD START
        const LOOP_DURATION: u32 = 200; //in ms
        let progress_thread_run = Arc::new(AtomicBool::new(true));

        let atomic_file_counter = Arc::new(AtomicUsize::new(0));

        let progress_thread_handle = if let Some(progress_sender) = progress_sender {
            let progress_send = progress_sender.clone();
            let progress_thread_run = progress_thread_run.clone();
            let atomic_file_counter = atomic_file_counter.clone();
            let music_to_check = self.music_to_check.len();
            thread::spawn(move || loop {
                progress_send
                    .unbounded_send(ProgressData {
                        checking_method: CheckingMethod::None,
                        current_stage: 2,
                        max_stage: 2,
                        entries_checked: atomic_file_counter.load(Ordering::Relaxed) as usize,
                        entries_to_check: music_to_check,
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

        let mut old_duplicates: Vec<Vec<MusicEntry>> = vec![self.music_entries.clone()];
        let mut new_duplicates: Vec<Vec<MusicEntry>> = Vec::new();

        if (self.music_similarity & MusicSimilarity::TITLE) == MusicSimilarity::TITLE {
            for vec_file_entry in old_duplicates {
                atomic_file_counter.fetch_add(1, Ordering::Relaxed);
                if stop_receiver.is_some() && stop_receiver.unwrap().try_recv().is_ok() {
                    // End thread which send info to gui
                    progress_thread_run.store(false, Ordering::Relaxed);
                    progress_thread_handle.join().unwrap();
                    return false;
                }
                let mut hash_map: BTreeMap<String, Vec<MusicEntry>> = Default::default();
                for file_entry in vec_file_entry {
                    let mut title = file_entry.title.to_lowercase().trim().to_string();
                    if self.approximate_comparison {
                        get_approximate_conversion(&mut title);
                    }
                    if !title.is_empty() {
                        hash_map.entry(title.clone()).or_insert_with(Vec::new);
                        hash_map.get_mut(title.as_str()).unwrap().push(file_entry);
                    }
                }
                for (_title, vec_file_entry) in hash_map {
                    if vec_file_entry.len() > 1 {
                        new_duplicates.push(vec_file_entry);
                    }
                }
            }
            old_duplicates = new_duplicates;
            new_duplicates = Vec::new();
        }

        if (self.music_similarity & MusicSimilarity::ARTIST) == MusicSimilarity::ARTIST {
            for vec_file_entry in old_duplicates {
                atomic_file_counter.fetch_add(1, Ordering::Relaxed);
                if stop_receiver.is_some() && stop_receiver.unwrap().try_recv().is_ok() {
                    // End thread which send info to gui
                    progress_thread_run.store(false, Ordering::Relaxed);
                    progress_thread_handle.join().unwrap();
                    return false;
                }
                let mut hash_map: BTreeMap<String, Vec<MusicEntry>> = Default::default();
                for file_entry in vec_file_entry {
                    let mut artist = file_entry.artist.to_lowercase().trim().to_string();
                    if self.approximate_comparison {
                        get_approximate_conversion(&mut artist);
                    }
                    if !artist.is_empty() {
                        hash_map.entry(artist.clone()).or_insert_with(Vec::new);
                        hash_map.get_mut(artist.as_str()).unwrap().push(file_entry);
                    }
                }
                for (_artist, vec_file_entry) in hash_map {
                    if vec_file_entry.len() > 1 {
                        new_duplicates.push(vec_file_entry);
                    }
                }
            }
            old_duplicates = new_duplicates;
            new_duplicates = Vec::new();
        }

        if (self.music_similarity & MusicSimilarity::ALBUM_TITLE) == MusicSimilarity::ALBUM_TITLE {
            for vec_file_entry in old_duplicates {
                atomic_file_counter.fetch_add(1, Ordering::Relaxed);
                if stop_receiver.is_some() && stop_receiver.unwrap().try_recv().is_ok() {
                    // End thread which send info to gui
                    progress_thread_run.store(false, Ordering::Relaxed);
                    progress_thread_handle.join().unwrap();
                    return false;
                }
                let mut hash_map: BTreeMap<String, Vec<MusicEntry>> = Default::default();
                for file_entry in vec_file_entry {
                    let mut album_title = file_entry.album_title.to_lowercase().trim().to_string();
                    if self.approximate_comparison {
                        get_approximate_conversion(&mut album_title);
                    }
                    if !album_title.is_empty() {
                        hash_map.entry(album_title.clone()).or_insert_with(Vec::new);
                        hash_map.get_mut(album_title.as_str()).unwrap().push(file_entry);
                    }
                }
                for (_album_title, vec_file_entry) in hash_map {
                    if vec_file_entry.len() > 1 {
                        new_duplicates.push(vec_file_entry);
                    }
                }
            }
            old_duplicates = new_duplicates;
            new_duplicates = Vec::new();
        }

        if (self.music_similarity & MusicSimilarity::ALBUM_ARTIST) == MusicSimilarity::ALBUM_ARTIST {
            for vec_file_entry in old_duplicates {
                atomic_file_counter.fetch_add(1, Ordering::Relaxed);
                if stop_receiver.is_some() && stop_receiver.unwrap().try_recv().is_ok() {
                    // End thread which send info to gui
                    progress_thread_run.store(false, Ordering::Relaxed);
                    progress_thread_handle.join().unwrap();
                    return false;
                }
                let mut hash_map: BTreeMap<String, Vec<MusicEntry>> = Default::default();
                for file_entry in vec_file_entry {
                    let mut album_artist = file_entry.album_artist.to_lowercase().trim().to_string();
                    if self.approximate_comparison {
                        get_approximate_conversion(&mut album_artist);
                    }
                    if !album_artist.is_empty() {
                        hash_map.entry(album_artist.clone()).or_insert_with(Vec::new);
                        hash_map.get_mut(album_artist.as_str()).unwrap().push(file_entry);
                    }
                }
                for (_album_artist, vec_file_entry) in hash_map {
                    if vec_file_entry.len() > 1 {
                        new_duplicates.push(vec_file_entry);
                    }
                }
            }
            old_duplicates = new_duplicates;
            new_duplicates = Vec::new();
        }

        if (self.music_similarity & MusicSimilarity::YEAR) == MusicSimilarity::YEAR {
            for vec_file_entry in old_duplicates {
                atomic_file_counter.fetch_add(1, Ordering::Relaxed);
                if stop_receiver.is_some() && stop_receiver.unwrap().try_recv().is_ok() {
                    // End thread which send info to gui
                    progress_thread_run.store(false, Ordering::Relaxed);
                    progress_thread_handle.join().unwrap();
                    return false;
                }
                let mut hash_map: BTreeMap<i32, Vec<MusicEntry>> = Default::default();
                for file_entry in vec_file_entry {
                    let year = file_entry.year;
                    if year != 0 {
                        hash_map.entry(year).or_insert_with(Vec::new);
                        hash_map.get_mut(&year).unwrap().push(file_entry);
                    }
                }
                for (_year, vec_file_entry) in hash_map {
                    if vec_file_entry.len() > 1 {
                        new_duplicates.push(vec_file_entry);
                    }
                }
            }
            old_duplicates = new_duplicates;
            // new_duplicates = Vec::new();
        }

        // End thread which send info to gui
        progress_thread_run.store(false, Ordering::Relaxed);
        progress_thread_handle.join().unwrap();

        self.duplicated_music_entries = old_duplicates;

        if self.use_reference_folders {
            let mut similars_vector = Default::default();
            mem::swap(&mut self.duplicated_music_entries, &mut similars_vector);
            let reference_directories = self.directories.reference_directories.clone();
            self.duplicated_music_entries_referenced = similars_vector
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
                .collect::<Vec<(MusicEntry, Vec<MusicEntry>)>>();
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

        Common::print_time(start_time, SystemTime::now(), "check_for_duplicates".to_string());

        // Clear unused data
        self.music_entries.clear();

        true
    }

    pub fn set_minimal_file_size(&mut self, minimal_file_size: u64) {
        self.minimal_file_size = match minimal_file_size {
            0 => 1,
            t => t,
        };
    }

    /// Function to delete files, from filed Vector
    fn delete_files(&mut self) {
        let start_time: SystemTime = SystemTime::now();
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

        Common::print_time(start_time, SystemTime::now(), "delete_files".to_string());
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
        println!("Delete Method - {:?}", self.delete_method);
        println!("-----------------------------------------");
    }
}

impl SaveResults for SameMusic {
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

        if !self.music_entries.is_empty() {
            writeln!(writer, "Found {} same music files.", self.information.number_of_duplicates).unwrap();
            for file_entry in self.music_entries.iter() {
                writeln!(writer, "{}", file_entry.path.display()).unwrap();
            }
        } else {
            write!(writer, "Not found any empty files.").unwrap();
        }
        Common::print_time(start_time, SystemTime::now(), "save_results_to_file".to_string());
        true
    }
}

impl PrintResults for SameMusic {
    /// Print information's about duplicated entries
    /// Only needed for CLI
    fn print_results(&self) {
        let start_time: SystemTime = SystemTime::now();
        println!("Found {} similar music files.\n", self.duplicated_music_entries.len());
        for vec_file_entry in self.duplicated_music_entries.iter() {
            for file_entry in vec_file_entry {
                println!(
                    "T: {}  -  A: {}  -  Y: {}  -  AT: {}  -  AA: {}  -  P: {}",
                    file_entry.title,
                    file_entry.artist,
                    file_entry.year,
                    file_entry.album_title,
                    file_entry.album_artist,
                    file_entry.path.display()
                );
            }
            println!();
        }

        Common::print_time(start_time, SystemTime::now(), "print_entries".to_string());
    }
}

fn get_approximate_conversion(what: &mut String) {
    let mut new_what = String::with_capacity(what.len());
    let mut tab_number = 0;
    let mut space_before = true;
    for character in what.chars() {
        match character {
            '(' => {
                tab_number += 1;
            }
            ')' => {
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
    }
}
