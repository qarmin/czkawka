use directories_next::ProjectDirs;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use gtk::prelude::*;
use gtk::{ListStore, TextView, TreeView, Widget};
use image::imageops::FilterType;
use image::DynamicImage;
use image::GenericImageView;

use crate::flg;
use czkawka_core::big_file::BigFile;
use czkawka_core::broken_files::BrokenFiles;
use czkawka_core::common_dir_traversal;
use czkawka_core::common_messages::Messages;
use czkawka_core::duplicate::DuplicateFinder;
use czkawka_core::empty_files::EmptyFiles;
use czkawka_core::empty_folder::EmptyFolder;
use czkawka_core::invalid_symlinks::InvalidSymlinks;
use czkawka_core::same_music::SameMusic;
use czkawka_core::similar_images::SimilarImages;
use czkawka_core::similar_videos::SimilarVideos;
use czkawka_core::temporary::Temporary;

use crate::notebook_enums::{NotebookMainEnum, NotebookUpperEnum, NUMBER_OF_NOTEBOOK_MAIN_TABS};

#[cfg(not(target_family = "windows"))]
pub const CHARACTER: char = '/';
#[cfg(target_family = "windows")]
pub const CHARACTER: char = '\\';

pub const KEY_DELETE: u32 = 119;
pub const KEY_ENTER: u32 = 36;
pub const KEY_SPACE: u32 = 65;

// pub const KEY_DOWN: u32 = 116;
// pub const KEY_UP: u32 = 111;
// pub const KEY_PG_DOWN: u32 = 117;
// pub const KEY_PG_UP: u32 = 112;
// pub const KEY_HOME: u32 = 115;
// pub const KEY_END: u32 = 110;

#[derive(Eq, PartialEq)]
pub enum PopoverTypes {
    All,
    Size,
    Reverse,
    Custom,
    Date,
    None,
}

#[derive(Eq, PartialEq, Copy, Clone, Hash)]
pub enum BottomButtonsEnum {
    Search,
    Select,
    Delete,
    Save,
    Symlink,
    Hardlink,
    Move,
    Compare,
}

pub struct NotebookObject {
    pub notebook_type: NotebookMainEnum,
    pub available_modes: [PopoverTypes; 5],
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
        available_modes: [PopoverTypes::All, PopoverTypes::Reverse, PopoverTypes::Custom, PopoverTypes::Date, PopoverTypes::None],
        column_activatable_button: Some(ColumnsDuplicates::ActivatableSelectButton as i32),
        column_path: ColumnsDuplicates::Path as i32,
        column_name: ColumnsDuplicates::Name as i32,
        column_selection: ColumnsDuplicates::SelectionButton as i32,
        column_color: Some(ColumnsDuplicates::Color as i32),
        column_dimensions: None,
        column_size: None,          // Do not add, useless in hash and size mode
        column_size_as_bytes: None, // Do not add, useless in hash and size mode
        column_modification_as_secs: Some(ColumnsDuplicates::ModificationAsSecs as i32),
    },
    NotebookObject {
        notebook_type: NotebookMainEnum::EmptyDirectories,
        available_modes: [PopoverTypes::All, PopoverTypes::Reverse, PopoverTypes::Custom, PopoverTypes::None, PopoverTypes::None],
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
        available_modes: [PopoverTypes::All, PopoverTypes::Reverse, PopoverTypes::Custom, PopoverTypes::None, PopoverTypes::None],
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
        available_modes: [PopoverTypes::All, PopoverTypes::Reverse, PopoverTypes::Custom, PopoverTypes::None, PopoverTypes::None],
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
        available_modes: [PopoverTypes::All, PopoverTypes::Reverse, PopoverTypes::Custom, PopoverTypes::None, PopoverTypes::None],
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
        available_modes: [PopoverTypes::All, PopoverTypes::Reverse, PopoverTypes::Custom, PopoverTypes::Date, PopoverTypes::Size],
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
        available_modes: [PopoverTypes::All, PopoverTypes::Reverse, PopoverTypes::Custom, PopoverTypes::Date, PopoverTypes::Size],
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
        available_modes: [PopoverTypes::All, PopoverTypes::Reverse, PopoverTypes::Custom, PopoverTypes::Date, PopoverTypes::Size],
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
        available_modes: [PopoverTypes::All, PopoverTypes::Reverse, PopoverTypes::Custom, PopoverTypes::None, PopoverTypes::None],
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
        available_modes: [PopoverTypes::All, PopoverTypes::Reverse, PopoverTypes::Custom, PopoverTypes::None, PopoverTypes::None],
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
    Size,
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
    ModificationAsSecs,
}

