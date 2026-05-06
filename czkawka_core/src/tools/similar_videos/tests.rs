use std::path::PathBuf;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Once};

use tempfile::TempDir;
use vid_dup_finder_lib::Cropdetect;

use crate::common::config_cache_path::set_config_cache_path_test;
use crate::common::tool_data::CommonData;
use crate::common::traits::Search;
use crate::tools::similar_videos::{
    DEFAULT_AUDIO_LENGTH_RATIO, DEFAULT_AUDIO_MAXIMUM_DIFFERENCE, DEFAULT_AUDIO_MIN_DURATION_SECONDS, DEFAULT_AUDIO_SIMILARITY_PERCENT, SimilarVideos, SimilarVideosParameters,
};

static INIT: Once = Once::new();

fn setup_cache_path() {
    INIT.call_once(|| {
        let temp_cache = TempDir::new().expect("Failed to create temp cache dir");
        let temp_config = TempDir::new().expect("Failed to create temp config dir");
        let cache_path = temp_cache.path().to_path_buf();
        let config_path = temp_config.path().to_path_buf();
        set_config_cache_path_test(cache_path, config_path);
        std::mem::forget(temp_cache);
        std::mem::forget(temp_config);
    });
}

fn make_params_visual() -> SimilarVideosParameters {
    SimilarVideosParameters::new(
        10,
        false,
        false,
        15,
        10,
        Cropdetect::Letterbox,
        false,
        0,
        false,
        2,
        false,
        DEFAULT_AUDIO_SIMILARITY_PERCENT,
        DEFAULT_AUDIO_MAXIMUM_DIFFERENCE,
        DEFAULT_AUDIO_LENGTH_RATIO,
        DEFAULT_AUDIO_MIN_DURATION_SECONDS,
    )
}

fn make_params_audio() -> SimilarVideosParameters {
    SimilarVideosParameters::new(
        10,
        false,
        false,
        15,
        10,
        Cropdetect::Letterbox,
        false,
        0,
        false,
        2,
        true,
        DEFAULT_AUDIO_SIMILARITY_PERCENT,
        DEFAULT_AUDIO_MAXIMUM_DIFFERENCE,
        DEFAULT_AUDIO_LENGTH_RATIO,
        DEFAULT_AUDIO_MIN_DURATION_SECONDS,
    )
}

fn get_audio_resources_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test_resources").join("audio")
}

// Tests are quite limited here, due to the needing of external ffmpeg libraries and video files.
// Just tested is that searching in an empty directory works as expected - no found similar videos

#[test]
fn test_similar_videos_empty_directory() {
    let temp_dir = TempDir::new().unwrap();
    let path = temp_dir.path();

    let mut finder = SimilarVideos::new(make_params_visual());
    finder.set_included_paths(vec![path.to_path_buf()]);
    finder.set_recursive_search(true);
    finder.set_use_cache(false);

    let stop_flag = Arc::new(AtomicBool::new(false));
    finder.search(&stop_flag, None);

    let info = finder.get_information();
    assert_eq!(info.number_of_duplicates, 0, "Should find no duplicates in empty directory");
    assert_eq!(info.number_of_groups, 0, "Should find no groups in empty directory");
}

#[test]
fn test_similar_videos_audio_mode_empty_directory() {
    setup_cache_path();

    let temp_dir = TempDir::new().unwrap();
    let path = temp_dir.path();

    let mut finder = SimilarVideos::new(make_params_audio());
    finder.set_included_paths(vec![path.to_path_buf()]);
    finder.set_recursive_search(true);
    finder.set_use_cache(false);

    let stop_flag = Arc::new(AtomicBool::new(false));
    finder.search(&stop_flag, None);

    let info = finder.get_information();
    assert_eq!(info.number_of_duplicates, 0, "Should find no duplicates in empty directory (audio mode)");
    assert_eq!(info.number_of_groups, 0, "Should find no groups in empty directory (audio mode)");
}

// Audio mode only processes VIDEO_FILES_EXTENSIONS, so MP3 audio test resources are invisible to it.
// This test verifies that scanning a directory containing only audio files (MP3) yields no results.
#[test]
fn test_similar_videos_audio_mode_ignores_non_video_files() {
    setup_cache_path();

    let test_path = get_audio_resources_path();
    assert!(test_path.exists(), "Test resources not found at \"{}\"", test_path.to_string_lossy());

    let mut finder = SimilarVideos::new(make_params_audio());
    finder.set_included_paths(vec![test_path]);
    finder.set_recursive_search(true);
    finder.set_use_cache(false);

    let stop_flag = Arc::new(AtomicBool::new(false));
    finder.search(&stop_flag, None);

    let info = finder.get_information();
    assert_eq!(info.number_of_duplicates, 0, "Should find no video duplicates when only audio (MP3) files are present");
    assert_eq!(info.number_of_groups, 0, "Should find no groups when only audio (MP3) files are present");
}
