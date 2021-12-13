use gtk::prelude::*;

use czkawka_core::similar_images::{get_string_from_similarity, Similarity, SIMILAR_VALUES};

use crate::gui_data::GuiData;
use crate::help_combo_box::IMAGES_HASH_SIZE_COMBO_BOX;

pub fn connect_similar_image_size_change(gui_data: &GuiData) {
    let label_similar_images_minimal_similarity = gui_data.main_notebook.label_similar_images_minimal_similarity.clone();
    label_similar_images_minimal_similarity.set_text(&get_string_from_similarity(&Similarity::Similar(SIMILAR_VALUES[0][5]), 8));

    let combo_box_image_hash_size = gui_data.main_notebook.combo_box_image_hash_size.clone();
    let label_similar_images_minimal_similarity = gui_data.main_notebook.label_similar_images_minimal_similarity.clone();
    let scale_similarity_similar_images = gui_data.main_notebook.scale_similarity_similar_images.clone();
    combo_box_image_hash_size.connect_changed(move |combo_box_image_hash_size| {
        let hash_size_index = combo_box_image_hash_size.active().unwrap() as usize;
        let hash_size = IMAGES_HASH_SIZE_COMBO_BOX[hash_size_index];
        match hash_size {
            8 => {
                scale_similarity_similar_images.set_range(0_f64, SIMILAR_VALUES[0][5] as f64);
                scale_similarity_similar_images.set_fill_level(SIMILAR_VALUES[0][5] as f64);
                label_similar_images_minimal_similarity.set_text(&get_string_from_similarity(&Similarity::Similar(SIMILAR_VALUES[0][5]), 8));
            }
            16 => {
                scale_similarity_similar_images.set_range(0_f64, SIMILAR_VALUES[1][5] as f64);
                scale_similarity_similar_images.set_fill_level(SIMILAR_VALUES[1][5] as f64);
                label_similar_images_minimal_similarity.set_text(&get_string_from_similarity(&Similarity::Similar(SIMILAR_VALUES[1][5]), 16));
            }
            32 => {
                scale_similarity_similar_images.set_range(0_f64, SIMILAR_VALUES[2][5] as f64);
                scale_similarity_similar_images.set_fill_level(SIMILAR_VALUES[2][5] as f64);
                label_similar_images_minimal_similarity.set_text(&get_string_from_similarity(&Similarity::Similar(SIMILAR_VALUES[2][5]), 32));
            }
            64 => {
                scale_similarity_similar_images.set_range(0_f64, SIMILAR_VALUES[3][5] as f64);
                scale_similarity_similar_images.set_fill_level(SIMILAR_VALUES[3][5] as f64);
                label_similar_images_minimal_similarity.set_text(&get_string_from_similarity(&Similarity::Similar(SIMILAR_VALUES[3][5]), 64));
            }
            _ => panic!(),
        }
    });
}
