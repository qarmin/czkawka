pub mod core;
#[cfg(test)]
mod tests;
pub mod traits;

use std::collections::BTreeMap;
use std::path::PathBuf;
use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::common::tool_data::CommonToolData;
use crate::common::traits::ResultEntry;

#[derive(Debug, Default, Clone)]
pub struct Info {
    pub number_of_files_with_exif: usize,
    pub scanning_time: Duration,
}

#[derive(Clone, Default)]
pub struct ExifFinderParameters {
    pub ignored_tags: Vec<String>,
}

impl ExifFinderParameters {
    pub fn new(ignored_tags: Vec<String>) -> Self {
        Self { ignored_tags }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ExifEntry {
    pub path: PathBuf,
    pub size: u64,
    pub modified_date: u64,
    pub exif_tags: Vec<String>,
    pub error: Option<String>,
}

impl ResultEntry for ExifEntry {
    fn get_path(&self) -> &std::path::Path {
        &self.path
    }
    fn get_modified_date(&self) -> u64 {
        self.modified_date
    }
    fn get_size(&self) -> u64 {
        self.size
    }
}

pub struct ExifRemover {
    common_data: CommonToolData,
    information: Info,
    exif_files: Vec<ExifEntry>,
    files_to_check: BTreeMap<String, ExifEntry>,
    params: ExifFinderParameters,
}

impl ExifRemover {
    pub const fn get_exif_files(&self) -> &Vec<ExifEntry> {
        &self.exif_files
    }

    pub const fn get_information(&self) -> &Info {
        &self.information
    }
}
