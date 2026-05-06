pub(crate) mod custom_select;

use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

use log::error;
use regex::Regex;
use slint::{ComponentHandle, Model, ModelRc, SharedString, VecModel};

use crate::common::{connect_i32_into_u64, create_model_from_model_vec};
use crate::connect_row_selection::checker::change_number_of_enabled_items;
use crate::connect_translation::translate_select_mode;
use crate::settings::model::{SavedCustomSelectColumnState, SavedCustomSelectTabState};
use crate::settings::{get_custom_select_state_file, load_data_from_file, save_data_to_file};
use crate::shared_models::SharedModels;
use crate::{ActiveTab, Callabler, CustomSelectColumnModel, GuiState, MainWindow, SelectMode, SelectModel, Settings, SingleMainListModel};

type SelectionResult = (u64, u64, ModelRc<SingleMainListModel>);

// TODO optimize this, not sure if it is possible to not copy entire model to just select item
// https://github.com/slint-ui/slint/discussions/4595
pub(crate) fn connect_select(app: &MainWindow, shared_models: &Arc<Mutex<SharedModels>>) {
    set_select_buttons(app);

    let a = app.as_weak();
    app.global::<Callabler>().on_update_select_buttons(move || {
        let app = a.upgrade().expect("Failed to upgrade app :(");
        set_select_buttons(&app);
    });

    let shared_models = shared_models.clone();
    let a = app.as_weak();
    app.global::<Callabler>().on_select_items(move |select_mode| {
        let app = a.upgrade().expect("Failed to upgrade app :(");
        let active_tab = app.global::<GuiState>().get_active_tab();
        let current_model = active_tab.get_tool_model(&app);

        let (checked_items, unchecked_items, new_model) = match select_mode {
            SelectMode::SelectAll => select_all(&current_model),
            SelectMode::UnselectAll => deselect_all(&current_model),
            SelectMode::InvertSelection => invert_selection(&current_model),
            SelectMode::InvertSelectionInGroup => invert_selection_in_group(&current_model),
            SelectMode::SelectTheBiggestSize => select_by_property(&current_model, active_tab, Property::Size, true),
            SelectMode::SelectTheSmallestSize => select_by_property(&current_model, active_tab, Property::Size, false),
            SelectMode::SelectTheBiggestResolution => select_by_property(&current_model, active_tab, Property::Resolution, false),
            SelectMode::SelectTheSmallestResolution => select_by_property(&current_model, active_tab, Property::Resolution, true),
            SelectMode::SelectNewest => select_by_property(&current_model, active_tab, Property::Date, true),
            SelectMode::SelectOldest => select_by_property(&current_model, active_tab, Property::Date, false),
            SelectMode::SelectShortestPath => select_by_property(&current_model, active_tab, Property::PathLength, false),
            SelectMode::SelectLongestPath => select_by_property(&current_model, active_tab, Property::PathLength, true),
            SelectMode::SelectAllExceptBiggestSize => select_all_except_by_property(&current_model, active_tab, Property::Size, true),
            SelectMode::SelectAllExceptSmallestSize => select_all_except_by_property(&current_model, active_tab, Property::Size, false),
            SelectMode::SelectAllExceptBiggestResolution => select_all_except_by_property(&current_model, active_tab, Property::Resolution, true),
            SelectMode::SelectAllExceptSmallestResolution => select_all_except_by_property(&current_model, active_tab, Property::Resolution, false),
            SelectMode::SelectAllExceptNewest => select_all_except_by_property(&current_model, active_tab, Property::Date, true),
            SelectMode::SelectAllExceptOldest => select_all_except_by_property(&current_model, active_tab, Property::Date, false),
            SelectMode::SelectAllExceptLongestPath => select_all_except_by_property(&current_model, active_tab, Property::PathLength, true),
            SelectMode::SelectAllExceptShortestPath => select_all_except_by_property(&current_model, active_tab, Property::PathLength, false),

            SelectMode::SelectCustom => return,
        };
        active_tab.set_tool_model(&app, new_model);
        change_number_of_enabled_items(&app, active_tab, checked_items as i64 - unchecked_items as i64);
    });

    app.global::<Callabler>().on_validate_regex(|regex_str| {
        if regex_str.is_empty() {
            return true;
        }
        Regex::new(regex_str.as_str()).is_ok()
    });

    let a = app.as_weak();
    app.global::<Callabler>().on_populate_custom_select_columns(move || {
        let app = a.upgrade().expect("Failed to upgrade app :(");
        let active_tab = app.global::<GuiState>().get_active_tab();
        let save_restore = app.global::<Settings>().get_popup_custom_select_save_restore();

        let mut columns = custom_select::build_custom_select_columns(active_tab);

        if save_restore {
            let tab_key = format!("{active_tab:?}");
            let saved_states: BTreeMap<String, SavedCustomSelectTabState> = load_data_from_file(get_custom_select_state_file()).unwrap_or_default();
            if let Some(saved) = saved_states.get(&tab_key) {
                for (col, saved_col) in columns.iter_mut().zip(saved.columns.iter()) {
                    col.enabled = saved_col.enabled;
                    col.filter_value = SharedString::from(saved_col.filter_value.as_str());
                }
                app.global::<GuiState>().set_custom_select_restored_case_sensitive(saved.case_sensitive);
                app.global::<GuiState>().set_custom_select_restored_leave_one_in_group(saved.leave_one_in_group);
            } else {
                app.global::<GuiState>().set_custom_select_restored_case_sensitive(false);
                app.global::<GuiState>().set_custom_select_restored_leave_one_in_group(true);
            }
        }

        app.global::<GuiState>().set_custom_select_columns(create_model_from_model_vec(&columns));
    });

    let a = app.as_weak();
    app.global::<Callabler>().on_update_custom_select_column(move |idx, enabled, filter_value| {
        let app = a.upgrade().expect("Failed to upgrade app :(");
        let model = app.global::<GuiState>().get_custom_select_columns();
        let idx = idx as usize;
        if let Some(mut col) = model.row_data(idx) {
            col.enabled = enabled;
            col.filter_value = filter_value;
            model.set_row_data(idx, col);
        }
    });

    let a = app.as_weak();
    app.global::<Callabler>()
        .on_select_items_custom_columns(move |select_mode, case_sensitive, leave_one_in_group, save_restore| {
            let app = a.upgrade().expect("Failed to upgrade app :(");
            let active_tab = app.global::<GuiState>().get_active_tab();
            let current_model = active_tab.get_tool_model(&app);
            let columns: Vec<CustomSelectColumnModel> = app.global::<GuiState>().get_custom_select_columns().iter().collect();

            let leave_one_in_group = leave_one_in_group && (active_tab.get_is_header_mode() && !shared_models.lock().expect("Lock poisoned").get_use_reference_folders(active_tab));

            if save_restore {
                let tab_key = format!("{active_tab:?}");
                let mut saved_states: BTreeMap<String, SavedCustomSelectTabState> = load_data_from_file(get_custom_select_state_file()).unwrap_or_default();
                let saved_columns = columns
                    .iter()
                    .map(|c| SavedCustomSelectColumnState {
                        enabled: c.enabled,
                        filter_value: c.filter_value.to_string(),
                    })
                    .collect();
                saved_states.insert(
                    tab_key,
                    SavedCustomSelectTabState {
                        case_sensitive,
                        leave_one_in_group,
                        columns: saved_columns,
                    },
                );
                if let Err(e) = save_data_to_file(get_custom_select_state_file(), &saved_states) {
                    error!("Failed to save custom select state: {e}");
                }
            }

            let (checked_items, unchecked_items, new_model) =
                custom_select::select_custom_columns(&current_model, active_tab, select_mode, &columns, case_sensitive, leave_one_in_group);
            active_tab.set_tool_model(&app, new_model);
            change_number_of_enabled_items(&app, active_tab, checked_items as i64 - unchecked_items as i64);
        });
}

