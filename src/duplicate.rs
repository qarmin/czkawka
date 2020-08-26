// Todo, należy upewnić się, że ma wystarczające uprawnienia do odczytu i usuwania
use std::collections::HashMap;
use std::process;

pub struct DuplicateFinder {
    number_of_checked_files: u64,
    number_of_files_which_has_duplicated_entries: u64,
    number_of_duplicated_files: u64,
    // files : Vec<HashMap<FileEntry, Vec<FileEntry>>>,
    files: HashMap<u64, Vec<FileEntry>>,
    // files : Vec<Vec<FileEntry>>,
    excluded_directories: Vec<String>,
    included_directories: Vec<String>,
}

impl DuplicateFinder {
    pub fn new() -> DuplicateFinder {
        DuplicateFinder {
            number_of_checked_files: 0,
            number_of_files_which_has_duplicated_entries: 0,
            number_of_duplicated_files: 0,
            files: Default::default(),
            excluded_directories: vec![],
            included_directories: vec![],
        }
    }
    pub fn clear(&mut self) {
        self.number_of_checked_files = 0;
        self.number_of_files_which_has_duplicated_entries = 0;
        self.number_of_duplicated_files = 0;
        self.files.clear();
        self.excluded_directories.clear();
        self.included_directories.clear();
    }
    pub fn find_duplicates(&mut self) {}
    pub fn save_to_file(&self) {}

    // Setting include directories, panics when there is not directories available
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
            checked_directories.push(directory);
        }

        if checked_directories.len() == 0 {
            println!("Not found even one correct path to include.");
            process::exit(1);
        }

        self.included_directories = checked_directories;

        println!("{:?}", self.included_directories);
    }

    //
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
            checked_directories.push(directory);
        }

        self.included_directories = checked_directories;

        println!("{:?}", &self.excluded_directories);
    }
    pub fn debug_print(&self) {
        println!("Number of all checked files - {}", self.number_of_checked_files);
        println!(
            "Number of all files with duplicates - {}",
            self.number_of_files_which_has_duplicated_entries
        );
        println!("Number of duplicated files - {}", self.number_of_duplicated_files);
        println!("Files list - {}", self.files.len());
        println!("Exclued  - {:?}", self.excluded_directories);
        println!("Included directiories - {:?}", self.included_directories);
    }
}

struct FileEntry {
    file_path: String,
    file_size: u64,
}
