use crate::common::{get_is_header_mode, get_tool_model, set_tool_model};
use crate::model_operations::{collect_path_name_from_model, deselect_all_items, filter_out_checked_items};
use crate::{Callabler, CurrentTab, GuiState, MainListModel, MainWindow};

use czkawka_core::common_messages::Messages;
use rayon::prelude::*;
use slint::{ComponentHandle, ModelRc, VecModel};
use std::path::{Path, PathBuf};
use std::{fs, path};

pub fn connect_move(app: &MainWindow) {
    let a = app.as_weak();
    app.global::<Callabler>().on_move_items(move |select_mode, preserve_structure, copy_mode| {
        let app = a.upgrade().unwrap();
        let active_tab = app.global::<GuiState>().get_active_tab();
        let current_model = get_tool_model(&app, active_tab);

        // If tree structure will be
        let preserve_structure = false;
        let copy_mode = true;
        let output_folder = "/home/rafal/Downloads/AAAAAAAA";

        let (errors, new_model) = move_operation(&current_model, preserve_structure, copy_mode, output_folder, active_tab);
        if let Some(new_model) = new_model {
            set_tool_model(&app, active_tab, new_model);
        }
        app.global::<GuiState>().set_info_text(Messages::new_from_errors(errors).create_messages_text().into());
    });
}

fn move_operation(
    items: &ModelRc<MainListModel>,
    preserve_structure: bool,
    copy_mode: bool,
    output_folder: &str,
    active_tab: CurrentTab,
) -> (Vec<String>, Option<ModelRc<MainListModel>>) {
    let (entries_to_move, mut entries_left) = filter_out_checked_items(items, get_is_header_mode(active_tab));

    if !entries_to_move.is_empty() {
        let vec_items_to_move = collect_path_name_from_model(&entries_to_move, active_tab);
        let errors = move_selected_items(vec_items_to_move, preserve_structure, copy_mode, output_folder);
        deselect_all_items(&mut entries_left);

        let r = ModelRc::new(VecModel::from(entries_left));
        return (errors, Some(r));
    }
    (vec![], None)
}

fn move_selected_items(items_to_move: Vec<(String, String)>, preserve_structure: bool, copy_mode: bool, output_folder: &str) -> Vec<String> {
    if let Err(err) = fs::create_dir_all(output_folder) {
        return vec![format!("Error while creating folder: {err}")];
    }

    if copy_mode {
        items_to_move
            .into_par_iter()
            .filter_map(|(path, name)| {
                let (input_file, output_file) = collect_path_and_create_folders(&path, &name, output_folder, preserve_structure);

                if let Err(e) = fs::copy(&input_file, &output_file) {
                    return Some(format!("Error while copying file {input_file:?} to {output_file:?}, reason {e}"));
                }
                None
            })
            .collect()
    } else {
        items_to_move
            .into_par_iter()
            .filter_map(|(path, name)| {
                let (input_file, output_file) = collect_path_and_create_folders(&path, &name, output_folder, preserve_structure);

                if output_file.exists() {
                    return Some(format!("File {output_file:?} already exists"));
                }

                // Try to rename file, may fail due various reasons
                if fs::rename(&input_file, &output_file).is_ok() {
                    return None;
                }

                // It is possible that this failed, because file is on different partition, so
                // we need to copy file and then remove old
                if let Err(e) = fs::copy(&input_file, &output_file) {
                    return Some(format!(
                        "Error while copying file {input_file:?} to {output_file:?}(moving into different partition), reason {e}"
                    ));
                }
                if let Err(e) = fs::remove_file(&input_file) {
                    return Some(format!("Error while removing file {input_file:?}(after copying into different partition), reason {e}"));
                }

                None
            })
            .collect()
    }
}

// Create input/output paths, and create output folder
fn collect_path_and_create_folders(input_path: &str, input_file: &str, output_path: &str, preserve_structure: bool) -> (PathBuf, PathBuf) {
    let mut input_full_path = PathBuf::from(input_path);
    input_full_path.push(input_file);

    let mut output_full_path = PathBuf::from(output_path);
    if preserve_structure {
        output_full_path.extend(Path::new(input_path).components().filter(|c| match c {
            path::Component::Normal(_) => true,
            _ => false,
        }));
    };
    let _ = fs::create_dir_all(&output_full_path);
    output_full_path.push(input_file);

    (input_full_path, output_full_path)
}
