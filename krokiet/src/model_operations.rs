use std::path::MAIN_SEPARATOR;

use slint::{Model, ModelRc, SharedString};

use crate::common::{get_str_name_idx, get_str_path_idx, get_str_proper_extension};
use crate::{CurrentTab, MainListModel};

pub fn deselect_all_items(items: &mut [MainListModel]) {
    for item in items {
        item.checked = false;
    }
}

#[allow(unused)]
pub fn select_all_items(items: &mut [MainListModel]) {
    for item in items {
        if !item.header_row {
            item.checked = true;
        }
    }
}

pub fn collect_full_path_from_model(items: &[MainListModel], active_tab: CurrentTab) -> Vec<String> {
    let path_idx = get_str_path_idx(active_tab);
    let name_idx = get_str_name_idx(active_tab);
    items
        .iter()
        .map(|item| {
            let path = get_shared_str_item(item, path_idx);
            let name = get_shared_str_item(item, name_idx);
            format!("{path}{MAIN_SEPARATOR}{name}")
        })
        .collect::<Vec<_>>()
}
pub fn collect_path_name_from_model(items: &[MainListModel], active_tab: CurrentTab) -> Vec<(String, String)> {
    let path_idx = get_str_path_idx(active_tab);
    let name_idx = get_str_name_idx(active_tab);
    items.iter().map(|item| (get_str_item(item, path_idx), get_str_item(item, name_idx))).collect::<Vec<_>>()
}

pub fn collect_path_name_and_proper_extension_from_model(items: &[MainListModel], active_tab: CurrentTab) -> Vec<(String, String, String)> {
    let path_idx = get_str_path_idx(active_tab);
    let name_idx = get_str_name_idx(active_tab);
    let ext_idx = get_str_proper_extension(active_tab);
    items
        .iter()
        .map(|item| (get_str_item(item, path_idx), get_str_item(item, name_idx), get_str_item(item, ext_idx)))
        .collect::<Vec<_>>()
}

#[inline]
pub fn get_str_item(main_list_model: &MainListModel, idx: usize) -> String {
    main_list_model.val_str.iter().nth(idx).unwrap_or_else(|| panic!("Failed to get {idx} element")).to_string()
}
#[inline]
pub fn get_shared_str_item(main_list_model: &MainListModel, idx: usize) -> SharedString {
    main_list_model.val_str.iter().nth(idx).unwrap_or_else(|| panic!("Failed to get {idx} element"))
}

// Removes orphan items in groups
pub fn remove_single_items_in_groups(mut items: Vec<MainListModel>, have_header: bool) -> Vec<MainListModel> {
    // When have header, we must also throw out orphaned items
    if have_header && !items.is_empty() {
        // First row must be header
        // If assert fails, that means, that we checked that for mode that not have headers
        // or that we somehow removed header row, which should be
        assert!(items[0].header_row);
        assert!(!items[0].checked);
        assert!(!items[0].selected_row);
        let is_filled_header = items[0].filled_header_row;

        if is_filled_header && items.len() <= 2 {
            if items.len() == 2 {
                if items[1].header_row {
                    items.clear();
                }
            } else {
                items.clear();
            }
        } else if !is_filled_header && items.len() <= 3 {
            if items.len() == 3 {
                if items[1].header_row || items[2].header_row {
                    items.clear();
                }
            } else {
                items.clear();
            }
        } else {
            let header_step = if is_filled_header { 1 } else { 2 };

            let mut last_header = 0;
            let mut new_items: Vec<MainListModel> = Vec::new();
            for i in 1..items.len() {
                if items[i].header_row {
                    if i - last_header > header_step {
                        new_items.extend(items[last_header..i].iter().cloned());
                    }
                    last_header = i;
                }
            }
            if items.len() - last_header > header_step {
                new_items.extend(items[last_header..].iter().cloned());
            }

            items = new_items;
        }
    }

    items
}

pub fn filter_out_checked_items(items: &ModelRc<MainListModel>, have_header: bool) -> (Vec<MainListModel>, Vec<MainListModel>) {
    if cfg!(debug_assertions) {
        check_if_header_is_checked(items);
        check_if_header_is_selected_but_should_not_be(items, have_header);
    }

    let (entries_to_delete, entries_left): (Vec<_>, Vec<_>) = items.iter().partition(|item| item.checked);

    let entries_left = remove_single_items_in_groups(entries_left, have_header);

    (entries_to_delete, entries_left)
}

// Function to verify if really headers are not checked
// Checked header is big bug
fn check_if_header_is_checked(items: &ModelRc<MainListModel>) {
    if cfg!(debug_assertions) {
        for item in items.iter() {
            if item.header_row {
                assert!(!item.checked);
            }
        }
    }
}

