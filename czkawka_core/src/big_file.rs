use std::collections::BTreeMap;
use std::fs::{File, Metadata};
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::sync::atomic::Ordering;
use std::sync::atomic::{AtomicBool, AtomicU64};
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{fs, thread};

use crossbeam_channel::Receiver;
use humansize::{file_size_opts as options, FileSize};
use rayon::prelude::*;

use crate::common::{Common, LOOP_DURATION};
use crate::common_directory::Directories;
use crate::common_extensions::Extensions;
use crate::common_items::ExcludedItems;
use crate::common_messages::Messages;
use crate::common_traits::{DebugPrint, PrintResults, SaveResults};
use crate::flc;
use crate::localizer_core::generate_translation_hashmap;

#[derive(Debug)]
pub struct ProgressData {
    pub files_checked: usize,
}

#[derive(Clone)]
pub struct FileEntry {
    pub path: PathBuf,
    pub size: u64,
    pub modified_date: u64,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum DeleteMethod {
    None,
    Delete,
}

/// Info struck with helpful information's about results
#[derive(Default)]
pub struct Info {
    pub number_of_real_files: usize,
}

impl Info {
    pub fn new() -> Self {
        Default::default()
    }
}

/// Struct with required information's to work
pub struct BigFile {
    text_messages: Messages,
    information: Info,
    big_files: BTreeMap<u64, Vec<FileEntry>>,
    excluded_items: ExcludedItems,
    directories: Directories,
    allowed_extensions: Extensions,
    recursive_search: bool,
    number_of_files_to_check: usize,
    delete_method: DeleteMethod,
    stopped_search: bool,
}

impl BigFile {
    pub fn new() -> Self {
        Self {
            text_messages: Default::default(),
            information: Info::new(),
            big_files: Default::default(),
            excluded_items: ExcludedItems::new(),
            directories: Directories::new(),
            allowed_extensions: Extensions::new(),
            recursive_search: true,
            number_of_files_to_check: 50,
            delete_method: DeleteMethod::None,
            stopped_search: false,
        }
    }

    pub fn find_big_files(&mut self, stop_receiver: Option<&Receiver<()>>, progress_sender: Option<&futures::channel::mpsc::UnboundedSender<ProgressData>>) {
        self.optimize_directories();
        if !self.look_for_big_files(stop_receiver, progress_sender) {
            self.stopped_search = true;
            return;
        }
        self.delete_files();
        self.debug_print();
    }
    pub fn get_stopped_search(&self) -> bool {
        self.stopped_search
    }

