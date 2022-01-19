use std::cell::RefCell;
use std::fs;
use std::ops::Deref;
use std::path::Path;
use std::rc::Rc;

use czkawka_core::common::get_dynamic_image_from_raw_image;
use directories_next::ProjectDirs;
use gtk::prelude::*;
use gtk::{CheckButton, Image, SelectionMode, TextView, TreeView};
use image::imageops::FilterType;
use image::GenericImageView;

use crate::flg;
use czkawka_core::similar_images::{IMAGE_RS_EXTENSIONS, RAW_IMAGE_EXTENSIONS, SIMILAR_VALUES};
use czkawka_core::similar_videos::MAX_TOLERANCE;

use crate::create_tree_view::*;
use crate::delete_things;
use crate::gui_structs::gui_data::*;
use crate::help_combo_box::{
    DUPLICATES_CHECK_METHOD_COMBO_BOX, DUPLICATES_HASH_TYPE_COMBO_BOX, IMAGES_HASH_SIZE_COMBO_BOX, IMAGES_HASH_TYPE_COMBO_BOX, IMAGES_RESIZE_ALGORITHM_COMBO_BOX,
};
use crate::help_functions::*;
use crate::language_functions::LANGUAGES_ALL;
use crate::localizer_core::generate_translation_hashmap;
use crate::notebook_enums::NotebookMainEnum;
use crate::opening_selecting_records::*;

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
        let buttons_compare = gui_data.bottom_buttons.buttons_compare.clone();

        // Disable and show buttons - only search button should be visible
        buttons_search.show();
        buttons_save.hide();
        buttons_delete.hide();
        buttons_select.hide();
        buttons_symlink.hide();
        buttons_hardlink.hide();
        buttons_move.hide();
        buttons_compare.hide();
    }
    //// Initialize language combo box
    {
        let combo_box_settings_language = gui_data.settings.combo_box_settings_language.clone();
        for lang in LANGUAGES_ALL {
            combo_box_settings_language.append_text(lang.combo_box_text);
        }
        combo_box_settings_language.set_active(Some(0));
    }
    //// Initialize main window combo boxes
    {
        {
            let combo_box_duplicate_check_method = gui_data.main_notebook.combo_box_duplicate_check_method.clone();
            for check_type in &DUPLICATES_CHECK_METHOD_COMBO_BOX {
                combo_box_duplicate_check_method.append_text(check_type.eng_name);
            }
            combo_box_duplicate_check_method.set_active(Some(0));
        }
        {
            let combo_box_duplicate_hash_type = gui_data.main_notebook.combo_box_duplicate_hash_type.clone();
            for hash_type in &DUPLICATES_HASH_TYPE_COMBO_BOX {
                combo_box_duplicate_hash_type.append_text(hash_type.eng_name);
            }
            combo_box_duplicate_hash_type.set_active(Some(0));
        }
    }
    {
        {
            let combo_box_image_hash_algorithm = gui_data.main_notebook.combo_box_image_hash_algorithm.clone();
            for check_type in &IMAGES_HASH_TYPE_COMBO_BOX {
                combo_box_image_hash_algorithm.append_text(check_type.eng_name);
            }
            combo_box_image_hash_algorithm.set_active(Some(0));
        }
        {
            let combo_box_image_hash_size = gui_data.main_notebook.combo_box_image_hash_size.clone();
            for check_type in &IMAGES_HASH_SIZE_COMBO_BOX {
                combo_box_image_hash_size.append_text(&check_type.to_string());
            }
            combo_box_image_hash_size.set_active(Some(0));
        }
        {
            let combo_box_image_resize_algorithm = gui_data.main_notebook.combo_box_image_resize_algorithm.clone();
            for resize in &IMAGES_RESIZE_ALGORITHM_COMBO_BOX {
                combo_box_image_resize_algorithm.append_text(resize.eng_name);
            }
            combo_box_image_resize_algorithm.set_active(Some(0));
        }
    }

    //// Initialize main scrolled view with notebook
    {
        // Set step increment
        {
            let scale_similarity_similar_images = gui_data.main_notebook.scale_similarity_similar_images.clone();
            scale_similarity_similar_images.set_range(0_f64, SIMILAR_VALUES[0][5] as f64); // This defaults to value of minimal size of hash 8
            scale_similarity_similar_images.set_fill_level(SIMILAR_VALUES[0][5] as f64);
            scale_similarity_similar_images.adjustment().set_step_increment(1_f64);
        }
        // Set step increment
        {
            let scale_similarity_similar_videos = gui_data.main_notebook.scale_similarity_similar_videos.clone();
            scale_similarity_similar_videos.set_range(0_f64, MAX_TOLERANCE as f64); // This defaults to value of minimal size of hash 8
            scale_similarity_similar_videos.set_value(15_f64);
            scale_similarity_similar_videos.set_fill_level(MAX_TOLERANCE as f64);
            scale_similarity_similar_videos.adjustment().set_step_increment(1_f64);
        }

        // Set Main Scrolled Window Treeviews
        {
            // Duplicate Files
            {
                let scrolled_window = gui_data.main_notebook.scrolled_window_duplicate_finder.clone();
                let tree_view = gui_data.main_notebook.tree_view_duplicate_finder.clone();

                let image_preview = gui_data.main_notebook.image_preview_duplicates.clone();
                image_preview.hide();

                let col_types: [glib::types::Type; 9] = [
                    glib::types::Type::BOOL,   // ActivatableSelectButton
                    glib::types::Type::BOOL,   // SelectionButton
                    glib::types::Type::STRING, // Size
                    glib::types::Type::STRING, // Name
                    glib::types::Type::STRING, // Path
                    glib::types::Type::STRING, // Modification
                    glib::types::Type::U64,    // ModificationAsSecs
                    glib::types::Type::STRING, // Color
                    glib::types::Type::STRING, // TextColor
                ];
                let list_store: gtk::ListStore = gtk::ListStore::new(&col_types);

                tree_view.set_model(Some(&list_store));
                tree_view.selection().set_mode(SelectionMode::Multiple);
                tree_view.selection().set_select_function(Some(Box::new(select_function_duplicates)));

                create_tree_view_duplicates(&tree_view);

                tree_view.set_widget_name("tree_view_duplicate_finder");
                scrolled_window.add(&tree_view);
                scrolled_window.show_all();
            }
            // Empty Folders
            {
                let scrolled_window = gui_data.main_notebook.scrolled_window_empty_folder_finder.clone();
                let tree_view = gui_data.main_notebook.tree_view_empty_folder_finder.clone();

                let col_types: [glib::types::Type; 5] = [
                    glib::types::Type::BOOL,   // SelectionButton
                    glib::types::Type::STRING, // Name
                    glib::types::Type::STRING, // Path
                    glib::types::Type::STRING, // Modification
                    glib::types::Type::U64,    // ModificationAsSecs
                ];
                let list_store: gtk::ListStore = gtk::ListStore::new(&col_types);

                tree_view.set_model(Some(&list_store));
                tree_view.selection().set_mode(SelectionMode::Multiple);

                create_tree_view_empty_folders(&tree_view);

                tree_view.set_widget_name("tree_view_empty_folder_finder");

                scrolled_window.add(&tree_view);
                scrolled_window.show_all();
            }
            // Empty Files
            {
                let scrolled_window = gui_data.main_notebook.scrolled_window_empty_files_finder.clone();
                let tree_view = gui_data.main_notebook.tree_view_empty_files_finder.clone();
                let col_types: [glib::types::Type; 5] = [
                    glib::types::Type::BOOL,   // SelectionButton
                    glib::types::Type::STRING, // Name
                    glib::types::Type::STRING, // Path
                    glib::types::Type::STRING, // Modification
                    glib::types::Type::U64,    // ModificationAsSecs
                ];
                let list_store: gtk::ListStore = gtk::ListStore::new(&col_types);

                tree_view.set_model(Some(&list_store));
                tree_view.selection().set_mode(SelectionMode::Multiple);

                create_tree_view_empty_files(&tree_view);

                tree_view.set_widget_name("tree_view_empty_files_finder");
                scrolled_window.add(&tree_view);
                scrolled_window.show_all();
            }
            // Temporary Files
            {
                let scrolled_window = gui_data.main_notebook.scrolled_window_temporary_files_finder.clone();
                let tree_view = gui_data.main_notebook.tree_view_temporary_files_finder.clone();

                let col_types: [glib::types::Type; 5] = [
                    glib::types::Type::BOOL,   // SelectionButton
                    glib::types::Type::STRING, // Name
                    glib::types::Type::STRING, // Path
                    glib::types::Type::STRING, // Modification
                    glib::types::Type::U64,    // ModificationAsSecs
                ];
                let list_store: gtk::ListStore = gtk::ListStore::new(&col_types);

                tree_view.set_model(Some(&list_store));
                tree_view.selection().set_mode(SelectionMode::Multiple);

                create_tree_view_temporary_files(&tree_view);

                tree_view.set_widget_name("tree_view_temporary_files_finder");
                scrolled_window.add(&tree_view);
                scrolled_window.show_all();
            }
            // Big Files
            {
                let scrolled_window = gui_data.main_notebook.scrolled_window_big_files_finder.clone();
                let tree_view = gui_data.main_notebook.tree_view_big_files_finder.clone();

                let col_types: [glib::types::Type; 7] = [
                    glib::types::Type::BOOL,   // SelectionButton
                    glib::types::Type::STRING, // Size
                    glib::types::Type::STRING, // Name
                    glib::types::Type::STRING, // Path
                    glib::types::Type::STRING, // Modification
                    glib::types::Type::U64,    // SizeAsBytes
                    glib::types::Type::U64,    // ModificationAsSecs
                ];
                let list_store: gtk::ListStore = gtk::ListStore::new(&col_types);

                tree_view.set_model(Some(&list_store));
                tree_view.selection().set_mode(SelectionMode::Multiple);

                create_tree_view_big_files(&tree_view);

                tree_view.set_widget_name("tree_view_big_files_finder");
                scrolled_window.add(&tree_view);
                scrolled_window.show_all();
            }
            // Similar Images
            {
                let scrolled_window = gui_data.main_notebook.scrolled_window_similar_images_finder.clone();
                let tree_view = gui_data.main_notebook.tree_view_similar_images_finder.clone();

                let image_preview = gui_data.main_notebook.image_preview_similar_images.clone();
                image_preview.hide();

                let col_types: [glib::types::Type; 12] = [
                    glib::types::Type::BOOL,   // ActivatableSelectButton
                    glib::types::Type::BOOL,   // SelectionButton
                    glib::types::Type::STRING, // Similarity
                    glib::types::Type::STRING, // Size
                    glib::types::Type::U64,    // SizeAsBytes
                    glib::types::Type::STRING, // Dimensions
                    glib::types::Type::STRING, // Name
                    glib::types::Type::STRING, // Path
                    glib::types::Type::STRING, // Modification
                    glib::types::Type::U64,    // ModificationAsSecs
                    glib::types::Type::STRING, // Color
                    glib::types::Type::STRING, // TextColor
                ];
                let list_store: gtk::ListStore = gtk::ListStore::new(&col_types);

                tree_view.set_model(Some(&list_store));
                tree_view.selection().set_mode(SelectionMode::Multiple);
                tree_view.selection().set_select_function(Some(Box::new(select_function_similar_images)));

                create_tree_view_similar_images(&tree_view);

                tree_view.set_widget_name("tree_view_similar_images_finder");
                scrolled_window.add(&tree_view);
                scrolled_window.show_all();
            }
            // Similar Videos
            {
                let scrolled_window = gui_data.main_notebook.scrolled_window_similar_videos_finder.clone();
                let tree_view = gui_data.main_notebook.tree_view_similar_videos_finder.clone();

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

                tree_view.set_model(Some(&list_store));
                tree_view.selection().set_mode(SelectionMode::Multiple);
                tree_view.selection().set_select_function(Some(Box::new(select_function_similar_videos)));

                create_tree_view_similar_videos(&tree_view);

                tree_view.set_widget_name("tree_view_similar_videos_finder");
                scrolled_window.add(&tree_view);
                scrolled_window.show_all();
            }
            // Same Music
            {
                let scrolled_window = gui_data.main_notebook.scrolled_window_same_music_finder.clone();
                let tree_view = gui_data.main_notebook.tree_view_same_music_finder.clone();

                let col_types: [glib::types::Type; 15] = [
                    glib::types::Type::BOOL,   // ActivatableSelectButton
                    glib::types::Type::BOOL,   // SelectionButton
                    glib::types::Type::STRING, // Size
                    glib::types::Type::U64,    // SizeAsBytes
                    glib::types::Type::STRING, // Name
                    glib::types::Type::STRING, // Path
                    glib::types::Type::STRING, // Title
                    glib::types::Type::STRING, // Artist
                    glib::types::Type::STRING, // AlbumTitle
                    glib::types::Type::STRING, // AlbumArtist
                    glib::types::Type::STRING, // Year
                    glib::types::Type::STRING, // Modification
                    glib::types::Type::U64,    // ModificationAsSecs
                    glib::types::Type::STRING, // Color
                    glib::types::Type::STRING, // TextColor
                ];
                let list_store: gtk::ListStore = gtk::ListStore::new(&col_types);

                tree_view.set_model(Some(&list_store));
                tree_view.selection().set_mode(SelectionMode::Multiple);
                tree_view.selection().set_select_function(Some(Box::new(select_function_same_music)));

                create_tree_view_same_music(&tree_view);

                tree_view.set_widget_name("tree_view_same_music_finder");
                scrolled_window.add(&tree_view);
                scrolled_window.show_all();
            }
            // Invalid Symlinks
            {
                let scrolled_window = gui_data.main_notebook.scrolled_window_invalid_symlinks.clone();
                let tree_view = gui_data.main_notebook.tree_view_invalid_symlinks.clone();

                let col_types: [glib::types::Type; 7] = [
                    glib::types::Type::BOOL,   // SelectionButton
                    glib::types::Type::STRING, // Name
                    glib::types::Type::STRING, // Path
                    glib::types::Type::STRING, // DestinationPath
                    glib::types::Type::STRING, // TypeOfError
                    glib::types::Type::STRING, // Modification
                    glib::types::Type::U64,    // ModificationAsSecs
                ];
                let list_store: gtk::ListStore = gtk::ListStore::new(&col_types);

                tree_view.set_model(Some(&list_store));
                tree_view.selection().set_mode(SelectionMode::Multiple);

                create_tree_view_invalid_symlinks(&tree_view);

                tree_view.set_widget_name("tree_view_invalid_symlinks");
                scrolled_window.add(&tree_view);
                scrolled_window.show_all();
            }
            // Broken Files
            {
                let scrolled_window = gui_data.main_notebook.scrolled_window_broken_files.clone();
                let tree_view = gui_data.main_notebook.tree_view_broken_files.clone();

                let col_types: [glib::types::Type; 6] = [
                    glib::types::Type::BOOL,   // SelectionButton
                    glib::types::Type::STRING, // Name
                    glib::types::Type::STRING, // Path
                    glib::types::Type::STRING, // ErrorType
                    glib::types::Type::STRING, // Modification
                    glib::types::Type::U64,    // ModificationAsSecs
                ];
                let list_store: gtk::ListStore = gtk::ListStore::new(&col_types);

                tree_view.set_model(Some(&list_store));
                tree_view.selection().set_mode(SelectionMode::Multiple);

                create_tree_view_broken_files(&tree_view);

                tree_view.set_widget_name("tree_view_broken_files");
                scrolled_window.add(&tree_view);
                scrolled_window.show_all();
            }
        }
    }

    //// Initialize upper notebook
    {
        // Set Included Directory
        {
            let scrolled_window = gui_data.upper_notebook.scrolled_window_included_directories.clone();
            let tree_view = gui_data.upper_notebook.tree_view_included_directories.clone();
            let evk = gui_data.upper_notebook.evk_tree_view_included_directories.clone();

            let col_types: [glib::types::Type; 2] = [
                glib::types::Type::STRING, // Path
                glib::types::Type::BOOL,   // ReferenceButton
            ];
            let list_store: gtk::ListStore = gtk::ListStore::new(&col_types);

            tree_view.set_model(Some(&list_store));
            tree_view.selection().set_mode(SelectionMode::Multiple);

            create_tree_view_included_directories(&tree_view);

            tree_view.set_widget_name("tree_view_upper_included_directories");
            scrolled_window.add(&tree_view);
            scrolled_window.show_all();

            tree_view.connect_button_press_event(opening_double_click_function_directories);
            evk.connect_key_pressed(opening_enter_function_ported_upper_directories);
            evk.connect_key_released(move |_event_controller_key, _key_value, key_code, _modifier_type| {
                if key_code == KEY_DELETE {
                    let list_store = get_list_store(&tree_view);
                    let selection = tree_view.selection();

                    let (vec_tree_path, _tree_model) = selection.selected_rows();

                    for tree_path in vec_tree_path.iter().rev() {
                        list_store.remove(&list_store.iter(tree_path).unwrap());
                    }
                }
            });
        }
        // Set Excluded Directory
        {
            let scrolled_window = gui_data.upper_notebook.scrolled_window_excluded_directories.clone();
            let tree_view = gui_data.upper_notebook.tree_view_excluded_directories.clone();
            let evk = gui_data.upper_notebook.evk_tree_view_excluded_directories.clone();

            let col_types: [glib::types::Type; 1] = [glib::types::Type::STRING];
            let list_store: gtk::ListStore = gtk::ListStore::new(&col_types);

            tree_view.set_model(Some(&list_store));
            tree_view.selection().set_mode(SelectionMode::Multiple);

            create_tree_view_excluded_directories(&tree_view);

            tree_view.set_widget_name("tree_view_upper_excluded_directories");
            scrolled_window.add(&tree_view);
            scrolled_window.show_all();

            tree_view.connect_button_press_event(opening_double_click_function_directories);
            evk.connect_key_pressed(opening_enter_function_ported_upper_directories);
            evk.connect_key_released(move |_event_controller_key, _key_value, key_code, _modifier_type| {
                if key_code == KEY_DELETE {
                    let list_store = get_list_store(&tree_view);
                    let selection = tree_view.selection();

                    let (vec_tree_path, _tree_model) = selection.selected_rows();

                    for tree_path in vec_tree_path.iter().rev() {
                        list_store.remove(&list_store.iter(tree_path).unwrap());
                    }
                }
            });
        }
    }

    //// Window progress
    {
        let window_progress = gui_data.progress_window.window_progress.clone();
        let stop_sender = gui_data.stop_sender.clone();

        window_progress.connect_delete_event(move |_, _| {
            stop_sender.send(()).unwrap();
            gtk::Inhibit(true)
        });
    }

    // This not need to be run in different code block, but this looks a little less complicated if is available in
    connect_event_buttons(gui_data);
    connect_event_mouse(gui_data);
}