#[derive(Clone, Copy)]
enum Property {
    Size,
    Date,
    PathLength,
    Resolution,
}

pub(crate) fn set_select_buttons(app: &MainWindow) {
    let active_tab = app.global::<GuiState>().get_active_tab();
    let settings = app.global::<Settings>();
    let mut base_buttons = vec![SelectMode::SelectCustom, SelectMode::SelectAll, SelectMode::UnselectAll, SelectMode::InvertSelection];

    let additional_buttons = match active_tab {
        ActiveTab::DuplicateFiles | ActiveTab::SimilarVideos | ActiveTab::SimilarMusic => vec![
            (SelectMode::InvertSelectionInGroup, true),
            (SelectMode::SelectOldest, settings.get_select_show_oldest()),
            (SelectMode::SelectNewest, settings.get_select_show_newest()),
            (SelectMode::SelectTheSmallestSize, settings.get_select_show_smallest_size()),
            (SelectMode::SelectTheBiggestSize, settings.get_select_show_biggest_size()),
            (SelectMode::SelectShortestPath, settings.get_select_show_shortest_path()),
            (SelectMode::SelectLongestPath, settings.get_select_show_longest_path()),
            (SelectMode::SelectAllExceptOldest, settings.get_select_show_except_oldest()),
            (SelectMode::SelectAllExceptNewest, settings.get_select_show_except_newest()),
            (SelectMode::SelectAllExceptSmallestSize, settings.get_select_show_except_smallest_size()),
            (SelectMode::SelectAllExceptBiggestSize, settings.get_select_show_except_biggest_size()),
            (SelectMode::SelectAllExceptShortestPath, settings.get_select_show_except_shortest_path()),
            (SelectMode::SelectAllExceptLongestPath, settings.get_select_show_except_longest_path()),
        ],
        ActiveTab::SimilarImages => vec![
            (SelectMode::InvertSelectionInGroup, true),
            (SelectMode::SelectOldest, settings.get_select_show_oldest()),
            (SelectMode::SelectNewest, settings.get_select_show_newest()),
            (SelectMode::SelectTheSmallestSize, settings.get_select_show_smallest_size()),
            (SelectMode::SelectTheBiggestSize, settings.get_select_show_biggest_size()),
            (SelectMode::SelectTheSmallestResolution, settings.get_select_show_smallest_resolution()),
            (SelectMode::SelectTheBiggestResolution, settings.get_select_show_biggest_resolution()),
            (SelectMode::SelectShortestPath, settings.get_select_show_shortest_path()),
            (SelectMode::SelectLongestPath, settings.get_select_show_longest_path()),
            (SelectMode::SelectAllExceptOldest, settings.get_select_show_except_oldest()),
            (SelectMode::SelectAllExceptNewest, settings.get_select_show_except_newest()),
            (SelectMode::SelectAllExceptSmallestSize, settings.get_select_show_except_smallest_size()),
            (SelectMode::SelectAllExceptBiggestSize, settings.get_select_show_except_biggest_size()),
            (SelectMode::SelectAllExceptSmallestResolution, settings.get_select_show_except_smallest_resolution()),
            (SelectMode::SelectAllExceptBiggestResolution, settings.get_select_show_except_biggest_resolution()),
            (SelectMode::SelectAllExceptShortestPath, settings.get_select_show_except_shortest_path()),
            (SelectMode::SelectAllExceptLongestPath, settings.get_select_show_except_longest_path()),
        ],
        ActiveTab::EmptyFolders
        | ActiveTab::BigFiles
        | ActiveTab::EmptyFiles
        | ActiveTab::TemporaryFiles
        | ActiveTab::InvalidSymlinks
        | ActiveTab::BrokenFiles
        | ActiveTab::BadExtensions
        | ActiveTab::BadNames
        | ActiveTab::ExifRemover
        | ActiveTab::VideoOptimizer
        | ActiveTab::Settings
        | ActiveTab::About => Vec::new(),
    };

    base_buttons.extend(additional_buttons.into_iter().filter_map(|(mode, enabled)| enabled.then_some(mode)));
    base_buttons.reverse();

    let new_select_model = base_buttons
        .into_iter()
        .map(|e| SelectModel {
            name: translate_select_mode(e),
            data: e,
        })
        .collect::<Vec<_>>();

    app.global::<GuiState>().set_select_results_list(ModelRc::new(VecModel::from(new_select_model)));
}

