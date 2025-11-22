pub mod core;
pub mod traits;
mod tests;

use std::fmt::Display;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::common::model::FileEntry;
use crate::common::tool_data::CommonToolData;
use crate::common::traits::ResultEntry;
use crate::flc;

#[derive(Default, Clone)]
pub struct Info {
    pub number_of_invalid_symlinks: usize,
}

const MAX_NUMBER_OF_SYMLINK_JUMPS: i32 = 20;

#[derive(Clone, Debug, PartialEq, Eq, Copy, Deserialize, Serialize)]
pub enum ErrorType {
    InfiniteRecursion,
    NonExistentFile,
}

impl ErrorType {
    pub fn translate(&self) -> String {
        match *self {
            Self::InfiniteRecursion => flc!("core_invalid_symlink_infinite_recursion"),
            Self::NonExistentFile => flc!("core_invalid_symlink_non_existent_destination"),
        }
    }
}

impl Display for ErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InfiniteRecursion => write!(f, "Infinite recursion"),
            Self::NonExistentFile => write!(f, "Non existent file"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct SymlinkInfo {
    pub destination_path: PathBuf,
    pub type_of_error: ErrorType,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SymlinksFileEntry {
    pub path: PathBuf,
    pub size: u64,
    pub modified_date: u64,
    pub symlink_info: SymlinkInfo,
}

impl ResultEntry for SymlinksFileEntry {
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

impl FileEntry {
    fn into_symlinks_entry(self, symlink_info: SymlinkInfo) -> SymlinksFileEntry {
        SymlinksFileEntry {
            size: self.size,
            path: self.path,
            modified_date: self.modified_date,

            symlink_info,
        }
    }
}

pub struct InvalidSymlinks {
    common_data: CommonToolData,
    information: Info,
    invalid_symlinks: Vec<SymlinksFileEntry>,
}

impl Default for InvalidSymlinks {
    fn default() -> Self {
        Self::new()
    }
}

impl InvalidSymlinks {
    pub const fn get_invalid_symlinks(&self) -> &Vec<SymlinksFileEntry> {
        &self.invalid_symlinks
    }

    pub const fn get_information(&self) -> &Info {
        &self.information
    }
}
