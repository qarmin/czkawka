#![allow(clippy::allow_attributes)]
#![allow(dead_code)]

#[cfg(test)]
mod tests {
    use std::fs;
    use std::os::unix;
    use std::sync::Arc;
    use std::sync::atomic::AtomicBool;

    use tempfile::TempDir;

    use crate::common::tool_data::CommonData;
    use crate::common::traits::Search;
    use crate::tools::invalid_symlinks::InvalidSymlinks;

    #[test]
    #[cfg(target_family = "unix")]
    fn test_find_invalid_symlinks() {
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path();

        let valid_target = path.join("valid_target.txt");
        fs::write(&valid_target, b"content").unwrap();

        let valid_link = path.join("valid_link");
        unix::fs::symlink(&valid_target, &valid_link).unwrap();

        let invalid_link = path.join("invalid_link");
        unix::fs::symlink(path.join("non_existent.txt"), &invalid_link).unwrap();

        let mut finder = InvalidSymlinks::new();
        finder.set_included_directory(vec![path.to_path_buf()]);
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
        unix::fs::symlink(&target, &link).unwrap();

        let mut finder = InvalidSymlinks::new();
        finder.set_included_directory(vec![path.to_path_buf()]);
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
        unix::fs::symlink(&target, &link).unwrap();

        fs::remove_file(&target).unwrap();

        let mut finder = InvalidSymlinks::new();
        finder.set_included_directory(vec![path.to_path_buf()]);
        finder.set_recursive_search(true);

        let stop_flag = Arc::new(AtomicBool::new(false));
        finder.search(&stop_flag, None);

        let info = finder.get_information();
        assert_eq!(info.number_of_invalid_symlinks, 1, "Should find the broken symlink");
    }
}
