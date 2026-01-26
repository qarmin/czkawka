pub mod basic_gui_cli;
pub mod cache;
pub mod codec_detection;
pub mod config_cache_path;
pub mod consts;
pub mod decoder_strategy;
pub mod dir_traversal;
pub mod directories;
pub mod extensions;
pub mod ffmpeg_utils;
pub mod gpu_detection;
pub mod image;
pub mod items;
pub mod logger;
pub mod model;
pub mod pre_scan;
pub mod process_utils;
pub mod progress_data;
pub mod progress_stop_handler;
pub mod temporal_segmentation;
pub mod tool_data;
pub mod traits;
pub mod video_utils;

use std::cmp::Ordering;
use std::ffi::OsString;
use std::io::Error;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::Duration;
use std::{fs, io, thread};

use items::SingleExcludedItem;
use log::debug;

use crate::common::consts::DEFAULT_WORKER_THREAD_SIZE;
use crate::flc;

static NUMBER_OF_THREADS: std::sync::LazyLock<Mutex<Option<usize>>> = std::sync::LazyLock::new(|| Mutex::new(None));
static ALL_AVAILABLE_THREADS: std::sync::LazyLock<Mutex<Option<usize>>> = std::sync::LazyLock::new(|| Mutex::new(None));

const MAX_SYMLINK_HARDLINK_ATTEMPTS: u8 = 5;

pub fn get_number_of_threads() -> usize {
    let data = NUMBER_OF_THREADS.lock().expect("Cannot fail").expect("Should be set before get");
    if data >= 1 { data } else { get_all_available_threads() }
}

pub fn get_all_available_threads() -> usize {
    let mut available_threads = ALL_AVAILABLE_THREADS.lock().expect("Cannot fail");

    if let Some(available_threads) = *available_threads {
        available_threads
    } else {
        let threads = thread::available_parallelism().map(std::num::NonZeroUsize::get).unwrap_or(1);
        *available_threads = Some(threads);
        threads
    }
}

