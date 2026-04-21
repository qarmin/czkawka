pub mod core;
pub mod traits;

use std::path::{Path, PathBuf};
use std::time::Duration;

use serde::Serialize;

use crate::common::tool_data::CommonToolData;
use crate::common::traits::ResultEntry;

pub const TEMP_EXTENSIONS: &[&str] = &[
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

/// Default extensions as a comma-separated string (mirrors `TEMP_EXTENSIONS`).
/// Used by UIs to pre-populate the extensions field and as the reset value.
pub const DEFAULT_TEMP_EXTENSIONS_STR: &str = "#,thumbs.db,.bak,~,.tmp,.temp,.ds_store,.crdownload,.part,.cache,.dmp,.download,.partial";

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

#[derive(Default, Clone, Copy)]
pub struct Info {
    pub number_of_temporary_files: usize,
    pub scanning_time: Duration,
}

#[derive(Clone, Debug)]
pub struct TemporaryParameters {
    /// Full list of extensions/suffixes treated as temporary files.
    /// Each entry is matched against the lowercased filename using `ends_with`.
    /// Defaults to the built-in `TEMP_EXTENSIONS` list.
    /// Must not be empty - the core will return an error if it is.
    pub extensions: Vec<String>,
}

impl Default for TemporaryParameters {
    fn default() -> Self {
        Self {
            extensions: TEMP_EXTENSIONS.iter().map(|s| s.to_string()).collect(),
        }
    }
}

impl TemporaryParameters {
    pub fn new() -> Self {
        Self::default()
    }
}

pub struct Temporary {
    common_data: CommonToolData,
    information: Info,
    temporary_files: Vec<TemporaryFileEntry>,
    params: TemporaryParameters,
}

impl Temporary {
    pub const fn get_temporary_files(&self) -> &Vec<TemporaryFileEntry> {
        &self.temporary_files
    }

    pub const fn get_information(&self) -> Info {
        self.information
    }
}