// In some modes header should not be visible, but if are, then it is a bug
fn check_if_header_is_selected_but_should_not_be(items: &ModelRc<MainListModel>, can_have_header: bool) {
    if cfg!(debug_assertions) {
        if !can_have_header {
            for item in items.iter() {
                assert!(!item.header_row);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use slint::{Model, ModelRc, SharedString, VecModel};

    use crate::MainListModel;
    use crate::model_operations::filter_out_checked_items;

    #[test]
    fn test_filter_out_checked_items_empty() {
        let items: ModelRc<MainListModel> = create_new_model(vec![]);

        let (to_delete, left) = filter_out_checked_items(&items, false);
        assert!(to_delete.is_empty());
        assert!(left.is_empty());
        let (to_delete, left) = filter_out_checked_items(&items, true);
        assert!(to_delete.is_empty());
        assert!(left.is_empty());
    }
    #[test]
    fn test_filter_one_simple_header() {
        let items = create_new_model(vec![(false, false, false, false, vec![])]);
        let (to_delete, left) = filter_out_checked_items(&items, false);
        assert!(to_delete.is_empty());
        assert_eq!(left.len(), items.iter().count());
    }

    #[test]
    fn test_filter_one_filled_header() {
        let items = create_new_model(vec![(false, true, true, false, vec![])]);
        let (to_delete, left) = filter_out_checked_items(&items, true);
        assert!(to_delete.is_empty());
        assert!(left.is_empty());
    }

    #[test]
    fn test_filter_one_empty_header() {
        let items = create_new_model(vec![(false, true, false, false, vec![])]);
        let (to_delete, left) = filter_out_checked_items(&items, true);
        assert!(to_delete.is_empty());
        assert!(left.is_empty());
    }

    #[test]
    #[should_panic]
    fn test_filter_invalid_non_header() {
        let items = create_new_model(vec![(false, true, true, false, vec![])]);
        filter_out_checked_items(&items, false);
    }
    #[test]
    #[should_panic]
    fn test_filter_invalid_header() {
        let items = create_new_model(vec![(false, false, true, false, vec![])]);
        filter_out_checked_items(&items, true);
    }

    #[test]
    fn test_filter_filled_header() {
        let items = create_new_model(vec![(false, true, true, false, vec!["1"]), (false, false, false, false, vec!["2"])]);
        let (to_delete, left) = filter_out_checked_items(&items, true);
        let to_delete_data = get_single_data_str_from_model(&to_delete);
        let left_data = get_single_data_str_from_model(&left);

        assert_eq!(to_delete_data, Vec::<String>::new());
        assert_eq!(left_data, vec!["1", "2"]);

        let items = create_new_model(vec![
            (false, true, true, false, vec!["1"]),
            (false, false, false, false, vec!["2"]),
            (false, false, false, false, vec!["3"]),
        ]);
        let (to_delete, left) = filter_out_checked_items(&items, true);
        let to_delete_data = get_single_data_str_from_model(&to_delete);
        let left_data = get_single_data_str_from_model(&left);

        assert_eq!(to_delete_data, Vec::<String>::new());
        assert_eq!(left_data, vec!["1", "2", "3"]);

        let items = create_new_model(vec![
            (false, true, true, false, vec!["1"]),
            (false, false, false, false, vec!["2"]),
            (false, true, true, false, vec!["3"]),
        ]);
        let (to_delete, left) = filter_out_checked_items(&items, true);
        let to_delete_data = get_single_data_str_from_model(&to_delete);
        let left_data = get_single_data_str_from_model(&left);

        assert_eq!(to_delete_data, Vec::<String>::new());
        assert_eq!(left_data, vec!["1", "2"]);

        let items = create_new_model(vec![
            (false, true, true, false, vec!["1"]),
            (true, false, false, false, vec!["2"]),
            (false, false, false, false, vec!["3"]),
        ]);
        let (to_delete, left) = filter_out_checked_items(&items, true);
        let to_delete_data = get_single_data_str_from_model(&to_delete);
        let left_data = get_single_data_str_from_model(&left);

        assert_eq!(to_delete_data, vec!["2"]);
        assert_eq!(left_data, vec!["1", "3"]);

        let items = create_new_model(vec![
            (false, true, true, false, vec!["1"]),
            (false, false, false, false, vec!["2"]),
            (false, false, false, false, vec!["3"]),
            (false, false, false, false, vec!["4"]),
            (false, true, true, false, vec!["5"]),
            (false, false, false, false, vec!["6"]),
            (false, false, false, false, vec!["7"]),
            (false, true, true, false, vec!["8"]),
            (false, false, false, false, vec!["9"]),
            (false, true, true, false, vec!["10"]),
        ]);
        let (to_delete, left) = filter_out_checked_items(&items, true);
        let to_delete_data = get_single_data_str_from_model(&to_delete);
        let left_data = get_single_data_str_from_model(&left);

        assert_eq!(to_delete_data, Vec::<String>::new());
        assert_eq!(left_data, vec!["1", "2", "3", "4", "5", "6", "7", "8", "9"]);

        for i in 1..20 {
            let mut vec_items = vec![(false, true, true, false, vec!["First"])];
            for j in i..21 {
                let is_header = (j - i) % 5 == 0;
                let item = if is_header {
                    (false, true, true, false, vec!["Header"])
                } else {
                    (false, false, false, false, vec!["Non header"])
                };
                vec_items.push(item);
            }
            filter_out_checked_items(&create_new_model(vec_items), true);
        }
    }

    #[test]
    fn test_filter_empty_header() {
        let items = create_new_model(vec![(false, true, false, false, vec!["1"]), (false, false, false, false, vec!["2"])]);
        let (to_delete, left) = filter_out_checked_items(&items, true);
        let to_delete_data = get_single_data_str_from_model(&to_delete);
        let left_data = get_single_data_str_from_model(&left);

        assert_eq!(to_delete_data, Vec::<String>::new());
        assert_eq!(left_data, Vec::<String>::new());

        let items = create_new_model(vec![
            (false, true, false, false, vec!["1"]),
            (false, false, false, false, vec!["2"]),
            (false, false, false, false, vec!["3"]),
        ]);
        let (to_delete, left) = filter_out_checked_items(&items, true);
        let to_delete_data = get_single_data_str_from_model(&to_delete);
        let left_data = get_single_data_str_from_model(&left);

        assert_eq!(to_delete_data, Vec::<String>::new());
        assert_eq!(left_data, vec!["1", "2", "3"]);

        let items = create_new_model(vec![
            (false, true, false, false, vec!["1"]),
            (false, false, false, false, vec!["2"]),
            (false, true, false, false, vec!["3"]),
        ]);
        let (to_delete, left) = filter_out_checked_items(&items, true);
        let to_delete_data = get_single_data_str_from_model(&to_delete);
        let left_data = get_single_data_str_from_model(&left);

        assert_eq!(to_delete_data, Vec::<String>::new());
        assert_eq!(left_data, Vec::<String>::new());

        let items = create_new_model(vec![
            (false, true, false, false, vec!["1"]),
            (true, false, false, false, vec!["2"]),
            (false, false, false, false, vec!["3"]),
        ]);
        let (to_delete, left) = filter_out_checked_items(&items, true);
        let to_delete_data = get_single_data_str_from_model(&to_delete);
        let left_data = get_single_data_str_from_model(&left);

        assert_eq!(to_delete_data, vec!["2"]);
        assert_eq!(left_data, Vec::<String>::new());

        let items = create_new_model(vec![
            (false, true, false, false, vec!["1"]),
            (false, false, false, false, vec!["2"]),
            (false, false, false, false, vec!["3"]),
            (false, false, false, false, vec!["4"]),
            (false, true, false, false, vec!["5"]),
            (false, false, false, false, vec!["6"]),
            (false, false, false, false, vec!["7"]),
            (false, true, false, false, vec!["8"]),
            (false, false, false, false, vec!["9"]),
            (false, true, false, false, vec!["10"]),
        ]);
        let (to_delete, left) = filter_out_checked_items(&items, true);
        let to_delete_data = get_single_data_str_from_model(&to_delete);
        let left_data = get_single_data_str_from_model(&left);

        assert_eq!(to_delete_data, Vec::<String>::new());
        assert_eq!(left_data, vec!["1", "2", "3", "4", "5", "6", "7"]);

        for i in 1..20 {
            let mut vec_items = vec![(false, true, false, false, vec!["First"])];
            for j in i..21 {
                let is_header = (j - i) % 5 == 0;
                let item = if is_header {
                    (false, true, false, false, vec!["Header"])
                } else {
                    (false, false, false, false, vec!["Non header"])
                };
                vec_items.push(item);
            }
            filter_out_checked_items(&create_new_model(vec_items), true);
        }
    }
    fn get_single_data_str_from_model(model: &[MainListModel]) -> Vec<String> {
        let mut d = model
            .iter()
            .map(|item| item.val_str.iter().next().expect("Failed to get first element").to_string())
            .collect::<Vec<_>>();
        d.sort();
        d
    }

    fn create_new_model(items: Vec<(bool, bool, bool, bool, Vec<&'static str>)>) -> ModelRc<MainListModel> {
        let model = VecModel::default();
        for item in items {
            let all_items: Vec<SharedString> = item.4.iter().map(|item| (*item).into()).collect::<Vec<_>>();
            let all_items = VecModel::from(all_items);
            if item.2 {
                assert!(item.1); // Header must be set when full header is set
            }
            model.push(MainListModel {
                checked: item.0,
                header_row: item.1,
                filled_header_row: item.2,
                selected_row: item.3,
                val_str: ModelRc::new(all_items),
                val_int: ModelRc::new(VecModel::default()),
            });
        }
        ModelRc::new(model)
    }
}
