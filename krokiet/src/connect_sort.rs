use std::mem;

use slint::{ComponentHandle, Model, ModelRc, VecModel};

use crate::common::connect_i32_into_u64;
use crate::connect_row_selection::recalculate_small_selection_if_needed;
use crate::connect_translation::translate_sort_mode;
use crate::{ActiveTab, Callabler, GuiState, MainListModel, MainWindow, SortMode, SortModel};

pub(crate) fn connect_sort(app: &MainWindow) {
    let a = app.as_weak();
    app.global::<Callabler>().on_sort_items(move |sort_mode| {
        let app = a.upgrade().expect("Failed to upgrade app :(");
        let active_tab = app.global::<GuiState>().get_active_tab();
        let current_model = active_tab.get_tool_model(&app);

        let new_model = match sort_mode {
            SortMode::Size => sorts::sort_by_size(&current_model, active_tab),
            SortMode::ParentName => sorts::sort_by_parent_name(&current_model, active_tab),
            SortMode::ItemName => sorts::sort_by_name(&current_model, active_tab),
            SortMode::FullName => sorts::sort_by_full_name(&current_model, active_tab),
            SortMode::ModificationDate => sorts::sort_modification_date(&current_model, active_tab),
            SortMode::Selection => sorts::sort_selection(&current_model, active_tab),
            SortMode::Reverse => sorts::reverse_sort(&current_model, active_tab),
            SortMode::Checked => sorts::sort_checked(&current_model, active_tab),
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
    let active_tab = app.global::<GuiState>().get_active_tab();
    let mut base_buttons = vec![
        SortMode::Checked,
        SortMode::FullName,
        SortMode::ItemName,
        SortMode::ModificationDate,
        SortMode::ParentName,
        SortMode::Reverse,
        SortMode::Selection,
    ];

    let additional_buttons = match active_tab.get_int_size_opt_idx() {
        Some(_) => vec![SortMode::Size],
        None => vec![],
    };

    base_buttons.extend(additional_buttons);
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
        ActiveTab, MainListModel, Model, ModelRc, VecModel, common_sort_function, connect_i32_into_u64, convert_group_header_into_rc_model, group_by_header,
        recalculate_small_selection_if_needed,
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

    pub(super) fn sort_checked(model: &ModelRc<MainListModel>, active_tab: ActiveTab) -> ModelRc<MainListModel> {
        let sort_function = |e: &MainListModel| !e.checked;

        common_sort_function(model, active_tab, sort_function)
    }

    pub(super) fn sort_selection(model: &ModelRc<MainListModel>, active_tab: ActiveTab) -> ModelRc<MainListModel> {
        let sort_function = |e: &MainListModel| !e.selected_row;

        common_sort_function(model, active_tab, sort_function)
    }

    pub(super) fn sort_modification_date(model: &ModelRc<MainListModel>, active_tab: ActiveTab) -> ModelRc<MainListModel> {
        let sort_function = |e: &MainListModel| {
            let modification_date = active_tab.get_int_modification_date_idx();
            let items = e.val_int.iter().collect::<Vec<_>>();
            connect_i32_into_u64(items[modification_date], items[modification_date + 1])
        };

        common_sort_function(model, active_tab, sort_function)
    }

    pub(super) fn sort_by_full_name(model: &ModelRc<MainListModel>, active_tab: ActiveTab) -> ModelRc<MainListModel> {
        let sort_function = |e: &MainListModel| {
            let name_idx = active_tab.get_str_name_idx();
            let path_idx = active_tab.get_str_path_idx();
            let items = e.val_str.iter().collect::<Vec<_>>();
            format!("{}/{}", items[path_idx], items[name_idx])
        };

        common_sort_function(model, active_tab, sort_function)
    }

    pub(super) fn sort_by_name(model: &ModelRc<MainListModel>, active_tab: ActiveTab) -> ModelRc<MainListModel> {
        let sort_function = |e: &MainListModel| {
            let name_idx = active_tab.get_str_name_idx();
            e.val_str
                .iter()
                .nth(name_idx)
                .unwrap_or_else(|| panic!("Failed to get name index - {name_idx} on {} items", e.val_str.iter().count()))
        };

        common_sort_function(model, active_tab, sort_function)
    }
    pub(super) fn sort_by_parent_name(model: &ModelRc<MainListModel>, active_tab: ActiveTab) -> ModelRc<MainListModel> {
        let sort_function = |e: &MainListModel| {
            let path_idx = active_tab.get_str_path_idx();
            e.val_str
                .iter()
                .nth(path_idx)
                .unwrap_or_else(|| panic!("Failed to get name index - {path_idx} on {} items", e.val_str.iter().count()))
        };

        common_sort_function(model, active_tab, sort_function)
    }

    pub(super) fn sort_by_size(model: &ModelRc<MainListModel>, active_tab: ActiveTab) -> ModelRc<MainListModel> {
        let sort_function = |e: &MainListModel| {
            let size_idx = active_tab.get_int_size_idx();
            let items = e.val_int.iter().collect::<Vec<_>>();
            connect_i32_into_u64(items[size_idx], items[size_idx + 1])
        };

        common_sort_function(model, active_tab, sort_function)
    }
}

fn common_sort_function<T: Ord>(model: &ModelRc<MainListModel>, active_tab: ActiveTab, sort_function: impl Fn(&MainListModel) -> T) -> ModelRc<MainListModel> {
    if !active_tab.get_is_header_mode() {
        let mut items = model.iter().collect::<Vec<_>>();
        items.sort_by_cached_key(&sort_function);
        let new_model = ModelRc::new(VecModel::from(items));
        recalculate_small_selection_if_needed(&new_model, active_tab);
        return new_model;
    }

    let mut grouped_items = group_by_header(model);
    for (_, items) in &mut grouped_items {
        items.sort_by_cached_key(&sort_function);
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
    use crate::connect_sort::sorts::{reverse_sort, sort_by_full_name, sort_by_name, sort_by_parent_name, sort_by_size, sort_checked, sort_modification_date, sort_selection};
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
    fn sort_by_size_sorts_flat_model_correctly() {
        initialize_selection_struct();
        let active_tab = ActiveTab::BigFiles;
        // To be sure that we set correct values in val_int, which must be equal to index
        assert_eq!(active_tab.get_int_size_idx(), 2);
        assert!(!active_tab.get_is_header_mode());

        let mut model = get_model_vec(3);
        model[0].val_int = create_model_from_model_vec(&[0, 0, 0, 10]);
        model[1].val_int = create_model_from_model_vec(&[0, 0, 0, 5]);
        model[2].val_int = create_model_from_model_vec(&[0, 0, 0, 20]);
        let model = create_model_from_model_vec(&model);

        let sorted_model = sort_by_size(&model, active_tab);

        assert_eq!(sorted_model.row_data(0).unwrap().val_int.iter().skip(2).collect::<Vec<_>>(), vec![0, 5]); // smallest
        assert_eq!(sorted_model.row_data(1).unwrap().val_int.iter().skip(2).collect::<Vec<_>>(), vec![0, 10]); // middle
        assert_eq!(sorted_model.row_data(2).unwrap().val_int.iter().skip(2).collect::<Vec<_>>(), vec![0, 20]); // largest
    }

    #[test]
    fn sort_by_size_sorts_grouped_model_correctly() {
        initialize_selection_struct();
        let active_tab = ActiveTab::SimilarImages;
        // To be sure that we set correct values in val_int, which must be equal to index
        assert_eq!(active_tab.get_int_size_idx(), 2);
        assert!(active_tab.get_is_header_mode());

        let mut model = get_model_vec(7);
        model[0].header_row = true;
        model[1].val_int = create_model_from_model_vec(&[0, 0, 0, 15]);
        model[2].val_int = create_model_from_model_vec(&[0, 0, 0, 5]);
        model[3].header_row = true;
        model[4].val_int = create_model_from_model_vec(&[0, 0, 1, 15]);
        model[5].val_int = create_model_from_model_vec(&[0, 0, 0, 10]);
        model[6].val_int = create_model_from_model_vec(&[0, 0, 0, 35]);
        let model = create_model_from_model_vec(&model);

        let sorted_model = sort_by_size(&model, active_tab);

        // Group 1
        assert!(sorted_model.row_data(0).unwrap().header_row);
        assert_eq!(sorted_model.row_data(1).unwrap().val_int.iter().skip(2).collect::<Vec<_>>(), vec![0, 5]);
        assert_eq!(sorted_model.row_data(2).unwrap().val_int.iter().skip(2).collect::<Vec<_>>(), vec![0, 15]);
        // Group 2
        assert!(sorted_model.row_data(3).unwrap().header_row);
        assert_eq!(sorted_model.row_data(4).unwrap().val_int.iter().skip(2).collect::<Vec<_>>(), vec![0, 10]);
        assert_eq!(sorted_model.row_data(5).unwrap().val_int.iter().skip(2).collect::<Vec<_>>(), vec![0, 35]);
        assert_eq!(sorted_model.row_data(6).unwrap().val_int.iter().skip(2).collect::<Vec<_>>(), vec![1, 15]);
    }

    #[test]
    fn sort_by_size_handles_empty_model() {
        initialize_selection_struct();
        let model = create_model_from_model_vec(&[]);

        let sorted_model = sort_by_size(&model, ActiveTab::SimilarImages);

        assert_eq!(sorted_model.row_count(), 0);
    }

    #[test]
    fn sort_by_parent_name_sorts_flat_model_correctly() {
        initialize_selection_struct();
        let active_tab = ActiveTab::BigFiles;
        // To be sure that we set correct values in val_int, which must be equal to index
        assert_eq!(active_tab.get_str_path_idx(), 2);
        assert!(!active_tab.get_is_header_mode());

        let mut model = get_model_vec(5);
        model[0].val_str = create_model_from_model_vec(&["".into(), "".into(), "E".into()]);
        model[1].val_str = create_model_from_model_vec(&["".into(), "".into(), "C".into()]);
        model[2].val_str = create_model_from_model_vec(&["".into(), "".into(), "D".into()]);
        model[3].val_str = create_model_from_model_vec(&["".into(), "".into(), "A".into()]);
        model[4].val_str = create_model_from_model_vec(&["".into(), "".into(), "B".into()]);
        let model = create_model_from_model_vec(&model);

        let sorted_model = sort_by_parent_name(&model, active_tab);

        assert_eq!(sorted_model.row_data(0).unwrap().val_str.iter().skip(2).collect::<Vec<_>>(), vec!["A"]);
        assert_eq!(sorted_model.row_data(1).unwrap().val_str.iter().skip(2).collect::<Vec<_>>(), vec!["B"]);
        assert_eq!(sorted_model.row_data(2).unwrap().val_str.iter().skip(2).collect::<Vec<_>>(), vec!["C"]);
        assert_eq!(sorted_model.row_data(3).unwrap().val_str.iter().skip(2).collect::<Vec<_>>(), vec!["D"]);
        assert_eq!(sorted_model.row_data(4).unwrap().val_str.iter().skip(2).collect::<Vec<_>>(), vec!["E"]);
    }

    #[test]
    fn sort_by_name_sorts_flat_model_correctly() {
        initialize_selection_struct();
        let active_tab = ActiveTab::BigFiles;
        // To be sure that we set correct values in val_int, which must be equal to index
        assert_eq!(active_tab.get_str_name_idx(), 1);
        assert!(!active_tab.get_is_header_mode());

        let mut model = get_model_vec(5);
        model[0].val_str = create_model_from_model_vec(&["".into(), "E".into()]);
        model[1].val_str = create_model_from_model_vec(&["".into(), "C".into()]);
        model[2].val_str = create_model_from_model_vec(&["".into(), "D".into()]);
        model[3].val_str = create_model_from_model_vec(&["".into(), "A".into()]);
        model[4].val_str = create_model_from_model_vec(&["".into(), "B".into()]);
        let model = create_model_from_model_vec(&model);

        let sorted_model = sort_by_name(&model, active_tab);

        assert_eq!(sorted_model.row_data(0).unwrap().val_str.iter().skip(1).collect::<Vec<_>>(), vec!["A"]);
        assert_eq!(sorted_model.row_data(1).unwrap().val_str.iter().skip(1).collect::<Vec<_>>(), vec!["B"]);
        assert_eq!(sorted_model.row_data(2).unwrap().val_str.iter().skip(1).collect::<Vec<_>>(), vec!["C"]);
        assert_eq!(sorted_model.row_data(3).unwrap().val_str.iter().skip(1).collect::<Vec<_>>(), vec!["D"]);
        assert_eq!(sorted_model.row_data(4).unwrap().val_str.iter().skip(1).collect::<Vec<_>>(), vec!["E"]);
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
    fn sort_by_modification_date_sorts_flat_model_correctly() {
        initialize_selection_struct();
        let active_tab = ActiveTab::BigFiles;
        // To be sure that we set correct values in val_int, which must be equal to index
        assert_eq!(active_tab.get_int_modification_date_idx(), 0);
        assert!(!active_tab.get_is_header_mode());

        let mut model = get_model_vec(5);
        model[0].val_int = create_model_from_model_vec(&[15, 17]);
        model[1].val_int = create_model_from_model_vec(&[14, 50]);
        model[2].val_int = create_model_from_model_vec(&[9, 10]);
        model[3].val_int = create_model_from_model_vec(&[9, 9]);
        model[4].val_int = create_model_from_model_vec(&[29, 0]);
        let model = create_model_from_model_vec(&model);

        let sorted_model = sort_modification_date(&model, active_tab);

        assert_eq!(sorted_model.row_data(0).unwrap().val_int.iter().collect::<Vec<_>>(), vec![9, 9]);
        assert_eq!(sorted_model.row_data(1).unwrap().val_int.iter().collect::<Vec<_>>(), vec![9, 10]);
        assert_eq!(sorted_model.row_data(2).unwrap().val_int.iter().collect::<Vec<_>>(), vec![14, 50]);
        assert_eq!(sorted_model.row_data(3).unwrap().val_int.iter().collect::<Vec<_>>(), vec![15, 17]);
        assert_eq!(sorted_model.row_data(4).unwrap().val_int.iter().collect::<Vec<_>>(), vec![29, 0]);
    }

    #[test]
    fn sort_by_checked_sorts_flat_model_correctly() {
        initialize_selection_struct();
        let active_tab = ActiveTab::BigFiles;
        // To be sure that we set correct values in val_int, which must be equal to index
        assert!(!active_tab.get_is_header_mode());

        let mut model = get_model_vec(4);
        model[0].checked = true;
        model[0].val_int = create_model_from_model_vec(&[15]);
        model[1].checked = false;
        model[1].val_int = create_model_from_model_vec(&[14]);
        model[2].checked = true;
        model[2].val_int = create_model_from_model_vec(&[9]);
        model[3].checked = false;
        model[3].val_int = create_model_from_model_vec(&[29]);
        let model = create_model_from_model_vec(&model);

        let sorted_model = sort_checked(&model, active_tab);

        assert!(sorted_model.row_data(0).unwrap().checked);
        assert_eq!(sorted_model.row_data(0).unwrap().val_int.iter().collect::<Vec<_>>(), vec![15]);
        assert!(sorted_model.row_data(1).unwrap().checked);
        assert_eq!(sorted_model.row_data(1).unwrap().val_int.iter().collect::<Vec<_>>(), vec![9]);
        assert!(!sorted_model.row_data(2).unwrap().checked);
        assert_eq!(sorted_model.row_data(2).unwrap().val_int.iter().collect::<Vec<_>>(), vec![14]);
        assert!(!sorted_model.row_data(3).unwrap().checked);
        assert_eq!(sorted_model.row_data(3).unwrap().val_int.iter().collect::<Vec<_>>(), vec![29]);
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
