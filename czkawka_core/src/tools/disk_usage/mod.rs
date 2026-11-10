pub mod category;
pub mod core;
pub mod layout;
pub mod report;
pub(crate) mod scan;
#[cfg(test)]
mod tests;
pub mod traits;
pub mod tree;

use std::time::Duration;

use crate::common::tool_data::CommonToolData;
use crate::tools::disk_usage::tree::DiskTree;

#[derive(Debug, Default, Clone)]
pub struct Info {
    pub number_of_files: usize,
    pub number_of_directories: usize,
    pub number_of_inaccessible_entries: usize,
    pub total_size: u64,
    pub scanning_time: Duration,
}

/// How files with an unknown extension are categorized.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ContentDetection {
    /// Only the extension is used.
    ExtensionOnly,
    /// The header is read only for files of at least [`ContentDetection::LARGE_FILE_SIZE`], which dominate the chart.
    #[default]
    LargeFiles,
    /// The header of every file with an unknown extension is read; much slower.
    AllFiles,
}

impl ContentDetection {
    pub const LARGE_FILE_SIZE: u64 = 1024 * 1024;

    pub const fn should_read_header(self, file_size: u64) -> bool {
        match self {
            Self::ExtensionOnly => false,
            Self::LargeFiles => file_size >= Self::LARGE_FILE_SIZE,
            Self::AllFiles => true,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DiskUsageParameters {
    pub content_detection: ContentDetection,
    /// Skips the "excluded items" patterns, whose defaults hide hidden folders, caches and `node_modules`.
    pub ignore_excluded_items: bool,
    /// Skips the "excluded paths", so even folders like `/proc` are measured.
    pub ignore_excluded_directories: bool,
    /// How many directory levels the text and JSON reports contain.
    pub report_depth: usize,
    /// How many of the biggest entries of every directory the reports contain.
    pub report_entries_per_directory: usize,
}

impl DiskUsageParameters {
    pub const DEFAULT_REPORT_DEPTH: usize = 2;
    pub const DEFAULT_REPORT_ENTRIES: usize = 10;

    pub fn new(content_detection: ContentDetection) -> Self {
        Self {
            content_detection,
            ignore_excluded_items: true,
            ignore_excluded_directories: false,
            report_depth: Self::DEFAULT_REPORT_DEPTH,
            report_entries_per_directory: Self::DEFAULT_REPORT_ENTRIES,
        }
    }
}

/// Measures how much space every file and directory takes and keeps the result as a [`DiskTree`].
pub struct DiskUsage {
    common_data: CommonToolData,
    information: Info,
    tree: DiskTree,
    params: DiskUsageParameters,
}

impl DiskUsage {
    pub const fn get_tree(&self) -> &DiskTree {
        &self.tree
    }

    pub const fn get_tree_mut(&mut self) -> &mut DiskTree {
        &mut self.tree
    }
}
