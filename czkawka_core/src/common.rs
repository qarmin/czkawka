use directories_next::ProjectDirs;
use image::{DynamicImage, ImageBuffer, Rgb};
use imagepipe::{ImageSource, Pipeline};
use std::ffi::OsString;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

/// Class for common functions used across other class/functions

pub const LOOP_DURATION: u32 = 200; //ms

pub struct Common();

pub fn open_cache_folder(cache_file_name: &str, save_to_cache: bool, use_json: bool, warnings: &mut Vec<String>) -> Option<((Option<File>, PathBuf), (Option<File>, PathBuf))> {
    if let Some(proj_dirs) = ProjectDirs::from("pl", "Qarmin", "Czkawka") {
        let cache_dir = PathBuf::from(proj_dirs.cache_dir());
        let cache_file = cache_dir.join(cache_file_name);
        let cache_file_json = cache_dir.join(cache_file_name.replace(".bin", ".json"));

        let mut file_handler_default = None;
        let mut file_handler_json = None;

        if save_to_cache {
            if cache_dir.exists() {
                if !cache_dir.is_dir() {
                    warnings.push(format!("Config dir {} is a file!", cache_dir.display()));
                    return None;
                }
            } else if let Err(e) = fs::create_dir_all(&cache_dir) {
                warnings.push(format!("Cannot create config dir {}, reason {}", cache_dir.display(), e));
                return None;
            }

            file_handler_default = Some(match OpenOptions::new().truncate(true).write(true).create(true).open(&cache_file) {
                Ok(t) => t,
                Err(e) => {
                    warnings.push(format!("Cannot create or open cache file {}, reason {}", cache_file.display(), e));
                    return None;
                }
            });
            if use_json {
                file_handler_json = Some(match OpenOptions::new().truncate(true).write(true).create(true).open(&cache_file_json) {
                    Ok(t) => t,
                    Err(e) => {
                        warnings.push(format!("Cannot create or open cache file {}, reason {}", cache_file_json.display(), e));
                        return None;
                    }
                });
            }
        } else {
            if let Ok(t) = OpenOptions::new().read(true).open(&cache_file) {
                file_handler_default = Some(t);
            } else {
                if use_json {
                    file_handler_json = Some(match OpenOptions::new().read(true).open(&cache_file_json) {
                        Ok(t) => t,
                        Err(_) => return None,
                    });
                } else {
                    // messages.push(format!("Cannot find or open cache file {}", cache_file.display())); // No error or warning
                    return None;
                }
            }
        };
        return Some(((file_handler_default, cache_file), (file_handler_json, cache_file_json)));
    }
    None
}

pub fn get_dynamic_image_from_raw_image(path: impl AsRef<Path> + std::fmt::Debug) -> Option<DynamicImage> {
    let file_handler = match OpenOptions::new().read(true).open(&path) {
        Ok(t) => t,
        Err(_e) => {
            return None;
        }
    };

    let mut reader = BufReader::new(file_handler);
    let raw = match rawloader::decode(&mut reader) {
        Ok(raw) => raw,
        Err(_e) => {
            return None;
        }
    };

    let width = raw.width;
    let height = raw.height;
    let source = ImageSource::Raw(raw);

    let mut pipeline = match Pipeline::new_from_source(source, width, height, true) {
        Ok(pipeline) => pipeline,
        Err(_e) => {
            return None;
        }
    };

    pipeline.run(None);
    let image = match pipeline.output_8bit(None) {
        Ok(image) => image,
        Err(_e) => {
            return None;
        }
    };

    let image = match ImageBuffer::<Rgb<u8>, Vec<u8>>::from_raw(image.width as u32, image.height as u32, image.data) {
        Some(image) => image,
        None => {
            return None;
        }
    };

    // println!("Properly hashed {:?}", path);
    Some(image::DynamicImage::ImageRgb8(image))
}

impl Common {
    /// Printing time which took between start and stop point and prints also function name
    #[allow(unused_variables)]
    pub fn print_time(start_time: SystemTime, end_time: SystemTime, function_name: String) {
        #[cfg(debug_assertions)]
        println!(
            "Execution of function \"{}\" took {:?}",
            function_name,
            end_time.duration_since(start_time).expect("Time cannot go reverse.")
        );
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
        if expression == "*" {
            return true;
        }

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
                let replaced = path.replace('/', "\\");
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
    use std::path::PathBuf;

    use crate::common::Common;

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
