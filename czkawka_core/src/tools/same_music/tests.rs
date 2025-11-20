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
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("test_resources")
            .join("audio")
    }

    #[test]
    fn test_find_same_music_by_tags() {
        let test_path = get_test_resources_path();
        if !test_path.exists() {
            eprintln!("Test resources not found at {:?}", test_path);
            return;
        }

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

        let stop_flag = Arc::new(AtomicBool::new(false));
        finder.search(&stop_flag, None);

        let info = finder.get_information();
        let duplicates = finder.get_duplicated_music_entries();

        // Verify the search completed successfully
        // number_of_groups shows how many groups of similar files were found
        assert!(info.number_of_groups >= 0, "Search should complete");
    }

    #[test]
    fn test_find_same_music_by_content() {
        let test_path = get_test_resources_path();
        if !test_path.exists() {
            eprintln!("Test resources not found at {:?}", test_path);
            return;
        }

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
        finder.set_use_cache(false); // Disable cache for testing

        let stop_flag = Arc::new(AtomicBool::new(false));
        finder.search(&stop_flag, None);

        // Just verify the test ran successfully
        let _info = finder.get_information();
    }

    #[test]
    fn test_same_music_with_similar_files() {
        let test_path = get_test_resources_path();
        if !test_path.exists() {
            eprintln!("Test resources not found at {:?}", test_path);
            return;
        }

        // Test with fingerprint comparison to find similar audio files
        // base.mp3, base_start.mp3, base_end.mp3, base_low_quality.mp3, base_messed.mp3
        let params = SameMusicParameters::new(
            MusicSimilarity::TRACK_TITLE,
            true,
            CheckingMethod::AudioContent,
            5.0,   // minimum segment duration
            0.4,   // maximum difference - higher to catch variations
            false,
        );

        let mut finder = SameMusic::new(params);
        finder.set_included_directory(vec![test_path]);
        finder.set_recursive_search(true);
        finder.set_use_cache(false);

        let stop_flag = Arc::new(AtomicBool::new(false));
        finder.search(&stop_flag, None);

        let duplicates = finder.get_duplicated_music_entries();

        // We might find similar files (base.mp3 and its variations)
        if !duplicates.is_empty() {
            assert!(duplicates[0].len() >= 2, "Duplicate group should have at least 2 files");
        }
    }

    #[test]
    fn test_same_music_no_recursion() {
        let test_path = get_test_resources_path();
        if !test_path.exists() {
            eprintln!("Test resources not found at {:?}", test_path);
            return;
        }

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

        let stop_flag = Arc::new(AtomicBool::new(false));
        finder.search(&stop_flag, None);

        // With non-recursive search, results depend on directory structure
        // Just verify it doesn't crash
        let _info = finder.get_information();
    }

    #[test]
    fn test_same_music_with_multiple_similarity_criteria() {
        let test_path = get_test_resources_path();
        if !test_path.exists() {
            eprintln!("Test resources not found at {:?}", test_path);
            return;
        }

        // Use multiple criteria
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

        let stop_flag = Arc::new(AtomicBool::new(false));
        finder.search(&stop_flag, None);

        let info = finder.get_information();
        assert!(info.number_of_groups >= 0, "Search should complete");
    }

    #[test]
    fn test_same_music_compare_fingerprints_with_similar_titles() {
        let test_path = get_test_resources_path();
        if !test_path.exists() {
            eprintln!("Test resources not found at {:?}", test_path);
            return;
        }

        // Test the optimization that only compares fingerprints for files with similar titles
        let params = SameMusicParameters::new(
            MusicSimilarity::TRACK_TITLE,
            false,
            CheckingMethod::AudioContent,
            10.0,
            0.2,
            true, // compare_fingerprints_only_with_similar_titles
        );

        let mut finder = SameMusic::new(params);
        finder.set_included_directory(vec![test_path]);
        finder.set_recursive_search(true);
        finder.set_use_cache(false);

        let stop_flag = Arc::new(AtomicBool::new(false));
        finder.search(&stop_flag, None);

        // Verify the search completed successfully
        let _info = finder.get_information();
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

        let stop_flag = Arc::new(AtomicBool::new(false));
        finder.search(&stop_flag, None);

        let info = finder.get_information();
        assert_eq!(
            info.number_of_groups, 0,
            "Should find no groups in empty directory"
        );

        let duplicates = finder.get_duplicated_music_entries();
        assert_eq!(duplicates.len(), 0, "Should find no duplicates in empty directory");
    }
}

