use czkawka_core::big_file::BigFile;
use czkawka_core::broken_files::BrokenFiles;
use czkawka_core::common_messages::Messages;
use czkawka_core::duplicate::DuplicateFinder;
use czkawka_core::empty_files::EmptyFiles;
use czkawka_core::empty_folder::EmptyFolder;
use czkawka_core::invalid_symlinks;
use czkawka_core::invalid_symlinks::InvalidSymlinks;
use czkawka_core::same_music::SameMusic;
use czkawka_core::similar_images::SimilarImages;
use czkawka_core::temporary::Temporary;
use czkawka_core::zeroed::ZeroedFiles;
use gtk::prelude::*;
use gtk::{ListStore, TextView};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub enum Message {
    Duplicates(DuplicateFinder),
    EmptyFolders(EmptyFolder),
    EmptyFiles(EmptyFiles),
    BigFiles(BigFile),
    Temporary(Temporary),
    SimilarImages(SimilarImages),
    ZeroedFiles(ZeroedFiles),
    SameMusic(SameMusic),
    InvalidSymlinks(InvalidSymlinks),
    BrokenFiles(BrokenFiles),
}

#[derive(Debug)]
pub enum ColumnsDuplicates {
    // Columns for duplicate treeview
    ActivatableSelectButton = 0,
    ActiveSelectButton,
    Name,
    Path,
    Modification,
    ModificationAsSecs,
    Color,
    TextColor,
}

pub enum ColumnsEmptyFolders {
    // Columns for empty folder treeview
    ActiveSelectButton = 0,
    Name,
    Path,
    Modification,
}
pub enum ColumnsDirectory {
    // Columns for Included and Excluded Directories in upper Notebook
    Path = 0,
}
pub enum ColumnsBigFiles {
    ActiveSelectButton = 0,
    Size,
    Name,
    Path,
    Modification,
}
pub enum ColumnsEmptyFiles {
    ActiveSelectButton = 0,
    Name,
    Path,
    Modification,
}
pub enum ColumnsTemporaryFiles {
    ActiveSelectButton = 0,
    Name,
    Path,
    Modification,
}
pub enum ColumnsSimilarImages {
    ActivatableSelectButton = 0,
    ActiveSelectButton,
    Similarity,
    Size,
    SizeAsBytes,
    Dimensions,
    Name,
    Path,
    Modification,
    ModificationAsSecs,
    Color,
    TextColor,
}
pub enum ColumnsZeroedFiles {
    ActiveSelectButton = 0,
    Size,
    SizeAsBytes,
    Name,
    Path,
    Modification,
}
pub enum ColumnsSameMusic {
    ActivatableSelectButton = 0,
    ActiveSelectButton,
    Size,
    SizeAsBytes,
    Name,
    Path,
    Title,
    Artist,
    AlbumTitle,
    AlbumArtist,
    Year,
    Modification,
    ModificationAsSecs,
    Color,
    TextColor,
}
pub enum ColumnsInvalidSymlinks {
    ActiveSelectButton = 0,
    Name,
    Path,
    DestinationPath,
    TypeOfError,
    Modification,
}

pub enum ColumnsBrokenFiles {
    ActiveSelectButton = 0,
    Name,
    Path,
    ErrorType,
    Modification,
}

pub const TEXT_COLOR: &str = "#ffffff";
pub const MAIN_ROW_COLOR: &str = "#343434";
pub const HEADER_ROW_COLOR: &str = "#272727";
//pub const MAIN_ROW_COLOR: &str = "#f4f434"; // TEST
//pub const HEADER_ROW_COLOR: &str = "#010101"; // TEST

pub fn get_string_from_list_store(tree_view: &gtk::TreeView) -> Vec<String> {
    let list_store: gtk::ListStore = get_list_store(tree_view);

    let mut string_vector: Vec<String> = Vec::new();

    let tree_iter = match list_store.iter_first() {
        Some(t) => t,
        None => {
            return string_vector;
        }
    };
    loop {
        string_vector.push(list_store.value(&tree_iter, 0).get::<String>().unwrap());
        if !list_store.iter_next(&tree_iter) {
            return string_vector;
        }
    }
}
pub fn get_path_buf_from_vector_of_strings(vec_string: Vec<String>) -> Vec<PathBuf> {
    vec_string.iter().map(PathBuf::from).collect()
}

