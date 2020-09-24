use humansize::{file_size_opts as options, FileSize};
use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::fs::{File, Metadata};
use std::io::prelude::*;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::common::{Common, Messages};

#[derive(PartialEq, Eq, Clone)]
pub enum CheckingMethod {
    NONE,
    SIZE,
    HASH,
}

#[derive(Eq, PartialEq, Clone)]
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
    pub created_date: SystemTime,
    pub modified_date: SystemTime,
}

/// Info struck with helpful information's about results
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
    pub fn new() -> Info {
        Info {
            number_of_checked_files: 0,
            number_of_ignored_files: 0,
            number_of_checked_folders: 0,
            number_of_ignored_things: 0,
            number_of_groups_by_size: 0,
            number_of_duplicated_files_by_size: 0,
            number_of_groups_by_hash: 0,
            number_of_duplicated_files_by_hash: 0,
            lost_space_by_size: 0,
            lost_space_by_hash: 0,
            bytes_read_when_hashing: 0,
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

/// Struct with required information's to work
pub struct DuplicateFinder {
    text_messages: Messages,
    information: Info,
    files_with_identical_size: BTreeMap<u64, Vec<FileEntry>>,
    files_with_identical_hashes: BTreeMap<u64, Vec<Vec<FileEntry>>>,
    allowed_extensions: Vec<String>,
    excluded_items: Vec<String>,
    excluded_directories: Vec<String>,
    included_directories: Vec<String>,
    recursive_search: bool,
    min_file_size: u64,
    check_method: CheckingMethod,
    delete_method: DeleteMethod,
}

impl DuplicateFinder {
    pub fn new() -> DuplicateFinder {
        DuplicateFinder {
            text_messages: Default::default(),
            information: Info::new(),
            files_with_identical_size: Default::default(),
            files_with_identical_hashes: Default::default(),
            excluded_items: vec![],
            excluded_directories: vec![],
            included_directories: vec![],
            recursive_search: true,
            allowed_extensions: vec![],
            check_method: CheckingMethod::NONE,
            delete_method: DeleteMethod::None,
            min_file_size: 1024,
        }
    }

    pub fn get_files_sorted_by_size(&self) -> &BTreeMap<u64, Vec<FileEntry>> {
        &self.files_with_identical_size
    }

    pub fn get_files_sorted_by_hash(&self) -> &BTreeMap<u64, Vec<Vec<FileEntry>>> {
        &self.files_with_identical_hashes
    }

    pub fn get_text_messages(&self) -> &Messages {
        &self.text_messages
    }
    pub fn get_information(&self) -> &Info {
        &self.information
    }

    pub fn find_duplicates(&mut self) {
        self.optimize_directories();
        self.check_files_size();
        if self.check_method == CheckingMethod::HASH {
            self.check_files_hash();
        }
        self.delete_files();
        self.debug_print();
    }

    pub fn set_check_method(&mut self, check_method: CheckingMethod) {
        self.check_method = check_method;
    }

    pub fn set_delete_method(&mut self, delete_method: DeleteMethod) {
        self.delete_method = delete_method;
    }

    pub fn set_min_file_size(&mut self, min_size: u64) {
        self.min_file_size = min_size;
    }

    pub fn set_recursive_search(&mut self, recursive_search: bool) {
        self.recursive_search = recursive_search;
    }
    pub fn set_excluded_items(&mut self, mut excluded_items: String) {
        let start_time: SystemTime = SystemTime::now();

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
            if expression == "DEFAULT" {
                // TODO add more files by default
                checked_expressions.push("*/.git/*".to_string());
                continue;
            }
            if !expression.contains('*') {
                self.text_messages.warnings.push("Excluded Items Warning: Wildcard * is required in expression, ignoring ".to_string() + expression.as_str());
                continue;
            }

            checked_expressions.push(expression);
        }
        self.excluded_items = checked_expressions;
        Common::print_time(start_time, SystemTime::now(), "set_excluded_items".to_string());
    }
    pub fn set_allowed_extensions(&mut self, mut allowed_extensions: String) {
        let start_time: SystemTime = SystemTime::now();
        if allowed_extensions.is_empty() {
            return;
        }
        allowed_extensions = allowed_extensions.replace("IMAGE", "jpg,kra,gif,png,bmp,tiff,webp,hdr,svg");
        allowed_extensions = allowed_extensions.replace("VIDEO", "mp4,flv,mkv,webm,vob,ogv,gifv,avi,mov,wmv,mpg,m4v,m4p,mpeg,3gp");
        allowed_extensions = allowed_extensions.replace("MUSIC", "mp3,flac,ogg,tta,wma,webm");
        allowed_extensions = allowed_extensions.replace("TEXT", "txt,doc,docx,odt,rtf");

        let extensions: Vec<String> = allowed_extensions.split(',').map(String::from).collect();
        for mut extension in extensions {
            if extension == "" || extension.replace('.', "").trim() == "" {
                continue;
            }

            if extension.starts_with('.') {
                extension = extension[1..].to_string();
            }

            if extension[1..].contains('.') {
                self.text_messages.warnings.push(".".to_string() + extension.as_str() + " is not valid extension(valid extension doesn't have dot inside)");
                continue;
            }

            if !self.allowed_extensions.contains(&extension.trim().to_string()) {
                self.allowed_extensions.push(extension.trim().to_string());
            }
        }

        if self.allowed_extensions.is_empty() {
            self.text_messages.messages.push("No valid extensions were provided, so allowing all extensions by default.".to_string());
        }
        Common::print_time(start_time, SystemTime::now(), "set_allowed_extensions".to_string());
    }
    pub fn set_include_directory(&mut self, mut include_directory: String) -> bool {
        let start_time: SystemTime = SystemTime::now();

        if include_directory.is_empty() {
            self.text_messages.errors.push("At least one directory must be provided".to_string());
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
                self.text_messages.warnings.push("Include Directory Warning: Wildcards in path are not supported, ignoring ".to_string() + directory.as_str());
                continue;
            }
            if !directory.starts_with('/') {
                self.text_messages.warnings.push("Include Directory Warning: Relative path are not supported, ignoring ".to_string() + directory.as_str());
                continue;
            }
            if !Path::new(&directory).exists() {
                self.text_messages.warnings.push("Include Directory Warning: Provided folder path must exits, ignoring ".to_string() + directory.as_str());
                continue;
            }
            if !Path::new(&directory).is_dir() {
                self.text_messages.warnings.push("Include Directory Warning: Provided path must point at the directory, ignoring ".to_string() + directory.as_str());
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
            self.text_messages.errors.push("Include Directory ERROR: Not found even one correct path to include which is required.".to_string());
            return false;
        }

        self.included_directories = checked_directories;

        Common::print_time(start_time, SystemTime::now(), "set_include_directory".to_string());
        true
    }

    pub fn set_exclude_directory(&mut self, mut exclude_directory: String) {
        let start_time: SystemTime = SystemTime::now();
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
                self.text_messages.errors.push("Exclude Directory ERROR: Excluding / is pointless, because it means that no files will be scanned.".to_string());
                break;
            }
            if directory.contains('*') {
                self.text_messages.warnings.push("Exclude Directory Warning: Wildcards in path are not supported, ignoring ".to_string() + directory.as_str());
                continue;
            }
            if !directory.starts_with('/') {
                self.text_messages.warnings.push("Exclude Directory Warning: Relative path are not supported, ignoring ".to_string() + directory.as_str());
                continue;
            }
            if !Path::new(&directory).exists() {
                self.text_messages.warnings.push("Exclude Directory Warning: Provided folder path must exits, ignoring ".to_string() + directory.as_str());
                continue;
            }
            if !Path::new(&directory).is_dir() {
                self.text_messages.warnings.push("Exclude Directory Warning: Provided path must point at the directory, ignoring ".to_string() + directory.as_str());
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

        Common::print_time(start_time, SystemTime::now(), "set_exclude_directory".to_string());
    }

    fn check_files_size(&mut self) {
        // TODO maybe add multithreading checking for file hash
        let start_time: SystemTime = SystemTime::now();
        let mut folders_to_check: Vec<String> = Vec::with_capacity(1024 * 2); // This should be small enough too not see to big difference and big enough to store most of paths without needing to resize vector

        // Add root folders for finding
        for id in &self.included_directories {
            folders_to_check.push(id.to_string());
        }
        self.information.number_of_checked_folders += folders_to_check.len();

        let mut current_folder: String;
        let mut next_folder: String;
        while !folders_to_check.is_empty() {
            current_folder = folders_to_check.pop().unwrap();

            let read_dir = match fs::read_dir(&current_folder) {
                Ok(t) => t,
                Err(_) => {
                    self.text_messages.warnings.push("Cannot open dir ".to_string() + current_folder.as_str());
                    continue;
                } // Permissions denied
            };
            for entry in read_dir {
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
                    // if entry_data.file_name().into_string().is_err() { // Probably this can be removed, if crash still will be happens, then uncomment this line
                    //     self.text_messages.warnings.push("Cannot read folder name in dir ".to_string() + current_folder.as_str());
                    //     continue; // Permissions denied
                    // }

                    if !self.recursive_search {
                        continue;
                    }

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
                } else if metadata.is_file() {
                    let mut have_valid_extension: bool;
                    let file_name_lowercase: String = entry_data.file_name().into_string().unwrap().to_lowercase();

                    // Checking allowed extensions
                    if !self.allowed_extensions.is_empty() {
                        have_valid_extension = false;
                        for i in &self.allowed_extensions {
                            if file_name_lowercase.ends_with((".".to_string() + i.to_lowercase().as_str()).as_str()) {
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
                                    self.text_messages.warnings.push("Unable to get creation date from file ".to_string() + current_file_name.as_str());
                                    SystemTime::now()
                                } // Permissions Denied
                            },
                            modified_date: match metadata.modified() {
                                Ok(t) => t,
                                Err(_) => {
                                    self.text_messages.warnings.push("Unable to get modification date from file ".to_string() + current_file_name.as_str());
                                    SystemTime::now()
                                } // Permissions Denied
                            },
                        };

                        self.files_with_identical_size.entry(metadata.len()).or_insert_with(Vec::new);
                        self.files_with_identical_size.get_mut(&metadata.len()).unwrap().push(fe);

                        self.information.number_of_checked_files += 1;
                    } else {
                        self.information.number_of_ignored_files += 1;
                    }
                } else {
                    // Probably this is symbolic links so we are free to ignore this
                    self.information.number_of_ignored_things += 1;
                }
            }
        }

        // Remove files with unique size
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

        if !self.files_with_identical_size.is_empty() {
            file.write_all(b"-------------------------------------------------Files with same size-------------------------------------------------\n").unwrap();
            file.write_all(
                ("Found ".to_string()
                    + self.information.number_of_duplicated_files_by_size.to_string().as_str()
                    + " duplicated files which in "
                    + self.information.number_of_groups_by_size.to_string().as_str()
                    + " groups which takes "
                    + self.information.lost_space_by_size.file_size(options::BINARY).unwrap().as_str()
                    + ".\n")
                    .as_bytes(),
            )
            .unwrap();
            for (size, files) in self.files_with_identical_size.iter().rev() {
                file.write_all(b"\n---- Size ").unwrap();
                file.write_all(size.file_size(options::BINARY).unwrap().as_bytes()).unwrap();
                file.write_all((" (".to_string() + size.to_string().as_str() + ")").as_bytes()).unwrap();
                file.write_all((" - ".to_string() + files.len().to_string().as_str() + " files").as_bytes()).unwrap();
                file.write_all(b"\n").unwrap();
                for file_entry in files {
                    file.write_all((file_entry.path.clone() + "\n").as_bytes()).unwrap();
                }
            }

            if !self.files_with_identical_hashes.is_empty() {
                file.write_all(b"-------------------------------------------------Files with same hashes-------------------------------------------------\n").unwrap();
                file.write_all(
                    ("Found ".to_string()
                        + self.information.number_of_duplicated_files_by_hash.to_string().as_str()
                        + " duplicated files which in "
                        + self.information.number_of_groups_by_hash.to_string().as_str()
                        + " groups which takes "
                        + self.information.lost_space_by_hash.file_size(options::BINARY).unwrap().as_str()
                        + ".\n")
                        .as_bytes(),
                )
                .unwrap();
                for (size, files) in self.files_with_identical_hashes.iter().rev() {
                    for vector in files {
                        file.write_all(b"\n---- Size ").unwrap();
                        file.write_all(size.file_size(options::BINARY).unwrap().as_bytes()).unwrap();
                        file.write_all((" (".to_string() + size.to_string().as_str() + ")").as_bytes()).unwrap();
                        file.write_all((" - ".to_string() + vector.len().to_string().as_str() + " files").as_bytes()).unwrap();
                        file.write_all(b"\n").unwrap();
                        for file_entry in vector {
                            file.write_all((file_entry.path.clone() + "\n").as_bytes()).unwrap();
                        }
                    }
                }
            }
        } else {
            file.write_all(b"Not found any empty folders.").unwrap();
        }
        Common::print_time(start_time, SystemTime::now(), "save_results_to_file".to_string());
        true
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
                        self.text_messages.warnings.push("Unable to check hash of file ".to_string() + file_entry.1.path.as_str());
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
                            self.text_messages.warnings.push("Error happened when checking hash of file ".to_string() + file_entry.1.path.as_str());
                            error_reading_file = true;
                            break;
                        }
                    };
                    if n == 0 {
                        break;
                    }
                    self.information.bytes_read_when_hashing += n as u64;
                    hasher.update(&buffer[..n]);
                }
                if !error_reading_file {
                    let hash_string: String = hasher.finalize().to_hex().to_string();
                    hashmap_with_hash.entry(hash_string.to_string()).or_insert_with(Vec::new);
                    hashmap_with_hash.get_mut(hash_string.as_str()).unwrap().push(file_entry.1.to_owned());
                }
            }
            for hash_entry in hashmap_with_hash {
                if hash_entry.1.len() > 1 {
                    self.files_with_identical_hashes.entry(*entry.0).or_insert_with(Vec::new);
                    self.files_with_identical_hashes.get_mut(entry.0).unwrap().push(hash_entry.1);
                }
            }
        }

        for (size, vector) in &self.files_with_identical_hashes {
            for vec_file_entry in vector {
                self.information.number_of_duplicated_files_by_hash += vec_file_entry.len() - 1;
                self.information.number_of_groups_by_hash += 1;
                self.information.lost_space_by_hash += (vec_file_entry.len() as u64 - 1) * size;
            }
        }

        Common::print_time(start_time, SystemTime::now(), "check_files_hash".to_string());
    }

    #[allow(dead_code)]
    #[allow(unreachable_code)]
    /// Setting include directories, panics when there is not directories available
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
        println!("Allowed extensions - {:?}", self.allowed_extensions);
        println!("Excluded items - {:?}", self.excluded_items);
        println!("Excluded directories - {:?}", self.excluded_directories);
        println!("Recursive search - {}", self.recursive_search.to_string());
        println!("Minimum file size - {:?}", self.min_file_size);
        println!("-----------------------------------------");
    }

    /// Print information's about duplicated entries
    pub fn print_duplicated_entries(&self) {
        let start_time: SystemTime = SystemTime::now();
        let mut number_of_files: u64 = 0;
        let mut number_of_groups: u64 = 0;

        match self.check_method {
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
            CheckingMethod::SIZE => {
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
            CheckingMethod::NONE => {
                panic!("Checking Method shouldn't be ever set to NONE");
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
        if self.recursive_search {
            // This is only point which can't be done when recursive search is disabled.
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
        }

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
            self.text_messages.errors.push("Optimize Directories ERROR: Excluded directories overlaps all included directories.".to_string());
            return false;
        }

        // Not needed, but better is to have sorted everything
        self.excluded_directories.sort();
        self.included_directories.sort();
        Common::print_time(start_time, SystemTime::now(), "optimize_directories".to_string());
        true
    }

    fn delete_files(&mut self) {
        let start_time: SystemTime = SystemTime::now();

        match self.check_method {
            CheckingMethod::HASH => {
                for entry in &self.files_with_identical_hashes {
                    for vector in entry.1 {
                        let tuple: (u64, usize, usize) = delete_files(&vector, &self.delete_method, &mut self.text_messages.warnings);
                        self.information.gained_space += tuple.0;
                        self.information.number_of_removed_files += tuple.1;
                        self.information.number_of_failed_to_remove_files += tuple.2;
                    }
                }
            }
            CheckingMethod::SIZE => {
                for entry in &self.files_with_identical_size {
                    let tuple: (u64, usize, usize) = delete_files(&entry.1, &self.delete_method, &mut self.text_messages.warnings);
                    self.information.gained_space += tuple.0;
                    self.information.number_of_removed_files += tuple.1;
                    self.information.number_of_failed_to_remove_files += tuple.2;
                }
            }
            CheckingMethod::NONE => {
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
                    warnings.push("Failed to delete".to_string() + vector[q_index].path.as_str());
                }
            };
        }
        DeleteMethod::OneNewest => {
            for (size, file) in vector.iter().enumerate() {
                let time_since_epoch = file.created_date.duration_since(UNIX_EPOCH).expect("Invalid file date").as_secs();
                if q_time == 0 || q_time < time_since_epoch {
                    q_time = time_since_epoch;
                    q_index = size;
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
                            warnings.push("Failed to delete".to_string() + vector[files.0].path.as_str());
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
                            warnings.push("Failed to delete".to_string() + vector[files.0].path.as_str());
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
