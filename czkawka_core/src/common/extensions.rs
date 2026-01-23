use std::ffi::OsStr;

use indexmap::IndexSet;

use crate::common::consts::{AUDIO_FILES_EXTENSIONS, IMAGE_RS_EXTENSIONS, TEXT_FILES_EXTENSIONS, VIDEO_FILES_EXTENSIONS};
use crate::flc;
use crate::helpers::messages::Messages;

#[derive(Debug, Clone, Default)]
pub struct Extensions {
    allowed_extensions_hashset: IndexSet<String>,
    excluded_extensions_hashset: IndexSet<String>,
}

impl Extensions {
    pub fn new() -> Self {
        Default::default()
    }

    pub(crate) fn filter_extensions(file_extensions: Vec<String>) -> (IndexSet<String>, Messages) {
        let mut messages = Messages::new();

        let extensions_hashset: IndexSet<String> = file_extensions
            .into_iter()
            .flat_map(|e| match e.trim().trim_start_matches(".").to_lowercase().as_str() {
                "image" => IMAGE_RS_EXTENSIONS.iter().map(|s| s.to_string()).collect(),
                "video" => VIDEO_FILES_EXTENSIONS.iter().map(|s| s.to_string()).collect(),
                "music" => AUDIO_FILES_EXTENSIONS.iter().map(|s| s.to_string()).collect(),
                "text" => TEXT_FILES_EXTENSIONS.iter().map(|s| s.to_string()).collect(),
                _ => vec![e],
            })
            .filter_map(|extension| {
                let e = extension.trim().trim_start_matches(".").to_lowercase();
                if e.is_empty() {
                    return None;
                }

                if e.contains(' ') {
                    messages
                        .warnings
                        .push(format!("{extension} is not a valid extension because it contains empty space inside"));
                    return None;
                }
                if e.contains('.') {
                    messages.warnings.push(format!("{extension} is not a valid extension because it contains dot inside"));
                    return None;
                }
                Some(e)
            })
            .collect();

        (extensions_hashset, messages)
    }

    pub(crate) fn set_allowed_extensions(&mut self, allowed_extensions: Vec<String>) -> Messages {
        let (extensions, messages) = Self::filter_extensions(allowed_extensions);

        self.allowed_extensions_hashset = extensions;
        messages
    }

    pub(crate) fn set_excluded_extensions(&mut self, excluded_extensions: Vec<String>) -> Messages {
        let (extensions, messages) = Self::filter_extensions(excluded_extensions);

        self.excluded_extensions_hashset = extensions;
        messages
    }

    #[expect(clippy::string_slice)] // Valid, because we address go to dot, which is known ascii character
    pub(crate) fn check_if_entry_have_valid_extension(&self, file_name: &OsStr) -> bool {
        if self.allowed_extensions_hashset.is_empty() && self.excluded_extensions_hashset.is_empty() {
            return true;
        }

        // Using entry_data.path().extension() is a lot of slower, even 5 times
        let Some(file_name_str) = file_name.to_str() else { return false };
        let Some(extension_idx) = file_name_str.rfind('.') else { return false };
        let extension = &file_name_str[extension_idx + 1..];

        if !self.allowed_extensions_hashset.is_empty() {
            if extension.chars().all(|c| c.is_ascii_lowercase()) {
                self.allowed_extensions_hashset.contains(extension)
            } else {
                self.allowed_extensions_hashset.contains(&extension.to_lowercase())
            }
        } else if extension.chars().all(|c| c.is_ascii_lowercase()) {
            !self.excluded_extensions_hashset.contains(extension)
        } else {
            !self.excluded_extensions_hashset.contains(&extension.to_lowercase())
        }
    }

    // E.g. when using similar videos, user can provide extensions like "mp4,flv", but if user provide "mp4,jpg" then
    // it will be only "mp4" because "jpg" is not valid extension for videos
    fn intersection_allowed_extensions(&mut self, file_extensions: &[&str]) {
        self.allowed_extensions_hashset.retain(|ext| file_extensions.contains(&ext.as_str()));
    }

