use czkawka_core::TOOLS_NUMBER;
use i18n_embed::unic_langid::LanguageIdentifier;
use log::error;
use slint::{ComponentHandle, ModelRc, SharedString, VecModel};

use crate::{Callabler, CurrentTab, GuiState, MainWindow, SelectMode, Settings, SortMode, SortModel, Translations, flk, localizer_krokiet};

struct Language {
    long_name: &'static str,
    short_name: &'static str,
    left_panel_size: f32, // Currently don't know how to automatically calculate this, so each language has its own size
}

const LANGUAGE_LIST: &[Language] = &[
    Language {
        long_name: "English",
        short_name: "en",
        left_panel_size: 120.0,
    },
    Language {
        long_name: "Polski (Polish)",
        short_name: "pl",
        left_panel_size: 150.0,
    },
];

pub fn connect_translations(app: &MainWindow) {
    init_languages(app);
    translate_items(app);

    let a = app.as_weak();
    app.global::<Callabler>().on_changed_language(move || {
        let app = a.upgrade().unwrap();
        change_language(&app);
    });
}

fn change_language(app: &MainWindow) {
    let localizers = vec![
        ("czkawka_core", czkawka_core::localizer_core::localizer_core()),
        ("krokiet", localizer_krokiet::localizer_krokiet()),
    ];

    let lang = app.global::<Settings>().get_language_index();
    let lang_items = &LANGUAGE_LIST[lang as usize];

    let lang_identifier = vec![LanguageIdentifier::from_bytes(lang_items.short_name.as_bytes()).expect("Failed to create LanguageIdentifier")];
    for (lib, localizer) in localizers {
        if let Err(error) = localizer.select(&lang_identifier) {
            error!("Error while loadings languages for {lib} {error:?}");
        }
    }

    app.global::<GuiState>().set_left_panel_width(lang_items.left_panel_size);
    translate_items(app);
}

fn init_languages(app: &MainWindow) {
    let new_languages_model: Vec<SharedString> = LANGUAGE_LIST.iter().map(|e| e.long_name.into()).collect::<Vec<_>>();

    app.global::<Settings>().set_languages_list(ModelRc::new(VecModel::from(new_languages_model)));
    app.global::<Settings>().set_language_index(0); // TODO loaded from settings
}

