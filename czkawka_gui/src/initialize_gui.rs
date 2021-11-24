use crate::connect_button_delete::{basic_remove, check_if_can_delete_files, check_if_deleting_all_files_in_group, empty_folder_remover, tree_remove};
use crate::create_tree_view::*;
use crate::double_click_opening::*;
use crate::gui_data::*;
use crate::help_functions::*;
use crate::notebook_enums::NotebookMainEnum;
use czkawka_core::similar_images::SIMILAR_VALUES;
use czkawka_core::similar_videos::MAX_TOLERANCE;
use directories_next::ProjectDirs;
use gtk::prelude::*;
use gtk::{CheckButton, Image, SelectionMode, TextView, TreeView};
use image::imageops::FilterType;
use image::GenericImageView;
use std::cmp::Ordering;
use std::fs;
use std::path::Path;

pub fn initialize_gui(gui_data: &mut GuiData) {
    //// Initialize button
    {
        let buttons_search = gui_data.bottom_buttons.buttons_search.clone();
        let buttons_save = gui_data.bottom_buttons.buttons_save.clone();
        let buttons_delete = gui_data.bottom_buttons.buttons_delete.clone();
        let buttons_select = gui_data.bottom_buttons.buttons_select.clone();
        let buttons_symlink = gui_data.bottom_buttons.buttons_symlink.clone();
        let buttons_hardlink = gui_data.bottom_buttons.buttons_hardlink.clone();
        let buttons_move = gui_data.bottom_buttons.buttons_move.clone();

        // Disable and show buttons - only search button should be visible
        buttons_search.show();
        buttons_save.hide();
        buttons_delete.hide();
        buttons_select.hide();
        buttons_symlink.hide();
        buttons_hardlink.hide();
        buttons_move.hide();
    }

    //// Initialize main scrolled view with notebook
    {
        let scrolled_window_duplicate_finder = gui_data.main_notebook.scrolled_window_duplicate_finder.clone();
        let scrolled_window_empty_folder_finder = gui_data.main_notebook.scrolled_window_empty_folder_finder.clone();
        let scrolled_window_empty_files_finder = gui_data.main_notebook.scrolled_window_empty_files_finder.clone();
        let scrolled_window_temporary_files_finder = gui_data.main_notebook.scrolled_window_temporary_files_finder.clone();
        let scrolled_window_big_files_finder = gui_data.main_notebook.scrolled_window_big_files_finder.clone();
        let scrolled_window_similar_images_finder = gui_data.main_notebook.scrolled_window_similar_images_finder.clone();
        let scrolled_window_similar_videos_finder = gui_data.main_notebook.scrolled_window_similar_videos_finder.clone();
        let scrolled_window_same_music_finder = gui_data.main_notebook.scrolled_window_same_music_finder.clone();
        let scrolled_window_invalid_symlinks = gui_data.main_notebook.scrolled_window_invalid_symlinks.clone();
        let scrolled_window_broken_files = gui_data.main_notebook.scrolled_window_broken_files.clone();

        let image_preview_similar_images = gui_data.main_notebook.image_preview_similar_images.clone();
        let image_preview_duplicates = gui_data.main_notebook.image_preview_duplicates.clone();
        let check_button_settings_show_preview_similar_images = gui_data.settings.check_button_settings_show_preview_similar_images.clone();
        let check_button_settings_show_preview_duplicates = gui_data.settings.check_button_settings_show_preview_duplicates.clone();
        let text_view_errors = gui_data.text_view_errors.clone();

        let scale_similarity_similar_images = gui_data.main_notebook.scale_similarity_similar_images.clone();
        let scale_similarity_similar_videos = gui_data.main_notebook.scale_similarity_similar_videos.clone();

        // Set step increment
        {
            scale_similarity_similar_images.set_range(0_f64, SIMILAR_VALUES[1][5] as f64); // This defaults to value of minimal size of hash 8
            scale_similarity_similar_images.set_fill_level(SIMILAR_VALUES[1][5] as f64);
            scale_similarity_similar_images.adjustment().set_step_increment(1_f64);
        }
        // Set step increment
        {
            scale_similarity_similar_videos.set_range(0_f64, MAX_TOLERANCE as f64); // This defaults to value of minimal size of hash 8
            scale_similarity_similar_videos.set_value(15_f64);
            scale_similarity_similar_videos.set_fill_level(MAX_TOLERANCE as f64);
            scale_similarity_similar_videos.adjustment().set_step_increment(1_f64);
        }

        // Set Main Scrolled Window Treeviews
        {
            // Duplicate Files
            {
                let image_preview_duplicates_cloned = image_preview_duplicates.clone();
                image_preview_duplicates.hide();
                let text_view_errors_cloned = text_view_errors.clone();
                let check_button_settings_show_preview_duplicates_cloned = check_button_settings_show_preview_duplicates.clone();

                let col_types: [glib::types::Type; 8] = [
                    glib::types::Type::BOOL,
                    glib::types::Type::BOOL,
                    glib::types::Type::STRING,
                    glib::types::Type::STRING,
                    glib::types::Type::STRING,
                    glib::types::Type::U64,
                    glib::types::Type::STRING,
                    glib::types::Type::STRING,
                ];
                let list_store: gtk::ListStore = gtk::ListStore::new(&col_types);

                let mut tree_view: gtk::TreeView = TreeView::with_model(&list_store);

                tree_view.selection().set_mode(SelectionMode::Multiple);
                tree_view.selection().set_select_function(Some(Box::new(select_function_duplicates)));

                create_tree_view_duplicates(&mut tree_view);

                tree_view.connect_button_press_event(opening_double_click_function);
                tree_view.connect_key_press_event(opening_enter_function);
                tree_view.connect_button_release_event(move |tree_view, _event| {
                    let nb_object = &NOTEBOOKS_INFOS[NotebookMainEnum::Duplicate as usize];
                    show_preview(
                        tree_view,
                        &text_view_errors_cloned,
                        &check_button_settings_show_preview_duplicates_cloned,
                        &image_preview_duplicates_cloned,
                        nb_object.column_path,
                        nb_object.column_name,
                    );
                    gtk::Inhibit(false)
                });

                tree_view.set_widget_name("tree_view_duplicate_finder");
                gui_data.main_notebook.tree_view_duplicate_finder = tree_view.clone();
                scrolled_window_duplicate_finder.add(&tree_view);
                scrolled_window_duplicate_finder.show_all();

                let text_view_errors_cloned = text_view_errors.clone();

                let check_button_settings_confirm_group_deletion = gui_data.settings.check_button_settings_confirm_group_deletion.clone();
                let check_button_settings_confirm_deletion = gui_data.settings.check_button_settings_confirm_deletion.clone();
                let window_main = gui_data.window_main.clone();

                let check_button_settings_use_trash = gui_data.settings.check_button_settings_use_trash.clone();
                tree_view.connect_key_release_event(move |tree_view, e| {
                    let nb_object = &NOTEBOOKS_INFOS[NotebookMainEnum::Duplicate as usize];
                    if let Some(button_number) = e.keycode() {
                        // Handle delete button
                        if button_number == 119 {
                            if tree_view.selection().selected_rows().0.is_empty() {
                                return gtk::Inhibit(false);
                            }
                            if !check_if_can_delete_files(&check_button_settings_confirm_deletion, &window_main) {
                                return gtk::Inhibit(false);
                            }
                            if check_button_settings_confirm_group_deletion.is_active()
                                && check_if_deleting_all_files_in_group(&tree_view.clone(), nb_object.column_color.unwrap(), nb_object.column_selection, &window_main, &check_button_settings_confirm_group_deletion)
                            {
                                return gtk::Inhibit(false);
                            }
                            tree_remove(
                                tree_view,
                                nb_object.column_name,
                                nb_object.column_path,
                                nb_object.column_color.unwrap(),
                                nb_object.column_selection,
                                &check_button_settings_use_trash,
                                &text_view_errors,
                            );
                            image_preview_duplicates.hide();
                        }
                    }
                    show_preview(
                        tree_view,
                        &text_view_errors_cloned,
                        &check_button_settings_show_preview_duplicates,
                        &image_preview_duplicates,
                        nb_object.column_path,
                        nb_object.column_name,
                    );
                    gtk::Inhibit(false)
                });
            }
            // Empty Folders
            {
                let col_types: [glib::types::Type; 4] = [glib::types::Type::BOOL, glib::types::Type::STRING, glib::types::Type::STRING, glib::types::Type::STRING];
                let list_store: gtk::ListStore = gtk::ListStore::new(&col_types);

                let mut tree_view: gtk::TreeView = TreeView::with_model(&list_store);

                tree_view.selection().set_mode(SelectionMode::Multiple);

                create_tree_view_empty_folders(&mut tree_view);

                tree_view.connect_button_press_event(opening_double_click_function);
                tree_view.connect_key_press_event(opening_enter_function);

                tree_view.set_widget_name("tree_view_empty_folder_finder");
                gui_data.main_notebook.tree_view_empty_folder_finder = tree_view.clone();
                scrolled_window_empty_folder_finder.add(&tree_view);
                scrolled_window_empty_folder_finder.show_all();

                let check_button_settings_use_trash = gui_data.settings.check_button_settings_use_trash.clone();
                let text_view_errors = gui_data.text_view_errors.clone();
                tree_view.connect_key_release_event(move |tree_view, e| {
                    if let Some(button_number) = e.keycode() {
                        // Handle delete button
                        if button_number == 119 {
                            let nb_object = &NOTEBOOKS_INFOS[NotebookMainEnum::EmptyDirectories as usize];
                            empty_folder_remover(tree_view, nb_object.column_name, nb_object.column_path, nb_object.column_selection, &check_button_settings_use_trash, &text_view_errors);
                        }
                    }
                    gtk::Inhibit(false)
                });
            }
            // Empty Files
            {
                let col_types: [glib::types::Type; 4] = [glib::types::Type::BOOL, glib::types::Type::STRING, glib::types::Type::STRING, glib::types::Type::STRING];
                let list_store: gtk::ListStore = gtk::ListStore::new(&col_types);

                let mut tree_view: gtk::TreeView = TreeView::with_model(&list_store);

                tree_view.selection().set_mode(SelectionMode::Multiple);

                create_tree_view_empty_files(&mut tree_view);

                tree_view.connect_button_press_event(opening_double_click_function);
                tree_view.connect_key_press_event(opening_enter_function);

                tree_view.set_widget_name("tree_view_empty_files_finder");
                gui_data.main_notebook.tree_view_empty_files_finder = tree_view.clone();
                scrolled_window_empty_files_finder.add(&tree_view);
                scrolled_window_empty_files_finder.show_all();

                let check_button_settings_use_trash = gui_data.settings.check_button_settings_use_trash.clone();
                let text_view_errors = gui_data.text_view_errors.clone();
                tree_view.connect_key_release_event(move |tree_view, e| {
                    if let Some(button_number) = e.keycode() {
                        // Handle delete button
                        if button_number == 119 {
                            let nb_object = &NOTEBOOKS_INFOS[NotebookMainEnum::EmptyFiles as usize];
                            basic_remove(tree_view, nb_object.column_name, nb_object.column_path, nb_object.column_selection, &check_button_settings_use_trash, &text_view_errors);
                        }
                    }
                    gtk::Inhibit(false)
                });
            }
            // Temporary Files
            {
                let col_types: [glib::types::Type; 4] = [glib::types::Type::BOOL, glib::types::Type::STRING, glib::types::Type::STRING, glib::types::Type::STRING];
                let list_store: gtk::ListStore = gtk::ListStore::new(&col_types);

                let mut tree_view: gtk::TreeView = TreeView::with_model(&list_store);

                tree_view.selection().set_mode(SelectionMode::Multiple);

                create_tree_view_temporary_files(&mut tree_view);

                tree_view.connect_button_press_event(opening_double_click_function);
                tree_view.connect_key_press_event(opening_enter_function);

                tree_view.set_widget_name("tree_view_temporary_files_finder");
                gui_data.main_notebook.tree_view_temporary_files_finder = tree_view.clone();
                scrolled_window_temporary_files_finder.add(&tree_view);
                scrolled_window_temporary_files_finder.show_all();

                let check_button_settings_use_trash = gui_data.settings.check_button_settings_use_trash.clone();
                let text_view_errors = gui_data.text_view_errors.clone();
                tree_view.connect_key_release_event(move |tree_view, e| {
                    if let Some(button_number) = e.keycode() {
                        // Handle delete button
                        if button_number == 119 {
                            let nb_object = &NOTEBOOKS_INFOS[NotebookMainEnum::Temporary as usize];
                            basic_remove(tree_view, nb_object.column_name, nb_object.column_path, nb_object.column_selection, &check_button_settings_use_trash, &text_view_errors);
                        }
                    }
                    gtk::Inhibit(false)
                });
            }
            // Big Files
            {
                let col_types: [glib::types::Type; 5] = [glib::types::Type::BOOL, glib::types::Type::STRING, glib::types::Type::STRING, glib::types::Type::STRING, glib::types::Type::STRING];
                let list_store: gtk::ListStore = gtk::ListStore::new(&col_types);

                let mut tree_view: gtk::TreeView = TreeView::with_model(&list_store);

                tree_view.selection().set_mode(SelectionMode::Multiple);

                create_tree_view_big_files(&mut tree_view);

                tree_view.connect_button_press_event(opening_double_click_function);
                tree_view.connect_key_press_event(opening_enter_function);

                tree_view.set_widget_name("tree_view_big_files_finder");
                gui_data.main_notebook.tree_view_big_files_finder = tree_view.clone();
                scrolled_window_big_files_finder.add(&tree_view);
                scrolled_window_big_files_finder.show_all();

                let check_button_settings_use_trash = gui_data.settings.check_button_settings_use_trash.clone();
                let text_view_errors = gui_data.text_view_errors.clone();
                tree_view.connect_key_release_event(move |tree_view, e| {
                    if let Some(button_number) = e.keycode() {
                        // Handle delete button
                        if button_number == 119 {
                            let nb_object = &NOTEBOOKS_INFOS[NotebookMainEnum::BigFiles as usize];
                            basic_remove(tree_view, nb_object.column_name, nb_object.column_path, nb_object.column_selection, &check_button_settings_use_trash, &text_view_errors);
                        }
                    }
                    gtk::Inhibit(false)
                });
            }
            // Similar Images
            {
                let image_preview_similar_images_clone = image_preview_similar_images.clone();
                image_preview_similar_images.hide();

                let col_types: [glib::types::Type; 12] = [
                    glib::types::Type::BOOL,
                    glib::types::Type::BOOL,
                    glib::types::Type::STRING,
                    glib::types::Type::STRING,
                    glib::types::Type::U64,
                    glib::types::Type::STRING,
                    glib::types::Type::STRING,
                    glib::types::Type::STRING,
                    glib::types::Type::STRING,
                    glib::types::Type::U64,
                    glib::types::Type::STRING,
                    glib::types::Type::STRING,
                ];
                let list_store: gtk::ListStore = gtk::ListStore::new(&col_types);

                let mut tree_view: gtk::TreeView = TreeView::with_model(&list_store);

                tree_view.selection().set_mode(SelectionMode::Multiple);
                tree_view.selection().set_select_function(Some(Box::new(select_function_similar_images)));

                create_tree_view_similar_images(&mut tree_view);

                tree_view.connect_button_press_event(opening_double_click_function);
                tree_view.connect_key_press_event(opening_enter_function);
                let text_view_errors = gui_data.text_view_errors.clone();
                tree_view.connect_button_release_event(move |tree_view, _event| {
                    let nb_object = &NOTEBOOKS_INFOS[NotebookMainEnum::SimilarImages as usize];
                    show_preview(
                        tree_view,
                        &text_view_errors,
                        &check_button_settings_show_preview_similar_images,
                        &image_preview_similar_images,
                        nb_object.column_path,
                        nb_object.column_name,
                    );
                    gtk::Inhibit(false)
                });

                tree_view.set_widget_name("tree_view_similar_images_finder");
                gui_data.main_notebook.tree_view_similar_images_finder = tree_view.clone();
                scrolled_window_similar_images_finder.add(&tree_view);
                scrolled_window_similar_images_finder.show_all();

                let image_preview_similar_images = image_preview_similar_images_clone.clone();
                let check_button_settings_show_preview_similar_images = gui_data.settings.check_button_settings_show_preview_similar_images.clone();
                let check_button_settings_confirm_group_deletion = gui_data.settings.check_button_settings_confirm_group_deletion.clone();
                let check_button_settings_confirm_deletion = gui_data.settings.check_button_settings_confirm_deletion.clone();
                let window_main = gui_data.window_main.clone();
                let check_button_settings_use_trash = gui_data.settings.check_button_settings_use_trash.clone();
                let text_view_errors = gui_data.text_view_errors.clone();
                tree_view.connect_key_release_event(move |tree_view, e| {
                    let nb_object = &NOTEBOOKS_INFOS[NotebookMainEnum::SimilarImages as usize];
                    if let Some(button_number) = e.keycode() {
                        // Handle delete button
                        if button_number == 119 {
                            if tree_view.selection().selected_rows().0.is_empty() {
                                return gtk::Inhibit(false);
                            }
                            if !check_if_can_delete_files(&check_button_settings_confirm_deletion, &window_main) {
                                return gtk::Inhibit(false);
                            }
                            if check_button_settings_confirm_group_deletion.is_active()
                                && check_if_deleting_all_files_in_group(&tree_view.clone(), nb_object.column_color.unwrap(), nb_object.column_selection, &window_main, &check_button_settings_confirm_group_deletion)
                            {
                                return gtk::Inhibit(false);
                            }
                            tree_remove(
                                tree_view,
                                nb_object.column_name,
                                nb_object.column_path,
                                nb_object.column_color.unwrap(),
                                nb_object.column_selection,
                                &check_button_settings_use_trash,
                                &text_view_errors,
                            );
                            image_preview_similar_images_clone.hide();
                        }
                    }
                    show_preview(
                        tree_view,
                        &text_view_errors,
                        &check_button_settings_show_preview_similar_images,
                        &image_preview_similar_images,
                        nb_object.column_path,
                        nb_object.column_name,
                    );
                    gtk::Inhibit(false)
                });
            }
            // Similar Videos
            {
                let col_types: [glib::types::Type; 10] = [
                    glib::types::Type::BOOL,   // ActivatableSelectButton
                    glib::types::Type::BOOL,   // SelectionButton
                    glib::types::Type::STRING, // Size
                    glib::types::Type::U64,    // SizeAsBytes
                    glib::types::Type::STRING, // Name
                    glib::types::Type::STRING, // Path
                    glib::types::Type::STRING, // Modification
                    glib::types::Type::U64,    // ModificationAsSecs
                    glib::types::Type::STRING, // Color
                    glib::types::Type::STRING, // TextColor
                ];
                let list_store: gtk::ListStore = gtk::ListStore::new(&col_types);

                let mut tree_view: gtk::TreeView = TreeView::with_model(&list_store);

                tree_view.selection().set_mode(SelectionMode::Multiple);
                tree_view.selection().set_select_function(Some(Box::new(select_function_similar_videos)));

                create_tree_view_similar_videos(&mut tree_view);

                tree_view.connect_button_press_event(opening_double_click_function);
                tree_view.connect_key_press_event(opening_enter_function);

                tree_view.set_widget_name("tree_view_similar_videos_finder");
                gui_data.main_notebook.tree_view_similar_videos_finder = tree_view.clone();
                scrolled_window_similar_videos_finder.add(&tree_view);
                scrolled_window_similar_videos_finder.show_all();

                let check_button_settings_confirm_deletion = gui_data.settings.check_button_settings_confirm_deletion.clone();
                let check_button_settings_confirm_group_deletion = gui_data.settings.check_button_settings_confirm_group_deletion.clone();
                let window_main = gui_data.window_main.clone();
                let check_button_settings_use_trash = gui_data.settings.check_button_settings_use_trash.clone();
                let text_view_errors = gui_data.text_view_errors.clone();
                tree_view.connect_key_release_event(move |tree_view, e| {
                    if let Some(button_number) = e.keycode() {
                        // Handle delete button
                        if button_number == 119 {
                            let nb_object = &NOTEBOOKS_INFOS[NotebookMainEnum::SimilarVideos as usize];
                            if tree_view.selection().selected_rows().0.is_empty() {
                                return gtk::Inhibit(false);
                            }
                            if !check_if_can_delete_files(&check_button_settings_confirm_deletion, &window_main) {
                                return gtk::Inhibit(false);
                            }
                            if check_button_settings_confirm_group_deletion.is_active()
                                && check_if_deleting_all_files_in_group(&tree_view.clone(), nb_object.column_color.unwrap(), nb_object.column_selection, &window_main, &check_button_settings_confirm_group_deletion)
                            {
                                return gtk::Inhibit(false);
                            }
                            tree_remove(
                                tree_view,
                                nb_object.column_name,
                                nb_object.column_path,
                                nb_object.column_color.unwrap(),
                                nb_object.column_selection,
                                &check_button_settings_use_trash,
                                &text_view_errors,
                            );
                        }
                    }
                    gtk::Inhibit(false)
                });
            }
            // Same Music
            {
                let col_types: [glib::types::Type; 15] = [
                    glib::types::Type::BOOL,
                    glib::types::Type::BOOL,
                    glib::types::Type::STRING,
                    glib::types::Type::U64,
                    glib::types::Type::STRING,
                    glib::types::Type::STRING,
                    glib::types::Type::STRING,
                    glib::types::Type::STRING,
                    glib::types::Type::STRING,
                    glib::types::Type::STRING,
                    glib::types::Type::STRING,
                    glib::types::Type::STRING,
                    glib::types::Type::U64,
                    glib::types::Type::STRING,
                    glib::types::Type::STRING,
                ];
                let list_store: gtk::ListStore = gtk::ListStore::new(&col_types);

                let mut tree_view: gtk::TreeView = TreeView::with_model(&list_store);

                tree_view.selection().set_mode(SelectionMode::Multiple);

                create_tree_view_same_music(&mut tree_view);
                tree_view.selection().set_select_function(Some(Box::new(select_function_same_music)));

                tree_view.connect_button_press_event(opening_double_click_function);
                tree_view.connect_key_press_event(opening_enter_function);

                tree_view.set_widget_name("tree_view_same_music_finder");
                gui_data.main_notebook.tree_view_same_music_finder = tree_view.clone();
                scrolled_window_same_music_finder.add(&tree_view);
                scrolled_window_same_music_finder.show_all();

                let check_button_settings_confirm_group_deletion = gui_data.settings.check_button_settings_confirm_group_deletion.clone();
                let window_main = gui_data.window_main.clone();
                let check_button_settings_use_trash = gui_data.settings.check_button_settings_use_trash.clone();
                let text_view_errors = gui_data.text_view_errors.clone();
                tree_view.connect_key_release_event(move |tree_view, e| {
                    if let Some(button_number) = e.keycode() {
                        // Handle delete button
                        if button_number == 119 {
                            let nb_object = &NOTEBOOKS_INFOS[NotebookMainEnum::SameMusic as usize];

                            if tree_view.selection().selected_rows().0.is_empty() {
                                return gtk::Inhibit(false);
                            }
                            if !check_if_can_delete_files(&check_button_settings_confirm_group_deletion, &window_main) {
                                return gtk::Inhibit(false);
                            }
                            if check_button_settings_confirm_group_deletion.is_active()
                                && check_if_deleting_all_files_in_group(&tree_view.clone(), nb_object.column_color.unwrap(), nb_object.column_selection, &window_main, &check_button_settings_confirm_group_deletion)
                            {
                                return gtk::Inhibit(false);
                            }
                            tree_remove(
                                tree_view,
                                nb_object.column_name,
                                nb_object.column_path,
                                nb_object.column_color.unwrap(),
                                nb_object.column_selection,
                                &check_button_settings_use_trash,
                                &text_view_errors,
                            );
                        }
                    }
                    gtk::Inhibit(false)
                });
            }
            // Invalid Symlinks
            {
                let col_types: [glib::types::Type; 6] = [
                    glib::types::Type::BOOL,
                    glib::types::Type::STRING,
                    glib::types::Type::STRING,
                    glib::types::Type::STRING,
                    glib::types::Type::STRING,
                    glib::types::Type::STRING,
                ];
                let list_store: gtk::ListStore = gtk::ListStore::new(&col_types);

                let mut tree_view: gtk::TreeView = TreeView::with_model(&list_store);

                tree_view.selection().set_mode(SelectionMode::Multiple);

                create_tree_view_invalid_symlinks(&mut tree_view);

                tree_view.connect_button_press_event(opening_double_click_function);
                tree_view.connect_key_press_event(opening_enter_function);

                tree_view.set_widget_name("tree_view_invalid_symlinks");
                gui_data.main_notebook.tree_view_invalid_symlinks = tree_view.clone();
                scrolled_window_invalid_symlinks.add(&tree_view);
                scrolled_window_invalid_symlinks.show_all();

                let check_button_settings_use_trash = gui_data.settings.check_button_settings_use_trash.clone();
                let text_view_errors = gui_data.text_view_errors.clone();
                tree_view.connect_key_release_event(move |tree_view, e| {
                    if let Some(button_number) = e.keycode() {
                        // Handle delete button
                        if button_number == 119 {
                            let nb_object = &NOTEBOOKS_INFOS[NotebookMainEnum::Symlinks as usize];
                            basic_remove(tree_view, nb_object.column_name, nb_object.column_path, nb_object.column_selection, &check_button_settings_use_trash, &text_view_errors);
                        }
                    }
                    gtk::Inhibit(false)
                });
            }
            // Broken Files
            {
                let col_types: [glib::types::Type; 5] = [glib::types::Type::BOOL, glib::types::Type::STRING, glib::types::Type::STRING, glib::types::Type::STRING, glib::types::Type::STRING];
                let list_store: gtk::ListStore = gtk::ListStore::new(&col_types);

                let mut tree_view: gtk::TreeView = TreeView::with_model(&list_store);

                tree_view.selection().set_mode(SelectionMode::Multiple);

                create_tree_view_broken_files(&mut tree_view);

                tree_view.connect_button_press_event(opening_double_click_function);
                tree_view.connect_key_press_event(opening_enter_function);

                tree_view.set_widget_name("tree_view_broken_files");
                gui_data.main_notebook.tree_view_broken_files = tree_view.clone();
                scrolled_window_broken_files.add(&tree_view);
                scrolled_window_broken_files.show_all();

                let check_button_settings_use_trash = gui_data.settings.check_button_settings_use_trash.clone();
                let text_view_errors = gui_data.text_view_errors.clone();
                tree_view.connect_key_release_event(move |tree_view, e| {
                    if let Some(button_number) = e.keycode() {
                        // Handle delete button
                        if button_number == 119 {
                            let nb_object = &NOTEBOOKS_INFOS[NotebookMainEnum::BrokenFiles as usize];
                            basic_remove(tree_view, nb_object.column_name, nb_object.column_path, nb_object.column_selection, &check_button_settings_use_trash, &text_view_errors);
                        }
                    }
                    gtk::Inhibit(false)
                });
            }
        }
    }

    //// Initialize upper notebook
    {
        let scrolled_window_included_directories = gui_data.upper_notebook.scrolled_window_included_directories.clone();
        let scrolled_window_excluded_directories = gui_data.upper_notebook.scrolled_window_excluded_directories.clone();

        // Set Included Directory
        {
            let col_types: [glib::types::Type; 1] = [glib::types::Type::STRING];
            let list_store: gtk::ListStore = gtk::ListStore::new(&col_types);

            let mut tree_view: gtk::TreeView = TreeView::with_model(&list_store);

            tree_view.selection().set_mode(SelectionMode::Multiple);

            create_tree_view_directories(&mut tree_view);

            gui_data.upper_notebook.tree_view_included_directories = tree_view.clone();
            scrolled_window_included_directories.add(&tree_view);
            scrolled_window_included_directories.show_all();

            tree_view.connect_key_release_event(move |tree_view, e| {
                if let Some(button_number) = e.keycode() {
                    // Handle delete button
                    if button_number == 119 {
                        let list_store = get_list_store(tree_view);
                        let selection = tree_view.selection();

                        let (vec_tree_path, _tree_model) = selection.selected_rows();

                        for tree_path in vec_tree_path.iter().rev() {
                            list_store.remove(&list_store.iter(tree_path).unwrap());
                        }
                    }
                }
                gtk::Inhibit(false)
            });
        }
        // Set Excluded Directory
        {
            let col_types: [glib::types::Type; 1] = [glib::types::Type::STRING];
            let list_store: gtk::ListStore = gtk::ListStore::new(&col_types);

            let mut tree_view: gtk::TreeView = TreeView::with_model(&list_store);

            tree_view.selection().set_mode(SelectionMode::Multiple);

            create_tree_view_directories(&mut tree_view);

            gui_data.upper_notebook.tree_view_excluded_directories = tree_view.clone();
            scrolled_window_excluded_directories.add(&tree_view);
            scrolled_window_excluded_directories.show_all();

            tree_view.connect_key_release_event(move |tree_view, e| {
                if let Some(button_number) = e.keycode() {
                    // Handle delete button
                    if button_number == 119 {
                        let list_store = get_list_store(tree_view);
                        let selection = tree_view.selection();

                        let (vec_tree_path, _tree_model) = selection.selected_rows();

                        for tree_path in vec_tree_path.iter().rev() {
                            list_store.remove(&list_store.iter(tree_path).unwrap());
                        }
                    }
                }
                gtk::Inhibit(false)
            });
        }
    }

    //// Window progress
    {
        let window_progress = gui_data.progress_window.window_progress.clone();
        let stop_sender = gui_data.stop_sender.clone();

        window_progress.hide_on_delete();

        window_progress.connect_delete_event(move |_e, _y| {
            stop_sender.send(()).unwrap();
            gtk::Inhibit(true)
        });
    }
}

