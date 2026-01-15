use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use crate::common::model::CheckingMethod;
use crate::common::tool_data::CommonData;
use crate::common::traits::Search;
use crate::tools::same_music::{MusicSimilarity, SameMusic, SameMusicParameters};

fn get_test_resources_path() -> PathBuf {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test_resources").join("audio");

    assert!(path.exists(), "Test resources not found at \"{}\"", path.to_string_lossy());

    path
}

#[test]
fn test_same_music_by_content_high_similarity() {
    let test_path = get_test_resources_path();

    let params = SameMusicParameters::new(MusicSimilarity::TRACK_TITLE, false, CheckingMethod::AudioContent, 10.0, 0.2, false);

    let mut finder = SameMusic::new(params);
    finder.set_included_paths(vec![test_path]);
    finder.set_recursive_search(true);
    finder.set_use_cache(false);

    let stop_flag = Arc::new(AtomicBool::new(false));
    finder.search(&stop_flag, None);

    let info = finder.get_information();
    let duplicates = finder.get_duplicated_music_entries();

    assert_eq!(info.number_of_duplicates, 1);
    assert_eq!(info.number_of_groups, 1);
    assert_eq!(duplicates.len(), 1);
    assert_eq!(duplicates.iter().map(|e| e.len()).sum::<usize>(), 2);
}

#[test]
fn test_same_music_by_content_medium_similarity() {
    let test_path = get_test_resources_path();

    let params = SameMusicParameters::new(MusicSimilarity::TRACK_TITLE, false, CheckingMethod::AudioContent, 10.0, 0.5, false);

    let mut finder = SameMusic::new(params);
    finder.set_included_paths(vec![test_path]);
    finder.set_recursive_search(true);
    finder.set_use_cache(false);

    let stop_flag = Arc::new(AtomicBool::new(false));
    finder.search(&stop_flag, None);

    let info = finder.get_information();
    let duplicates = finder.get_duplicated_music_entries();

    assert_eq!(info.number_of_duplicates, 1);
    assert_eq!(info.number_of_groups, 1);
    assert_eq!(duplicates.len(), 1);
    assert_eq!(duplicates.iter().map(|e| e.len()).sum::<usize>(), 2);
}

#[test]
fn test_same_music_by_content_low_similarity() {
    let test_path = get_test_resources_path();

    let params = SameMusicParameters::new(MusicSimilarity::TRACK_TITLE, false, CheckingMethod::AudioContent, 10.0, 0.8, false);

    let mut finder = SameMusic::new(params);
    finder.set_included_paths(vec![test_path]);
    finder.set_recursive_search(true);
    finder.set_use_cache(false);

    let stop_flag = Arc::new(AtomicBool::new(false));
    finder.search(&stop_flag, None);

    let info = finder.get_information();
    let duplicates = finder.get_duplicated_music_entries();

    assert_eq!(info.number_of_duplicates, 3);
    assert_eq!(info.number_of_groups, 1);
    assert_eq!(duplicates.len(), 1);
    assert_eq!(duplicates.iter().map(|e| e.len()).sum::<usize>(), 4);
}

#[test]
fn test_same_music_by_tags_title_artist() {
    let test_path = get_test_resources_path();

    let params = SameMusicParameters::new(
        MusicSimilarity::TRACK_TITLE | MusicSimilarity::TRACK_ARTIST,
        false,
        CheckingMethod::AudioTags,
        10.0,
        0.2,
        false,
    );

    let mut finder = SameMusic::new(params);
    finder.set_included_paths(vec![test_path]);
    finder.set_recursive_search(true);
    finder.set_use_cache(false);

    let stop_flag = Arc::new(AtomicBool::new(false));
    finder.search(&stop_flag, None);

    let info = finder.get_information();
    let duplicates = finder.get_duplicated_music_entries();

    assert_eq!(info.number_of_duplicates, 4);
    assert_eq!(info.number_of_groups, 1);
    assert_eq!(duplicates.len(), 1);
    assert_eq!(duplicates[0].len(), 5);
}

#[test]
fn test_same_music_by_tags_year() {
    let test_path = get_test_resources_path();

    let params = SameMusicParameters::new(MusicSimilarity::YEAR, false, CheckingMethod::AudioTags, 10.0, 0.2, false);

    let mut finder = SameMusic::new(params);
    finder.set_included_paths(vec![test_path]);
    finder.set_recursive_search(true);
    finder.set_use_cache(false);

    let stop_flag = Arc::new(AtomicBool::new(false));
    finder.search(&stop_flag, None);

    let info = finder.get_information();
    let duplicates = finder.get_duplicated_music_entries();

    assert_eq!(info.number_of_duplicates, 0);
    assert_eq!(info.number_of_groups, 0);
    assert_eq!(duplicates.len(), 0);
}

