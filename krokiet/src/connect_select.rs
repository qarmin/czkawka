use slint::{ComponentHandle, Model, ModelRc, VecModel};

use crate::common::connect_i32_into_u64;
use crate::connect_row_selection::checker::change_number_of_enabled_items;
use crate::connect_translation::translate_select_mode;
use crate::{ActiveTab, Callabler, GuiState, MainListModel, MainWindow, SelectMode, SelectModel};

type SelectionResult = (u64, u64, ModelRc<MainListModel>);

// TODO optimize this, not sure if it is possible to not copy entire model to just select item
// https://github.com/slint-ui/slint/discussions/4595
pub(crate) fn connect_select(app: &MainWindow) {
    let a = app.as_weak();
    app.global::<Callabler>().on_select_items(move |select_mode| {
        let app = a.upgrade().expect("Failed to upgrade app :(");
        let active_tab = app.global::<GuiState>().get_active_tab();
        let current_model = active_tab.get_tool_model(&app);

        let (checked_items, unchecked_items, new_model) = match select_mode {
            SelectMode::SelectAll => select_all(&current_model),
            SelectMode::UnselectAll => deselect_all(&current_model),
            SelectMode::InvertSelection => invert_selection(&current_model),
            SelectMode::SelectTheBiggestSize => select_by_size_date(&current_model, active_tab, true, true),
            SelectMode::SelectTheSmallestSize => select_by_size_date(&current_model, active_tab, false, true),
            SelectMode::SelectTheBiggestResolution => select_by_resolution(&current_model, active_tab, true),
            SelectMode::SelectTheSmallestResolution => select_by_resolution(&current_model, active_tab, false),
            SelectMode::SelectNewest => select_by_size_date(&current_model, active_tab, true, false),
            SelectMode::SelectOldest => select_by_size_date(&current_model, active_tab, false, false),
        };
        active_tab.set_tool_model(&app, new_model);
        change_number_of_enabled_items(&app, active_tab, checked_items as i64 - unchecked_items as i64);
    });
}

pub(crate) fn connect_showing_proper_select_buttons(app: &MainWindow) {
    set_select_buttons(app);
    let a = app.as_weak();
    app.global::<Callabler>().on_tab_changed(move || {
        let app = a.upgrade().expect("Failed to upgrade app :(");
        set_select_buttons(&app);
    });
}

fn set_select_buttons(app: &MainWindow) {
    let active_tab = app.global::<GuiState>().get_active_tab();
    let mut base_buttons = vec![SelectMode::SelectAll, SelectMode::UnselectAll, SelectMode::InvertSelection];

    let additional_buttons = match active_tab {
        ActiveTab::DuplicateFiles | ActiveTab::SimilarVideos | ActiveTab::SimilarMusic => vec![
            SelectMode::SelectOldest,
            SelectMode::SelectNewest,
            SelectMode::SelectTheSmallestSize,
            SelectMode::SelectTheBiggestSize,
        ],
        ActiveTab::SimilarImages => vec![
            SelectMode::SelectOldest,
            SelectMode::SelectNewest,
            SelectMode::SelectTheSmallestSize,
            SelectMode::SelectTheBiggestSize,
            SelectMode::SelectTheSmallestResolution,
            SelectMode::SelectTheBiggestResolution,
        ],
        ActiveTab::EmptyFolders
        | ActiveTab::BigFiles
        | ActiveTab::EmptyFiles
        | ActiveTab::TemporaryFiles
        | ActiveTab::InvalidSymlinks
        | ActiveTab::BrokenFiles
        | ActiveTab::BadExtensions
        | ActiveTab::VideoOptimizer
        | ActiveTab::ExifRemover
        | ActiveTab::Settings
        | ActiveTab::About => vec![], // Not available in settings and about, so may be set any value here
    };

    base_buttons.extend(additional_buttons);
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

// TODO, when model will be able to contain i64 instead two i32, this function could be merged with select_by_size_date
fn select_by_resolution(model: &ModelRc<MainListModel>, active_tab: ActiveTab, biggest: bool) -> SelectionResult {
    let mut checked_items = 0;

    let is_header_mode = active_tab.get_is_header_mode();
    assert!(is_header_mode); // non header modes not really have reason to use this function

    let mut old_data = model.iter().collect::<Vec<_>>();
    let headers_idx = find_header_idx_and_deselect_all(&mut old_data);
    let width_idx = active_tab.get_int_width_idx();
    let height_idx = active_tab.get_int_height_idx();

    if biggest {
        for i in 0..(headers_idx.len() - 1) {
            let mut max_item = 0;
            let mut max_item_idx = 1;
            #[expect(clippy::needless_range_loop)]
            for j in (headers_idx[i] + 1)..headers_idx[i + 1] {
                let int_data = old_data[j].val_int.iter().collect::<Vec<_>>();
                let item = int_data[width_idx] * int_data[height_idx];
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
                let int_data = old_data[j].val_int.iter().collect::<Vec<_>>();
                let item = (int_data[width_idx] * int_data[height_idx]) as u64;
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

fn select_by_size_date(model: &ModelRc<MainListModel>, active_tab: ActiveTab, biggest_newest: bool, size: bool) -> SelectionResult {
    let mut checked_items = 0;

    let is_header_mode = active_tab.get_is_header_mode();
    assert!(is_header_mode); // non header modes not really have reason to use this function

    let mut old_data = model.iter().collect::<Vec<_>>();
    let headers_idx = find_header_idx_and_deselect_all(&mut old_data);
    let item_idx = if size {
        active_tab.get_int_size_idx()
    } else {
        active_tab.get_int_modification_date_idx()
    };

    if biggest_newest {
        for i in 0..(headers_idx.len() - 1) {
            let mut max_item = 0;
            let mut max_item_idx = 1;
            #[expect(clippy::needless_range_loop)]
            for j in (headers_idx[i] + 1)..headers_idx[i + 1] {
                let int_data = old_data[j].val_int.iter().collect::<Vec<_>>();
                let item = connect_i32_into_u64(int_data[item_idx], int_data[item_idx + 1]);
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
                let int_data = old_data[j].val_int.iter().collect::<Vec<_>>();
                let item = connect_i32_into_u64(int_data[item_idx], int_data[item_idx + 1]);
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

fn select_all(model: &ModelRc<MainListModel>) -> SelectionResult {
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

fn deselect_all(model: &ModelRc<MainListModel>) -> SelectionResult {
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

fn invert_selection(model: &ModelRc<MainListModel>) -> SelectionResult {
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

fn find_header_idx_and_deselect_all(old_data: &mut [MainListModel]) -> Vec<usize> {
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
    use crate::test_common::{create_model_from_model_vec, get_model_vec};

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
        assert!(model[1].checked); // header row
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
        assert!(!new_model.row_data(1).unwrap().checked); // header row
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
        assert!(!new_model.row_data(1).unwrap().checked); // header row
        assert!(new_model.row_data(2).unwrap().checked);
        assert!(new_model.row_data(3).unwrap().checked);
        assert!(new_model.row_data(4).unwrap().checked);
    }
}
