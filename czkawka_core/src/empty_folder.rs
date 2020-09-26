use crate::common::Common;
use crate::common_directory::Directories;
use crate::common_messages::Messages;
use crate::common_traits::{DebugPrint, PrintResults, SaveResults};
use std::collections::BTreeMap;
use std::fs;
use std::fs::{File, Metadata};
use std::io::Write;
use std::time::SystemTime;

/// Enum with values which show if folder is empty.
/// In function "optimize_folders" automatically "Maybe" is changed to "Yes", so it is not necessary to put it here
#[derive(Eq, PartialEq, Copy, Clone)]
enum FolderEmptiness {
    No,
    Maybe,
}

/// Struct assigned to each checked folder with parent path(used to ignore parent if children are not empty) and flag which shows if folder is empty
#[derive(Clone)]
pub struct FolderEntry {
    parent_path: Option<String>, // Usable only when finding
    is_empty: FolderEmptiness,
    pub modified_date: SystemTime,
}

/// Struct to store most basics info about all folder
pub struct EmptyFolder {
    information: Info,
    delete_folders: bool,
    text_messages: Messages,
    empty_folder_list: BTreeMap<String, FolderEntry>, // Path, FolderEntry
    directories: Directories,
}

/// Info struck with helpful information's about results
pub struct Info {
    number_of_checked_folders: usize,
    pub number_of_empty_folders: usize,
}
impl Info {
    pub fn new() -> Info {
        Info {
            number_of_checked_folders: 0,
            number_of_empty_folders: 0,
        }
    }
}

impl Default for Info {
    fn default() -> Self {
        Self::new()
    }
}
/// Method implementation for EmptyFolder
impl EmptyFolder {
    /// New function providing basics values
    pub fn new() -> EmptyFolder {
        EmptyFolder {
            information: Default::default(),
            delete_folders: false,
            text_messages: Messages::new(),
            empty_folder_list: Default::default(),
            directories: Directories::new(),
        }
    }

    pub fn get_empty_folder_list(&self) -> &BTreeMap<String, FolderEntry> {
        &self.empty_folder_list
    }

    pub fn get_text_messages(&self) -> &Messages {
        &self.text_messages
    }
    pub fn get_information(&self) -> &Info {
        &self.information
    }