fn extract_comparable_field(model: &SingleMainListModel, property: Property, active_tab: ActiveTab) -> u64 {
    let mut val_ints = model.val_int.iter();
    match property {
        Property::Size => {
            let high = val_ints.nth(active_tab.get_int_size_idx()).expect("can find file size property");
            let low = val_ints.next().expect("can find file size property");
            connect_i32_into_u64(high, low)
        }
        Property::Date => {
            let high = val_ints.nth(active_tab.get_int_modification_date_idx()).expect("can find file last modified property");
            let low = val_ints.next().expect("can find file last modified property");
            connect_i32_into_u64(high, low)
        }
        Property::PathLength => {
            let path_len = model.val_str.iter().nth(active_tab.get_str_path_idx()).expect("can find file path property").len();
            let name_len = model.val_str.iter().nth(active_tab.get_str_name_idx()).expect("can find file name property").len();
            // Primary key: directory path length; secondary key: filename length.
            // Packed into a single u64 so the existing comparison infrastructure works unchanged.
            ((path_len as u64) << 32) | (name_len as u64)
        }
        Property::Resolution => val_ints.nth(active_tab.get_int_pixel_count_idx()).expect("can find pixel count proerty") as u64,
    }
}

fn select_by_property(model: &ModelRc<SingleMainListModel>, active_tab: ActiveTab, property: Property, increasing_order: bool) -> SelectionResult {
    let mut checked_items = 0;

    let is_header_mode = active_tab.get_is_header_mode();
    assert!(is_header_mode); // non header modes not really have reason to use this function

    let mut old_data = model.iter().collect::<Vec<_>>();
    let headers_idx = find_header_idx_and_deselect_all(&mut old_data);
    if increasing_order {
        for i in 0..(headers_idx.len() - 1) {
            let mut max_item = 0;
            let mut max_item_idx = 1;
            #[expect(clippy::needless_range_loop)]
            for j in (headers_idx[i] + 1)..headers_idx[i + 1] {
                let item = extract_comparable_field(&old_data[j], property, active_tab);
                if item > max_item {
                    max_item = item;
                    max_item_idx = j;
                }
            }
            if !old_data[max_item_idx].checked {
                checked_items += 1;
            }
            old_data[max_item_idx].checked = true;
        }
    } else {
        for i in 0..(headers_idx.len() - 1) {
            let mut min_item = u64::MAX;
            let mut min_item_idx = 1;
            #[expect(clippy::needless_range_loop)]
            for j in (headers_idx[i] + 1)..headers_idx[i + 1] {
                let item = extract_comparable_field(&old_data[j], property, active_tab);
                if item < min_item {
                    min_item = item;
                    min_item_idx = j;
                }
            }
            if !old_data[min_item_idx].checked {
                checked_items += 1;
            }
            old_data[min_item_idx].checked = true;
        }
    }

    (checked_items, 0, ModelRc::new(VecModel::from(old_data)))
}

