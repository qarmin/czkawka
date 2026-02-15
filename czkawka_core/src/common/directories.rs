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

    pub(crate) original_excluded_paths: Vec<PathBuf>,
    pub(crate) original_included_paths: Vec<PathBuf>,
    pub(crate) original_reference_paths: Vec<PathBuf>,

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
        let paths = if cfg!(target_family = "windows") {
            reference_paths.clone().into_iter().map(normalize_windows_path).collect()
        } else {
            reference_paths.clone()
        };
        self.original_reference_paths = reference_paths;
        self.process_paths(paths, true, false)
    }

    pub(crate) fn set_included_paths(&mut self, included_paths: Vec<PathBuf>) -> Messages {
        self.included_files = Vec::new();
        self.included_directories = Vec::new();
        let paths = if cfg!(target_family = "windows") {
            included_paths.clone().into_iter().map(normalize_windows_path).collect()
        } else {
            included_paths.clone()
        };
        self.original_included_paths = included_paths;
        self.process_paths(paths, false, false)
    }

    pub(crate) fn set_excluded_paths(&mut self, excluded_paths: Vec<PathBuf>) -> Messages {
        self.excluded_files = Vec::new();
        self.excluded_directories = Vec::new();
        let paths = if cfg!(target_family = "windows") {
            excluded_paths.clone().into_iter().map(normalize_windows_path).collect()
        } else {
            excluded_paths.clone()
        };
        self.original_excluded_paths = excluded_paths;
        self.process_paths(paths, false, true)
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

            let (dir, msg) = Self::canonicalize_and_clear_path(&path, is_excluded);

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

    pub(crate) fn optimize_directories(&mut self, recursive_search: bool, skip_exist_check: bool) -> Result<Messages, Messages> {
        let mut messages: Messages = Messages::new();

        if self.original_included_paths.is_empty() {
            messages.critical = Some(flc!("core_cannot_start_scan_no_included_paths"));
            return Err(messages);
        }

        if self.included_directories.is_empty() && self.included_files.is_empty() {
            messages.critical = Some(flc!("core_skip_exist_check_all_included_paths_nonexistent"));
            return Err(messages);
        }

        // Remove duplicated entries like: "/", "/"
        for items in &mut [
            &mut self.included_directories,
            &mut self.excluded_directories,
            &mut self.reference_directories,
            &mut self.included_files,
            &mut self.excluded_files,
            &mut self.reference_files,
        ] {
            items.sort_unstable();
            items.dedup();
        }

        // Optimize for duplicated included directories - "/", "/home". "/home/Pulpit" to "/"
        // Do not use when not using recursive search
        if recursive_search && !self.exclude_other_filesystems.unwrap_or(false) {
            for kk in [&mut self.included_directories, &mut self.excluded_directories] {
                let cloned = kk.clone();
                kk.retain(|item| !cloned.iter().any(|other_item| item != other_item && item.starts_with(other_item)));
            }
        }

        // Remove included directories which are inside any excluded directory
        // Same with included files
        for kk in [&mut self.included_directories, &mut self.included_files] {
            kk.retain(|id| !self.excluded_directories.iter().any(|ed| id.starts_with(ed)));
        }

        // Remove included files inside included directories
        {
            let kk = &mut self.included_files;
            kk.retain(|id| !self.included_directories.iter().any(|ed| id.starts_with(ed)));
        }

        // Also check if files are not excluded directly
        {
            let kk = &mut self.included_files;
            kk.retain(|id| !self.excluded_directories.iter().any(|ed| id == ed));
        }

        // Remove non existed directories and files
        if !skip_exist_check {
            for kk in [
                &mut self.excluded_files,
                &mut self.excluded_directories,
                &mut self.included_files,
                &mut self.included_directories,
            ] {
                kk.retain(|path| path.exists());
            }
        }

        // Excluded paths must are inside included path, because otherwise they are pointless
        // So first, removing included files, that are inside excluded directories
        // So this will allow to remove excluded directories outside included directories
        self.included_files.retain(|ifile| !self.excluded_directories.iter().any(|ed| ifile.starts_with(ed)));
        self.excluded_directories.retain(|ed| self.included_directories.iter().any(|id| ed.starts_with(id)));

        // Selecting Reference folders
        {
            self.reference_directories.retain(|folder| self.included_directories.iter().any(|e| folder.starts_with(e)));
            self.reference_files
                .retain(|file| self.included_directories.iter().any(|e| file.starts_with(e)) || self.included_files.iter().any(|f| file == f));
        }

        // Not needed, but better is to have sorted everything
        for items in &mut [
            &mut self.included_directories,
            &mut self.excluded_directories,
            &mut self.reference_directories,
            &mut self.included_files,
            &mut self.excluded_files,
            &mut self.reference_files,
        ] {
            items.sort_unstable();
        }

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

        if self.included_directories.is_empty() && self.included_files.is_empty() {
            messages.critical = Some(flc!("core_missing_no_chosen_included_path"));
            return Err(messages);
        }

        if self.reference_directories == self.included_directories && self.included_files == self.reference_files {
            messages.critical = Some(flc!("core_reference_included_paths_same"));
            return Err(messages);
        }

        Ok(messages)
    }

    pub(crate) fn is_in_referenced_directory(&self, path: &Path) -> bool {
        self.reference_directories.iter().any(|e| path.starts_with(e));
        self.reference_files.iter().any(|e| e.as_path() == path);
        self.reference_directories.iter().any(|e| path.starts_with(e)) || self.reference_files.iter().any(|e| e.as_path() == path)
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
            }
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

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_no_included_paths_errors() {
        let mut d = Directories::new();
        let msgs = d.optimize_directories(true, true).unwrap_err();
        assert!(msgs.critical.is_some());
    }

    #[test]
    fn test_dedup_included_directories() {
        let p = PathBuf::from("/this/path/does/not/exist/dedup");
        let mut d = Directories::new();
        d.included_directories.push(p.clone());
        d.included_directories.push(p.clone());
        d.original_included_paths.push(p.clone());
        d.original_included_paths.push(p.clone());
        let _msgs = d.optimize_directories(true, true).unwrap();
        assert_eq!(d.included_directories, vec![p]);
    }

    #[test]
    fn test_excluded_removes_included_inside() {
        let base = PathBuf::from("/this/base/does/not/exist");
        let sub = base.join("sub");
        let mut d = Directories::new();
        d.included_directories.push(sub.clone());
        d.original_included_paths.push(sub);
        d.excluded_directories.push(base);
        let _msgs = d.optimize_directories(true, true).unwrap_err();
        assert_eq!(d.included_directories, Vec::<PathBuf>::new());
    }

    #[test]
    fn test_optimize_nested_included_directories_dedup() {
        let mut d = Directories::new();
        d.included_directories.push(PathBuf::from("/"));
        d.original_included_paths.push(PathBuf::from("/"));
        d.included_directories.push(PathBuf::from("/home"));
        d.original_included_paths.push(PathBuf::from("/home"));
        d.included_directories.push(PathBuf::from("/home/Pulpit"));
        d.original_included_paths.push(PathBuf::from("/home/Pulpit"));

        // use recursive_search = true and skip_exist_check = true as requested
        let msgs = d.optimize_directories(true, true).unwrap();
        // only root should remain after dedup
        assert_eq!(d.included_directories, vec![PathBuf::from("/")]);
        assert!(msgs.critical.is_none());
    }

    #[test]
    fn test_excluded_directories_pruned_to_inside_included() {
        let mut d = Directories::new();
        d.included_directories.push(PathBuf::from("/this/include"));
        d.original_included_paths.push(PathBuf::from("/this/include"));
        d.excluded_directories.push(PathBuf::from("/this/include/sub"));
        d.excluded_directories.push(PathBuf::from("/other/place"));
        d.original_excluded_paths.push(PathBuf::from("/this/include/sub"));
        d.original_excluded_paths.push(PathBuf::from("/other/place"));

        let _msgs = d.optimize_directories(true, true).unwrap();
        assert_eq!(d.included_directories, vec![PathBuf::from("/this/include")]);
        assert_eq!(d.excluded_directories, vec![PathBuf::from("/this/include/sub")]);
    }

    #[test]
    fn test_reference_dirs_and_files_retained_correctly() {
        let mut d = Directories::new();
        d.included_directories.push(PathBuf::from("/a"));
        d.original_included_paths.push(PathBuf::from("/a"));
        d.included_files.push(PathBuf::from("/a/included_file.txt"));
        d.original_included_paths.push(PathBuf::from("/a/included_file.txt"));

        d.reference_directories.push(PathBuf::from("/a/sub"));
        d.reference_directories.push(PathBuf::from("/other"));

        d.reference_files.push(PathBuf::from("/a/included_file.txt"));
        d.reference_files.push(PathBuf::from("/other/file2.txt"));

        let _msgs = d.optimize_directories(true, true).unwrap();

        assert_eq!(d.included_directories, vec![PathBuf::from("/a")]);
        assert_eq!(d.excluded_directories, Vec::<PathBuf>::new());
        assert_eq!(d.included_files, Vec::<PathBuf>::new());
        assert_eq!(d.excluded_files, Vec::<PathBuf>::new());
        assert_eq!(d.reference_directories, vec![PathBuf::from("/a/sub")]);
        assert_eq!(d.reference_files, vec![PathBuf::from("/a/included_file.txt")]);
    }

    #[test]
    fn test_reference_equals_included_error() {
        let mut d = Directories::new();
        d.included_directories.push(PathBuf::from("/same"));
        d.reference_directories.push(PathBuf::from("/same"));
        d.included_files = Vec::new();
        d.reference_files = Vec::new();

        let msgs = d.optimize_directories(true, true).unwrap_err();
        assert!(msgs.critical.is_some());
    }

    #[test]
    fn test_included_files_removed_when_equal_to_excluded_directory() {
        let mut d = Directories::new();
        d.included_directories.push(PathBuf::from("/base"));
        d.original_included_paths.push(PathBuf::from("/base"));
        d.included_files.push(PathBuf::from("/base/file"));
        d.original_included_paths.push(PathBuf::from("/base/file"));

        // excluded directory equals included file path
        d.excluded_directories.push(PathBuf::from("/base/file"));
        d.original_excluded_paths.push(PathBuf::from("/base/file"));

        let _msgs = d.optimize_directories(true, true).unwrap();
        // included_files should be removed because it equals an excluded directory
        assert!(d.included_files.is_empty());
        // excluded_directories should be retained as it's inside included_directories
        assert_eq!(d.excluded_directories, vec![PathBuf::from("/base/file")]);
        assert_eq!(d.included_directories, vec![PathBuf::from("/base")]);
    }
}
