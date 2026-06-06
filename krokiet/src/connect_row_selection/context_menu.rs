use std::path::MAIN_SEPARATOR;

use slint::{ComponentHandle, Model, ModelRc, VecModel};

use super::clipboard::set_clipboard;
use super::reset_selection;
use crate::connect_directories_changes::add_excluded_paths;
use crate::connect_row_selection::checker::set_number_of_enabled_items;
use crate::model_operations::remove_single_items_in_groups;
use crate::{Callabler, GuiState, MainWindow, Settings, SingleMainListModel};

pub(crate) fn connect_context_menu_actions(app: &MainWindow) {
    connect_remove_from_results(app);
    connect_remove_all_from_folder(app);
    connect_remove_all_from_folder_recursive(app);
    connect_select_all_from_folder(app);
    connect_select_all_from_folder_recursive(app);
    connect_exclude_parent_folder(app);
    connect_exclude_item(app);
    connect_copy_file_name(app);
    connect_copy_parent_folder_path(app);
    connect_copy_full_path(app);
    connect_rename_item(app);
}

fn connect_remove_from_results(app: &MainWindow) {
    let a = app.as_weak();
    app.global::<Callabler>().on_row_remove_from_results(move |idx| {
        let app = a.upgrade().expect("Failed to upgrade app");
        let active_tab = app.global::<GuiState>().get_active_tab();
        let model = active_tab.get_tool_model(&app);
        let idx = idx as usize;

        let row = model
            .row_data(idx)
            .unwrap_or_else(|| panic!("Row idx={idx} out of bounds (row_count={})", model.row_count()));
        if row.header_row {
            return;
        }

        let new_items: Vec<SingleMainListModel> = model.iter().enumerate().filter_map(|(i, r)| if i == idx { None } else { Some(r) }).collect();
        let cleaned = remove_single_items_in_groups(new_items, active_tab.get_is_header_mode());
        active_tab.set_tool_model(&app, ModelRc::new(VecModel::from(cleaned)));
        reset_selection(&app, active_tab, true);
    });
}

fn connect_remove_all_from_folder(app: &MainWindow) {
    let a = app.as_weak();
    app.global::<Callabler>().on_row_remove_all_from_folder(move |idx| {
        remove_all_from_folder_impl(&a.upgrade().expect("Failed to upgrade app"), idx as usize, false);
    });
}

fn connect_remove_all_from_folder_recursive(app: &MainWindow) {
    let a = app.as_weak();
    app.global::<Callabler>().on_row_remove_all_from_folder_recursive(move |idx| {
        remove_all_from_folder_impl(&a.upgrade().expect("Failed to upgrade app"), idx as usize, true);
    });
}

fn remove_all_from_folder_impl(app: &MainWindow, idx: usize, recursive: bool) {
    let active_tab = app.global::<GuiState>().get_active_tab();
    let model = active_tab.get_tool_model(app);
    let path_idx = active_tab.get_str_path_idx();

    let clicked_row = model
        .row_data(idx)
        .unwrap_or_else(|| panic!("Row idx={idx} out of bounds (row_count={})", model.row_count()));
    if clicked_row.header_row {
        return;
    }
    let target_path = clicked_row.val_str.iter().nth(path_idx).map(|s| s.to_string()).unwrap_or_default();
    let target_prefix = format!("{target_path}{}", std::path::MAIN_SEPARATOR);

    let mut in_reference_group = false;
    let new_items: Vec<SingleMainListModel> = model
        .iter()
        .filter(|row| {
            if row.header_row {
                in_reference_group = row.filled_header_row;
                true
            } else if in_reference_group {
                true // never remove items from a reference-folder group
            } else {
                let p = row.val_str.iter().nth(path_idx).map(|s| s.as_str().to_owned()).unwrap_or_default();
                if recursive {
                    p != target_path && !p.starts_with(&target_prefix)
                } else {
                    p != target_path
                }
            }
        })
        .collect();
    let cleaned = remove_single_items_in_groups(new_items, active_tab.get_is_header_mode());
    active_tab.set_tool_model(app, ModelRc::new(VecModel::from(cleaned)));
    reset_selection(app, active_tab, true);
}

