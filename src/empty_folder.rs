use crate::common::Common;
use std::collections::HashMap;
use std::fs::Metadata;
use std::path::Path;
use std::time::SystemTime;
use std::{fs, process};

#[derive(Eq, PartialEq, Copy, Clone)]
enum FolderEmptiness {
    No,
    Maybe,
}

#[derive(Clone)]
struct FolderEntry {
    parent_path: Option<String>,
    is_empty: FolderEmptiness,
}

pub struct EmptyFolder {
    number_of_checked_folders: usize,
    number_of_empty_folders: usize,
    empty_folder_list: HashMap<String, FolderEntry>,
    excluded_directories: Vec<String>,
    included_directories: Vec<String>,
}

impl EmptyFolder {
    pub fn new() -> EmptyFolder {
        EmptyFolder {
            number_of_checked_folders: 0,
            number_of_empty_folders: 0,
            empty_folder_list: Default::default(),
            excluded_directories: vec![],
            included_directories: vec![],
        }
    }

    pub fn find_empty_folders(mut self, delete_folders: bool) {
        self.optimize_directories();
        self.debug_print();
        self.check_for_empty_folders(true);
        self.check_for_empty_folders(false); // Not needed for CLI, but it is better to check this
        self.optimize_folders();
        self.print_empty_folders();
        if delete_folders {
            self.delete_empty_folders();
        }
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
    }

    /// Function to check if folder are empty, initial_checking is used to check again if folder is
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
            // Add root folders for finding
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
            current_folder = folders_to_check.pop().unwrap();

