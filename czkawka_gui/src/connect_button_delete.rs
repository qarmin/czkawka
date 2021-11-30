use std::collections::BTreeMap;
use std::fs;
use std::fs::Metadata;

use gtk::prelude::*;
use gtk::{Align, CheckButton, Dialog, ResponseType, TextView};

use crate::gui_data::GuiData;
use crate::help_functions::*;
use crate::notebook_enums::*;
use crate::validate_notebook_data;

// TODO add support for checking if really symlink doesn't point to correct directory/file

pub fn connect_button_delete(gui_data: &GuiData) {
    let buttons_delete = gui_data.bottom_buttons.buttons_delete.clone();

    let gui_data = gui_data.clone(); // TODO this maybe can be replaced, not sure if worth to do it

    buttons_delete.connect_clicked(move |_| {
        glib::MainContext::default().spawn_local(delete_things(gui_data.clone()));
    });
}

pub async fn delete_things(gui_data: GuiData) {
    validate_notebook_data(&gui_data); // TODO, disable this - only used as test if ever

    let notebook_main = gui_data.main_notebook.notebook_main.clone();
    let window_main = gui_data.window_main.clone();
    let check_button_settings_confirm_deletion = gui_data.settings.check_button_settings_confirm_deletion.clone();
    let check_button_settings_confirm_group_deletion = gui_data.settings.check_button_settings_confirm_group_deletion.clone();
    let image_preview_similar_images = gui_data.main_notebook.image_preview_similar_images.clone();
    let image_preview_duplicates = gui_data.main_notebook.image_preview_duplicates.clone();

    let main_tree_views = gui_data.main_notebook.get_main_tree_views();

    let check_button_settings_use_trash = gui_data.settings.check_button_settings_use_trash.clone();

    let text_view_errors = gui_data.text_view_errors.clone();
    if !check_if_can_delete_files(&check_button_settings_confirm_deletion, &window_main).await {
        return;
    }

    let nb_number = notebook_main.current_page().unwrap();
    let tree_view = &main_tree_views[nb_number as usize];
    let nb_object = &NOTEBOOKS_INFOS[nb_number as usize];

    if let Some(column_color) = nb_object.column_color {
        if !check_button_settings_confirm_group_deletion.is_active() || !check_if_deleting_all_files_in_group(tree_view, column_color, nb_object.column_selection, &window_main, &check_button_settings_confirm_group_deletion).await {
            tree_remove(
                &tree_view.clone(),
                nb_object.column_name,
                nb_object.column_path,
                column_color,
                nb_object.column_selection,
                &check_button_settings_use_trash,
                &text_view_errors,
            );
        }
    } else {
        if nb_number == NotebookMainEnum::EmptyDirectories as u32 {
            empty_folder_remover(&tree_view.clone(), nb_object.column_name, nb_object.column_path, nb_object.column_selection, &check_button_settings_use_trash, &text_view_errors);
        } else {
            basic_remove(&tree_view.clone(), nb_object.column_name, nb_object.column_path, nb_object.column_selection, &check_button_settings_use_trash, &text_view_errors);
        }
    }

    match &nb_object.notebook_type {
        NotebookMainEnum::SimilarImages => {
            image_preview_similar_images.hide();
        }
        NotebookMainEnum::Duplicate => {
            image_preview_duplicates.hide();
        }
        _ => {}
    }
}

pub async fn check_if_can_delete_files(check_button_settings_confirm_deletion: &gtk::CheckButton, window_main: &gtk::Window) -> bool {
    if check_button_settings_confirm_deletion.is_active() {
        let (confirmation_dialog_delete, check_button) = create_dialog_ask_for_deletion(window_main);

        let response_type = confirmation_dialog_delete.run_future().await;
        if response_type == gtk::ResponseType::Ok {
            if !check_button.is_active() {
                check_button_settings_confirm_deletion.set_active(false);
            }
            confirmation_dialog_delete.hide();
            confirmation_dialog_delete.close();
        } else {
            confirmation_dialog_delete.hide();
            confirmation_dialog_delete.close();
            return false;
        };
    }
    true
}

fn create_dialog_ask_for_deletion(_window_main: &gtk::Window) -> (Dialog, CheckButton) {
    let confirmation_dialog_delete = gtk::Dialog::builder().title("Delete confirmation").build();
    let button_ok = confirmation_dialog_delete.add_button("Ok", ResponseType::Ok);
    confirmation_dialog_delete.add_button("Close", ResponseType::Cancel);

    let label: gtk::Label = gtk::Label::new(Some("Are you sure that you want to delete files?"));
    let check_button: gtk::CheckButton = gtk::CheckButton::with_label("Ask next time");
    check_button.set_active(true);
    check_button.set_halign(Align::Center);

    button_ok.grab_focus();

    let internal_box = get_dialog_box_child(&confirmation_dialog_delete);
    internal_box.add(&label);
    internal_box.add(&check_button);

    confirmation_dialog_delete.show_all();
    (confirmation_dialog_delete, check_button)
}