fn connect_select_all_from_folder(app: &MainWindow) {
    let a = app.as_weak();
    app.global::<Callabler>().on_row_select_all_from_folder(move |idx| {
        select_all_from_folder_impl(&a.upgrade().expect("Failed to upgrade app"), idx as usize, false);
    });
}

fn connect_select_all_from_folder_recursive(app: &MainWindow) {
    let a = app.as_weak();
    app.global::<Callabler>().on_row_select_all_from_folder_recursive(move |idx| {
        select_all_from_folder_impl(&a.upgrade().expect("Failed to upgrade app"), idx as usize, true);
    });
}

fn select_all_from_folder_impl(app: &MainWindow, idx: usize, recursive: bool) {
    let active_tab = app.global::<GuiState>().get_active_tab();
    let model = active_tab.get_tool_model(app);
    let path_idx = active_tab.get_str_path_idx();
    let is_header_mode = active_tab.get_is_header_mode();

    let clicked_row = model
        .row_data(idx)
        .unwrap_or_else(|| panic!("Row idx={idx} out of bounds (row_count={})", model.row_count()));
    if clicked_row.header_row {
        return;
    }
    let target_path = clicked_row.val_str.iter().nth(path_idx).map(|s| s.to_string()).unwrap_or_default();
    let target_prefix = format!("{target_path}{}", std::path::MAIN_SEPARATOR);

    let items: Vec<SingleMainListModel> = model.iter().collect();
    let n = items.len();

    let path_matches = |i: usize| -> bool {
        if items[i].header_row {
            return false;
        }
        let p = items[i].val_str.iter().nth(path_idx).map(|s| s.as_str().to_owned()).unwrap_or_default();
        if recursive {
            p == target_path || p.starts_with(&target_prefix)
        } else {
            p == target_path
        }
    };

    // should_check[i] = true  → set checked=true for that item (never uncheck)
    let mut should_check = vec![false; n];

    if !is_header_mode {
        // Flat list: simply check all matching items.
        for i in 0..n {
            if path_matches(i) && !items[i].checked {
                should_check[i] = true;
            }
        }
    } else {
        // Group-aware: iterate group by group.
        let mut i = 0;
        while i < n {
            if !items[i].header_row {
                i += 1;
                continue;
            }
            let is_reference = items[i].filled_header_row;
            i += 1; // skip the header row

            let group_start = i;
            while i < n && !items[i].header_row {
                i += 1;
            }
            let group_end = i; // exclusive
            let total_in_group = group_end - group_start;
            if total_in_group == 0 {
                continue;
            }

            let currently_checked = (group_start..group_end).filter(|&j| items[j].checked).count();
            let matching_unchecked: Vec<usize> = (group_start..group_end).filter(|&j| path_matches(j) && !items[j].checked).collect();
            let would_be_checked = currently_checked + matching_unchecked.len();

            if is_reference {
                // The reference header itself is permanently unchecked, so there is always
                // at least one "uncheckable" item in the group → no restriction needed.
                for j in matching_unchecked {
                    should_check[j] = true;
                }
            } else if would_be_checked == total_in_group && currently_checked < total_in_group {
                // Checking all matching items would make the whole group checked, and the
                // group is not already full → check all but skip the last matching unchecked
                // so one item remains unchecked.
                let count = matching_unchecked.len();
                if count > 1 {
                    for &j in &matching_unchecked[..count - 1] {
                        should_check[j] = true;
                    }
                }
                // count == 1: checking that single item would fill the group → skip it.
                // count == 0: nothing to check (impossible to reach here, but harmless).
            } else {
                // Either the group won't be fully checked, or it is already fully checked
                // (currently_checked == total_in_group → matching_unchecked is empty).
                // In both cases just check all matching unchecked items.
                for j in matching_unchecked {
                    should_check[j] = true;
                }
            }
        }
    }

    let new_items: Vec<SingleMainListModel> = items
        .into_iter()
        .enumerate()
        .map(|(i, mut row)| {
            if should_check[i] {
                row.checked = true;
            }
            row.focused_row = false; // keep in sync with TOOLS_SELECTION reset below
            row
        })
        .collect();

    let checked_count = new_items.iter().filter(|r| r.checked).count() as u64;
    active_tab.set_tool_model(app, ModelRc::new(VecModel::from(new_items)));
    reset_selection(app, active_tab, true);
    set_number_of_enabled_items(app, active_tab, checked_count);
}