// Selects all items in each group EXCEPT the one with the extreme property value.
// `increasing_order: true`  → spares the biggest/newest/longest item (selects all others).
// `increasing_order: false` → spares the smallest/oldest/shortest item (selects all others).
fn select_all_except_by_property(model: &ModelRc<SingleMainListModel>, active_tab: ActiveTab, property: Property, increasing_order: bool) -> SelectionResult {
    let mut checked_items = 0;
    let mut unchecked_items = 0;

    let is_header_mode = active_tab.get_is_header_mode();
    assert!(is_header_mode);

    let mut old_data = model.iter().collect::<Vec<_>>();
    // Capture previous checked state before find_header_idx_and_deselect_all clears all flags,
    // so the returned delta is relative to the actual prior selection.
    let prev_checked: Vec<bool> = old_data.iter().map(|m| m.checked).collect();
    let headers_idx = find_header_idx_and_deselect_all(&mut old_data);

    for i in 0..(headers_idx.len() - 1) {
        let group_start = headers_idx[i] + 1;
        let group_end = headers_idx[i + 1];

        // Find the extreme item to spare.
        let mut extreme_val = if increasing_order { 0u64 } else { u64::MAX };
        let mut extreme_idx = group_start;
        for j in group_start..group_end {
            let val = extract_comparable_field(&old_data[j], property, active_tab);
            if increasing_order && val > extreme_val || !increasing_order && val < extreme_val {
                extreme_val = val;
                extreme_idx = j;
            }
        }

        // Select every item except the extreme one.
        for j in group_start..group_end {
            if j == extreme_idx {
                if prev_checked[j] {
                    unchecked_items += 1;
                }
                old_data[j].checked = false;
            } else {
                if !prev_checked[j] {
                    checked_items += 1;
                }
                old_data[j].checked = true;
            }
        }
    }

    (checked_items, unchecked_items, ModelRc::new(VecModel::from(old_data)))
}

fn select_all(model: &ModelRc<SingleMainListModel>) -> SelectionResult {
    let mut checked_items = 0;
    let mut old_data = model.iter().collect::<Vec<_>>();
    for x in &mut old_data {
        if !x.header_row {
            if !x.checked {
                checked_items += 1;
            }
            x.checked = true;
        }
    }
    (checked_items, 0, ModelRc::new(VecModel::from(old_data)))
}

fn deselect_all(model: &ModelRc<SingleMainListModel>) -> SelectionResult {
    let mut unchecked_items = 0;
    let mut old_data = model.iter().collect::<Vec<_>>();
    for x in &mut old_data {
        if x.checked {
            unchecked_items += 1;
        }
        x.checked = false;
    }
    (0, unchecked_items, ModelRc::new(VecModel::from(old_data)))
}

