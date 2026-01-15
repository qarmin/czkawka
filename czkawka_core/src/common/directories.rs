use std::path::{Path, PathBuf};
#[cfg(target_family = "unix")]
use std::{fs, os::unix::fs::MetadataExt};

use crate::common::normalize_windows_path;
use crate::common::traits::ResultEntry;
use crate::flc;
use crate::helpers::messages::Messages;

#[derive(Debug, Clone, Default)]
pub struct Directories {
    pub(crate) excluded_directories: Vec<PathBuf>,
    pub(crate) included_directories: Vec<PathBuf>,
    pub(crate) reference_directories: Vec<PathBuf>,
    pub(crate) excluded_files: Vec<PathBuf>,
    pub(crate) included_files: Vec<PathBuf>,
    pub(crate) reference_files: Vec<PathBuf>,
    pub(crate) exclude_other_filesystems: Option<bool>,
    #[cfg(target_family = "unix")]
    pub(crate) included_dev_ids: Vec<u64>,
}

impl Directories {
    pub fn new() -> Self {
        Default::default()
    }

    pub(crate) fn set_reference_paths(&mut self, reference_paths: Vec<PathBuf>) -> Messages {
        self.reference_files = Vec::new();
        self.reference_directories = Vec::new();
        self.process_paths(reference_paths, true, false)
    }

    pub(crate) fn set_included_paths(&mut self, included_paths: Vec<PathBuf>) -> Messages {
        self.included_files = Vec::new();
        self.included_directories = Vec::new();
        self.process_paths(included_paths, false, false)
    }

    pub(crate) fn set_excluded_paths(&mut self, excluded_paths: Vec<PathBuf>) -> Messages {
        self.excluded_files = Vec::new();
        self.excluded_directories = Vec::new();
        self.process_paths(excluded_paths, false, true)
    }

    fn process_paths(&mut self, paths: Vec<PathBuf>, is_reference: bool, is_excluded: bool) -> Messages {
        let mut messages: Messages = Messages::new();

        if paths.is_empty() {
            return messages;
        }

        for path in paths {
            if is_excluded && path.to_string_lossy() == "/" {
                messages.errors.push(flc!("core_excluded_paths_pointless_slash"));
                break;
            }

            let (dir, msg) = Self::canonicalize_and_clear_path(&path, false);

            messages.extend_with_another_messages(msg);

            if let Some(dir) = dir {
                match (dir.is_file(), is_reference, is_excluded) {
                    (false, true, false) => self.reference_directories.push(dir),
                    (false, false, false) => self.included_directories.push(dir),
                    (false, false, true) => self.excluded_directories.push(dir),

                    (true, true, false) => self.reference_files.push(dir),
                    (true, false, false) => self.included_files.push(dir),
                    (true, false, true) => self.excluded_files.push(dir),
                    _ => unreachable!("Invalid combination of parameters in process_paths"),
                }
            }
        }

        messages
    }

