use gtk4::prelude::*;

use crate::gui_structs::gui_data::GuiData;
use crate::gui_structs::gui_popovers_sort::GuiSortPopovers;
use crate::help_functions::PopoverTypes;
use crate::notebook_enums::{to_notebook_main_enum, NotebookMainEnum};
use crate::notebook_info::NOTEBOOKS_INFO;

pub fn connect_button_sort(gui_data: &GuiData) {
    let popovers_sort = gui_data.popovers_sort.clone();
    let notebook_main = gui_data.main_notebook.notebook_main.clone();
    let gc_buttons_sort = gui_data.bottom_buttons.gc_buttons_sort.clone();

    gc_buttons_sort.connect_pressed(move |_, _, _, _| {
        show_required_popovers(&popovers_sort, to_notebook_main_enum(notebook_main.current_page().expect("Current page not set")));
    });
}

fn show_required_popovers(popovers_sort: &GuiSortPopovers, current_mode: NotebookMainEnum) {
    let buttons_popover_sort_file_name = popovers_sort.buttons_popover_sort_file_name.clone();
    let buttons_popover_sort_size = popovers_sort.buttons_popover_sort_size.clone();
    let buttons_popover_sort_folder_name = popovers_sort.buttons_popover_sort_folder_name.clone();
    let buttons_popover_sort_full_name = popovers_sort.buttons_popover_sort_full_name.clone();
    let buttons_popover_sort_selection = popovers_sort.buttons_popover_sort_selection.clone();

    let arr = &NOTEBOOKS_INFO[current_mode as usize].available_modes;

    buttons_popover_sort_full_name.hide();

    if arr.contains(&PopoverTypes::All) {
        buttons_popover_sort_selection.show();
        buttons_popover_sort_file_name.show();
        buttons_popover_sort_folder_name.show();
        // buttons_popover_sort_full_name.show(); // TODO, this needs to be handled a little different
    } else {
        buttons_popover_sort_selection.hide();
        buttons_popover_sort_file_name.hide();
        buttons_popover_sort_folder_name.hide();
        // buttons_popover_sort_full_name.hide();
    }

    if arr.contains(&PopoverTypes::Size) {
        buttons_popover_sort_size.show();
    } else {
        buttons_popover_sort_size.hide();
    }
}
