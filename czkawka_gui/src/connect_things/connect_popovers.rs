use gtk::prelude::*;
use gtk::{ResponseType, TreeIter, Window};
use regex::Regex;

use czkawka_core::common::Common;

use crate::flg;
use crate::gui_structs::gui_data::GuiData;
use crate::help_functions::*;

// File length variable allows users to choose duplicates which have shorter file name
// e.g. 'tar.gz' will be selected instead 'tar.gz (copy)' etc.

fn popover_select_all(popover: &gtk::Popover, tree_view: &gtk::TreeView, column_button_selection: u32, column_color: Option<i32>) {
    let model = get_list_store(tree_view);

    if let Some(iter) = model.iter_first() {
        if let Some(column_color) = column_color {
            loop {
                if model.value(&iter, column_color).get::<String>().unwrap() == MAIN_ROW_COLOR {
                    model.set_value(&iter, column_button_selection, &true.to_value());
                }
                if !model.iter_next(&iter) {
                    break;
                }
            }
        } else {
            loop {
                model.set_value(&iter, column_button_selection, &true.to_value());

                if !model.iter_next(&iter) {
                    break;
                }
            }
        }
    }
    popover.popdown();
}

fn popover_unselect_all(popover: &gtk::Popover, tree_view: &gtk::TreeView, column_button_selection: u32) {
    let model = get_list_store(tree_view);

    if let Some(iter) = model.iter_first() {
        loop {
            model.set_value(&iter, column_button_selection, &false.to_value());

            if !model.iter_next(&iter) {
                break;
            }
        }
    }
    popover.popdown();
}

fn popover_reverse(popover: &gtk::Popover, tree_view: &gtk::TreeView, column_button_selection: u32, column_color: Option<i32>) {
    let model = get_list_store(tree_view);

    if let Some(iter) = model.iter_first() {
        if let Some(column_color) = column_color {
            loop {
                if model.value(&iter, column_color).get::<String>().unwrap() == MAIN_ROW_COLOR {
                    let current_value: bool = model.value(&iter, column_button_selection as i32).get::<bool>().unwrap();
                    model.set_value(&iter, column_button_selection, &(!current_value).to_value());
                }
                if !model.iter_next(&iter) {
                    break;
                }
            }
        } else {
            loop {
                let current_value: bool = model.value(&iter, column_button_selection as i32).get::<bool>().unwrap();
                model.set_value(&iter, column_button_selection, &(!current_value).to_value());

                if !model.iter_next(&iter) {
                    break;
                }
            }
        }
    }
    popover.popdown();
}

fn popover_all_except_oldest_newest(
    popover: &gtk::Popover,
    tree_view: &gtk::TreeView,
    column_color: i32,
    column_modification_as_secs: i32,
    column_file_name: i32,
    column_button_selection: u32,
    except_oldest: bool,
) {
    let model = get_list_store(tree_view);

    if let Some(iter) = model.iter_first() {
        let mut end: bool = false;
        loop {
            let mut tree_iter_array: Vec<TreeIter> = Vec::new();
            let mut used_index: Option<usize> = None;
            let mut current_index: usize = 0;
            let mut modification_time_min_max: u64 = match except_oldest {
                true => u64::MAX,
                false => 0,
            };

            let mut file_length: usize = 0;

            loop {
                let color = model.value(&iter, column_color).get::<String>().unwrap();
                if color == HEADER_ROW_COLOR {
                    if !model.iter_next(&iter) {
                        end = true;
                    }
                    break;
                }
                tree_iter_array.push(iter.clone());
                let modification = model.value(&iter, column_modification_as_secs).get::<u64>().unwrap();
                let current_file_length = model.value(&iter, column_file_name).get::<String>().unwrap().len();
                if except_oldest {
                    if modification < modification_time_min_max || (modification == modification_time_min_max && current_file_length < file_length) {
                        file_length = current_file_length;
                        modification_time_min_max = modification;
                        used_index = Some(current_index);
                    }
                } else {
                    if modification > modification_time_min_max || (modification == modification_time_min_max && current_file_length < file_length) {
                        file_length = current_file_length;
                        modification_time_min_max = modification;
                        used_index = Some(current_index);
                    }
                }
                current_index += 1;

                if !model.iter_next(&iter) {
                    end = true;
                    break;
                }
            }
            if used_index == None {
                continue;
            }
            for (index, tree_iter) in tree_iter_array.iter().enumerate() {
                if index != used_index.unwrap() {
                    model.set_value(tree_iter, column_button_selection, &true.to_value());
                } else {
                    model.set_value(tree_iter, column_button_selection, &false.to_value());
                }
            }

            if end {
                break;
            }
        }
    }

    popover.popdown();
}