    fn canonicalize_and_clear_path(path: &Path, is_excluded: bool) -> (Option<PathBuf>, Messages) {
        let mut messages = Messages::new();
        let mut path = path.to_path_buf();
        if !path.exists() {
            if !is_excluded {
                messages.warnings.push(flc!("core_path_must_exists", path = path.to_string_lossy().to_string()));
            }
            return (None, messages);
        }

        if !path.is_dir() && !path.is_file() {
            messages.warnings.push(flc!("core_must_be_directory_or_file", path = path.to_string_lossy().to_string()));
            return (None, messages);
        }

        // Try to canonicalize them
        if cfg!(windows) {
            // Only canonicalize if it's not a network path
            // This can be done by checking if path starts with \\?\UNC\
            if let Ok(dir_can) = path.canonicalize()
                && let Some(dir_can_str) = dir_can.to_string_lossy().strip_prefix(r"\\?\")
                && dir_can_str.chars().nth(1) == Some(':')
            {
                path = PathBuf::from(dir_can_str);
            }
        } else {
            if let Ok(dir) = path.canonicalize() {
                path = dir;
            }
        }

        (Some(path), messages)
    }

    #[cfg(target_family = "unix")]
    pub(crate) fn set_exclude_other_filesystems(&mut self, exclude_other_filesystems: bool) {
        self.exclude_other_filesystems = Some(exclude_other_filesystems);
    }

    // TODO, after adding reference files, everything needs to be checked very carefully
    pub(crate) fn optimize_directories(&mut self, recursive_search: bool) -> Messages {
        let mut messages: Messages = Messages::new();

        // Remove duplicated entries like: "/", "/"
        [
            &mut self.included_directories,
            &mut self.excluded_directories,
            &mut self.reference_directories,
            &mut self.included_files,
            &mut self.excluded_files,
            &mut self.reference_files,
        ]
        .iter_mut()
        .for_each(|items| {
            if cfg!(target_family = "windows") {
                items.iter_mut().for_each(|item| {
                    *item = normalize_windows_path(item.clone());
                })
            }
            items.sort_unstable();
            items.dedup();
        });

        // Optimize for duplicated included directories - "/", "/home". "/home/Pulpit" to "/"
        // Do not use when not using recursive search
        if recursive_search && !self.exclude_other_filesystems.unwrap_or(false) {
            for kk in [&mut self.included_directories, &mut self.excluded_directories] {
                let cloned = kk.clone();
                kk.retain(|item| !cloned.iter().any(|other_item| item != other_item && item.starts_with(other_item)))
            }
        }

        // Remove included directories which are inside any excluded directory
        // Same with included files
        for kk in [&mut self.included_directories, &mut self.included_files] {
            kk.retain(|id| !self.excluded_directories.iter().any(|ed| id.starts_with(ed)))
        }

        // Also check if files are not excluded directly
        for kk in [&mut self.included_files] {
            kk.retain(|id| !self.excluded_directories.iter().any(|ed| id == ed));
        }

        // Remove non existed directories and files
        for kk in [
            &mut self.excluded_files,
            &mut self.excluded_directories,
            &mut self.included_files,
            &mut self.included_directories,
        ] {
            kk.retain(|path| path.exists())
        }

        // Excluded paths must are inside included path, because otherwise they are pointless
        // So first, removing included files, that are inside excluded directories
        // So this will allow to remove excluded directories outside included directories
        self.included_files.retain(|ifile| !self.excluded_directories.iter().any(|ed| ifile.starts_with(ed)));
        self.excluded_directories.retain(|ed| self.included_directories.iter().any(|id| ed.starts_with(id)));

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

        if self.included_directories.is_empty() && self.included_files.is_empty() {
            messages.errors.push(flc!("core_path_overlap"));
            return messages;
        }

        // Not needed, but better is to have sorted everything
        [
            &mut self.included_directories,
            &mut self.excluded_directories,
            &mut self.reference_directories,
            &mut self.included_files,
            &mut self.excluded_files,
            &mut self.reference_files,
        ]
        .iter_mut()
        .for_each(|items| items.sort_unstable());

        // Get device IDs for included directories, probably ther better solution would be to get one id per directory, but this is faster, but a little less precise
        #[cfg(target_family = "unix")]
        if self.exclude_other_filesystems() {
            for d in &self.included_directories {
                match fs::metadata(d) {
                    Ok(m) => self.included_dev_ids.push(m.dev()),
                    Err(_) => messages.errors.push(flc!("core_paths_unable_to_get_device_id", path = d.to_string_lossy().to_string())),
                }
            }
        }

        messages
    }

    pub(crate) fn is_in_referenced_directory(&self, path: &Path) -> bool {
        self.reference_directories.iter().any(|e| path.starts_with(e))
    }

    pub(crate) fn is_excluded(&self, path: &Path) -> bool {
        #[cfg(target_family = "windows")]
        let path = normalize_windows_path(path);
        // We're assuming that `excluded_directories` are already normalized
        self.excluded_directories.iter().any(|p| p.as_path() == path)
    }

    #[cfg(target_family = "unix")]
    pub(crate) fn exclude_other_filesystems(&self) -> bool {
        self.exclude_other_filesystems.unwrap_or(false)
    }

    #[cfg(target_family = "unix")]
    pub(crate) fn is_on_other_filesystems<P: AsRef<Path>>(&self, path: P) -> Result<bool, String> {
        let path = path.as_ref();
        match fs::metadata(path) {
            Ok(m) => {
                if m.file_type().is_file() && !self.included_files.is_empty() && self.included_files.contains(&path.to_path_buf()) {
                    return Ok(false); // Exact equality for included files is always allowed
                }
                Ok(!self.included_dev_ids.iter().any(|&id| id == m.dev()))
            },
            Err(_) => Err(flc!("core_paths_unable_to_get_device_id", path = path.to_string_lossy().to_string())),
        }
    }

    pub(crate) fn filter_reference_folders<T>(&self, entries_to_check: Vec<Vec<T>>) -> Vec<(T, Vec<T>)>
    where
        T: ResultEntry,
    {
        entries_to_check
            .into_iter()
            .filter_map(|vec_file_entry| {
                let (mut files_from_referenced_folders, normal_files): (Vec<_>, Vec<_>) = vec_file_entry.into_iter().partition(|e| self.is_in_referenced_directory(e.get_path()));

                if normal_files.is_empty() {
                    None
                } else {
                    files_from_referenced_folders.pop().map(|file| (file, normal_files))
                }
            })
            .collect::<Vec<(T, Vec<T>)>>()
    }
}
