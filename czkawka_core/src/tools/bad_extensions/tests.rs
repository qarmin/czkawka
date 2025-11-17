#[cfg(test)]
mod tests {
    use std::fs;
    use std::io::Write;
    use std::sync::Arc;
    use std::sync::atomic::AtomicBool;

    use tempfile::TempDir;

    use crate::common::tool_data::CommonData;
    use crate::common::traits::Search;
    use crate::tools::bad_extensions::{BadExtensions, BadExtensionsParameters};

    #[test]
    fn test_find_bad_extension_png_as_jpg() {
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path();

        // Create a PNG file with .jpg extension
        let png_data = vec![
            0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, // PNG signature
            0x00, 0x00, 0x00, 0x0D, // IHDR chunk
        ];
        let mut file = fs::File::create(path.join("image.jpg")).unwrap();
        file.write_all(&png_data).unwrap();

        let params = BadExtensionsParameters::new();
        let mut finder = BadExtensions::new(params);
        finder.set_included_directory(vec![path.to_path_buf()]);

        let stop_flag = Arc::new(AtomicBool::new(false));
        finder.search(&stop_flag, None);

        let bad_files = finder.get_bad_extensions_files();
        assert_eq!(bad_files.len(), 1, "Should find 1 file with bad extension");
        assert_eq!(bad_files[0].current_extension, "jpg", "Current extension should be jpg");
        assert_eq!(bad_files[0].proper_extension, "png", "Proper extension should be png");
    }

    #[test]
    fn test_correct_extension() {
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path();

        // Create a PNG file with correct .png extension
        let png_data = vec![
            0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, // PNG signature
            0x00, 0x00, 0x00, 0x0D,
        ];
        let mut file = fs::File::create(path.join("image.png")).unwrap();
        file.write_all(&png_data).unwrap();

        let params = BadExtensionsParameters::new();
        let mut finder = BadExtensions::new(params);
        finder.set_included_directory(vec![path.to_path_buf()]);

        let stop_flag = Arc::new(AtomicBool::new(false));
        finder.search(&stop_flag, None);

        let bad_files = finder.get_bad_extensions_files();
        assert_eq!(bad_files.len(), 0, "Should find no files with bad extension");
    }

    #[test]
    fn test_file_without_extension_excluded() {
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path();

        // Create a PNG file without extension
        let png_data = vec![
            0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A,
            0x00, 0x00, 0x00, 0x0D,
        ];
        let mut file = fs::File::create(path.join("image_no_ext")).unwrap();
        file.write_all(&png_data).unwrap();

        let mut params = BadExtensionsParameters::new();
        params.include_files_without_extension = false;
        let mut finder = BadExtensions::new(params);

        finder.set_included_directory(vec![path.to_path_buf()]);
        finder.set_recursive_search(true);

        let stop_flag = Arc::new(AtomicBool::new(false));
        finder.search(&stop_flag, None);

        let bad_files = finder.get_bad_extensions_files();
        assert_eq!(bad_files.len(), 0, "Should not include files without extension when disabled");
    }

    #[test]
    fn test_file_without_extension_included() {
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path();

        // Create a PNG file without extension
        let png_data = vec![
            0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A,
            0x00, 0x00, 0x00, 0x0D,
        ];
        let mut file = fs::File::create(path.join("image_no_ext")).unwrap();
        file.write_all(&png_data).unwrap();

        let mut params = BadExtensionsParameters::new();
        params.include_files_without_extension = true;

        let mut finder = BadExtensions::new(params);
        finder.set_included_directory(vec![path.to_path_buf()]);

        let stop_flag = Arc::new(AtomicBool::new(false));
        finder.search(&stop_flag, None);

        let bad_files = finder.get_bad_extensions_files();
        assert_eq!(bad_files.len(), 1, "Should include files without extension when enabled");
        assert_eq!(bad_files[0].current_extension, "", "Current extension should be empty");
        assert_eq!(bad_files[0].proper_extension, "png");
    }
}