pub enum ColumnsIncludedDirectory {
    // Columns for Included Directories in upper Notebook
    Path = 0,
    ReferenceButton,
}
pub enum ColumnsExcludedDirectory {
    // Columns for Excluded Directories in upper Notebook
    Path = 0,
}

pub enum ColumnsBigFiles {
    SelectionButton = 0,
    Size,
    Name,
    Path,
    Modification,
    SizeAsBytes,
    ModificationAsSecs,
}

pub enum ColumnsEmptyFiles {
    SelectionButton = 0,
    Name,
    Path,
    Modification,
    ModificationAsSecs,
}

pub enum ColumnsTemporaryFiles {
    SelectionButton = 0,
    Name,
    Path,
    Modification,
    ModificationAsSecs,
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
    ModificationAsSecs,
}

pub enum ColumnsBrokenFiles {
    SelectionButton = 0,
    Name,
    Path,
    ErrorType,
    Modification,
    ModificationAsSecs,
}

pub const TEXT_COLOR: &str = "#ffffff";
pub const MAIN_ROW_COLOR: &str = "#343434";
pub const HEADER_ROW_COLOR: &str = "#272727";
//pub const MAIN_ROW_COLOR: &str = "#f4f434"; // TEST
//pub const HEADER_ROW_COLOR: &str = "#010101"; // TEST

pub fn get_string_from_list_store(tree_view: &gtk::TreeView, column_full_path: i32, column_selection: Option<i32>) -> Vec<String> {
    let list_store: gtk::ListStore = get_list_store(tree_view);

    let mut string_vector: Vec<String> = Vec::new();

    let tree_iter = match list_store.iter_first() {
        Some(t) => t,
        None => {
            return string_vector;
        }
    };
    match column_selection {
        Some(column_selection) => loop {
            if list_store.value(&tree_iter, column_selection).get::<bool>().unwrap() {
                string_vector.push(list_store.value(&tree_iter, column_full_path).get::<String>().unwrap());
            }
            if !list_store.iter_next(&tree_iter) {
                return string_vector;
            }
        },
        None => loop {
            string_vector.push(list_store.value(&tree_iter, column_full_path).get::<String>().unwrap());
            if !list_store.iter_next(&tree_iter) {
                return string_vector;
            }
        },
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
        messages += format!("############### {}({}) ###############\n", flg!("text_view_messages"), text_messages.messages.len()).as_str();
    }
    for text in &text_messages.messages {
        messages += text.as_str();
        messages += "\n";
    }
    // if !text_messages.messages.is_empty() {
    //     messages += "\n";
    // }
    if !text_messages.warnings.is_empty() {
        messages += format!("############### {}({}) ###############\n", flg!("text_view_warnings"), text_messages.warnings.len()).as_str();
    }
    for text in &text_messages.warnings {
        messages += text.as_str();
        messages += "\n";
    }
    // if !text_messages.warnings.is_empty() {
    //     messages += "\n";
    // }
    if !text_messages.errors.is_empty() {
        messages += format!("############### {}({}) ###############\n", flg!("text_view_errors"), text_messages.errors.len()).as_str();
    }
    for text in &text_messages.errors {
        messages += text.as_str();
        messages += "\n";
    }
    // if !text_messages.errors.is_empty() {
    //     messages += "\n";
    // }

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
    if current_text.is_empty() {
        buffer.set_text(string_to_append);
    } else {
        buffer.set_text(format!("{}\n{}", current_text, string_to_append).as_str());
    }
}

pub fn set_buttons(hashmap: &mut HashMap<BottomButtonsEnum, bool>, buttons_array: &[gtk::Widget], button_names: &[BottomButtonsEnum]) {
    for (index, button) in buttons_array.iter().enumerate() {
        if *hashmap.get_mut(&button_names[index]).unwrap() {
            button.show();
        } else {
            button.hide();
        }
    }
}

pub fn hide_all_buttons(buttons_array: &[Widget]) {
    for button in buttons_array {
        button.hide();
    }
}

