use crate::help_functions::*;
use gtk::prelude::*;

const KEY_ENTER: u16 = 36;
const KEY_SPACE: u16 = 65;

// TODO add option to open files and folders from context menu activated by pressing ONCE with right mouse button

pub fn opening_double_click_function_duplicates(tree_view: &gtk::TreeView, event: &gdk::EventButton) -> gtk::Inhibit {
    if event.event_type() == gdk::EventType::DoubleButtonPress && event.button() == 1 {
        common_open_function(tree_view, ColumnsDuplicates::Name as i32, ColumnsDuplicates::Path as i32, OpenMode::PathAndName);
    } else if event.event_type() == gdk::EventType::DoubleButtonPress && event.button() == 3 {
        common_open_function(tree_view, ColumnsDuplicates::Name as i32, ColumnsDuplicates::Path as i32, OpenMode::OnlyPath);
    }
    gtk::Inhibit(false)
}
pub fn opening_enter_function_duplicates(tree_view: &gtk::TreeView, event: &gdk::EventKey) -> gtk::Inhibit {
    handle_tree_keypress(tree_view, event, ColumnsDuplicates::Name as u32, ColumnsDuplicates::Path as u32, ColumnsDuplicates::ActiveSelectButton as u32)
}

pub fn opening_double_click_function_empty_folders(tree_view: &gtk::TreeView, event: &gdk::EventButton) -> gtk::Inhibit {
    if event.event_type() == gdk::EventType::DoubleButtonPress && event.button() == 1 {
        common_open_function(tree_view, ColumnsEmptyFolders::Name as i32, ColumnsEmptyFolders::Path as i32, OpenMode::PathAndName);
    } else if event.event_type() == gdk::EventType::DoubleButtonPress && event.button() == 3 {
        common_open_function(tree_view, ColumnsEmptyFolders::Name as i32, ColumnsEmptyFolders::Path as i32, OpenMode::OnlyPath);
    }
    gtk::Inhibit(false)
}
pub fn opening_enter_function_empty_folders(tree_view: &gtk::TreeView, event: &gdk::EventKey) -> gtk::Inhibit {
    handle_tree_keypress(tree_view, event, ColumnsEmptyFolders::Name as u32, ColumnsEmptyFolders::Path as u32, ColumnsEmptyFolders::ActiveSelectButton as u32)
}

pub fn opening_double_click_function_empty_files(tree_view: &gtk::TreeView, event: &gdk::EventButton) -> gtk::Inhibit {
    if event.event_type() == gdk::EventType::DoubleButtonPress && event.button() == 1 {
        common_open_function(tree_view, ColumnsEmptyFiles::Name as i32, ColumnsEmptyFiles::Path as i32, OpenMode::PathAndName);
    } else if event.event_type() == gdk::EventType::DoubleButtonPress && event.button() == 3 {
        common_open_function(tree_view, ColumnsEmptyFiles::Name as i32, ColumnsEmptyFiles::Path as i32, OpenMode::OnlyPath);
    }
    gtk::Inhibit(false)
}
pub fn opening_enter_function_empty_files(tree_view: &gtk::TreeView, event: &gdk::EventKey) -> gtk::Inhibit {
    handle_tree_keypress(tree_view, event, ColumnsEmptyFiles::Name as u32, ColumnsEmptyFiles::Path as u32, ColumnsEmptyFiles::ActiveSelectButton as u32)
}

pub fn opening_double_click_function_temporary_files(tree_view: &gtk::TreeView, event: &gdk::EventButton) -> gtk::Inhibit {
    if event.event_type() == gdk::EventType::DoubleButtonPress && event.button() == 1 {
        common_open_function(tree_view, ColumnsTemporaryFiles::Name as i32, ColumnsTemporaryFiles::Path as i32, OpenMode::PathAndName);
    } else if event.event_type() == gdk::EventType::DoubleButtonPress && event.button() == 3 {
        common_open_function(tree_view, ColumnsTemporaryFiles::Name as i32, ColumnsTemporaryFiles::Path as i32, OpenMode::OnlyPath);
    }
    gtk::Inhibit(false)
}
pub fn opening_enter_function_temporary_files(tree_view: &gtk::TreeView, event: &gdk::EventKey) -> gtk::Inhibit {
    handle_tree_keypress(tree_view, event, ColumnsTemporaryFiles::Name as u32, ColumnsTemporaryFiles::Path as u32, ColumnsTemporaryFiles::ActiveSelectButton as u32)
}

