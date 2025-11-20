#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use std::sync::Arc;
    use std::sync::atomic::AtomicBool;

    use crate::common::model::CheckingMethod;
    use crate::common::tool_data::CommonData;
    use crate::common::traits::Search;
    use crate::tools::same_music::{MusicSimilarity, SameMusic, SameMusicParameters};

    fn get_test_resources_path() -> PathBuf {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("test_resources")
            .join("audio");

        assert!(path.exists(), "Test resources not found at {:?}", path);

        path
    }

    #[test]
    fn test_find_same_music_by_tags() {
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
        finder.set_included_directory(vec![test_path]);
        finder.set_recursive_search(true);
        finder.set_use_cache(false);

        let stop_flag = Arc::new(AtomicBool::new(false));
        finder.search(&stop_flag, None);

        let info = finder.get_information();
        let duplicates = finder.get_duplicated_music_entries();

        assert_eq!(info.number_of_groups, 5);
        assert!(info.number_of_duplicates > 0);
        assert!(!duplicates.is_empty());
    }

    #[test]
    fn test_find_same_music_by_content() {
        let test_path = get_test_resources_path();

        let params = SameMusicParameters::new(
            MusicSimilarity::TRACK_TITLE,
            false,
            CheckingMethod::AudioContent,
            10.0,
            0.2,
            false,
        );

        let mut finder = SameMusic::new(params);
        finder.set_included_directory(vec![test_path]);
        finder.set_recursive_search(true);
        finder.set_use_cache(false);

        let stop_flag = Arc::new(AtomicBool::new(false));
        finder.search(&stop_flag, None);

        let info = finder.get_information();
        let duplicates = finder.get_duplicated_music_entries();

        assert!(info.number_of_groups >= 1);
        assert!(info.number_of_duplicates >= 2);
        assert!(!duplicates.is_empty());
    }

    #[test]
    fn test_same_music_with_similar_files() {
        let test_path = get_test_resources_path();

        let params = SameMusicParameters::new(
            MusicSimilarity::TRACK_TITLE,
            true,
            CheckingMethod::AudioContent,
            5.0,
            0.4,
            false,
        );

        let mut finder = SameMusic::new(params);
        finder.set_included_directory(vec![test_path]);
        finder.set_recursive_search(true);
        finder.set_use_cache(false);

        let stop_flag = Arc::new(AtomicBool::new(false));
        finder.search(&stop_flag, None);

        let info = finder.get_information();
        let duplicates = finder.get_duplicated_music_entries();

        assert!(info.number_of_groups >= 1);
        if !duplicates.is_empty() {
            assert!(duplicates[0].len() >= 2);
        }
    }

    #[test]
    fn test_same_music_no_recursion() {
        let test_path = get_test_resources_path();

        let params = SameMusicParameters::new(
            MusicSimilarity::TRACK_TITLE,
            false,
            CheckingMethod::AudioTags,
            10.0,
            0.2,
            false,
        );

        let mut finder = SameMusic::new(params);
        finder.set_included_directory(vec![test_path]);
        finder.set_recursive_search(false);
        finder.set_use_cache(false);

        let stop_flag = Arc::new(AtomicBool::new(false));
        finder.search(&stop_flag, None);

        let info = finder.get_information();
        let duplicates = finder.get_duplicated_music_entries();

        assert!(info.number_of_groups >= 0);
        assert_eq!(duplicates.len(), info.number_of_groups as usize);
    }

    #[test]
    fn test_same_music_with_multiple_similarity_criteria() {
        let test_path = get_test_resources_path();

        let params = SameMusicParameters::new(
            MusicSimilarity::TRACK_TITLE | MusicSimilarity::TRACK_ARTIST | MusicSimilarity::YEAR,
            false,
            CheckingMethod::AudioTags,
            10.0,
            0.2,
            false,
        );

        let mut finder = SameMusic::new(params);
        finder.set_included_directory(vec![test_path]);
        finder.set_recursive_search(true);
        finder.set_use_cache(false);

        let stop_flag = Arc::new(AtomicBool::new(false));
        finder.search(&stop_flag, None);

        let info = finder.get_information();
        let duplicates = finder.get_duplicated_music_entries();

        assert_eq!(info.number_of_groups, 5);
        assert!(info.number_of_duplicates >= 10);
        assert!(!duplicates.is_empty());
    }

    #[test]
    fn test_same_music_compare_fingerprints_with_similar_titles() {
        let test_path = get_test_resources_path();

        let params = SameMusicParameters::new(
            MusicSimilarity::TRACK_TITLE,
            false,
            CheckingMethod::AudioContent,
            10.0,
            0.2,
            true,
        );

        let mut finder = SameMusic::new(params);
        finder.set_included_directory(vec![test_path]);
        finder.set_recursive_search(true);
        finder.set_use_cache(false);

        let stop_flag = Arc::new(AtomicBool::new(false));
        finder.search(&stop_flag, None);

        let info = finder.get_information();
        let duplicates = finder.get_duplicated_music_entries();

        assert_eq!(info.number_of_groups, 45);
        assert!(info.number_of_duplicates >= 90);
        assert!(!duplicates.is_empty());
    }

    #[test]
    fn test_same_music_empty_directory() {
        use tempfile::TempDir;

        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path();

        let params = SameMusicParameters::new(
            MusicSimilarity::TRACK_TITLE,
            false,
            CheckingMethod::AudioTags,
            10.0,
            0.2,
            false,
        );

        let mut finder = SameMusic::new(params);
        finder.set_included_directory(vec![path.to_path_buf()]);
        finder.set_recursive_search(true);
        finder.set_use_cache(false);

        let stop_flag = Arc::new(AtomicBool::new(false));
        finder.search(&stop_flag, None);

        let info = finder.get_information();
        let duplicates = finder.get_duplicated_music_entries();

        assert_eq!(info.number_of_groups, 0);
        assert_eq!(info.number_of_duplicates, 0);
        assert!(duplicates.is_empty());
    }
}

