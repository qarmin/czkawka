use std::path::MAIN_SEPARATOR;

use gtk4::{Box as GtkBox, Dialog, Scale, ScrollType, TreeView};
use gtk4::prelude::*;
use crate::notebook_enums::{NotebookMainEnum, NotebookUpperEnum};
use crate::notebook_info::{NOTEBOOKS_INFO, NotebookObject};

pub fn get_dialog_box_child(dialog: &Dialog) -> GtkBox {
    dialog.child().expect("Dialog has no child").downcast::<GtkBox>().expect("Dialog child is not Box")
}

pub fn change_dimension_to_krotka(dimensions: &str) -> (u64, u64) {
    let vec = dimensions.split('x').collect::<Vec<_>>();
    assert_eq!(vec.len(), 2);
    let number1 = vec[0].parse::<u64>().expect("Invalid data in image dimension in position 0");
    let number2 = vec[1].parse::<u64>().expect("Invalid data in image dimension in position 1");
    (number1, number2)
}

pub fn get_notebook_enum_from_tree_view(tree_view: &TreeView) -> NotebookMainEnum {
    match tree_view.widget_name().as_str() {
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
        e => panic!("{e}"),
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
    match tree_view.widget_name().as_str() {
        "tree_view_upper_included_directories" => NotebookUpperEnum::IncludedDirectories,
        "tree_view_upper_excluded_directories" => NotebookUpperEnum::ExcludedDirectories,
        e => panic!("{e}"),
    }
}

pub fn get_tree_view_name_from_notebook_upper_enum(notebook_upper_enum: NotebookUpperEnum) -> &'static str {
    match notebook_upper_enum {
        NotebookUpperEnum::IncludedDirectories => "tree_view_upper_included_directories",
        NotebookUpperEnum::ExcludedDirectories => "tree_view_upper_excluded_directories",
        NotebookUpperEnum::ItemsConfiguration => panic!(),
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

pub fn get_max_file_name(file_name: &str, max_length: usize) -> String {
    assert!(max_length > 10);
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