// To generate this, check misc folder
// This is ugly workaround for missing fluent language support in slint
fn translate_items(app: &MainWindow) {
    let translation = app.global::<Translations>();

    translation.set_yes_button_text(flk!("yes_button").into());
    translation.set_no_button_text(flk!("no_button").into());
    translation.set_ok_button_text(flk!("ok_button").into());
    translation.set_cancel_button_text(flk!("cancel_button").into());
    translation.set_are_you_want_to_continue_text(flk!("are_you_want_to_continue").into());
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
    translation.set_add_button_text(flk!("add_button").into());
    translation.set_remove_button_text(flk!("remove_button").into());
    translation.set_manual_add_button_text(flk!("manual_add_button").into());
    translation.set_included_directories_text(flk!("included_directories").into());
    translation.set_excluded_directories_text(flk!("excluded_directories").into());
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
    translation.set_sort_by_item_name_text(flk!("sort_by_item_name").into());
    translation.set_sort_by_parent_name_text(flk!("sort_by_parent_name").into());
    translation.set_sort_by_full_name_text(flk!("sort_by_full_name").into());
    translation.set_sort_by_size_text(flk!("sort_by_size").into());
    translation.set_sort_by_modification_date_text(flk!("sort_by_modification_date").into());
    translation.set_sort_by_selection_text(flk!("sort_by_selection").into());
    translation.set_sort_reverse_text(flk!("sort_reverse").into());
    translation.set_selection_all_text(flk!("selection_all").into());
    translation.set_selection_deselect_all_text(flk!("selection_deselect_all").into());
    translation.set_selection_invert_selection_text(flk!("selection_invert_selection").into());
    translation.set_selection_the_biggest_size_text(flk!("selection_the_biggest_size").into());
    translation.set_selection_the_biggest_resolution_text(flk!("selection_the_biggest_resolution").into());
    translation.set_selection_the_smallest_size_text(flk!("selection_the_smallest_size").into());
    translation.set_selection_the_smallest_resolution_text(flk!("selection_the_smallest_resolution").into());
    translation.set_selection_newest_text(flk!("selection_newest").into());
    translation.set_selection_oldest_text(flk!("selection_oldest").into());
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
    translation.set_subsettings_broken_files_pdf_text(flk!("subsettings_broken_files_pdf").into());
    translation.set_subsettings_broken_files_archive_text(flk!("subsettings_broken_files_archive").into());
    translation.set_subsettings_broken_files_image_text(flk!("subsettings_broken_files_image").into());
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
    translation.set_settings_duplicate_hide_hard_links_text(flk!("settings_duplicate_hide_hard_links").into());
    translation.set_settings_duplicate_minimal_hash_cache_size_text(flk!("settings_duplicate_minimal_hash_cache_size").into());
    translation.set_settings_duplicate_use_prehash_text(flk!("settings_duplicate_use_prehash").into());
    translation.set_settings_duplicate_minimal_prehash_cache_size_text(flk!("settings_duplicate_minimal_prehash_cache_size").into());
    translation.set_settings_duplicate_delete_outdated_entries_text(flk!("settings_duplicate_delete_outdated_entries").into());
    translation.set_settings_similar_images_show_image_preview_text(flk!("settings_similar_images_show_image_preview").into());
    translation.set_settings_similar_images_hide_hard_links_text(flk!("settings_similar_images_hide_hard_links").into());
    translation.set_settings_delete_outdated_entries_text(flk!("settings_delete_outdated_entries").into());
    translation.set_settings_similar_videos_hide_hard_links_text(flk!("settings_similar_videos_hide_hard_links").into());
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
    translation.set_settings_similar_images_tool_text(flk!("settings_similar_images_tool").into());
    translation.set_settings_similar_music_tool_text(flk!("settings_similar_music_tool").into());
    translation.set_settings_duplicate_tool_text(flk!("settings_duplicate_tool").into());
    translation.set_settings_general_settings_text(flk!("settings_general_settings").into());
    translation.set_settings_settings_text(flk!("settings_settings").into());
    translation.set_popup_save_title_text(flk!("popup_save_title").into());
    translation.set_popup_save_message_text(flk!("popup_save_message").into());
    translation.set_popup_rename_title_text(flk!("popup_rename_title").into());
    translation.set_popup_rename_message_text(flk!("popup_rename_message").into());
    translation.set_popup_new_directories_title_text(flk!("popup_new_directories_title").into());
    translation.set_popup_move_title_text(flk!("popup_move_title").into());
    translation.set_popup_move_message_text(flk!("popup_move_message").into());
    translation.set_popup_move_copy_checkbox_text(flk!("popup_move_copy_checkbox").into());
    translation.set_popup_move_preserve_folder_checkbox_text(flk!("popup_move_preserve_folder_checkbox").into());
    translation.set_delete_text(flk!("delete").into());
    translation.set_delete_confirmation_text(flk!("delete_confirmation").into());
    translation.set_stopping_scan_text(flk!("stopping_scan").into());
    translation.set_searching_text(flk!("searching").into());

    let tools_model: [(SharedString, CurrentTab); TOOLS_NUMBER] = [
        (flk!("tool_duplicate_files").into(), CurrentTab::DuplicateFiles),
        (flk!("tool_empty_folders").into(), CurrentTab::EmptyFolders),
        (flk!("tool_big_files").into(), CurrentTab::BigFiles),
        (flk!("tool_empty_files").into(), CurrentTab::EmptyFiles),
        (flk!("tool_temporary_files").into(), CurrentTab::TemporaryFiles),
        (flk!("tool_similar_images").into(), CurrentTab::SimilarImages),
        (flk!("tool_similar_videos").into(), CurrentTab::SimilarVideos),
        (flk!("tool_music_duplicates").into(), CurrentTab::SimilarMusic),
        (flk!("tool_invalid_symlinks").into(), CurrentTab::InvalidSymlinks),
        (flk!("tool_broken_files").into(), CurrentTab::BrokenFiles),
        (flk!("tool_bad_extensions").into(), CurrentTab::BadExtensions),
    ];
    let gui_state = app.global::<GuiState>();
    gui_state.set_tools_model(ModelRc::new(VecModel::from(tools_model.to_vec())));

    let sort_model: [SortModel; 7] = [
        SortModel {
            data: SortMode::ItemName,
            name: flk!("sort_by_item_name").into(),
        },
        SortModel {
            data: SortMode::ParentName,
            name: flk!("sort_by_parent_name").into(),
        },
        SortModel {
            data: SortMode::FullName,
            name: flk!("sort_by_full_name").into(),
        },
        SortModel {
            data: SortMode::Size,
            name: flk!("sort_by_size").into(),
        },
        SortModel {
            data: SortMode::ModificationDate,
            name: flk!("sort_by_modification_date").into(),
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
    }
}
