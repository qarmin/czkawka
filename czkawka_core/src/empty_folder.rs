use std::collections::BTreeMap;
use std::fs;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::time::SystemTime;

use crossbeam_channel::Receiver;

use crate::common::Common;
use crate::common_dir_traversal::{Collect, DirTraversalBuilder, DirTraversalResult, FolderEmptiness, FolderEntry, ProgressData};
use crate::common_directory::Directories;
use crate::common_items::ExcludedItems;
use crate::common_messages::Messages;
use crate::common_traits::{DebugPrint, PrintResults, SaveResults};

/// Struct to store most basics info about all folder
pub struct EmptyFolder {
    information: Info,
    delete_folders: bool,
    text_messages: Messages,
    excluded_items: ExcludedItems,
    empty_folder_list: BTreeMap<PathBuf, FolderEntry>, // Path, FolderEntry
    directories: Directories,
    stopped_search: bool,
}

/// Info struck with helpful information's about results
#[derive(Default)]
pub struct Info {
    pub number_of_empty_folders: usize,
}

impl Info {
    #[must_use]
    pub fn new() -> Self {
        Default::default()
    }
}

/// Method implementation for `EmptyFolder`
impl EmptyFolder {
    /// New function providing basics values
    #[must_use]
    pub fn new() -> Self {
        Self {
            information: Default::default(),
            delete_folders: false,
            text_messages: Messages::new(),
            excluded_items: Default::default(),
            empty_folder_list: Default::default(),
            directories: Directories::new(),
            stopped_search: false,
        }
    }

    #[must_use]
    pub fn get_stopped_search(&self) -> bool {
        self.stopped_search
    }

    #[must_use]
    pub const fn get_empty_folder_list(&self) -> &BTreeMap<PathBuf, FolderEntry> {
        &self.empty_folder_list
    }

    #[must_use]
    pub const fn get_text_messages(&self) -> &Messages {
        &self.text_messages
    }
    #[must_use]
    pub const fn get_information(&self) -> &Info {
        &self.information
    }

    #[cfg(target_family = "unix")]
    pub fn set_exclude_other_filesystems(&mut self, exclude_other_filesystems: bool) {
        self.directories.set_exclude_other_filesystems(exclude_other_filesystems);
    }
    #[cfg(not(target_family = "unix"))]
    pub fn set_exclude_other_filesystems(&mut self, _exclude_other_filesystems: bool) {}

    pub fn set_excluded_items(&mut self, excluded_items: Vec<String>) {
        self.excluded_items.set_excluded_items(excluded_items, &mut self.text_messages);
    }

    pub fn set_excluded_directory(&mut self, excluded_directory: Vec<PathBuf>) {
        self.directories.set_excluded_directory(excluded_directory, &mut self.text_messages);
    }
    /// Public function used by CLI to search for empty folders
    pub fn find_empty_folders(&mut self, stop_receiver: Option<&Receiver<()>>, progress_sender: Option<&futures::channel::mpsc::UnboundedSender<ProgressData>>) {
        self.directories.optimize_directories(true, &mut self.text_messages);
        if !self.check_for_empty_folders(stop_receiver, progress_sender) {
            self.stopped_search = true;
            return;
        }
        self.optimize_folders();
        if self.delete_folders {
            self.delete_empty_folders();
        }
        self.debug_print();
    }

    pub fn set_delete_folder(&mut self, delete_folder: bool) {
        self.delete_folders = delete_folder;
    }

    /// Clean directory tree
    /// If directory contains only 2 empty folders, then this directory should be removed instead two empty folders inside because it will produce another empty folder.
    fn optimize_folders(&mut self) {
        let mut new_directory_folders: BTreeMap<PathBuf, FolderEntry> = Default::default();

        for (name, folder_entry) in &self.empty_folder_list {
            match &folder_entry.parent_path {
                Some(t) => {
                    if !self.empty_folder_list.contains_key(t) {
                        new_directory_folders.insert(name.clone(), folder_entry.clone());
                    }
                }
                None => {
                    new_directory_folders.insert(name.clone(), folder_entry.clone());
                }
            }
        }
        self.empty_folder_list = new_directory_folders;
        self.information.number_of_empty_folders = self.empty_folder_list.len();
    }

