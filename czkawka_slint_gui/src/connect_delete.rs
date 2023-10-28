use std::borrow::Borrow;

use slint::{ComponentHandle, Model, ModelRc, VecModel};

use crate::MainListModel;
use crate::{CurrentTab, MainWindow};

pub fn connect_delete_button(app: &MainWindow) {
    let a = app.as_weak();
    app.on_deleted(move || {
        let app = a.upgrade().unwrap();

        let active_tab = app.get_active_tab();

        match active_tab {
            CurrentTab::EmptyFolders => handle_delete_empty_folders(&app),
            _ => panic!(),
        }
    });
}

fn handle_delete_empty_folders(app: &MainWindow) {
    let r = app.get_empty_folder_model();
    let (entries_to_delete, mut entries_left) = filter_out_checked_items(r.borrow(), false);

    if !entries_to_delete.is_empty() {
        remove_selected_items(entries_to_delete);
        deselect_all_items(&mut entries_left);

        let r = ModelRc::new(VecModel::from(entries_left));
        app.set_empty_folder_model(r);
    }
}

// TODO delete in parallel items, consider to add progress bar
fn remove_selected_items(items: Vec<MainListModel>) {
    dbg!(format!("Items to remove {}", items.len()));
    items.into_iter().for_each(|_item| {});
}

fn deselect_all_items(items: &mut [MainListModel]) {
    items.iter_mut().for_each(|item| {
        item.selected_row = false;
    });
}

fn filter_out_checked_items(items: &ModelRc<MainListModel>, have_header: bool) -> (Vec<MainListModel>, Vec<MainListModel>) {
    if cfg!(debug_assertions) {
        check_if_header_is_checked(items);
        check_if_header_is_selected_but_should_not_be(items, have_header);
    }

    let (entries_to_delete, mut entries_left): (Vec<_>, Vec<_>) = items.iter().partition(|item| item.checked);

    if have_header && !entries_left.is_empty() {
        // First row must be header
        assert!(entries_left[0].header_row);

        if entries_left.len() == 3 {
            // First row is header, so if second or third is also header, then there is no enough items to fill model
            if entries_left[1].header_row || entries_left[2].header_row {
                entries_left = Vec::new();
            }
        } else if entries_left.len() < 3 {
            // Not have enough items to fill model
            entries_left = Vec::new();
        } else {
            let mut last_header = 0;
            let mut new_items: Vec<MainListModel> = Vec::new();
            for i in 1..entries_left.len() {
                if entries_left[i].header_row {
                    if i - last_header > 2 {
                        new_items.extend(entries_left[last_header..i].iter().cloned());
                    }
                    last_header = i;
                }
            }
            if entries_left.len() - last_header > 2 {
                new_items.extend(entries_left[last_header..].iter().cloned());
            }

            entries_left = new_items;
        }
    }

    (entries_to_delete, entries_left)
}

// Function to verify if really headers are not checked
// Checked header is big bug
#[cfg(debug_assertions)]
fn check_if_header_is_checked(items: &ModelRc<MainListModel>) {
    for item in items.iter() {
        if item.header_row {
            assert!(!item.checked);
        }
    }
}

// In some modes header should not be visible, but if are, then it is a bug
#[cfg(debug_assertions)]
fn check_if_header_is_selected_but_should_not_be(items: &ModelRc<MainListModel>, can_have_header: bool) {
    if !can_have_header {
        for item in items.iter() {
            assert!(!item.header_row);
        }
    }
}

#[cfg(test)]
mod tests {
    use slint::{Model, ModelRc, SharedString, VecModel};

    use crate::common::MainListModel;
    use crate::connect_delete::filter_out_checked_items;

    #[test]
    fn test_filter_out_checked_items_empty() {
        let vec_items = Vec::new();
        let items: ModelRc<MainListModel> = ModelRc::new(VecModel::from(vec_items));
        let (to_delete, left) = filter_out_checked_items(&items, false);
        assert!(to_delete.is_empty());
        assert!(left.is_empty());
        let (to_delete, left) = filter_out_checked_items(&items, true);
        assert!(to_delete.is_empty());
        assert!(left.is_empty());
    }
    #[test]
    fn test_filter_out_checked_items_one_element_valid_normal() {
        let vec_items = vec![(false, false, false, ModelRc::new(VecModel::default()))];
        let items: ModelRc<MainListModel> = ModelRc::new(VecModel::from(vec_items));
        let (to_delete, left) = filter_out_checked_items(&items, false);
        assert!(to_delete.is_empty());
        assert_eq!(left.len(), items.iter().count());
    }

    #[test]
    fn test_filter_out_checked_items_one_element_valid_header() {
        let vec_items = vec![(false, true, false, ModelRc::new(VecModel::default()))];
        let items: ModelRc<MainListModel> = ModelRc::new(VecModel::from(vec_items));
        let (to_delete, left) = filter_out_checked_items(&items, true);
        assert!(to_delete.is_empty());
        assert!(left.is_empty());
    }

