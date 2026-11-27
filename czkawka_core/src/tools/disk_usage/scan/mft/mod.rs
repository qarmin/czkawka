mod assemble;
mod reader;
mod volume;

use std::collections::HashMap;
use std::fmt;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use ntfs_mft::{MftError, Volume};

use crate::tools::disk_usage::scan::mft::assemble::MftIndex;
use crate::tools::disk_usage::scan::mft::volume::split_drive_path;
use crate::tools::disk_usage::scan::portable::PortableSource;
use crate::tools::disk_usage::scan::{ScanContext, TreeSource};
use crate::tools::disk_usage::tree::{DiskNode, DiskTree};

/// Why a path could not be read from the master file table and was scanned the ordinary way.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum MftFallbackReason {
    NotElevated,
    NotDriveLetterPath,
    NotFound,
    Failed(String),
}

impl fmt::Display for MftFallbackReason {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotElevated => formatter.write_str("administrator privileges are required to read the NTFS master file table"),
            Self::NotDriveLetterPath => formatter.write_str("only paths on a drive letter can be read from the NTFS master file table"),
            Self::NotFound => formatter.write_str("the path was not found in the NTFS master file table"),
            Self::Failed(message) => write!(formatter, "reading the NTFS master file table failed: {message}"),
        }
    }
}

impl From<MftError> for MftFallbackReason {
    fn from(error: MftError) -> Self {
        match error {
            MftError::NotElevated => Self::NotElevated,
            other => Self::Failed(other.to_string()),
        }
    }
}

/// Reads sizes from the NTFS master file table, which is much faster than walking directories.
/// Paths that cannot be read this way are scanned by the portable source, and the first reason is remembered.
#[derive(Default)]
pub(crate) struct MftSource {
    first_fallback: Mutex<Option<MftFallbackReason>>,
}

impl MftSource {
    pub(crate) fn take_fallback_reason(&self) -> Option<MftFallbackReason> {
        self.first_fallback.lock().ok().and_then(|mut reason| reason.take())
    }

    fn remember_fallback(&self, root: &Path, reason: MftFallbackReason) {
        log::warn!("Scanning {} without the NTFS master file table: {reason}", root.display());
        if let Ok(mut first) = self.first_fallback.lock() {
            first.get_or_insert(reason);
        }
    }
}

impl TreeSource for MftSource {
    fn scan(&self, roots: &[PathBuf], context: &ScanContext) -> DiskTree {
        let mut volumes: HashMap<char, Result<MftIndex, MftFallbackReason>> = HashMap::new();
        let nodes = roots
            .iter()
            .map(|root| match scan_root(root, &mut volumes, context) {
                Ok(node) => node,
                Err(reason) => {
                    self.remember_fallback(root, reason);
                    PortableSource.scan(std::slice::from_ref(root), context).roots.remove(0)
                }
            })
            .collect();
        DiskTree::new(nodes)
    }
}

fn scan_root(root: &Path, volumes: &mut HashMap<char, Result<MftIndex, MftFallbackReason>>, context: &ScanContext) -> Result<DiskNode, MftFallbackReason> {
    let root_text = root.to_string_lossy();
    let (letter, components) = split_drive_path(&root_text).ok_or(MftFallbackReason::NotDriveLetterPath)?;
    let index = volumes.entry(letter).or_insert_with(|| read_index(letter, context)).as_ref().map_err(Clone::clone)?;
    index.build_root(root, &components, context).ok_or(MftFallbackReason::NotFound)
}

fn read_index(letter: char, context: &ScanContext) -> Result<MftIndex, MftFallbackReason> {
    let volume = open_volume(letter)?;
    let entries = reader::read_volume(&volume, context.stop_flag, context.entries_counter)?;
    Ok(MftIndex::new(entries))
}

#[cfg(windows)]
fn open_volume(letter: char) -> Result<Volume, MftFallbackReason> {
    Volume::from_drive_letter(letter).map_err(Into::into)
}

#[cfg(not(windows))]
fn open_volume(_letter: char) -> Result<Volume, MftFallbackReason> {
    Err(MftFallbackReason::Failed("drive letters exist only on Windows".to_string()))
}

#[cfg(windows)]
pub(crate) fn probe_system_drive() -> Result<(), MftFallbackReason> {
    let system_drive = std::env::var("SystemDrive").ok().and_then(|drive| drive.chars().next()).unwrap_or('C');
    open_volume(system_drive.to_ascii_uppercase()).map(drop)
}
