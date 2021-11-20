use crate::gui_data::GuiData;
use crate::help_functions::get_list_store;
use gtk::prelude::*;

#[cfg(target_family = "windows")]
use czkawka_core::common::Common;

pub fn connect_selection_of_directories(gui_data: &GuiData) {
    // Add manually directory
    {
        let tree_view_included_directories = gui_data.upper_notebook.tree_view_included_directories.clone();
        let window_main = gui_data.window_main.clone();
        let buttons_manual_add_directory = gui_data.upper_notebook.buttons_manual_add_directory.clone();
        buttons_manual_add_directory.connect_clicked(move |_| {
            let dialog_manual_add_directory = gtk::Dialog::with_buttons(Some("Add directory manually"), Some(&window_main), gtk::DialogFlags::MODAL, &[("Ok", gtk::ResponseType::Ok), ("Close", gtk::ResponseType::Cancel)]);
            let entry: gtk::Entry = gtk::Entry::new();

            for widgets in dialog_manual_add_directory.children() {
                // By default GtkBox is child of dialog, so we can easily add other things to it
                widgets.clone().downcast::<gtk::Box>().unwrap().add(&entry);
            }

            dialog_manual_add_directory.show_all();

            let response_type = dialog_manual_add_directory.run();
            if response_type == gtk::ResponseType::Ok {
                let text = entry.text().to_string().trim().to_string();

                #[cfg(target_family = "windows")]
                let text = Common::normalize_windows_path(text).to_string_lossy().to_string();

                if !text.is_empty() {
                    let list_store = get_list_store(&tree_view_included_directories);

                    let values: [(u32, &dyn ToValue); 1] = [(0, &text)];
                    list_store.set(&list_store.append(), &values);
                }
            } else {
                dialog_manual_add_directory.close();
                return;
            }
            dialog_manual_add_directory.close();
        });
    }
    // Add manually excluded directory
    {
        let tree_view_excluded_directories = gui_data.upper_notebook.tree_view_excluded_directories.clone();
        let window_main = gui_data.window_main.clone();
        let buttons_manual_add_excluded_directory = gui_data.upper_notebook.buttons_manual_add_excluded_directory.clone();
        buttons_manual_add_excluded_directory.connect_clicked(move |_| {
            let dialog_manual_add_directory = gtk::Dialog::with_buttons(
                Some("Add excluded directory manually"),
                Some(&window_main),
                gtk::DialogFlags::MODAL,
                &[("Ok", gtk::ResponseType::Ok), ("Close", gtk::ResponseType::Cancel)],
            );
            let entry: gtk::Entry = gtk::Entry::new();

            for widgets in dialog_manual_add_directory.children() {
                // By default GtkBox is child of dialog, so we can easily add other things to it
                widgets.clone().downcast::<gtk::Box>().unwrap().add(&entry);
            }

            dialog_manual_add_directory.show_all();

            let response_type = dialog_manual_add_directory.run();
            if response_type == gtk::ResponseType::Ok {
                let text = entry.text().to_string().trim().to_string();

                #[cfg(target_family = "windows")]
                let text = Common::normalize_windows_path(text).to_string_lossy().to_string();

                if !text.is_empty() {
                    let list_store = get_list_store(&tree_view_excluded_directories);

                    let values: [(u32, &dyn ToValue); 1] = [(0, &text)];
                    list_store.set(&list_store.append(), &values);
                }
            } else {
                dialog_manual_add_directory.close();
                return;
            }
            dialog_manual_add_directory.close();
        });
    }
    // Add included directory
    {
        let tree_view_included_directories = gui_data.upper_notebook.tree_view_included_directories.clone();
        let window_main = gui_data.window_main.clone();
        let buttons_add_included_directory = gui_data.upper_notebook.buttons_add_included_directory.clone();
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
                let folder = chooser.filenames();

                let list_store = get_list_store(&tree_view_included_directories);

                for file_entry in &folder {
                    let values: [(u32, &dyn ToValue); 1] = [(0, &file_entry.to_string_lossy().to_string())];
                    list_store.set(&list_store.append(), &values);
                }
            }
            chooser.close();
        });
    }
    // Add excluded directory
    {
        let tree_view_excluded_directories = gui_data.upper_notebook.tree_view_excluded_directories.clone();
        let window_main = gui_data.window_main.clone();
        let buttons_add_excluded_directory = gui_data.upper_notebook.buttons_add_excluded_directory.clone();
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
                let folder = chooser.filenames();

                let list_store = get_list_store(&tree_view_excluded_directories);

                for file_entry in &folder {
                    let values: [(u32, &dyn ToValue); 1] = [(0, &file_entry.to_string_lossy().to_string())];
                    list_store.set(&list_store.append(), &values);
                }
            }
            chooser.close();
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
