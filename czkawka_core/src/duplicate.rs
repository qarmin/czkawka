use crossbeam_channel::Receiver;
use humansize::{file_size_opts as options, FileSize};
use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::fs::{File, Metadata};
use std::io::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::common::Common;
use crate::common_directory::Directories;
use crate::common_extensions::Extensions;
use crate::common_items::ExcludedItems;
use crate::common_messages::Messages;
use crate::common_traits::*;

const HASH_MB_LIMIT_BYTES: u64 = 1024 * 1024; // 1MB

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum CheckingMethod {
    None,
    Size,
    Hash,
    HashMB,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum DeleteMethod {
    None,
    AllExceptNewest,
    AllExceptOldest,
    OneOldest,
    OneNewest,
}

#[derive(Clone)]
pub struct FileEntry {
    pub path: String,
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
    pub number_of_groups_by_size: usize,
    pub number_of_duplicated_files_by_size: usize,
    pub number_of_groups_by_hash: usize,
    pub number_of_duplicated_files_by_hash: usize,
    pub lost_space_by_size: u64,
    pub lost_space_by_hash: u64,
    pub bytes_read_when_hashing: u64,
    pub number_of_removed_files: usize,
    pub number_of_failed_to_remove_files: usize,
    pub gained_space: u64,
}

impl Info {
    pub fn new() -> Self {
        Default::default()
    }
}

/// Struct with required information's to work
pub struct DuplicateFinder {
    text_messages: Messages,
    information: Info,
    files_with_identical_size: BTreeMap<u64, Vec<FileEntry>>,        // File Size, File Entry
    files_with_identical_hashes: BTreeMap<u64, Vec<Vec<FileEntry>>>, // File Size, File Entry
    directories: Directories,
    allowed_extensions: Extensions,
    excluded_items: ExcludedItems,
    recursive_search: bool,
    minimal_file_size: u64,
    check_method: CheckingMethod,
    delete_method: DeleteMethod,
    stopped_search: bool,
}

impl DuplicateFinder {
    pub fn new() -> Self {
        Self {
            text_messages: Messages::new(),
            information: Info::new(),
            files_with_identical_size: Default::default(),
            files_with_identical_hashes: Default::default(),
            recursive_search: true,
            allowed_extensions: Extensions::new(),
            check_method: CheckingMethod::None,
            delete_method: DeleteMethod::None,
            minimal_file_size: 1024,
            directories: Directories::new(),
            excluded_items: ExcludedItems::new(),
            stopped_search: false,
        }
    }

    pub fn find_duplicates(&mut self, rx: Option<&Receiver<()>>) {
        self.directories.optimize_directories(self.recursive_search, &mut self.text_messages);
        if !self.check_files_size(rx) {
            self.stopped_search = true;
            return;
        }
        #[allow(clippy::collapsible_if)]
        if self.check_method == CheckingMethod::Hash || self.check_method == CheckingMethod::HashMB {
            if !self.check_files_hash(rx) {
                self.stopped_search = true;
                return;
            }
        }
        self.delete_files();
        self.debug_print();
    }

    pub const fn get_check_method(&self) -> &CheckingMethod {
        &self.check_method
    }

    pub fn get_stopped_search(&self) -> bool {
        self.stopped_search
    }

    pub const fn get_files_sorted_by_size(&self) -> &BTreeMap<u64, Vec<FileEntry>> {
        &self.files_with_identical_size
    }

    pub const fn get_files_sorted_by_hash(&self) -> &BTreeMap<u64, Vec<Vec<FileEntry>>> {
        &self.files_with_identical_hashes
    }

    pub const fn get_text_messages(&self) -> &Messages {
        &self.text_messages
    }

    pub const fn get_information(&self) -> &Info {
        &self.information
    }

    pub fn set_check_method(&mut self, check_method: CheckingMethod) {
        self.check_method = check_method;
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

    /// Read file length and puts it to different boxes(each for different lengths)
    /// If in box is only 1 result, then it is removed
    fn check_files_size(&mut self, rx: Option<&Receiver<()>>) -> bool {
        // TODO maybe add multithreading checking for file hash
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
                    // let mut have_valid_extension: bool;
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
                    if metadata.len() >= self.minimal_file_size {
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
                            size: metadata.len(),
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

                        // Adding files to BTreeMap
                        self.files_with_identical_size.entry(metadata.len()).or_insert_with(Vec::new);
                        self.files_with_identical_size.get_mut(&metadata.len()).unwrap().push(fe);

                        self.information.number_of_checked_files += 1;
                    } else {
                        // Probably this is symbolic links so we are free to ignore this
                        self.information.number_of_ignored_files += 1;
                    }
                } else {
                    // Probably this is symbolic links so we are free to ignore this
                    self.information.number_of_ignored_things += 1;
                }
            }
        }

        // Create new BTreeMap without single size entries(files have not duplicates)
        let mut new_map: BTreeMap<u64, Vec<FileEntry>> = Default::default();

        self.information.number_of_duplicated_files_by_size = 0;

        for (size, vector) in &self.files_with_identical_size {
            if vector.len() > 1 {
                self.information.number_of_duplicated_files_by_size += vector.len() - 1;
                self.information.number_of_groups_by_size += 1;
                self.information.lost_space_by_size += (vector.len() as u64 - 1) * size;
                new_map.insert(*size, vector.clone());
            }
        }
        self.files_with_identical_size = new_map;

        Common::print_time(start_time, SystemTime::now(), "check_files_size".to_string());
        true
    }

    /// The slowest checking type, which must be applied after checking for size
    fn check_files_hash(&mut self, rx: Option<&Receiver<()>>) -> bool {
        let start_time: SystemTime = SystemTime::now();
        let mut file_handler: File;
        let mut hashmap_with_hash: HashMap<String, Vec<FileEntry>>;

        for (size, vector) in &self.files_with_identical_size {
            hashmap_with_hash = Default::default();

            for file_entry in vector {
                if rx.is_some() && rx.unwrap().try_recv().is_ok() {
                    return false;
                }
                file_handler = match File::open(&file_entry.path) {
                    Ok(t) => t,
                    Err(_) => {
                        self.text_messages.warnings.push("Unable to check hash of file ".to_string() + file_entry.path.as_str());
                        continue;
                    }
                };

                let mut error_reading_file: bool = false;

                let mut hasher: blake3::Hasher = blake3::Hasher::new();
                let mut buffer = [0u8; 16384];
                let mut read_bytes: u64 = 0;
                loop {
                    let n = match file_handler.read(&mut buffer) {
                        Ok(t) => t,
                        Err(_) => {
                            self.text_messages.warnings.push("Error happened when checking hash of file ".to_string() + file_entry.path.as_str());
                            error_reading_file = true;
                            break;
                        }
                    };
                    if n == 0 {
                        break;
                    }

                    read_bytes += n as u64;
                    self.information.bytes_read_when_hashing += n as u64;
                    hasher.update(&buffer[..n]);

                    if self.check_method == CheckingMethod::HashMB && read_bytes >= HASH_MB_LIMIT_BYTES {
                        break;
                    }
                }
                if !error_reading_file {
                    let hash_string: String = hasher.finalize().to_hex().to_string();
                    hashmap_with_hash.entry(hash_string.to_string()).or_insert_with(Vec::new);
                    hashmap_with_hash.get_mut(hash_string.as_str()).unwrap().push(file_entry.to_owned());
                }
            }
            for (_string, vector) in hashmap_with_hash {
                if vector.len() > 1 {
                    self.files_with_identical_hashes.entry(*size).or_insert_with(Vec::new);
                    self.files_with_identical_hashes.get_mut(size).unwrap().push(vector);
                }
            }
        }

        for (size, vector_vectors) in &self.files_with_identical_hashes {
            for vector in vector_vectors {
                self.information.number_of_duplicated_files_by_hash += vector.len() - 1;
                self.information.number_of_groups_by_hash += 1;
                self.information.lost_space_by_hash += (vector.len() as u64 - 1) * size;
            }
        }

        Common::print_time(start_time, SystemTime::now(), "check_files_hash".to_string());
        true
    }

    /// Function to delete files, from filed before BTreeMap
    /// Using another function to delete files to avoid duplicates data
    fn delete_files(&mut self) {
        let start_time: SystemTime = SystemTime::now();

        match self.check_method {
            CheckingMethod::Hash | CheckingMethod::HashMB => {
                for vector_vectors in self.files_with_identical_hashes.values() {
                    for vector in vector_vectors.iter() {
                        let tuple: (u64, usize, usize) = delete_files(vector, &self.delete_method, &mut self.text_messages.warnings);
                        self.information.gained_space += tuple.0;
                        self.information.number_of_removed_files += tuple.1;
                        self.information.number_of_failed_to_remove_files += tuple.2;
                    }
                }
            }
            CheckingMethod::Size => {
                for vector in self.files_with_identical_size.values() {
                    let tuple: (u64, usize, usize) = delete_files(vector, &self.delete_method, &mut self.text_messages.warnings);
                    self.information.gained_space += tuple.0;
                    self.information.number_of_removed_files += tuple.1;
                    self.information.number_of_failed_to_remove_files += tuple.2;
                }
            }
            CheckingMethod::None => {
                //Just do nothing
                panic!("Checking method should never be none.");
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

impl DebugPrint for DuplicateFinder {
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
        println!(
            "Number of duplicated files by size(in groups) - {} ({})",
            self.information.number_of_duplicated_files_by_size, self.information.number_of_groups_by_size
        );
        println!(
            "Number of duplicated files by hash(in groups) - {} ({})",
            self.information.number_of_duplicated_files_by_hash, self.information.number_of_groups_by_hash
        );
        println!("Lost space by size - {} ({} bytes)", self.information.lost_space_by_size.file_size(options::BINARY).unwrap(), self.information.lost_space_by_size);
        println!("Lost space by hash - {} ({} bytes)", self.information.lost_space_by_hash.file_size(options::BINARY).unwrap(), self.information.lost_space_by_hash);
        println!(
            "Gained space by removing duplicated entries - {} ({} bytes)",
            self.information.gained_space.file_size(options::BINARY).unwrap(),
            self.information.gained_space
        );
        println!(
            "Bytes read when hashing - {} ({} bytes)",
            self.information.bytes_read_when_hashing.file_size(options::BINARY).unwrap(),
            self.information.bytes_read_when_hashing
        );
        println!("Number of removed files - {}", self.information.number_of_removed_files);
        println!("Number of failed to remove files - {}", self.information.number_of_failed_to_remove_files);

        println!("### Other");

        println!("Files list size - {}", self.files_with_identical_size.len());
        println!("Hashed Files list size - {}", self.files_with_identical_hashes.len());
        println!("Allowed extensions - {:?}", self.allowed_extensions.file_extensions);
        println!("Excluded items - {:?}", self.excluded_items.items);
        println!("Included directories - {:?}", self.directories.included_directories);
        println!("Excluded directories - {:?}", self.directories.excluded_directories);
        println!("Recursive search - {}", self.recursive_search.to_string());
        println!("Minimum file size - {:?}", self.minimal_file_size);
        println!("Checking Method - {:?}", self.check_method);
        println!("Delete Method - {:?}", self.delete_method);
        println!("-----------------------------------------");
    }
}
impl SaveResults for DuplicateFinder {
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

        if !self.files_with_identical_size.is_empty() {
            file.write_all(b"-------------------------------------------------Files with same size-------------------------------------------------\n").unwrap();
            file.write_all(
                format!(
                    "Found {} duplicated files which in {} groups which takes {}.\n",
                    self.information.number_of_duplicated_files_by_size,
                    self.information.number_of_groups_by_size,
                    self.information.lost_space_by_size.file_size(options::BINARY).unwrap()
                )
                .as_bytes(),
            )
            .unwrap();
            for (size, vector) in self.files_with_identical_size.iter().rev() {
                file.write_all(format!("\n---- Size {} ({}) - {} files \n", size.file_size(options::BINARY).unwrap(), size, vector.len()).as_bytes()).unwrap();
                for file_entry in vector {
                    file.write_all(format!("{} \n", file_entry.path).as_bytes()).unwrap();
                }
            }

            if !self.files_with_identical_hashes.is_empty() {
                file.write_all(b"-------------------------------------------------Files with same hashes-------------------------------------------------\n").unwrap();
                file.write_all(
                    format!(
                        "Found {} duplicated files which in {} groups which takes {}.\n",
                        self.information.number_of_duplicated_files_by_hash,
                        self.information.number_of_groups_by_hash,
                        self.information.lost_space_by_hash.file_size(options::BINARY).unwrap()
                    )
                    .as_bytes(),
                )
                .unwrap();
                for (size, vectors_vector) in self.files_with_identical_hashes.iter().rev() {
                    for vector in vectors_vector {
                        file.write_all(format!("\n---- Size {} ({}) - {} files \n", size.file_size(options::BINARY).unwrap(), size, vector.len()).as_bytes()).unwrap();
                        for file_entry in vector {
                            file.write_all(format!("{} \n", file_entry.path).as_bytes()).unwrap();
                        }
                    }
                }
            }
        } else {
            file.write_all(b"Not found any duplicates.").unwrap();
        }
        Common::print_time(start_time, SystemTime::now(), "save_results_to_file".to_string());
        true
    }
}
impl PrintResults for DuplicateFinder {
    /// Print information's about duplicated entries
    /// Only needed for CLI
    fn print_results(&self) {
        let start_time: SystemTime = SystemTime::now();
        let mut number_of_files: u64 = 0;
        let mut number_of_groups: u64 = 0;

        match self.check_method {
            CheckingMethod::Hash | CheckingMethod::HashMB => {
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
                    self.information.lost_space_by_size.file_size(options::BINARY).unwrap()
                );
                for (size, vector) in self.files_with_identical_hashes.iter().rev() {
                    for j in vector {
                        println!("Size - {} ({}) - {} files ", size.file_size(options::BINARY).unwrap(), size, j.len());
                        for k in j {
                            println!("{}", k.path);
                        }
                        println!("----");
                    }
                    println!();
                }
            }
            CheckingMethod::Size => {
                for i in &self.files_with_identical_size {
                    number_of_files += i.1.len() as u64;
                    number_of_groups += 1;
                }
                println!(
                    "Found {} files in {} groups with same size(may have different content) which took {}:",
                    number_of_files,
                    number_of_groups,
                    self.information.lost_space_by_size.file_size(options::BINARY).unwrap()
                );
                for (size, vector) in &self.files_with_identical_size {
                    println!("Size - {} ({}) - {} files ", size.file_size(options::BINARY).unwrap(), size, vector.len());
                    for j in vector {
                        println!("{}", j.path);
                    }
                    println!();
                }
            }
            CheckingMethod::None => {
                panic!("Checking Method shouldn't be ever set to None");
            }
        }
        Common::print_time(start_time, SystemTime::now(), "print_entries".to_string());
    }
}

