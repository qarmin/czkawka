use gtk::prelude::*;

use czkawka_core::duplicate::CheckingMethod;

use crate::gui_data::GuiData;
use crate::help_combo_box::DUPLICATES_CHECK_METHOD_COMBO_BOX;

pub fn connect_duplicate_combo_box(gui_data: &GuiData) {
    let combo_box_duplicate_check_method = gui_data.main_notebook.combo_box_duplicate_check_method.clone();
    let combo_box_duplicate_hash_type = gui_data.main_notebook.combo_box_duplicate_hash_type.clone();
    combo_box_duplicate_check_method.connect_changed(move |combo_box_duplicate_check_method| {
        let chosen_index = combo_box_duplicate_check_method.active().unwrap() as usize;
        if DUPLICATES_CHECK_METHOD_COMBO_BOX[chosen_index].check_method == CheckingMethod::Hash {
            combo_box_duplicate_hash_type.set_sensitive(true);
        } else {
            combo_box_duplicate_hash_type.set_sensitive(false);
        }
    });
}
