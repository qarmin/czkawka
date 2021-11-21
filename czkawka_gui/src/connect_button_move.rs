use crate::gui_data::GuiData;
use crate::help_functions::*;
use crate::notebook_enums::*;
use gtk::prelude::*;
use std::path::{Path, PathBuf};

pub fn connect_button_move(gui_data: &GuiData) {
    let gui_data = gui_data.clone();

    let buttons_move = gui_data.bottom_buttons.buttons_move.clone();
    let notebook_main = gui_data.main_notebook.notebook_main.clone();

    let tree_view_duplicate_finder = gui_data.main_notebook.tree_view_duplicate_finder.clone();
    let tree_view_empty_folder_finder = gui_data.main_notebook.tree_view_empty_folder_finder.clone();
    let tree_view_big_files_finder = gui_data.main_notebook.tree_view_big_files_finder.clone();
    let tree_view_empty_files_finder = gui_data.main_notebook.tree_view_empty_files_finder.clone();
    let tree_view_temporary_files_finder = gui_data.main_notebook.tree_view_temporary_files_finder.clone();
    let tree_view_similar_images_finder = gui_data.main_notebook.tree_view_similar_images_finder.clone();
    let tree_view_zeroed_files_finder = gui_data.main_notebook.tree_view_zeroed_files_finder.clone();
    let tree_view_same_music_finder = gui_data.main_notebook.tree_view_same_music_finder.clone();
    let tree_view_invalid_symlinks = gui_data.main_notebook.tree_view_invalid_symlinks.clone();
    let tree_view_broken_files = gui_data.main_notebook.tree_view_broken_files.clone();

    let image_preview_similar_images = gui_data.main_notebook.image_preview_similar_images.clone();

    buttons_move.connect_clicked(move |_| match to_notebook_main_enum(notebook_main.current_page().unwrap()) {
        NotebookMainEnum::Duplicate => {
            move_things(
                tree_view_duplicate_finder.clone(),
                ColumnsDuplicates::Name as i32,
                ColumnsDuplicates::Path as i32,
                Some(ColumnsDuplicates::Color as i32),
                ColumnsDuplicates::ActiveSelectButton as i32,
                &gui_data,
            );
        }
        NotebookMainEnum::SameMusic => {
            move_things(
                tree_view_same_music_finder.clone(),
                ColumnsSameMusic::Name as i32,
                ColumnsSameMusic::Path as i32,
                Some(ColumnsSameMusic::Color as i32),
                ColumnsSameMusic::ActiveSelectButton as i32,
                &gui_data,
            );
        }
        NotebookMainEnum::SimilarImages => {
            move_things(
                tree_view_similar_images_finder.clone(),
                ColumnsSimilarImages::Name as i32,
                ColumnsSimilarImages::Path as i32,
                Some(ColumnsSimilarImages::Color as i32),
                ColumnsSimilarImages::ActiveSelectButton as i32,
                &gui_data,
            );
            image_preview_similar_images.hide();
        }
        NotebookMainEnum::BigFiles => {
            move_things(
                tree_view_big_files_finder.clone(),
                ColumnsBigFiles::Name as i32,
                ColumnsBigFiles::Path as i32,
                None,
                ColumnsBigFiles::ActiveSelectButton as i32,
                &gui_data,
            );
        }
        NotebookMainEnum::Zeroed => {
            move_things(
                tree_view_zeroed_files_finder.clone(),
                ColumnsZeroedFiles::Name as i32,
                ColumnsZeroedFiles::Path as i32,
                None,
                ColumnsZeroedFiles::ActiveSelectButton as i32,
                &gui_data,
            );
        }
        NotebookMainEnum::BrokenFiles => {
            move_things(
                tree_view_broken_files.clone(),
                ColumnsBrokenFiles::Name as i32,
                ColumnsBrokenFiles::Path as i32,
                None,
                ColumnsBrokenFiles::ActiveSelectButton as i32,
                &gui_data,
            );
        }
        NotebookMainEnum::EmptyDirectories => {
            move_things(
                tree_view_empty_folder_finder.clone(),
                ColumnsEmptyFolders::Name as i32,
                ColumnsEmptyFolders::Path as i32,
                None,
                ColumnsEmptyFolders::ActiveSelectButton as i32,
                &gui_data,
            );
        }
        NotebookMainEnum::EmptyFiles => {
            move_things(
                tree_view_empty_files_finder.clone(),
                ColumnsEmptyFiles::Name as i32,
                ColumnsEmptyFiles::Path as i32,
                None,
                ColumnsEmptyFiles::ActiveSelectButton as i32,
                &gui_data,
            );
        }
        NotebookMainEnum::Symlinks => {
            move_things(
                tree_view_invalid_symlinks.clone(),
                ColumnsInvalidSymlinks::Name as i32,
                ColumnsInvalidSymlinks::Path as i32,
                None,
                ColumnsInvalidSymlinks::ActiveSelectButton as i32,
                &gui_data,
            );
        }
        NotebookMainEnum::Temporary => {
            move_things(
                tree_view_temporary_files_finder.clone(),
                ColumnsTemporaryFiles::Name as i32,
                ColumnsTemporaryFiles::Path as i32,
                None,
                ColumnsTemporaryFiles::ActiveSelectButton as i32,
                &gui_data,
            );
        }
    });
}

