use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::Ordering;

use log::{error, info};

use crate::MainWindow;
use crate::connect_row_selection::clipboard::set_clipboard;
use crate::disk_usage::view::{ChartContext, refresh_after_zoom, selected_address};

fn path_of(context: &ChartContext, address: &[u32]) -> Option<PathBuf> {
    let models = context.shared_models.lock().expect("Mutex poisoned");
    models.shared_disk_usage_state.as_ref()?.get_tree().path(address)
}

fn selected_path(context: &ChartContext) -> Option<PathBuf> {
    path_of(context, &selected_address(context)?)
}

pub(super) fn open_selected(context: &ChartContext, parent: bool) {
    if let Some(address) = selected_address(context) {
        open_address(context, &address, parent);
    }
}

pub(super) fn open_address(context: &ChartContext, address: &[u32], parent: bool) {
    let Some(path) = path_of(context, address) else {
        return;
    };
    let target = if parent { path.parent().map_or(path.clone(), Path::to_path_buf) } else { path };
    if let Err(e) = open::that(&target) {
        error!("Failed to open \"{}\": {e}", target.to_string_lossy());
    }
}

pub(super) fn copy_selected_path(context: &ChartContext) {
    if let Some(path) = selected_path(context) {
        set_clipboard(path.to_string_lossy().into_owned());
    }
}

pub(super) fn move_selected_to_trash(app: &MainWindow, context: &Arc<ChartContext>) {
    let Some(address) = selected_address(context) else {
        return;
    };
    {
        let mut models = context.shared_models.lock().expect("Mutex poisoned");
        let Some(tool) = models.shared_disk_usage_state.as_mut() else {
            return;
        };
        let Some(path) = tool.get_tree().path(&address) else {
            return;
        };
        // TODO: really move the item to the trash; for now it is only removed from the chart
        info!("Moving \"{}\" to the trash (not implemented yet, hidden from the chart only)", path.to_string_lossy());
        tool.get_tree_mut().remove(&address);
    }
    context.tree_version.fetch_add(1, Ordering::Relaxed);
    {
        let mut view = context.view.lock().expect("View mutex poisoned");
        shift_address_after_removal(&mut view.zoom, &address);
        view.selected = None;
    }
    refresh_after_zoom(app, context);
}

/// Keeps `address` pointing at the same node after the node at `removed` was cut out of the tree.
/// If `address` was inside the removed node, it moves up to the parent of the removed node.
fn shift_address_after_removal(address: &mut Vec<u32>, removed: &[u32]) {
    let Some((&removed_index, removed_parent)) = removed.split_last() else {
        return;
    };
    let Some(&at_level) = address.get(removed_parent.len()) else {
        return;
    };
    if !address.starts_with(removed_parent) {
        return;
    }
    if at_level == removed_index {
        address.truncate(removed_parent.len());
    } else if at_level > removed_index
        && let Some(slot) = address.get_mut(removed_parent.len())
    {
        *slot -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::shift_address_after_removal;

    fn shifted(address: &[u32], removed: &[u32]) -> Vec<u32> {
        let mut address = address.to_vec();
        shift_address_after_removal(&mut address, removed);
        address
    }

    #[test]
    fn later_sibling_moves_down_by_one() {
        assert_eq!(shifted(&[0, 3, 1], &[0, 1]), [0, 2, 1]);
    }

    #[test]
    fn earlier_sibling_stays() {
        assert_eq!(shifted(&[0, 0, 5], &[0, 1]), [0, 0, 5]);
    }

    #[test]
    fn address_inside_removed_node_moves_to_its_parent() {
        assert_eq!(shifted(&[0, 1, 4, 2], &[0, 1]), [0]);
    }

    #[test]
    fn unrelated_branch_stays() {
        assert_eq!(shifted(&[1, 5], &[0, 1]), [1, 5]);
    }

    #[test]
    fn removing_root_shifts_later_roots() {
        assert_eq!(shifted(&[2, 1], &[0]), [1, 1]);
    }
}
