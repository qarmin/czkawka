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
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("test_resources")
            .join("images");

        assert!(path.exists(), "Test resources not found at {:?}", path);

        path
    }

    #[test]
    fn test_find_similar_images() {
        let test_path = get_test_resources_path();

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
        finder.set_recursive_search(true);
        finder.set_use_cache(false);

        let stop_flag = Arc::new(AtomicBool::new(false));
        finder.search(&stop_flag, None);

        let info = finder.get_information();
        let similar_images = finder.get_similar_images();

        assert_eq!(info.number_of_duplicates, 20);
        assert_eq!(info.number_of_groups, 20);
        assert_eq!(similar_images.len(), 40);
    }

    #[test]
    fn test_similar_images_with_rotated() {
        let test_path = get_test_resources_path();

        let params = SimilarImagesParameters::new(
            20,
            16,
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

        assert_eq!(info.number_of_duplicates, 20);
        assert_eq!(info.number_of_groups, 20);
        assert_eq!(similar_images.len(), 40);
    }

    #[test]
    fn test_similar_images_different_hash_sizes() {
        let test_path = get_test_resources_path();

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

            let info = finder.get_information();
            let similar_images = finder.get_similar_images();

            assert_eq!(info.number_of_duplicates, 20);
            assert_eq!(info.number_of_groups, 20);
            assert_eq!(similar_images.len(), 40);
        }
    }

    #[test]
    fn test_similar_images_different_algorithms() {
        let test_path = get_test_resources_path();

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

            let info = finder.get_information();
            let similar_images = finder.get_similar_images();

            assert_eq!(info.number_of_duplicates, 20);
            assert_eq!(info.number_of_groups, 20);
            assert_eq!(similar_images.len(), 40);
        }
    }

    #[test]
    fn test_similar_images_exclude_same_size() {
        let test_path = get_test_resources_path();

        let params = SimilarImagesParameters::new(
            10,
            8,
            HashAlg::Gradient,
            FilterType::Lanczos3,
            true,
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

        assert!(info.number_of_groups >= 0);
        for group in similar_images {
            if group.len() > 1 {
                let first_size = group[0].size;
                let all_same_size = group.iter().all(|img| img.size == first_size);
                assert!(!all_same_size);
            }
        }
    }

    #[test]
    fn test_similar_images_no_recursion() {
        let test_path = get_test_resources_path();

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

        let info = finder.get_information();
        let similar_images = finder.get_similar_images();

        assert_eq!(info.number_of_duplicates, 20);
        assert_eq!(info.number_of_groups, 20);
        assert_eq!(similar_images.len(), 40);
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
        let similar_images = finder.get_similar_images();

        assert_eq!(info.number_of_duplicates, 20);
        assert_eq!(info.number_of_groups, 20);
        assert_eq!(similar_images.len(), 40);
    }

    #[test]
    fn test_similar_images_high_similarity_threshold() {
        let test_path = get_test_resources_path();

        let params = SimilarImagesParameters::new(
            1,
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

        let info = finder.get_information();
        let similar_images = finder.get_similar_images();

        assert_eq!(info.number_of_duplicates, 20);
        assert_eq!(info.number_of_groups, 20);
        assert_eq!(similar_images.len(), 40);
    }

    #[test]
    fn test_similar_images_different_filters() {
        let test_path = get_test_resources_path();

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

            let info = finder.get_information();
            let similar_images = finder.get_similar_images();

            assert_eq!(info.number_of_duplicates, 20);
            assert_eq!(info.number_of_groups, 20);
            assert_eq!(similar_images.len(), 40);
        }
    }
}