pub fn split_path(path: &Path) -> (String, String) {
    match (path.parent(), path.file_name()) {
        (Some(dir), Some(file)) => (dir.display().to_string(), file.to_string_lossy().into_owned()),
        (Some(dir), None) => (dir.display().to_string(), String::new()),
        (None, _) => (String::new(), String::new()),
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

    text_view.buffer().unwrap().set_text(messages.as_str());
}

pub fn reset_text_view(text_view: &TextView) {
    text_view.buffer().unwrap().set_text("");
}

pub fn add_text_to_text_view(text_view: &TextView, string_to_append: &str) {
    let buffer = text_view.buffer().unwrap();
    let current_text = match buffer.text(&buffer.start_iter(), &buffer.end_iter(), true) {
        Some(t) => t.to_string(),
        None => "".to_string(),
    };
    buffer.set_text(format!("{}\n{}", current_text, string_to_append).as_str());
}

pub fn select_function_duplicates(_tree_selection: &gtk::TreeSelection, tree_model: &gtk::TreeModel, tree_path: &gtk::TreePath, _is_path_currently_selected: bool) -> bool {
    // let name = tree_model.value(&tree_model.iter(tree_path).unwrap(),ColumnsDuplicates::Name as i32).get::<String>().unwrap();
    // let path = tree_model.value(&tree_model.iter(tree_path).unwrap(), ColumnsDuplicates::Path as i32).get::<String>().unwrap();
    // let modification = tree_model.value(&tree_model.iter(tree_path).unwrap(),ColumnsDuplicates::Modification as i32).get::<String>().unwrap();
    let color = tree_model.value(&tree_model.iter(tree_path).unwrap(), ColumnsDuplicates::Color as i32).get::<String>().unwrap();

    if color == HEADER_ROW_COLOR {
        return false;
    }

    true
}
pub fn select_function_same_music(_tree_selection: &gtk::TreeSelection, tree_model: &gtk::TreeModel, tree_path: &gtk::TreePath, _is_path_currently_selected: bool) -> bool {
    let color = tree_model.value(&tree_model.iter(tree_path).unwrap(), ColumnsSameMusic::Color as i32).get::<String>().unwrap();

    if color == HEADER_ROW_COLOR {
        return false;
    }

    true
}
pub fn select_function_similar_images(_tree_selection: &gtk::TreeSelection, tree_model: &gtk::TreeModel, tree_path: &gtk::TreePath, _is_path_currently_selected: bool) -> bool {
    let color = tree_model.value(&tree_model.iter(tree_path).unwrap(), ColumnsSimilarImages::Color as i32).get::<String>().unwrap();

    if color == HEADER_ROW_COLOR {
        return false;
    }

    true
}

pub fn set_buttons(hashmap: &mut HashMap<String, bool>, buttons_array: &[gtk::Button], button_names: &[String]) {
    for (index, button) in buttons_array.iter().enumerate() {
        if *hashmap.get_mut(button_names[index].as_str()).unwrap() {
            button.show();
        } else {
            button.hide();
        }
    }
}
pub fn hide_all_buttons(buttons_array: &[gtk::Button]) {
    for button in buttons_array {
        button.hide();
    }
}

pub fn get_text_from_invalid_symlink_cause(error: &invalid_symlinks::ErrorType) -> &str {
    match error {
        invalid_symlinks::ErrorType::InfiniteRecursion => "Infinite recursion",
        invalid_symlinks::ErrorType::NonExistentFile => "Non existent destination file",
    }
}

pub fn get_list_store(tree_view: &gtk::TreeView) -> ListStore {
    tree_view.model().unwrap().downcast::<gtk::ListStore>().unwrap()
}
pub fn get_dialog_box_child(dialog: &gtk::Dialog) -> gtk::Box {
    dialog.children()[0].clone().downcast::<gtk::Box>().unwrap()
}

pub fn change_dimension_to_krotka(dimensions: String) -> (u64, u64) {
    #[allow(clippy::single_char_pattern)]
    let vec = dimensions.split::<&str>("x").collect::<Vec<_>>();
    assert_eq!(vec.len(), 2); // 400x400 - should only have two elements, if have more, then something is not good
    let number1 = vec[0].parse::<u64>().expect("Invalid data in image dimension in position 0");
    let number2 = vec[1].parse::<u64>().expect("Invalid data in image dimension in position 1");
    (number1, number2)
}