/// Functions to remove slice(vector) of files with provided method
/// Returns size of removed elements, number of deleted and failed to delete files and modified warning list
fn delete_files(vector: &[FileEntry], delete_method: &DeleteMethod, warnings: &mut Vec<String>) -> (u64, usize, usize) {
    assert!(vector.len() > 1, "Vector length must be bigger than 1(This should be done in previous steps).");
    let mut q_index: usize = 0;
    let mut q_time: u64 = 0;

    let mut gained_space: u64 = 0;
    let mut removed_files: usize = 0;
    let mut failed_to_remove_files: usize = 0;

    match delete_method {
        DeleteMethod::OneOldest => {
            for (index, file) in vector.iter().enumerate() {
                if q_time == 0 || q_time > file.modified_date {
                    q_time = file.modified_date;
                    q_index = index;
                }
            }
            match fs::remove_file(vector[q_index].path.clone()) {
                Ok(_) => {
                    removed_files += 1;
                    gained_space += vector[q_index].size;
                }
                Err(_) => {
                    failed_to_remove_files += 1;
                    warnings.push("Failed to delete".to_string() + vector[q_index].path.as_str());
                }
            };
        }
        DeleteMethod::OneNewest => {
            for (index, file) in vector.iter().enumerate() {
                if q_time == 0 || q_time < file.modified_date {
                    q_time = file.modified_date;
                    q_index = index;
                }
            }
            match fs::remove_file(vector[q_index].path.clone()) {
                Ok(_) => {
                    removed_files += 1;
                    gained_space += vector[q_index].size;
                }
                Err(_) => {
                    failed_to_remove_files += 1;
                    warnings.push("Failed to delete".to_string() + vector[q_index].path.as_str());
                }
            };
        }
        DeleteMethod::AllExceptOldest => {
            for (index, file) in vector.iter().enumerate() {
                if q_time == 0 || q_time > file.modified_date {
                    q_time = file.modified_date;
                    q_index = index;
                }
            }
            for (index, file) in vector.iter().enumerate() {
                if q_index != index {
                    match fs::remove_file(file.path.clone()) {
                        Ok(_) => {
                            removed_files += 1;
                            gained_space += file.size;
                        }
                        Err(_) => {
                            failed_to_remove_files += 1;
                            warnings.push("Failed to delete".to_string() + file.path.as_str());
                        }
                    };
                }
            }
        }
        DeleteMethod::AllExceptNewest => {
            for (index, file) in vector.iter().enumerate() {
                if q_time == 0 || q_time < file.modified_date {
                    q_time = file.modified_date;
                    q_index = index;
                }
            }
            for (index, file) in vector.iter().enumerate() {
                if q_index != index {
                    match fs::remove_file(file.path.clone()) {
                        Ok(_) => {
                            removed_files += 1;
                            gained_space += file.size;
                        }
                        Err(_) => {
                            failed_to_remove_files += 1;
                            warnings.push("Failed to delete".to_string() + file.path.as_str());
                        }
                    };
                }
            }
        }
        DeleteMethod::None => {
            // Just don't remove files
        }
    };
    (gained_space, removed_files, failed_to_remove_files)
}
