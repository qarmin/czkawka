use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use tempfile::TempDir;

use crate::common::tool_data::CommonData;
use crate::common::traits::Search;
use crate::tools::broken_files::{BrokenFiles, BrokenFilesParameters, CheckedTypes};

fn get_test_resources_path() -> PathBuf {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test_resources");
    assert!(path.exists(), "Test resources not found at {path:?}");
    path
}

fn corrupt_file(source: &PathBuf, dest: &PathBuf, bytes_to_corrupt: usize) {
    let mut content = fs::read(source).unwrap();
    for byte in content.iter_mut().take(bytes_to_corrupt) {
        *byte = 0x11;
    }
    fs::write(dest, content).unwrap();
}

#[test]
fn test_find_broken_image() {
    let temp_dir = TempDir::new().unwrap();
    let test_resources = get_test_resources_path();

    let source_image = test_resources.join("images").join("normal.jpg");
    let broken_image = temp_dir.path().join("broken.jpg");
    corrupt_file(&source_image, &broken_image, 10);

    let params = BrokenFilesParameters::new(CheckedTypes::IMAGE);
    let mut finder = BrokenFiles::new(params);
    finder.set_included_directory(vec![temp_dir.path().to_path_buf()]);
    finder.set_recursive_search(true);
    finder.set_use_cache(false);

    let stop_flag = Arc::new(AtomicBool::new(false));
    finder.search(&stop_flag, None);

    let broken_files = finder.get_broken_files();
    assert_eq!(broken_files.len(), 1, "Should find 1 broken image file");
    assert!(!broken_files[0].error_string.is_empty(), "Error string should not be empty");
}

#[test]
fn test_valid_image() {
    let temp_dir = TempDir::new().unwrap();
    let test_resources = get_test_resources_path();

    let source_image = test_resources.join("images").join("normal.jpg");
    let valid_image = temp_dir.path().join("valid.jpg");
    fs::copy(&source_image, &valid_image).unwrap();

    let params = BrokenFilesParameters::new(CheckedTypes::IMAGE);
    let mut finder = BrokenFiles::new(params);
    finder.set_included_directory(vec![temp_dir.path().to_path_buf()]);
    finder.set_recursive_search(true);
    finder.set_use_cache(false);

    let stop_flag = Arc::new(AtomicBool::new(false));
    finder.search(&stop_flag, None);

    let broken_files = finder.get_broken_files();
    assert_eq!(broken_files.len(), 0, "Should find no broken image files");
}

#[test]
fn test_broken_audio() {
    let temp_dir = TempDir::new().unwrap();
    let test_resources = get_test_resources_path();

    let source_audio = test_resources.join("audio").join("base.mp3");
    let broken_audio = temp_dir.path().join("broken.mp3");
    let file_len = fs::metadata(&source_audio).unwrap().len();
    corrupt_file(&source_audio, &broken_audio, file_len as usize);

    let good_audio = temp_dir.path().join("good.mp3");
    fs::copy(&source_audio, &good_audio).unwrap();

    let params = BrokenFilesParameters::new(CheckedTypes::AUDIO);
    let mut finder = BrokenFiles::new(params);
    finder.set_included_directory(vec![temp_dir.path().to_path_buf()]);
    finder.set_recursive_search(true);
    finder.set_use_cache(false);

    let stop_flag = Arc::new(AtomicBool::new(false));
    finder.search(&stop_flag, None);

    let broken_files = finder.get_broken_files();
    assert_eq!(broken_files.len(), 1, "Should find 1 broken audio file");
    assert!(!broken_files[0].error_string.is_empty(), "Error string should not be empty");
}

#[test]
fn test_mixed_valid_and_broken_images() {
    let temp_dir = TempDir::new().unwrap();
    let test_resources = get_test_resources_path();

    let source_image1 = test_resources.join("images").join("normal.jpg");
    fs::copy(&source_image1, temp_dir.path().join("valid.jpg")).unwrap();

    let source_image2 = test_resources.join("images").join("normal2.jpg");
    corrupt_file(&source_image2, &temp_dir.path().join("broken.jpg"), 10);

    let params = BrokenFilesParameters::new(CheckedTypes::IMAGE);
    let mut finder = BrokenFiles::new(params);
    finder.set_included_directory(vec![temp_dir.path().to_path_buf()]);
    finder.set_recursive_search(true);
    finder.set_use_cache(false);

    let stop_flag = Arc::new(AtomicBool::new(false));
    finder.search(&stop_flag, None);

    let broken_files = finder.get_broken_files();
    let info = finder.get_information();

    assert_eq!(broken_files.len(), 1, "Should find only 1 broken file out of 2 total");
    assert_eq!(info.number_of_broken_files, 1, "Info should report 1 broken file");
}

#[test]
fn test_multiple_file_types() {
    let temp_dir = TempDir::new().unwrap();
    let test_resources = get_test_resources_path();

    let source_image = test_resources.join("images").join("normal.jpg");
    corrupt_file(&source_image, &temp_dir.path().join("broken.jpg"), 10);

    let source_audio = test_resources.join("audio").join("base.mp3");
    let file_len = fs::metadata(&source_audio).unwrap().len();
    corrupt_file(&source_audio, &temp_dir.path().join("broken.mp3"), file_len as usize);

    let params = BrokenFilesParameters::new(CheckedTypes::IMAGE | CheckedTypes::AUDIO);
    let mut finder = BrokenFiles::new(params);
    finder.set_included_directory(vec![temp_dir.path().to_path_buf()]);
    finder.set_recursive_search(true);
    finder.set_use_cache(false);

    let stop_flag = Arc::new(AtomicBool::new(false));
    finder.search(&stop_flag, None);

    let broken_files = finder.get_broken_files();
    assert_eq!(broken_files.len(), 2, "Should find 2 broken files");
}

#[test]
fn test_empty_directory() {
    let temp_dir = TempDir::new().unwrap();

    let params = BrokenFilesParameters::new(CheckedTypes::IMAGE);
    let mut finder = BrokenFiles::new(params);
    finder.set_included_directory(vec![temp_dir.path().to_path_buf()]);
    finder.set_recursive_search(true);
    finder.set_use_cache(false);

    let stop_flag = Arc::new(AtomicBool::new(false));
    finder.search(&stop_flag, None);

    let broken_files = finder.get_broken_files();
    assert_eq!(broken_files.len(), 0, "Should find no broken files in empty directory");
}

#[test]
fn test_no_file_types_selected() {
    let temp_dir = TempDir::new().unwrap();
    let test_resources = get_test_resources_path();

    let source_image = test_resources.join("images").join("normal.jpg");
    corrupt_file(&source_image, &temp_dir.path().join("broken.jpg"), 10);

    let params = BrokenFilesParameters::new(CheckedTypes::NONE);
    let mut finder = BrokenFiles::new(params);
    finder.set_included_directory(vec![temp_dir.path().to_path_buf()]);
    finder.set_recursive_search(true);
    finder.set_use_cache(false);

    let stop_flag = Arc::new(AtomicBool::new(false));
    finder.search(&stop_flag, None);

    let broken_files = finder.get_broken_files();
    assert_eq!(broken_files.len(), 0, "Should find no files when no types are selected");
}
