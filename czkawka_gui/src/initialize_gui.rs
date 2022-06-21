use std::cell::RefCell;
use std::ops::Deref;
use std::path::Path;
use std::rc::Rc;

use gdk4::gdk_pixbuf::Pixbuf;
use gtk4::gdk_pixbuf::InterpType;
use gtk4::prelude::*;
use gtk4::{CheckButton, Image, SelectionMode, TextView, TreeView};

#[cfg(feature = "heif")]
use czkawka_core::common::get_dynamic_image_from_heic;
use czkawka_core::common::{HEIC_EXTENSIONS, IMAGE_RS_EXTENSIONS, RAW_IMAGE_EXTENSIONS};
use czkawka_core::similar_images::SIMILAR_VALUES;
use czkawka_core::similar_videos::MAX_TOLERANCE;

use crate::create_tree_view::*;
use crate::delete_things;
use crate::flg;
use crate::gui_structs::gui_data::*;
use crate::help_combo_box::{
    DUPLICATES_CHECK_METHOD_COMBO_BOX, DUPLICATES_HASH_TYPE_COMBO_BOX, IMAGES_HASH_SIZE_COMBO_BOX, IMAGES_HASH_TYPE_COMBO_BOX, IMAGES_RESIZE_ALGORITHM_COMBO_BOX,
};
use crate::help_functions::*;
use crate::language_functions::LANGUAGES_ALL;
use crate::localizer_core::generate_translation_hashmap;
use crate::notebook_enums::NotebookMainEnum;
use crate::notebook_info::NOTEBOOKS_INFO;
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

                let list_store: gtk4::ListStore = gtk4::ListStore::new(NOTEBOOKS_INFO[NotebookMainEnum::Duplicate as usize].columns_types);

                tree_view.set_model(Some(&list_store));
                tree_view.selection().set_mode(SelectionMode::Multiple);
                tree_view.selection().set_select_function(select_function_duplicates);

                create_tree_view_duplicates(&tree_view);

                tree_view.set_widget_name("tree_view_duplicate_finder");
                scrolled_window.set_child(Some(&tree_view));
                scrolled_window.show();
            }
            // Empty Folders
            {
                let scrolled_window = gui_data.main_notebook.scrolled_window_empty_folder_finder.clone();
                let tree_view = gui_data.main_notebook.tree_view_empty_folder_finder.clone();

                let list_store: gtk4::ListStore = gtk4::ListStore::new(NOTEBOOKS_INFO[NotebookMainEnum::EmptyDirectories as usize].columns_types);

                tree_view.set_model(Some(&list_store));
                tree_view.selection().set_mode(SelectionMode::Multiple);

                create_tree_view_empty_folders(&tree_view);

                tree_view.set_widget_name("tree_view_empty_folder_finder");

                scrolled_window.set_child(Some(&tree_view));
                scrolled_window.show();
            }
            // Empty Files
            {
                let scrolled_window = gui_data.main_notebook.scrolled_window_empty_files_finder.clone();
                let tree_view = gui_data.main_notebook.tree_view_empty_files_finder.clone();

                let list_store: gtk4::ListStore = gtk4::ListStore::new(NOTEBOOKS_INFO[NotebookMainEnum::EmptyFiles as usize].columns_types);

                tree_view.set_model(Some(&list_store));
                tree_view.selection().set_mode(SelectionMode::Multiple);

                create_tree_view_empty_files(&tree_view);

                tree_view.set_widget_name("tree_view_empty_files_finder");
                scrolled_window.set_child(Some(&tree_view));
                scrolled_window.show();
            }
            // Temporary Files
            {
                let scrolled_window = gui_data.main_notebook.scrolled_window_temporary_files_finder.clone();
                let tree_view = gui_data.main_notebook.tree_view_temporary_files_finder.clone();

                let list_store: gtk4::ListStore = gtk4::ListStore::new(NOTEBOOKS_INFO[NotebookMainEnum::Temporary as usize].columns_types);

                tree_view.set_model(Some(&list_store));
                tree_view.selection().set_mode(SelectionMode::Multiple);

                create_tree_view_temporary_files(&tree_view);

                tree_view.set_widget_name("tree_view_temporary_files_finder");
                scrolled_window.set_child(Some(&tree_view));
                scrolled_window.show();
            }
            // Big Files
            {
                let scrolled_window = gui_data.main_notebook.scrolled_window_big_files_finder.clone();
                let tree_view = gui_data.main_notebook.tree_view_big_files_finder.clone();

                let list_store: gtk4::ListStore = gtk4::ListStore::new(NOTEBOOKS_INFO[NotebookMainEnum::BigFiles as usize].columns_types);

                tree_view.set_model(Some(&list_store));
                tree_view.selection().set_mode(SelectionMode::Multiple);

                create_tree_view_big_files(&tree_view);

                tree_view.set_widget_name("tree_view_big_files_finder");
                scrolled_window.set_child(Some(&tree_view));
                scrolled_window.show();
            }
            // Similar Images
            {
                let scrolled_window = gui_data.main_notebook.scrolled_window_similar_images_finder.clone();
                let tree_view = gui_data.main_notebook.tree_view_similar_images_finder.clone();

                let image_preview = gui_data.main_notebook.image_preview_similar_images.clone();
                image_preview.hide();

                let list_store: gtk4::ListStore = gtk4::ListStore::new(NOTEBOOKS_INFO[NotebookMainEnum::SimilarImages as usize].columns_types);

                tree_view.set_model(Some(&list_store));
                tree_view.selection().set_mode(SelectionMode::Multiple);
                tree_view.selection().set_select_function(select_function_similar_images);

                create_tree_view_similar_images(&tree_view);

                tree_view.set_widget_name("tree_view_similar_images_finder");
                scrolled_window.set_child(Some(&tree_view));
                scrolled_window.show();
            }
            // Similar Videos
            {
                let scrolled_window = gui_data.main_notebook.scrolled_window_similar_videos_finder.clone();
                let tree_view = gui_data.main_notebook.tree_view_similar_videos_finder.clone();

                let list_store: gtk4::ListStore = gtk4::ListStore::new(NOTEBOOKS_INFO[NotebookMainEnum::SimilarVideos as usize].columns_types);

                tree_view.set_model(Some(&list_store));
                tree_view.selection().set_mode(SelectionMode::Multiple);
                tree_view.selection().set_select_function(select_function_similar_videos);

                create_tree_view_similar_videos(&tree_view);

                tree_view.set_widget_name("tree_view_similar_videos_finder");
                scrolled_window.set_child(Some(&tree_view));
                scrolled_window.show();
            }
            // Same Music
            {
                let scrolled_window = gui_data.main_notebook.scrolled_window_same_music_finder.clone();
                let tree_view = gui_data.main_notebook.tree_view_same_music_finder.clone();

                let list_store: gtk4::ListStore = gtk4::ListStore::new(NOTEBOOKS_INFO[NotebookMainEnum::SameMusic as usize].columns_types);

                tree_view.set_model(Some(&list_store));
                tree_view.selection().set_mode(SelectionMode::Multiple);
                tree_view.selection().set_select_function(select_function_same_music);

                create_tree_view_same_music(&tree_view);

                tree_view.set_widget_name("tree_view_same_music_finder");
                scrolled_window.set_child(Some(&tree_view));
                scrolled_window.show();
            }
            // Invalid Symlinks
            {
                let scrolled_window = gui_data.main_notebook.scrolled_window_invalid_symlinks.clone();
                let tree_view = gui_data.main_notebook.tree_view_invalid_symlinks.clone();

                let list_store: gtk4::ListStore = gtk4::ListStore::new(NOTEBOOKS_INFO[NotebookMainEnum::Symlinks as usize].columns_types);

                tree_view.set_model(Some(&list_store));
                tree_view.selection().set_mode(SelectionMode::Multiple);

                create_tree_view_invalid_symlinks(&tree_view);

                tree_view.set_widget_name("tree_view_invalid_symlinks");
                scrolled_window.set_child(Some(&tree_view));
                scrolled_window.show();
            }
            // Broken Files
            {
                let scrolled_window = gui_data.main_notebook.scrolled_window_broken_files.clone();
                let tree_view = gui_data.main_notebook.tree_view_broken_files.clone();

                let list_store: gtk4::ListStore = gtk4::ListStore::new(NOTEBOOKS_INFO[NotebookMainEnum::BrokenFiles as usize].columns_types);

                tree_view.set_model(Some(&list_store));
                tree_view.selection().set_mode(SelectionMode::Multiple);

                create_tree_view_broken_files(&tree_view);

                tree_view.set_widget_name("tree_view_broken_files");
                scrolled_window.set_child(Some(&tree_view));
                scrolled_window.show();
            }
            // Bad Extensions
            {
                let scrolled_window = gui_data.main_notebook.scrolled_window_bad_extensions.clone();
                let tree_view = gui_data.main_notebook.tree_view_bad_extensions.clone();

                let list_store: gtk4::ListStore = gtk4::ListStore::new(NOTEBOOKS_INFO[NotebookMainEnum::BadExtensions as usize].columns_types);

                tree_view.set_model(Some(&list_store));
                tree_view.selection().set_mode(SelectionMode::Multiple);

                create_tree_view_broken_files(&tree_view);

                tree_view.set_widget_name("tree_view_bad_extensions");
                scrolled_window.set_child(Some(&tree_view));
                scrolled_window.show();
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
            let gc = gui_data.upper_notebook.gc_tree_view_included_directories.clone();

            let col_types: [glib::types::Type; 2] = [
                glib::types::Type::STRING, // Path
                glib::types::Type::BOOL,   // ReferenceButton
            ];
            let list_store: gtk4::ListStore = gtk4::ListStore::new(&col_types);

            tree_view.set_model(Some(&list_store));
            tree_view.selection().set_mode(SelectionMode::Multiple);

            create_tree_view_included_directories(&tree_view);

            tree_view.set_widget_name("tree_view_upper_included_directories");
            scrolled_window.set_child(Some(&tree_view));
            scrolled_window.show();

            gc.connect_pressed(opening_double_click_function_directories);
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
            let gc = gui_data.upper_notebook.gc_tree_view_excluded_directories.clone();

            let col_types: [glib::types::Type; 1] = [glib::types::Type::STRING];
            let list_store: gtk4::ListStore = gtk4::ListStore::new(&col_types);

            tree_view.set_model(Some(&list_store));
            tree_view.selection().set_mode(SelectionMode::Multiple);

            create_tree_view_excluded_directories(&tree_view);

            tree_view.set_widget_name("tree_view_upper_excluded_directories");
            scrolled_window.set_child(Some(&tree_view));
            scrolled_window.show();

            gc.connect_pressed(opening_double_click_function_directories);
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

        window_progress.connect_close_request(move |_| {
            stop_sender.send(()).unwrap();
            gtk4::Inhibit(true)
        });
    }

    // This not need to be run in different code block, but this looks a little less complicated if is available in
    connect_event_buttons(gui_data);
    connect_event_mouse(gui_data);
}

