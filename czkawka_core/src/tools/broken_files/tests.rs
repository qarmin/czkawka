use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use tempfile::TempDir;

use crate::common::tool_data::CommonData;
use crate::common::traits::Search;
use crate::tools::broken_files::{BrokenEntry, BrokenFiles, BrokenFilesParameters, CheckedTypes};

fn run_check(dir: &TempDir, checked_types: CheckedTypes) -> Vec<BrokenEntry> {
    let params = BrokenFilesParameters::new(checked_types);
    let mut finder = BrokenFiles::new(params);
    finder.set_included_paths(vec![dir.path().to_path_buf()]);
    finder.set_recursive_search(true);
    finder.set_use_cache(false);
    let stop_flag = Arc::new(AtomicBool::new(false));
    finder.search(&stop_flag, None);
    finder.get_broken_files().clone()
}

fn get_test_resources_path() -> PathBuf {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test_resources");
    assert!(path.exists(), "Test resources not found at \"{}\"", path.to_string_lossy());
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
    finder.set_included_paths(vec![temp_dir.path().to_path_buf()]);
    finder.set_recursive_search(true);
    finder.set_use_cache(false);

    let stop_flag = Arc::new(AtomicBool::new(false));
    finder.search(&stop_flag, None);

    let broken_files = finder.get_broken_files();
    assert_eq!(broken_files.len(), 1, "Should find 1 broken image file");
    assert!(broken_files[0].has_errors(), "Errors should not be empty");
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
    finder.set_included_paths(vec![temp_dir.path().to_path_buf()]);
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
    finder.set_included_paths(vec![temp_dir.path().to_path_buf()]);
    finder.set_recursive_search(true);
    finder.set_use_cache(false);

    let stop_flag = Arc::new(AtomicBool::new(false));
    finder.search(&stop_flag, None);

    let broken_files = finder.get_broken_files();
    assert_eq!(broken_files.len(), 1, "Should find 1 broken audio file");
    assert!(broken_files[0].has_errors(), "Errors should not be empty");
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
    finder.set_included_paths(vec![temp_dir.path().to_path_buf()]);
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
    finder.set_included_paths(vec![temp_dir.path().to_path_buf()]);
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
    finder.set_included_paths(vec![temp_dir.path().to_path_buf()]);
    finder.set_recursive_search(true);
    finder.set_use_cache(false);

    let stop_flag = Arc::new(AtomicBool::new(false));
    finder.search(&stop_flag, None);

    let broken_files = finder.get_broken_files();
    assert_eq!(broken_files.len(), 0, "Should find no broken files in empty directory");
}

//  JSON