fn connect_exclude_parent_folder(app: &MainWindow) {
    let a = app.as_weak();
    app.global::<Callabler>().on_row_exclude_parent_folder(move |idx| {
        let app = a.upgrade().expect("Failed to upgrade app");
        let active_tab = app.global::<GuiState>().get_active_tab();
        let model = active_tab.get_tool_model(&app);
        let path_idx = active_tab.get_str_path_idx();

        let row = model
            .row_data(idx as usize)
            .unwrap_or_else(|| panic!("Row idx={idx} out of bounds (row_count={})", model.row_count()));
        if row.header_row {
            return;
        }
        let path = row
            .val_str
            .iter()
            .nth(path_idx)
            .unwrap_or_else(|| panic!("path_idx={path_idx} out of bounds, full val_str={:?}", row.val_str.iter().collect::<Vec<_>>()))
            .to_string();
        add_excluded_paths(&app.global::<Settings>(), std::slice::from_ref(&path));

        // Also remove matching rows from results, keeping reference-group items.
        let mut in_reference_group = false;
        let new_items: Vec<SingleMainListModel> = model
            .iter()
            .filter(|r| {
                if r.header_row {
                    in_reference_group = r.filled_header_row;
                    true
                } else if in_reference_group {
                    true
                } else {
                    r.val_str.iter().nth(path_idx).is_none_or(|p| p.as_str() != path)
                }
            })
            .collect();
        let cleaned = remove_single_items_in_groups(new_items, active_tab.get_is_header_mode());
        active_tab.set_tool_model(&app, ModelRc::new(VecModel::from(cleaned)));
        reset_selection(&app, active_tab, true);
    });
}

fn connect_exclude_item(app: &MainWindow) {
    let a = app.as_weak();
    app.global::<Callabler>().on_row_exclude_item(move |idx| {
        let app = a.upgrade().expect("Failed to upgrade app");
        let active_tab = app.global::<GuiState>().get_active_tab();
        let model = active_tab.get_tool_model(&app);
        let path_idx = active_tab.get_str_path_idx();
        let name_idx = active_tab.get_str_name_idx();
        let idx = idx as usize;

        let row = model
            .row_data(idx)
            .unwrap_or_else(|| panic!("Row idx={idx} out of bounds (row_count={})", model.row_count()));
        if row.header_row {
            return;
        }
        let path = row
            .val_str
            .iter()
            .nth(path_idx)
            .unwrap_or_else(|| panic!("path_idx={path_idx} out of bounds, full val_str={:?}", row.val_str.iter().collect::<Vec<_>>()))
            .to_string();
        let name = row
            .val_str
            .iter()
            .nth(name_idx)
            .unwrap_or_else(|| panic!("name_idx={name_idx} out of bounds, full val_str={:?}", row.val_str.iter().collect::<Vec<_>>()))
            .to_string();
        let full_path = std::path::PathBuf::from(&path).join(&name).to_string_lossy().to_string();
        add_excluded_paths(&app.global::<Settings>(), &[full_path]);

        // Remove the specific item from results.
        let new_items: Vec<SingleMainListModel> = model.iter().enumerate().filter_map(|(i, r)| if i == idx { None } else { Some(r) }).collect();
        let cleaned = remove_single_items_in_groups(new_items, active_tab.get_is_header_mode());
        active_tab.set_tool_model(&app, ModelRc::new(VecModel::from(cleaned)));
        reset_selection(&app, active_tab, true);
    });
}

