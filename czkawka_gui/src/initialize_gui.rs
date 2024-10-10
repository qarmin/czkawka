use std::cell::RefCell;
use std::rc::Rc;

use czkawka_core::common_image::get_dynamic_image_from_path;
use czkawka_core::similar_images::SIMILAR_VALUES;
use czkawka_core::similar_videos::MAX_TOLERANCE;
use gdk4::gdk_pixbuf::Pixbuf;
use glib::types::Type;
use gtk4::gdk_pixbuf::InterpType;
use gtk4::prelude::*;
use gtk4::{CheckButton, Image, ScrolledWindow, SelectionMode, TextView, TreeModel, TreePath, TreeSelection, TreeView};

use crate::create_tree_view::*;
use crate::gui_structs::gui_data::*;
use crate::help_combo_box::{
    DUPLICATES_CHECK_METHOD_COMBO_BOX, DUPLICATES_HASH_TYPE_COMBO_BOX, IMAGES_HASH_SIZE_COMBO_BOX, IMAGES_HASH_TYPE_COMBO_BOX, IMAGES_RESIZE_ALGORITHM_COMBO_BOX,
};
use crate::help_functions::*;
use crate::language_functions::LANGUAGES_ALL;
use crate::localizer_core::generate_translation_hashmap;
use crate::notebook_enums::{NotebookMainEnum, NotebookUpperEnum};
use crate::notebook_info::NOTEBOOKS_INFO;
use crate::opening_selecting_records::*;
use crate::{delete_things, flg};

pub fn initialize_gui(gui_data: &GuiData) {
    //// Initialize button
    {
        let buttons = &gui_data.bottom_buttons.buttons_array;
        for button in buttons {
            button.hide();
        }
        gui_data.bottom_buttons.buttons_search.show();
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
            for check_type in IMAGES_HASH_TYPE_COMBO_BOX {
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
            scale_set_min_max_values(&scale_similarity_similar_images, 0_f64, SIMILAR_VALUES[0][5] as f64, 15_f64, Some(1_f64));
        }
        // Set step increment
        {
            let scale_similarity_similar_videos = gui_data.main_notebook.scale_similarity_similar_videos.clone();
            scale_set_min_max_values(&scale_similarity_similar_videos, 0_f64, MAX_TOLERANCE as f64, 15_f64, Some(1_f64));
        }

        // Set Main Scrolled Window Treeviews
        {
            create_column_types(
                &gui_data.main_notebook.scrolled_window_duplicate_finder,
                &gui_data.main_notebook.tree_view_duplicate_finder,
                NotebookMainEnum::Duplicate,
                Some(select_function_duplicates),
                create_tree_view_duplicates,
                Some(&gui_data.main_notebook.image_preview_duplicates),
            );
            create_column_types(
                &gui_data.main_notebook.scrolled_window_similar_images_finder,
                &gui_data.main_notebook.tree_view_similar_images_finder,
                NotebookMainEnum::SimilarImages,
                Some(select_function_similar_images),
                create_tree_view_similar_images,
                Some(&gui_data.main_notebook.image_preview_similar_images),
            );
            create_column_types(
                &gui_data.main_notebook.scrolled_window_similar_videos_finder,
                &gui_data.main_notebook.tree_view_similar_videos_finder,
                NotebookMainEnum::SimilarVideos,
                Some(select_function_similar_videos),
                create_tree_view_similar_videos,
                None,
            );
            create_column_types(
                &gui_data.main_notebook.scrolled_window_same_music_finder,
                &gui_data.main_notebook.tree_view_same_music_finder,
                NotebookMainEnum::SameMusic,
                Some(select_function_same_music),
                create_tree_view_same_music,
                None,
            );
            create_column_types(
                &gui_data.main_notebook.scrolled_window_empty_folder_finder,
                &gui_data.main_notebook.tree_view_empty_folder_finder,
                NotebookMainEnum::EmptyDirectories,
                None,
                create_tree_view_empty_folders,
                None,
            );
            create_column_types(
                &gui_data.main_notebook.scrolled_window_empty_files_finder,
                &gui_data.main_notebook.tree_view_empty_files_finder,
                NotebookMainEnum::EmptyFiles,
                None,
                create_tree_view_empty_files,
                None,
            );
            create_column_types(
                &gui_data.main_notebook.scrolled_window_temporary_files_finder,
                &gui_data.main_notebook.tree_view_temporary_files_finder,
                NotebookMainEnum::Temporary,
                None,
                create_tree_view_temporary_files,
                None,
            );
            create_column_types(
                &gui_data.main_notebook.scrolled_window_big_files_finder,
                &gui_data.main_notebook.tree_view_big_files_finder,
                NotebookMainEnum::BigFiles,
                None,
                create_tree_view_big_files,
                None,
            );
            create_column_types(
                &gui_data.main_notebook.scrolled_window_invalid_symlinks,
                &gui_data.main_notebook.tree_view_invalid_symlinks,
                NotebookMainEnum::Symlinks,
                None,
                create_tree_view_invalid_symlinks,
                None,
            );
            create_column_types(
                &gui_data.main_notebook.scrolled_window_broken_files,
                &gui_data.main_notebook.tree_view_broken_files,
                NotebookMainEnum::BrokenFiles,
                None,
                create_tree_view_broken_files,
                None,
            );
            create_column_types(
                &gui_data.main_notebook.scrolled_window_bad_extensions,
                &gui_data.main_notebook.tree_view_bad_extensions,
                NotebookMainEnum::BadExtensions,
                None,
                create_tree_view_bad_extensions,
                None,
            );
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

            let col_types: [Type; 2] = [
                Type::STRING, // Path
                Type::BOOL,   // ReferenceButton
            ];
            let list_store: gtk4::ListStore = gtk4::ListStore::new(&col_types);

            tree_view.set_model(Some(&list_store));
            tree_view.selection().set_mode(SelectionMode::Multiple);

            create_tree_view_included_directories(&tree_view);

            tree_view.set_widget_name(get_tree_view_name_from_notebook_upper_enum(NotebookUpperEnum::IncludedDirectories));
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
                        list_store.remove(&list_store.iter(tree_path).expect("Using invalid tree_path"));
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

            let col_types: [Type; 1] = [Type::STRING];
            let list_store: gtk4::ListStore = gtk4::ListStore::new(&col_types);

            tree_view.set_model(Some(&list_store));
            tree_view.selection().set_mode(SelectionMode::Multiple);

            create_tree_view_excluded_directories(&tree_view);

            tree_view.set_widget_name(get_tree_view_name_from_notebook_upper_enum(NotebookUpperEnum::ExcludedDirectories));
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
                        list_store.remove(&list_store.iter(tree_path).expect("Using invalid tree_path"));
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
            stop_sender.send(()).expect("Failed to send stop signal");
            glib::Propagation::Stop
        });
    }

    // This not need to be run in different code block, but this looks a little less complicated if is available in
    connect_event_buttons(gui_data);
    connect_event_mouse(gui_data);
}

