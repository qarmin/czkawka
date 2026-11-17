use std::borrow::Cow;
use std::collections::HashMap;
use std::path::Path;

use crate::tools::disk_usage::category::FileCategory;
use crate::tools::disk_usage::layout::Layout;
use crate::tools::disk_usage::tree::{DiskNode, NodeKind};

/// Total size of the files with one extension; an empty extension stands for files without any.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtensionStat {
    pub extension: Box<str>,
    pub size: u64,
    pub count: u64,
    pub category: FileCategory,
}

/// Lowercase extension of a file name; empty when there is none.
pub fn extension_of(name: &str) -> Cow<'_, str> {
    let extension = Path::new(name).extension().and_then(|extension| extension.to_str()).unwrap_or("");
    if extension.bytes().any(|byte| byte.is_ascii_uppercase()) {
        Cow::Owned(extension.to_ascii_lowercase())
    } else {
        Cow::Borrowed(extension)
    }
}

/// Extensions of all files below the roots, biggest first.
pub fn extension_stats(roots: &[DiskNode]) -> Vec<ExtensionStat> {
    let mut stats = HashMap::<Box<str>, ExtensionStat>::new();
    let mut pending = roots.iter().collect::<Vec<_>>();
    while let Some(node) = pending.pop() {
        pending.extend(&node.children);
        if node.kind != NodeKind::File {
            continue;
        }
        let extension = extension_of(&node.name);
        if let Some(stat) = stats.get_mut(&*extension) {
            stat.size = stat.size.saturating_add(node.size);
            stat.count += 1;
        } else {
            let key: Box<str> = extension.into();
            let stat = ExtensionStat {
                extension: key.clone(),
                size: node.size,
                count: 1,
                category: node.category,
            };
            stats.insert(key, stat);
        }
    }
    let mut stats = stats.into_values().collect::<Vec<_>>();
    stats.sort_unstable_by(|left, right| right.size.cmp(&left.size).then_with(|| left.extension.cmp(&right.extension)));
    stats
}

/// For every item of the layout, whether it is a file with the extension; `roots` are the nodes the layout was built from.
pub fn items_with_extension(layout: &Layout, roots: &[DiskNode], extension: &str) -> Vec<bool> {
    let mut nodes = Vec::<Option<&DiskNode>>::with_capacity(layout.items.len());
    for item in &layout.items {
        let node = if item.is_omitted_block() {
            None
        } else {
            match item.parent {
                None => roots.get(item.index_in_parent as usize),
                Some(parent) => nodes
                    .get(parent as usize)
                    .copied()
                    .flatten()
                    .and_then(|parent| parent.children.get(item.index_in_parent as usize)),
            }
        };
        nodes.push(node);
    }
    nodes
        .iter()
        .map(|node| node.is_some_and(|node| node.kind == NodeKind::File && extension_of(&node.name) == extension))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tools::disk_usage::layout::{LayoutKind, LayoutOptions};
    use crate::tools::disk_usage::tree::DiskTree;

    fn sample_roots() -> Vec<DiskNode> {
        vec![DiskNode::directory(
            "/root",
            vec![
                DiskNode::file("a.MP3", 100, FileCategory::Audio),
                DiskNode::file("b.mp3", 50, FileCategory::Audio),
                DiskNode::file("c.txt", 10, FileCategory::Document),
                DiskNode::file("Makefile", 5, FileCategory::Other),
                DiskNode::directory("sub", vec![DiskNode::file("d.txt", 20, FileCategory::Document)]),
            ],
        )]
    }

    #[test]
    fn stats_merge_case_and_sort_by_size() {
        let stats = extension_stats(&sample_roots());
        let summary = stats.iter().map(|stat| (&*stat.extension, stat.size, stat.count)).collect::<Vec<_>>();
        assert_eq!(summary, [("mp3", 150, 2), ("txt", 30, 2), ("", 5, 1)]);
    }

    #[test]
    fn mask_marks_only_files_with_the_extension() {
        let roots = sample_roots();
        let layout = Layout::build(&DiskTree::new(roots.clone()), &LayoutOptions::new(LayoutKind::Treemap));
        let mask = items_with_extension(&layout, &roots, "txt");
        let matching = layout
            .items
            .iter()
            .zip(&mask)
            .filter(|(_, matches)| **matches)
            .map(|(item, _)| item.size)
            .collect::<Vec<_>>();
        assert_eq!(matching.iter().sum::<u64>(), 30);
        assert_eq!(matching.len(), 2);
    }
}
