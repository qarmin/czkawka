#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;
    use std::sync::Arc;
    use std::sync::atomic::AtomicBool;

    use crate::common::tool_data::CommonData;
    use crate::common::traits::Search;
    use crate::tools::bad_names::{BadNames, BadNamesParameters, CharsetFixMethod, NameFixerParams, NameIssues};
    use crate::tools::bad_names::core::{check_file_name, generate_fixed_name};

    #[test]
    fn test_uppercase_extension_detection() {
        let temp_dir = tempfile::tempdir().unwrap();
        let test_file = temp_dir.path().join("test.TXT");
        fs::write(&test_file, "test").unwrap();

        let params = BadNamesParameters::new(NameIssues {
            uppercase_extension: true,
            emoji_used: false,
            space_at_start_or_end: false,
            non_ascii_name: None,
            restricted_charset: None,
        });
        let mut bad_names = BadNames::new(params);
        bad_names.get_cd_mut().directories.set_included_paths(vec![temp_dir.path().to_path_buf()]);

        let stop_flag = Arc::new(AtomicBool::new(false));
        bad_names.search(&stop_flag, None);

        assert_eq!(bad_names.get_bad_names_files().len(), 1);
        assert!(bad_names.get_bad_names_files()[0].issues.uppercase_extension);
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
            non_ascii_name: None,
            restricted_charset: None,
        });
        let mut bad_names = BadNames::new(params);
        bad_names.get_cd_mut().directories.set_included_paths(vec![temp_dir.path().to_path_buf()]);

        let stop_flag = Arc::new(AtomicBool::new(false));
        bad_names.search(&stop_flag, None);

        assert_eq!(bad_names.get_bad_names_files().len(), 1);
        assert!(bad_names.get_bad_names_files()[0].issues.emoji_used);
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
            non_ascii_name: None,
            restricted_charset: None,
        });
        let mut bad_names = BadNames::new(params);
        bad_names.get_cd_mut().directories.set_included_paths(vec![temp_dir.path().to_path_buf()]);

        let stop_flag = Arc::new(AtomicBool::new(false));
        bad_names.search(&stop_flag, None);

        assert_eq!(bad_names.get_bad_names_files().len(), 1);
        assert!(bad_names.get_bad_names_files()[0].issues.space_at_start_or_end);
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
            non_ascii_name: None,
            restricted_charset: None,
        });
        let mut bad_names = BadNames::new(params);
        bad_names.get_cd_mut().directories.set_included_paths(vec![temp_dir.path().to_path_buf()]);

        let stop_flag = Arc::new(AtomicBool::new(false));
        bad_names.search(&stop_flag, None);

        assert_eq!(bad_names.get_bad_names_files().len(), 1);
        assert!(bad_names.get_bad_names_files()[0].issues.space_at_start_or_end);
    }

    #[test]
    fn test_non_ascii_detection() {
        let temp_dir = tempfile::tempdir().unwrap();
        let test_file = temp_dir.path().join("tëst.txt");
        fs::write(&test_file, "test").unwrap();

        let params = BadNamesParameters::new(NameIssues {
            uppercase_extension: false,
            emoji_used: false,
            space_at_start_or_end: false,
            non_ascii_name: Some(CharsetFixMethod::default()),
            restricted_charset: None,
        });
        let mut bad_names = BadNames::new(params);
        bad_names.get_cd_mut().directories.set_included_paths(vec![temp_dir.path().to_path_buf()]);

        let stop_flag = Arc::new(AtomicBool::new(false));
        bad_names.search(&stop_flag, None);

        assert_eq!(bad_names.get_bad_names_files().len(), 1);
        assert!(bad_names.get_bad_names_files()[0].issues.non_ascii_name.is_some());
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
            non_ascii_name: None,
            restricted_charset: Some(CharsetFixMethod::default()),
        });
        let mut bad_names = BadNames::new(params);
        bad_names.get_cd_mut().directories.set_included_paths(vec![temp_dir.path().to_path_buf()]);

        let stop_flag = Arc::new(AtomicBool::new(false));
        bad_names.search(&stop_flag, None);

        assert_eq!(bad_names.get_bad_names_files().len(), 1);
        assert!(bad_names.get_bad_names_files()[0].issues.restricted_charset.is_some());
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
        let issues = bad_names.get_bad_names_files()[0].issues;
        assert!(issues.uppercase_extension);
        assert!(issues.emoji_used);
        assert!(issues.space_at_start_or_end);
        assert!(issues.non_ascii_name.is_some());
    }

    // Unit tests for check_file_name and generate_fixed_name
    #[test]
    fn test_uppercase_extension_fix() {
        let check_params = NameIssues {
            uppercase_extension: true,
            ..NameIssues::none()
        };
        let fix_params = NameFixerParams {
            fix_uppercase_extension: true,
            ..NameFixerParams::default()
        };

        let mut errors = Vec::new();
        let test_cases = [
            ("test.TXT", "test.txt"),
            ("file.Jpg", "file.jpg"),
            ("document.PDF", "document.pdf"),
            ("image.PnG", "image.png"),
        ];

        for (input, expected_output) in test_cases {
            let path = Path::new(input);
            if let Some(_issues) = check_file_name(path, &check_params) {
                if let Some(new_name) = generate_fixed_name(path, &fix_params) {
                    if new_name != expected_output {
                        errors.push(format!("Input: '{}', Expected: '{}', Got: '{}'", input, expected_output, new_name));
                    }

                    let fixed_path = Path::new(&new_name);
                    if generate_fixed_name(fixed_path, &fix_params).is_some() {
                        errors.push(format!("Double fix should return None for: '{}'", new_name));
                    }
                } else {
                    errors.push(format!("Input: '{}' was not fixed", input));
                }
            } else {
                errors.push(format!("Input: '{}' was not detected as having issues", input));
            }
        }

        if !errors.is_empty() {
            panic!("Uppercase extension tests failed:\n{}", errors.join("\n"));
        }
    }

    #[test]
    fn test_non_ascii_fix_transliterate() {
        let check_params = NameIssues {
            non_ascii_name: Some(CharsetFixMethod::Transliterate),
            ..NameIssues::none()
        };
        let fix_params = NameFixerParams {
            fix_non_ascii: Some(CharsetFixMethod::Transliterate),
            ..NameFixerParams::default()
        };

        let mut errors = Vec::new();
        let test_cases = [
            ("tëst.txt", "test.txt"),
            ("café.pdf", "cafe.pdf"),
            ("Kraków.doc", "Krakow.doc"),
        ];

        for (input, expected_output) in test_cases {
            let path = Path::new(input);
            if let Some(_issues) = check_file_name(path, &check_params) {
                if let Some(new_name) = generate_fixed_name(path, &fix_params) {
                    if new_name != expected_output {
                        errors.push(format!("Input: '{}', Expected: '{}', Got: '{}'", input, expected_output, new_name));
                    }

                    let fixed_path = Path::new(&new_name);
                    if generate_fixed_name(fixed_path, &fix_params).is_some() {
                        errors.push(format!("Double fix should return None for: '{}'", new_name));
                    }
                } else {
                    errors.push(format!("Input: '{}' was not fixed", input));
                }
            } else {
                errors.push(format!("Input: '{}' was not detected as having issues", input));
            }
        }

        if !errors.is_empty() {
            panic!("Non-ASCII transliterate tests failed:\n{}", errors.join("\n"));
        }
    }

    #[test]
    fn test_non_ascii_fix_replace_underscore() {
        let check_params = NameIssues {
            non_ascii_name: Some(CharsetFixMethod::ReplaceWithUnderscore),
            ..NameIssues::none()
        };
        let fix_params = NameFixerParams {
            fix_non_ascii: Some(CharsetFixMethod::ReplaceWithUnderscore),
            ..NameFixerParams::default()
        };

        let mut errors = Vec::new();
        let test_cases = [
            ("tëst.txt", "t_st.txt"),
            ("café.pdf", "caf_.pdf"),
        ];

        for (input, expected_output) in test_cases {
            let path = Path::new(input);
            if let Some(_issues) = check_file_name(path, &check_params) {
                if let Some(new_name) = generate_fixed_name(path, &fix_params) {
                    if new_name != expected_output {
                        errors.push(format!("Input: '{}', Expected: '{}', Got: '{}'", input, expected_output, new_name));
                    }

                    let fixed_path = Path::new(&new_name);
                    if generate_fixed_name(fixed_path, &fix_params).is_some() {
                        errors.push(format!("Double fix should return None for: '{}'", new_name));
                    }
                } else {
                    errors.push(format!("Input: '{}' was not fixed", input));
                }
            } else {
                errors.push(format!("Input: '{}' was not detected as having issues", input));
            }
        }

        if !errors.is_empty() {
            panic!("Non-ASCII replace underscore tests failed:\n{}", errors.join("\n"));
        }
    }

    #[test]
    fn test_emoji_fix() {
        let check_params = NameIssues {
            emoji_used: true,
            ..NameIssues::none()
        };
        let fix_params = NameFixerParams {
            fix_emoji: true,
            ..NameFixerParams::default()
        };

        let mut errors = Vec::new();
        let test_cases = [
            ("test😀.txt", "test.txt"),
            ("file🎉🎊.doc", "file.doc"),
            ("image❤️.png", "image.png"),
        ];

        for (input, expected_output) in test_cases {
            let path = Path::new(input);
            if let Some(_issues) = check_file_name(path, &check_params) {
                if let Some(new_name) = generate_fixed_name(path, &fix_params) {
                    if new_name != expected_output {
                        errors.push(format!("Input: '{}', Expected: '{}', Got: '{}'", input, expected_output, new_name));
                    }

                    let fixed_path = Path::new(&new_name);
                    if generate_fixed_name(fixed_path, &fix_params).is_some() {
                        errors.push(format!("Double fix should return None for: '{}'", new_name));
                    }
                } else {
                    errors.push(format!("Input: '{}' was not fixed", input));
                }
            } else {
                errors.push(format!("Input: '{}' was not detected as having issues", input));
            }
        }

        if !errors.is_empty() {
            panic!("Emoji fix tests failed:\n{}", errors.join("\n"));
        }
    }

    #[test]
    fn test_space_at_start_end_fix() {
        let check_params = NameIssues {
            space_at_start_or_end: true,
            ..NameIssues::none()
        };
        let fix_params = NameFixerParams {
            fix_space_at_start_or_end: true,
            ..NameFixerParams::default()
        };

        let mut errors = Vec::new();
        let test_cases = [
            (" test.txt", "test.txt"),
            ("test .txt", "test.txt"),
            (" test .txt", "test.txt"),
            ("  test  .txt", "test.txt"),
            ("test. txt ", "test.txt"),
        ];

        for (input, expected_output) in test_cases {
            let path = Path::new(input);
            if let Some(_issues) = check_file_name(path, &check_params) {
                if let Some(new_name) = generate_fixed_name(path, &fix_params) {
                    if new_name != expected_output {
                        errors.push(format!("Input: '{}', Expected: '{}', Got: '{}'", input, expected_output, new_name));
                    }

                    let fixed_path = Path::new(&new_name);
                    if generate_fixed_name(fixed_path, &fix_params).is_some() {
                        errors.push(format!("Double fix should return None for: '{}'", new_name));
                    }
                } else {
                    errors.push(format!("Input: '{}' was not fixed", input));
                }
            } else {
                errors.push(format!("Input: '{}' was not detected as having issues", input));
            }
        }

        if !errors.is_empty() {
            panic!("Space at start/end tests failed:\n{}", errors.join("\n"));
        }
    }

    #[test]
    fn test_restricted_charset_fix_transliterate() {
        let check_params = NameIssues {
            restricted_charset: Some(CharsetFixMethod::Transliterate),
            ..NameIssues::none()
        };
        let fix_params = NameFixerParams {
            fix_restricted_charset: Some(CharsetFixMethod::Transliterate),
            ..NameFixerParams::default()
        };

        let mut errors = Vec::new();
        let test_cases = [
            ("test@file.txt", "test_file.txt"),
            ("my#doc.pdf", "my_doc.pdf"),
            ("file-name.doc", "file_name.doc"),
            ("tëst@file.txt", "test_file.txt"),
        ];

        for (input, expected_output) in test_cases {
            let path = Path::new(input);
            if let Some(_issues) = check_file_name(path, &check_params) {
                if let Some(new_name) = generate_fixed_name(path, &fix_params) {
                    if new_name != expected_output {
                        errors.push(format!("Input: '{}', Expected: '{}', Got: '{}'", input, expected_output, new_name));
                    }

                    let fixed_path = Path::new(&new_name);
                    if generate_fixed_name(fixed_path, &fix_params).is_some() {
                        errors.push(format!("Double fix should return None for: '{}'", new_name));
                    }
                } else {
                    errors.push(format!("Input: '{}' was not fixed", input));
                }
            } else {
                errors.push(format!("Input: '{}' was not detected as having issues", input));
            }
        }

        if !errors.is_empty() {
            panic!("Restricted charset transliterate tests failed:\n{}", errors.join("\n"));
        }
    }

    #[test]
    fn test_restricted_charset_fix_replace_underscore() {
        let check_params = NameIssues {
            restricted_charset: Some(CharsetFixMethod::ReplaceWithUnderscore),
            ..NameIssues::none()
        };
        let fix_params = NameFixerParams {
            fix_restricted_charset: Some(CharsetFixMethod::ReplaceWithUnderscore),
            ..NameFixerParams::default()
        };

        let mut errors = Vec::new();
        let test_cases = [
            ("test@file.txt", "test_file.txt"),
            ("my#doc.pdf", "my_doc.pdf"),
        ];

        for (input, expected_output) in test_cases {
            let path = Path::new(input);
            if let Some(_issues) = check_file_name(path, &check_params) {
                if let Some(new_name) = generate_fixed_name(path, &fix_params) {
                    if new_name != expected_output {
                        errors.push(format!("Input: '{}', Expected: '{}', Got: '{}'", input, expected_output, new_name));
                    }

                    let fixed_path = Path::new(&new_name);
                    if generate_fixed_name(fixed_path, &fix_params).is_some() {
                        errors.push(format!("Double fix should return None for: '{}'", new_name));
                    }
                } else {
                    errors.push(format!("Input: '{}' was not fixed", input));
                }
            } else {
                errors.push(format!("Input: '{}' was not detected as having issues", input));
            }
        }

        if !errors.is_empty() {
            panic!("Restricted charset replace underscore tests failed:\n{}", errors.join("\n"));
        }
    }

    #[test]
    fn test_combined_fixes() {
        let check_params = NameIssues::all();
        let fix_params = NameFixerParams {
            fix_uppercase_extension: true,
            fix_emoji: true,
            fix_space_at_start_or_end: true,
            fix_non_ascii: Some(CharsetFixMethod::Transliterate),
            fix_restricted_charset: Some(CharsetFixMethod::Transliterate),
        };

        let mut errors = Vec::new();
        let test_cases = [
            (" tëst😀 . TXT ", "test.txt"),
            ("file .JPG", "file.jpg"),
            ("  café☕  .Pdf  ", "cafe.pdf"),
            ("test@ëmoji😀.PNG", "test_emoji.png"),
        ];

        for (input, expected_output) in test_cases {
            let path = Path::new(input);
            if let Some(_issues) = check_file_name(path, &check_params) {
                if let Some(new_name) = generate_fixed_name(path, &fix_params) {
                    if new_name != expected_output {
                        errors.push(format!("Input: '{}', Expected: '{}', Got: '{}'", input, expected_output, new_name));
                    }

                    let fixed_path = Path::new(&new_name);
                    if generate_fixed_name(fixed_path, &fix_params).is_some() {
                        errors.push(format!("Double fix should return None for: '{}'", new_name));
                    }
                } else {
                    errors.push(format!("Input: '{}' was not fixed", input));
                }
            } else {
                errors.push(format!("Input: '{}' was not detected as having issues", input));
            }
        }

        if !errors.is_empty() {
            panic!("Combined fixes tests failed:\n{}", errors.join("\n"));
        }
    }

    #[test]
    fn test_double_validation_all_methods() {
        let mut errors = Vec::new();

        for method in [
            CharsetFixMethod::ReplaceWithUnderscore,
            CharsetFixMethod::ReplaceWithSpace,
            CharsetFixMethod::Delete,
            CharsetFixMethod::Transliterate,
        ] {
            let fix_params = NameFixerParams {
                fix_non_ascii: Some(method),
                ..NameFixerParams::default()
            };

            let path = Path::new("tëst.txt");
            if let Some(new_name) = generate_fixed_name(path, &fix_params) {
                let fixed_path = Path::new(&new_name);
                if generate_fixed_name(fixed_path, &fix_params).is_some() {
                    errors.push(format!("Double fix failed for non_ascii with method: {:?}", method));
                }
            }
        }

        for method in [
            CharsetFixMethod::ReplaceWithUnderscore,
            CharsetFixMethod::ReplaceWithSpace,
            CharsetFixMethod::Delete,
            CharsetFixMethod::Transliterate,
        ] {
            let fix_params = NameFixerParams {
                fix_restricted_charset: Some(method),
                ..NameFixerParams::default()
            };

            let path = Path::new("test@file.txt");
            if let Some(new_name) = generate_fixed_name(path, &fix_params) {
                let fixed_path = Path::new(&new_name);
                if generate_fixed_name(fixed_path, &fix_params).is_some() {
                    errors.push(format!("Double fix failed for restricted_charset with method: {:?}", method));
                }
            }
        }

        if !errors.is_empty() {
            panic!("Double validation tests failed:\n{}", errors.join("\n"));
        }
    }
}
