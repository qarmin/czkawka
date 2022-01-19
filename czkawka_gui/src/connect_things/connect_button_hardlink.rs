use std::fs;
use std::path::PathBuf;

use gtk::prelude::*;
use gtk::{Align, CheckButton, Dialog, ResponseType, TextView, TreeIter, TreePath};

use crate::flg;
use czkawka_core::duplicate::make_hard_link;

use crate::gui_structs::gui_data::GuiData;
use crate::help_functions::*;
use crate::localizer_core::generate_translation_hashmap;
use crate::notebook_enums::*;

pub fn connect_button_hardlink_symlink(gui_data: &GuiData) {
    // Hardlinking
    {
        let buttons_hardlink = gui_data.bottom_buttons.buttons_hardlink.clone();

        let gui_data = gui_data.clone();

        buttons_hardlink.connect_clicked(move |_| {
            glib::MainContext::default().spawn_local(sym_hard_link_things(gui_data.clone(), true));
        });
    }

    // Symlinking
    {
        let buttons_symlink = gui_data.bottom_buttons.buttons_symlink.clone();

        let gui_data = gui_data.clone();

        buttons_symlink.connect_clicked(move |_| {
            glib::MainContext::default().spawn_local(sym_hard_link_things(gui_data.clone(), false));
        });
    }
}

pub async fn sym_hard_link_things(gui_data: GuiData, hardlinking: bool) {
    let notebook_main = gui_data.main_notebook.notebook_main.clone();
    let main_tree_views = gui_data.main_notebook.get_main_tree_views();

    let image_preview_similar_images = gui_data.main_notebook.image_preview_similar_images.clone();
    let image_preview_duplicates = gui_data.main_notebook.image_preview_duplicates.clone();

    let text_view_errors = gui_data.text_view_errors.clone();
    let preview_path = gui_data.preview_path.clone();
    let window_main = gui_data.window_main.clone();

    let nb_number = notebook_main.current_page().unwrap();
    let tree_view = &main_tree_views[nb_number as usize];
    let nb_object = &NOTEBOOKS_INFOS[nb_number as usize];

    let column_color = nb_object.column_color.expect("Linking can be only used for tree views with grouped results");

    let check_button_settings_confirm_link = gui_data.settings.check_button_settings_confirm_link.clone();

    if !check_if_anything_is_selected_async(tree_view, column_color, nb_object.column_selection).await {
        return;
    }

    if !check_if_can_link_files(&check_button_settings_confirm_link, &window_main).await {
        return;
    }

    if !check_if_changing_one_item_in_group_and_continue(tree_view, column_color, nb_object.column_selection, &window_main).await {
        return;
    }

    hardlink_symlink(
        tree_view,
        nb_object.column_name,
        nb_object.column_path,
        column_color,
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
            *preview_path.borrow_mut() = "".to_string();
        }
        _ => {}
    }
}

pub fn hardlink_symlink(
    tree_view: &gtk::TreeView,
    column_file_name: i32,
    column_path: i32,
    column_color: i32,
    column_selection: i32,
    hardlinking: bool,
    text_view_errors: &TextView,
) {
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
            let full_file_path = get_full_name_from_path_name(&path, &file_name);

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
                    add_text_to_text_view(
                        text_view_errors,
                        flg!(
                            "delete_file_failed",
                            generate_translation_hashmap(vec![("name", file_to_symlink.to_string()), ("reason", e.to_string())])
                        )
                        .as_str(),
                    );
                    continue;
                };

                #[cfg(target_family = "unix")]
                {
                    if let Err(e) = std::os::unix::fs::symlink(&symhardlink_data.original_data, &file_to_symlink) {
                        add_text_to_text_view(
                            text_view_errors,
                            flg!(
                                "delete_file_failed",
                                generate_translation_hashmap(vec![("name", file_to_symlink.to_string()), ("reason", e.to_string())])
                            )
                            .as_str(),
                        );
                        continue;
                    };
                }
                #[cfg(target_family = "windows")]
                {
                    if let Err(e) = std::os::windows::fs::symlink_file(&symhardlink_data.original_data, &file_to_symlink) {
                        add_text_to_text_view(
                            text_view_errors,
                            flg!(
                                "delete_file_failed",
                                generate_translation_hashmap(vec![("name", file_to_symlink.to_string()), ("reason", e.to_string())])
                            )
                            .as_str(),
                        );
                        continue;
                    };
                }
            }
        }
    }
    for tree_path in vec_tree_path_to_remove.iter().rev() {
        model.remove(&model.iter(tree_path).unwrap());
    }

    clean_invalid_headers(&model, column_color, column_path);
}

