use czkawka_core::TOOLS_NUMBER;
use i18n_embed::DesktopLanguageRequester;
use i18n_embed::unic_langid::LanguageIdentifier;
use log::{error, info};
use slint::{ComponentHandle, ModelRc, SharedString, VecModel};

use crate::{ActiveTab, Callabler, GuiState, MainWindow, SelectMode, Settings, SortMode, SortModel, Translations, flk, localizer_krokiet};

pub struct Language {
    pub long_name: &'static str,
    pub short_name: &'static str,
    pub left_panel_size: f32, // Currently don't know how to automatically calculate this, so each language has its own size
}

// When changing, do not forget to update languages_list inside slint file
pub const LANGUAGE_LIST: &[Language] = &[
    Language {
        long_name: "English",
        short_name: "en",
        left_panel_size: 130.0,
    },
    Language {
        long_name: "Polski (Polish)",
        short_name: "pl",
        left_panel_size: 160.0,
    },
    Language {
        long_name: "Français (French)",
        short_name: "fr",
        left_panel_size: 190.0,
    },
    Language {
        long_name: "Italiano (Italian)",
        short_name: "it",
        left_panel_size: 155.0,
    },
    Language {
        long_name: "Русский (Russian)",
        short_name: "ru",
        left_panel_size: 195.0,
    },
    Language {
        long_name: "український (Ukrainian)",
        short_name: "uk",
        left_panel_size: 195.0,
    },
    Language {
        long_name: "한국어 (Korean)",
        short_name: "ko",
        left_panel_size: 155.0,
    },
    Language {
        long_name: "Česky (Czech)",
        short_name: "cs",
        left_panel_size: 180.0,
    },
    Language {
        long_name: "Deutsch (German)",
        short_name: "de",
        left_panel_size: 165.0,
    },
    Language {
        long_name: "日本語 (Japanese)",
        short_name: "ja",
        left_panel_size: 165.0,
    },
    Language {
        long_name: "Português (Portuguese)",
        short_name: "pt-PT",
        left_panel_size: 175.0,
    },
    Language {
        long_name: "Português Brasileiro (Brazilian Portuguese)",
        short_name: "pt-BR",
        left_panel_size: 205.0,
    },
    Language {
        long_name: "简体中文 (Simplified Chinese)",
        short_name: "zh-CN",
        left_panel_size: 125.0,
    },
    Language {
        long_name: "繁體中文 (Traditional Chinese)",
        short_name: "zh-TW",
        left_panel_size: 145.0,
    },
    Language {
        long_name: "Español (Spanish)",
        short_name: "es-ES",
        left_panel_size: 175.0,
    },
    Language {
        long_name: "Norsk (Norwegian)",
        short_name: "no",
        left_panel_size: 145.0,
    },
    Language {
        long_name: "Svenska (Swedish)",
        short_name: "sv-SE",
        left_panel_size: 140.0,
    },
    Language {
        long_name: "العربية (Arabic)",
        short_name: "ar",
        left_panel_size: 145.0,
    },
    Language {
        long_name: "Български (Bulgarian)",
        short_name: "bg",
        left_panel_size: 205.0,
    },
    Language {
        long_name: "Ελληνικά (Greek)",
        short_name: "el",
        left_panel_size: 170.0,
    },
    Language {
        long_name: "Nederlands (Dutch)",
        short_name: "nl",
        left_panel_size: 175.0,
    },
    Language {
        long_name: "Română (Romanian)",
        short_name: "ro",
        left_panel_size: 150.0,
    },
    Language {
        long_name: "Türkçe (Turkish)",
        short_name: "tr",
        left_panel_size: 160.0,
    },
];

pub(crate) fn connect_translations(app: &MainWindow) {
    change_language(app);

    let a = app.as_weak();
    app.global::<Callabler>().on_changed_language(move || {
        let app = a.upgrade().unwrap();
        change_language(&app);
    });
}

