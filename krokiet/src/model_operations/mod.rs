pub mod model_processor;

#[allow(dead_code)]
use slint::{Model, ModelRc};

use crate::MainListModel;
use crate::simpler_model::SimplerMainListModel;

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
#[cfg(test)]
mod tests {
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
}