fn invert_selection(model: &ModelRc<SingleMainListModel>) -> SelectionResult {
    let mut checked_items = 0;
    let mut unchecked_items = 0;
    let mut old_data = model.iter().collect::<Vec<_>>();
    for x in &mut old_data {
        if !x.header_row {
            if x.checked {
                unchecked_items += 1;
            } else {
                checked_items += 1;
            }

            x.checked = !x.checked;
        }
    }
    (checked_items, unchecked_items, ModelRc::new(VecModel::from(old_data)))
}

fn invert_selection_in_group(model: &ModelRc<SingleMainListModel>) -> SelectionResult {
    let mut checked_items = 0;
    let mut unchecked_items = 0;
    let mut old_data = model.iter().collect::<Vec<_>>();

    let header_idx: Vec<usize> = old_data
        .iter()
        .enumerate()
        .filter_map(|(idx, m)| if m.header_row { Some(idx) } else { None })
        .chain(std::iter::once(old_data.len()))
        .collect();

    for group_idx in 0..header_idx.len() - 1 {
        let group_start = header_idx[group_idx] + 1;
        let group_end = header_idx[group_idx + 1];

        let has_selection_in_group = old_data[group_start..group_end].iter().any(|x| x.checked);

        if !has_selection_in_group {
            continue;
        }

        for i in group_start..group_end {
            if old_data[i].checked {
                unchecked_items += 1;
            } else {
                checked_items += 1;
            }
            old_data[i].checked = !old_data[i].checked;
        }
    }

    (checked_items, unchecked_items, ModelRc::new(VecModel::from(old_data)))
}

fn find_header_idx_and_deselect_all(old_data: &mut [SingleMainListModel]) -> Vec<usize> {
    let mut header_idx = old_data
        .iter()
        .enumerate()
        .filter_map(|(idx, m)| if m.header_row { Some(idx) } else { None })
        .collect::<Vec<_>>();
    header_idx.push(old_data.len());

    for x in old_data.iter_mut() {
        if !x.header_row {
            x.checked = false;
        }
    }
    header_idx
}

#[cfg(test)]
mod tests {
    use slint::{ModelRc, SharedString, VecModel};

    use super::*;
    use crate::common::{MAX_INT_DATA_DUPLICATE_FILES, MAX_STR_DATA_DUPLICATE_FILES, create_model_from_model_vec, split_u64_into_i32s};
    use crate::test_common::get_model_vec;

    // Builds a DuplicateFiles row with the given size encoded in val_int.
    // IntDataDuplicateFiles layout: [ModDatePart1, ModDatePart2, SizePart1, SizePart2]
    fn make_item_with_size(size: u64) -> SingleMainListModel {
        let (part1, part2) = split_u64_into_i32s(size);
        let ints: [i32; MAX_INT_DATA_DUPLICATE_FILES] = [0, 0, part1, part2];
        let strs: [SharedString; MAX_STR_DATA_DUPLICATE_FILES] = [SharedString::from(""), SharedString::from(""), SharedString::from(""), SharedString::from("")];
        SingleMainListModel {
            val_int: ModelRc::new(VecModel::from(ints.to_vec())),
            val_str: ModelRc::new(VecModel::from(strs.to_vec())),
            ..crate::test_common::get_main_list_model()
        }
    }

    // Builds a DuplicateFiles row with the given path and name in val_str.
    // StrDataDuplicateFiles layout: [Size_display, Name, Path, ModDate]
    fn make_item_with_path(path: &str, name: &str) -> SingleMainListModel {
        let ints: [i32; MAX_INT_DATA_DUPLICATE_FILES] = [0; MAX_INT_DATA_DUPLICATE_FILES];
        let strs: [SharedString; MAX_STR_DATA_DUPLICATE_FILES] = [SharedString::from(""), SharedString::from(name), SharedString::from(path), SharedString::from("")];
        SingleMainListModel {
            val_int: ModelRc::new(VecModel::from(ints.to_vec())),
            val_str: ModelRc::new(VecModel::from(strs.to_vec())),
            ..crate::test_common::get_main_list_model()
        }
    }

    // ...existing code...

    #[test]
    fn find_header_idx_returns_correct_indices_for_headers() {
        let mut model = get_model_vec(5);
        model[1].header_row = true;
        model[3].header_row = true;

        let header_indices = find_header_idx_and_deselect_all(&mut model);

        assert_eq!(header_indices, vec![1, 3, 5]);
    }

    #[test]
    fn find_header_idx_marks_all_non_header_rows_as_unchecked() {
        let mut model = get_model_vec(5);
        for row in &mut model {
            row.checked = true;
        }
        model[1].header_row = true;

        find_header_idx_and_deselect_all(&mut model);

        assert!(!model[0].checked);
        assert!(model[1].checked);
        assert!(!model[2].checked);
        assert!(!model[3].checked);
        assert!(!model[4].checked);
    }

