use crate::gui_data::GuiData;
use crate::help_functions::*;
use crate::notebook_enums::*;
use czkawka_core::common::Common;
use gtk::prelude::*;
use gtk::TreeIter;

// File length variable allows users to choose duplicates which have shorter file name
// e.g. 'tar.gz' will be selected instead 'tar.gz (copy)' etc.

fn popover_select_all(popover: &gtk::Popover, tree_view: &gtk::TreeView, column_button_selection: u32) {
    let model = get_list_store(tree_view);

    if let Some(iter) = model.iter_first() {
        loop {
            model.set_value(&iter, column_button_selection, &true.to_value());

            if !model.iter_next(&iter) {
                break;
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
fn popover_reverse(popover: &gtk::Popover, tree_view: &gtk::TreeView, column_button_selection: u32) {
    let model = get_list_store(tree_view);

    if let Some(iter) = model.iter_first() {
        loop {
            let current_value: bool = model.value(&iter, column_button_selection as i32).get::<bool>().unwrap();
            model.set_value(&iter, column_button_selection, &(!current_value).to_value());

            if !model.iter_next(&iter) {
                break;
            }
        }
    }
    popover.popdown();
}

fn popover_all_except_oldest(popover: &gtk::Popover, tree_view: &gtk::TreeView, column_color: i32, column_modification_as_secs: i32, column_file_name: i32, column_button_selection: u32) {
    let model = get_list_store(tree_view);

    if let Some(iter) = model.iter_first() {
        let mut end: bool = false;
        loop {
            let mut tree_iter_array: Vec<TreeIter> = Vec::new();
            let mut oldest_index: Option<usize> = None;
            let mut current_index: usize = 0;
            let mut oldest_modification_time: u64 = u64::MAX;

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
                if modification < oldest_modification_time || (modification == oldest_modification_time && current_file_length < file_length) {
                    file_length = current_file_length;
                    oldest_modification_time = modification;
                    oldest_index = Some(current_index);
                }

                current_index += 1;

                if !model.iter_next(&iter) {
                    end = true;
                    break;
                }
            }
            if oldest_index == None {
                continue;
            }
            for (index, tree_iter) in tree_iter_array.iter().enumerate() {
                if index != oldest_index.unwrap() {
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
fn popover_all_except_newest(popover: &gtk::Popover, tree_view: &gtk::TreeView, column_color: i32, column_modification_as_secs: i32, column_file_name: i32, column_button_selection: u32) {
    let model = get_list_store(tree_view);

    if let Some(iter) = model.iter_first() {
        let mut end: bool = false;
        loop {
            let mut tree_iter_array: Vec<TreeIter> = Vec::new();
            let mut newest_index: Option<usize> = None;
            let mut current_index: usize = 0;
            let mut newest_modification_time: u64 = 0;

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
                if modification > newest_modification_time || (modification == newest_modification_time && current_file_length < file_length) {
                    file_length = current_file_length;
                    newest_modification_time = modification;
                    newest_index = Some(current_index);
                }

                current_index += 1;

                if !model.iter_next(&iter) {
                    end = true;
                    break;
                }
            }
            if newest_index == None {
                continue;
            }
            for (index, tree_iter) in tree_iter_array.iter().enumerate() {
                if index != newest_index.unwrap() {
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
fn popover_one_oldest(popover: &gtk::Popover, tree_view: &gtk::TreeView, column_color: i32, column_modification_as_secs: i32, column_file_name: i32, column_button_selection: u32) {
    let model = get_list_store(tree_view);

    if let Some(iter) = model.iter_first() {
        let mut end: bool = false;
        loop {
            let mut tree_iter_array: Vec<TreeIter> = Vec::new();
            let mut oldest_index: Option<usize> = None;
            let mut current_index: usize = 0;
            let mut oldest_modification_time: u64 = u64::MAX;

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
                if modification < oldest_modification_time || (modification == oldest_modification_time && current_file_length > file_length) {
                    file_length = current_file_length;
                    oldest_modification_time = modification;
                    oldest_index = Some(current_index);
                }

                current_index += 1;

                if !model.iter_next(&iter) {
                    end = true;
                    break;
                }
            }
            if oldest_index == None {
                continue;
            }
            for (index, tree_iter) in tree_iter_array.iter().enumerate() {
                if index == oldest_index.unwrap() {
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
fn popover_one_newest(popover: &gtk::Popover, tree_view: &gtk::TreeView, column_color: i32, column_modification_as_secs: i32, column_file_name: i32, column_button_selection: u32) {
    let model = get_list_store(tree_view);

    if let Some(iter) = model.iter_first() {
        let mut end: bool = false;
        loop {
            let mut tree_iter_array: Vec<TreeIter> = Vec::new();
            let mut newest_index: Option<usize> = None;
            let mut current_index: usize = 0;
            let mut newest_modification_time: u64 = 0;

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
                if modification > newest_modification_time || (modification == newest_modification_time && current_file_length > file_length) {
                    file_length = current_file_length;
                    newest_modification_time = modification;
                    newest_index = Some(current_index);
                }

                current_index += 1;

                if !model.iter_next(&iter) {
                    end = true;
                    break;
                }
            }
            if newest_index == None {
                continue;
            }
            for (index, tree_iter) in tree_iter_array.iter().enumerate() {
                if index == newest_index.unwrap() {
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

fn popover_select_custom(popover: &gtk::Popover, gui_data: &GuiData, tree_view: &gtk::TreeView, column_color: Option<i32>, column_file_name: i32, column_path: i32, column_button_selection: u32) {
    popover.popdown();

    let wildcard: String;
    enum WildcardType {
        Path,
        Name,
        PathName,
    }
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

        for widgets in confirmation_dialog_delete.children() {
            // By default GtkBox is child of dialog, so we can easily add other things to it
            widgets.downcast::<gtk::Box>().unwrap().add(&grid);
        }

        confirmation_dialog_delete.show_all();

        let response_type = confirmation_dialog_delete.run();
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

        #[cfg(target_family = "windows")]
        let wildcard = wildcard.replace("/", "\\");
        #[cfg(target_family = "windows")]
        let wildcard = wildcard.as_str();

        let model = get_list_store(tree_view);

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
                        model.set_value(&iter, column_button_selection, &true.to_value());
                    }
                }
                WildcardType::Name => {
                    if Common::regex_check(wildcard, name) {
                        model.set_value(&iter, column_button_selection, &true.to_value());
                    }
                }
                WildcardType::PathName => {
                    if Common::regex_check(wildcard, format!("{}/{}", path, name)) {
                        model.set_value(&iter, column_button_selection, &true.to_value());
                    }
                }
            }

            if !model.iter_next(&iter) {
                break;
            }
        }
    }
}
fn popover_unselect_custom(popover: &gtk::Popover, gui_data: &GuiData, tree_view: &gtk::TreeView, column_color: Option<i32>, column_file_name: i32, column_path: i32, column_button_selection: u32) {
    popover.popdown();

    let wildcard: String;
    enum WildcardType {
        Path,
        Name,
        PathName,
    }
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

        #[cfg(target_family = "windows")]
        let wildcard = wildcard.replace("/", "\\");
        #[cfg(target_family = "windows")]
        let wildcard = wildcard.as_str();

        let model = get_list_store(tree_view);

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
                        model.set_value(&iter, column_button_selection, &false.to_value());
                    }
                }
                WildcardType::Name => {
                    if Common::regex_check(wildcard, name) {
                        model.set_value(&iter, column_button_selection, &false.to_value());
                    }
                }
                WildcardType::PathName => {
                    if Common::regex_check(wildcard, format!("{}/{}", path, name)) {
                        model.set_value(&iter, column_button_selection, &false.to_value());
                    }
                }
            }

            if !model.iter_next(&iter) {
                break;
            }
        }
    }
}

fn popover_all_except_biggest(popover: &gtk::Popover, tree_view: &gtk::TreeView, column_color: i32, column_size_as_bytes: i32, column_dimensions: i32, column_button_selection: u32) {
    let model = get_list_store(tree_view);

    if let Some(iter) = model.iter_first() {
        let mut end: bool = false;
        loop {
            let mut tree_iter_array: Vec<TreeIter> = Vec::new();
            let mut biggest_index: Option<usize> = None;
            let mut current_index: usize = 0;
            let mut biggest_size_as_bytes: u64 = 0;
            let mut biggest_number_of_pixels: u64 = 0;

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

                if number_of_pixels > biggest_number_of_pixels || (number_of_pixels == biggest_number_of_pixels && size_as_bytes > biggest_size_as_bytes) {
                    biggest_number_of_pixels = number_of_pixels;
                    biggest_size_as_bytes = size_as_bytes;
                    biggest_index = Some(current_index);
                }

                current_index += 1;

                if !model.iter_next(&iter) {
                    end = true;
                    break;
                }
            }
            if biggest_index == None {
                continue;
            }
            for (index, tree_iter) in tree_iter_array.iter().enumerate() {
                if index != biggest_index.unwrap() {
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
fn popover_all_except_smallest(popover: &gtk::Popover, tree_view: &gtk::TreeView, column_color: i32, column_size_as_bytes: i32, column_dimensions: i32, column_button_selection: u32) {
    let model = get_list_store(tree_view);

    if let Some(iter) = model.iter_first() {
        let mut end: bool = false;
        loop {
            let mut tree_iter_array: Vec<TreeIter> = Vec::new();
            let mut smallest_index: Option<usize> = None;
            let mut current_index: usize = 0;
            let mut smallest_size_as_bytes: u64 = u64::MAX;
            let mut smallest_number_of_pixels: u64 = u64::MAX;

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

                if number_of_pixels < smallest_number_of_pixels || (number_of_pixels == smallest_number_of_pixels && size_as_bytes < smallest_size_as_bytes) {
                    smallest_number_of_pixels = number_of_pixels;
                    smallest_size_as_bytes = size_as_bytes;
                    smallest_index = Some(current_index);
                }

                current_index += 1;

                if !model.iter_next(&iter) {
                    end = true;
                    break;
                }
            }
            if smallest_index == None {
                continue;
            }
            for (index, tree_iter) in tree_iter_array.iter().enumerate() {
                if index != smallest_index.unwrap() {
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

#[derive(Clone)]
pub struct PopoverObject {
    pub notebook_type: NotebookMainEnum,
    pub available_modes: Vec<String>,
    pub tree_view: gtk::TreeView,
    pub column_path: i32,
    pub column_name: i32,
    pub column_selection: u32, // TODo Change this to i32 after properly implement all things
    pub column_color: Option<i32>,
    pub column_dimensions: Option<i32>,
    pub column_size: Option<i32>,
    pub column_size_as_bytes: Option<i32>,
    pub column_modification_as_secs: Option<i32>,
}

pub fn find_name(notebook_type: &NotebookMainEnum, vec: &[PopoverObject]) -> Option<PopoverObject> {
    for e in vec {
        if e.notebook_type == *notebook_type {
            return Some(e.clone());
        }
    }
    None
}

pub fn connect_popovers(gui_data: &GuiData) {
    let popover_objects = vec![
        PopoverObject {
            notebook_type: NotebookMainEnum::Duplicate,
            available_modes: vec!["all", "reverse", "custom", "date"].iter().map(|e| e.to_string()).collect(),
            tree_view: gui_data.main_notebook.tree_view_duplicate_finder.clone(),
            column_path: ColumnsDuplicates::Path as i32,
            column_name: ColumnsDuplicates::Name as i32,
            column_selection: ColumnsDuplicates::ActiveSelectButton as u32,
            column_color: Some(ColumnsDuplicates::Color as i32),
            column_dimensions: None,
            column_size: None,
            column_size_as_bytes: None,
            column_modification_as_secs: Some(ColumnsDuplicates::ModificationAsSecs as i32),
        },
        PopoverObject {
            notebook_type: NotebookMainEnum::SameMusic,
            available_modes: vec!["all", "reverse", "custom", "date"].iter().map(|e| e.to_string()).collect(),
            tree_view: gui_data.main_notebook.tree_view_same_music_finder.clone(),
            column_path: ColumnsSameMusic::Path as i32,
            column_name: ColumnsSameMusic::Name as i32,
            column_selection: ColumnsSameMusic::ActiveSelectButton as u32,
            column_color: Some(ColumnsSameMusic::Color as i32),
            column_dimensions: None,
            column_size: None,
            column_size_as_bytes: Some(ColumnsSameMusic::SizeAsBytes as i32),
            column_modification_as_secs: Some(ColumnsSameMusic::ModificationAsSecs as i32),
        },
        PopoverObject {
            notebook_type: NotebookMainEnum::SimilarImages,
            available_modes: vec!["all", "reverse", "custom", "date"].iter().map(|e| e.to_string()).collect(),
            tree_view: gui_data.main_notebook.tree_view_similar_images_finder.clone(),
            column_path: ColumnsSimilarImages::Path as i32,
            column_name: ColumnsSimilarImages::Name as i32,
            column_selection: ColumnsSimilarImages::ActiveSelectButton as u32,
            column_color: Some(ColumnsSimilarImages::Color as i32),
            column_dimensions: Some(ColumnsSimilarImages::Dimensions as i32),
            column_size: Some(ColumnsSimilarImages::Size as i32),
            column_size_as_bytes: Some(ColumnsSimilarImages::SizeAsBytes as i32),
            column_modification_as_secs: Some(ColumnsSimilarImages::ModificationAsSecs as i32),
        },
        PopoverObject {
            notebook_type: NotebookMainEnum::EmptyDirectories,
            available_modes: vec!["all", "reverse", "custom"].iter().map(|e| e.to_string()).collect(),
            tree_view: gui_data.main_notebook.tree_view_empty_folder_finder.clone(),
            column_path: ColumnsEmptyFolders::Path as i32,
            column_name: ColumnsEmptyFolders::Name as i32,
            column_selection: ColumnsEmptyFolders::ActiveSelectButton as u32,
            column_color: None,
            column_dimensions: None,
            column_size: None,
            column_size_as_bytes: None,
            column_modification_as_secs: None,
        },
        PopoverObject {
            notebook_type: NotebookMainEnum::EmptyFiles,
            available_modes: vec!["all", "reverse", "custom"].iter().map(|e| e.to_string()).collect(),
            tree_view: gui_data.main_notebook.tree_view_empty_files_finder.clone(),
            column_path: ColumnsEmptyFiles::Path as i32,
            column_name: ColumnsEmptyFiles::Name as i32,
            column_selection: ColumnsEmptyFiles::ActiveSelectButton as u32,
            column_color: None,
            column_dimensions: None,
            column_size: None,
            column_size_as_bytes: None,
            column_modification_as_secs: None,
        },
        PopoverObject {
            notebook_type: NotebookMainEnum::Temporary,
            available_modes: vec!["all", "reverse", "custom"].iter().map(|e| e.to_string()).collect(),
            tree_view: gui_data.main_notebook.tree_view_temporary_files_finder.clone(),
            column_path: ColumnsTemporaryFiles::Path as i32,
            column_name: ColumnsTemporaryFiles::Name as i32,
            column_selection: ColumnsTemporaryFiles::ActiveSelectButton as u32,
            column_color: None,
            column_dimensions: None,
            column_size: None,
            column_size_as_bytes: None,
            column_modification_as_secs: None,
        },
        PopoverObject {
            notebook_type: NotebookMainEnum::BigFiles,
            available_modes: vec!["all", "reverse", "custom"].iter().map(|e| e.to_string()).collect(),
            tree_view: gui_data.main_notebook.tree_view_big_files_finder.clone(),
            column_path: ColumnsBigFiles::Path as i32,
            column_name: ColumnsBigFiles::Name as i32,
            column_selection: ColumnsBigFiles::ActiveSelectButton as u32,
            column_color: None,
            column_dimensions: None,
            column_size: None,
            column_size_as_bytes: None,
            column_modification_as_secs: None,
        },
        PopoverObject {
            notebook_type: NotebookMainEnum::Zeroed,
            available_modes: vec!["all", "reverse", "custom"].iter().map(|e| e.to_string()).collect(),
            tree_view: gui_data.main_notebook.tree_view_zeroed_files_finder.clone(),
            column_path: ColumnsZeroedFiles::Path as i32,
            column_name: ColumnsZeroedFiles::Name as i32,
            column_selection: ColumnsZeroedFiles::ActiveSelectButton as u32,
            column_color: None,
            column_dimensions: None,
            column_size: None,
            column_size_as_bytes: Some(ColumnsZeroedFiles::SizeAsBytes as i32),
            column_modification_as_secs: None,
        },
        PopoverObject {
            notebook_type: NotebookMainEnum::BrokenFiles,
            available_modes: vec!["all", "reverse", "custom"].iter().map(|e| e.to_string()).collect(),
            tree_view: gui_data.main_notebook.tree_view_broken_files.clone(),
            column_path: ColumnsBrokenFiles::Path as i32,
            column_name: ColumnsBrokenFiles::Name as i32,
            column_selection: ColumnsBrokenFiles::ActiveSelectButton as u32,
            column_color: None,
            column_dimensions: None,
            column_size: None,
            column_size_as_bytes: None,
            column_modification_as_secs: None,
        },
        PopoverObject {
            notebook_type: NotebookMainEnum::Symlinks,
            available_modes: vec!["all", "reverse", "custom"].iter().map(|e| e.to_string()).collect(),
            tree_view: gui_data.main_notebook.tree_view_invalid_symlinks.clone(),
            column_path: ColumnsInvalidSymlinks::Path as i32,
            column_name: ColumnsInvalidSymlinks::Name as i32,
            column_selection: ColumnsInvalidSymlinks::ActiveSelectButton as u32,
            column_color: None,
            column_dimensions: None,
            column_size: None,
            column_size_as_bytes: None,
            column_modification_as_secs: None,
        },
    ];

    let popover_select = gui_data.popovers.popover_select.clone();
    let buttons_popover_select_all = gui_data.popovers.buttons_popover_select_all.clone();
    let notebook_main = gui_data.main_notebook.notebook_main.clone();
    let vec_popover_objects = popover_objects.clone();
    buttons_popover_select_all.connect_clicked(move |_| {
        let object_popover = find_name(&to_notebook_main_enum(notebook_main.current_page().unwrap()), &vec_popover_objects).unwrap();
        popover_select_all(&popover_select, &object_popover.tree_view, object_popover.column_selection);
    });

    let popover_select = gui_data.popovers.popover_select.clone();
    let buttons_popover_unselect_all = gui_data.popovers.buttons_popover_unselect_all.clone();
    let notebook_main = gui_data.main_notebook.notebook_main.clone();
    let vec_popover_objects = popover_objects.clone();
    buttons_popover_unselect_all.connect_clicked(move |_| {
        let object_popover = find_name(&to_notebook_main_enum(notebook_main.current_page().unwrap()), &vec_popover_objects).unwrap();
        popover_unselect_all(&popover_select, &object_popover.tree_view, object_popover.column_selection);
    });

    let popover_select = gui_data.popovers.popover_select.clone();
    let buttons_popover_reverse = gui_data.popovers.buttons_popover_reverse.clone();
    let notebook_main = gui_data.main_notebook.notebook_main.clone();
    let vec_popover_objects = popover_objects.clone();
    buttons_popover_reverse.connect_clicked(move |_| {
        let object_popover = find_name(&to_notebook_main_enum(notebook_main.current_page().unwrap()), &vec_popover_objects).unwrap();
        popover_reverse(&popover_select, &object_popover.tree_view, object_popover.column_selection);
    });

    let popover_select = gui_data.popovers.popover_select.clone();
    let buttons_popover_select_all_except_oldest = gui_data.popovers.buttons_popover_select_all_except_oldest.clone();
    let notebook_main = gui_data.main_notebook.notebook_main.clone();
    let vec_popover_objects = popover_objects.clone();
    buttons_popover_select_all_except_oldest.connect_clicked(move |_| {
        let object_popover = find_name(&to_notebook_main_enum(notebook_main.current_page().unwrap()), &vec_popover_objects).unwrap();
        popover_all_except_oldest(
            &popover_select,
            &object_popover.tree_view,
            object_popover.column_color.unwrap(),
            object_popover.column_modification_as_secs.unwrap(),
            object_popover.column_name,
            object_popover.column_selection,
        );
    });

    let popover_select = gui_data.popovers.popover_select.clone();
    let buttons_popover_select_all_except_newest = gui_data.popovers.buttons_popover_select_all_except_newest.clone();
    let notebook_main = gui_data.main_notebook.notebook_main.clone();
    let vec_popover_objects = popover_objects.clone();
    buttons_popover_select_all_except_newest.connect_clicked(move |_| {
        let object_popover = find_name(&to_notebook_main_enum(notebook_main.current_page().unwrap()), &vec_popover_objects).unwrap();
        popover_all_except_newest(
            &popover_select,
            &object_popover.tree_view,
            object_popover.column_color.unwrap(),
            object_popover.column_modification_as_secs.unwrap(),
            object_popover.column_name,
            object_popover.column_selection,
        );
    });

    let popover_select = gui_data.popovers.popover_select.clone();
    let buttons_popover_select_one_oldest = gui_data.popovers.buttons_popover_select_one_oldest.clone();
    let notebook_main = gui_data.main_notebook.notebook_main.clone();
    let vec_popover_objects = popover_objects.clone();
    buttons_popover_select_one_oldest.connect_clicked(move |_| {
        let object_popover = find_name(&to_notebook_main_enum(notebook_main.current_page().unwrap()), &vec_popover_objects).unwrap();
        popover_one_oldest(
            &popover_select,
            &object_popover.tree_view,
            object_popover.column_color.unwrap(),
            object_popover.column_modification_as_secs.unwrap(),
            object_popover.column_name,
            object_popover.column_selection,
        );
    });

    let popover_select = gui_data.popovers.popover_select.clone();
    let buttons_popover_select_one_newest = gui_data.popovers.buttons_popover_select_one_newest.clone();
    let notebook_main = gui_data.main_notebook.notebook_main.clone();
    let vec_popover_objects = popover_objects.clone();
    buttons_popover_select_one_newest.connect_clicked(move |_| {
        let object_popover = find_name(&to_notebook_main_enum(notebook_main.current_page().unwrap()), &vec_popover_objects).unwrap();
        popover_one_newest(
            &popover_select,
            &object_popover.tree_view,
            object_popover.column_color.unwrap(),
            object_popover.column_modification_as_secs.unwrap(),
            object_popover.column_name,
            object_popover.column_selection,
        );
    });

    let popover_select = gui_data.popovers.popover_select.clone();
    let buttons_popover_select_custom = gui_data.popovers.buttons_popover_select_custom.clone();
    let notebook_main = gui_data.main_notebook.notebook_main.clone();
    let vec_popover_objects = popover_objects.clone();
    let gui_data_clone = gui_data.clone();
    buttons_popover_select_custom.connect_clicked(move |_| {
        let object_popover = find_name(&to_notebook_main_enum(notebook_main.current_page().unwrap()), &vec_popover_objects).unwrap();
        popover_select_custom(
            &popover_select,
            &gui_data_clone,
            &object_popover.tree_view,
            object_popover.column_color,
            object_popover.column_name,
            object_popover.column_path,
            object_popover.column_selection,
        );
    });

    let popover_select = gui_data.popovers.popover_select.clone();
    let buttons_popover_unselect_custom = gui_data.popovers.buttons_popover_unselect_custom.clone();
    let notebook_main = gui_data.main_notebook.notebook_main.clone();
    let vec_popover_objects = popover_objects.clone();
    let gui_data_clone = gui_data.clone();
    buttons_popover_unselect_custom.connect_clicked(move |_| {
        let object_popover = find_name(&to_notebook_main_enum(notebook_main.current_page().unwrap()), &vec_popover_objects).unwrap();
        popover_unselect_custom(
            &popover_select,
            &gui_data_clone,
            &object_popover.tree_view,
            object_popover.column_color,
            object_popover.column_name,
            object_popover.column_path,
            object_popover.column_selection,
        );
    });

    let popover_select = gui_data.popovers.popover_select.clone();
    let buttons_popover_select_all_images_except_biggest = gui_data.popovers.buttons_popover_select_all_images_except_biggest.clone();
    let notebook_main = gui_data.main_notebook.notebook_main.clone();
    let vec_popover_objects = popover_objects.clone();
    buttons_popover_select_all_images_except_biggest.connect_clicked(move |_| {
        let object_popover = find_name(&to_notebook_main_enum(notebook_main.current_page().unwrap()), &vec_popover_objects).unwrap();
        popover_all_except_biggest(
            &popover_select,
            &object_popover.tree_view,
            object_popover.column_color.unwrap(),
            object_popover.column_size_as_bytes.unwrap(),
            object_popover.column_dimensions.unwrap(),
            object_popover.column_selection,
        );
    });

    let popover_select = gui_data.popovers.popover_select.clone();
    let buttons_popover_select_all_images_except_smallest = gui_data.popovers.buttons_popover_select_all_images_except_smallest.clone();
    let notebook_main = gui_data.main_notebook.notebook_main.clone();
    let vec_popover_objects = popover_objects; //.clone();
    buttons_popover_select_all_images_except_smallest.connect_clicked(move |_| {
        let object_popover = find_name(&to_notebook_main_enum(notebook_main.current_page().unwrap()), &vec_popover_objects).unwrap();
        popover_all_except_smallest(
            &popover_select,
            &object_popover.tree_view,
            object_popover.column_color.unwrap(),
            object_popover.column_size_as_bytes.unwrap(),
            object_popover.column_dimensions.unwrap(),
            object_popover.column_selection,
        );
    });
}
