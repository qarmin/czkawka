use gdk4::{Key, ModifierType};
use gtk4::prelude::*;
use gtk4::GestureClick;

use crate::help_functions::*;
use crate::notebook_enums::NotebookUpperEnum;

// TODO add option to open files and folders from context menu activated by pressing ONCE with right mouse button

pub fn opening_enter_function_ported_upper_directories(
    event_controller: &gtk4::EventControllerKey,
    _key_value: Key,
    key_code: u32,
    _modifier_type: ModifierType,
) -> glib::Propagation {
    let tree_view = event_controller
        .widget()
        .expect("Item has no widget")
        .downcast::<gtk4::TreeView>()
        .expect("Widget is not TreeView");

    if cfg!(debug_assertions) {
        println!("key_code {key_code}");
    }

    match get_notebook_upper_enum_from_tree_view(&tree_view) {
        NotebookUpperEnum::IncludedDirectories => {
            handle_tree_keypress_upper_directories(
                &tree_view,
                key_code,
                ColumnsIncludedDirectory::Path as i32,
                Some(ColumnsIncludedDirectory::ReferenceButton as i32),
            );
        }
        NotebookUpperEnum::ExcludedDirectories => {
            handle_tree_keypress_upper_directories(&tree_view, key_code, ColumnsExcludedDirectory::Path as i32, None);
        }
        _ => {
            panic!()
        }
    }
    // false // True catches signal, and don't send it to function, e.g. up button is caught and don't move selection
    glib::Propagation::Proceed
}

pub fn opening_middle_mouse_function(gesture_click: &GestureClick, _number_of_clicks: i32, _b: f64, _c: f64) {
    let tree_view = gesture_click
        .widget()
        .expect("Item has no widget")
        .downcast::<gtk4::TreeView>()
        .expect("Widget is not TreeView");

    let nt_object = get_notebook_object_from_tree_view(&tree_view);
    if let Some(column_header) = nt_object.column_header {
        if gesture_click.current_button() == 2 {
            reverse_selection(&tree_view, column_header, nt_object.column_selection);
        }
    }
}

pub fn opening_double_click_function_directories(gesture_click: &GestureClick, number_of_clicks: i32, _b: f64, _c: f64) {
    let tree_view = gesture_click
        .widget()
        .expect("Item has no widget")
        .downcast::<gtk4::TreeView>()
        .expect("Widget is not TreeView");

    if number_of_clicks == 2 && (gesture_click.current_button() == 1 || gesture_click.current_button() == 3) {
        match get_notebook_upper_enum_from_tree_view(&tree_view) {
            NotebookUpperEnum::IncludedDirectories => {
                common_open_function_upper_directories(&tree_view, ColumnsIncludedDirectory::Path as i32);
            }
            NotebookUpperEnum::ExcludedDirectories => {
                common_open_function_upper_directories(&tree_view, ColumnsExcludedDirectory::Path as i32);
            }
            _ => {
                panic!()
            }
        }
    }
}

pub fn opening_enter_function_ported(event_controller: &gtk4::EventControllerKey, _key: Key, key_code: u32, _modifier_type: ModifierType) -> glib::Propagation {
    let tree_view = event_controller
        .widget()
        .expect("Item has no widget")
        .downcast::<gtk4::TreeView>()
        .expect("Widget is not TreeView");
    if cfg!(debug_assertions) {
        println!("key_code {key_code}");
    }

    let nt_object = get_notebook_object_from_tree_view(&tree_view);
    handle_tree_keypress(
        &tree_view,
        key_code,
        nt_object.column_name,
        nt_object.column_path,
        nt_object.column_selection,
        nt_object.column_header,
    );
    glib::Propagation::Proceed // True catches signal, and don't send it to function, e.g. up button is caught and don't move selection
}

pub fn opening_double_click_function(gesture_click: &GestureClick, number_of_clicks: i32, _b: f64, _c: f64) {
    let tree_view = gesture_click
        .widget()
        .expect("Item has no widget")
        .downcast::<gtk4::TreeView>()
        .expect("Widget is not TreeView");

    let nt_object = get_notebook_object_from_tree_view(&tree_view);
    if number_of_clicks == 2 {
        if gesture_click.current_button() == 1 {
            common_open_function(&tree_view, nt_object.column_name, nt_object.column_path, &OpenMode::PathAndName);
        } else if gesture_click.current_button() == 3 {
            common_open_function(&tree_view, nt_object.column_name, nt_object.column_path, &OpenMode::OnlyPath);
        }
    }
}

enum OpenMode {
    OnlyPath,
    PathAndName,
}

fn common_mark_function(tree_view: &gtk4::TreeView, column_selection: i32, column_header: Option<i32>) {
    let selection = tree_view.selection();
    let (selected_rows, tree_model) = selection.selected_rows();

    let model = get_list_store(tree_view);

    for tree_path in selected_rows.iter().rev() {
        if let Some(column_header) = column_header {
            if model.get::<bool>(&model.iter(tree_path).expect("Using invalid tree_path"), column_header) {
                continue;
            }
        }
        let value = !tree_model.get::<bool>(&tree_model.iter(tree_path).expect("Invalid tree_path"), column_selection);
        model.set_value(&tree_model.iter(tree_path).expect("Invalid tree_path"), column_selection as u32, &value.to_value());
    }
}

