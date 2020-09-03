// TODO when using GUI all or most println!() should be used as variables passed by argument
use humansize::{file_size_opts as options, FileSize};
use std::collections::{BTreeMap, HashMap};
use std::fs::{File, Metadata};
use std::io::prelude::*;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{fs, process};

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

pub struct DuplicateFinder {
    number_of_checked_files: usize,
    number_of_ignored_files: usize,
    number_of_checked_folders: usize,
    number_of_ignored_things: usize,
    number_of_duplicated_files: usize,
    files_with_identical_size: HashMap<u64, Vec<FileEntry>>,
    files_with_identical_hashes: BTreeMap<u64, Vec<Vec<FileEntry>>>,
    allowed_extensions: Vec<String>, // jpg, jpeg, mp4
    lost_space: u64,
    // excluded_items: Vec<String>, // TODO, support for e.g. */.git/*
    excluded_directories: Vec<String>,
    included_directories: Vec<String>,
    min_file_size: u64,
}

impl DuplicateFinder {
    pub fn new() -> DuplicateFinder {
        DuplicateFinder {
            number_of_checked_files: 0,
            number_of_ignored_files: 0,
            number_of_checked_folders: 0,
            number_of_ignored_things: 0,
            number_of_duplicated_files: 0,
            files_with_identical_size: Default::default(),
            files_with_identical_hashes: Default::default(),
            // excluded_items: vec![],
            excluded_directories: vec![],
            included_directories: vec![],
            min_file_size: 1024,
            allowed_extensions: vec![],
            lost_space: 0,
        }
    }

    pub fn find_duplicates(mut self, check_method: &CheckingMethod, delete_method: &DeleteMethod) {
        self.optimize_directories();
        self.debug_print();
        self.check_files_size();
        self.remove_files_with_unique_size();
        if *check_method == CheckingMethod::HASH {
            self.check_files_hash();
        }
        self.calculate_lost_space(check_method);
        self.print_duplicated_entries(check_method);
        self.delete_files(check_method, delete_method);
    }

    pub fn set_min_file_size(&mut self, min_size: u64) {
        self.min_file_size = min_size;
    }

