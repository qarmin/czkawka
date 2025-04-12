use std::mem;

use slint::{ComponentHandle, Model, ModelRc, VecModel};

use crate::common::{connect_i32_into_u64, get_int_size_idx, get_is_header_mode, get_str_name_idx, get_tool_model, set_tool_model};
use crate::{Callabler, CurrentTab, GuiState, MainListModel, MainWindow, SortMode};

pub fn connect_sort(app: &MainWindow) {
    let a = app.as_weak();
    app.global::<Callabler>().on_sort_items(move |sort_mode| {
        let app = a.upgrade().expect("Failed to upgrade app :(");
        let active_tab = app.global::<GuiState>().get_active_tab();
        let current_model = get_tool_model(&app, active_tab);

        let new_model = match sort_mode {
            SortMode::Size => sort_by_size(&current_model, active_tab),
            SortMode::ParentName => sort_by_parent_name(&current_model, active_tab),
            _ => todo!(),
        };

        set_tool_model(&app, active_tab, new_model);
    });
}

fn sort_by_parent_name(model: &ModelRc<MainListModel>, active_tab: CurrentTab) -> ModelRc<MainListModel> {
    let sort_function = |e: &MainListModel| {
        let name_idx = get_str_name_idx(active_tab);
        e.val_str.iter().nth(name_idx).expect("Failed to get name index")
    };

    common_sort_function(model, active_tab, sort_function)
}

fn sort_by_size(model: &ModelRc<MainListModel>, active_tab: CurrentTab) -> ModelRc<MainListModel> {
    let sort_function = |e: &MainListModel| {
        let size_idx = get_int_size_idx(active_tab);
        let items = e.val_int.iter().collect::<Vec<_>>();
        connect_i32_into_u64(items[size_idx], items[size_idx + 1])
    };

    common_sort_function(model, active_tab, sort_function)
}

fn common_sort_function<T: Ord>(model: &ModelRc<MainListModel>, active_tab: CurrentTab, sort_function: impl Fn(&MainListModel) -> T) -> ModelRc<MainListModel> {
    if !get_is_header_mode(active_tab) {
        let mut items = model.iter().collect::<Vec<_>>();
        items.sort_by_cached_key(&sort_function);
        return ModelRc::new(VecModel::from(items));
    }

    let mut grouped_items = group_by_header(model);
    for (_, items) in &mut grouped_items {
        items.sort_by_cached_key(&sort_function);
    }

    convert_group_header_into_rc_model(grouped_items, model.row_count())
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

    use crate::common::{get_int_size_idx, get_is_header_mode};
    use crate::connect_sort::{convert_group_header_into_rc_model, group_by_header, sort_by_size};
    use crate::test_common::{create_model_from_model_vec, get_model_vec};
    use crate::{CurrentTab, MainListModel};

    #[test]
    fn group_by_header_splits_items_into_groups_correctly() {
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
        let model = create_model_from_model_vec(&[]);

        let grouped = group_by_header(&model);

        assert!(grouped.is_empty());
    }

    #[test]
    #[should_panic]
    fn group_by_header_panics_when_no_header_before_items() {
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
        let mut model = get_model_vec(3);
        model[0].header_row = true;
        model[1].header_row = true;
        model[2].header_row = true;
        let model = create_model_from_model_vec(&model);

        group_by_header(&model);
    }

    #[test]
    fn convert_group_header_into_rc_model_combines_groups_correctly() {
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
        let grouped: Vec<(MainListModel, Vec<MainListModel>)> = vec![];

        let combined_model = convert_group_header_into_rc_model(grouped, 0);

        assert_eq!(combined_model.row_count(), 0);
    }

    #[test]
    fn sort_by_size_sorts_flat_model_correctly() {
        let current_tab = CurrentTab::BigFiles;
        // To be sure that we set correct values in val_int, which must be equal to index
        assert_eq!(get_int_size_idx(current_tab), 2);
        assert!(!get_is_header_mode(current_tab));

        let mut model = get_model_vec(3);
        model[0].val_int = create_model_from_model_vec(&[0, 0, 0, 10]);
        model[1].val_int = create_model_from_model_vec(&[0, 0, 0, 5]);
        model[2].val_int = create_model_from_model_vec(&[0, 0, 0, 20]);
        let model = create_model_from_model_vec(&model);

        let sorted_model = sort_by_size(&model, current_tab);

        assert_eq!(sorted_model.row_data(0).unwrap().val_int.iter().skip(2).collect::<Vec<_>>(), vec![0, 5]); // smallest
        assert_eq!(sorted_model.row_data(1).unwrap().val_int.iter().skip(2).collect::<Vec<_>>(), vec![0, 10]); // middle
        assert_eq!(sorted_model.row_data(2).unwrap().val_int.iter().skip(2).collect::<Vec<_>>(), vec![0, 20]); // largest
    }

    #[test]
    fn sort_by_size_sorts_grouped_model_correctly() {
        let current_tab = CurrentTab::SimilarImages;
        // To be sure that we set correct values in val_int, which must be equal to index
        assert_eq!(get_int_size_idx(current_tab), 2);
        assert!(get_is_header_mode(current_tab));

        let mut model = get_model_vec(6);
        model[0].header_row = true;
        model[1].val_int = create_model_from_model_vec(&[0, 0, 0, 15]);
        model[2].val_int = create_model_from_model_vec(&[0, 0, 0, 5]);
        model[3].header_row = true;
        model[4].val_int = create_model_from_model_vec(&[0, 0, 0, 25]);
        model[5].val_int = create_model_from_model_vec(&[0, 0, 0, 10]);
        let model = create_model_from_model_vec(&model);

        let sorted_model = sort_by_size(&model, current_tab);

        // Group 1
        assert!(sorted_model.row_data(0).unwrap().header_row);
        assert_eq!(sorted_model.row_data(1).unwrap().val_int.iter().skip(2).collect::<Vec<_>>(), vec![0, 5]); // smallest
        assert_eq!(sorted_model.row_data(2).unwrap().val_int.iter().skip(2).collect::<Vec<_>>(), vec![0, 15]); // largest
        // Group 2
        assert!(sorted_model.row_data(3).unwrap().header_row);
        assert_eq!(sorted_model.row_data(4).unwrap().val_int.iter().skip(2).collect::<Vec<_>>(), vec![0, 10]); // smallest
        assert_eq!(sorted_model.row_data(5).unwrap().val_int.iter().skip(2).collect::<Vec<_>>(), vec![0, 25]); // largest
    }

    #[test]
    fn sort_by_size_handles_empty_model() {
        let model = create_model_from_model_vec(&[]);

        let sorted_model = sort_by_size(&model, CurrentTab::SimilarImages);

        assert_eq!(sorted_model.row_count(), 0);
    }
}
