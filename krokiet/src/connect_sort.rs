use std::mem;

use slint::{ComponentHandle, Model, ModelRc, VecModel};

use crate::common::{SortIdx, connect_i32_into_u64};
use crate::connect_row_selection::recalculate_small_selection_if_needed;
use crate::connect_translation::translate_sort_mode;
use crate::{ActiveTab, Callabler, GuiState, MainListModel, MainWindow, SortColumnMode, SortMode, SortModel};

pub(crate) fn connect_sort_column(app: &MainWindow) {
    let a = app.as_weak();
    app.global::<Callabler>().on_change_sort_column_mode(move |sort_column_mode, column_idx| {
        let app = a.upgrade().expect("Failed to upgrade app :(");
        let active_tab = app.global::<GuiState>().get_active_tab();
        let model = active_tab.get_tool_model(&app);

        let idx = active_tab.get_str_int_sort_idx(column_idx);
        let new_model = match idx {
            SortIdx::StrIdx(str_idx) => {
                let sort_function = |e: &MainListModel| {
                    e.val_str
                        .iter()
                        .nth(str_idx as usize)
                        .unwrap_or_else(|| panic!("Failed to get str index - {str_idx} on {} items", e.val_str.iter().count()))
                };

                common_sort_function(&model, active_tab, sort_function, sort_column_mode == SortColumnMode::Descending)
            }
            SortIdx::IntIdx(int_idx) => {
                let sort_function = |e: &MainListModel| {
                    e.val_int
                        .iter()
                        .nth(int_idx as usize)
                        .unwrap_or_else(|| panic!("Failed to get int index - {int_idx} on {} items", e.val_int.iter().count()))
                };

                common_sort_function(&model, active_tab, sort_function, sort_column_mode == SortColumnMode::Descending)
            }
            SortIdx::IntIdxPair(int_idx1, int_idx2) => {
                let sort_function = |e: &MainListModel| {
                    let items = e.val_int.iter().collect::<Vec<_>>();
                    connect_i32_into_u64(items[int_idx1 as usize], items[int_idx2 as usize])
                };

                common_sort_function(&model, active_tab, sort_function, sort_column_mode == SortColumnMode::Descending)
            }
            SortIdx::Selection => {
                if sort_column_mode == SortColumnMode::Ascending {
                    let sort_function = |e: &MainListModel| e.checked;
                    common_sort_function(&model, active_tab, sort_function, false)
                } else {
                    let sort_function = |e: &MainListModel| !e.checked;
                    common_sort_function(&model, active_tab, sort_function, false)
                }
            }
        };

        active_tab.set_tool_model(&app, new_model);
    });
}

pub(crate) fn connect_sort(app: &MainWindow) {
    let a = app.as_weak();
    app.global::<Callabler>().on_sort_items(move |sort_mode| {
        let app = a.upgrade().expect("Failed to upgrade app :(");
        let active_tab = app.global::<GuiState>().get_active_tab();
        let current_model = active_tab.get_tool_model(&app);

        let new_model = match sort_mode {
            SortMode::FullName => sorts::sort_by_full_name(&current_model, active_tab),
            SortMode::Selection => sorts::sort_selection(&current_model, active_tab),
            SortMode::Reverse => sorts::reverse_sort(&current_model, active_tab),
        };

        active_tab.set_tool_model(&app, new_model);
    });
}

pub(crate) fn connect_showing_proper_sort_buttons(app: &MainWindow) {
    set_sort_buttons(app);
    let a = app.as_weak();
    app.global::<Callabler>().on_tab_changed(move || {
        let app = a.upgrade().expect("Failed to upgrade app :(");
        set_sort_buttons(&app);
    });
}

fn set_sort_buttons(app: &MainWindow) {
    let mut base_buttons = vec![SortMode::FullName, SortMode::Reverse, SortMode::Selection];
    base_buttons.reverse();

    let new_sort_model = base_buttons
        .into_iter()
        .map(|e| SortModel {
            name: translate_sort_mode(e),
            data: e,
        })
        .collect::<Vec<_>>();

    app.global::<GuiState>().set_sort_results_list(ModelRc::new(VecModel::from(new_sort_model)));
}

