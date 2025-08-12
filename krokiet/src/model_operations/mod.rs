pub mod model_processor;

use slint::ComponentHandle;
#[allow(dead_code)]
use slint::{Model, ModelRc};

use crate::connect_row_selection::checker::get_number_of_enabled_items;
use crate::simpler_model::SimplerMainListModel;
use crate::{GuiState, MainListModel, MainWindow};

pub type ProcessingResult = Vec<(usize, SimplerMainListModel, Option<Result<(), String>>)>;

impl MainListModel {
    #[allow(clippy::print_stdout)]
    #[allow(dead_code)]
    pub(crate) fn debug_print(&self) {
        let val_int: Vec<i32> = self.val_int.iter().collect();
        let val_str: Vec<String> = self.val_str.iter().map(|e| e.to_string()).collect();
        println!(
            "MainListModel: checked: {}, filled_header_row: {}, header_row: {}, selected_row: {}, val_int: {:?}, val_str: {:?}",
            self.checked, self.filled_header_row, self.header_row, self.selected_row, val_int, val_str
        );
    }
}

pub trait DebugPrintModelRc {
    #[allow(dead_code)]
    fn debug_print_model_rc(&self);
}
impl DebugPrintModelRc for ModelRc<MainListModel> {
    #[allow(clippy::print_stdout)]
    fn debug_print_model_rc(&self) {
        println!("=====================START DEBUG PRINT RC MODELS=====================");
        println!("Model with {} items", self.iter().count());
        for item in self.iter() {
            item.debug_print();
        }
        println!("=====================END DEBUG PRINT RC MODELS=====================");
    }
}