    // Tool extensions may be set by the tool itself, e.g. similar images may only use image extensions
    pub(crate) fn set_and_validate_extensions(&mut self, tool_extensions: Option<&[&str]>) -> Result<(), String> {
        let user_set_any_allowed_extensions = !self.allowed_extensions_hashset.is_empty();
        let tool_have_any_extensions = tool_extensions.is_some();

        // If user not set any extensions and tool not have any allowed extension, it is fine
        if !user_set_any_allowed_extensions && !tool_have_any_extensions {
            return Ok(());
        }

        if let Some(tool_extensions) = tool_extensions {
            // If there is no selected allowed extensions, that means that are all allowed
            // If there are some allowed extensions, we need to do intersection with tool extensions
            if user_set_any_allowed_extensions {
                self.intersection_allowed_extensions(tool_extensions);
            } else {
                self.allowed_extensions_hashset = tool_extensions.iter().map(|ext| ext.trim_start_matches('.').to_string()).collect();
            }
        }

        let both_extensions = self.allowed_extensions_hashset.intersection(&self.excluded_extensions_hashset).cloned().collect::<Vec<_>>();
        self.allowed_extensions_hashset.retain(|ext| !both_extensions.contains(ext));
        self.excluded_extensions_hashset.retain(|ext| !both_extensions.contains(ext));

        if self.allowed_extensions_hashset.is_empty() {
            if let Some(tool_extensions) = tool_extensions {
                Err(flc!("core_needs_allowed_extensions_limited_by_tool", extensions = tool_extensions.join(", ")))
            } else {
                Err(flc!("core_needs_allowed_extensions"))
            }
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_filter_extensions_basic_and_replacements() {
        // Empty string
        let (exts, msgs) = Extensions::filter_extensions(vec![]);
        assert!(exts.is_empty());
        assert!(msgs.messages.is_empty() && msgs.warnings.is_empty() && msgs.errors.is_empty());

        // Basic extensions
        let (exts, msgs) = Extensions::filter_extensions(vec!["jpg".to_string(), "png".to_string(), "gif".to_string()]);
        assert_eq!(exts.len(), 3);
        assert!(exts.contains("jpg") && exts.contains("png") && exts.contains("gif"));
        assert!(msgs.warnings.is_empty());

        // With dots
        let (exts, _) = Extensions::filter_extensions(vec![".jpg".to_string(), ".png".to_string()]);
        assert_eq!(exts.len(), 2);
        assert!(exts.contains("jpg") && exts.contains("png"));

        // IMAGE replacement
        let (exts, _) = Extensions::filter_extensions(vec!["IMAGE".to_string()]);
        assert!(exts.contains("jpg") && exts.contains("png") && exts.contains("bmp"));

        // VIDEO replacement
        let (exts, _) = Extensions::filter_extensions(vec!["VIDEO".to_string()]);
        assert!(exts.contains("mp4") && exts.contains("mkv") && exts.contains("avi"));

        // Invalid extensions with dot inside
        let (exts, msgs) = Extensions::filter_extensions(vec!["jpg".to_string(), "test.bad".to_string(), "png".to_string()]);
        assert_eq!(exts.len(), 2);
        assert!(!exts.contains("test.bad"));
        assert!(msgs.warnings.iter().any(|w| w.contains("test.bad")));

        // Invalid extensions with space
        let (exts, msgs) = Extensions::filter_extensions(vec!["jpg".to_string(), "bad ext".to_string(), "png".to_string()]);
        assert!(!exts.contains("bad ext"));
        assert!(msgs.warnings.iter().any(|w| w.contains("bad ext")));
    }

    #[test]
    fn test_check_if_entry_have_valid_extension() {
        let temp_dir = tempfile::tempdir().unwrap();
        let file_jpg = temp_dir.path().join("test.jpg");
        let file_png = temp_dir.path().join("test.PNG");
        let file_gif = temp_dir.path().join("test.gif");
        let file_txt = temp_dir.path().join("test.txt");
        let file_no_ext = temp_dir.path().join("noext");

        fs::write(&file_jpg, "test").unwrap();
        fs::write(&file_png, "test").unwrap();
        fs::write(&file_gif, "test").unwrap();
        fs::write(&file_txt, "test").unwrap();
        fs::write(&file_no_ext, "test").unwrap();

        // No extensions set - all should pass
        let ext = Extensions::new();
        assert!(
            ext.check_if_entry_have_valid_extension(
                &fs::read_dir(&temp_dir)
                    .unwrap()
                    .find(|e| e.as_ref().unwrap().file_name() == "test.jpg")
                    .unwrap()
                    .unwrap()
                    .file_name()
            )
        );

        // Allowed extensions
        let mut ext = Extensions::new();
        ext.set_allowed_extensions(vec!["jpg".to_string(), "png".to_string()]);
        let entries: Vec<_> = fs::read_dir(&temp_dir).unwrap().map(|e| e.unwrap()).collect();
        assert!(ext.check_if_entry_have_valid_extension(&entries.iter().find(|e| e.file_name() == "test.jpg").unwrap().file_name()));
        assert!(ext.check_if_entry_have_valid_extension(&entries.iter().find(|e| e.file_name() == "test.PNG").unwrap().file_name())); // case insensitive
        assert!(!ext.check_if_entry_have_valid_extension(&entries.iter().find(|e| e.file_name() == "test.gif").unwrap().file_name()));
        assert!(!ext.check_if_entry_have_valid_extension(&entries.iter().find(|e| e.file_name() == "noext").unwrap().file_name()));

        // Excluded extensions
        let mut ext = Extensions::new();
        ext.set_excluded_extensions(vec!["txt".to_string()]);
        assert!(ext.check_if_entry_have_valid_extension(&entries.iter().find(|e| e.file_name() == "test.jpg").unwrap().file_name()));
        assert!(!ext.check_if_entry_have_valid_extension(&entries.iter().find(|e| e.file_name() == "test.txt").unwrap().file_name()));
    }
}