pub fn get_text_from_invalid_symlink_cause(error: &common_dir_traversal::ErrorType) -> String {
    match error {
        common_dir_traversal::ErrorType::InfiniteRecursion => flg!("invalid_symlink_infinite_recursion"),
        common_dir_traversal::ErrorType::NonExistentFile => flg!("invalid_symlink_non_existent_destination"),
    }
}

pub fn get_list_store(tree_view: &gtk::TreeView) -> ListStore {
    tree_view.model().unwrap().downcast::<gtk::ListStore>().unwrap()
}

pub fn get_dialog_box_child(dialog: &gtk::Dialog) -> gtk::Box {
    dialog.child().unwrap().downcast::<gtk::Box>().unwrap()
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
        e => {
            panic!("{}", e)
        }
    }
}

pub fn get_notebook_upper_enum_from_tree_view(tree_view: &gtk::TreeView) -> NotebookUpperEnum {
    match (*tree_view).widget_name().to_string().as_str() {
        "tree_view_upper_included_directories" => NotebookUpperEnum::IncludedDirectories,
        "tree_view_upper_excluded_directories" => NotebookUpperEnum::ExcludedDirectories,
        e => {
            panic!("{}", e)
        }
    }
}

pub fn get_notebook_object_from_tree_view(tree_view: &gtk::TreeView) -> &NotebookObject {
    let nb_enum = get_notebook_enum_from_tree_view(tree_view);
    &NOTEBOOKS_INFOS[nb_enum as usize]
}

pub fn get_full_name_from_path_name(path: &str, name: &str) -> String {
    let mut string = String::with_capacity(path.len() + name.len() + 1);
    string.push_str(path);
    string.push(CHARACTER);
    string.push_str(name);
    string
}

