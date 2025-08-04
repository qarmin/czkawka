mod core;
mod traits;
mod workarounds;

use std::path::{Path, PathBuf};

use serde::Serialize;

use crate::common::model::FileEntry;
use crate::common::tool_data::{CommonData, CommonToolData};
use crate::common::traits::*;

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

#[derive(Default)]
pub struct Info {
    pub number_of_files_with_bad_extension: usize,
}

pub struct BadExtensionsParameters {
    pub include_files_without_extension: bool,
}

impl BadExtensionsParameters {
    pub fn new() -> Self {
        Self {
            include_files_without_extension: false,
        } // TODO add option to all modes
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

    pub fn get_params(&self) -> &BadExtensionsParameters {
        &self.params
    }

    pub const fn get_information(&self) -> &Info {
        &self.information
    }
}

impl CommonData for BadExtensions {
    fn get_cd(&self) -> &CommonToolData {
        &self.common_data
    }
    fn get_cd_mut(&mut self) -> &mut CommonToolData {
        &mut self.common_data
    }
    fn found_any_broken_files(&self) -> bool {
        self.get_information().number_of_files_with_bad_extension > 0
    }
}