fn common_open_function(tree_view: &gtk4::TreeView, column_name: i32, column_path: i32, opening_mode: &OpenMode) {
    let selection = tree_view.selection();
    let (selected_rows, tree_model) = selection.selected_rows();

    for tree_path in selected_rows.iter().rev() {
        let name = tree_model.get::<String>(&tree_model.iter(tree_path).expect("Invalid tree_path"), column_name);
        let path = tree_model.get::<String>(&tree_model.iter(tree_path).expect("Invalid tree_path"), column_path);

        let end_path = match opening_mode {
            OpenMode::OnlyPath => path,
            OpenMode::PathAndName => get_full_name_from_path_name(&path, &name),
        };

        if let Err(e) = open::that(&end_path) {
            println!("Failed to open file {end_path}, reason {e}");
        };
    }
}

fn reverse_selection(tree_view: &gtk4::TreeView, column_header: i32, column_selection: i32) {
    let (selected_rows, model) = tree_view.selection().selected_rows();
    let model = model.downcast::<gtk4::ListStore>().expect("Invalid list store model");

    if selected_rows.len() != 1 {
        return; // Multiple selection is not supported because it is a lot of harder to do it properly
    }
    let tree_path = selected_rows[0].clone();
    let current_iter = model.iter(&tree_path).expect("Invalid tree_path");

    if model.get::<bool>(&current_iter, column_header) {
        return; // Selecting header is not supported(this is available by using reference)
    }

    // This will revert selection of current selected item, but I don't think that this is needed
    // let current_value = model.get::<bool>(&current_iter, column_selection);
    // model.set_value(&current_iter, column_selection as u32, &(!current_value).to_value());

    let to_upper_iter = current_iter;
    loop {
        if !model.iter_previous(&to_upper_iter) {
            break;
        }
        if model.get::<bool>(&to_upper_iter, column_header) {
            break;
        }

        let current_value = model.get::<bool>(&to_upper_iter, column_selection);
        model.set_value(&to_upper_iter, column_selection as u32, &(!current_value).to_value());
    }

    let to_lower_iter = current_iter;
    loop {
        if !model.iter_next(&to_lower_iter) {
            break;
        }
        if model.get::<bool>(&to_lower_iter, column_header) {
            break;
        }

        let current_value = model.get::<bool>(&to_lower_iter, column_selection);
        model.set_value(&to_lower_iter, column_selection as u32, &(!current_value).to_value());
    }
}

fn common_open_function_upper_directories(tree_view: &gtk4::TreeView, column_full_path: i32) {
    let selection = tree_view.selection();
    let (selected_rows, tree_model) = selection.selected_rows();

    for tree_path in selected_rows.iter().rev() {
        let full_path = tree_model.get::<String>(&tree_model.iter(tree_path).expect("Invalid tree_path"), column_full_path);

        if let Err(e) = open::that(&full_path) {
            println!("Failed to open file {full_path}, reason {e}");
        };
    }
}

fn handle_tree_keypress_upper_directories(tree_view: &gtk4::TreeView, key_code: u32, full_path_column: i32, mark_column: Option<i32>) {
    match key_code {
        KEY_ENTER => {
            common_open_function_upper_directories(tree_view, full_path_column);
        }
        KEY_SPACE => {
            if let Some(mark_column) = mark_column {
                common_mark_function(tree_view, mark_column, None);
            }
        }
        _ => {}
    }
}

fn handle_tree_keypress(tree_view: &gtk4::TreeView, key_code: u32, name_column: i32, path_column: i32, mark_column: i32, column_header: Option<i32>) {
    match key_code {
        KEY_ENTER => {
            common_open_function(tree_view, name_column, path_column, &OpenMode::PathAndName);
        }
        KEY_SPACE => {
            common_mark_function(tree_view, mark_column, column_header);
        }
        _ => {}
    }
}

pub fn select_function_duplicates(_tree_selection: &gtk4::TreeSelection, tree_model: &gtk4::TreeModel, tree_path: &gtk4::TreePath, _is_path_currently_selected: bool) -> bool {
    !tree_model.get::<bool>(&tree_model.iter(tree_path).expect("Invalid tree_path"), ColumnsDuplicates::IsHeader as i32)
}

pub fn select_function_same_music(_tree_selection: &gtk4::TreeSelection, tree_model: &gtk4::TreeModel, tree_path: &gtk4::TreePath, _is_path_currently_selected: bool) -> bool {
    !tree_model.get::<bool>(&tree_model.iter(tree_path).expect("Invalid tree_path"), ColumnsSameMusic::IsHeader as i32)
}

pub fn select_function_similar_images(_tree_selection: &gtk4::TreeSelection, tree_model: &gtk4::TreeModel, tree_path: &gtk4::TreePath, _is_path_currently_selected: bool) -> bool {
    !tree_model.get::<bool>(&tree_model.iter(tree_path).expect("Invalid tree_path"), ColumnsSimilarImages::IsHeader as i32)
}

pub fn select_function_similar_videos(_tree_selection: &gtk4::TreeSelection, tree_model: &gtk4::TreeModel, tree_path: &gtk4::TreePath, _is_path_currently_selected: bool) -> bool {
    !tree_model.get::<bool>(&tree_model.iter(tree_path).expect("Invalid tree_path"), ColumnsSimilarVideos::IsHeader as i32)
}

pub fn select_function_always_true(_tree_selection: &gtk4::TreeSelection, _tree_model: &gtk4::TreeModel, _tree_path: &gtk4::TreePath, _is_path_currently_selected: bool) -> bool {
    true
}