    /// Public function used by CLI to search for empty folders
    pub fn find_empty_folders(&mut self) {
        self.directories.optimize_directories(true, &mut self.text_messages);
        self.check_for_empty_folders(true);
        //self.check_for_empty_folders(false); // Second check, should be done before deleting to be sure that empty folder is still empty
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
        let mut new_directory_folders: BTreeMap<String, FolderEntry> = Default::default();

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
    /// Parameter initial_checking for second check before deleting to be sure that checked folder is still empty
    fn check_for_empty_folders(&mut self, initial_checking: bool) {
        let start_time: SystemTime = SystemTime::now();
        let mut folders_to_check: Vec<String> = Vec::with_capacity(1024 * 2); // This should be small enough too not see to big difference and big enough to store most of paths without needing to resize vector
        let mut folders_checked: BTreeMap<String, FolderEntry> = Default::default();

        if initial_checking {
            // Add root folders for finding
            for id in &self.directories.included_directories {
                folders_checked.insert(
                    id.clone(),
                    FolderEntry {
                        parent_path: None,
                        is_empty: FolderEmptiness::Maybe,
                        modified_date: SystemTime::now(),
                    },
                );
                folders_to_check.push(id.clone());
            }
        } else {
            // Add folders searched before
            for (name, folder_entry) in &self.empty_folder_list {
                folders_checked.insert(
                    name.clone(),
                    FolderEntry {
                        parent_path: None,
                        is_empty: FolderEmptiness::Maybe,
                        modified_date: folder_entry.modified_date,
                    },
                );
                folders_to_check.push(name.clone());
            }
        }

        let mut current_folder: String;
        let mut next_folder: String;
        while !folders_to_check.is_empty() {
            self.information.number_of_checked_folders += 1;
            current_folder = folders_to_check.pop().unwrap();
            // Checked folder may be deleted or we may not have permissions to open it so we assume that this folder is not be empty
            let read_dir = match fs::read_dir(&current_folder) {
                Ok(t) => t,
                Err(_) => {
                    folders_checked.get_mut(&current_folder).unwrap().is_empty = FolderEmptiness::No;
                    continue;
                }
            };

            for entry in read_dir {
                let entry_data = match entry {
                    Ok(t) => t,
                    Err(_) => continue, //Permissions denied
                };
                let metadata: Metadata = match entry_data.metadata() {
                    Ok(t) => t,
                    Err(_) => continue, //Permissions denied
                };
                // If child is dir, still folder may be considered as empty if all children are only directories.
                if metadata.is_dir() {
                    next_folder = "".to_owned() + &current_folder + &entry_data.file_name().into_string().unwrap() + "/";
                    folders_to_check.push(next_folder.clone());

                    folders_checked.insert(
                        next_folder.clone(),
                        FolderEntry {
                            parent_path: Option::from(current_folder.clone()),
                            is_empty: FolderEmptiness::Maybe,
                            modified_date: match metadata.modified() {
                                Ok(t) => t,
                                Err(_) => {
                                    self.text_messages.warnings.push(format!("Failed to read modification date of folder {}", current_folder));
                                    SystemTime::now()
                                }
                            },
                        },
                    );
                } else {
                    // Not folder so it may be a file or symbolic link so it isn't empty
                    folders_checked.get_mut(&current_folder).unwrap().is_empty = FolderEmptiness::No;
                    let mut d = folders_checked.get_mut(&current_folder).unwrap();
                    let mut cf: String;
                    // Loop to recursively set as non empty this and all his parent folders
                    loop {
                        d.is_empty = FolderEmptiness::No;
                        if d.parent_path != None {
                            cf = d.parent_path.clone().unwrap();
                            d = folders_checked.get_mut(&cf).unwrap();
                        } else {
                            break;
                        }
                    }
                }
            }
        }

        // Now we check if checked folders are really empty, and if are, then
        if initial_checking {
            // We need to set empty folder list
            for (name, folder_entry) in folders_checked {
                if folder_entry.is_empty != FolderEmptiness::No {
                    self.empty_folder_list.insert(name, folder_entry);
                }
            }
        } else {
            // We need to check if parent of folder isn't also empty, because we wan't to delete only parent with two empty folders except this folders and at the end parent folder
            let mut new_folders_list: BTreeMap<String, FolderEntry> = Default::default();
            for (name, folder_entry) in folders_checked {
                if folder_entry.is_empty != FolderEmptiness::No && self.empty_folder_list.contains_key(&name) {
                    new_folders_list.insert(name, folder_entry);
                }
            }
            self.empty_folder_list = new_folders_list;
        }

        Common::print_time(start_time, SystemTime::now(), "check_for_empty_folder".to_string());
    }

    /// Deletes earlier found empty folders
    fn delete_empty_folders(&mut self) {
        let start_time: SystemTime = SystemTime::now();
        // Folders may be deleted or require too big privileges
        for name in self.empty_folder_list.keys() {
            match fs::remove_dir_all(name) {
                Ok(_) => (),
                Err(_) => self.text_messages.warnings.push(format!("Failed to remove folder {}", name)),
            };
        }

        Common::print_time(start_time, SystemTime::now(), "delete_files".to_string());
    }

    /// Set included dir which needs to be relative, exists etc.
    pub fn set_included_directory(&mut self, included_directory: String) {
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
        println!("Number of all checked folders - {}", self.information.number_of_checked_folders);
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

        let mut file = match File::create(&file_name) {
            Ok(t) => t,
            Err(_) => {
                self.text_messages.errors.push("Failed to create file ".to_string() + file_name.as_str());
                return false;
            }
        };

        match file.write_all(format!("Results of searching in {:?}\n", self.directories.included_directories).as_bytes()) {
            Ok(_) => (),
            Err(_) => {
                self.text_messages.errors.push("Failed to save results to file ".to_string() + file_name.as_str());
                return false;
            }
        }

        if !self.empty_folder_list.is_empty() {
            file.write_all(b"-------------------------------------------------Empty folder list-------------------------------------------------\n").unwrap();
            file.write_all(("Found ".to_string() + self.information.number_of_empty_folders.to_string().as_str() + " empty folders which in " + ".\n").as_bytes())
                .unwrap();
            for name in self.empty_folder_list.keys() {
                file.write_all((name.clone() + "\n").as_bytes()).unwrap();
            }
        } else {
            file.write_all(b"Not found any empty folders.").unwrap();
        }
        Common::print_time(start_time, SystemTime::now(), "save_results_to_file".to_string());
        true
    }
}
impl PrintResults for EmptyFolder {
    /// Prints basic info about empty folders // TODO print better
    fn print_results(&self) {
        if !self.empty_folder_list.is_empty() {
            println!("Found {} empty folders", self.empty_folder_list.len());
        }
        for i in &self.empty_folder_list {
            println!("{}", i.0);
        }
    }
}
