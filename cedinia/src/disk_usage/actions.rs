use std::path::Path;
use std::sync::Arc;

use czkawka_core::tools::disk_usage::tree::DiskNode;
use slint::{ComponentHandle, SharedString};

use crate::disk_usage::list_model::RowKind;
use crate::disk_usage::{VIEW, refresh};
use crate::{AppState, DiskUsageState, MainWindow, flc};

fn count_files(node: &DiskNode) -> usize {
    if node.is_directory() { node.children.iter().map(count_files).sum() } else { 1 }
}

fn file_name_of(path: &std::path::Path) -> String {
    path.file_name()
        .map_or_else(|| path.to_string_lossy().into_owned(), |name| name.to_string_lossy().into_owned())
}

pub(super) fn show_menu(win: &MainWindow, index: i32) {
    let target = VIEW.with_borrow_mut(|view| {
        let row = view
            .rows
            .get(index as usize)
            .unwrap_or_else(|| panic!("show_menu: index {index} out of bounds (rows={})", view.rows.len()));
        let address = row.address.clone().filter(|_| matches!(row.kind, RowKind::Directory | RowKind::File))?;
        let path = view.tree.as_ref()?.path(&address)?;
        view.menu_target = Some(address);
        Some((path, row.kind == RowKind::Directory))
    });
    let Some((path, is_directory)) = target else {
        return;
    };

    let name = file_name_of(&path);
    let state = win.global::<DiskUsageState>();
    state.set_menu_parent(path.parent().map(|parent| parent.to_string_lossy().into_owned()).unwrap_or_default().into());
    state.set_confirm_message(flc!("disk_usage_confirm_delete", name = name.as_str()).into());
    state.set_menu_name(name.into());
    state.set_menu_path(path.to_string_lossy().as_ref().into());
    state.set_menu_is_directory(is_directory);
    state.set_menu_visible(true);
}

pub(super) fn delete_target(win: &MainWindow) {
    let target = VIEW.with_borrow_mut(|view| {
        let address = view.menu_target.take()?;
        let path = view.tree.as_ref()?.path(&address)?;
        Some((address, path))
    });
    let Some((address, path)) = target else {
        return;
    };

    let weak = win.as_weak();
    std::thread::spawn(move || {
        let result = crate::file_actions::delete_path(&path.to_string_lossy());
        if let Err(error) = weak.upgrade_in_event_loop(move |win| finish_delete(&win, &address, &path, result)) {
            log::error!("Disk usage: window gone before the delete result could be shown: {error}");
        }
    });
}

fn finish_delete(win: &MainWindow, address: &[u32], path: &Path, result: Result<(), String>) {
    let name = file_name_of(path);
    let message = match result {
        Ok(()) => {
            remove_from_view(address, path);
            refresh(win);
            flc!("disk_usage_deleted", name = name.as_str())
        }
        Err(error) => {
            log::error!("Disk usage: failed to delete {}: {error}", path.display());
            flc!("disk_usage_delete_failed", name = name.as_str(), error = error.as_str())
        }
    };
    win.global::<AppState>().set_status_message(SharedString::from(message));
}

fn remove_from_view(address: &[u32], path: &Path) {
    VIEW.with_borrow_mut(|view| {
        let Some(tree) = view.tree.as_mut() else {
            return;
        };
        if tree.path(address).as_deref() != Some(path) {
            return;
        }
        if let Some(removed) = Arc::make_mut(tree).remove(address) {
            view.info.number_of_files = view.info.number_of_files.saturating_sub(count_files(&removed));
        }
    });
}
