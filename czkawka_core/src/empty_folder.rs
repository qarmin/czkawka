use crate::common::{Common, Messages};
use std::collections::HashMap;
use std::fs::{File, Metadata};
use std::io::Write;
use std::path::Path;
use std::time::SystemTime;
use std::{fs, process};

/// Enum with values which show if folder is empty.
/// In function "optimize_folders" automatically "Maybe" is changed to "Yes", so it is not necessery to put it here
#[derive(Eq, PartialEq, Copy, Clone)]
enum FolderEmptiness {
    No,
    Maybe,
}

/// Struct assigned to each checked folder with parent path(used to ignore parent if children are not empty) and flag which shows if folder is empty
#[derive(Clone)]
struct FolderEntry {
    parent_path: Option<String>,
    is_empty: FolderEmptiness,
}

/// Struct to store most basics info about  all folder
pub struct EmptyFolder {
    information: Info,
    delete_folders: bool,
    text_messages: Messages,
    empty_folder_list: HashMap<String, FolderEntry>, // Path, FolderEntry
    included_directories: Vec<String>,
}

/// Info struck with helpful information's about results
pub struct Info {
    number_of_checked_folders: usize,
    number_of_empty_folders: usize,
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
            text_messages: Default::default(),
            empty_folder_list: Default::default(),
            included_directories: vec![],
        }
    }

    pub fn get_text_messages(&self) -> &Messages {
        &self.text_messages
    }

    /// Public function used by CLI to search for empty folders
    pub fn find_empty_folders(&mut self) {
        self.optimize_directories();
        self.check_for_empty_folders(true);
        self.check_for_empty_folders(false); // Not needed for CLI, but it is better to check this
        self.optimize_folders();
        if self.delete_folders {
            self.delete_empty_folders();
        }
        self.debug_print();
    }

    pub fn set_delete_folder(&mut self, delete_folder: bool) {
        self.delete_folders = delete_folder;
    }

    pub fn save_results_to_file(&mut self, file_name: &str) -> bool {
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

        match file.write_all(format!("Results of searching in {:?}\n", self.included_directories).as_bytes()) {
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

    /// Clean directory tree
    /// If directory contains only 2 empty folders, then this directory should be removed instead two empty folders inside because it will produce another empty folder.
    fn optimize_folders(&mut self) {
        let mut new_directory_folders: HashMap<String, FolderEntry> = Default::default();

        for entry in &self.empty_folder_list {
            match &entry.1.parent_path {
                Some(t) => {
                    if !self.empty_folder_list.contains_key(t) {
                        new_directory_folders.insert(entry.0.clone(), entry.1.clone());
                    }
                }
                None => {
                    new_directory_folders.insert(entry.0.clone(), entry.1.clone());
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
        let mut folders_checked: HashMap<String, FolderEntry> = Default::default();

        if initial_checking {
            // Add root folders for finding
            for id in &self.included_directories {
                folders_checked.insert(
                    id.clone(),
                    FolderEntry {
                        parent_path: None,
                        is_empty: FolderEmptiness::Maybe,
                    },
                );
                folders_to_check.push(id.clone());
            }
        } else {
            // Add folders searched before
            for id in &self.empty_folder_list {
                folders_checked.insert(
                    id.0.clone(),
                    FolderEntry {
                        parent_path: None,
                        is_empty: FolderEmptiness::Maybe,
                    },
                );
                folders_to_check.push(id.0.clone());
            }
        }

        let mut current_folder: String;
        let mut next_folder: String;
        while !folders_to_check.is_empty() {
            self.information.number_of_checked_folders += 1;
            current_folder = folders_to_check.pop().unwrap();
            // Checked folder may be deleted so we assume that cannot removed folder be empty
            let read_dir = match fs::read_dir(&current_folder) {
                Ok(t) => t,
                _ => {
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
                        },
                    );
                } else {
                    // Not folder so it may be a file or symbolic link so it isn't empty
                    folders_checked.get_mut(&current_folder).unwrap().is_empty = FolderEmptiness::No;
                    let mut d = folders_checked.get_mut(&current_folder).unwrap();
                    let mut cf: String;
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
        if initial_checking {
            // We need to set empty folder list
            for entry in folders_checked {
                if entry.1.is_empty != FolderEmptiness::No {
                    self.empty_folder_list.insert(entry.0, entry.1);
                }
            }
        } else {
            // We need to check if parent of folder isn't also empty, because we wan't to delete only parent with two empty folders except this folders and at the end parent folder
            let mut new_folders_list: HashMap<String, FolderEntry> = Default::default();
            for entry in folders_checked {
                if entry.1.is_empty != FolderEmptiness::No && self.empty_folder_list.contains_key(&entry.0) {
                    new_folders_list.insert(entry.0, entry.1);
                }
            }
            self.empty_folder_list = new_folders_list;
        }

        Common::print_time(start_time, SystemTime::now(), "check_for_empty_folder".to_string());
    }

    /// Deletes earlier found empty folders
    fn delete_empty_folders(&self) {
        let start_time: SystemTime = SystemTime::now();
        let mut errors: Vec<String> = Vec::new();
        // Folders may be deleted or require too big privileges
        for entry in &self.empty_folder_list {
            match fs::remove_dir_all(entry.0) {
                Ok(_) => (),
                Err(_) => errors.push(entry.0.clone()),
            };
        }

        if !errors.is_empty() {
            println!("Failed to delete some files, because they have got deleted earlier or you have too low privileges - try run it as root.");
            println!("List of files which wasn't deleted:");
        }
        for i in errors {
            println!("{}", i);
        }
        Common::print_time(start_time, SystemTime::now(), "delete_files".to_string());
    }

    /// Prints basic info about empty folders // TODO print better
    pub fn print_empty_folders(&self) {
        if !self.empty_folder_list.is_empty() {
            println!("Found {} empty folders", self.empty_folder_list.len());
        }
        for i in &self.empty_folder_list {
            println!("{}", i.0);
        }
    }

    /// Debug print
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
        println!("Included directories - {:?}", self.included_directories);
        println!("-----------------------------------------");
    }

    // TODO maybe move this and one from duplicated finder to one common class to avoid duplicating code
    /// Optimize include and exclude directories by removing duplicates etc.
    fn optimize_directories(&mut self) {
        let start_time: SystemTime = SystemTime::now();

        let mut optimized_included: Vec<String> = Vec::<String>::new();
        // Remove duplicated entries like: "/", "/"

        self.included_directories.sort();

        self.included_directories.dedup();

        // Optimize for duplicated included directories - "/", "/home". "/home/Pulpit" to "/"

        let mut is_inside: bool;
        for id_checked in &self.included_directories {
            is_inside = false;
            for id_help in &self.included_directories {
                if id_checked == id_help {
                    // We checking same element
                    continue;
                }
                if id_checked.starts_with(id_help) {
                    is_inside = true;
                    break;
                }
            }
            if !is_inside {
                optimized_included.push(id_checked.to_string());
            }
        }

        self.included_directories = optimized_included;
        optimized_included = Vec::<String>::new();

        // Remove non existed directories
        for id in &self.included_directories {
            let path = Path::new(id);
            if path.exists() {
                optimized_included.push(id.to_string());
            }
        }

        self.included_directories = optimized_included;
        //optimized_included = Vec::<String>::new();

        if self.included_directories.is_empty() {
            println!("Optimize Directories ERROR: Excluded directories overlaps all included directories.");
            process::exit(1);
        }

        // Not needed, but better is to have sorted everything
        self.included_directories.sort();
        Common::print_time(start_time, SystemTime::now(), "optimize_directories".to_string());
    }

    /// Set include dir which needs to be relative, exists,
    pub fn set_include_directory(&mut self, mut include_directory: String) {
        let start_time: SystemTime = SystemTime::now();

        if include_directory.is_empty() {
            println!("At least one directory must be provided");
        }

        include_directory = include_directory.replace("\"", "");
        let directories: Vec<String> = include_directory.split(',').map(String::from).collect();
        let mut checked_directories: Vec<String> = Vec::new();

        for directory in directories {
            let directory: String = directory.trim().to_string();

            if directory == "" {
                continue;
            }
            if directory == "/" {
                println!("Using / is probably not good idea, you may go out of ram.");
            }
            if directory.contains('*') {
                println!("Include Directory ERROR: Wildcards are not supported, ignoring path {}.", directory);
                continue;
            }
            if directory.starts_with('~') {
                println!("Include Directory ERROR: ~ in path isn't supported, ignoring path {}.", directory);
                continue;
            }
            if !directory.starts_with('/') {
                println!("Include Directory ERROR: Relative path are not supported, ignoring path {}.", directory);
                continue;
            }
            if !Path::new(&directory).exists() {
                println!("Include Directory ERROR: Path {} doesn't exists.", directory);
                continue;
            }
            if !Path::new(&directory).is_dir() {
                println!("Include Directory ERROR: {} isn't folder.", directory);
                continue;
            }

            // directory must end with /, due to possiblity of incorrect assumption, that e.g. /home/rafal is top folder to /home/rafalinho
            if !directory.ends_with('/') {
                checked_directories.push(directory + "/");
            } else {
                checked_directories.push(directory);
            }
        }

        if checked_directories.is_empty() {
            println!("Not found even one correct path to include.");
            process::exit(1);
        }

        self.included_directories = checked_directories;

        Common::print_time(start_time, SystemTime::now(), "set_include_directory".to_string());
    }
}
impl Default for EmptyFolder {
    fn default() -> Self {
        Self::new()
    }
}