fn create_column_types(
    scrolled_window: &ScrolledWindow,
    tree_view: &TreeView,
    notebook_enum: NotebookMainEnum,
    select_function: Option<fn(&TreeSelection, &TreeModel, &TreePath, bool) -> bool>,
    create_tree_view_func: fn(&TreeView),
    image_preview: Option<&Image>,
) {
    if let Some(image_preview) = image_preview {
        image_preview.hide();
    }
    let list_store: gtk4::ListStore = gtk4::ListStore::new(NOTEBOOKS_INFO[notebook_enum as usize].columns_types);

    tree_view.set_model(Some(&list_store));
    tree_view.selection().set_mode(SelectionMode::Multiple);
    if let Some(select_function) = select_function {
        tree_view.selection().set_select_function(select_function);
    }

    create_tree_view_func(tree_view);

    tree_view.set_widget_name(get_tree_view_name_from_notebook_enum(notebook_enum));
    scrolled_window.set_child(Some(tree_view));
    scrolled_window.show();
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
        let check_button_settings_use_rust_preview = gui_data.settings.check_button_settings_use_rust_preview.clone();

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
                &preview_path,
                nb_object.column_path,
                nb_object.column_name,
                check_button_settings_use_rust_preview.is_active(),
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
        let check_button_settings_use_rust_preview = gui_data.settings.check_button_settings_use_rust_preview.clone();

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
                &preview_path,
                nb_object.column_path,
                nb_object.column_name,
                check_button_settings_use_rust_preview.is_active(),
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
        let check_button_settings_use_rust_preview = gui_data.settings.check_button_settings_use_rust_preview.clone();

        evk.connect_key_pressed(opening_enter_function_ported);

        evk.connect_key_released(move |event_controller_key, _key_value, key_code, _modifier_type| {
            if key_code == KEY_DELETE {
                glib::MainContext::default().spawn_local(delete_things(gui_data_clone.clone()));
            }
            let preview_path = preview_path.clone();
            let nb_object = &NOTEBOOKS_INFO[NotebookMainEnum::Duplicate as usize];
            show_preview(
                &event_controller_key
                    .widget()
                    .expect("Item has no widget")
                    .downcast::<TreeView>()
                    .expect("Widget is not TreeView"),
                &text_view_errors,
                &check_button_settings_show_preview,
                &image_preview,
                &preview_path,
                nb_object.column_path,
                nb_object.column_name,
                check_button_settings_use_rust_preview.is_active(),
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
        let check_button_settings_use_rust_preview = gui_data.settings.check_button_settings_use_rust_preview.clone();

        evk.connect_key_pressed(opening_enter_function_ported);

        evk.connect_key_released(move |event_controller_key, _key_value, key_code, _modifier_type| {
            if key_code == KEY_DELETE {
                glib::MainContext::default().spawn_local(delete_things(gui_data_clone.clone()));
            }
            let preview_path = preview_path.clone();
            let nb_object = &NOTEBOOKS_INFO[NotebookMainEnum::SimilarImages as usize];
            show_preview(
                &event_controller_key
                    .widget()
                    .expect("Item has no widget")
                    .downcast::<TreeView>()
                    .expect("Widget is not TreeView"),
                &text_view_errors,
                &check_button_settings_show_preview_similar_images,
                &image_preview,
                &preview_path,
                nb_object.column_path,
                nb_object.column_name,
                check_button_settings_use_rust_preview.is_active(),
            );
        });
    }
}

fn show_preview(
    tree_view: &TreeView,
    text_view_errors: &TextView,
    check_button_settings_show_preview: &CheckButton,
    image_preview: &Image,
    preview_path: &Rc<RefCell<String>>,
    column_path: i32,
    column_name: i32,
    use_rust_preview: bool,
) {
    let (selected_rows, tree_model) = tree_view.selection().selected_rows();

    let mut created_image = false;

    // Only show preview when selected is only one item, because there is no method to recognize current clicked item in multiselection
    if selected_rows.len() == 1 && check_button_settings_show_preview.is_active() {
        let tree_path = selected_rows[0].clone();
        // TODO labels on {} are in testing stage, so we just ignore for now this warning until found better idea how to fix this
        #[allow(clippy::never_loop)]
        'dir: loop {
            let path = tree_model.get::<String>(&tree_model.iter(&tree_path).expect("Invalid tree_path"), column_path);
            let name = tree_model.get::<String>(&tree_model.iter(&tree_path).expect("Invalid tree_path"), column_name);

            let file_name = get_full_name_from_path_name(&path, &name);

            if file_name == preview_path.borrow().as_str() {
                return; // Preview is already created, no need to recreate it
            }

            let mut pixbuf = if use_rust_preview {
                let image = match get_dynamic_image_from_path(&file_name) {
                    Ok(t) => t,
                    Err(e) => {
                        add_text_to_text_view(text_view_errors, flg!("preview_image_opening_failure", name = file_name, reason = e.to_string()).as_str());
                        break 'dir;
                    }
                };

                match get_pixbuf_from_dynamic_image(&image) {
                    Ok(t) => t,
                    Err(e) => {
                        add_text_to_text_view(text_view_errors, flg!("preview_image_opening_failure", name = file_name, reason = e.to_string()).as_str());
                        break 'dir;
                    }
                }
            } else {
                match Pixbuf::from_file(&file_name) {
                    Ok(pixbuf) => pixbuf,
                    Err(e) => {
                        add_text_to_text_view(
                            text_view_errors,
                            flg!(
                                "preview_image_opening_failure",
                                generate_translation_hashmap(vec![("name", file_name), ("reason", e.to_string())])
                            )
                            .as_str(),
                        );
                        break 'dir;
                    }
                }
            };
            pixbuf = match resize_pixbuf_dimension(&pixbuf, (800, 800), InterpType::Bilinear) {
                None => {
                    add_text_to_text_view(text_view_errors, flg!("preview_image_resize_failure", name = file_name).as_str());
                    break 'dir;
                }
                Some(pixbuf) => pixbuf,
            };

            image_preview.set_from_pixbuf(Some(&pixbuf));
            {
                let mut preview_path = preview_path.borrow_mut();
                *preview_path = file_name;
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
            *preview_path = String::new();
        }
    }
}