// TODO - tests
// Removes orphan items in groups
pub(crate) fn remove_single_items_in_groups(mut items: Vec<MainListModel>, have_header: bool) -> Vec<MainListModel> {
    // When have header, we must also throw out orphaned items
    if have_header && !items.is_empty() {
        // First row must be header
        // If assert fails, that means, that we checked that for mode that not have headers
        // or that we somehow removed header row, which cannot happen without serious bug
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

pub struct CheckedItemsInfo {
    pub checked_items_number: u64,
    pub groups_with_checked_items: Option<CheckedGroupItemsInfo>,
}
pub struct CheckedGroupItemsInfo {
    pub groups_with_checked_items: u64,
    pub number_of_groups_with_all_items_checked: u64,
}

// TODO - this will be broken for models with reference folders
fn get_checked_group_info_from_model(model: &ModelRc<MainListModel>) -> CheckedItemsInfo {
    if model.iter().next().is_none() {
        // Here I could panic, but i think that it is still possbile to go here, without doing anything wrong
        return CheckedItemsInfo {
            checked_items_number: 0,
            groups_with_checked_items: None,
        };
    }

    let mut checked_items_number = 0;
    let mut groups_with_checked_items = 0;
    let mut number_of_groups_with_all_items_checked = 0;

    let mut current_group_all_checked = true;
    let mut group_with_selected_item = false;

    // TODO Maybe a little useless, check if really needed
    let model_collected = model.iter().collect::<Vec<_>>();
    assert!(model_collected[0].header_row);
    assert!(!model_collected.last().expect("Is not empty").header_row);

    for item in model_collected.iter().skip(1) {
        if item.header_row {
            if current_group_all_checked {
                number_of_groups_with_all_items_checked += 1;
            }
            if group_with_selected_item {
                groups_with_checked_items += 1;
            }
            current_group_all_checked = true;
            group_with_selected_item = false;
        } else {
            if item.checked {
                checked_items_number += 1;
                group_with_selected_item = true;
            } else {
                current_group_all_checked = false;
            }
        }
    }
    if model_collected.len() > 1 {
        if current_group_all_checked {
            number_of_groups_with_all_items_checked += 1;
        }
        if group_with_selected_item {
            groups_with_checked_items += 1;
        }
    }

    CheckedItemsInfo {
        checked_items_number,
        groups_with_checked_items: Some(CheckedGroupItemsInfo {
            groups_with_checked_items,
            number_of_groups_with_all_items_checked,
        }),
    }
}

pub(crate) fn get_checked_info_from_app(app: &MainWindow) -> CheckedItemsInfo {
    let active_tab = app.global::<GuiState>().get_active_tab();
    let model = active_tab.get_tool_model(app);
    if active_tab.get_is_header_mode() {
        get_checked_group_info_from_model(&model)
    } else {
        let checked_items_number = get_number_of_enabled_items(app, active_tab);
        // Alternatively, this can be manually calculated here
        // let checked_items_number = model.iter().filter(|item| item.checked).count();
        CheckedItemsInfo {
            checked_items_number,
            groups_with_checked_items: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use slint::VecModel;

    use super::*;
    use crate::test_common::get_model_vec;

    #[test]
    fn remove_single_items_elements() {
        let mut items = get_model_vec(3);
        items[0].header_row = true;
        items[1].header_row = true;
        let result = remove_single_items_in_groups(items, true);
        assert!(result.is_empty());

        let mut items = get_model_vec(3);
        items[0].header_row = true;
        items[2].header_row = true;
        let result = remove_single_items_in_groups(items, true);
        assert!(result.is_empty());

        let mut items = get_model_vec(3);
        items[0].header_row = true;
        let result = remove_single_items_in_groups(items, true);
        assert_eq!(result.len(), 3);

        let mut items = get_model_vec(3);
        items[0].header_row = true;
        items[0].filled_header_row = true;
        items[2].header_row = true;
        items[2].filled_header_row = true;
        let result = remove_single_items_in_groups(items, true);
        assert_eq!(result.len(), 2);
        assert!(result[0].header_row);
        assert!(!result[1].header_row);

        let mut items = get_model_vec(2);
        items[0].header_row = true;
        let result = remove_single_items_in_groups(items, true);
        assert_eq!(result.len(), 0);

        let mut items = get_model_vec(10);
        items[0].header_row = true;
        items[9].header_row = true;
        let result = remove_single_items_in_groups(items, true);
        assert_eq!(result.len(), 9);

        let mut items = get_model_vec(2);
        items[0].header_row = true;
        items[0].filled_header_row = true;
        items[1].header_row = false;
        let result = remove_single_items_in_groups(items, true);
        assert_eq!(result.len(), 2);

        let mut items = get_model_vec(2);
        items[0].header_row = true;
        items[0].filled_header_row = true;
        items[1].header_row = true;
        let result = remove_single_items_in_groups(items, true);
        assert!(result.is_empty());

        let mut items = get_model_vec(1);
        items[0].header_row = true;
        items[0].filled_header_row = true;
        let result = remove_single_items_in_groups(items, true);
        assert!(result.is_empty());

        let items = Vec::new();
        let result = remove_single_items_in_groups(items, true);
        assert!(result.is_empty());

        let mut items = get_model_vec(4);
        items[0].header_row = true;
        items[0].filled_header_row = true;
        let result = remove_single_items_in_groups(items.clone(), true);
        assert_eq!(result.len(), 4);
    }

    #[test]
    #[should_panic]
    fn panics_when_first_row_is_not_header_but_have_header() {
        let mut items = get_model_vec(2);
        items[0].header_row = false;
        remove_single_items_in_groups(items, true);
    }

    #[test]
    #[should_panic]
    fn panics_when_first_row_is_checked_but_have_header() {
        let mut items = get_model_vec(2);
        items[0].header_row = true;
        items[0].checked = true;
        remove_single_items_in_groups(items, true);
    }

    #[test]
    #[should_panic]
    fn panics_when_first_row_is_selected_but_have_header() {
        let mut items = get_model_vec(2);
        items[0].header_row = true;
        items[0].selected_row = true;
        remove_single_items_in_groups(items, true);
    }

    #[test]
    fn check_checked_function() {
        let mut items = get_model_vec(4);
        items[0].header_row = true;
        items[1].checked = true;
        items[2].checked = true;
        items[3].checked = true;

        let model = ModelRc::new(VecModel::from(items));
        let result = get_checked_group_info_from_model(&model);
        let groups_info = result.groups_with_checked_items.unwrap();
        assert_eq!(result.checked_items_number, 3);
        assert_eq!(groups_info.groups_with_checked_items, 1);
        assert_eq!(groups_info.number_of_groups_with_all_items_checked, 1);

        let mut items = get_model_vec(8);
        items[0].header_row = true;
        items[1].checked = true;
        items[2].checked = true;
        items[3].checked = false;
        items[4].header_row = true;
        items[5].checked = true;
        items[6].header_row = true;
        items[7].checked = false;

        let model = ModelRc::new(VecModel::from(items));
        let result = get_checked_group_info_from_model(&model);
        let groups_info = result.groups_with_checked_items.unwrap();
        assert_eq!(result.checked_items_number, 3);
        assert_eq!(groups_info.groups_with_checked_items, 2);
        assert_eq!(groups_info.number_of_groups_with_all_items_checked, 1);
    }
}
