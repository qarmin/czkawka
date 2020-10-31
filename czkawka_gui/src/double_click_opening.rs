use crate::help_functions::*;
use gtk::prelude::*;

pub fn opening_double_click_function_similar_images(tree_view: &gtk::TreeView, event: &gdk::EventButton) -> gtk::Inhibit {
    if event.get_event_type() == gdk::EventType::DoubleButtonPress {
        let selection = tree_view.get_selection();
        let (selection_rows, tree_model) = selection.get_selected_rows();

        for tree_path in selection_rows.iter().rev() {
            let name = tree_model.get_value(&tree_model.get_iter(tree_path).unwrap(), ColumnsSimilarImages::Name as i32).get::<String>().unwrap().unwrap();
            let path = tree_model.get_value(&tree_model.get_iter(tree_path).unwrap(), ColumnsSimilarImages::Path as i32).get::<String>().unwrap().unwrap();

            if open::that(format!("{}/{}", path, name)).is_err() {
                println!("Failed to open {}/{}", path, name);
            }
        }
    }
    gtk::Inhibit(false)
}

pub fn opening_double_click_function_duplicates(tree_view: &gtk::TreeView, event: &gdk::EventButton) -> gtk::Inhibit {
    if event.get_event_type() == gdk::EventType::DoubleButtonPress {
        let selection = tree_view.get_selection();
        let (selection_rows, tree_model) = selection.get_selected_rows();

        for tree_path in selection_rows.iter().rev() {
            let name = tree_model.get_value(&tree_model.get_iter(tree_path).unwrap(), ColumnsDuplicates::Name as i32).get::<String>().unwrap().unwrap();
            let path = tree_model.get_value(&tree_model.get_iter(tree_path).unwrap(), ColumnsDuplicates::Path as i32).get::<String>().unwrap().unwrap();

            if open::that(format!("{}/{}", path, name)).is_err() {
                println!("Failed to open {}/{}", path, name);
            }
        }
    }
    gtk::Inhibit(false)
}
pub fn opening_double_click_function_empty_folders(tree_view: &gtk::TreeView, event: &gdk::EventButton) -> gtk::Inhibit {
    if event.get_event_type() == gdk::EventType::DoubleButtonPress {
        let selection = tree_view.get_selection();
        let (selection_rows, tree_model) = selection.get_selected_rows();

        for tree_path in selection_rows.iter().rev() {
            let name = tree_model.get_value(&tree_model.get_iter(tree_path).unwrap(), ColumnsEmptyFolders::Name as i32).get::<String>().unwrap().unwrap();
            let path = tree_model.get_value(&tree_model.get_iter(tree_path).unwrap(), ColumnsEmptyFolders::Path as i32).get::<String>().unwrap().unwrap();

            if open::that(format!("{}/{}", path, name)).is_err() {
                println!("Failed to open {}/{}", path, name);
            }
        }
    }
    gtk::Inhibit(false)
}
pub fn opening_double_click_function_empty_files(tree_view: &gtk::TreeView, event: &gdk::EventButton) -> gtk::Inhibit {
    if event.get_event_type() == gdk::EventType::DoubleButtonPress {
        let selection = tree_view.get_selection();
        let (selection_rows, tree_model) = selection.get_selected_rows();

        for tree_path in selection_rows.iter().rev() {
            let name = tree_model.get_value(&tree_model.get_iter(tree_path).unwrap(), ColumnsEmptyFiles::Name as i32).get::<String>().unwrap().unwrap();
            let path = tree_model.get_value(&tree_model.get_iter(tree_path).unwrap(), ColumnsEmptyFiles::Path as i32).get::<String>().unwrap().unwrap();

            if open::that(format!("{}/{}", path, name)).is_err() {
                println!("Failed to open {}/{}", path, name);
            }
        }
    }
    gtk::Inhibit(false)
}

pub fn opening_double_click_function_temporary_files(tree_view: &gtk::TreeView, event: &gdk::EventButton) -> gtk::Inhibit {
    if event.get_event_type() == gdk::EventType::DoubleButtonPress {
        let selection = tree_view.get_selection();
        let (selection_rows, tree_model) = selection.get_selected_rows();

        for tree_path in selection_rows.iter().rev() {
            let name = tree_model.get_value(&tree_model.get_iter(tree_path).unwrap(), ColumnsTemporaryFiles::Name as i32).get::<String>().unwrap().unwrap();
            let path = tree_model.get_value(&tree_model.get_iter(tree_path).unwrap(), ColumnsTemporaryFiles::Path as i32).get::<String>().unwrap().unwrap();

            if open::that(format!("{}/{}", path, name)).is_err() {
                println!("Failed to open {}/{}", path, name);
            }
        }
    }
    gtk::Inhibit(false)
}

pub fn opening_double_click_function_big_files(tree_view: &gtk::TreeView, event: &gdk::EventButton) -> gtk::Inhibit {
    if event.get_event_type() == gdk::EventType::DoubleButtonPress {
        let selection = tree_view.get_selection();
        let (selection_rows, tree_model) = selection.get_selected_rows();

        for tree_path in selection_rows.iter().rev() {
            let name = tree_model.get_value(&tree_model.get_iter(tree_path).unwrap(), ColumnsBigFiles::Name as i32).get::<String>().unwrap().unwrap();
            let path = tree_model.get_value(&tree_model.get_iter(tree_path).unwrap(), ColumnsBigFiles::Path as i32).get::<String>().unwrap().unwrap();

            if open::that(format!("{}/{}", path, name)).is_err() {
                println!("Failed to open {}/{}", path, name);
            }
        }
    }
    gtk::Inhibit(false)
}

pub fn opening_double_click_function_zeroed_files(tree_view: &gtk::TreeView, event: &gdk::EventButton) -> gtk::Inhibit {
    if event.get_event_type() == gdk::EventType::DoubleButtonPress {
        let selection = tree_view.get_selection();
        let (selection_rows, tree_model) = selection.get_selected_rows();

        for tree_path in selection_rows.iter().rev() {
            let name = tree_model.get_value(&tree_model.get_iter(tree_path).unwrap(), ColumnsZeroedFiles::Name as i32).get::<String>().unwrap().unwrap();
            let path = tree_model.get_value(&tree_model.get_iter(tree_path).unwrap(), ColumnsZeroedFiles::Path as i32).get::<String>().unwrap().unwrap();

            if open::that(format!("{}/{}", path, name)).is_err() {
                println!("Failed to open {}/{}", path, name);
            }
        }
    }
    gtk::Inhibit(false)
}
