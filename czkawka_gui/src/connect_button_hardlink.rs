use crate::gui_data::GuiData;
use crate::help_functions::*;
use crate::notebook_enums::*;
use czkawka_core::duplicate::make_hard_link;
use gtk::prelude::*;
use gtk::{TreeIter, TreePath};
use std::fs;
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
            hardlink_symlink(
                tree_view_duplicate_finder.clone(),
                ColumnsDuplicates::Name as i32,
                ColumnsDuplicates::Path as i32,
                ColumnsDuplicates::Color as i32,
                ColumnsDuplicates::ActiveSelectButton as i32,
                true,
                &gui_data,
            );
        }
        NotebookMainEnum::SameMusic => {
            hardlink_symlink(
                tree_view_same_music_finder.clone(),
                ColumnsSameMusic::Name as i32,
                ColumnsSameMusic::Path as i32,
                ColumnsSameMusic::Color as i32,
                ColumnsSameMusic::ActiveSelectButton as i32,
                true,
                &gui_data,
            );
        }
        NotebookMainEnum::SimilarImages => {
            hardlink_symlink(
                tree_view_similar_images_finder.clone(),
                ColumnsSimilarImages::Name as i32,
                ColumnsSimilarImages::Path as i32,
                ColumnsSimilarImages::Color as i32,
                ColumnsSimilarImages::ActiveSelectButton as i32,
                true,
                &gui_data,
            );
            image_preview_similar_images.hide();
        }
        e => panic!("Not existent {:?}", e),
    });
}

pub fn hardlink_symlink(tree_view: gtk::TreeView, column_file_name: i32, column_path: i32, column_color: i32, column_selection: i32, hardlinking: bool, gui_data: &GuiData) {
    let text_view_errors = gui_data.text_view_errors.clone();
    reset_text_view(&text_view_errors);

    let model = get_list_store(&tree_view);

    #[derive(Debug)]
    struct SymHardlinkData {
        original_data: String,
        files_to_symhardlink: Vec<String>,
    }
    let mut vec_tree_path_to_remove: Vec<TreePath> = Vec::new(); // List of hardlinked files without its root
    let mut vec_symhardlink_data: Vec<SymHardlinkData> = Vec::new();

    let current_iter: TreeIter = match model.iter_first() {
        Some(t) => t,
        None => return, // No records
    };

    let mut selected_rows = Vec::new();
    if let Some(iter) = model.iter_first() {
        loop {
            if model.value(&iter, column_color).get::<String>().unwrap() == MAIN_ROW_COLOR && model.value(&iter, column_selection).get::<bool>().unwrap() {
                selected_rows.push(model.path(&iter).unwrap());
            }
            if !model.iter_next(&iter) {
                break;
            }
        }
    }

    let mut current_symhardlink_data: Option<SymHardlinkData> = None;
    let mut current_selected_index = 0;
    loop {
        if model.value(&current_iter, column_color).get::<String>().unwrap() == HEADER_ROW_COLOR {
            if let Some(current_symhardlink_data) = current_symhardlink_data {
                if !current_symhardlink_data.files_to_symhardlink.is_empty() {
                    vec_symhardlink_data.push(current_symhardlink_data);
                }
            }

            current_symhardlink_data = None;
            if !model.iter_next(&current_iter) {
                panic!("HEADER, shouldn't be a last item.");
            }
            continue;
        }

        if model.path(&current_iter).unwrap() == selected_rows[current_selected_index] {
            let file_name = model.value(&current_iter, column_file_name).get::<String>().unwrap();
            let path = model.value(&current_iter, column_path).get::<String>().unwrap();
            let full_file_path = format!("{}/{}", path, file_name);

            if current_symhardlink_data.is_some() {
                vec_tree_path_to_remove.push(model.path(&current_iter).unwrap());
                let mut temp_data = current_symhardlink_data.unwrap();
                temp_data.files_to_symhardlink.push(full_file_path);
                current_symhardlink_data = Some(temp_data);
            } else {
                current_symhardlink_data = Some(SymHardlinkData {
                    original_data: full_file_path,
                    files_to_symhardlink: vec![],
                });
            }

            if current_selected_index != selected_rows.len() - 1 {
                current_selected_index += 1;
            } else {
                if let Some(current_symhardlink_data) = current_symhardlink_data {
                    if !current_symhardlink_data.files_to_symhardlink.is_empty() {
                        vec_symhardlink_data.push(current_symhardlink_data);
                    }
                }
                break; // There is no more selected items, so we just end checking
            }
        }

        if !model.iter_next(&current_iter) {
            if let Some(current_symhardlink_data) = current_symhardlink_data {
                if !current_symhardlink_data.files_to_symhardlink.is_empty() {
                    vec_symhardlink_data.push(current_symhardlink_data);
                }
            }

            break;
        }
    }
    if hardlinking {
        dbg!(&vec_symhardlink_data);
        for symhardlink_data in vec_symhardlink_data {
            dbg!(&symhardlink_data);
            for file_to_hardlink in symhardlink_data.files_to_symhardlink {
                match make_hard_link(&PathBuf::from(&symhardlink_data.original_data), &PathBuf::from(&file_to_hardlink)) {
                    Ok(_) => (),
                    Err(e) => {
                        add_text_to_text_view(&text_view_errors, format!("Failed to hardlink {}, reason {}", file_to_hardlink, e).as_str());
                        continue;
                    }
                }
            }
            println!();
        }
    } else {
        for symhardlink_data in vec_symhardlink_data {
            for file_to_symlink in symhardlink_data.files_to_symhardlink {
                match fs::remove_file(&file_to_symlink) {
                    Ok(_) => (),
                    Err(e) => {
                        add_text_to_text_view(&text_view_errors, format!("Failed to remove file {} when creating symlink, reason {}", file_to_symlink, e).as_str());
                        continue;
                    }
                };

                #[cfg(target_family = "unix")]
                {
                    match std::os::unix::fs::symlink(&symhardlink_data.original_data, &file_to_symlink) {
                        Ok(_) => (),
                        Err(e) => {
                            add_text_to_text_view(&text_view_errors, format!("Failed to remove file {} when creating symlink, reason {}", file_to_symlink, e).as_str());
                            continue;
                        }
                    };
                }
                #[cfg(target_family = "windows")]
                {
                    match std::os::windows::fs::symlink_file(&symhardlink_data.original_data, &file_to_symlink) {
                        Ok(_) => (),
                        Err(e) => {
                            add_text_to_text_view(&text_view_errors, format!("Failed to remove file {} when creating symlink, reason {}", file_to_symlink, e).as_str());
                            continue;
                        }
                    };
                }
            }
            println!();
        }
    }
    for tree_path in vec_tree_path_to_remove.iter().rev() {
        model.remove(&model.iter(tree_path).unwrap());
    }

    // Remove only child from header
    if let Some(first_iter) = model.iter_first() {
        let mut vec_tree_path_to_delete: Vec<gtk::TreePath> = Vec::new();
        let mut current_iter = first_iter;
        if model.value(&current_iter, column_color).get::<String>().unwrap() != HEADER_ROW_COLOR {
            panic!(); // First element should be header
        };

        let mut next_iter;
        let mut next_next_iter;
        'main: loop {
            if model.value(&current_iter, column_color).get::<String>().unwrap() != HEADER_ROW_COLOR {
                panic!(); // First element should be header
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
}
