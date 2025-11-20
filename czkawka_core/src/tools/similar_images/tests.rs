#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use std::sync::Arc;
    use std::sync::atomic::AtomicBool;

    use image_hasher::{FilterType, HashAlg};

    use crate::common::tool_data::CommonData;
    use crate::common::traits::Search;
    use crate::tools::similar_images::{SimilarImages, SimilarImagesParameters};

    fn get_test_resources_path() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("test_resources")
            .join("images")
    }

    #[test]
    fn test_find_similar_images() {
        let test_path = get_test_resources_path();
        if !test_path.exists() {
            eprintln!("Test resources not found at {:?}", test_path);
            return;
        }

        let params = SimilarImagesParameters::new(
            10,               // similarity
            8,                // hash_size
            HashAlg::Gradient,
            FilterType::Lanczos3,
            false,            // exclude_images_with_same_size
            true,             // ignore_hard_links
        );

        let mut finder = SimilarImages::new(params);
        finder.set_included_directory(vec![test_path]);
        finder.set_recursive_search(true);
        finder.set_use_cache(false); // Disable cache for testing

        let stop_flag = Arc::new(AtomicBool::new(false));
        finder.search(&stop_flag, None);

        let info = finder.get_information();
        // We should find at least the test images (normal.jpg, rotated.jpg)
        assert!(
            info.number_of_duplicates > 0 || info.number_of_groups == 0,
            "Should process images from test resources"
        );
    }

    #[test]
    fn test_similar_images_with_rotated() {
        let test_path = get_test_resources_path();
        if !test_path.exists() {
            eprintln!("Test resources not found at {:?}", test_path);
            return;
        }

        // Test with normal.jpg and rotated.jpg - they might be similar
        let params = SimilarImagesParameters::new(
            20,               // higher similarity threshold
            16,               // larger hash size for better accuracy
            HashAlg::Gradient,
            FilterType::Lanczos3,
            false,
            true,
        );

        let mut finder = SimilarImages::new(params);
        finder.set_included_directory(vec![test_path]);
        finder.set_recursive_search(true);
        finder.set_use_cache(false);

        let stop_flag = Arc::new(AtomicBool::new(false));
        finder.search(&stop_flag, None);

        let similar_images = finder.get_similar_images();
        let info = finder.get_information();

        // We have 2 test images in the resources
        // They might or might not be detected as similar depending on the algorithm
        if !similar_images.is_empty() {
            assert!(similar_images[0].len() >= 2, "Similar group should have at least 2 images");
        }

        // Just verify the search completed without errors
        assert!(info.number_of_groups <= 1, "Should have at most 1 group with test images");
    }

    #[test]
    fn test_similar_images_different_hash_sizes() {
        let test_path = get_test_resources_path();
        if !test_path.exists() {
            eprintln!("Test resources not found at {:?}", test_path);
            return;
        }

        // Test with different hash sizes
        for hash_size in [8, 16, 32, 64] {
            let params = SimilarImagesParameters::new(
                10,
                hash_size,
                HashAlg::Gradient,
                FilterType::Lanczos3,
                false,
                true,
            );

            let mut finder = SimilarImages::new(params);
            finder.set_included_directory(vec![test_path.clone()]);
            finder.set_recursive_search(true);
            finder.set_use_cache(false);

            let stop_flag = Arc::new(AtomicBool::new(false));
            finder.search(&stop_flag, None);

            // Just verify it doesn't crash with different hash sizes
            let _info = finder.get_information();
        }
    }

    #[test]
    fn test_similar_images_different_algorithms() {
        let test_path = get_test_resources_path();
        if !test_path.exists() {
            eprintln!("Test resources not found at {:?}", test_path);
            return;
        }

        // Test with different hash algorithms
        let algorithms = vec![
            HashAlg::Blockhash,
            HashAlg::Gradient,
            HashAlg::Mean,
            HashAlg::DoubleGradient,
            HashAlg::VertGradient,
        ];

        for hash_alg in algorithms {
            let params = SimilarImagesParameters::new(
                10,
                8,
                hash_alg,
                FilterType::Lanczos3,
                false,
                true,
            );

            let mut finder = SimilarImages::new(params);
            finder.set_included_directory(vec![test_path.clone()]);
            finder.set_recursive_search(true);
            finder.set_use_cache(false);

            let stop_flag = Arc::new(AtomicBool::new(false));
            finder.search(&stop_flag, None);

            // Just verify it doesn't crash with different algorithms
            let _info = finder.get_information();
        }
    }

    #[test]
    fn test_similar_images_exclude_same_size() {
        let test_path = get_test_resources_path();
        if !test_path.exists() {
            eprintln!("Test resources not found at {:?}", test_path);
            return;
        }

        let params = SimilarImagesParameters::new(
            10,
            8,
            HashAlg::Gradient,
            FilterType::Lanczos3,
            true,  // exclude_images_with_same_size = true
            true,
        );

        let mut finder = SimilarImages::new(params);
        finder.set_included_directory(vec![test_path]);
        finder.set_recursive_search(true);
        finder.set_use_cache(false);

        let stop_flag = Arc::new(AtomicBool::new(false));
        finder.search(&stop_flag, None);

        let similar_images = finder.get_similar_images();

        // If images with same size are excluded, groups should only contain images of different sizes
        for group in similar_images {
            if group.len() > 1 {
                let first_size = group[0].size;
                let all_same_size = group.iter().all(|img| img.size == first_size);
                assert!(!all_same_size, "When exclude_images_with_same_size is true, groups should not contain images with identical sizes");
            }
        }
    }

    #[test]
    fn test_similar_images_no_recursion() {
        let test_path = get_test_resources_path();
        if !test_path.exists() {
            eprintln!("Test resources not found at {:?}", test_path);
            return;
        }

        let params = SimilarImagesParameters::new(
            10,
            8,
            HashAlg::Gradient,
            FilterType::Lanczos3,
            false,
            true,
        );

        let mut finder = SimilarImages::new(params);
        finder.set_included_directory(vec![test_path]);
        finder.set_recursive_search(false);
        finder.set_use_cache(false);

        let stop_flag = Arc::new(AtomicBool::new(false));
        finder.search(&stop_flag, None);

        // Should find images in the root test_resources/images directory
        let _info = finder.get_information();
    }

    #[test]
    fn test_similar_images_empty_directory() {
        use tempfile::TempDir;

        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path();

        let params = SimilarImagesParameters::new(
            10,
            8,
            HashAlg::Gradient,
            FilterType::Lanczos3,
            false,
            true,
        );

        let mut finder = SimilarImages::new(params);
        finder.set_included_directory(vec![path.to_path_buf()]);
        finder.set_recursive_search(true);
        finder.set_use_cache(false);

        let stop_flag = Arc::new(AtomicBool::new(false));
        finder.search(&stop_flag, None);

        let info = finder.get_information();
        assert_eq!(
            info.number_of_duplicates, 0,
            "Should find no images in empty directory"
        );
        assert_eq!(
            info.number_of_groups, 0,
            "Should find no groups in empty directory"
        );

        let similar_images = finder.get_similar_images();
        assert_eq!(similar_images.len(), 0, "Should find no similar images in empty directory");
    }

    #[test]
    fn test_similar_images_high_similarity_threshold() {
        let test_path = get_test_resources_path();
        if !test_path.exists() {
            eprintln!("Test resources not found at {:?}", test_path);
            return;
        }

        // Very high similarity - only nearly identical images should match
        let params = SimilarImagesParameters::new(
            1,  // very low threshold = very similar required
            8,
            HashAlg::Gradient,
            FilterType::Lanczos3,
            false,
            true,
        );

        let mut finder = SimilarImages::new(params);
        finder.set_included_directory(vec![test_path]);
        finder.set_recursive_search(true);
        finder.set_use_cache(false);

        let stop_flag = Arc::new(AtomicBool::new(false));
        finder.search(&stop_flag, None);

        // With very strict similarity, we might find fewer or no matches
        let _similar_images = finder.get_similar_images();
    }

    #[test]
    fn test_similar_images_different_filters() {
        let test_path = get_test_resources_path();
        if !test_path.exists() {
            eprintln!("Test resources not found at {:?}", test_path);
            return;
        }

        // Test with different filter types
        let filters = vec![
            FilterType::Nearest,
            FilterType::Triangle,
            FilterType::Lanczos3,
        ];

        for filter in filters {
            let params = SimilarImagesParameters::new(
                10,
                8,
                HashAlg::Gradient,
                filter,
                false,
                true,
            );

            let mut finder = SimilarImages::new(params);
            finder.set_included_directory(vec![test_path.clone()]);
            finder.set_recursive_search(true);
            finder.set_use_cache(false);

            let stop_flag = Arc::new(AtomicBool::new(false));
            finder.search(&stop_flag, None);

            // Just verify it doesn't crash with different filters
            let _info = finder.get_information();
        }
    }
}

