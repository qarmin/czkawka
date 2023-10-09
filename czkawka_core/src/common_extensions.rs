use crate::common_messages::Messages;

#[derive(Debug, Clone, Default)]
pub struct Extensions {
    file_extensions: Vec<String>,
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

            if !extension.starts_with('.') {
                extension = format!(".{extension}");
            }

            if extension[1..].contains('.') {
                messages.warnings.push(format!("{extension} is not valid extension because contains dot inside"));
                continue;
            }

            if extension[1..].contains(' ') {
                messages.warnings.push(format!("{extension} is not valid extension because contains empty space inside"));
                continue;
            }

            if !self.file_extensions.contains(&extension) {
                self.file_extensions.push(extension);
            }
        }

        if self.file_extensions.is_empty() {
            messages
                .messages
                .push("No valid extensions were provided, so allowing all extensions by default.".to_string());
        }
        messages
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
            self.file_extensions.push((*extension).to_string());
        }
    }

    pub fn validate_allowed_extensions(&mut self, file_extensions: &[&str]) {
        let mut current_file_extensions = Vec::new();

        for extension in file_extensions {
            assert!(extension.starts_with('.'));
            if self.file_extensions.contains(&(*extension).to_string()) {
                current_file_extensions.push((*extension).to_string());
            }
        }
        self.file_extensions = current_file_extensions;
    }
}
