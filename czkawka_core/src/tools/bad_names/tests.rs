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
            restricted_charset_allowed: None,
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
            restricted_charset_allowed: None,
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
            restricted_charset_allowed: None,
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
            restricted_charset_allowed: None,
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
            restricted_charset_allowed: None,
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
    fn test_restricted_charset_detection() {
        let temp_dir = tempfile::tempdir().unwrap();
        let test_file = temp_dir.path().join("test@file.txt");
        fs::write(&test_file, "test").unwrap();

        let params = BadNamesParameters::new(NameIssues {
            uppercase_extension: false,
            emoji_used: false,
            space_at_start_or_end: false,
            non_ascii_graphical: false,
            restricted_charset_allowed: Some(vec!['_', '-', ' ']),
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
            restricted_charset_allowed: None,
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
        assert_eq!(bad_names.get_bad_names_files()[0].new_name, "test.txt");
    }

    use std::path::Path;

    use crate::tools::bad_names::core::check_and_generate_new_name;

    #[test]
    fn test_uppercase_extension_unit() {
        let check_params = NameIssues {
            uppercase_extension: true,
            ..NameIssues::default()
        };

        let mut errors = Vec::new();
        let test_cases = [
            ("test.TXT", "test.txt"),
            ("file.Jpg", "file.jpg"),
            ("document.PDF", "document.pdf"),
            ("image.PnG", "image.png"),
            ("video.MP4", "video.mp4"),
            ("archive.ZIP", "archive.zip"),
            ("data.CSV", "data.csv"),
            ("presentation.PPTX", "presentation.pptx"),
            ("script.Py", "script.py"),
            ("code.Js", "code.js"),
            ("style.Css", "style.css"),
            ("page.Html", "page.html"),
            ("config.Json", "config.json"),
            ("readme.Md", "readme.md"),
            ("Makefile.Mk", "Makefile.mk"),
            ("abc.cde.TXT", "abc.cde.txt"),
            ("file.backup.PDF", "file.backup.pdf"),
            ("my.file.name.JPG", "my.file.name.jpg"),
            ("test.1.2.3.Zip", "test.1.2.3.zip"),
            ("document.v2.0.Doc", "document.v2.0.doc"),
        ];

        for (input, expected_output) in test_cases {
            let path = Path::new(input);
            if let Some(new_name) = check_and_generate_new_name(path, &check_params) {
                if new_name != expected_output {
                    errors.push(format!("Input: '{input}', Expected: '{expected_output}', Got: '{new_name}'"));
                }

                let fixed_path = Path::new(&new_name);
                if check_and_generate_new_name(fixed_path, &check_params).is_some() {
                    errors.push(format!("Double fix should return None for: '{new_name}'"));
                }
            } else {
                errors.push(format!("Input: '{input}' was not fixed"));
            }
        }

        assert!(errors.is_empty(), "Uppercase extension tests failed:\n{}", errors.join("\n"));
    }

    #[test]
    fn test_emoji_removal_unit() {
        let check_params = NameIssues {
            emoji_used: true,
            ..NameIssues::default()
        };

        let mut errors = Vec::new();
        let test_cases = [
            ("test😀.txt", "test.txt"),
            ("file🎉🎊.doc", "file.doc"),
            ("image❤.png", "image.png"),
            ("video🔥.mp4", "video.mp4"),
            ("doc👍.pdf", "doc.pdf"),
            ("report😊😊😊.xlsx", "report.xlsx"),
            ("photo🌟.jpg", "photo.jpg"),
            ("music🎵🎶.mp3", "music.mp3"),
            ("readme📝.md", "readme.md"),
            ("party🎈🎉🎊🎁.txt", "party.txt"),
            ("love💕💖💗💘.doc", "love.doc"),
            ("fire🔥🔥🔥.log", "fire.log"),
            ("star⭐.txt", "star.txt"),
            ("food🍕🍔🍟.jpg", "food.jpg"),
            ("weather☀🌧⛈.csv", "weather.csv"),
            ("test😀.backup.txt", "test.backup.txt"),
            ("my.file🎉.doc", "my.file.doc"),
            ("archive.v1.2🔥.zip", "archive.v1.2.zip"),
        ];

        for (input, expected_output) in test_cases {
            let path = Path::new(input);
            if let Some(new_name) = check_and_generate_new_name(path, &check_params) {
                if new_name != expected_output {
                    errors.push(format!("Input: '{input}', Expected: '{expected_output}', Got: '{new_name}'"));
                }

                let fixed_path = Path::new(&new_name);
                if check_and_generate_new_name(fixed_path, &check_params).is_some() {
                    errors.push(format!("Double fix should return None for: '{new_name}'"));
                }
            } else {
                errors.push(format!("Input: '{input}' was not fixed"));
            }
        }

        assert!(errors.is_empty(), "Emoji removal tests failed:\n{}", errors.join("\n"));
    }

    #[test]
    fn test_space_at_start_end_unit() {
        let check_params = NameIssues {
            space_at_start_or_end: true,
            ..NameIssues::default()
        };

        let mut errors = Vec::new();
        let test_cases = [
            (" test.txt", "test.txt"),
            ("test .txt", "test.txt"),
            (" test .txt", "test.txt"),
            ("  test  .txt", "test.txt"),
            ("test. txt ", "test.txt"),
            ("   file   .doc", "file.doc"),
            ("image .png", "image.png"),
            (" video.mp4", "video.mp4"),
            ("document .pdf", "document.pdf"),
            (" report .xlsx", "report.xlsx"),
            ("     data     .csv", "data.csv"),
            ("photo . jpg ", "photo.jpg"),
            (" music .mp3", "music.mp3"),
            ("readme . md ", "readme.md"),
            ("  archive  . zip ", "archive.zip"),
            (" abc.cde.txt", "abc.cde.txt"),
            ("abc.cde .txt", "abc.cde.txt"),
            (" my.file.name .doc", "my.file.name.doc"),
            ("  test.1.2  . pdf ", "test.1.2.pdf"),
        ];

        for (input, expected_output) in test_cases {
            let path = Path::new(input);
            if let Some(new_name) = check_and_generate_new_name(path, &check_params) {
                if new_name != expected_output {
                    errors.push(format!("Input: '{input}', Expected: '{expected_output}', Got: '{new_name}'"));
                }

                let fixed_path = Path::new(&new_name);
                if check_and_generate_new_name(fixed_path, &check_params).is_some() {
                    errors.push(format!("Double fix should return None for: '{new_name}'"));
                }
            } else {
                errors.push(format!("Input: '{input}' was not fixed"));
            }
        }

        assert!(errors.is_empty(), "Space at start/end tests failed:\n{}", errors.join("\n"));
    }

    #[test]
    fn test_non_ascii_graphical_unit() {
        let check_params = NameIssues {
            non_ascii_graphical: true,
            ..NameIssues::default()
        };

        let mut errors = Vec::new();
        let test_cases = [
            ("tëst.txt", "test.txt"),
            ("café.pdf", "cafe.pdf"),
            ("Kraków.doc", "Krakow.doc"),
            ("Łódź.txt", "Lodz.txt"),
            ("naïve.doc", "naive.doc"),
            ("résumé.pdf", "resume.pdf"),
            ("São Paulo.txt", "Sao Paulo.txt"),
            ("Zürich.doc", "Zurich.doc"),
            ("Москва.txt", "Moskva.txt"),
            ("日本.txt", "Ri Ben.txt"),
            ("über.pdf", "uber.pdf"),
            ("señor.txt", "senor.txt"),
            ("Ærø.doc", "AEro.doc"),
            ("niño.txt", "nino.txt"),
            ("Björk.mp3", "Bjork.mp3"),
            ("François.doc", "Francois.doc"),
            ("Ñoño.txt", "Nono.txt"),
            ("Østergård.pdf", "Ostergard.pdf"),
            ("Łukasz.txt", "Lukasz.txt"),
            ("Müller.doc", "Muller.doc"),
            ("pièces", "pieces"),
            // (`•`->`*`, `¿`->`?`, unmapped->`[?]`); these must be stripped, not kept.
            ("a•b.txt", "ab.txt"),
            ("All Posts • Instagram(1).txt", "All Posts  Instagram(1).txt"),
            ("¿que.txt", "que.txt"),
            ("x\u{0378}y.txt", "xy.txt"),
        ];

        for (input, expected_output) in test_cases {
            let path = Path::new(input);
            if let Some(new_name) = check_and_generate_new_name(path, &check_params) {
                if new_name != expected_output {
                    errors.push(format!("Input: '{input}', Expected: '{expected_output}', Got: '{new_name}'"));
                }

                let fixed_path = Path::new(&new_name);
                if check_and_generate_new_name(fixed_path, &check_params).is_some() {
                    errors.push(format!("Double fix should return None for: '{new_name}'"));
                }
            } else {
                errors.push(format!("Input: '{input}' was not fixed"));
            }
        }

        assert!(errors.is_empty(), "Non-ASCII graphical tests failed:\n{}", errors.join("\n"));
    }

    #[test]
    fn test_restricted_charset_unit() {
        let check_params = NameIssues {
            restricted_charset_allowed: Some(vec!['_', '-', ' ']),
            ..NameIssues::default()
        };

        let mut errors = Vec::new();
        let test_cases = [
            ("test@file.txt", "testfile.txt"),
            ("my#doc.pdf", "mydoc.pdf"),
            ("file$name.doc", "filename.doc"),
            ("data%set.csv", "dataset.csv"),
            ("script&code.js", "scriptcode.js"),
            ("image*pic.png", "imagepic.png"),
            ("video(1).mp4", "video1.mp4"),
            ("photo[2].jpg", "photo2.jpg"),
            ("doc{test}.pdf", "doctest.pdf"),
            ("file|name.txt", "filename.txt"),
            ("test:file.doc", "testfile.doc"),
            ("name;value.csv", "namevalue.csv"),
            ("file'name.txt", "filename.txt"),
            ("test\"quote.doc", "testquote.doc"),
            ("data<less.xml", "dataless.xml"),
            ("file>more.txt", "filemore.txt"),
            ("question?.log", "question.log"),
            ("wild*.txt", "wild.txt"),
            ("comma,.csv", "comma.csv"),
        ];

        for (input, expected_output) in test_cases {
            let path = Path::new(input);
            if let Some(new_name) = check_and_generate_new_name(path, &check_params) {
                if new_name != expected_output {
                    errors.push(format!("Input: '{input}', Expected: '{expected_output}', Got: '{new_name}'"));
                }

                let fixed_path = Path::new(&new_name);
                if check_and_generate_new_name(fixed_path, &check_params).is_some() {
                    errors.push(format!("Double fix should return None for: '{new_name}'"));
                }
            } else {
                errors.push(format!("Input: '{input}' was not fixed"));
            }
        }

        assert!(errors.is_empty(), "Restricted charset tests failed:\n{}", errors.join("\n"));
    }

    #[test]
    fn test_duplicated_non_alphanumeric_unit() {
        let check_params = NameIssues {
            remove_duplicated_non_alphanumeric: true,
            ..NameIssues::default()
        };

        let mut errors = Vec::new();
        let test_cases = [
            ("test__file.txt", "test_file.txt"),
            ("my--doc.pdf", "my-doc.pdf"),
            ("file  name.doc", "file name.doc"),
            ("data...set.csv", "data.set.csv"),
            ("script___code.js", "script_code.js"),
            ("image---pic.png", "image-pic.png"),
            ("test____file----name.txt", "test_file-name.txt"),
            ("multiple   spaces.doc", "multiple spaces.doc"),
            ("under______score.log", "under_score.log"),
            ("dash-------line.txt", "dash-line.txt"),
            ("mixed__--__test.doc", "mixed_-_test.doc"),
            ("file,,,,name.csv", "file,name.csv"),
            ("test;;;;code.txt", "test;code.txt"),
            ("data::::value.xml", "data:value.xml"),
            ("triple___---...test.txt", "triple_-.test.txt"),
            ("many        spaces.doc", "many spaces.doc"),
            ("dots......dots.txt", "dots.dots.txt"),
            ("under_score.txt", "under_score.txt"),
            ("normal-file.txt", "normal-file.txt"),
        ];

        for (input, expected_output) in test_cases {
            let path = Path::new(input);
            let result = check_and_generate_new_name(path, &check_params);

            if input == expected_output {
                // No change expected
                if let Some(result) = result {
                    errors.push(format!("Input: '{input}' should not be modified but got: '{result}'"));
                }
            } else {
                // Change expected
                if let Some(new_name) = result {
                    if new_name != expected_output {
                        errors.push(format!("Input: '{input}', Expected: '{expected_output}', Got: '{new_name}'"));
                    }

                    let fixed_path = Path::new(&new_name);
                    if check_and_generate_new_name(fixed_path, &check_params).is_some() {
                        errors.push(format!("Double fix should return None for: '{new_name}'"));
                    }
                } else {
                    errors.push(format!("Input: '{input}' was not fixed"));
                }
            }
        }

        assert!(errors.is_empty(), "Duplicated non-alphanumeric tests failed:\n{}", errors.join("\n"));
    }

    #[test]
    fn test_combined_all_issues_unit() {
        let check_params = NameIssues {
            uppercase_extension: true,
            emoji_used: true,
            space_at_start_or_end: true,
            non_ascii_graphical: true,
            restricted_charset_allowed: Some(vec!['_', '-', ' ']),
            remove_duplicated_non_alphanumeric: true,
        };

        let mut errors = Vec::new();
        let test_cases = [
            (" tëst😀 .TXT ", "test.txt"),
            ("  café☕  .Pdf  ", "cafe.pdf"),
            (" über@file😊 .Txt ", "uberfile.txt"),
            ("test__😀__file.JPG", "test_file.jpg"),
            (" Kraków🎉 .Doc ", "Krakow.doc"),
            ("  résumé##  .PDF  ", "resume.pdf"),
            ("São Paulo  .TXT", "Sao Paulo.txt"),
            (" file___name😀😀.PNG ", "file_name.png"),
            ("test  @@  emoji🎉.MP4", "test emoji.mp4"),
            (" Łódź---file .CSV ", "Lodz-file.csv"),
            ("über__müller😊.XLSX", "uber_muller.xlsx"),
            (" data___set🔥 . JSON ", "data_set.json"),
            ("test  ##  ëmoji😀.Doc", "test emoji.doc"),
            (" François___Müller .PDF ", "Francois_Muller.pdf"),
            ("multi___issue___test😀😀 .TXT ", "multi_issue_test.txt"),
        ];

        for (input, expected_output) in test_cases {
            let path = Path::new(input);
            if let Some(new_name) = check_and_generate_new_name(path, &check_params) {
                if new_name != expected_output {
                    errors.push(format!("Input: '{input}', Expected: '{expected_output}', Got: '{new_name}'"));
                }

                let fixed_path = Path::new(&new_name);
                if check_and_generate_new_name(fixed_path, &check_params).is_some() {
                    errors.push(format!("Double fix should return None for: '{new_name}'"));
                }
            } else {
                errors.push(format!("Input: '{input}' was not fixed"));
            }
        }

        assert!(errors.is_empty(), "Combined all issues tests failed:\n{}", errors.join("\n"));
    }

    #[test]
    fn test_no_issues_no_changes() {
        let check_params = NameIssues::all();

        let mut errors = Vec::new();
        let test_cases = [
            "normal_file.txt",
            "test-file.doc",
            "MyDocument.pdf",
            "data_2024.csv",
            "image-001.jpg",
            "video_final.mp4",
            "report-2024-01.xlsx",
            "README.md",
            "config.json",
            "script.py",
        ];

        for input in test_cases {
            let path = Path::new(input);
            if let Some(new_name) = check_and_generate_new_name(path, &check_params) {
                errors.push(format!("Input: '{input}' should not be changed but got: '{new_name}'"));
            }
        }

        assert!(errors.is_empty(), "No issues no changes tests failed:\n{}", errors.join("\n"));
    }

    #[test]
    fn test_edge_cases_unit() {
        let check_params = NameIssues::all();

        let mut errors = Vec::new();
        let test_cases = [
            ("😀.txt", ".txt"),
            ("   .TXT", ".txt"),
            ("😀😀😀.txt", ".txt"),
            ("___", "_"),
            ("---", "-"),
            ("...", "."),
            (" 😀 .TXT ", ".txt"),
            ("test.", "test"),
            (".test", ".test"),
        ];

        for (input, expected_output) in test_cases {
            let path = Path::new(input);
            let result = check_and_generate_new_name(path, &check_params);

            if input == expected_output {
                if let Some(new_name) = result {
                    errors.push(format!("Edge case input: '{input}' should not be modified but got: '{new_name}'"));
                }
            } else {
                if let Some(new_name) = result {
                    if new_name != expected_output {
                        errors.push(format!("Edge case input: '{input}', Expected: '{expected_output}', Got: '{new_name}'"));
                    }
                } else {
                    errors.push(format!("Edge case input: '{input}' was not fixed"));
                }
            }
        }

        assert!(errors.is_empty(), "Edge cases tests failed:\n{}", errors.join("\n"));
    }
}