fn show_preview(tree_view: &TreeView, text_view_errors: &TextView, check_button_settings_show_preview: &CheckButton, image_preview_similar_images: &Image, column_path: i32, column_name: i32) {
    let (selected_rows, tree_model) = tree_view.selection().selected_rows();

    let mut created_image = false;

    // Only show preview when selected is only one item, because there is no method to recognize current clicked item in multiselection
    if selected_rows.len() == 1 && check_button_settings_show_preview.is_active() {
        let tree_path = selected_rows[0].clone();
        if let Some(proj_dirs) = ProjectDirs::from("pl", "Qarmin", "Czkawka") {
            // TODO labels on {} are in testing stage, so we just ignore for now this warning until found better idea how to fix this
            #[allow(clippy::never_loop)]
            'dir: loop {
                let cache_dir = proj_dirs.cache_dir();
                if cache_dir.exists() {
                    if !cache_dir.is_dir() {
                        add_text_to_text_view(text_view_errors, format!("Path {} doesn't point at folder, which is needed by image preview", cache_dir.display()).as_str());
                        break 'dir;
                    }
                } else if let Err(e) = fs::create_dir_all(cache_dir) {
                    add_text_to_text_view(text_view_errors, format!("Failed to create dir {} needed by image preview, reason {}", cache_dir.display(), e).as_str());
                    break 'dir;
                }
                let path = tree_model.value(&tree_model.iter(&tree_path).unwrap(), column_path).get::<String>().unwrap();
                let name = tree_model.value(&tree_model.iter(&tree_path).unwrap(), column_name).get::<String>().unwrap();

                let file_name = format!("{}/{}", path, name);
                let file_name = file_name.as_str();

                if let Some(extension) = Path::new(file_name).extension() {
                    if !["jpg", "jpeg", "png", "bmp", "tiff", "tif", "tga", "ff", "gif", "jif", "jfi"].contains(&extension.to_string_lossy().to_string().to_lowercase().as_str()) {
                        break 'dir;
                    }

                    let img = match image::open(&file_name) {
                        Ok(t) => t,
                        Err(e) => {
                            add_text_to_text_view(text_view_errors, format!("Failed to open temporary image file {}, reason {}", file_name, e).as_str());
                            break 'dir;
                        }
                    };
                    if img.width() == 0 || img.height() == 0 {
                        add_text_to_text_view(text_view_errors, format!("Cannot create preview of image {}, with 0 width or height", file_name).as_str());
                        break 'dir;
                    }
                    let ratio = img.width() / img.height();
                    let requested_dimensions = (400, 400);
                    let mut new_size;
                    match ratio.cmp(&(requested_dimensions.0 / requested_dimensions.1)) {
                        Ordering::Greater => {
                            new_size = (requested_dimensions.0, (img.height() * requested_dimensions.0) / img.width());
                            new_size = (std::cmp::max(new_size.0, 1), std::cmp::max(new_size.1, 1));
                        }
                        Ordering::Less => {
                            new_size = ((img.width() * requested_dimensions.1) / img.height(), requested_dimensions.1);
                            new_size = (std::cmp::max(new_size.0, 1), std::cmp::max(new_size.1, 1));
                        }
                        Ordering::Equal => {
                            new_size = requested_dimensions;
                            new_size = (std::cmp::max(new_size.0, 1), std::cmp::max(new_size.1, 1));
                        }
                    }
                    let img = img.resize(new_size.0, new_size.1, FilterType::Triangle);
                    let file_dir = cache_dir.join(format!("cached_file.{}", extension.to_string_lossy().to_lowercase()));
                    if let Err(e) = img.save(&file_dir) {
                        add_text_to_text_view(text_view_errors, format!("Failed to save temporary image file to {}, reason {}", file_dir.display(), e).as_str());
                        let _ = fs::remove_file(&file_dir);
                        break 'dir;
                    }
                    let string_dir = file_dir.to_string_lossy().to_string();
                    image_preview_similar_images.set_from_file(string_dir);
                    if let Err(e) = fs::remove_file(&file_dir) {
                        add_text_to_text_view(text_view_errors, format!("Failed to delete temporary image file to {}, reason {}", file_dir.display(), e).as_str());
                        break 'dir;
                    }
                    created_image = true;
                }
                break 'dir;
            }
        }
    }
    if created_image {
        image_preview_similar_images.show();
    } else {
        image_preview_similar_images.hide();
    }
}
