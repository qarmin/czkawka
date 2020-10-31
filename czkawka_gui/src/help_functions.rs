use czkawka_core::common_messages::Messages;
use czkawka_core::similar_files::Similarity;
use gtk::prelude::*;
use std::collections::HashMap;

pub enum ColumnsDuplicates {
    // Columns for duplicate treeview
    Name = 0,
    Path,
    Modification,
    ModificationAsSecs,
    Color,
    TextColor,
}

pub enum ColumnsEmptyFolders {
    // Columns for empty folder treeview
    Name = 0,
    Path,
    Modification,
}
pub enum ColumnsDirectory {
    // Columns for Included and Excluded Directories in upper Notebook
    Path = 0,
}
pub enum ColumnsBigFiles {
    Size = 0,
    Name,
    Path,
    Modification,
}
pub enum ColumnsEmptyFiles {
    Name = 0,
    Path,
    Modification,
}
pub enum ColumnsTemporaryFiles {
    Name = 0,
    Path,
    Modification,
}
pub enum ColumnsSimilarImages {
    Similarity = 0,
    Size,
    Dimensions,
    Name,
    Path,
    Modification,
    Color,
    TextColor,
}
pub enum ColumnsZeroedFiles {
    Size = 0,
    Name,
    Path,
    Modification,
}

pub const TEXT_COLOR: &str = "#ffffff";
pub const MAIN_ROW_COLOR: &str = "#343434";
pub const HEADER_ROW_COLOR: &str = "#272727";
//pub const MAIN_ROW_COLOR: &str = "#f4f434"; // TEST
//pub const HEADER_ROW_COLOR: &str = "#010101"; // TEST

pub fn get_string_from_list_store(scrolled_window: &gtk::ScrolledWindow) -> String {
    let tree_view: gtk::TreeView = scrolled_window.get_children().get(0).unwrap().clone().downcast::<gtk::TreeView>().unwrap();
    let list_store: gtk::ListStore = tree_view.get_model().unwrap().downcast::<gtk::ListStore>().unwrap();
    let mut first: bool = true;

    let mut return_string: String = "".to_string();
    let tree_iter = match list_store.get_iter_first() {
        Some(t) => t,
        None => return return_string,
    };
    loop {
        if !first {
            return_string += ",";
        } else {
            first = false;
        }
        return_string += list_store.get_value(&tree_iter, 0).get::<String>().unwrap().unwrap().as_str();
        if !list_store.iter_next(&tree_iter) {
            return return_string;
        }
    }
}

pub fn print_text_messages_to_text_view(text_messages: &Messages, text_view: &gtk::TextView) {
    let mut messages: String = String::from("");
    if !text_messages.messages.is_empty() {
        messages += "############### MESSAGES ###############\n";
    }
    for text in &text_messages.messages {
        messages += text.as_str();
        messages += "\n";
    }
    if !text_messages.messages.is_empty() {
        messages += "\n";
    }
    if !text_messages.warnings.is_empty() {
        messages += "############### WARNINGS ###############\n";
    }
    for text in &text_messages.warnings {
        messages += text.as_str();
        messages += "\n";
    }
    if !text_messages.warnings.is_empty() {
        messages += "\n";
    }
    if !text_messages.errors.is_empty() {
        messages += "############### ERRORS ###############\n";
    }
    for text in &text_messages.errors {
        messages += text.as_str();
        messages += "\n";
    }
    if !text_messages.errors.is_empty() {
        messages += "\n";
    }

    text_view.get_buffer().unwrap().set_text(messages.as_str());
}

pub fn select_function_duplicates(_tree_selection: &gtk::TreeSelection, tree_model: &gtk::TreeModel, tree_path: &gtk::TreePath, _is_path_currently_selected: bool) -> bool {
    // let name = tree_model.get_value(&tree_model.get_iter(tree_path).unwrap(),ColumnsDuplicates::Name as i32).get::<String>().unwrap().unwrap();
    // let path = tree_model.get_value(&tree_model.get_iter(tree_path).unwrap(), ColumnsDuplicates::Path as i32).get::<String>().unwrap().unwrap();
    // let modification = tree_model.get_value(&tree_model.get_iter(tree_path).unwrap(),ColumnsDuplicates::Modification as i32).get::<String>().unwrap().unwrap();
    let color = tree_model.get_value(&tree_model.get_iter(tree_path).unwrap(), ColumnsDuplicates::Color as i32).get::<String>().unwrap().unwrap();

    if color == HEADER_ROW_COLOR {
        return false;
    }

    true
}

pub fn set_buttons(hashmap: &mut HashMap<String, bool>, buttons_array: &[gtk::Button], button_names: &[&str]) {
    for (index, button) in buttons_array.iter().enumerate() {
        if *hashmap.get_mut(button_names[index]).unwrap() {
            button.show();
        } else {
            button.hide();
        }
    }
}
// pub fn hide_all_buttons(buttons_array: &[gtk::Button]) {
//     for button in buttons_array {
//         button.hide();
//     }
// }

pub fn hide_all_buttons_except(except_name: &str, buttons_array: &[gtk::Button], button_names: &[&str]) {
    for (index, button) in buttons_array.iter().enumerate() {
        if except_name == button_names[index] {
            button.show();
        } else {
            button.hide();
        }
    }
}

pub fn get_text_from_similarity(similarity: &Similarity) -> &str {
    match similarity {
        Similarity::None => "Original",
        Similarity::Small => "Small",
        Similarity::Medium => "Medium",
        Similarity::High => "High",
        Similarity::VeryHigh => "Very High",
    }
}
