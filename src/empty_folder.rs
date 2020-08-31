use crate::common::Common;
use std::collections::HashMap;
use std::fs::Metadata;
use std::path::Path;
use std::time::SystemTime;
use std::{fs, process};

enum FolderEmptiness {
    Yes,
    No,
    Maybe,
}
struct FolderEntry {
    self_path: String,
    parent_path: Option<String>,
    is_empty: FolderEmptiness,
}
impl FolderEntry {}

pub struct EmptyFolder {
    number_of_checked_folders: usize,
    number_of_empty_folders: usize,
    empty_folder_list: Vec<String>,
    excluded_directories: Vec<String>,
    included_directories: Vec<String>,
}

impl EmptyFolder {
    pub fn new() -> EmptyFolder {
        EmptyFolder {
            number_of_checked_folders: 0,
            number_of_empty_folders: 0,
            empty_folder_list: Vec::new(),
            excluded_directories: vec![],
            included_directories: vec![],
        }
    }

    pub fn find_empty_folders(mut self, delete_folders: bool) {
        self.optimize_directories();
        self.debug_print();
        self.check_for_empty_folders();
        self.print_empty_folders();
        if delete_folders {
            self.delete_empty_folders();
        }
    }
    fn check_for_empty_folders(&self) {
        let start_time: SystemTime = SystemTime::now();
        let mut folders_to_check: Vec<String> = Vec::with_capacity(1024 * 2); // This should be small enough too not see to big difference and big enough to store most of paths without needing to resize vector
        let mut folders_checked: HashMap<String, FolderEntry> = Default::default();

        // Add root folders for finding
        for id in &self.included_directories {
            folders_checked.insert(
                id.clone(),
                FolderEntry {
                    self_path: id.clone(),
                    parent_path: None,
                    is_empty: FolderEmptiness::Maybe,
                },
            );
            folders_to_check.push(id.to_string());
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
                                self_path: next_folder,
                                parent_path: Option::from(current_folder.clone()),
                                is_empty: FolderEmptiness::Maybe,
                            },
                        );
                    }
                } else {
                    // Not folder so it may be a file or symbolic link
                    folders_checked.get_mut(&current_folder).unwrap().is_empty = FolderEmptiness::No;
                    let mut d = folders_checked.get_mut(&current_folder).unwrap();
                    let mut cf: String = current_folder.clone();
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
        self.debug_print();
        Common::print_time(start_time, SystemTime::now(), "check_files_size".to_string());
    }
    fn delete_empty_folders(&self) {
        // Need to check again because folder may stop be empty
    }
    fn print_empty_folders(&self) {
        if !self.empty_folder_list.is_empty() {
            println!("Found {} empty folders", self.empty_folder_list.len());
        }
        for i in &self.empty_folder_list {
            println!("{}", i);
        }
    }

    fn debug_print(&self) {
        println!("---------------DEBUG PRINT---------------");
        println!("Number of all checked folders - {}", self.number_of_checked_folders);
        println!("Number of empty folders - {}", self.number_of_empty_folders);
        for i in &self.empty_folder_list {
            println!("# {} ", i.clone());
        }
        println!("Excluded directories - {:?}", self.excluded_directories);
        println!("Included directories - {:?}", self.included_directories);
        println!("-----------------------------------------");
    }

    // TODO maybe move this and one from  duplicated finder to one common class to avoid duplicating code
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
            if directory == "/" {
                println!("Using / is probably not good idea, you may go out of ram.");
            }
            if directory.contains('*') {
                println!("Include Directory ERROR: Wildcards are not supported, please don't use it.");
                process::exit(1);
            }
            if directory.starts_with('~') {
                println!("Include Directory ERROR: ~ in path isn't supported.");
                process::exit(1);
            }
            if !directory.starts_with('/') {
                println!("Include Directory ERROR: Relative path are not supported.");
                process::exit(1);
            }
            if !Path::new(&directory).exists() {
                println!("Include Directory ERROR: Path {} doesn't exists.", directory);
                process::exit(1);
            }
            if !Path::new(&directory).exists() {
                println!("Include Directory ERROR: {} isn't folder.", directory);
                process::exit(1);
            }

            // directory must end with /, due to possiblity of incorrect assumption, that e.g. /home/rafal is top folder to /home/rafalinho
            if !directory.ends_with('/') {
                checked_directories.push(directory.trim().to_string() + "/");
            } else {
                checked_directories.push(directory.trim().to_string());
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
            if directory == "/" {
                println!("Exclude Directory ERROR: Excluding / is pointless, because it means that no files will be scanned.");
            }
            if directory.contains('*') {
                println!("Exclude Directory ERROR: Wildcards are not supported, please don't use it.");
                process::exit(1);
            }
            if directory.starts_with('~') {
                println!("Exclude Directory ERROR: ~ in path isn't supported.");
                process::exit(1);
            }
            if !directory.starts_with('/') {
                println!("Exclude Directory ERROR: Relative path are not supported.");
                process::exit(1);
            }
            if !Path::new(&directory).exists() {
                println!("Exclude Directory WARNING: Path {} doesn't exists.", directory);
                //process::exit(1); // Better just print warning witohut closing
            }
            if !Path::new(&directory).exists() {
                println!("Exclude Directory ERROR: {} isn't folder.", directory);
                process::exit(1);
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
