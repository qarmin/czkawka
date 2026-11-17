use std::cmp::Reverse;
use std::io::Write;

use humansize::{BINARY, format_size};
use serde::Serialize;

use crate::tools::disk_usage::tree::{DiskNode, DiskTree, NodeKind};

/// One entry of the depth- and width-limited view of a [`DiskTree`] used for text and JSON output.
#[derive(Debug, Clone, Serialize)]
pub struct ReportNode {
    pub name: String,
    pub kind: &'static str,
    pub category: &'static str,
    pub size: u64,
    /// Share of the whole scanned size, 0-100.
    pub percent: f64,
    pub children: Vec<Self>,
}

/// Keeps `max_depth` levels (roots are level 1) and the `max_entries` biggest children of every directory.
pub fn build_report(tree: &DiskTree, max_depth: usize, max_entries: usize) -> Vec<ReportNode> {
    let total = tree.total_size();
    tree.roots.iter().map(|root| report_node(root, 1, max_depth, max_entries, total)).collect()
}

fn report_node(node: &DiskNode, depth: usize, max_depth: usize, max_entries: usize, total: u64) -> ReportNode {
    let children = if depth < max_depth {
        let mut biggest = node.children.iter().collect::<Vec<_>>();
        biggest.sort_unstable_by_key(|child| Reverse(child.size));
        biggest
            .into_iter()
            .take(max_entries)
            .map(|child| report_node(child, depth + 1, max_depth, max_entries, total))
            .collect()
    } else {
        Vec::new()
    };
    ReportNode {
        name: node.name.to_string(),
        kind: kind_name(node.kind),
        category: node.category.name(),
        size: node.size,
        percent: percent_of(node.size, total),
        children,
    }
}

fn percent_of(size: u64, total: u64) -> f64 {
    if total == 0 { 0.0 } else { size as f64 * 100.0 / total as f64 }
}

const fn kind_name(kind: NodeKind) -> &'static str {
    match kind {
        NodeKind::File => "file",
        NodeKind::Directory => "directory",
        NodeKind::Symlink => "symlink",
        NodeKind::Inaccessible => "inaccessible",
    }
}

pub fn write_report<T: Write>(writer: &mut T, report: &[ReportNode]) -> std::io::Result<()> {
    report.iter().try_for_each(|node| write_report_node(writer, node, 0))
}

fn write_report_node<T: Write>(writer: &mut T, node: &ReportNode, indent: usize) -> std::io::Result<()> {
    let marker = if node.kind == "directory" { "/" } else { "" };
    writeln!(
        writer,
        "{:indent$}{:>10} {:>5.1}%  {}{marker}",
        "",
        format_size(node.size, BINARY),
        node.percent,
        node.name,
        indent = indent * 2
    )?;
    node.children.iter().try_for_each(|child| write_report_node(writer, child, indent + 1))
}
