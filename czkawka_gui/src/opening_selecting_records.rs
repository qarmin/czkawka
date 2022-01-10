use gdk::ModifierType;
use gtk::prelude::*;

use crate::help_functions::*;
use crate::notebook_enums::NotebookUpperEnum;

// TODO add option to open files and folders from context menu activated by pressing ONCE with right mouse button

pub fn opening_enter_function_ported(event_controller: &gtk::EventControllerKey, _key_value: u32, key_code: u32, _modifier_type: ModifierType) -> bool {
    let tree_view = event_controller.widget().unwrap().downcast::<gtk::TreeView>().unwrap();
    #[cfg(debug_assertions)]
    {
        println!("key_code {}", key_code);
    }

    let nt_object = get_notebook_object_from_tree_view(&tree_view);
    handle_tree_keypress(
        &tree_view,
        key_code,
        nt_object.column_name,
        nt_object.column_path,
        nt_object.column_selection,
        nt_object.column_color,
    );
    false // True catches signal, and don't send it to function, e.g. up button is catched and don't move selection
}

pub fn opening_enter_function_ported_upper_directories(event_controller: &gtk::EventControllerKey, _key_value: u32, key_code: u32, _modifier_type: ModifierType) -> bool {
    let tree_view = event_controller.widget().unwrap().downcast::<gtk::TreeView>().unwrap();
    #[cfg(debug_assertions)]
    {
        println!("key_code {}", key_code);
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
    false // True catches signal, and don't send it to function, e.g. up button is catched and don't move selection
}

pub fn opening_double_click_function(tree_view: &gtk::TreeView, event: &gdk::EventButton) -> gtk::Inhibit {
    let nt_object = get_notebook_object_from_tree_view(tree_view);
    if event.event_type() == gdk::EventType::DoubleButtonPress && event.button() == 1 {
        common_open_function(tree_view, nt_object.column_name, nt_object.column_path, OpenMode::PathAndName);
    } else if event.event_type() == gdk::EventType::DoubleButtonPress && event.button() == 3 {
        common_open_function(tree_view, nt_object.column_name, nt_object.column_path, OpenMode::OnlyPath);
    }
    gtk::Inhibit(false)
}

pub fn opening_middle_mouse_function(tree_view: &gtk::TreeView, event: &gdk::EventButton) -> gtk::Inhibit {
    let nt_object = get_notebook_object_from_tree_view(tree_view);
    if let Some(column_color) = nt_object.column_color {
        if event.button() == 2 {
            reverse_selection(tree_view, column_color, nt_object.column_selection);
        }
    }
    gtk::Inhibit(false)
}

pub fn opening_double_click_function_directories(tree_view: &gtk::TreeView, event: &gdk::EventButton) -> gtk::Inhibit {
    if event.event_type() == gdk::EventType::DoubleButtonPress && (event.button() == 1 || event.button() == 3) {
        match get_notebook_upper_enum_from_tree_view(tree_view) {
            NotebookUpperEnum::IncludedDirectories => {
                common_open_function_upper_directories(tree_view, ColumnsIncludedDirectory::Path as i32);
            }
            NotebookUpperEnum::ExcludedDirectories => {
                common_open_function_upper_directories(tree_view, ColumnsExcludedDirectory::Path as i32);
            }
            _ => {
                panic!()
            }
        }
    }
    gtk::Inhibit(false)
}

// // GTK 4
// pub fn opening_enter_function_ported(event_controller: &gtk4::EventControllerKey, _key: gdk4::keys::Key, key_code: u32, _modifier_type: ModifierType) -> gtk4::Inhibit {
//     let tree_view = event_controller.widget().unwrap().downcast::<gtk4::TreeView>().unwrap();
//     #[cfg(debug_assertions)]
//         {
//             println!("key_code {}", key_code);
//         }
//
//     let nt_object = get_notebook_object_from_tree_view(&tree_view);
//     handle_tree_keypress(&tree_view, key_code, nt_object.column_name, nt_object.column_path, nt_object.column_selection);
//     Inhibit(false) // True catches signal, and don't send it to function, e.g. up button is catched and don't move selection
// }
//
// pub fn opening_double_click_function(gesture_click: &GestureClick, number_of_clicks: i32, _b: f64, _c: f64) {
//     let tree_view = gesture_click.widget().unwrap().downcast::<gtk4::TreeView>().unwrap();
//
//     let nt_object = get_notebook_object_from_tree_view(&tree_view);
//     if number_of_clicks == 2 {
//         if gesture_click.current_button() == 1 {
//             common_open_function(&tree_view, nt_object.column_name, nt_object.column_path, OpenMode::PathAndName);
//         } else if gesture_click.current_button() == 3 {
//             common_open_function(&tree_view, nt_object.column_name, nt_object.column_path, OpenMode::OnlyPath);
//         }
//     }
// }

enum OpenMode {
    OnlyPath,
    PathAndName,
}

fn common_mark_function(tree_view: &gtk::TreeView, column_selection: i32, column_color: Option<i32>) {
    let selection = tree_view.selection();
    let (selected_rows, tree_model) = selection.selected_rows();

    let model = get_list_store(tree_view);

    for tree_path in selected_rows.iter().rev() {
        if let Some(column_color) = column_color {
            if model.value(&model.iter(tree_path).unwrap(), column_color).get::<String>().unwrap() == HEADER_ROW_COLOR {
                continue;
            }
        }
        let value = !tree_model.value(&tree_model.iter(tree_path).unwrap(), column_selection).get::<bool>().unwrap();
        model.set_value(&tree_model.iter(tree_path).unwrap(), column_selection as u32, &value.to_value());
    }
}

fn common_open_function(tree_view: &gtk::TreeView, column_name: i32, column_path: i32, opening_mode: OpenMode) {
    let selection = tree_view.selection();
    let (selected_rows, tree_model) = selection.selected_rows();

    for tree_path in selected_rows.iter().rev() {
        let name = tree_model.value(&tree_model.iter(tree_path).unwrap(), column_name).get::<String>().unwrap();
        let path = tree_model.value(&tree_model.iter(tree_path).unwrap(), column_path).get::<String>().unwrap();

        let end_path = match opening_mode {
            OpenMode::OnlyPath => path,
            OpenMode::PathAndName => get_full_name_from_path_name(&path, &name),
        };

        open::that_in_background(&end_path);

        // if let Err(e) = open::that(&end_path) {
        //     println!("Failed to open {} - Error {}", end_path, e);
        // }
    }
}
fn reverse_selection(tree_view: &gtk::TreeView, column_color: i32, column_selection: i32) {
    let (selected_rows, model) = tree_view.selection().selected_rows();
    let model = model.downcast::<gtk::ListStore>().unwrap();

    if selected_rows.len() != 1 {
        return; // Multiple selection is not supported because it is a lot of harder to do it properly
    }
    let tree_path = selected_rows[0].clone();
    let current_iter = model.iter(&tree_path).unwrap();

    if model.value(&current_iter, column_color).get::<String>().unwrap() == HEADER_ROW_COLOR {
        return; // Selecting header is not supported(this is available by using reference)
    }

    // This will revert selection of current selected item, but I don't think that this is needed
    // let current_value = model.value(&current_iter, column_selection).get::<bool>().unwrap();
    // model.set_value(&current_iter, column_selection as u32, &(!current_value).to_value());

    let to_upper_iter = current_iter.clone();
    loop {
        if !model.iter_previous(&to_upper_iter) {
            break;
        }
        if model.value(&to_upper_iter, column_color).get::<String>().unwrap() == HEADER_ROW_COLOR {
            break;
        }

        let current_value = model.value(&to_upper_iter, column_selection).get::<bool>().unwrap();
        model.set_value(&to_upper_iter, column_selection as u32, &(!current_value).to_value());
    }

    let to_lower_iter = current_iter;
    loop {
        if !model.iter_next(&to_lower_iter) {
            break;
        }
        if model.value(&to_lower_iter, column_color).get::<String>().unwrap() == HEADER_ROW_COLOR {
            break;
        }

        let current_value = model.value(&to_lower_iter, column_selection).get::<bool>().unwrap();
        model.set_value(&to_lower_iter, column_selection as u32, &(!current_value).to_value());
    }
}

fn common_open_function_upper_directories(tree_view: &gtk::TreeView, column_full_path: i32) {
    let selection = tree_view.selection();
    let (selected_rows, tree_model) = selection.selected_rows();

    for tree_path in selected_rows.iter().rev() {
        let full_path = tree_model.value(&tree_model.iter(tree_path).unwrap(), column_full_path).get::<String>().unwrap();

        open::that_in_background(&full_path);
    }
}

fn handle_tree_keypress_upper_directories(tree_view: &gtk::TreeView, key_code: u32, full_path_column: i32, mark_column: Option<i32>) {
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

fn handle_tree_keypress(tree_view: &gtk::TreeView, key_code: u32, name_column: i32, path_column: i32, mark_column: i32, column_color: Option<i32>) {
    match key_code {
        KEY_ENTER => {
            common_open_function(tree_view, name_column, path_column, OpenMode::PathAndName);
        }
        KEY_SPACE => {
            common_mark_function(tree_view, mark_column, column_color);
        }
        _ => {}
    }
}

pub fn select_function_duplicates(_tree_selection: &gtk::TreeSelection, tree_model: &gtk::TreeModel, tree_path: &gtk::TreePath, _is_path_currently_selected: bool) -> bool {
    let color = tree_model
        .value(&tree_model.iter(tree_path).unwrap(), ColumnsDuplicates::Color as i32)
        .get::<String>()
        .unwrap();

    if color == HEADER_ROW_COLOR {
        return false;
    }

    true
}

pub fn select_function_same_music(_tree_selection: &gtk::TreeSelection, tree_model: &gtk::TreeModel, tree_path: &gtk::TreePath, _is_path_currently_selected: bool) -> bool {
    let color = tree_model
        .value(&tree_model.iter(tree_path).unwrap(), ColumnsSameMusic::Color as i32)
        .get::<String>()
        .unwrap();

    if color == HEADER_ROW_COLOR {
        return false;
    }

    true
}

pub fn select_function_similar_images(_tree_selection: &gtk::TreeSelection, tree_model: &gtk::TreeModel, tree_path: &gtk::TreePath, _is_path_currently_selected: bool) -> bool {
    let color = tree_model
        .value(&tree_model.iter(tree_path).unwrap(), ColumnsSimilarImages::Color as i32)
        .get::<String>()
        .unwrap();

    if color == HEADER_ROW_COLOR {
        return false;
    }

    true
}

pub fn select_function_similar_videos(_tree_selection: &gtk::TreeSelection, tree_model: &gtk::TreeModel, tree_path: &gtk::TreePath, _is_path_currently_selected: bool) -> bool {
    let color = tree_model
        .value(&tree_model.iter(tree_path).unwrap(), ColumnsSimilarVideos::Color as i32)
        .get::<String>()
        .unwrap();

    if color == HEADER_ROW_COLOR {
        return false;
    }

    true
}
pub fn select_function_always_true(_tree_selection: &gtk::TreeSelection, _tree_model: &gtk::TreeModel, _tree_path: &gtk::TreePath, _is_path_currently_selected: bool) -> bool {
    true
}
