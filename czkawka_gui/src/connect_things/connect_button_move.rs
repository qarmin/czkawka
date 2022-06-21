use std::path::{Path, PathBuf};

use gtk4::prelude::*;
use gtk4::{ResponseType, TreePath};

use crate::flg;
use crate::gui_structs::gui_data::GuiData;
use crate::help_functions::*;
use crate::localizer_core::generate_translation_hashmap;
use crate::notebook_enums::*;
use crate::notebook_info::NOTEBOOKS_INFO;

pub fn connect_button_move(gui_data: &GuiData) {
    let buttons_move = gui_data.bottom_buttons.buttons_move.clone();
    let notebook_main = gui_data.main_notebook.notebook_main.clone();

    let main_tree_views = gui_data.main_notebook.get_main_tree_views();

    let image_preview_similar_images = gui_data.main_notebook.image_preview_similar_images.clone();
    let image_preview_duplicates = gui_data.main_notebook.image_preview_duplicates.clone();

    let entry_info = gui_data.entry_info.clone();
    let text_view_errors = gui_data.text_view_errors.clone();

    let window_main = gui_data.window_main.clone();

    let preview_path = gui_data.preview_path.clone();

    buttons_move.connect_clicked(move |_| {
        let nb_number = notebook_main.current_page().unwrap();
        let tree_view = &main_tree_views[nb_number as usize];
        let nb_object = &NOTEBOOKS_INFO[nb_number as usize];

        let (number_of_selected_items, _number_of_selected_groups) = check_how_much_elements_is_selected(tree_view, nb_object.column_header, nb_object.column_selection);

        // Nothing is selected
        if number_of_selected_items == 0 {
            return;
        }
        move_things(
            tree_view,
            nb_object.column_name,
            nb_object.column_path,
            nb_object.column_header,
            nb_object.column_selection,
            &entry_info,
            &text_view_errors,
            &window_main,
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
    });
}

// TODO add progress bar
fn move_things(
    tree_view: &gtk4::TreeView,
    column_file_name: i32,
    column_path: i32,
    column_header: Option<i32>,
    column_selection: i32,
    entry_info: &gtk4::Entry,
    text_view_errors: &gtk4::TextView,
    window_main: &gtk4::Window,
) {
    reset_text_view(text_view_errors);

    let chooser = gtk4::FileChooserDialog::builder()
        .title(&flg!("move_files_title_dialog"))
        .action(gtk4::FileChooserAction::SelectFolder)
        .transient_for(window_main)
        .modal(true)
        .build();
    chooser.add_button(&flg!("general_ok_button"), ResponseType::Ok);
    chooser.add_button(&flg!("general_close_button"), ResponseType::Cancel);

    chooser.set_select_multiple(false);
    chooser.show();

    let entry_info = entry_info.clone();
    let text_view_errors = text_view_errors.clone();
    let tree_view = tree_view.clone();
    chooser.connect_response(move |file_chooser, response_type| {
        if response_type == gtk4::ResponseType::Ok {
            let mut folders: Vec<PathBuf> = Vec::new();
            let g_files = file_chooser.files();
            for index in 0..g_files.n_items() {
                let file = &g_files.item(index);
                if let Some(file) = file {
                    let ss = file.clone().downcast::<gtk4::gio::File>().unwrap();
                    if let Some(path_buf) = ss.path() {
                        folders.push(path_buf);
                    }
                }
            }

            if folders.len() != 1 {
                add_text_to_text_view(
                    &text_view_errors,
                    flg!(
                        "move_files_choose_more_than_1_path",
                        generate_translation_hashmap(vec![("path_number", folders.len().to_string())])
                    )
                    .as_str(),
                );
            } else {
                let folder = folders[0].clone();
                if let Some(column_header) = column_header {
                    move_with_tree(
                        &tree_view,
                        column_file_name,
                        column_path,
                        column_header,
                        column_selection,
                        folder,
                        &entry_info,
                        &text_view_errors,
                    );
                } else {
                    move_with_list(&tree_view, column_file_name, column_path, column_selection, folder, &entry_info, &text_view_errors);
                }
            }
        }
        file_chooser.close();
    });
}

fn move_with_tree(
    tree_view: &gtk4::TreeView,
    column_file_name: i32,
    column_path: i32,
    column_header: i32,
    column_selection: i32,
    destination_folder: PathBuf,
    entry_info: &gtk4::Entry,
    text_view_errors: &gtk4::TextView,
) {
    let model = get_list_store(tree_view);

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

    move_files_common(&selected_rows, &model, column_file_name, column_path, &destination_folder, entry_info, text_view_errors);

    clean_invalid_headers(&model, column_header, column_path);
}

fn move_with_list(
    tree_view: &gtk4::TreeView,
    column_file_name: i32,
    column_path: i32,
    column_selection: i32,
    destination_folder: PathBuf,
    entry_info: &gtk4::Entry,
    text_view_errors: &gtk4::TextView,
) {
    let model = get_list_store(tree_view);

    let mut selected_rows = Vec::new();

    if let Some(iter) = model.iter_first() {
        loop {
            if model.get::<bool>(&iter, column_selection) {
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

    move_files_common(&selected_rows, &model, column_file_name, column_path, &destination_folder, entry_info, text_view_errors)
}

fn move_files_common(
    selected_rows: &[TreePath],
    model: &gtk4::ListStore,
    column_file_name: i32,
    column_path: i32,
    destination_folder: &Path,
    entry_info: &gtk4::Entry,
    text_view_errors: &gtk4::TextView,
) {
    let mut messages: String = "".to_string();

    let mut moved_files: u32 = 0;

    // Save to variable paths of files, and remove it when not removing all occurrences.
    'next_result: for tree_path in selected_rows.iter().rev() {
        let iter = model.iter(tree_path).unwrap();

        let file_name = model.get::<String>(&iter, column_file_name);
        let path = model.get::<String>(&iter, column_path);

        let thing = get_full_name_from_path_name(&path, &file_name);
        let destination_file = destination_folder.join(file_name);
        if Path::new(&thing).is_dir() {
            if let Err(e) = fs_extra::dir::move_dir(&thing, &destination_file, &fs_extra::dir::CopyOptions::new()) {
                messages += flg!("move_folder_failed", generate_translation_hashmap(vec![("name", thing), ("reason", e.to_string())])).as_str();
                messages += "\n";
                continue 'next_result;
            }
        } else {
            if let Err(e) = fs_extra::file::move_file(&thing, &destination_file, &fs_extra::file::CopyOptions::new()) {
                messages += flg!("move_file_failed", generate_translation_hashmap(vec![("name", thing), ("reason", e.to_string())])).as_str();
                messages += "\n";

                continue 'next_result;
            }
        }
        model.remove(&iter);
        moved_files += 1;
    }

    entry_info.set_text(
        flg!(
            "move_stats",
            generate_translation_hashmap(vec![("num_files", moved_files.to_string()), ("all_files", selected_rows.len().to_string())])
        )
        .as_str(),
    );

    text_view_errors.buffer().set_text(messages.as_str());
}
