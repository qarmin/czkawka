use czkawka_core::tools::similar_images::SIMILAR_VALUES;
use czkawka_core::tools::similar_videos::MAX_TOLERANCE;
use glib::types::Type;
use gtk4::SelectionMode;
use gtk4::prelude::*;

use crate::create_tree_view::{create_tree_view_excluded_directories, create_tree_view_included_directories};
use crate::dicom_traits::ComboBoxTraits;
use crate::gui_structs::gui_data::GuiData;
use crate::help_combo_box::{
    DUPLICATES_CHECK_METHOD_COMBO_BOX, DUPLICATES_HASH_TYPE_COMBO_BOX, IMAGES_HASH_SIZE_COMBO_BOX, IMAGES_HASH_TYPE_COMBO_BOX, IMAGES_RESIZE_ALGORITHM_COMBO_BOX,
};
use crate::help_functions::{KEY_DELETE, get_list_store, get_tree_view_name_from_notebook_upper_enum, scale_set_min_max_values};
use crate::language_functions::LANGUAGES_ALL;
use crate::notebook_enums::NotebookUpperEnum;
use crate::opening_selecting_records::{opening_double_click_function_directories, opening_enter_function_ported_upper_directories};

pub(crate) fn initialize_gui(gui_data: &GuiData) {
    //// Initialize button
    {
        let buttons = &gui_data.bottom_buttons.buttons_array;
        for button in buttons {
            button.hide();
        }
        gui_data.bottom_buttons.buttons_search.show();
    }
    //// Initialize language combo box
    gui_data
        .settings
        .combo_box_settings_language
        .set_model_and_first(LANGUAGES_ALL.iter().map(|e| &e.combo_box_text));

    gui_data
        .main_notebook
        .combo_box_duplicate_check_method
        .set_model_and_first(DUPLICATES_CHECK_METHOD_COMBO_BOX.iter().map(|e| &e.eng_name));
    gui_data
        .main_notebook
        .combo_box_duplicate_hash_type
        .set_model_and_first(DUPLICATES_HASH_TYPE_COMBO_BOX.iter().map(|e| &e.eng_name));

    gui_data
        .main_notebook
        .combo_box_image_hash_algorithm
        .set_model_and_first(IMAGES_HASH_TYPE_COMBO_BOX.iter().map(|e| &e.eng_name));
    gui_data
        .main_notebook
        .combo_box_image_hash_size
        .set_model_and_first(IMAGES_HASH_SIZE_COMBO_BOX.iter().map(|e| e.to_string()));
    gui_data
        .main_notebook
        .combo_box_image_resize_algorithm
        .set_model_and_first(IMAGES_RESIZE_ALGORITHM_COMBO_BOX.iter().map(|e| &e.eng_name));

    //// Initialize main scrolled view with notebook
    {
        // Set step increment
        let scale_similarity_similar_images = gui_data.main_notebook.scale_similarity_similar_images.clone();
        scale_set_min_max_values(&scale_similarity_similar_images, 0_f64, SIMILAR_VALUES[0][5] as f64, 15_f64, Some(1_f64));

        // Set step increment
        let scale_similarity_similar_videos = gui_data.main_notebook.scale_similarity_similar_videos.clone();
        scale_set_min_max_values(&scale_similarity_similar_videos, 0_f64, MAX_TOLERANCE as f64, 15_f64, Some(1_f64));
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
        let stop_flag = gui_data.stop_flag.clone();

        window_progress.connect_close_request(move |_| {
            stop_flag.store(true, std::sync::atomic::Ordering::Relaxed);
            glib::Propagation::Stop
        });
    }
}
