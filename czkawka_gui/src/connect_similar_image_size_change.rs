use crate::gui_data::GuiData;
use czkawka_core::similar_images::SIMILAR_VALUES;
use gtk::prelude::*;

pub fn connect_similar_image_size_change(gui_data: &GuiData) {
    // This should set values to max possible value like in return_similarity_from_similarity_preset and get_string_from_similarity
    {
        let radio_button_similar_hash_size_4 = gui_data.main_notebook.radio_button_similar_hash_size_4.clone();
        let scale_similarity = gui_data.main_notebook.scale_similarity.clone();
        radio_button_similar_hash_size_4.connect_clicked(move |_| {
            scale_similarity.set_range(0_f64, SIMILAR_VALUES[0][5] as f64);
            scale_similarity.set_fill_level(SIMILAR_VALUES[0][5] as f64);
        });
    }
    {
        let radio_button_similar_hash_size_8 = gui_data.main_notebook.radio_button_similar_hash_size_8.clone();
        let scale_similarity = gui_data.main_notebook.scale_similarity.clone();
        radio_button_similar_hash_size_8.connect_clicked(move |_| {
            scale_similarity.set_range(0_f64, SIMILAR_VALUES[1][5] as f64);
            scale_similarity.set_fill_level(SIMILAR_VALUES[1][5] as f64);
        });
    }
    {
        let radio_button_similar_hash_size_16 = gui_data.main_notebook.radio_button_similar_hash_size_16.clone();
        let scale_similarity = gui_data.main_notebook.scale_similarity.clone();
        radio_button_similar_hash_size_16.connect_clicked(move |_| {
            scale_similarity.set_range(0_f64, SIMILAR_VALUES[2][5] as f64);
            scale_similarity.set_fill_level(SIMILAR_VALUES[2][5] as f64);
        });
    }
}
