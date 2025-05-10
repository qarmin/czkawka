use czkawka_core::TOOLS_NUMBER;
use i18n_embed::DesktopLanguageRequester;
use i18n_embed::unic_langid::LanguageIdentifier;
use log::{error, info};
use slint::{ComponentHandle, ModelRc, SharedString, VecModel};

use crate::{Callabler, CurrentTab, GuiState, MainWindow, SelectMode, Settings, SortMode, SortModel, Translations, flk, localizer_krokiet};

pub struct Language {
    pub long_name: &'static str,
    pub short_name: &'static str,
    pub left_panel_size: f32, // Currently don't know how to automatically calculate this, so each language has its own size
}

pub const LANGUAGE_LIST: &[Language] = &[
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
    Language {
        long_name: "Français (French)",
        short_name: "fr",
        left_panel_size: 180.0,
    },
    Language {
        long_name: "Italiano (Italian)",
        short_name: "it",
        left_panel_size: 145.0,
    },
    Language {
        long_name: "Русский (Russian)",
        short_name: "ru",
        left_panel_size: 185.0,
    },
    Language {
        long_name: "український (Ukrainian)",
        short_name: "uk",
        left_panel_size: 185.0,
    },
    Language {
        long_name: "한국인 (Korean)",
        short_name: "ko",
        left_panel_size: 145.0,
    },
    Language {
        long_name: "Česky (Czech)",
        short_name: "cs",
        left_panel_size: 170.0,
    },
    Language {
        long_name: "Deutsch (German)",
        short_name: "de",
        left_panel_size: 155.0,
    },
    Language {
        long_name: "やまと (Japanese)",
        short_name: "ja",
        left_panel_size: 155.0,
    },
    Language {
        long_name: "Português (Portuguese)",
        short_name: "pt-PT",
        left_panel_size: 165.0,
    },
    Language {
        long_name: "Português Brasileiro (Brazilian Portuguese)",
        short_name: "pt-BR",
        left_panel_size: 165.0,
    },
    Language {
        long_name: "简体中文 (Simplified Chinese)",
        short_name: "zh-CN",
        left_panel_size: 115.0,
    },
    Language {
        long_name: "繁體中文 (Traditional Chinese)",
        short_name: "zh-TW",
        left_panel_size: 135.0,
    },
    Language {
        long_name: "Español (Spanish)",
        short_name: "es-ES",
        left_panel_size: 165.0,
    },
    Language {
        long_name: "Norsk (Norwegian)",
        short_name: "no",
        left_panel_size: 135.0,
    },
    Language {
        long_name: "Swedish (Svenska)",
        short_name: "sv-SE",
        left_panel_size: 130.0,
    },
    Language {
        long_name: "المملكة العربية السعودية (Saudi Arabia)",
        short_name: "ar",
        left_panel_size: 135.0,
    },
    Language {
        long_name: "България (Bulgaria)",
        short_name: "bg",
        left_panel_size: 165.0,
    },
    Language {
        long_name: "Ελλάδα (Greece)",
        short_name: "el",
        left_panel_size: 160.0,
    },
    Language {
        long_name: "Nederland (Netherlands)",
        short_name: "nl",
        left_panel_size: 165.0,
    },
    Language {
        long_name: "România (Romania)",
        short_name: "ro",
        left_panel_size: 140.0,
    },
];

pub fn connect_translations(app: &MainWindow) {
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
            error!("Error while loadings languages for {lib} {error:?}");
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

    let sort_model: [SortModel; 8] = [
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
            data: SortMode::Checked,
            name: flk!("sort_by_checked").into(),
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
    let type_of_error = flk!("column_type_of_error");
    let symlink_name = flk!("column_symlink_name");
    let symlink_folder = flk!("column_symlink_folder");
    let destination_path = flk!("column_destination_path");
    let current_extension = flk!("column_current_extension");
    let proper_extension = flk!("column_proper_extension");

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
    settings.set_similar_videos_column_name(fnm(&[&selection, &size, &file_name, &path, &mod_date]));
    settings.set_similar_music_column_name(fnm(&[&selection, &size, &file_name, &title, &artist, &year, &bitrate, &length, &genre, &path, &mod_date]));
    settings.set_invalid_symlink_column_name(fnm(&[&selection, &symlink_name, &symlink_folder, &destination_path, &mod_date]));
    settings.set_broken_files_column_name(fnm(&[&selection, &file_name, &path, &type_of_error, &size, &mod_date]));
    settings.set_bad_extensions_column_name(fnm(&[&selection, &file_name, &path, &current_extension, &proper_extension]));
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