fn connect_event_mouse(gui_data: &GuiData) {
    // GTK 4
    for gc in [
        &gui_data.main_notebook.gc_tree_view_duplicate_finder,
        &gui_data.main_notebook.gc_tree_view_empty_folder_finder,
        &gui_data.main_notebook.gc_tree_view_empty_files_finder,
        &gui_data.main_notebook.gc_tree_view_temporary_files_finder,
        &gui_data.main_notebook.gc_tree_view_big_files_finder,
        &gui_data.main_notebook.gc_tree_view_similar_images_finder,
        &gui_data.main_notebook.gc_tree_view_similar_videos_finder,
        &gui_data.main_notebook.gc_tree_view_same_music_finder,
        &gui_data.main_notebook.gc_tree_view_invalid_symlinks,
        &gui_data.main_notebook.gc_tree_view_broken_files,
        &gui_data.main_notebook.gc_tree_view_bad_extensions,
    ] {
        gc.set_button(0);
        gc.connect_pressed(opening_double_click_function);
        gc.connect_released(opening_middle_mouse_function); // TODO GTK 4 - https://github.com/gtk-rs/gtk4-rs/issues/1043
    }

    // Duplicate
    {
        let text_view_errors = gui_data.text_view_errors.clone();
        let check_button_settings_show_preview = gui_data.settings.check_button_settings_show_preview_duplicates.clone();
        let image_preview = gui_data.main_notebook.image_preview_duplicates.clone();
        let preview_path = gui_data.preview_path.clone();
        let tree_view = gui_data.main_notebook.tree_view_duplicate_finder.clone();

        tree_view.set_property("activate-on-single-click", true);

        // TODO GTK 4, currently not works, connect_pressed shows previous thing - https://gitlab.gnome.org/GNOME/gtk/-/issues/4939
        // Use connect_released when it will be fixed, currently using connect_row_activated workaround
        tree_view.connect_row_activated(move |tree_view, _b, _c| {
            let nb_object = &NOTEBOOKS_INFO[NotebookMainEnum::Duplicate as usize];
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
        });
    }
    // Similar Images
    {
        let text_view_errors = gui_data.text_view_errors.clone();
        let check_button_settings_show_preview = gui_data.settings.check_button_settings_show_preview_similar_images.clone();
        let preview_path = gui_data.preview_path.clone();
        let image_preview = gui_data.main_notebook.image_preview_similar_images.clone();
        let tree_view = gui_data.main_notebook.tree_view_similar_images_finder.clone();

        tree_view.set_property("activate-on-single-click", true);

        // TODO GTK 4, currently not works, connect_pressed shows previous thing
        tree_view.connect_row_activated(move |tree_view, _b, _c| {
            let nb_object = &NOTEBOOKS_INFO[NotebookMainEnum::SimilarImages as usize];
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
        });
    }
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
        gui_data.main_notebook.evk_tree_view_bad_extensions.clone(),
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
            let nb_object = &NOTEBOOKS_INFO[NotebookMainEnum::Duplicate as usize];
            show_preview(
                &event_controller_key.widget().downcast::<TreeView>().unwrap(),
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
            let nb_object = &NOTEBOOKS_INFO[NotebookMainEnum::SimilarImages as usize];
            show_preview(
                &event_controller_key.widget().downcast::<TreeView>().unwrap(),
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
        // TODO labels on {} are in testing stage, so we just ignore for now this warning until found better idea how to fix this
        #[allow(clippy::never_loop)]
        'dir: loop {
            let path = tree_model.get::<String>(&tree_model.iter(&tree_path).unwrap(), column_path);
            let name = tree_model.get::<String>(&tree_model.iter(&tree_path).unwrap(), column_name);

            let file_name = get_full_name_from_path_name(&path, &name);
            let file_name = file_name.as_str();

            {
                let preview_path = preview_path.borrow();
                let preview_path = preview_path.deref();
                if file_name == preview_path {
                    return; // Preview is already created, no need to recreate it
                }
            }

            let is_heic;
            let is_webp;
            if let Some(extension) = Path::new(&name).extension() {
                let extension = format!(".{}", extension.to_string_lossy().to_lowercase());
                is_heic = HEIC_EXTENSIONS.contains(&extension.as_str());
                is_webp = ".webp" == extension;
                if !RAW_IMAGE_EXTENSIONS.contains(&extension.as_str()) && !IMAGE_RS_EXTENSIONS.contains(&extension.as_str()) && !is_heic {
                    break 'dir;
                }
            } else {
                break 'dir;
            }
            let mut pixbuf = if is_heic || is_webp {
                let image = if is_heic {
                    #[cfg(feature = "heif")]
                    match get_dynamic_image_from_heic(file_name) {
                        Ok(t) => t,
                        Err(e) => {
                            add_text_to_text_view(
                                text_view_errors,
                                flg!(
                                    "preview_image_opening_failure",
                                    generate_translation_hashmap(vec![("name", file_name.to_string()), ("reason", e.to_string())])
                                )
                                .as_str(),
                            );
                            break 'dir;
                        }
                    }

                    #[cfg(not(feature = "heif"))]
                    panic!("")
                } else if is_webp {
                    match image::open(file_name) {
                        Ok(t) => t,
                        Err(e) => {
                            add_text_to_text_view(
                                text_view_errors,
                                flg!(
                                    "preview_image_opening_failure",
                                    generate_translation_hashmap(vec![("name", file_name.to_string()), ("reason", e.to_string())])
                                )
                                .as_str(),
                            );
                            break 'dir;
                        }
                    }
                } else {
                    panic!("");
                };

                match get_pixbuf_from_dynamic_image(&image) {
                    Ok(t) => t,
                    Err(e) => {
                        add_text_to_text_view(
                            text_view_errors,
                            flg!(
                                "preview_image_opening_failure",
                                generate_translation_hashmap(vec![("name", file_name.to_string()), ("reason", e.to_string())])
                            )
                            .as_str(),
                        );
                        break 'dir;
                    }
                }
            } else {
                match Pixbuf::from_file(file_name) {
                    Ok(pixbuf) => pixbuf,
                    Err(e) => {
                        add_text_to_text_view(
                            text_view_errors,
                            flg!(
                                "preview_image_opening_failure",
                                generate_translation_hashmap(vec![("name", file_name.to_string()), ("reason", e.to_string())])
                            )
                            .as_str(),
                        );
                        break 'dir;
                    }
                }
            };

            pixbuf = match resize_pixbuf_dimension(pixbuf, (800, 800), InterpType::Nearest) {
                None => {
                    add_text_to_text_view(
                        text_view_errors,
                        flg!("preview_image_resize_failure", generate_translation_hashmap(vec![("name", file_name.to_string())])).as_str(),
                    );
                    break 'dir;
                }
                Some(pixbuf) => pixbuf,
            };

            image_preview.set_from_pixbuf(Some(&pixbuf));
            {
                let mut preview_path = preview_path.borrow_mut();
                *preview_path = file_name.to_string();
            }

            created_image = true;

            break 'dir;
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
