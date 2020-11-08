use std::fs;
use std::fs::{File, Metadata};
use std::io::prelude::*;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::common::Common;
use crate::common_directory::Directories;
use crate::common_extensions::Extensions;
use crate::common_items::ExcludedItems;
use crate::common_messages::Messages;
use crate::common_traits::*;
use crossbeam_channel::Receiver;
use rayon::prelude::*;

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum DeleteMethod {
    None,
    Delete,
}

#[derive(Clone)]
pub struct FileEntry {
    pub path: PathBuf,
    pub size: u64,
    pub modified_date: u64,
}

/// Info struck with helpful information's about results
#[derive(Default)]
pub struct Info {
    pub number_of_checked_files: usize,
    pub number_of_checked_folders: usize,
    pub number_of_ignored_files: usize,
    pub number_of_ignored_things: usize,
    pub number_of_zeroed_files: usize,
    pub number_of_removed_files: usize,
    pub number_of_failed_to_remove_files: usize,
}
impl Info {
    pub fn new() -> Self {
        Default::default()
    }
}

/// Struct with required information's to work
pub struct ZeroedFiles {
    text_messages: Messages,
    information: Info,
    zeroed_files: Vec<FileEntry>,
    directories: Directories,
    allowed_extensions: Extensions,
    excluded_items: ExcludedItems,
    recursive_search: bool,
    delete_method: DeleteMethod,
    stopped_search: bool,
    minimal_file_size: u64,
    files_to_check: Vec<FileEntry>,
}

impl ZeroedFiles {
    pub fn new() -> Self {
        Self {
            text_messages: Messages::new(),
            information: Info::new(),
            recursive_search: true,
            allowed_extensions: Extensions::new(),
            directories: Directories::new(),
            excluded_items: ExcludedItems::new(),
            zeroed_files: vec![],
            delete_method: DeleteMethod::None,
            stopped_search: false,
            minimal_file_size: 1024,
            files_to_check: Vec::with_capacity(1024),
        }
    }

    pub fn find_zeroed_files(&mut self, stop_receiver: Option<&Receiver<()>>) {
        self.directories.optimize_directories(self.recursive_search, &mut self.text_messages);
        if !self.check_files(stop_receiver) {
            self.stopped_search = true;
            return;
        }
        if !self.check_for_zeroed_files(stop_receiver) {
            self.stopped_search = true;
            return;
        }
        self.delete_files();
        self.debug_print();
    }

    pub fn get_stopped_search(&self) -> bool {
        self.stopped_search
    }

