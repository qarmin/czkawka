use czkawka_core::tools::similar_images::SIMILAR_VALUES;
use czkawka_core::tools::similar_videos::MAX_TOLERANCE;
use gtk4::prelude::*;

use crate::gtk_traits::ComboBoxTraits;
use crate::gui_structs::gui_data::GuiData;
use crate::help_combo_box::{
    DUPLICATES_CHECK_METHOD_COMBO_BOX, DUPLICATES_HASH_TYPE_COMBO_BOX, IMAGES_GEOMETRIC_INVARIANCE_COMBO_BOX, IMAGES_HASH_SIZE_COMBO_BOX, IMAGES_HASH_TYPE_COMBO_BOX,
    IMAGES_RESIZE_ALGORITHM_COMBO_BOX,
};
use crate::help_functions::scale_set_min_max_values;
use crate::language_functions::LANGUAGES_ALL;

pub(crate) fn initialize_gui(gui_data: &GuiData) {
    //// Initialize button
    {
        let buttons = &gui_data.bottom_buttons.buttons_array;
        for button in buttons {
            button.set_visible(false);
        }
        gui_data.bottom_buttons.buttons_search.set_visible(true);
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
    gui_data
        .main_notebook
        .combo_box_image_geometric_invariance
        .set_model_and_first(IMAGES_GEOMETRIC_INVARIANCE_COMBO_BOX.iter().map(|e| &e.eng_name));

    //// Initialize main scrolled view with notebook
    {
        // Set step increment
        let scale_similarity_similar_images = gui_data.main_notebook.scale_similarity_similar_images.clone();
        scale_set_min_max_values(&scale_similarity_similar_images, 0_f64, SIMILAR_VALUES[0][5] as f64, 15_f64, Some(1_f64));

        // Set step increment
        let scale_similarity_similar_videos = gui_data.main_notebook.scale_similarity_similar_videos.clone();
        scale_set_min_max_values(&scale_similarity_similar_videos, 0_f64, MAX_TOLERANCE as f64, 15_f64, Some(1_f64));
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