// After e.g. deleting files, header may become orphan or have one child, so should be deleted in this case
pub fn clean_invalid_headers(model: &gtk::ListStore, column_color: i32, column_path: i32) {
    // Remove only child from header
    if let Some(first_iter) = model.iter_first() {
        let mut vec_tree_path_to_delete: Vec<gtk::TreePath> = Vec::new();
        let mut current_iter = first_iter;
        if model.value(&current_iter, column_color).get::<String>().unwrap() != HEADER_ROW_COLOR {
            panic!("First deleted element, should be a header"); // First element should be header
        };

        let mut next_iter;
        let mut next_next_iter;

        // Empty means default check type
        if model.value(&current_iter, column_path).get::<String>().unwrap().is_empty() {
            'main: loop {
                if model.value(&current_iter, column_color).get::<String>().unwrap() != HEADER_ROW_COLOR {
                    panic!("First deleted element, should be a header"); // First element should be header
                };

                next_iter = current_iter.clone();
                if !model.iter_next(&next_iter) {
                    // There is only single header left (H1 -> END) -> (NOTHING)
                    vec_tree_path_to_delete.push(model.path(&current_iter).unwrap());
                    break 'main;
                }

                if model.value(&next_iter, column_color).get::<String>().unwrap() == HEADER_ROW_COLOR {
                    // There are two headers each others(we remove just first) -> (H1 -> H2) -> (H2)
                    vec_tree_path_to_delete.push(model.path(&current_iter).unwrap());
                    current_iter = next_iter.clone();
                    continue 'main;
                }

                next_next_iter = next_iter.clone();
                if !model.iter_next(&next_next_iter) {
                    // There is only one child of header left, so we remove it with header (H1 -> C1 -> END) -> (NOTHING)
                    vec_tree_path_to_delete.push(model.path(&current_iter).unwrap());
                    vec_tree_path_to_delete.push(model.path(&next_iter).unwrap());
                    break 'main;
                }

                if model.value(&next_next_iter, column_color).get::<String>().unwrap() == HEADER_ROW_COLOR {
                    // One child between two headers, we can remove them  (H1 -> C1 -> H2) -> (H2)
                    vec_tree_path_to_delete.push(model.path(&current_iter).unwrap());
                    vec_tree_path_to_delete.push(model.path(&next_iter).unwrap());
                    current_iter = next_next_iter.clone();
                    continue 'main;
                }

                loop {
                    // (H1 -> C1 -> C2 -> Cn -> END) -> (NO CHANGE, BECAUSE IS GOOD)
                    if !model.iter_next(&next_next_iter) {
                        break 'main;
                    }
                    // Move to next header
                    if model.value(&next_next_iter, column_color).get::<String>().unwrap() == HEADER_ROW_COLOR {
                        current_iter = next_next_iter.clone();
                        continue 'main;
                    }
                }
            }
            for tree_path in vec_tree_path_to_delete.iter().rev() {
                model.remove(&model.iter(tree_path).unwrap());
            }
        }
        // Non empty means that header points at reference folder
        else {
            'reference: loop {
                if model.value(&current_iter, column_color).get::<String>().unwrap() != HEADER_ROW_COLOR {
                    panic!("First deleted element, should be a header"); // First element should be header
                };

                next_iter = current_iter.clone();
                if !model.iter_next(&next_iter) {
                    // There is only single header left (H1 -> END) -> (NOTHING)
                    vec_tree_path_to_delete.push(model.path(&current_iter).unwrap());
                    break 'reference;
                }

                if model.value(&next_iter, column_color).get::<String>().unwrap() == HEADER_ROW_COLOR {
                    // There are two headers each others(we remove just first) -> (H1 -> H2) -> (H2)
                    vec_tree_path_to_delete.push(model.path(&current_iter).unwrap());
                    current_iter = next_iter.clone();
                    continue 'reference;
                }

                next_next_iter = next_iter.clone();
                if !model.iter_next(&next_next_iter) {
                    // There is only one child of header left, so we remove it with header (H1 -> C1 -> END) -> (NOTHING)
                    break 'reference;
                }

                if model.value(&next_next_iter, column_color).get::<String>().unwrap() == HEADER_ROW_COLOR {
                    // One child between two headers, we can remove them  (H1 -> C1 -> H2) -> (H2)
                    current_iter = next_next_iter.clone();
                    continue 'reference;
                }

                loop {
                    // (H1 -> C1 -> C2 -> Cn -> END) -> (NO CHANGE, BECAUSE IS GOOD)
                    if !model.iter_next(&next_next_iter) {
                        break 'reference;
                    }
                    // Move to next header
                    if model.value(&next_next_iter, column_color).get::<String>().unwrap() == HEADER_ROW_COLOR {
                        current_iter = next_next_iter.clone();
                        continue 'reference;
                    }
                }
            }
            for tree_path in vec_tree_path_to_delete.iter().rev() {
                model.remove(&model.iter(tree_path).unwrap());
            }
        }
    }

    // Last step, remove orphan header if exists
    if let Some(iter) = model.iter_first() {
        if !model.iter_next(&iter) {
            model.clear();
        }
    }
}
pub fn check_how_much_elements_is_selected(tree_view: &TreeView, column_color: Option<i32>, column_selection: i32) -> (u64, u64) {
    let mut number_of_selected_items: u64 = 0;
    let mut number_of_selected_groups: u64 = 0;

    let model = get_list_store(tree_view);

    let mut is_item_currently_selected_in_group: bool = false;

    // First iter
    if let Some(iter) = model.iter_first() {
        if let Some(column_color) = column_color {
            assert_eq!(model.value(&iter, column_color).get::<String>().unwrap(), HEADER_ROW_COLOR); // First element should be header

            loop {
                if !model.iter_next(&iter) {
                    break;
                }

                if model.value(&iter, column_color).get::<String>().unwrap() == HEADER_ROW_COLOR {
                    is_item_currently_selected_in_group = false;
                } else {
                    if model.value(&iter, column_selection).get::<bool>().unwrap() {
                        number_of_selected_items += 1;

                        if !is_item_currently_selected_in_group {
                            number_of_selected_groups += 1;
                        }
                        is_item_currently_selected_in_group = true;
                    }
                }
            }
        } else {
            loop {
                if !model.iter_next(&iter) {
                    break;
                }

                if model.value(&iter, column_selection).get::<bool>().unwrap() {
                    number_of_selected_items += 1;
                }
            }
        }
    }

    (number_of_selected_items, number_of_selected_groups)
}

