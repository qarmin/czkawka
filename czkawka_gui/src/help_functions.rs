use std::cmp::Ordering;
use std::collections::HashMap;
use std::path::PathBuf;

use gdk4::gdk_pixbuf::{InterpType, Pixbuf};
use glib::Error;
use gtk4::prelude::*;
use gtk4::{ListStore, TextView, TreeView, Widget};
use image::codecs::jpeg::JpegEncoder;
use image::{DynamicImage, EncodableLayout};
use once_cell::sync::OnceCell;

use czkawka_core::bad_extensions::BadExtensions;
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

use crate::flg;
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
    pub column_header: Option<i32>,
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
        column_header: Some(ColumnsDuplicates::IsHeader as i32),
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
        column_header: None,
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
        column_header: None,
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
        column_header: None,
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
        column_header: None,
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
        column_header: Some(ColumnsSimilarImages::IsHeader as i32),
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
        column_header: Some(ColumnsSimilarVideos::IsHeader as i32),
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
        column_header: Some(ColumnsSameMusic::IsHeader as i32),
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
        column_header: None,
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
        column_header: None,
        column_dimensions: None,
        column_size: None,
        column_size_as_bytes: None,
        column_modification_as_secs: None,
    },
    NotebookObject {
        notebook_type: NotebookMainEnum::BadExtensions,
        available_modes: [PopoverTypes::All, PopoverTypes::Reverse, PopoverTypes::Custom, PopoverTypes::None, PopoverTypes::None],
        column_activatable_button: None,
        column_path: ColumnsBadExtensions::Path as i32,
        column_name: ColumnsBadExtensions::Name as i32,
        column_selection: ColumnsBadExtensions::SelectionButton as i32,
        column_header: None,
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
    BadExtensions(BadExtensions),
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
    IsHeader,
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
    IsHeader,
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
    IsHeader,
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
    Year,
    Bitrate,
    BitrateAsNumber,
    Length,
    Genre,
    Modification,
    ModificationAsSecs,
    Color,
    IsHeader,
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

pub enum ColumnsBadExtensions {
    SelectionButton = 0,
    Name,
    Path,
    CurrentExtension,
    ValidExtensions,
    Modification,
    ModificationAsSecs,
}

pub const MAIN_ROW_COLOR: &str = "#222222";
pub const HEADER_ROW_COLOR: &str = "#111111";
pub const TEXT_COLOR: &str = "#ffffff";

pub fn get_string_from_list_store(tree_view: &TreeView, column_full_path: i32, column_selection: Option<i32>) -> Vec<String> {
    let list_store: ListStore = get_list_store(tree_view);

    let mut string_vector: Vec<String> = Vec::new();

    let tree_iter = match list_store.iter_first() {
        Some(t) => t,
        None => {
            return string_vector;
        }
    };
    match column_selection {
        Some(column_selection) => loop {
            if list_store.get::<bool>(&tree_iter, column_selection) {
                string_vector.push(list_store.get::<String>(&tree_iter, column_full_path));
            }
            if !list_store.iter_next(&tree_iter) {
                return string_vector;
            }
        },
        None => loop {
            string_vector.push(list_store.get::<String>(&tree_iter, column_full_path));
            if !list_store.iter_next(&tree_iter) {
                return string_vector;
            }
        },
    }
}

pub fn get_path_buf_from_vector_of_strings(vec_string: Vec<String>) -> Vec<PathBuf> {
    vec_string.iter().map(PathBuf::from).collect()
}

pub fn print_text_messages_to_text_view(text_messages: &Messages, text_view: &TextView) {
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

    text_view.buffer().set_text(messages.as_str());
}

pub fn reset_text_view(text_view: &TextView) {
    text_view.buffer().set_text("");
}

pub fn add_text_to_text_view(text_view: &TextView, string_to_append: &str) {
    let buffer = text_view.buffer();
    let current_text = buffer.text(&buffer.start_iter(), &buffer.end_iter(), true).to_string();
    if current_text.is_empty() {
        buffer.set_text(string_to_append);
    } else {
        buffer.set_text(format!("{}\n{}", current_text, string_to_append).as_str());
    }
}

pub fn set_buttons(hashmap: &mut HashMap<BottomButtonsEnum, bool>, buttons_array: &[Widget], button_names: &[BottomButtonsEnum]) {
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

pub fn get_list_store(tree_view: &TreeView) -> ListStore {
    tree_view.model().unwrap().downcast::<ListStore>().unwrap()
}

pub fn get_dialog_box_child(dialog: &gtk4::Dialog) -> gtk4::Box {
    dialog.child().unwrap().downcast::<gtk4::Box>().unwrap()
}

pub fn change_dimension_to_krotka(dimensions: String) -> (u64, u64) {
    #[allow(clippy::single_char_pattern)]
    let vec = dimensions.split::<&str>("x").collect::<Vec<_>>();
    assert_eq!(vec.len(), 2); // 400x400 - should only have two elements, if have more, then something is not good
    let number1 = vec[0].parse::<u64>().expect("Invalid data in image dimension in position 0");
    let number2 = vec[1].parse::<u64>().expect("Invalid data in image dimension in position 1");
    (number1, number2)
}

pub fn get_notebook_enum_from_tree_view(tree_view: &TreeView) -> NotebookMainEnum {
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
        "tree_view_bad_extensions" => NotebookMainEnum::BadExtensions,
        e => {
            panic!("{}", e)
        }
    }
}

pub fn get_notebook_upper_enum_from_tree_view(tree_view: &TreeView) -> NotebookUpperEnum {
    match (*tree_view).widget_name().to_string().as_str() {
        "tree_view_upper_included_directories" => NotebookUpperEnum::IncludedDirectories,
        "tree_view_upper_excluded_directories" => NotebookUpperEnum::ExcludedDirectories,
        e => {
            panic!("{}", e)
        }
    }
}

pub fn get_notebook_object_from_tree_view(tree_view: &TreeView) -> &NotebookObject {
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
pub fn clean_invalid_headers(model: &ListStore, column_header: i32, column_path: i32) {
    // Remove only child from header
    if let Some(first_iter) = model.iter_first() {
        let mut vec_tree_path_to_delete: Vec<gtk4::TreePath> = Vec::new();
        let mut current_iter = first_iter;
        if !model.get::<bool>(&current_iter, column_header) {
            panic!("First deleted element, should be a header"); // First element should be header
        };

        let mut next_iter;
        let mut next_next_iter;

        // Empty means default check type
        if model.get::<String>(&current_iter, column_path).is_empty() {
            'main: loop {
                if !model.get::<bool>(&current_iter, column_header) {
                    panic!("First deleted element, should be a header"); // First element should be header
                };

                next_iter = current_iter;
                if !model.iter_next(&next_iter) {
                    // There is only single header left (H1 -> END) -> (NOTHING)
                    vec_tree_path_to_delete.push(model.path(&current_iter));
                    break 'main;
                }

                if model.get::<bool>(&next_iter, column_header) {
                    // There are two headers each others(we remove just first) -> (H1 -> H2) -> (H2)
                    vec_tree_path_to_delete.push(model.path(&current_iter));
                    current_iter = next_iter;
                    continue 'main;
                }

                next_next_iter = next_iter;
                if !model.iter_next(&next_next_iter) {
                    // There is only one child of header left, so we remove it with header (H1 -> C1 -> END) -> (NOTHING)
                    vec_tree_path_to_delete.push(model.path(&current_iter));
                    vec_tree_path_to_delete.push(model.path(&next_iter));
                    break 'main;
                }

                if model.get::<bool>(&next_next_iter, column_header) {
                    // One child between two headers, we can remove them  (H1 -> C1 -> H2) -> (H2)
                    vec_tree_path_to_delete.push(model.path(&current_iter));
                    vec_tree_path_to_delete.push(model.path(&next_iter));
                    current_iter = next_next_iter;
                    continue 'main;
                }

                loop {
                    // (H1 -> C1 -> C2 -> Cn -> END) -> (NO CHANGE, BECAUSE IS GOOD)
                    if !model.iter_next(&next_next_iter) {
                        break 'main;
                    }
                    // Move to next header
                    if model.get::<bool>(&next_next_iter, column_header) {
                        current_iter = next_next_iter;
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
                if !model.get::<bool>(&current_iter, column_header) {
                    panic!("First deleted element, should be a header"); // First element should be header
                };

                next_iter = current_iter;
                if !model.iter_next(&next_iter) {
                    // There is only single header left (H1 -> END) -> (NOTHING)
                    vec_tree_path_to_delete.push(model.path(&current_iter));
                    break 'reference;
                }

                if model.get::<bool>(&next_iter, column_header) {
                    // There are two headers each others(we remove just first) -> (H1 -> H2) -> (H2)
                    vec_tree_path_to_delete.push(model.path(&current_iter));
                    current_iter = next_iter;
                    continue 'reference;
                }

                next_next_iter = next_iter;
                if !model.iter_next(&next_next_iter) {
                    // There is only one child of header left, so we remove it with header (H1 -> C1 -> END) -> (NOTHING)
                    break 'reference;
                }

                if model.get::<bool>(&next_next_iter, column_header) {
                    // One child between two headers, we can remove them  (H1 -> C1 -> H2) -> (H2)
                    current_iter = next_next_iter;
                    continue 'reference;
                }

                loop {
                    // (H1 -> C1 -> C2 -> Cn -> END) -> (NO CHANGE, BECAUSE IS GOOD)
                    if !model.iter_next(&next_next_iter) {
                        break 'reference;
                    }
                    // Move to next header
                    if model.get::<bool>(&next_next_iter, column_header) {
                        current_iter = next_next_iter;
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

pub fn check_how_much_elements_is_selected(tree_view: &TreeView, column_header: Option<i32>, column_selection: i32) -> (u64, u64) {
    let mut number_of_selected_items: u64 = 0;
    let mut number_of_selected_groups: u64 = 0;

    let model = get_list_store(tree_view);

    let mut is_item_currently_selected_in_group: bool = false;

    // First iter
    if let Some(iter) = model.iter_first() {
        if let Some(column_header) = column_header {
            assert!(model.get::<bool>(&iter, column_header)); // First element should be header

            loop {
                if !model.iter_next(&iter) {
                    break;
                }

                if model.get::<bool>(&iter, column_header) {
                    is_item_currently_selected_in_group = false;
                } else {
                    if model.get::<bool>(&iter, column_selection) {
                        number_of_selected_items += 1;

                        if !is_item_currently_selected_in_group {
                            number_of_selected_groups += 1;
                        }
                        is_item_currently_selected_in_group = true;
                    }
                }
            }
        } else {
            if model.get::<bool>(&iter, column_selection) {
                number_of_selected_items += 1;
            }
            loop {
                if !model.iter_next(&iter) {
                    break;
                }

                if model.get::<bool>(&iter, column_selection) {
                    number_of_selected_items += 1;
                }
            }
        }
    }

    (number_of_selected_items, number_of_selected_groups)
}

/// Counts how much headers/groups is in treeview
pub fn count_number_of_groups(tree_view: &TreeView, column_header: i32) -> u32 {
    let mut number_of_selected_groups = 0;

    let model = get_list_store(tree_view);

    if let Some(iter) = model.iter_first() {
        assert!(model.get::<bool>(&iter, column_header)); // First element should be header
        number_of_selected_groups += 1;

        loop {
            if !model.iter_next(&iter) {
                break;
            }

            if model.get::<bool>(&iter, column_header) {
                number_of_selected_groups += 1;
            }
        }
    }
    number_of_selected_groups
}

pub fn resize_pixbuf_dimension(pixbuf: Pixbuf, requested_size: (i32, i32), interp_type: InterpType) -> Option<Pixbuf> {
    let current_ratio = pixbuf.width() as f32 / pixbuf.height() as f32;
    let mut new_size;
    match current_ratio.partial_cmp(&(requested_size.0 as f32 / requested_size.1 as f32)).unwrap() {
        Ordering::Greater => {
            new_size = (requested_size.0, (pixbuf.height() * requested_size.0) / pixbuf.width());
            new_size = (std::cmp::max(new_size.0, 1), std::cmp::max(new_size.1, 1));
        }
        Ordering::Less => {
            new_size = ((pixbuf.width() * requested_size.1) / pixbuf.height(), requested_size.1);
            new_size = (std::cmp::max(new_size.0, 1), std::cmp::max(new_size.1, 1));
        }
        Ordering::Equal => {
            new_size = (requested_size.0, requested_size.1);
            new_size = (std::cmp::max(new_size.0, 1), std::cmp::max(new_size.1, 1));
        }
    }
    pixbuf.scale_simple(new_size.0, new_size.1, interp_type)
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

pub fn get_custom_label_from_widget<P: IsA<Widget>>(item: &P) -> gtk4::Label {
    let mut widgets_to_check = vec![item.clone().upcast::<Widget>()];

    while let Some(widget) = widgets_to_check.pop() {
        if let Ok(label) = widget.clone().downcast::<gtk4::Label>() {
            return label;
        } else {
            widgets_to_check.extend(get_all_children(&widget));
        }
    }
    panic!("Button doesn't have proper custom label child");
}

pub fn get_custom_image_from_widget<P: IsA<Widget>>(item: &P) -> gtk4::Image {
    let mut widgets_to_check = vec![item.clone().upcast::<Widget>()];

    while let Some(widget) = widgets_to_check.pop() {
        if let Ok(image) = widget.clone().downcast::<gtk4::Image>() {
            return image;
        } else {
            widgets_to_check.extend(get_all_children(&widget));
        }
    }
    panic!("Button doesn't have proper custom label child");
}

#[allow(dead_code)]
pub fn debug_print_widget<P: IsA<Widget>>(item: &P) {
    let mut widgets_to_check = vec![(0, 0, item.clone().upcast::<Widget>())];

    let mut next_free_number = 1;
    println!("{}, {}, {:?} ", widgets_to_check[0].0, widgets_to_check[0].1, widgets_to_check[0].2);

    while let Some((current_number, parent_number, widget)) = widgets_to_check.pop() {
        for widget in get_all_children(&widget) {
            widgets_to_check.push((next_free_number, current_number, widget));
            next_free_number += 1;
        }
        println!("{}, {}, {:?} ", current_number, parent_number, widget);
    }
}

pub fn get_all_boxes_from_widget<P: IsA<Widget>>(item: &P) -> Vec<gtk4::Box> {
    let mut widgets_to_check = vec![item.clone().upcast::<Widget>()];
    let mut boxes = Vec::new();

    while let Some(widget) = widgets_to_check.pop() {
        widgets_to_check.extend(get_all_children(&widget));
        if let Ok(bbox) = widget.clone().downcast::<gtk4::Box>() {
            boxes.push(bbox);
        }
    }
    boxes
}

pub fn get_all_children<P: IsA<Widget>>(wid: &P) -> Vec<Widget> {
    let mut vector = vec![];
    if let Some(mut child) = wid.first_child() {
        vector.push(child.clone());
        loop {
            child = match child.next_sibling() {
                Some(t) => t,
                None => break,
            };
            vector.push(child.clone());
        }
    }

    vector
}

const SIZE_OF_ICON: i32 = 18;
const TYPE_OF_INTERPOLATION: InterpType = InterpType::Tiles;

pub fn set_icon_of_button<P: IsA<Widget>>(button: &P, data: &'static [u8]) {
    let image = get_custom_image_from_widget(&button.clone());
    let pixbuf = Pixbuf::from_read(std::io::BufReader::new(data)).unwrap();
    let pixbuf = pixbuf.scale_simple(SIZE_OF_ICON, SIZE_OF_ICON, TYPE_OF_INTERPOLATION).unwrap();
    image.set_from_pixbuf(Some(&pixbuf));
}

static mut IMAGE_PREVIEW_ARRAY: OnceCell<Vec<u8>> = OnceCell::new();
pub fn get_pixbuf_from_dynamic_image(dynamic_image: &DynamicImage) -> Result<Pixbuf, Error> {
    let mut output = Vec::new();
    JpegEncoder::new(&mut output).encode_image(dynamic_image).unwrap();
    let arra;
    unsafe {
        IMAGE_PREVIEW_ARRAY.take();
        IMAGE_PREVIEW_ARRAY.set(output).unwrap();
        arra = IMAGE_PREVIEW_ARRAY.get().unwrap().as_bytes();
    }
    Pixbuf::from_read(arra)
}
