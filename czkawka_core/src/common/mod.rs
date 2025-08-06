pub mod cache;
pub mod config_cache_path;
pub mod consts;
pub mod dir_traversal;
pub mod directories;
pub mod extensions;
pub mod image;
pub mod items;
pub mod logger;
pub mod model;
pub mod progress_data;
pub mod progress_stop_handler;
pub mod tool_data;
pub mod traits;

use std::cmp::Ordering;
use std::ffi::OsString;
use std::io::Error;
use std::path::{Path, PathBuf};
use std::{fs, io, thread};

use items::SingleExcludedItem;
use log::debug;

use crate::common::consts::{DEFAULT_WORKER_THREAD_SIZE, TEMP_HARDLINK_FILE};

static NUMBER_OF_THREADS: state::InitCell<usize> = state::InitCell::new();
static ALL_AVAILABLE_THREADS: state::InitCell<usize> = state::InitCell::new();

pub fn get_number_of_threads() -> usize {
    let data = NUMBER_OF_THREADS.get();
    if *data >= 1 { *data } else { get_all_available_threads() }
}

pub fn get_all_available_threads() -> usize {
    *ALL_AVAILABLE_THREADS.get_or_init(|| {
        let available_threads = thread::available_parallelism().map(std::num::NonZeroUsize::get).unwrap_or(1);
        ALL_AVAILABLE_THREADS.set(available_threads);
        available_threads
    })
}

pub fn set_number_of_threads(thread_number: usize) {
    NUMBER_OF_THREADS.set(thread_number);

    let additional_message = if thread_number == 0 {
        format!(
            " (0 - means that all available threads will be used({}))",
            thread::available_parallelism().map(std::num::NonZeroUsize::get).unwrap_or(1)
        )
    } else {
        "".to_string()
    };
    debug!("Number of threads set to {thread_number}{additional_message}");

    rayon::ThreadPoolBuilder::new()
        .num_threads(get_number_of_threads())
        .stack_size(DEFAULT_WORKER_THREAD_SIZE)
        .build_global()
        .expect("Cannot set number of threads");
}

pub fn check_if_folder_contains_only_empty_folders(path: impl AsRef<Path>) -> Result<(), String> {
    let path = path.as_ref();
    if !path.is_dir() {
        return Err(format!("Trying to remove folder \"{}\" which is not a directory", path.to_string_lossy()));
    }

    let mut entries_to_check = Vec::new();
    let Ok(initial_entry) = path.read_dir() else {
        return Err(format!("Cannot read directory \"{}\"", path.to_string_lossy()));
    };
    for entry in initial_entry {
        if let Ok(entry) = entry {
            entries_to_check.push(entry);
        } else {
            return Err(format!("Cannot read entry from directory \"{}\"", path.to_string_lossy()));
        }
    }
    loop {
        let Some(entry) = entries_to_check.pop() else {
            break;
        };
        let Some(file_type) = entry.file_type().ok() else {
            return Err(format!(
                "Folder contains file with unknown type \"{}\" inside \"{}\"",
                entry.path().to_string_lossy(),
                path.to_string_lossy()
            ));
        };

        if !file_type.is_dir() {
            return Err(format!("Folder contains file \"{}\" inside \"{}\"", entry.path().to_string_lossy(), path.to_string_lossy()));
        }
        let Ok(internal_read_dir) = entry.path().read_dir() else {
            return Err(format!(
                "Cannot read directory \"{}\" inside \"{}\"",
                entry.path().to_string_lossy(),
                path.to_string_lossy()
            ));
        };
        for internal_elements in internal_read_dir {
            if let Ok(internal_element) = internal_elements {
                entries_to_check.push(internal_element);
            } else {
                return Err(format!(
                    "Cannot read entry from directory \"{}\" inside \"{}\"",
                    entry.path().to_string_lossy(),
                    path.to_string_lossy()
                ));
            }
        }
    }

    Ok(())
}

pub fn remove_folder_if_contains_only_empty_folders(path: impl AsRef<Path>, remove_to_trash: bool) -> Result<(), String> {
    check_if_folder_contains_only_empty_folders(&path)?;

    let path = path.as_ref();

    if remove_to_trash {
        trash::delete(path).map_err(|e| format!("Cannot move folder \"{}\" to trash, reason {e}", path.to_string_lossy()))
    } else {
        fs::remove_dir_all(path).map_err(|e| format!("Cannot remove directory \"{}\", reason {e}", path.to_string_lossy()))
    }
}

pub fn split_path(path: &Path) -> (String, String) {
    match (path.parent(), path.file_name()) {
        (Some(dir), Some(file)) => (dir.to_string_lossy().to_string(), file.to_string_lossy().into_owned()),
        (Some(dir), None) => (dir.to_string_lossy().to_string(), String::new()),
        (None, _) => (String::new(), String::new()),
    }
}

pub fn split_path_compare(path_a: &Path, path_b: &Path) -> Ordering {
    match path_a.parent().cmp(&path_b.parent()) {
        Ordering::Equal => path_a.file_name().cmp(&path_b.file_name()),
        other => other,
    }
}

