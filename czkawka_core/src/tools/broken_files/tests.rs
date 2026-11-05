use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use tempfile::TempDir;

use crate::common::image::register_image_decoding_hooks;
use crate::common::tool_data::CommonData;
use crate::common::traits::Search;
use crate::tools::broken_files::{BrokenEntry, BrokenFiles, BrokenFilesParameters, CheckedTypes, CheckedTypesSingle};

fn run_check(dir: &TempDir, checked_types: CheckedTypes) -> Vec<BrokenEntry> {
    let params = BrokenFilesParameters::new(checked_types);
    let mut finder = BrokenFiles::new(params);
    finder.set_included_paths(vec![dir.path().to_path_buf()]);
    finder.set_recursive_search(true);
    finder.set_use_cache(false);
    let stop_flag = Arc::new(AtomicBool::new(false));
    finder.search(&stop_flag, None);
    finder.get_broken_files().clone()
}

fn get_test_resources_path() -> PathBuf {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test_resources");
    assert!(path.exists(), "Test resources not found at \"{}\"", path.to_string_lossy());
    path
}

fn corrupt_file(source: &PathBuf, dest: &PathBuf, bytes_to_corrupt: usize) {
    let mut content = fs::read(source).unwrap();
    for byte in content.iter_mut().take(bytes_to_corrupt) {
        *byte = 0x11;
    }
    fs::write(dest, content).unwrap();
}

#[test]
fn test_find_broken_image() {
    let temp_dir = TempDir::new().unwrap();
    let test_resources = get_test_resources_path();

    let source_image = test_resources.join("images").join("normal.jpg");
    let broken_image = temp_dir.path().join("broken.jpg");
    corrupt_file(&source_image, &broken_image, 10);

    let params = BrokenFilesParameters::new(CheckedTypes::IMAGE);
    let mut finder = BrokenFiles::new(params);
    finder.set_included_paths(vec![temp_dir.path().to_path_buf()]);
    finder.set_recursive_search(true);
    finder.set_use_cache(false);

    let stop_flag = Arc::new(AtomicBool::new(false));
    finder.search(&stop_flag, None);

    let broken_files = finder.get_broken_files();
    assert_eq!(broken_files.len(), 1, "Should find 1 broken image file");
    assert!(broken_files[0].has_errors(), "Errors should not be empty");
}

#[test]
fn test_valid_image() {
    let temp_dir = TempDir::new().unwrap();
    let test_resources = get_test_resources_path();

    let source_image = test_resources.join("images").join("normal.jpg");
    let valid_image = temp_dir.path().join("valid.jpg");
    fs::copy(&source_image, &valid_image).unwrap();

    let params = BrokenFilesParameters::new(CheckedTypes::IMAGE);
    let mut finder = BrokenFiles::new(params);
    finder.set_included_paths(vec![temp_dir.path().to_path_buf()]);
    finder.set_recursive_search(true);
    finder.set_use_cache(false);

    let stop_flag = Arc::new(AtomicBool::new(false));
    finder.search(&stop_flag, None);

    let broken_files = finder.get_broken_files();
    assert_eq!(broken_files.len(), 0, "Should find no broken image files");
}

#[test]
fn test_broken_audio() {
    let temp_dir = TempDir::new().unwrap();
    let test_resources = get_test_resources_path();

    let source_audio = test_resources.join("audio").join("base.mp3");
    let broken_audio = temp_dir.path().join("broken.mp3");
    let file_len = fs::metadata(&source_audio).unwrap().len();
    corrupt_file(&source_audio, &broken_audio, file_len as usize);

    let good_audio = temp_dir.path().join("good.mp3");
    fs::copy(&source_audio, &good_audio).unwrap();

    let params = BrokenFilesParameters::new(CheckedTypes::AUDIO);
    let mut finder = BrokenFiles::new(params);
    finder.set_included_paths(vec![temp_dir.path().to_path_buf()]);
    finder.set_recursive_search(true);
    finder.set_use_cache(false);

    let stop_flag = Arc::new(AtomicBool::new(false));
    finder.search(&stop_flag, None);

    let broken_files = finder.get_broken_files();
    assert_eq!(broken_files.len(), 1, "Should find 1 broken audio file");
    assert!(broken_files[0].has_errors(), "Errors should not be empty");
}

