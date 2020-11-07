extern crate gtk;
use crate::gui_data::GuiData;
use crate::help_functions::*;
use gtk::prelude::*;
use gtk::TreeIter;

pub fn connect_popover_duplicate(gui_data: &GuiData) {
    // Select all button
    {
        let buttons_popover_select_all = gui_data.buttons_popover_select_all.clone();
        let scrolled_window_duplicate_finder = gui_data.scrolled_window_duplicate_finder.clone();
        let popover_select = gui_data.popover_select.clone();
        buttons_popover_select_all.connect_clicked(move |_| {
            let tree_view = get_tree_view(&scrolled_window_duplicate_finder);
            let selection = tree_view.get_selection();

            selection.select_all();
            popover_select.popdown();
        });
    }

    // Unselect all button
    {
        let scrolled_window_duplicate_finder = gui_data.scrolled_window_duplicate_finder.clone();
        let buttons_popover_unselect_all = gui_data.buttons_popover_unselect_all.clone();
        let popover_select = gui_data.popover_select.clone();
        buttons_popover_unselect_all.connect_clicked(move |_| {
            let tree_view = get_tree_view(&scrolled_window_duplicate_finder);
            let selection = tree_view.get_selection();

            selection.unselect_all();
            popover_select.popdown();
        });
    }

    // Reverse selection
    {
        let scrolled_window_duplicate_finder = gui_data.scrolled_window_duplicate_finder.clone();
        let popover_select = gui_data.popover_select.clone();
        let buttons_popover_reverse = gui_data.buttons_popover_reverse.clone();
        buttons_popover_reverse.connect_clicked(move |_| {
            let tree_view = get_tree_view(&scrolled_window_duplicate_finder);
            let selection = tree_view.get_selection();

            let (vector_tree_path, tree_model) = selection.get_selected_rows();

            if vector_tree_path.is_empty() {
                selection.select_all();
            } else {
                let tree_iter_all = tree_model.get_iter_first().unwrap(); // Never should be available button where there is no available records

                let mut current_path_index = 0;
                let mut tree_iter_selected: TreeIter;
                loop {
                    if current_path_index >= vector_tree_path.len() {
                        selection.select_iter(&tree_iter_all);
                    } else {
                        tree_iter_selected = tree_model.get_iter(vector_tree_path.get(current_path_index).unwrap()).unwrap();
                        if tree_model.get_path(&tree_iter_all).unwrap() == tree_model.get_path(&tree_iter_selected).unwrap() {
                            selection.unselect_iter(&tree_iter_selected);
                            current_path_index += 1;
                        } else {
                            selection.select_iter(&tree_iter_all);
                        }
                    }
                    if !tree_model.iter_next(&tree_iter_all) {
                        break;
                    }
                }
            }

            popover_select.popdown();
        });
    }

    // All except oldest
    {
        let scrolled_window_duplicate_finder = gui_data.scrolled_window_duplicate_finder.clone();
        let popover_select = gui_data.popover_select.clone();
        let buttons_popover_select_all_except_oldest = gui_data.buttons_popover_select_all_except_oldest.clone();
        buttons_popover_select_all_except_oldest.connect_clicked(move |_| {
            let tree_view = get_tree_view(&scrolled_window_duplicate_finder);
            let selection = tree_view.get_selection();
            let tree_model = tree_view.get_model().unwrap();

            let tree_iter_all = tree_model.get_iter_first().unwrap(); // Never should be available button where there is no available records

            let mut end: bool = false;

            loop {
                let mut tree_iter_array: Vec<TreeIter> = Vec::new();
                let mut oldest_index: Option<usize> = None;
                let mut current_index: usize = 0;
                let mut oldest_modification_time: u64 = u64::max_value();

                loop {
                    let color = tree_model.get_value(&tree_iter_all, ColumnsDuplicates::Color as i32).get::<String>().unwrap().unwrap();
                    if color == HEADER_ROW_COLOR {
                        if !tree_model.iter_next(&tree_iter_all) {
                            end = true;
                        }
                        break;
                    }
                    tree_iter_array.push(tree_iter_all.clone());
                    let modification = tree_model.get_value(&tree_iter_all, ColumnsDuplicates::ModificationAsSecs as i32).get::<u64>().unwrap().unwrap();
                    if modification < oldest_modification_time {
                        oldest_modification_time = modification;
                        oldest_index = Some(current_index);
                    }

                    current_index += 1;

                    if !tree_model.iter_next(&tree_iter_all) {
                        end = true;
                        break;
                    }
                }
                if oldest_index == None {
                    continue;
                }
                for (index, tree_iter) in tree_iter_array.iter().enumerate() {
                    if index != oldest_index.unwrap() {
                        selection.select_iter(tree_iter);
                    } else {
                        selection.unselect_iter(tree_iter);
                    }
                }

                if end {
                    break;
                }
            }

            popover_select.popdown();
        });
    }

    // All except newest
    {
        let scrolled_window_duplicate_finder = gui_data.scrolled_window_duplicate_finder.clone();
        let popover_select = gui_data.popover_select.clone();
        let buttons_popover_select_all_except_newest = gui_data.buttons_popover_select_all_except_newest.clone();
        buttons_popover_select_all_except_newest.connect_clicked(move |_| {
            let tree_view = get_tree_view(&scrolled_window_duplicate_finder);
            let selection = tree_view.get_selection();
            let tree_model = tree_view.get_model().unwrap();

            let tree_iter_all = tree_model.get_iter_first().unwrap(); // Never should be available button where there is no available records

            let mut end: bool = false;

            loop {
                let mut tree_iter_array: Vec<TreeIter> = Vec::new();
                let mut newest_index: Option<usize> = None;
                let mut current_index: usize = 0;
                let mut newest_modification_time: u64 = 0;

                loop {
                    let color = tree_model.get_value(&tree_iter_all, ColumnsDuplicates::Color as i32).get::<String>().unwrap().unwrap();
                    if color == HEADER_ROW_COLOR {
                        if !tree_model.iter_next(&tree_iter_all) {
                            end = true;
                        }
                        break;
                    }
                    tree_iter_array.push(tree_iter_all.clone());
                    let modification = tree_model.get_value(&tree_iter_all, ColumnsDuplicates::ModificationAsSecs as i32).get::<u64>().unwrap().unwrap();
                    if modification > newest_modification_time {
                        newest_modification_time = modification;
                        newest_index = Some(current_index);
                    }

                    current_index += 1;

                    if !tree_model.iter_next(&tree_iter_all) {
                        end = true;
                        break;
                    }
                }
                if newest_index == None {
                    continue;
                }
                for (index, tree_iter) in tree_iter_array.iter().enumerate() {
                    if index != newest_index.unwrap() {
                        selection.select_iter(tree_iter);
                    } else {
                        selection.unselect_iter(tree_iter);
                    }
                }

                if end {
                    break;
                }
            }

            popover_select.popdown();
        });
    }

    // All one oldest
    {
        let scrolled_window_duplicate_finder = gui_data.scrolled_window_duplicate_finder.clone();
        let popover_select = gui_data.popover_select.clone();
        let buttons_popover_select_one_oldest = gui_data.buttons_popover_select_one_oldest.clone();
        buttons_popover_select_one_oldest.connect_clicked(move |_| {
            let tree_view = get_tree_view(&scrolled_window_duplicate_finder);
            let selection = tree_view.get_selection();
            let tree_model = tree_view.get_model().unwrap();

            let tree_iter_all = tree_model.get_iter_first().unwrap(); // Never should be available button where there is no available records

            let mut end: bool = false;

            loop {
                let mut tree_iter_array: Vec<TreeIter> = Vec::new();
                let mut oldest_index: Option<usize> = None;
                let mut current_index: usize = 0;
                let mut oldest_modification_time: u64 = u64::max_value();

                loop {
                    let color = tree_model.get_value(&tree_iter_all, ColumnsDuplicates::Color as i32).get::<String>().unwrap().unwrap();
                    if color == HEADER_ROW_COLOR {
                        if !tree_model.iter_next(&tree_iter_all) {
                            end = true;
                        }
                        break;
                    }
                    tree_iter_array.push(tree_iter_all.clone());
                    let modification = tree_model.get_value(&tree_iter_all, ColumnsDuplicates::ModificationAsSecs as i32).get::<u64>().unwrap().unwrap();
                    if modification < oldest_modification_time {
                        oldest_modification_time = modification;
                        oldest_index = Some(current_index);
                    }

                    current_index += 1;

                    if !tree_model.iter_next(&tree_iter_all) {
                        end = true;
                        break;
                    }
                }
                if oldest_index == None {
                    continue;
                }
                for (index, tree_iter) in tree_iter_array.iter().enumerate() {
                    if index == oldest_index.unwrap() {
                        selection.select_iter(tree_iter);
                    } else {
                        selection.unselect_iter(tree_iter);
                    }
                }

                if end {
                    break;
                }
            }

            popover_select.popdown();
        });
    }
    // All one newest
    {
        let scrolled_window_duplicate_finder = gui_data.scrolled_window_duplicate_finder.clone();
        let buttons_popover_select_one_newest = gui_data.buttons_popover_select_one_newest.clone();
        let popover_select = gui_data.popover_select.clone();
        buttons_popover_select_one_newest.connect_clicked(move |_| {
            let tree_view = get_tree_view(&scrolled_window_duplicate_finder);
            let selection = tree_view.get_selection();
            let tree_model = tree_view.get_model().unwrap();

            let tree_iter_all = tree_model.get_iter_first().unwrap(); // Never should be available button where there is no available records

            let mut end: bool = false;

            loop {
                let mut tree_iter_array: Vec<TreeIter> = Vec::new();
                let mut newest_index: Option<usize> = None;
                let mut current_index: usize = 0;
                let mut newest_modification_time: u64 = 0;

                loop {
                    let color = tree_model.get_value(&tree_iter_all, ColumnsDuplicates::Color as i32).get::<String>().unwrap().unwrap();
                    if color == HEADER_ROW_COLOR {
                        if !tree_model.iter_next(&tree_iter_all) {
                            end = true;
                        }
                        break;
                    }
                    tree_iter_array.push(tree_iter_all.clone());
                    let modification = tree_model.get_value(&tree_iter_all, ColumnsDuplicates::ModificationAsSecs as i32).get::<u64>().unwrap().unwrap();
                    if modification > newest_modification_time {
                        newest_modification_time = modification;
                        newest_index = Some(current_index);
                    }

                    current_index += 1;

                    if !tree_model.iter_next(&tree_iter_all) {
                        end = true;
                        break;
                    }
                }
                if newest_index == None {
                    continue;
                }
                for (index, tree_iter) in tree_iter_array.iter().enumerate() {
                    if index == newest_index.unwrap() {
                        selection.select_iter(tree_iter);
                    } else {
                        selection.unselect_iter(tree_iter);
                    }
                }

                if end {
                    break;
                }
            }

            popover_select.popdown();
        });
    }
}
