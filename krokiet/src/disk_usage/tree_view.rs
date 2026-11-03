use std::collections::HashSet;
use std::sync::Arc;

use czkawka_core::tools::disk_usage::tree::DiskNode;
use humansize::{BINARY, format_size};
use slint::{ComponentHandle, Model, ModelRc, VecModel};

use crate::disk_usage::view::{ChartContext, describe_address, request_render};
use crate::disk_usage::{actions, describe};
use crate::{DiskUsageState, DiskUsageTreeRow, MainWindow};

const MAX_ROWS_PER_DIRECTORY: usize = 300;
const DIRECTORY_BRIGHTNESS: f32 = 0.62;

struct RowBuilder<'a> {
    expanded: &'a HashSet<Vec<u32>>,
    selected: Option<&'a [u32]>,
    rows: Vec<DiskUsageTreeRow>,
    addresses: Vec<Vec<u32>>,
}

impl RowBuilder<'_> {
    fn add(&mut self, node: &DiskNode, address: &[u32], depth: usize, parent_size: u64) {
        let expandable = node.is_directory() && !node.children.is_empty();
        let expanded = expandable && self.expanded.contains(address);
        self.rows.push(DiskUsageTreeRow {
            name: node.name.to_string().into(),
            size_text: format_size(node.size, BINARY).into(),
            percent: if parent_size == 0 {
                0.0
            } else {
                (node.size as f64 * 100.0 / parent_size as f64) as f32
            },
            depth: depth as i32,
            expandable,
            expanded,
            selected: self.selected == Some(address),
            color: row_color(node),
        });
        self.addresses.push(address.to_vec());
        if expanded {
            for (index, child) in node.children.iter().enumerate().take(MAX_ROWS_PER_DIRECTORY) {
                let mut child_address = address.to_vec();
                child_address.push(index as u32);
                self.add(child, &child_address, depth + 1, node.size);
            }
        }
    }
}

fn row_color(node: &DiskNode) -> slint::Color {
    let factor = if node.is_directory() { DIRECTORY_BRIGHTNESS } else { 1.0 };
    let [red, green, blue] = node.category.color().map(|channel| (f32::from(channel) * factor) as u8);
    slint::Color::from_rgb_u8(red, green, blue)
}

/// Opens only the first level: the roots of the scan, or the directory the chart is zoomed into.
pub(super) fn reset_expanded(context: &ChartContext) {
    let roots = context
        .shared_models
        .lock()
        .expect("Mutex poisoned")
        .shared_disk_usage_state
        .as_ref()
        .map_or(0, |tool| tool.get_tree().roots.len());
    let mut view = context.view.lock().expect("View mutex poisoned");
    view.expanded.clear();
    if view.zoom.is_empty() {
        let root_addresses = (0..roots).map(|index| vec![index as u32]).collect::<Vec<_>>();
        view.expanded.extend(root_addresses);
    } else {
        let zoom = view.zoom.clone();
        view.expanded.insert(zoom);
    }
}

/// Opens every directory above the selected item, so it is visible in the list.
pub(super) fn expand_ancestors_of_selection(context: &ChartContext) {
    let mut view = context.view.lock().expect("View mutex poisoned");
    let Some(selected) = view.selected.clone() else {
        return;
    };
    for length in 1..selected.len() {
        if let Some(ancestor) = selected.get(..length) {
            view.expanded.insert(ancestor.to_vec());
        }
    }
}

pub(super) fn rebuild(app: &MainWindow, context: &ChartContext) {
    let (zoom, expanded, selected) = {
        let view = context.view.lock().expect("View mutex poisoned");
        (view.zoom.clone(), view.expanded.clone(), view.selected.clone())
    };
    let mut builder = RowBuilder {
        expanded: &expanded,
        selected: selected.as_deref(),
        rows: Vec::new(),
        addresses: Vec::new(),
    };
    {
        let models = context.shared_models.lock().expect("Mutex poisoned");
        if let Some(tool) = models.shared_disk_usage_state.as_ref() {
            let tree = tool.get_tree();
            if zoom.is_empty() {
                let total = tree.total_size();
                for (index, root) in tree.roots.iter().enumerate() {
                    builder.add(root, &[index as u32], 0, total);
                }
            } else if let Some(node) = tree.node(&zoom) {
                builder.add(node, &zoom, 0, node.size);
            }
        }
    }

    let selected_row = selected.as_ref().and_then(|selected| builder.addresses.iter().position(|address| address == selected));
    let RowBuilder { rows, addresses, .. } = builder;
    context.view.lock().expect("View mutex poisoned").tree_addresses = addresses;
    let state = app.global::<DiskUsageState>();
    state.set_tree_rows(ModelRc::new(VecModel::from(rows)));
    state.set_tree_selected_row(selected_row.map_or(-1, |row| row as i32));
}

