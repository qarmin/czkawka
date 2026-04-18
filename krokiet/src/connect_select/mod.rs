pub(crate) mod custom_select;

use std::sync::{Arc, Mutex};

use regex::Regex;
use slint::{ComponentHandle, Model, ModelRc, VecModel};

use crate::common::{connect_i32_into_u64, create_model_from_model_vec};
use crate::connect_row_selection::checker::change_number_of_enabled_items;
use crate::connect_translation::translate_select_mode;
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
        let columns = custom_select::build_custom_select_columns(active_tab);
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
        .on_select_items_custom_columns(move |select_mode, case_sensitive, leave_one_in_group| {
            let app = a.upgrade().expect("Failed to upgrade app :(");
            let active_tab = app.global::<GuiState>().get_active_tab();
            let current_model = active_tab.get_tool_model(&app);
            let columns: Vec<CustomSelectColumnModel> = app.global::<GuiState>().get_custom_select_columns().iter().collect();

            let leave_one_in_group = leave_one_in_group && (active_tab.get_is_header_mode() && !shared_models.lock().expect("Lock poisoned").get_use_reference_folders(active_tab));

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
    let mut val_strs = model.val_str.iter();
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
        Property::PathLength => val_strs.nth(active_tab.get_str_path_idx()).expect("can find file path property").len() as u64,
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
                if old_data[j].checked {
                    unchecked_items += 1;
                }
                old_data[j].checked = false;
            } else {
                if !old_data[j].checked {
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
    use super::*;
    use crate::common::create_model_from_model_vec;
    use crate::test_common::get_model_vec;

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
}