#[test]
fn test_mixed_valid_and_broken_images() {
    let temp_dir = TempDir::new().unwrap();
    let test_resources = get_test_resources_path();

    let source_image1 = test_resources.join("images").join("normal.jpg");
    fs::copy(&source_image1, temp_dir.path().join("valid.jpg")).unwrap();

    let source_image2 = test_resources.join("images").join("normal2.jpg");
    corrupt_file(&source_image2, &temp_dir.path().join("broken.jpg"), 10);

    let params = BrokenFilesParameters::new(CheckedTypes::IMAGE);
    let mut finder = BrokenFiles::new(params);
    finder.set_included_paths(vec![temp_dir.path().to_path_buf()]);
    finder.set_recursive_search(true);
    finder.set_use_cache(false);

    let stop_flag = Arc::new(AtomicBool::new(false));
    finder.search(&stop_flag, None);

    let broken_files = finder.get_broken_files();
    let info = finder.get_information();

    assert_eq!(broken_files.len(), 1, "Should find only 1 broken file out of 2 total");
    assert_eq!(info.number_of_broken_files, 1, "Info should report 1 broken file");
}

#[test]
fn test_multiple_file_types() {
    let temp_dir = TempDir::new().unwrap();
    let test_resources = get_test_resources_path();

    let source_image = test_resources.join("images").join("normal.jpg");
    corrupt_file(&source_image, &temp_dir.path().join("broken.jpg"), 10);

    let source_audio = test_resources.join("audio").join("base.mp3");
    let file_len = fs::metadata(&source_audio).unwrap().len();
    corrupt_file(&source_audio, &temp_dir.path().join("broken.mp3"), file_len as usize);

    let params = BrokenFilesParameters::new(CheckedTypes::IMAGE | CheckedTypes::AUDIO);
    let mut finder = BrokenFiles::new(params);
    finder.set_included_paths(vec![temp_dir.path().to_path_buf()]);
    finder.set_recursive_search(true);
    finder.set_use_cache(false);

    let stop_flag = Arc::new(AtomicBool::new(false));
    finder.search(&stop_flag, None);

    let broken_files = finder.get_broken_files();
    assert_eq!(broken_files.len(), 2, "Should find 2 broken files");
}

#[test]
fn test_empty_directory() {
    let temp_dir = TempDir::new().unwrap();

    let params = BrokenFilesParameters::new(CheckedTypes::IMAGE);
    let mut finder = BrokenFiles::new(params);
    finder.set_included_paths(vec![temp_dir.path().to_path_buf()]);
    finder.set_recursive_search(true);
    finder.set_use_cache(false);

    let stop_flag = Arc::new(AtomicBool::new(false));
    finder.search(&stop_flag, None);

    let broken_files = finder.get_broken_files();
    assert_eq!(broken_files.len(), 0, "Should find no broken files in empty directory");
}

//  JSON

