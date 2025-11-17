#[cfg(test)]
mod tests {
    use std::fs;
    use std::sync::Arc;
    use std::sync::atomic::AtomicBool;

    use tempfile::TempDir;

    use crate::common::tool_data::CommonData;
    #[test]
    fn test_find_biggest_files() {
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path();

        // Create files with different sizes
        fs::write(path.join("small.txt"), b"12").unwrap(); // 2 bytes
        fs::write(path.join("medium.txt"), b"12345").unwrap(); // 5 bytes
        fs::write(path.join("large.txt"), vec![b'A'; 100]).unwrap(); // 100 bytes

        let params = BigFileParameters::new(2, SearchMode::BiggestFiles);
        finder.set_included_directory(vec![path.to_path_buf()]);
        finder.set_included_directory(path.to_path_buf());
        let mut finder = BigFile::new(params);

        let stop_flag = Arc::new(AtomicBool::new(false));
        finder.search(&stop_flag, None);

        let big_files = finder.get_big_files();
        assert_eq!(big_files.len(), 2, "Should find 2 biggest files");
        assert_eq!(big_files[0].size, 100, "First file should be 100 bytes");
        assert_eq!(big_files[1].size, 5, "Second file should be 5 bytes");
    }

    #[test]
    fn test_find_smallest_files() {
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path();

        // Create files with different sizes
        fs::write(path.join("small.txt"), b"12").unwrap(); // 2 bytes
        fs::write(path.join("medium.txt"), b"12345").unwrap(); // 5 bytes
        fs::write(path.join("large.txt"), vec![b'A'; 100]).unwrap(); // 100 bytes

        let params = BigFileParameters::new(2, SearchMode::SmallestFiles);
        let mut finder = BigFile::new(params);
        finder.set_included_directory(vec![path.to_path_buf()]);
        finder.set_recursive_search(true);

        let stop_flag = Arc::new(AtomicBool::new(false));
        finder.search(&stop_flag, None);

        let big_files = finder.get_big_files();
        assert_eq!(big_files.len(), 2, "Should find 2 smallest files");
        assert_eq!(big_files[0].size, 2, "First file should be 2 bytes");
        assert_eq!(big_files[1].size, 5, "Second file should be 5 bytes");
    }

    #[test]
    fn test_limit_number_of_files() {
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path();

        // Create 5 files
        for i in 1..=5 {
            fs::write(path.join(format!("file{}.txt", i)), vec![b'A'; i * 10]).unwrap();
        }

        let params = BigFileParameters::new(3, SearchMode::BiggestFiles);
        let mut finder = BigFile::new(params);
        finder.set_included_directory(path.to_path_buf());
        finder.set_included_directory(vec![path.to_path_buf()]);

        let stop_flag = Arc::new(AtomicBool::new(false));
        finder.search(&stop_flag, None);

        let big_files = finder.get_big_files();
        assert_eq!(big_files.len(), 3, "Should limit results to 3 files");
    }

    #[test]
    fn test_empty_directory() {
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path();

        let params = BigFileParameters::new(5, SearchMode::BiggestFiles);
        let mut finder = BigFile::new(params);
        finder.set_included_directory(path.to_path_buf());
        finder.set_recursive_search(true);
        finder.set_included_directory(vec![path.to_path_buf()]);
        let stop_flag = Arc::new(AtomicBool::new(false));
        finder.search(&stop_flag, None);

        let big_files = finder.get_big_files();
        assert!(big_files.is_empty(), "Should find no files in empty directory");
    }
}

