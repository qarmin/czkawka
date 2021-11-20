use crate::gui_data::GuiData;
use crate::notebook_enums::*;
use gtk::prelude::*;
use std::collections::HashMap;
//  TODO Replace `all`, `image_size` etc. with this
// pub enum PopoverType {
//     All,
//     ImageSize,
//     Reverse,
//     Custom,
//     Date,
// }

pub fn connect_button_select(gui_data: &GuiData) {
    // let mode = ["all", "image_size", "reverse", "custom", "date"];
    let mut hashmap: HashMap<NotebookMainEnum, Vec<&str>> = Default::default();
    {
        // Remember to update connect_popovers file, because this data are connected to each others
        hashmap.insert(NotebookMainEnum::SimilarImages, vec!["all", "image_size", "reverse", "custom", "date"]);
        hashmap.insert(NotebookMainEnum::Duplicate, vec!["all", "reverse", "custom", "date"]);
        hashmap.insert(NotebookMainEnum::SameMusic, vec!["all", "reverse", "custom", "date"]);

        hashmap.insert(NotebookMainEnum::EmptyFiles, vec!["all", "reverse", "custom"]);
        hashmap.insert(NotebookMainEnum::EmptyDirectories, vec!["all", "reverse", "custom"]);
        hashmap.insert(NotebookMainEnum::BigFiles, vec!["all", "reverse", "custom"]);
        hashmap.insert(NotebookMainEnum::Symlinks, vec!["all", "reverse", "custom"]);
        hashmap.insert(NotebookMainEnum::Zeroed, vec!["all", "reverse", "custom"]);
        hashmap.insert(NotebookMainEnum::Temporary, vec!["all", "reverse", "custom"]);
        hashmap.insert(NotebookMainEnum::BrokenFiles, vec!["all", "reverse", "custom"]);
    }

    let gui_data = gui_data.clone();
    let notebook_main = gui_data.main_notebook.notebook_main.clone();
    let buttons_select_clone = gui_data.bottom_buttons.buttons_select.clone();
    let popover_select = gui_data.popovers.popover_select.clone();
    let buttons_select = gui_data.bottom_buttons.buttons_select.clone();

    buttons_select_clone.connect_clicked(move |_| {
        show_required_popovers(&gui_data, &to_notebook_main_enum(notebook_main.current_page().unwrap()), &hashmap);
        popover_select.set_relative_to(Some(&buttons_select));
        popover_select.popup();
    });
}

fn show_required_popovers(gui_data: &GuiData, current_mode: &NotebookMainEnum, hashmap: &HashMap<NotebookMainEnum, Vec<&str>>) {
    let buttons_popover_select_all = gui_data.popovers.buttons_popover_select_all.clone();
    let buttons_popover_unselect_all = gui_data.popovers.buttons_popover_unselect_all.clone();
    let buttons_popover_reverse = gui_data.popovers.buttons_popover_reverse.clone();
    let buttons_popover_select_all_except_oldest = gui_data.popovers.buttons_popover_select_all_except_oldest.clone();
    let buttons_popover_select_all_except_newest = gui_data.popovers.buttons_popover_select_all_except_newest.clone();
    let buttons_popover_select_one_oldest = gui_data.popovers.buttons_popover_select_one_oldest.clone();
    let buttons_popover_select_one_newest = gui_data.popovers.buttons_popover_select_one_newest.clone();
    let buttons_popover_select_custom = gui_data.popovers.buttons_popover_select_custom.clone();
    let buttons_popover_unselect_custom = gui_data.popovers.buttons_popover_unselect_custom.clone();
    let buttons_popover_select_all_images_except_biggest = gui_data.popovers.buttons_popover_select_all_images_except_biggest.clone();
    let buttons_popover_select_all_images_except_smallest = gui_data.popovers.buttons_popover_select_all_images_except_smallest.clone();

    let separator_select_custom = gui_data.popovers.separator_select_custom.clone();
    let separator_select_date = gui_data.popovers.separator_select_date.clone();
    let separator_select_image_size = gui_data.popovers.separator_select_image_size.clone();
    let separator_select_reverse = gui_data.popovers.separator_select_reverse.clone();

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