#[test]
fn test_valid_json() {
    let dir = TempDir::new().unwrap();
    fs::write(dir.path().join("valid.json"), r#"{"key": "value", "number": 42, "arr": [1, 2, 3]}"#).unwrap();
    assert_eq!(run_check(&dir, CheckedTypes::MARKUP).len(), 0, "valid JSON should not be flagged");
}

#[test]
fn test_broken_json_unclosed_brace() {
    let dir = TempDir::new().unwrap();
    fs::write(dir.path().join("broken.json"), r#"{"key": "value""#).unwrap();
    let results = run_check(&dir, CheckedTypes::MARKUP);
    assert_eq!(results.len(), 1);
    assert!(results[0].has_errors());
}

#[test]
fn test_broken_json_invalid_value() {
    let dir = TempDir::new().unwrap();
    fs::write(dir.path().join("broken.json"), r#"{"key": undefined}"#).unwrap();
    let results = run_check(&dir, CheckedTypes::MARKUP);
    assert_eq!(results.len(), 1);
    assert!(results[0].has_errors());
}

//  TOML

#[test]
fn test_valid_toml() {
    let dir = TempDir::new().unwrap();
    fs::write(dir.path().join("valid.toml"), "[section]\nkey = \"value\"\nnumber = 42\n").unwrap();
    assert_eq!(run_check(&dir, CheckedTypes::MARKUP).len(), 0, "valid TOML should not be flagged");
}

#[test]
fn test_broken_toml_missing_value() {
    let dir = TempDir::new().unwrap();
    fs::write(dir.path().join("broken.toml"), "[section]\nkey =\n").unwrap();
    let results = run_check(&dir, CheckedTypes::MARKUP);
    assert_eq!(results.len(), 1);
    assert!(results[0].has_errors());
}

#[test]
fn test_broken_toml_duplicate_key() {
    let dir = TempDir::new().unwrap();
    fs::write(dir.path().join("broken.toml"), "key = 1\nkey = 2\n").unwrap();
    let results = run_check(&dir, CheckedTypes::MARKUP);
    assert_eq!(results.len(), 1);
    assert!(results[0].has_errors());
}

//  YAML

#[test]
fn test_valid_yaml() {
    let dir = TempDir::new().unwrap();
    fs::write(dir.path().join("valid.yaml"), "key: value\nnumber: 42\nlist:\n  - a\n  - b\n").unwrap();
    assert_eq!(run_check(&dir, CheckedTypes::MARKUP).len(), 0, "valid YAML should not be flagged");
}

#[test]
fn test_broken_yaml_bad_indent() {
    let dir = TempDir::new().unwrap();
    fs::write(dir.path().join("broken.yaml"), "key: value\n  bad_indent: {\n").unwrap();
    let results = run_check(&dir, CheckedTypes::MARKUP);
    assert_eq!(results.len(), 1);
    assert!(results[0].has_errors());
}

//  XML

#[test]
fn test_valid_xml() {
    let dir = TempDir::new().unwrap();
    fs::write(dir.path().join("valid.xml"), r#"<?xml version="1.0"?><root><child>value</child></root>"#).unwrap();
    assert_eq!(run_check(&dir, CheckedTypes::MARKUP).len(), 0, "valid XML should not be flagged");
}

#[test]
fn test_broken_xml_mismatched_tag() {
    let dir = TempDir::new().unwrap();
    fs::write(dir.path().join("broken.xml"), "<root><child>value</root>").unwrap();
    let results = run_check(&dir, CheckedTypes::MARKUP);
    assert_eq!(results.len(), 1);
    assert!(results[0].has_errors());
}

#[test]
fn test_broken_xml_unclosed_tag() {
    let dir = TempDir::new().unwrap();
    fs::write(dir.path().join("broken.xml"), "<root><unclosed").unwrap();
    let results = run_check(&dir, CheckedTypes::MARKUP);
    assert_eq!(results.len(), 1);
    assert!(results[0].has_errors());
}

//  SVG

#[test]
fn test_valid_svg() {
    let dir = TempDir::new().unwrap();
    fs::write(
        dir.path().join("valid.svg"),
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="100" height="100"><rect width="10" height="10"/></svg>"#,
    )
    .unwrap();
    assert_eq!(run_check(&dir, CheckedTypes::MARKUP).len(), 0, "valid SVG should not be flagged");
}

#[test]
fn test_broken_svg_not_xml() {
    let dir = TempDir::new().unwrap();
    fs::write(dir.path().join("broken.svg"), "this is not svg content at all <<<>>>").unwrap();
    let results = run_check(&dir, CheckedTypes::MARKUP);
    assert_eq!(results.len(), 1);
    assert!(results[0].has_errors());
}

//  No file types selected

#[test]
fn test_no_file_types_selected() {
    let temp_dir = TempDir::new().unwrap();
    let test_resources = get_test_resources_path();

    let source_image = test_resources.join("images").join("normal.jpg");
    corrupt_file(&source_image, &temp_dir.path().join("broken.jpg"), 10);

    let params = BrokenFilesParameters::new(CheckedTypes::NONE);
    let mut finder = BrokenFiles::new(params);
    finder.set_included_paths(vec![temp_dir.path().to_path_buf()]);
    finder.set_recursive_search(true);
    finder.set_use_cache(false);

    let stop_flag = Arc::new(AtomicBool::new(false));
    finder.search(&stop_flag, None);

    let broken_files = finder.get_broken_files();
    assert_eq!(broken_files.len(), 0, "Should find no files when no types are selected");
}

//  Video cache

fn make_broken_entry(name: &str, errors: &[(CheckedTypesSingle, Option<&str>)]) -> BrokenEntry {
    BrokenEntry {
        path: PathBuf::from(name),
        modified_date: 0,
        size: 0,
        errors: errors.iter().map(|(k, v)| (*k, v.map(str::to_string))).collect(),
        repairability: None,
        profiles_when_assessed: 0,
    }
}

#[test]
fn test_video_entry_checked_only_with_ffprobe_needs_recheck_when_both_methods_enabled() {
    let entry = make_broken_entry("a.mp4", &[(CheckedTypesSingle::VideoFfprobe, None)]);
    let both = CheckedTypes::VIDEO_FFPROBE | CheckedTypes::VIDEO_FFMPEG;
    assert!(BrokenFiles::video_entry_missing_required_checks(&entry, both));
}

#[test]
fn test_video_entry_checked_with_both_methods_does_not_need_recheck() {
    let entry = make_broken_entry("a.mp4", &[(CheckedTypesSingle::VideoFfprobe, None), (CheckedTypesSingle::VideoFfmpeg, None)]);
    let both = CheckedTypes::VIDEO_FFPROBE | CheckedTypes::VIDEO_FFMPEG;
    assert!(!BrokenFiles::video_entry_missing_required_checks(&entry, both));
}

#[test]
fn test_move_cached_video_entries_promotes_only_entries_missing_required_checks() {
    let mut records_already_cached = BTreeMap::from([
        (
            "ffprobe_only.mp4".to_string(),
            make_broken_entry("ffprobe_only.mp4", &[(CheckedTypesSingle::VideoFfprobe, None)]),
        ),
        (
            "both.mp4".to_string(),
            make_broken_entry("both.mp4", &[(CheckedTypesSingle::VideoFfprobe, None), (CheckedTypesSingle::VideoFfmpeg, None)]),
        ),
    ]);
    let mut non_cached_files_to_check = BTreeMap::new();

    let both = CheckedTypes::VIDEO_FFPROBE | CheckedTypes::VIDEO_FFMPEG;
    BrokenFiles::move_cached_entries_missing_required_checks(&mut records_already_cached, &mut non_cached_files_to_check, both);

    assert!(records_already_cached.contains_key("both.mp4"), "fully checked entry should stay cached");
    assert!(!records_already_cached.contains_key("ffprobe_only.mp4"), "partially checked entry should be promoted");
    assert!(non_cached_files_to_check.contains_key("ffprobe_only.mp4"));

    let promoted = &non_cached_files_to_check["ffprobe_only.mp4"];
    assert!(promoted.errors.contains_key(&CheckedTypesSingle::VideoFfprobe));
    assert!(!promoted.errors.contains_key(&CheckedTypesSingle::VideoFfmpeg));
}

#[test]
fn test_non_video_entry_never_needs_recheck_when_video_modes_enabled() {
    let entry = make_broken_entry("image.jpg", &[(CheckedTypesSingle::Image, None)]);
    let both = CheckedTypes::VIDEO_FFPROBE | CheckedTypes::VIDEO_FFMPEG;
    assert!(!BrokenFiles::video_entry_missing_required_checks(&entry, both));
}

#[test]
fn test_move_cached_entries_keeps_non_video_entries_cached_when_video_modes_enabled() {
    let mut records_already_cached = BTreeMap::from([
        ("image.jpg".to_string(), make_broken_entry("image.jpg", &[(CheckedTypesSingle::Image, None)])),
        ("archive.zip".to_string(), make_broken_entry("archive.zip", &[(CheckedTypesSingle::Archive, None)])),
    ]);
    let mut non_cached_files_to_check = BTreeMap::new();

    let both = CheckedTypes::VIDEO_FFPROBE | CheckedTypes::VIDEO_FFMPEG;
    BrokenFiles::move_cached_entries_missing_required_checks(&mut records_already_cached, &mut non_cached_files_to_check, both);

    assert!(records_already_cached.contains_key("image.jpg"), "non-video entries should stay cached");
    assert!(records_already_cached.contains_key("archive.zip"), "non-video entries should stay cached");
    assert!(non_cached_files_to_check.is_empty(), "no non-video entry should be promoted for recheck");
}

fn get_heif_images_path() -> std::path::PathBuf {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test_resources").join("heif_images");
    assert!(path.exists(), "HEIF test resources not found at \"{}\"", path.to_string_lossy());
    path
}

#[test]
fn test_valid_avif_not_flagged_as_broken() {
    register_image_decoding_hooks();
    let temp_dir = TempDir::new().unwrap();
    let src = get_heif_images_path().join("small_red.avif");
    fs::copy(&src, temp_dir.path().join("small_red.avif")).unwrap();

    let results = run_check(&temp_dir, CheckedTypes::IMAGE);
    assert_eq!(results.len(), 0, "Valid AVIF should not be flagged as broken");
}

#[test]
fn test_corrupt_avif_is_flagged_as_broken() {
    register_image_decoding_hooks();
    let temp_dir = TempDir::new().unwrap();
    let src = get_heif_images_path().join("small_red.avif");
    let dest = temp_dir.path().join("broken.avif");
    corrupt_file(&src, &dest, 20);

    let results = run_check(&temp_dir, CheckedTypes::IMAGE);
    assert_eq!(results.len(), 1, "Corrupt AVIF should be detected as broken");
    assert!(results[0].has_errors(), "Broken AVIF entry should have errors");
}

#[test]
fn test_multiple_avif_valid_and_broken() {
    register_image_decoding_hooks();
    let temp_dir = TempDir::new().unwrap();
    let heif_path = get_heif_images_path();

    fs::copy(heif_path.join("large_red.avif"), temp_dir.path().join("valid.avif")).unwrap();
    fs::copy(heif_path.join("small_blue.avif"), temp_dir.path().join("valid2.avif")).unwrap();
    corrupt_file(&heif_path.join("small_red.avif"), &temp_dir.path().join("broken.avif"), 20);

    let results = run_check(&temp_dir, CheckedTypes::IMAGE);
    assert_eq!(results.len(), 1, "Only the corrupted AVIF should be detected");
    assert!(results[0].has_errors());
}

//  Repairability

fn write_truncated_noisy_png(path: &PathBuf) {
    use std::io::Cursor;

    use png::{BitDepth, ColorType, Encoder};

    use crate::file_fixer::corrupt::{DamageKind, DamageSpec, Fraction, apply_damage};
    use crate::file_fixer::formats::png_chunks::split_idat;

    let mut state = 12345u32;
    let pixels: Vec<u8> = (0..64 * 200 * 3)
        .map(|_| {
            state = state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            (state >> 24) as u8
        })
        .collect();
    let mut encoded = Vec::new();
    let mut encoder = Encoder::new(Cursor::new(&mut encoded), 64, 200);
    encoder.set_color(ColorType::Rgb);
    encoder.set_depth(BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&pixels).unwrap();
    writer.finish().unwrap();

    let keep = Fraction::new(0.6).unwrap();
    let damaged = apply_damage(&split_idat(&encoded, 2048), &DamageSpec::new(DamageKind::TruncateTail { keep }, 1));
    fs::write(path, damaged.data).unwrap();
}

#[test]
fn test_scan_reports_repairability_only_when_asked() {
    use crate::tools::file_repair::{AssessParameters, RepairPreset, Repairability};

    let temp_dir = TempDir::new().unwrap();
    crate::common::config_cache_path::set_config_cache_path_test(temp_dir.path().join("cache"), temp_dir.path().join("config"));
    let scanned = temp_dir.path().join("scanned");
    fs::create_dir(&scanned).unwrap();
    write_truncated_noisy_png(&scanned.join("photo.png"));

    let mut finder = BrokenFiles::new(BrokenFilesParameters::new(CheckedTypes::IMAGE).with_repair_assessment(AssessParameters::default()));
    finder.set_included_paths(vec![scanned.clone()]);
    finder.set_recursive_search(true);
    finder.set_use_cache(false);
    finder.search(&Arc::new(AtomicBool::new(false)), None);
    assert_eq!(finder.get_broken_files().len(), 1);
    assert_eq!(finder.get_broken_files()[0].repairability, Some(Repairability::Repairable(RepairPreset::Balanced)));

    let mut plain_finder = BrokenFiles::new(BrokenFilesParameters::new(CheckedTypes::IMAGE));
    plain_finder.set_included_paths(vec![scanned]);
    plain_finder.set_recursive_search(true);
    plain_finder.set_use_cache(false);
    plain_finder.search(&Arc::new(AtomicBool::new(false)), None);
    assert_eq!(plain_finder.get_broken_files().len(), 1);
    assert_eq!(plain_finder.get_broken_files()[0].repairability, None);
}

#[test]
fn test_assessment_is_redone_only_when_the_verdict_may_have_changed() {
    use crate::tools::broken_files::repair::needs_assessment;
    use crate::tools::file_repair::{AssessParameters, RepairPreset, Repairability};

    let params = AssessParameters::default();
    let mut entry = make_broken_entry("photo.png", &[(CheckedTypesSingle::Image, Some("broken"))]);
    assert!(needs_assessment(&entry, &params, 0), "never assessed");

    entry.repairability = Some(Repairability::Repairable(RepairPreset::Safe));
    assert!(!needs_assessment(&entry, &params, 5), "a found repair stays valid");

    entry.repairability = Some(Repairability::NotRepairable);
    entry.profiles_when_assessed = 3;
    assert!(!needs_assessment(&entry, &params, 3), "nothing new to learn from");
    assert!(needs_assessment(&entry, &params, 4), "new camera profiles may help");

    entry.repairability = Some(Repairability::TooLarge);
    entry.size = 100;
    assert!(needs_assessment(&entry, &AssessParameters { max_file_size: 1000, ..params }, 3), "limit was raised");
    assert!(!needs_assessment(&entry, &AssessParameters { max_file_size: 10, ..params }, 3), "still too large");

    let healthy = make_broken_entry("photo.png", &[(CheckedTypesSingle::Image, None)]);
    assert!(!needs_assessment(&healthy, &params, 0), "healthy files are never assessed");
}

#[test]
fn test_only_repairable_drops_files_that_cannot_be_fixed() {
    use crate::tools::file_repair::AssessParameters;

    let temp_dir = TempDir::new().unwrap();
    crate::common::config_cache_path::set_config_cache_path_test(temp_dir.path().join("cache"), temp_dir.path().join("config"));
    let scanned = temp_dir.path().join("scanned");
    fs::create_dir(&scanned).unwrap();
    write_truncated_noisy_png(&scanned.join("fixable.png"));
    fs::write(scanned.join("hopeless.png"), b"this is not an image at all").unwrap();

    let scan = |only_repairable: bool| {
        let params = BrokenFilesParameters::new(CheckedTypes::IMAGE)
            .with_repair_assessment(AssessParameters::default())
            .with_only_repairable(only_repairable);
        let mut finder = BrokenFiles::new(params);
        finder.set_included_paths(vec![scanned.clone()]);
        finder.set_recursive_search(true);
        finder.set_use_cache(false);
        finder.search(&Arc::new(AtomicBool::new(false)), None);
        finder
            .get_broken_files()
            .iter()
            .map(|entry| entry.path.file_name().unwrap().to_string_lossy().to_string())
            .collect::<Vec<_>>()
    };

    assert_eq!(scan(false), vec!["fixable.png", "hopeless.png"]);
    assert_eq!(scan(true), vec!["fixable.png"]);
}
