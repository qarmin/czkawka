pub mod core;
pub mod traits;

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::common::model::FileEntry;
use crate::common::tool_data::CommonToolData;
use crate::common::traits::*;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct BrokenEntry {
    pub path: PathBuf,
    pub modified_date: u64,
    pub size: u64,
    pub type_of_file: TypeOfFile,
    pub error_string: String,
}
impl ResultEntry for BrokenEntry {
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
    fn into_broken_entry(self) -> BrokenEntry {
        BrokenEntry {
            size: self.size,
            path: self.path,
            modified_date: self.modified_date,

            type_of_file: TypeOfFile::Unknown,
            error_string: String::new(),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum TypeOfFile {
    Unknown = -1,
    Image = 0,
    ArchiveZip,
    Audio,
    PDF,
}

bitflags! {
    #[derive(PartialEq, Copy, Clone, Debug)]
    pub struct CheckedTypes : u32 {
        const NONE = 0;

        const PDF = 0b1;
        const AUDIO = 0b10;
        const IMAGE = 0b100;
        const ARCHIVE = 0b1000;
    }
}

#[derive(Default, Clone)]
pub struct Info {
    pub number_of_broken_files: usize,
}

#[derive(Clone)]
pub struct BrokenFilesParameters {
    pub checked_types: CheckedTypes,
}

impl BrokenFilesParameters {
    pub fn new(checked_types: CheckedTypes) -> Self {
        Self { checked_types }
    }
}

pub struct BrokenFiles {
    common_data: CommonToolData,
    information: Info,
    files_to_check: BTreeMap<String, BrokenEntry>,
    broken_files: Vec<BrokenEntry>,
    params: BrokenFilesParameters,
}

impl BrokenFiles {
    pub const fn get_broken_files(&self) -> &Vec<BrokenEntry> {
        &self.broken_files
    }

    pub(crate) fn get_params(&self) -> &BrokenFilesParameters {
        &self.params
    }

    pub const fn get_information(&self) -> &Info {
        &self.information
    }
}
