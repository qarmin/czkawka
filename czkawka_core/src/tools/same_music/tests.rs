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

    assert_eq!(info.number_of_duplicates, 2);
    assert_eq!(info.number_of_groups, 1);
    assert_eq!(duplicates.len(), 1);
    assert_eq!(duplicates.iter().map(|e| e.len()).sum::<usize>(), 3);
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

#[test]
fn test_same_music_reference_mode_deletes_only_non_reference() {
    use std::fs;
    use std::path::Path;

    use tempfile::TempDir;

    use crate::common::tool_data::DeleteMethod;
    use crate::common::traits::DeletingItems;
    use crate::tools::same_music::MusicEntry;

    let temp_dir = TempDir::new().unwrap();
    let reference = temp_dir.path().join("reference.mp3");
    let duplicate = temp_dir.path().join("duplicate.mp3");
    fs::write(&reference, "ref").unwrap();
    fs::write(&duplicate, "dup").unwrap();

    let mk = |path: &Path| MusicEntry {
        size: 3,
        path: path.to_path_buf(),
        modified_date: 0,
        fingerprint: Vec::new(),
        track_title: String::new(),
        track_artist: String::new(),
        year: String::new(),
        length: 0,
        genre: String::new(),
        bitrate: 0,
    };

    let params = SameMusicParameters::new(MusicSimilarity::TRACK_TITLE, false, CheckingMethod::AudioTags, 10.0, 0.2, false);
    let mut finder = SameMusic::new(params);
    finder.set_delete_method(DeleteMethod::Delete);
    finder.set_move_to_trash(false);
    finder.set_use_reference_folders(true);
    finder.duplicated_music_entries_referenced = vec![(mk(&reference), vec![mk(&duplicate)])];

    let stop_flag = Arc::new(AtomicBool::new(false));
    finder.delete_files(&stop_flag, None);

    assert!(reference.exists(), "Reference track must be kept");
    assert!(!duplicate.exists(), "Non-reference duplicate must be deleted (#1643)");
}