#[test]
fn test_valid_json() {
    let dir = TempDir::new().unwrap();
    fs::write(dir.path().join("valid.json"), r#"{"key": "value", "number": 42, "arr": [1, 2, 3]}"#).unwrap();
    assert_eq!(run_check(&dir, CheckedTypes::MARKUP).len(), 0, "valid JSON should not be flagged");
}

#[test]
fn test_broken_json_unclosed_brace() {
    let dir = TempDir::new().unwrap();
    fs::write(dir.path().join("broken.json"), r#"{"key": "value""#).unwrap();
    let results = run_check(&dir, CheckedTypes::MARKUP);
    assert_eq!(results.len(), 1);
    assert!(results[0].has_errors());
}

#[test]
fn test_broken_json_invalid_value() {
    let dir = TempDir::new().unwrap();
    fs::write(dir.path().join("broken.json"), r#"{"key": undefined}"#).unwrap();
    let results = run_check(&dir, CheckedTypes::MARKUP);
    assert_eq!(results.len(), 1);
    assert!(results[0].has_errors());
}

//  TOML

#[test]
fn test_valid_toml() {
    let dir = TempDir::new().unwrap();
    fs::write(dir.path().join("valid.toml"), "[section]\nkey = \"value\"\nnumber = 42\n").unwrap();
    assert_eq!(run_check(&dir, CheckedTypes::MARKUP).len(), 0, "valid TOML should not be flagged");
}

#[test]
fn test_broken_toml_missing_value() {
    let dir = TempDir::new().unwrap();
    fs::write(dir.path().join("broken.toml"), "[section]\nkey =\n").unwrap();
    let results = run_check(&dir, CheckedTypes::MARKUP);
    assert_eq!(results.len(), 1);
    assert!(results[0].has_errors());
}

#[test]
fn test_broken_toml_duplicate_key() {
    let dir = TempDir::new().unwrap();
    fs::write(dir.path().join("broken.toml"), "key = 1\nkey = 2\n").unwrap();
    let results = run_check(&dir, CheckedTypes::MARKUP);
    assert_eq!(results.len(), 1);
    assert!(results[0].has_errors());
}

//  YAML

#[test]
fn test_valid_yaml() {
    let dir = TempDir::new().unwrap();
    fs::write(dir.path().join("valid.yaml"), "key: value\nnumber: 42\nlist:\n  - a\n  - b\n").unwrap();
    assert_eq!(run_check(&dir, CheckedTypes::MARKUP).len(), 0, "valid YAML should not be flagged");
}

#[test]
fn test_broken_yaml_bad_indent() {
    let dir = TempDir::new().unwrap();
    fs::write(dir.path().join("broken.yaml"), "key: value\n  bad_indent: {\n").unwrap();
    let results = run_check(&dir, CheckedTypes::MARKUP);
    assert_eq!(results.len(), 1);
    assert!(results[0].has_errors());
}

//  XML

#[test]
fn test_valid_xml() {
    let dir = TempDir::new().unwrap();
    fs::write(dir.path().join("valid.xml"), r#"<?xml version="1.0"?><root><child>value</child></root>"#).unwrap();
    assert_eq!(run_check(&dir, CheckedTypes::MARKUP).len(), 0, "valid XML should not be flagged");
}

#[test]
fn test_broken_xml_mismatched_tag() {
    let dir = TempDir::new().unwrap();
    fs::write(dir.path().join("broken.xml"), "<root><child>value</root>").unwrap();
    let results = run_check(&dir, CheckedTypes::MARKUP);
    assert_eq!(results.len(), 1);
    assert!(results[0].has_errors());
}

#[test]
fn test_broken_xml_unclosed_tag() {
    let dir = TempDir::new().unwrap();
    fs::write(dir.path().join("broken.xml"), "<root><unclosed").unwrap();
    let results = run_check(&dir, CheckedTypes::MARKUP);
    assert_eq!(results.len(), 1);
    assert!(results[0].has_errors());
}

//  SVG

#[test]
fn test_valid_svg() {
    let dir = TempDir::new().unwrap();
    fs::write(
        dir.path().join("valid.svg"),
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="100" height="100"><rect width="10" height="10"/></svg>"#,
    )
    .unwrap();
    assert_eq!(run_check(&dir, CheckedTypes::MARKUP).len(), 0, "valid SVG should not be flagged");
}

#[test]
fn test_broken_svg_not_xml() {
    let dir = TempDir::new().unwrap();
    fs::write(dir.path().join("broken.svg"), "this is not svg content at all <<<>>>").unwrap();
    let results = run_check(&dir, CheckedTypes::MARKUP);
    assert_eq!(results.len(), 1);
    assert!(results[0].has_errors());
}

//  No file types selected

#[test]
fn test_no_file_types_selected() {
    let temp_dir = TempDir::new().unwrap();
    let test_resources = get_test_resources_path();

    let source_image = test_resources.join("images").join("normal.jpg");
    corrupt_file(&source_image, &temp_dir.path().join("broken.jpg"), 10);

    let params = BrokenFilesParameters::new(CheckedTypes::NONE);
    let mut finder = BrokenFiles::new(params);
    finder.set_included_paths(vec![temp_dir.path().to_path_buf()]);
    finder.set_recursive_search(true);
    finder.set_use_cache(false);

    let stop_flag = Arc::new(AtomicBool::new(false));
    finder.search(&stop_flag, None);

    let broken_files = finder.get_broken_files();
    assert_eq!(broken_files.len(), 0, "Should find no files when no types are selected");
}
