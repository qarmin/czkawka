use std::path::PathBuf;

use gtk::prelude::*;
use gtk::{ResponseType, TreeView, Window};

use crate::flg;
#[cfg(target_family = "windows")]
use czkawka_core::common::Common;

use crate::gui_structs::gui_data::GuiData;
use crate::help_functions::{get_dialog_box_child, get_list_store, ColumnsExcludedDirectory, ColumnsIncludedDirectory};

pub fn connect_selection_of_directories(gui_data: &GuiData) {
    // Add manually directory
    {
        let tree_view_included_directories = gui_data.upper_notebook.tree_view_included_directories.clone();
        let window_main = gui_data.window_main.clone();
        let buttons_manual_add_included_directory = gui_data.upper_notebook.buttons_manual_add_included_directory.clone();
        buttons_manual_add_included_directory.connect_clicked(move |_| {
            add_manually_directories(&window_main, &tree_view_included_directories, false);
        });
    }
    // Add manually excluded directory
    {
        let tree_view_excluded_directories = gui_data.upper_notebook.tree_view_excluded_directories.clone();
        let window_main = gui_data.window_main.clone();
        let buttons_manual_add_excluded_directory = gui_data.upper_notebook.buttons_manual_add_excluded_directory.clone();
        buttons_manual_add_excluded_directory.connect_clicked(move |_| {
            add_manually_directories(&window_main, &tree_view_excluded_directories, true);
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
    let folders_to = if excluded_items {
        flg!("exclude_folders_dialog_title")
    } else {
        flg!("include_folders_dialog_title")
    };

    let file_chooser = gtk::FileChooserDialog::builder()
        .title(&folders_to)
        .action(gtk::FileChooserAction::SelectFolder)
        .transient_for(window_main)
        .modal(true)
        .build();
    file_chooser.add_button(&flg!("general_ok_button"), ResponseType::Ok);
    file_chooser.add_button(&flg!("general_close_button"), ResponseType::Cancel);

    file_chooser.set_select_multiple(true);
    file_chooser.show_all();

    let tree_view = tree_view.clone();
    file_chooser.connect_response(move |file_chooser, response_type| {
        if response_type == gtk::ResponseType::Ok {
            let folders: Vec<PathBuf> = file_chooser.filenames();
            // GTK 4
            // folders = Vec::new();
            // if let Some(g_files) = file_chooser.files() {
            //     for index in 0..g_files.n_items() {
            //         let file = &g_files.item(index);
            //         if let Some(file) = file {
            //             println!("{:?}", file);
            //             let ss = file.clone().downcast::<gtk4::gio::File>().unwrap();
            //             if let Some(path_buf) = ss.path() {
            //                 folders.push(path_buf);
            //             }
            //         }
            //     }
            // }

            let list_store = get_list_store(&tree_view);

            if excluded_items {
                for file_entry in &folders {
                    let values: [(u32, &dyn ToValue); 1] = [(ColumnsExcludedDirectory::Path as u32, &file_entry.to_string_lossy().to_string())];
                    list_store.set(&list_store.append(), &values);
                }
            } else {
                for file_entry in &folders {
                    let values: [(u32, &dyn ToValue); 2] = [
                        (ColumnsIncludedDirectory::Path as u32, &file_entry.to_string_lossy().to_string()),
                        (ColumnsIncludedDirectory::ReferenceButton as u32, &false),
                    ];
                    list_store.set(&list_store.append(), &values);
                }
            }
        }
        file_chooser.close();
    });
}

fn add_manually_directories(window_main: &Window, tree_view: &TreeView, excluded_items: bool) {
    let dialog = gtk::Dialog::builder()
        .title(&flg!("include_manually_directories_dialog_title"))
        .transient_for(window_main)
        .modal(true)
        .build();
    dialog.add_button(&flg!("general_ok_button"), ResponseType::Ok);
    dialog.add_button(&flg!("general_close_button"), ResponseType::Cancel);

    let entry: gtk::Entry = gtk::Entry::new();

    get_dialog_box_child(&dialog).add(&entry);

    dialog.show_all();

    let tree_view = tree_view.clone();
    dialog.connect_response(move |dialog, response_type| {
        if response_type == gtk::ResponseType::Ok {
            let text = entry.text().to_string().trim().to_string();

            #[cfg(target_family = "windows")]
            let text = Common::normalize_windows_path(text).to_string_lossy().to_string();

            if !text.is_empty() {
                let list_store = get_list_store(&tree_view);

                if excluded_items {
                    let values: [(u32, &dyn ToValue); 1] = [(ColumnsExcludedDirectory::Path as u32, &text)];
                    list_store.set(&list_store.append(), &values);
                } else {
                    let values: [(u32, &dyn ToValue); 2] = [(ColumnsIncludedDirectory::Path as u32, &text), (ColumnsIncludedDirectory::ReferenceButton as u32, &false)];
                    list_store.set(&list_store.append(), &values);
                }
            }
        }
        dialog.close();
    });
}
