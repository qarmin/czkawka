use czkawka_core::tools::disk_usage::category::FileCategory;
use czkawka_core::tools::disk_usage::tree::{DiskNode, DiskTree, NodeKind};

/// The list shows only the biggest entries of a directory so that huge directories stay fast on phones.
pub(crate) const MAX_ROWS: usize = 200;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RowKind {
    Directory,
    File,
    Inaccessible,
    /// Everything that did not fit in [`MAX_ROWS`].
    Others,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Row {
    pub name: String,
    pub size: u64,
    /// Share of the size of the directory that holds the entry.
    pub fraction: f32,
    pub kind: RowKind,
    pub category: FileCategory,
    /// Number of entries hidden in an [`RowKind::Others`] row.
    pub hidden_entries: usize,
    /// Address in the tree; `None` for [`RowKind::Others`].
    pub address: Option<Vec<u32>>,
}

fn children_of<'a>(tree: &'a DiskTree, current: &[u32]) -> Option<&'a [DiskNode]> {
    if current.is_empty() {
        Some(&tree.roots)
    } else {
        tree.node(current).map(|node| node.children.as_slice())
    }
}

fn size_of_level(tree: &DiskTree, current: &[u32]) -> u64 {
    if current.is_empty() {
        tree.total_size()
    } else {
        tree.node(current).map_or(0, |node| node.size)
    }
}

fn fraction(size: u64, total: u64) -> f32 {
    if total == 0 { 0.0 } else { (size as f64 / total as f64) as f32 }
}

fn row_kind(node: &DiskNode) -> RowKind {
    match node.kind {
        NodeKind::Directory => RowKind::Directory,
        NodeKind::File | NodeKind::Symlink => RowKind::File,
        NodeKind::Inaccessible => RowKind::Inaccessible,
    }
}

fn child_address(current: &[u32], index: usize) -> Vec<u32> {
    current.iter().copied().chain(std::iter::once(index as u32)).collect()
}

/// Rows for the entries of the directory at `current` (the whole scan when empty), biggest first.
pub(crate) fn build_rows(tree: &DiskTree, current: &[u32]) -> Vec<Row> {
    let Some(children) = children_of(tree, current) else {
        return Vec::new();
    };
    let total = size_of_level(tree, current);

    let mut rows = children
        .iter()
        .enumerate()
        .take(MAX_ROWS)
        .map(|(index, node)| Row {
            name: node.name.to_string(),
            size: node.size,
            fraction: fraction(node.size, total),
            kind: row_kind(node),
            category: node.category,
            hidden_entries: 0,
            address: Some(child_address(current, index)),
        })
        .collect::<Vec<_>>();

    let hidden = children.get(MAX_ROWS..).unwrap_or_default();
    if !hidden.is_empty() {
        let hidden_size = hidden.iter().map(|node| node.size).sum();
        rows.push(Row {
            name: String::new(),
            size: hidden_size,
            fraction: fraction(hidden_size, total),
            kind: RowKind::Others,
            category: FileCategory::Other,
            hidden_entries: hidden.len(),
            address: None,
        });
    }
    rows
}

/// Names of the directories leading to `current`, starting with `all_label` for the whole scan.
pub(crate) fn breadcrumb(tree: &DiskTree, current: &[u32], all_label: &str) -> Vec<String> {
    let names = (1..=current.len()).filter_map(|length| tree.node(&current[..length]).map(|node| node.name.to_string()));
    std::iter::once(all_label.to_string()).chain(names).collect()
}

/// The address of the directory shown after tapping the breadcrumb segment `level` (0 is the whole scan).
pub(crate) fn address_at_level(current: &[u32], level: usize) -> Vec<u32> {
    current.get(..level).unwrap_or(current).to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_fraction(actual: f32, expected: f32) {
        assert!((actual - expected).abs() < 1e-6, "fraction {actual} != {expected}");
    }

    fn sample_tree() -> DiskTree {
        let photos = DiskNode::directory(
            "photos",
            vec![DiskNode::file("a.jpg", 600, FileCategory::Image), DiskNode::file("b.jpg", 200, FileCategory::Image)],
        );
        let root = DiskNode::directory("/data", vec![photos, DiskNode::file("movie.mkv", 200, FileCategory::Video)]);
        DiskTree::new(vec![root])
    }

    #[test]
    fn root_level_lists_roots_with_whole_scan_as_total() {
        let rows = build_rows(&sample_tree(), &[]);

        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].name, "/data");
        assert_eq!(rows[0].size, 1000);
        assert_fraction(rows[0].fraction, 1.0);
        assert_eq!(rows[0].kind, RowKind::Directory);
        assert_eq!(rows[0].address, Some(vec![0]));
    }

    #[test]
    fn directory_level_lists_children_biggest_first_with_fractions() {
        let rows = build_rows(&sample_tree(), &[0]);

        assert_eq!(rows.iter().map(|row| row.name.as_str()).collect::<Vec<_>>(), ["photos", "movie.mkv"]);
        assert_fraction(rows[0].fraction, 0.8);
        assert_fraction(rows[1].fraction, 0.2);
        assert_eq!(rows[0].address, Some(vec![0, 0]));
        assert_eq!(rows[1].kind, RowKind::File);
    }

    #[test]
    fn unknown_address_gives_no_rows() {
        assert!(build_rows(&sample_tree(), &[5, 5]).is_empty());
    }

    #[test]
    fn entries_over_the_limit_are_merged_into_one_others_row() {
        let files = (0..MAX_ROWS + 3).map(|index| DiskNode::file(format!("f{index:04}"), 10, FileCategory::Other)).collect();
        let tree = DiskTree::new(vec![DiskNode::directory("/many", files)]);

        let rows = build_rows(&tree, &[0]);

        assert_eq!(rows.len(), MAX_ROWS + 1);
        let others = rows.last().unwrap();
        assert_eq!(others.kind, RowKind::Others);
        assert_eq!(others.hidden_entries, 3);
        assert_eq!(others.size, 30);
        assert_eq!(others.address, None);
    }

    #[test]
    fn empty_directory_has_zero_fractions_and_no_rows() {
        let tree = DiskTree::new(vec![DiskNode::directory("/empty", Vec::new())]);

        assert!(build_rows(&tree, &[0]).is_empty());
        assert_fraction(build_rows(&tree, &[])[0].fraction, 0.0);
    }

    #[test]
    fn breadcrumb_starts_with_label_and_follows_the_address() {
        let tree = sample_tree();

        assert_eq!(breadcrumb(&tree, &[], "All"), ["All"]);
        assert_eq!(breadcrumb(&tree, &[0, 0], "All"), ["All", "/data", "photos"]);
    }

    #[test]
    fn navigation_helpers_move_up_the_address() {
        assert_eq!(address_at_level(&[0, 1, 2], 0), Vec::<u32>::new());
        assert_eq!(address_at_level(&[0, 1, 2], 2), vec![0, 1]);
        assert_eq!(address_at_level(&[0, 1], 9), vec![0, 1]);
    }
}