pub fn find_the_closest_language_idx_to_system() -> usize {
    let requested_languages = DesktopLanguageRequester::requested_languages();

    if let Some(language) = requested_languages.first() {
        let strip_function = |s: &str| s.chars().take_while(|c| c.is_ascii_alphabetic()).collect::<String>();

        let system_language = strip_function(&language.to_string());
        info!("System language: {system_language}");
        for (idx, lang) in LANGUAGE_LIST.iter().enumerate() {
            let lang_short = strip_function(lang.short_name);
            info!("Language: {}", lang.short_name);
            if system_language == lang_short {
                return idx;
            }
        }
    }
    0
}

pub(crate) fn change_language(app: &MainWindow) {
    let localizers = vec![
        ("czkawka_core", czkawka_core::localizer_core::localizer_core()),
        ("krokiet", localizer_krokiet::localizer_krokiet()),
    ];

    let lang = app.global::<Settings>().get_language_index();
    let lang_items = &LANGUAGE_LIST[lang as usize];

    let lang_identifier = vec![LanguageIdentifier::from_bytes(lang_items.short_name.as_bytes()).expect("Failed to create LanguageIdentifier")];
    for (lib, localizer) in localizers {
        if let Err(error) = localizer.select(&lang_identifier) {
            error!("Error while loading languages for {lib} {error:?}");
        }
    }

    app.global::<GuiState>().set_left_panel_width(lang_items.left_panel_size);
    translate_items(app);
}

