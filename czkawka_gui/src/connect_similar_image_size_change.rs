use gtk::prelude::*;

use czkawka_core::similar_images::{get_string_from_similarity, Similarity, SIMILAR_VALUES};

use crate::gui_data::GuiData;

pub fn connect_similar_image_size_change(gui_data: &GuiData) {
    let label_similar_images_minimal_similarity = gui_data.main_notebook.label_similar_images_minimal_similarity.clone();
    label_similar_images_minimal_similarity.set_text(&get_string_from_similarity(&Similarity::Similar(SIMILAR_VALUES[0][5]), 8));

    {
        let radio_button_similar_hash_size_8 = gui_data.main_notebook.radio_button_similar_hash_size_8.clone();
        let label_similar_images_minimal_similarity = gui_data.main_notebook.label_similar_images_minimal_similarity.clone();
        let scale_similarity_similar_images = gui_data.main_notebook.scale_similarity_similar_images.clone();
        radio_button_similar_hash_size_8.connect_toggled(move |_| {
            scale_similarity_similar_images.set_range(0_f64, SIMILAR_VALUES[0][5] as f64);
            scale_similarity_similar_images.set_fill_level(SIMILAR_VALUES[0][5] as f64);
            label_similar_images_minimal_similarity.set_text(&get_string_from_similarity(&Similarity::Similar(SIMILAR_VALUES[0][5]), 8));
        });
    }
    {
        let radio_button_similar_hash_size_16 = gui_data.main_notebook.radio_button_similar_hash_size_16.clone();
        let label_similar_images_minimal_similarity = gui_data.main_notebook.label_similar_images_minimal_similarity.clone();
        let scale_similarity_similar_images = gui_data.main_notebook.scale_similarity_similar_images.clone();
        radio_button_similar_hash_size_16.connect_toggled(move |_| {
            scale_similarity_similar_images.set_range(0_f64, SIMILAR_VALUES[1][5] as f64);
            scale_similarity_similar_images.set_fill_level(SIMILAR_VALUES[1][5] as f64);
            label_similar_images_minimal_similarity.set_text(&get_string_from_similarity(&Similarity::Similar(SIMILAR_VALUES[1][5]), 16));
        });
    }
    {
        let radio_button_similar_hash_size_32 = gui_data.main_notebook.radio_button_similar_hash_size_32.clone();
        let label_similar_images_minimal_similarity = gui_data.main_notebook.label_similar_images_minimal_similarity.clone();
        let scale_similarity_similar_images = gui_data.main_notebook.scale_similarity_similar_images.clone();
        radio_button_similar_hash_size_32.connect_toggled(move |_| {
            scale_similarity_similar_images.set_range(0_f64, SIMILAR_VALUES[2][5] as f64);
            scale_similarity_similar_images.set_fill_level(SIMILAR_VALUES[2][5] as f64);
            label_similar_images_minimal_similarity.set_text(&get_string_from_similarity(&Similarity::Similar(SIMILAR_VALUES[2][5]), 32));
        });
    }
    {
        let radio_button_similar_hash_size_64 = gui_data.main_notebook.radio_button_similar_hash_size_64.clone();
        let label_similar_images_minimal_similarity = gui_data.main_notebook.label_similar_images_minimal_similarity.clone();
        let scale_similarity_similar_images = gui_data.main_notebook.scale_similarity_similar_images.clone();
        radio_button_similar_hash_size_64.connect_toggled(move |_| {
            scale_similarity_similar_images.set_range(0_f64, SIMILAR_VALUES[3][5] as f64);
            scale_similarity_similar_images.set_fill_level(SIMILAR_VALUES[3][5] as f64);
            label_similar_images_minimal_similarity.set_text(&get_string_from_similarity(&Similarity::Similar(SIMILAR_VALUES[3][5]), 64));
        });
    }
}
