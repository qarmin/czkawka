// Todo, należy upewnić się, że ma wystarczające uprawnienia do odczytu i usuwania
use std::collections::HashMap;
use std::fs::Metadata;
use std::path::Path;
use std::time::SystemTime;
use std::{fs, process};

pub struct DuplicateFinder {
    number_of_checked_files: u64,
    number_of_checked_folders: u64,
    number_of_ignored_things: u64,
    number_of_files_which_has_duplicated_entries: u64,
    number_of_duplicated_files: u64,
    // files : Vec<HashMap<FileEntry, Vec<FileEntry>>>,
    files: HashMap<u64, FileEntry>,
    files_with_duplicated_entries: HashMap<u64, FileEntry>,
    // duplicated_entries // Same as files, but only with 2+ entries
    // files : Vec<Vec<FileEntry>>,
    excluded_directories: Vec<String>,
    included_directories: Vec<String>,
}

impl DuplicateFinder {
    pub fn new() -> DuplicateFinder {
        DuplicateFinder {
            number_of_checked_files: 0,
            number_of_checked_folders: 0,
            number_of_ignored_things: 0,
            number_of_files_which_has_duplicated_entries: 0,
            number_of_duplicated_files: 0,
            files: Default::default(),
            files_with_duplicated_entries: Default::default(),
            excluded_directories: vec![],
            included_directories: vec![],
        }
    }
    // pub fn clear(&mut self) {
    //     self.number_of_checked_files = 0;
    //     self.number_of_checked_folders = 0;
    //     self.number_of_ignored_things = 0;
    //     self.number_of_files_which_has_duplicated_entries = 0;
    //     self.number_of_duplicated_files = 0;
    //     self.files.clear();
    //     self.excluded_directories.clear();
    //     self.included_directories.clear();
    // }
    pub fn find_duplicates(&mut self) {
        //let mut path;
        let start_time: SystemTime = SystemTime::now();
        let mut folders_to_check: Vec<String> = Vec::with_capacity(1024 * 16); // This should be small enough too not see to big difference and big enough to store most of paths without needing to resize vector

        // Add root folders for finding
        for id in &self.included_directories {
            folders_to_check.push(id.to_string());
        }

        let mut current_folder: String;
        let mut next_folder: String;
        while !folders_to_check.is_empty() {
            current_folder = folders_to_check.pop().unwrap();

            let read_dir = fs::read_dir(&current_folder);
            let read_dir = match read_dir {
                Ok(t) => t,
                _ => continue,
            };
            for entry in read_dir {
                let entry_data = entry.unwrap();
                let metadata: Metadata = entry_data.metadata().unwrap();
                if metadata.is_dir() {
                    let mut is_excluded_dir = false;
                    next_folder = "".to_owned() + &current_folder + &entry_data.file_name().into_string().unwrap() + "/";
                    for ed in &self.excluded_directories {
                        if next_folder == ed.to_string() {
                            is_excluded_dir = true;
                            break;
                        }
                    }
                    if !is_excluded_dir {
                        folders_to_check.push(next_folder);
                    }
                    self.number_of_checked_folders += 1;

                //println!("Directory\t - {:?}", next_folder); // DEBUG
                } else if metadata.is_file() {
                    let current_file_name = "".to_owned() + &current_folder + &entry_data.file_name().into_string().unwrap();
                    //file_to_check
                    let fe: FileEntry = FileEntry {
                        path: current_file_name,
                        size: metadata.len(),
                        created_date: metadata.created().unwrap(),
                        modified_date: metadata.modified().unwrap(),
                    };
                    self.files.insert(metadata.len(), fe);

                    self.number_of_checked_files += 1;
                // println!("File\t\t - {:?}", current_file); // DEBUG
                } else {
                    // Probably this is symbolic links so we are free to ignore this
                    // println!("Found another type of file {} {:?}","".to_owned() + &current_folder + &entry_data.file_name().into_string().unwrap(), metadata) //DEBUG
                    self.number_of_ignored_things += 1;
                }
            }
        }
        self.debug_print();
        let end_time: SystemTime = SystemTime::now();
        println!("Duration of finding duplicates {:?}", end_time.duration_since(start_time).expect("a"));
    }
    // pub fn save_to_file(&self) {}

