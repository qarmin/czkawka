use std::path::{Path, PathBuf};
#[cfg(target_family = "unix")]
use std::{fs, os::unix::fs::MetadataExt};

use crate::common::normalize_windows_path;
use crate::common_messages::Messages;
use crate::flc;

#[derive(Debug, Clone, Default)]
pub struct Directories {
    pub excluded_directories: Vec<PathBuf>,
    pub included_directories: Vec<PathBuf>,
    pub reference_directories: Vec<PathBuf>,
    pub exclude_other_filesystems: Option<bool>,
    #[cfg(target_family = "unix")]
    pub included_dev_ids: Vec<u64>,
}

impl Directories {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn set_reference_directory(&mut self, reference_directory: &[PathBuf]) -> Messages {
        let mut messages: Messages = Messages::new();

        self.reference_directories = reference_directory
            .iter()
            .filter_map(|directory| {
                let (dir, msg) = Self::canonicalize_and_clear_path(directory, false);

                messages.extend_with_another_messages(msg);

                dir
            })
            .collect::<Vec<PathBuf>>();

        messages
    }

    pub fn set_included_directory(&mut self, included_directory: Vec<PathBuf>) -> Messages {
        let mut messages: Messages = Messages::new();

        if included_directory.is_empty() {
            messages.errors.push(flc!("core_missing_no_chosen_included_directory"));
            return messages;
        }

        let directories: Vec<PathBuf> = included_directory;

        let mut checked_directories: Vec<PathBuf> = Vec::new();
        for directory in directories {
            let (dir, msg) = Self::canonicalize_and_clear_path(&directory, false);

            messages.extend_with_another_messages(msg);

            if let Some(dir) = dir {
                checked_directories.push(dir);
            }
        }

        if checked_directories.is_empty() {
            messages.warnings.push(flc!("core_included_directory_zero_valid_directories"));
            return messages;
        }

        self.included_directories = checked_directories;

        messages
    }

    pub fn set_excluded_directory(&mut self, excluded_directory: Vec<PathBuf>) -> Messages {
        let mut messages: Messages = Messages::new();

        if excluded_directory.is_empty() {
            return messages;
        }

        let directories: Vec<PathBuf> = excluded_directory;

        let mut checked_directories: Vec<PathBuf> = Vec::new();
        for directory in directories {
            let directory_as_string = directory.to_string_lossy();
            if directory_as_string == "/" {
                messages.errors.push(flc!("core_excluded_directory_pointless_slash"));
                break;
            }

            let (dir, msg) = Self::canonicalize_and_clear_path(&directory, true);

            messages.extend_with_another_messages(msg);

            if let Some(dir) = dir {
                checked_directories.push(dir);
            }
        }
        self.excluded_directories = checked_directories;

        messages
    }

    fn canonicalize_and_clear_path(directory: &Path, is_excluded: bool) -> (Option<PathBuf>, Messages) {
        let mut messages = Messages::new();
        let mut directory = directory.to_path_buf();
        if !directory.exists() {
            if !is_excluded {
                messages.warnings.push(flc!("core_directory_must_exists", path = directory.to_string_lossy().to_string()));
            }
            return (None, messages);
        }

        if !directory.is_dir() {
            messages
                .warnings
                .push(flc!("core_directory_must_be_directory", path = directory.to_string_lossy().to_string()));
            return (None, messages);
        }

        // Try to canonicalize them
        if cfg!(windows) {
            // Only canonicalize if it's not a network path
            // This can be done by checking if path starts with \\?\UNC\
            if let Ok(dir_can) = directory.canonicalize() {
                let dir_can_str = dir_can.to_string_lossy().to_string();
                if let Some(dir_can_str) = dir_can_str.strip_prefix(r"\\?\") {
                    if dir_can_str.chars().nth(1) == Some(':') {
                        directory = PathBuf::from(dir_can_str);
                    }
                }
            }
        } else {
            if let Ok(dir) = directory.canonicalize() {
                directory = dir;
            }
        }
        (Some(directory), messages)
    }

    #[cfg(target_family = "unix")]
    pub fn set_exclude_other_filesystems(&mut self, exclude_other_filesystems: bool) {
        self.exclude_other_filesystems = Some(exclude_other_filesystems);
    }

    pub fn optimize_directories(&mut self, recursive_search: bool) -> Messages {
        let mut messages: Messages = Messages::new();

        let mut optimized_included: Vec<PathBuf> = Vec::new();
        let mut optimized_excluded: Vec<PathBuf> = Vec::new();

        if cfg!(target_family = "windows") {
            self.included_directories = self.included_directories.iter().map(normalize_windows_path).collect();
            self.excluded_directories = self.excluded_directories.iter().map(normalize_windows_path).collect();
            self.reference_directories = self.reference_directories.iter().map(normalize_windows_path).collect();
        }

        // Remove duplicated entries like: "/", "/"

        self.excluded_directories.sort_unstable();
        self.included_directories.sort_unstable();
        self.reference_directories.sort_unstable();

        self.excluded_directories.dedup();
        self.included_directories.dedup();
        self.reference_directories.dedup();

        // Optimize for duplicated included directories - "/", "/home". "/home/Pulpit" to "/"
        // Do not use when not using recursive search or using
        if recursive_search && !self.exclude_other_filesystems.unwrap_or(false) {
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

        // Selecting Reference folders
        {
            let mut ref_folders = Vec::new();
            for folder in &self.reference_directories {
                if self.included_directories.iter().any(|e| folder.starts_with(e)) {
                    ref_folders.push(folder.clone());
                }
            }
            self.reference_directories = ref_folders;
        }

        if self.included_directories.is_empty() {
            messages.errors.push(flc!("core_directory_overlap"));
            return messages;
        }

        // Not needed, but better is to have sorted everything
        self.excluded_directories.sort_unstable();
        self.included_directories.sort_unstable();

        // Get device IDs for included directories
        #[cfg(target_family = "unix")]
        if self.exclude_other_filesystems() {
            for d in &self.included_directories {
                match fs::metadata(d) {
                    Ok(m) => self.included_dev_ids.push(m.dev()),
                    Err(_) => messages.errors.push(flc!("core_directory_unable_to_get_device_id", path = d.to_string_lossy().to_string())),
                }
            }
        }

        messages
    }

    pub fn is_in_referenced_directory(&self, path: &Path) -> bool {
        self.reference_directories.iter().any(|e| path.starts_with(e))
    }

    pub fn is_excluded(&self, path: &Path) -> bool {
        #[cfg(target_family = "windows")]
        let path = normalize_windows_path(path);
        // We're assuming that `excluded_directories` are already normalized
        self.excluded_directories.iter().any(|p| p.as_path() == path)
    }

    #[cfg(target_family = "unix")]
    pub fn exclude_other_filesystems(&self) -> bool {
        self.exclude_other_filesystems.unwrap_or(false)
    }

    #[cfg(target_family = "unix")]
    pub fn is_on_other_filesystems(&self, path: impl AsRef<Path>) -> Result<bool, String> {
        let path = path.as_ref();
        match fs::metadata(path) {
            Ok(m) => Ok(!self.included_dev_ids.iter().any(|&id| id == m.dev())),
            Err(_) => Err(flc!("core_directory_unable_to_get_device_id", path = path.to_string_lossy().to_string())),
        }
    }
}