            let read_dir = match fs::read_dir(&current_folder) {
                Ok(t) => t,
                _ => {
                    folders_checked.get_mut(&current_folder).unwrap().is_empty = FolderEmptiness::No;
                    continue;
                }
            };
            for entry in read_dir {
                let entry_data = entry.unwrap();
                let metadata: Metadata = entry_data.metadata().unwrap();
                if metadata.is_dir() {
                    let mut is_excluded_dir = false;
                    next_folder = "".to_owned() + &current_folder + &entry_data.file_name().into_string().unwrap() + "/";
                    for ed in &self.excluded_directories {
                        if next_folder == *ed {
                            is_excluded_dir = true;
                            break;
                        }
                    }
                    if !is_excluded_dir {
                        folders_to_check.push(next_folder.clone());

                        folders_checked.insert(
                            next_folder.clone(),
                            FolderEntry {
                                parent_path: Option::from(current_folder.clone()),
                                is_empty: FolderEmptiness::Maybe,
                            },
                        );
                    }
                } else {
                    // Not folder so it may be a file or symbolic link
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
            for entry in folders_checked {
                if entry.1.is_empty != FolderEmptiness::No {
                    self.empty_folder_list.insert(entry.0, entry.1);
                }
            }
        } else {
            // Sprawdzenie
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

    fn delete_empty_folders(&self) {
        let start_time: SystemTime = SystemTime::now();
        let mut errors: Vec<String> = Vec::new();
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

    fn print_empty_folders(&self) {
        if !self.empty_folder_list.is_empty() {
            println!("Found {} empty folders", self.empty_folder_list.len());
        }
        for i in &self.empty_folder_list {
            println!("{}", i.0);
        }
    }

    fn debug_print(&self) {
        if false {
            println!("---------------DEBUG PRINT---------------");
            println!("Number of all checked folders - {}", self.number_of_checked_folders);
            println!("Number of empty folders - {}", self.number_of_empty_folders);
            for i in &self.empty_folder_list {
                println!("# {} ", i.0.clone());
            }
            println!("Excluded directories - {:?}", self.excluded_directories);
            println!("Included directories - {:?}", self.included_directories);
            println!("-----------------------------------------");
        }
    }

    // TODO maybe move this and one from duplicated finder to one common class to avoid duplicating code
    fn optimize_directories(&mut self) {
        let start_time: SystemTime = SystemTime::now();

        let mut optimized_included: Vec<String> = Vec::<String>::new();
        let mut optimized_excluded: Vec<String> = Vec::<String>::new();
        // Remove duplicated entries like: "/", "/"

        self.excluded_directories.sort();
        self.included_directories.sort();

        self.excluded_directories.dedup();
        self.included_directories.dedup();

        // Optimize for duplicated included directories - "/", "/home". "/home/Pulpit" to "/"
        let mut is_inside: bool;
        for ed_checked in &self.excluded_directories {
            is_inside = false;
            for ed_help in &self.excluded_directories {
                if ed_checked == ed_help {
                    // We checking same element
                    continue;
                }
                if ed_checked.starts_with(ed_help) {
                    is_inside = true;
                    break;
                }
            }
            if !is_inside {
                optimized_excluded.push(ed_checked.to_string());
            }
        }

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
        self.excluded_directories = optimized_excluded;
        optimized_excluded = Vec::<String>::new();

        // Remove include directories which are inside any exclude directory
        for id in &self.included_directories {
            let mut is_inside: bool = false;
            for ed in &self.excluded_directories {
                if id.starts_with(ed) {
                    is_inside = true;
                    break;
                }
            }
            if !is_inside {
                optimized_included.push(id.to_string());
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

        for ed in &self.excluded_directories {
            let path = Path::new(ed);
            if path.exists() {
                optimized_excluded.push(ed.to_string());
            }
        }

        self.included_directories = optimized_included;
        // optimized_included = Vec::<String>::new();
        self.excluded_directories = optimized_excluded;
        optimized_excluded = Vec::<String>::new();

        // Excluded paths must are inside include path, because
        for ed in &self.excluded_directories {
            let mut is_inside: bool = false;
            for id in &self.included_directories {
                if ed.starts_with(id) {
                    is_inside = true;
                    break;
                }
            }
            if is_inside {
                optimized_excluded.push(ed.to_string());
            }
        }

        self.excluded_directories = optimized_excluded;
        // optimized_excluded = Vec::<String>::new();

        if self.included_directories.is_empty() {
            println!("Optimize Directories ERROR: Excluded directories overlaps all included directories.");
            process::exit(1);
        }

        // Not needed, but better is to have sorted everything
        self.excluded_directories.sort();
        self.included_directories.sort();
        Common::print_time(start_time, SystemTime::now(), "optimize_directories".to_string());
    }
    pub fn set_include_directory(&mut self, mut include_directory: String) {
        // let start_time: SystemTime = SystemTime::now();

        if include_directory.is_empty() {
            println!("At least one directory must be provided");
        }

        include_directory = include_directory.replace("\"", "");
        let directories: Vec<String> = include_directory.split(',').map(String::from).collect();
        let mut checked_directories: Vec<String> = Vec::new();

        for directory in directories {
            let directory : String = directory.trim().to_string();

            if directory == "" {
                continue
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
            if !Path::new(&directory).exists() {
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

        //Common::print_time(start_time, SystemTime::now(), "set_include_directory".to_string());
    }

    pub fn set_exclude_directory(&mut self, mut exclude_directory: String) {
        //let start_time: SystemTime = SystemTime::now();
        if exclude_directory.is_empty() {
            return;
        }

        exclude_directory = exclude_directory.replace("\"", "");
        let directories: Vec<String> = exclude_directory.split(',').map(String::from).collect();
        let mut checked_directories: Vec<String> = Vec::new();

        for directory in directories {
            let directory : String = directory.trim().to_string();

            if directory == "" {
                continue
            }
            if directory == "/" {
                println!("Exclude Directory ERROR: Excluding / is pointless, because it means that no files will be scanned.");
                break;
            }
            if directory.contains('*') {
                println!("Exclude Directory ERROR: Wildcards are not supported, ignoring path {}.", directory);
                continue;
            }
            if directory.starts_with('~') {
                println!("Exclude Directory ERROR: ~ in path isn't supported, ignoring path {}.", directory);
                continue;
            }
            if !directory.starts_with('/') {
                println!("Exclude Directory ERROR: Relative path are not supported, ignoring path {}.", directory);
                continue;
            }
            if !Path::new(&directory).exists() {
                println!("Exclude Directory ERROR: Path {} doesn't exists.", directory);
                continue;
            }
            if !Path::new(&directory).exists() {
                println!("Exclude Directory ERROR: {} isn't folder.", directory);
                continue;
            }

            // directory must end with /, due to possiblity of incorrect assumption, that e.g. /home/rafal is top folder to /home/rafalinho
            if !directory.ends_with('/') {
                checked_directories.push(directory.trim().to_string() + "/");
            } else {
                checked_directories.push(directory.trim().to_string());
            }
        }
        self.excluded_directories = checked_directories;

        //Common::print_time(start_time, SystemTime::now(), "set_exclude_directory".to_string());
    }
}
