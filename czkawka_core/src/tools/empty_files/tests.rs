use std::fs;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use tempfile::{Builder, TempDir};

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

#[cfg(unix)]
mod included_hidden_dirs {
    use std::path::{Path, PathBuf};

    use super::*;

    fn visible_temp_dir() -> TempDir {
        Builder::new().prefix("czk_hidden_test_").tempdir().unwrap()
    }

    fn create_empty(path: &Path) {
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, b"").unwrap();
    }

    fn scan(included: Vec<PathBuf>, excluded_items: &[&str]) -> Vec<String> {
        let mut finder = EmptyFiles::default();
        finder.set_included_paths(included);
        finder.set_excluded_items(excluded_items.iter().map(ToString::to_string).collect());
        finder.set_recursive_search(true);
        finder.search(&Arc::new(AtomicBool::new(false)), None);

        let mut names: Vec<String> = finder.get_empty_files().iter().map(|e| e.path.to_string_lossy().into_owned()).collect();
        names.sort();
        names
    }

    fn relative(base: &Path, found: Vec<String>) -> Vec<String> {
        let prefix = format!("{}/", base.display());
        found.into_iter().map(|p| p.strip_prefix(&prefix).unwrap_or(&p).to_string()).collect()
    }

    #[test]
    fn included_hidden_root_is_scanned_despite_hidden_pattern() {
        let tmp = visible_temp_dir();
        let hidden = tmp.path().join(".A");
        create_empty(&hidden.join("file.txt"));
        create_empty(&hidden.join("sub/deep/file.txt"));

        let found = relative(tmp.path(), scan(vec![hidden], &["*/.*"]));
        assert_eq!(found, vec![".A/file.txt", ".A/sub/deep/file.txt"]);
    }

    #[test]
    fn hidden_root_with_default_preset_is_scanned() {
        let tmp = visible_temp_dir();
        let hidden = tmp.path().join(".A");
        create_empty(&hidden.join("file.txt"));

        let found = relative(tmp.path(), scan(vec![hidden], &["DEFAULT"]));
        assert_eq!(found, vec![".A/file.txt"]);
    }

    #[test]
    fn hidden_subdir_inside_included_hidden_root_is_still_excluded() {
        let tmp = visible_temp_dir();
        let hidden = tmp.path().join(".A");
        create_empty(&hidden.join("file.txt"));
        create_empty(&hidden.join("sub/file.txt"));
        create_empty(&hidden.join(".b/file.txt"));
        create_empty(&hidden.join("sub/.c/file.txt"));

        let found = relative(tmp.path(), scan(vec![hidden], &["*/.*"]));
        assert_eq!(found, vec![".A/file.txt", ".A/sub/file.txt"]);
    }

    #[test]
    fn hidden_dir_inside_normal_root_is_still_excluded() {
        let tmp = visible_temp_dir();
        let root = tmp.path().join("B");
        create_empty(&root.join("visible.txt"));
        create_empty(&root.join(".hidden/file.txt"));

        let found = relative(tmp.path(), scan(vec![root], &["*/.*"]));
        assert_eq!(found, vec!["B/visible.txt"]);
    }

    #[test]
    fn hidden_sibling_not_included_stays_excluded() {
        let tmp = visible_temp_dir();
        let included = tmp.path().join(".A");
        create_empty(&included.join("file.txt"));
        create_empty(&tmp.path().join(".other/file.txt"));
        let normal = tmp.path().join("B");
        create_empty(&normal.join("file.txt"));
        create_empty(&normal.join(".hidden/file.txt"));

        let found = relative(tmp.path(), scan(vec![included, normal], &["*/.*"]));
        assert_eq!(found, vec![".A/file.txt", "B/file.txt"]);
    }

    #[test]
    fn unrelated_pattern_still_applies_inside_included_hidden_root() {
        let tmp = visible_temp_dir();
        let hidden = tmp.path().join(".A");
        create_empty(&hidden.join("file.txt"));
        create_empty(&hidden.join(".git/config"));
        create_empty(&hidden.join("sub/.git/config"));

        let found = relative(tmp.path(), scan(vec![hidden], &["*/.*", "*/.git/*"]));
        assert_eq!(found, vec![".A/file.txt"]);
    }

    #[test]
    fn hidden_root_nested_in_included_normal_root_is_dropped_as_redundant_and_excluded() {
        let tmp = visible_temp_dir();
        let parent = tmp.path().join("P");
        create_empty(&parent.join("file.txt"));
        create_empty(&parent.join(".h/file.txt"));
        let nested_hidden = parent.join(".h");

        let found = relative(tmp.path(), scan(vec![parent, nested_hidden], &["*/.*"]));
        assert_eq!(found, vec!["P/file.txt"]);
    }

    #[test]
    fn pattern_with_trailing_wildcard_is_lifted_for_included_root() {
        let tmp = visible_temp_dir();
        let root = tmp.path().join("node_modules");
        create_empty(&root.join("a.txt"));
        let normal = tmp.path().join("N");
        create_empty(&normal.join("node_modules/a.txt"));
        create_empty(&normal.join("b.txt"));

        let found = relative(tmp.path(), scan(vec![root, normal], &["*/node_modules/*"]));
        assert_eq!(found, vec!["N/b.txt", "node_modules/a.txt"]);
    }

    #[test]
    fn without_excluded_items_hidden_dirs_are_scanned() {
        let tmp = visible_temp_dir();
        create_empty(&tmp.path().join(".A/file.txt"));
        create_empty(&tmp.path().join("B/.c/file.txt"));

        let found = relative(tmp.path(), scan(vec![tmp.path().to_path_buf()], &[]));
        assert_eq!(found, vec![".A/file.txt", "B/.c/file.txt"]);
    }
}
