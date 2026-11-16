use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::atomic::Ordering;

use crate::common::progress_stop_handler::check_if_stop_received;
use crate::tools::disk_usage::category::FileCategory;
use crate::tools::disk_usage::scan::ScanContext;
use crate::tools::disk_usage::scan::portable::is_excluded;
use crate::tools::disk_usage::tree::DiskNode;

pub(super) const ROOT_RECORD: u64 = 5;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum MftEntryKind {
    File,
    Directory,
    /// Symlinks and junctions; never followed.
    Symlink,
}

/// One name of a file or directory; a file with several hard links has one entry per link.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct MftEntry {
    pub record: u64,
    pub parent_record: u64,
    pub name: Box<str>,
    pub kind: MftEntryKind,
    /// Only the first link of a file carries its size, so hard links are counted once.
    pub size: u64,
}

/// All entries of a volume, grouped by the directory that holds them.
pub(super) struct MftIndex {
    entries: Vec<MftEntry>,
    children_by_parent: HashMap<u64, Vec<usize>>,
}

impl MftIndex {
    pub(super) fn new(entries: Vec<MftEntry>) -> Self {
        let mut children_by_parent: HashMap<u64, Vec<usize>> = HashMap::new();
        for (index, entry) in entries.iter().enumerate() {
            if entry.record != ROOT_RECORD {
                children_by_parent.entry(entry.parent_record).or_default().push(index);
            }
        }
        Self { entries, children_by_parent }
    }

    /// Builds the node of `root`, which is `components` below the volume root; `None` when there is no such entry.
    pub(super) fn build_root(&self, root: &Path, components: &[&str], context: &ScanContext) -> Option<DiskNode> {
        let name = root.to_string_lossy();
        if components.is_empty() {
            return Some(self.build_directory(ROOT_RECORD, &name, &mut root.to_path_buf(), context));
        }
        let mut parent_record = ROOT_RECORD;
        let mut target = None;
        for component in components {
            let entry = self.find_child(parent_record, component)?;
            parent_record = entry.record;
            target = Some(entry);
        }
        let entry = target?;
        let mut path = root.to_path_buf();
        Some(match entry.kind {
            MftEntryKind::Directory => self.build_directory(entry.record, &name, &mut path, context),
            MftEntryKind::File => DiskNode::file(&*name, entry.size, categorize(&entry.name, entry.size, path.parent(), context)),
            MftEntryKind::Symlink => DiskNode::symlink(&*name),
        })
    }

    fn find_child(&self, parent_record: u64, name: &str) -> Option<&MftEntry> {
        let children = self.children_by_parent.get(&parent_record)?;
        let lowercase_name = name.to_lowercase();
        children
            .iter()
            .filter_map(|&index| self.entries.get(index))
            .find(|entry| &*entry.name == name || entry.name.to_lowercase() == lowercase_name)
    }

    fn build_directory(&self, record: u64, name: &str, path: &mut PathBuf, context: &ScanContext) -> DiskNode {
        if check_if_stop_received(context.stop_flag) {
            return DiskNode::directory(name, Vec::new());
        }
        let mut children = Vec::new();
        for entry in self.children_by_parent.get(&record).into_iter().flatten().filter_map(|&index| self.entries.get(index)) {
            path.push(&*entry.name);
            if !is_excluded(path, entry.kind == MftEntryKind::Directory, context) {
                context.entries_counter.fetch_add(1, Ordering::Relaxed);
                children.push(match entry.kind {
                    MftEntryKind::Directory => self.build_directory(entry.record, &entry.name, path, context),
                    MftEntryKind::Symlink => DiskNode::symlink(&*entry.name),
                    MftEntryKind::File => DiskNode::file(&*entry.name, entry.size, categorize(&entry.name, entry.size, path.parent(), context)),
                });
            }
            path.pop();
        }
        DiskNode::directory(name, children)
    }
}

