#[cfg(test)]
mod tests {
    use std::fs;
    use std::sync::Arc;
    use std::sync::atomic::AtomicBool;

    use tempfile::TempDir;

    use crate::common::tool_data::CommonData;
    use crate::common::traits::Search;
    use crate::tools::empty_folder::EmptyFolder;

    #[test]
    fn test_find_empty_folders() {
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path();

        // Create empty directories
        fs::create_dir(path.join("empty1")).unwrap();
        fs::create_dir(path.join("empty2")).unwrap();

        // Create non-empty directory
        let non_empty = path.join("non_empty");
        fs::create_dir(&non_empty).unwrap();
        fs::write(non_empty.join("file.txt"), b"content").unwrap();

        let mut finder = EmptyFolder::new();
        finder.set_included_directory(path.to_path_buf());
        finder.set_recursive_search(true);

        let stop_flag = Arc::new(AtomicBool::new(false));
        finder.search(&stop_flag, None);

        let info = finder.get_information();
        assert_eq!(info.number_of_empty_folders, 2, "Should find 2 empty folders");
    }

    #[test]
    fn test_nested_empty_folders() {
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path();

        // Create nested empty directories
        let parent = path.join("parent");
        let child = parent.join("child");
        fs::create_dir(&parent).unwrap();
        fs::create_dir(&child).unwrap();

        let mut finder = EmptyFolder::new();
        finder.set_included_directory(vec![path.to_path_buf()]);
        finder.set_recursive_search(true);

        let stop_flag = Arc::new(AtomicBool::new(false));
        finder.search(&stop_flag, None);

        // After optimization, only the deepest empty folder should be counted
        let info = finder.get_information();
        assert!(info.number_of_empty_folders > 0, "Should find empty folders");
    }

    #[test]
    fn test_no_empty_folders() {
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path();

        // Create directory with file
        let dir = path.join("dir");
        fs::create_dir(&dir).unwrap();
        fs::write(dir.join("file.txt"), b"content").unwrap();

        let mut finder = EmptyFolder::new();
        finder.set_included_directory(vec![path.to_path_buf()]);
        finder.set_recursive_search(true);

        let stop_flag = Arc::new(AtomicBool::new(false));
        finder.search(&stop_flag, None);

        let info = finder.get_information();
        assert_eq!(info.number_of_empty_folders, 0, "Should find no empty folders");
    }

    #[test]
    fn test_folder_with_only_empty_subfolders() {
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path();

        // Create parent with only empty subdirectories
        let parent = path.join("parent");
        fs::create_dir(&parent).unwrap();
        fs::create_dir(parent.join("empty_child1")).unwrap();
        fs::create_dir(parent.join("empty_child2")).unwrap();

        let mut finder = EmptyFolder::new();
        finder.set_included_directory(vec![path.to_path_buf()]);
        finder.set_recursive_search(true);

        let stop_flag = Arc::new(AtomicBool::new(false));
        finder.search(&stop_flag, None);

        let info = finder.get_information();
        // Parent and children are all empty
        assert!(info.number_of_empty_folders >= 2, "Should find multiple empty folders");
    }
}

