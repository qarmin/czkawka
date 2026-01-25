#[cfg(test)]
mod tests2 {
    use std::fs;
    use std::sync::Arc;
    use std::sync::atomic::AtomicBool;

    use crate::common::tool_data::CommonData;
    use crate::common::traits::Search;
    use crate::tools::bad_names::{BadNames, BadNamesParameters, NameIssues};

    #[test]
    fn test_uppercase_extension_detection() {
        let temp_dir = tempfile::tempdir().unwrap();
        let test_file = temp_dir.path().join("test.TXT");
        fs::write(&test_file, "test").unwrap();

        let params = BadNamesParameters::new(NameIssues {
            uppercase_extension: true,
            emoji_used: false,
            space_at_start_or_end: false,
            non_ascii_graphical: false,
            restricted_charset_allowed: vec![],
            remove_duplicated_non_alphanumeric: false,
        });
        let mut bad_names = BadNames::new(params);
        bad_names.get_cd_mut().directories.set_included_paths(vec![temp_dir.path().to_path_buf()]);

        let stop_flag = Arc::new(AtomicBool::new(false));
        bad_names.search(&stop_flag, None);

        assert_eq!(bad_names.get_bad_names_files().len(), 1);
        assert_eq!(bad_names.get_bad_names_files()[0].new_name, "test.txt");
    }

    #[test]
    fn test_emoji_detection() {
        let temp_dir = tempfile::tempdir().unwrap();
        let test_file = temp_dir.path().join("test😀.txt");
        fs::write(&test_file, "test").unwrap();

        let params = BadNamesParameters::new(NameIssues {
            uppercase_extension: false,
            emoji_used: true,
            space_at_start_or_end: false,
            non_ascii_graphical: false,
            restricted_charset_allowed: vec![],
            remove_duplicated_non_alphanumeric: false,
        });
        let mut bad_names = BadNames::new(params);
        bad_names.get_cd_mut().directories.set_included_paths(vec![temp_dir.path().to_path_buf()]);

        let stop_flag = Arc::new(AtomicBool::new(false));
        bad_names.search(&stop_flag, None);

        assert_eq!(bad_names.get_bad_names_files().len(), 1);
        assert_eq!(bad_names.get_bad_names_files()[0].new_name, "test.txt");
    }

    #[test]
    fn test_space_at_start_end_stem_detection() {
        let temp_dir = tempfile::tempdir().unwrap();
        let test_file = temp_dir.path().join(" test .txt");
        fs::write(&test_file, "test").unwrap();

        let params = BadNamesParameters::new(NameIssues {
            uppercase_extension: false,
            emoji_used: false,
            space_at_start_or_end: true,
            non_ascii_graphical: false,
            restricted_charset_allowed: vec![],
            remove_duplicated_non_alphanumeric: false,
        });
        let mut bad_names = BadNames::new(params);
        bad_names.get_cd_mut().directories.set_included_paths(vec![temp_dir.path().to_path_buf()]);

        let stop_flag = Arc::new(AtomicBool::new(false));
        bad_names.search(&stop_flag, None);

        assert_eq!(bad_names.get_bad_names_files().len(), 1);
        assert_eq!(bad_names.get_bad_names_files()[0].new_name, "test.txt");
    }

    #[test]
    fn test_space_at_start_end_extension_detection() {
        let temp_dir = tempfile::tempdir().unwrap();
        let test_file = temp_dir.path().join("test. txt ");
        fs::write(&test_file, "test").unwrap();

        let params = BadNamesParameters::new(NameIssues {
            uppercase_extension: false,
            emoji_used: false,
            space_at_start_or_end: true,
            non_ascii_graphical: false,
            restricted_charset_allowed: vec![],
            remove_duplicated_non_alphanumeric: false,
        });
        let mut bad_names = BadNames::new(params);
        bad_names.get_cd_mut().directories.set_included_paths(vec![temp_dir.path().to_path_buf()]);

        let stop_flag = Arc::new(AtomicBool::new(false));
        bad_names.search(&stop_flag, None);

        assert_eq!(bad_names.get_bad_names_files().len(), 1);
        assert_eq!(bad_names.get_bad_names_files()[0].new_name, "test.txt");
    }

