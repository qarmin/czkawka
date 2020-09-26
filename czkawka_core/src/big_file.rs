use crate::common::{Common, Messages};
use humansize::{file_size_opts as options, FileSize};
use std::collections::BTreeMap;
use std::fs;
use std::fs::{File, Metadata};
use std::io::Write;
use std::path::Path;
use std::time::SystemTime;

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
    pub taken_space: u64,
    pub number_of_real_files: usize,
}
impl Info {
    pub fn new() -> Info {
        Info {
            number_of_checked_files: 0,
            number_of_checked_folders: 0,
            number_of_ignored_files: 0,
            number_of_ignored_things: 0,
            taken_space: 0,
            number_of_real_files: 0,
        }
    }
}

impl Default for Info {
    fn default() -> Self {
        Self::new()
    }
}

/// Struct with required information's to work
pub struct BigFile {
    text_messages: Messages,
    information: Info,
    big_files: BTreeMap<u64, Vec<FileEntry>>,
    excluded_items: Vec<String>,
    excluded_directories: Vec<String>,
    included_directories: Vec<String>,
    allowed_extensions: Vec<String>,
    recursive_search: bool,
    number_of_files_to_check: usize,
}

impl BigFile {
    pub fn new() -> BigFile {
        BigFile {
            text_messages: Default::default(),
            information: Info::new(),
            big_files: Default::default(),
            excluded_items: vec![],
            excluded_directories: vec![],
            included_directories: vec![],
            allowed_extensions: vec![],
            recursive_search: true,
            number_of_files_to_check: 50,
        }
    }
    pub fn get_text_messages(&self) -> &Messages {
        &self.text_messages
    }

    pub fn get_information(&self) -> &Info {
        &self.information
    }

    pub fn find_big_files(&mut self) {
        self.optimize_directories();
        self.look_for_big_files();
        self.debug_print();
    }

    pub fn set_recursive_search(&mut self, recursive_search: bool) {
        self.recursive_search = recursive_search;
    }

    /// Saving results to provided file
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

        match file.write_all(
            format!(
                "Results of searching {:?} with excluded directories {:?} and excluded items {:?}\n",
                self.included_directories, self.excluded_directories, self.excluded_items
            )
            .as_bytes(),
        ) {
            Ok(_) => (),
            Err(_) => {
                self.text_messages.errors.push("Failed to save results to file ".to_string() + file_name.as_str());
                return false;
            }
        }

        if self.information.number_of_real_files != 0 {
            file.write_all(format!("{} the biggest files.\n\n", self.information.number_of_real_files).as_bytes()).unwrap();

            for (size, files) in self.big_files.iter().rev() {
                for file_entry in files {
                    file.write_all(format!("{} ({}) -  {}\n", size.file_size(options::BINARY).unwrap(), size, file_entry.path.clone()).as_bytes()).unwrap();
                }
            }
        } else {
            file.write_all(b"Not found any empty folders.").unwrap();
        }
        Common::print_time(start_time, SystemTime::now(), "save_results_to_file".to_string());
        true
    }

    /// List of allowed extensions, only files with this extensions will be checking if are duplicates
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

    fn look_for_big_files(&mut self) {
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
                    if have_valid_extension {
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

                        self.big_files.entry(metadata.len()).or_insert_with(Vec::new);
                        self.big_files.get_mut(&metadata.len()).unwrap().push(fe);

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

        //
        let mut new_map: BTreeMap<u64, Vec<FileEntry>> = Default::default();

        for (size, vector) in self.big_files.iter().rev() {
            if self.information.number_of_real_files < self.number_of_files_to_check {
                for file in vector {
                    if self.information.number_of_real_files < self.number_of_files_to_check {
                        new_map.entry(*size).or_insert_with(Vec::new);
                        new_map.get_mut(size).unwrap().push(file.clone());
                        self.information.taken_space += size;
                        self.information.number_of_real_files += 1;
                    } else {
                        break;
                    }
                }
            } else {
                break;
            }
        }
        self.big_files = new_map;

        Common::print_time(start_time, SystemTime::now(), "look_for_big_files".to_string());
    }

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

        println!("### Other");
        println!("Big files size {} in {} groups", self.information.number_of_real_files, self.big_files.len());
        println!("Allowed extensions - {:?}", self.allowed_extensions);
        println!("Excluded items - {:?}", self.excluded_items);
        println!("Included directories - {:?}", self.included_directories);
        println!("Excluded directories - {:?}", self.excluded_directories);
        println!("Recursive search - {}", self.recursive_search.to_string());
        println!("Number of files to check - {:?}", self.number_of_files_to_check);
        println!("-----------------------------------------");
    }

    pub fn set_number_of_files_to_check(&mut self, number_of_files_to_check: usize) {
        self.number_of_files_to_check = number_of_files_to_check;
    }

    /// Print information's about duplicated entries
    /// Only needed for CLI
    pub fn print_duplicated_entries(&self) {
        let start_time: SystemTime = SystemTime::now();
        println!("Found {} files which take {}:", self.information.number_of_real_files, self.information.taken_space.file_size(options::BINARY).unwrap());
        for (size, vector) in self.big_files.iter().rev() {
            // TODO Align all to same width
            for entry in vector {
                println!("{} ({}) -  {}", size.file_size(options::BINARY).unwrap(), size, entry.path);
            }
        }
        Common::print_time(start_time, SystemTime::now(), "print_duplicated_entries".to_string());
    }

    /// Setting excluded items which needs to contains * wildcrard
    /// Are a lot of slower than absolute path, so it should be used to heavy
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

    /// Remove unused entries when included or excluded overlaps with each other or are duplicated etc.
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

    /// Setting include directories, at least one must be provided
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

    /// Setting absolute path to exclude
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
}
impl Default for BigFile {
    fn default() -> Self {
        Self::new()
    }
}