mod sorts {
    use super::{
        ActiveTab, MainListModel, Model, ModelRc, VecModel, common_sort_function, convert_group_header_into_rc_model, group_by_header, recalculate_small_selection_if_needed,
    };

    pub(super) fn reverse_sort(model: &ModelRc<MainListModel>, active_tab: ActiveTab) -> ModelRc<MainListModel> {
        if !active_tab.get_is_header_mode() {
            let mut items = model.iter().collect::<Vec<_>>();
            items.reverse();
            let new_model = ModelRc::new(VecModel::from(items));
            recalculate_small_selection_if_needed(&new_model, active_tab);
            return new_model;
        }

        let mut grouped_items = group_by_header(model);
        for (_, items) in &mut grouped_items {
            items.reverse();
        }

        let new_model = convert_group_header_into_rc_model(grouped_items, model.row_count());
        recalculate_small_selection_if_needed(&new_model, active_tab);
        new_model
    }

    pub(super) fn sort_selection(model: &ModelRc<MainListModel>, active_tab: ActiveTab) -> ModelRc<MainListModel> {
        let sort_function = |e: &MainListModel| !e.selected_row;

        common_sort_function(model, active_tab, sort_function, false)
    }

    pub(super) fn sort_by_full_name(model: &ModelRc<MainListModel>, active_tab: ActiveTab) -> ModelRc<MainListModel> {
        let sort_function = |e: &MainListModel| {
            let name_idx = active_tab.get_str_name_idx();
            let path_idx = active_tab.get_str_path_idx();
            let items = e.val_str.iter().collect::<Vec<_>>();
            format!("{}/{}", items[path_idx], items[name_idx])
        };

        common_sort_function(model, active_tab, sort_function, false)
    }
}

fn common_sort_function<T: Ord>(model: &ModelRc<MainListModel>, active_tab: ActiveTab, sort_function: impl Fn(&MainListModel) -> T, reverse: bool) -> ModelRc<MainListModel> {
    if !active_tab.get_is_header_mode() {
        let mut items = model.iter().collect::<Vec<_>>();
        items.sort_by_cached_key(&sort_function);
        if reverse {
            items.reverse();
        }
        let new_model = ModelRc::new(VecModel::from(items));
        recalculate_small_selection_if_needed(&new_model, active_tab);
        return new_model;
    }

    let mut grouped_items = group_by_header(model);
    for (_, items) in &mut grouped_items {
        items.sort_by_cached_key(&sort_function);
        if reverse {
            items.reverse();
        }
    }

    let new_model = convert_group_header_into_rc_model(grouped_items, model.row_count());
    recalculate_small_selection_if_needed(&new_model, active_tab);
    new_model
}

fn convert_group_header_into_rc_model(grouped: Vec<(MainListModel, Vec<MainListModel>)>, model_size: usize) -> ModelRc<MainListModel> {
    let mut items = Vec::with_capacity(model_size);
    for (header, group) in grouped {
        items.push(header);
        items.extend(group);
    }
    ModelRc::new(VecModel::from(items))
}

fn group_by_header(model: &ModelRc<MainListModel>) -> Vec<(MainListModel, Vec<MainListModel>)> {
    let mut grouped_items: Vec<(MainListModel, Vec<MainListModel>)> = vec![];

    let mut current_header: Option<MainListModel> = None;
    let mut current_group: Vec<MainListModel> = vec![];
    for item in model.iter() {
        if item.header_row {
            if let Some(header) = current_header.take() {
                assert!(!current_group.is_empty());
                grouped_items.push((header, mem::take(&mut current_group)));
            } else {
                assert!(current_group.is_empty());
            }
            current_header = Some(item.clone());
        } else {
            assert!(current_header.is_some());
            current_group.push(item.clone());
        }
    }

    if let Some(header) = current_header {
        assert!(!current_group.is_empty());
        grouped_items.push((header, current_group));
    } else {
        assert!(current_group.is_empty());
    }

    grouped_items
}

#[cfg(test)]
mod tests {
    use slint::Model;

    use crate::connect_row_selection::initialize_selection_struct;
    use crate::connect_sort::sorts::{reverse_sort, sort_by_full_name, sort_selection};
    use crate::connect_sort::{convert_group_header_into_rc_model, group_by_header};
    use crate::test_common::{create_model_from_model_vec, get_model_vec};
    use crate::{ActiveTab, MainListModel};