    /// Function to check if folder are empty.
    /// Parameter `initial_checking` for second check before deleting to be sure that checked folder is still empty
    fn check_for_empty_folders(&mut self, stop_receiver: Option<&Receiver<()>>, progress_sender: Option<&futures::channel::mpsc::UnboundedSender<ProgressData>>) -> bool {
        let result = DirTraversalBuilder::new()
            .root_dirs(self.directories.included_directories.clone())
            .group_by(|_fe| ())
            .stop_receiver(stop_receiver)
            .progress_sender(progress_sender)
            .directories(self.directories.clone())
            .excluded_items(self.excluded_items.clone())
            .collect(Collect::EmptyFolders)
            .max_stage(0)
            .build()
            .run();
        match result {
            DirTraversalResult::SuccessFiles { .. } => {
                unreachable!()
            }
            DirTraversalResult::SuccessFolders {
                start_time,
                folder_entries,
                warnings,
            } => {
                // We need to set empty folder list
                #[allow(unused_mut)] // Used is later by Windows build
                for (mut name, folder_entry) in folder_entries {
                    if folder_entry.is_empty != FolderEmptiness::No {
                        self.empty_folder_list.insert(name, folder_entry);
                    }
                }

                self.text_messages.warnings.extend(warnings);

                Common::print_time(start_time, SystemTime::now(), "check_for_empty_folder");
                true
            }
            DirTraversalResult::Stopped => false,
        }
    }

    /// Deletes earlier found empty folders
    fn delete_empty_folders(&mut self) {
        let start_time: SystemTime = SystemTime::now();
        // Folders may be deleted or require too big privileges
        for name in self.empty_folder_list.keys() {
            match fs::remove_dir_all(name) {
                Ok(_) => (),
                Err(e) => self.text_messages.warnings.push(format!("Failed to remove folder {}, reason {}", name.display(), e)),
            };
        }

        Common::print_time(start_time, SystemTime::now(), "delete_files");
    }

    /// Set included dir which needs to be relative, exists etc.
    pub fn set_included_directory(&mut self, included_directory: Vec<PathBuf>) {
        self.directories.set_included_directory(included_directory, &mut self.text_messages);
    }
}

impl Default for EmptyFolder {
    fn default() -> Self {
        Self::new()
    }
}

impl DebugPrint for EmptyFolder {
    #[allow(dead_code)]
    #[allow(unreachable_code)]
    fn debug_print(&self) {
        #[cfg(not(debug_assertions))]
        {
            return;
        }

        println!("---------------DEBUG PRINT---------------");
        println!("Number of empty folders - {}", self.information.number_of_empty_folders);
        println!("Included directories - {:?}", self.directories.included_directories);
        println!("-----------------------------------------");
    }
}

impl SaveResults for EmptyFolder {
    fn save_results_to_file(&mut self, file_name: &str) -> bool {
        let start_time: SystemTime = SystemTime::now();
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
            "Results of searching {:?} with excluded directories {:?}",
            self.directories.included_directories, self.directories.excluded_directories
        ) {
            self.text_messages.errors.push(format!("Failed to save results to file {file_name}, reason {e}"));
            return false;
        }

        if !self.empty_folder_list.is_empty() {
            writeln!(
                writer,
                "-------------------------------------------------Empty folder list-------------------------------------------------"
            )
            .unwrap();
            writeln!(writer, "Found {} empty folders", self.information.number_of_empty_folders).unwrap();
            for name in self.empty_folder_list.keys() {
                writeln!(writer, "{}", name.display()).unwrap();
            }
        } else {
            write!(writer, "Not found any empty folders.").unwrap();
        }
        Common::print_time(start_time, SystemTime::now(), "save_results_to_file");
        true
    }
}

impl PrintResults for EmptyFolder {
    fn print_results(&self) {
        if !self.empty_folder_list.is_empty() {
            println!("Found {} empty folders", self.empty_folder_list.len());
        }
        for name in self.empty_folder_list.keys() {
            println!("{}", name.display());
        }
    }
}
