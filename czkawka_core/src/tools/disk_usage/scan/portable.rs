use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::Ordering;

use rayon::prelude::*;

use crate::common::progress_stop_handler::check_if_stop_received;
use crate::tools::disk_usage::category::FileCategory;
use crate::tools::disk_usage::scan::{ScanContext, TreeSource};
use crate::tools::disk_usage::tree::{DiskNode, DiskTree};

pub(crate) struct PortableSource;

impl TreeSource for PortableSource {
    fn scan(&self, roots: &[PathBuf], context: &ScanContext) -> DiskTree {
        let nodes = roots.par_iter().map(|root| scan_root(root, context)).collect();
        DiskTree::new(nodes)
    }
}

fn scan_root(root: &Path, context: &ScanContext) -> DiskNode {
    let name = root.to_string_lossy();
    match fs::metadata(root) {
        Ok(metadata) if metadata.is_dir() => scan_directory(root, &name, context),
        Ok(metadata) => DiskNode::file(&*name, metadata.len(), file_category(root, metadata.len(), context)),
        Err(_) => DiskNode::inaccessible(&*name),
    }
}

fn scan_directory(path: &Path, name: &str, context: &ScanContext) -> DiskNode {
    if check_if_stop_received(context.stop_flag) {
        return DiskNode::directory(name, Vec::new());
    }
    let Ok(read_dir) = fs::read_dir(path) else {
        return DiskNode::inaccessible(name);
    };

    let mut children = Vec::new();
    let mut subdirectories = Vec::new();
    for entry in read_dir.flatten() {
        let Ok(file_type) = entry.file_type() else {
            continue;
        };
        let entry_path = entry.path();
        if is_excluded(&entry_path, file_type.is_dir(), context) {
            continue;
        }
        context.entries_counter.fetch_add(1, Ordering::Relaxed);
        let entry_name = entry.file_name().to_string_lossy().into_owned();
        if file_type.is_dir() {
            subdirectories.push((entry_path, entry_name));
        } else if file_type.is_symlink() {
            children.push(DiskNode::symlink(entry_name));
        } else {
            children.push(match entry.metadata() {
                Ok(metadata) => DiskNode::file(entry_name, metadata.len(), file_category(&entry_path, metadata.len(), context)),
                Err(_) => DiskNode::inaccessible(entry_name),
            });
        }
    }

    children.par_extend(
        subdirectories
            .into_par_iter()
            .map(|(subdirectory, subdirectory_name)| scan_directory(&subdirectory, &subdirectory_name, context)),
    );
    DiskNode::directory(name, children)
}

fn is_excluded(path: &Path, is_directory: bool, context: &ScanContext) -> bool {
    if !context.ignore_excluded_items && context.excluded_items.is_excluded(path) {
        return true;
    }
    if is_directory {
        (!context.ignore_excluded_directories && context.directories.is_excluded_dir(path)) || is_on_other_filesystem(path, context)
    } else {
        !context.ignore_excluded_directories && context.directories.is_excluded_file(path)
    }
}

#[cfg(target_family = "unix")]
fn is_on_other_filesystem(path: &Path, context: &ScanContext) -> bool {
    context.directories.exclude_other_filesystems() && context.directories.is_on_other_filesystems(path).unwrap_or(false)
}

#[cfg(not(target_family = "unix"))]
fn is_on_other_filesystem(_path: &Path, _context: &ScanContext) -> bool {
    false
}

fn file_category(path: &Path, size: u64, context: &ScanContext) -> FileCategory {
    let by_extension = FileCategory::from_path(path);
    if by_extension == FileCategory::Other && context.content_detection.should_read_header(size) {
        FileCategory::from_content(path).unwrap_or(FileCategory::Other)
    } else {
        by_extension
    }
}
