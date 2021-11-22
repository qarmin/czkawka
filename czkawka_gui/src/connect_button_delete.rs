use crate::gui_data::GuiData;
use crate::help_functions::*;
use crate::notebook_enums::*;
use gtk::prelude::*;
use gtk::Align;
use std::collections::BTreeMap;
use std::fs;
use std::fs::Metadata;

// TODO add support for checking if really symlink doesn't point to correct directory/file

pub fn connect_button_delete(gui_data: &GuiData) {
    let gui_data = gui_data.clone();
    let buttons_delete = gui_data.bottom_buttons.buttons_delete.clone();
    let tree_view_duplicate_finder = gui_data.main_notebook.tree_view_duplicate_finder.clone();
    let notebook_main = gui_data.main_notebook.notebook_main.clone();
    let window_main = gui_data.window_main.clone();
    let tree_view_empty_folder_finder = gui_data.main_notebook.tree_view_empty_folder_finder.clone();
    let tree_view_big_files_finder = gui_data.main_notebook.tree_view_big_files_finder.clone();
    let tree_view_empty_files_finder = gui_data.main_notebook.tree_view_empty_files_finder.clone();
    let tree_view_temporary_files_finder = gui_data.main_notebook.tree_view_temporary_files_finder.clone();
    let tree_view_similar_images_finder = gui_data.main_notebook.tree_view_similar_images_finder.clone();
    let tree_view_zeroed_files_finder = gui_data.main_notebook.tree_view_zeroed_files_finder.clone();
    let tree_view_same_music_finder = gui_data.main_notebook.tree_view_same_music_finder.clone();
    let tree_view_invalid_symlinks = gui_data.main_notebook.tree_view_invalid_symlinks.clone();
    let tree_view_broken_files = gui_data.main_notebook.tree_view_broken_files.clone();
    let check_button_settings_confirm_deletion = gui_data.settings.check_button_settings_confirm_deletion.clone();
    let check_button_settings_confirm_group_deletion = gui_data.settings.check_button_settings_confirm_group_deletion.clone();
    let image_preview_similar_images = gui_data.main_notebook.image_preview_similar_images.clone();

    buttons_delete.connect_clicked(move |_| {
        if !check_if_can_delete_files(&check_button_settings_confirm_deletion, &window_main) {
            return;
        }

        match to_notebook_main_enum(notebook_main.current_page().unwrap()) {
            NotebookMainEnum::Duplicate => {
                if !check_button_settings_confirm_group_deletion.is_active()
                    || !check_if_deleting_all_files_in_group(
                        &tree_view_duplicate_finder.clone(),
                        ColumnsDuplicates::Color as i32,
                        ColumnsDuplicates::ActiveSelectButton as i32,
                        &window_main,
                        &check_button_settings_confirm_group_deletion,
                    )
                {
                    tree_remove(
                        &tree_view_duplicate_finder.clone(),
                        ColumnsDuplicates::Name as i32,
                        ColumnsDuplicates::Path as i32,
                        ColumnsDuplicates::Color as i32,
                        ColumnsDuplicates::ActiveSelectButton as i32,
                        &gui_data,
                    );
                }
            }
            NotebookMainEnum::EmptyDirectories => {
                empty_folder_remover(
                    &tree_view_empty_folder_finder.clone(),
                    ColumnsEmptyFolders::Name as i32,
                    ColumnsEmptyFolders::Path as i32,
                    ColumnsEmptyFolders::ActiveSelectButton as i32,
                    &gui_data,
                );
            }
            NotebookMainEnum::EmptyFiles => {
                basic_remove(
                    &tree_view_empty_files_finder.clone(),
                    ColumnsEmptyFiles::Name as i32,
                    ColumnsEmptyFiles::Path as i32,
                    ColumnsEmptyFiles::ActiveSelectButton as i32,
                    &gui_data,
                );
            }
            NotebookMainEnum::Temporary => {
                basic_remove(
                    &tree_view_temporary_files_finder.clone(),
                    ColumnsTemporaryFiles::Name as i32,
                    ColumnsTemporaryFiles::Path as i32,
                    ColumnsTemporaryFiles::ActiveSelectButton as i32,
                    &gui_data,
                );
            }
            NotebookMainEnum::BigFiles => {
                basic_remove(&tree_view_big_files_finder.clone(), ColumnsBigFiles::Name as i32, ColumnsBigFiles::Path as i32, ColumnsBigFiles::ActiveSelectButton as i32, &gui_data);
            }
            NotebookMainEnum::SimilarImages => {
                if !check_button_settings_confirm_group_deletion.is_active()
                    || !check_if_deleting_all_files_in_group(
                        &tree_view_similar_images_finder.clone(),
                        ColumnsSimilarImages::Color as i32,
                        ColumnsSimilarImages::ActiveSelectButton as i32,
                        &window_main,
                        &check_button_settings_confirm_group_deletion,
                    )
                {
                    tree_remove(
                        &tree_view_similar_images_finder.clone(),
                        ColumnsSimilarImages::Name as i32,
                        ColumnsSimilarImages::Path as i32,
                        ColumnsSimilarImages::Color as i32,
                        ColumnsSimilarImages::ActiveSelectButton as i32,
                        &gui_data,
                    );
                    image_preview_similar_images.hide();
                }
            }
            NotebookMainEnum::Zeroed => {
                basic_remove(
                    &tree_view_zeroed_files_finder.clone(),
                    ColumnsZeroedFiles::Name as i32,
                    ColumnsZeroedFiles::Path as i32,
                    ColumnsZeroedFiles::ActiveSelectButton as i32,
                    &gui_data,
                );
            }
            NotebookMainEnum::SameMusic => {
                if !check_button_settings_confirm_group_deletion.is_active()
                    || !check_if_deleting_all_files_in_group(
                        &tree_view_same_music_finder.clone(),
                        ColumnsSameMusic::Color as i32,
                        ColumnsSameMusic::ActiveSelectButton as i32,
                        &window_main,
                        &check_button_settings_confirm_group_deletion,
                    )
                {
                    tree_remove(
                        &tree_view_same_music_finder.clone(),
                        ColumnsSameMusic::Name as i32,
                        ColumnsSameMusic::Path as i32,
                        ColumnsSameMusic::Color as i32,
                        ColumnsSameMusic::ActiveSelectButton as i32,
                        &gui_data,
                    );
                }
            }
            NotebookMainEnum::Symlinks => {
                basic_remove(
                    &tree_view_invalid_symlinks.clone(),
                    ColumnsInvalidSymlinks::Name as i32,
                    ColumnsInvalidSymlinks::Path as i32,
                    ColumnsInvalidSymlinks::ActiveSelectButton as i32,
                    &gui_data,
                );
            }
            NotebookMainEnum::BrokenFiles => {
                basic_remove(
                    &tree_view_broken_files.clone(),
                    ColumnsBrokenFiles::Name as i32,
                    ColumnsBrokenFiles::Path as i32,
                    ColumnsInvalidSymlinks::ActiveSelectButton as i32,
                    &gui_data,
                );
            }
        }
    });
}

