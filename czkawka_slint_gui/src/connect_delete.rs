use crate::common::ModelType;
use crate::MainWindow;
use slint::{ComponentHandle, Model, ModelRc, VecModel};
use std::borrow::Borrow;

pub fn connect_delete_button(app: &MainWindow) {
    let a = app.as_weak();
    app.on_deleted(move || {
        let app = a.upgrade().unwrap();

        let r = app.get_empty_folder_model();
        let m = r.borrow();
        let (entries_to_delete, mut entries_left): (Vec<ModelType>, Vec<ModelType>) = m.iter().partition(|(checked, _header_row, _selected_row, _data)| *checked);

        if !entries_to_delete.is_empty() {
            dbg!(format!("Items to remove {}", entries_to_delete.len()));
            entries_to_delete.into_iter().for_each(|(_checked, _header_row, _selected_row, _data)| {
                // TODO delete in parallel items, consider to add progress bar
            });
            entries_left.iter_mut().for_each(|(_checked, _header_row, selected_row, _data)| {
                *selected_row = false;
            });
            let r = ModelRc::new(VecModel::from(entries_left));
            app.set_empty_folder_model(r);
        }
    });
}

fn filter_out_checked_items(items: &ModelRc<ModelType>, have_header: bool) -> (Vec<ModelType>, Vec<ModelType>) {
    if cfg!(debug_assertions) {
        check_if_header_is_checked(items);
        check_if_header_is_selected_but_should_not_be(items, have_header);
    }

    let (entries_to_delete, mut entries_left): (Vec<_>, Vec<_>) = items.iter().partition(|(checked, _header_row, _selected_row, _data)| *checked);

    if have_header && !entries_left.is_empty() {
        // First row must be header
        assert!(entries_left[0].1);

        if entries_left.len() == 3 {
            // First row is header, so if second or third is also header, then there is no enough items to fill model
            if entries_left[1].1 || entries_left[2].1 {
                entries_left = Vec::new();
            }
        } else if entries_left.len() < 3 {
            // Not have enough items to fill model
            entries_left = Vec::new();
        } else {
            let mut last_header = 0;
            let mut new_items: Vec<ModelType> = Vec::new();
            for i in 1..entries_left.len() {
                if entries_left[i].1 {
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
fn check_if_header_is_checked(items: &ModelRc<ModelType>) {
    for i in items.iter() {
        let (checked, header_row, _selected_row, _data) = i;
        if header_row {
            assert!(!checked);
        }
    }
}

// In some modes header should not be visible
#[cfg(debug_assertions)]
fn check_if_header_is_selected_but_should_not_be(items: &ModelRc<ModelType>, can_have_header: bool) {
    if !can_have_header {
        for i in items.iter() {
            let (_checked, header_row, _selected_row, _data) = i;
            assert!(!header_row);
        }
    }
}

#[test]
fn test_filter_out_checked_items_empty() {
    let vec_items = Vec::new();
    let items: ModelRc<ModelType> = ModelRc::new(VecModel::from(vec_items));
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
    let items: ModelRc<ModelType> = ModelRc::new(VecModel::from(vec_items));
    let (to_delete, left) = filter_out_checked_items(&items, false);
    assert!(to_delete.is_empty());
    assert_eq!(left.len(), items.iter().count());
}

#[test]
fn test_filter_out_checked_items_one_element_valid_header() {
    let vec_items = vec![(false, true, false, ModelRc::new(VecModel::default()))];
    let items: ModelRc<ModelType> = ModelRc::new(VecModel::from(vec_items));
    let (to_delete, left) = filter_out_checked_items(&items, true);
    assert!(to_delete.is_empty());
    assert_eq!(left.len(), items.iter().count());
}

#[test]
#[should_panic]
fn test_filter_out_checked_items_one_element_invalid_normal() {
    let vec_items = vec![(false, true, false, ModelRc::new(VecModel::default()))];
    let items: ModelRc<ModelType> = ModelRc::new(VecModel::from(vec_items));
    filter_out_checked_items(&items, false);
}
#[test]
#[should_panic]
fn test_filter_out_checked_items_one_element_invalid_header() {
    let vec_items = vec![(false, false, false, ModelRc::new(VecModel::default()))];
    let items: ModelRc<ModelType> = ModelRc::new(VecModel::from(vec_items));
    filter_out_checked_items(&items, true);
}
