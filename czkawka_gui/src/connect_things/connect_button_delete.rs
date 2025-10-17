use std::collections::BTreeMap;
use std::fs;

use czkawka_core::common::check_if_folder_contains_only_empty_folders;
use gtk4::prelude::*;
use gtk4::{Align, CheckButton, Dialog, Orientation, ResponseType, TextView};
use log::debug;

use crate::flg;
use crate::gui_structs::common_tree_view::SubView;
use crate::gui_structs::gui_data::GuiData;
use crate::help_functions::{check_how_much_elements_is_selected, clean_invalid_headers, get_full_name_from_path_name, get_list_store};
use crate::notebook_enums::NotebookMainEnum;

// TODO add support for checking if really symlink doesn't point to correct directory/file

pub(crate) fn connect_button_delete(gui_data: &GuiData) {
    let buttons_delete = gui_data.bottom_buttons.buttons_delete.clone();

    let gui_data = gui_data.clone(); // TODO this maybe can be replaced, not sure if worth to clone everything

    buttons_delete.connect_clicked(move |_| {
        glib::MainContext::default().spawn_local(delete_things(gui_data.clone()));
    });
}

pub async fn delete_things(gui_data: GuiData) {
    let window_main = gui_data.window_main.clone();
    let check_button_settings_confirm_deletion = gui_data.settings.check_button_settings_confirm_deletion.clone();
    let check_button_settings_confirm_group_deletion = gui_data.settings.check_button_settings_confirm_group_deletion.clone();
    let image_preview_similar_images = gui_data.main_notebook.image_preview_similar_images.clone();
    let image_preview_duplicates = gui_data.main_notebook.image_preview_duplicates.clone();

    let check_button_settings_use_trash = gui_data.settings.check_button_settings_use_trash.clone();

    let preview_path = gui_data.main_notebook.common_tree_views.preview_path.clone();

    let text_view_errors = gui_data.text_view_errors.clone();

    let sv = gui_data.main_notebook.common_tree_views.get_current_subview();

    let (number_of_selected_items, number_of_selected_groups) = check_how_much_elements_is_selected(&sv.tree_view, sv.nb_object.column_header, sv.nb_object.column_selection);

    // Nothing is selected
    if number_of_selected_items == 0 {
        return;
    }

    if !check_if_can_delete_files(&check_button_settings_confirm_deletion, &window_main, number_of_selected_items, number_of_selected_groups).await {
        return;
    }

    if let Some(column_header) = sv.nb_object.column_header {
        if !check_button_settings_confirm_group_deletion.is_active()
            || !check_if_deleting_all_files_in_group(
                &sv.tree_view,
                column_header,
                sv.nb_object.column_selection,
                sv.nb_object.column_path,
                &window_main,
                &check_button_settings_confirm_group_deletion,
            )
            .await
        {
            tree_remove(sv, column_header, &check_button_settings_use_trash, &text_view_errors);
        }
    } else if sv.nb_object.notebook_type == NotebookMainEnum::EmptyDirectories {
        empty_folder_remover(sv, &check_button_settings_use_trash, &text_view_errors);
    } else {
        basic_remove(sv, &check_button_settings_use_trash, &text_view_errors);
    }

    match &sv.nb_object.notebook_type {
        NotebookMainEnum::SimilarImages | NotebookMainEnum::Duplicate => {
            if sv.nb_object.notebook_type == NotebookMainEnum::SimilarImages {
                image_preview_similar_images.hide();
            } else {
                image_preview_duplicates.hide();
            }
            *preview_path.borrow_mut() = String::new();
        }
        _ => {}
    }
}

pub async fn check_if_can_delete_files(
    check_button_settings_confirm_deletion: &CheckButton,
    window_main: &gtk4::Window,
    number_of_selected_items: u64,
    number_of_selected_groups: u64,
) -> bool {
    if check_button_settings_confirm_deletion.is_active() {
        let (confirmation_dialog_delete, check_button) = create_dialog_ask_for_deletion(window_main, number_of_selected_items, number_of_selected_groups);

        let response_type = confirmation_dialog_delete.run_future().await;
        if response_type == ResponseType::Ok {
            if !check_button.is_active() {
                check_button_settings_confirm_deletion.set_active(false);
            }
            confirmation_dialog_delete.hide();
            confirmation_dialog_delete.close();
        } else {
            confirmation_dialog_delete.hide();
            confirmation_dialog_delete.close();
            return false;
        }
    }
    true
}

