use gtk4::prelude::*;

use crate::gui_structs::common_tree_view::SubView;
use crate::gui_structs::gui_data::GuiData;
use crate::gui_structs::gui_popovers_select::GuiSelectPopovers;
use crate::helpers::enums::PopoverTypes;

pub(crate) fn connect_button_select(gui_data: &GuiData) {
    let popovers_select = gui_data.popovers_select.clone();
    let gc_buttons_select = gui_data.bottom_buttons.gc_buttons_select.clone();
    let common_tree_views = gui_data.main_notebook.common_tree_views.clone();

    gc_buttons_select.connect_pressed(move |_, _, _, _| {
        show_required_popovers(&popovers_select, common_tree_views.get_current_subview());
    });
}

fn show_required_popovers(popovers_select: &GuiSelectPopovers, sv: &SubView) {
    let buttons_popover_select_all = popovers_select.buttons_popover_select_all.clone();
    let buttons_popover_unselect_all = popovers_select.buttons_popover_unselect_all.clone();
    let buttons_popover_reverse = popovers_select.buttons_popover_reverse.clone();
    let buttons_popover_select_all_except_shortest_path = popovers_select.buttons_popover_select_all_except_shortest_path.clone();
    let buttons_popover_select_all_except_longest_path = popovers_select.buttons_popover_select_all_except_longest_path.clone();
    let buttons_popover_select_all_except_oldest = popovers_select.buttons_popover_select_all_except_oldest.clone();
    let buttons_popover_select_all_except_newest = popovers_select.buttons_popover_select_all_except_newest.clone();
    let buttons_popover_select_one_oldest = popovers_select.buttons_popover_select_one_oldest.clone();
    let buttons_popover_select_one_newest = popovers_select.buttons_popover_select_one_newest.clone();
    let buttons_popover_select_custom = popovers_select.buttons_popover_select_custom.clone();
    let buttons_popover_unselect_custom = popovers_select.buttons_popover_unselect_custom.clone();
    let buttons_popover_select_all_images_except_biggest = popovers_select.buttons_popover_select_all_images_except_biggest.clone();
    let buttons_popover_select_all_images_except_smallest = popovers_select.buttons_popover_select_all_images_except_smallest.clone();

    let separator_select_shortest_path = popovers_select.separator_select_shortest_path.clone();
    let separator_select_custom = popovers_select.separator_select_custom.clone();
    let separator_select_date = popovers_select.separator_select_date.clone();
    let separator_select_image_size = popovers_select.separator_select_image_size.clone();
    let separator_select_reverse = popovers_select.separator_select_reverse.clone();

    let arr = sv.nb_object.available_modes;

    if arr.contains(&PopoverTypes::All) {
        buttons_popover_select_all.set_visible(true);
        buttons_popover_unselect_all.set_visible(true);
    } else {
        buttons_popover_select_all.set_visible(false);
        buttons_popover_unselect_all.set_visible(false);
    }

    if arr.contains(&PopoverTypes::Size) {
        buttons_popover_select_all_images_except_biggest.set_visible(true);
        buttons_popover_select_all_images_except_smallest.set_visible(true);
        separator_select_image_size.set_visible(true);
    } else {
        buttons_popover_select_all_images_except_biggest.set_visible(false);
        buttons_popover_select_all_images_except_smallest.set_visible(false);
        separator_select_image_size.set_visible(false);
    }

    if arr.contains(&PopoverTypes::Reverse) {
        buttons_popover_reverse.set_visible(true);
        separator_select_reverse.set_visible(true);
    } else {
        buttons_popover_reverse.set_visible(false);
        separator_select_reverse.set_visible(false);
    }

    if arr.contains(&PopoverTypes::Custom) {
        buttons_popover_select_custom.set_visible(true);
        buttons_popover_unselect_custom.set_visible(true);
        separator_select_custom.set_visible(true);
    } else {
        buttons_popover_select_custom.set_visible(false);
        buttons_popover_unselect_custom.set_visible(false);
        separator_select_custom.set_visible(false);
    }

    if arr.contains(&PopoverTypes::Date) {
        buttons_popover_select_all_except_oldest.set_visible(true);
        buttons_popover_select_all_except_newest.set_visible(true);
        buttons_popover_select_one_oldest.set_visible(true);
        buttons_popover_select_one_newest.set_visible(true);
        separator_select_date.set_visible(true);
    } else {
        buttons_popover_select_all_except_oldest.set_visible(false);
        buttons_popover_select_all_except_newest.set_visible(false);
        buttons_popover_select_one_oldest.set_visible(false);
        buttons_popover_select_one_newest.set_visible(false);
        separator_select_date.set_visible(false);
    }
    
    if arr.contains(&PopoverTypes::PathLength) {
        buttons_popover_select_all_except_shortest_path.set_visible(true);
        buttons_popover_select_all_except_longest_path.set_visible(true);
        separator_select_shortest_path.set_visible(true);
    } else {
        buttons_popover_select_all_except_shortest_path.set_visible(false);
        buttons_popover_select_all_except_longest_path.set_visible(false);
        separator_select_shortest_path.set_visible(false);
    }
}
