extern crate gtk;
use crate::gui_data::GuiData;
use gtk::prelude::*;

pub fn connect_button_select(gui_data: &GuiData) {
    let notebook_main_children_names = gui_data.notebook_main_children_names.clone();
    let notebook_main = gui_data.notebook_main.clone();
    let buttons_select_clone = gui_data.buttons_select.clone();
    let popover_select_duplicate = gui_data.popover_select_duplicate.clone();
    let popover_select_simple_list = gui_data.popover_select_simple_list.clone();
    let popover_select_very_simple_list = gui_data.popover_select_very_simple_list.clone();
    let buttons_select = gui_data.buttons_select.clone();
    buttons_select_clone.connect_clicked(move |_| match notebook_main_children_names.get(notebook_main.get_current_page().unwrap() as usize).unwrap().as_str() {
        "notebook_main_duplicate_finder_label" | "notebook_main_same_music_finder" | "notebook_main_similar_images_finder_label" => {
            popover_select_duplicate.set_relative_to(Some(&buttons_select));
            popover_select_duplicate.popup();
        }
        "scrolled_window_main_empty_folder_finder" | "scrolled_window_main_empty_files_finder" | "scrolled_window_main_temporary_files_finder" | "notebook_big_main_file_finder" | "notebook_main_zeroed_files_finder" => {
            popover_select_simple_list.set_relative_to(Some(&buttons_select));
            popover_select_simple_list.popup();
        }
        "scrolled_window_invalid_symlinks" => {
            popover_select_very_simple_list.set_relative_to(Some(&buttons_select));
            popover_select_very_simple_list.popup();
        }
        e => panic!("Not existent {}", e),
    });
}