// To generate this, check misc folder
// This is ugly workaround for missing fluent language support in slint
fn translate_items(app: &MainWindow) {
    let translation = app.global::<Translations>();
    let settings = app.global::<Settings>();

    translation.set_ok_button_text(flk!("ok_button").into());
    translation.set_cancel_button_text(flk!("cancel_button").into());
    translation.set_do_you_want_to_continue_text(flk!("do_you_want_to_continue").into());
    translation.set_main_window_title_text(flk!("main_window_title").into());
    translation.set_scan_button_text(flk!("scan_button").into());
    translation.set_stop_button_text(flk!("stop_button").into());
    translation.set_select_button_text(flk!("select_button").into());
    translation.set_move_button_text(flk!("move_button").into());
    translation.set_delete_button_text(flk!("delete_button").into());
    translation.set_save_button_text(flk!("save_button").into());
    translation.set_sort_button_text(flk!("sort_button").into());
    translation.set_rename_button_text(flk!("rename_button").into());
    translation.set_motto_text(flk!("motto").into());
    translation.set_unicorn_text(flk!("unicorn").into());
    translation.set_repository_text(flk!("repository").into());
    translation.set_instruction_text(flk!("instruction").into());
    translation.set_donation_text(flk!("donation").into());
    translation.set_translation_text(flk!("translation").into());
    translation.set_included_paths_text(flk!("included_paths").into());
    translation.set_excluded_paths_text(flk!("excluded_paths").into());
    translation.set_ref_text(flk!("ref").into());
    translation.set_path_text(flk!("path").into());
    translation.set_tool_duplicate_files_text(flk!("tool_duplicate_files").into());
    translation.set_tool_empty_folders_text(flk!("tool_empty_folders").into());
    translation.set_tool_big_files_text(flk!("tool_big_files").into());
    translation.set_tool_empty_files_text(flk!("tool_empty_files").into());
    translation.set_tool_temporary_files_text(flk!("tool_temporary_files").into());
    translation.set_tool_similar_images_text(flk!("tool_similar_images").into());
    translation.set_tool_similar_videos_text(flk!("tool_similar_videos").into());
    translation.set_tool_music_duplicates_text(flk!("tool_music_duplicates").into());
    translation.set_tool_invalid_symlinks_text(flk!("tool_invalid_symlinks").into());
    translation.set_tool_broken_files_text(flk!("tool_broken_files").into());
    translation.set_tool_bad_extensions_text(flk!("tool_bad_extensions").into());
    translation.set_tool_exif_remover_text(flk!("tool_exif_remover").into());
    translation.set_tool_video_optimizer_text(flk!("tool_video_optimizer").into());
    translation.set_tool_bad_names_text(flk!("tool_bad_names").into());
    translation.set_sort_by_full_name_text(flk!("sort_by_full_name").into());
    translation.set_sort_by_selection_text(flk!("sort_by_selection").into());
    translation.set_sort_reverse_text(flk!("sort_reverse").into());
    translation.set_settings_dark_theme_text(flk!("settings_dark_theme").into());
    translation.set_settings_show_only_icons_text(flk!("settings_show_only_icons").into());
    translation.set_settings_global_settings_text(flk!("settings_global_settings").into());
    translation.set_selection_all_text(flk!("selection_all").into());
    translation.set_selection_deselect_all_text(flk!("selection_deselect_all").into());
    translation.set_stage_current_text(flk!("stage_current").into());
    translation.set_stage_all_text(flk!("stage_all").into());
    translation.set_subsettings_text(flk!("subsettings").into());
    translation.set_subsettings_images_hash_size_text(flk!("subsettings_images_hash_size").into());
    translation.set_subsettings_images_resize_algorithm_text(flk!("subsettings_images_resize_algorithm").into());
    translation.set_subsettings_images_ignore_same_size_text(flk!("subsettings_images_ignore_same_size").into());
    translation.set_subsettings_images_max_difference_text(flk!("subsettings_images_max_difference").into());
    translation.set_subsettings_images_duplicates_hash_type_text(flk!("subsettings_images_duplicates_hash_type").into());
    translation.set_subsettings_duplicates_check_method_text(flk!("subsettings_duplicates_check_method").into());
    translation.set_subsettings_duplicates_name_case_sensitive_text(flk!("subsettings_duplicates_name_case_sensitive").into());
    translation.set_subsettings_biggest_files_sub_method_text(flk!("subsettings_biggest_files_sub_method").into());
    translation.set_subsettings_biggest_files_sub_number_of_files_text(flk!("subsettings_biggest_files_sub_number_of_files").into());
    translation.set_subsettings_videos_max_difference_text(flk!("subsettings_videos_max_difference").into());
    translation.set_subsettings_videos_ignore_same_size_text(flk!("subsettings_videos_ignore_same_size").into());
    translation.set_subsettings_music_audio_check_type_text(flk!("subsettings_music_audio_check_type").into());
    translation.set_subsettings_music_approximate_comparison_text(flk!("subsettings_music_approximate_comparison").into());
    translation.set_subsettings_music_compared_tags_text(flk!("subsettings_music_compared_tags").into());
    translation.set_subsettings_music_title_text(flk!("subsettings_music_title").into());
    translation.set_subsettings_music_artist_text(flk!("subsettings_music_artist").into());
    translation.set_subsettings_music_bitrate_text(flk!("subsettings_music_bitrate").into());
    translation.set_subsettings_music_genre_text(flk!("subsettings_music_genre").into());
    translation.set_subsettings_music_year_text(flk!("subsettings_music_year").into());
    translation.set_subsettings_music_length_text(flk!("subsettings_music_length").into());
    translation.set_subsettings_music_max_difference_text(flk!("subsettings_music_max_difference").into());
    translation.set_subsettings_music_minimal_fragment_duration_text(flk!("subsettings_music_minimal_fragment_duration").into());
    translation.set_subsettings_music_compare_fingerprints_only_with_similar_titles_text(flk!("subsettings_music_compare_fingerprints_only_with_similar_titles").into());
    translation.set_subsettings_broken_files_type_text(flk!("subsettings_broken_files_type").into());
    translation.set_subsettings_broken_files_audio_text(flk!("subsettings_broken_files_audio").into());
    translation.set_subsettings_broken_files_video_text(flk!("subsettings_broken_files_video").into());
    translation.set_subsettings_broken_files_pdf_text(flk!("subsettings_broken_files_pdf").into());
    translation.set_subsettings_broken_files_archive_text(flk!("subsettings_broken_files_archive").into());
    translation.set_subsettings_broken_files_image_text(flk!("subsettings_broken_files_image").into());
    translation.set_subsettings_broken_files_video_info_text(flk!("subsettings_broken_files_video_info").into());
    translation.set_subsettings_bad_names_issues_text(flk!("subsettings_bad_names_issues").into());
    translation.set_subsettings_bad_names_uppercase_extension_text(flk!("subsettings_bad_names_uppercase_extension").into());
    translation.set_subsettings_bad_names_uppercase_extension_hint_text(flk!("subsettings_bad_names_uppercase_extension_hint").into());
    translation.set_subsettings_bad_names_emoji_used_text(flk!("subsettings_bad_names_emoji_used").into());
    translation.set_subsettings_bad_names_emoji_used_hint_text(flk!("subsettings_bad_names_emoji_used_hint").into());
    translation.set_subsettings_bad_names_space_at_start_end_text(flk!("subsettings_bad_names_space_at_start_end").into());
    translation.set_subsettings_bad_names_space_at_start_end_hint_text(flk!("subsettings_bad_names_space_at_start_end_hint").into());
    translation.set_subsettings_bad_names_non_ascii_text(flk!("subsettings_bad_names_non_ascii").into());
    translation.set_subsettings_bad_names_non_ascii_hint_text(flk!("subsettings_bad_names_non_ascii_hint").into());
    translation.set_subsettings_bad_names_restricted_charset_text(flk!("subsettings_bad_names_restricted_charset").into());
    translation.set_subsettings_bad_names_restricted_charset_hint_text(flk!("subsettings_bad_names_restricted_charset_hint").into());
    translation.set_subsettings_bad_names_allowed_chars_text(flk!("subsettings_bad_names_allowed_chars").into());
    translation.set_subsettings_bad_names_remove_duplicated_text(flk!("subsettings_bad_names_remove_duplicated").into());
    translation.set_subsettings_bad_names_remove_duplicated_hint_text(flk!("subsettings_bad_names_remove_duplicated_hint").into());
    translation.set_subsettings_video_optimizer_mode_text(flk!("subsettings_video_optimizer_mode").into());
    translation.set_subsettings_video_optimizer_crop_type_text(flk!("subsettings_video_optimizer_crop_type").into());
    translation.set_subsettings_video_optimizer_black_pixel_threshold_text(flk!("subsettings_video_optimizer_black_pixel_threshold").into());
    translation.set_subsettings_video_optimizer_black_pixel_threshold_hint_text(flk!("subsettings_video_optimizer_black_pixel_threshold_hint").into());
    translation.set_subsettings_video_optimizer_black_bar_min_percentage_text(flk!("subsettings_video_optimizer_black_bar_min_percentage").into());
    translation.set_subsettings_video_optimizer_black_bar_min_percentage_hint_text(flk!("subsettings_video_optimizer_black_bar_min_percentage_hint").into());
    translation.set_subsettings_video_optimizer_max_samples_text(flk!("subsettings_video_optimizer_max_samples").into());
    translation.set_subsettings_video_optimizer_max_samples_hint_text(flk!("subsettings_video_optimizer_max_samples_hint").into());
    translation.set_subsettings_video_optimizer_min_crop_size_text(flk!("subsettings_video_optimizer_min_crop_size").into());
    translation.set_subsettings_video_optimizer_min_crop_size_hint_text(flk!("subsettings_video_optimizer_min_crop_size_hint").into());
    translation.set_subsettings_video_optimizer_video_codec_text(flk!("subsettings_video_optimizer_video_codec").into());
    translation.set_subsettings_video_optimizer_excluded_codecs_text(flk!("subsettings_video_optimizer_excluded_codecs").into());
    translation.set_subsettings_video_optimizer_video_quality_text(flk!("subsettings_video_optimizer_video_quality").into());
    translation.set_subsettings_reset_text(flk!("subsettings_reset").into());
    translation.set_subsettings_exif_ignored_tags_text(flk!("subsettings_exif_ignored_tags_text").into());
    translation.set_subsettings_exif_ignored_tags_hint_text(flk!("subsettings_exif_ignored_tags_hint_text").into());
    translation.set_clean_button_text(flk!("clean_button_text").into());
    translation.set_clean_text(flk!("clean_text").into());
    translation.set_clean_confirmation_text(flk!("clean_confirmation_text").into());
    translation.set_crop_videos_text(flk!("crop_videos_text").into());
    translation.set_crop_video_confirmation_text(flk!("crop_video_confirmation_text").into());
    translation.set_crop_reencode_video_text(flk!("crop_reencode_video_text").into());
    translation.set_reencode_videos_text(flk!("reencode_videos_text").into());
    translation.set_optimize_button_text(flk!("optimize_button_text").into());
    translation.set_optimize_confirmation_text(flk!("optimize_confirmation_text").into());
    translation.set_optimize_fail_if_bigger_text(flk!("optimize_fail_if_bigger_text").into());
    translation.set_optimize_overwrite_files_text(flk!("optimize_overwrite_files_text").into());
    translation.set_optimize_limit_video_size_text(flk!("optimize_limit_video_size_text").into());
    translation.set_optimize_max_width_text(flk!("optimize_max_width_text").into());
    translation.set_optimize_max_height_text(flk!("optimize_max_height_text").into());
    translation.set_hardlink_button_text(flk!("hardlink_button_text").into());
    translation.set_hardlink_text(flk!("hardlink_text").into());
    translation.set_hardlink_confirmation_text(flk!("hardlink_confirmation_text").into());
    translation.set_softlink_button_text(flk!("softlink_button_text").into());
    translation.set_softlink_text(flk!("softlink_text").into());
    translation.set_softlink_confirmation_text(flk!("softlink_confirmation_text").into());
    translation.set_move_confirmation_text(flk!("move_confirmation_text").into());
    translation.set_rename_confirmation_text(flk!("rename_confirmation_text").into());
    translation.set_settings_excluded_items_text(flk!("settings_excluded_items").into());
    translation.set_settings_allowed_extensions_text(flk!("settings_allowed_extensions").into());
    translation.set_settings_excluded_extensions_text(flk!("settings_excluded_extensions").into());
    translation.set_settings_file_size_text(flk!("settings_file_size").into());
    translation.set_settings_minimum_file_size_text(flk!("settings_minimum_file_size").into());
    translation.set_settings_maximum_file_size_text(flk!("settings_maximum_file_size").into());
    translation.set_settings_recursive_search_text(flk!("settings_recursive_search").into());
    translation.set_settings_use_cache_text(flk!("settings_use_cache").into());
    translation.set_settings_save_as_json_text(flk!("settings_save_as_json").into());
    translation.set_settings_move_to_trash_text(flk!("settings_move_to_trash").into());
    translation.set_settings_ignore_other_filesystems_text(flk!("settings_ignore_other_filesystems").into());
    translation.set_settings_thread_number_text(flk!("settings_thread_number").into());
    translation.set_settings_restart_required_text(flk!("settings_restart_required").into());
    translation.set_settings_duplicate_image_preview_text(flk!("settings_duplicate_image_preview").into());
    translation.set_settings_similar_videos_preview_text(flk!("settings_video_thumbnails_preview").into());
    translation.set_settings_application_scale_text(flk!("settings_application_scale_text").into());
    translation.set_settings_application_scale_hint_text(flk!("settings_application_scale_hint_text").into());
    translation.set_settings_restart_required_scale_text(flk!("settings_restart_required_scale_text").into());
    translation.set_settings_use_manual_application_scale_text(flk!("settings_use_manual_application_scale_text").into());
    translation.set_settings_duplicate_minimal_hash_cache_size_text(flk!("settings_duplicate_minimal_hash_cache_size").into());
    translation.set_settings_duplicate_use_prehash_text(flk!("settings_duplicate_use_prehash").into());
    translation.set_settings_duplicate_minimal_prehash_cache_size_text(flk!("settings_duplicate_minimal_prehash_cache_size").into());
    translation.set_settings_delete_outdated_cache_entries_text(flk!("settings_delete_outdated_cache_entries").into());
    translation.set_settings_delete_outdated_cache_entries_hint_text(flk!("settings_delete_outdated_cache_entries_hint").into());
    translation.set_settings_hide_hard_links_text(flk!("settings_hide_hard_links").into());
    translation.set_settings_hide_hard_links_hint_text(flk!("settings_hide_hard_links_hint").into());
    translation.set_settings_similar_images_show_image_preview_text(flk!("settings_similar_images_show_image_preview").into());
    translation.set_settings_open_config_folder_text(flk!("settings_open_config_folder").into());
    translation.set_settings_open_cache_folder_text(flk!("settings_open_cache_folder").into());
    translation.set_settings_language_text(flk!("settings_language").into());
    translation.set_settings_current_preset_text(flk!("settings_current_preset").into());
    translation.set_settings_edit_name_text(flk!("settings_edit_name").into());
    translation.set_settings_choose_name_for_prefix_text(flk!("settings_choose_name_for_prefix").into());
    translation.set_settings_save_text(flk!("settings_save").into());
    translation.set_settings_load_text(flk!("settings_load").into());
    translation.set_settings_reset_text(flk!("settings_reset").into());
    translation.set_settings_similar_videos_tool_text(flk!("settings_similar_videos_tool").into());
    translation.set_settings_video_thumbnails_clear_unused_thumbnails_text(flk!("settings_video_thumbnails_clear_unused_thumbnails").into());
    translation.set_settings_video_thumbnails_header_text(flk!("settings_video_thumbnails_header").into());
    translation.set_settings_video_thumbnails_generate_text(flk!("settings_video_thumbnails_generate").into());
    translation.set_settings_video_thumbnails_position_text(flk!("settings_video_thumbnails_position").into());
    translation.set_settings_video_thumbnails_generate_grid_text(flk!("settings_video_thumbnails_generate_grid").into());
    translation.set_settings_video_thumbnails_generate_grid_hint_text(flk!("settings_video_thumbnails_generate_grid_hint").into());
    translation.set_settings_video_thumbnails_grid_tiles_per_side_text(flk!("settings_video_thumbnails_grid_tiles_per_side").into());
    translation.set_settings_video_thumbnails_grid_tiles_per_side_hint_text(flk!("settings_video_thumbnails_grid_tiles_per_side_hint").into());
    translation.set_settings_similar_images_tool_text(flk!("settings_similar_images_tool").into());
    translation.set_settings_general_settings_text(flk!("settings_general_settings").into());
    translation.set_settings_settings_text(flk!("settings_settings").into());
    translation.set_popup_save_title_text(flk!("popup_save_title").into());
    translation.set_popup_save_message_text(flk!("popup_save_message").into());
    translation.set_popup_rename_title_text(flk!("popup_rename_title").into());
    translation.set_popup_new_directories_title_text(flk!("popup_new_paths_title").into());
    translation.set_popup_move_title_text(flk!("popup_move_title").into());
    translation.set_popup_move_copy_checkbox_text(flk!("popup_move_copy_checkbox").into());
    translation.set_popup_move_preserve_folder_checkbox_text(flk!("popup_move_preserve_folder_checkbox").into());
    translation.set_delete_text(flk!("delete").into());
    translation.set_delete_confirmation_text(flk!("rust_delete_confirmation").into());
    translation.set_stopping_scan_text(flk!("stopping_scan").into());
    translation.set_searching_text(flk!("searching").into());
    translation.set_subsettings_videos_crop_detect_text(flk!("subsettings_videos_crop_detect").into());
    translation.set_subsettings_videos_skip_forward_amount_text(flk!("subsettings_videos_skip_forward_amount").into());
    translation.set_subsettings_videos_vid_hash_duration_text(flk!("subsettings_videos_vid_hash_duration").into());
    translation.set_settings_load_tabs_sizes_at_startup_text(flk!("settings_load_tabs_sizes_at_startup").into());
    translation.set_settings_load_windows_size_at_startup_text(flk!("settings_load_windows_size_at_startup").into());
    translation.set_settings_limit_lines_of_messages_text(flk!("settings_limit_lines_of_messages").into());
    translation.set_settings_play_audio_on_scan_completion_text(flk!("settings_play_audio_on_scan_completion_text").into());
    translation.set_settings_audio_feature_hint_text(flk!("settings_audio_feature_hint_text").into());
    translation.set_settings_audio_env_variable_hint_text(flk!("settings_audio_env_variable_hint_text").into());
    translation.set_settings_cache_number_size_text("".into());
    translation.set_settings_video_thumbnails_number_size_text("".into());
    translation.set_settings_log_number_size_text("".into());
    translation.set_settings_video_thumbnails_clear_unused_thumbnails_text(flk!("settings_video_thumbnails_clear_unused_thumbnails").into());
    translation.set_clean_exif_overwrite_files_text(flk!("clean_exif_overwrite_files_text").into());
    translation.set_subsettings_broken_files_video_info_text(flk!("subsettings_broken_files_video_info").into());
    translation.set_stop_text(flk!("stop_text").into());
    translation.set_settings_cache_header_text(flk!("settings_cache_header_text").into());
    translation.set_settings_clean_cache_button_text(flk!("settings_clean_cache_button_text").into());
    translation.set_popup_clean_cache_title_text(flk!("popup_clean_cache_title_text").into());
    translation.set_popup_clean_cache_confirmation_text(flk!("popup_clean_cache_confirmation_text").into());
    translation.set_popup_clean_cache_progress_text(flk!("popup_clean_cache_progress_text").into());
    translation.set_popup_clean_cache_current_file_text(flk!("popup_clean_cache_current_file_text").into());
    translation.set_popup_clean_cache_file_progress_text(flk!("popup_clean_cache_file_progress_text").into());
    translation.set_popup_clean_cache_overall_progress_text(flk!("popup_clean_cache_overall_progress_text").into());
    translation.set_popup_clean_cache_stopped_by_user_text(flk!("popup_clean_cache_stopped_by_user_text").into());
    translation.set_popup_clean_cache_finished_text(flk!("popup_clean_cache_finished_text").into());
    translation.set_popup_clean_cache_error_details_text(flk!("popup_clean_cache_error_details_text").into());
    translation.set_popup_clean_cache_files_with_errors(flk!("popup_clean_cache_files_with_errors").into());

    let tools_model: [(SharedString, ActiveTab); TOOLS_NUMBER] = [
        (flk!("tool_duplicate_files").into(), ActiveTab::DuplicateFiles),
        (flk!("tool_empty_folders").into(), ActiveTab::EmptyFolders),
        (flk!("tool_big_files").into(), ActiveTab::BigFiles),
        (flk!("tool_empty_files").into(), ActiveTab::EmptyFiles),
        (flk!("tool_temporary_files").into(), ActiveTab::TemporaryFiles),
        (flk!("tool_similar_images").into(), ActiveTab::SimilarImages),
        (flk!("tool_similar_videos").into(), ActiveTab::SimilarVideos),
        (flk!("tool_music_duplicates").into(), ActiveTab::SimilarMusic),
        (flk!("tool_invalid_symlinks").into(), ActiveTab::InvalidSymlinks),
        (flk!("tool_broken_files").into(), ActiveTab::BrokenFiles),
        (flk!("tool_bad_extensions").into(), ActiveTab::BadExtensions),
        (flk!("tool_bad_names").into(), ActiveTab::BadNames),
        (flk!("tool_exif_remover").into(), ActiveTab::ExifRemover),
        (flk!("tool_video_optimizer").into(), ActiveTab::VideoOptimizer),
    ];
    let gui_state = app.global::<GuiState>();
    gui_state.set_tools_model(ModelRc::new(VecModel::from(tools_model.to_vec())));

    let sort_model: [SortModel; 3] = [
        SortModel {
            data: SortMode::FullName,
            name: flk!("sort_by_full_name").into(),
        },
        SortModel {
            data: SortMode::Selection,
            name: flk!("sort_by_selection").into(),
        },
        SortModel {
            data: SortMode::Reverse,
            name: flk!("sort_reverse").into(),
        },
    ];

    gui_state.set_sort_results_list(ModelRc::new(VecModel::from(sort_model.to_vec())));

    let selection = flk!("column_selection");
    let size = flk!("column_size");
    let file_name = flk!("column_file_name");
    let path = flk!("column_path");
    let mod_date = flk!("column_modification_date");
    let similarity = flk!("column_similarity");
    let dimensions = flk!("column_dimensions");
    let title = flk!("column_title");
    let artist = flk!("column_artist");
    let year = flk!("column_year");
    let bitrate = flk!("column_bitrate");
    let length = flk!("column_length");
    let genre = flk!("column_genre");
    let fps = flk!("column_fps");
    let codec = flk!("column_codec");
    let duration = flk!("column_duration");
    let type_of_error = flk!("column_type_of_error");
    let symlink_name = flk!("column_symlink_name");
    let symlink_folder = flk!("column_symlink_folder");
    let destination_path = flk!("column_destination_path");
    let current_extension = flk!("column_current_extension");
    let proper_extension = flk!("column_proper_extension");
    let exif_tags = flk!("column_exif_tags");
    let new_dimensions = flk!("column_new_dimensions");
    let new_name = flk!("column_new_name");

    let fnm = |model: &[&str]| {
        let shared_string = model.iter().map(|s| (*s).into()).collect::<Vec<SharedString>>();
        ModelRc::new(VecModel::from(shared_string))
    };

    settings.set_duplicates_column_name(fnm(&[&selection, &size, &file_name, &path, &mod_date]));
    settings.set_empty_folders_column_name(fnm(&[&selection, &file_name, &path, &mod_date]));
    settings.set_empty_files_column_name(fnm(&[&selection, &file_name, &path, &mod_date]));
    settings.set_temporary_files_column_name(fnm(&[&selection, &file_name, &path, &mod_date]));
    settings.set_big_files_column_name(fnm(&[&selection, &size, &file_name, &path, &mod_date]));
    settings.set_similar_images_column_name(fnm(&[&selection, &similarity, &size, &dimensions, &file_name, &path, &mod_date]));
    settings.set_similar_videos_column_name(fnm(&[&selection, &size, &file_name, &path, &dimensions, &duration, &bitrate, &fps, &codec, &mod_date]));
    settings.set_similar_music_column_name(fnm(&[&selection, &size, &file_name, &title, &artist, &year, &bitrate, &length, &genre, &path, &mod_date]));
    settings.set_invalid_symlink_column_name(fnm(&[&selection, &symlink_name, &symlink_folder, &destination_path, &mod_date]));
    settings.set_broken_files_column_name(fnm(&[&selection, &file_name, &path, &type_of_error, &size, &mod_date]));
    settings.set_bad_extensions_column_name(fnm(&[&selection, &file_name, &path, &current_extension, &proper_extension]));
    settings.set_exif_remover_column_name(fnm(&[&selection, &size, &file_name, &path, &exif_tags, &mod_date]));
    settings.set_video_optimizer_column_name(fnm(&[&selection, &size, &file_name, &path, &codec, &dimensions, &new_dimensions, &mod_date]));
    settings.set_bad_names_column_name(fnm(&[&selection, &file_name, &new_name, &path]));
}

