pub mod core;
#[cfg(test)]
mod tests;
pub mod traits;

use std::time::Duration;

use crate::common::model::FileEntry;
use crate::common::tool_data::CommonToolData;

#[derive(Default, Clone, Copy)]
pub struct Info {
    pub number_of_empty_files: usize,
    pub scanning_time: Duration,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct EmptyFilesParameters {
    /// Also find non-empty files whose entire content consists of null bytes (`\0`).
    pub search_zero_byte_content_files: bool,
    /// Also find non-empty files whose entire content consists of non-printable ASCII
    /// characters (null, space, tab, CR, LF, VT, FF).
    pub search_non_printable_content_files: bool,
}

pub struct EmptyFiles {
    common_data: CommonToolData,
    information: Info,
    empty_files: Vec<FileEntry>,
    files_to_check: Vec<FileEntry>,
    pub params: EmptyFilesParameters,
}

impl Default for EmptyFiles {
    fn default() -> Self {
        Self::new(EmptyFilesParameters::default())
    }
}

impl EmptyFiles {
    pub const fn get_empty_files(&self) -> &Vec<FileEntry> {
        &self.empty_files
    }

    pub const fn get_information(&self) -> Info {
        self.information
    }
}
