use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use image_hasher::{FilterType, HashAlg};

use crate::common::tool_data::CommonData;
use crate::common::traits::Search;
use crate::tools::similar_images::{SimilarImages, SimilarImagesParameters};

fn get_test_resources_path() -> PathBuf {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test_resources").join("images");

    assert!(path.exists(), "Test resources not found at \"{}\"", path.to_string_lossy());

    path
}

#[test]
fn test_similar_images() {
    let test_path = get_test_resources_path();

    let algo_filter_hash_sim_found = [
        (HashAlg::Gradient, FilterType::Lanczos3, 8, 222240, 2, 1, 3),
        (HashAlg::Gradient, FilterType::Lanczos3, 8, 15, 1, 1, 2),
        (HashAlg::Gradient, FilterType::Lanczos3, 8, 8, 0, 0, 0),
        (HashAlg::Blockhash, FilterType::Lanczos3, 8, 40, 2, 1, 3),
        (HashAlg::Blockhash, FilterType::Lanczos3, 8, 15, 1, 1, 2),
        (HashAlg::Blockhash, FilterType::Lanczos3, 8, 2, 0, 0, 0),
        (HashAlg::Mean, FilterType::Lanczos3, 8, 40, 2, 1, 3),
        (HashAlg::Mean, FilterType::Lanczos3, 8, 15, 1, 1, 2),
        (HashAlg::Mean, FilterType::Lanczos3, 8, 2, 0, 0, 0),
        (HashAlg::DoubleGradient, FilterType::Lanczos3, 8, 40, 2, 1, 3),
        (HashAlg::DoubleGradient, FilterType::Lanczos3, 8, 15, 1, 1, 2),
        (HashAlg::DoubleGradient, FilterType::Lanczos3, 8, 2, 0, 0, 0),
        (HashAlg::VertGradient, FilterType::Lanczos3, 8, 40, 2, 1, 3),
        (HashAlg::VertGradient, FilterType::Lanczos3, 8, 15, 1, 1, 2),
        (HashAlg::VertGradient, FilterType::Lanczos3, 8, 2, 0, 0, 0),
        (HashAlg::Gradient, FilterType::Gaussian, 16, 15, 0, 0, 0),
        (HashAlg::Gradient, FilterType::Gaussian, 16, 32, 1, 1, 2),
        (HashAlg::VertGradient, FilterType::Nearest, 16, 32, 1, 1, 2),
    ];

    for (idx, (hash_alg, filter_type, hash_size, similarity, duplicates, groups, all_in_similar)) in algo_filter_hash_sim_found.into_iter().enumerate() {
        let params = SimilarImagesParameters::new(similarity, hash_size, hash_alg, filter_type, false, false);

        let mut finder = SimilarImages::new(params);
        finder.set_included_paths(vec![test_path.clone()]);
        finder.set_recursive_search(true);
        finder.set_use_cache(false);

        let stop_flag = Arc::new(AtomicBool::new(false));
        finder.search(&stop_flag, None);

        let info = finder.get_information();
        let similar_images = finder.get_similar_images();

        let msg = format!("Failed for algo/filter/hash/similarity set {idx}: {hash_alg:?}/{filter_type:?}/{hash_size}/{similarity}");

        assert_eq!(info.initial_found_files, 3, "{msg}");
        assert_eq!(info.number_of_duplicates, duplicates, "{msg}");
        assert_eq!(info.number_of_groups, groups, "{msg}");
        assert_eq!(similar_images.len(), groups, "{msg}");
        assert_eq!(similar_images.iter().map(|e| e.len()).sum::<usize>(), all_in_similar, "{msg}");
    }
}

#[test]
fn test_similar_images_exclude_same_size() {
    let test_path = get_test_resources_path();

    let params = SimilarImagesParameters::new(10, 8, HashAlg::Gradient, FilterType::Lanczos3, true, false);

    let mut finder = SimilarImages::new(params);
    finder.set_included_paths(vec![test_path]);
    finder.set_recursive_search(true);
    finder.set_use_cache(false);

    let stop_flag = Arc::new(AtomicBool::new(false));
    finder.search(&stop_flag, None);

    let similar_images = finder.get_similar_images();
    let info = finder.get_information();

    assert!(info.number_of_groups > 0);
    for group in similar_images {
        if group.len() > 1 {
            let first_size = group[0].size;
            let all_same_size = group.iter().all(|img| img.size == first_size);
            assert!(!all_same_size);
        }
    }
}

#[test]
fn test_similar_images_empty_directory() {
    use tempfile::TempDir;

    let temp_dir = TempDir::new().unwrap();
    let path = temp_dir.path();

    let params = SimilarImagesParameters::new(10, 8, HashAlg::Gradient, FilterType::Lanczos3, false, false);

    let mut finder = SimilarImages::new(params);
    finder.set_included_paths(vec![path.to_path_buf()]);
    finder.set_recursive_search(true);
    finder.set_use_cache(false);

    let stop_flag = Arc::new(AtomicBool::new(false));
    finder.search(&stop_flag, None);

    let info = finder.get_information();
    let similar_images = finder.get_similar_images();

    assert_eq!(info.number_of_duplicates, 0);
    assert_eq!(info.number_of_groups, 0);
    assert_eq!(similar_images.len(), 0);
}

#[test]
fn test_similar_images_hide_hard_links() {
    use std::fs;

    use tempfile::TempDir;

    let temp_dir = TempDir::new().unwrap();
    let path = temp_dir.path();

    let test_image_src = get_test_resources_path().join("normal.jpg");
    let img1_path = path.join("image1.jpg");
    let img2_path = path.join("image2.jpg");
    fs::copy(&test_image_src, &img1_path).unwrap();

    #[cfg(target_family = "unix")]
    fs::hard_link(&img1_path, &img2_path).unwrap();
    #[cfg(target_family = "windows")]
    {
        if fs::hard_link(&img1_path, &img2_path).is_err() {
            return;
        }
    }

    {
        let params = SimilarImagesParameters::new(10, 8, HashAlg::Gradient, FilterType::Lanczos3, false, false);
        let mut finder = SimilarImages::new(params);
        finder.set_hide_hard_links(true);
        finder.set_included_paths(vec![path.to_path_buf()]);
        finder.set_recursive_search(true);
        finder.set_use_cache(false);

        let stop_flag = Arc::new(AtomicBool::new(false));
        finder.search(&stop_flag, None);

        let info = finder.get_information();
        assert_eq!(info.number_of_duplicates, 0);
        assert_eq!(info.number_of_groups, 0);
    }

    {
        let params = SimilarImagesParameters::new(10, 8, HashAlg::Gradient, FilterType::Lanczos3, false, false);
        let mut finder = SimilarImages::new(params);
        finder.set_hide_hard_links(false);
        finder.set_included_paths(vec![path.to_path_buf()]);
        finder.set_recursive_search(true);
        finder.set_use_cache(false);

        let stop_flag = Arc::new(AtomicBool::new(false));
        finder.search(&stop_flag, None);

        let info = finder.get_information();
        assert_eq!(info.number_of_duplicates, 1);
        assert_eq!(info.number_of_groups, 1);
    }
}
