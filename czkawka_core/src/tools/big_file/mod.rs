pub mod core;
pub mod traits;
#[cfg(test)]
mod tests;

use crate::common::model::FileEntry;
use crate::common::tool_data::CommonToolData;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum SearchMode {
    BiggestFiles,
    SmallestFiles,
}

#[derive(Debug, Default, Clone)]
pub struct Info {
    pub number_of_real_files: usize,
}

#[derive(Clone)]
pub struct BigFileParameters {
    pub number_of_files_to_check: usize,
    pub search_mode: SearchMode,
}

impl BigFileParameters {
    pub fn new(number_of_files: usize, search_mode: SearchMode) -> Self {
        Self {
            number_of_files_to_check: number_of_files.max(1),
            search_mode,
        }
    }
}

pub struct BigFile {
    common_data: CommonToolData,
    information: Info,
    big_files: Vec<FileEntry>,
    params: BigFileParameters,
}

impl BigFile {
    pub const fn get_big_files(&self) -> &Vec<FileEntry> {
        &self.big_files
    }
}
