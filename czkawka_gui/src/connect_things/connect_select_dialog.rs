use gtk4::prelude::*;

use crate::gui_structs::common_tree_view::SubView;
use crate::gui_structs::gui_data::GuiData;
use crate::gui_structs::gui_select_dialog::GuiSelectDialog;

use super::connect_popovers_select::{
    exec_all_except_biggest_smallest, exec_all_except_longest_shortest_path, exec_all_except_oldest_newest, exec_custom_filter, exec_mark_same_size,
    exec_one_longest_shortest_path, exec_one_longest_shortest_path_oldest_newest, exec_one_longest_shortest_path_same_size, exec_one_oldest_newest,
    exec_one_oldest_newest_same_path, exec_one_oldest_newest_same_size, exec_reverse, exec_select_all, exec_unselect_all,
};

pub(crate) fn connect_select_dialog(gui_data: &GuiData) {
    let sd = gui_data.select_dialog.clone();
    let common_tree_views = gui_data.main_notebook.common_tree_views.clone();

    let sd_apply = sd.clone();
    sd.button_apply.connect_clicked(move |_| {
        let sv = common_tree_views.get_current_subview();
        dispatch_apply(&sd_apply, sv);
        sd_apply.dialog.set_visible(false);
    });
}

fn dispatch_apply(sd: &GuiSelectDialog, sv: &SubView) {
    if sd.action_select_all.is_active() {
        exec_select_all(sv);
        return;
    }
    if sd.action_unselect_all.is_active() {
        exec_unselect_all(sv);
        return;
    }
    if sd.action_reverse.is_active() {
        exec_reverse(sv);
        return;
    }
    if sd.action_mark_same_size.is_active() {
        exec_mark_same_size(sv);
        return;
    }
    if sd.action_custom_select.is_active() || sd.action_custom_unselect.is_active() {
        let select = sd.action_custom_select.is_active();
        exec_custom_filter(
            sv,
            select,
            &sd.custom_entry_name.text(),
            &sd.custom_entry_path.text(),
            &sd.custom_entry_regex.text(),
            sd.custom_check_name.is_active(),
            sd.custom_check_path.is_active(),
            sd.custom_check_regex.is_active(),
            sd.custom_check_case_sensitive.is_active(),
            sd.custom_check_all_in_group.is_active(),
        );
        return;
    }

    // AllExcept or SelectOne – need criterion
    let is_all_except = sd.action_all_except.is_active();
    let is_select_one = sd.action_select_one.is_active();

    if !is_all_except && !is_select_one {
        return;
    }

    if sd.criterion_date.is_active() {
        let oldest = sd.direction_oldest.is_active();
        if is_all_except {
            exec_all_except_oldest_newest(sv, oldest);
        } else {
            // SelectOne + Date
            if sd.path_filter_longest.is_active() {
                exec_one_longest_shortest_path_oldest_newest(sv, true, oldest);
            } else if sd.path_filter_shortest.is_active() {
                exec_one_longest_shortest_path_oldest_newest(sv, false, oldest);
            } else if sd.cond_same_size.is_active() {
                exec_one_oldest_newest_same_size(sv, oldest);
            } else if sd.cond_same_path.is_active() {
                exec_one_oldest_newest_same_path(sv, oldest);
            } else {
                exec_one_oldest_newest(sv, oldest);
            }
        }
        return;
    }

    if sd.criterion_path.is_active() {
        if is_select_one {
            let check_longest = sd.path_direction_longest.is_active();
            if sd.cond_same_size.is_active() {
                exec_one_longest_shortest_path_same_size(sv, check_longest);
            } else {
                exec_one_longest_shortest_path(sv, check_longest);
            }
        } else {
            // AllExcept: keep shortest/longest, select all others
            let except_longest = sd.path_direction_shortest.is_active(); // keep shortest = except longest
            exec_all_except_longest_shortest_path(sv, except_longest);
        }
        return;
    }

    if sd.criterion_size.is_active() {
        let except_biggest = sd.size_direction_biggest.is_active(); // keep biggest = except biggest (unselect biggest, select rest)
        exec_all_except_biggest_smallest(sv, except_biggest);
    }
}
