use crate::help_functions::*;
use gtk::prelude::*;

pub fn opening_double_click_function_duplicates(tree_view: &gtk::TreeView, event: &gdk::EventButton) -> gtk::Inhibit {
    if event.get_event_type() == gdk::EventType::DoubleButtonPress {
        common_open_function(tree_view, ColumnsDuplicates::Name as i32, ColumnsDuplicates::Path as i32);
    }
    gtk::Inhibit(false)
}
pub fn opening_double_click_function_empty_folders(tree_view: &gtk::TreeView, event: &gdk::EventButton) -> gtk::Inhibit {
    if event.get_event_type() == gdk::EventType::DoubleButtonPress {
        common_open_function(tree_view, ColumnsEmptyFolders::Name as i32, ColumnsEmptyFolders::Path as i32);
    }
    gtk::Inhibit(false)
}
pub fn opening_double_click_function_empty_files(tree_view: &gtk::TreeView, event: &gdk::EventButton) -> gtk::Inhibit {
    if event.get_event_type() == gdk::EventType::DoubleButtonPress {
        common_open_function(tree_view, ColumnsEmptyFiles::Name as i32, ColumnsEmptyFiles::Path as i32);
    }
    gtk::Inhibit(false)
}

pub fn opening_double_click_function_temporary_files(tree_view: &gtk::TreeView, event: &gdk::EventButton) -> gtk::Inhibit {
    if event.get_event_type() == gdk::EventType::DoubleButtonPress {
        common_open_function(tree_view, ColumnsTemporaryFiles::Name as i32, ColumnsTemporaryFiles::Path as i32);
    }
    gtk::Inhibit(false)
}

pub fn opening_double_click_function_big_files(tree_view: &gtk::TreeView, event: &gdk::EventButton) -> gtk::Inhibit {
    if event.get_event_type() == gdk::EventType::DoubleButtonPress {
        common_open_function(tree_view, ColumnsBigFiles::Name as i32, ColumnsBigFiles::Path as i32);
    }
    gtk::Inhibit(false)
}

pub fn opening_double_click_function_zeroed_files(tree_view: &gtk::TreeView, event: &gdk::EventButton) -> gtk::Inhibit {
    if event.get_event_type() == gdk::EventType::DoubleButtonPress {
        common_open_function(tree_view, ColumnsZeroedFiles::Name as i32, ColumnsZeroedFiles::Path as i32);
    }
    gtk::Inhibit(false)
}

pub fn opening_double_click_function_same_music(tree_view: &gtk::TreeView, event: &gdk::EventButton) -> gtk::Inhibit {
    if event.get_event_type() == gdk::EventType::DoubleButtonPress {
        common_open_function(tree_view, ColumnsSameMusic::Name as i32, ColumnsSameMusic::Path as i32);
    }
    gtk::Inhibit(false)
}

pub fn opening_double_click_function_similar_images(tree_view: &gtk::TreeView, event: &gdk::EventButton) -> gtk::Inhibit {
    if event.get_event_type() == gdk::EventType::DoubleButtonPress {
        common_open_function(tree_view, ColumnsSimilarImages::Name as i32, ColumnsSimilarImages::Path as i32);
    }
    gtk::Inhibit(false)
}

pub fn opening_double_click_function_invalid_symlinks(tree_view: &gtk::TreeView, event: &gdk::EventButton) -> gtk::Inhibit {
    if event.get_event_type() == gdk::EventType::DoubleButtonPress {
        common_open_function(tree_view, ColumnsInvalidSymlinks::Name as i32, ColumnsInvalidSymlinks::Path as i32);
    }
    gtk::Inhibit(false)
}

pub fn common_open_function(tree_view: &gtk::TreeView, column_name: i32, column_path: i32) {
    let selection = tree_view.get_selection();
    let (selection_rows, tree_model) = selection.get_selected_rows();

    for tree_path in selection_rows.iter().rev() {
        let name = tree_model.get_value(&tree_model.get_iter(tree_path).unwrap(), column_name).get::<String>().unwrap().unwrap();
        let path = tree_model.get_value(&tree_model.get_iter(tree_path).unwrap(), column_path).get::<String>().unwrap().unwrap();

        if open::that(format!("{}/{}", path, name)).is_err() {
            println!("Failed to open {}/{}", path, name);
        }
    }
}
