use humansize::{file_size_opts as options, FileSize};
use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::fs::{File, Metadata};
use std::io::prelude::*;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::common::Common;

#[derive(PartialEq)]
pub enum CheckingMethod {
    SIZE,
    HASH,
}

#[derive(Eq, PartialEq)]
pub enum DeleteMethod {
    None,
    AllExceptNewest,
    AllExceptOldest,
    OneOldest,
    OneNewest,
}

#[derive(Clone)]
struct FileEntry {
    pub path: String,
    pub size: u64,
    pub created_date: SystemTime,
    pub modified_date: SystemTime,
}

/// Struct with required information's to work
pub struct DuplicateFinder {
    infos: Info,
    files_with_identical_size: HashMap<u64, Vec<FileEntry>>,
    files_with_identical_hashes: BTreeMap<u64, Vec<Vec<FileEntry>>>,
    allowed_extensions: Vec<String>, // jpg, jpeg, mp4
    excluded_items: Vec<String>,     // TODO, support for e.g. */.git/*
    excluded_directories: Vec<String>,
    included_directories: Vec<String>,
    min_file_size: u64,
}

/// Info struck with helpful information's about results
pub struct Info {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub messages: Vec<String>,
    pub number_of_checked_files: usize,
    pub number_of_checked_folders: usize,
    pub number_of_ignored_files: usize,
    pub number_of_ignored_things: usize,
    pub number_of_duplicated_files: usize,
    pub lost_space: u64,
    pub number_of_removed_files: usize,
    pub number_of_failed_to_remove_files: usize,
    pub gained_space: u64,
}
impl Info {
    pub fn new() -> Info {
        Info {
            errors: vec![],
            warnings: vec![],
            messages: vec![],
            number_of_checked_files: 0,
            number_of_ignored_files: 0,
            number_of_checked_folders: 0,
            number_of_ignored_things: 0,
            number_of_duplicated_files: 0,
            lost_space: 0,
            number_of_removed_files: 0,
            number_of_failed_to_remove_files: 0,
            gained_space: 0,
        }
    }
}

impl Default for Info {
    fn default() -> Self {
        Self::new()
    }
}

impl DuplicateFinder {
    pub fn new() -> DuplicateFinder {
        DuplicateFinder {
            infos: Info::new(),
            files_with_identical_size: Default::default(),
            files_with_identical_hashes: Default::default(),
            excluded_items: vec![],
            excluded_directories: vec![],
            included_directories: vec![],
            min_file_size: 1024,
            allowed_extensions: vec![],
        }
    }
    pub fn get_infos(&self) -> &Info {
        &self.infos
    }

    pub fn find_duplicates(&mut self, check_method: &CheckingMethod, delete_method: &DeleteMethod) {
        self.optimize_directories();
        self.check_files_size();
        self.remove_files_with_unique_size();
        if *check_method == CheckingMethod::HASH {
            self.check_files_hash();
        }
        self.calculate_lost_space(check_method);
        self.delete_files(check_method, delete_method);
        self.debug_print();
    }

    pub fn set_min_file_size(&mut self, min_size: u64) {
        self.min_file_size = min_size;
    }