pub fn opening_double_click_function_big_files(tree_view: &gtk::TreeView, event: &gdk::EventButton) -> gtk::Inhibit {
    if event.event_type() == gdk::EventType::DoubleButtonPress && event.button() == 1 {
        common_open_function(tree_view, ColumnsBigFiles::Name as i32, ColumnsBigFiles::Path as i32, OpenMode::PathAndName);
    } else if event.event_type() == gdk::EventType::DoubleButtonPress && event.button() == 3 {
        common_open_function(tree_view, ColumnsBigFiles::Name as i32, ColumnsBigFiles::Path as i32, OpenMode::OnlyPath);
    }
    gtk::Inhibit(false)
}
pub fn opening_enter_function_big_files(tree_view: &gtk::TreeView, event: &gdk::EventKey) -> gtk::Inhibit {
    handle_tree_keypress(tree_view, event, ColumnsBigFiles::Name as u32, ColumnsBigFiles::Path as u32, ColumnsBigFiles::ActiveSelectButton as u32)
}

pub fn opening_double_click_function_zeroed_files(tree_view: &gtk::TreeView, event: &gdk::EventButton) -> gtk::Inhibit {
    if event.event_type() == gdk::EventType::DoubleButtonPress && event.button() == 1 {
        common_open_function(tree_view, ColumnsZeroedFiles::Name as i32, ColumnsZeroedFiles::Path as i32, OpenMode::PathAndName);
    } else if event.event_type() == gdk::EventType::DoubleButtonPress && event.button() == 3 {
        common_open_function(tree_view, ColumnsZeroedFiles::Name as i32, ColumnsZeroedFiles::Path as i32, OpenMode::OnlyPath);
    }
    gtk::Inhibit(false)
}
pub fn opening_enter_function_zeroed_files(tree_view: &gtk::TreeView, event: &gdk::EventKey) -> gtk::Inhibit {
    handle_tree_keypress(tree_view, event, ColumnsZeroedFiles::Name as u32, ColumnsZeroedFiles::Path as u32, ColumnsZeroedFiles::ActiveSelectButton as u32)
}

pub fn opening_double_click_function_same_music(tree_view: &gtk::TreeView, event: &gdk::EventButton) -> gtk::Inhibit {
    if event.event_type() == gdk::EventType::DoubleButtonPress && event.button() == 1 {
        common_open_function(tree_view, ColumnsSameMusic::Name as i32, ColumnsSameMusic::Path as i32, OpenMode::PathAndName);
    } else if event.event_type() == gdk::EventType::DoubleButtonPress && event.button() == 3 {
        common_open_function(tree_view, ColumnsSameMusic::Name as i32, ColumnsSameMusic::Path as i32, OpenMode::OnlyPath);
    }
    gtk::Inhibit(false)
}
pub fn opening_enter_function_same_music(tree_view: &gtk::TreeView, event: &gdk::EventKey) -> gtk::Inhibit {
    handle_tree_keypress(tree_view, event, ColumnsSameMusic::Name as u32, ColumnsSameMusic::Path as u32, ColumnsSameMusic::ActiveSelectButton as u32)
}

pub fn opening_double_click_function_similar_images(tree_view: &gtk::TreeView, event: &gdk::EventButton) -> gtk::Inhibit {
    if event.event_type() == gdk::EventType::DoubleButtonPress && event.button() == 1 {
        common_open_function(tree_view, ColumnsSimilarImages::Name as i32, ColumnsSimilarImages::Path as i32, OpenMode::PathAndName);
    } else if event.event_type() == gdk::EventType::DoubleButtonPress && event.button() == 3 {
        common_open_function(tree_view, ColumnsSimilarImages::Name as i32, ColumnsSimilarImages::Path as i32, OpenMode::OnlyPath);
    }
    gtk::Inhibit(false)
}

