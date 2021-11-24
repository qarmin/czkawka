use crate::notebook_enums::{to_notebook_main_enum, NotebookMainEnum, NUMBER_OF_NOTEBOOK_MAIN_TABS};
use crate::GuiData;
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
use czkawka_core::similar_videos::SimilarVideos;
use czkawka_core::temporary::Temporary;
use gtk::prelude::*;
use gtk::{ListStore, TextView};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[derive(Eq, PartialEq)]
pub enum PopoverTypes {
    All,
    ImageSize,
    Reverse,
    Custom,
    Date,
    None,
}

pub struct NotebookObject {
    pub notebook_type: NotebookMainEnum,
    pub available_modes: [PopoverTypes; 4],
    pub column_activatable_button: Option<i32>,
    pub column_path: i32,
    pub column_name: i32,
    pub column_selection: i32,
    pub column_color: Option<i32>,
    pub column_dimensions: Option<i32>,
    pub column_size: Option<i32>,
    pub column_size_as_bytes: Option<i32>,
    pub column_modification_as_secs: Option<i32>,
}

pub static NOTEBOOKS_INFOS: [NotebookObject; NUMBER_OF_NOTEBOOK_MAIN_TABS] = [
    NotebookObject {
        notebook_type: NotebookMainEnum::Duplicate,
        available_modes: [PopoverTypes::All, PopoverTypes::Reverse, PopoverTypes::Custom, PopoverTypes::Date],
        column_activatable_button: Some(ColumnsDuplicates::ActivatableSelectButton as i32),
        column_path: ColumnsDuplicates::Path as i32,
        column_name: ColumnsDuplicates::Name as i32,
        column_selection: ColumnsDuplicates::SelectionButton as i32,
        column_color: Some(ColumnsDuplicates::Color as i32),
        column_dimensions: None,
        column_size: None,
        column_size_as_bytes: None,
        column_modification_as_secs: Some(ColumnsDuplicates::ModificationAsSecs as i32),
    },
    NotebookObject {
        notebook_type: NotebookMainEnum::EmptyDirectories,
        available_modes: [PopoverTypes::All, PopoverTypes::Reverse, PopoverTypes::Custom, PopoverTypes::None],
        column_activatable_button: None,
        column_path: ColumnsEmptyFolders::Path as i32,
        column_name: ColumnsEmptyFolders::Name as i32,
        column_selection: ColumnsEmptyFolders::SelectionButton as i32,
        column_color: None,
        column_dimensions: None,
        column_size: None,
        column_size_as_bytes: None,
        column_modification_as_secs: None,
    },
    NotebookObject {
        notebook_type: NotebookMainEnum::BigFiles,
        available_modes: [PopoverTypes::All, PopoverTypes::Reverse, PopoverTypes::Custom, PopoverTypes::None],
        column_activatable_button: None,
        column_path: ColumnsBigFiles::Path as i32,
        column_name: ColumnsBigFiles::Name as i32,
        column_selection: ColumnsBigFiles::SelectionButton as i32,
        column_color: None,
        column_dimensions: None,
        column_size: None,
        column_size_as_bytes: None,
        column_modification_as_secs: None,
    },
    NotebookObject {
        notebook_type: NotebookMainEnum::EmptyFiles,
        available_modes: [PopoverTypes::All, PopoverTypes::Reverse, PopoverTypes::Custom, PopoverTypes::None],
        column_activatable_button: None,
        column_path: ColumnsEmptyFiles::Path as i32,
        column_name: ColumnsEmptyFiles::Name as i32,
        column_selection: ColumnsEmptyFiles::SelectionButton as i32,
        column_color: None,
        column_dimensions: None,
        column_size: None,
        column_size_as_bytes: None,
        column_modification_as_secs: None,
    },
    NotebookObject {
        notebook_type: NotebookMainEnum::Temporary,
        available_modes: [PopoverTypes::All, PopoverTypes::Reverse, PopoverTypes::Custom, PopoverTypes::None],
        column_activatable_button: None,
        column_path: ColumnsTemporaryFiles::Path as i32,
        column_name: ColumnsTemporaryFiles::Name as i32,
        column_selection: ColumnsTemporaryFiles::SelectionButton as i32,
        column_color: None,
        column_dimensions: None,
        column_size: None,
        column_size_as_bytes: None,
        column_modification_as_secs: None,
    },
    NotebookObject {
        notebook_type: NotebookMainEnum::SimilarImages,
        available_modes: [PopoverTypes::All, PopoverTypes::Reverse, PopoverTypes::Custom, PopoverTypes::Date],
        column_activatable_button: Some(ColumnsSimilarImages::ActivatableSelectButton as i32),
        column_path: ColumnsSimilarImages::Path as i32,
        column_name: ColumnsSimilarImages::Name as i32,
        column_selection: ColumnsSimilarImages::SelectionButton as i32,
        column_color: Some(ColumnsSimilarImages::Color as i32),
        column_dimensions: Some(ColumnsSimilarImages::Dimensions as i32),
        column_size: Some(ColumnsSimilarImages::Size as i32),
        column_size_as_bytes: Some(ColumnsSimilarImages::SizeAsBytes as i32),
        column_modification_as_secs: Some(ColumnsSimilarImages::ModificationAsSecs as i32),
    },
    NotebookObject {
        notebook_type: NotebookMainEnum::SimilarVideos,
        available_modes: [PopoverTypes::All, PopoverTypes::Reverse, PopoverTypes::Custom, PopoverTypes::Date],
        column_activatable_button: Some(ColumnsSimilarVideos::ActivatableSelectButton as i32),
        column_path: ColumnsSimilarVideos::Path as i32,
        column_name: ColumnsSimilarVideos::Name as i32,
        column_selection: ColumnsSimilarVideos::SelectionButton as i32,
        column_color: Some(ColumnsSimilarVideos::Color as i32),
        column_dimensions: None,
        column_size: Some(ColumnsSimilarVideos::Size as i32),
        column_size_as_bytes: Some(ColumnsSimilarVideos::SizeAsBytes as i32),
        column_modification_as_secs: Some(ColumnsSimilarVideos::ModificationAsSecs as i32),
    },
    NotebookObject {
        notebook_type: NotebookMainEnum::SameMusic,
        available_modes: [PopoverTypes::All, PopoverTypes::Reverse, PopoverTypes::Custom, PopoverTypes::Date],
        column_activatable_button: Some(ColumnsSameMusic::ActivatableSelectButton as i32),
        column_path: ColumnsSameMusic::Path as i32,
        column_name: ColumnsSameMusic::Name as i32,
        column_selection: ColumnsSameMusic::SelectionButton as i32,
        column_color: Some(ColumnsSameMusic::Color as i32),
        column_dimensions: None,
        column_size: None,
        column_size_as_bytes: Some(ColumnsSameMusic::SizeAsBytes as i32),
        column_modification_as_secs: Some(ColumnsSameMusic::ModificationAsSecs as i32),
    },
    NotebookObject {
        notebook_type: NotebookMainEnum::Symlinks,
        available_modes: [PopoverTypes::All, PopoverTypes::Reverse, PopoverTypes::Custom, PopoverTypes::None],
        column_activatable_button: None,
        column_path: ColumnsInvalidSymlinks::Path as i32,
        column_name: ColumnsInvalidSymlinks::Name as i32,
        column_selection: ColumnsInvalidSymlinks::SelectionButton as i32,
        column_color: None,
        column_dimensions: None,
        column_size: None,
        column_size_as_bytes: None,
        column_modification_as_secs: None,
    },
    NotebookObject {
        notebook_type: NotebookMainEnum::BrokenFiles,
        available_modes: [PopoverTypes::All, PopoverTypes::Reverse, PopoverTypes::Custom, PopoverTypes::None],
        column_activatable_button: None,
        column_path: ColumnsBrokenFiles::Path as i32,
        column_name: ColumnsBrokenFiles::Name as i32,
        column_selection: ColumnsBrokenFiles::SelectionButton as i32,
        column_color: None,
        column_dimensions: None,
        column_size: None,
        column_size_as_bytes: None,
        column_modification_as_secs: None,
    },
];