    pub fn set_excluded_items(&mut self, mut excluded_items: String) {
        // let start_time: SystemTime = SystemTime::now();

        if excluded_items.is_empty() {
            return;
        }

        excluded_items = excluded_items.replace("\"", "");
        let expressions: Vec<String> = excluded_items.split(',').map(String::from).collect();
        let mut checked_expressions: Vec<String> = Vec::new();

        for expression in expressions {
            let expression: String = expression.trim().to_string();

            if expression == "" {
                continue;
            }
            if !expression.contains('*') {
                self.infos.warnings.push("Excluded Items Warning: Wildcard * is required in expression, ignoring ".to_string() + &*expression);
                continue;
            }

            checked_expressions.push(expression);
        }

        self.excluded_items = checked_expressions;
    }
    pub fn set_allowed_extensions(&mut self, mut allowed_extensions: String) {
        if allowed_extensions.is_empty() {
            self.infos.messages.push("No allowed extension was provided, so all are allowed".to_string());
            return;
        }
        allowed_extensions = allowed_extensions.replace("IMAGE", "jpg,kra,gif,png,bmp,tiff,webp,hdr,svg");
        allowed_extensions = allowed_extensions.replace("VIDEO", "mp4,flv,mkv,webm,vob,ogv,gifv,avi,mov,wmv,mpg,m4v,m4p,mpeg,3gp");
        allowed_extensions = allowed_extensions.replace("MUSIC", "mp3,flac,ogg,tta,wma,webm");
        allowed_extensions = allowed_extensions.replace("TEXT", "txt,doc,docx,odt,rtf");

        let extensions: Vec<String> = allowed_extensions.split(',').map(String::from).collect();
        for mut extension in extensions {
            if extension == "" {
                continue;
            }

            if extension.contains('.') {
                if !extension.starts_with('.') {
                    self.infos.warnings.push(extension + " is not valid extension(valid extension doesn't have dot inside)");
                    continue;
                }
                extension = extension.replace('.', "");
            }
            if !self.allowed_extensions.contains(&extension.trim().to_string()) {
                self.allowed_extensions.push(extension.trim().to_string());
            }
        }

        if self.allowed_extensions.is_empty() {
            self.infos.messages.push("No valid extensions were provided, so allowing all extensions by default.".to_string());
        }
    }
    pub fn set_include_directory(&mut self, mut include_directory: String) -> bool {
        // let start_time: SystemTime = SystemTime::now();

        if include_directory.is_empty() {
            self.infos.errors.push("At least one directory must be provided".to_string());
            return false;
        }

        include_directory = include_directory.replace("\"", "");
        let directories: Vec<String> = include_directory.split(',').map(String::from).collect();
        let mut checked_directories: Vec<String> = Vec::new();

        for directory in directories {
            let directory: String = directory.trim().to_string();

            if directory == "" {
                continue;
            }
            if directory.contains('*') {
                self.infos.warnings.push("Include Directory Warning: Wildcards in path are not supported, ignoring ".to_string() + &*directory);
                continue;
            }
            if !directory.starts_with('/') {
                self.infos.warnings.push("Include Directory Warning: Relative path are not supported, ignoring ".to_string() + &*directory);
                continue;
            }
            if !Path::new(&directory).exists() {
                self.infos.warnings.push("Include Directory Warning: Provided folder path must exits, ignoring ".to_string() + &*directory);
                continue;
            }
            if !Path::new(&directory).is_dir() {
                self.infos.warnings.push("Include Directory Warning: Provided path must point at the directory, ignoring ".to_string() + &*directory);
                continue;
            }

            // directory must end with /, due to possibility of incorrect assumption, that e.g. /home/rafal is top folder to /home/rafalinho
            if !directory.ends_with('/') {
                checked_directories.push(directory + "/");
            } else {
                checked_directories.push(directory);
            }
        }

        if checked_directories.is_empty() {
            self.infos.errors.push("Include Directory ERROR: Not found even one correct path to include which is required.".to_string());
            return false;
        }

        self.included_directories = checked_directories;

        //Common::print_time(start_time, SystemTime::now(), "set_include_directory".to_string());
        true
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
            let directory: String = directory.trim().to_string();

            if directory == "" {
                continue;
            }
            if directory == "/" {
                self.infos.errors.push("Exclude Directory ERROR: Excluding / is pointless, because it means that no files will be scanned.".to_string());
                break;
            }
            if directory.contains('*') {
                self.infos.warnings.push("Exclude Directory Warning: Wildcards in path are not supported, ignoring ".to_string() + &*directory);
                continue;
            }
            if !directory.starts_with('/') {
                self.infos.warnings.push("Exclude Directory Warning: Relative path are not supported, ignoring ".to_string() + &*directory);
                continue;
            }
            if !Path::new(&directory).exists() {
                self.infos.warnings.push("Exclude Directory Warning: Provided folder path must exits, ignoring ".to_string() + &*directory);
                continue;
            }
            if !Path::new(&directory).is_dir() {
                self.infos.warnings.push("Exclude Directory Warning: Provided path must point at the directory, ignoring ".to_string() + &*directory);
                continue;
            }

            // directory must end with /, due to possibility of incorrect assumption, that e.g. /home/rafal is top folder to /home/rafalinho
            if !directory.ends_with('/') {
                checked_directories.push(directory.trim().to_string() + "/");
            } else {
                checked_directories.push(directory.trim().to_string());
            }
        }
        self.excluded_directories = checked_directories;