    pub const fn get_big_files(&self) -> &BTreeMap<u64, Vec<FileEntry>> {
        &self.big_files
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

    /// List of allowed extensions, only files with this extensions will be checking if are duplicates
    pub fn set_allowed_extensions(&mut self, allowed_extensions: String) {
        self.allowed_extensions.set_allowed_extensions(allowed_extensions, &mut self.text_messages);
    }

    fn look_for_big_files(&mut self, stop_receiver: Option<&Receiver<()>>, progress_sender: Option<&futures::channel::mpsc::UnboundedSender<ProgressData>>) -> bool {
        let start_time: SystemTime = SystemTime::now();
        let mut folders_to_check: Vec<PathBuf> = Vec::with_capacity(1024 * 2); // This should be small enough too not see to big difference and big enough to store most of paths without needing to resize vector

        // Add root folders for finding
        for id in &self.directories.included_directories {
            folders_to_check.push(id.clone());
        }

        //// PROGRESS THREAD START
        let progress_thread_run = Arc::new(AtomicBool::new(true));

        let atomic_file_counter = Arc::new(AtomicU64::new(0));

        let progress_thread_handle = if let Some(progress_sender) = progress_sender {
            let progress_send = progress_sender.clone();
            let progress_thread_run = progress_thread_run.clone();
            let atomic_file_counter = atomic_file_counter.clone();
            thread::spawn(move || loop {
                progress_send
                    .unbounded_send(ProgressData {
                        files_checked: atomic_file_counter.load(Ordering::Relaxed) as usize,
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
        while !folders_to_check.is_empty() {
            if stop_receiver.is_some() && stop_receiver.unwrap().try_recv().is_ok() {
                // End thread which send info to gui
                progress_thread_run.store(false, Ordering::Relaxed);
                progress_thread_handle.join().unwrap();
                return false;
            }

            let segments: Vec<_> = folders_to_check
                .par_iter()
                .map(|current_folder| {
                    let mut dir_result = vec![];
                    let mut warnings = vec![];
                    let mut fe_result = vec![];
                    // Read current dir childrens
                    let read_dir = match fs::read_dir(&current_folder) {
                        Ok(t) => t,
                        Err(e) => {
                            warnings.push(flc!(
                                "core_cannot_open_dir",
                                generate_translation_hashmap(vec![("dir", current_folder.display().to_string()), ("reason", e.to_string())])
                            ));
                            return (dir_result, warnings, fe_result);
                        }
                    };

                    // Check every sub folder/file/link etc.
                    'dir: for entry in read_dir {
                        let entry_data = match entry {
                            Ok(t) => t,
                            Err(e) => {
                                warnings.push(flc!(
                                    "core_cannot_read_entry_dir",
                                    generate_translation_hashmap(vec![("dir", current_folder.display().to_string()), ("reason", e.to_string())])
                                ));
                                continue 'dir;
                            }
                        };
                        let metadata: Metadata = match entry_data.metadata() {
                            Ok(t) => t,
                            Err(e) => {
                                warnings.push(flc!(
                                    "core_cannot_read_metadata_dir",
                                    generate_translation_hashmap(vec![("dir", current_folder.display().to_string()), ("reason", e.to_string())])
                                ));
                                continue 'dir;
                            }
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

                            dir_result.push(next_folder);
                        } else if metadata.is_file() {
                            atomic_file_counter.fetch_add(1, Ordering::Relaxed);

                            let file_name_lowercase: String = match entry_data.file_name().into_string() {
                                Ok(t) => t,
                                Err(_inspected) => {
                                    warnings.push(flc!(
                                        "core_file_not_utf8_name",
                                        generate_translation_hashmap(vec![("name", entry_data.path().display().to_string())])
                                    ));
                                    continue 'dir;
                                }
                            }
                            .to_lowercase();

                            if !self.allowed_extensions.matches_filename(&file_name_lowercase) {
                                continue 'dir;
                            }

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
                                            warnings.push(flc!(
                                                "core_file_modified_before_epoch",
                                                generate_translation_hashmap(vec![("name", current_file_name.display().to_string())])
                                            ));
                                            0
                                        }
                                    },
                                    Err(e) => {
                                        warnings.push(flc!(
                                            "core_file_no_modification_date",
                                            generate_translation_hashmap(vec![("name", current_file_name.display().to_string()), ("reason", e.to_string())])
                                        ));
                                        0
                                    }
                                },
                            };

                            fe_result.push((fe.size, fe));
                        }
                    }
                    (dir_result, warnings, fe_result)
                })
                .collect();

            // Advance the frontier
            folders_to_check.clear();

            // Process collected data
            for (segment, warnings, fe_result) in segments {
                folders_to_check.extend(segment);
                self.text_messages.warnings.extend(warnings);
                for (size, fe) in fe_result {
                    self.big_files.entry(size).or_insert_with(Vec::new);
                    self.big_files.get_mut(&size).unwrap().push(fe);
                }
            }
        }

        // End thread which send info to gui
        progress_thread_run.store(false, Ordering::Relaxed);
        progress_thread_handle.join().unwrap();

        // Extract n biggest files to new TreeMap
        let mut new_map: BTreeMap<u64, Vec<FileEntry>> = Default::default();

        for (size, vector) in self.big_files.iter().rev() {
            if self.information.number_of_real_files < self.number_of_files_to_check {
                for file in vector {
                    if self.information.number_of_real_files < self.number_of_files_to_check {
                        new_map.entry(*size).or_insert_with(Vec::new);
                        new_map.get_mut(size).unwrap().push(file.clone());
                        self.information.number_of_real_files += 1;
                    } else {
                        break;
                    }
                }
            } else {
                break;
            }
        }
        self.big_files = new_map;

        Common::print_time(start_time, SystemTime::now(), "look_for_big_files".to_string());
        true
    }

    pub fn set_number_of_files_to_check(&mut self, number_of_files_to_check: usize) {
        self.number_of_files_to_check = number_of_files_to_check;
    }

    /// Setting excluded items which needs to contains * wildcard
    /// Are a lot of slower than absolute path, so it should be used to heavy
    pub fn set_excluded_items(&mut self, excluded_items: Vec<String>) {
        self.excluded_items.set_excluded_items(excluded_items, &mut self.text_messages);
    }

    fn optimize_directories(&mut self) {
        self.directories.optimize_directories(self.recursive_search, &mut self.text_messages);
    }

    pub fn set_included_directory(&mut self, included_directory: Vec<PathBuf>) {
        self.directories.set_included_directory(included_directory, &mut self.text_messages);
    }

    pub fn set_excluded_directory(&mut self, excluded_directory: Vec<PathBuf>) {
        self.directories.set_excluded_directory(excluded_directory, &mut self.text_messages);
    }

    /// Function to delete files, from filed Vector
    fn delete_files(&mut self) {
        let start_time: SystemTime = SystemTime::now();

        match self.delete_method {
            DeleteMethod::Delete => {
                for vec_file_entry in self.big_files.values() {
                    for file_entry in vec_file_entry {
                        if fs::remove_file(file_entry.path.clone()).is_err() {
                            self.text_messages.warnings.push(file_entry.path.display().to_string());
                        }
                    }
                }
            }
            DeleteMethod::None => {
                //Just do nothing
            }
        }

        Common::print_time(start_time, SystemTime::now(), "delete_files".to_string());
    }
}

impl Default for BigFile {
    fn default() -> Self {
        Self::new()
    }
}

impl DebugPrint for BigFile {
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
        println!("Big files size {} in {} groups", self.information.number_of_real_files, self.big_files.len());
        println!("Excluded items - {:?}", self.excluded_items.items);
        println!("Included directories - {:?}", self.directories.included_directories);
        println!("Excluded directories - {:?}", self.directories.excluded_directories);
        println!("Recursive search - {}", self.recursive_search);
        println!("Number of files to check - {:?}", self.number_of_files_to_check);
        println!("-----------------------------------------");
    }
}

impl SaveResults for BigFile {
    /// Saving results to provided file
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

        if self.information.number_of_real_files != 0 {
            write!(writer, "{} the biggest files.\n\n", self.information.number_of_real_files).unwrap();

            for (size, files) in self.big_files.iter().rev() {
                for file_entry in files {
                    writeln!(writer, "{} ({}) - {}", size.file_size(options::BINARY).unwrap(), size, file_entry.path.display()).unwrap();
                }
            }
        } else {
            write!(writer, "Not found any files.").unwrap();
        }
        Common::print_time(start_time, SystemTime::now(), "save_results_to_file".to_string());
        true
    }
}

impl PrintResults for BigFile {
    fn print_results(&self) {
        let start_time: SystemTime = SystemTime::now();
        for (size, vector) in self.big_files.iter().rev() {
            // TODO Align all to same width
            for entry in vector {
                println!("{} ({} bytes) - {}", size.file_size(options::BINARY).unwrap(), size, entry.path.display());
            }
        }
        Common::print_time(start_time, SystemTime::now(), "print_entries".to_string());
    }
}