pub fn set_number_of_threads(thread_number: usize) {
    *NUMBER_OF_THREADS.lock().expect("Cannot fail") = Some(thread_number);

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

pub fn check_if_folder_contains_only_empty_folders<P: AsRef<Path>>(path: P) -> Result<(), String> {
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

pub fn remove_folder_if_contains_only_empty_folders<P: AsRef<Path>>(path: P, remove_to_trash: bool) -> Result<(), String> {
    check_if_folder_contains_only_empty_folders(&path)?;

    let path = path.as_ref();

    if remove_to_trash {
        trash::delete(path).map_err(|e| format!("Cannot move folder \"{}\" to trash, reason {e}", path.to_string_lossy()))
    } else {
        fs::remove_dir_all(path).map_err(|e| format!("Cannot remove directory \"{}\", reason {e}", path.to_string_lossy()))
    }
}

pub fn remove_single_file<P: AsRef<Path>>(full_path: P, remove_to_trash: bool) -> Result<(), String> {
    if remove_to_trash {
        if let Err(e) = trash::delete(&full_path) {
            return Err(flc!(
                "core_error_moving_to_trash",
                file = full_path.as_ref().to_string_lossy().to_string(),
                error = e.to_string()
            ));
        }
    } else {
        if let Err(e) = fs::remove_file(&full_path) {
            return Err(flc!("core_error_removing", file = full_path.as_ref().to_string_lossy().to_string(), error = e.to_string()));
        }
    }
    Ok(())
}

pub fn remove_single_folder(full_path: &str, remove_to_trash: bool) -> Result<(), String> {
    if remove_to_trash {
        if let Err(e) = trash::delete(full_path) {
            return Err(flc!("core_error_moving_to_trash", file = full_path, error = e.to_string()));
        }
    } else {
        if let Err(e) = fs::remove_dir_all(full_path) {
            return Err(flc!("core_error_removing", file = full_path, error = e.to_string()));
        }
    }
    Ok(())
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

pub fn format_time(duration: Duration) -> String {
    let hours = duration.as_secs() / 3600;
    let minutes = duration.as_secs() % 3600 / 60;
    let secs = duration.as_secs() % 60;
    let millis = duration.subsec_millis();
    if hours == 0 && minutes == 0 && secs == 0 {
        format!("{millis}ms")
    } else if hours == 0 && minutes == 0 {
        if millis / 10 == 0 { format!("{secs}s") } else { format!("{secs}.{:02}s", millis / 10) }
    } else if hours == 0 {
        if secs == 0 { format!("{minutes}m") } else { format!("{minutes}m {secs}s") }
    } else {
        if secs == 0 && minutes == 0 {
            format!("{hours}h")
        } else if secs == 0 {
            format!("{hours}h {minutes}m")
        } else {
            format!("{hours}h {minutes}m {secs}s")
        }
    }
}

pub(crate) fn create_crash_message(library_name: &str, file_path: &str, home_library_url: &str) -> String {
    format!(
        "{library_name} library crashed when opening \"{file_path}\", please check if this is fixed with the latest version of {library_name} and if it is not fixed, please report bug here - {home_library_url}"
    )
}

#[expect(clippy::string_slice)]
#[expect(clippy::indexing_slicing)]
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

#[expect(clippy::string_slice)] // Is in char boundary
pub fn normalize_windows_path<P: AsRef<Path>>(path_to_change: P) -> PathBuf {
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

// Function to create hardlink, when destination exists
// This is always true in this app, because creating hardlink, to newly created file is pointless
pub fn make_hard_link<P: AsRef<Path>, Q: AsRef<Path>>(src: P, dst: Q) -> io::Result<()> {
    let src = src.as_ref();
    let dst = dst.as_ref();
    let dst_dir = dst.parent().ok_or_else(|| Error::other("No parent"))?;
    let mut temp;
    let mut attempts = MAX_SYMLINK_HARDLINK_ATTEMPTS;
    loop {
        temp = dst_dir.join(format!("{}.czkawka_tmp", rand::random::<u128>()));
        if !temp.exists() {
            break;
        }
        attempts -= 1;
        if attempts == 0 {
            return Err(Error::other("Cannot choose temporary file for hardlink creation"));
        }
    }
    fs::rename(dst, temp.as_path())?;
    match fs::hard_link(src, dst) {
        Ok(()) => {
            fs::remove_file(&temp)?;
            Ok(())
        }
        Err(e) => {
            let _ = fs::rename(&temp, dst);
            Err(e)
        }
    }
}

#[cfg(any(target_family = "unix", target_family = "windows"))]
pub fn make_file_symlink<P: AsRef<Path>, Q: AsRef<Path>>(src: P, dst: Q) -> io::Result<()> {
    let src = src.as_ref();
    let dst = dst.as_ref();
    let dst_dir = dst.parent().ok_or_else(|| Error::other("No parent"))?;
    let mut temp;
    let mut attempts = MAX_SYMLINK_HARDLINK_ATTEMPTS;
    loop {
        temp = dst_dir.join(format!("{}.czkawka_tmp", rand::random::<u128>()));
        if !temp.exists() {
            break;
        }
        attempts -= 1;
        if attempts == 0 {
            return Err(Error::other("Cannot choose temporary file for symlink creation"));
        }
    }
    fs::rename(dst, temp.as_path())?;
    let result: Result<_, _>;
    #[cfg(target_family = "unix")]
    {
        result = std::os::unix::fs::symlink(src, dst);
    }
    #[cfg(target_family = "windows")]
    {
        result = std::os::windows::fs::symlink_file(src, dst);
    }
    match result {
        Ok(()) => {
            fs::remove_file(&temp)?;
            Ok(())
        }
        Err(e) => {
            let _ = fs::rename(&temp, dst);
            Err(e)
        }
    }
}

#[cfg(not(any(target_family = "unix", target_family = "windows")))]
pub fn make_file_symlink<P: AsRef<Path>, Q: AsRef<Path>>(src: P, dst: Q) -> io::Result<()> {
    Err(Error::new(io::ErrorKind::Other, "Soft links are not supported on this platform"))
}

pub fn debug_save_file(path: &str, data: &str) {
    use std::io::Write;
    if let Ok(mut f) = fs::OpenOptions::new().create(true).append(true).open(path) {
        let _ = writeln!(f, "{data}");
    }
}

#[cfg(test)]
mod test {
    use std::fs::{File, Metadata, read_dir};
    use std::io::Write;
    #[cfg(target_family = "unix")]
    use std::os::unix::fs::MetadataExt;

    use tempfile::tempdir;

    use super::*;
    use crate::common::items::new_excluded_item;

    #[cfg(target_family = "unix")]
    fn assert_inode(before: &Metadata, after: &Metadata) {
        assert_eq!(before.ino(), after.ino());
    }

    #[cfg(target_family = "windows")]
    fn assert_inode(_: &Metadata, _: &Metadata) {}

    #[cfg(target_family = "unix")]
    fn assert_different_inode(before: &Metadata, after: &Metadata) {
        assert_ne!(before.ino(), after.ino());
    }

    #[cfg(target_family = "windows")]
    fn assert_different_inode(_before: &Metadata, _after: &Metadata) {}

    #[test]
    fn test_make_hard_link() -> io::Result<()> {
        // Test 1: Basic hardlink creation
        {
            let dir = tempfile::Builder::new().tempdir()?;
            let (src, dst) = (dir.path().join("a"), dir.path().join("b"));
            File::create(&src)?;
            let metadata = fs::metadata(&src)?;
            File::create(&dst)?;
            let dst_metadata_before = fs::metadata(&dst)?;

            assert_different_inode(&metadata, &dst_metadata_before);

            make_hard_link(&src, &dst)?;

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
        }

        // Test 2: Hardlink creation fails when source doesn't exist
        {
            let dir = tempfile::Builder::new().tempdir()?;
            let (src, dst) = (dir.path().join("a"), dir.path().join("b"));
            File::create(&dst)?;
            let metadata = fs::metadata(&dst)?;

            assert!(make_hard_link(&src, &dst).is_err());

            assert_inode(&metadata, &fs::metadata(&dst)?);
            assert_eq!(metadata.permissions(), fs::metadata(&dst)?.permissions());
            assert_eq!(metadata.modified()?, fs::metadata(&dst)?.modified()?);

            assert_eq!(vec![dst], read_dir(&dir)?.flatten().map(|e| e.path()).collect::<Vec<PathBuf>>());
        }

        // Test 3: Hardlink with content preservation
        {
            let dir = tempfile::Builder::new().tempdir()?;
            let (src, dst) = (dir.path().join("src_file"), dir.path().join("dst_file"));
            let content = "test content for hardlink";
            {
                let mut f = File::create(&src)?;
                writeln!(f, "{content}")?;
            }
            {
                let mut f = File::create(&dst)?;
                writeln!(f, "old content")?;
            }

            let src_metadata = fs::metadata(&src)?;
            let dst_metadata_before = fs::metadata(&dst)?;

            assert_different_inode(&src_metadata, &dst_metadata_before);

            make_hard_link(&src, &dst)?;

            let src_content = fs::read_to_string(&src)?;
            let dst_content = fs::read_to_string(&dst)?;
            assert_eq!(src_content, dst_content);
            assert_eq!(src_content, format!("{content}\n"));
            assert_inode(&src_metadata, &fs::metadata(&dst)?);
        }

        // Test 4: Hardlink on readonly file
        #[cfg(target_family = "unix")]
        {
            let dir = tempfile::Builder::new().tempdir()?;
            let (src, dst) = (dir.path().join("readonly_src"), dir.path().join("readonly_dst"));

            {
                let mut f = File::create(&src)?;
                writeln!(f, "readonly content")?;
            }

            let mut perms = fs::metadata(&src)?.permissions();
            perms.set_readonly(true);
            fs::set_permissions(&src, perms)?;

            assert!(fs::metadata(&src)?.permissions().readonly());

            {
                let mut f = File::create(&dst)?;
                writeln!(f, "dst content")?;
            }

            let src_metadata_before = fs::metadata(&src)?;
            let dst_metadata_before = fs::metadata(&dst)?;

            assert_different_inode(&src_metadata_before, &dst_metadata_before);

            make_hard_link(&src, &dst).unwrap();

            assert_inode(&src_metadata_before, &fs::metadata(&dst)?);
            assert_eq!(fs::read_to_string(&src)?, fs::read_to_string(&dst)?);

            assert!(fs::metadata(&src)?.permissions().readonly());
            assert!(fs::metadata(&dst)?.permissions().readonly());
        }

        // Test 5: Hardlink on readonly destination file
        #[cfg(target_family = "unix")]
        {
            let dir = tempfile::Builder::new().tempdir()?;
            let (src, dst) = (dir.path().join("src_normal"), dir.path().join("dst_readonly"));

            {
                let mut f = File::create(&src)?;
                writeln!(f, "source content")?;
            }

            {
                let mut f = File::create(&dst)?;
                writeln!(f, "destination content")?;
            }
            let mut perms = fs::metadata(&dst)?.permissions();
            perms.set_readonly(true);
            fs::set_permissions(&dst, perms)?;

            assert!(fs::metadata(&dst)?.permissions().readonly());

            let src_metadata = fs::metadata(&src)?;
            let dst_metadata_before = fs::metadata(&dst)?;

            assert_different_inode(&src_metadata, &dst_metadata_before);

            make_hard_link(&src, &dst).unwrap();

            assert_inode(&src_metadata, &fs::metadata(&dst)?);
            assert_eq!(fs::read_to_string(&src)?, fs::read_to_string(&dst)?);
        }

        // Test 6: Hardlink when destination doesn't exist - should fail
        {
            let dir = tempfile::Builder::new().tempdir()?;
            let (src, dst) = (dir.path().join("src"), dir.path().join("nonexistent"));
            File::create(&src)?;

            let result = make_hard_link(&src, &dst);
            assert!(result.is_err(), "Should fail when destination doesn't exist");
        }

        // Test 7: Hardlink preserves file size
        {
            let dir = tempfile::Builder::new().tempdir()?;
            let (src, dst) = (dir.path().join("large_src"), dir.path().join("large_dst"));

            let large_content = "x".repeat(10000);
            {
                let mut f = File::create(&src)?;
                write!(f, "{large_content}")?;
            }
            File::create(&dst)?;

            let src_size = fs::metadata(&src)?.len();
            let src_metadata = fs::metadata(&src)?;
            let dst_metadata_before = fs::metadata(&dst)?;

            assert_different_inode(&src_metadata, &dst_metadata_before);

            make_hard_link(&src, &dst)?;

            assert_eq!(src_size, fs::metadata(&dst)?.len());
            assert_eq!(large_content, fs::read_to_string(&dst)?);
        }

        // Test 8: Multiple hardlinks to same file
        {
            let dir = tempfile::Builder::new().tempdir()?;
            let src = dir.path().join("original");
            let dst1 = dir.path().join("link1");
            let dst2 = dir.path().join("link2");

            {
                let mut f = File::create(&src)?;
                writeln!(f, "original")?;
            }
            File::create(&dst1)?;
            File::create(&dst2)?;

            let src_metadata = fs::metadata(&src)?;
            let dst1_metadata_before = fs::metadata(&dst1)?;
            let dst2_metadata_before = fs::metadata(&dst2)?;

            // Before hardlinks - all files should have different inodes
            assert_different_inode(&src_metadata, &dst1_metadata_before);
            assert_different_inode(&src_metadata, &dst2_metadata_before);
            assert_different_inode(&dst1_metadata_before, &dst2_metadata_before);

            make_hard_link(&src, &dst1)?;
            make_hard_link(&src, &dst2)?;

            assert_inode(&src_metadata, &fs::metadata(&dst1)?);
            assert_inode(&src_metadata, &fs::metadata(&dst2)?);
        }

        Ok(())
    }

    #[cfg(any(target_family = "unix", target_family = "windows"))]
    #[test]
    fn test_make_file_symlink() -> io::Result<()> {
        let dir = tempfile::Builder::new().tempdir()?;
        let (src, dst) = (dir.path().join("a"), dir.path().join("b"));
        let content = "hello softlink";
        {
            let mut f = File::create(&src)?;
            writeln!(f, "{content}")?;
        }
        File::create(&dst)?;

        make_file_symlink(&src, &dst)?;

        let symlink_meta = fs::symlink_metadata(&dst)?;
        assert!(symlink_meta.file_type().is_symlink());

        let src_content = fs::read_to_string(&src)?;
        let dst_content = fs::read_to_string(&dst)?;
        assert_eq!(src_content, dst_content);

        let mut actual = read_dir(&dir)?.flatten().map(|e| e.path()).collect::<Vec<PathBuf>>();
        actual.sort_unstable();
        assert_eq!(vec![src, dst], actual);
        Ok(())
    }

    #[cfg(any(target_family = "unix", target_family = "windows"))]
    #[test]
    fn test_make_file_symlink_fails() -> io::Result<()> {
        let dir = tempfile::Builder::new().tempdir()?;
        let (src, dst) = (dir.path().join("a"), dir.path().join("b"));
        {
            let mut f = File::create(&dst)?;
            writeln!(f, "original")?;
        }
        let metadata = fs::metadata(&dst)?;

        match make_file_symlink(&src, &dst) {
            Err(_) => {
                assert_eq!(fs::read_to_string(&dst)?, "original\n");
                assert_eq!(metadata.permissions(), fs::metadata(&dst)?.permissions());
            }
            Ok(()) => {
                let symlink_meta = fs::symlink_metadata(&dst)?;
                assert!(symlink_meta.file_type().is_symlink());
                fs::read_to_string(&dst).unwrap_err();
            }
        }
        Ok(())
    }

    #[test]
    fn test_remove_folder_if_contains_only_empty_folders() {
        let dir = tempdir().expect("Cannot create temporary directory");
        let sub_dir = dir.path().join("sub_dir");
        fs::create_dir(&sub_dir).expect("Cannot create directory");

        // Test with empty directory
        remove_folder_if_contains_only_empty_folders(&sub_dir, false).unwrap();
        assert!(!Path::new(&sub_dir).exists());

        // Test with directory containing an empty directory
        fs::create_dir(&sub_dir).expect("Cannot create directory");
        fs::create_dir(sub_dir.join("empty_sub_dir")).expect("Cannot create directory");
        remove_folder_if_contains_only_empty_folders(&sub_dir, false).unwrap();
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

    #[test]
    fn test_format_time() {
        assert_eq!(format_time(Duration::from_millis(0)), "0ms");
        assert_eq!(format_time(Duration::from_millis(1)), "1ms");
        assert_eq!(format_time(Duration::from_millis(999)), "999ms");

        assert_eq!(format_time(Duration::from_millis(1000)), "1s");
        assert_eq!(format_time(Duration::from_millis(1234)), "1.23s");
        assert_eq!(format_time(Duration::from_millis(5678)), "5.67s");
        assert_eq!(format_time(Duration::from_secs(59)), "59s");

        assert_eq!(format_time(Duration::from_secs(60)), "1m");
        assert_eq!(format_time(Duration::from_secs(61)), "1m 1s");
        assert_eq!(format_time(Duration::from_millis(61234)), "1m 1s");
        assert_eq!(format_time(Duration::from_secs(125)), "2m 5s");
        assert_eq!(format_time(Duration::from_secs(3599)), "59m 59s");

        assert_eq!(format_time(Duration::from_secs(3600)), "1h");
        assert_eq!(format_time(Duration::from_secs(3661)), "1h 1m 1s");
        assert_eq!(format_time(Duration::from_secs(7384)), "2h 3m 4s");
        assert_eq!(format_time(Duration::from_secs(86400)), "24h");

        assert_eq!(format_time(Duration::from_millis(999)), "999ms");
        assert_eq!(format_time(Duration::from_millis(1001)), "1s");
        assert_eq!(format_time(Duration::from_millis(59999)), "59.99s");
        assert_eq!(format_time(Duration::from_millis(60000)), "1m");
        assert_eq!(format_time(Duration::from_millis(60100)), "1m");
        assert_eq!(format_time(Duration::from_millis(120000)), "2m");
    }
}
