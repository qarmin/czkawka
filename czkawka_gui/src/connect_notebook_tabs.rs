extern crate gtk;
use crate::gui_data::GuiData;
use crate::help_functions::*;
use gtk::prelude::*;
use std::collections::HashMap;

pub fn connect_notebook_tabs(gui_data: &GuiData) {
    let shared_buttons = gui_data.shared_buttons.clone();
    let buttons_array = gui_data.buttons_array.clone();
    let notebook_main_children_names = gui_data.notebook_main_children_names.clone();
    let notebook_main_clone = gui_data.notebook_main.clone();
    let buttons_names = gui_data.buttons_names.clone();
    let upper_notebooks_labels = gui_data.upper_notebooks_labels.clone();
    let shared_upper_notebooks = gui_data.shared_upper_notebooks.clone();
    let notebook_upper = gui_data.notebook_upper.clone();
    let notebook_upper_children_names = gui_data.notebook_upper_children_names.clone();

    notebook_main_clone.connect_switch_page(move |_, _, number| {
        let page: &str;
        match notebook_main_children_names.get(number as usize).unwrap().as_str() {
            "notebook_main_duplicate_finder_label" => {
                page = "duplicate";
            }
            "scrolled_window_main_empty_folder_finder" => {
                page = "empty_folder";
            }
            "scrolled_window_main_empty_files_finder" => page = "empty_file",
            "scrolled_window_main_temporary_files_finder" => page = "temporary_file",
            "notebook_big_main_file_finder" => page = "big_file",
            "notebook_main_similar_images_finder_label" => page = "similar_images",
            "notebook_main_zeroed_files_finder" => page = "zeroed_files",
            e => {
                panic!("Not existent page {}", e);
            }
        };
        // Buttons
        set_buttons(&mut *shared_buttons.borrow_mut().get_mut(page).unwrap(), &buttons_array, &buttons_names);
        // Upper notebook
        {
            //let upper_notebooks_labels = [/*"general",*/"included_directories","excluded_directories","excluded_items","allowed_extensions"];
            let mut hashmap: HashMap<&str, &str> = Default::default();
            //hashmap.insert("notebook_upper_general","general");
            hashmap.insert("notebook_upper_included_directories", "included_directories");
            hashmap.insert("notebook_upper_excluded_directories", "excluded_directories");
            hashmap.insert("notebook_upper_excluded_items", "excluded_items");
            hashmap.insert("notebook_upper_allowed_extensions", "allowed_extensions");

            for tab in &notebook_upper_children_names {
                let name = hashmap.get(tab.as_str()).unwrap().to_string();
                let index = upper_notebooks_labels.iter().position(|x| *x == name).unwrap();
                if *shared_upper_notebooks.borrow_mut().get_mut(page).unwrap().get_mut(&name).unwrap() {
                    notebook_upper.get_children().get(index).unwrap().show();
                } else {
                    notebook_upper.get_children().get(index).unwrap().hide();
                }
            }
        }
    });
}