    #[test]
    fn group_by_header_splits_items_into_groups_correctly() {
        initialize_selection_struct();
        let mut model = get_model_vec(6);
        model[0].header_row = true;
        model[1].header_row = false;
        model[2].header_row = false;
        model[3].header_row = true;
        model[4].header_row = false;
        model[5].header_row = false;
        let model = create_model_from_model_vec(&model);

        let grouped = group_by_header(&model);

        assert_eq!(grouped.len(), 2);
        assert_eq!(grouped[0].0, model.row_data(0).unwrap());
        assert_eq!(grouped[0].1.len(), 2);
        assert_eq!(grouped[0].1[0], model.row_data(1).unwrap());
        assert_eq!(grouped[0].1[1], model.row_data(2).unwrap());
        assert_eq!(grouped[1].0, model.row_data(3).unwrap());
        assert_eq!(grouped[1].1.len(), 2);
        assert_eq!(grouped[1].1[0], model.row_data(4).unwrap());
        assert_eq!(grouped[1].1[1], model.row_data(5).unwrap());
    }

    #[test]
    fn group_by_header_handles_empty_model() {
        initialize_selection_struct();
        let model = create_model_from_model_vec(&[]);

        let grouped = group_by_header(&model);

        assert!(grouped.is_empty());
    }

    #[test]
    #[should_panic]
    fn group_by_header_panics_when_no_header_before_items() {
        initialize_selection_struct();
        let mut model = get_model_vec(3);
        model[0].header_row = false;
        model[1].header_row = false;
        model[2].header_row = false;
        let model = create_model_from_model_vec(&model);

        group_by_header(&model);
    }

    #[test]
    #[should_panic]
    fn group_by_header_panics_when_group_is_empty() {
        initialize_selection_struct();
        let mut model = get_model_vec(3);
        model[0].header_row = true;
        model[1].header_row = true;
        model[2].header_row = true;
        let model = create_model_from_model_vec(&model);

        group_by_header(&model);
    }

    #[test]
    fn convert_group_header_into_rc_model_combines_groups_correctly() {
        initialize_selection_struct();
        let mut model = get_model_vec(6);
        model[0].header_row = true;
        model[1].header_row = false;
        model[2].header_row = false;
        model[3].header_row = true;
        model[4].header_row = false;
        model[5].header_row = false;

        let grouped = vec![
            (model[0].clone(), vec![model[1].clone(), model[2].clone()]),
            (model[3].clone(), vec![model[4].clone(), model[5].clone()]),
        ];

        let combined_model = convert_group_header_into_rc_model(grouped, model.len());

        assert_eq!(combined_model.row_count(), 6);
        assert_eq!(combined_model.row_data(0).unwrap(), model[0]);
        assert_eq!(combined_model.row_data(1).unwrap(), model[1]);
        assert_eq!(combined_model.row_data(2).unwrap(), model[2]);
        assert_eq!(combined_model.row_data(3).unwrap(), model[3]);
        assert_eq!(combined_model.row_data(4).unwrap(), model[4]);
        assert_eq!(combined_model.row_data(5).unwrap(), model[5]);
    }

    #[test]
    fn convert_group_header_into_rc_model_handles_empty_groups() {
        initialize_selection_struct();
        let grouped: Vec<(MainListModel, Vec<MainListModel>)> = vec![];

        let combined_model = convert_group_header_into_rc_model(grouped, 0);

        assert_eq!(combined_model.row_count(), 0);
    }

