use rayon::prelude::*;
use slint::{ComponentHandle, Model, ModelRc, VecModel};
use std::path::MAIN_SEPARATOR;

use czkawka_core::common::remove_folder_if_contains_only_empty_folders;

use crate::common::{get_is_header_mode, get_name_idx, get_path_idx};
use crate::{Callabler, CurrentTab, GuiState, MainListModel, MainWindow};

pub fn connect_delete_button(app: &MainWindow) {
    let a = app.as_weak();
    app.global::<Callabler>().on_delete_selected_items(move || {
        let app = a.upgrade().unwrap();

        let active_tab = app.global::<GuiState>().get_active_tab();

        let model = match active_tab {
            CurrentTab::EmptyFolders => app.get_empty_folder_model(),
            CurrentTab::SimilarImages => app.get_similar_images_model(),
            CurrentTab::EmptyFiles => app.get_empty_files_model(),
            CurrentTab::Settings => panic!("Button should be disabled"),
        };

        let new_model = handle_delete_items(&model, active_tab);

        if let Some(new_model) = new_model {
            match active_tab {
                CurrentTab::EmptyFolders => app.set_empty_folder_model(new_model),
                CurrentTab::SimilarImages => app.set_similar_images_model(new_model),
                CurrentTab::EmptyFiles => app.set_empty_files_model(new_model),
                CurrentTab::Settings => panic!("Button should be disabled"),
            }
        }

        app.global::<GuiState>().set_preview_visible(false);
    });
}

fn handle_delete_items(items: &ModelRc<MainListModel>, active_tab: CurrentTab) -> Option<ModelRc<MainListModel>> {
    let (entries_to_delete, mut entries_left) = filter_out_checked_items(items, get_is_header_mode(active_tab));

    if !entries_to_delete.is_empty() {
        remove_selected_items(entries_to_delete, active_tab);
        deselect_all_items(&mut entries_left);

        let r = ModelRc::new(VecModel::from(entries_left)); // TODO here maybe should also stay old model if entries cannot be removed
        return Some(r);
    }
    None
}

// TODO delete in parallel items, consider to add progress bar
// For empty folders double check if folders are really empty - this function probably should be run in thread
// and at the end should be send signal to main thread to update model
// TODO handle also situations where cannot delete file/folder
fn remove_selected_items(items: Vec<MainListModel>, active_tab: CurrentTab) {
    let path_idx = get_path_idx(active_tab);
    let name_idx = get_name_idx(active_tab);
    let items_to_remove = items
        .iter()
        .map(|item| {
            let path = item.val.iter().nth(path_idx).unwrap();
            let name = item.val.iter().nth(name_idx).unwrap();
            format!("{}{}{}", path, MAIN_SEPARATOR, name)
        })
        .collect::<Vec<_>>();

    // Iterate over empty folders and not delete them if they are not empty
    if active_tab == CurrentTab::EmptyFolders {
        items_to_remove.into_par_iter().for_each(|item| {
            remove_folder_if_contains_only_empty_folders(item);
        });
    } else {
        items_to_remove.into_par_iter().for_each(|item| {
            let _ = std::fs::remove_file(item);
        });
    }
}

fn deselect_all_items(items: &mut [MainListModel]) {
    for item in items {
        item.selected_row = false;
    }
}

