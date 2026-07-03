use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use image::{DynamicImage, ImageBuffer, Rgba};
use image_hasher::{FilterType, HashAlg};
use tempfile::TempDir;

use crate::common::tool_data::CommonData;
use crate::common::traits::Search;
use crate::tools::similar_images::{GeometricInvariance, SimilarImages, SimilarImagesParameters};

fn get_test_resources_path() -> PathBuf {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test_resources").join("images");

    assert!(path.exists(), "Test resources not found at \"{}\"", path.to_string_lossy());

    path
}

fn create_asymmetric_test_image(path: &Path) -> DynamicImage {
    let mut img = ImageBuffer::from_pixel(32, 24, Rgba([0_u8, 0_u8, 0_u8, 255_u8]));
    for x in 0..32 {
        img.put_pixel(x, 0, Rgba([255_u8, 0_u8, 0_u8, 255_u8]));
    }
    img.put_pixel(5, 10, Rgba([0_u8, 255_u8, 0_u8, 255_u8]));
    img.put_pixel(20, 18, Rgba([0_u8, 0_u8, 255_u8, 255_u8]));

    let dynamic = DynamicImage::ImageRgba8(img);
    dynamic.save(path).expect("Failed to save base test image");
    dynamic
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
        let params = SimilarImagesParameters::new(similarity, hash_size, hash_alg, filter_type, false, false, GeometricInvariance::Off);

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

    let params = SimilarImagesParameters::new(10, 8, HashAlg::Gradient, FilterType::Lanczos3, true, false, GeometricInvariance::Off);

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
    let temp_dir = TempDir::new().unwrap();
    let path = temp_dir.path();

    let params = SimilarImagesParameters::new(10, 8, HashAlg::Gradient, FilterType::Lanczos3, false, false, GeometricInvariance::Off);

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
fn test_similar_images_mirror_flip_invariance() {
    let temp_dir = TempDir::new().unwrap();
    let base_path = temp_dir.path().join("base.png");
    let flipped_path = temp_dir.path().join("flipped.png");

    let base = create_asymmetric_test_image(&base_path);
    base.fliph().save(&flipped_path).expect("Failed to save flipped image");

    let params = SimilarImagesParameters::new(0, 8, HashAlg::Gradient, FilterType::Lanczos3, false, false, GeometricInvariance::MirrorFlip);
    let mut finder = SimilarImages::new(params);
    finder.set_included_paths(vec![temp_dir.path().to_path_buf()]);
    finder.set_recursive_search(true);
    finder.set_use_cache(false);

    let stop_flag = Arc::new(AtomicBool::new(false));
    finder.search(&stop_flag, None);

    let info = finder.get_information();
    let similar_images = finder.get_similar_images();

    assert_eq!(info.number_of_groups, 1);
    assert_eq!(info.number_of_duplicates, 1);
    assert_eq!(similar_images.len(), 1);
    assert_eq!(similar_images[0].len(), 2);
}

#[test]
fn test_similar_images_rotate_invariance() {
    let temp_dir = TempDir::new().unwrap();
    let base_path = temp_dir.path().join("base.png");
    let rotated_path = temp_dir.path().join("rotated.png");

    let base = create_asymmetric_test_image(&base_path);
    base.rotate90().save(&rotated_path).expect("Failed to save rotated image");

    let params = SimilarImagesParameters::new(0, 8, HashAlg::Gradient, FilterType::Lanczos3, false, false, GeometricInvariance::MirrorFlipRotate90);
    let mut finder = SimilarImages::new(params);
    finder.set_included_paths(vec![temp_dir.path().to_path_buf()]);
    finder.set_recursive_search(true);
    finder.set_use_cache(false);

    let stop_flag = Arc::new(AtomicBool::new(false));
    finder.search(&stop_flag, None);

    let info = finder.get_information();
    let similar_images = finder.get_similar_images();

    assert_eq!(info.number_of_groups, 1);
    assert_eq!(info.number_of_duplicates, 1);
    assert_eq!(similar_images.len(), 1);
    assert_eq!(similar_images[0].len(), 2);
}

#[cfg(target_family = "unix")]
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
        let params = SimilarImagesParameters::new(10, 8, HashAlg::Gradient, FilterType::Lanczos3, false, false, GeometricInvariance::Off);
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
        let params = SimilarImagesParameters::new(10, 8, HashAlg::Gradient, FilterType::Lanczos3, false, false, GeometricInvariance::Off);
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

#[test]
fn test_similar_images_reference_mode_deletes_only_non_reference() {
    use std::fs;
    use std::path::Path;

    use tempfile::TempDir;

    use crate::common::tool_data::DeleteMethod;
    use crate::common::traits::DeletingItems;
    use crate::tools::similar_images::ImagesEntry;

    let temp_dir = TempDir::new().unwrap();
    let reference = temp_dir.path().join("reference.jpg");
    let duplicate = temp_dir.path().join("duplicate.jpg");
    fs::write(&reference, "ref").unwrap();
    fs::write(&duplicate, "dup").unwrap();

    let mk = |path: &Path| ImagesEntry {
        path: path.to_path_buf(),
        size: 3,
        width: 1,
        height: 1,
        modified_date: 0,
        hashes: Vec::new(),
        difference: 0,
    };

    let params = SimilarImagesParameters::new(10, 8, HashAlg::Gradient, FilterType::Lanczos3, false, false, GeometricInvariance::Off);
    let mut finder = SimilarImages::new(params);
    finder.set_delete_method(DeleteMethod::Delete);
    finder.set_move_to_trash(false);
    finder.set_use_reference_folders(true);
    finder.similar_referenced_vectors = vec![(mk(&reference), vec![mk(&duplicate)])];

    let stop_flag = Arc::new(AtomicBool::new(false));
    let _ = finder.delete_files(&stop_flag, None);

    assert!(reference.exists(), "Reference image must be kept");
    assert!(!duplicate.exists(), "Non-reference duplicate must be deleted (#1643)");
}

#[cfg(feature = "libavif")]
fn get_heif_images_path() -> PathBuf {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test_resources").join("heif_images");
    assert!(path.exists(), "HEIF test resources not found at \"{}\"", path.to_string_lossy());
    path
}

#[cfg(feature = "libavif")]
#[test]
fn test_similar_images_avif_files_are_found() {
    let test_path = get_heif_images_path();

    let params = SimilarImagesParameters::new(30, 8, HashAlg::Gradient, FilterType::Lanczos3, false, false, GeometricInvariance::Off);
    let mut finder = SimilarImages::new(params);
    finder.set_included_paths(vec![test_path]);
    finder.set_recursive_search(false);
    finder.set_use_cache(false);

    let stop_flag = Arc::new(AtomicBool::new(false));
    finder.search(&stop_flag, None);

    let info = finder.get_information();
    assert_eq!(info.initial_found_files, 3, "Should find all 3 AVIF test files");
}

#[cfg(feature = "libavif")]
#[test]
fn test_similar_images_avif_solid_colors_are_all_similar_under_gradient_hash() {
    // Gradient hash of solid-color images is all-zeros regardless of color,
    // so all three solid-color AVIF test images hash identically and land in one group.
    let test_path = get_heif_images_path();

    let params = SimilarImagesParameters::new(0, 8, HashAlg::Gradient, FilterType::Lanczos3, false, false, GeometricInvariance::Off);
    let mut finder = SimilarImages::new(params);
    finder.set_included_paths(vec![test_path]);
    finder.set_recursive_search(false);
    finder.set_use_cache(false);

    let stop_flag = Arc::new(AtomicBool::new(false));
    finder.search(&stop_flag, None);

    let info = finder.get_information();
    assert_eq!(info.initial_found_files, 3);
    assert_eq!(info.number_of_groups, 1, "All solid-color AVIFs should be one group under gradient hash");
    assert_eq!(info.number_of_duplicates, 2);
}