fn connect_event_mouse(gui_data: &GuiData) {
    for tree_view in gui_data.main_notebook.get_main_tree_views() {
        tree_view.connect_button_press_event(opening_double_click_function);
        tree_view.connect_button_release_event(opening_middle_mouse_function);
    }

    // Duplicate
    {
        let text_view_errors = gui_data.text_view_errors.clone();
        let check_button_settings_show_preview = gui_data.settings.check_button_settings_show_preview_duplicates.clone();
        let image_preview = gui_data.main_notebook.image_preview_duplicates.clone();
        let preview_path = gui_data.preview_path.clone();
        let tree_view = gui_data.main_notebook.tree_view_duplicate_finder.clone();

        tree_view.connect_button_release_event(move |tree_view, _event| {
            let nb_object = &NOTEBOOKS_INFOS[NotebookMainEnum::Duplicate as usize];
            let preview_path = preview_path.clone();
            show_preview(
                tree_view,
                &text_view_errors,
                &check_button_settings_show_preview,
                &image_preview,
                preview_path,
                nb_object.column_path,
                nb_object.column_name,
            );

            gtk::Inhibit(false)
        });
    }
    // Similar Images
    {
        let text_view_errors = gui_data.text_view_errors.clone();
        let tree_view = gui_data.main_notebook.tree_view_similar_images_finder.clone();
        let check_button_settings_show_preview = gui_data.settings.check_button_settings_show_preview_similar_images.clone();
        let preview_path = gui_data.preview_path.clone();
        let image_preview = gui_data.main_notebook.image_preview_similar_images.clone();

        tree_view.connect_button_release_event(move |tree_view, _event| {
            let nb_object = &NOTEBOOKS_INFOS[NotebookMainEnum::SimilarImages as usize];
            let preview_path = preview_path.clone();
            show_preview(
                tree_view,
                &text_view_errors,
                &check_button_settings_show_preview,
                &image_preview,
                preview_path,
                nb_object.column_path,
                nb_object.column_name,
            );
            gtk::Inhibit(false)
        });
    }

    // GTK 4
    // for gc in [
    //     gui_data.main_notebook.gc_tree_view_duplicate_finder.clone(),
    //     gui_data.main_notebook.gc_tree_view_empty_folder_finder.clone(),
    //     gui_data.main_notebook.gc_tree_view_empty_files_finder.clone(),
    //     gui_data.main_notebook.gc_tree_view_temporary_files_finder.clone(),
    //     gui_data.main_notebook.gc_tree_view_big_files_finder.clone(),
    //     gui_data.main_notebook.gc_tree_view_similar_images_finder.clone(),
    //     gui_data.main_notebook.gc_tree_view_similar_videos_finder.clone(),
    //     gui_data.main_notebook.gc_tree_view_same_music_finder.clone(),
    //     gui_data.main_notebook.gc_tree_view_invalid_symlinks.clone(),
    //     gui_data.main_notebook.gc_tree_view_broken_files.clone(),
    // ] {
    //     gc.set_button(0);
    //     gc.connect_pressed(opening_double_click_function);
    // }
    //
    // // Duplicate
    // {
    //     let text_view_errors = gui_data.text_view_errors.clone();
    //     let check_button_settings_show_preview = gui_data.settings.check_button_settings_show_preview_duplicates.clone();
    //     let image_preview = gui_data.main_notebook.image_preview_duplicates.clone();
    //     let preview_path = gui_data.preview_path.clone();
    //
    //     let gc = gui_data.main_notebook.gc_tree_view_duplicate_finder.clone();
    //
    //     gc.connect_released(move |gc, _event, _, _| {
    //         let tree_view = gc.widget().unwrap().downcast::<gtk4::TreeView>().unwrap();
    //         let nb_object = &NOTEBOOKS_INFOS[NotebookMainEnum::Duplicate as usize];
    //         let preview_path = preview_path.clone();
    //         show_preview(&tree_view, &text_view_errors, &check_button_settings_show_preview, &image_preview, preview_path, nb_object.column_path, nb_object.column_name);
    //     });
    // }
    // // Similar Images
    // {
    //     let text_view_errors = gui_data.text_view_errors.clone();
    //     let check_button_settings_show_preview = gui_data.settings.check_button_settings_show_preview_similar_images.clone();
    //     let preview_path = gui_data.preview_path.clone();
    //     let image_preview = gui_data.main_notebook.image_preview_similar_images.clone();
    //
    //     let gc = gui_data.main_notebook.gc_tree_view_similar_images_finder.clone();
    //
    //     gc.connect_released(move |gc, _event, _, _| {
    //         let tree_view = gc.widget().unwrap().downcast::<gtk4::TreeView>().unwrap();
    //         let nb_object = &NOTEBOOKS_INFOS[NotebookMainEnum::SimilarImages as usize];
    //         let preview_path = preview_path.clone();
    //         show_preview(&tree_view, &text_view_errors, &check_button_settings_show_preview, &image_preview, preview_path, nb_object.column_path, nb_object.column_name);
    //     });
    // }
}
fn connect_event_buttons(gui_data: &GuiData) {
    for evk in [
        //gui_data.main_notebook.evk_tree_view_duplicate_finder.clone(), // Manual - needs to show/hide preview
        gui_data.main_notebook.evk_tree_view_empty_folder_finder.clone(),
        gui_data.main_notebook.evk_tree_view_empty_files_finder.clone(),
        gui_data.main_notebook.evk_tree_view_temporary_files_finder.clone(),
        gui_data.main_notebook.evk_tree_view_big_files_finder.clone(),
        //gui_data.main_notebook.evk_tree_view_similar_images_finder.clone(),// Manual - needs to show/hide preview
        gui_data.main_notebook.evk_tree_view_similar_videos_finder.clone(),
        gui_data.main_notebook.evk_tree_view_same_music_finder.clone(),
        gui_data.main_notebook.evk_tree_view_invalid_symlinks.clone(),
        gui_data.main_notebook.evk_tree_view_broken_files.clone(),
    ] {
        let gui_data_clone = gui_data.clone();
        evk.connect_key_pressed(opening_enter_function_ported);

        evk.connect_key_released(move |_event_controller_key, _key_value, key_code, _modifier_type| {
            if key_code == KEY_DELETE {
                glib::MainContext::default().spawn_local(delete_things(gui_data_clone.clone()));
            }
        });
    }
    // Duplicate
    {
        let gui_data_clone = gui_data.clone();
        let text_view_errors = gui_data.text_view_errors.clone();
        let check_button_settings_show_preview = gui_data.settings.check_button_settings_show_preview_duplicates.clone();
        let image_preview = gui_data.main_notebook.image_preview_duplicates.clone();
        let preview_path = gui_data.preview_path.clone();
        let evk = gui_data.main_notebook.evk_tree_view_duplicate_finder.clone();

        evk.connect_key_pressed(opening_enter_function_ported);

        evk.connect_key_released(move |event_controller_key, _key_value, key_code, _modifier_type| {
            if key_code == KEY_DELETE {
                glib::MainContext::default().spawn_local(delete_things(gui_data_clone.clone()));
            }
            let preview_path = preview_path.clone();
            let nb_object = &NOTEBOOKS_INFOS[NotebookMainEnum::Duplicate as usize];
            show_preview(
                &event_controller_key.widget().unwrap().downcast::<gtk::TreeView>().unwrap(),
                &text_view_errors,
                &check_button_settings_show_preview,
                &image_preview,
                preview_path,
                nb_object.column_path,
                nb_object.column_name,
            );
        });
    }
    // Similar Images
    {
        let check_button_settings_show_preview_similar_images = gui_data.settings.check_button_settings_show_preview_similar_images.clone();
        let text_view_errors = gui_data.text_view_errors.clone();
        let image_preview = gui_data.main_notebook.image_preview_similar_images.clone();
        let gui_data_clone = gui_data.clone();
        let preview_path = gui_data.preview_path.clone();
        let evk = gui_data.main_notebook.evk_tree_view_similar_images_finder.clone();

        evk.connect_key_pressed(opening_enter_function_ported);

        evk.connect_key_released(move |event_controller_key, _key_value, key_code, _modifier_type| {
            if key_code == KEY_DELETE {
                glib::MainContext::default().spawn_local(delete_things(gui_data_clone.clone()));
            }
            let preview_path = preview_path.clone();
            let nb_object = &NOTEBOOKS_INFOS[NotebookMainEnum::SimilarImages as usize];
            show_preview(
                &event_controller_key.widget().unwrap().downcast::<gtk::TreeView>().unwrap(),
                &text_view_errors,
                &check_button_settings_show_preview_similar_images,
                &image_preview,
                preview_path,
                nb_object.column_path,
                nb_object.column_name,
            );
        });
    }
}

