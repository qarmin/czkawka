use std::path::PathBuf;

use gtk4::prelude::*;
use gtk4::{FileChooserNative, Orientation, ResponseType, TreeView, Window};

#[cfg(target_family = "windows")]
use czkawka_core::common::Common;

use crate::flg;
use crate::gui_structs::gui_data::GuiData;
use crate::help_functions::{check_if_value_is_in_list_store, get_list_store, ColumnsExcludedDirectory, ColumnsIncludedDirectory};

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
        let buttons_add_included_directory = gui_data.upper_notebook.buttons_add_included_directory.clone();
        let file_dialog_include_exclude_folder_selection = gui_data.file_dialog_include_exclude_folder_selection.clone();
        buttons_add_included_directory.connect_clicked(move |_| {
            add_chosen_directories(&file_dialog_include_exclude_folder_selection, &tree_view_included_directories, false);
        });
    }
    // Add excluded directory
    {
        let tree_view_excluded_directories = gui_data.upper_notebook.tree_view_excluded_directories.clone();
        let buttons_add_excluded_directory = gui_data.upper_notebook.buttons_add_excluded_directory.clone();
        let file_dialog_include_exclude_folder_selection = gui_data.file_dialog_include_exclude_folder_selection.clone();
        buttons_add_excluded_directory.connect_clicked(move |_| {
            add_chosen_directories(&file_dialog_include_exclude_folder_selection, &tree_view_excluded_directories, true);
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

fn add_chosen_directories(file_dialog_include_exclude_folder_selection: &FileChooserNative, tree_view: &TreeView, excluded_items: bool) {
    let folders_to = if excluded_items {
        flg!("exclude_folders_dialog_title")
    } else {
        flg!("include_folders_dialog_title")
    };

    file_dialog_include_exclude_folder_selection.show();
    file_dialog_include_exclude_folder_selection.set_title(&folders_to);

    let tree_view = tree_view.clone();
    file_dialog_include_exclude_folder_selection.connect_response(move |file_chooser, response_type| {
        if response_type == ResponseType::Accept {
            let mut folders: Vec<PathBuf> = Vec::new();
            let g_files = file_chooser.files();
            for index in 0..g_files.n_items() {
                let file = &g_files.item(index);
                if let Some(file) = file {
                    let ss = file.clone().downcast::<gtk4::gio::File>().unwrap();
                    if let Some(path_buf) = ss.path() {
                        folders.push(path_buf);
                    }
                }
            }

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
    });
}

fn add_manually_directories(window_main: &Window, tree_view: &TreeView, excluded_items: bool) {
    let dialog = gtk4::Dialog::builder()
        .title(flg!("include_manually_directories_dialog_title"))
        .transient_for(window_main)
        .modal(true)
        .build();

    dialog.set_default_size(300, 0);

    let entry: gtk4::Entry = gtk4::Entry::new();

    let added_button = dialog.add_button(&flg!("general_ok_button"), ResponseType::Ok);
    dialog.add_button(&flg!("general_close_button"), ResponseType::Cancel);

    let parent = added_button.parent().unwrap().parent().unwrap().downcast::<gtk4::Box>().unwrap(); // TODO Hack, but not so ugly as before
    parent.set_orientation(Orientation::Vertical);
    parent.insert_child_after(&entry, None::<&gtk4::Widget>);

    dialog.show();

    let tree_view = tree_view.clone();
    dialog.connect_response(move |dialog, response_type| {
        if response_type == ResponseType::Ok {
            for text in entry.text().split(';') {
                let mut text = text.trim().to_string();
                #[cfg(target_family = "windows")]
                let mut text = Common::normalize_windows_path(text).to_string_lossy().to_string();

                remove_ending_slashes(&mut text);

                if !text.is_empty() {
                    let list_store = get_list_store(&tree_view);

                    if excluded_items {
                        if !(check_if_value_is_in_list_store(&list_store, ColumnsExcludedDirectory::Path as i32, &text)) {
                            let values: [(u32, &dyn ToValue); 1] = [(ColumnsExcludedDirectory::Path as u32, &text)];
                            list_store.set(&list_store.append(), &values);
                        }
                    } else {
                        if !check_if_value_is_in_list_store(&list_store, ColumnsIncludedDirectory::Path as i32, &text) {
                            let values: [(u32, &dyn ToValue); 2] = [(ColumnsIncludedDirectory::Path as u32, &text), (ColumnsIncludedDirectory::ReferenceButton as u32, &false)];
                            list_store.set(&list_store.append(), &values);
                        }
                    }
                }
            }
        }
        dialog.close();
    });
}

fn remove_ending_slashes(original_string: &mut String) {
    let mut windows_disk_path: bool = false;
    let mut chars = original_string.chars();
    if let Some(first_character) = chars.next() {
        if first_character.is_alphabetic() {
            if let Some(second_character) = chars.next() {
                if second_character == ':' {
                    windows_disk_path = true;
                    original_string.push('/'); // In case of adding window path without ending slash e.g. C: instead C:/ or C:\
                }
            }
        }
    }

    while (original_string != "/" && (original_string.ends_with('/') || original_string.ends_with('\\'))) && (!windows_disk_path || original_string.len() > 3) {
        original_string.pop();
    }
}

#[test]
pub fn test_remove_ending_slashes() {
    let mut original = "/home/rafal".to_string();
    remove_ending_slashes(&mut original);
    assert_eq!(&original, "/home/rafal");

    let mut original = "/home/rafal/".to_string();
    remove_ending_slashes(&mut original);
    assert_eq!(&original, "/home/rafal");

    let mut original = "/home/rafal\\".to_string();
    remove_ending_slashes(&mut original);
    assert_eq!(&original, "/home/rafal");

    let mut original = "/home/rafal/////////".to_string();
    remove_ending_slashes(&mut original);
    assert_eq!(&original, "/home/rafal");

    let mut original = "/home/rafal/\\//////\\\\".to_string();
    remove_ending_slashes(&mut original);
    assert_eq!(&original, "/home/rafal");

    let mut original = "/home/rafal\\\\\\\\\\\\\\\\".to_string();
    remove_ending_slashes(&mut original);
    assert_eq!(&original, "/home/rafal");

    let mut original = "\\\\\\\\\\\\\\\\\\\\\\\\".to_string();
    remove_ending_slashes(&mut original);
    assert_eq!(&original, "");

    let mut original = "//////////".to_string();
    remove_ending_slashes(&mut original);
    assert_eq!(&original, "/");

    let mut original = "C:/".to_string();
    remove_ending_slashes(&mut original);
    assert_eq!(&original, "C:/");

    let mut original = "C:\\".to_string();
    remove_ending_slashes(&mut original);
    assert_eq!(&original, "C:\\");

    let mut original = "C://////////".to_string();
    remove_ending_slashes(&mut original);
    assert_eq!(&original, "C:/");

    let mut original = "C:/roman/function/".to_string();
    remove_ending_slashes(&mut original);
    assert_eq!(&original, "C:/roman/function");

    let mut original = "C:/staszek/without".to_string();
    remove_ending_slashes(&mut original);
    assert_eq!(&original, "C:/staszek/without");

    let mut original = "C:\\\\\\\\\\".to_string();
    remove_ending_slashes(&mut original);
    assert_eq!(&original, "C:\\");
}
