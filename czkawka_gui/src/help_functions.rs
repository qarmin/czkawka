use gdk4::gdk_pixbuf::{InterpType, Pixbuf};
use glib::Error;
use gtk4::prelude::*;
use gtk4::{ListStore, Scale, ScrollType, TextView, TreeView, Widget};
use image::codecs::jpeg::JpegEncoder;
use image::{DynamicImage, EncodableLayout};
use once_cell::sync::OnceCell;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::io::BufReader;
use std::path::{PathBuf, MAIN_SEPARATOR};
use std::rc::Rc;

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
use crate::notebook_enums::{NotebookMainEnum, NotebookUpperEnum};
use crate::notebook_info::{NotebookObject, NOTEBOOKS_INFO};

pub const KEY_DELETE: u32 = 119;
pub const KEY_ENTER: u32 = 36;
pub const KEY_SPACE: u32 = 65;

pub type SharedState<T> = Rc<RefCell<Option<T>>>;

// pub const KEY_DOWN: u32 = 116;
// pub const KEY_UP: u32 = 111;
// pub const KEY_PG_DOWN: u32 = 117;
// pub const KEY_PG_UP: u32 = 112;
// pub const KEY_HOME: u32 = 115;
// pub const KEY_END: u32 = 110;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum PopoverTypes {
    All,
    Size,
    Reverse,
    Custom,
    Date,
}

#[derive(Eq, PartialEq, Copy, Clone, Hash, Debug)]
pub enum BottomButtonsEnum {
    Search,
    Select,
    Delete,
    Save,
    Symlink,
    Hardlink,
    Move,
    Compare,
    Sort,
}

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

#[derive(Clone, Copy)]
pub enum ColumnsDuplicates {
    // Columns for duplicate treeview
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

#[derive(Clone, Copy)]
pub enum ColumnsEmptyFolders {
    // Columns for empty folder treeview
    SelectionButton = 0,
    Name,
    Path,
    Modification,
    ModificationAsSecs,
}

#[derive(Clone, Copy)]
pub enum ColumnsIncludedDirectory {
    // Columns for Included Directories in upper Notebook
    Path = 0,
    ReferenceButton,
}

#[derive(Clone, Copy)]
pub enum ColumnsExcludedDirectory {
    // Columns for Excluded Directories in upper Notebook
    Path = 0,
}

#[derive(Clone, Copy)]
pub enum ColumnsBigFiles {
    SelectionButton = 0,
    Size,
    Name,
    Path,
    Modification,
    SizeAsBytes,
    ModificationAsSecs,
}

#[derive(Clone, Copy)]
pub enum ColumnsEmptyFiles {
    SelectionButton = 0,
    Name,
    Path,
    Modification,
    ModificationAsSecs,
}

#[derive(Clone, Copy)]
pub enum ColumnsTemporaryFiles {
    SelectionButton = 0,
    Name,
    Path,
    Modification,
    ModificationAsSecs,
}

#[derive(Clone, Copy)]
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

#[derive(Clone, Copy)]
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

#[derive(Clone, Copy)]
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

#[derive(Clone, Copy)]
pub enum ColumnsInvalidSymlinks {
    SelectionButton = 0,
    Name,
    Path,
    DestinationPath,
    TypeOfError,
    Modification,
    ModificationAsSecs,
}

#[derive(Clone, Copy)]
pub enum ColumnsBrokenFiles {
    SelectionButton = 0,
    Name,
    Path,
    ErrorType,
    Modification,
    ModificationAsSecs,
}

#[derive(Clone, Copy)]
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