fn popover_one_oldest_newest(
    popover: &gtk::Popover,
    tree_view: &gtk::TreeView,
    column_color: i32,
    column_modification_as_secs: i32,
    column_file_name: i32,
    column_button_selection: u32,
    check_oldest: bool,
) {
    let model = get_list_store(tree_view);

    if let Some(iter) = model.iter_first() {
        let mut end: bool = false;
        loop {
            let mut tree_iter_array: Vec<TreeIter> = Vec::new();
            let mut used_index: Option<usize> = None;
            let mut current_index: usize = 0;
            let mut modification_time_min_max: u64 = match check_oldest {
                true => u64::MAX,
                false => 0,
            };

            let mut file_length: usize = 0;

            loop {
                let color = model.value(&iter, column_color).get::<String>().unwrap();
                if color == HEADER_ROW_COLOR {
                    if !model.iter_next(&iter) {
                        end = true;
                    }
                    break;
                }
                tree_iter_array.push(iter.clone());
                let modification = model.value(&iter, column_modification_as_secs).get::<u64>().unwrap();
                let current_file_length = model.value(&iter, column_file_name).get::<String>().unwrap().len();
                if check_oldest {
                    if modification < modification_time_min_max || (modification == modification_time_min_max && current_file_length > file_length) {
                        file_length = current_file_length;
                        modification_time_min_max = modification;
                        used_index = Some(current_index);
                    }
                } else {
                    if modification > modification_time_min_max || (modification == modification_time_min_max && current_file_length > file_length) {
                        file_length = current_file_length;
                        modification_time_min_max = modification;
                        used_index = Some(current_index);
                    }
                }

                current_index += 1;

                if !model.iter_next(&iter) {
                    end = true;
                    break;
                }
            }
            if used_index == None {
                continue;
            }
            for (index, tree_iter) in tree_iter_array.iter().enumerate() {
                if index == used_index.unwrap() {
                    model.set_value(tree_iter, column_button_selection, &true.to_value());
                } else {
                    model.set_value(tree_iter, column_button_selection, &false.to_value());
                }
            }

            if end {
                break;
            }
        }
    }

    popover.popdown();
}