fn connect_copy_file_name(app: &MainWindow) {
    let a = app.as_weak();
    app.global::<Callabler>().on_row_copy_file_name(move |idx| {
        let app = a.upgrade().expect("Failed to upgrade app");
        let active_tab = app.global::<GuiState>().get_active_tab();
        let model = active_tab.get_tool_model(&app);
        let name_idx = active_tab.get_str_name_idx();

        let row = model
            .row_data(idx as usize)
            .unwrap_or_else(|| panic!("Row idx={idx} out of bounds (row_count={})", model.row_count()));
        if row.header_row {
            return;
        }
        let name = row
            .val_str
            .iter()
            .nth(name_idx)
            .unwrap_or_else(|| panic!("name_idx={name_idx} out of bounds, full val_str={:?}", row.val_str.iter().collect::<Vec<_>>()))
            .to_string();
        set_clipboard(name);
    });
}

fn connect_copy_parent_folder_path(app: &MainWindow) {
    let a = app.as_weak();
    app.global::<Callabler>().on_row_copy_parent_folder_path(move |idx| {
        let app = a.upgrade().expect("Failed to upgrade app");
        let active_tab = app.global::<GuiState>().get_active_tab();
        let model = active_tab.get_tool_model(&app);
        let path_idx = active_tab.get_str_path_idx();

        let row = model
            .row_data(idx as usize)
            .unwrap_or_else(|| panic!("Row idx={idx} out of bounds (row_count={})", model.row_count()));
        if row.header_row {
            return;
        }
        let path = row
            .val_str
            .iter()
            .nth(path_idx)
            .unwrap_or_else(|| panic!("path_idx={path_idx} out of bounds, full val_str={:?}", row.val_str.iter().collect::<Vec<_>>()))
            .to_string();
        set_clipboard(path);
    });
}

fn connect_copy_full_path(app: &MainWindow) {
    let a = app.as_weak();
    app.global::<Callabler>().on_row_copy_full_path(move |idx| {
        let app = a.upgrade().expect("Failed to upgrade app");
        let active_tab = app.global::<GuiState>().get_active_tab();
        let model = active_tab.get_tool_model(&app);
        let name_idx = active_tab.get_str_name_idx();
        let path_idx = active_tab.get_str_path_idx();

        let row = model
            .row_data(idx as usize)
            .unwrap_or_else(|| panic!("Row idx={idx} out of bounds (row_count={})", model.row_count()));
        if row.header_row {
            return;
        }
        let name = row
            .val_str
            .iter()
            .nth(name_idx)
            .unwrap_or_else(|| panic!("name_idx={name_idx} out of bounds, full val_str={:?}", row.val_str.iter().collect::<Vec<_>>()))
            .to_string();
        let path = row
            .val_str
            .iter()
            .nth(path_idx)
            .unwrap_or_else(|| panic!("path_idx={path_idx} out of bounds, full val_str={:?}", row.val_str.iter().collect::<Vec<_>>()))
            .to_string();
        let full_path = if path.is_empty() { name } else { format!("{path}{MAIN_SEPARATOR}{name}") };
        set_clipboard(full_path);
    });
}

fn connect_rename_item(app: &MainWindow) {
    let a = app.as_weak();
    app.global::<Callabler>().on_row_rename_item(move |idx| {
        let app = a.upgrade().expect("Failed to upgrade app");
        let active_tab = app.global::<GuiState>().get_active_tab();
        let model = active_tab.get_tool_model(&app);
        let name_idx = active_tab.get_str_name_idx();

        let row = model
            .row_data(idx as usize)
            .unwrap_or_else(|| panic!("Row idx={idx} out of bounds (row_count={})", model.row_count()));
        if row.header_row {
            return;
        }
        let name = row.val_str.iter().nth(name_idx).map(|s| s.to_string()).unwrap_or_default();
        app.invoke_show_rename_single_file_popup(idx, name.into());
    });
}