    let Some(tree_iter) = list_store.iter_first() else {
        return string_vector;
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

pub fn get_path_buf_from_vector_of_strings(vec_string: &[String]) -> Vec<PathBuf> {
    vec_string.iter().map(PathBuf::from).collect()
}

pub fn print_text_messages_to_text_view(text_messages: &Messages, text_view: &TextView) {
    let mut messages: String = String::new();
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
        buffer.set_text(format!("{current_text}\n{string_to_append}").as_str());
    }
}

pub fn set_buttons(hashmap: &mut HashMap<BottomButtonsEnum, bool>, buttons_array: &[Widget], button_names: &[BottomButtonsEnum]) {
    for (index, button) in buttons_array.iter().enumerate() {
        if *hashmap.get_mut(&button_names[index]).expect("Invalid button name") {
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

pub fn get_text_from_invalid_symlink_cause(error: common_dir_traversal::ErrorType) -> String {
    match error {
        common_dir_traversal::ErrorType::InfiniteRecursion => flg!("invalid_symlink_infinite_recursion"),
        common_dir_traversal::ErrorType::NonExistentFile => flg!("invalid_symlink_non_existent_destination"),
    }
}

pub fn get_list_store(tree_view: &TreeView) -> ListStore {
    tree_view.model().expect("Tree view have no model").downcast::<ListStore>().expect("Model is not ListStore")
}

pub fn get_dialog_box_child(dialog: &gtk4::Dialog) -> gtk4::Box {
    dialog.child().expect("Dialog have no chile").downcast::<gtk4::Box>().expect("Dialog child is not Box")
}

pub fn change_dimension_to_krotka(dimensions: &str) -> (u64, u64) {
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

pub fn get_tree_view_name_from_notebook_enum(notebook_enum: NotebookMainEnum) -> &'static str {
    match notebook_enum {
        NotebookMainEnum::Duplicate => "tree_view_duplicate_finder",
        NotebookMainEnum::EmptyDirectories => "tree_view_empty_folder_finder",
        NotebookMainEnum::EmptyFiles => "tree_view_empty_files_finder",
        NotebookMainEnum::Temporary => "tree_view_temporary_files_finder",
        NotebookMainEnum::BigFiles => "tree_view_big_files_finder",
        NotebookMainEnum::SimilarImages => "tree_view_similar_images_finder",
        NotebookMainEnum::SimilarVideos => "tree_view_similar_videos_finder",
        NotebookMainEnum::SameMusic => "tree_view_same_music_finder",
        NotebookMainEnum::Symlinks => "tree_view_invalid_symlinks",
        NotebookMainEnum::BrokenFiles => "tree_view_broken_files",
        NotebookMainEnum::BadExtensions => "tree_view_bad_extensions",
    }
}

pub fn get_notebook_upper_enum_from_tree_view(tree_view: &TreeView) -> NotebookUpperEnum {
    match (*tree_view).widget_name().to_string().as_str() {
        "tree_view_upper_included_directories" => NotebookUpperEnum::IncludedDirectories,
        "tree_view_upper_excluded_directories" => NotebookUpperEnum::ExcludedDirectories,
        e => panic!("{}", e),
    }
}

pub fn get_tree_view_name_from_notebook_upper_enum(notebook_upper_enum: NotebookUpperEnum) -> &'static str {
    match notebook_upper_enum {
        NotebookUpperEnum::IncludedDirectories => "tree_view_upper_included_directories",
        NotebookUpperEnum::ExcludedDirectories => "tree_view_upper_excluded_directories",
        _ => panic!(),
    }
}

pub fn get_notebook_object_from_tree_view(tree_view: &TreeView) -> &NotebookObject {
    let nb_enum = get_notebook_enum_from_tree_view(tree_view);
    &NOTEBOOKS_INFO[nb_enum as usize]
}

pub fn get_full_name_from_path_name(path: &str, name: &str) -> String {
    let mut string = String::with_capacity(path.len() + name.len() + 1);
    string.push_str(path);
    string.push(MAIN_SEPARATOR);
    string.push_str(name);
    string
}

// After e.g. deleting files, header may become orphan or have one child, so should be deleted in this case
pub fn clean_invalid_headers(model: &ListStore, column_header: i32, column_path: i32) {
    // Remove only child from header
    if let Some(first_iter) = model.iter_first() {
        let mut vec_tree_path_to_delete: Vec<gtk4::TreePath> = Vec::new();
        let mut current_iter = first_iter;
        // First element should be header
        assert!(model.get::<bool>(&current_iter, column_header), "First deleted element, should be a header");

        let mut next_iter;
        let mut next_next_iter;

        // Empty means default check type
        if model.get::<String>(&current_iter, column_path).is_empty() {
            'main: loop {
                // First element should be header
                assert!(model.get::<bool>(&current_iter, column_header), "First deleted element, should be a header");

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
                model.remove(&model.iter(tree_path).expect("Using invalid tree_path"));
            }
        }
        // Non empty means that header points at reference folder
        else {
            'reference: loop {
                // First element should be header
                assert!(model.get::<bool>(&current_iter, column_header), "First deleted element, should be a header");

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
                model.remove(&model.iter(tree_path).expect("Using invalid tree_path"));
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

pub fn resize_pixbuf_dimension(pixbuf: &Pixbuf, requested_size: (i32, i32), interp_type: InterpType) -> Option<Pixbuf> {
    let current_ratio = pixbuf.width() as f32 / pixbuf.height() as f32;
    let mut new_size;
    match current_ratio.total_cmp(&(requested_size.0 as f32 / requested_size.1 as f32)) {
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
    let characters_in_filename = file_name.chars().count();
    if characters_in_filename > max_length {
        let start_characters = 10;
        let difference = characters_in_filename - max_length;
        let second_part_start = start_characters + difference;
        let mut string_pre = String::new();
        let mut string_after = String::new();

        for (index, character) in file_name.chars().enumerate() {
            if index < start_characters {
                string_pre.push(character);
            } else if index >= second_part_start {
                string_after.push(character);
            }
        }

        format!("{string_pre} ... {string_after}")
    } else {
        file_name.to_string()
    }
}

pub fn get_custom_label_from_widget<P: IsA<Widget>>(item: &P) -> gtk4::Label {
    let mut widgets_to_check = vec![item.clone().upcast::<Widget>()];

    while let Some(widget) = widgets_to_check.pop() {
        if let Ok(label) = widget.clone().downcast::<gtk4::Label>() {
            return label;
        }
        widgets_to_check.extend(get_all_direct_children(&widget));
    }
    panic!("Button doesn't have proper custom label child");
}

pub fn get_custom_image_from_widget<P: IsA<Widget>>(item: &P) -> gtk4::Image {
    let mut widgets_to_check = vec![item.clone().upcast::<Widget>()];

    while let Some(widget) = widgets_to_check.pop() {
        if let Ok(image) = widget.clone().downcast::<gtk4::Image>() {
            return image;
        }
        widgets_to_check.extend(get_all_direct_children(&widget));
    }
    panic!("Button doesn't have proper custom label child");
}

#[allow(dead_code)]
pub fn debug_print_widget<P: IsA<Widget>>(item: &P) {
    let mut widgets_to_check = vec![(0, 0, item.clone().upcast::<Widget>())];

    let mut next_free_number = 1;
    println!("{}, {}, {:?} ", widgets_to_check[0].0, widgets_to_check[0].1, widgets_to_check[0].2);

    while let Some((current_number, parent_number, widget)) = widgets_to_check.pop() {
        for widget in get_all_direct_children(&widget) {
            widgets_to_check.push((next_free_number, current_number, widget));
            next_free_number += 1;
        }
        println!("{current_number}, {parent_number}, {widget:?} ");
    }
}

pub fn get_all_boxes_from_widget<P: IsA<Widget>>(item: &P) -> Vec<gtk4::Box> {
    let mut widgets_to_check = vec![item.clone().upcast::<Widget>()];
    let mut boxes = Vec::new();

    while let Some(widget) = widgets_to_check.pop() {
        widgets_to_check.extend(get_all_direct_children(&widget));
        if let Ok(bbox) = widget.clone().downcast::<gtk4::Box>() {
            boxes.push(bbox);
        }
    }
    boxes
}

pub fn get_all_direct_children<P: IsA<Widget>>(wid: &P) -> Vec<Widget> {
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
    let pixbuf = Pixbuf::from_read(BufReader::new(data)).expect("Failed to create pixbuf from data");
    let pixbuf = pixbuf.scale_simple(SIZE_OF_ICON, SIZE_OF_ICON, TYPE_OF_INTERPOLATION).expect("Failed to scale pixbuf");
    image.set_from_pixbuf(Some(&pixbuf));
}

static mut IMAGE_PREVIEW_ARRAY: OnceCell<Vec<u8>> = OnceCell::new();

pub fn get_pixbuf_from_dynamic_image(dynamic_image: &DynamicImage) -> Result<Pixbuf, Error> {
    let mut output = Vec::new();
    JpegEncoder::new(&mut output).encode_image(dynamic_image).expect("Failed to encode jpeg image"); // TODO remove here unwrap
    let arra;
    unsafe {
        IMAGE_PREVIEW_ARRAY.take();
        IMAGE_PREVIEW_ARRAY.set(output).expect("Setting image preview array failed");
        arra = IMAGE_PREVIEW_ARRAY.get().expect("Getting image preview array failed").as_bytes();
    }
    Pixbuf::from_read(arra)
}

pub fn check_if_value_is_in_list_store(list_store: &ListStore, column: i32, value: &str) -> bool {
    if let Some(iter) = list_store.iter_first() {
        loop {
            let list_store_value: String = list_store.get::<String>(&iter, column);

            if value == list_store_value {
                return true;
            }

            if !list_store.iter_next(&iter) {
                break;
            }
        }
    }

    false
}

pub fn check_if_list_store_column_have_all_same_values(list_store: &ListStore, column: i32, value: bool) -> bool {
    if let Some(iter) = list_store.iter_first() {
        loop {
            let list_store_value: bool = list_store.get::<bool>(&iter, column);

            if value != list_store_value {
                return false;
            }

            if !list_store.iter_next(&iter) {
                break;
            }
        }
        return true;
    }
    false
}

pub fn scale_set_min_max_values(scale: &Scale, minimum: f64, maximum: f64, current_value: f64, step: Option<f64>) {
    scale.set_range(minimum, maximum);
    scale.set_fill_level(maximum);
    scale.set_value(current_value);
    if let Some(step) = step {
        scale.adjustment().set_step_increment(step);
    }
}

pub fn scale_step_function(scale: &Scale, _scroll_type: ScrollType, value: f64) -> glib::Propagation {
    scale.set_increments(1_f64, 1_f64);
    scale.set_round_digits(0);
    scale.set_fill_level(value.round());
    glib::Propagation::Proceed
}

#[cfg(test)]
mod test {
    use glib::types::Type;
    use glib::Value;
    use gtk4::prelude::*;
    use gtk4::{Orientation, TreeView};
    use image::DynamicImage;

    use crate::help_functions::{
        change_dimension_to_krotka, check_if_list_store_column_have_all_same_values, check_if_value_is_in_list_store, get_all_boxes_from_widget, get_all_direct_children,
        get_max_file_name, get_pixbuf_from_dynamic_image, get_string_from_list_store,
    };

    #[gtk4::test]
    fn test_get_string_from_list_store() {
        let columns_types: &[Type] = &[Type::STRING];
        let list_store = gtk4::ListStore::new(columns_types);
        let tree_view = TreeView::with_model(&list_store);

        let values_to_add: &[(u32, &dyn ToValue)] = &[(0, &"test"), (0, &"test2"), (0, &"test3")];
        for i in values_to_add {
            list_store.set(&list_store.append(), &[*i]);
        }
        assert_eq!(
            get_string_from_list_store(&tree_view, 0, None),
            vec!["test".to_string(), "test2".to_string(), "test3".to_string()]
        );

        let columns_types: &[Type] = &[Type::BOOL, Type::STRING];
        let list_store = gtk4::ListStore::new(columns_types);
        let tree_view = TreeView::with_model(&list_store);

        let values_to_add: &[&[(u32, &dyn ToValue)]] = &[
            &[(0, &Into::<Value>::into(true)), (1, &Into::<Value>::into("test"))],
            &[(0, &Into::<Value>::into(true)), (1, &Into::<Value>::into("test2"))],
            &[(0, &Into::<Value>::into(false)), (1, &Into::<Value>::into("test3"))],
        ];
        for i in values_to_add {
            list_store.set(&list_store.append(), i);
        }
        assert_eq!(get_string_from_list_store(&tree_view, 1, Some(0)), vec!["test".to_string(), "test2".to_string()]);
    }

    #[gtk4::test]
    fn test_check_if_list_store_column_have_all_same_values() {
        let columns_types: &[Type] = &[Type::BOOL];
        let list_store = gtk4::ListStore::new(columns_types);

        list_store.clear();
        let values_to_add: &[(u32, &dyn ToValue)] = &[(0, &true), (0, &true), (0, &false)];
        for i in values_to_add {
            list_store.set(&list_store.append(), &[*i]);
        }
        assert!(!check_if_list_store_column_have_all_same_values(&list_store, 0, true));
        assert!(!check_if_list_store_column_have_all_same_values(&list_store, 0, false));

        list_store.clear();
        let values_to_add: &[(u32, &dyn ToValue)] = &[(0, &true), (0, &true), (0, &true)];
        for i in values_to_add {
            list_store.set(&list_store.append(), &[*i]);
        }
        assert!(check_if_list_store_column_have_all_same_values(&list_store, 0, true));
        assert!(!check_if_list_store_column_have_all_same_values(&list_store, 0, false));

        list_store.clear();
        let values_to_add: &[(u32, &dyn ToValue)] = &[(0, &false)];
        for i in values_to_add {
            list_store.set(&list_store.append(), &[*i]);
        }
        assert!(!check_if_list_store_column_have_all_same_values(&list_store, 0, true));
        assert!(check_if_list_store_column_have_all_same_values(&list_store, 0, false));

        list_store.clear();
        assert!(!check_if_list_store_column_have_all_same_values(&list_store, 0, true));
        assert!(!check_if_list_store_column_have_all_same_values(&list_store, 0, false));
    }

    #[gtk4::test]
    fn test_check_if_value_is_in_list_store() {
        let columns_types: &[Type] = &[Type::STRING];
        let list_store = gtk4::ListStore::new(columns_types);
        let values_to_add: &[(u32, &dyn ToValue)] = &[(0, &"Koczkodan"), (0, &"Kachir")];
        for i in values_to_add {
            list_store.set(&list_store.append(), &[*i]);
        }
        assert!(check_if_value_is_in_list_store(&list_store, 0, "Koczkodan"));
        assert!(check_if_value_is_in_list_store(&list_store, 0, "Kachir"));
        assert!(!check_if_value_is_in_list_store(&list_store, 0, "Koczkodan2"));

        let columns_types: &[Type] = &[Type::STRING, Type::STRING];
        let list_store = gtk4::ListStore::new(columns_types);
        let values_to_add: &[&[(u32, &dyn ToValue)]] = &[&[(0, &"Koczkodan"), (1, &"Krakus")], &[(0, &"Kachir"), (1, &"Wodnica")]];
        for i in values_to_add {
            list_store.set(&list_store.append(), i);
        }
        assert!(check_if_value_is_in_list_store(&list_store, 0, "Koczkodan"));
        assert!(check_if_value_is_in_list_store(&list_store, 1, "Krakus"));
        assert!(check_if_value_is_in_list_store(&list_store, 0, "Kachir"));
        assert!(check_if_value_is_in_list_store(&list_store, 1, "Wodnica"));
        assert!(!check_if_value_is_in_list_store(&list_store, 0, "Krakus"));
        assert!(!check_if_value_is_in_list_store(&list_store, 1, "Kachir"));
    }

    #[test]
    fn test_file_name_shortener() {
        let name_to_check = "/home/rafal/czkawek/romek/atomek.txt";
        assert_eq!(get_max_file_name(name_to_check, 20), "/home/rafa ... atomek.txt");
        assert_eq!(get_max_file_name(name_to_check, 21), "/home/rafa ... /atomek.txt");
        let name_to_check = "/home/rafal/czkawek/romek/czekistan/atomek.txt";
        assert_eq!(get_max_file_name(name_to_check, 21), "/home/rafa ... /atomek.txt");
        assert_eq!(get_max_file_name(name_to_check, 80), name_to_check);
        let name_to_check = "/home/rafal/‍🌈🌈🌈🌈🌈🌈🌈🌈🌈🌈🌈🌈🌈.txt";
        assert_eq!(get_max_file_name(name_to_check, 21), "/home/rafa ... 🌈🌈🌈🌈🌈🌈🌈.txt");
        assert_eq!(get_max_file_name(name_to_check, 20), "/home/rafa ... 🌈🌈🌈🌈🌈🌈.txt");
        assert_eq!(get_max_file_name(name_to_check, 19), "/home/rafa ... 🌈🌈🌈🌈🌈.txt");
        let name_to_check = "/home/rafal/‍🏳️‍🌈️🏳️‍🌈️🏳️‍🌈️🏳️‍🌈️🏳️‍🌈️🏳️‍🌈️🏳️‍🌈️🏳️‍🌈️🏳️‍🌈️.txt";
        assert_eq!(get_max_file_name(name_to_check, 21), "/home/rafa ... 🌈\u{fe0f}🏳\u{fe0f}\u{200d}🌈\u{fe0f}.txt");
        assert_eq!(get_max_file_name(name_to_check, 20), "/home/rafa ... \u{fe0f}🏳\u{fe0f}\u{200d}🌈\u{fe0f}.txt");
        assert_eq!(get_max_file_name(name_to_check, 19), "/home/rafa ... 🏳\u{fe0f}\u{200d}🌈\u{fe0f}.txt");
        assert_eq!(get_max_file_name(name_to_check, 18), "/home/rafa ... \u{fe0f}\u{200d}🌈\u{fe0f}.txt");
        assert_eq!(get_max_file_name(name_to_check, 17), "/home/rafa ... \u{200d}🌈\u{fe0f}.txt");
        assert_eq!(get_max_file_name(name_to_check, 16), "/home/rafa ... 🌈\u{fe0f}.txt");
    }

    #[test]
    fn test_pixbuf_from_dynamic_image() {
        let dynamic_image = DynamicImage::new_rgb8(1, 1);
        get_pixbuf_from_dynamic_image(&dynamic_image).expect("Failed to get pixbuf from dynamic image");
        get_pixbuf_from_dynamic_image(&dynamic_image).expect("Failed to get pixbuf from dynamic image");
        get_pixbuf_from_dynamic_image(&dynamic_image).expect("Failed to get pixbuf from dynamic image");
        get_pixbuf_from_dynamic_image(&dynamic_image).expect("Failed to get pixbuf from dynamic image");
    }

    #[test]
    fn test_change_dimension_to_krotka() {
        assert_eq!(change_dimension_to_krotka("50x50"), (50, 50));
        assert_eq!(change_dimension_to_krotka("6000x6000"), (6000, 6000));
    }

    #[gtk4::test]
    fn test_get_all_direct_children() {
        let obj = gtk4::Box::new(Orientation::Horizontal, 0);
        let obj2 = gtk4::Box::new(Orientation::Horizontal, 0);
        let obj3 = gtk4::Image::new();
        let obj4 = gtk4::Image::new();
        let obj5 = gtk4::Image::new();
        obj.append(&obj2);
        obj.append(&obj3);
        obj2.append(&obj4);
        obj2.append(&obj5);
        assert_eq!(get_all_direct_children(&obj).len(), 2);
    }

    #[gtk4::test]
    fn test_get_all_boxes_from_widget() {
        let obj = gtk4::Box::new(Orientation::Horizontal, 0);
        let obj2 = gtk4::Box::new(Orientation::Horizontal, 0);
        let obj3 = gtk4::Image::new();
        let obj4 = gtk4::Image::new();
        let obj5 = gtk4::Image::new();
        obj.append(&obj2);
        obj.append(&obj3);
        obj2.append(&obj4);
        obj2.append(&obj5);
        assert_eq!(get_all_boxes_from_widget(&obj).len(), 2);
    }
}