pub fn opening_enter_function_similar_images(tree_view: &gtk::TreeView, event: &gdk::EventKey) -> gtk::Inhibit {
    handle_tree_keypress(tree_view, event, ColumnsSimilarImages::Name as u32, ColumnsSimilarImages::Path as u32, ColumnsSimilarImages::ActiveSelectButton as u32)
}

pub fn opening_double_click_function_invalid_symlinks(tree_view: &gtk::TreeView, event: &gdk::EventButton) -> gtk::Inhibit {
    if event.event_type() == gdk::EventType::DoubleButtonPress && event.button() == 1 {
        common_open_function(tree_view, ColumnsInvalidSymlinks::Name as i32, ColumnsInvalidSymlinks::Path as i32, OpenMode::PathAndName);
    } else if event.event_type() == gdk::EventType::DoubleButtonPress && event.button() == 3 {
        common_open_function(tree_view, ColumnsInvalidSymlinks::Name as i32, ColumnsInvalidSymlinks::Path as i32, OpenMode::OnlyPath);
    }
    gtk::Inhibit(false)
}
pub fn opening_enter_function_invalid_symlinks(tree_view: &gtk::TreeView, event: &gdk::EventKey) -> gtk::Inhibit {
    handle_tree_keypress(tree_view, event, ColumnsInvalidSymlinks::Name as u32, ColumnsInvalidSymlinks::Path as u32, ColumnsInvalidSymlinks::ActiveSelectButton as u32)
}

pub fn opening_double_click_function_broken_files(tree_view: &gtk::TreeView, event: &gdk::EventButton) -> gtk::Inhibit {
    if event.event_type() == gdk::EventType::DoubleButtonPress && event.button() == 1 {
        common_open_function(tree_view, ColumnsBrokenFiles::Name as i32, ColumnsBrokenFiles::Path as i32, OpenMode::PathAndName);
    } else if event.event_type() == gdk::EventType::DoubleButtonPress && event.button() == 3 {
        common_open_function(tree_view, ColumnsBrokenFiles::Name as i32, ColumnsBrokenFiles::Path as i32, OpenMode::OnlyPath);
    }
    gtk::Inhibit(false)
}
pub fn opening_enter_function_broken_files(tree_view: &gtk::TreeView, event: &gdk::EventKey) -> gtk::Inhibit {
    handle_tree_keypress(tree_view, event, ColumnsBrokenFiles::Name as u32, ColumnsBrokenFiles::Path as u32, ColumnsBrokenFiles::ActiveSelectButton as u32)
}

enum OpenMode {
    OnlyPath,
    PathAndName,
}

fn common_mark_function(tree_view: &gtk::TreeView, column_name: u32) {
    let selection = tree_view.selection();
    let (selection_rows, tree_model) = selection.selected_rows();

    let model = get_list_store(tree_view);

    for tree_path in selection_rows.iter().rev() {
        let value = !tree_model.value(&tree_model.iter(tree_path).unwrap(), column_name as i32).get::<bool>().unwrap();
        model.set_value(&tree_model.iter(tree_path).unwrap(), column_name, &value.to_value());
    }
}

fn common_open_function(tree_view: &gtk::TreeView, column_name: i32, column_path: i32, opening_mode: OpenMode) {
    let selection = tree_view.selection();
    let (selection_rows, tree_model) = selection.selected_rows();

    for tree_path in selection_rows.iter().rev() {
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

fn handle_tree_keypress(tree_view: &gtk::TreeView, event: &gdk::EventKey, name_column: u32, path_column: u32, mark_column: u32) -> gtk::Inhibit {
    match event.keycode() {
        Some(KEY_ENTER) => {
            // Enter
            common_open_function(tree_view, name_column as i32, path_column as i32, OpenMode::PathAndName);
        }
        Some(KEY_SPACE) => {
            // Space
            common_mark_function(tree_view, mark_column);
        }
        _ => {}
    }
    gtk::Inhibit(false)
}