    #[test]
    fn select_all_marks_all_non_header_rows_as_checked() {
        let mut model = get_model_vec(5);
        model[1].header_row = true;
        let model = create_model_from_model_vec(&model);

        let (checked_items, unchecked_items, new_model) = select_all(&model);

        assert_eq!(checked_items, 4);
        assert_eq!(unchecked_items, 0);
        assert!(new_model.row_data(0).unwrap().checked);
        assert!(!new_model.row_data(1).unwrap().checked);
        assert!(new_model.row_data(2).unwrap().checked);
        assert!(new_model.row_data(3).unwrap().checked);
        assert!(new_model.row_data(4).unwrap().checked);
    }

    #[test]
    fn deselect_all_unmarks_all_rows_as_checked() {
        let mut model = get_model_vec(5);
        for row in &mut model {
            row.checked = true;
        }
        let model = create_model_from_model_vec(&model);

        let (checked_items, unchecked_items, new_model) = deselect_all(&model);

        assert_eq!(checked_items, 0);
        assert_eq!(unchecked_items, 5);
        assert!(!new_model.row_data(0).unwrap().checked);
        assert!(!new_model.row_data(1).unwrap().checked);
        assert!(!new_model.row_data(2).unwrap().checked);
        assert!(!new_model.row_data(3).unwrap().checked);
        assert!(!new_model.row_data(4).unwrap().checked);
    }

    #[test]
    fn invert_selection_toggles_checked_state_for_non_header_rows() {
        let mut model = get_model_vec(5);
        model[0].checked = true;
        model[1].header_row = true;
        model[2].checked = false;
        let model = create_model_from_model_vec(&model);

        let (checked_items, unchecked_items, new_model) = invert_selection(&model);

        assert_eq!(checked_items, 3);
        assert_eq!(unchecked_items, 1);
        assert!(!new_model.row_data(0).unwrap().checked);
        assert!(!new_model.row_data(1).unwrap().checked);
        assert!(new_model.row_data(2).unwrap().checked);
        assert!(new_model.row_data(3).unwrap().checked);
        assert!(new_model.row_data(4).unwrap().checked);
    }

    #[test]
    fn invert_selection_in_group_only_changes_groups_with_selection() {
        let mut model = get_model_vec(7);
        model[1].header_row = true; // Group 1 header
        // Group 1 (indices 0): item 0 - no selection initially
        model[2].header_row = true; // Group 2 header
        // Group 2 (indices 3,4,5,6): items 3,4,5,6 - item 3 selected
        model[3].checked = true;
        let model = create_model_from_model_vec(&model);

        let (checked_items, unchecked_items, new_model) = invert_selection_in_group(&model);

        // Group 1 has no selection, should remain unchanged
        assert!(!new_model.row_data(0).unwrap().checked);
        // Group 2 has selection, should invert: item 3 checked->unchecked, others unchecked->checked
        assert!(!new_model.row_data(3).unwrap().checked);
        assert!(new_model.row_data(4).unwrap().checked);
        assert!(new_model.row_data(5).unwrap().checked);
        assert!(new_model.row_data(6).unwrap().checked);
        // Verify counts: 3 items flipped from unchecked to checked, 1 from checked to unchecked
        assert_eq!(checked_items, 3);
        assert_eq!(unchecked_items, 1);
    }

    #[test]
    fn invert_selection_in_group_does_nothing_when_no_groups_have_selection() {
        let mut model = get_model_vec(5);
        model[1].header_row = true;
        let model = create_model_from_model_vec(&model);

        let (checked_items, unchecked_items, new_model) = invert_selection_in_group(&model);

        assert_eq!(checked_items, 0);
        assert_eq!(unchecked_items, 0);
        assert!(!new_model.row_data(0).unwrap().checked);
        assert!(!new_model.row_data(2).unwrap().checked);
    }

    //  select_all_except_by_property

    #[test]
    fn select_all_except_biggest_spares_largest_item() {
        // Layout: [header, small(100), medium(200), large(300)]
        let mut header = crate::test_common::get_main_list_model();
        header.header_row = true;
        let items = vec![header, make_item_with_size(100), make_item_with_size(200), make_item_with_size(300)];
        let model = create_model_from_model_vec(&items);

        let (checked_items, unchecked_items, new_model) = select_all_except_by_property(&model, ActiveTab::DuplicateFiles, Property::Size, true);

        assert_eq!(checked_items, 2);
        assert_eq!(unchecked_items, 0);
        assert!(new_model.row_data(1).unwrap().checked); // 100 – selected
        assert!(new_model.row_data(2).unwrap().checked); // 200 – selected
        assert!(!new_model.row_data(3).unwrap().checked); // 300 – spared (biggest)
    }