/// Counts how much headers/groups is in treeview
pub fn count_number_of_groups(tree_view: &TreeView, column_color: i32) -> u32 {
    let mut number_of_selected_groups = 0;

    let model = get_list_store(tree_view);

    if let Some(iter) = model.iter_first() {
        assert_eq!(model.value(&iter, column_color).get::<String>().unwrap(), HEADER_ROW_COLOR); // First element should be header
        number_of_selected_groups += 1;

        loop {
            if !model.iter_next(&iter) {
                break;
            }

            if model.value(&iter, column_color).get::<String>().unwrap() == HEADER_ROW_COLOR {
                number_of_selected_groups += 1;
            }
        }
    }
    number_of_selected_groups
}

pub fn resize_dynamic_image_dimension(img: DynamicImage, requested_size: (u32, u32), filter_type: &FilterType) -> DynamicImage {
    let current_ratio = img.width() as f32 / img.height() as f32;
    let mut new_size;
    match current_ratio.partial_cmp(&(requested_size.0 as f32 / requested_size.1 as f32)).unwrap() {
        Ordering::Greater => {
            new_size = (requested_size.0, (img.height() * requested_size.0) / img.width());
            new_size = (std::cmp::max(new_size.0, 1), std::cmp::max(new_size.1, 1));
        }
        Ordering::Less => {
            new_size = ((img.width() * requested_size.1) / img.height(), requested_size.1);
            new_size = (std::cmp::max(new_size.0, 1), std::cmp::max(new_size.1, 1));
        }
        Ordering::Equal => {
            new_size = (requested_size.0, requested_size.1);
            new_size = (std::cmp::max(new_size.0, 1), std::cmp::max(new_size.1, 1));
        }
    }
    img.resize(new_size.0, new_size.1, *filter_type)
}

pub fn get_image_path_temporary(file_name: &str, number: u32, extension: &str) -> PathBuf {
    let path_buf;
    if let Some(proj_dirs) = ProjectDirs::from("pl", "Qarmin", "Czkawka") {
        path_buf = PathBuf::from(proj_dirs.cache_dir());
    } else {
        path_buf = PathBuf::new().join("/var");
    }
    path_buf.join(format!("{}{}.{}", file_name, number, extension))
}

pub fn get_max_file_name(file_name: &str, max_length: usize) -> String {
    assert!(max_length > 10); // Maybe in future will be supported lower values
    if file_name.len() > max_length {
        let difference = file_name.len() - max_length;

        let mut string = "".to_string();
        string += &file_name[0..10];
        string += " ... ";
        string += &file_name[10 + difference..];
        string
    } else {
        file_name.to_string()
    }
}

pub fn get_custom_label_from_button_with_image(button: &gtk::Bin) -> gtk::Label {
    let internal_box = button.child().unwrap().downcast::<gtk::Box>().unwrap();
    for child in internal_box.children() {
        if let Ok(t) = child.downcast::<gtk::Label>() {
            return t;
        }
    }
    panic!("Button doesn't have proper custom label child");
}

// GTK 4
// pub fn get_custom_label_from_button_with_image<P: IsA<gtk4::Widget>>(button: &P) -> gtk4::Label {
//     let internal_box = button.first_child().unwrap().downcast::<gtk4::Box>().unwrap();
//     for child in get_all_children(&internal_box) {
//         if let Ok(t) = child.downcast::<gtk4::Label>() {
//             return t;
//         }
//     }
//     panic!("Button doesn't have proper custom label child");
// }
// TODO needs GTK 4.6 to be able to set as child of menu button a box
// pub fn get_custom_label_from_menubutton_with_image<P: IsA<gtk4::Widget>>(button: &P) -> gtk4::Label {
//     println!("{:?}", get_all_children(button));
//     for c1 in get_all_children(button) {
//         if let Ok(internal_box) = c1.downcast::<gtk4::Box>() {
//             for child in get_all_children(&internal_box) {
//                 if let Ok(t) = child.downcast::<gtk4::Label>() {
//                     return t;
//                 }
//             }
//         }
//     }
//     panic!("Menu Button doesn't have proper custom label child");
// }

// GTK 4
// pub fn get_all_children<P: IsA<gtk::Widget>>(wid: &P) -> Vec<gtk::Widget> {
//     let mut vector = vec![];
//     if let Some(mut child) = wid.first_child() {
//         vector.push(child.clone());
//         loop {
//             child = match child.next_sibling() {
//                 Some(t) => t,
//                 None => break,
//             };
//             vector.push(child.clone());
//         }
//     }
//
//     return vector;
// }