pub(crate) fn create_crash_message(library_name: &str, file_path: &str, home_library_url: &str) -> String {
    format!(
        "{library_name} library crashed when opening \"{file_path}\", please check if this is fixed with the latest version of {library_name} and if it is not fixed, please report bug here - {home_library_url}"
    )
}

#[allow(clippy::string_slice)]
pub fn regex_check(expression_item: &SingleExcludedItem, directory_name: &str) -> bool {
    if expression_item.expression_splits.is_empty() {
        return true;
    }

    // Early checking if directory contains all parts needed by expression
    for split in &expression_item.unique_extensions_splits {
        if !directory_name.contains(split) {
            return false;
        }
    }

    // `git*` shouldn't be true for `/gitsfafasfs`
    if !expression_item.expression.starts_with('*')
        && directory_name
            .find(&expression_item.expression_splits[0])
            .expect("Cannot fail, because split must exists in directory_name")
            > 0
    {
        return false;
    }
    // `*home` shouldn't be true for `/homeowner`
    if !expression_item.expression.ends_with('*')
        && !directory_name.ends_with(expression_item.expression_splits.last().expect("Cannot fail, because at least one item is available"))
    {
        return false;
    }

    // At the end we check if parts between * are correctly positioned
    let mut last_split_point = directory_name.find(&expression_item.expression_splits[0]).expect("Cannot fail, because is checked earlier");
    let mut current_index: usize = 0;
    let mut found_index: usize;
    for spl in &expression_item.expression_splits[1..] {
        found_index = match directory_name[current_index..].find(spl) {
            Some(t) => t,
            None => return false,
        };
        current_index = last_split_point + spl.len();
        last_split_point = found_index + current_index;
    }
    true
}

#[allow(clippy::string_slice)] // Is in char boundary
pub fn normalize_windows_path(path_to_change: impl AsRef<Path>) -> PathBuf {
    let path = path_to_change.as_ref();

    // Don't do anything, because network path may be case intensive
    if path.to_string_lossy().starts_with('\\') {
        return path.to_path_buf();
    }

    match path.to_str() {
        Some(path) if path.is_char_boundary(1) => {
            let replaced = path.replace('/', "\\");
            let mut new_path = OsString::new();
            if replaced[1..].starts_with(':') {
                new_path.push(replaced[..1].to_ascii_uppercase());
                new_path.push(replaced[1..].to_ascii_lowercase());
            } else {
                new_path.push(replaced.to_ascii_lowercase());
            }
            PathBuf::from(new_path)
        }
        _ => path.to_path_buf(),
    }
}

pub fn make_hard_link(src: &Path, dst: &Path) -> io::Result<()> {
    let dst_dir = dst.parent().ok_or_else(|| Error::other("No parent"))?;
    let temp = dst_dir.join(TEMP_HARDLINK_FILE);
    fs::rename(dst, temp.as_path())?;
    let result = fs::hard_link(src, dst);
    if result.is_err() {
        fs::rename(temp.as_path(), dst)?;
    }
    fs::remove_file(temp)?;
    result
}

#[cfg(test)]
mod test {
    use std::fs::{File, Metadata, read_dir};
    use std::io::Write;
    #[cfg(target_family = "windows")]
    use std::os::fs::MetadataExt;
    #[cfg(target_family = "unix")]
    use std::os::unix::fs::MetadataExt;
    use std::path::{Path, PathBuf};
    use std::{fs, io};

    use tempfile::tempdir;

    use crate::common::items::new_excluded_item;
    use crate::common::{make_hard_link, normalize_windows_path, regex_check, remove_folder_if_contains_only_empty_folders};

    #[cfg(target_family = "unix")]
    fn assert_inode(before: &Metadata, after: &Metadata) {
        assert_eq!(before.ino(), after.ino());
    }

    #[cfg(target_family = "windows")]
    fn assert_inode(_: &Metadata, _: &Metadata) {}

    #[test]
    fn test_make_hard_link() -> io::Result<()> {
        let dir = tempfile::Builder::new().tempdir()?;
        let (src, dst) = (dir.path().join("a"), dir.path().join("b"));
        File::create(&src)?;
        let metadata = fs::metadata(&src)?;
        File::create(&dst)?;

        make_hard_link(&src, &dst)?;

        assert_inode(&metadata, &fs::metadata(&dst)?);
        assert_eq!(metadata.permissions(), fs::metadata(&dst)?.permissions());
        assert_eq!(metadata.modified()?, fs::metadata(&dst)?.modified()?);
        assert_inode(&metadata, &fs::metadata(&src)?);
        assert_eq!(metadata.permissions(), fs::metadata(&src)?.permissions());
        assert_eq!(metadata.modified()?, fs::metadata(&src)?.modified()?);

        let mut actual = read_dir(&dir)?.flatten().map(|e| e.path()).collect::<Vec<PathBuf>>();
        actual.sort_unstable();
        assert_eq!(vec![src, dst], actual);
        Ok(())
    }
    #[test]
    fn test_make_hard_link_fails() -> io::Result<()> {
        let dir = tempfile::Builder::new().tempdir()?;
        let (src, dst) = (dir.path().join("a"), dir.path().join("b"));
        File::create(&dst)?;
        let metadata = fs::metadata(&dst)?;

        assert!(make_hard_link(&src, &dst).is_err());

        assert_inode(&metadata, &fs::metadata(&dst)?);
        assert_eq!(metadata.permissions(), fs::metadata(&dst)?.permissions());
        assert_eq!(metadata.modified()?, fs::metadata(&dst)?.modified()?);

        assert_eq!(vec![dst], read_dir(&dir)?.flatten().map(|e| e.path()).collect::<Vec<PathBuf>>());
        Ok(())
    }

