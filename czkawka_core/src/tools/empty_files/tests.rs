use std::fs;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use tempfile::TempDir;

use crate::common::tool_data::CommonData;
use crate::common::traits::Search;
use crate::tools::empty_files::EmptyFiles;

#[test]
fn test_find_empty_files() {
    let temp_dir = TempDir::new().unwrap();
    let path = temp_dir.path();

    // Create empty files
    fs::write(path.join("empty1.txt"), b"").unwrap();
    fs::write(path.join("empty2.txt"), b"").unwrap();
    fs::write(path.join("not_empty.txt"), b"content").unwrap();

    let mut finder = EmptyFiles::new();
    finder.set_included_paths(vec![path.to_path_buf()]);
    finder.set_recursive_search(true);

    let stop_flag = Arc::new(AtomicBool::new(false));
    finder.search(&stop_flag, None);

    let info = finder.get_information();
    assert_eq!(info.number_of_empty_files, 2, "Should find 2 empty files");
    assert_eq!(finder.get_empty_files().len(), 2, "Empty files list should contain 2 files");
}

#[test]
fn test_no_empty_files() {
    let temp_dir = TempDir::new().unwrap();
    let path = temp_dir.path();

    // Create only non-empty files
    fs::write(path.join("file1.txt"), b"content1").unwrap();
    fs::write(path.join("file2.txt"), b"content2").unwrap();

    let mut finder = EmptyFiles::new();
    finder.set_included_paths(vec![path.to_path_buf()]);
    finder.set_recursive_search(true);

    let stop_flag = Arc::new(AtomicBool::new(false));
    finder.search(&stop_flag, None);

    let info = finder.get_information();
    assert_eq!(info.number_of_empty_files, 0, "Should find no empty files");
    assert!(finder.get_empty_files().is_empty(), "Empty files list should be empty");
}

#[test]
fn test_recursive_search_empty_files() {
    let temp_dir = TempDir::new().unwrap();
    let path = temp_dir.path();

    let subdir = path.join("subdir");
    fs::create_dir(&subdir).unwrap();

    // Create empty files in different directories
    fs::write(path.join("empty1.txt"), b"").unwrap();
    fs::write(subdir.join("empty2.txt"), b"").unwrap();

    let mut finder = EmptyFiles::new();
    finder.set_included_paths(vec![path.to_path_buf()]);

    let stop_flag = Arc::new(AtomicBool::new(false));
    finder.search(&stop_flag, None);

    let info = finder.get_information();
    assert_eq!(info.number_of_empty_files, 2, "Should find empty files in subdirectories");
}
