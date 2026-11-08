#[cfg(feature = "ntfs_mft")]
pub(crate) mod mft;
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

/// Builds the tree with the fastest requested source; the second value tells why the NTFS master file table was not used.
pub(crate) fn scan_roots(read_ntfs_mft: bool, roots: &[PathBuf], context: &ScanContext) -> (DiskTree, Option<String>) {
    #[cfg(feature = "ntfs_mft")]
    if read_ntfs_mft {
        let source = mft::MftSource::default();
        let tree = source.scan(roots, context);
        return (tree, source.take_fallback_reason().map(|reason| reason.to_string()));
    }
    let unsupported = read_ntfs_mft.then(|| "this build does not support reading the NTFS master file table".to_string());
    (portable::PortableSource.scan(roots, context), unsupported)
}

/// Whether the NTFS master file table can be read on this machine right now.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NtfsMftAvailability {
    /// Not Windows, or the build lacks the `ntfs_mft` feature; frontends should hide the option.
    NotSupported,
    /// Windows, but the process is not elevated; the scan would silently use the slower way.
    NotElevated,
    Available,
    /// Windows and elevated, but the system volume could not be opened.
    Failed(String),
}

#[cfg(all(feature = "ntfs_mft", windows))]
pub fn ntfs_mft_availability() -> NtfsMftAvailability {
    match mft::probe_system_drive() {
        Ok(()) => NtfsMftAvailability::Available,
        Err(mft::MftFallbackReason::NotElevated) => NtfsMftAvailability::NotElevated,
        Err(other) => NtfsMftAvailability::Failed(other.to_string()),
    }
}

#[cfg(not(all(feature = "ntfs_mft", windows)))]
pub fn ntfs_mft_availability() -> NtfsMftAvailability {
    NtfsMftAvailability::NotSupported
}