fn popover_custom_select_unselect(
    popover: &gtk::Popover,
    window_main: &Window,
    tree_view: &gtk::TreeView,
    column_color: Option<i32>,
    column_file_name: i32,
    column_path: i32,
    column_button_selection: u32,
    select_things: bool,
) {
    popover.popdown();

    let window_title = match select_things {
        false => flg!("popover_custom_mode_unselect"),
        true => flg!("popover_custom_mode_select"),
    };

    // Dialog for select/unselect items
    {
        let dialog = gtk::Dialog::builder().title(&window_title).transient_for(window_main).modal(true).build();
        dialog.add_button(&flg!("general_ok_button"), ResponseType::Ok);
        dialog.add_button(&flg!("general_close_button"), ResponseType::Cancel);

        let check_button_path = gtk::CheckButton::builder().label(&flg!("popover_custom_regex_path_label")).build();
        let check_button_name = gtk::CheckButton::builder().label(&flg!("popover_custom_regex_name_label")).build();
        let check_button_rust_regex = gtk::CheckButton::builder().label(&flg!("popover_custom_regex_regex_label")).build();

        let check_button_select_not_all_results = gtk::CheckButton::builder().label(&flg!("popover_custom_all_in_group_label")).build();
        check_button_select_not_all_results.set_active(true);

        let entry_path = gtk::Entry::new();
        let entry_name = gtk::Entry::new();
        let entry_rust_regex = gtk::Entry::new();
        entry_rust_regex.set_sensitive(false); // By default check button regex is disabled

        let label_regex_valid = gtk::Label::new(None);

        // Tooltips
        {
            check_button_path.set_tooltip_text(Some(&flg!("popover_custom_path_check_button_entry_tooltip")));
            entry_path.set_tooltip_text(Some(&flg!("popover_custom_path_check_button_entry_tooltip")));

            check_button_name.set_tooltip_text(Some(&flg!("popover_custom_name_check_button_entry_tooltip")));
            entry_name.set_tooltip_text(Some(&flg!("popover_custom_name_check_button_entry_tooltip")));

            check_button_rust_regex.set_tooltip_text(Some(&flg!("popover_custom_regex_check_button_entry_tooltip")));
            entry_rust_regex.set_tooltip_text(Some(&flg!("popover_custom_regex_check_button_entry_tooltip")));

            check_button_select_not_all_results.set_tooltip_text(Some(&flg!("popover_custom_not_all_check_button_tooltip")));
        }
        {
            let label_regex_valid = label_regex_valid.clone();
            entry_rust_regex.connect_changed(move |entry_rust_regex| {
                let message;
                let text_to_check = entry_rust_regex.text().to_string();
                if text_to_check.is_empty() {
                    message = "".to_string();
                } else {
                    match Regex::new(&text_to_check) {
                        Ok(_) => message = flg!("popover_valid_regex"),
                        Err(_) => message = flg!("popover_invalid_regex"),
                    }
                }

                // TODO add red and green color to text
                // let attributes_list = AttrList::new();
                // let p_a = PangoAttribute::init();
                // let attribute = PangoAttrFontDesc { attr };
                // attributes_list.insert(attribute);
                // label_regex_valid.set_attributes(Some(&attributes_list));
                label_regex_valid.set_text(&message);
            });
        }

        // Disable other modes when Rust Regex is enabled
        {
            let check_button_path = check_button_path.clone();
            let check_button_name = check_button_name.clone();
            let check_button_rust_regex = check_button_rust_regex.clone();
            let entry_path = entry_path.clone();
            let entry_name = entry_name.clone();
            let entry_rust_regex = entry_rust_regex.clone();
            check_button_rust_regex.connect_toggled(move |check_button_rust_regex| {
                if check_button_rust_regex.is_active() {
                    check_button_path.set_sensitive(false);
                    check_button_name.set_sensitive(false);
                    entry_path.set_sensitive(false);
                    entry_name.set_sensitive(false);
                    entry_rust_regex.set_sensitive(true);
                } else {
                    check_button_path.set_sensitive(true);
                    check_button_name.set_sensitive(true);
                    entry_path.set_sensitive(true);
                    entry_name.set_sensitive(true);
                    entry_rust_regex.set_sensitive(false);
                }
            });
        }

        // Configure look of things
        {
            // TODO Label should have const width, and rest should fill entry, but for now is 50%-50%
            let grid = gtk::Grid::new();
            grid.set_row_homogeneous(true);
            grid.set_column_homogeneous(true);

            grid.attach(&check_button_name, 0, 1, 1, 1);
            grid.attach(&check_button_path, 0, 2, 1, 1);
            grid.attach(&check_button_rust_regex, 0, 3, 1, 1);

            grid.attach(&entry_name, 1, 1, 1, 1);
            grid.attach(&entry_path, 1, 2, 1, 1);
            grid.attach(&entry_rust_regex, 1, 3, 1, 1);

            grid.attach(&label_regex_valid, 0, 4, 2, 1);

            if select_things {
                grid.attach(&check_button_select_not_all_results, 0, 5, 2, 1);
            }

            let box_widget = get_dialog_box_child(&dialog);
            box_widget.add(&grid);

            dialog.show_all();
        }

        let tree_view = tree_view.clone();
        dialog.connect_response(move |confirmation_dialog_select_unselect, response_type| {
            let name_widcard = entry_name.text().trim().to_string();
            let path_widcard = entry_path.text().trim().to_string();
            let regex_widcard = entry_rust_regex.text().trim().to_string();

            #[cfg(target_family = "windows")]
            let name_widcard = name_widcard.replace("/", "\\");
            #[cfg(target_family = "windows")]
            let path_widcard = name_widcard.replace("/", "\\");

            if response_type == gtk::ResponseType::Ok {
                let check_path = check_button_path.is_active();
                let check_name = check_button_name.is_active();
                let check_regex = check_button_rust_regex.is_active();

                let check_all_selected = check_button_select_not_all_results.is_active();

                if check_button_path.is_active() || check_button_name.is_active() || check_button_rust_regex.is_active() {
                    let compiled_regex = match check_regex {
                        true => match Regex::new(&regex_widcard) {
                            Ok(t) => t,
                            Err(_) => {
                                eprintln!("What? Regex should compile properly.");
                                confirmation_dialog_select_unselect.close();
                                return;
                            }
                        },
                        false => Regex::new("").unwrap(),
                    };

                    let model = get_list_store(&tree_view);

                    let iter = match model.iter_first() {
                        Some(t) => t,
                        None => {
                            confirmation_dialog_select_unselect.close();
                            return;
                        }
                    };

                    let mut number_of_all_things = 0;
                    let mut number_of_already_selected_things = 0;
                    let mut vec_of_iters: Vec<TreeIter> = Vec::new();
                    loop {
                        if let Some(column_color) = column_color {
                            let color = model.value(&iter, column_color).get::<String>().unwrap();
                            if color == HEADER_ROW_COLOR {
                                if select_things {
                                    if check_all_selected && (number_of_all_things - number_of_already_selected_things == vec_of_iters.len()) {
                                        vec_of_iters.pop();
                                    }
                                    for iter in vec_of_iters {
                                        model.set_value(&iter, column_button_selection, &true.to_value());
                                    }
                                } else {
                                    for iter in vec_of_iters {
                                        model.set_value(&iter, column_button_selection, &false.to_value());
                                    }
                                }

                                if !model.iter_next(&iter) {
                                    break;
                                }

                                number_of_all_things = 0;
                                number_of_already_selected_things = 0;
                                vec_of_iters = Vec::new();
                                continue;
                            }
                        }

                        let is_selected = model.value(&iter, column_button_selection as i32).get::<bool>().unwrap();
                        let path = model.value(&iter, column_path).get::<String>().unwrap();
                        let name = model.value(&iter, column_file_name).get::<String>().unwrap();

                        let path_and_name = get_full_name_from_path_name(&path, &name);

                        let mut need_to_change_thing: bool = false;

                        number_of_all_things += 1;
                        if check_regex && compiled_regex.find(&path_and_name).is_some() {
                            need_to_change_thing = true;
                        } else {
                            if check_name && Common::regex_check(&name_widcard, &name) {
                                need_to_change_thing = true;
                            }
                            if check_path && Common::regex_check(&path_widcard, &path) {
                                need_to_change_thing = true;
                            }
                        }

                        if need_to_change_thing {
                            if select_things {
                                if is_selected {
                                    number_of_already_selected_things += 1;
                                } else {
                                    vec_of_iters.push(iter.clone());
                                }
                            } else {
                                vec_of_iters.push(iter.clone());
                            }
                        }

                        if !model.iter_next(&iter) {
                            if select_things {
                                if check_all_selected && (number_of_all_things - number_of_already_selected_things == vec_of_iters.len()) {
                                    vec_of_iters.pop();
                                }
                                for iter in vec_of_iters {
                                    model.set_value(&iter, column_button_selection, &true.to_value());
                                }
                            } else {
                                for iter in vec_of_iters {
                                    model.set_value(&iter, column_button_selection, &false.to_value());
                                }
                            }
                            break;
                        }
                    }
                }
            }
            confirmation_dialog_select_unselect.close();
        });
    }
}