    pub const fn get_zeroed_files(&self) -> &Vec<FileEntry> {
        &self.zeroed_files
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

    pub fn set_minimal_file_size(&mut self, minimal_file_size: u64) {
        self.minimal_file_size = match minimal_file_size {
            0 => 1,
            t => t,
        };
    }

    pub fn set_recursive_search(&mut self, recursive_search: bool) {
        self.recursive_search = recursive_search;
    }

    pub fn set_included_directory(&mut self, included_directory: String) -> bool {
        self.directories.set_included_directory(included_directory, &mut self.text_messages)
    }

    pub fn set_excluded_directory(&mut self, excluded_directory: String) {
        self.directories.set_excluded_directory(excluded_directory, &mut self.text_messages);
    }
    pub fn set_allowed_extensions(&mut self, allowed_extensions: String) {
        self.allowed_extensions.set_allowed_extensions(allowed_extensions, &mut self.text_messages);
    }

    pub fn set_excluded_items(&mut self, excluded_items: String) {
        self.excluded_items.set_excluded_items(excluded_items, &mut self.text_messages);
    }

    /// Check files for files which have 0
    fn check_files(&mut self, stop_receiver: Option<&Receiver<()>>) -> bool {
        let start_time: SystemTime = SystemTime::now();
        let mut folders_to_check: Vec<PathBuf> = Vec::with_capacity(1024 * 2); // This should be small enough too not see to big difference and big enough to store most of paths without needing to resize vector

        // Add root folders for finding
        for id in &self.directories.included_directories {
            folders_to_check.push(id.clone());
        }
        self.information.number_of_checked_folders += folders_to_check.len();

        while !folders_to_check.is_empty() {
            if stop_receiver.is_some() && stop_receiver.unwrap().try_recv().is_ok() {
                return false;
            }
            let current_folder = folders_to_check.pop().unwrap();

            // Read current dir, if permission are denied just go to next
            let read_dir = match fs::read_dir(&current_folder) {
                Ok(t) => t,
                Err(_) => {
                    self.text_messages.warnings.push(format!("Cannot open dir {}", current_folder.display()));
                    continue;
                } // Permissions denied
            };

            // Check every sub folder/file/link etc.
            'dir: for entry in read_dir {
                let entry_data = match entry {
                    Ok(t) => t,
                    Err(_) => {
                        self.text_messages.warnings.push(format!("Cannot read entry in dir {}", current_folder.display()));
                        continue;
                    } //Permissions denied
                };
                let metadata: Metadata = match entry_data.metadata() {
                    Ok(t) => t,
                    Err(_) => {
                        self.text_messages.warnings.push(format!("Cannot read metadata in dir {}", current_folder.display()));
                        continue;
                    } //Permissions denied
                };
                if metadata.is_dir() {
                    self.information.number_of_checked_folders += 1;

                    if !self.recursive_search {
                        continue;
                    }

                    let next_folder = current_folder.join(entry_data.file_name());
                    if self.directories.is_excluded(&next_folder) || self.excluded_items.is_excluded(&next_folder) {
                        continue 'dir;
                    }

                    folders_to_check.push(next_folder);
                } else if metadata.is_file() {
                    if metadata.len() == 0 || metadata.len() < self.minimal_file_size {
                        self.information.number_of_ignored_files += 1;
                        continue 'dir;
                    }

                    let file_name_lowercase: String = match entry_data.file_name().into_string() {
                        Ok(t) => t,
                        Err(_) => continue,
                    }
                    .to_lowercase();

                    // Checking allowed extensions
                    if !self.allowed_extensions.file_extensions.is_empty() {
                        let allowed = self.allowed_extensions.file_extensions.iter().any(|e| file_name_lowercase.ends_with((".".to_string() + e.to_lowercase().as_str()).as_str()));
                        if !allowed {
                            // Not an allowed extension, ignore it.
                            self.information.number_of_ignored_files += 1;
                            continue 'dir;
                        }
                    }
                    // Checking files
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
                                Err(_) => {
                                    self.text_messages.warnings.push(format!("File {} seems to be modified before Unix Epoch.", current_file_name.display()));
                                    0
                                }
                            },
                            Err(_) => {
                                self.text_messages.warnings.push(format!("Unable to get modification date from file {}", current_file_name.display()));
                                continue;
                            } // Permissions Denied
                        },
                    };

                    // Adding files to Vector
                    self.files_to_check.push(fe);

                    self.information.number_of_checked_files += 1;
                } else {
                    // Probably this is symbolic links so we are free to ignore this
                    self.information.number_of_ignored_things += 1;
                }
            }
        }

        Common::print_time(start_time, SystemTime::now(), "check_files".to_string());
        true
    }

    /// Check files for files which have 0
    fn check_for_zeroed_files(&mut self, stop_receiver: Option<&Receiver<()>>) -> bool {
        let start_time: SystemTime = SystemTime::now();

        self.zeroed_files = self
            .files_to_check
            .par_iter()
            .map(|file_entry| {
                if stop_receiver.is_some() && stop_receiver.unwrap().try_recv().is_ok() {
                    // This will not break
                    return None;
                }

                let file_entry = file_entry.clone();
                let mut n;
                let mut file_handler: File = match File::open(&file_entry.path) {
                    Ok(t) => t,
                    Err(_) => {
                        return Some(None);
                    }
                };

                // First search
                let mut buffer = [0u8; 64];
                n = match file_handler.read(&mut buffer) {
                    Ok(t) => t,
                    Err(_) => {
                        return Some(None);
                    }
                };
                for i in buffer[0..n].iter() {
                    if *i != 0 {
                        return Some(None);
                    }
                }
                // Second search
                loop {
                    let mut buffer = [0u8; 1024 * 32];
                    n = match file_handler.read(&mut buffer) {
                        Ok(t) => t,
                        Err(_) => {
                            return Some(None);
                        }
                    };
                    for i in buffer[0..n].iter() {
                        if *i != 0 {
                            return Some(None);
                        }
                    }
                    if n == 0 {
                        break;
                    }
                }

                Some(Some(file_entry))
            })
            .while_some()
            .filter(|file_entry| file_entry.is_some())
            .map(|file_entry| file_entry.unwrap())
            .collect::<Vec<_>>();

        self.information.number_of_zeroed_files = self.zeroed_files.len();

        Common::print_time(start_time, SystemTime::now(), "search for zeroed_files".to_string());
        true
    }

    /// Function to delete files, from filed Vector
    fn delete_files(&mut self) {
        let start_time: SystemTime = SystemTime::now();

        match self.delete_method {
            DeleteMethod::Delete => {
                for file_entry in &self.zeroed_files {
                    if fs::remove_file(file_entry.path.clone()).is_err() {
                        self.text_messages.warnings.push(file_entry.path.display().to_string());
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
impl Default for ZeroedFiles {
    fn default() -> Self {
        Self::new()
    }
}

impl DebugPrint for ZeroedFiles {
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
        println!("Number of checked files - {}", self.information.number_of_checked_files);
        println!("Number of checked folders - {}", self.information.number_of_checked_folders);
        println!("Number of ignored files - {}", self.information.number_of_ignored_files);
        println!("Number of ignored things(like symbolic links) - {}", self.information.number_of_ignored_things);
        println!("Number of removed files - {}", self.information.number_of_removed_files);
        println!("Number of failed to remove files - {}", self.information.number_of_failed_to_remove_files);

        println!("### Other");

        println!("Zeroed list size - {}", self.zeroed_files.len());
        println!("Allowed extensions - {:?}", self.allowed_extensions.file_extensions);
        println!("Excluded items - {:?}", self.excluded_items.items);
        println!("Included directories - {:?}", self.directories.included_directories);
        println!("Excluded directories - {:?}", self.directories.excluded_directories);
        println!("Recursive search - {}", self.recursive_search.to_string());
        println!("Delete Method - {:?}", self.delete_method);
        println!("Minimal File Size - {:?}", self.minimal_file_size);
        println!("-----------------------------------------");
    }
}
impl SaveResults for ZeroedFiles {
    fn save_results_to_file(&mut self, file_name: &str) -> bool {
        let start_time: SystemTime = SystemTime::now();
        let file_name: String = match file_name {
            "" => "results.txt".to_string(),
            k => k.to_string(),
        };

        let mut file = match File::create(&file_name) {
            Ok(t) => t,
            Err(_) => {
                self.text_messages.errors.push(format!("Failed to create file {}", file_name));
                return false;
            }
        };

        if writeln!(
            file,
            "Results of searching {:?} with excluded directories {:?} and excluded items {:?}",
            self.directories.included_directories, self.directories.excluded_directories, self.excluded_items.items
        )
        .is_err()
        {
            self.text_messages.errors.push(format!("Failed to save results to file {}", file_name));
            return false;
        }

        if !self.zeroed_files.is_empty() {
            writeln!(file, "Found {} zeroed files.", self.information.number_of_zeroed_files).unwrap();
            for file_entry in self.zeroed_files.iter() {
                writeln!(file, "{}", file_entry.path.display()).unwrap();
            }
        } else {
            write!(file, "Not found any zeroed files.").unwrap();
        }
        Common::print_time(start_time, SystemTime::now(), "save_results_to_file".to_string());
        true
    }
}
impl PrintResults for ZeroedFiles {
    /// Print information's about duplicated entries
    /// Only needed for CLI
    fn print_results(&self) {
        let start_time: SystemTime = SystemTime::now();
        println!("Found {} zeroed files.\n", self.information.number_of_zeroed_files);
        for file_entry in self.zeroed_files.iter() {
            println!("{}", file_entry.path.display());
        }

        Common::print_time(start_time, SystemTime::now(), "print_entries".to_string());
    }
}
