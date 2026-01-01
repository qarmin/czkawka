use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use tempfile::TempDir;

use crate::common::tool_data::CommonData;
use crate::common::traits::Search;
use crate::tools::exif_remover::{ExifFinderParameters, ExifRemover};

fn get_test_resources_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test_resources").join("images")
}

#[test]
fn test_find_exif_files() {
    let temp_dir = TempDir::new().unwrap();
    let path = temp_dir.path();

    let source_image = get_test_resources_path().join("normal.jpg");
    let dest_image = path.join("test.jpg");
    fs::copy(&source_image, &dest_image).unwrap();

    let mut finder = ExifRemover::new(ExifFinderParameters::default());
    finder.set_included_directory(vec![path.to_path_buf()]);

    let stop_flag = Arc::new(AtomicBool::new(false));
    finder.search(&stop_flag, None);

    let info = finder.get_information();
    assert_eq!(info.number_of_files_with_exif, 1, "Should find at least one file with EXIF data");

    let exif_files = finder.get_exif_files();
    assert_eq!(exif_files.len(), 1, "Should find exactly one file with EXIF");
    assert!(!exif_files[0].exif_tags.is_empty(), "EXIF tags should not be empty");
}

#[test]
fn test_empty_directory() {
    let temp_dir = TempDir::new().unwrap();
    let path = temp_dir.path();

    let mut finder = ExifRemover::new(ExifFinderParameters::default());
    finder.set_included_directory(vec![path.to_path_buf()]);

    let stop_flag = Arc::new(AtomicBool::new(false));
    finder.search(&stop_flag, None);

    let exif_files = finder.get_exif_files();
    assert_eq!(exif_files.len(), 0, "Should find no files with EXIF in empty directory");
}

#[test]
fn test_non_image_files() {
    let temp_dir = TempDir::new().unwrap();
    let path = temp_dir.path();

    fs::write(path.join("test.txt"), b"This is not an image").unwrap();

    let mut finder = ExifRemover::new(ExifFinderParameters::default());
    finder.set_included_directory(vec![path.to_path_buf()]);

    let stop_flag = Arc::new(AtomicBool::new(false));
    finder.search(&stop_flag, None);

    let exif_files = finder.get_exif_files();
    assert_eq!(exif_files.len(), 0, "Should not find EXIF in non-image files");
}
