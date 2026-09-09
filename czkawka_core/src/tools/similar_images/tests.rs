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
fn test_similar_images_avif_striped_pattern_are_all_similar_under_gradient_hash() {
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
    assert_eq!(info.number_of_groups, 1, "All striped AVIFs should be one group under gradient hash");
    assert_eq!(info.number_of_duplicates, 2);
}

#[cfg(test)]
mod structureless_hash_tests {
    use image::{DynamicImage, ImageBuffer, Rgb};
    use image_hasher::{FilterType, HashAlg, HasherConfig};

    use crate::tools::similar_images::SimilarImages;

    // Two unrelated pictures that share only a hard light/dark vertical split. Blockhash
    // thresholds every block against the median, so the low contrast detail inside each half
    // never crosses it and both images hash to the same "one vertical edge" pattern.
    fn split_tone_image(seed: u32) -> DynamicImage {
        let buf = ImageBuffer::from_fn(256, 256, |x, y| {
            let base = if x < 128 { 242 } else { 38 };
            let detail = ((x * seed + y * (seed + 3)) % 11) as i32;
            let v = (base + detail).clamp(0, 255) as u8;
            Rgb([v, v, v])
        });
        DynamicImage::ImageRgb8(buf)
    }

    // Broad shapes plus fine detail and grain, so the hash varies from row to row the way it
    // does for real pictures. A plain smooth gradient is deliberately not used here: it is
    // itself close to structureless and would be rejected.
    fn detailed_image(seed: u32) -> DynamicImage {
        let buf = ImageBuffer::from_fn(256, 256, |x, y| {
            let fx = x as f32 / 256.0;
            let fy = y as f32 / 256.0;
            let broad = (fx * 3.0 * seed as f32).sin() * (fy * 4.0).cos() * 60.0;
            let fine = (fx * 37.0).sin() * (fy * 41.0 + seed as f32).cos() * 45.0;
            let grain = f32::from(u8::try_from((x.wrapping_mul(2_654_435_761_u32.wrapping_add(seed)) ^ y.wrapping_mul(40_503)) % 64).unwrap_or_default()) - 32.0;
            let v = (broad + fine + grain + 128.0).clamp(0.0, 255.0) as u8;
            Rgb([v, v, v])
        });
        DynamicImage::ImageRgb8(buf)
    }

    fn hash_of(image: &DynamicImage, hash_size: u32) -> Vec<u8> {
        HasherConfig::new()
            .hash_size(hash_size, hash_size)
            .hash_alg(HashAlg::Blockhash)
            .resize_filter(FilterType::Lanczos3)
            .to_hasher()
            .hash_image(image)
            .as_bytes()
            .to_vec()
    }

    #[test]
    fn unrelated_split_tone_images_collide_and_are_rejected() {
        for hash_size in [16_u32, 32, 64] {
            let first = hash_of(&split_tone_image(1), hash_size);
            let second = hash_of(&split_tone_image(5), hash_size);

            let distance: u32 = first.iter().zip(&second).map(|(a, b)| (a ^ b).count_ones()).sum();
            assert_eq!(distance, 0, "unrelated split tone images should collide at hash size {hash_size}");

            assert!(
                !SimilarImages::is_hash_valid(&first, hash_size as u8),
                "structureless hash should be rejected at hash size {hash_size}"
            );
        }
    }

    #[test]
    fn detailed_images_are_kept() {
        for hash_size in [16_u32, 32, 64] {
            for seed in 1..=3 {
                let hash = hash_of(&detailed_image(seed), hash_size);
                assert!(
                    SimilarImages::is_hash_valid(&hash, hash_size as u8),
                    "detailed image should be kept at hash size {hash_size}, seed {seed}"
                );
            }
        }
    }

    #[test]
    fn structure_check_is_skipped_for_small_hashes() {
        // An 8x8 matrix has too few lines to distinguish a degenerate hash from a simple
        // image, so the check must not run and must not reject anything there.
        let hash = hash_of(&split_tone_image(1), 8);
        assert!(SimilarImages::is_hash_valid(&hash, 8));
    }

    #[test]
    fn uniform_hashes_are_still_rejected() {
        assert!(!SimilarImages::is_hash_valid(&vec![0_u8; 128], 32));
        assert!(!SimilarImages::is_hash_valid(&vec![255_u8; 128], 32));
        assert!(!SimilarImages::is_hash_valid(&Vec::new(), 32));
    }

    #[test]
    fn non_square_hash_layout_is_left_alone() {
        // DoubleGradient does not produce a hash_size x hash_size matrix, so the structural
        // check cannot interpret it and must pass it through.
        let image = split_tone_image(1);
        let hash = HasherConfig::new()
            .hash_size(32, 32)
            .hash_alg(HashAlg::DoubleGradient)
            .resize_filter(FilterType::Lanczos3)
            .to_hasher()
            .hash_image(&image)
            .as_bytes()
            .to_vec();
        assert_ne!(hash.len() * 8, 32 * 32, "DoubleGradient is expected to use a different layout");
        assert!(SimilarImages::is_hash_valid(&hash, 32));
    }
}
