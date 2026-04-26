use std::fs;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use tempfile::TempDir;

use crate::common::tool_data::CommonData;
use crate::common::traits::Search;
use crate::tools::empty_files::{EmptyFiles, EmptyFilesParameters};

#[test]
fn test_find_empty_files() {
    let temp_dir = TempDir::new().unwrap();
    let path = temp_dir.path();

    fs::write(path.join("empty1.txt"), b"").unwrap();
    fs::write(path.join("empty2.txt"), b"").unwrap();
    fs::write(path.join("not_empty.txt"), b"content").unwrap();

    let mut finder = EmptyFiles::default();
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

    fs::write(path.join("file1.txt"), b"content1").unwrap();
    fs::write(path.join("file2.txt"), b"content2").unwrap();

    let mut finder = EmptyFiles::default();
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

    fs::write(path.join("empty1.txt"), b"").unwrap();
    fs::write(subdir.join("empty2.txt"), b"").unwrap();

    let mut finder = EmptyFiles::default();
    finder.set_included_paths(vec![path.to_path_buf()]);

    let stop_flag = Arc::new(AtomicBool::new(false));
    finder.search(&stop_flag, None);

    let info = finder.get_information();
    assert_eq!(info.number_of_empty_files, 2, "Should find empty files in subdirectories");
}

#[test]
fn test_find_zero_byte_content_files() {
    let temp_dir = TempDir::new().unwrap();
    let path = temp_dir.path();

    fs::write(path.join("zero_bytes.txt"), b"\x00\x00\x00").unwrap();
    fs::write(path.join("regular.txt"), b"content").unwrap();
    fs::write(path.join("truly_empty.txt"), b"").unwrap();

    let mut finder = EmptyFiles::new(EmptyFilesParameters {
        search_zero_byte_content_files: true,
        ..Default::default()
    });
    finder.set_included_paths(vec![path.to_path_buf()]);
    finder.set_recursive_search(true);

    let stop_flag = Arc::new(AtomicBool::new(false));
    finder.search(&stop_flag, None);

    assert_eq!(finder.get_empty_files().len(), 2, "Should find zero-size file and null-content file");
}

#[test]
fn test_find_whitespace_content_files() {
    let temp_dir = TempDir::new().unwrap();
    let path = temp_dir.path();

    fs::write(path.join("whitespace.txt"), b"   \n\t\r").unwrap();
    fs::write(path.join("null.txt"), b"\x00\x00").unwrap();
    fs::write(path.join("regular.txt"), b"hello").unwrap();
    fs::write(path.join("truly_empty.txt"), b"").unwrap();

    let mut finder = EmptyFiles::new(EmptyFilesParameters {
        search_non_printable_content_files: true,
        ..Default::default()
    });
    finder.set_included_paths(vec![path.to_path_buf()]);
    finder.set_recursive_search(true);

    let stop_flag = Arc::new(AtomicBool::new(false));
    finder.search(&stop_flag, None);

    // whitespace.txt + null.txt (null is a subset of whitespace) + truly_empty.txt
    assert_eq!(finder.get_empty_files().len(), 3, "Should find whitespace, null, and zero-size files");
}
