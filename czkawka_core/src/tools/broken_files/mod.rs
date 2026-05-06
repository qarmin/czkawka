use bitflags::bitflags;

pub mod core;
#[cfg(test)]
mod tests;
pub mod traits;

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::common::model::FileEntry;
use crate::common::tool_data::CommonToolData;
use crate::common::traits::ResultEntry;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Debug)]
pub enum CheckedTypesSingle {
    Pdf,
    Audio,
    Image,
    Archive,
    VideoFfprobe,
    VideoFfmpeg,
    Font,
    Markup,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct BrokenEntry {
    pub path: PathBuf,
    pub modified_date: u64,
    pub size: u64,
    pub errors: BTreeMap<CheckedTypesSingle, Option<String>>,
}

impl BrokenEntry {
    pub fn has_errors(&self) -> bool {
        self.errors.values().any(Option::is_some)
    }

    pub fn get_error_string(&self) -> String {
        self.errors.values().filter_map(Option::as_deref).collect::<Vec<_>>().join(", ")
    }
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
            errors: BTreeMap::new(),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum TypeOfFile {
    Image = 0,
    ArchiveZip,
    Audio,
    Pdf,
    Video,
    Archive7z,
    ArchiveGz,
    ArchiveTar,
    ArchiveZst,
    Font,
    Json,
    Xml,
    Toml,
    Yaml,
    ArchiveBz2,
    ArchiveXz,
    Svg,
}

bitflags! {
    #[derive(PartialEq, Copy, Clone, Debug)]
    pub struct CheckedTypes : u32 {
        const NONE = 0;

        const PDF = 0b1;
        const AUDIO = 0b10;
        const IMAGE = 0b100;
        const ARCHIVE = 0b1000;
        const VIDEO_FFPROBE = 0b10000;
        const VIDEO_FFMPEG = 0b100000;
        const FONT = 0b1000000;
        const MARKUP = 0b10000000;
    }
}

#[derive(Default, Clone, Copy)]
pub struct Info {
    pub number_of_broken_files: usize,
    pub scanning_time: Duration,
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

    pub const fn get_information(&self) -> Info {
        self.information
    }
}
