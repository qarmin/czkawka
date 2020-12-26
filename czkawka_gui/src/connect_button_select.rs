extern crate gtk;
use crate::gui_data::GuiData;
use gtk::prelude::*;
use std::collections::HashMap;

pub fn connect_button_select(gui_data: &GuiData) {
    // let mode = ["all", "image_size", "reverse", "custom", "date"];
    let mut hashmap: HashMap<&str, Vec<&str>> = Default::default();
    {
        // Remember to update connect_popovers file, because this data are connected to each others
        hashmap.insert("images", vec!["all", "image_size", "reverse", "custom", "date"]);
        hashmap.insert("duplicate", vec!["all", "reverse", "custom", "date"]);
        hashmap.insert("music", vec!["all", "reverse", "custom", "date"]);

        hashmap.insert("empty_files", vec!["all", "reverse", "custom"]);
        hashmap.insert("empty_folders", vec!["all", "reverse", "custom"]);
        hashmap.insert("big", vec!["all", "reverse", "custom"]);
        hashmap.insert("symlinks", vec!["all", "reverse", "custom"]);
        hashmap.insert("zeroed", vec!["all", "reverse", "custom"]);
        hashmap.insert("temporary", vec!["all", "reverse", "custom"]);
    }

    let gui_data = gui_data.clone();
    let notebook_main_children_names = gui_data.notebook_main_children_names.clone();
    let notebook_main = gui_data.notebook_main.clone();
    let buttons_select_clone = gui_data.buttons_select.clone();
    let popover_select = gui_data.popover_select.clone();
    let buttons_select = gui_data.buttons_select.clone();

    buttons_select_clone.connect_clicked(move |_| {
        let current_mode;

        match notebook_main_children_names.get(notebook_main.get_current_page().unwrap() as usize).unwrap().as_str() {
            "notebook_main_duplicate_finder_label" => {
                current_mode = "duplicate";
            }
            "notebook_main_same_music_finder" => {
                current_mode = "music";
            }
            "notebook_main_similar_images_finder_label" => {
                current_mode = "images";
            }
            "scrolled_window_main_empty_folder_finder" => {
                current_mode = "empty_folders";
            }
            "scrolled_window_main_empty_files_finder" => {
                current_mode = "empty_files";
            }
            "scrolled_window_main_temporary_files_finder" => {
                current_mode = "temporary";
            }
            "notebook_big_main_file_finder" => {
                current_mode = "big";
            }
            "notebook_main_zeroed_files_finder" => {
                current_mode = "zeroed";
            }
            "scrolled_window_invalid_symlinks" => {
                current_mode = "symlinks";
            }
            e => panic!("Not existent {}", e),
        }
        show_required_popovers(&gui_data, current_mode, &hashmap);
        popover_select.set_relative_to(Some(&buttons_select));
        popover_select.popup();
    });
}

fn show_required_popovers(gui_data: &GuiData, current_mode: &str, hashmap: &HashMap<&str, Vec<&str>>) {
    let buttons_popover_select_all = gui_data.buttons_popover_select_all.clone();
    let buttons_popover_unselect_all = gui_data.buttons_popover_unselect_all.clone();
    let buttons_popover_reverse = gui_data.buttons_popover_reverse.clone();
    let buttons_popover_select_all_except_oldest = gui_data.buttons_popover_select_all_except_oldest.clone();
    let buttons_popover_select_all_except_newest = gui_data.buttons_popover_select_all_except_newest.clone();
    let buttons_popover_select_one_oldest = gui_data.buttons_popover_select_one_oldest.clone();
    let buttons_popover_select_one_newest = gui_data.buttons_popover_select_one_newest.clone();
    let buttons_popover_select_custom = gui_data.buttons_popover_select_custom.clone();
    let buttons_popover_unselect_custom = gui_data.buttons_popover_unselect_custom.clone();
    let buttons_popover_select_all_images_except_biggest = gui_data.buttons_popover_select_all_images_except_biggest.clone();
    let buttons_popover_select_all_images_except_smallest = gui_data.buttons_popover_select_all_images_except_smallest.clone();

    let separator_select_custom = gui_data.separator_select_custom.clone();
    let separator_select_date = gui_data.separator_select_date.clone();
    let separator_select_image_size = gui_data.separator_select_image_size.clone();
    let separator_select_reverse = gui_data.separator_select_reverse.clone();

    let vec = hashmap.get(current_mode).unwrap();

    if vec.contains(&"all") {
        buttons_popover_select_all.show();
        buttons_popover_unselect_all.show();
    } else {
        buttons_popover_select_all.hide();
        buttons_popover_unselect_all.hide();
    }

    if vec.contains(&"image_size") {
        buttons_popover_select_all_images_except_biggest.show();
        buttons_popover_select_all_images_except_smallest.show();
        separator_select_image_size.show();
    } else {
        buttons_popover_select_all_images_except_biggest.hide();
        buttons_popover_select_all_images_except_smallest.hide();
        separator_select_image_size.hide();
    }

    if vec.contains(&"reverse") {
        buttons_popover_reverse.show();
        separator_select_reverse.show();
    } else {
        buttons_popover_reverse.hide();
        separator_select_reverse.hide();
    }

    if vec.contains(&"custom") {
        buttons_popover_select_custom.show();
        buttons_popover_unselect_custom.show();
        separator_select_custom.show();
    } else {
        buttons_popover_select_custom.hide();
        buttons_popover_unselect_custom.hide();
        separator_select_custom.hide();
    }

    if vec.contains(&"date") {
        buttons_popover_select_all_except_oldest.show();
        buttons_popover_select_all_except_newest.show();
        buttons_popover_select_one_oldest.show();
        buttons_popover_select_one_newest.show();
        separator_select_date.show();
    } else {
        buttons_popover_select_all_except_oldest.hide();
        buttons_popover_select_all_except_newest.hide();
        buttons_popover_select_one_oldest.hide();
        buttons_popover_select_one_newest.hide();
        separator_select_date.hide();
    }
}
