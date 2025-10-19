use std::path::Path;

use fs_extra::dir::CopyOptions;
use gtk4::prelude::*;
use gtk4::{ResponseType, TreePath};
use log::debug;

use crate::connect_things::file_chooser_helpers::extract_paths_from_file_chooser;
use crate::flg;
use crate::gui_structs::common_tree_view::SubView;
use crate::gui_structs::gui_data::GuiData;
use crate::help_functions::{add_text_to_text_view, check_how_much_elements_is_selected, clean_invalid_headers, get_full_name_from_path_name, reset_text_view};

pub(crate) fn connect_button_move(gui_data: &GuiData) {
    let buttons_move = gui_data.bottom_buttons.buttons_move.clone();

    let entry_info = gui_data.entry_info.clone();
    let text_view_errors = gui_data.text_view_errors.clone();

    let file_dialog_move_to_folder = gui_data.file_dialog_move_to_folder.clone();
    let common_tree_views = gui_data.main_notebook.common_tree_views.clone();

    file_dialog_move_to_folder.connect_response(move |file_chooser, response_type| {
        let sv = common_tree_views.get_current_subview();

        let (number_of_selected_items, _number_of_selected_groups) = check_how_much_elements_is_selected(sv);

        // Nothing is selected
        if number_of_selected_items == 0 {
            return;
        }

        reset_text_view(&text_view_errors);

        if response_type == ResponseType::Accept {
            let folders = extract_paths_from_file_chooser(file_chooser);

            if folders.len() != 1 {
                add_text_to_text_view(&text_view_errors, flg!("move_files_choose_more_than_1_path", path_number = folders.len()).as_str());
            } else {
                let folder = folders[0].clone();
                if sv.nb_object.column_header.is_some() {
                    move_with_tree(sv, &folder, &entry_info, &text_view_errors);
                } else {
                    move_with_list(sv, &folder, &entry_info, &text_view_errors);
                }
            }
        }
        common_tree_views.hide_preview();
    });

    buttons_move.connect_clicked(move |_| {
        file_dialog_move_to_folder.show();
    });
}

fn move_with_tree(sv: &SubView, destination_folder: &Path, entry_info: &gtk4::Entry, text_view_errors: &gtk4::TextView) {
    let model = sv.get_model();
    let column_header = sv.nb_object.column_header.expect("Using move_with_tree without header column");

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

    move_files_common(
        &selected_rows,
        &model,
        sv.nb_object.column_name,
        sv.nb_object.column_path,
        destination_folder,
        entry_info,
        text_view_errors,
    );

    clean_invalid_headers(&model, column_header, sv.nb_object.column_path);
}

fn move_with_list(sv: &SubView, destination_folder: &Path, entry_info: &gtk4::Entry, text_view_errors: &gtk4::TextView) {
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

    move_files_common(
        &selected_rows,
        &model,
        sv.nb_object.column_name,
        sv.nb_object.column_path,
        destination_folder,
        entry_info,
        text_view_errors,
    );
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
    let mut messages: String = String::new();

    let mut moved_files: u32 = 0;

    debug!("Starting to move {} files", selected_rows.len());
    let start_time = std::time::Instant::now();

    // Save to variable paths of files, and remove it when not removing all occurrences.
    'next_result: for tree_path in selected_rows.iter().rev() {
        let iter = model.iter(tree_path).expect("Using invalid tree_path");

        let file_name = model.get::<String>(&iter, column_file_name);
        let path = model.get::<String>(&iter, column_path);

        let thing = get_full_name_from_path_name(&path, &file_name);
        let destination_file = destination_folder.join(&file_name);
        if Path::new(&thing).is_dir() {
            if let Err(e) = fs_extra::dir::move_dir(&thing, &destination_file, &CopyOptions::new()) {
                messages += flg!("move_folder_failed", name = thing, reason = e.to_string()).as_str();
                messages += "\n";
                continue 'next_result;
            }
        } else if let Err(e) = fs_extra::file::move_file(&thing, &destination_file, &fs_extra::file::CopyOptions::new()) {
            messages += flg!("move_file_failed", name = thing, reason = e.to_string()).as_str();
            messages += "\n";

            continue 'next_result;
        }
        model.remove(&iter);
        moved_files += 1;
    }

    debug!("Moved {moved_files} files in {:?}", start_time.elapsed());

    entry_info.set_text(flg!("move_stats", num_files = moved_files, all_files = selected_rows.len()).as_str());

    text_view_errors.buffer().set_text(messages.as_str());
}