fn popover_all_except_biggest_smallest(
    popover: &gtk::Popover,
    tree_view: &gtk::TreeView,
    column_color: i32,
    column_size_as_bytes: i32,
    column_dimensions: Option<i32>,
    column_button_selection: u32,
    except_biggest: bool,
) {
    let model = get_list_store(tree_view);

    if let Some(iter) = model.iter_first() {
        let mut end: bool = false;
        loop {
            let mut tree_iter_array: Vec<TreeIter> = Vec::new();
            let mut used_index: Option<usize> = None;
            let mut current_index: usize = 0;
            let mut size_as_bytes_min_max: u64 = match except_biggest {
                true => 0,
                false => u64::MAX,
            };
            let mut number_of_pixels_min_max: u64 = match except_biggest {
                true => 0,
                false => u64::MAX,
            };

            loop {
                let color = model.value(&iter, column_color).get::<String>().unwrap();
                if color == HEADER_ROW_COLOR {
                    if !model.iter_next(&iter) {
                        end = true;
                    }
                    break;
                }
                tree_iter_array.push(iter.clone());
                let size_as_bytes = model.value(&iter, column_size_as_bytes).get::<u64>().unwrap();

                // If dimension exists, then needs to be checked images
                if let Some(column_dimensions) = column_dimensions {
                    let dimensions_string = model.value(&iter, column_dimensions).get::<String>().unwrap();

                    let dimensions = change_dimension_to_krotka(dimensions_string);
                    let number_of_pixels = dimensions.0 * dimensions.1;

                    if except_biggest {
                        if number_of_pixels > number_of_pixels_min_max || (number_of_pixels == number_of_pixels_min_max && size_as_bytes > size_as_bytes_min_max) {
                            number_of_pixels_min_max = number_of_pixels;
                            size_as_bytes_min_max = size_as_bytes;
                            used_index = Some(current_index);
                        }
                    } else {
                        if number_of_pixels < number_of_pixels_min_max || (number_of_pixels == number_of_pixels_min_max && size_as_bytes < size_as_bytes_min_max) {
                            number_of_pixels_min_max = number_of_pixels;
                            size_as_bytes_min_max = size_as_bytes;
                            used_index = Some(current_index);
                        }
                    }
                } else {
                    if except_biggest {
                        if size_as_bytes > size_as_bytes_min_max {
                            size_as_bytes_min_max = size_as_bytes;
                            used_index = Some(current_index);
                        }
                    } else {
                        if size_as_bytes < size_as_bytes_min_max {
                            size_as_bytes_min_max = size_as_bytes;
                            used_index = Some(current_index);
                        }
                    }
                }

                current_index += 1;

                if !model.iter_next(&iter) {
                    end = true;
                    break;
                }
            }
            if used_index == None {
                continue;
            }
            for (index, tree_iter) in tree_iter_array.iter().enumerate() {
                if index != used_index.unwrap() {
                    model.set_value(tree_iter, column_button_selection, &true.to_value());
                } else {
                    model.set_value(tree_iter, column_button_selection, &false.to_value());
                }
            }

            if end {
                break;
            }
        }
    }

    popover.popdown();
}

