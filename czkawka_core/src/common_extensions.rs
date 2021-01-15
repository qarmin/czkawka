use crate::common::Common;
use crate::common_messages::Messages;
use std::time::SystemTime;

#[derive(Default)]
pub struct Extensions {
    pub file_extensions: Vec<String>,
}

impl Extensions {
    pub fn new() -> Self {
        Default::default()
    }
    /// List of allowed extensions, only files with this extensions will be checking if are duplicates
    /// After, extensions cannot contains any dot, commas etc.
    pub fn set_allowed_extensions(&mut self, mut allowed_extensions: String, text_messages: &mut Messages) {
        let start_time: SystemTime = SystemTime::now();
        if allowed_extensions.is_empty() {
            return;
        }
        allowed_extensions = allowed_extensions.replace("IMAGE", "jpg,kra,gif,png,bmp,tiff,hdr,svg");
        allowed_extensions = allowed_extensions.replace("VIDEO", "mp4,flv,mkv,webm,vob,ogv,gifv,avi,mov,wmv,mpg,m4v,m4p,mpeg,3gp");
        allowed_extensions = allowed_extensions.replace("MUSIC", "mp3,flac,ogg,tta,wma,webm");
        allowed_extensions = allowed_extensions.replace("TEXT", "txt,doc,docx,odt,rtf");

        let extensions: Vec<String> = allowed_extensions.split(',').map(String::from).collect();
        for mut extension in extensions {
            if extension.is_empty() || extension.replace('.', "").trim() == "" {
                continue;
            }

            if extension.starts_with('.') {
                extension = extension[1..].to_string();
            }

            if extension[1..].contains('.') {
                text_messages.warnings.push(".".to_string() + extension.as_str() + " is not valid extension(valid extension doesn't have dot inside)");
                continue;
            }

            if !self.file_extensions.contains(&extension.trim().to_string()) {
                self.file_extensions.push(extension.trim().to_string());
            }
        }

        if self.file_extensions.is_empty() {
            text_messages.messages.push("No valid extensions were provided, so allowing all extensions by default.".to_string());
        }
        Common::print_time(start_time, SystemTime::now(), "set_allowed_extensions".to_string());
    }
}