    #[test]
    fn select_all_except_smallest_spares_smallest_item() {
        // Layout: [header, small(100), medium(200), large(300)]
        let mut header = crate::test_common::get_main_list_model();
        header.header_row = true;
        let items = vec![header, make_item_with_size(100), make_item_with_size(200), make_item_with_size(300)];
        let model = create_model_from_model_vec(&items);

        let (checked_items, unchecked_items, new_model) = select_all_except_by_property(&model, ActiveTab::DuplicateFiles, Property::Size, false);

        assert_eq!(checked_items, 2);
        assert_eq!(unchecked_items, 0);
        assert!(!new_model.row_data(1).unwrap().checked); // 100 – spared (smallest)
        assert!(new_model.row_data(2).unwrap().checked); // 200 – selected
        assert!(new_model.row_data(3).unwrap().checked); // 300 – selected
    }

    #[test]
    fn select_all_except_operates_independently_per_group() {
        // Group 1: [header, 100, 300]  – spare 300 (biggest) / spare 100 (smallest)
        // Group 2: [header, 50, 150]   – spare 150 (biggest) / spare 50  (smallest)
        let mut h1 = crate::test_common::get_main_list_model();
        h1.header_row = true;
        let mut h2 = crate::test_common::get_main_list_model();
        h2.header_row = true;
        let items = vec![
            h1,
            make_item_with_size(100),
            make_item_with_size(300),
            h2,
            make_item_with_size(50),
            make_item_with_size(150),
        ];
        let model = create_model_from_model_vec(&items);

        let (_checked, _unchecked, new_model) = select_all_except_by_property(&model, ActiveTab::DuplicateFiles, Property::Size, true);

        assert!(new_model.row_data(1).unwrap().checked); // 100 – selected
        assert!(!new_model.row_data(2).unwrap().checked); // 300 – spared (biggest in group 1)
        assert!(new_model.row_data(4).unwrap().checked); // 50 – selected
        assert!(!new_model.row_data(5).unwrap().checked); // 150 – spared (biggest in group 2)
    }

    #[test]
    fn select_all_except_delta_accounts_for_previously_checked_items() {
        // Items 1 and 2 are pre-checked; biggest (300) will be spared → item 1 gets unchecked again.
        let mut header = crate::test_common::get_main_list_model();
        header.header_row = true;
        let mut item_small = make_item_with_size(100);
        item_small.checked = true; // pre-checked
        let mut item_medium = make_item_with_size(200);
        item_medium.checked = true; // pre-checked
        let item_large = make_item_with_size(300);
        let items = vec![header, item_small, item_medium, item_large];
        let model = create_model_from_model_vec(&items);

        let (checked_items, unchecked_items, new_model) = select_all_except_by_property(&model, ActiveTab::DuplicateFiles, Property::Size, true);

        // item_small was already checked → not counted as newly checked
        // item_medium was already checked → not counted as newly checked
        // item_large was unchecked and will be spared → no change counted for it
        assert_eq!(checked_items, 0);
        // item_large was not checked, so it contributes 0 unchecked_items; medium was checked but stays checked
        // Actually item_large remains unchecked (spared), so unchecked_items should be 0
        assert_eq!(unchecked_items, 0);
        assert!(new_model.row_data(1).unwrap().checked);
        assert!(new_model.row_data(2).unwrap().checked);
        assert!(!new_model.row_data(3).unwrap().checked);
    }

    #[test]
    fn select_all_except_counts_unchecked_when_spared_item_was_previously_checked() {
        // Only item_large is pre-checked; it will be spared → unchecked_items += 1.
        let mut header = crate::test_common::get_main_list_model();
        header.header_row = true;
        let item_small = make_item_with_size(100);
        let item_medium = make_item_with_size(200);
        let mut item_large = make_item_with_size(300);
        item_large.checked = true; // pre-checked, but will be spared
        let items = vec![header, item_small, item_medium, item_large];
        let model = create_model_from_model_vec(&items);

        let (checked_items, unchecked_items, new_model) = select_all_except_by_property(&model, ActiveTab::DuplicateFiles, Property::Size, true);

        assert_eq!(checked_items, 2); // small and medium go from unchecked → checked
        assert_eq!(unchecked_items, 1); // large goes from checked → unchecked (spared)
        assert!(new_model.row_data(1).unwrap().checked);
        assert!(new_model.row_data(2).unwrap().checked);
        assert!(!new_model.row_data(3).unwrap().checked);
    }

