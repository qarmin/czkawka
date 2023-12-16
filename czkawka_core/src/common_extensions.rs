use crate::common_messages::Messages;
use std::collections::HashSet;
use std::fs::DirEntry;

#[derive(Debug, Clone, Default)]
pub struct Extensions {
    file_extensions_hashset: HashSet<String>,
}

impl Extensions {
    pub fn new() -> Self {
        Default::default()
    }
    /// List of allowed extensions, only files with this extensions will be checking if are duplicates
    /// After, extensions cannot contains any dot, commas etc.
    pub fn set_allowed_extensions(&mut self, mut allowed_extensions: String) -> Messages {
        let mut messages = Messages::new();

        if allowed_extensions.trim().is_empty() {
            return messages;
        }
        allowed_extensions = allowed_extensions.replace("IMAGE", "jpg,kra,gif,png,bmp,tiff,hdr,svg");
        allowed_extensions = allowed_extensions.replace("VIDEO", "mp4,flv,mkv,webm,vob,ogv,gifv,avi,mov,wmv,mpg,m4v,m4p,mpeg,3gp");
        allowed_extensions = allowed_extensions.replace("MUSIC", "mp3,flac,ogg,tta,wma,webm");
        allowed_extensions = allowed_extensions.replace("TEXT", "txt,doc,docx,odt,rtf");

        let extensions: Vec<String> = allowed_extensions.split(',').map(str::trim).map(String::from).collect();
        for mut extension in extensions {
            if extension.is_empty() || extension.replace(['.', ' '], "").trim().is_empty() {
                continue;
            }

            if extension.starts_with('.') {
                extension = extension[1..].to_string();
            }

            if extension.contains('.') {
                messages.warnings.push(format!("{extension} is not valid extension because contains dot inside"));
                continue;
            }

            if extension.contains(' ') {
                messages.warnings.push(format!("{extension} is not valid extension because contains empty space inside"));
                continue;
            }

            self.file_extensions_hashset.insert(extension);
        }

        if self.file_extensions_hashset.is_empty() {
            messages
                .messages
                .push("No valid extensions were provided, so allowing all extensions by default.".to_string());
        }
        messages
    }

    pub fn matches_filename(&self, file_name: &str) -> bool {
        // assert_eq!(file_name, file_name.to_lowercase());
        if !self.file_extensions_hashset.is_empty() && !self.file_extensions_hashset.iter().any(|e| file_name.ends_with(e)) {
            return false;
        }
        true
    }
    pub fn check_if_entry_ends_with_extension(&self, entry_data: &DirEntry) -> bool {
        if self.file_extensions_hashset.is_empty() {
            return true;
        }

        let file_name = entry_data.file_name();
        let Some(file_name_str) = file_name.to_str() else { return false };
        let Some(extension_idx) = file_name_str.rfind('.') else { return false };
        let extension = &file_name_str[extension_idx + 1..];

        if extension.chars().all(|c| c.is_ascii_lowercase()) {
            self.file_extensions_hashset.contains(extension)
        } else {
            self.file_extensions_hashset.contains(&extension.to_lowercase())
        }
    }

    pub fn set_any_extensions(&self) -> bool {
        !self.file_extensions_hashset.is_empty()
    }

    fn extend_allowed_extensions(&mut self, file_extensions: &[&str]) {
        for extension in file_extensions {
            let extension_without_dot = extension.trim_start_matches('.');
            self.file_extensions_hashset.insert(extension_without_dot.to_string());
        }
    }

    // E.g. when using similar videos, user can provide extensions like "mp4,flv", but if user provide "mp4,jpg" then
    // it will be only "mp4" because "jpg" is not valid extension for videos
    fn union_allowed_extensions(&mut self, file_extensions: &[&str]) {
        let mut new_extensions = HashSet::new();
        for extension in file_extensions {
            let extension_without_dot = extension.trim_start_matches('.');
            new_extensions.insert(extension_without_dot.to_string());
        }
    }

    pub fn set_and_validate_extensions(&mut self, file_extensions: &[&str]) {
        if self.file_extensions_hashset.is_empty() {
            self.extend_allowed_extensions(file_extensions);
        } else {
            self.union_allowed_extensions(file_extensions);
        }
    }
}
