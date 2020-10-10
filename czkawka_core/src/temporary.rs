use std::fs;
use std::fs::{File, Metadata};
use std::io::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::common::Common;
use crate::common_directory::Directories;
use crate::common_items::ExcludedItems;
use crate::common_messages::Messages;
use crate::common_traits::*;
use crossbeam_channel::Receiver;

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum DeleteMethod {
    None,
    Delete,
}

#[derive(Clone)]
pub struct FileEntry {
    pub path: String,
    pub modified_date: u64,
}

/// Info struck with helpful information's about results
#[derive(Default)]
pub struct Info {
    pub number_of_checked_files: usize,
    pub number_of_checked_folders: usize,
    pub number_of_ignored_files: usize,
    pub number_of_ignored_things: usize,
    pub number_of_temporary_files: usize,
    pub number_of_removed_files: usize,
    pub number_of_failed_to_remove_files: usize,
}
impl Info {
    pub fn new() -> Self {
        Default::default()
    }
}

/// Struct with required information's to work
pub struct Temporary {
    text_messages: Messages,
    information: Info,
    temporary_files: Vec<FileEntry>,
    directories: Directories,
    excluded_items: ExcludedItems,
    recursive_search: bool,
    delete_method: DeleteMethod,
    stopped_search: bool,
}

impl Temporary {
    pub fn new() -> Self {
        Self {
            text_messages: Messages::new(),
            information: Info::new(),
            recursive_search: true,
            directories: Directories::new(),
            excluded_items: ExcludedItems::new(),
            delete_method: DeleteMethod::None,
            temporary_files: vec![],
            stopped_search: false,
        }
    }

    /// Finding temporary files, save results to internal struct variables
    pub fn find_temporary_files(&mut self, rx: Option<&Receiver<()>>) {
        self.directories.optimize_directories(self.recursive_search, &mut self.text_messages);
        if !self.check_files(rx) {
            self.stopped_search = true;
            return;
        }
        self.delete_files();
        self.debug_print();
    }
    pub fn get_stopped_search(&self) -> bool {
        self.stopped_search
    }

    pub const fn get_temporary_files(&self) -> &Vec<FileEntry> {
        &self.temporary_files
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

    pub fn set_included_directory(&mut self, included_directory: String) -> bool {
        self.directories.set_included_directory(included_directory, &mut self.text_messages)
    }

    pub fn set_excluded_directory(&mut self, excluded_directory: String) {
        self.directories.set_excluded_directory(excluded_directory, &mut self.text_messages);
    }

    pub fn set_excluded_items(&mut self, excluded_items: String) {
        self.excluded_items.set_excluded_items(excluded_items, &mut self.text_messages);
    }

    fn check_files(&mut self, rx: Option<&Receiver<()>>) -> bool {
        let start_time: SystemTime = SystemTime::now();
        let mut folders_to_check: Vec<String> = Vec::with_capacity(1024 * 2); // This should be small enough too not see to big difference and big enough to store most of paths without needing to resize vector

        // Add root folders for finding
        for id in &self.directories.included_directories {
            folders_to_check.push(id.to_string());
        }
        self.information.number_of_checked_folders += folders_to_check.len();

        let mut current_folder: String;
        let mut next_folder: String;
        while !folders_to_check.is_empty() {
            if rx.is_some() && rx.unwrap().try_recv().is_ok() {
                return false;
            }
            current_folder = folders_to_check.pop().unwrap();

            #[cfg(target_family = "windows")]
            {
                current_folder = Common::prettier_windows_path(&current_folder);
            }
            // Read current dir, if permission are denied just go to next
            let read_dir = match fs::read_dir(&current_folder) {
                Ok(t) => t,
                Err(_) => {
                    self.text_messages.warnings.push("Cannot open dir ".to_string() + current_folder.as_str());
                    continue;
                } // Permissions denied
            };

            // Check every sub folder/file/link etc.
            'dir: for entry in read_dir {
                let entry_data = match entry {
                    Ok(t) => t,
                    Err(_) => {
                        self.text_messages.warnings.push("Cannot read entry in dir ".to_string() + current_folder.as_str());
                        continue;
                    } //Permissions denied
                };
                let metadata: Metadata = match entry_data.metadata() {
                    Ok(t) => t,
                    Err(_) => {
                        self.text_messages.warnings.push("Cannot read metadata in dir ".to_string() + current_folder.as_str());
                        continue;
                    } //Permissions denied
                };
                if metadata.is_dir() {
                    self.information.number_of_checked_folders += 1;

                    if !self.recursive_search {
                        continue;
                    }

                    next_folder = "".to_owned()
                        + &current_folder
                        + match &entry_data.file_name().into_string() {
                            Ok(t) => t,
                            Err(_) => continue,
                        }
                        + "/";

                    for ed in &self.directories.excluded_directories {
                        if next_folder == *ed {
                            continue 'dir;
                        }
                    }
                    for expression in &self.excluded_items.items {
                        if Common::regex_check(expression, &next_folder) {
                            continue 'dir;
                        }
                    }
                    folders_to_check.push(next_folder);
                } else if metadata.is_file() {
                    let file_name_lowercase: String = match entry_data.file_name().into_string() {
                        Ok(t) => t,
                        Err(_) => continue,
                    }
                    .to_lowercase();

                    // Temporary files which needs to have dot in name(not sure if exists without dot)
                    let temporary_with_dot = ["#", "thumbs.db", ".bak", "~", ".tmp", ".temp", ".ds_store", ".crdownload", ".part", ".cache", ".dmp", ".download", ".partial"];

                    if !file_name_lowercase.contains('.') || !temporary_with_dot.iter().any(|f| file_name_lowercase.ends_with(f)) {
                        self.information.number_of_ignored_files += 1;
                        continue 'dir;
                    }

                    // Checking files
                    #[allow(unused_mut)] // Used is later by Windows build
                    let mut current_file_name = "".to_owned()
                        + &current_folder
                        + match &entry_data.file_name().into_string() {
                            Ok(t) => t,
                            Err(_) => continue,
                        };

                    // Checking expressions
                    for expression in &self.excluded_items.items {
                        if Common::regex_check(expression, &current_file_name) {
                            continue 'dir;
                        }
                    }

                    #[cfg(target_family = "windows")]
                    {
                        current_file_name = Common::prettier_windows_path(&current_file_name);
                    }

                    // Creating new file entry
                    let fe: FileEntry = FileEntry {
                        path: current_file_name.clone(),
                        modified_date: match metadata.modified() {
                            Ok(t) => match t.duration_since(UNIX_EPOCH) {
                                Ok(d) => d.as_secs(),
                                Err(_) => {
                                    self.text_messages.warnings.push(format!("File {} seems to be modified before Unix Epoch.", current_file_name));
                                    0
                                }
                            },
                            Err(_) => {
                                self.text_messages.warnings.push("Unable to get modification date from file ".to_string() + current_file_name.as_str());
                                continue;
                            } // Permissions Denied
                        },
                    };

                    // Adding files to Vector
                    self.temporary_files.push(fe);

                    self.information.number_of_checked_files += 1;
                } else {
                    // Probably this is symbolic links so we are free to ignore this
                    self.information.number_of_ignored_things += 1;
                }
            }
        }
        self.information.number_of_temporary_files = self.temporary_files.len();

        Common::print_time(start_time, SystemTime::now(), "check_files_size".to_string());
        true
    }

