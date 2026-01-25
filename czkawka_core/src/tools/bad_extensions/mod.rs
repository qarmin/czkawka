pub mod core;
#[cfg(test)]
mod tests;
pub mod traits;
mod workarounds;

use std::path::{Path, PathBuf};
use std::time::Duration;

use serde::Serialize;

use crate::common::model::FileEntry;
use crate::common::tool_data::CommonToolData;
use crate::common::traits::ResultEntry;

#[derive(Clone, Serialize, Debug)]
pub struct BadFileEntry {
    pub path: PathBuf,
    pub modified_date: u64,
    pub size: u64,
    pub current_extension: String,
    pub proper_extensions_group: String,
    pub proper_extension: String,
}

impl ResultEntry for BadFileEntry {
    fn get_path(&self) -> &Path {
        &self.path
    }
    fn get_modified_date(&self) -> u64 {
        self.modified_date
    }
    fn get_size(&self) -> u64 {
        self.size
    }
}

#[derive(Default, Clone, Copy)]
pub struct Info {
    pub number_of_files_with_bad_extension: usize,
    pub scanning_time: Duration,
}

#[derive(Clone)]
pub struct BadExtensionsParameters {
    pub include_files_without_extension: bool,
}

impl BadExtensionsParameters {
    pub fn new() -> Self {
        Self {
            include_files_without_extension: false,
        }
    }
}
impl Default for BadExtensionsParameters {
    fn default() -> Self {
        Self::new()
    }
}

pub struct BadExtensions {
    common_data: CommonToolData,
    information: Info,
    files_to_check: Vec<FileEntry>,
    bad_extensions_files: Vec<BadFileEntry>,
    params: BadExtensionsParameters,
}

impl BadExtensions {
    pub const fn get_bad_extensions_files(&self) -> &Vec<BadFileEntry> {
        &self.bad_extensions_files
    }
}