pub fn check_if_can_delete_files(check_button_settings_confirm_deletion: &gtk::CheckButton, window_main: &gtk::Window) -> bool {
    if check_button_settings_confirm_deletion.is_active() {
        let confirmation_dialog_delete = gtk::Dialog::with_buttons(
            Some("Delete confirmation"),
            Some(window_main),
            gtk::DialogFlags::DESTROY_WITH_PARENT,
            &[("Ok", gtk::ResponseType::Ok), ("Close", gtk::ResponseType::Cancel)],
        );
        let label: gtk::Label = gtk::Label::new(Some("Are you sure that you want to delete files?"));
        let check_button: gtk::CheckButton = gtk::CheckButton::with_label("Ask next time");
        check_button.set_active(true);
        check_button.set_halign(Align::Center);

        let button_box = confirmation_dialog_delete.children()[0].clone().downcast::<gtk::Box>().unwrap().children()[0].clone().downcast::<gtk::Box>().unwrap().children()[0]
            .clone()
            .downcast::<gtk::ButtonBox>()
            .unwrap();

        let button_ok = button_box.children()[0].clone();
        button_ok.grab_focus();

        let internal_box = confirmation_dialog_delete.children()[0].clone().downcast::<gtk::Box>().unwrap();
        internal_box.add(&label);
        internal_box.add(&check_button);

        confirmation_dialog_delete.show_all();

        let response_type = confirmation_dialog_delete.run();
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

pub fn check_if_deleting_all_files_in_group(tree_view: &gtk::TreeView, column_color: i32, column_selection: i32, window_main: &gtk::Window, check_button_settings_confirm_group_deletion: &gtk::CheckButton) -> bool {
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
        let confirmation_dialog_group_delete = gtk::Dialog::with_buttons(
            Some("Confirmation of deleting all files in group"),
            Some(window_main),
            gtk::DialogFlags::MODAL,
            &[("Ok", gtk::ResponseType::Ok), ("Close", gtk::ResponseType::Cancel)],
        );
        let label: gtk::Label = gtk::Label::new(Some("In some groups there are selected all records."));
        let label2: gtk::Label = gtk::Label::new(Some("Are you sure that you want to delete them?"));
        let check_button: gtk::CheckButton = gtk::CheckButton::with_label("Ask next time");
        check_button.set_active(true);
        check_button.set_halign(Align::Center);

        let button_box = confirmation_dialog_group_delete.children()[0].clone().downcast::<gtk::Box>().unwrap().children()[0]
            .clone()
            .downcast::<gtk::Box>()
            .unwrap()
            .children()[0]
            .clone()
            .downcast::<gtk::ButtonBox>()
            .unwrap();

        let button_ok = button_box.children()[0].clone();
        button_ok.grab_focus();

        let internal_box = confirmation_dialog_group_delete.children()[0].clone().downcast::<gtk::Box>().unwrap();
        internal_box.add(&label);
        internal_box.add(&label2);
        internal_box.add(&check_button);

        confirmation_dialog_group_delete.show_all();

        let response_type = confirmation_dialog_group_delete.run();
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

pub fn empty_folder_remover(tree_view: &gtk::TreeView, column_file_name: i32, column_path: i32, column_selection: i32, gui_data: &GuiData) {
    let text_view_errors = gui_data.text_view_errors.clone();
    let use_trash = gui_data.settings.check_button_settings_use_trash.clone().is_active();

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

pub fn basic_remove(tree_view: &gtk::TreeView, column_file_name: i32, column_path: i32, column_selection: i32, gui_data: &GuiData) {
    let text_view_errors = gui_data.text_view_errors.clone();
    let use_trash = gui_data.settings.check_button_settings_use_trash.clone().is_active();

    let model = get_list_store(tree_view);

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

    // Must be deleted from end to start, because when deleting entries, TreePath(and also TreeIter) will points to invalid data
    for tree_path in selection_rows.iter().rev() {
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
pub fn tree_remove(tree_view: &gtk::TreeView, column_file_name: i32, column_path: i32, column_color: i32, column_selection: i32, gui_data: &GuiData) {
    let text_view_errors = gui_data.text_view_errors.clone();
    let use_trash = gui_data.settings.check_button_settings_use_trash.clone().is_active();

    let model = get_list_store(tree_view);

    let mut messages: String = "".to_string();

    let mut vec_path_to_delete: Vec<(String, String)> = Vec::new();
    let mut map_with_path_to_delete: BTreeMap<String, Vec<String>> = Default::default(); // BTreeMap<Path,Vec<FileName>>

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

    // Save to variable paths of files, and remove it when not removing all occurrences.
    for tree_path in selection_rows.iter().rev() {
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