pub(crate) fn translate_select_mode(select_mode: SelectMode) -> SharedString {
    match select_mode {
        SelectMode::SelectAll => flk!("selection_all").into(),
        SelectMode::UnselectAll => flk!("selection_deselect_all").into(),
        SelectMode::InvertSelection => flk!("selection_invert_selection").into(),
        SelectMode::SelectTheBiggestSize => flk!("selection_the_biggest_size").into(),
        SelectMode::SelectTheBiggestResolution => flk!("selection_the_biggest_resolution").into(),
        SelectMode::SelectTheSmallestSize => flk!("selection_the_smallest_size").into(),
        SelectMode::SelectTheSmallestResolution => flk!("selection_the_smallest_resolution").into(),
        SelectMode::SelectNewest => flk!("selection_newest").into(),
        SelectMode::SelectOldest => flk!("selection_oldest").into(),
        SelectMode::SelectShortestPath => flk!("selection_shortest_path").into(),
        SelectMode::SelectLongestPath => flk!("selection_longest_path").into(),
    }
}

pub(crate) fn translate_sort_mode(sort_mode: SortMode) -> SharedString {
    match sort_mode {
        SortMode::FullName => flk!("sort_by_full_name").into(),
        SortMode::Selection => flk!("sort_by_selection").into(),
        SortMode::Reverse => flk!("sort_reverse").into(),
    }
}