        //Common::print_time(start_time, SystemTime::now(), "set_exclude_directory".to_string());
    }
    fn calculate_lost_space(&mut self, check_method: &CheckingMethod) {
        let mut bytes: u64 = 0;

        match check_method {
            CheckingMethod::SIZE => {
                for i in &self.files_with_identical_size {
                    bytes += i.0 * (i.1.len() as u64 - 1);
                }
            }
            CheckingMethod::HASH => {
                for i in &self.files_with_identical_hashes {
                    for j in i.1 {
                        bytes += i.0 * (j.len() as u64 - 1);
                    }
                }
            }
        }
        self.infos.lost_space = bytes;
    }

    fn check_files_size(&mut self) {
        // TODO maybe add multithreading checking for file hash
        let start_time: SystemTime = SystemTime::now();
        let mut folders_to_check: Vec<String> = Vec::with_capacity(1024 * 2); // This should be small enough too not see to big difference and big enough to store most of paths without needing to resize vector

        // Add root folders for finding
        for id in &self.included_directories {
            folders_to_check.push(id.to_string());
        }

        let mut current_folder: String;
        let mut next_folder: String;
        while !folders_to_check.is_empty() {
            current_folder = folders_to_check.pop().unwrap();

            let read_dir = match fs::read_dir(&current_folder) {
                Ok(t) => t,
                Err(_) => {
                    self.infos.warnings.push("Cannot open dir ".to_string() + &*current_folder);
                    continue;
                } // Permissions denied
            };
            for entry in read_dir {
                let entry_data = match entry {
                    Ok(t) => t,
                    Err(_) => {
                        self.infos.warnings.push("Cannot read entry in dir ".to_string() + &*current_folder);
                        continue;
                    } //Permissions denied
                };
                let metadata: Metadata = match entry_data.metadata() {
                    Ok(t) => t,
                    Err(_) => {
                        self.infos.warnings.push("Cannot read metadata in dir ".to_string() + &*current_folder);
                        continue;
                    } //Permissions denied
                };
                if metadata.is_dir() {
                    // if entry_data.file_name().into_string().is_err() { // Probably this can be removed, if crash still will be happens, then uncomment this line
                    //     self.infos.warnings.push("Cannot read folder name in dir ".to_string() + &*current_folder);
                    //     continue; // Permissions denied
                    // }

                    let mut is_excluded_dir = false;
                    next_folder = "".to_owned() + &current_folder + &entry_data.file_name().into_string().unwrap() + "/";

                    for ed in &self.excluded_directories {
                        if next_folder == *ed {
                            is_excluded_dir = true;
                            break;
                        }
                    }
                    if !is_excluded_dir {
                        let mut found_expression: bool = false;
                        for expression in &self.excluded_items {
                            if Common::regex_check(expression, &next_folder) {
                                found_expression = true;
                                break;
                            }
                        }
                        if found_expression {
                            break;
                        }
                        folders_to_check.push(next_folder);
                    }
                    self.infos.number_of_checked_folders += 1;
                } else if metadata.is_file() {
                    let mut have_valid_extension: bool;
                    let file_name_lowercase: String = entry_data.file_name().into_string().unwrap().to_lowercase();

                    // Checking allowed extensions
                    if !self.allowed_extensions.is_empty() {
                        have_valid_extension = false;
                        for i in &self.allowed_extensions {
                            if file_name_lowercase.ends_with(&(".".to_string() + &*i.to_lowercase().to_string())) {
                                have_valid_extension = true;
                                break;
                            }
                        }
                    } else {
                        have_valid_extension = true;
                    }

                    // Checking files
                    if metadata.len() >= self.min_file_size && have_valid_extension {
                        let current_file_name = "".to_owned() + &current_folder + &entry_data.file_name().into_string().unwrap();

                        // Checking expressions
                        let mut found_expression: bool = false;
                        for expression in &self.excluded_items {
                            if Common::regex_check(expression, &current_file_name) {
                                found_expression = true;
                                break;
                            }
                        }
                        if found_expression {
                            break;
                        }

                        // Creating new file entry
                        let fe: FileEntry = FileEntry {
                            path: current_file_name.clone(),
                            size: metadata.len(),
                            created_date: match metadata.created() {
                                Ok(t) => t,
                                Err(_) => {
                                    self.infos.warnings.push("Unable to get creation date from file ".to_string() + &*current_file_name);
                                    SystemTime::now()
                                } // Permissions Denied
                            },
                            modified_date: match metadata.modified() {
                                Ok(t) => t,
                                Err(_) => {
                                    self.infos.warnings.push("Unable to get modification date from file ".to_string() + &*current_file_name);
                                    SystemTime::now()
                                } // Permissions Denied
                            },
                        };

                        self.files_with_identical_size.entry(metadata.len()).or_insert_with(Vec::new);
                        self.files_with_identical_size.get_mut(&metadata.len()).unwrap().push(fe);

                        self.infos.number_of_checked_files += 1;
                    } else {
                        self.infos.number_of_ignored_files += 1;
                    }
                } else {
                    // Probably this is symbolic links so we are free to ignore this
                    self.infos.number_of_ignored_things += 1;
                }
            }
        }
        Common::print_time(start_time, SystemTime::now(), "check_files_size".to_string());
        //println!("Duration of finding duplicates {:?}", end_time.duration_since(start_time).expect("a"));
    }
    // pub fn save_results_to_file(&self) {} // TODO Saving results to files

    /// Remove files which have unique size
    fn remove_files_with_unique_size(&mut self) {
        let start_time: SystemTime = SystemTime::now();
        let mut new_hashmap: HashMap<u64, Vec<FileEntry>> = Default::default();

        self.infos.number_of_duplicated_files = 0;

        for entry in &self.files_with_identical_size {
            if entry.1.len() > 1 {
                self.infos.number_of_duplicated_files += entry.1.len() - 1;
                new_hashmap.insert(*entry.0, entry.1.clone());
            }
        }

        self.files_with_identical_size = new_hashmap;

        Common::print_time(start_time, SystemTime::now(), "remove_files_with_unique_size".to_string());
    }

    /// Should be slower than checking in different ways, but still needs to be checked
    fn check_files_hash(&mut self) {
        let start_time: SystemTime = SystemTime::now();
        let mut file_handler: File;
        let mut hashmap_with_hash: HashMap<String, Vec<FileEntry>>;

        for entry in &self.files_with_identical_size {
            hashmap_with_hash = Default::default();

            for file_entry in entry.1.iter().enumerate() {
                file_handler = match File::open(&file_entry.1.path) {
                    Ok(t) => t,
                    Err(_) => {
                        self.infos.warnings.push("Unable to check hash of file ".to_string() + &*file_entry.1.path);
                        continue;
                    }
                };

                let mut error_reading_file: bool = false;

                let mut hasher: blake3::Hasher = blake3::Hasher::new();
                let mut buffer = [0u8; 16384];
                loop {
                    let n = match file_handler.read(&mut buffer) {
                        Ok(t) => t,
                        Err(_) => {
                            self.infos.warnings.push("Error happened when checking hash of file ".to_string() + &*file_entry.1.path);
                            error_reading_file = true;
                            break;
                        }
                    }; //.unwrap();
                    if n == 0 {
                        break;
                    }
                    hasher.update(&buffer[..n]);
                }
                if !error_reading_file {
                    let hash_string: String = hasher.finalize().to_hex().to_string();
                    hashmap_with_hash.entry(hash_string.to_string()).or_insert_with(Vec::new);
                    hashmap_with_hash.get_mut(&*hash_string).unwrap().push(file_entry.1.to_owned());
                }
            }
            for hash_entry in hashmap_with_hash {
                if hash_entry.1.len() > 1 {
                    self.files_with_identical_hashes.entry(*entry.0).or_insert_with(Vec::new);
                    self.files_with_identical_hashes.get_mut(entry.0).unwrap().push(hash_entry.1);
                }
            }
        }
        Common::print_time(start_time, SystemTime::now(), "check_files_hash".to_string());
    }

    #[allow(dead_code)]
    /// Setting include directories, panics when there is not directories available
    fn debug_print(&self) {
        println!("---------------DEBUG PRINT---------------");
        println!("Number of checked files - {}", self.infos.number_of_checked_files);
        println!("Number of checked folders - {}", self.infos.number_of_checked_folders);
        println!("Number of ignored files - {}", self.infos.number_of_ignored_files);
        println!("Number of ignored things(like symbolic links) - {}", self.infos.number_of_ignored_things);
        println!("Number of duplicated files - {}", self.infos.number_of_duplicated_files);
        let mut file_size: u64 = 0;
        for i in &self.files_with_identical_size {
            file_size += i.1.len() as u64;
        }
        println!("Files list size - {} ({})", self.files_with_identical_size.len(), file_size);
        let mut hashed_file_size: u64 = 0;
        for i in &self.files_with_identical_hashes {
            for j in i.1 {
                hashed_file_size += j.len() as u64;
            }
        }
        println!("Hashed Files list size - {} ({})", self.files_with_identical_hashes.len(), hashed_file_size);
        println!("Number of removed files - {}", self.infos.number_of_removed_files);
        println!("Number of failed to remove files - {}", self.infos.number_of_failed_to_remove_files);
        println!("Lost space - {} ({} bytes)", self.infos.lost_space.file_size(options::BINARY).unwrap(), self.infos.lost_space);
        println!("Gained space by removing duplicated entries - {} ({} bytes)", self.infos.gained_space.file_size(options::BINARY).unwrap(), self.infos.gained_space);
        println!("Excluded directories - {:?}", self.excluded_directories);
        println!("Included directories - {:?}", self.included_directories);
        println!("-----------------------------------------");
    }

    /// Print information about duplicated entries
    pub fn print_duplicated_entries(&self, check_method: &CheckingMethod) {
        let start_time: SystemTime = SystemTime::now();
        let mut number_of_files: u64 = 0;
        let mut number_of_groups: u64 = 0;

        match check_method {
            CheckingMethod::HASH => {
                for (_size, vector) in self.files_with_identical_hashes.iter() {
                    for j in vector {
                        number_of_files += j.len() as u64;
                        number_of_groups += 1;
                    }
                }
                println!(
                    "Found {} duplicated files in {} groups with same content which took {}:",
                    number_of_files,
                    number_of_groups,
                    self.infos.lost_space.file_size(options::BINARY).unwrap()
                );
                for (key, vector) in self.files_with_identical_hashes.iter().rev() {
                    println!("Size - {}", key.file_size(options::BINARY).unwrap());
                    for j in vector {
                        for k in j {
                            println!("{}", k.path);
                        }
                        println!("----");
                    }
                    println!();
                }
            }
            CheckingMethod::SIZE => {
                for i in &self.files_with_identical_size {
                    number_of_files += i.1.len() as u64;
                    number_of_groups += 1;
                }
                println!(
                    "Found {} files in {} groups with same size(may have different content) which took {}:",
                    number_of_files,
                    number_of_groups,
                    self.infos.lost_space.file_size(options::BINARY).unwrap()
                );
                for i in &self.files_with_identical_size {
                    println!("Size - {}", i.0);
                    for j in i.1 {
                        println!("{}", j.path);
                    }
                    println!();
                }
            }
        }
        Common::print_time(start_time, SystemTime::now(), "print_duplicated_entries".to_string());
    }
    /// Remove unused entries when included or excluded overlaps with each other or are duplicated
    fn optimize_directories(&mut self) -> bool {
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
            self.infos.errors.push("Optimize Directories ERROR: Excluded directories overlaps all included directories.".to_string());
            return false;
        }

        // Not needed, but better is to have sorted everything
        self.excluded_directories.sort();
        self.included_directories.sort();
        Common::print_time(start_time, SystemTime::now(), "optimize_directories".to_string());
        true
    }

    fn delete_files(&mut self, check_method: &CheckingMethod, delete_method: &DeleteMethod) {
        if *delete_method == DeleteMethod::None {
            return;
        }
        let start_time: SystemTime = SystemTime::now();

        match check_method {
            CheckingMethod::HASH => {
                for entry in &self.files_with_identical_hashes {
                    for vector in entry.1 {
                        let tuple: (u64, usize, usize) = delete_files(&vector, &delete_method, &mut self.infos.warnings);
                        self.infos.gained_space += tuple.0;
                        self.infos.number_of_removed_files += tuple.1;
                        self.infos.number_of_failed_to_remove_files += tuple.2;
                    }
                }
            }
            CheckingMethod::SIZE => {
                for entry in &self.files_with_identical_size {
                    let tuple: (u64, usize, usize) = delete_files(&entry.1, &delete_method, &mut self.infos.warnings);
                    self.infos.gained_space += tuple.0;
                    self.infos.number_of_removed_files += tuple.1;
                    self.infos.number_of_failed_to_remove_files += tuple.2;
                }
            }
        }

        Common::print_time(start_time, SystemTime::now(), "delete_files".to_string());
    }
}
impl Default for DuplicateFinder {
    fn default() -> Self {
        Self::new()
    }
}

