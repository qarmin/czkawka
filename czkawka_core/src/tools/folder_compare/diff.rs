use std::collections::{BTreeMap, HashMap};
use std::path::PathBuf;

use crate::tools::folder_compare::hashing::ContentHash;
use crate::tools::folder_compare::scan::FsNode;
use crate::tools::folder_compare::{DiffNode, DiffStatus, EntryKind};

#[derive(Clone)]
struct SideEntry<'a> {
    path: PathBuf,
    node: &'a FsNode,
}

type Entries<'a> = BTreeMap<&'a str, SideEntry<'a>>;

impl SideEntry<'_> {
    fn file_count(&self) -> u64 {
        if self.node.is_directory() { self.node.file_count } else { 1 }
    }
}

/// Children of all roots merged by name; the first root wins when the same name appears in several.
fn root_entries(roots: &[(PathBuf, FsNode)]) -> Entries<'_> {
    let mut entries = Entries::new();
    for (path, node) in roots {
        add_children(path, node, &mut entries);
    }
    entries
}

fn add_children<'a>(path: &std::path::Path, node: &'a FsNode, entries: &mut Entries<'a>) {
    for child in &node.children {
        entries.entry(&child.name).or_insert_with(|| SideEntry {
            path: path.join(&*child.name),
            node: child,
        });
    }
}

fn children_of<'a>(entry: &SideEntry<'a>) -> Entries<'a> {
    let mut entries = Entries::new();
    add_children(&entry.path, entry.node, &mut entries);
    entries
}

/// Pairs of files with the same relative path and size: the only files whose content has to be read.
pub(super) fn collect_pair_files(left: &[(PathBuf, FsNode)], right: &[(PathBuf, FsNode)]) -> Vec<(PathBuf, u64)> {
    let mut files = Vec::new();
    collect_pairs(&root_entries(left), &root_entries(right), &mut files);
    files
}

fn collect_pairs(left: &Entries, right: &Entries, files: &mut Vec<(PathBuf, u64)>) {
    for (name, left_entry) in left {
        let Some(right_entry) = right.get(name) else { continue };
        match (left_entry.node.kind, right_entry.node.kind) {
            (EntryKind::Directory, EntryKind::Directory) => collect_pairs(&children_of(left_entry), &children_of(right_entry), files),
            (EntryKind::File, EntryKind::File) if left_entry.node.size == right_entry.node.size => {
                files.push((left_entry.path.clone(), left_entry.node.size));
                files.push((right_entry.path.clone(), right_entry.node.size));
            }
            _ => {}
        }
    }
}

pub(super) struct DiffBuilder<'a> {
    pub shared_hashes: &'a HashMap<PathBuf, ContentHash>,
}

impl DiffBuilder<'_> {
    pub fn build(&self, left: &[(PathBuf, FsNode)], right: &[(PathBuf, FsNode)]) -> Vec<DiffNode> {
        self.diff_entries(&root_entries(left), &root_entries(right))
    }

    fn diff_entries(&self, left: &Entries, right: &Entries) -> Vec<DiffNode> {
        let names: BTreeMap<&str, ()> = left.keys().chain(right.keys()).map(|name| (*name, ())).collect();
        names.into_keys().map(|name| self.diff_entry(name, left.get(name), right.get(name))).collect()
    }

    fn diff_entry(&self, name: &str, left: Option<&SideEntry>, right: Option<&SideEntry>) -> DiffNode {
        let mut node = DiffNode {
            name: name.to_string(),
            kind: left.or(right).map_or(EntryKind::File, |entry| entry.node.kind),
            status: DiffStatus::Different,
            left_path: left.map(|entry| entry.path.clone()),
            right_path: right.map(|entry| entry.path.clone()),
            left_size: left.map_or(0, |entry| entry.node.size),
            right_size: right.map_or(0, |entry| entry.node.size),
            left_files: left.map_or(0, SideEntry::file_count),
            right_files: right.map_or(0, SideEntry::file_count),
            children: Vec::new(),
        };
        match (left, right) {
            (Some(_), None) => node.status = DiffStatus::OnlyLeft,
            (None, Some(_)) => node.status = DiffStatus::OnlyRight,
            (Some(left), Some(right)) if left.node.kind != right.node.kind => {}
            (Some(left), Some(right)) if left.node.is_directory() => {
                node.children = self.diff_entries(&children_of(left), &children_of(right));
                node.status = folder_status(&node.children);
                if node.status == DiffStatus::Identical {
                    node.children.clear();
                }
            }
            (Some(left), Some(right)) => node.status = self.leaf_status(left, right),
            (None, None) => {}
        }
        node
    }

    fn leaf_status(&self, left: &SideEntry, right: &SideEntry) -> DiffStatus {
        let identical = match left.node.kind {
            EntryKind::Symlink => std::fs::read_link(&left.path).ok() == std::fs::read_link(&right.path).ok(),
            _ => left.node.size == right.node.size && self.shared_hashes.get(&left.path).is_some_and(|hash| self.shared_hashes.get(&right.path) == Some(hash)),
        };
        if identical { DiffStatus::Identical } else { DiffStatus::Different }
    }
}

fn folder_status(children: &[DiffNode]) -> DiffStatus {
    let has = |statuses: &[DiffStatus]| children.iter().any(|child| statuses.contains(&child.status));
    let has_left = has(&[DiffStatus::OnlyLeft, DiffStatus::LeftHasMore]);
    let has_right = has(&[DiffStatus::OnlyRight, DiffStatus::RightHasMore]);
    let has_different = has(&[DiffStatus::Different]);
    match (has_left, has_right, has_different) {
        (false, false, false) => DiffStatus::Identical,
        (true, false, false) => DiffStatus::LeftHasMore,
        (false, true, false) => DiffStatus::RightHasMore,
        _ => DiffStatus::Different,
    }
}

pub(super) fn count_differences(nodes: &[DiffNode]) -> usize {
    nodes
        .iter()
        .map(|node| {
            let own = match node.status {
                DiffStatus::OnlyLeft | DiffStatus::OnlyRight => 1,
                DiffStatus::Different if node.kind != EntryKind::Directory => 1,
                _ => 0,
            };
            own + count_differences(&node.children)
        })
        .sum()
}
