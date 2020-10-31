extern crate gtk;
use crate::gui_data::GuiData;
use crate::help_functions::*;
use gtk::prelude::*;
use std::fs;
use std::fs::Metadata;

pub fn connect_button_delete(gui_data: &GuiData) {
    let buttons_delete = gui_data.buttons_delete.clone();
    let shared_confirmation_dialog_delete_dialog_showing_state = gui_data.shared_confirmation_dialog_delete_dialog_showing_state.clone();
    let scrolled_window_duplicate_finder = gui_data.scrolled_window_duplicate_finder.clone();
    let text_view_errors = gui_data.text_view_errors.clone();
    let notebook_main_children_names = gui_data.notebook_main_children_names.clone();
    let notebook_main = gui_data.notebook_main.clone();
    let window_main = gui_data.window_main.clone();
    let scrolled_window_main_empty_folder_finder = gui_data.scrolled_window_main_empty_folder_finder.clone();
    let scrolled_window_big_files_finder = gui_data.scrolled_window_big_files_finder.clone();
    let scrolled_window_main_empty_files_finder = gui_data.scrolled_window_main_empty_files_finder.clone();
    let scrolled_window_main_temporary_files_finder = gui_data.scrolled_window_main_temporary_files_finder.clone();
    let scrolled_window_similar_images_finder = gui_data.scrolled_window_similar_images_finder.clone();
    let scrolled_window_zeroed_files_finder = gui_data.scrolled_window_zeroed_files_finder.clone();

    buttons_delete.connect_clicked(move |_| {
        if *shared_confirmation_dialog_delete_dialog_showing_state.borrow_mut() {
            let confirmation_dialog_delete = gtk::Dialog::with_buttons(
                Option::from("Delete confirmation"),
                Option::from(&window_main),
                gtk::DialogFlags::MODAL,
                &[("Ok", gtk::ResponseType::Ok), ("Close", gtk::ResponseType::Cancel)],
            );
            let label: gtk::Label = gtk::Label::new(Some("Are you sure that you want to delete files?"));
            let check_button: gtk::CheckButton = gtk::CheckButton::with_label("Ask in future");
            check_button.set_active(true);

            for widgets in confirmation_dialog_delete.get_children() {
                // By default GtkBox is child of dialog, so we can easily add other things to it
                widgets.clone().downcast::<gtk::Box>().unwrap().add(&label);
                widgets.downcast::<gtk::Box>().unwrap().add(&check_button);
            }

            confirmation_dialog_delete.show_all();

            let response_type = confirmation_dialog_delete.run();
            if response_type == gtk::ResponseType::Ok {
                if !check_button.get_active() {
                    *shared_confirmation_dialog_delete_dialog_showing_state.borrow_mut() = false;
                }
            } else {
                confirmation_dialog_delete.close();
                return;
            }
            confirmation_dialog_delete.close();
        }

        match notebook_main_children_names.get(notebook_main.get_current_page().unwrap() as usize).unwrap().as_str() {
            "notebook_main_duplicate_finder_label" => {
                let tree_view = scrolled_window_duplicate_finder.get_children().get(0).unwrap().clone().downcast::<gtk::TreeView>().unwrap();
                let selection = tree_view.get_selection();

                let (selection_rows, tree_model) = selection.get_selected_rows();
                if selection_rows.is_empty() {
                    return;
                }
                let list_store = tree_model.clone().downcast::<gtk::ListStore>().unwrap();

                // let new_tree_model = TreeModel::new(); // TODO - maybe create new model when inserting a new data, because this seems to be not optimal when using thousands of rows

                let mut messages: String = "".to_string();

                // Must be deleted from end to start, because when deleting entries, TreePath(and also TreeIter) will points to invalid data
                for tree_path in selection_rows.iter().rev() {
                    let name = tree_model.get_value(&tree_model.get_iter(tree_path).unwrap(), ColumnsDuplicates::Name as i32).get::<String>().unwrap().unwrap();
                    let path = tree_model.get_value(&tree_model.get_iter(tree_path).unwrap(), ColumnsDuplicates::Path as i32).get::<String>().unwrap().unwrap();

                    match fs::remove_file(format!("{}/{}", path, name)) {
                        Ok(_) => {
                            list_store.remove(&list_store.get_iter(tree_path).unwrap());
                        }
                        Err(_) => messages += format!("Failed to remove file {}/{} because file doesn't exists or you don't have permissions.\n", path, name).as_str(),
                    }
                }

                text_view_errors.get_buffer().unwrap().set_text(messages.as_str());
                selection.unselect_all();
            }
            "scrolled_window_main_empty_folder_finder" => {
                let tree_view = scrolled_window_main_empty_folder_finder.get_children().get(0).unwrap().clone().downcast::<gtk::TreeView>().unwrap();
                let selection = tree_view.get_selection();

                let (selection_rows, tree_model) = selection.get_selected_rows();
                if selection_rows.is_empty() {
                    return;
                }
                let list_store = tree_model.clone().downcast::<gtk::ListStore>().unwrap();

                // let new_tree_model = TreeModel::new(); // TODO - maybe create new model when inserting a new data, because this seems to be not optimal when using thousands of rows

                let mut messages: String = "".to_string();

                // Must be deleted from end to start, because when deleting entries, TreePath(and also TreeIter) will points to invalid data
                for tree_path in selection_rows.iter().rev() {
                    let name = tree_model.get_value(&tree_model.get_iter(tree_path).unwrap(), ColumnsEmptyFolders::Name as i32).get::<String>().unwrap().unwrap();
                    let path = tree_model.get_value(&tree_model.get_iter(tree_path).unwrap(), ColumnsEmptyFolders::Path as i32).get::<String>().unwrap().unwrap();

                    // We must check if folder is really empty or contains only other empty folders
                    let mut error_happened = false;
                    let mut folders_to_check: Vec<String> = vec![format!("{}/{}", path, name)];
                    let mut current_folder: String;
                    let mut next_folder: String;
                    'dir: while !folders_to_check.is_empty() {
                        current_folder = folders_to_check.pop().unwrap();
                        let read_dir = match fs::read_dir(&current_folder) {
                            Ok(t) => t,
                            Err(_) => {
                                error_happened = true;
                                break 'dir;
                            }
                        };

                        for entry in read_dir {
                            let entry_data = match entry {
                                Ok(t) => t,
                                Err(_) => {
                                    error_happened = true;
                                    break 'dir;
                                }
                            };
                            let metadata: Metadata = match entry_data.metadata() {
                                Ok(t) => t,
                                Err(_) => {
                                    error_happened = true;
                                    break 'dir;
                                }
                            };
                            if metadata.is_dir() {
                                next_folder = "".to_owned()
                                    + &current_folder
                                    + "/"
                                    + match &entry_data.file_name().into_string() {
                                        Ok(t) => t,
                                        Err(_) => {
                                            error_happened = true;
                                            break 'dir;
                                        }
                                    };
                                folders_to_check.push(next_folder.clone());
                            } else {
                                error_happened = true;
                            }
                        }
                    }

                    if !error_happened {
                        match fs::remove_dir_all(format!("{}/{}", path, name)) {
                            Ok(_) => {
                                list_store.remove(&list_store.get_iter(tree_path).unwrap());
                            }
                            Err(_) => error_happened = true,
                        }
                    }
                    if error_happened {
                        messages += format!("Failed to remove folder {}/{} because folder doesn't exists, you don't have permissions or isn't empty.\n", path, name).as_str()
                    }
                }

                text_view_errors.get_buffer().unwrap().set_text(messages.as_str());
                selection.unselect_all();
            }
            "scrolled_window_main_empty_files_finder" => {
                let tree_view = scrolled_window_main_empty_files_finder.get_children().get(0).unwrap().clone().downcast::<gtk::TreeView>().unwrap();
                let selection = tree_view.get_selection();

                let (selection_rows, tree_model) = selection.get_selected_rows();
                if selection_rows.is_empty() {
                    return;
                }
                let list_store = tree_model.clone().downcast::<gtk::ListStore>().unwrap();

                // let new_tree_model = TreeModel::new(); // TODO - maybe create new model when inserting a new data, because this seems to be not optimal when using thousands of rows

                let mut messages: String = "".to_string();

                // Must be deleted from end to start, because when deleting entries, TreePath(and also TreeIter) will points to invalid data
                for tree_path in selection_rows.iter().rev() {
                    let name = tree_model.get_value(&tree_model.get_iter(tree_path).unwrap(), ColumnsEmptyFiles::Name as i32).get::<String>().unwrap().unwrap();
                    let path = tree_model.get_value(&tree_model.get_iter(tree_path).unwrap(), ColumnsEmptyFiles::Path as i32).get::<String>().unwrap().unwrap();

                    match fs::remove_file(format!("{}/{}", path, name)) {
                        Ok(_) => {
                            list_store.remove(&list_store.get_iter(tree_path).unwrap());
                        }
                        Err(_) => messages += format!("Failed to remove file {}/{} because file doesn't exists or you don't have permissions.\n", path, name).as_str(),
                    }
                }

                text_view_errors.get_buffer().unwrap().set_text(messages.as_str());
                selection.unselect_all();
            }
            "scrolled_window_main_temporary_files_finder" => {
                let tree_view = scrolled_window_main_temporary_files_finder.get_children().get(0).unwrap().clone().downcast::<gtk::TreeView>().unwrap();
                let selection = tree_view.get_selection();

                let (selection_rows, tree_model) = selection.get_selected_rows();
                if selection_rows.is_empty() {
                    return;
                }
                let list_store = tree_model.clone().downcast::<gtk::ListStore>().unwrap();

                // let new_tree_model = TreeModel::new(); // TODO - maybe create new model when inserting a new data, because this seems to be not optimal when using thousands of rows

                let mut messages: String = "".to_string();

                // Must be deleted from end to start, because when deleting entries, TreePath(and also TreeIter) will points to invalid data
                for tree_path in selection_rows.iter().rev() {
                    let name = tree_model.get_value(&tree_model.get_iter(tree_path).unwrap(), ColumnsTemporaryFiles::Name as i32).get::<String>().unwrap().unwrap();
                    let path = tree_model.get_value(&tree_model.get_iter(tree_path).unwrap(), ColumnsTemporaryFiles::Path as i32).get::<String>().unwrap().unwrap();

                    match fs::remove_file(format!("{}/{}", path, name)) {
                        Ok(_) => {
                            list_store.remove(&list_store.get_iter(tree_path).unwrap());
                        }
                        Err(_) => messages += format!("Failed to remove file {}/{} because file doesn't exists or you don't have permissions.\n", path, name).as_str(),
                    }
                }

                text_view_errors.get_buffer().unwrap().set_text(messages.as_str());
                selection.unselect_all();
            }
            "notebook_big_main_file_finder" => {
                let tree_view = scrolled_window_big_files_finder.get_children().get(0).unwrap().clone().downcast::<gtk::TreeView>().unwrap();
                let selection = tree_view.get_selection();

                let (selection_rows, tree_model) = selection.get_selected_rows();
                if selection_rows.is_empty() {
                    return;
                }
                let list_store = tree_model.clone().downcast::<gtk::ListStore>().unwrap();

                // let new_tree_model = TreeModel::new(); // TODO - maybe create new model when inserting a new data, because this seems to be not optimal when using thousands of rows

                let mut messages: String = "".to_string();

                // Must be deleted from end to start, because when deleting entries, TreePath(and also TreeIter) will points to invalid data
                for tree_path in selection_rows.iter().rev() {
                    let name = tree_model.get_value(&tree_model.get_iter(tree_path).unwrap(), ColumnsBigFiles::Name as i32).get::<String>().unwrap().unwrap();
                    let path = tree_model.get_value(&tree_model.get_iter(tree_path).unwrap(), ColumnsBigFiles::Path as i32).get::<String>().unwrap().unwrap();

                    match fs::remove_file(format!("{}/{}", path, name)) {
                        Ok(_) => {
                            list_store.remove(&list_store.get_iter(tree_path).unwrap());
                        }
                        Err(_) => messages += format!("Failed to remove file {}/{} because file doesn't exists or you don't have permissions.\n", path, name).as_str(),
                    }
                }

                text_view_errors.get_buffer().unwrap().set_text(messages.as_str());
                selection.unselect_all();
            }
            "notebook_main_similar_images_finder_label" => {
                let tree_view = scrolled_window_similar_images_finder.get_children().get(0).unwrap().clone().downcast::<gtk::TreeView>().unwrap();
                let selection = tree_view.get_selection();

                let (selection_rows, tree_model) = selection.get_selected_rows();
                if selection_rows.is_empty() {
                    return;
                }
                let list_store = tree_model.clone().downcast::<gtk::ListStore>().unwrap();

                // let new_tree_model = TreeModel::new(); // TODO - maybe create new model when inserting a new data, because this seems to be not optimal when using thousands of rows

                let mut messages: String = "".to_string();

                let mut vec_path_to_delete: Vec<(String, String)> = Vec::new();

                // Just remove file, later must be deleted list entry with all occurencies
                for tree_path in selection_rows.iter().rev() {
                    let name = tree_model.get_value(&tree_model.get_iter(tree_path).unwrap(), ColumnsSimilarImages::Name as i32).get::<String>().unwrap().unwrap();
                    let path = tree_model.get_value(&tree_model.get_iter(tree_path).unwrap(), ColumnsSimilarImages::Path as i32).get::<String>().unwrap().unwrap();

                    if fs::remove_file(format!("{}/{}", path, name)).is_err() {
                        messages += format!(
                            "Failed to remove file {}/{}. It is possible that you already deleted it, because similar images shows all possible file doesn't exists or you don't have permissions.\n",
                            path, name
                        )
                        .as_str()
                    }
                    vec_path_to_delete.push((path, name));
                }
                // Must be deleted from end to start, because when deleting entries, TreePath(and also TreeIter) will points to invalid data
                for path_to_delete in vec_path_to_delete {
                    let mut vec_tree_path_to_delete: Vec<gtk::TreePath> = Vec::new();

                    let iter = match list_store.get_iter_first() {
                        Some(t) => t,
                        None => break,
                    };
                    let mut take_child_mode = false; // When original image is searched one, we must remove all occurences of its children
                    let mut prepared_for_delete;
                    loop {
                        prepared_for_delete = false;
                        if take_child_mode {
                            let color = tree_model.get_value(&iter, ColumnsSimilarImages::Color as i32).get::<String>().unwrap().unwrap();
                            if color == HEADER_ROW_COLOR {
                                take_child_mode = false;
                            } else {
                                prepared_for_delete = true;
                            }
                        } else {
                            let path = tree_model.get_value(&iter, ColumnsSimilarImages::Path as i32).get::<String>().unwrap().unwrap();
                            if path == path_to_delete.0 {
                                let name = tree_model.get_value(&iter, ColumnsSimilarImages::Name as i32).get::<String>().unwrap().unwrap();
                                if name == path_to_delete.1 {
                                    let color = tree_model.get_value(&iter, ColumnsSimilarImages::Color as i32).get::<String>().unwrap().unwrap();
                                    if color == HEADER_ROW_COLOR {
                                        take_child_mode = true;
                                    }
                                    prepared_for_delete = true;
                                }
                            }
                        }

                        if prepared_for_delete {
                            vec_tree_path_to_delete.push(list_store.get_path(&iter).unwrap());
                        }

                        if !list_store.iter_next(&iter) {
                            break;
                        }
                    }

                    for tree_path in vec_tree_path_to_delete.iter().rev() {
                        list_store.remove(&list_store.get_iter(&tree_path).unwrap());
                    }
                }
                // End run to remove single header rows(without duplicates)
                if let Some(next_iter) = list_store.get_iter_first() {
                    let mut header_was_before = false;
                    let mut vec_tree_path_to_delete: Vec<gtk::TreePath> = Vec::new();
                    let mut current_iter = next_iter.clone();
                    loop {
                        let color = tree_model.get_value(&next_iter, ColumnsSimilarImages::Color as i32).get::<String>().unwrap().unwrap();
                        if color == HEADER_ROW_COLOR {
                            if header_was_before {
                                vec_tree_path_to_delete.push(list_store.get_path(&current_iter).unwrap());
                            } else {
                                header_was_before = true;
                            }
                        } else {
                            header_was_before = false;
                        }

                        current_iter = next_iter.clone();
                        if !list_store.iter_next(&next_iter) {
                            break;
                        }
                    }
                }

                // Last step, remove orphan header if exists
                if let Some(iter) = list_store.get_iter_first() {
                    if !list_store.iter_next(&iter) {
                        list_store.clear();
                    }
                }

                text_view_errors.get_buffer().unwrap().set_text(messages.as_str());
                selection.unselect_all();
            }
            "notebook_main_zeroed_files_finder" => {
                let tree_view = scrolled_window_zeroed_files_finder.get_children().get(0).unwrap().clone().downcast::<gtk::TreeView>().unwrap();
                let selection = tree_view.get_selection();

                let (selection_rows, tree_model) = selection.get_selected_rows();
                if selection_rows.is_empty() {
                    return;
                }
                let list_store = tree_model.clone().downcast::<gtk::ListStore>().unwrap();

                // let new_tree_model = TreeModel::new(); // TODO - maybe create new model when inserting a new data, because this seems to be not optimal when using thousands of rows

                let mut messages: String = "".to_string();

                // Must be deleted from end to start, because when deleting entries, TreePath(and also TreeIter) will points to invalid data
                for tree_path in selection_rows.iter().rev() {
                    let name = tree_model.get_value(&tree_model.get_iter(tree_path).unwrap(), ColumnsZeroedFiles::Name as i32).get::<String>().unwrap().unwrap();
                    let path = tree_model.get_value(&tree_model.get_iter(tree_path).unwrap(), ColumnsZeroedFiles::Path as i32).get::<String>().unwrap().unwrap();

                    match fs::remove_file(format!("{}/{}", path, name)) {
                        Ok(_) => {
                            list_store.remove(&list_store.get_iter(tree_path).unwrap());
                        }
                        Err(_) => messages += format!("Failed to remove file {}/{} because file doesn't exists or you don't have permissions.\n", path, name).as_str(),
                    }
                }

                text_view_errors.get_buffer().unwrap().set_text(messages.as_str());
                selection.unselect_all();
            }
            e => panic!("Not existent {}", e),
        }
    });
}
