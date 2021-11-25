use crate::gui_data::GuiData;
use crate::gui_popovers::GuiPopovers;
use crate::help_functions::PopoverTypes;
use crate::notebook_enums::*;
use gtk::prelude::*;
use std::collections::HashMap;

pub fn connect_button_select(gui_data: &GuiData) {
    let mut hashmap: HashMap<NotebookMainEnum, Vec<PopoverTypes>> = Default::default();
    {
        // Remember to update connect_popovers file, because this data are connected to each others
        hashmap.insert(NotebookMainEnum::SimilarImages, vec![PopoverTypes::All, PopoverTypes::ImageSize, PopoverTypes::Reverse, PopoverTypes::Custom, PopoverTypes::Date]);
        hashmap.insert(NotebookMainEnum::SimilarVideos, vec![PopoverTypes::All, PopoverTypes::Reverse, PopoverTypes::Custom, PopoverTypes::Date]);
        hashmap.insert(NotebookMainEnum::Duplicate, vec![PopoverTypes::All, PopoverTypes::Reverse, PopoverTypes::Custom, PopoverTypes::Date]);
        hashmap.insert(NotebookMainEnum::SameMusic, vec![PopoverTypes::All, PopoverTypes::Reverse, PopoverTypes::Custom, PopoverTypes::Date]);

        hashmap.insert(NotebookMainEnum::EmptyFiles, vec![PopoverTypes::All, PopoverTypes::Reverse, PopoverTypes::Custom]);
        hashmap.insert(NotebookMainEnum::EmptyDirectories, vec![PopoverTypes::All, PopoverTypes::Reverse, PopoverTypes::Custom]);
        hashmap.insert(NotebookMainEnum::BigFiles, vec![PopoverTypes::All, PopoverTypes::Reverse, PopoverTypes::Custom]);
        hashmap.insert(NotebookMainEnum::Symlinks, vec![PopoverTypes::All, PopoverTypes::Reverse, PopoverTypes::Custom]);
        hashmap.insert(NotebookMainEnum::Temporary, vec![PopoverTypes::All, PopoverTypes::Reverse, PopoverTypes::Custom]);
        hashmap.insert(NotebookMainEnum::BrokenFiles, vec![PopoverTypes::All, PopoverTypes::Reverse, PopoverTypes::Custom]);
    }
    assert_eq!(hashmap.len(), NUMBER_OF_NOTEBOOK_MAIN_TABS);

    let popovers = gui_data.popovers.clone();
    let notebook_main = gui_data.main_notebook.notebook_main.clone();
    let buttons_select_clone = gui_data.bottom_buttons.buttons_select.clone();
    let popover_select = gui_data.popovers.popover_select.clone();
    let buttons_select = gui_data.bottom_buttons.buttons_select.clone();

    buttons_select_clone.connect_clicked(move |_| {
        show_required_popovers(&popovers, &to_notebook_main_enum(notebook_main.current_page().unwrap()), &hashmap);
        popover_select.set_relative_to(Some(&buttons_select));
        popover_select.popup();
    });
}

fn show_required_popovers(popovers: &GuiPopovers, current_mode: &NotebookMainEnum, hashmap: &HashMap<NotebookMainEnum, Vec<PopoverTypes>>) {
    let buttons_popover_select_all = popovers.buttons_popover_select_all.clone();
    let buttons_popover_unselect_all = popovers.buttons_popover_unselect_all.clone();
    let buttons_popover_reverse = popovers.buttons_popover_reverse.clone();
    let buttons_popover_select_all_except_oldest = popovers.buttons_popover_select_all_except_oldest.clone();
    let buttons_popover_select_all_except_newest = popovers.buttons_popover_select_all_except_newest.clone();
    let buttons_popover_select_one_oldest = popovers.buttons_popover_select_one_oldest.clone();
    let buttons_popover_select_one_newest = popovers.buttons_popover_select_one_newest.clone();
    let buttons_popover_select_custom = popovers.buttons_popover_select_custom.clone();
    let buttons_popover_unselect_custom = popovers.buttons_popover_unselect_custom.clone();
    let buttons_popover_select_all_images_except_biggest = popovers.buttons_popover_select_all_images_except_biggest.clone();
    let buttons_popover_select_all_images_except_smallest = popovers.buttons_popover_select_all_images_except_smallest.clone();

    let separator_select_custom = popovers.separator_select_custom.clone();
    let separator_select_date = popovers.separator_select_date.clone();
    let separator_select_image_size = popovers.separator_select_image_size.clone();
    let separator_select_reverse = popovers.separator_select_reverse.clone();

    let vec = hashmap.get(current_mode).unwrap();

    if vec.contains(&PopoverTypes::All) {
        buttons_popover_select_all.show();
        buttons_popover_unselect_all.show();
    } else {
        buttons_popover_select_all.hide();
        buttons_popover_unselect_all.hide();
    }

    if vec.contains(&PopoverTypes::ImageSize) {
        buttons_popover_select_all_images_except_biggest.show();
        buttons_popover_select_all_images_except_smallest.show();
        separator_select_image_size.show();
    } else {
        buttons_popover_select_all_images_except_biggest.hide();
        buttons_popover_select_all_images_except_smallest.hide();
        separator_select_image_size.hide();
    }

    if vec.contains(&PopoverTypes::Reverse) {
        buttons_popover_reverse.show();
        separator_select_reverse.show();
    } else {
        buttons_popover_reverse.hide();
        separator_select_reverse.hide();
    }

    if vec.contains(&PopoverTypes::Custom) {
        buttons_popover_select_custom.show();
        buttons_popover_unselect_custom.show();
        separator_select_custom.show();
    } else {
        buttons_popover_select_custom.hide();
        buttons_popover_unselect_custom.hide();
        separator_select_custom.hide();
    }

    if vec.contains(&PopoverTypes::Date) {
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
