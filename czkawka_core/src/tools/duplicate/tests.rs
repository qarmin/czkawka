#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use tempfile::TempDir;
    use crate::tools::duplicate::{DuplicateFinder, DuplicateFinderParameters};

    use crate::common::traits::Search;
    #[test]
    fn test_find_duplicates_by_hash() {
    use crate::common::traits::Search;
        let temp_dir = TempDir::new().unwrap();
    use crate::common::traits::Search;
        let path = temp_dir.path();

        // Create duplicate files with same content
        fs::write(path.join("file1.txt"), b"duplicate content").unwrap();
        fs::write(path.join("file2.txt"), b"duplicate content").unwrap();
        fs::write(path.join("unique.txt"), b"unique content").unwrap();

        let params = DuplicateFinderParameters::new(
            CheckingMethod::Hash,
            HashType::Blake3,
            false,
            false,
            0,
            0,
            true,
        );

        let mut finder = DuplicateFinder::new(params);
        finder.set_included_directory(vec![path.to_path_buf()]);
        finder.set_recursive_search(true);

        let stop_flag = Arc::new(AtomicBool::new(false));
        finder.search(&stop_flag, None);

        let info = finder.get_information();
        assert_eq!(info.number_of_groups_by_hash, 1, "Should find 1 group of duplicates");
        assert_eq!(info.number_of_duplicated_files_by_hash, 1, "Should find 1 duplicate file");
    }

    #[test]
    fn test_find_duplicates_by_size() {
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path();

        // Create files with same size
        fs::write(path.join("file1.txt"), b"12345").unwrap();
        fs::write(path.join("file2.txt"), b"abcde").unwrap();
        fs::write(path.join("unique.txt"), b"123").unwrap();

        let params = DuplicateFinderParameters::new(
            CheckingMethod::Size,
            HashType::Blake3,
            false,
            false,
            0,
            0,
            true,
        );

        let mut finder = DuplicateFinder::new(params);
        finder.set_included_directory(vec![path.to_path_buf()]);
        finder.set_recursive_search(true);

        let stop_flag = Arc::new(AtomicBool::new(false));
        finder.search(&stop_flag, None);

        let info = finder.get_information();
        assert_eq!(info.number_of_groups_by_size, 1, "Should find 1 group by size");
        assert_eq!(info.number_of_duplicated_files_by_size, 1, "Should find 1 duplicate by size");
    }

    #[test]
    fn test_find_duplicates_by_name() {
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path();

        let dir1 = path.join("dir1");
        let dir2 = path.join("dir2");
        fs::create_dir(&dir1).unwrap();
        fs::create_dir(&dir2).unwrap();

        // Create files with same name in different directories
        fs::write(dir1.join("duplicate.txt"), b"content1").unwrap();
        fs::write(dir2.join("duplicate.txt"), b"content2").unwrap();
        fs::write(dir1.join("unique.txt"), b"unique").unwrap();

        let params = DuplicateFinderParameters::new(
            CheckingMethod::Name,
            HashType::Blake3,
            false,
            false,
            0,
            0,
            true,
        );

        let mut finder = DuplicateFinder::new(params);
        finder.set_recursive_search(true);
        finder.set_included_directory(vec![path.to_path_buf()]);
        let stop_flag = Arc::new(AtomicBool::new(false));
        finder.search(&stop_flag, None);

        let info = finder.get_information();
        assert_eq!(info.number_of_groups_by_name, 1, "Should find 1 group by name");
        assert_eq!(info.number_of_duplicated_files_by_name, 1, "Should find 1 duplicate by name");
    }

    #[test]
    fn test_case_insensitive_name_comparison() {
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path();

        // Create files with same name but different case
        fs::write(path.join("TEST.txt"), b"content1").unwrap();
        fs::write(path.join("test.txt"), b"content2").unwrap();

        let params = DuplicateFinderParameters::new(
            CheckingMethod::Name,
            HashType::Blake3,
            false,
            false,
            0,
            0,
            false, // case insensitive
        );

        let mut finder = DuplicateFinder::new(params);
        finder.set_recursive_search(true);
        finder.set_included_directory(vec![path.to_path_buf()]);

        let stop_flag = Arc::new(AtomicBool::new(false));
        let info = finder.get_information();
        assert_eq!(info.number_of_groups_by_name, 1, "Should find duplicates with case insensitive search");
    }

    #[test]
    fn test_no_duplicates_found() {
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path();

        // Create unique files
        fs::write(path.join("file1.txt"), b"content1").unwrap();
        fs::write(path.join("file2.txt"), b"content2").unwrap();

        let params = DuplicateFinderParameters::new(
            CheckingMethod::Hash,
            HashType::Blake3,
            false,
            false,
            0,
            0,
            true,
        );

        let mut finder = DuplicateFinder::new(params);
        finder.set_included_directory(vec![path.to_path_buf()]);
        finder.set_recursive_search(true);

        let stop_flag = Arc::new(AtomicBool::new(false));
        finder.search(&stop_flag, None);

        let info = finder.get_information();
        assert_eq!(info.number_of_groups_by_hash, 0, "Should find no duplicate groups");
        assert_eq!(info.lost_space_by_hash, 0, "Should have no lost space");
    }

    #[test]
    fn test_lost_space_calculation() {
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path();

        // Create 3 files with 100 bytes each, all duplicates
        let content = vec![b'A'; 100];
        fs::write(path.join("file1.txt"), &content).unwrap();
        fs::write(path.join("file2.txt"), &content).unwrap();
        fs::write(path.join("file3.txt"), &content).unwrap();

        let params = DuplicateFinderParameters::new(
            CheckingMethod::Hash,
            HashType::Blake3,
            false,
            false,
            0,
            0,
            true,
        );

        let mut finder = DuplicateFinder::new(params);
        finder.set_included_directory(vec![path.to_path_buf()]);

        let stop_flag = Arc::new(AtomicBool::new(false));
        finder.search(&stop_flag, None);

        let info = finder.get_information();
        assert_eq!(info.lost_space_by_hash, 200, "Should calculate 200 bytes lost space (2 duplicate files * 100 bytes)");
    }
}

