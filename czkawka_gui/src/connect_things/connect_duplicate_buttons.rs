use czkawka_core::common_dir_traversal::CheckingMethod;
use gtk4::prelude::*;

use crate::gui_structs::gui_data::GuiData;
use crate::help_combo_box::DUPLICATES_CHECK_METHOD_COMBO_BOX;

pub fn connect_duplicate_combo_box(gui_data: &GuiData) {
    let combo_box_duplicate_check_method = gui_data.main_notebook.combo_box_duplicate_check_method.clone();
    let combo_box_duplicate_hash_type = gui_data.main_notebook.combo_box_duplicate_hash_type.clone();
    let label_duplicate_hash_type = gui_data.main_notebook.label_duplicate_hash_type.clone();
    let check_button_duplicate_case_sensitive_name = gui_data.main_notebook.check_button_duplicate_case_sensitive_name.clone();
    combo_box_duplicate_check_method.connect_changed(move |combo_box_duplicate_check_method| {
        // None active can be if when adding elements(this signal is activated when e.g. adding new fields or removing them)
        if let Some(chosen_index) = combo_box_duplicate_check_method.active() {
            if DUPLICATES_CHECK_METHOD_COMBO_BOX[chosen_index as usize].check_method == CheckingMethod::Hash {
                combo_box_duplicate_hash_type.set_visible(true);
                label_duplicate_hash_type.set_visible(true);
            } else {
                combo_box_duplicate_hash_type.set_visible(false);
                label_duplicate_hash_type.set_visible(false);
            }

            if [CheckingMethod::Name, CheckingMethod::SizeName].contains(&DUPLICATES_CHECK_METHOD_COMBO_BOX[chosen_index as usize].check_method) {
                check_button_duplicate_case_sensitive_name.set_visible(true);
            } else {
                check_button_duplicate_case_sensitive_name.set_visible(false);
            }
        }
    });
}