fn show_preview(
    tree_view: &TreeView,
    text_view_errors: &TextView,
    check_button_settings_show_preview: &CheckButton,
    image_preview: &Image,
    preview_path: Rc<RefCell<String>>,
    column_path: i32,
    column_name: i32,
) {
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
                        add_text_to_text_view(
                            text_view_errors,
                            format!("Path {} doesn't point at folder, which is needed by image preview", cache_dir.display()).as_str(),
                        );
                        break 'dir;
                    }
                } else if let Err(e) = fs::create_dir_all(cache_dir) {
                    add_text_to_text_view(
                        text_view_errors,
                        flg!(
                            "preview_failed_to_create_cache_dir",
                            generate_translation_hashmap(vec![("name", cache_dir.display().to_string()), ("reason", e.to_string())])
                        )
                        .as_str(),
                    );
                    break 'dir;
                }
                let path = tree_model.value(&tree_model.iter(&tree_path).unwrap(), column_path).get::<String>().unwrap();
                let name = tree_model.value(&tree_model.iter(&tree_path).unwrap(), column_name).get::<String>().unwrap();

                let file_name = get_full_name_from_path_name(&path, &name);
                let file_name = file_name.as_str();

                if let Some(extension) = Path::new(file_name).extension() {
                    let extension_lowercase = format!(".{}", extension.to_string_lossy().to_lowercase());

                    let is_raw_image = RAW_IMAGE_EXTENSIONS.contains(&extension_lowercase.as_str());
                    if !IMAGE_RS_EXTENSIONS.contains(&extension_lowercase.as_str()) && !is_raw_image {
                        break 'dir;
                    }

                    {
                        let preview_path = preview_path.borrow();
                        let preview_path = preview_path.deref();
                        if file_name == preview_path {
                            return; // Preview is already created, no need to recreate it
                        }
                    }
                    let img;
                    if !is_raw_image {
                        img = match image::open(&file_name) {
                            Ok(t) => t,
                            Err(e) => {
                                add_text_to_text_view(
                                    text_view_errors,
                                    flg!(
                                        "preview_temporary_file",
                                        generate_translation_hashmap(vec![("name", file_name.to_string()), ("reason", e.to_string())])
                                    )
                                    .as_str(),
                                );
                                break 'dir;
                            }
                        };
                    } else {
                        img = match get_dynamic_image_from_raw_image(file_name) {
                            Some(t) => t,
                            None => {
                                add_text_to_text_view(
                                    text_view_errors,
                                    flg!(
                                        "preview_temporary_file",
                                        generate_translation_hashmap(vec![("name", file_name.to_string()), ("reason", "None".to_string())])
                                    )
                                    .as_str(),
                                );
                                break 'dir;
                            }
                        }
                    }
                    if img.width() == 0 || img.height() == 0 {
                        add_text_to_text_view(
                            text_view_errors,
                            flg!("preview_0_size", generate_translation_hashmap(vec![("name", file_name.to_string())])).as_str(),
                        );
                        break 'dir;
                    }
                    let img = resize_dynamic_image_dimension(img, (400, 400), &FilterType::Triangle); // Triangle and Nearest is the fastest
                    let file_dir = match is_raw_image {
                        true => cache_dir.join("cached_file.jpg"),
                        false => cache_dir.join(format!("cached_file.{}", extension.to_string_lossy().to_lowercase())),
                    };
                    if let Err(e) = img.save(&file_dir) {
                        add_text_to_text_view(
                            text_view_errors,
                            flg!(
                                "preview_temporary_image_save",
                                generate_translation_hashmap(vec![("name", file_dir.display().to_string()), ("reason", e.to_string())])
                            )
                            .as_str(),
                        );
                        let _ = fs::remove_file(&file_dir);
                        break 'dir;
                    }
                    let string_dir = file_dir.to_string_lossy().to_string();
                    image_preview.set_from_file(string_dir);

                    {
                        let mut preview_path = preview_path.borrow_mut();
                        *preview_path = file_name.to_string();
                    }

                    if let Err(e) = fs::remove_file(&file_dir) {
                        add_text_to_text_view(
                            text_view_errors,
                            flg!(
                                "preview_temporary_image_remove",
                                generate_translation_hashmap(vec![("name", file_dir.display().to_string()), ("reason", e.to_string())])
                            )
                            .as_str(),
                        );
                        break 'dir;
                    }
                    created_image = true;
                }
                break 'dir;
            }
        }
    }
    if created_image {
        image_preview.show();
    } else {
        image_preview.hide();
        {
            let mut preview_path = preview_path.borrow_mut();
            *preview_path = "".to_string();
        }
    }
}