pub fn connect_popovers(gui_data: &GuiData) {
    let popover_select = gui_data.popovers.popover_select.clone();
    let buttons_popover_select_all = gui_data.popovers.buttons_popover_select_all.clone();
    let notebook_main = gui_data.main_notebook.notebook_main.clone();
    let main_tree_views = gui_data.main_notebook.get_main_tree_views();

    buttons_popover_select_all.connect_clicked(move |_| {
        let nb_number = notebook_main.current_page().unwrap();
        let tree_view = &main_tree_views[nb_number as usize];
        let nb_object = &NOTEBOOKS_INFOS[nb_number as usize];

        popover_select_all(&popover_select, tree_view, nb_object.column_selection as u32, nb_object.column_color);
    });

    let popover_select = gui_data.popovers.popover_select.clone();
    let buttons_popover_unselect_all = gui_data.popovers.buttons_popover_unselect_all.clone();
    let notebook_main = gui_data.main_notebook.notebook_main.clone();
    let main_tree_views = gui_data.main_notebook.get_main_tree_views();
    buttons_popover_unselect_all.connect_clicked(move |_| {
        let nb_number = notebook_main.current_page().unwrap();
        let tree_view = &main_tree_views[nb_number as usize];
        let nb_object = &NOTEBOOKS_INFOS[nb_number as usize];

        popover_unselect_all(&popover_select, tree_view, nb_object.column_selection as u32);
    });

    let popover_select = gui_data.popovers.popover_select.clone();
    let buttons_popover_reverse = gui_data.popovers.buttons_popover_reverse.clone();
    let notebook_main = gui_data.main_notebook.notebook_main.clone();
    let main_tree_views = gui_data.main_notebook.get_main_tree_views();
    buttons_popover_reverse.connect_clicked(move |_| {
        let nb_number = notebook_main.current_page().unwrap();
        let tree_view = &main_tree_views[nb_number as usize];
        let nb_object = &NOTEBOOKS_INFOS[nb_number as usize];

        popover_reverse(&popover_select, tree_view, nb_object.column_selection as u32, nb_object.column_color);
    });

    let popover_select = gui_data.popovers.popover_select.clone();
    let buttons_popover_select_all_except_oldest = gui_data.popovers.buttons_popover_select_all_except_oldest.clone();
    let notebook_main = gui_data.main_notebook.notebook_main.clone();
    let main_tree_views = gui_data.main_notebook.get_main_tree_views();
    buttons_popover_select_all_except_oldest.connect_clicked(move |_| {
        let nb_number = notebook_main.current_page().unwrap();
        let tree_view = &main_tree_views[nb_number as usize];
        let nb_object = &NOTEBOOKS_INFOS[nb_number as usize];

        popover_all_except_oldest_newest(
            &popover_select,
            tree_view,
            nb_object.column_color.expect("AEO can't be used without headers"),
            nb_object.column_modification_as_secs.expect("AEO needs modification as secs column"),
            nb_object.column_name,
            nb_object.column_selection as u32,
            true,
        );
    });

    let popover_select = gui_data.popovers.popover_select.clone();
    let buttons_popover_select_all_except_newest = gui_data.popovers.buttons_popover_select_all_except_newest.clone();
    let notebook_main = gui_data.main_notebook.notebook_main.clone();
    let main_tree_views = gui_data.main_notebook.get_main_tree_views();
    buttons_popover_select_all_except_newest.connect_clicked(move |_| {
        let nb_number = notebook_main.current_page().unwrap();
        let tree_view = &main_tree_views[nb_number as usize];
        let nb_object = &NOTEBOOKS_INFOS[nb_number as usize];

        popover_all_except_oldest_newest(
            &popover_select,
            tree_view,
            nb_object.column_color.expect("AEN can't be used without headers"),
            nb_object.column_modification_as_secs.expect("AEN needs modification as secs column"),
            nb_object.column_name,
            nb_object.column_selection as u32,
            false,
        );
    });

    let popover_select = gui_data.popovers.popover_select.clone();
    let buttons_popover_select_one_oldest = gui_data.popovers.buttons_popover_select_one_oldest.clone();
    let notebook_main = gui_data.main_notebook.notebook_main.clone();
    let main_tree_views = gui_data.main_notebook.get_main_tree_views();
    buttons_popover_select_one_oldest.connect_clicked(move |_| {
        let nb_number = notebook_main.current_page().unwrap();
        let tree_view = &main_tree_views[nb_number as usize];
        let nb_object = &NOTEBOOKS_INFOS[nb_number as usize];

        popover_one_oldest_newest(
            &popover_select,
            tree_view,
            nb_object.column_color.expect("OO can't be used without headers"),
            nb_object.column_modification_as_secs.expect("OO needs modification as secs column"),
            nb_object.column_name,
            nb_object.column_selection as u32,
            true,
        );
    });

    let popover_select = gui_data.popovers.popover_select.clone();
    let buttons_popover_select_one_newest = gui_data.popovers.buttons_popover_select_one_newest.clone();
    let notebook_main = gui_data.main_notebook.notebook_main.clone();
    let main_tree_views = gui_data.main_notebook.get_main_tree_views();
    buttons_popover_select_one_newest.connect_clicked(move |_| {
        let nb_number = notebook_main.current_page().unwrap();
        let tree_view = &main_tree_views[nb_number as usize];
        let nb_object = &NOTEBOOKS_INFOS[nb_number as usize];

        popover_one_oldest_newest(
            &popover_select,
            tree_view,
            nb_object.column_color.expect("ON can't be used without headers"),
            nb_object.column_modification_as_secs.expect("ON needs modification as secs column"),
            nb_object.column_name,
            nb_object.column_selection as u32,
            false,
        );
    });

    let popover_select = gui_data.popovers.popover_select.clone();
    let buttons_popover_select_custom = gui_data.popovers.buttons_popover_select_custom.clone();
    let notebook_main = gui_data.main_notebook.notebook_main.clone();
    let main_tree_views = gui_data.main_notebook.get_main_tree_views();
    let window_main = gui_data.window_main.clone();
    buttons_popover_select_custom.connect_clicked(move |_| {
        let nb_number = notebook_main.current_page().unwrap();
        let tree_view = &main_tree_views[nb_number as usize];
        let nb_object = &NOTEBOOKS_INFOS[nb_number as usize];

        popover_custom_select_unselect(
            &popover_select,
            &window_main,
            tree_view,
            nb_object.column_color,
            nb_object.column_name,
            nb_object.column_path,
            nb_object.column_selection as u32,
            true,
        );
    });

    let popover_select = gui_data.popovers.popover_select.clone();
    let buttons_popover_unselect_custom = gui_data.popovers.buttons_popover_unselect_custom.clone();
    let notebook_main = gui_data.main_notebook.notebook_main.clone();
    let main_tree_views = gui_data.main_notebook.get_main_tree_views();
    let window_main = gui_data.window_main.clone();
    buttons_popover_unselect_custom.connect_clicked(move |_| {
        let nb_number = notebook_main.current_page().unwrap();
        let tree_view = &main_tree_views[nb_number as usize];
        let nb_object = &NOTEBOOKS_INFOS[nb_number as usize];

        popover_custom_select_unselect(
            &popover_select,
            &window_main,
            tree_view,
            nb_object.column_color,
            nb_object.column_name,
            nb_object.column_path,
            nb_object.column_selection as u32,
            false,
        );
    });

    let popover_select = gui_data.popovers.popover_select.clone();
    let buttons_popover_select_all_images_except_biggest = gui_data.popovers.buttons_popover_select_all_images_except_biggest.clone();
    let notebook_main = gui_data.main_notebook.notebook_main.clone();
    let main_tree_views = gui_data.main_notebook.get_main_tree_views();
    buttons_popover_select_all_images_except_biggest.connect_clicked(move |_| {
        let nb_number = notebook_main.current_page().unwrap();
        let tree_view = &main_tree_views[nb_number as usize];
        let nb_object = &NOTEBOOKS_INFOS[nb_number as usize];

        popover_all_except_biggest_smallest(
            &popover_select,
            tree_view,
            nb_object.column_color.expect("AEB can't be used without headers"),
            nb_object.column_size_as_bytes.expect("AEB needs size as bytes column"),
            nb_object.column_dimensions,
            nb_object.column_selection as u32,
            true,
        );
    });

    let popover_select = gui_data.popovers.popover_select.clone();
    let buttons_popover_select_all_images_except_smallest = gui_data.popovers.buttons_popover_select_all_images_except_smallest.clone();
    let notebook_main = gui_data.main_notebook.notebook_main.clone();
    let main_tree_views = gui_data.main_notebook.get_main_tree_views();
    buttons_popover_select_all_images_except_smallest.connect_clicked(move |_| {
        let nb_number = notebook_main.current_page().unwrap();
        let tree_view = &main_tree_views[nb_number as usize];
        let nb_object = &NOTEBOOKS_INFOS[nb_number as usize];

        popover_all_except_biggest_smallest(
            &popover_select,
            tree_view,
            nb_object.column_color.expect("AES can't be used without headers"),
            nb_object.column_size_as_bytes.expect("AES needs size as bytes column"),
            nb_object.column_dimensions,
            nb_object.column_selection as u32,
            false,
        );
    });
}
