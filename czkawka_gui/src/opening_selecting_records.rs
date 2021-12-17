use crate::help_functions::*;
use gdk::ModifierType;
use gtk::prelude::*;

// TODO add option to open files and folders from context menu activated by pressing ONCE with right mouse button

pub fn opening_enter_function_ported(event_controller: &gtk::EventControllerKey, _key_value: u32, key_code: u32, _modifier_type: ModifierType) -> bool {
    let tree_view = event_controller.widget().unwrap().downcast::<gtk::TreeView>().unwrap();
    #[cfg(debug_assertions)]
    {
        println!("key_code {}", key_code);
    }

    let nt_object = get_notebook_object_from_tree_view(&tree_view);
    handle_tree_keypress(&tree_view, key_code, nt_object.column_name, nt_object.column_path, nt_object.column_selection);
    false // True catches signal, and don't send it to function, e.g. up button is catched and don't move selection
}

// GTK 4
// pub fn opening_enter_function_ported_(event_controller: &gtk4::EventControllerKey, _key: gdk4::keys::Key, key_code: u32, _modifier_type: ModifierType) -> gtk4::Inhibit {
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

pub fn opening_double_click_function(tree_view: &gtk::TreeView, event: &gdk::EventButton) -> gtk::Inhibit {
    let nt_object = get_notebook_object_from_tree_view(tree_view);
    if event.event_type() == gdk::EventType::DoubleButtonPress && event.button() == 1 {
        common_open_function(tree_view, nt_object.column_name, nt_object.column_path, OpenMode::PathAndName);
    } else if event.event_type() == gdk::EventType::DoubleButtonPress && event.button() == 3 {
        common_open_function(tree_view, nt_object.column_name, nt_object.column_path, OpenMode::OnlyPath);
    }
    gtk::Inhibit(false)
}

enum OpenMode {
    OnlyPath,
    PathAndName,
}

fn common_mark_function(tree_view: &gtk::TreeView, column_name: i32) {
    let selection = tree_view.selection();
    let (selected_rows, tree_model) = selection.selected_rows();

    let model = get_list_store(tree_view);

    for tree_path in selected_rows.iter().rev() {
        let value = !tree_model.value(&tree_model.iter(tree_path).unwrap(), column_name).get::<bool>().unwrap();
        model.set_value(&tree_model.iter(tree_path).unwrap(), column_name as u32, &value.to_value());
    }
}

fn common_open_function(tree_view: &gtk::TreeView, column_name: i32, column_path: i32, opening_mode: OpenMode) {
    let selection = tree_view.selection();
    let (selected_rows, tree_model) = selection.selected_rows();

    for tree_path in selected_rows.iter().rev() {
        let end_path;
        let name = tree_model.value(&tree_model.iter(tree_path).unwrap(), column_name).get::<String>().unwrap();
        let path = tree_model.value(&tree_model.iter(tree_path).unwrap(), column_path).get::<String>().unwrap();

        match opening_mode {
            OpenMode::OnlyPath => {
                end_path = path;
            }
            OpenMode::PathAndName => {
                end_path = format!("{}/{}", path, name);
            }
        }

        open::that_in_background(&end_path);

        // if let Err(e) = open::that(&end_path) {
        //     println!("Failed to open {} - Error {}", end_path, e);
        // }
    }
}

fn handle_tree_keypress(tree_view: &gtk::TreeView, key_code: u32, name_column: i32, path_column: i32, mark_column: i32) {
    match key_code {
        KEY_ENTER => {
            // Enter
            common_open_function(tree_view, name_column, path_column, OpenMode::PathAndName);
        }
        KEY_SPACE => {
            // Space
            common_mark_function(tree_view, mark_column);
        }
        _ => {}
    }
}

pub fn select_function_duplicates(_tree_selection: &gtk::TreeSelection, tree_model: &gtk::TreeModel, tree_path: &gtk::TreePath, _is_path_currently_selected: bool) -> bool {
    let color = tree_model.value(&tree_model.iter(tree_path).unwrap(), ColumnsDuplicates::Color as i32).get::<String>().unwrap();

    if color == HEADER_ROW_COLOR {
        return false;
    }

    true
}

pub fn select_function_same_music(_tree_selection: &gtk::TreeSelection, tree_model: &gtk::TreeModel, tree_path: &gtk::TreePath, _is_path_currently_selected: bool) -> bool {
    let color = tree_model.value(&tree_model.iter(tree_path).unwrap(), ColumnsSameMusic::Color as i32).get::<String>().unwrap();

    if color == HEADER_ROW_COLOR {
        return false;
    }

    true
}

pub fn select_function_similar_images(_tree_selection: &gtk::TreeSelection, tree_model: &gtk::TreeModel, tree_path: &gtk::TreePath, _is_path_currently_selected: bool) -> bool {
    let color = tree_model.value(&tree_model.iter(tree_path).unwrap(), ColumnsSimilarImages::Color as i32).get::<String>().unwrap();

    if color == HEADER_ROW_COLOR {
        return false;
    }

    true
}

pub fn select_function_similar_videos(_tree_selection: &gtk::TreeSelection, tree_model: &gtk::TreeModel, tree_path: &gtk::TreePath, _is_path_currently_selected: bool) -> bool {
    let color = tree_model.value(&tree_model.iter(tree_path).unwrap(), ColumnsSimilarVideos::Color as i32).get::<String>().unwrap();

    if color == HEADER_ROW_COLOR {
        return false;
    }

    true
}
