#[cfg(target_family = "unix")]
use std::fs;
#[cfg(target_family = "unix")]
use std::sync::Arc;
#[cfg(target_family = "unix")]
use std::sync::atomic::AtomicBool;

#[cfg(target_family = "unix")]
use tempfile::TempDir;

#[cfg(target_family = "unix")]
use crate::common::tool_data::CommonData;
#[cfg(target_family = "unix")]
use crate::common::traits::Search;
#[cfg(target_family = "unix")]
use crate::tools::invalid_symlinks::InvalidSymlinks;

#[test]
#[cfg(target_family = "unix")]
fn test_find_invalid_symlinks() {
    let temp_dir = TempDir::new().unwrap();
    let path = temp_dir.path();

    let valid_target = path.join("valid_target.txt");
    fs::write(&valid_target, b"content").unwrap();

    let valid_link = path.join("valid_link");
    std::os::unix::fs::symlink(&valid_target, &valid_link).unwrap();

    let invalid_link = path.join("invalid_link");
    std::os::unix::fs::symlink(path.join("non_existent.txt"), &invalid_link).unwrap();

    let mut finder = InvalidSymlinks::new();
    finder.set_included_paths(vec![path.to_path_buf()]);
    finder.set_recursive_search(true);

    let stop_flag = Arc::new(AtomicBool::new(false));
    finder.search(&stop_flag, None);

    let info = finder.get_information();
    assert_eq!(info.number_of_invalid_symlinks, 1, "Should find 1 invalid symlink");
}

#[test]
#[cfg(target_family = "unix")]
fn test_no_invalid_symlinks() {
    let temp_dir = TempDir::new().unwrap();
    let path = temp_dir.path();

    let target = path.join("target.txt");
    fs::write(&target, b"content").unwrap();

    let link = path.join("link");
    std::os::unix::fs::symlink(&target, &link).unwrap();

    let mut finder = InvalidSymlinks::new();
    finder.set_included_paths(vec![path.to_path_buf()]);
    finder.set_recursive_search(true);

    let stop_flag = Arc::new(AtomicBool::new(false));
    finder.search(&stop_flag, None);

    let info = finder.get_information();
    assert_eq!(info.number_of_invalid_symlinks, 0, "Should find no invalid symlinks");
}

#[test]
#[cfg(target_family = "unix")]
fn test_deleted_target_creates_invalid_symlink() {
    let temp_dir = TempDir::new().unwrap();
    let path = temp_dir.path();

    let target = path.join("target.txt");
    fs::write(&target, b"content").unwrap();

    let link = path.join("link");
    std::os::unix::fs::symlink(&target, &link).unwrap();

    fs::remove_file(&target).unwrap();

    let mut finder = InvalidSymlinks::new();
    finder.set_included_paths(vec![path.to_path_buf()]);
    finder.set_recursive_search(true);

    let stop_flag = Arc::new(AtomicBool::new(false));
    finder.search(&stop_flag, None);

    let info = finder.get_information();
    assert_eq!(info.number_of_invalid_symlinks, 1, "Should find the broken symlink");
}

#[test]
#[cfg(target_family = "unix")]
fn test_symlink_loop_reported_as_infinite_recursion() {
    use crate::tools::invalid_symlinks::ErrorType;

    let temp_dir = TempDir::new().unwrap();
    let path = temp_dir.path();

    std::os::unix::fs::symlink(path.join("loop_b"), path.join("loop_a")).unwrap();
    std::os::unix::fs::symlink(path.join("loop_a"), path.join("loop_b")).unwrap();

    let mut finder = InvalidSymlinks::new();
    finder.set_included_paths(vec![path.to_path_buf()]);
    finder.set_recursive_search(true);

    let stop_flag = Arc::new(AtomicBool::new(false));
    finder.search(&stop_flag, None);

    let symlinks = finder.get_invalid_symlinks();
    assert_eq!(symlinks.len(), 2);
    for entry in symlinks {
        assert_eq!(entry.symlink_info.type_of_error, ErrorType::InfiniteRecursion, "{}", entry.path.display());
    }
}

#[test]
#[cfg(target_family = "unix")]
fn test_long_valid_symlink_chain_is_not_invalid() {
    let temp_dir = TempDir::new().unwrap();
    let path = temp_dir.path();

    let target = path.join("target.txt");
    fs::write(&target, b"content").unwrap();

    let mut previous = target;
    for i in 0..25 {
        let link = path.join(format!("link_{i}"));
        std::os::unix::fs::symlink(&previous, &link).unwrap();
        previous = link;
    }
    assert_eq!(fs::read(&previous).unwrap(), b"content");

    let mut finder = InvalidSymlinks::new();
    finder.set_included_paths(vec![path.to_path_buf()]);
    finder.set_recursive_search(true);

    let stop_flag = Arc::new(AtomicBool::new(false));
    finder.search(&stop_flag, None);

    let info = finder.get_information();
    assert_eq!(info.number_of_invalid_symlinks, 0, "{:?}", finder.get_invalid_symlinks());
}
