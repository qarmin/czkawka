use crate::create_tree_view::*;
use crate::double_click_opening::*;
use crate::gui_data::*;
use crate::help_functions::*;
use gtk::prelude::*;
use gtk::{SelectionMode, TreeView};
use std::env;

pub fn startup_configuration(gui_data: &GuiData) {
    //// Setup default look(duplicate finder)
    {
        let entry_info = gui_data.entry_info.clone();
        let buttons_search = gui_data.buttons_search.clone();
        let buttons_save = gui_data.buttons_save.clone();
        let buttons_delete = gui_data.buttons_delete.clone();
        let buttons_select = gui_data.buttons_select.clone();
        let scrolled_window_duplicate_finder = gui_data.scrolled_window_duplicate_finder.clone();
        let scrolled_window_main_empty_folder_finder = gui_data.scrolled_window_main_empty_folder_finder.clone();
        let scrolled_window_main_empty_files_finder = gui_data.scrolled_window_main_empty_files_finder.clone();
        let scrolled_window_main_temporary_files_finder = gui_data.scrolled_window_main_temporary_files_finder.clone();
        let scrolled_window_big_files_finder = gui_data.scrolled_window_big_files_finder.clone();
        let scrolled_window_similar_images_finder = gui_data.scrolled_window_similar_images_finder.clone();
        let scrolled_window_same_music_finder = gui_data.scrolled_window_same_music_finder.clone();
        let scrolled_window_zeroed_files_finder = gui_data.scrolled_window_zeroed_files_finder.clone();
        let scrolled_window_included_directories = gui_data.scrolled_window_included_directories.clone();
        let scrolled_window_excluded_directories = gui_data.scrolled_window_excluded_directories.clone();
        let entry_excluded_items = gui_data.entry_excluded_items.clone();

        entry_info.set_text("Duplicated Files");

        // Disable and show buttons
        buttons_search.show();
        buttons_save.hide();
        buttons_delete.hide();
        buttons_select.hide();

        // Set Main Scrolled Window Treeviews
        {
            // Duplicate Files
            {
                let col_types: [glib::types::Type; 6] = [
                    glib::types::Type::String,
                    glib::types::Type::String,
                    glib::types::Type::String,
                    glib::types::Type::U64,
                    glib::types::Type::String,
                    glib::types::Type::String,
                ];
                let list_store: gtk::ListStore = gtk::ListStore::new(&col_types);

                let mut tree_view: gtk::TreeView = TreeView::with_model(&list_store);

                tree_view.get_selection().set_mode(SelectionMode::Multiple);
                tree_view.get_selection().set_select_function(Some(Box::new(select_function_duplicates)));

                create_tree_view_duplicates(&mut tree_view);

                tree_view.connect_button_press_event(opening_double_click_function_duplicates);

                scrolled_window_duplicate_finder.add(&tree_view);
                scrolled_window_duplicate_finder.show_all();
            }
            // Empty Folders
            {
                let col_types: [glib::types::Type; 3] = [glib::types::Type::String, glib::types::Type::String, glib::types::Type::String];
                let list_store: gtk::ListStore = gtk::ListStore::new(&col_types);

                let mut tree_view: gtk::TreeView = TreeView::with_model(&list_store);

                tree_view.get_selection().set_mode(SelectionMode::Multiple);

                create_tree_view_empty_folders(&mut tree_view);

                tree_view.connect_button_press_event(opening_double_click_function_empty_folders);

                scrolled_window_main_empty_folder_finder.add(&tree_view);
                scrolled_window_main_empty_folder_finder.show_all();
            }
            // Empty Files
            {
                let col_types: [glib::types::Type; 3] = [glib::types::Type::String, glib::types::Type::String, glib::types::Type::String];
                let list_store: gtk::ListStore = gtk::ListStore::new(&col_types);

                let mut tree_view: gtk::TreeView = TreeView::with_model(&list_store);

                tree_view.get_selection().set_mode(SelectionMode::Multiple);

                create_tree_view_empty_files(&mut tree_view);

                tree_view.connect_button_press_event(opening_double_click_function_empty_files);

                scrolled_window_main_empty_files_finder.add(&tree_view);
                scrolled_window_main_empty_files_finder.show_all();
            }
            // Temporary Files
            {
                let col_types: [glib::types::Type; 3] = [glib::types::Type::String, glib::types::Type::String, glib::types::Type::String];
                let list_store: gtk::ListStore = gtk::ListStore::new(&col_types);

                let mut tree_view: gtk::TreeView = TreeView::with_model(&list_store);

                tree_view.get_selection().set_mode(SelectionMode::Multiple);

                create_tree_view_temporary_files(&mut tree_view);

                tree_view.connect_button_press_event(opening_double_click_function_temporary_files);

                scrolled_window_main_temporary_files_finder.add(&tree_view);
                scrolled_window_main_temporary_files_finder.show_all();
            }
            // Big Files
            {
                let col_types: [glib::types::Type; 4] = [glib::types::Type::String, glib::types::Type::String, glib::types::Type::String, glib::types::Type::String];
                let list_store: gtk::ListStore = gtk::ListStore::new(&col_types);

                let mut tree_view: gtk::TreeView = TreeView::with_model(&list_store);

                tree_view.get_selection().set_mode(SelectionMode::Multiple);

                create_tree_view_big_files(&mut tree_view);

                tree_view.connect_button_press_event(opening_double_click_function_big_files);

                scrolled_window_big_files_finder.add(&tree_view);
                scrolled_window_big_files_finder.show_all();
            }
            // Similar Images
            {
                let col_types: [glib::types::Type; 9] = [
                    glib::types::Type::String,
                    glib::types::Type::String,
                    glib::types::Type::String,
                    glib::types::Type::String,
                    glib::types::Type::String,
                    glib::types::Type::String,
                    glib::types::Type::U64,
                    glib::types::Type::String,
                    glib::types::Type::String,
                ];
                let list_store: gtk::ListStore = gtk::ListStore::new(&col_types);

                let mut tree_view: gtk::TreeView = TreeView::with_model(&list_store);

                tree_view.get_selection().set_mode(SelectionMode::Multiple);
                tree_view.get_selection().set_select_function(Some(Box::new(select_function_similar_images)));

                create_tree_view_similar_images(&mut tree_view);

                tree_view.connect_button_press_event(opening_double_click_function_similar_images);

                scrolled_window_similar_images_finder.add(&tree_view);
                scrolled_window_similar_images_finder.show_all();
            }
            // Zeroed Files
            {
                let col_types: [glib::types::Type; 4] = [glib::types::Type::String, glib::types::Type::String, glib::types::Type::String, glib::types::Type::String];
                let list_store: gtk::ListStore = gtk::ListStore::new(&col_types);

                let mut tree_view: gtk::TreeView = TreeView::with_model(&list_store);

                tree_view.get_selection().set_mode(SelectionMode::Multiple);

                create_tree_view_zeroed_files(&mut tree_view);

                tree_view.connect_button_press_event(opening_double_click_function_zeroed_files);

                scrolled_window_zeroed_files_finder.add(&tree_view);
                scrolled_window_zeroed_files_finder.show_all();
            }
            // Same Files
            {
                let col_types: [glib::types::Type; 12] = [
                    glib::types::Type::String,
                    glib::types::Type::String,
                    glib::types::Type::String,
                    glib::types::Type::String,
                    glib::types::Type::String,
                    glib::types::Type::String,
                    glib::types::Type::String,
                    glib::types::Type::String,
                    glib::types::Type::String,
                    glib::types::Type::U64,
                    glib::types::Type::String,
                    glib::types::Type::String,
                ];
                let list_store: gtk::ListStore = gtk::ListStore::new(&col_types);

                let mut tree_view: gtk::TreeView = TreeView::with_model(&list_store);

                tree_view.get_selection().set_mode(SelectionMode::Multiple);

                create_tree_view_same_music(&mut tree_view);
                tree_view.get_selection().set_select_function(Some(Box::new(select_function_same_music)));

                tree_view.connect_button_press_event(opening_double_click_function_same_music);

                scrolled_window_same_music_finder.add(&tree_view);
                scrolled_window_same_music_finder.show_all();
            }
        }

        // Set Included Directory
        {
            let col_types: [glib::types::Type; 2] = [glib::types::Type::String, glib::types::Type::String];
            let list_store: gtk::ListStore = gtk::ListStore::new(&col_types);

            let mut tree_view_included_directory: gtk::TreeView = TreeView::with_model(&list_store);

            tree_view_included_directory.get_selection().set_mode(SelectionMode::Single);

            create_tree_view_directories(&mut tree_view_included_directory);

            let col_indices = [0, 1];

            let current_dir: String = match env::current_dir() {
                Ok(t) => t.to_str().unwrap().to_string(),
                Err(_) => {
                    if cfg!(target_family = "unix") {
                        println!("Failed to read current directory, setting /home instead");
                        "/home".to_string()
                    } else if cfg!(target_family = "windows") {
                        println!("Failed to read current directory, setting C:\\ instead");
                        "C:\\".to_string()
                    } else {
                        "".to_string()
                    }
                }
            };

            let values: [&dyn ToValue; 2] = [&current_dir, &(MAIN_ROW_COLOR.to_string())];
            list_store.set(&list_store.append(), &col_indices, &values);

            scrolled_window_included_directories.add(&tree_view_included_directory);
            scrolled_window_included_directories.show_all();
        }
        // Set Excluded Directory
        {
            let col_types: [glib::types::Type; 2] = [glib::types::Type::String, glib::types::Type::String];
            let list_store: gtk::ListStore = gtk::ListStore::new(&col_types);

            let mut tree_view_excluded_directory: gtk::TreeView = TreeView::with_model(&list_store);

            tree_view_excluded_directory.get_selection().set_mode(SelectionMode::Single);

            create_tree_view_directories(&mut tree_view_excluded_directory);

            let col_indices = [0, 1];

            if cfg!(target_family = "unix") {
                for i in ["/proc", "/dev", "/sys", "/run", "/snap"].iter() {
                    let values: [&dyn ToValue; 2] = [&i, &(MAIN_ROW_COLOR.to_string())];
                    list_store.set(&list_store.append(), &col_indices, &values);
                }
            }

            scrolled_window_excluded_directories.add(&tree_view_excluded_directory);
            scrolled_window_excluded_directories.show_all();
        }
        // Set Excluded Items
        {
            if cfg!(target_family = "unix") {
                entry_excluded_items.set_text("*/.git/*,*/node_modules/*,*/lost+found/*,*/Trash/*");
            }
            if cfg!(target_family = "windows") {
                entry_excluded_items.set_text("*/.git/*,*/node_modules/*,*/lost+found/*,*:/windows/*");
            }
        }
    }
}
