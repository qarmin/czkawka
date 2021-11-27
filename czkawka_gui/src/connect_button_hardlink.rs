use std::fs;
use std::path::PathBuf;

use gtk::prelude::*;
use gtk::{TextView, TreeIter, TreePath};

use czkawka_core::duplicate::make_hard_link;

use crate::gui_data::GuiData;
use crate::help_functions::*;
use crate::notebook_enums::*;

pub fn connect_button_hardlink_symlink(gui_data: &GuiData) {
    let buttons_hardlink = gui_data.bottom_buttons.buttons_hardlink.clone();

    let notebook_main = gui_data.main_notebook.notebook_main.clone();
    let main_tree_views = gui_data.main_notebook.get_main_tree_views();

    let image_preview_similar_images = gui_data.main_notebook.image_preview_similar_images.clone();
    let image_preview_duplicates = gui_data.main_notebook.image_preview_duplicates.clone();

    let text_view_errors = gui_data.text_view_errors.clone();

    buttons_hardlink.connect_clicked(move |_| {
        let nb_number = notebook_main.current_page().unwrap();
        let tree_view = &main_tree_views[nb_number as usize];
        let nb_object = &NOTEBOOKS_INFOS[nb_number as usize];

        let column_color = nb_object.column_color.expect("Hardinkning can be only used for tree views with grouped results");
        hardlink_symlink(tree_view, nb_object.column_name, nb_object.column_path, column_color, nb_object.column_selection, true, &text_view_errors);

        match &nb_object.notebook_type {
            NotebookMainEnum::SimilarImages => {
                image_preview_similar_images.hide();
            }
            NotebookMainEnum::Duplicate => {
                image_preview_duplicates.hide();
            }
            _ => {}
        }
    });

    let buttons_symlink = gui_data.bottom_buttons.buttons_symlink.clone();

    let notebook_main = gui_data.main_notebook.notebook_main.clone();
    let main_tree_views = gui_data.main_notebook.get_main_tree_views();

    let image_preview_similar_images = gui_data.main_notebook.image_preview_similar_images.clone();
    let image_preview_duplicates = gui_data.main_notebook.image_preview_duplicates.clone();

    let text_view_errors = gui_data.text_view_errors.clone();

    buttons_symlink.connect_clicked(move |_| {
        let nb_number = notebook_main.current_page().unwrap();
        let tree_view = &main_tree_views[nb_number as usize];
        let nb_object = &NOTEBOOKS_INFOS[nb_number as usize];

        let column_color = nb_object.column_color.expect("Symlinking can be only used for tree views with grouped results");
        hardlink_symlink(tree_view, nb_object.column_name, nb_object.column_path, column_color, nb_object.column_selection, false, &text_view_errors);

        match &nb_object.notebook_type {
            NotebookMainEnum::SimilarImages => {
                image_preview_similar_images.hide();
            }
            NotebookMainEnum::Duplicate => {
                image_preview_duplicates.hide();
            }
            _ => {}
        }
    });
}

pub fn hardlink_symlink(tree_view: &gtk::TreeView, column_file_name: i32, column_path: i32, column_color: i32, column_selection: i32, hardlinking: bool, text_view_errors: &TextView) {
    reset_text_view(text_view_errors);

    let model = get_list_store(tree_view);

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
            if model.value(&iter, column_selection).get::<bool>().unwrap() {
                if model.value(&iter, column_color).get::<String>().unwrap() == MAIN_ROW_COLOR {
                    selected_rows.push(model.path(&iter).unwrap());
                } else {
                    panic!("Header row shouldn't be selected, please report bug.");
                }
            }
            if !model.iter_next(&iter) {
                break;
            }
        }
    }

    if selected_rows.is_empty() {
        return; // No selected rows
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
                        add_text_to_text_view(text_view_errors, format!("Failed to hardlink {}, reason {}", file_to_hardlink, e).as_str());
                        continue;
                    }
                }
            }
        }
    } else {
        for symhardlink_data in vec_symhardlink_data {
            for file_to_symlink in symhardlink_data.files_to_symhardlink {
                match fs::remove_file(&file_to_symlink) {
                    Ok(_) => (),
                    Err(e) => {
                        add_text_to_text_view(text_view_errors, format!("Failed to remove file {} when creating symlink, reason {}", file_to_symlink, e).as_str());
                        continue;
                    }
                };

                #[cfg(target_family = "unix")]
                {
                    match std::os::unix::fs::symlink(&symhardlink_data.original_data, &file_to_symlink) {
                        Ok(_) => (),
                        Err(e) => {
                            add_text_to_text_view(text_view_errors, format!("Failed to remove file {} when creating symlink, reason {}", file_to_symlink, e).as_str());
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
        }
    }
    for tree_path in vec_tree_path_to_remove.iter().rev() {
        model.remove(&model.iter(tree_path).unwrap());
    }

    clean_invalid_headers(&model, column_color);
}
