extern crate gtk;
use crate::gui_data::GuiData;
use crate::help_functions::*;
use czkawka_core::common::Common;
use gtk::prelude::*;
use gtk::TreeIter;

// File length variable allows users to choose duplicates which have shorter file name
// e.g. 'tar.gz' will be selected instead 'tar.gz (copy)' etc.

fn popover_select_all(popover: &gtk::Popover, scrolled_window: &gtk::ScrolledWindow) {
    let tree_view = get_tree_view(&scrolled_window);
    let selection = tree_view.get_selection();

    selection.select_all();
    popover.popdown();
}
fn popover_unselect_all(popover: &gtk::Popover, scrolled_window: &gtk::ScrolledWindow) {
    let tree_view = get_tree_view(&scrolled_window);
    let selection = tree_view.get_selection();

    selection.unselect_all();
    popover.popdown();
}
fn popover_reverse(popover: &gtk::Popover, scrolled_window: &gtk::ScrolledWindow) {
    let tree_view = get_tree_view(&scrolled_window);
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

    popover.popdown();
}

fn popover_all_except_oldest(popover: &gtk::Popover, scrolled_window: &gtk::ScrolledWindow, column_color: i32, column_modification_as_secs: i32, column_file_name: i32) {
    let tree_view = get_tree_view(&scrolled_window);
    let selection = tree_view.get_selection();
    let tree_model = tree_view.get_model().unwrap();

    let tree_iter_all = tree_model.get_iter_first().unwrap(); // Never should be available button where there is no available records

    let mut end: bool = false;

    loop {
        let mut tree_iter_array: Vec<TreeIter> = Vec::new();
        let mut oldest_index: Option<usize> = None;
        let mut current_index: usize = 0;
        let mut oldest_modification_time: u64 = u64::max_value();

        let mut file_length: usize = 0;

        loop {
            let color = tree_model.get_value(&tree_iter_all, column_color).get::<String>().unwrap().unwrap();
            if color == HEADER_ROW_COLOR {
                if !tree_model.iter_next(&tree_iter_all) {
                    end = true;
                }
                break;
            }
            tree_iter_array.push(tree_iter_all.clone());
            let modification = tree_model.get_value(&tree_iter_all, column_modification_as_secs).get::<u64>().unwrap().unwrap();
            let current_file_length = tree_model.get_value(&tree_iter_all, column_file_name).get::<String>().unwrap().unwrap().len();
            if modification < oldest_modification_time || (modification == oldest_modification_time && current_file_length < file_length) {
                file_length = current_file_length;
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

    popover.popdown();
}
fn popover_all_except_newest(popover: &gtk::Popover, scrolled_window: &gtk::ScrolledWindow, column_color: i32, column_modification_as_secs: i32, column_file_name: i32) {
    let tree_view = get_tree_view(&scrolled_window);
    let selection = tree_view.get_selection();
    let tree_model = tree_view.get_model().unwrap();

    let tree_iter_all = tree_model.get_iter_first().unwrap(); // Never should be available button where there is no available records

    let mut end: bool = false;

    loop {
        let mut tree_iter_array: Vec<TreeIter> = Vec::new();
        let mut newest_index: Option<usize> = None;
        let mut current_index: usize = 0;
        let mut newest_modification_time: u64 = 0;

        let mut file_length: usize = 0;

        loop {
            let color = tree_model.get_value(&tree_iter_all, column_color).get::<String>().unwrap().unwrap();
            if color == HEADER_ROW_COLOR {
                if !tree_model.iter_next(&tree_iter_all) {
                    end = true;
                }
                break;
            }
            tree_iter_array.push(tree_iter_all.clone());
            let modification = tree_model.get_value(&tree_iter_all, column_modification_as_secs).get::<u64>().unwrap().unwrap();
            let current_file_length = tree_model.get_value(&tree_iter_all, column_file_name).get::<String>().unwrap().unwrap().len();
            if modification > newest_modification_time || (modification == newest_modification_time && current_file_length < file_length) {
                file_length = current_file_length;
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

    popover.popdown();
}
fn popover_one_oldest(popover: &gtk::Popover, scrolled_window: &gtk::ScrolledWindow, column_color: i32, column_modification_as_secs: i32, column_file_name: i32) {
    let tree_view = get_tree_view(&scrolled_window);
    let selection = tree_view.get_selection();
    let tree_model = tree_view.get_model().unwrap();

    let tree_iter_all = tree_model.get_iter_first().unwrap(); // Never should be available button where there is no available records

    let mut end: bool = false;

    loop {
        let mut tree_iter_array: Vec<TreeIter> = Vec::new();
        let mut oldest_index: Option<usize> = None;
        let mut current_index: usize = 0;
        let mut oldest_modification_time: u64 = u64::max_value();

        let mut file_length: usize = 0;

        loop {
            let color = tree_model.get_value(&tree_iter_all, column_color).get::<String>().unwrap().unwrap();
            if color == HEADER_ROW_COLOR {
                if !tree_model.iter_next(&tree_iter_all) {
                    end = true;
                }
                break;
            }
            tree_iter_array.push(tree_iter_all.clone());
            let modification = tree_model.get_value(&tree_iter_all, column_modification_as_secs).get::<u64>().unwrap().unwrap();
            let current_file_length = tree_model.get_value(&tree_iter_all, column_file_name).get::<String>().unwrap().unwrap().len();
            if modification < oldest_modification_time || (modification == oldest_modification_time && current_file_length > file_length) {
                file_length = current_file_length;
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

    popover.popdown();
}
fn popover_one_newest(popover: &gtk::Popover, scrolled_window: &gtk::ScrolledWindow, column_color: i32, column_modification_as_secs: i32, column_file_name: i32) {
    let tree_view = get_tree_view(&scrolled_window);
    let selection = tree_view.get_selection();
    let tree_model = tree_view.get_model().unwrap();

    let tree_iter_all = tree_model.get_iter_first().unwrap(); // Never should be available button where there is no available records

    let mut end: bool = false;

    loop {
        let mut tree_iter_array: Vec<TreeIter> = Vec::new();
        let mut newest_index: Option<usize> = None;
        let mut current_index: usize = 0;
        let mut newest_modification_time: u64 = 0;

        let mut file_length: usize = 0;
        loop {
            let color = tree_model.get_value(&tree_iter_all, column_color).get::<String>().unwrap().unwrap();
            if color == HEADER_ROW_COLOR {
                if !tree_model.iter_next(&tree_iter_all) {
                    end = true;
                }
                break;
            }
            tree_iter_array.push(tree_iter_all.clone());
            let modification = tree_model.get_value(&tree_iter_all, column_modification_as_secs).get::<u64>().unwrap().unwrap();
            let current_file_length = tree_model.get_value(&tree_iter_all, column_file_name).get::<String>().unwrap().unwrap().len();
            if modification > newest_modification_time || (modification == newest_modification_time && current_file_length > file_length) {
                file_length = current_file_length;
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

    popover.popdown();
}

fn popover_select_custom(popover: &gtk::Popover, gui_data: &GuiData, scrolled_window: &gtk::ScrolledWindow, column_color: Option<i32>, column_file_name: i32, column_path: i32) {
    popover.popdown();

    let wildcard: String;
    enum WildcardType {
        Path,
        Name,
        PathName,
    };
    let wildcard_type: WildcardType;

    // Accept Dialog
    {
        let window_main = gui_data.window_main.clone();
        let confirmation_dialog_delete = gtk::Dialog::with_buttons(Some("Select custom"), Some(&window_main), gtk::DialogFlags::MODAL, &[("Ok", gtk::ResponseType::Ok), ("Close", gtk::ResponseType::Cancel)]);
        let label: gtk::Label = gtk::Label::new(Some("Usage: */folder-nr*/* or name-version-*.txt"));

        let radio_path = gtk::RadioButton::with_label("Path");
        let radio_name = gtk::RadioButton::with_label_from_widget(&radio_path, "Name");
        let radio_name_path = gtk::RadioButton::with_label_from_widget(&radio_path, "Path + Name");

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

        for widgets in confirmation_dialog_delete.get_children() {
            // By default GtkBox is child of dialog, so we can easily add other things to it
            widgets.downcast::<gtk::Box>().unwrap().add(&grid);
        }

        confirmation_dialog_delete.show_all();

        let response_type = confirmation_dialog_delete.run();
        if response_type == gtk::ResponseType::Ok {
            if radio_path.get_active() {
                wildcard_type = WildcardType::Path;
                wildcard = entry_path.get_text().to_string();
            } else if radio_name.get_active() {
                wildcard_type = WildcardType::Name;
                wildcard = entry_name.get_text().to_string();
            } else if radio_name_path.get_active() {
                wildcard_type = WildcardType::PathName;
                wildcard = entry_name_path.get_text().to_string();
            } else {
                panic!("Non handled option in select wildcard");
            }
        } else {
            confirmation_dialog_delete.close();
            return;
        }
        confirmation_dialog_delete.close();
    }
    if !wildcard.is_empty() {
        let wildcard = wildcard.trim();

        let tree_view = get_tree_view(&scrolled_window);
        let selection = tree_view.get_selection();
        let tree_model = tree_view.get_model().unwrap();

        let tree_iter = tree_model.get_iter_first().unwrap(); // Never should be available button where there is no available records

        loop {
            if let Some(column_color) = column_color {
                let color = tree_model.get_value(&tree_iter, column_color).get::<String>().unwrap().unwrap();
                if color == HEADER_ROW_COLOR {
                    if !tree_model.iter_next(&tree_iter) {
                        break;
                    }
                    continue;
                }
            }

            let path = tree_model.get_value(&tree_iter, column_path).get::<String>().unwrap().unwrap();
            let name = tree_model.get_value(&tree_iter, column_file_name).get::<String>().unwrap().unwrap();
            match wildcard_type {
                WildcardType::Path => {
                    if Common::regex_check(wildcard, path) {
                        selection.select_iter(&tree_iter);
                    }
                }
                WildcardType::Name => {
                    if Common::regex_check(wildcard, name) {
                        selection.select_iter(&tree_iter);
                    }
                }
                WildcardType::PathName => {
                    if Common::regex_check(wildcard, format!("{}/{}", path, name)) {
                        selection.select_iter(&tree_iter);
                    }
                }
            }

            if !tree_model.iter_next(&tree_iter) {
                break;
            }
        }
    }
}
fn popover_unselect_custom(popover: &gtk::Popover, gui_data: &GuiData, scrolled_window: &gtk::ScrolledWindow, column_color: Option<i32>, column_file_name: i32, column_path: i32) {
    popover.popdown();

    let wildcard: String;
    enum WildcardType {
        Path,
        Name,
        PathName,
    };
    let wildcard_type: WildcardType;

    // Accept Dialog
    {
        let window_main = gui_data.window_main.clone();
        let confirmation_dialog_delete = gtk::Dialog::with_buttons(Some("Unselect custom"), Some(&window_main), gtk::DialogFlags::MODAL, &[("Ok", gtk::ResponseType::Ok), ("Close", gtk::ResponseType::Cancel)]);
        let label: gtk::Label = gtk::Label::new(Some("Usage: */folder-nr*/* or name-version-*.txt"));

        let radio_path = gtk::RadioButton::with_label("Path");
        let radio_name = gtk::RadioButton::with_label_from_widget(&radio_path, "Name");
        let radio_name_path = gtk::RadioButton::with_label_from_widget(&radio_path, "Path + Name");

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

        let response_type = confirmation_dialog_delete.run();
        if response_type == gtk::ResponseType::Ok {
            if radio_path.get_active() {
                wildcard_type = WildcardType::Path;
                wildcard = entry_path.get_text().to_string();
            } else if radio_name.get_active() {
                wildcard_type = WildcardType::Name;
                wildcard = entry_name.get_text().to_string();
            } else if radio_name_path.get_active() {
                wildcard_type = WildcardType::PathName;
                wildcard = entry_name_path.get_text().to_string();
            } else {
                panic!("Non handled option in unselect wildcard");
            }
        } else {
            confirmation_dialog_delete.close();
            return;
        }
        confirmation_dialog_delete.close();
    }
    if !wildcard.is_empty() {
        let wildcard = wildcard.trim();

        let tree_view = get_tree_view(&scrolled_window);
        let selection = tree_view.get_selection();
        let tree_model = tree_view.get_model().unwrap();

        let tree_iter = tree_model.get_iter_first().unwrap(); // Never should be available button where there is no available records

        loop {
            if let Some(column_color) = column_color {
                let color = tree_model.get_value(&tree_iter, column_color).get::<String>().unwrap().unwrap();
                if color == HEADER_ROW_COLOR {
                    if !tree_model.iter_next(&tree_iter) {
                        break;
                    }
                    continue;
                }
            }

            let path = tree_model.get_value(&tree_iter, column_path).get::<String>().unwrap().unwrap();
            let name = tree_model.get_value(&tree_iter, column_file_name).get::<String>().unwrap().unwrap();
            match wildcard_type {
                WildcardType::Path => {
                    if Common::regex_check(wildcard, path) {
                        selection.unselect_iter(&tree_iter);
                    }
                }
                WildcardType::Name => {
                    if Common::regex_check(wildcard, name) {
                        selection.unselect_iter(&tree_iter);
                    }
                }
                WildcardType::PathName => {
                    if Common::regex_check(wildcard, format!("{}/{}", path, name)) {
                        selection.unselect_iter(&tree_iter);
                    }
                }
            }

            if !tree_model.iter_next(&tree_iter) {
                break;
            }
        }
    }
}

fn popover_all_except_biggest(popover: &gtk::Popover, scrolled_window: &gtk::ScrolledWindow, column_color: i32, column_size_as_bytes: i32, column_dimensions: i32) {
    let tree_view = get_tree_view(&scrolled_window);
    let selection = tree_view.get_selection();
    let tree_model = tree_view.get_model().unwrap();

    let tree_iter_all = tree_model.get_iter_first().unwrap(); // Never should be available button where there is no available records

    let mut end: bool = false;

    loop {
        let mut tree_iter_array: Vec<TreeIter> = Vec::new();
        let mut biggest_index: Option<usize> = None;
        let mut current_index: usize = 0;
        let mut biggest_size_as_bytes: u64 = 0;
        let mut biggest_number_of_pixels: u64 = 0;

        loop {
            let color = tree_model.get_value(&tree_iter_all, column_color).get::<String>().unwrap().unwrap();
            if color == HEADER_ROW_COLOR {
                if !tree_model.iter_next(&tree_iter_all) {
                    end = true;
                }
                break;
            }
            tree_iter_array.push(tree_iter_all.clone());
            let size_as_bytes = tree_model.get_value(&tree_iter_all, column_size_as_bytes).get::<u64>().unwrap().unwrap();
            let dimensions_string = tree_model.get_value(&tree_iter_all, column_dimensions).get::<String>().unwrap().unwrap();

            let dimensions = change_dimension_to_krotka(dimensions_string);
            let number_of_pixels = dimensions.0 * dimensions.1;

            if number_of_pixels > biggest_number_of_pixels || (number_of_pixels == biggest_number_of_pixels && size_as_bytes > biggest_size_as_bytes) {
                biggest_number_of_pixels = number_of_pixels;
                biggest_size_as_bytes = size_as_bytes;
                biggest_index = Some(current_index);
            }

            current_index += 1;

            if !tree_model.iter_next(&tree_iter_all) {
                end = true;
                break;
            }
        }
        if biggest_index == None {
            continue;
        }
        for (index, tree_iter) in tree_iter_array.iter().enumerate() {
            if index != biggest_index.unwrap() {
                selection.select_iter(tree_iter);
            } else {
                selection.unselect_iter(tree_iter);
            }
        }

        if end {
            break;
        }
    }

    popover.popdown();
}
fn popover_all_except_smallest(popover: &gtk::Popover, scrolled_window: &gtk::ScrolledWindow, column_color: i32, column_size_as_bytes: i32, column_dimensions: i32) {
    let tree_view = get_tree_view(&scrolled_window);
    let selection = tree_view.get_selection();
    let tree_model = tree_view.get_model().unwrap();

    let tree_iter_all = tree_model.get_iter_first().unwrap(); // Never should be available button where there is no available records

    let mut end: bool = false;

    loop {
        let mut tree_iter_array: Vec<TreeIter> = Vec::new();
        let mut smallest_index: Option<usize> = None;
        let mut current_index: usize = 0;
        let mut smallest_size_as_bytes: u64 = u64::max_value();
        let mut smallest_number_of_pixels: u64 = u64::max_value();

        loop {
            let color = tree_model.get_value(&tree_iter_all, column_color).get::<String>().unwrap().unwrap();
            if color == HEADER_ROW_COLOR {
                if !tree_model.iter_next(&tree_iter_all) {
                    end = true;
                }
                break;
            }
            tree_iter_array.push(tree_iter_all.clone());
            let size_as_bytes = tree_model.get_value(&tree_iter_all, column_size_as_bytes).get::<u64>().unwrap().unwrap();
            let dimensions_string = tree_model.get_value(&tree_iter_all, column_dimensions).get::<String>().unwrap().unwrap();

            let dimensions = change_dimension_to_krotka(dimensions_string);
            let number_of_pixels = dimensions.0 * dimensions.1;

            if number_of_pixels < smallest_number_of_pixels || (number_of_pixels == smallest_number_of_pixels && size_as_bytes < smallest_size_as_bytes) {
                smallest_number_of_pixels = number_of_pixels;
                smallest_size_as_bytes = size_as_bytes;
                smallest_index = Some(current_index);
            }

            current_index += 1;

            if !tree_model.iter_next(&tree_iter_all) {
                end = true;
                break;
            }
        }
        if smallest_index == None {
            continue;
        }
        for (index, tree_iter) in tree_iter_array.iter().enumerate() {
            if index != smallest_index.unwrap() {
                selection.select_iter(tree_iter);
            } else {
                selection.unselect_iter(tree_iter);
            }
        }

        if end {
            break;
        }
    }

    popover.popdown();
}

#[derive(Clone)]
pub struct PopoverObject {
    pub name: String,
    pub available_modes: Vec<String>,
    pub scrolled_windows: gtk::ScrolledWindow,
    pub column_path: Option<i32>,
    pub column_name: Option<i32>,
    pub column_color: Option<i32>,
    pub column_dimensions: Option<i32>,
    pub column_size: Option<i32>,
    pub column_size_as_bytes: Option<i32>,
    pub column_modification_as_secs: Option<i32>,
}

pub fn find_name(name: &str, vec: &[PopoverObject]) -> Option<PopoverObject> {
    for e in vec {
        if e.name == *name {
            return Some(e.clone());
        }
    }
    None
}

pub fn connect_popovers(gui_data: &GuiData) {
    let popover_objects = vec![
        PopoverObject {
            name: "notebook_main_duplicate_finder_label".to_string(),
            available_modes: vec!["all", "reverse", "custom", "date"].iter().map(|e| e.to_string()).collect(),
            scrolled_windows: gui_data.scrolled_window_duplicate_finder.clone(),
            column_path: Some(ColumnsDuplicates::Path as i32),
            column_name: Some(ColumnsDuplicates::Name as i32),
            column_color: Some(ColumnsDuplicates::Color as i32),
            column_dimensions: None,
            column_size: None,
            column_size_as_bytes: None,
            column_modification_as_secs: Some(ColumnsDuplicates::ModificationAsSecs as i32),
        },
        PopoverObject {
            name: "notebook_main_same_music_finder".to_string(),
            available_modes: vec!["all", "reverse", "custom", "date"].iter().map(|e| e.to_string()).collect(),
            scrolled_windows: gui_data.scrolled_window_same_music_finder.clone(),
            column_path: Some(ColumnsSameMusic::Path as i32),
            column_name: Some(ColumnsSameMusic::Name as i32),
            column_color: Some(ColumnsSameMusic::Color as i32),
            column_dimensions: None,
            column_size: None,
            column_size_as_bytes: Some(ColumnsSameMusic::SizeAsBytes as i32),
            column_modification_as_secs: Some(ColumnsSameMusic::ModificationAsSecs as i32),
        },
        PopoverObject {
            name: "notebook_main_similar_images_finder_label".to_string(),
            available_modes: vec!["all", "reverse", "custom", "date"].iter().map(|e| e.to_string()).collect(),
            scrolled_windows: gui_data.scrolled_window_similar_images_finder.clone(),
            column_path: Some(ColumnsSimilarImages::Path as i32),
            column_name: Some(ColumnsSimilarImages::Name as i32),
            column_color: Some(ColumnsSimilarImages::Color as i32),
            column_dimensions: Some(ColumnsSimilarImages::Dimensions as i32),
            column_size: Some(ColumnsSimilarImages::Size as i32),
            column_size_as_bytes: Some(ColumnsSimilarImages::SizeAsBytes as i32),
            column_modification_as_secs: Some(ColumnsSimilarImages::ModificationAsSecs as i32),
        },
        PopoverObject {
            name: "scrolled_window_main_empty_folder_finder".to_string(),
            available_modes: vec!["all", "reverse", "custom"].iter().map(|e| e.to_string()).collect(),
            scrolled_windows: gui_data.scrolled_window_main_empty_folder_finder.clone(),
            column_path: Some(ColumnsEmptyFolders::Path as i32),
            column_name: Some(ColumnsEmptyFolders::Name as i32),
            column_color: None,
            column_dimensions: None,
            column_size: None,
            column_size_as_bytes: None,
            column_modification_as_secs: None,
        },
        PopoverObject {
            name: "scrolled_window_main_empty_files_finder".to_string(),
            available_modes: vec!["all", "reverse", "custom"].iter().map(|e| e.to_string()).collect(),
            scrolled_windows: gui_data.scrolled_window_main_empty_files_finder.clone(),
            column_path: Some(ColumnsEmptyFiles::Path as i32),
            column_name: Some(ColumnsEmptyFiles::Name as i32),
            column_color: None,
            column_dimensions: None,
            column_size: None,
            column_size_as_bytes: None,
            column_modification_as_secs: None,
        },
        PopoverObject {
            name: "scrolled_window_main_temporary_files_finder".to_string(),
            available_modes: vec!["all", "reverse", "custom"].iter().map(|e| e.to_string()).collect(),
            scrolled_windows: gui_data.scrolled_window_main_temporary_files_finder.clone(),
            column_path: Some(ColumnsTemporaryFiles::Path as i32),
            column_name: Some(ColumnsTemporaryFiles::Name as i32),
            column_color: None,
            column_dimensions: None,
            column_size: None,
            column_size_as_bytes: None,
            column_modification_as_secs: None,
        },
        PopoverObject {
            name: "notebook_big_main_file_finder".to_string(),
            available_modes: vec!["all", "reverse", "custom"].iter().map(|e| e.to_string()).collect(),
            scrolled_windows: gui_data.scrolled_window_big_files_finder.clone(),
            column_path: Some(ColumnsBigFiles::Path as i32),
            column_name: Some(ColumnsBigFiles::Name as i32),
            column_color: None,
            column_dimensions: None,
            column_size: None,
            column_size_as_bytes: None,
            column_modification_as_secs: None,
        },
        PopoverObject {
            name: "notebook_main_zeroed_files_finder".to_string(),
            available_modes: vec!["all", "reverse", "custom"].iter().map(|e| e.to_string()).collect(),
            scrolled_windows: gui_data.scrolled_window_zeroed_files_finder.clone(),
            column_path: Some(ColumnsZeroedFiles::Path as i32),
            column_name: Some(ColumnsZeroedFiles::Name as i32),
            column_color: None,
            column_dimensions: None,
            column_size: None,
            column_size_as_bytes: Some(ColumnsZeroedFiles::SizeAsBytes as i32),
            column_modification_as_secs: None,
        },
    ];

    let popover_select = gui_data.popover_select.clone();
    let notebook_main_children_names = gui_data.notebook_main_children_names.clone();
    let buttons_popover_select_all = gui_data.buttons_popover_select_all.clone();
    let notebook_main = gui_data.notebook_main.clone();
    let vec_popover_objects = popover_objects.clone();
    buttons_popover_select_all.connect_clicked(move |_| {
        let object_popover = find_name(notebook_main_children_names.get(notebook_main.get_current_page().unwrap() as usize).unwrap(), &vec_popover_objects).unwrap();
        popover_select_all(&popover_select, &object_popover.scrolled_windows);
    });

    let popover_select = gui_data.popover_select.clone();
    let notebook_main_children_names = gui_data.notebook_main_children_names.clone();
    let buttons_popover_unselect_all = gui_data.buttons_popover_unselect_all.clone();
    let notebook_main = gui_data.notebook_main.clone();
    let vec_popover_objects = popover_objects.clone();
    buttons_popover_unselect_all.connect_clicked(move |_| {
        let object_popover = find_name(notebook_main_children_names.get(notebook_main.get_current_page().unwrap() as usize).unwrap(), &vec_popover_objects).unwrap();
        popover_unselect_all(&popover_select, &object_popover.scrolled_windows);
    });

    let popover_select = gui_data.popover_select.clone();
    let notebook_main_children_names = gui_data.notebook_main_children_names.clone();
    let buttons_popover_reverse = gui_data.buttons_popover_reverse.clone();
    let notebook_main = gui_data.notebook_main.clone();
    let vec_popover_objects = popover_objects.clone();
    buttons_popover_reverse.connect_clicked(move |_| {
        let object_popover = find_name(notebook_main_children_names.get(notebook_main.get_current_page().unwrap() as usize).unwrap(), &vec_popover_objects).unwrap();
        popover_reverse(&popover_select, &object_popover.scrolled_windows);
    });

    let popover_select = gui_data.popover_select.clone();
    let notebook_main_children_names = gui_data.notebook_main_children_names.clone();
    let buttons_popover_select_all_except_oldest = gui_data.buttons_popover_select_all_except_oldest.clone();
    let notebook_main = gui_data.notebook_main.clone();
    let vec_popover_objects = popover_objects.clone();
    buttons_popover_select_all_except_oldest.connect_clicked(move |_| {
        let object_popover = find_name(notebook_main_children_names.get(notebook_main.get_current_page().unwrap() as usize).unwrap(), &vec_popover_objects).unwrap();
        popover_all_except_oldest(
            &popover_select,
            &object_popover.scrolled_windows,
            object_popover.column_color.unwrap(),
            object_popover.column_modification_as_secs.unwrap(),
            object_popover.column_name.unwrap(),
        );
    });

    let popover_select = gui_data.popover_select.clone();
    let notebook_main_children_names = gui_data.notebook_main_children_names.clone();
    let buttons_popover_select_all_except_newest = gui_data.buttons_popover_select_all_except_newest.clone();
    let notebook_main = gui_data.notebook_main.clone();
    let vec_popover_objects = popover_objects.clone();
    buttons_popover_select_all_except_newest.connect_clicked(move |_| {
        let object_popover = find_name(notebook_main_children_names.get(notebook_main.get_current_page().unwrap() as usize).unwrap(), &vec_popover_objects).unwrap();
        popover_all_except_newest(
            &popover_select,
            &object_popover.scrolled_windows,
            object_popover.column_color.unwrap(),
            object_popover.column_modification_as_secs.unwrap(),
            object_popover.column_name.unwrap(),
        );
    });

    let popover_select = gui_data.popover_select.clone();
    let notebook_main_children_names = gui_data.notebook_main_children_names.clone();
    let buttons_popover_select_one_oldest = gui_data.buttons_popover_select_one_oldest.clone();
    let notebook_main = gui_data.notebook_main.clone();
    let vec_popover_objects = popover_objects.clone();
    buttons_popover_select_one_oldest.connect_clicked(move |_| {
        let object_popover = find_name(notebook_main_children_names.get(notebook_main.get_current_page().unwrap() as usize).unwrap(), &vec_popover_objects).unwrap();
        popover_one_oldest(
            &popover_select,
            &object_popover.scrolled_windows,
            object_popover.column_color.unwrap(),
            object_popover.column_modification_as_secs.unwrap(),
            object_popover.column_name.unwrap(),
        );
    });

    let popover_select = gui_data.popover_select.clone();
    let notebook_main_children_names = gui_data.notebook_main_children_names.clone();
    let buttons_popover_select_one_newest = gui_data.buttons_popover_select_one_newest.clone();
    let notebook_main = gui_data.notebook_main.clone();
    let vec_popover_objects = popover_objects.clone();
    buttons_popover_select_one_newest.connect_clicked(move |_| {
        let object_popover = find_name(notebook_main_children_names.get(notebook_main.get_current_page().unwrap() as usize).unwrap(), &vec_popover_objects).unwrap();
        popover_one_newest(
            &popover_select,
            &object_popover.scrolled_windows,
            object_popover.column_color.unwrap(),
            object_popover.column_modification_as_secs.unwrap(),
            object_popover.column_name.unwrap(),
        );
    });

    let popover_select = gui_data.popover_select.clone();
    let notebook_main_children_names = gui_data.notebook_main_children_names.clone();
    let buttons_popover_select_custom = gui_data.buttons_popover_select_custom.clone();
    let notebook_main = gui_data.notebook_main.clone();
    let vec_popover_objects = popover_objects.clone();
    let gui_data_clone = gui_data.clone();
    buttons_popover_select_custom.connect_clicked(move |_| {
        let object_popover = find_name(notebook_main_children_names.get(notebook_main.get_current_page().unwrap() as usize).unwrap(), &vec_popover_objects).unwrap();
        popover_select_custom(
            &popover_select,
            &gui_data_clone,
            &object_popover.scrolled_windows,
            object_popover.column_color,
            object_popover.column_name.unwrap(),
            object_popover.column_path.unwrap(),
        );
    });

    let popover_select = gui_data.popover_select.clone();
    let notebook_main_children_names = gui_data.notebook_main_children_names.clone();
    let buttons_popover_unselect_custom = gui_data.buttons_popover_unselect_custom.clone();
    let notebook_main = gui_data.notebook_main.clone();
    let vec_popover_objects = popover_objects.clone();
    let gui_data_clone = gui_data.clone();
    buttons_popover_unselect_custom.connect_clicked(move |_| {
        let object_popover = find_name(notebook_main_children_names.get(notebook_main.get_current_page().unwrap() as usize).unwrap(), &vec_popover_objects).unwrap();
        popover_unselect_custom(
            &popover_select,
            &gui_data_clone,
            &object_popover.scrolled_windows,
            object_popover.column_color,
            object_popover.column_name.unwrap(),
            object_popover.column_path.unwrap(),
        );
    });

    let popover_select = gui_data.popover_select.clone();
    let notebook_main_children_names = gui_data.notebook_main_children_names.clone();
    let buttons_popover_select_all_images_except_biggest = gui_data.buttons_popover_select_all_images_except_biggest.clone();
    let notebook_main = gui_data.notebook_main.clone();
    let vec_popover_objects = popover_objects.clone();
    buttons_popover_select_all_images_except_biggest.connect_clicked(move |_| {
        let object_popover = find_name(notebook_main_children_names.get(notebook_main.get_current_page().unwrap() as usize).unwrap(), &vec_popover_objects).unwrap();
        popover_all_except_biggest(
            &popover_select,
            &object_popover.scrolled_windows,
            object_popover.column_color.unwrap(),
            object_popover.column_size_as_bytes.unwrap(),
            object_popover.column_dimensions.unwrap(),
        );
    });

    let popover_select = gui_data.popover_select.clone();
    let notebook_main_children_names = gui_data.notebook_main_children_names.clone();
    let buttons_popover_select_all_images_except_smallest = gui_data.buttons_popover_select_all_images_except_smallest.clone();
    let notebook_main = gui_data.notebook_main.clone();
    let vec_popover_objects = popover_objects; //.clone();
    buttons_popover_select_all_images_except_smallest.connect_clicked(move |_| {
        let object_popover = find_name(notebook_main_children_names.get(notebook_main.get_current_page().unwrap() as usize).unwrap(), &vec_popover_objects).unwrap();
        popover_all_except_smallest(
            &popover_select,
            &object_popover.scrolled_windows,
            object_popover.column_color.unwrap(),
            object_popover.column_size_as_bytes.unwrap(),
            object_popover.column_dimensions.unwrap(),
        );
    });
}
