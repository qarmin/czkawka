use std::fs;
use std::path::Path;
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
                match fs::remove_dir_all(&entry) {
                    Ok(_) => (),
                    Err(_) => warnings.push("Failed to remove folder ".to_owned() + entry.as_str()),
                }
            } else {
                match fs::remove_file(&entry) {
                    Ok(_) => (),
                    Err(_) => warnings.push("Failed to remove file ".to_owned() + entry.as_str()),
                }
            }
        }
        warnings
    }
    pub fn delete_one_entry(entry: &str) -> String {
        let path: &Path = Path::new(entry);
        let mut warning: String = String::from("");
        if path.is_dir() {
            match fs::remove_dir_all(&entry) {
                Ok(_) => (),
                Err(_) => warning = "Failed to remove folder ".to_owned() + entry,
            }
        } else {
            match fs::remove_file(&entry) {
                Ok(_) => (),
                Err(_) => warning = "Failed to remove file ".to_owned() + entry,
            }
        }
        warning
    }

    /// Function to check if directory match expression
    pub fn regex_check(expression: &str, directory: &str) -> bool {
        if !expression.contains('*') {
            #[cfg(debug_assertions)]
            {
                println!("Invalid expression ERROR: Expression should have *");
            }
            return false;
        }

        let temp_splits: Vec<&str> = expression.split('*').collect();
        let mut splits: Vec<&str> = Vec::new();
        for i in temp_splits {
            if i != "" {
                splits.push(i);
            }
        }
        if splits.is_empty() {
            return false;
        }

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
    #[allow(clippy::ptr_arg)]
    pub fn prettier_windows_path(path_to_change: &String) -> String {
        path_to_change[..1].to_uppercase() + path_to_change[1..].to_lowercase().replace("\\", "/").as_str()
    }
}

#[cfg(test)]
mod test {
    use crate::common::Common;

    #[test]
    fn test_regex() {
        assert!(Common::regex_check("*home*", "/home/rafal"));
        assert!(Common::regex_check("*home", "/home"));
        assert!(Common::regex_check("*home/", "/home/"));
        assert!(Common::regex_check("*home/*", "/home/"));
        assert!(Common::regex_check("*.git*", "/home/.git"));
        assert!(Common::regex_check("*/home/rafal*rafal*rafal*rafal*", "/home/rafal/rafalrafalrafal"));
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
        assert!(!Common::regex_check("AAA", "AAA"));
    }
    #[test]
    fn test_windows_path() {
        assert_eq!("C:/path.txt", Common::prettier_windows_path(&"c:/PATH.tXt".to_string()));
        assert_eq!("H:/reka/weza/roman.txt", Common::prettier_windows_path(&"h:/RekA/Weza\\roMan.Txt".to_string()));
        assert_eq!("T:/a", Common::prettier_windows_path(&"T:\\A".to_string()));
    }
}