    #[test]
    fn test_remove_folder_if_contains_only_empty_folders() {
        let dir = tempdir().expect("Cannot create temporary directory");
        let sub_dir = dir.path().join("sub_dir");
        fs::create_dir(&sub_dir).expect("Cannot create directory");

        // Test with empty directory
        assert!(remove_folder_if_contains_only_empty_folders(&sub_dir, false).is_ok());
        assert!(!Path::new(&sub_dir).exists());

        // Test with directory containing an empty directory
        fs::create_dir(&sub_dir).expect("Cannot create directory");
        fs::create_dir(sub_dir.join("empty_sub_dir")).expect("Cannot create directory");
        assert!(remove_folder_if_contains_only_empty_folders(&sub_dir, false).is_ok());
        assert!(!Path::new(&sub_dir).exists());

        // Test with directory containing a file
        fs::create_dir(&sub_dir).expect("Cannot create directory");
        let mut file = File::create(sub_dir.join("file.txt")).expect("Cannot create file");
        writeln!(file, "Hello, world!").expect("Cannot write to file");
        assert!(remove_folder_if_contains_only_empty_folders(&sub_dir, false).is_err());
        assert!(Path::new(&sub_dir).exists());
    }

    #[test]
    fn test_regex() {
        assert!(regex_check(&new_excluded_item("*"), "/home/rafal"));
        assert!(regex_check(&new_excluded_item("*home*"), "/home/rafal"));
        assert!(regex_check(&new_excluded_item("*home"), "/home"));
        assert!(regex_check(&new_excluded_item("*home/"), "/home/"));
        assert!(regex_check(&new_excluded_item("*home/*"), "/home/"));
        assert!(regex_check(&new_excluded_item("*.git*"), "/home/.git"));
        assert!(regex_check(&new_excluded_item("*/home/rafal*rafal*rafal*rafal*"), "/home/rafal/rafalrafalrafal"));
        assert!(regex_check(&new_excluded_item("AAA"), "AAA"));
        assert!(regex_check(&new_excluded_item("AAA*"), "AAABDGG/QQPW*"));
        assert!(!regex_check(&new_excluded_item("*home"), "/home/"));
        assert!(!regex_check(&new_excluded_item("*home"), "/homefasfasfasfasf/"));
        assert!(!regex_check(&new_excluded_item("*home"), "/homefasfasfasfasf"));
        assert!(!regex_check(&new_excluded_item("rafal*afal*fal"), "rafal"));
        assert!(!regex_check(&new_excluded_item("rafal*a"), "rafal"));
        assert!(!regex_check(&new_excluded_item("AAAAAAAA****"), "/AAAAAAAAAAAAAAAAA"));
        assert!(!regex_check(&new_excluded_item("*.git/*"), "/home/.git"));
        assert!(!regex_check(&new_excluded_item("*home/*koc"), "/koc/home/"));
        assert!(!regex_check(&new_excluded_item("*home/"), "/home"));
        assert!(!regex_check(&new_excluded_item("*TTT"), "/GGG"));
        assert!(regex_check(
            &new_excluded_item("*/home/*/.local/share/containers"),
            "/var/home/roman/.local/share/containers"
        ));

        if cfg!(target_family = "windows") {
            assert!(regex_check(&new_excluded_item("*\\home"), "C:\\home"));
            assert!(regex_check(&new_excluded_item("*/home"), "C:\\home"));
        }
    }

    #[test]
    fn test_windows_path() {
        assert_eq!(PathBuf::from("C:\\path.txt"), normalize_windows_path("c:/PATH.tXt"));
        assert_eq!(PathBuf::from("H:\\reka\\weza\\roman.txt"), normalize_windows_path("h:/RekA/Weza\\roMan.Txt"));
        assert_eq!(PathBuf::from("T:\\a"), normalize_windows_path("T:\\A"));
        assert_eq!(PathBuf::from("\\\\aBBa"), normalize_windows_path("\\\\aBBa"));
        assert_eq!(PathBuf::from("a"), normalize_windows_path("a"));
        assert_eq!(PathBuf::from(""), normalize_windows_path(""));
    }
}