// TODO create and show folder chooser where user can select path
fn move_things(tree_view: gtk::TreeView, column_file_name: i32, column_path: i32, column_color: Option<i32>, column_selection: i32, gui_data: &GuiData) {
    let text_view_errors = gui_data.text_view_errors.clone();
    let window_main = gui_data.window_main.clone();

    reset_text_view(&text_view_errors);

    let chooser = gtk::FileChooserDialog::with_buttons(
        Some("Choose folder to which you want to move duplicated files"),
        Some(&window_main),
        gtk::FileChooserAction::SelectFolder,
        &[("Ok", gtk::ResponseType::Ok), ("Close", gtk::ResponseType::Cancel)],
    );
    chooser.set_select_multiple(true);
    chooser.show_all();
    let response_type = chooser.run();
    if response_type == gtk::ResponseType::Ok {
        let folders = chooser.filenames();
        if folders.len() != 1 {
            add_text_to_text_view(&text_view_errors, format!("Only 1 path must be selected to be able to copy there duplicated files, found {:?}", folders).as_str());
        } else {
            let folder = folders[0].clone();
            if let Some(column_color) = column_color {
                move_with_tree(tree_view, column_file_name, column_path, column_color, column_selection, gui_data, folder);
            } else {
                move_with_list(tree_view, column_file_name, column_path, column_selection, gui_data, folder);
            }
        }
    }
    chooser.close();
}
fn move_with_tree(tree_view: gtk::TreeView, column_file_name: i32, column_path: i32, column_color: i32, column_selection: i32, gui_data: &GuiData, destination_folder: PathBuf) {
    let text_view_errors = gui_data.text_view_errors.clone();
    let entry_info = gui_data.entry_info.clone();

    let model = get_list_store(&tree_view);

    let mut messages: String = "".to_string();

    let mut selection_rows = Vec::new();

    if let Some(iter) = model.iter_first() {
        loop {
            if model.value(&iter, column_selection).get::<bool>().unwrap() {
                // TODO, this maybe isn't required if we will be sure that any header cannot be selected
                if model.value(&iter, column_color).get::<String>().unwrap() == MAIN_ROW_COLOR {
                    selection_rows.push(model.path(&iter).unwrap());
                } else {
                    panic!("Header row shouldn't have selected, selection button");
                }
            }

            if !model.iter_next(&iter) {
                break;
            }
        }
    }

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

    // TODO move this to different function, this is used in different places
    // Remove only child from header
    if let Some(first_iter) = model.iter_first() {
        let mut vec_tree_path_to_delete: Vec<gtk::TreePath> = Vec::new();
        let mut current_iter = first_iter;
        if model.value(&current_iter, column_color).get::<String>().unwrap() != HEADER_ROW_COLOR {
            panic!("First deleted element, should be a header"); // First element should be header
        };

        let mut next_iter;
        let mut next_next_iter;
        'main: loop {
            if model.value(&current_iter, column_color).get::<String>().unwrap() != HEADER_ROW_COLOR {
                panic!("First deleted element, should be a header"); // First element should be header
            };

            next_iter = current_iter.clone();
            if !model.iter_next(&next_iter) {
                // There is only single header left (H1 -> END) -> (NOTHING)
                vec_tree_path_to_delete.push(model.path(&current_iter).unwrap());
                break 'main;
            }

            if model.value(&next_iter, column_color).get::<String>().unwrap() == HEADER_ROW_COLOR {
                // There are two headers each others(we remove just first) -> (H1 -> H2) -> (H2)
                vec_tree_path_to_delete.push(model.path(&current_iter).unwrap());
                current_iter = next_iter.clone();
                continue 'main;
            }

            next_next_iter = next_iter.clone();
            if !model.iter_next(&next_next_iter) {
                // There is only one child of header left, so we remove it with header (H1 -> C1 -> END) -> (NOTHING)
                vec_tree_path_to_delete.push(model.path(&current_iter).unwrap());
                vec_tree_path_to_delete.push(model.path(&next_iter).unwrap());
                break 'main;
            }

            if model.value(&next_next_iter, column_color).get::<String>().unwrap() == HEADER_ROW_COLOR {
                // One child between two headers, we can remove them  (H1 -> C1 -> H2) -> (H2)
                vec_tree_path_to_delete.push(model.path(&current_iter).unwrap());
                vec_tree_path_to_delete.push(model.path(&next_iter).unwrap());
                current_iter = next_next_iter.clone();
                continue 'main;
            }

            loop {
                // (H1 -> C1 -> C2 -> Cn -> END) -> (NO CHANGE, BECAUSE IS GOOD)
                if !model.iter_next(&next_next_iter) {
                    break 'main;
                }
                // Move to next header
                if model.value(&next_next_iter, column_color).get::<String>().unwrap() == HEADER_ROW_COLOR {
                    current_iter = next_next_iter.clone();
                    continue 'main;
                }
            }
        }
        for tree_path in vec_tree_path_to_delete.iter().rev() {
            model.remove(&model.iter(tree_path).unwrap());
        }
    }

    // Last step, remove orphan header if exists
    if let Some(iter) = model.iter_first() {
        if !model.iter_next(&iter) {
            model.clear();
        }
    }

    text_view_errors.buffer().unwrap().set_text(messages.as_str());
}
fn move_with_list(tree_view: gtk::TreeView, column_file_name: i32, column_path: i32, column_selection: i32, gui_data: &GuiData, destination_folder: PathBuf) {
    let text_view_errors = gui_data.text_view_errors.clone();
    let entry_info = gui_data.entry_info.clone();

    let model = get_list_store(&tree_view);

    let mut messages: String = "".to_string();

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
