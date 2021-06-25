extern crate gtk;
use crate::gui_data::GuiData;
use crate::help_functions::*;
use crate::notebook_enums::*;
use gtk::prelude::*;
use gtk::{TreeIter, TreePath};
use std::fs;

pub fn connect_button_symlink(gui_data: &GuiData) {
    let gui_data = gui_data.clone();

    let buttons_symlink = gui_data.bottom_buttons.buttons_symlink.clone();
    let notebook_main = gui_data.main_notebook.notebook_main.clone();

    let tree_view_duplicate_finder = gui_data.main_notebook.tree_view_duplicate_finder.clone();
    let tree_view_similar_images_finder = gui_data.main_notebook.tree_view_similar_images_finder.clone();
    let tree_view_same_music_finder = gui_data.main_notebook.tree_view_same_music_finder.clone();

    let image_preview_similar_images = gui_data.main_notebook.image_preview_similar_images.clone();

    buttons_symlink.connect_clicked(move |_| match to_notebook_main_enum(notebook_main.current_page().unwrap()) {
        NotebookMainEnum::Duplicate => {
            symlink(tree_view_duplicate_finder.clone(), ColumnsDuplicates::Name as i32, ColumnsDuplicates::Path as i32, ColumnsDuplicates::Color as i32, &gui_data);
        }
        NotebookMainEnum::SameMusic => {
            symlink(tree_view_same_music_finder.clone(), ColumnsSameMusic::Name as i32, ColumnsSameMusic::Path as i32, ColumnsSameMusic::Color as i32, &gui_data);
        }
        NotebookMainEnum::SimilarImages => {
            symlink(
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
fn symlink(tree_view: gtk::TreeView, column_file_name: i32, column_path: i32, column_color: i32, gui_data: &GuiData) {
    let text_view_errors = gui_data.text_view_errors.clone();
    reset_text_view(&text_view_errors);

    let list_store = get_list_store(&tree_view);
    let selection = tree_view.selection();

    let (selection_rows, tree_model) = selection.selected_rows();
    if selection_rows.is_empty() {
        return;
    }

    struct SymlinkData {
        original_data: String,
        files_to_symlink: Vec<String>,
    }
    let mut vec_tree_path_to_remove: Vec<TreePath> = Vec::new(); // List of symlinked files without its root
    let mut vec_symlink_data: Vec<SymlinkData> = Vec::new();

    let current_iter: TreeIter = tree_model.iter_first().unwrap(); // Symlink button should be only visible when more than 1 element is visible, otherwise it needs to be fixed
    let mut current_symlink_data: Option<SymlinkData> = None;
    let mut current_selected_index = 0;
    loop {
        if tree_model.value(&current_iter, column_color).get::<String>().unwrap() == HEADER_ROW_COLOR {
            if let Some(current_symlink_data) = current_symlink_data {
                if !current_symlink_data.files_to_symlink.is_empty() {
                    vec_symlink_data.push(current_symlink_data);
                }
            }

            current_symlink_data = None;
            if !tree_model.iter_next(&current_iter) {
                panic!("HEADER, shouldn't be a last item.");
            }
            continue;
        }

        if tree_model.path(&current_iter).unwrap() == selection_rows[current_selected_index] {
            let file_name = tree_model.value(&current_iter, column_file_name).get::<String>().unwrap();
            let path = tree_model.value(&current_iter, column_path).get::<String>().unwrap();
            let full_file_path = format!("{}/{}", path, file_name);

            if current_symlink_data.is_some() {
                vec_tree_path_to_remove.push(tree_model.path(&current_iter).unwrap());
                let mut temp_data = current_symlink_data.unwrap();
                temp_data.files_to_symlink.push(full_file_path);
                current_symlink_data = Some(temp_data);
            } else {
                current_symlink_data = Some(SymlinkData {
                    original_data: full_file_path,
                    files_to_symlink: vec![],
                });
            }

            if current_selected_index != selection_rows.len() - 1 {
                current_selected_index += 1;
            } else {
                if let Some(current_symlink_data) = current_symlink_data {
                    if !current_symlink_data.files_to_symlink.is_empty() {
                        vec_symlink_data.push(current_symlink_data);
                    }
                }
                break; // There is no more selected items, so we just end checking
            }
        }

        if !tree_model.iter_next(&current_iter) {
            if let Some(current_symlink_data) = current_symlink_data {
                if !current_symlink_data.files_to_symlink.is_empty() {
                    vec_symlink_data.push(current_symlink_data);
                }
            }

            break;
        }
    }
    for symlink_data in vec_symlink_data {
        for file_to_symlink in symlink_data.files_to_symlink {
            match fs::remove_file(&file_to_symlink) {
                Ok(_) => (),
                Err(_) => {
                    add_text_to_text_view(&text_view_errors, format!("Failed to remove file {} when creating symlink.", file_to_symlink).as_str());
                    continue;
                }
            };

            #[cfg(target_family = "unix")]
            {
                match std::os::unix::fs::symlink(&symlink_data.original_data, &file_to_symlink) {
                    Ok(_) => (),
                    Err(_) => {
                        add_text_to_text_view(&text_view_errors, format!("Failed to remove file {} when creating symlink.", file_to_symlink).as_str());
                        continue;
                    }
                };
            }
            // TODO Add this, because for now it not working ()
            // #[cfg(target_family = "windows")]
            // {
            //     match std::os::windows::fs::symlink(&symlink_data.original_data, &file_to_symlink) {
            //         Ok(_) => (),
            //         Err(_) => {
            //             add_text_to_text_view(&text_view_errors, format!("Failed to remove file {} when creating symlink.", file_to_symlink).as_str());
            //             continue;
            //         }
            //     };
            // }
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
        if tree_model.value(&current_iter, column_color).get::<String>().unwrap() != HEADER_ROW_COLOR {
            panic!(); // First element should be header
        };

        let mut next_iter;
        let mut next_next_iter;
        'main: loop {
            if tree_model.value(&current_iter, column_color).get::<String>().unwrap() != HEADER_ROW_COLOR {
                panic!(); // First element should be header
            };

            next_iter = current_iter.clone();
            if !list_store.iter_next(&next_iter) {
                // There is only single header left (H1 -> END) -> (NOTHING)
                vec_tree_path_to_delete.push(list_store.path(&current_iter).unwrap());
                break 'main;
            }

            if tree_model.value(&next_iter, column_color).get::<String>().unwrap() == HEADER_ROW_COLOR {
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

            if tree_model.value(&next_next_iter, column_color).get::<String>().unwrap() == HEADER_ROW_COLOR {
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
                if tree_model.value(&next_next_iter, column_color).get::<String>().unwrap() == HEADER_ROW_COLOR {
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
