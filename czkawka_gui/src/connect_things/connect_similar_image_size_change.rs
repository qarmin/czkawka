use gtk4::prelude::*;

use czkawka_core::similar_images::{get_string_from_similarity, SIMILAR_VALUES};

use crate::gui_structs::gui_data::GuiData;
use crate::help_combo_box::IMAGES_HASH_SIZE_COMBO_BOX;

pub fn connect_similar_image_size_change(gui_data: &GuiData) {
    let label_similar_images_minimal_similarity = gui_data.main_notebook.label_similar_images_minimal_similarity.clone();
    label_similar_images_minimal_similarity.set_text(&get_string_from_similarity(&SIMILAR_VALUES[0][5], 8));

    let combo_box_image_hash_size = gui_data.main_notebook.combo_box_image_hash_size.clone();
    let label_similar_images_minimal_similarity = gui_data.main_notebook.label_similar_images_minimal_similarity.clone();
    let scale_similarity_similar_images = gui_data.main_notebook.scale_similarity_similar_images.clone();
    combo_box_image_hash_size.connect_changed(move |combo_box_image_hash_size| {
        let hash_size_index = combo_box_image_hash_size.active().expect("Failed to get active item") as usize;
        let hash_size = IMAGES_HASH_SIZE_COMBO_BOX[hash_size_index];

        let index = match hash_size {
            8 => 0,
            16 => 1,
            32 => 2,
            64 => 3,
            _ => panic!(),
        };

        scale_similarity_similar_images.set_range(0_f64, SIMILAR_VALUES[index][5] as f64);
        scale_similarity_similar_images.set_fill_level(SIMILAR_VALUES[index][5] as f64);
        label_similar_images_minimal_similarity.set_text(&get_string_from_similarity(&SIMILAR_VALUES[index][5], hash_size as u8));
    });
}