    #[test]
    fn test_non_ascii_graphical_detection() {
        let temp_dir = tempfile::tempdir().unwrap();
        let test_file = temp_dir.path().join("tëst.txt");
        fs::write(&test_file, "test").unwrap();

        let params = BadNamesParameters::new(NameIssues {
            uppercase_extension: false,
            emoji_used: false,
            space_at_start_or_end: false,
            non_ascii_graphical: true,
            restricted_charset_allowed: vec![],
            remove_duplicated_non_alphanumeric: false,
        });
        let mut bad_names = BadNames::new(params);
        bad_names.get_cd_mut().directories.set_included_paths(vec![temp_dir.path().to_path_buf()]);

        let stop_flag = Arc::new(AtomicBool::new(false));
        bad_names.search(&stop_flag, None);

        assert_eq!(bad_names.get_bad_names_files().len(), 1);
        assert_eq!(bad_names.get_bad_names_files()[0].new_name, "tst.txt");
    }

    #[test]
    fn test_restricted_charset_detection() {
        let temp_dir = tempfile::tempdir().unwrap();
        let test_file = temp_dir.path().join("test@file.txt");
        fs::write(&test_file, "test").unwrap();

        let params = BadNamesParameters::new(NameIssues {
            uppercase_extension: false,
            emoji_used: false,
            space_at_start_or_end: false,
            non_ascii_graphical: false,
            restricted_charset_allowed: vec!['_', '-', ' '],  // Allow only these + alphanumeric
            remove_duplicated_non_alphanumeric: false,
        });
        let mut bad_names = BadNames::new(params);
        bad_names.get_cd_mut().directories.set_included_paths(vec![temp_dir.path().to_path_buf()]);

        let stop_flag = Arc::new(AtomicBool::new(false));
        bad_names.search(&stop_flag, None);

        assert_eq!(bad_names.get_bad_names_files().len(), 1);
        assert_eq!(bad_names.get_bad_names_files()[0].new_name, "testfile.txt");
    }

    #[test]
    fn test_duplicated_non_alphanumeric() {
        let temp_dir = tempfile::tempdir().unwrap();
        let test_file = temp_dir.path().join("test__file--name.txt");
        fs::write(&test_file, "test").unwrap();

        let params = BadNamesParameters::new(NameIssues {
            uppercase_extension: false,
            emoji_used: false,
            space_at_start_or_end: false,
            non_ascii_graphical: false,
            restricted_charset_allowed: vec![],
            remove_duplicated_non_alphanumeric: true,
        });
        let mut bad_names = BadNames::new(params);
        bad_names.get_cd_mut().directories.set_included_paths(vec![temp_dir.path().to_path_buf()]);

        let stop_flag = Arc::new(AtomicBool::new(false));
        bad_names.search(&stop_flag, None);

        assert_eq!(bad_names.get_bad_names_files().len(), 1);
        assert_eq!(bad_names.get_bad_names_files()[0].new_name, "test_file-name.txt");
    }

    #[test]
    fn test_multiple_issues() {
        let temp_dir = tempfile::tempdir().unwrap();
        let test_file = temp_dir.path().join(" tëst😀 .TXT ");
        fs::write(&test_file, "test").unwrap();

        let mut bad_names = BadNames::new(BadNamesParameters::new(NameIssues::all()));
        bad_names.get_cd_mut().directories.set_included_paths(vec![temp_dir.path().to_path_buf()]);

        let stop_flag = Arc::new(AtomicBool::new(false));
        bad_names.search(&stop_flag, None);

        assert_eq!(bad_names.get_bad_names_files().len(), 1);
        assert_eq!(bad_names.get_bad_names_files()[0].new_name, "tst.txt");
    }
}