fn delete_files(vector: &[FileEntry], delete_method: &DeleteMethod, warnings: &mut Vec<String>) -> (u64, usize, usize) {
    assert!(vector.len() > 1, "Vector length must be bigger than 1(This should be done in previous steps).");
    let mut q_index: usize = 0;
    let mut q_time: u64 = 0;

    let mut gained_space: u64 = 0;
    let mut removed_files: usize = 0;
    let mut failed_to_remove_files: usize = 0;

    match delete_method {
        DeleteMethod::OneOldest => {
            for files in vector.iter().enumerate() {
                let time_since_epoch = files.1.created_date.duration_since(UNIX_EPOCH).expect("Invalid file date").as_secs();
                if q_time == 0 || q_time > time_since_epoch {
                    q_time = time_since_epoch;
                    q_index = files.0;
                }
            }
            match fs::remove_file(vector[q_index].path.clone()) {
                Ok(_) => {
                    removed_files += 1;
                    gained_space += vector[q_index].size;
                }
                Err(_) => {
                    failed_to_remove_files += 1;
                    warnings.push("Failed to delete".to_string() + &*vector[q_index].path);
                }
            };
        }
        DeleteMethod::OneNewest => {
            for files in vector.iter().enumerate() {
                let time_since_epoch = files.1.created_date.duration_since(UNIX_EPOCH).expect("Invalid file date").as_secs();
                if q_time == 0 || q_time < time_since_epoch {
                    q_time = time_since_epoch;
                    q_index = files.0;
                }
            }
            match fs::remove_file(vector[q_index].path.clone()) {
                Ok(_) => {
                    removed_files += 1;
                    gained_space += vector[q_index].size;
                }
                Err(_) => {
                    failed_to_remove_files += 1;
                    warnings.push("Failed to delete".to_string() + &*vector[q_index].path);
                }
            };
        }
        DeleteMethod::AllExceptOldest => {
            for files in vector.iter().enumerate() {
                let time_since_epoch = files.1.created_date.duration_since(UNIX_EPOCH).expect("Invalid file date").as_secs();
                if q_time == 0 || q_time > time_since_epoch {
                    q_time = time_since_epoch;
                    q_index = files.0;
                }
            }
            for files in vector.iter().enumerate() {
                if q_index != files.0 {
                    match fs::remove_file(vector[files.0].path.clone()) {
                        Ok(_) => {
                            removed_files += 1;
                            gained_space += vector[files.0].size;
                        }
                        Err(_) => {
                            failed_to_remove_files += 1;
                            warnings.push("Failed to delete".to_string() + &*vector[files.0].path);
                        }
                    };
                }
            }
        }
        DeleteMethod::AllExceptNewest => {
            for files in vector.iter().enumerate() {
                let time_since_epoch = files.1.created_date.duration_since(UNIX_EPOCH).expect("Invalid file date").as_secs();
                if q_time == 0 || q_time < time_since_epoch {
                    q_time = time_since_epoch;
                    q_index = files.0;
                }
            }
            for files in vector.iter().enumerate() {
                if q_index != files.0 {
                    match fs::remove_file(vector[files.0].path.clone()) {
                        Ok(_) => {
                            removed_files += 1;
                            gained_space += vector[files.0].size;
                        }
                        Err(_) => {
                            failed_to_remove_files += 1;
                            warnings.push("Failed to delete".to_string() + &*vector[files.0].path);
                        }
                    };
                }
            }
        }
        DeleteMethod::None => {
            panic!();
        }
    };
    println!("{}    {}    {}", gained_space, removed_files, failed_to_remove_files);
    (gained_space, removed_files, failed_to_remove_files)
}
