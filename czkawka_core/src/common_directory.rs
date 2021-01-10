use crate::common::Common;
use crate::common_messages::Messages;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

#[derive(Default)]
pub struct Directories {
    pub excluded_directories: Vec<PathBuf>,
    pub included_directories: Vec<PathBuf>,
}
impl Directories {
    pub fn new() -> Self {
        Default::default()
    }

    /// Setting included directories, at least one must be provided
    pub fn set_included_directory(&mut self, included_directory: Vec<PathBuf>, text_messages: &mut Messages) -> bool {
        let start_time: SystemTime = SystemTime::now();

        if included_directory.is_empty() {
            text_messages.errors.push("At least one directory must be provided".to_string());
            return false;
        }

        let directories: Vec<PathBuf> = included_directory;

        let mut checked_directories: Vec<PathBuf> = Vec::new();
        for directory in directories {
            if directory.to_string_lossy().contains('*') {
                text_messages.warnings.push(format!("Included Directory Warning: Wildcards in path are not supported, ignoring {}", directory.display()));
                continue;
            }

            #[cfg(not(target_family = "windows"))]
            if directory.is_relative() {
                text_messages.warnings.push(format!("Included Directory Warning: Relative path are not supported, ignoring {}", directory.display()));
                continue;
            }
            #[cfg(target_family = "windows")]
            if directory.is_relative() && !directory.starts_with("\\") {
                text_messages.warnings.push(format!("Included Directory Warning: Relative path are not supported, ignoring {}", directory.display()));
                continue;
            }

            if !directory.exists() {
                text_messages.warnings.push(format!("Included Directory Warning: Provided folder path must exits, ignoring {}", directory.display()));
                continue;
            }
            if !directory.is_dir() {
                text_messages.warnings.push(format!("Included Directory Warning: Provided path must point at the directory, ignoring {}", directory.display()));
                continue;
            }
            checked_directories.push(directory);
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
    pub fn set_excluded_directory(&mut self, excluded_directory: Vec<PathBuf>, text_messages: &mut Messages) {
        let start_time: SystemTime = SystemTime::now();
        if excluded_directory.is_empty() {
            return;
        }

        let directories: Vec<PathBuf> = excluded_directory;

        let mut checked_directories: Vec<PathBuf> = Vec::new();
        for directory in directories {
            let directory_as_string = directory.to_string_lossy();
            if directory_as_string == "/" {
                text_messages.errors.push("Excluded Directory ERROR: Excluding / is pointless, because it means that no files will be scanned.".to_string());
                break;
            }
            if directory_as_string.contains('*') {
                text_messages.warnings.push(format!("Excluded Directory Warning: Wildcards in path are not supported, ignoring {}", directory.display()));
                continue;
            }
            #[cfg(not(target_family = "windows"))]
            if directory.is_relative() {
                text_messages.warnings.push(format!("Excluded Directory Warning: Relative path are not supported, ignoring {}", directory.display()));
                continue;
            }
            #[cfg(target_family = "windows")]
            if directory.is_relative() && !directory.starts_with("\\") {
                text_messages.warnings.push(format!("Excluded Directory Warning: Relative path are not supported, ignoring {}", directory.display()));
                continue;
            }

            if !directory.exists() {
                // text_messages.warnings.push(format!("Excluded Directory Warning: Provided folder path must exits, ignoring {}", directory.display()));
                continue;
            }
            if !directory.is_dir() {
                text_messages.warnings.push(format!("Excluded Directory Warning: Provided path must point at the directory, ignoring {}", directory.display()));
                continue;
            }
            checked_directories.push(directory);
        }
        self.excluded_directories = checked_directories;

        Common::print_time(start_time, SystemTime::now(), "set_excluded_directory".to_string());
    }

    /// Remove unused entries when included or excluded overlaps with each other or are duplicated etc.
    pub fn optimize_directories(&mut self, recursive_search: bool, text_messages: &mut Messages) -> bool {
        let start_time: SystemTime = SystemTime::now();

        let mut optimized_included: Vec<PathBuf> = Vec::new();
        let mut optimized_excluded: Vec<PathBuf> = Vec::new();

        if cfg!(target_family = "windows") {
            self.included_directories = self.included_directories.iter().map(Common::normalize_windows_path).collect();
            self.excluded_directories = self.excluded_directories.iter().map(Common::normalize_windows_path).collect();
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
                    optimized_excluded.push(ed_checked.clone());
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
                    optimized_included.push(id_checked.clone());
                }
            }

            self.included_directories = optimized_included;
            optimized_included = Vec::new();
            self.excluded_directories = optimized_excluded;
            optimized_excluded = Vec::new();
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
                optimized_included.push(id.clone());
            }
        }
        self.included_directories = optimized_included;
        optimized_included = Vec::new();

        // Remove non existed directories
        for id in &self.included_directories {
            let path = Path::new(id);
            if path.exists() {
                optimized_included.push(id.clone());
            }
        }

        for ed in &self.excluded_directories {
            let path = Path::new(ed);
            if path.exists() {
                optimized_excluded.push(ed.clone());
            }
        }

        self.included_directories = optimized_included;
        self.excluded_directories = optimized_excluded;
        optimized_excluded = Vec::new();

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
                optimized_excluded.push(ed.clone());
            }
        }

        self.excluded_directories = optimized_excluded;

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

    /// Checks whether a specified directory is excluded from searching
    pub fn is_excluded(&self, path: impl AsRef<Path>) -> bool {
        let path = path.as_ref();
        #[cfg(target_family = "windows")]
        let path = Common::normalize_windows_path(path);
        // We're assuming that `excluded_directories` are already normalized
        self.excluded_directories.iter().any(|p| p.as_path() == path)
    }
}
