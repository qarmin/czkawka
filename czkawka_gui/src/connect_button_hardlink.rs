extern crate gtk;
use crate::gui_data::GuiData;
use crate::help_functions::*;
use crate::notebook_enums::*;
use czkawka_core::duplicate::make_hard_link;
use gtk::prelude::*;
use gtk::{TreeIter, TreePath};
use std::path::PathBuf;

pub fn connect_button_hardlink(gui_data: &GuiData) {
    let gui_data = gui_data.clone();

    let buttons_hardlink = gui_data.bottom_buttons.buttons_hardlink.clone();
    let notebook_main = gui_data.main_notebook.notebook_main.clone();

    let tree_view_duplicate_finder = gui_data.main_notebook.tree_view_duplicate_finder.clone();
    let tree_view_similar_images_finder = gui_data.main_notebook.tree_view_similar_images_finder.clone();
    let tree_view_same_music_finder = gui_data.main_notebook.tree_view_same_music_finder.clone();

    let image_preview_similar_images = gui_data.main_notebook.image_preview_similar_images.clone();

    buttons_hardlink.connect_clicked(move |_| match to_notebook_main_enum(notebook_main.current_page().unwrap()) {
        NotebookMainEnum::Duplicate => {
            hardlink(tree_view_duplicate_finder.clone(), ColumnsDuplicates::Name as i32, ColumnsDuplicates::Path as i32, ColumnsDuplicates::Color as i32, &gui_data);
        }
        NotebookMainEnum::SameMusic => {
            hardlink(tree_view_same_music_finder.clone(), ColumnsSameMusic::Name as i32, ColumnsSameMusic::Path as i32, ColumnsSameMusic::Color as i32, &gui_data);
        }
        NotebookMainEnum::SimilarImages => {
            hardlink(
                tree_view_similar_images_finder.clone(),
                ColumnsSimilarImages::Name as i32,
                ColumnsSimilarImages::Path as i32,
                ColumnsSimilarImages::Color as i32,
                &gui_data,
            );
            image_preview_similar_images.hide();
        }
        e => panic!("Not existent {:?}", e),
    });
}
fn hardlink(tree_view: gtk::TreeView, column_file_name: i32, column_path: i32, column_color: i32, gui_data: &GuiData) {
    let text_view_errors = gui_data.text_view_errors.clone();
    reset_text_view(&text_view_errors);

    let list_store = get_list_store(&tree_view);
    let selection = tree_view.selection();

    let (selection_rows, tree_model) = selection.selected_rows();
    if selection_rows.is_empty() {
        return;
    }

    struct HardlinkData {
        original_data: String,
        files_to_hardlink: Vec<String>,
    }
    let mut vec_tree_path_to_remove: Vec<TreePath> = Vec::new(); // List of hardlinked files without its root
    let mut vec_hardlink_data: Vec<HardlinkData> = Vec::new();

    let current_iter: TreeIter = tree_model.iter_first().unwrap(); // Hardlink button should be only visible when more than 1 element is visible, otherwise it needs to be fixed
    let mut current_hardlink_data: Option<HardlinkData> = None;
    let mut current_selected_index = 0;
    loop {
        if tree_model.value(&current_iter, column_color).get::<String>().unwrap().unwrap() == HEADER_ROW_COLOR {
            if let Some(current_hardlink_data) = current_hardlink_data {
                if !current_hardlink_data.files_to_hardlink.is_empty() {
                    vec_hardlink_data.push(current_hardlink_data);
                }
            }

            current_hardlink_data = None;
            if !tree_model.iter_next(&current_iter) {
                panic!("HEADER, shouldn't be a last item.");
            }
            continue;
        }

        if tree_model.path(&current_iter).unwrap() == selection_rows[current_selected_index] {
            let file_name = tree_model.value(&current_iter, column_file_name).get::<String>().unwrap().unwrap();
            let path = tree_model.value(&current_iter, column_path).get::<String>().unwrap().unwrap();
            let full_file_path = format!("{}/{}", path, file_name);

            if current_hardlink_data.is_some() {
                vec_tree_path_to_remove.push(tree_model.path(&current_iter).unwrap());
                let mut temp_data = current_hardlink_data.unwrap();
                temp_data.files_to_hardlink.push(full_file_path);
                current_hardlink_data = Some(temp_data);
            } else {
                current_hardlink_data = Some(HardlinkData {
                    original_data: full_file_path,
                    files_to_hardlink: vec![],
                });
            }

            if current_selected_index != selection_rows.len() - 1 {
                current_selected_index += 1;
            } else {
                if let Some(current_hardlink_data) = current_hardlink_data {
                    if !current_hardlink_data.files_to_hardlink.is_empty() {
                        vec_hardlink_data.push(current_hardlink_data);
                    }
                }
                break; // There is no more selected items, so we just end checking
            }
        }

        if !tree_model.iter_next(&current_iter) {
            if let Some(current_hardlink_data) = current_hardlink_data {
                if !current_hardlink_data.files_to_hardlink.is_empty() {
                    vec_hardlink_data.push(current_hardlink_data);
                }
            }

            break;
        }
    }
    for hardlink_data in vec_hardlink_data {
        for file_to_hardlink in hardlink_data.files_to_hardlink {
            match make_hard_link(&PathBuf::from(&hardlink_data.original_data), &PathBuf::from(&file_to_hardlink)) {
                Ok(_) => (),
                Err(_) => {
                    add_text_to_text_view(&text_view_errors, format!("Failed to hardlink {}.", file_to_hardlink).as_str());
                    continue;
                }
            }
        }
        println!();
    }
    for tree_path in vec_tree_path_to_remove.iter().rev() {
        list_store.remove(&tree_model.iter(tree_path).unwrap());
    }

    // Remove only child from header
    if let Some(first_iter) = list_store.iter_first() {
        let mut vec_tree_path_to_delete: Vec<gtk::TreePath> = Vec::new();
        let mut current_iter = first_iter;
        if tree_model.value(&current_iter, column_color).get::<String>().unwrap().unwrap() != HEADER_ROW_COLOR {
            panic!(); // First element should be header
        };

        let mut next_iter;
        let mut next_next_iter;
        'main: loop {
            if tree_model.value(&current_iter, column_color).get::<String>().unwrap().unwrap() != HEADER_ROW_COLOR {
                panic!(); // First element should be header
            };

            next_iter = current_iter.clone();
            if !list_store.iter_next(&next_iter) {
                // There is only single header left (H1 -> END) -> (NOTHING)
                vec_tree_path_to_delete.push(list_store.path(&current_iter).unwrap());
                break 'main;
            }

            if tree_model.value(&next_iter, column_color).get::<String>().unwrap().unwrap() == HEADER_ROW_COLOR {
                // There are two headers each others(we remove just first) -> (H1 -> H2) -> (H2)
                vec_tree_path_to_delete.push(list_store.path(&current_iter).unwrap());
                current_iter = next_iter.clone();
                continue 'main;
            }

            next_next_iter = next_iter.clone();
            if !list_store.iter_next(&next_next_iter) {
                // There is only one child of header left, so we remove it with header (H1 -> C1 -> END) -> (NOTHING)
                vec_tree_path_to_delete.push(list_store.path(&current_iter).unwrap());
                vec_tree_path_to_delete.push(list_store.path(&next_iter).unwrap());
                break 'main;
            }

            if tree_model.value(&next_next_iter, column_color).get::<String>().unwrap().unwrap() == HEADER_ROW_COLOR {
                // One child between two headers, we can remove them  (H1 -> C1 -> H2) -> (H2)
                vec_tree_path_to_delete.push(list_store.path(&current_iter).unwrap());
                vec_tree_path_to_delete.push(list_store.path(&next_iter).unwrap());
                current_iter = next_next_iter.clone();
                continue 'main;
            }

            loop {
                // (H1 -> C1 -> C2 -> Cn -> END) -> (NO CHANGE, BECAUSE IS GOOD)
                if !list_store.iter_next(&next_next_iter) {
                    break 'main;
                }
                // Move to next header
                if tree_model.value(&next_next_iter, column_color).get::<String>().unwrap().unwrap() == HEADER_ROW_COLOR {
                    current_iter = next_next_iter.clone();
                    continue 'main;
                }
            }
        }
        for tree_path in vec_tree_path_to_delete.iter().rev() {
            list_store.remove(&list_store.iter(&tree_path).unwrap());
        }
    }

    selection.unselect_all();
}
