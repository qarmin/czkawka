use std::fs;
use std::path::PathBuf;

use gtk4::prelude::*;
use gtk4::{Align, CheckButton, Dialog, Orientation, ResponseType, TextView, TreeIter, TreePath};

use czkawka_core::duplicate::make_hard_link;

use crate::flg;
use crate::gui_structs::gui_data::GuiData;
use crate::help_functions::*;
use crate::notebook_enums::*;
use crate::notebook_info::NOTEBOOKS_INFO;

#[derive(PartialEq, Eq, Copy, Clone)]
enum TypeOfTool {
    Hardlinking,
    Symlinking,
}

#[derive(Debug)]
struct SymHardlinkData {
    original_data: String,
    files_to_symhardlink: Vec<String>,
}

pub fn connect_button_hardlink_symlink(gui_data: &GuiData) {
    // Hardlinking
    {
        let buttons_hardlink = gui_data.bottom_buttons.buttons_hardlink.clone();

        let gui_data = gui_data.clone();

        buttons_hardlink.connect_clicked(move |_| {
            glib::MainContext::default().spawn_local(sym_hard_link_things(gui_data.clone(), TypeOfTool::Hardlinking));
        });
    }

    // Symlinking
    {
        let buttons_symlink = gui_data.bottom_buttons.buttons_symlink.clone();

        let gui_data = gui_data.clone();

        buttons_symlink.connect_clicked(move |_| {
            glib::MainContext::default().spawn_local(sym_hard_link_things(gui_data.clone(), TypeOfTool::Symlinking));
        });
    }
}

async fn sym_hard_link_things(gui_data: GuiData, hardlinking: TypeOfTool) {
    let notebook_main = gui_data.main_notebook.notebook_main.clone();
    let main_tree_views = gui_data.main_notebook.get_main_tree_views();

    let image_preview_similar_images = gui_data.main_notebook.image_preview_similar_images.clone();
    let image_preview_duplicates = gui_data.main_notebook.image_preview_duplicates.clone();

    let text_view_errors = gui_data.text_view_errors.clone();
    let preview_path = gui_data.preview_path.clone();
    let window_main = gui_data.window_main.clone();

    let nb_number = notebook_main.current_page().expect("Current page not set");
    let tree_view = &main_tree_views[nb_number as usize];
    let nb_object = &NOTEBOOKS_INFO[nb_number as usize];

    let column_header = nb_object.column_header.expect("Linking can be only used for tree views with grouped results");

    let check_button_settings_confirm_link = gui_data.settings.check_button_settings_confirm_link.clone();

    if !check_if_anything_is_selected_async(tree_view, column_header, nb_object.column_selection) {
        return;
    }

    if !check_if_can_link_files(&check_button_settings_confirm_link, &window_main).await {
        return;
    }

    if !check_if_changing_one_item_in_group_and_continue(tree_view, column_header, nb_object.column_selection, &window_main).await {
        return;
    }

    hardlink_symlink(
        tree_view,
        nb_object.column_name,
        nb_object.column_path,
        column_header,
        nb_object.column_selection,
        hardlinking,
        &text_view_errors,
    );

    match &nb_object.notebook_type {
        NotebookMainEnum::SimilarImages | NotebookMainEnum::Duplicate => {
            if nb_object.notebook_type == NotebookMainEnum::SimilarImages {
                image_preview_similar_images.hide();
            } else {
                image_preview_duplicates.hide();
            }
            *preview_path.borrow_mut() = String::new();
        }
        _ => {}
    }
}