fn address_of_row(context: &ChartContext, row: i32) -> Option<Vec<u32>> {
    let row = usize::try_from(row).ok()?;
    context.view.lock().expect("View mutex poisoned").tree_addresses.get(row).cloned()
}

pub(super) fn toggle_row(app: &MainWindow, context: &ChartContext, row: i32) {
    let Some(address) = address_of_row(context, row) else {
        return;
    };
    {
        let mut view = context.view.lock().expect("View mutex poisoned");
        if !view.expanded.remove(&address) {
            view.expanded.insert(address);
        }
    }
    rebuild(app, context);
}

pub(super) fn select_row(app: &MainWindow, context: &Arc<ChartContext>, row: i32) {
    let Some(address) = address_of_row(context, row) else {
        return;
    };
    let info = describe_address(context, &address);
    context.view.lock().expect("View mutex poisoned").selected = Some(address);
    describe::show_selection(app, info.as_ref());
    mark_selected_rows(app, context);
    request_render(app, context);
}

/// Updates the highlight in place; replacing the whole model would drop the row under the cursor in the middle of a double click.
fn mark_selected_rows(app: &MainWindow, context: &ChartContext) {
    let view = context.view.lock().expect("View mutex poisoned");
    let state = app.global::<DiskUsageState>();
    let rows = state.get_tree_rows();
    for (index, address) in view.tree_addresses.iter().enumerate() {
        let selected = view.selected.as_ref() == Some(address);
        if let Some(mut row) = rows.row_data(index)
            && row.selected != selected
        {
            row.selected = selected;
            rows.set_row_data(index, row);
        }
    }
    let selected_row = view.tree_addresses.iter().position(|address| view.selected.as_ref() == Some(address));
    state.set_tree_selected_row(selected_row.map_or(-1, |row| row as i32));
}

/// Opens the file or directory of the row in the system.
pub(super) fn activate_row(context: &ChartContext, row: i32) {
    if let Some(address) = address_of_row(context, row) {
        actions::open_address(context, &address, false);
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use czkawka_core::tools::disk_usage::category::FileCategory;
    use czkawka_core::tools::disk_usage::tree::DiskNode;

    use super::RowBuilder;

    fn sample_root() -> DiskNode {
        let music = DiskNode::directory(
            "music",
            vec![DiskNode::file("a.mp3", 300, FileCategory::Audio), DiskNode::file("b.mp3", 100, FileCategory::Audio)],
        );
        DiskNode::directory("/root", vec![music, DiskNode::file("c.bin", 100, FileCategory::Binary)])
    }

    fn build(expanded: &[&[u32]], selected: Option<&[u32]>) -> RowBuilder<'static> {
        let expanded = Box::leak(Box::new(expanded.iter().map(|address| address.to_vec()).collect::<HashSet<_>>()));
        let mut builder = RowBuilder {
            expanded,
            selected: selected.map(|address| &*Box::leak(address.to_vec().into_boxed_slice())),
            rows: Vec::new(),
            addresses: Vec::new(),
        };
        let root = sample_root();
        builder.add(&root, &[0], 0, root.size);
        builder
    }

    #[test]
    fn collapsed_root_is_a_single_row() {
        let builder = build(&[], None);

        assert_eq!(builder.rows.len(), 1);
        assert!(builder.rows[0].expandable);
        assert!(!builder.rows[0].expanded);
    }

    #[test]
    fn expanded_directories_list_their_children_with_depth_and_share_of_parent() {
        let builder = build(&[&[0], &[0, 0]], None);

        let names = builder.rows.iter().map(|row| row.name.to_string()).collect::<Vec<_>>();
        assert_eq!(names, ["/root", "music", "a.mp3", "b.mp3", "c.bin"]);
        assert_eq!(builder.rows.iter().map(|row| row.depth).collect::<Vec<_>>(), [0, 1, 2, 2, 1]);
        assert!((builder.rows[2].percent - 75.0).abs() < 1e-4, "300 of 400 bytes in music");
        assert!((builder.rows[1].percent - 80.0).abs() < 1e-4, "400 of 500 bytes in root");
        assert!(!builder.rows[4].expandable, "files cannot be expanded");
    }

    #[test]
    fn addresses_follow_rows_and_selection_marks_one_row() {
        let builder = build(&[&[0], &[0, 0]], Some(&[0, 0, 1]));

        assert_eq!(builder.addresses[3], [0, 0, 1]);
        assert_eq!(builder.rows.iter().filter(|row| row.selected).count(), 1);
        assert!(builder.rows[3].selected);
    }
}