    #[test]
    fn select_all_except_longest_path_spares_item_with_longest_full_path() {
        // StrDataDuplicateFiles: [Size_display, Name, Path, ModDate]
        // Primary sort key: directory path length; secondary: filename length.
        let mut header = crate::test_common::get_main_list_model();
        header.header_row = true;
        // short: "/a" (dir=2) + "x.jpg"    → key (2, 5)
        // medium: "/ab" (dir=3) + "x.jpg"  → key (3, 5)
        // long: "/abc" (dir=4) + "x.jpg"   → key (4, 5)
        let items = vec![
            header,
            make_item_with_path("/a", "x.jpg"),
            make_item_with_path("/ab", "x.jpg"),
            make_item_with_path("/abc", "x.jpg"),
        ];
        let model = create_model_from_model_vec(&items);

        let (_checked, _unchecked, new_model) = select_all_except_by_property(&model, ActiveTab::DuplicateFiles, Property::PathLength, true);

        assert!(new_model.row_data(1).unwrap().checked); // short – selected
        assert!(new_model.row_data(2).unwrap().checked); // medium – selected
        assert!(!new_model.row_data(3).unwrap().checked); // long – spared (longest)
    }

    #[test]
    fn select_all_except_shortest_path_spares_item_with_shortest_full_path() {
        // Primary sort key: directory path length; secondary: filename length.
        let mut header = crate::test_common::get_main_list_model();
        header.header_row = true;
        let items = vec![
            header,
            make_item_with_path("/a", "x.jpg"),
            make_item_with_path("/ab", "x.jpg"),
            make_item_with_path("/abc", "x.jpg"),
        ];
        let model = create_model_from_model_vec(&items);

        let (_checked, _unchecked, new_model) = select_all_except_by_property(&model, ActiveTab::DuplicateFiles, Property::PathLength, false);

        assert!(!new_model.row_data(1).unwrap().checked); // short – spared (shortest)
        assert!(new_model.row_data(2).unwrap().checked); // medium – selected
        assert!(new_model.row_data(3).unwrap().checked); // long – selected
    }

    #[test]
    fn select_all_except_shortest_path_uses_filename_to_break_ties_in_same_directory() {
        // All files share the same directory; path length differs only by filename.
        let mut header = crate::test_common::get_main_list_model();
        header.header_row = true;
        // IMG_0001.JPG (13 chars) < IMG_0001-1.JPG (15 chars) < IMG_0001-2.JPG (15 chars)
        let items = vec![
            header,
            make_item_with_path("/photos", "IMG_0001.JPG"),
            make_item_with_path("/photos", "IMG_0001-1.JPG"),
            make_item_with_path("/photos", "IMG_0001-2.JPG"),
        ];
        let model = create_model_from_model_vec(&items);

        let (_checked, _unchecked, new_model) = select_all_except_by_property(&model, ActiveTab::DuplicateFiles, Property::PathLength, false);

        assert!(!new_model.row_data(1).unwrap().checked); // IMG_0001.JPG – spared (shortest)
        assert!(new_model.row_data(2).unwrap().checked);
        assert!(new_model.row_data(3).unwrap().checked);
    }

    #[test]
    fn select_all_except_shortest_path_prefers_directory_length_over_filename_length() {
        // File in a shorter directory but with a long name must still be spared
        // over a file in a longer directory with a short name.
        // Before the fix, summing lengths could invert this result.
        let mut header = crate::test_common::get_main_list_model();
        header.header_row = true;
        // short_dir + long_name  → dir=3, name=20 → was sum=23, now key=(3<<32)|20
        // long_dir  + short_name → dir=20, name=3  → was sum=23, now key=(20<<32)|3
        let items = vec![
            header,
            make_item_with_path("/sd", "a_very_long_filename.jpg"),   // dir shorter → spared
            make_item_with_path("/a/much/longer/directory", "b.jpg"), // dir longer → selected
        ];
        let model = create_model_from_model_vec(&items);

        let (_checked, _unchecked, new_model) = select_all_except_by_property(&model, ActiveTab::DuplicateFiles, Property::PathLength, false);

        assert!(!new_model.row_data(1).unwrap().checked); // shorter dir – spared
        assert!(new_model.row_data(2).unwrap().checked); // longer dir – selected
    }
}
