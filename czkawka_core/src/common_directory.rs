use crate::common::Common;
use crate::common_messages::Messages;
use std::path::Path;
use std::time::SystemTime;

#[derive(Default)]
pub struct Directories {
    pub excluded_directories: Vec<String>,
    pub included_directories: Vec<String>,
}
impl Directories {
    pub fn new() -> Self {
        Default::default()
    }

    /// Setting included directories, at least one must be provided
    pub fn set_included_directory(&mut self, mut included_directory: String, text_messages: &mut Messages) -> bool {
        let start_time: SystemTime = SystemTime::now();

        if included_directory.is_empty() {
            text_messages.errors.push("At least one directory must be provided".to_string());
            return false;
        }

        included_directory = included_directory.replace("\"", "");
        let directories: Vec<String> = included_directory.split(',').map(String::from).collect();
        let mut checked_directories: Vec<String> = Vec::new();

        for directory in directories {
            let directory: String = directory.trim().to_string();

            if directory == "" {
                continue;
            }
            if directory.contains('*') {
                text_messages.warnings.push("Included Directory Warning: Wildcards in path are not supported, ignoring ".to_string() + directory.as_str());
                continue;
            }
            #[cfg(target_family = "unix")]
            if !directory.starts_with('/') {
                text_messages.warnings.push("Included Directory Warning: Relative path are not supported, ignoring ".to_string() + directory.as_str());
                continue;
            }
            #[cfg(target_family = "windows")]
            if !(directory[..directory.len()].starts_with(":/") || !directory[..directory.len()].starts_with(":\\")) {
                text_messages.warnings.push("Included Directory Warning: Relative path are not supported, ignoring ".to_string() + directory.as_str());
                continue;
            }
            if !Path::new(&directory).exists() {
                text_messages.warnings.push("Included Directory Warning: Provided folder path must exits, ignoring ".to_string() + directory.as_str());
                continue;
            }
            if !Path::new(&directory).is_dir() {
                text_messages.warnings.push("Included Directory Warning: Provided path must point at the directory, ignoring ".to_string() + directory.as_str());
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
            text_messages.errors.push("Included Directory ERROR: Not found even one correct path to included which is required.".to_string());
            return false;
        }

        self.included_directories = checked_directories;

        Common::print_time(start_time, SystemTime::now(), "set_included_directory".to_string());
        true
    }

    /// Setting absolute path to exclude
    pub fn set_excluded_directory(&mut self, mut excluded_directory: String, text_messages: &mut Messages) {
        let start_time: SystemTime = SystemTime::now();
        if excluded_directory.is_empty() {
            return;
        }

        excluded_directory = excluded_directory.replace("\"", "");
        let directories: Vec<String> = excluded_directory.split(',').map(String::from).collect();
        let mut checked_directories: Vec<String> = Vec::new();

        for directory in directories {
            let directory: String = directory.trim().to_string().replace("\\", "/");

            if directory == "" {
                continue;
            }
            if directory == "/" {
                text_messages.errors.push("Excluded Directory ERROR: Excluding / is pointless, because it means that no files will be scanned.".to_string());
                break;
            }
            if directory.contains('*') {
                text_messages.warnings.push("Excluded Directory Warning: Wildcards in path are not supported, ignoring ".to_string() + directory.as_str());
                continue;
            }
            #[cfg(target_family = "unix")]
            if !directory.starts_with('/') {
                text_messages.warnings.push("Excluded Directory Warning: Relative path are not supported, ignoring ".to_string() + directory.as_str());
                continue;
            }
            #[cfg(target_family = "windows")]
            if !(directory[..directory.len()].starts_with(":/") || !directory[..directory.len()].starts_with(":\\")) {
                text_messages.warnings.push("Excluded Directory Warning: Relative path are not supported, ignoring ".to_string() + directory.as_str());
                continue;
            }
            if !Path::new(&directory).is_dir() {
                text_messages.warnings.push("Excluded Directory Warning: Provided path must point at the directory, ignoring ".to_string() + directory.as_str());
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

        Common::print_time(start_time, SystemTime::now(), "set_excluded_directory".to_string());
    }

    /// Remove unused entries when included or excluded overlaps with each other or are duplicated etc.
    pub fn optimize_directories(&mut self, recursive_search: bool, text_messages: &mut Messages) -> bool {
        let start_time: SystemTime = SystemTime::now();

        let mut optimized_included: Vec<String> = Vec::<String>::new();
        let mut optimized_excluded: Vec<String> = Vec::<String>::new();

        // Windows(or specific EXT4 extension) doesn't recognize size of letters so we must remove one of directory e.g. - C:/h.txt, C:/H.txt
        #[cfg(target_family = "windows")]
        {
            self.included_directories = self.included_directories.iter().map(Common::prettier_windows_path).collect();
            self.excluded_directories = self.excluded_directories.iter().map(Common::prettier_windows_path).collect();
        }

        // Remove duplicated entries like: "/", "/"

        self.excluded_directories.sort();
        self.included_directories.sort();

        self.excluded_directories.dedup();
        self.included_directories.dedup();

        // Optimize for duplicated included directories - "/", "/home". "/home/Pulpit" to "/"
        if recursive_search {
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

        // Remove included directories which are inside any excluded directory
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

        // Excluded paths must are inside included path, because
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
            text_messages.errors.push("Optimize Directories ERROR: Excluded directories overlaps all included directories.".to_string());
            return false;
        }

        // Not needed, but better is to have sorted everything
        self.excluded_directories.sort();
        self.included_directories.sort();
        Common::print_time(start_time, SystemTime::now(), "optimize_directories".to_string());
        true
    }
}
