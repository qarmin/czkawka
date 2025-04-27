// use czkawka_core::common::get_all_available_threads;
// use slint::{ComponentHandle, VecModel};
//
// use crate::settings::StringComboBoxItems;
// use crate::{GuiState, MainWindow, Settings};
//
// // Some info needs to be send to gui at the start like available thread number in OS.
// pub fn set_initial_gui_infos(app: &MainWindow) {
//     let threads = get_all_available_threads();
//     let settings = app.global::<Settings>();
//     app.global::<GuiState>().set_maximum_threads(threads as f32);
//
//     let collected_items = StringComboBoxItems::get_items();
//
//     settings.set_languages_list(VecModel::from_slice(&StringComboBoxItems::get_display_names(&collected_items.languages)));
//     settings.set_similar_images_sub_available_hash_size(VecModel::from_slice(&StringComboBoxItems::get_display_names(&collected_items.hash_size)));
//     settings.set_similar_images_sub_available_resize_algorithm(VecModel::from_slice(&StringComboBoxItems::get_display_names(&collected_items.resize_algorithm)));
//     settings.set_similar_images_sub_available_hash_type(VecModel::from_slice(&StringComboBoxItems::get_display_names(&collected_items.image_hash_alg)));
//     settings.set_biggest_files_sub_method(VecModel::from_slice(&StringComboBoxItems::get_display_names(&collected_items.biggest_files_method)));
//     settings.set_duplicates_sub_check_method(VecModel::from_slice(&StringComboBoxItems::get_display_names(&collected_items.duplicates_check_method)));
//     settings.set_duplicates_sub_available_hash_type(VecModel::from_slice(&StringComboBoxItems::get_display_names(&collected_items.duplicates_hash_type)));
// }