    pub fn set_excluded_items(&mut self, _excluded_items: String) {
        // TODO Still don't know how to exactly parse this
        // Things like /.git/ should be by default hidden with help of this *.git*
    }
    pub fn set_allowed_extensions(&mut self, mut allowed_extensions: String) {
        if allowed_extensions.is_empty() {
            println!("No allowed extension was provided, so all are allowed");
        }
        allowed_extensions = allowed_extensions.replace("IMAGE", "jpg,kra,gif,png,bmp,tiff,webp,hdr,svg");
        allowed_extensions = allowed_extensions.replace("VIDEO", "mp4,flv,mkv,webm,vob,ogv,gifv,avi,mov,wmv,mpg,m4v,m4p,mpeg,3gp");
        allowed_extensions = allowed_extensions.replace("MUSIC", "mp3,flac,ogg,tta,wma,webm");
        allowed_extensions = allowed_extensions.replace("TEXT", "txt,doc,docx,odt,rtf");

        let extensions: Vec<String> = allowed_extensions.split(',').map(String::from).collect();
        for mut extension in extensions {
            if extension.contains('.') {
                if !extension.starts_with('.') {
                    println!("{} is not valid extension(valid extension doesn't have dot inside)", extension);
                    continue;
                }
                extension = extension.replace('.', "");
            }
            if !self.allowed_extensions.contains(&extension.trim().to_string()) {
                self.allowed_extensions.push(extension.trim().to_string());
            }
        }

        if self.allowed_extensions.is_empty() {
            println!("No valid extensions were provided, so allowing all extensions by default.");
        }
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

            // directory must end with /, due to possibility of incorrect assumption, that e.g. /home/rafal is top folder to /home/rafalinho
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
            let directory: String = directory.trim().to_string();

            if directory == "" {
                continue;
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
            if !Path::new(&directory).is_dir() {
                println!("Exclude Directory ERROR: {} isn't folder.", directory);
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
        self.lost_space = bytes;
    }

    // TODO - Still isn't used but it will be probably required with GUI
    // pub fn clear(&mut self) {
    //
    //     self.number_of_checked_files = 0;
    //     self.number_of_checked_folders = 0;
    //     self.number_of_ignored_things = 0;
    //     self.number_of_files_which_has_duplicated_entries = 0;
    //     self.number_of_duplicated_files = 0;
    //     self.files_sizeclear();
    //     self.excluded_directories.clear();
    //     self.included_directories.clear();
    // }
    fn check_files_size(&mut self) {
        // TODO maybe add multithreading checking for file hash
        let start_time: SystemTime = SystemTime::now();
        let mut folders_to_check: Vec<String> = Vec::with_capacity(1024 * 2); // This should be small enough too not see to big difference and
                                                                              // big enough to store most of paths without needing to resize vector

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
                _ => continue,
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
                        folders_to_check.push(next_folder);
                    }
                    self.number_of_checked_folders += 1;

                //println!("Directory\t - {:?}", next_folder); // DEBUG
                } else if metadata.is_file() {
                    let mut have_valid_extension: bool;
                    let file_name_lowercase: String = entry_data.file_name().into_string().unwrap().to_lowercase();

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

                    if metadata.len() >= self.min_file_size && have_valid_extension {
                        let current_file_name = "".to_owned() + &current_folder + &entry_data.file_name().into_string().unwrap();
                        // println!("File\t\t - {:?}", current_file_name); // DEBUG
                        //file_to_check
                        let fe: FileEntry = FileEntry {
                            path: current_file_name,
                            size: metadata.len(),
                            created_date: metadata.created().unwrap(),
                            modified_date: metadata.modified().unwrap(),
                        };
                        // // self.files_with_identical_size.entry from below should be faster according to clippy
                        // if !self.files_with_identical_size.contains_key(&metadata.len()) {
                        //     self.files_with_identical_size.insert(metadata.len(), Vec::new());
                        // }
                        self.files_with_identical_size.entry(metadata.len()).or_insert_with(Vec::new);

                        self.files_with_identical_size.get_mut(&metadata.len()).unwrap().push(fe);

                        self.number_of_checked_files += 1;
                    } else {
                        self.number_of_ignored_files += 1;
                    }
                } else {
                    // Probably this is symbolic links so we are free to ignore this
                    // println!("Found another type of file {} {:?}","".to_owned() + &current_folder + &entry_data.file_name().into_string().unwrap(), metadata) //DEBUG
                    self.number_of_ignored_things += 1;
                }
            }
        }
        self.debug_print();
        Common::print_time(start_time, SystemTime::now(), "check_files_size".to_string());
        //println!("Duration of finding duplicates {:?}", end_time.duration_since(start_time).expect("a"));
    }
    // pub fn save_results_to_file(&self) {}

    /// Remove files which have unique size
    fn remove_files_with_unique_size(&mut self) {
        let start_time: SystemTime = SystemTime::now();
        self.debug_print();
        let mut new_hashmap: HashMap<u64, Vec<FileEntry>> = Default::default();

        self.number_of_duplicated_files = 0;

        for entry in &self.files_with_identical_size {
            if entry.1.len() > 1 {
                self.number_of_duplicated_files += entry.1.len() - 1;
                new_hashmap.insert(*entry.0, entry.1.clone());
            }
        }

        self.files_with_identical_size = new_hashmap;

        self.debug_print();
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
                        continue;
                    }
                };

                let mut hasher: blake3::Hasher = blake3::Hasher::new();
                let mut buffer = [0u8; 16384];
                loop {
                    let n = file_handler.read(&mut buffer).unwrap();
                    if n == 0 {
                        break;
                    }
                    hasher.update(&buffer[..n]);
                }
                let hash_string: String = hasher.finalize().to_hex().to_string();
                hashmap_with_hash.entry(hash_string.to_string()).or_insert_with(Vec::new);
                hashmap_with_hash.get_mut(&*hash_string).unwrap().push(file_entry.1.to_owned());
            }
            for hash_entry in hashmap_with_hash {
                if hash_entry.1.len() > 1 {
                    self.files_with_identical_hashes.entry(*entry.0).or_insert_with(Vec::new);
                    self.files_with_identical_hashes.get_mut(entry.0).unwrap().push(hash_entry.1);
                    // self.files_with_identical_hashes.insert(*entry.0,hash_entry.1);
                }
            }
        }
        self.debug_print();
        Common::print_time(start_time, SystemTime::now(), "check_files_hash".to_string());
    }
    // /// I'm not sure about performance, so maybe I
    // pub fn find_small_duplicates_by_hashing(mut self){
    //     let start_time: SystemTime = SystemTime::now();
    //     let size_limit_for_small_files u64 =  // 16 MB
    //     let mut new_hashmap
    //
    //     Common::print_time(start_time, SystemTime::now(), "find_duplicates_by_comparing_begin_bytes_of_file".to_string());
    // }

    /// Setting include directories, panics when there is not directories available

    fn debug_print(&self) {
        // println!("---------------DEBUG PRINT---------------");
        // println!("Number of all checked files - {}", self.number_of_checked_files);
        // println!("Number of all ignored files - {}", self.number_of_ignored_files);
        // println!("Number of all checked folders - {}", self.number_of_checked_folders);
        // println!("Number of all ignored things - {}", self.number_of_ignored_things);
        // println!("Number of duplicated files - {}", self.number_of_duplicated_files);
        // let mut file_size: u64 = 0;
        // for i in &self.files_with_identical_size {
        //     file_size += i.1.len() as u64;
        // }
        // println!("Files list size - {} ({})", self.files_with_identical_size.len(), file_size);
        // let mut hashed_file_size: u64 = 0;
        // for i in &self.files_with_identical_hashes {
        //     for j in i.1 {
        //         hashed_file_size += j.len() as u64;
        //     }
        // }
        // println!("Hashed Files list size - {} ({})", self.files_with_identical_hashes.len(), hashed_file_size);
        // println!("Excluded directories - {:?}", self.excluded_directories);
        // println!("Included directories - {:?}", self.included_directories);
        // println!("-----------------------------------------");
    }

    fn print_duplicated_entries(&self, check_method: &CheckingMethod) {
        let start_time: SystemTime = SystemTime::now();
        let mut number_of_files: u64 = 0;
        let mut number_of_groups: u64 = 0;

        match check_method {
            CheckingMethod::HASH => {
                for i in &self.files_with_identical_hashes {
                    for j in i.1 {
                        number_of_files += j.len() as u64;
                        number_of_groups += 1;
                    }
                }
                println!(
                    "Found {} duplicated files in {} groups with same content which took {}:",
                    number_of_files,
                    number_of_groups,
                    self.lost_space.file_size(options::BINARY).unwrap()
                );
                for i in &self.files_with_identical_hashes {
                    println!("Size - {}", i.0.file_size(options::BINARY).unwrap());
                    for j in i.1 {
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
                    self.lost_space.file_size(options::BINARY).unwrap()
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
    /// ```
    // let df : DuplicateFinder = saf
    /// ```
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

    fn delete_files(&mut self, check_method: &CheckingMethod, delete_method: &DeleteMethod) {
        if *delete_method == DeleteMethod::None {
            return;
        }
        let start_time: SystemTime = SystemTime::now();
        let mut errors: Vec<String> = Vec::new();
        match check_method {
            CheckingMethod::HASH => {
                for entry in &self.files_with_identical_hashes {
                    for vector in entry.1 {
                        delete_files(&vector, &delete_method, &mut errors);
                    }
                }
            }
            CheckingMethod::SIZE => {
                for entry in &self.files_with_identical_size {
                    delete_files(&entry.1, &delete_method, &mut errors);
                }
            }
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
}
fn delete_files(vector: &[FileEntry], delete_method: &DeleteMethod, errors: &mut Vec<String>) {
    assert!(vector.len() > 1, "Vector length must be bigger than 1(This should be done in previous steps).");
    let mut q_index: usize = 0;
    let mut q_time: u64 = 0;
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
                Ok(_) => (),
                Err(_) => errors.push(vector[q_index].path.clone()),
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
                Ok(_) => (),
                Err(_) => errors.push(vector[q_index].path.clone()),
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
                        Ok(_) => (),
                        Err(_) => errors.push(vector[files.0].path.clone()),
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
                        Ok(_) => (),
                        Err(_) => errors.push(vector[files.0].path.clone()),
                    };
                }
            }
        }
        DeleteMethod::None => {
            panic!();
        }
    };
}