pub enum Message {
    Duplicates(DuplicateFinder),
    EmptyFolders(EmptyFolder),
    EmptyFiles(EmptyFiles),
    BigFiles(BigFile),
    Temporary(Temporary),
    SimilarImages(SimilarImages),
    SimilarVideos(SimilarVideos),
    SameMusic(SameMusic),
    InvalidSymlinks(InvalidSymlinks),
    BrokenFiles(BrokenFiles),
}

pub enum ColumnsDuplicates {
    // Columns for duplicate treeview
    ActivatableSelectButton = 0,
    SelectionButton,
    Name,
    Path,
    Modification,
    ModificationAsSecs,
    Color,
    TextColor,
}

pub enum ColumnsEmptyFolders {
    // Columns for empty folder treeview
    SelectionButton = 0,
    Name,
    Path,
    Modification,
}
pub enum ColumnsDirectory {
    // Columns for Included and Excluded Directories in upper Notebook
    Path = 0,
}
pub enum ColumnsBigFiles {
    SelectionButton = 0,
    Size,
    Name,
    Path,
    Modification,
}
pub enum ColumnsEmptyFiles {
    SelectionButton = 0,
    Name,
    Path,
    Modification,
}
pub enum ColumnsTemporaryFiles {
    SelectionButton = 0,
    Name,
    Path,
    Modification,
}
pub enum ColumnsSimilarImages {
    ActivatableSelectButton = 0,
    SelectionButton,
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

pub enum ColumnsSimilarVideos {
    ActivatableSelectButton = 0,
    SelectionButton,
    Size,
    SizeAsBytes,
    Name,
    Path,
    Modification,
    ModificationAsSecs,
    Color,
    TextColor,
}
pub enum ColumnsSameMusic {
    ActivatableSelectButton = 0,
    SelectionButton,
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
    SelectionButton = 0,
    Name,
    Path,
    DestinationPath,
    TypeOfError,
    Modification,
}

pub enum ColumnsBrokenFiles {
    SelectionButton = 0,
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
pub fn select_function_similar_videos(_tree_selection: &gtk::TreeSelection, tree_model: &gtk::TreeModel, tree_path: &gtk::TreePath, _is_path_currently_selected: bool) -> bool {
    let color = tree_model.value(&tree_model.iter(tree_path).unwrap(), ColumnsSimilarVideos::Color as i32).get::<String>().unwrap();

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

pub fn get_notebook_enum_from_tree_view(tree_view: &gtk::TreeView) -> NotebookMainEnum {
    match (*tree_view).widget_name().to_string().as_str() {
        "tree_view_duplicate_finder" => NotebookMainEnum::Duplicate,
        "tree_view_empty_folder_finder" => NotebookMainEnum::EmptyDirectories,
        "tree_view_empty_files_finder" => NotebookMainEnum::EmptyFiles,
        "tree_view_temporary_files_finder" => NotebookMainEnum::Temporary,
        "tree_view_big_files_finder" => NotebookMainEnum::BigFiles,
        "tree_view_similar_images_finder" => NotebookMainEnum::SimilarImages,
        "tree_view_similar_videos_finder" => NotebookMainEnum::SimilarVideos,
        "tree_view_same_music_finder" => NotebookMainEnum::SameMusic,
        "tree_view_invalid_symlinks" => NotebookMainEnum::Symlinks,
        "tree_view_broken_files" => NotebookMainEnum::BrokenFiles,
        _ => panic!(),
    }
}
pub fn get_notebook_object_from_tree_view(tree_view: &gtk::TreeView) -> &NotebookObject {
    let nb_enum = get_notebook_enum_from_tree_view(tree_view);
    &NOTEBOOKS_INFOS[nb_enum as usize]
}
pub fn validate_notebook_data(gui_data: &GuiData) {
    // Test treeviews names, each treeview should have set name same as variable name
    let tree_view_arr: [&gtk::TreeView; NUMBER_OF_NOTEBOOK_MAIN_TABS] = [
        &gui_data.main_notebook.tree_view_duplicate_finder,
        &gui_data.main_notebook.tree_view_similar_videos_finder,
        &gui_data.main_notebook.tree_view_temporary_files_finder,
        &gui_data.main_notebook.tree_view_big_files_finder,
        &gui_data.main_notebook.tree_view_empty_files_finder,
        &gui_data.main_notebook.tree_view_broken_files,
        &gui_data.main_notebook.tree_view_empty_folder_finder,
        &gui_data.main_notebook.tree_view_same_music_finder,
        &gui_data.main_notebook.tree_view_similar_images_finder,
        &gui_data.main_notebook.tree_view_invalid_symlinks,
    ];
    for (_i, item) in tree_view_arr.iter().enumerate() {
        // println!("Checking {} element", i);

        get_notebook_enum_from_tree_view(item);
    }

    // This test main info about notebooks
    // Should have same order as notebook enum types
    for (i, item) in NOTEBOOKS_INFOS.iter().enumerate() {
        let en = to_notebook_main_enum(i as u32);
        assert_eq!(item.notebook_type, en);
    }

    // Tests if data returned from array get_notebook_enum_from_tree_view are in right
    for (i, item) in gui_data.main_notebook.get_main_tree_views().iter().enumerate() {
        let nb_en = get_notebook_enum_from_tree_view(item);
        assert_eq!(to_notebook_main_enum(i as u32), nb_en);
    }
}