    /// Function to delete files, from filed Vector
    fn delete_files(&mut self) {
        let start_time: SystemTime = SystemTime::now();

        match self.delete_method {
            DeleteMethod::Delete => {
                for file_entry in &self.temporary_files {
                    if fs::remove_file(file_entry.path.clone()).is_err() {
                        self.text_messages.warnings.push(file_entry.path.clone());
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
impl Default for Temporary {
    fn default() -> Self {
        Self::new()
    }
}

impl DebugPrint for Temporary {
    #[allow(dead_code)]
    #[allow(unreachable_code)]
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

        println!("Temporary list size - {}", self.temporary_files.len());
        println!("Excluded items - {:?}", self.excluded_items.items);
        println!("Included directories - {:?}", self.directories.included_directories);
        println!("Excluded directories - {:?}", self.directories.excluded_directories);
        println!("Recursive search - {}", self.recursive_search.to_string());
        println!("Delete Method - {:?}", self.delete_method);
        println!("-----------------------------------------");
    }
}
impl SaveResults for Temporary {
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

        match file.write_all(
            format!(
                "Results of searching {:?} with excluded directories {:?} and excluded items {:?}\n",
                self.directories.included_directories, self.directories.excluded_directories, self.excluded_items.items
            )
            .as_bytes(),
        ) {
            Ok(_) => (),
            Err(_) => {
                self.text_messages.errors.push(format!("Failed to save results to file {}", file_name));
                return false;
            }
        }

        if !self.temporary_files.is_empty() {
            file.write_all(format!("Found {} temporary files.\n", self.information.number_of_temporary_files).as_bytes()).unwrap();
            for file_entry in self.temporary_files.iter() {
                file.write_all(format!("{} \n", file_entry.path).as_bytes()).unwrap();
            }
        } else {
            file.write_all(b"Not found any temporary files.").unwrap();
        }
        Common::print_time(start_time, SystemTime::now(), "save_results_to_file".to_string());
        true
    }
}
impl PrintResults for Temporary {
    fn print_results(&self) {
        let start_time: SystemTime = SystemTime::now();
        println!("Found {} temporary files.\n", self.information.number_of_temporary_files);
        for file_entry in self.temporary_files.iter() {
            println!("{}", file_entry.path);
        }

        Common::print_time(start_time, SystemTime::now(), "print_entries".to_string());
    }
}