fn create_dialog_group_deletion(_window_main: &gtk::Window) -> (Dialog, CheckButton) {
    let confirmation_dialog_group_delete = gtk::Dialog::builder().title("Confirmation of deleting all files in group").build();
    let button_ok = confirmation_dialog_group_delete.add_button("Ok", ResponseType::Ok);
    confirmation_dialog_group_delete.add_button("Close", ResponseType::Cancel);

    let label: gtk::Label = gtk::Label::new(Some("In some groups there are selected all records."));
    let label2: gtk::Label = gtk::Label::new(Some("Are you sure that you want to delete them?"));
    let check_button: gtk::CheckButton = gtk::CheckButton::with_label("Ask next time");
    check_button.set_active(true);
    check_button.set_halign(Align::Center);

    button_ok.grab_focus();

    let internal_box = get_dialog_box_child(&confirmation_dialog_group_delete);
    internal_box.add(&label);
    internal_box.add(&label2);
    internal_box.add(&check_button);

    confirmation_dialog_group_delete.show_all();
    (confirmation_dialog_group_delete, check_button)
}

pub async fn check_if_deleting_all_files_in_group(tree_view: &gtk::TreeView, column_color: i32, column_selection: i32, window_main: &gtk::Window, check_button_settings_confirm_group_deletion: &gtk::CheckButton) -> bool {
    let model = get_list_store(tree_view);

    let mut selected_all_records: bool = true;

    if let Some(iter) = model.iter_first() {
        assert_eq!(model.value(&iter, column_color).get::<String>().unwrap(), HEADER_ROW_COLOR); // First element should be header

        loop {
            if !model.iter_next(&iter) {
                break;
            }

            if model.value(&iter, column_color).get::<String>().unwrap() == HEADER_ROW_COLOR {
                if selected_all_records {
                    break;
                }
                selected_all_records = true;
            } else {
                if !model.value(&iter, column_selection).get::<bool>().unwrap() {
                    selected_all_records = false;
                }
            }
        }
    } else {
        return false;
    }

    if !selected_all_records {
        return false;
    } else {
        let (confirmation_dialog_group_delete, check_button) = create_dialog_group_deletion(window_main);

        let response_type = confirmation_dialog_group_delete.run_future().await;
        if response_type == gtk::ResponseType::Ok {
            if !check_button.is_active() {
                check_button_settings_confirm_group_deletion.set_active(false);
            }
        } else {
            confirmation_dialog_group_delete.hide();
            confirmation_dialog_group_delete.close();
            return true;
        }
        confirmation_dialog_group_delete.hide();
        confirmation_dialog_group_delete.close();
    }

    false
}

