use std::time::SystemTime;

use crate::common::Common;
use crate::common_messages::Messages;

#[derive(Clone, Default)]
pub struct Extensions {
    file_extensions: Vec<String>,
}

impl Extensions {
    pub fn new() -> Self {
        Default::default()
    }
    /// List of allowed extensions, only files with this extensions will be checking if are duplicates
    /// After, extensions cannot contains any dot, commas etc.
    pub fn set_allowed_extensions(&mut self, mut allowed_extensions: String, text_messages: &mut Messages) {
        let start_time: SystemTime = SystemTime::now();
        if allowed_extensions.trim().is_empty() {
            return;
        }
        allowed_extensions = allowed_extensions.replace("IMAGE", "jpg,kra,gif,png,bmp,tiff,hdr,svg");
        allowed_extensions = allowed_extensions.replace("VIDEO", "mp4,flv,mkv,webm,vob,ogv,gifv,avi,mov,wmv,mpg,m4v,m4p,mpeg,3gp");
        allowed_extensions = allowed_extensions.replace("MUSIC", "mp3,flac,ogg,tta,wma,webm");
        allowed_extensions = allowed_extensions.replace("TEXT", "txt,doc,docx,odt,rtf");

        let extensions: Vec<String> = allowed_extensions.split(',').map(|e| e.trim()).map(String::from).collect();
        for mut extension in extensions {
            if extension.is_empty() || extension.replace('.', "").replace(' ', "").trim().is_empty() {
                continue;
            }

            if !extension.starts_with('.') {
                extension = format!(".{}", extension);
            }

            if extension[1..].contains('.') {
                text_messages.warnings.push(format!("{} is not valid extension because contains dot inside", extension));
                continue;
            }

            if extension[1..].contains(' ') {
                text_messages
                    .warnings
                    .push(format!("{} is not valid extension because contains empty space inside", extension));
                continue;
            }

            if !self.file_extensions.contains(&extension) {
                self.file_extensions.push(extension);
            }
        }

        if self.file_extensions.is_empty() {
            text_messages
                .messages
                .push("No valid extensions were provided, so allowing all extensions by default.".to_string());
        }
        Common::print_time(start_time, SystemTime::now(), "set_allowed_extensions".to_string());
    }

    pub fn matches_filename(&self, file_name: &str) -> bool {
        // assert_eq!(file_name, file_name.to_lowercase());
        if !self.file_extensions.is_empty() && !self.file_extensions.iter().any(|e| file_name.ends_with(e)) {
            return false;
        }
        true
    }

    pub fn using_custom_extensions(&self) -> bool {
        !self.file_extensions.is_empty()
    }

    pub fn extend_allowed_extensions(&mut self, file_extensions: &[&str]) {
        for extension in file_extensions {
            assert!(extension.starts_with('.'));
            self.file_extensions.push(extension.to_string());
        }
    }
}
