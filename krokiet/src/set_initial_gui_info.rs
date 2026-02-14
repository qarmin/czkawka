use czkawka_core::common::get_all_available_threads;
use slint::{ComponentHandle, Model, SharedString};

use crate::settings::combo_box::StringComboBoxItems;
use crate::{GuiState, MainWindow, Settings};

pub(crate) fn set_initial_gui_infos(app: &MainWindow) {
    let threads = get_all_available_threads();
    let settings = app.global::<Settings>();
    app.global::<GuiState>().set_maximum_threads(threads as f32);

    let collected_items = StringComboBoxItems::get_items();
    let StringComboBoxItems {
        languages,
        hash_size,
        resize_algorithm,
        image_hash_alg,
        duplicates_hash_type,
        biggest_files_method,
        audio_check_type,
        duplicates_check_method,
        videos_crop_detect,
        video_optimizer_crop_type,
        video_optimizer_mode,
        video_optimizer_video_codec,
    } = &*collected_items;

    let languages_display_names = StringComboBoxItems::get_display_names(languages);
    let hash_size_display_names = StringComboBoxItems::get_display_names(hash_size);
    let resize_algorithm_display_names = StringComboBoxItems::get_display_names(resize_algorithm);
    let image_hash_alg_display_names = StringComboBoxItems::get_display_names(image_hash_alg);
    let duplicates_hash_type_display_names = StringComboBoxItems::get_display_names(duplicates_hash_type);
    let biggest_files_method_display_names = StringComboBoxItems::get_display_names(biggest_files_method);
    let audio_check_type_display_names = StringComboBoxItems::get_display_names(audio_check_type);
    let duplicates_check_method_display_names = StringComboBoxItems::get_display_names(duplicates_check_method);
    let videos_crop_detect_display_names = StringComboBoxItems::get_display_names(videos_crop_detect);
    let video_optimizer_crop_type_display_names = StringComboBoxItems::get_display_names(video_optimizer_crop_type);
    let video_optimizer_mode_display_names = StringComboBoxItems::get_display_names(video_optimizer_mode);
    let video_optimizer_video_codec_display_names = StringComboBoxItems::get_display_names(video_optimizer_video_codec);

    // Currently this is not possible due to slint bug - after 11.0 version I will try to fight with this - https://github.com/slint-ui/slint/issues/7632
    // For now I just assert that names will be in sync with slint files

    // settings.set_languages_list(VecModel::from_slice(&languages_display_names));
    // settings.set_similar_images_sub_available_hash_size(VecModel::from_slice(&hash_size_display_names));
    // settings.set_similar_images_sub_available_resize_algorithm(VecModel::from_slice(&resize_algorithm_display_names));
    // settings.set_similar_images_sub_available_hash_type(VecModel::from_slice(&image_hash_alg_display_names));
    // settings.set_biggest_files_sub_method(VecModel::from_slice(&biggest_files_method_display_names));
    // settings.set_duplicates_sub_check_method(VecModel::from_slice(&duplicates_check_method_display_names));
    // settings.set_duplicates_sub_available_hash_type(VecModel::from_slice(&duplicates_hash_type_display_names));
    // settings.set_similar_music_sub_audio_check_type(VecModel::from_slice(&audio_check_type_display_names));
    // settings.set_similar_videos_crop_detect(VecModel::from_slice(&videos_crop_detect_display_names));
    // settings.set_video_optimizer_sub_crop_type(VecModel::from_slice(&video_optimizer_crop_type_display_names));
    // settings.set_video_optimizer_sub_mode(VecModel::from_slice(&video_optimizer_mode_display_names));
    // settings.set_video_optimizer_sub_video_codec_config(VecModel::from_slice(&video_optimizer_video_codec_display_names));

    assert_eq!(settings.get_languages_list().iter().collect::<Vec<SharedString>>(), languages_display_names);
    assert_eq!(
        settings.get_similar_images_sub_available_hash_size().iter().collect::<Vec<SharedString>>(),
        hash_size_display_names
    );
    assert_eq!(
        settings.get_similar_images_sub_available_resize_algorithm().iter().collect::<Vec<SharedString>>(),
        resize_algorithm_display_names
    );
    assert_eq!(
        settings.get_similar_images_sub_available_hash_type().iter().collect::<Vec<SharedString>>(),
        image_hash_alg_display_names
    );
    assert_eq!(
        settings.get_biggest_files_sub_method().iter().collect::<Vec<SharedString>>(),
        biggest_files_method_display_names
    );
    assert_eq!(
        settings.get_duplicates_sub_check_method().iter().collect::<Vec<SharedString>>(),
        duplicates_check_method_display_names
    );
    assert_eq!(
        settings.get_duplicates_sub_available_hash_type().iter().collect::<Vec<SharedString>>(),
        duplicates_hash_type_display_names
    );
    assert_eq!(
        settings.get_similar_music_sub_audio_check_type().iter().collect::<Vec<SharedString>>(),
        audio_check_type_display_names
    );
    assert_eq!(
        settings.get_similar_videos_crop_detect().iter().collect::<Vec<SharedString>>(),
        videos_crop_detect_display_names
    );
    assert_eq!(
        settings.get_video_optimizer_sub_crop_type().iter().collect::<Vec<SharedString>>(),
        video_optimizer_crop_type_display_names
    );
    assert_eq!(
        settings.get_video_optimizer_sub_mode().iter().collect::<Vec<SharedString>>(),
        video_optimizer_mode_display_names
    );
    assert_eq!(
        settings.get_video_optimizer_sub_video_codec_config().iter().collect::<Vec<SharedString>>(),
        video_optimizer_video_codec_display_names
    );
}
