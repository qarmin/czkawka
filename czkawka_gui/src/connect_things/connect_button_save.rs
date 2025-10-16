use std::cell::RefCell;
use std::collections::HashMap;
use std::env;
use std::rc::Rc;

use czkawka_core::common::traits::PrintResults;
use gtk4::prelude::*;
use gtk4::{Button, Entry};

use crate::flg;
use crate::gui_structs::gui_data::GuiData;
use crate::help_functions::BottomButtonsEnum;
use crate::notebook_enums::{NotebookMainEnum, to_notebook_main_enum};

pub(crate) fn connect_button_save(gui_data: &GuiData) {
    let buttons_save = gui_data.bottom_buttons.buttons_save.clone();
    let buttons_save_clone = gui_data.bottom_buttons.buttons_save.clone();
    let shared_buttons = gui_data.shared_buttons.clone();
    let entry_info = gui_data.entry_info.clone();
    let common_tree_views = gui_data.main_notebook.common_tree_views.clone();

    buttons_save.connect_clicked(move |_| {
        let current_path = match env::current_dir() {
            Ok(t) => t.to_string_lossy().to_string(),
            Err(_) => "__unknown__".to_string(),
        };

        let subview = common_tree_views.get_current_subview();

        if let Err(e) = subview.shared_model_enum.save_all_in_one(&current_path) {
            entry_info.set_text(&format!("Failed to save results to folder {current_path}, reason {e}"));
            return;
        }

        post_save_things(
            subview.enum_value,
            &shared_buttons,
            &entry_info,
            &buttons_save_clone,
            current_path,
        );
    });
}

fn post_save_things(
    type_of_tab: NotebookMainEnum,
    shared_buttons: &Rc<RefCell<HashMap<NotebookMainEnum, HashMap<BottomButtonsEnum, bool>>>>,
    entry_info: &Entry,
    buttons_save: &Button,
    current_path: String,
) {
    entry_info.set_text(&flg!("save_results_to_file", name = current_path));
    // Set state
    {
        buttons_save.hide();
        *shared_buttons
            .borrow_mut()
            .get_mut(&type_of_tab)
            .expect("Failed to get current tab")
            .get_mut(&BottomButtonsEnum::Save)
            .expect("Failed to get save button") = false;
    }
}
