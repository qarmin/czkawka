use gtk::prelude::*;

use crate::gui_data::GuiData;

pub fn connect_duplicate_buttons(gui_data: &GuiData) {
    let radio_button_duplicates_hash = gui_data.main_notebook.radio_button_duplicates_hash.clone();
    let radio_button_hash_type_blake3 = gui_data.main_notebook.radio_button_hash_type_blake3.clone();
    let radio_button_hash_type_xxh3 = gui_data.main_notebook.radio_button_hash_type_xxh3.clone();
    let radio_button_hash_type_crc32 = gui_data.main_notebook.radio_button_hash_type_crc32.clone();
    radio_button_duplicates_hash.connect_toggled(move |radio_button_duplicates_hash| {
        if radio_button_duplicates_hash.is_active() {
            radio_button_hash_type_blake3.set_sensitive(true);
            radio_button_hash_type_xxh3.set_sensitive(true);
            radio_button_hash_type_crc32.set_sensitive(true);
        } else {
            radio_button_hash_type_blake3.set_sensitive(false);
            radio_button_hash_type_xxh3.set_sensitive(false);
            radio_button_hash_type_crc32.set_sensitive(false);
        }
    });
    let radio_button_duplicates_hash = gui_data.main_notebook.radio_button_duplicates_hash.clone();
    let radio_button_duplicates_name = gui_data.main_notebook.radio_button_duplicates_name.clone();
    let radio_button_hash_type_blake3 = gui_data.main_notebook.radio_button_hash_type_blake3.clone();
    let radio_button_hash_type_xxh3 = gui_data.main_notebook.radio_button_hash_type_xxh3.clone();
    let radio_button_hash_type_crc32 = gui_data.main_notebook.radio_button_hash_type_crc32.clone();
    radio_button_duplicates_name.connect_toggled(move |_| {
        if radio_button_duplicates_hash.is_active() {
            radio_button_hash_type_blake3.set_sensitive(true);
            radio_button_hash_type_xxh3.set_sensitive(true);
            radio_button_hash_type_crc32.set_sensitive(true);
        } else {
            radio_button_hash_type_blake3.set_sensitive(false);
            radio_button_hash_type_xxh3.set_sensitive(false);
            radio_button_hash_type_crc32.set_sensitive(false);
        }
    });
    let radio_button_duplicates_hash = gui_data.main_notebook.radio_button_duplicates_hash.clone();
    let radio_button_duplicates_size = gui_data.main_notebook.radio_button_duplicates_size.clone();
    let radio_button_hash_type_blake3 = gui_data.main_notebook.radio_button_hash_type_blake3.clone();
    let radio_button_hash_type_xxh3 = gui_data.main_notebook.radio_button_hash_type_xxh3.clone();
    let radio_button_hash_type_crc32 = gui_data.main_notebook.radio_button_hash_type_crc32.clone();
    radio_button_duplicates_size.connect_toggled(move |_| {
        if radio_button_duplicates_hash.is_active() {
            radio_button_hash_type_blake3.set_sensitive(true);
            radio_button_hash_type_xxh3.set_sensitive(true);
            radio_button_hash_type_crc32.set_sensitive(true);
        } else {
            radio_button_hash_type_blake3.set_sensitive(false);
            radio_button_hash_type_xxh3.set_sensitive(false);
            radio_button_hash_type_crc32.set_sensitive(false);
        }
    });
}
