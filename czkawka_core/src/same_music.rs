use std::fs::{File, Metadata};
use std::io::prelude::*;
use std::path::PathBuf;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::{fs, thread};

use crate::common::Common;
use crate::common_directory::Directories;
use crate::common_items::ExcludedItems;
use crate::common_messages::Messages;
use crate::common_traits::*;
use audiotags::Tag;
use crossbeam_channel::Receiver;
use rayon::prelude::*;
use std::collections::BTreeMap;
use std::io::BufWriter;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread::sleep;

#[derive(Debug)]
pub struct ProgressData {
    pub current_stage: u8,
    pub max_stage: u8,
    pub music_checked: usize,
    pub music_to_check: usize,
}

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
pub struct FileEntry {
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

/// Info struck with helpful information's about results
#[derive(Default)]
pub struct Info {
    pub number_of_music_entries: usize,
    pub number_of_removed_files: usize,
    pub number_of_failed_to_remove_files: usize,
    pub number_of_duplicates_music_files: usize,
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
    music_entries: Vec<FileEntry>,
    duplicated_music_entries: Vec<Vec<FileEntry>>,
    directories: Directories,
    excluded_items: ExcludedItems,
    minimal_file_size: u64,
    maximal_file_size: u64,
    recursive_search: bool,
    delete_method: DeleteMethod,
    music_similarity: MusicSimilarity,
    stopped_search: bool,
}

impl SameMusic {
    pub fn new() -> Self {
        Self {
            text_messages: Messages::new(),
            information: Info::new(),
            recursive_search: true,
            directories: Directories::new(),
            excluded_items: ExcludedItems::new(),
            music_entries: Vec::with_capacity(2048),
            delete_method: DeleteMethod::None,
            music_similarity: MusicSimilarity::NONE,
            stopped_search: false,
            minimal_file_size: 8192,
            maximal_file_size: u64::MAX,
            duplicated_music_entries: vec![],
            music_to_check: Vec::with_capacity(2048),
        }
    }

    pub fn find_same_music(&mut self, stop_receiver: Option<&Receiver<()>>, progress_sender: Option<&futures::channel::mpsc::UnboundedSender<ProgressData>>) {
        self.directories.optimize_directories(self.recursive_search, &mut self.text_messages);
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

    pub const fn get_duplicated_music_entries(&self) -> &Vec<Vec<FileEntry>> {
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

    pub fn set_recursive_search(&mut self, recursive_search: bool) {
        self.recursive_search = recursive_search;
    }

    pub fn set_included_directory(&mut self, included_directory: Vec<PathBuf>) -> bool {
        self.directories.set_included_directory(included_directory, &mut self.text_messages)
    }

    pub fn set_excluded_directory(&mut self, excluded_directory: Vec<PathBuf>) {
        self.directories.set_excluded_directory(excluded_directory, &mut self.text_messages);
    }

    pub fn set_excluded_items(&mut self, excluded_items: Vec<String>) {
        self.excluded_items.set_excluded_items(excluded_items, &mut self.text_messages);
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

    /// Check files for any with size == 0
    fn check_files(&mut self, stop_receiver: Option<&Receiver<()>>, progress_sender: Option<&futures::channel::mpsc::UnboundedSender<ProgressData>>) -> bool {
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
                        max_stage: 2,
                        music_checked: atomic_file_counter.load(Ordering::Relaxed) as usize,
                        music_to_check: 0,
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
                    if self.directories.is_excluded(&next_folder) || self.excluded_items.is_excluded(&next_folder) {
                        continue 'dir;
                    }

                    folders_to_check.push(next_folder);
                } else if metadata.is_file() {
                    atomic_file_counter.fetch_add(1, Ordering::Relaxed);
                    // Checking files
                    if (self.minimal_file_size..=self.maximal_file_size).contains(&metadata.len()) {
                        let current_file_name = current_folder.join(entry_data.file_name());
                        if self.excluded_items.is_excluded(&current_file_name) {
                            continue 'dir;
                        }

                        let allowed_extensions = [".mp3", ".flac", ".m4a"];

                        if !allowed_extensions.iter().any(|r| current_file_name.to_string_lossy().ends_with(r)) {
                            continue 'dir;
                        }

                        // Creating new file entry
                        let file_entry: FileEntry = FileEntry {
                            size: metadata.len(),
                            path: current_file_name.clone(),
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
                                    continue 'dir;
                                } // Permissions Denied
                            },
                            title: "".to_string(),

                            artist: "".to_string(),
                            album_title: "".to_string(),
                            album_artist: "".to_string(),
                            year: 0,
                        };

                        // Adding files to Vector
                        self.music_to_check.push(file_entry);
                    }
                }
            }
        }
        // End thread which send info to gui
        progress_thread_run.store(false, Ordering::Relaxed);
        progress_thread_handle.join().unwrap();
        self.information.number_of_music_entries = self.music_entries.len();

