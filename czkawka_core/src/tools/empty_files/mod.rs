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

pub struct EmptyFiles {
    common_data: CommonToolData,
    information: Info,
    empty_files: Vec<FileEntry>,
}

impl Default for EmptyFiles {
    fn default() -> Self {
        Self::new()
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