    /// Setting include directories, panics when there is not directories available
    pub fn set_include_directory(&mut self, mut include_directory: String) {
        if include_directory.len() == 0 {
            println!("At least one directory must be provided")
        }

        include_directory = include_directory.replace("\"", "");
        let directories: Vec<String> = include_directory.split(",").map(String::from).collect();
        let mut checked_directories: Vec<String> = Vec::new();

        for directory in directories {
            if directory == "/" {
                println!("Using / is probably not good idea, you may go out of ram.");
            }
            if directory.contains("*") {
                println!("Include Directory ERROR: Wildcards are not supported, please don't use it.");
                process::exit(1);
            }
            if directory.starts_with("~") {
                println!("Include Directory ERROR: ~ in path isn't supported.");
                process::exit(1);
            }
            if !directory.starts_with("/") {
                println!("Include Directory ERROR: Relative path are not supported.");
                process::exit(1);
            }
            if !Path::new(&directory).exists() {
                println!("Include Directory ERROR: Path {} doens't exists.", directory);
                process::exit(1);
            }
            if !Path::new(&directory).exists() {
                println!("Include Directory ERROR: {} isn't folder.", directory);
                process::exit(1);
            }

            // directory must end with /, due to possiblity of incorrect assumption, that e.g. /home/rafal is top folder to /home/rafalinho
            if !directory.ends_with("/") {
                checked_directories.push(directory + "/");
            } else {
                checked_directories.push(directory);
            }
        }

        if checked_directories.len() == 0 {
            println!("Not found even one correct path to include.");
            process::exit(1);
        }

        self.included_directories = checked_directories;

        println!("Included directories - {:?}", self.included_directories);
    }

    pub fn set_exclude_directory(&mut self, mut exclude_directory: String) {
        if exclude_directory.len() == 0 {
            return;
        }

        exclude_directory = exclude_directory.replace("\"", "");
        let directories: Vec<String> = exclude_directory.split(",").map(String::from).collect();
        let mut checked_directories: Vec<String> = Vec::new();

        for directory in directories {
            if directory == "/" {
                println!("Exclude Directory ERROR: Excluding / is pointless, because it means that no files will be scanned.");
            }
            if directory.contains("*") {
                println!("Exclude Directory ERROR: Wildcards are not supported, please don't use it.");
                process::exit(1);
            }
            if directory.starts_with("~") {
                println!("Exclude Directory ERROR: ~ in path isn't supported.");
                process::exit(1);
            }
            if !directory.starts_with("/") {
                println!("Exclude Directory ERROR: Relative path are not supported.");
                process::exit(1);
            }
            if !Path::new(&directory).exists() {
                println!("Exclude Directory ERROR: Path {} doens't exists.", directory);
                process::exit(1);
            }
            if !Path::new(&directory).exists() {
                println!("Exclude Directory ERROR: {} isn't folder.", directory);
                process::exit(1);
            }

            // directory must end with /, due to possiblity of incorrect assumption, that e.g. /home/rafal is top folder to /home/rafalinho
            if !directory.ends_with("/") {
                checked_directories.push(directory + "/");
            } else {
                checked_directories.push(directory);
            }
        }

        self.excluded_directories = checked_directories;

        println!("Excluded directories - {:?}", &self.excluded_directories);
    }

    pub fn debug_print(&self) {
        println!("---------------DEBUG PRINT---------------");
        println!("Number of all checked files - {}", self.number_of_checked_files);
        println!("Number of all checked folders - {}", self.number_of_checked_folders);
        println!("Number of all ignored things - {}", self.number_of_ignored_things);
        println!("Number of all files with duplicates - {}", self.number_of_files_which_has_duplicated_entries);
        println!("Number of duplicated files - {}", self.number_of_duplicated_files);
        println!("Files list - {}", self.files.len());
        println!("Excluded directories - {:?}", self.excluded_directories);
        println!("Included directories - {:?}", self.included_directories);
        println!("-----------------------------------------");
    }
    /// Remove unused entries when included or excluded overlaps with each other or are duplicated
    /// ```
    /// let df : DuplicateFinder = saf
    /// ```
    pub fn optimize_directories(&mut self) {
        let mut optimized_included: Vec<String> = Vec::<String>::new();
        let mut optimized_excluded: Vec<String> = Vec::<String>::new();
        // Remove duplicated entries like: "/", "/"

        self.excluded_directories.sort();
        self.included_directories.sort();

        self.excluded_directories.dedup();
        self.included_directories.dedup();

        // Optimize for duplicated included directories - "/", "/home". "/home/Pulpit" to "/"- TODO
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
            if is_inside == false {
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
            if is_inside == false {
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

        // Excluded paths must are inside include path, because  TODO
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

        if self.included_directories.len() == 0 {
            println!("Optimize Directories ERROR: Excluded directories overlaps all included directories.");
            process::exit(1);
        }

        // Not needed, but better is to have sorted everything
        self.excluded_directories.sort();
        self.included_directories.sort();
    }
}

struct FileEntry {
    pub path: String,
    pub size: u64,
    pub created_date: SystemTime,
    pub modified_date: SystemTime,
}
