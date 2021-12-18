use std::path::{Path, PathBuf};

use czkawka_core::fl;
use gtk::prelude::*;
use gtk::{ResponseType, TreePath};

use crate::gui_data::GuiData;
use crate::help_functions::*;
use crate::notebook_enums::*;

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
        let nb_object = &NOTEBOOKS_INFOS[nb_number as usize];

        move_things(
            tree_view,
            nb_object.column_name,
            nb_object.column_path,
            nb_object.column_color,
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
fn move_things(tree_view: &gtk::TreeView, column_file_name: i32, column_path: i32, column_color: Option<i32>, column_selection: i32, entry_info: &gtk::Entry, text_view_errors: &gtk::TextView, window_main: &gtk::Window) {
    reset_text_view(text_view_errors);

    let chooser = gtk::FileChooserDialog::builder()
        .title(&fl!("move_files_title_dialog"))
        .action(gtk::FileChooserAction::SelectFolder)
        .transient_for(window_main)
        .modal(true)
        .build();
    chooser.add_button(&fl!("general_ok_button"), ResponseType::Ok);
    chooser.add_button(&fl!("general_close_button"), ResponseType::Cancel);

    chooser.set_select_multiple(false);
    chooser.show_all();

    let entry_info = entry_info.clone();
    let text_view_errors = text_view_errors.clone();
    let tree_view = tree_view.clone();
    chooser.connect_response(move |file_chooser, response_type| {
        if response_type == gtk::ResponseType::Ok {
            let folders: Vec<PathBuf> = file_chooser.filenames();
            // GTK 4
            // folders = Vec::new();
            // if let Some(g_files) = file_chooser.files() {
            //     for index in 0..g_files.n_items() {
            //         let file = &g_files.item(index);
            //         if let Some(file) = file {
            //             println!("{:?}", file);
            //             let ss = file.clone().downcast::<gtk4::gio::File>().unwrap();
            //             if let Some(path_buf) = ss.path() {
            //                 folders.push(path_buf);
            //             }
            //         }
            //     }
            // }

            if folders.len() != 1 {
                add_text_to_text_view(&text_view_errors, format!("{} {:?}", &fl!("move_files_choose_more_than_1_path"), folders).as_str());
            } else {
                let folder = folders[0].clone();
                if let Some(column_color) = column_color {
                    move_with_tree(&tree_view, column_file_name, column_path, column_color, column_selection, folder, &entry_info, &text_view_errors);
                } else {
                    move_with_list(&tree_view, column_file_name, column_path, column_selection, folder, &entry_info, &text_view_errors);
                }
            }
        }
        file_chooser.close();
    });
}

fn move_with_tree(tree_view: &gtk::TreeView, column_file_name: i32, column_path: i32, column_color: i32, column_selection: i32, destination_folder: PathBuf, entry_info: &gtk::Entry, text_view_errors: &gtk::TextView) {
    let model = get_list_store(tree_view);

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

    move_files_common(&selected_rows, &model, column_file_name, column_path, &destination_folder, entry_info, text_view_errors);

    clean_invalid_headers(&model, column_color);
}

fn move_with_list(tree_view: &gtk::TreeView, column_file_name: i32, column_path: i32, column_selection: i32, destination_folder: PathBuf, entry_info: &gtk::Entry, text_view_errors: &gtk::TextView) {
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

    move_files_common(&selected_rows, &model, column_file_name, column_path, &destination_folder, entry_info, text_view_errors)
}

fn move_files_common(selected_rows: &[TreePath], model: &gtk::ListStore, column_file_name: i32, column_path: i32, destination_folder: &Path, entry_info: &gtk::Entry, text_view_errors: &gtk::TextView) {
    let mut messages: String = "".to_string();

    let mut moved_files: u32 = 0;

    // Save to variable paths of files, and remove it when not removing all occurrences.
    'next_result: for tree_path in selected_rows.iter().rev() {
        let iter = model.iter(tree_path).unwrap();

        let file_name = model.value(&iter, column_file_name).get::<String>().unwrap();
        let path = model.value(&iter, column_path).get::<String>().unwrap();

        let thing = format!("{}/{}", path, file_name);
        let destination_file = destination_folder.join(file_name);
        if Path::new(&thing).is_dir() {
            if let Err(e) = fs_extra::dir::move_dir(&thing, &destination_file, &fs_extra::dir::CopyOptions::new()) {
                messages += format!("{}, reason {}\n", fl!("move_folder_failed"), e).as_str();
                continue 'next_result;
            }
        } else {
            if let Err(e) = fs_extra::file::move_file(&thing, &destination_file, &fs_extra::file::CopyOptions::new()) {
                messages += format!("{}, reason {}\n", fl!("move_file_failed"), e).as_str();
                continue 'next_result;
            }
        }
        model.remove(&iter);
        moved_files += 1;
    }
    entry_info.set_text(format!("{} {}/{} {}", fl!("move_stats_1"), moved_files, selected_rows.len(), fl!("move_stats_2")).as_str());

    text_view_errors.buffer().unwrap().set_text(messages.as_str());
}
