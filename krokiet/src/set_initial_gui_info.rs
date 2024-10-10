use czkawka_core::common::get_all_available_threads;
use slint::{ComponentHandle, SharedString, VecModel};

use crate::settings::{
    ALLOWED_BIG_FILE_SIZE_VALUES, ALLOWED_DUPLICATES_CHECK_METHOD_VALUES, ALLOWED_DUPLICATES_HASH_TYPE_VALUES, ALLOWED_HASH_SIZE_VALUES, ALLOWED_IMAGE_HASH_ALG_VALUES,
    ALLOWED_RESIZE_ALGORITHM_VALUES,
};
use crate::{GuiState, MainWindow, Settings};

// Some info needs to be send to gui at the start like available thread number in OS.
pub fn set_initial_gui_infos(app: &MainWindow) {
    let threads = get_all_available_threads();
    let settings = app.global::<Settings>();
    app.global::<GuiState>().set_maximum_threads(threads as f32);

    let available_hash_size: Vec<SharedString> = ALLOWED_HASH_SIZE_VALUES
        .iter()
        .map(|(hash_size, _max_similarity)| hash_size.to_string().into())
        .collect::<Vec<_>>();
    let available_resize_algorithm: Vec<SharedString> = ALLOWED_RESIZE_ALGORITHM_VALUES
        .iter()
        .map(|(_settings_key, gui_name, _filter_type)| (*gui_name).into())
        .collect::<Vec<_>>();
    let available_hash_type: Vec<SharedString> = ALLOWED_IMAGE_HASH_ALG_VALUES
        .iter()
        .map(|(_settings_key, gui_name, _hash_type)| (*gui_name).into())
        .collect::<Vec<_>>();
    let available_big_file_search_mode: Vec<SharedString> = ALLOWED_BIG_FILE_SIZE_VALUES
        .iter()
        .map(|(_settings_key, gui_name, _search_mode)| (*gui_name).into())
        .collect::<Vec<_>>();
    let available_duplicates_check_method: Vec<SharedString> = ALLOWED_DUPLICATES_CHECK_METHOD_VALUES
        .iter()
        .map(|(_settings_key, gui_name, _checking_method)| (*gui_name).into())
        .collect::<Vec<_>>();
    let available_duplicates_hash_type: Vec<SharedString> = ALLOWED_DUPLICATES_HASH_TYPE_VALUES
        .iter()
        .map(|(_settings_key, gui_name, _hash_type)| (*gui_name).into())
        .collect::<Vec<_>>();

    settings.set_similar_images_sub_available_hash_size(VecModel::from_slice(&available_hash_size));
    settings.set_similar_images_sub_available_resize_algorithm(VecModel::from_slice(&available_resize_algorithm));
    settings.set_similar_images_sub_available_hash_type(VecModel::from_slice(&available_hash_type));
    settings.set_biggest_files_sub_method(VecModel::from_slice(&available_big_file_search_mode));
    settings.set_duplicates_sub_check_method(VecModel::from_slice(&available_duplicates_check_method));
    settings.set_duplicates_sub_available_hash_type(VecModel::from_slice(&available_duplicates_hash_type));
}
