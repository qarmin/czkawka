use czkawka_core::common::get_all_available_threads;
use slint::{ComponentHandle, VecModel};

use crate::settings::combo_box::StringComboBoxItems;
use crate::{GuiState, MainWindow, Settings};

pub(crate) fn set_initial_gui_infos(app: &MainWindow) {
    let threads = get_all_available_threads();
    app.global::<GuiState>().set_maximum_threads(threads as f32);

    apply_combo_box_translations(app);
}

pub(crate) fn apply_combo_box_translations(app: &MainWindow) {
    let settings = app.global::<Settings>();

    let collected_items = StringComboBoxItems::get_items();
    let StringComboBoxItems {
        languages,
        hash_size,
        resize_algorithm,
        image_geometric_invariance,
        image_hash_alg,
        duplicates_hash_type,
        biggest_files_method,
        audio_check_type,
        duplicates_check_method,
        video_optimizer_crop_type,
        video_optimizer_mode,
        video_optimizer_video_codec,
        video_optimizer_noise_reduction,
        similar_videos_visual_preset,
        similar_videos_audio_preset,
    } = &*collected_items;

    settings.set_languages_list(VecModel::from_slice(&StringComboBoxItems::get_display_names(languages)));
    settings.set_similar_images_sub_available_hash_size(VecModel::from_slice(&StringComboBoxItems::get_display_names(hash_size)));
    settings.set_similar_images_sub_available_resize_algorithm(VecModel::from_slice(&StringComboBoxItems::get_display_names(resize_algorithm)));
    settings.set_similar_images_sub_available_hash_type(VecModel::from_slice(&StringComboBoxItems::get_display_names(image_hash_alg)));
    settings.set_similar_images_sub_available_geometric_invariance(VecModel::from_slice(&StringComboBoxItems::get_display_names(image_geometric_invariance)));
    settings.set_biggest_files_sub_method(VecModel::from_slice(&StringComboBoxItems::get_display_names(biggest_files_method)));
    settings.set_duplicates_sub_check_method(VecModel::from_slice(&StringComboBoxItems::get_display_names(duplicates_check_method)));
    settings.set_duplicates_sub_available_hash_type(VecModel::from_slice(&StringComboBoxItems::get_display_names(duplicates_hash_type)));
    settings.set_similar_music_sub_audio_check_type(VecModel::from_slice(&StringComboBoxItems::get_display_names(audio_check_type)));
    settings.set_video_optimizer_sub_crop_type(VecModel::from_slice(&StringComboBoxItems::get_display_names(video_optimizer_crop_type)));
    settings.set_video_optimizer_sub_mode(VecModel::from_slice(&StringComboBoxItems::get_display_names(video_optimizer_mode)));
    settings.set_video_optimizer_sub_video_codec_config(VecModel::from_slice(&StringComboBoxItems::get_display_names(video_optimizer_video_codec)));
    settings.set_video_optimizer_sub_noise_reduction(VecModel::from_slice(&StringComboBoxItems::get_display_names(video_optimizer_noise_reduction)));
    settings.set_similar_videos_visual_preset_names(VecModel::from_slice(&StringComboBoxItems::get_display_names(similar_videos_visual_preset)));
    settings.set_similar_videos_audio_preset_names(VecModel::from_slice(&StringComboBoxItems::get_display_names(similar_videos_audio_preset)));
}
