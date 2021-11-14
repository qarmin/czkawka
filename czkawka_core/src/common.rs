use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

/// Class for common functions used across other class/functions

pub struct Common();
impl Common {
    /// Printing time which took between start and stop point and prints also function name
    #[allow(unused_variables)]
    pub fn print_time(start_time: SystemTime, end_time: SystemTime, function_name: String) {
        #[cfg(debug_assertions)]
        println!("Execution of function \"{}\" took {:?}", function_name, end_time.duration_since(start_time).expect("Time cannot go reverse."));
    }

    pub fn delete_multiple_entries(entries: &[String]) -> Vec<String> {
        let mut path: &Path;
        let mut warnings: Vec<String> = Vec::new();
        for entry in entries {
            path = Path::new(entry);
            if path.is_dir() {
                if let Err(e) = fs::remove_dir_all(&entry) {
                    warnings.push(format!("Failed to remove folder {}, reason {}", entry, e));
                }
            } else if let Err(e) = fs::remove_file(&entry) {
                warnings.push(format!("Failed to remove file {}, reason {}", entry, e));
            }
        }
        warnings
    }
    pub fn delete_one_entry(entry: &str) -> String {
        let path: &Path = Path::new(entry);
        let mut warning: String = String::from("");
        if path.is_dir() {
            if let Err(e) = fs::remove_dir_all(&entry) {
                warning = format!("Failed to remove folder {}, reason {}", entry, e)
            }
        } else if let Err(e) = fs::remove_file(&entry) {
            warning = format!("Failed to remove file {}, reason {}", entry, e)
        }
        warning
    }

    /// Function to check if directory match expression
    pub fn regex_check(expression: &str, directory: impl AsRef<Path>) -> bool {
        // if !expression.contains('*') {
        //     #[cfg(debug_assertions)]
        //     {
        //         println!("Invalid expression Warning: Expression should have *,");
        //     }
        //     //return false;
        // }

        let temp_splits: Vec<&str> = expression.split('*').collect();
        let mut splits: Vec<&str> = Vec::new();
        for i in temp_splits {
            if !i.is_empty() {
                splits.push(i);
            }
        }
        if splits.is_empty() {
            return false;
        }

        // Get rid of non unicode characters
        let directory = directory.as_ref().to_string_lossy();

        // Early checking if directory contains all parts needed by expression
        for split in &splits {
            if !directory.contains(split) {
                return false;
            }
        }

        let mut position_of_splits: Vec<usize> = Vec::new();

        // `git*` shouldn't be true for `/gitsfafasfs`
        if !expression.starts_with('*') && directory.find(&splits[0]).unwrap() > 0 {
            return false;
        }
        // `*home` shouldn't be true for `/homeowner`
        if !expression.ends_with('*') && !directory.ends_with(splits.last().unwrap()) {
            return false;
        }

        // At the end we check if parts between * are correctly positioned
        position_of_splits.push(directory.find(&splits[0]).unwrap());
        let mut current_index: usize;
        let mut found_index: usize;
        for i in splits[1..].iter().enumerate() {
            current_index = *position_of_splits.get(i.0).unwrap() + i.1.len();
            found_index = match directory[current_index..].find(i.1) {
                Some(t) => t,
                None => return false,
            };
            position_of_splits.push(found_index + current_index);
        }
        true
    }

    pub fn normalize_windows_path(path_to_change: impl AsRef<Path>) -> PathBuf {
        let path = path_to_change.as_ref();

        // Don't do anything, because network path may be case intensive
        if path.to_string_lossy().starts_with('\\') {
            return path.to_path_buf();
        }

        match path.to_str() {
            Some(path) if path.is_char_boundary(1) => {
                let replaced = path.replace("/", "\\");
                let mut new_path = OsString::new();
                if replaced[1..].starts_with(':') {
                    new_path.push(replaced[..1].to_ascii_uppercase());
                    new_path.push(replaced[1..].to_ascii_lowercase());
                } else {
                    new_path.push(replaced.to_ascii_lowercase());
                }
                PathBuf::from(new_path)
            }
            _ => path.to_path_buf(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::common::Common;
    use std::path::PathBuf;

    #[test]
    fn test_regex() {
        assert!(Common::regex_check("*home*", "/home/rafal"));
        assert!(Common::regex_check("*home", "/home"));
        assert!(Common::regex_check("*home/", "/home/"));
        assert!(Common::regex_check("*home/*", "/home/"));
        assert!(Common::regex_check("*.git*", "/home/.git"));
        assert!(Common::regex_check("*/home/rafal*rafal*rafal*rafal*", "/home/rafal/rafalrafalrafal"));
        assert!(Common::regex_check("AAA", "AAA"));
        assert!(Common::regex_check("AAA*", "AAABDGG/QQPW*"));
        assert!(!Common::regex_check("*home", "/home/"));
        assert!(!Common::regex_check("*home", "/homefasfasfasfasf/"));
        assert!(!Common::regex_check("*home", "/homefasfasfasfasf"));
        assert!(!Common::regex_check("rafal*afal*fal", "rafal"));
        assert!(!Common::regex_check("rafal*a", "rafal"));
        assert!(!Common::regex_check("AAAAAAAA****", "/AAAAAAAAAAAAAAAAA"));
        assert!(!Common::regex_check("*.git/*", "/home/.git"));
        assert!(!Common::regex_check("*home/*koc", "/koc/home/"));
        assert!(!Common::regex_check("*home/", "/home"));
        assert!(!Common::regex_check("*TTT", "/GGG"));

        #[cfg(target_family = "windows")]
        {
            assert!(Common::regex_check("*\\home", "C:\\home"));
            assert!(Common::regex_check("*/home", "C:\\home"));
        }
    }
    #[test]
    fn test_windows_path() {
        assert_eq!(PathBuf::from("C:\\path.txt"), Common::normalize_windows_path("c:/PATH.tXt"));
        assert_eq!(PathBuf::from("H:\\reka\\weza\\roman.txt"), Common::normalize_windows_path("h:/RekA/Weza\\roMan.Txt"));
        assert_eq!(PathBuf::from("T:\\a"), Common::normalize_windows_path("T:\\A"));
        assert_eq!(PathBuf::from("\\\\aBBa"), Common::normalize_windows_path("\\\\aBBa"));
        assert_eq!(PathBuf::from("a"), Common::normalize_windows_path("a"));
        assert_eq!(PathBuf::from(""), Common::normalize_windows_path(""));
    }
}
