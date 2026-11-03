pub(crate) mod portable;

use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize};

use crate::common::directories::Directories;
use crate::common::items::ExcludedItems;
use crate::tools::disk_usage::ContentDetection;
use crate::tools::disk_usage::tree::DiskTree;

pub(crate) struct ScanContext<'a> {
    pub directories: &'a Directories,
    pub stop_flag: &'a Arc<AtomicBool>,
    pub entries_counter: &'a AtomicUsize,
    pub content_detection: ContentDetection,
    pub excluded_items: &'a ExcludedItems,
    pub ignore_excluded_items: bool,
    pub ignore_excluded_directories: bool,
}

/// A way of building the size tree; a faster backend (e.g. reading the NTFS MFT) can be added next to the portable one.
pub(crate) trait TreeSource: Sync {
    /// Roots may be directories or single files. When stopped, returns whatever was gathered so far.
    fn scan(&self, roots: &[PathBuf], context: &ScanContext) -> DiskTree;
}
