use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use rayon::prelude::*;

use crate::common::directories::Directories;
use crate::common::extensions::Extensions;
use crate::common::items::ExcludedItems;
use crate::common::progress_stop_handler::check_if_stop_received;
use crate::tools::folder_compare::EntryKind;

#[derive(Debug, Clone)]
pub(super) struct FsNode {
    pub name: Box<str>,
    pub kind: EntryKind,
    /// Size of a file, or the sum of all descendants for a folder.
    pub size: u64,
    /// Files and symlinks below a folder; zero for a file or symlink itself.
    pub file_count: u64,
    /// Sorted by name.
    pub children: Vec<Self>,
}

impl FsNode {
    fn leaf(name: Box<str>, kind: EntryKind, size: u64) -> Self {
        Self {
            name,
            kind,
            size,
            file_count: 0,
            children: Vec::new(),
        }
    }

    fn directory(name: Box<str>, mut children: Vec<Self>) -> Self {
        children.sort_unstable_by(|left, right| left.name.cmp(&right.name));
        let size = children.iter().map(|child| child.size).sum();
        let file_count = children.iter().map(|child| if child.kind == EntryKind::Directory { child.file_count } else { 1 }).sum();
        Self {
            name,
            kind: EntryKind::Directory,
            size,
            file_count,
            children,
        }
    }

    pub fn is_directory(&self) -> bool {
        self.kind == EntryKind::Directory
    }
}

/// File size filters are deliberately not applied: a skipped small file would make different folders look identical.
pub(super) struct ScanContext<'a> {
    pub directories: &'a Directories,
    pub excluded_items: &'a ExcludedItems,
    pub extensions: &'a Extensions,
    pub stop_flag: &'a Arc<AtomicBool>,
    pub entries_counter: &'a AtomicUsize,
}

pub(super) fn scan_roots(roots: &[PathBuf], context: &ScanContext) -> Vec<FsNode> {
    roots.par_iter().map(|root| scan_directory(root, &root.to_string_lossy(), context)).collect()
}

fn scan_directory(path: &Path, name: &str, context: &ScanContext) -> FsNode {
    if check_if_stop_received(context.stop_flag) {
        return FsNode::directory(name.into(), Vec::new());
    }
    let Ok(read_dir) = fs::read_dir(path) else {
        return FsNode::directory(name.into(), Vec::new());
    };

    let mut children = Vec::new();
    let mut subdirectories = Vec::new();
    for entry in read_dir.flatten() {
        let Ok(file_type) = entry.file_type() else { continue };
        let entry_path = entry.path();
        if context.excluded_items.is_excluded(&entry_path) {
            continue;
        }
        let entry_name = entry.file_name();
        if file_type.is_dir() {
            if !context.directories.is_excluded_dir(&entry_path) {
                subdirectories.push((entry_path, entry_name.to_string_lossy().into_owned()));
            }
            continue;
        }
        if context.directories.is_excluded_file(&entry_path) || !context.extensions.check_if_entry_have_valid_extension(&entry_name) {
            continue;
        }
        context.entries_counter.fetch_add(1, Ordering::Relaxed);
        let name = entry_name.to_string_lossy().into_owned().into_boxed_str();
        if file_type.is_symlink() {
            children.push(FsNode::leaf(name, EntryKind::Symlink, 0));
        } else if let Ok(metadata) = entry.metadata() {
            children.push(FsNode::leaf(name, EntryKind::File, metadata.len()));
        }
    }

    children.par_extend(
        subdirectories
            .into_par_iter()
            .map(|(subdirectory, subdirectory_name)| scan_directory(&subdirectory, &subdirectory_name, context)),
    );
    FsNode::directory(name.into(), children)
}

/// Roots nested inside another root would be scanned twice.
pub(super) fn without_nested_roots(roots: &[PathBuf]) -> Vec<PathBuf> {
    let mut result: Vec<PathBuf> = roots
        .iter()
        .filter(|root| !roots.iter().any(|other| other != *root && root.starts_with(other)))
        .cloned()
        .collect();
    result.sort_unstable();
    result.dedup();
    result
}
