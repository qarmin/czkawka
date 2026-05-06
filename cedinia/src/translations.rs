use slint::{ComponentHandle, ModelRc, SharedString, VecModel};

use crate::settings::gui_settings_values::StringComboBoxItems;
use crate::{BigFilesSettings, DuplicateSettings, GeneralSettings, MainWindow, SameMusicSettings, SimilarImagesSettings, SimilarVideosSettings, Translations, flc};

pub(crate) fn translate_items(app: &MainWindow) {
    let t = app.global::<Translations>();

    t.set_app_name_text(flc!("app_name").into());
    t.set_tool_duplicate_files_text(flc!("tool_duplicate_files").into());
    t.set_tool_empty_folders_text(flc!("tool_empty_folders").into());
    t.set_tool_similar_images_text(flc!("tool_similar_images").into());
    t.set_tool_empty_files_text(flc!("tool_empty_files").into());
    t.set_tool_temporary_files_text(flc!("tool_temporary_files").into());
    t.set_tool_big_files_text(flc!("tool_big_files").into());
    t.set_tool_broken_files_text(flc!("tool_broken_files").into());
    t.set_tool_bad_extensions_text(flc!("tool_bad_extensions").into());
    t.set_tool_same_music_text(flc!("tool_same_music").into());
    t.set_tool_bad_names_text(flc!("tool_bad_names").into());
    t.set_tool_exif_remover_text(flc!("tool_exif_remover").into());
    t.set_tool_similar_videos_text(flc!("tool_similar_videos").into());
    t.set_tool_directories_text(flc!("tool_directories").into());
    t.set_tool_settings_text(flc!("tool_settings").into());

    t.set_home_dup_description_text(flc!("home_dup_description").into());
    t.set_home_empty_folders_description_text(flc!("home_empty_folders_description").into());
    t.set_home_similar_images_description_text(flc!("home_similar_images_description").into());
    t.set_home_empty_files_description_text(flc!("home_empty_files_description").into());
    t.set_home_temp_files_description_text(flc!("home_temp_files_description").into());
    t.set_home_big_files_description_text(flc!("home_big_files_description").into());
    t.set_home_broken_files_description_text(flc!("home_broken_files_description").into());
    t.set_home_bad_extensions_description_text(flc!("home_bad_extensions_description").into());
    t.set_home_same_music_description_text(flc!("home_same_music_description").into());
    t.set_home_bad_names_description_text(flc!("home_bad_names_description").into());
    t.set_home_exif_description_text(flc!("home_exif_description").into());
    t.set_home_similar_videos_description_text(flc!("home_similar_videos_description").into());

    t.set_scanning_text(flc!("scanning").into());
    t.set_stopping_text(flc!("stopping").into());
    t.set_no_results_text(flc!("no_results").into());
    t.set_press_start_text(flc!("press_start").into());
    t.set_select_label_text(flc!("select_label").into());
    t.set_deselect_label_text(flc!("deselect_label").into());
    t.set_list_label_text(flc!("list_label").into());
    t.set_gallery_label_text(flc!("gallery_label").into());

    t.set_selection_popup_title_text(flc!("selection_popup_title").into());
    t.set_select_all_text(flc!("select_all").into());
    t.set_select_except_one_text(flc!("select_except_one").into());
    t.set_select_except_largest_text(flc!("select_except_largest").into());
    t.set_select_except_smallest_text(flc!("select_except_smallest").into());
    t.set_select_largest_text(flc!("select_largest").into());
    t.set_select_smallest_text(flc!("select_smallest").into());
    t.set_select_except_highest_res_text(flc!("select_except_highest_res").into());
    t.set_select_except_lowest_res_text(flc!("select_except_lowest_res").into());
    t.set_select_highest_res_text(flc!("select_highest_res").into());
    t.set_select_lowest_res_text(flc!("select_lowest_res").into());
    t.set_invert_selection_text(flc!("invert_selection").into());
    t.set_close_text(flc!("close").into());

    t.set_deselection_popup_title_text(flc!("deselection_popup_title").into());
    t.set_deselect_all_text(flc!("deselect_all").into());
    t.set_deselect_except_one_text(flc!("deselect_except_one").into());

    t.set_cancel_text(flc!("cancel").into());
    t.set_delete_text(flc!("delete").into());
    t.set_rename_text(flc!("rename").into());

    t.set_delete_errors_title_text(flc!("delete_errors_title").into());
    t.set_ok_text(flc!("ok").into());

    t.set_stopping_overlay_title_text(flc!("stopping_overlay_title").into());
    t.set_stopping_overlay_body_text(flc!("stopping_overlay_body").into());

    t.set_permission_title_text(flc!("permission_title").into());
    t.set_permission_body_text(flc!("permission_body").into());
    t.set_grant_text(flc!("grant").into());
    t.set_no_permission_scan_warning_text(flc!("no_permission_scan_warning").into());

    t.set_settings_tab_general_text(flc!("settings_tab_general").into());
    t.set_settings_tab_tools_text(flc!("settings_tab_tools").into());
    t.set_settings_tab_diagnostics_text(flc!("settings_tab_diagnostics").into());

    t.set_settings_use_cache_text(flc!("settings_use_cache").into());
    t.set_settings_use_cache_desc_text(flc!("settings_use_cache_desc").into());
    t.set_settings_ignore_hidden_text(flc!("settings_ignore_hidden").into());
    t.set_settings_ignore_hidden_desc_text(flc!("settings_ignore_hidden_desc").into());
    t.set_settings_show_notification_text(flc!("settings_show_notification").into());
    t.set_settings_show_notification_desc_text(flc!("settings_show_notification_desc").into());
    t.set_settings_notify_only_background_text(flc!("settings_notify_only_background").into());
    t.set_settings_notify_only_background_desc_text(flc!("settings_notify_only_background_desc").into());
    t.set_notifications_disabled_banner_text(flc!("notifications_disabled_banner").into());
    t.set_notifications_enable_button_text(flc!("notifications_enable_button").into());
    t.set_settings_scan_label_text(flc!("settings_scan_label").into());
    t.set_settings_filters_label_text(flc!("settings_filters_label").into());
    t.set_settings_min_file_size_text(flc!("settings_min_file_size").into());
    t.set_settings_max_file_size_text(flc!("settings_max_file_size").into());
    t.set_settings_language_text(flc!("settings_language").into());
    t.set_settings_language_restart_text(flc!("settings_language_restart").into());
    t.set_settings_common_label_text(flc!("settings_common_label").into());
    t.set_settings_hash_type_desc_text(flc!("settings_hash_type_desc").into());
    t.set_settings_similarity_desc_text(flc!("settings_similarity_desc").into());
    t.set_settings_hash_size_desc_text(flc!("settings_hash_size_desc").into());
    t.set_settings_excluded_items_text(flc!("settings_excluded_items").into());
    t.set_settings_excluded_items_placeholder_text(flc!("settings_excluded_items_placeholder").into());
    t.set_settings_allowed_extensions_text(flc!("settings_allowed_extensions").into());
    t.set_settings_allowed_extensions_placeholder_text(flc!("settings_allowed_extensions_placeholder").into());
    t.set_settings_excluded_extensions_text(flc!("settings_excluded_extensions").into());
    t.set_settings_excluded_extensions_placeholder_text(flc!("settings_excluded_extensions_placeholder").into());

    t.set_settings_duplicates_header_text(flc!("settings_duplicates_header").into());
    t.set_settings_check_method_label_text(flc!("settings_check_method_label").into());
    t.set_settings_check_method_text(flc!("settings_check_method").into());
    t.set_settings_hash_type_label_text(flc!("settings_hash_type_label").into());
    t.set_settings_hash_type_text(flc!("settings_hash_type").into());
    t.set_settings_similar_images_header_text(flc!("settings_similar_images_header").into());
    t.set_settings_similarity_preset_text(flc!("settings_similarity_preset").into());
    t.set_settings_hash_size_text(flc!("settings_hash_size").into());
    t.set_settings_hash_alg_text(flc!("settings_hash_alg").into());
    t.set_settings_image_filter_text(flc!("settings_image_filter").into());
    t.set_settings_ignore_same_size_text(flc!("settings_ignore_same_size").into());
    t.set_settings_gallery_image_fit_cover_text(flc!("settings_gallery_image_fit_cover").into());
    t.set_settings_gallery_image_fit_cover_desc_text(flc!("settings_gallery_image_fit_cover_desc").into());
    t.set_settings_big_files_header_text(flc!("settings_big_files_header").into());
    t.set_settings_search_mode_text(flc!("settings_search_mode").into());
    t.set_settings_file_count_text(flc!("settings_file_count").into());
    t.set_settings_same_music_header_text(flc!("settings_same_music_header").into());
    t.set_settings_music_check_method_text(flc!("settings_music_check_method").into());
    t.set_settings_music_compare_tags_label_text(flc!("settings_music_compare_tags_label").into());
    t.set_settings_music_title_text(flc!("settings_music_title").into());
    t.set_settings_music_artist_text(flc!("settings_music_artist").into());
    t.set_settings_music_year_text(flc!("settings_music_year").into());
    t.set_settings_music_length_text(flc!("settings_music_length").into());
    t.set_settings_music_genre_text(flc!("settings_music_genre").into());
    t.set_settings_music_bitrate_text(flc!("settings_music_bitrate").into());
    t.set_settings_music_approx_text(flc!("settings_music_approx").into());
    t.set_settings_temporary_files_header_text(flc!("settings_temporary_files_header").into());
    t.set_settings_temporary_files_extensions_label_text(flc!("settings_temporary_files_extensions_label").into());
    t.set_settings_temporary_files_extensions_placeholder_text(flc!("settings_temporary_files_extensions_placeholder").into());
    t.set_settings_temporary_files_reset_text(flc!("settings_temporary_files_reset").into());
    t.set_settings_broken_files_header_text(flc!("settings_broken_files_header").into());
    t.set_settings_broken_files_note_text(flc!("settings_broken_files_note").into());
    t.set_settings_broken_files_types_label_text(flc!("settings_broken_files_types_label").into());
    t.set_settings_broken_audio_text(flc!("settings_broken_audio").into());
    t.set_settings_broken_pdf_text(flc!("settings_broken_pdf").into());
    t.set_settings_broken_archive_text(flc!("settings_broken_archive").into());
    t.set_settings_broken_image_text(flc!("settings_broken_image").into());
    t.set_settings_broken_font_text(flc!("settings_broken_font").into());
    t.set_settings_broken_markup_text(flc!("settings_broken_markup").into());
    t.set_settings_similar_videos_header_text(flc!("settings_similar_videos_header").into());
    t.set_settings_similar_videos_audio_preset_text(flc!("settings_similar_videos_audio_preset").into());
    t.set_settings_similar_videos_audio_preset_desc_text(flc!("settings_similar_videos_audio_preset_desc").into());
    t.set_settings_bad_names_header_text(flc!("settings_bad_names_header").into());
    t.set_settings_bad_names_checks_label_text(flc!("settings_bad_names_checks_label").into());
    t.set_settings_bad_names_uppercase_ext_text(flc!("settings_bad_names_uppercase_ext").into());
    t.set_settings_bad_names_emoji_text(flc!("settings_bad_names_emoji").into());
    t.set_settings_bad_names_space_text(flc!("settings_bad_names_space").into());
    t.set_settings_bad_names_non_ascii_text(flc!("settings_bad_names_non_ascii").into());
    t.set_settings_bad_names_duplicated_text(flc!("settings_bad_names_duplicated").into());

    t.set_diagnostics_header_text(flc!("diagnostics_header").into());
    t.set_diagnostics_thumbnails_text(flc!("diagnostics_thumbnails").into());
    t.set_diagnostics_app_cache_text(flc!("diagnostics_app_cache").into());
    t.set_diagnostics_refresh_text(flc!("diagnostics_refresh").into());
    t.set_diagnostics_clear_thumbnails_text(flc!("diagnostics_clear_thumbnails").into());
    t.set_diagnostics_open_thumbnails_folder_text(flc!("diagnostics_open_thumbnails_folder").into());
    t.set_diagnostics_clear_cache_text(flc!("diagnostics_clear_cache").into());
    t.set_diagnostics_open_cache_folder_text(flc!("diagnostics_open_cache_folder").into());
    t.set_diagnostics_collect_test_text(flc!("diagnostics_collect_test").into());
    t.set_diagnostics_collect_test_desc_text(flc!("diagnostics_collect_test_desc").into());
    t.set_diagnostics_collect_test_run_text(flc!("diagnostics_collect_test_run").into());
    t.set_diagnostics_collect_test_stop_text(flc!("diagnostics_collect_test_stop").into());
    t.set_collect_test_cancelled_text(flc!("collect_test_cancelled").into());
    t.set_diag_confirm_clear_thumbnails_text(flc!("diag_confirm_clear_thumbnails").into());
    t.set_diag_confirm_clear_cache_text(flc!("diag_confirm_clear_cache").into());

    t.set_collect_test_title_text(flc!("collect_test_title").into());
    t.set_collect_test_volumes_text(flc!("collect_test_volumes").into());
    t.set_collect_test_folders_text(flc!("collect_test_folders").into());
    t.set_collect_test_files_text(flc!("collect_test_files").into());
    t.set_collect_test_time_text(flc!("collect_test_time").into());

    t.set_directories_include_header_text(flc!("directories_include_header").into());
    t.set_directories_included_text(flc!("directories_included").into());
    t.set_directories_exclude_header_text(flc!("directories_exclude_header").into());
    t.set_directories_excluded_header_text(flc!("directories_excluded_header").into());
    t.set_directories_add_text(flc!("directories_add").into());
    t.set_directories_volume_header_text(flc!("directories_volume_header").into());
    t.set_directories_volume_refresh_text(flc!("directories_volume_refresh").into());
    t.set_directories_volume_add_text(flc!("directories_volume_add").into());
    t.set_no_paths_text(flc!("no_paths").into());
    t.set_gallery_delete_button_text(flc!("gallery_delete_button").into());
    t.set_gallery_back_text(flc!("gallery_back").into());
    t.set_gallery_confirm_delete_text(flc!("gallery_confirm_delete").into());
    t.set_deleting_files_text(flc!("deleting_files").into());
    t.set_stop_text(flc!("stop").into());
    t.set_files_suffix_text(flc!("files_suffix").into());
    t.set_scanning_fallback_text(flc!("scanning_fallback").into());
    t.set_app_subtitle_text(flc!("app_subtitle").into());
    t.set_app_license_text(flc!("app_license").into());
    t.set_about_app_label_text(flc!("about_app_label").into());
    t.set_cache_label_text(flc!("cache_label").into());

    t.set_nav_home_text(flc!("nav_home").into());
    t.set_nav_dirs_text(flc!("nav_dirs").into());
    t.set_nav_settings_text(flc!("nav_settings").into());

    t.set_status_ready_text(flc!("status_ready").into());
    t.set_status_stopped_text(flc!("status_stopped").into());
    t.set_status_no_results_text(flc!("status_no_results").into());
    t.set_status_deleted_selected_text(flc!("status_deleted_selected").into());
    t.set_status_deleted_with_errors_text(flc!("status_deleted_with_errors").into());
    t.set_scan_not_started_text(flc!("scan_not_started").into());
    t.set_found_items_prefix_text(flc!("found_items_prefix").into());
    t.set_found_items_suffix_text(flc!("found_items_suffix").into());
    t.set_deleted_items_prefix_text(flc!("deleted_items_prefix").into());
    t.set_deleted_items_suffix_text(flc!("deleted_items_suffix").into());
    t.set_deleted_errors_suffix_text(flc!("deleted_errors_suffix").into());
    t.set_renamed_prefix_text(flc!("renamed_prefix").into());
    t.set_renamed_files_suffix_text(flc!("renamed_files_suffix").into());
    t.set_renamed_errors_suffix_text(flc!("renamed_errors_suffix").into());
    t.set_cleaned_exif_prefix_text(flc!("cleaned_exif_prefix").into());
    t.set_cleaned_exif_suffix_text(flc!("cleaned_exif_suffix").into());
    t.set_cleaned_exif_errors_suffix_text(flc!("cleaned_exif_errors_suffix").into());
    t.set_and_more_prefix_text(flc!("and_more_prefix").into());
    t.set_and_more_suffix_text(flc!("and_more_suffix").into());

    t.set_about_repo_text(flc!("about_repo").into());
    t.set_about_translate_text(flc!("about_translate").into());
    t.set_about_donate_text(flc!("about_donate").into());

    t.set_same_music_fingerprint_warning_text(flc!("same_music_fingerprint_warning").into());

    t.set_directories_referenced_tooltip_text(flc!("directories_referenced_tooltip").into());
    t.set_directories_include_section_header_text(flc!("directories_include_section_header").into());
    t.set_directories_exclude_section_header_text(flc!("directories_exclude_section_header").into());
    t.set_directories_custom_paths_text(flc!("directories_custom_paths").into());
    t.set_directories_check_button_text(flc!("directories_check_button").into());
    t.set_directories_check_popup_title_text(flc!("directories_check_popup_title").into());
    t.set_directories_check_label_included_text(flc!("directories_check_label_included").into());
    t.set_directories_check_label_excluded_text(flc!("directories_check_label_excluded").into());
    t.set_directories_check_label_referenced_text(flc!("directories_check_label_referenced").into());
    t.set_directories_check_label_would_scan_text(flc!("directories_check_label_would_scan").into());
    t.set_directories_check_label_processable_text(flc!("directories_check_label_processable").into());
    t.set_directories_check_scanning_text(flc!("directories_check_scanning").into());
    t.set_directories_check_warning_no_processable_text(flc!("directories_check_warning_no_processable").into());
    t.set_path_edit_title_include_text(flc!("path_edit_title_include").into());
    t.set_path_edit_title_exclude_text(flc!("path_edit_title_exclude").into());
    t.set_path_edit_placeholder_text(flc!("path_edit_placeholder").into());
    t.set_path_edit_not_exists_text(flc!("path_edit_not_exists").into());
    t.set_path_edit_is_dir_text(flc!("path_edit_is_dir").into());
    t.set_path_edit_is_file_text(flc!("path_edit_is_file").into());
    t.set_path_edit_no_newlines_text(flc!("path_edit_no_newlines").into());

    t.set_licenses_label_text(flc!("licenses_label").into());
    t.set_third_party_licenses_text(flc!("third_party_licenses").into());
    t.set_licenses_popup_title_text(flc!("licenses_popup_title").into());

    t.set_ctx_menu_title_text(flc!("ctx_menu_title").into());
    t.set_ctx_open_file_text(flc!("ctx_open_file").into());
    t.set_ctx_open_folder_text(flc!("ctx_open_folder").into());
    t.set_settings_ignore_same_resolution_text(flc!("settings_ignore_same_resolution").into());
    t.set_settings_appearance_label_text(flc!("settings_appearance_label").into());
    t.set_settings_dark_theme_text(flc!("settings_dark_theme").into());
    t.set_settings_dark_theme_desc_text(flc!("settings_dark_theme_desc").into());
    t.set_compare_label_text(flc!("compare_label").into());
    t.set_compare_loading_text(flc!("compare_loading").into());
    t.set_compare_cancelling_text(flc!("compare_cancelling").into());
    t.set_compare_computing_text(flc!("compare_computing").into());
    t.set_compare_mode_normal_text(flc!("compare_mode_normal").into());
    t.set_compare_mode_split_text(flc!("compare_mode_split").into());
    t.set_compare_mode_overlay_text(flc!("compare_mode_overlay").into());
    t.set_compare_mode_diff_text(flc!("compare_mode_diff").into());
    t.set_compare_res_mismatch_text(flc!("compare_res_mismatch").into());
    t.set_dir_open_folder_text(flc!("dir_open_folder").into());

    use std::fmt::Debug;

    use crate::settings::gui_settings_values::StringComboBoxItem;

    fn make_options<T: Clone + Debug>(items: &[StringComboBoxItem<T>]) -> ModelRc<SharedString> {
        ModelRc::new(VecModel::from(items.iter().map(|i| SharedString::from(i.translated_display_name())).collect::<Vec<_>>()))
    }

    let combo_items = StringComboBoxItems::new();

    let g = app.global::<GeneralSettings>();
    g.set_min_file_size_options(make_options(&combo_items.min_file_size));
    g.set_max_file_size_options(make_options(&combo_items.max_file_size));

    let dup = app.global::<DuplicateSettings>();
    dup.set_check_method_options(make_options(&combo_items.duplicates_check_method));

    let bfs = app.global::<BigFilesSettings>();
    bfs.set_search_mode_options(make_options(&combo_items.biggest_files_method));

    let si = app.global::<SimilarImagesSettings>();
    si.set_similarity_preset_options(make_options(&combo_items.similarity_preset));

    let sm = app.global::<SameMusicSettings>();
    sm.set_check_method_options(make_options(&combo_items.same_music_check_method));

    let sv = app.global::<SimilarVideosSettings>();
    sv.set_audio_preset_options(make_options(&combo_items.similar_videos_audio_preset));
}