        Common::print_time(start_time, SystemTime::now(), "check_files".to_string());
        true
    }

    fn check_records_multithreaded(&mut self, stop_receiver: Option<&Receiver<()>>, progress_sender: Option<&futures::channel::mpsc::UnboundedSender<ProgressData>>) -> bool {
        let start_time: SystemTime = SystemTime::now();

        let check_was_breaked = AtomicBool::new(false); // Used for breaking from GUI and ending check thread

        //// PROGRESS THREAD START
        const LOOP_DURATION: u32 = 200; //in ms
        let progress_thread_run = Arc::new(AtomicBool::new(true));

        let atomic_file_counter = Arc::new(AtomicUsize::new(0));

        let progress_thread_handle;
        if let Some(progress_sender) = progress_sender {
            let progress_send = progress_sender.clone();
            let progress_thread_run = progress_thread_run.clone();
            let atomic_file_counter = atomic_file_counter.clone();
            let music_to_check = self.music_to_check.len();
            progress_thread_handle = thread::spawn(move || loop {
                progress_send
                    .unbounded_send(ProgressData {
                        current_stage: 1,
                        max_stage: 2,
                        music_checked: atomic_file_counter.load(Ordering::Relaxed) as usize,
                        music_to_check,
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

        let vec_file_entry = self
            .music_to_check
            .par_iter()
            .map(|file_entry| {
                atomic_file_counter.fetch_add(1, Ordering::Relaxed);
                if stop_receiver.is_some() && stop_receiver.unwrap().try_recv().is_ok() {
                    check_was_breaked.store(true, Ordering::Relaxed);
                    return None;
                }
                let mut file_entry = file_entry.clone();

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

        // Clean for duplicate files
        self.music_to_check.clear();

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

        let progress_thread_handle;
        if let Some(progress_sender) = progress_sender {
            let progress_send = progress_sender.clone();
            let progress_thread_run = progress_thread_run.clone();
            let atomic_file_counter = atomic_file_counter.clone();
            let music_to_check = self.music_to_check.len();
            progress_thread_handle = thread::spawn(move || loop {
                progress_send
                    .unbounded_send(ProgressData {
                        current_stage: 2,
                        max_stage: 2,
                        music_checked: atomic_file_counter.load(Ordering::Relaxed) as usize,
                        music_to_check,
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

        let mut old_duplicates: Vec<Vec<FileEntry>> = vec![self.music_entries.clone()];
        let mut new_duplicates: Vec<Vec<FileEntry>> = Vec::new();

        if (self.music_similarity & MusicSimilarity::TITLE) == MusicSimilarity::TITLE {
            for vec_file_entry in old_duplicates {
                atomic_file_counter.fetch_add(1, Ordering::Relaxed);
                if stop_receiver.is_some() && stop_receiver.unwrap().try_recv().is_ok() {
                    // End thread which send info to gui
                    progress_thread_run.store(false, Ordering::Relaxed);
                    progress_thread_handle.join().unwrap();
                    return false;
                }
                let mut hash_map: BTreeMap<String, Vec<FileEntry>> = Default::default();
                for file_entry in vec_file_entry {
                    let title = file_entry.title.to_lowercase().trim().to_string();
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
                let mut hash_map: BTreeMap<String, Vec<FileEntry>> = Default::default();
                for file_entry in vec_file_entry {
                    let artist = file_entry.artist.to_lowercase().trim().to_string();
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
                let mut hash_map: BTreeMap<String, Vec<FileEntry>> = Default::default();
                for file_entry in vec_file_entry {
                    let album_title = file_entry.album_title.to_lowercase().trim().to_string();
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
                let mut hash_map: BTreeMap<String, Vec<FileEntry>> = Default::default();
                for file_entry in vec_file_entry {
                    let album_artist = file_entry.album_artist.to_lowercase().trim().to_string();
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
                let mut hash_map: BTreeMap<i32, Vec<FileEntry>> = Default::default();
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

        self.duplicated_music_entries = old_duplicates;

        for vec in &self.duplicated_music_entries {
            self.information.number_of_duplicates_music_files += vec.len() - 1;
        }
        // End thread which send info to gui
        progress_thread_run.store(false, Ordering::Relaxed);
        progress_thread_handle.join().unwrap();

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
        println!("Number of removed files - {}", self.information.number_of_removed_files);
        println!("Number of failed to remove files - {}", self.information.number_of_failed_to_remove_files);
        println!("Number of duplicated music files - {}", self.information.number_of_duplicates_music_files);

        println!("### Other");

        println!("Excluded items - {:?}", self.excluded_items.items);
        println!("Minimum file size - {:?}", self.minimal_file_size);
        println!("Found files music - {}", self.music_entries.len());
        println!("Found duplicated files music - {}", self.duplicated_music_entries.len());
        println!("Included directories - {:?}", self.directories.included_directories);
        println!("Excluded directories - {:?}", self.directories.excluded_directories);
        println!("Recursive search - {}", self.recursive_search.to_string());
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
            writeln!(writer, "Found {} same music files.", self.information.number_of_music_entries).unwrap();
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
                    "T: {}  -  A: {}  -  AT: {}  -  AA: {}  -  Y: {}  -  P: {}",
                    file_entry.title,
                    file_entry.artist,
                    file_entry.album_title,
                    file_entry.album_artist,
                    file_entry.year,
                    file_entry.path.display()
                );
            }
            println!();
        }

        Common::print_time(start_time, SystemTime::now(), "print_entries".to_string());
    }
}