fn filter_out_checked_items(items: &ModelRc<MainListModel>, have_header: bool) -> (Vec<MainListModel>, Vec<MainListModel>) {
    if cfg!(debug_assertions) {
        check_if_header_is_checked(items);
        check_if_header_is_selected_but_should_not_be(items, have_header);
    }

    let (entries_to_delete, mut entries_left): (Vec<_>, Vec<_>) = items.iter().partition(|item| item.checked);

    // When have header, we must also throw out orphaned items - this needs to be
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

    use crate::connect_delete::filter_out_checked_items;
    use crate::MainListModel;

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
    fn test_filter_out_checked_items_one_element_valid_normal() {
        let items = create_new_model(vec![(false, false, false, vec![])]);
        let (to_delete, left) = filter_out_checked_items(&items, false);
        assert!(to_delete.is_empty());
        assert_eq!(left.len(), items.iter().count());
    }

    #[test]
    fn test_filter_out_checked_items_one_element_valid_header() {
        let items = create_new_model(vec![(false, true, false, vec![])]);
        let (to_delete, left) = filter_out_checked_items(&items, true);
        assert!(to_delete.is_empty());
        assert!(left.is_empty());
    }

    #[test]
    #[should_panic]
    fn test_filter_out_checked_items_one_element_invalid_normal() {
        let items = create_new_model(vec![(false, true, false, vec![])]);
        filter_out_checked_items(&items, false);
    }
    #[test]
    #[should_panic]
    fn test_filter_out_checked_items_one_element_invalid_header() {
        let items = create_new_model(vec![(false, false, false, vec![])]);
        filter_out_checked_items(&items, true);
    }

    #[test]
    fn test_filter_out_checked_items_multiple_element_valid_normal() {
        let items = create_new_model(vec![
            (false, false, false, vec!["1"]),
            (false, false, false, vec!["2"]),
            (true, false, false, vec!["3"]),
            (true, false, false, vec!["4"]),
            (false, false, false, vec!["5"]),
        ]);
        let (to_delete, left) = filter_out_checked_items(&items, false);
        let to_delete_data = get_single_data_from_model(&to_delete);
        let left_data = get_single_data_from_model(&left);

        assert_eq!(to_delete_data, vec!["3", "4"]);
        assert_eq!(left_data, vec!["1", "2", "5"]);
    }

    #[test]
    fn test_filter_out_checked_items_multiple_element_valid_header() {
        let items = create_new_model(vec![
            (false, true, false, vec!["1"]),
            (false, false, false, vec!["2"]),
            (true, false, false, vec!["3"]),
            (false, true, false, vec!["4"]),
            (false, false, false, vec!["5"]),
            (false, true, false, vec!["6"]),
            (false, false, false, vec!["7"]),
            (false, false, false, vec!["8"]),
        ]);
        let (to_delete, left) = filter_out_checked_items(&items, true);
        let to_delete_data = get_single_data_from_model(&to_delete);
        let left_data = get_single_data_from_model(&left);

        assert_eq!(to_delete_data, vec!["3"]);
        assert_eq!(left_data, vec!["6", "7", "8"]);
    }

    #[test]
    fn test_filter_out_checked_items_multiple2_element_valid_header() {
        let items = create_new_model(vec![
            (false, true, false, vec!["1"]),
            (false, false, false, vec!["2"]),
            (true, false, false, vec!["3"]),
            (false, false, false, vec!["4"]),
            (false, false, false, vec!["5"]),
            (false, false, false, vec!["6"]),
            (false, true, false, vec!["7"]),
            (false, false, false, vec!["8"]),
        ]);
        let (to_delete, left) = filter_out_checked_items(&items, true);
        let to_delete_data = get_single_data_from_model(&to_delete);
        let left_data = get_single_data_from_model(&left);

        assert_eq!(to_delete_data, vec!["3"]);
        assert_eq!(left_data, vec!["1", "2", "4", "5", "6"]);
    }

    fn get_single_data_from_model(model: &[MainListModel]) -> Vec<String> {
        let mut d = model.iter().map(|item| item.val.iter().next().unwrap().to_string()).collect::<Vec<_>>();
        d.sort();
        d
    }

    fn create_new_model(items: Vec<(bool, bool, bool, Vec<&'static str>)>) -> ModelRc<MainListModel> {
        let model = VecModel::default();
        for item in items {
            let all_items: Vec<SharedString> = item.3.iter().map(|item| (*item).into()).collect::<Vec<_>>();
            let all_items = VecModel::from(all_items);
            model.push(MainListModel {
                checked: item.0,
                header_row: item.1,
                selected_row: item.2,
                val: ModelRc::new(all_items),
            });
        }
        ModelRc::new(model)
    }
}