#[test]
fn test_same_music_by_tags_genre() {
    let test_path = get_test_resources_path();

    let params = SameMusicParameters::new(MusicSimilarity::GENRE, false, CheckingMethod::AudioTags, 10.0, 0.2, false);

    let mut finder = SameMusic::new(params);
    finder.set_included_paths(vec![test_path]);
    finder.set_recursive_search(true);
    finder.set_use_cache(false);

    let stop_flag = Arc::new(AtomicBool::new(false));
    finder.search(&stop_flag, None);

    let info = finder.get_information();
    let duplicates = finder.get_duplicated_music_entries();

    assert_eq!(info.number_of_duplicates, 4);
    assert_eq!(info.number_of_groups, 1);
    assert_eq!(duplicates.len(), 1);
    assert_eq!(duplicates[0].len(), 5);
}

#[test]
fn test_same_music_by_tags_bitrate() {
    let test_path = get_test_resources_path();

    let params = SameMusicParameters::new(MusicSimilarity::BITRATE, false, CheckingMethod::AudioTags, 10.0, 0.2, false);

    let mut finder = SameMusic::new(params);
    finder.set_included_paths(vec![test_path]);
    finder.set_recursive_search(true);
    finder.set_use_cache(false);

    let stop_flag = Arc::new(AtomicBool::new(false));
    finder.search(&stop_flag, None);

    let info = finder.get_information();
    let duplicates = finder.get_duplicated_music_entries();

    assert_eq!(info.number_of_duplicates, 2);
    assert_eq!(info.number_of_groups, 1);
    assert_eq!(duplicates.len(), 1);
    assert_eq!(duplicates.iter().map(|e| e.len()).sum::<usize>(), 3);
}

#[test]
fn test_same_music_by_tags_all_criteria() {
    let test_path = get_test_resources_path();

    let params = SameMusicParameters::new(
        MusicSimilarity::TRACK_TITLE | MusicSimilarity::TRACK_ARTIST | MusicSimilarity::YEAR | MusicSimilarity::GENRE,
        false,
        CheckingMethod::AudioTags,
        10.0,
        0.2,
        false,
    );

    let mut finder = SameMusic::new(params);
    finder.set_included_paths(vec![test_path]);
    finder.set_recursive_search(true);
    finder.set_use_cache(false);

    let stop_flag = Arc::new(AtomicBool::new(false));
    finder.search(&stop_flag, None);

    let info = finder.get_information();
    let duplicates = finder.get_duplicated_music_entries();

    assert_eq!(info.number_of_duplicates, 0);
    assert_eq!(info.number_of_groups, 0);
    assert_eq!(duplicates.len(), 0);
}

#[test]
fn test_same_music_approximate_comparison() {
    let test_path = get_test_resources_path();

    let params = SameMusicParameters::new(
        MusicSimilarity::TRACK_TITLE | MusicSimilarity::TRACK_ARTIST,
        true,
        CheckingMethod::AudioTags,
        10.0,
        0.2,
        false,
    );

    let mut finder = SameMusic::new(params);
    finder.set_included_paths(vec![test_path]);
    finder.set_recursive_search(true);
    finder.set_use_cache(false);

    let stop_flag = Arc::new(AtomicBool::new(false));
    finder.search(&stop_flag, None);

    let info = finder.get_information();
    let duplicates = finder.get_duplicated_music_entries();

    assert_eq!(info.number_of_duplicates, 4);
    assert_eq!(info.number_of_groups, 1);
    assert_eq!(duplicates.len(), 1);
    assert_eq!(duplicates[0].len(), 5);
}

#[test]
fn test_same_music_empty_directory() {
    use tempfile::TempDir;

    let temp_dir = TempDir::new().unwrap();
    let path = temp_dir.path();

    let params = SameMusicParameters::new(MusicSimilarity::TRACK_TITLE, false, CheckingMethod::AudioTags, 10.0, 0.2, false);

    let mut finder = SameMusic::new(params);
    finder.set_included_paths(vec![path.to_path_buf()]);
    finder.set_recursive_search(true);
    finder.set_use_cache(false);

    let stop_flag = Arc::new(AtomicBool::new(false));
    finder.search(&stop_flag, None);

    let info = finder.get_information();
    let duplicates = finder.get_duplicated_music_entries();

    assert_eq!(info.number_of_duplicates, 0);
    assert_eq!(info.number_of_groups, 0);
    assert_eq!(duplicates.len(), 0);
}