    #[test]
    fn sort_by_full_name_sorts_flat_model_correctly() {
        initialize_selection_struct();
        let active_tab = ActiveTab::BigFiles;
        // To be sure that we set correct values in val_int, which must be equal to index
        assert_eq!(active_tab.get_str_name_idx(), 1);
        assert_eq!(active_tab.get_str_path_idx(), 2);
        assert!(!active_tab.get_is_header_mode());

        let mut model = get_model_vec(5);
        model[0].val_str = create_model_from_model_vec(&["".into(), "E".into(), "A".into()]);
        model[1].val_str = create_model_from_model_vec(&["".into(), "D".into(), "B".into()]);
        model[2].val_str = create_model_from_model_vec(&["".into(), "A".into(), "C".into()]);
        model[3].val_str = create_model_from_model_vec(&["".into(), "A".into(), "D".into()]);
        model[4].val_str = create_model_from_model_vec(&["".into(), "F".into(), "B".into()]);
        let model = create_model_from_model_vec(&model);

        let sorted_model = sort_by_full_name(&model, active_tab);

        assert_eq!(sorted_model.row_data(0).unwrap().val_str.iter().skip(1).collect::<Vec<_>>(), vec!["E", "A"]);
        assert_eq!(sorted_model.row_data(1).unwrap().val_str.iter().skip(1).collect::<Vec<_>>(), vec!["D", "B"]);
        assert_eq!(sorted_model.row_data(2).unwrap().val_str.iter().skip(1).collect::<Vec<_>>(), vec!["F", "B"]);
        assert_eq!(sorted_model.row_data(3).unwrap().val_str.iter().skip(1).collect::<Vec<_>>(), vec!["A", "C"]);
        assert_eq!(sorted_model.row_data(4).unwrap().val_str.iter().skip(1).collect::<Vec<_>>(), vec!["A", "D"]);
    }

    #[test]
    fn sort_by_selection_sorts_flat_model_correctly() {
        initialize_selection_struct();
        let active_tab = ActiveTab::BigFiles;
        // To be sure that we set correct values in val_int, which must be equal to index
        assert!(!active_tab.get_is_header_mode());

        let mut model = get_model_vec(4);
        model[0].selected_row = true;
        model[0].val_int = create_model_from_model_vec(&[15]);
        model[1].selected_row = false;
        model[1].val_int = create_model_from_model_vec(&[14]);
        model[2].selected_row = true;
        model[2].val_int = create_model_from_model_vec(&[9]);
        model[3].selected_row = false;
        model[3].val_int = create_model_from_model_vec(&[29]);
        let model = create_model_from_model_vec(&model);

        let sorted_model = sort_selection(&model, active_tab);

        assert!(sorted_model.row_data(0).unwrap().selected_row);
        assert_eq!(sorted_model.row_data(0).unwrap().val_int.iter().collect::<Vec<_>>(), vec![15]);
        assert!(sorted_model.row_data(1).unwrap().selected_row);
        assert_eq!(sorted_model.row_data(1).unwrap().val_int.iter().collect::<Vec<_>>(), vec![9]);
        assert!(!sorted_model.row_data(2).unwrap().selected_row);
        assert_eq!(sorted_model.row_data(2).unwrap().val_int.iter().collect::<Vec<_>>(), vec![14]);
        assert!(!sorted_model.row_data(3).unwrap().selected_row);
        assert_eq!(sorted_model.row_data(3).unwrap().val_int.iter().collect::<Vec<_>>(), vec![29]);
    }

    #[test]
    fn sort_reverse_sorts_flat_model_correctly() {
        initialize_selection_struct();
        let active_tab = ActiveTab::BigFiles;
        // To be sure that we set correct values in val_int, which must be equal to index
        assert_eq!(active_tab.get_int_modification_date_idx(), 0);
        assert!(!active_tab.get_is_header_mode());

        let mut model = get_model_vec(5);
        model[0].val_int = create_model_from_model_vec(&[9, 9]);
        model[1].val_int = create_model_from_model_vec(&[9, 10]);
        model[2].val_int = create_model_from_model_vec(&[14, 50]);
        model[3].val_int = create_model_from_model_vec(&[15, 17]);
        model[4].val_int = create_model_from_model_vec(&[29, 0]);
        let model = create_model_from_model_vec(&model);

        let sorted_model = reverse_sort(&model, active_tab);

        assert_eq!(sorted_model.row_data(0).unwrap().val_int.iter().collect::<Vec<_>>(), vec![29, 0]);
        assert_eq!(sorted_model.row_data(1).unwrap().val_int.iter().collect::<Vec<_>>(), vec![15, 17]);
        assert_eq!(sorted_model.row_data(2).unwrap().val_int.iter().collect::<Vec<_>>(), vec![14, 50]);
        assert_eq!(sorted_model.row_data(3).unwrap().val_int.iter().collect::<Vec<_>>(), vec![9, 10]);
        assert_eq!(sorted_model.row_data(4).unwrap().val_int.iter().collect::<Vec<_>>(), vec![9, 9]);
    }
}
