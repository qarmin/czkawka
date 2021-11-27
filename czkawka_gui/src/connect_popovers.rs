use gtk::prelude::*;
use gtk::{TreeIter, Window};

use czkawka_core::common::Common;

use crate::gui_data::GuiData;
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

fn popover_all_except_oldest_newest(popover: &gtk::Popover, tree_view: &gtk::TreeView, column_color: i32, column_modification_as_secs: i32, column_file_name: i32, column_button_selection: u32, except_oldest: bool) {
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

fn popover_one_oldest_newest(popover: &gtk::Popover, tree_view: &gtk::TreeView, column_color: i32, column_modification_as_secs: i32, column_file_name: i32, column_button_selection: u32, check_oldest: bool) {
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

fn popover_custom_select_unselect(popover: &gtk::Popover, window_main: &Window, tree_view: &gtk::TreeView, column_color: Option<i32>, column_file_name: i32, column_path: i32, column_button_selection: u32, select_things: bool) {
    popover.popdown();

    enum WildcardType {
        Path,
        Name,
        PathName,
    }

    let window_title = match select_things {
        false => "Unselect Custom",
        true => "Select Custom",
    };

    // Accept Dialog
    {
        let confirmation_dialog_delete = gtk::Dialog::with_buttons(Some(window_title), Some(window_main), gtk::DialogFlags::MODAL, &[("Ok", gtk::ResponseType::Ok), ("Close", gtk::ResponseType::Cancel)]);
        let label: gtk::Label = gtk::Label::new(Some("Usage: */folder-nr*/* or name-version-*.txt"));

        let radio_path = gtk::RadioButton::builder().label("Path").build();
        let radio_name_path = gtk::RadioButton::builder().label("Path + Name").build();
        radio_name_path.join_group(Some(&radio_path));
        let radio_name = gtk::RadioButton::builder().label("Name").build();
        radio_name.join_group(Some(&radio_path)); // TODO, not sure why this not exists for builder, but should

        let entry_path = gtk::Entry::new();
        let entry_name = gtk::Entry::new();
        let entry_name_path = gtk::Entry::new();

        label.set_margin_bottom(5);
        label.set_margin_end(5);
        label.set_margin_start(5);

        // TODO Label should have const width, and rest should fill entry, but for now is 50%-50%
        let grid = gtk::Grid::new();
        grid.set_row_homogeneous(true);
        grid.set_column_homogeneous(true);

        grid.attach(&label, 0, 0, 2, 1);

        grid.attach(&radio_path, 0, 1, 1, 1);
        grid.attach(&radio_name, 0, 2, 1, 1);
        grid.attach(&radio_name_path, 0, 3, 1, 1);

        grid.attach(&entry_path, 1, 1, 1, 1);
        grid.attach(&entry_name, 1, 2, 1, 1);
        grid.attach(&entry_name_path, 1, 3, 1, 1);

        let box_widget = get_dialog_box_child(&confirmation_dialog_delete);
        box_widget.add(&grid);

        confirmation_dialog_delete.show_all();

        let tree_view = tree_view.clone();
        confirmation_dialog_delete.connect_response(move |confirmation_dialog_delete, response_type| {
            let wildcard_type: WildcardType;
            let wildcard: String;

            if response_type == gtk::ResponseType::Ok {
                if radio_path.is_active() {
                    wildcard_type = WildcardType::Path;
                    wildcard = entry_path.text().to_string();
                } else if radio_name.is_active() {
                    wildcard_type = WildcardType::Name;
                    wildcard = entry_name.text().to_string();
                } else if radio_name_path.is_active() {
                    wildcard_type = WildcardType::PathName;
                    wildcard = entry_name_path.text().to_string();
                } else {
                    panic!("Non handled option in wildcard");
                }

                if !wildcard.is_empty() {
                    let wildcard = wildcard.trim();

                    #[cfg(target_family = "windows")]
                    let wildcard = wildcard.replace("/", "\\");
                    #[cfg(target_family = "windows")]
                    let wildcard = wildcard.as_str();

                    let model = get_list_store(&tree_view);

                    let iter = model.iter_first().unwrap(); // Never should be available button where there is no available records

                    loop {
                        if let Some(column_color) = column_color {
                            let color = model.value(&iter, column_color).get::<String>().unwrap();
                            if color == HEADER_ROW_COLOR {
                                if !model.iter_next(&iter) {
                                    break;
                                }
                                continue;
                            }
                        }

                        let path = model.value(&iter, column_path).get::<String>().unwrap();
                        let name = model.value(&iter, column_file_name).get::<String>().unwrap();
                        match wildcard_type {
                            WildcardType::Path => {
                                if Common::regex_check(wildcard, path) {
                                    model.set_value(&iter, column_button_selection, &select_things.to_value());
                                }
                            }
                            WildcardType::Name => {
                                if Common::regex_check(wildcard, name) {
                                    model.set_value(&iter, column_button_selection, &select_things.to_value());
                                }
                            }
                            WildcardType::PathName => {
                                if Common::regex_check(wildcard, format!("{}/{}", path, name)) {
                                    model.set_value(&iter, column_button_selection, &select_things.to_value());
                                }
                            }
                        }

                        if !model.iter_next(&iter) {
                            break;
                        }
                    }
                }
            } else {
                confirmation_dialog_delete.close();
                return;
            }
            confirmation_dialog_delete.close();
        });
    }
}

fn popover_all_except_biggest_smallest(popover: &gtk::Popover, tree_view: &gtk::TreeView, column_color: i32, column_size_as_bytes: i32, column_dimensions: i32, column_button_selection: u32, except_biggest: bool) {
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
            nb_object.column_dimensions.expect("AEB needs dimensions column"),
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
            nb_object.column_dimensions.expect("AES needs dimensions column"),
            nb_object.column_selection as u32,
            false,
        );
    });
}
