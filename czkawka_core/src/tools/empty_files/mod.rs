pub mod core;
pub mod traits;
#[cfg(test)]
mod tests;

use crate::common::model::FileEntry;
use crate::common::tool_data::CommonToolData;

#[derive(Default, Clone)]
pub struct Info {
    pub number_of_empty_files: usize,
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

    pub const fn get_information(&self) -> &Info {
        &self.information
    }
}
