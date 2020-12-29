extern crate gtk;
use crate::gui_data::GuiData;
use crate::help_functions::{get_list_store, get_tree_view};
use gtk::prelude::*;

pub fn connect_selection_of_directories(gui_data: &GuiData) {
    // Add included directory
    {
        let scrolled_window_included_directories = gui_data.scrolled_window_included_directories.clone();
        let window_main = gui_data.window_main.clone();
        let buttons_add_included_directory = gui_data.buttons_add_included_directory.clone();
        buttons_add_included_directory.connect_clicked(move |_| {
            let chooser = gtk::FileChooserDialog::with_buttons(
                Some("Folders to include"),
                Some(&window_main),
                gtk::FileChooserAction::SelectFolder,
                &[("Ok", gtk::ResponseType::Ok), ("Close", gtk::ResponseType::Cancel)],
            );
            chooser.set_select_multiple(true);
            chooser.show_all();
            let response_type = chooser.run();
            if response_type == gtk::ResponseType::Ok {
                let folder = chooser.get_filenames();

                let tree_view = scrolled_window_included_directories.get_children().get(0).unwrap().clone().downcast::<gtk::TreeView>().unwrap();
                let list_store = tree_view.get_model().unwrap().downcast::<gtk::ListStore>().unwrap();

                let col_indices = [0];
                for file_entry in &folder {
                    let values: [&dyn ToValue; 1] = [&file_entry.to_string_lossy().to_string()];
                    list_store.set(&list_store.append(), &col_indices, &values);
                }
            }
            chooser.close();
        });
    }
    // Add excluded directory
    {
        let scrolled_window_excluded_directories = gui_data.scrolled_window_excluded_directories.clone();
        let window_main = gui_data.window_main.clone();
        let buttons_add_excluded_directory = gui_data.buttons_add_excluded_directory.clone();
        buttons_add_excluded_directory.connect_clicked(move |_| {
            let chooser = gtk::FileChooserDialog::with_buttons(
                Some("Folders to exclude"),
                Some(&window_main),
                gtk::FileChooserAction::SelectFolder,
                &[("Ok", gtk::ResponseType::Ok), ("Close", gtk::ResponseType::Cancel)],
            );
            chooser.set_select_multiple(true);
            chooser.show_all();
            let response_type = chooser.run();
            if response_type == gtk::ResponseType::Ok {
                let folder = chooser.get_filenames();

                let tree_view = scrolled_window_excluded_directories.get_children().get(0).unwrap().clone().downcast::<gtk::TreeView>().unwrap();
                let list_store = tree_view.get_model().unwrap().downcast::<gtk::ListStore>().unwrap();

                let col_indices = [0];

                for file_entry in &folder {
                    let values: [&dyn ToValue; 1] = [&file_entry.to_string_lossy().to_string()];
                    list_store.set(&list_store.append(), &col_indices, &values);
                }
            }
            chooser.close();
        });
    }
    // Remove Excluded Folder
    {
        let buttons_remove_excluded_directory = gui_data.buttons_remove_excluded_directory.clone();
        let scrolled_window_excluded_directories = gui_data.scrolled_window_excluded_directories.clone();
        buttons_remove_excluded_directory.connect_clicked(move |_| {
            let tree_view = get_tree_view(&scrolled_window_excluded_directories);
            let list_store = get_list_store(&scrolled_window_excluded_directories);
            let selection = tree_view.get_selection();

            let (vec_tree_path, _tree_model) = selection.get_selected_rows();

            for tree_path in vec_tree_path.iter().rev() {
                list_store.remove(&list_store.get_iter(tree_path).unwrap());
            }
        });
    }
    // Remove Included Folder
    {
        let buttons_remove_included_directory = gui_data.buttons_remove_included_directory.clone();
        let scrolled_window_included_directories = gui_data.scrolled_window_included_directories.clone();
        buttons_remove_included_directory.connect_clicked(move |_| {
            let tree_view = get_tree_view(&scrolled_window_included_directories);
            let list_store = get_list_store(&scrolled_window_included_directories);
            let selection = tree_view.get_selection();

            let (vec_tree_path, _tree_model) = selection.get_selected_rows();

            for tree_path in vec_tree_path.iter().rev() {
                list_store.remove(&list_store.get_iter(tree_path).unwrap());
            }
        });
    }
}
