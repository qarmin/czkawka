use gtk::prelude::*;
use gtk::{ResponseType, TreeView, Window};

#[cfg(target_family = "windows")]
use czkawka_core::common::Common;

use crate::gui_data::GuiData;
use crate::help_functions::{get_list_store, ColumnsDirectory};

pub fn connect_selection_of_directories(gui_data: &GuiData) {
    // Add manually directory
    {
        let tree_view_included_directories = gui_data.upper_notebook.tree_view_included_directories.clone();
        let window_main = gui_data.window_main.clone();
        let buttons_manual_add_directory = gui_data.upper_notebook.buttons_manual_add_directory.clone();
        buttons_manual_add_directory.connect_clicked(move |_| {
            add_manually_directories(&window_main, &tree_view_included_directories);
        });
    }
    // Add manually excluded directory
    {
        let tree_view_excluded_directories = gui_data.upper_notebook.tree_view_excluded_directories.clone();
        let window_main = gui_data.window_main.clone();
        let buttons_manual_add_excluded_directory = gui_data.upper_notebook.buttons_manual_add_excluded_directory.clone();
        buttons_manual_add_excluded_directory.connect_clicked(move |_| {
            add_manually_directories(&window_main, &tree_view_excluded_directories);
        });
    }
    // Add included directory
    {
        let tree_view_included_directories = gui_data.upper_notebook.tree_view_included_directories.clone();
        let window_main = gui_data.window_main.clone();
        let buttons_add_included_directory = gui_data.upper_notebook.buttons_add_included_directory.clone();
        buttons_add_included_directory.connect_clicked(move |_| {
            add_chosen_directories(&window_main, &tree_view_included_directories, false);
        });
    }
    // Add excluded directory
    {
        let tree_view_excluded_directories = gui_data.upper_notebook.tree_view_excluded_directories.clone();
        let window_main = gui_data.window_main.clone();
        let buttons_add_excluded_directory = gui_data.upper_notebook.buttons_add_excluded_directory.clone();
        buttons_add_excluded_directory.connect_clicked(move |_| {
            add_chosen_directories(&window_main, &tree_view_excluded_directories, true);
        });
    }
    // Remove Excluded Folder
    {
        let buttons_remove_excluded_directory = gui_data.upper_notebook.buttons_remove_excluded_directory.clone();
        let tree_view_excluded_directories = gui_data.upper_notebook.tree_view_excluded_directories.clone();
        buttons_remove_excluded_directory.connect_clicked(move |_| {
            let list_store = get_list_store(&tree_view_excluded_directories);
            let selection = tree_view_excluded_directories.selection();

            let (vec_tree_path, _tree_model) = selection.selected_rows();

            for tree_path in vec_tree_path.iter().rev() {
                list_store.remove(&list_store.iter(tree_path).unwrap());
            }
        });
    }
    // Remove Included Folder
    {
        let buttons_remove_included_directory = gui_data.upper_notebook.buttons_remove_included_directory.clone();
        let tree_view_included_directories = gui_data.upper_notebook.tree_view_included_directories.clone();
        buttons_remove_included_directory.connect_clicked(move |_| {
            let list_store = get_list_store(&tree_view_included_directories);
            let selection = tree_view_included_directories.selection();

            let (vec_tree_path, _tree_model) = selection.selected_rows();

            for tree_path in vec_tree_path.iter().rev() {
                list_store.remove(&list_store.iter(tree_path).unwrap());
            }
        });
    }
}

fn add_chosen_directories(window_main: &Window, tree_view: &TreeView, excluded_items: bool) {
    let folders_to = if excluded_items { "Folders to exclude" } else { "Folders to include" };

    let chooser = gtk::FileChooserDialog::builder().title(folders_to).action(gtk::FileChooserAction::SelectFolder).build();
    chooser.add_button("Ok", ResponseType::Ok);
    chooser.add_button("Close", ResponseType::Cancel);

    chooser.set_select_multiple(true);
    chooser.show_all();

    window_main.set_sensitive(false);

    let tree_view = tree_view.clone();
    let window_main = window_main.clone();
    chooser.connect_response(move |chooser, response_type| {
        if response_type == gtk::ResponseType::Ok {
            let folder = chooser.filenames();

            let list_store = get_list_store(&tree_view);

            for file_entry in &folder {
                let values: [(u32, &dyn ToValue); 1] = [(ColumnsDirectory::Path as u32, &file_entry.to_string_lossy().to_string())];
                list_store.set(&list_store.append(), &values);
            }
        }
        chooser.close();
        window_main.set_sensitive(true);
    });
}

fn add_manually_directories(window_main: &Window, tree_view: &TreeView) {
    let dialog_manual_add_directory = gtk::Dialog::builder().title("Add directory manually").build();
    dialog_manual_add_directory.add_button("Ok", ResponseType::Ok);
    dialog_manual_add_directory.add_button("Close", ResponseType::Cancel);

    window_main.set_sensitive(false);

    let entry: gtk::Entry = gtk::Entry::new();

    for widgets in dialog_manual_add_directory.children() {
        // By default GtkBox is child of dialog, so we can easily add other things to it
        widgets.clone().downcast::<gtk::Box>().unwrap().add(&entry);
    }

    dialog_manual_add_directory.show_all();

    let tree_view = tree_view.clone();
    let window_main = window_main.clone();
    dialog_manual_add_directory.connect_response(move |dialog_manual_add_directory, response_type| {
        if response_type == gtk::ResponseType::Ok {
            let text = entry.text().to_string().trim().to_string();

            #[cfg(target_family = "windows")]
            let text = Common::normalize_windows_path(text).to_string_lossy().to_string();

            if !text.is_empty() {
                let list_store = get_list_store(&tree_view);

                let values: [(u32, &dyn ToValue); 1] = [(ColumnsDirectory::Path as u32, &text)];
                list_store.set(&list_store.append(), &values);
            }
        }
        dialog_manual_add_directory.close();
        window_main.set_sensitive(true);
    });
}