fn hardlink_symlink(
    tree_view: &gtk4::TreeView,
    column_file_name: i32,
    column_path: i32,
    column_header: i32,
    column_selection: i32,
    hardlinking: TypeOfTool,
    text_view_errors: &TextView,
) {
    reset_text_view(text_view_errors);

    let model = get_list_store(tree_view);

    let mut vec_tree_path_to_remove: Vec<TreePath> = Vec::new(); // List of hardlinked files without its root
    let mut vec_symhardlink_data: Vec<SymHardlinkData> = Vec::new();

    let current_iter: TreeIter = match model.iter_first() {
        Some(t) => t,
        None => return, // No records
    };

    let mut selected_rows = Vec::new();
    if let Some(iter) = model.iter_first() {
        loop {
            if model.get::<bool>(&iter, column_selection) {
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

    let mut current_symhardlink_data: Option<SymHardlinkData> = None;
    let mut current_selected_index = 0;
    loop {
        if model.get::<bool>(&current_iter, column_header) {
            if let Some(current_symhardlink_data) = current_symhardlink_data {
                if !current_symhardlink_data.files_to_symhardlink.is_empty() {
                    vec_symhardlink_data.push(current_symhardlink_data);
                }
            }

            current_symhardlink_data = None;
            assert!(model.iter_next(&current_iter), "HEADER, shouldn't be a last item.");
            continue;
        }

        if model.path(&current_iter) == selected_rows[current_selected_index] {
            let file_name = model.get::<String>(&current_iter, column_file_name);
            let path = model.get::<String>(&current_iter, column_path);
            let full_file_path = get_full_name_from_path_name(&path, &file_name);

            if let Some(mut current_data) = current_symhardlink_data {
                vec_tree_path_to_remove.push(model.path(&current_iter));
                current_data.files_to_symhardlink.push(full_file_path);
                current_symhardlink_data = Some(current_data);
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
    if hardlinking == TypeOfTool::Hardlinking {
        for symhardlink_data in vec_symhardlink_data {
            for file_to_hardlink in symhardlink_data.files_to_symhardlink {
                if let Err(e) = make_hard_link(&PathBuf::from(&symhardlink_data.original_data), &PathBuf::from(&file_to_hardlink)) {
                    add_text_to_text_view(text_view_errors, format!("{} {}, reason {}", flg!("hardlink_failed"), file_to_hardlink, e).as_str());
                    continue;
                }
            }
        }
    } else {
        for symhardlink_data in vec_symhardlink_data {
            for file_to_symlink in symhardlink_data.files_to_symhardlink {
                if let Err(e) = fs::remove_file(&file_to_symlink) {
                    add_text_to_text_view(text_view_errors, flg!("delete_file_failed", name = file_to_symlink, reason = e.to_string()).as_str());
                    continue;
                };

                #[cfg(target_family = "unix")]
                {
                    if let Err(e) = std::os::unix::fs::symlink(&symhardlink_data.original_data, &file_to_symlink) {
                        add_text_to_text_view(text_view_errors, flg!("delete_file_failed", name = file_to_symlink, reason = e.to_string()).as_str());
                        continue;
                    };
                }
                #[cfg(target_family = "windows")]
                {
                    if let Err(e) = std::os::windows::fs::symlink_file(&symhardlink_data.original_data, &file_to_symlink) {
                        add_text_to_text_view(text_view_errors, flg!("delete_file_failed", name = file_to_symlink, reason = e.to_string()).as_str());
                        continue;
                    };
                }
            }
        }
    }
    for tree_path in vec_tree_path_to_remove.iter().rev() {
        model.remove(&model.iter(tree_path).expect("Using invalid tree_path"));
    }

    clean_invalid_headers(&model, column_header, column_path);
}

fn create_dialog_non_group(window_main: &gtk4::Window) -> Dialog {
    let dialog = Dialog::builder()
        .title(flg!("hard_sym_invalid_selection_title_dialog"))
        .transient_for(window_main)
        .modal(true)
        .build();
    let button_ok = dialog.add_button(&flg!("general_ok_button"), ResponseType::Ok);
    dialog.add_button(&flg!("general_close_button"), ResponseType::Cancel);

    let label: gtk4::Label = gtk4::Label::new(Some(&flg!("hard_sym_invalid_selection_label_1")));
    let label2: gtk4::Label = gtk4::Label::new(Some(&flg!("hard_sym_invalid_selection_label_2")));
    let label3: gtk4::Label = gtk4::Label::new(Some(&flg!("hard_sym_invalid_selection_label_3")));

    button_ok.grab_focus();

    let parent = button_ok.parent().expect("Hack 1").parent().expect("Hack 2").downcast::<gtk4::Box>().expect("Hack 3"); // TODO Hack, but not so ugly as before
    parent.set_orientation(Orientation::Vertical);
    parent.insert_child_after(&label, None::<&gtk4::Widget>);
    parent.insert_child_after(&label2, Some(&label));
    parent.insert_child_after(&label3, Some(&label2));

    dialog.show();
    dialog
}

pub async fn check_if_changing_one_item_in_group_and_continue(tree_view: &gtk4::TreeView, column_header: i32, column_selection: i32, window_main: &gtk4::Window) -> bool {
    let model = get_list_store(tree_view);

    let mut selected_values_in_group = 0;

    if let Some(iter) = model.iter_first() {
        assert!(model.get::<bool>(&iter, column_header)); // First element should be header

        loop {
            if !model.iter_next(&iter) {
                break;
            }

            if model.get::<bool>(&iter, column_header) {
                if selected_values_in_group == 1 {
                    break;
                }
                selected_values_in_group = 0;
            } else {
                if model.get::<bool>(&iter, column_selection) {
                    selected_values_in_group += 1;
                }
            }
        }
    } else {
        return false; // No available records
    }

    if selected_values_in_group == 1 {
        let confirmation_dialog = create_dialog_non_group(window_main);

        let response_type = confirmation_dialog.run_future().await;
        if response_type != ResponseType::Ok {
            confirmation_dialog.hide();
            confirmation_dialog.close();
            return false;
        }
        confirmation_dialog.hide();
        confirmation_dialog.close();
    }

    true
}

pub fn check_if_anything_is_selected_async(tree_view: &gtk4::TreeView, column_header: i32, column_selection: i32) -> bool {
    let model = get_list_store(tree_view);

    if let Some(iter) = model.iter_first() {
        assert!(model.get::<bool>(&iter, column_header)); // First element should be header

        loop {
            if !model.iter_next(&iter) {
                break;
            }

            if !model.get::<bool>(&iter, column_header) && model.get::<bool>(&iter, column_selection) {
                return true;
            }
        }
    }

    false
}

pub async fn check_if_can_link_files(check_button_settings_confirm_link: &CheckButton, window_main: &gtk4::Window) -> bool {
    if check_button_settings_confirm_link.is_active() {
        let (confirmation_dialog_link, check_button) = create_dialog_ask_for_linking(window_main);

        let response_type = confirmation_dialog_link.run_future().await;
        if response_type == ResponseType::Ok {
            if !check_button.is_active() {
                check_button_settings_confirm_link.set_active(false);
            }
            confirmation_dialog_link.hide();
            confirmation_dialog_link.close();
        } else {
            confirmation_dialog_link.hide();
            confirmation_dialog_link.close();
            return false;
        };
    }
    true
}

fn create_dialog_ask_for_linking(window_main: &gtk4::Window) -> (Dialog, CheckButton) {
    let dialog = Dialog::builder().title(flg!("hard_sym_link_title_dialog")).transient_for(window_main).modal(true).build();
    let button_ok = dialog.add_button(&flg!("general_ok_button"), ResponseType::Ok);
    dialog.add_button(&flg!("general_close_button"), ResponseType::Cancel);

    let label: gtk4::Label = gtk4::Label::new(Some(&flg!("hard_sym_link_label")));
    let check_button: CheckButton = CheckButton::builder().label(flg!("dialogs_ask_next_time")).active(true).halign(Align::Center).build();

    button_ok.grab_focus();

    let parent = button_ok.parent().expect("Hack 1").parent().expect("Hack 2").downcast::<gtk4::Box>().expect("Hack 3"); // TODO Hack, but not so ugly as before
    parent.set_orientation(Orientation::Vertical);
    parent.insert_child_after(&label, None::<&gtk4::Widget>);
    parent.insert_child_after(&check_button, Some(&label));

    dialog.show();
    (dialog, check_button)
}