fn create_dialog_non_group(window_main: &gtk::Window) -> Dialog {
    let dialog = gtk::Dialog::builder()
        .title(&flg!("hard_sym_invalid_selection_title_dialog"))
        .transient_for(window_main)
        .modal(true)
        .build();
    let button_ok = dialog.add_button(&flg!("general_ok_button"), ResponseType::Ok);
    dialog.add_button(&flg!("general_close_button"), ResponseType::Cancel);

    let label: gtk::Label = gtk::Label::new(Some(&flg!("hard_sym_invalid_selection_label_1")));
    let label2: gtk::Label = gtk::Label::new(Some(&flg!("hard_sym_invalid_selection_label_2")));
    let label3: gtk::Label = gtk::Label::new(Some(&flg!("hard_sym_invalid_selection_label_3")));

    button_ok.grab_focus();

    let internal_box = get_dialog_box_child(&dialog);
    internal_box.add(&label);
    internal_box.add(&label2);
    internal_box.add(&label3);

    dialog.show_all();
    dialog
}

pub async fn check_if_changing_one_item_in_group_and_continue(tree_view: &gtk::TreeView, column_color: i32, column_selection: i32, window_main: &gtk::Window) -> bool {
    let model = get_list_store(tree_view);

    let mut selected_values_in_group = 0;

    if let Some(iter) = model.iter_first() {
        assert_eq!(model.value(&iter, column_color).get::<String>().unwrap(), HEADER_ROW_COLOR); // First element should be header

        loop {
            if !model.iter_next(&iter) {
                break;
            }

            if model.value(&iter, column_color).get::<String>().unwrap() == HEADER_ROW_COLOR {
                if selected_values_in_group == 1 {
                    break;
                }
                selected_values_in_group = 0;
            } else {
                if model.value(&iter, column_selection).get::<bool>().unwrap() {
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
        if response_type != gtk::ResponseType::Ok {
            confirmation_dialog.hide();
            confirmation_dialog.close();
            return false;
        }
        confirmation_dialog.hide();
        confirmation_dialog.close();
    }

    true
}

pub async fn check_if_anything_is_selected_async(tree_view: &gtk::TreeView, column_color: i32, column_selection: i32) -> bool {
    let model = get_list_store(tree_view);

    if let Some(iter) = model.iter_first() {
        assert_eq!(model.value(&iter, column_color).get::<String>().unwrap(), HEADER_ROW_COLOR); // First element should be header

        loop {
            if !model.iter_next(&iter) {
                break;
            }

            if model.value(&iter, column_color).get::<String>().unwrap() == MAIN_ROW_COLOR && model.value(&iter, column_selection).get::<bool>().unwrap() {
                return true;
            }
        }
    }

    false
}

pub async fn check_if_can_link_files(check_button_settings_confirm_link: &gtk::CheckButton, window_main: &gtk::Window) -> bool {
    if check_button_settings_confirm_link.is_active() {
        let (confirmation_dialog_link, check_button) = create_dialog_ask_for_linking(window_main);

        let response_type = confirmation_dialog_link.run_future().await;
        if response_type == gtk::ResponseType::Ok {
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

fn create_dialog_ask_for_linking(window_main: &gtk::Window) -> (Dialog, CheckButton) {
    let dialog = gtk::Dialog::builder()
        .title(&flg!("hard_sym_link_title_dialog"))
        .transient_for(window_main)
        .modal(true)
        .build();
    let button_ok = dialog.add_button(&flg!("general_ok_button"), ResponseType::Ok);
    dialog.add_button(&flg!("general_close_button"), ResponseType::Cancel);

    let label: gtk::Label = gtk::Label::new(Some(&flg!("hard_sym_link_label")));
    let check_button: gtk::CheckButton = gtk::CheckButton::with_label(&flg!("dialogs_ask_next_time"));
    check_button.set_active(true);
    check_button.set_halign(Align::Center);

    button_ok.grab_focus();

    let internal_box = get_dialog_box_child(&dialog);
    internal_box.add(&label);
    internal_box.add(&check_button);

    dialog.show_all();
    (dialog, check_button)
}
