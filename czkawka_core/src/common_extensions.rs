use std::collections::HashSet;
use std::fs::DirEntry;

use crate::common_messages::Messages;

#[derive(Debug, Clone, Default)]
pub struct Extensions {
    allowed_extensions_hashset: HashSet<String>,
    excluded_extensions_hashset: HashSet<String>,
}

impl Extensions {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn filter_extensions(mut file_extensions: String) -> (HashSet<String>, Messages) {
        let mut messages = Messages::new();
        let mut extensions_hashset = HashSet::new();

        if file_extensions.trim().is_empty() {
            return (Default::default(), messages);
        }
        file_extensions = file_extensions.replace("IMAGE", "jpg,kra,gif,png,bmp,tiff,hdr,svg");
        file_extensions = file_extensions.replace("VIDEO", "mp4,flv,mkv,webm,vob,ogv,gifv,avi,mov,wmv,mpg,m4v,m4p,mpeg,3gp");
        file_extensions = file_extensions.replace("MUSIC", "mp3,flac,ogg,tta,wma,webm");
        file_extensions = file_extensions.replace("TEXT", "txt,doc,docx,odt,rtf");

        let extensions: Vec<String> = file_extensions.split(',').map(str::trim).map(String::from).collect();
        for mut extension in extensions {
            if extension.is_empty() || extension.replace(['.', ' '], "").trim().is_empty() {
                continue;
            }

            if extension.starts_with('.') {
                extension = extension.chars().skip(1).collect::<String>();
            }

            if extension.contains('.') {
                messages.warnings.push(format!("{extension} is not valid extension because contains dot inside"));
                continue;
            }

            if extension.contains(' ') {
                messages.warnings.push(format!("{extension} is not valid extension because contains empty space inside"));
                continue;
            }

            extensions_hashset.insert(extension);
        }
        (extensions_hashset, messages)
    }

    /// List of allowed extensions, only files with this extensions will be checking if are duplicates
    /// After, extensions cannot contain any dot, commas etc.
    pub fn set_allowed_extensions(&mut self, allowed_extensions: String) -> Messages {
        let (extensions, messages) = Self::filter_extensions(allowed_extensions);

        self.allowed_extensions_hashset = extensions;
        messages
    }

    pub fn set_excluded_extensions(&mut self, excluded_extensions: String) -> Messages {
        let (extensions, messages) = Self::filter_extensions(excluded_extensions);

        self.excluded_extensions_hashset = extensions;
        messages
    }

    pub fn check_if_entry_have_valid_extension(&self, entry_data: &DirEntry) -> bool {
        if self.allowed_extensions_hashset.is_empty() && self.excluded_extensions_hashset.is_empty() {
            return true;
        }

        // Using entry_data.path().extension() is a lot of slower, even 5 times
        let file_name = entry_data.file_name();
        let Some(file_name_str) = file_name.to_str() else { return false };
        let Some(extension_idx) = file_name_str.rfind('.') else { return false };
        let extension = &file_name_str[extension_idx + 1..];

        if !self.allowed_extensions_hashset.is_empty() {
            if extension.chars().all(|c| c.is_ascii_lowercase()) {
                self.allowed_extensions_hashset.contains(extension)
            } else {
                self.allowed_extensions_hashset.contains(&extension.to_lowercase())
            }
        } else {
            if extension.chars().all(|c| c.is_ascii_lowercase()) {
                !self.excluded_extensions_hashset.contains(extension)
            } else {
                !self.excluded_extensions_hashset.contains(&extension.to_lowercase())
            }
        }
    }

    pub fn set_any_extensions(&self) -> bool {
        !self.allowed_extensions_hashset.is_empty()
    }

    fn extend_allowed_extensions(&mut self, file_extensions: &[&str]) {
        for extension in file_extensions {
            let extension_without_dot = extension.trim_start_matches('.');
            self.allowed_extensions_hashset.insert(extension_without_dot.to_string());
        }
    }

    // E.g. when using similar videos, user can provide extensions like "mp4,flv", but if user provide "mp4,jpg" then
    // it will be only "mp4" because "jpg" is not valid extension for videos
    #[allow(clippy::unused_self)]
    fn union_allowed_extensions(&mut self, file_extensions: &[&str]) {
        let mut new_extensions = HashSet::new();
        for extension in file_extensions {
            let extension_without_dot = extension.trim_start_matches('.');
            new_extensions.insert(extension_without_dot.to_string());
        }
    }

    pub fn set_and_validate_allowed_extensions(&mut self, file_extensions: &[&str]) {
        if self.allowed_extensions_hashset.is_empty() {
            self.extend_allowed_extensions(file_extensions);
        } else {
            self.union_allowed_extensions(file_extensions);
        }
    }
}
