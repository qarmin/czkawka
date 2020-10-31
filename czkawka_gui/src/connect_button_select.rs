extern crate gtk;
use crate::gui_data::GuiData;
use gtk::prelude::*;

pub fn connect_button_select(gui_data: &GuiData) {
    let notebook_main_children_names = gui_data.notebook_main_children_names.clone();
    let notebook_main = gui_data.notebook_main.clone();
    let buttons_select_clone = gui_data.buttons_select.clone();
    let popover_select = gui_data.popover_select.clone();
    let buttons_select = gui_data.buttons_select.clone();
    buttons_select_clone.connect_clicked(move |_| match notebook_main_children_names.get(notebook_main.get_current_page().unwrap() as usize).unwrap().as_str() {
        "notebook_main_duplicate_finder_label" => {
            // Only popup popup
            popover_select.set_relative_to(Some(&buttons_select));
            popover_select.popup();
        }
        e => panic!("Not existent {}", e),
    });
}