fn create_dialog_ask_for_deletion(window_main: &gtk4::Window, number_of_selected_items: u64, number_of_selected_groups: u64) -> (Dialog, CheckButton) {
    let dialog = Dialog::builder().title(flg!("delete_title_dialog")).transient_for(window_main).modal(true).build();
    let button_ok = dialog.add_button(&flg!("general_ok_button"), ResponseType::Ok);
    dialog.add_button(&flg!("general_close_button"), ResponseType::Cancel);

    dialog.set_default_size(300, 0);

    let label: gtk4::Label = gtk4::Label::new(Some(&flg!("delete_question_label")));
    let label2: gtk4::Label = match number_of_selected_groups {
        0 => gtk4::Label::new(Some(&flg!("delete_items_label", items = number_of_selected_items))),
        _ => gtk4::Label::new(Some(&flg!(
            "delete_items_groups_label",
            items = number_of_selected_items,
            groups = number_of_selected_groups
        ))),
    };

    let check_button: CheckButton = CheckButton::builder()
        .label(flg!("dialogs_ask_next_time"))
        .active(true)
        .halign(Align::Center)
        .margin_top(5)
        .build();

    button_ok.grab_focus();

    let parent = button_ok.parent().expect("Hack 1").parent().expect("Hack 2").downcast::<gtk4::Box>().expect("Hack 3"); // TODO Hack, but not so ugly as before
    parent.set_orientation(Orientation::Vertical);
    parent.insert_child_after(&label, None::<&gtk4::Widget>);
    parent.insert_child_after(&label2, Some(&label));
    parent.insert_child_after(&check_button, Some(&label2));

    dialog.show();
    (dialog, check_button)
}

fn create_dialog_group_deletion(window_main: &gtk4::Window) -> (Dialog, CheckButton) {
    let dialog = Dialog::builder()
        .title(flg!("delete_all_files_in_group_title"))
        .transient_for(window_main)
        .modal(true)
        .build();
    let button_ok = dialog.add_button(&flg!("general_ok_button"), ResponseType::Ok);
    dialog.add_button(&flg!("general_close_button"), ResponseType::Cancel);

    let label: gtk4::Label = gtk4::Label::new(Some(&flg!("delete_all_files_in_group_label1")));
    let label2: gtk4::Label = gtk4::Label::new(Some(&flg!("delete_all_files_in_group_label2")));
    let check_button: CheckButton = CheckButton::builder().label(flg!("dialogs_ask_next_time")).active(true).halign(Align::Center).build();

    button_ok.grab_focus();

    let parent = button_ok.parent().expect("Hack 1").parent().expect("Hack 2").downcast::<gtk4::Box>().expect("Hack 3"); // TODO Hack, but not so ugly as before
    parent.set_orientation(Orientation::Vertical);
    parent.insert_child_after(&label, None::<&gtk4::Widget>);
    parent.insert_child_after(&label2, Some(&label));
    parent.insert_child_after(&check_button, Some(&label2));

    dialog.show();
    (dialog, check_button)
}