    #[test]
    #[should_panic]
    fn test_filter_out_checked_items_one_element_invalid_normal() {
        let vec_items = vec![(false, true, false, ModelRc::new(VecModel::default()))];
        let items: ModelRc<MainListModel> = ModelRc::new(VecModel::from(vec_items));
        filter_out_checked_items(&items, false);
    }
    #[test]
    #[should_panic]
    fn test_filter_out_checked_items_one_element_invalid_header() {
        let vec_items = vec![(false, false, false, ModelRc::new(VecModel::default()))];
        let items: ModelRc<MainListModel> = ModelRc::new(VecModel::from(vec_items));
        filter_out_checked_items(&items, true);
    }
    //
    // #[test]
    // fn test_filter_out_checked_items_multiple_element_valid_normal() {
    //     let vec_items = vec![
    //         (false, false, false, ModelRc::new(VecModel::from_slice(&[SharedString::from("1")]))),
    //         (false, false, false, ModelRc::new(VecModel::from_slice(&[SharedString::from("2")]))),
    //         (true, false, false, ModelRc::new(VecModel::from_slice(&[SharedString::from("3")]))),
    //         (true, false, false, ModelRc::new(VecModel::from_slice(&[SharedString::from("4")]))),
    //         (false, false, false, ModelRc::new(VecModel::from_slice(&[SharedString::from("5")]))),
    //     ];
    //     let items: ModelRc<MainListModel> = ModelRc::new(VecModel::from(vec_items));
    //     let (to_delete, left) = filter_out_checked_items(&items, false);
    //     let to_delete_data = get_single_data_from_model(&to_delete);
    //     let left_data = get_single_data_from_model(&left);
    //
    //     assert_eq!(to_delete_data, vec!["3", "4"]);
    //     assert_eq!(left_data, vec!["1", "2", "5"]);
    // }
    //
    // #[test]
    // fn test_filter_out_checked_items_multiple_element_valid_header() {
    //     let vec_items = vec![
    //         (false, true, false, ModelRc::new(VecModel::from_slice(&[SharedString::from("1")]))),
    //         (false, false, false, ModelRc::new(VecModel::from_slice(&[SharedString::from("2")]))),
    //         (true, false, false, ModelRc::new(VecModel::from_slice(&[SharedString::from("3")]))),
    //         (false, true, false, ModelRc::new(VecModel::from_slice(&[SharedString::from("4")]))),
    //         (false, false, false, ModelRc::new(VecModel::from_slice(&[SharedString::from("5")]))),
    //         (false, true, false, ModelRc::new(VecModel::from_slice(&[SharedString::from("6")]))),
    //         (false, false, false, ModelRc::new(VecModel::from_slice(&[SharedString::from("7")]))),
    //         (false, false, false, ModelRc::new(VecModel::from_slice(&[SharedString::from("8")]))),
    //     ];
    //     let items: ModelRc<MainListModel> = ModelRc::new(VecModel::from(vec_items));
    //     let (to_delete, left) = filter_out_checked_items(&items, true);
    //     let to_delete_data = get_single_data_from_model(&to_delete);
    //     let left_data = get_single_data_from_model(&left);
    //
    //     assert_eq!(to_delete_data, vec!["3"]);
    //     assert_eq!(left_data, vec!["6", "7", "8"]);
    // }
    //
    // #[test]
    // fn test_filter_out_checked_items_multiple2_element_valid_header() {
    //     let vec_items = vec![
    //         (false, true, false, ModelRc::new(VecModel::from_slice(&[SharedString::from("1")]))),
    //         (false, false, false, ModelRc::new(VecModel::from_slice(&[SharedString::from("2")]))),
    //         (true, false, false, ModelRc::new(VecModel::from_slice(&[SharedString::from("3")]))),
    //         (false, false, false, ModelRc::new(VecModel::from_slice(&[SharedString::from("4")]))),
    //         (false, false, false, ModelRc::new(VecModel::from_slice(&[SharedString::from("5")]))),
    //         (false, false, false, ModelRc::new(VecModel::from_slice(&[SharedString::from("6")]))),
    //         (false, true, false, ModelRc::new(VecModel::from_slice(&[SharedString::from("7")]))),
    //         (false, false, false, ModelRc::new(VecModel::from_slice(&[SharedString::from("8")]))),
    //     ];
    //     let items: ModelRc<MainListModel> = ModelRc::new(VecModel::from(vec_items));
    //     let (to_delete, left) = filter_out_checked_items(&items, true);
    //     let to_delete_data = get_single_data_from_model(&to_delete);
    //     let left_data = get_single_data_from_model(&left);
    //
    //     assert_eq!(to_delete_data, vec!["3"]);
    //     assert_eq!(left_data, vec!["1", "2", "4", "5", "6"]);
    // }

    fn get_single_data_from_model(model: &[MainListModel]) -> Vec<String> {
        let mut d = model.iter().map(|item| item.data.iter().next().unwrap().to_string()).collect::<Vec<_>>();
        d.sort();
        d
    }
}