fn categorize(name: &str, size: u64, directory: Option<&Path>, context: &ScanContext) -> FileCategory {
    let by_extension = name.rsplit_once('.').map_or(FileCategory::Other, |(_, extension)| FileCategory::from_extension(extension));
    match directory {
        Some(directory) if by_extension == FileCategory::Other && context.content_detection.should_read_header(size) => {
            FileCategory::from_content(&directory.join(name)).unwrap_or(FileCategory::Other)
        }
        _ => by_extension,
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, AtomicUsize};

    use super::*;
    use crate::common::directories::Directories;
    use crate::common::items::ExcludedItems;
    use crate::tools::disk_usage::ContentDetection;

    fn entry(record: u64, parent_record: u64, name: &str, kind: MftEntryKind, size: u64) -> MftEntry {
        MftEntry {
            record,
            parent_record,
            name: name.into(),
            kind,
            size,
        }
    }

    fn sample_index() -> MftIndex {
        use MftEntryKind::{Directory, File, Symlink};
        MftIndex::new(vec![
            entry(ROOT_RECORD, ROOT_RECORD, ".", Directory, 0),
            entry(30, ROOT_RECORD, "Users", Directory, 0),
            entry(31, 30, "song.MP3", File, 100),
            entry(32, 30, "notes.txt", File, 10),
            entry(33, 30, "link", Symlink, 0),
            entry(34, 30, "Deep", Directory, 0),
            entry(35, 34, "movie.mkv", File, 1000),
            entry(36, ROOT_RECORD, "top.bin", File, 20),
            entry(37, 999, "orphan.txt", File, 5),
        ])
    }

    fn build(index: &MftIndex, root: &str, components: &[&str], stop: bool) -> Option<DiskNode> {
        let directories = Directories::default();
        let excluded_items = ExcludedItems::default();
        let stop_flag = Arc::new(AtomicBool::new(stop));
        let entries_counter = AtomicUsize::new(0);
        let context = ScanContext {
            directories: &directories,
            stop_flag: &stop_flag,
            entries_counter: &entries_counter,
            content_detection: ContentDetection::ExtensionOnly,
            excluded_items: &excluded_items,
            ignore_excluded_items: true,
            ignore_excluded_directories: true,
        };
        index.build_root(Path::new(root), components, &context)
    }

    #[test]
    fn whole_volume_sums_sizes_and_skips_orphans() {
        let node = build(&sample_index(), "C:\\", &[], false).unwrap();
        assert_eq!(node.size, 100 + 10 + 1000 + 20);
        assert_eq!(node.children.len(), 2);
    }

    #[test]
    fn subdirectory_is_found_case_insensitively() {
        let node = build(&sample_index(), "C:\\users", &["users"], false).unwrap();
        assert_eq!(&*node.name, "C:\\users");
        assert_eq!(node.size, 1110);
        assert_eq!(
            node.children.iter().map(|child| &*child.name).collect::<Vec<_>>(),
            ["Deep", "song.MP3", "notes.txt", "link"]
        );
    }

    #[test]
    fn categories_come_from_extension_and_symlinks_have_no_size() {
        let node = build(&sample_index(), "C:\\Users", &["Users"], false).unwrap();
        let by_name = |name: &str| node.children.iter().find(|child| &*child.name == name).unwrap();
        assert_eq!(by_name("song.MP3").category, FileCategory::Audio);
        assert_eq!(by_name("notes.txt").category, FileCategory::Document);
        assert_eq!(by_name("link").size, 0);
    }

    #[test]
    fn single_file_root() {
        let node = build(&sample_index(), "C:\\top.bin", &["top.bin"], false).unwrap();
        assert_eq!(node.size, 20);
        assert_eq!(node.category, FileCategory::Binary);
    }

    #[test]
    fn missing_root_gives_none() {
        assert!(build(&sample_index(), "C:\\Nope", &["Nope"], false).is_none());
        assert!(build(&sample_index(), "C:\\top.bin\\inside", &["top.bin", "inside"], false).is_none());
    }

    #[test]
    fn stopping_returns_what_is_known_without_panicking() {
        let node = build(&sample_index(), "C:\\", &[], true).unwrap();
        assert_eq!(node.size, 0);
    }
}
