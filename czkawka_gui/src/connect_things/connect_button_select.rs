use gtk4::prelude::*;

use crate::gui_structs::gui_data::GuiData;
use crate::gui_structs::gui_popovers_select::GuiSelectPopovers;
use crate::help_functions::PopoverTypes;
use crate::notebook_enums::*;
use crate::notebook_info::NOTEBOOKS_INFO;

pub fn connect_button_select(gui_data: &GuiData) {
    let popovers_select = gui_data.popovers_select.clone();
    let notebook_main = gui_data.main_notebook.notebook_main.clone();
    let gc_buttons_select = gui_data.bottom_buttons.gc_buttons_select.clone();

    gc_buttons_select.connect_pressed(move |_, _, _, _| {
        show_required_popovers(&popovers_select, to_notebook_main_enum(notebook_main.current_page().expect("Current page not set")));
    });
}

fn show_required_popovers(popovers_select: &GuiSelectPopovers, current_mode: NotebookMainEnum) {
    let buttons_popover_select_all = popovers_select.buttons_popover_select_all.clone();
    let buttons_popover_unselect_all = popovers_select.buttons_popover_unselect_all.clone();
    let buttons_popover_reverse = popovers_select.buttons_popover_reverse.clone();
    let buttons_popover_select_all_except_oldest = popovers_select.buttons_popover_select_all_except_oldest.clone();
    let buttons_popover_select_all_except_newest = popovers_select.buttons_popover_select_all_except_newest.clone();
    let buttons_popover_select_one_oldest = popovers_select.buttons_popover_select_one_oldest.clone();
    let buttons_popover_select_one_newest = popovers_select.buttons_popover_select_one_newest.clone();
    let buttons_popover_select_custom = popovers_select.buttons_popover_select_custom.clone();
    let buttons_popover_unselect_custom = popovers_select.buttons_popover_unselect_custom.clone();
    let buttons_popover_select_all_images_except_biggest = popovers_select.buttons_popover_select_all_images_except_biggest.clone();
    let buttons_popover_select_all_images_except_smallest = popovers_select.buttons_popover_select_all_images_except_smallest.clone();

    let separator_select_custom = popovers_select.separator_select_custom.clone();
    let separator_select_date = popovers_select.separator_select_date.clone();
    let separator_select_image_size = popovers_select.separator_select_image_size.clone();
    let separator_select_reverse = popovers_select.separator_select_reverse.clone();

    let arr = &NOTEBOOKS_INFO[current_mode as usize].available_modes;

    if arr.contains(&PopoverTypes::All) {
        buttons_popover_select_all.show();
        buttons_popover_unselect_all.show();
    } else {
        buttons_popover_select_all.hide();
        buttons_popover_unselect_all.hide();
    }

    if arr.contains(&PopoverTypes::Size) {
        buttons_popover_select_all_images_except_biggest.show();
        buttons_popover_select_all_images_except_smallest.show();
        separator_select_image_size.show();
    } else {
        buttons_popover_select_all_images_except_biggest.hide();
        buttons_popover_select_all_images_except_smallest.hide();
        separator_select_image_size.hide();
    }

    if arr.contains(&PopoverTypes::Reverse) {
        buttons_popover_reverse.show();
        separator_select_reverse.show();
    } else {
        buttons_popover_reverse.hide();
        separator_select_reverse.hide();
    }

    if arr.contains(&PopoverTypes::Custom) {
        buttons_popover_select_custom.show();
        buttons_popover_unselect_custom.show();
        separator_select_custom.show();
    } else {
        buttons_popover_select_custom.hide();
        buttons_popover_unselect_custom.hide();
        separator_select_custom.hide();
    }

    if arr.contains(&PopoverTypes::Date) {
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
