pub mod core;
pub mod traits;

use std::path::{Path, PathBuf};

use serde::Serialize;

use crate::common::tool_data::CommonToolData;
use crate::common::traits::ResultEntry;

const TEMP_EXTENSIONS: &[&str] = &[
    "#",
    "thumbs.db",
    ".bak",
    "~",
    ".tmp",
    ".temp",
    ".ds_store",
    ".crdownload",
    ".part",
    ".cache",
    ".dmp",
    ".download",
    ".partial",
];

#[derive(Clone, Serialize, Debug)]
pub struct TemporaryFileEntry {
    pub path: PathBuf,
    pub modified_date: u64,
    pub size: u64,
}

impl ResultEntry for TemporaryFileEntry {
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

#[derive(Default, Clone)]
pub struct Info {
    pub number_of_temporary_files: usize,
}

pub struct Temporary {
    common_data: CommonToolData,
    information: Info,
    temporary_files: Vec<TemporaryFileEntry>,
}

impl Default for Temporary {
    fn default() -> Self {
        Self::new()
    }
}

impl Temporary {
    pub const fn get_temporary_files(&self) -> &Vec<TemporaryFileEntry> {
        &self.temporary_files
    }

    pub const fn get_information(&self) -> &Info {
        &self.information
    }
}