pub fn empty_folder_remover(tree_view: &gtk::TreeView, column_file_name: i32, column_path: i32, column_selection: i32, check_button_settings_use_trash: &CheckButton, text_view_errors: &TextView) {
    let use_trash = check_button_settings_use_trash.is_active();

    let model = get_list_store(tree_view);

    let mut selected_rows = Vec::new();

    if let Some(iter) = model.iter_first() {
        loop {
            if model.value(&iter, column_selection).get::<bool>().unwrap() {
                selected_rows.push(model.path(&iter).unwrap());
            }
            if !model.iter_next(&iter) {
                break;
            }
        }
    }

    if selected_rows.is_empty() {
        return; // No selected rows
    }

    let mut messages: String = "".to_string();

    // Must be deleted from end to start, because when deleting entries, TreePath(and also TreeIter) will points to invalid data
    for tree_path in selected_rows.iter().rev() {
        let iter = model.iter(tree_path).unwrap();

        let name = model.value(&iter, column_file_name).get::<String>().unwrap();
        let path = model.value(&iter, column_path).get::<String>().unwrap();

        // We must check if folder is really empty or contains only other empty folders
        let mut error_happened = false;
        let mut folders_to_check: Vec<String> = vec![format!("{}/{}", path, name)];
        let mut current_folder: String;
        let mut next_folder: String;
        'dir: while !folders_to_check.is_empty() {
            current_folder = folders_to_check.pop().unwrap();
            let read_dir = match fs::read_dir(&current_folder) {
                Ok(t) => t,
                Err(_inspected) => {
                    error_happened = true;
                    break 'dir;
                }
            };

            for entry in read_dir {
                let entry_data = match entry {
                    Ok(t) => t,
                    Err(_inspected) => {
                        error_happened = true;
                        break 'dir;
                    }
                };
                let metadata: Metadata = match entry_data.metadata() {
                    Ok(t) => t,
                    Err(_inspected) => {
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
                            Err(_inspected) => {
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
            if !use_trash {
                match fs::remove_dir_all(format!("{}/{}", path, name)) {
                    Ok(_) => {
                        model.remove(&iter);
                    }
                    Err(_inspected) => error_happened = true,
                }
            } else {
                match trash::delete(format!("{}/{}", path, name)) {
                    Ok(_) => {
                        model.remove(&iter);
                    }
                    Err(_inspected) => error_happened = true,
                }
            }
        }
        if error_happened {
            messages += format!("Failed to remove folder {}/{} because folder doesn't exists, you don't have permissions or isn't empty.\n", path, name).as_str()
        }
    }

    text_view_errors.buffer().unwrap().set_text(messages.as_str());
}

pub fn basic_remove(tree_view: &gtk::TreeView, column_file_name: i32, column_path: i32, column_selection: i32, check_button_settings_use_trash: &CheckButton, text_view_errors: &TextView) {
    let use_trash = check_button_settings_use_trash.is_active();

    let model = get_list_store(tree_view);

    let mut messages: String = "".to_string();

    let mut selected_rows = Vec::new();

    if let Some(iter) = model.iter_first() {
        loop {
            if model.value(&iter, column_selection).get::<bool>().unwrap() {
                selected_rows.push(model.path(&iter).unwrap());
            }

            if !model.iter_next(&iter) {
                break;
            }
        }
    }

    if selected_rows.is_empty() {
        return; // No selected rows
    }

    // Must be deleted from end to start, because when deleting entries, TreePath(and also TreeIter) will points to invalid data
    for tree_path in selected_rows.iter().rev() {
        let iter = model.iter(tree_path).unwrap();

        let name = model.value(&iter, column_file_name).get::<String>().unwrap();
        let path = model.value(&iter, column_path).get::<String>().unwrap();

        if !use_trash {
            match fs::remove_file(format!("{}/{}", path, name)) {
                Ok(_) => {
                    model.remove(&iter);
                }
                Err(e) => messages += format!("Failed to remove file {}/{}, reason {}\n", path, name, e).as_str(),
            }
        } else {
            match trash::delete(format!("{}/{}", path, name)) {
                Ok(_) => {
                    model.remove(&iter);
                }
                Err(e) => messages += format!("Failed to remove file {}/{}, reason {}\n", path, name, e).as_str(),
            }
        }
    }

    text_view_errors.buffer().unwrap().set_text(messages.as_str());
}

// Remove all occurrences - remove every element which have same path and name as even non selected ones
pub fn tree_remove(tree_view: &gtk::TreeView, column_file_name: i32, column_path: i32, column_color: i32, column_selection: i32, check_button_settings_use_trash: &CheckButton, text_view_errors: &TextView) {
    let use_trash = check_button_settings_use_trash.is_active();

    let model = get_list_store(tree_view);

    let mut messages: String = "".to_string();

    let mut vec_path_to_delete: Vec<(String, String)> = Vec::new();
    let mut map_with_path_to_delete: BTreeMap<String, Vec<String>> = Default::default(); // BTreeMap<Path,Vec<FileName>>

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

    // Save to variable paths of files, and remove it when not removing all occurrences.
    for tree_path in selected_rows.iter().rev() {
        let iter = model.iter(tree_path).unwrap();

        let file_name = model.value(&iter, column_file_name).get::<String>().unwrap();
        let path = model.value(&iter, column_path).get::<String>().unwrap();

        model.remove(&iter);

        map_with_path_to_delete.entry(path.clone()).or_insert_with(Vec::new);
        map_with_path_to_delete.get_mut(path.as_str()).unwrap().push(file_name);
    }

    // Delete duplicated entries, and remove real files
    for (path, mut vec_file_name) in map_with_path_to_delete {
        vec_file_name.sort();
        vec_file_name.dedup();
        for file_name in vec_file_name {
            if !use_trash {
                if let Err(e) = fs::remove_file(format!("{}/{}", path.clone(), file_name.clone())) {
                    messages += format!("Failed to remove file {}/{}, reason {}\n", path, file_name, e).as_str()
                }
            } else if let Err(e) = trash::delete(format!("{}/{}", path.clone(), file_name.clone())) {
                messages += format!("Failed to remove file {}/{}, reason {}\n", path, file_name, e).as_str()
            }

            vec_path_to_delete.push((path.clone(), file_name.clone()));
        }
    }

    clean_invalid_headers(&model, column_color);

    text_view_errors.buffer().unwrap().set_text(messages.as_str());
}
