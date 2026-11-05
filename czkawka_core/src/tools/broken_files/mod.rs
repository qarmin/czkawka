use bitflags::bitflags;

pub mod core;
mod repair;
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
use crate::tools::file_repair::{AssessParameters, Repairability};

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
    /// Filled only when the scan was asked to check whether broken files can be repaired.
    pub repairability: Option<Repairability>,
    /// Camera profiles known when `repairability` was computed; a different count may change the verdict.
    pub profiles_when_assessed: u32,
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
            repairability: None,
            profiles_when_assessed: 0,
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
    /// `None` skips the trial repairs entirely.
    pub repair_assessment: Option<AssessParameters>,
    /// Drops every file that is not known to be repairable; has an effect only together with `repair_assessment`.
    pub only_repairable: bool,
}

impl BrokenFilesParameters {
    pub fn new(checked_types: CheckedTypes) -> Self {
        Self {
            checked_types,
            repair_assessment: None,
            only_repairable: false,
        }
    }

    pub fn with_repair_assessment(mut self, assessment: AssessParameters) -> Self {
        self.repair_assessment = Some(assessment);
        self
    }

    pub fn with_only_repairable(mut self, only_repairable: bool) -> Self {
        self.only_repairable = only_repairable;
        self
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