pub async fn check_if_deleting_all_files_in_group(
    tree_view: &gtk4::TreeView,
    column_header: i32,
    column_selection: i32,
    column_path: i32,
    window_main: &gtk4::Window,
    check_button_settings_confirm_group_deletion: &CheckButton,
) -> bool {
    let model = get_list_store(tree_view);

    let mut selected_all_records: bool = true;

    if let Some(iter) = model.iter_first() {
        assert!(model.get::<bool>(&iter, column_header)); // First element should be header

        // It is safe to remove any number of files in reference mode
        if !model.get::<String>(&iter, column_path).is_empty() {
            return false;
        }

        loop {
            if !model.iter_next(&iter) {
                break;
            }

            if model.get::<bool>(&iter, column_header) {
                if selected_all_records {
                    break;
                }
                selected_all_records = true;
            } else if !model.get::<bool>(&iter, column_selection) {
                selected_all_records = false;
            }
        }
    } else {
        return false;
    }

    if !selected_all_records {
        return false;
    }

    let (confirmation_dialog_group_delete, check_button) = create_dialog_group_deletion(window_main);

    let response_type = confirmation_dialog_group_delete.run_future().await;
    if response_type == ResponseType::Ok {
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

    false
}

pub(crate) fn empty_folder_remover(sv: &SubView, check_button_settings_use_trash: &CheckButton, text_view_errors: &TextView) {
    let use_trash = check_button_settings_use_trash.is_active();

    let model = sv.get_model();

    let mut selected_rows = Vec::new();

    if let Some(iter) = model.iter_first() {
        loop {
            if model.get::<bool>(&iter, sv.nb_object.column_selection) {
                selected_rows.push(model.path(&iter));
            }
            if !model.iter_next(&iter) {
                break;
            }
        }
    }

    if selected_rows.is_empty() {
        return; // No selected rows
    }

    debug!("Starting to delete {} folders", selected_rows.len());
    let start_time = std::time::Instant::now();
    let mut deleted_folders: u32 = 0;

    let mut messages: String = String::new();

    // Must be deleted from end to start, because when deleting entries, TreePath(and also TreeIter) will points to invalid data
    for tree_path in selected_rows.iter().rev() {
        let iter = model.iter(tree_path).expect("Using invalid tree_path");

        let name = model.get::<String>(&iter, sv.nb_object.column_name);
        let path = model.get::<String>(&iter, sv.nb_object.column_path);
        let full_path = get_full_name_from_path_name(&path, &name);

        // We must check if folder is really empty or contains only other empty folders
        let mut error_happened = check_if_folder_contains_only_empty_folders(&full_path).is_err();

        if !error_happened {
            if !use_trash {
                match fs::remove_dir_all(&full_path) {
                    Ok(()) => {
                        model.remove(&iter);
                        deleted_folders += 1;
                    }
                    Err(_inspected) => error_happened = true,
                }
            } else {
                match trash::delete(&full_path) {
                    Ok(()) => {
                        model.remove(&iter);
                        deleted_folders += 1;
                    }
                    Err(_inspected) => error_happened = true,
                }
            }
        }

        // This could be changed to add more specific error message, what exactly happened
        if error_happened {
            messages += &flg!("delete_folder_failed", dir = full_path);
            messages += "\n";
        }
    }

    debug!("Deleted {deleted_folders} folders in {:?}", start_time.elapsed());

    text_view_errors.buffer().set_text(messages.as_str());
}

pub(crate) fn basic_remove(sv: &SubView, check_button_settings_use_trash: &CheckButton, text_view_errors: &TextView) {
    let use_trash = check_button_settings_use_trash.is_active();

    let model = sv.get_model();

    let mut messages: String = String::new();

    let mut selected_rows = Vec::new();

    if let Some(iter) = model.iter_first() {
        loop {
            if model.get::<bool>(&iter, sv.nb_object.column_selection) {
                selected_rows.push(model.path(&iter));
            }

            if !model.iter_next(&iter) {
                break;
            }
        }
    }

    if selected_rows.is_empty() {
        return; // No selected rows
    }

    debug!("Starting to delete {} files", selected_rows.len());
    let start_time = std::time::Instant::now();
    let mut deleted_files: u32 = 0;

    // Must be deleted from end to start, because when deleting entries, TreePath(and also TreeIter) will points to invalid data
    for tree_path in selected_rows.iter().rev() {
        let iter = model.iter(tree_path).expect("Using invalid tree_path");

        let name = model.get::<String>(&iter, sv.nb_object.column_name);
        let path = model.get::<String>(&iter, sv.nb_object.column_path);

        if !use_trash {
            match fs::remove_file(get_full_name_from_path_name(&path, &name)) {
                Ok(()) => {
                    model.remove(&iter);
                    deleted_files += 1;
                }

                Err(e) => {
                    messages += flg!("delete_file_failed", name = get_full_name_from_path_name(&path, &name), reason = e.to_string()).as_str();
                    messages += "\n";
                }
            }
        } else {
            match trash::delete(get_full_name_from_path_name(&path, &name)) {
                Ok(()) => {
                    model.remove(&iter);
                    deleted_files += 1;
                }
                Err(e) => {
                    messages += flg!("delete_file_failed", name = get_full_name_from_path_name(&path, &name), reason = e.to_string()).as_str();
                    messages += "\n";
                }
            }
        }
    }

    debug!("Deleted {deleted_files} files in {:?}", start_time.elapsed());

    text_view_errors.buffer().set_text(messages.as_str());
}

// Remove all occurrences - remove every element which have same path and name as even non selected ones
pub(crate) fn tree_remove(sv: &SubView, column_header: i32, check_button_settings_use_trash: &CheckButton, text_view_errors: &TextView) {
    let use_trash = check_button_settings_use_trash.is_active();

    let model = sv.get_model();

    let mut messages: String = String::new();

    // TODO - looks like a but - this var is not deleted
    #[expect(clippy::collection_is_never_read)]
    let mut vec_path_to_delete: Vec<(String, String)> = Vec::new();
    let mut map_with_path_to_delete: BTreeMap<String, Vec<String>> = Default::default(); // BTreeMap<Path,Vec<FileName>>

    let mut selected_rows = Vec::new();

    if let Some(iter) = model.iter_first() {
        loop {
            if model.get::<bool>(&iter, sv.nb_object.column_selection) {
                if !model.get::<bool>(&iter, column_header) {
                    selected_rows.push(model.path(&iter));
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
        let iter = model.iter(tree_path).expect("Using invalid tree_path");

        let file_name = model.get::<String>(&iter, sv.nb_object.column_name);
        let path = model.get::<String>(&iter, sv.nb_object.column_path);

        model.remove(&iter);

        map_with_path_to_delete.entry(path.clone()).or_default().push(file_name);
    }

    // Delete duplicated entries, and remove real files
    for (path, mut vec_file_name) in map_with_path_to_delete {
        vec_file_name.sort_unstable();
        vec_file_name.dedup();
        for file_name in vec_file_name {
            if !use_trash {
                if let Err(e) = fs::remove_file(get_full_name_from_path_name(&path, &file_name)) {
                    messages += flg!("delete_file_failed", name = get_full_name_from_path_name(&path, &file_name), reason = e.to_string()).as_str();
                    messages += "\n";
                }
            } else if let Err(e) = trash::delete(get_full_name_from_path_name(&path, &file_name)) {
                messages += flg!("delete_file_failed", name = get_full_name_from_path_name(&path, &file_name), reason = e.to_string()).as_str();
                messages += "\n";
            }

            vec_path_to_delete.push((path.clone(), file_name.clone()));
        }
    }

    clean_invalid_headers(&model, column_header, sv.nb_object.column_path);

    text_view_errors.buffer().set_text(messages.as_str());
}
