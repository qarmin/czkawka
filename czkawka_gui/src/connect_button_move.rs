use crate::gui_data::GuiData;
use crate::help_functions::*;
use crate::notebook_enums::*;
use gtk::prelude::*;
use gtk::TreePath;
use std::path::{Path, PathBuf};

pub fn connect_button_move(gui_data: &GuiData) {
    let buttons_move = gui_data.bottom_buttons.buttons_move.clone();
    let notebook_main = gui_data.main_notebook.notebook_main.clone();

    let main_tree_views = gui_data.main_notebook.get_main_tree_views();

    let image_preview_similar_images = gui_data.main_notebook.image_preview_similar_images.clone();
    let image_preview_duplicates = gui_data.main_notebook.image_preview_duplicates.clone();

    let entry_info = gui_data.entry_info.clone();
    let text_view_errors = gui_data.text_view_errors.clone();

    let window_main = gui_data.window_main.clone();

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

// TODO add progress bar
fn move_things(tree_view: &gtk::TreeView, column_file_name: i32, column_path: i32, column_color: Option<i32>, column_selection: i32, entry_info: &gtk::Entry, text_view_errors: &gtk::TextView, window_main: &gtk::Window) {
    reset_text_view(text_view_errors);

    let chooser = gtk::FileChooserDialog::with_buttons(
        Some("Choose folder to which you want to move duplicated files"),
        Some(window_main),
        gtk::FileChooserAction::SelectFolder,
        &[("Ok", gtk::ResponseType::Ok), ("Close", gtk::ResponseType::Cancel)],
    );
    chooser.set_select_multiple(true);
    chooser.show_all();
    let response_type = chooser.run();
    if response_type == gtk::ResponseType::Ok {
        let folders = chooser.filenames();
        if folders.len() != 1 {
            add_text_to_text_view(text_view_errors, format!("Only 1 path must be selected to be able to copy there duplicated files, found {:?}", folders).as_str());
        } else {
            let folder = folders[0].clone();
            if let Some(column_color) = column_color {
                move_with_tree(tree_view, column_file_name, column_path, column_color, column_selection, folder, entry_info, text_view_errors);
            } else {
                move_with_list(tree_view, column_file_name, column_path, column_selection, folder, entry_info, text_view_errors);
            }
        }
    }
    chooser.close();
}
fn move_with_tree(tree_view: &gtk::TreeView, column_file_name: i32, column_path: i32, column_color: i32, column_selection: i32, destination_folder: PathBuf, entry_info: &gtk::Entry, text_view_errors: &gtk::TextView) {
    let model = get_list_store(tree_view);

    let mut selection_rows = Vec::new();

    if let Some(iter) = model.iter_first() {
        loop {
            if model.value(&iter, column_selection).get::<bool>().unwrap() {
                if model.value(&iter, column_color).get::<String>().unwrap() == MAIN_ROW_COLOR {
                    selection_rows.push(model.path(&iter).unwrap());
                } else {
                    panic!("Header row shouldn't be selected, please report bug.");
                }
            }

            if !model.iter_next(&iter) {
                break;
            }
        }
    }

    move_files_common(&selection_rows, &model, column_file_name, column_path, &destination_folder, entry_info, text_view_errors);

    clean_invalid_headers(&model, column_color);
}

fn move_with_list(tree_view: &gtk::TreeView, column_file_name: i32, column_path: i32, column_selection: i32, destination_folder: PathBuf, entry_info: &gtk::Entry, text_view_errors: &gtk::TextView) {
    let model = get_list_store(tree_view);

    let mut selection_rows = Vec::new();

    if let Some(iter) = model.iter_first() {
        loop {
            if model.value(&iter, column_selection).get::<bool>().unwrap() {
                selection_rows.push(model.path(&iter).unwrap());
            }

            if !model.iter_next(&iter) {
                break;
            }
        }
    }

    move_files_common(&selection_rows, &model, column_file_name, column_path, &destination_folder, entry_info, text_view_errors)
}

fn move_files_common(selection_rows: &[TreePath], model: &gtk::ListStore, column_file_name: i32, column_path: i32, destination_folder: &Path, entry_info: &gtk::Entry, text_view_errors: &gtk::TextView) {
    let mut messages: String = "".to_string();

    let mut moved_files: u32 = 0;

    // Save to variable paths of files, and remove it when not removing all occurrences.
    'next_result: for tree_path in selection_rows.iter().rev() {
        let iter = model.iter(tree_path).unwrap();

        let file_name = model.value(&iter, column_file_name).get::<String>().unwrap();
        let path = model.value(&iter, column_path).get::<String>().unwrap();

        let thing = format!("{}/{}", path, file_name);
        let destination_file = destination_folder.join(file_name);
        if Path::new(&thing).is_dir() {
            if let Err(e) = fs_extra::dir::move_dir(&thing, &destination_file, &fs_extra::dir::CopyOptions::new()) {
                messages += format!("Failed to move folder, reason {}\n", e).as_str();
                continue 'next_result;
            }
        } else {
            if let Err(e) = fs_extra::file::move_file(&thing, &destination_file, &fs_extra::file::CopyOptions::new()) {
                messages += format!("Failed to move file, reason {}\n", e).as_str();
                continue 'next_result;
            }
        }
        model.remove(&iter);
        moved_files += 1;
    }
    entry_info.set_text(format!("Properly moved {}/{} files/folders", moved_files, selection_rows.len()).as_str());

    text_view_errors.buffer().unwrap().set_text(messages.as_str());
}
