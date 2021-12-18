use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::{env, fs};

use czkawka_core::fl;
use directories_next::ProjectDirs;
use gtk::prelude::*;
use gtk::{ScrolledWindow, TextView};

use crate::gui_settings::GuiSettings;
use crate::gui_upper_notebook::GuiUpperNotebook;
use crate::help_functions::*;
use crate::language_functions::{get_language_from_combo_box_text, LANGUAGES_ALL};

// TODO organize this better, add specific functions that will allow to load from files specific strings
const SAVE_FILE_NAME: &str = "czkawka_gui_config.txt";

pub fn save_configuration(manual_execution: bool, upper_notebook: &GuiUpperNotebook, settings: &GuiSettings, text_view_errors: &TextView) {
    let check_button_settings_save_at_exit = settings.check_button_settings_save_at_exit.clone();
    let text_view_errors = text_view_errors.clone();

    reset_text_view(&text_view_errors);

    if !manual_execution && !check_button_settings_save_at_exit.is_active() {
        // When check button is deselected, not save configuration at exit
        return;
    }
    if let Some(proj_dirs) = ProjectDirs::from("pl", "Qarmin", "Czkawka") {
        // Lin: /home/username/.config/czkawka
        // Win: C:\Users\Username\AppData\Roaming\Qarmin\Czkawka\config
        // Mac: /Users/Username/Library/Application Support/pl.Qarmin.Czkawka

        let config_dir = proj_dirs.config_dir();
        if config_dir.exists() {
            if !config_dir.is_dir() {
                add_text_to_text_view(&text_view_errors, format!("Cannot create save file inside {} because this isn't a folder.", config_dir.display()).as_str());
                return;
            }
        } else if let Err(e) = fs::create_dir_all(config_dir) {
            add_text_to_text_view(&text_view_errors, format!("Failed configuration to create configuration folder {}, reason {}", config_dir.display(), e).as_str());
            return;
        }
        let mut data_to_save: Vec<String> = Vec::with_capacity(16);

        //// Included Directories
        data_to_save.push("--included_directories:".to_string());
        let tree_view_included_directories = upper_notebook.tree_view_included_directories.clone();
        let list_store = get_list_store(&tree_view_included_directories);
        if let Some(iter) = list_store.iter_first() {
            loop {
                data_to_save.push(list_store.value(&iter, ColumnsDirectory::Path as i32).get::<String>().unwrap());
                if !list_store.iter_next(&iter) {
                    break;
                }
            }
        }

        //// Excluded Directories
        data_to_save.push("--excluded_directories:".to_string());
        let tree_view_excluded_directories = upper_notebook.tree_view_excluded_directories.clone();
        let list_store = get_list_store(&tree_view_excluded_directories);
        if let Some(iter) = list_store.iter_first() {
            loop {
                data_to_save.push(list_store.value(&iter, ColumnsDirectory::Path as i32).get::<String>().unwrap());
                if !list_store.iter_next(&iter) {
                    break;
                }
            }
        }

        {
            //// Excluded Items
            data_to_save.push("--excluded_items:".to_string());
            let entry_excluded_items = upper_notebook.entry_excluded_items.clone();
            for item in entry_excluded_items.text().split(',') {
                if item.trim().is_empty() {
                    continue;
                }
                data_to_save.push(item.to_string());
            }

            //// Allowed extensions
            data_to_save.push("--allowed_extensions:".to_string());
            let entry_allowed_extensions = upper_notebook.entry_allowed_extensions.clone();
            for extension in entry_allowed_extensions.text().split(',') {
                if extension.trim().is_empty() {
                    continue;
                }
                data_to_save.push(extension.to_string());
            }

            //// Save at exit
            data_to_save.push("--save_at_exit:".to_string());
            let check_button_settings_save_at_exit = settings.check_button_settings_save_at_exit.clone();
            data_to_save.push(check_button_settings_save_at_exit.is_active().to_string());

            //// Load at start
            data_to_save.push("--load_at_start:".to_string());
            let check_button_settings_load_at_start = settings.check_button_settings_load_at_start.clone();
            data_to_save.push(check_button_settings_load_at_start.is_active().to_string());

            //// Confirm deletion of files
            data_to_save.push("--confirm_deletion:".to_string());
            let check_button_settings_confirm_deletion = settings.check_button_settings_confirm_deletion.clone();
            data_to_save.push(check_button_settings_confirm_deletion.is_active().to_string());

            //// Confirm deletion of all files in group
            data_to_save.push("--confirm_group_deletion:".to_string());
            let check_button_settings_confirm_group_deletion = settings.check_button_settings_confirm_group_deletion.clone();
            data_to_save.push(check_button_settings_confirm_group_deletion.is_active().to_string());

            //// Show image previews in similar images
            data_to_save.push("--show_previews_similar_images:".to_string());
            let check_button_settings_show_preview_similar_images = settings.check_button_settings_show_preview_similar_images.clone();
            data_to_save.push(check_button_settings_show_preview_similar_images.is_active().to_string());

            //// Show image previews in duplicates
            data_to_save.push("--show_previews_duplicates:".to_string());
            let check_button_settings_show_preview_duplicates = settings.check_button_settings_show_preview_duplicates.clone();
            data_to_save.push(check_button_settings_show_preview_duplicates.is_active().to_string());

            //// Show bottom text panel with errors
            data_to_save.push("--bottom_text_panel:".to_string());
            let check_button_settings_show_text_view = settings.check_button_settings_show_text_view.clone();
            data_to_save.push(check_button_settings_show_text_view.is_active().to_string());

            //// Hide/Show hard linked files, with same inodes
            data_to_save.push("--hide_hard_links:".to_string());
            let check_button_settings_hide_hard_links = settings.check_button_settings_hide_hard_links.clone();
            data_to_save.push(check_button_settings_hide_hard_links.is_active().to_string());

            //// Use cache system
            data_to_save.push("--use_cache:".to_string());
            let check_button_settings_use_cache = settings.check_button_settings_use_cache.clone();
            data_to_save.push(check_button_settings_use_cache.is_active().to_string());

            //// Delete to trash
            data_to_save.push("--use_trash:".to_string());
            let check_button_settings_use_trash = settings.check_button_settings_use_trash.clone();
            data_to_save.push(check_button_settings_use_trash.is_active().to_string());

            //// minimal cache file size
            data_to_save.push("--cache_minimal_file_size:".to_string());
            let entry_settings_cache_file_minimal_size = settings.entry_settings_cache_file_minimal_size.clone();
            data_to_save.push(entry_settings_cache_file_minimal_size.text().as_str().parse::<u64>().unwrap_or(1024 * 1024 / 4).to_string());

            //// Duplicates, delete outdated entries to trash
            data_to_save.push("--delete_outdated_entries_duplicates:".to_string());
            let check_button_settings_duplicates_delete_outdated_cache = settings.check_button_settings_duplicates_delete_outdated_cache.clone();
            data_to_save.push(check_button_settings_duplicates_delete_outdated_cache.is_active().to_string());

            //// Similar Images, delete outdated entries to trash
            data_to_save.push("--delete_outdated_entries_similar_images:".to_string());
            let check_button_settings_similar_images_delete_outdated_cache = settings.check_button_settings_similar_images_delete_outdated_cache.clone();
            data_to_save.push(check_button_settings_similar_images_delete_outdated_cache.is_active().to_string());

            //// Similar Videos, delete outdated entries to trash
            data_to_save.push("--delete_outdated_entries_similar_videos:".to_string());
            let check_button_settings_similar_videos_delete_outdated_cache = settings.check_button_settings_similar_videos_delete_outdated_cache.clone();
            data_to_save.push(check_button_settings_similar_videos_delete_outdated_cache.is_active().to_string());

            //// Use prehash cache system
            data_to_save.push("--use_prehash_cache:".to_string());
            let check_button_duplicates_use_prehash_cache = settings.check_button_duplicates_use_prehash_cache.clone();
            data_to_save.push(check_button_duplicates_use_prehash_cache.is_active().to_string());

            //// minimal prehash cache file size
            data_to_save.push("--cache_prehash_minimal_file_size:".to_string());
            let entry_settings_prehash_cache_file_minimal_size = settings.entry_settings_prehash_cache_file_minimal_size.clone();
            data_to_save.push(entry_settings_prehash_cache_file_minimal_size.text().as_str().parse::<u64>().unwrap_or(0).to_string());

            //// language
            data_to_save.push("--language:".to_string());
            let combo_box_settings_language = settings.combo_box_settings_language.clone();
            data_to_save.push(get_language_from_combo_box_text(combo_box_settings_language.active_text().unwrap().to_string()).short_text.to_string());
        }

        // Creating/Opening config file

        let config_file = config_dir.join(Path::new(SAVE_FILE_NAME));

        let mut config_file_handler = match File::create(&config_file) {
            Ok(t) => t,
            Err(e) => {
                add_text_to_text_view(&text_view_errors, format!("Failed to create config file {}, reason {}", config_dir.display(), e).as_str());
                return;
            }
        };

        let mut data_saved: bool = false;
        for data in data_to_save {
            match writeln!(config_file_handler, "{}", data) {
                Ok(_inspected) => {
                    data_saved = true;
                }
                Err(_inspected) => {
                    data_saved = false;
                    break;
                }
            }
        }
        if data_saved {
            add_text_to_text_view(&text_view_errors, format!("{} {}", fl!("saving_loading_saving_success"), config_file.display()).as_str());
        } else {
            add_text_to_text_view(&text_view_errors, format!("Failed to save configuration data to file {}", config_file.display()).as_str());
        }
    } else {
        add_text_to_text_view(&text_view_errors, "Failed to get home directory, so can't save file.");
    }
}

enum TypeOfLoadedData {
    None,
    IncludedDirectories,
    ExcludedDirectories,
    ExcludedItems,
    AllowedExtensions,
    LoadingAtStart,
    SavingAtExit,
    ConfirmDeletion,
    ConfirmGroupDeletion,
    ShowPreviewSimilarImages,
    ShowPreviewDuplicates,
    BottomTextPanel,
    HideHardLinks,
    UseCache,
    UseTrash,
    CacheMinimalSize,
    DeleteCacheDuplicates,
    DeleteCacheSimilarImages,
    DeleteCacheSimilarVideos,
    UsePrehashCache,
    CachePrehashMinimalSize,
    Language,
}

pub fn load_configuration(manual_execution: bool, upper_notebook: &GuiUpperNotebook, settings: &GuiSettings, text_view_errors: &TextView, scrolled_window_errors: &ScrolledWindow) {
    let text_view_errors = text_view_errors.clone();

    reset_text_view(&text_view_errors);

    if let Some(proj_dirs) = ProjectDirs::from("pl", "Qarmin", "Czkawka") {
        // Lin: /home/username/.config/czkawka
        // Win: C:\Users\Username\AppData\Roaming\Qarmin\Czkawka\config
        // Mac: /Users/Username/Library/Application Support/pl.Qarmin.Czkawka

        let config_dir = proj_dirs.config_dir();
        let config_file = config_dir.join(Path::new(SAVE_FILE_NAME));
        if !config_file.exists() || !config_file.is_file() {
            if manual_execution {
                // Don't show errors when there is no configuration file when starting app
                add_text_to_text_view(&text_view_errors, format!("Cannot load configuration from file {:?}.", config_file.display()).as_str());
            }
            return;
        }

        // Loading Data
        let loaded_data: String = match fs::read_to_string(&config_file) {
            Ok(t) => t,
            Err(e) => {
                add_text_to_text_view(&text_view_errors, format!("Failed to read data from file {:?}, reason {}", config_file, e).as_str());
                return;
            }
        };

        let mut short_language: String;

        // Load here language, default system language could change value in settings so we don't want to lose this value
        {
            short_language = get_language_from_combo_box_text(settings.combo_box_settings_language.active_text().unwrap().to_string()).short_text.to_string();
        }

        // Parsing Data - this are default values

        let mut included_directories: Vec<String> = Vec::new();
        let mut excluded_directories: Vec<String> = Vec::new();
        let mut excluded_items: Vec<String> = Vec::new();
        let mut allowed_extensions: Vec<String> = Vec::new();
        let mut loading_at_start: bool = true;
        let mut saving_at_exit: bool = true;
        let mut confirm_deletion: bool = true;
        let mut confirm_group_deletion: bool = true;
        let mut show_previews_similar_images: bool = true;
        let mut show_previews_duplicates: bool = true;
        let mut bottom_text_panel: bool = true;
        let mut hide_hard_links: bool = true;
        let mut use_cache: bool = true;
        let mut use_trash: bool = false;
        let mut cache_minimal_size: u64 = 2 * 1024 * 1024;
        let mut delete_outdated_cache_dupliactes: bool = true;
        let mut delete_outdated_cache_similar_images: bool = true;
        let mut delete_outdated_cache_similar_videos: bool = false;
        let mut use_prehash_cache: bool = false;
        let mut cache_prehash_minimal_size: u64 = 0;

        let mut current_type = TypeOfLoadedData::None;
        for (line_number, line) in loaded_data.replace("\r\n", "\n").split('\n').enumerate() {
            let line: String = line.trim().to_string();
            if line.is_empty() {
                continue; // Empty line, so we just skip it
            }
            if line.starts_with("--included_directories") {
                current_type = TypeOfLoadedData::IncludedDirectories;
            } else if line.starts_with("--excluded_directories") {
                current_type = TypeOfLoadedData::ExcludedDirectories;
            } else if line.starts_with("--excluded_items") {
                current_type = TypeOfLoadedData::ExcludedItems;
            } else if line.starts_with("--allowed_extensions") {
                current_type = TypeOfLoadedData::AllowedExtensions;
            } else if line.starts_with("--load_at_start") {
                current_type = TypeOfLoadedData::LoadingAtStart;
            } else if line.starts_with("--save_at_exit") {
                current_type = TypeOfLoadedData::SavingAtExit;
            } else if line.starts_with("--confirm_deletion") {
                current_type = TypeOfLoadedData::ConfirmDeletion;
            } else if line.starts_with("--confirm_group_deletion") {
                current_type = TypeOfLoadedData::ConfirmGroupDeletion;
            } else if line.starts_with("--show_previews_similar_images") {
                current_type = TypeOfLoadedData::ShowPreviewSimilarImages;
            } else if line.starts_with("--show_previews_duplicates") {
                current_type = TypeOfLoadedData::ShowPreviewDuplicates;
            } else if line.starts_with("--bottom_text_panel") {
                current_type = TypeOfLoadedData::BottomTextPanel;
            } else if line.starts_with("--hide_hard_links") {
                current_type = TypeOfLoadedData::HideHardLinks;
            } else if line.starts_with("--use_cache") {
                current_type = TypeOfLoadedData::UseCache;
            } else if line.starts_with("--use_trash") {
                current_type = TypeOfLoadedData::UseTrash;
            } else if line.starts_with("--cache_minimal_file_size") {
                current_type = TypeOfLoadedData::CacheMinimalSize;
            } else if line.starts_with("--delete_outdated_entries_duplicates") {
                current_type = TypeOfLoadedData::DeleteCacheDuplicates;
            } else if line.starts_with("--delete_outdated_entries_similar_videos") {
                current_type = TypeOfLoadedData::DeleteCacheSimilarVideos;
            } else if line.starts_with("--delete_outdated_entries_similar_images") {
                current_type = TypeOfLoadedData::DeleteCacheSimilarImages;
            } else if line.starts_with("--use_prehash_cache") {
                current_type = TypeOfLoadedData::UsePrehashCache;
            } else if line.starts_with("--cache_prehash_minimal_file_size") {
                current_type = TypeOfLoadedData::CachePrehashMinimalSize;
            } else if line.starts_with("--language") {
                current_type = TypeOfLoadedData::Language;
            } else if line.starts_with("--") {
                current_type = TypeOfLoadedData::None;
                add_text_to_text_view(
                    &text_view_errors,
                    format!("Found invalid header in line {} \"{}\" when loading file {:?} (save file may be from different Czkawka version)", line_number, line, config_file).as_str(),
                );
            } else {
                match current_type {
                    TypeOfLoadedData::None => {
                        add_text_to_text_view(
                            &text_view_errors,
                            format!("Found orphan data in line {} \"{}\" when loading file {:?} (save file may be from different Czkawka version)", line_number, line, config_file).as_str(),
                        );
                    }
                    TypeOfLoadedData::IncludedDirectories => {
                        included_directories.push(line);
                    }
                    TypeOfLoadedData::ExcludedDirectories => {
                        excluded_directories.push(line);
                    }
                    TypeOfLoadedData::ExcludedItems => {
                        excluded_items.push(line);
                    }
                    TypeOfLoadedData::AllowedExtensions => {
                        allowed_extensions.push(line);
                    }
                    TypeOfLoadedData::LoadingAtStart => {
                        let line = line.to_lowercase();
                        if line == "1" || line == "true" {
                            loading_at_start = true;
                        } else if line == "0" || line == "false" {
                            loading_at_start = false;
                        } else {
                            add_text_to_text_view(
                                &text_view_errors,
                                format!("Found invalid data in line {} \"{}\" isn't proper value(0/1/true/false) when loading file {:?}", line_number, line, config_file).as_str(),
                            );
                        }
                    }
                    TypeOfLoadedData::SavingAtExit => {
                        let line = line.to_lowercase();
                        if line == "1" || line == "true" {
                            saving_at_exit = true;
                        } else if line == "0" || line == "false" {
                            saving_at_exit = false;
                        } else {
                            add_text_to_text_view(
                                &text_view_errors,
                                format!("Found invalid data in line {} \"{}\" isn't proper value(0/1/true/false) when loading file {:?}", line_number, line, config_file).as_str(),
                            );
                        }
                    }
                    TypeOfLoadedData::ConfirmDeletion => {
                        let line = line.to_lowercase();
                        if line == "1" || line == "true" {
                            confirm_deletion = true;
                        } else if line == "0" || line == "false" {
                            confirm_deletion = false;
                        } else {
                            add_text_to_text_view(
                                &text_view_errors,
                                format!("Found invalid data in line {} \"{}\" isn't proper value(0/1/true/false) when loading file {:?}", line_number, line, config_file).as_str(),
                            );
                        }
                    }
                    TypeOfLoadedData::ConfirmGroupDeletion => {
                        let line = line.to_lowercase();
                        if line == "1" || line == "true" {
                            confirm_group_deletion = true;
                        } else if line == "0" || line == "false" {
                            confirm_group_deletion = false;
                        } else {
                            add_text_to_text_view(
                                &text_view_errors,
                                format!("Found invalid data in line {} \"{}\" isn't proper value(0/1/true/false) when loading file {:?}", line_number, line, config_file).as_str(),
                            );
                        }
                    }
                    TypeOfLoadedData::ShowPreviewSimilarImages => {
                        let line = line.to_lowercase();
                        if line == "1" || line == "true" {
                            show_previews_similar_images = true;
                        } else if line == "0" || line == "false" {
                            show_previews_similar_images = false;
                        } else {
                            add_text_to_text_view(
                                &text_view_errors,
                                format!("Found invalid data in line {} \"{}\" isn't proper value(0/1/true/false) when loading file {:?}", line_number, line, config_file).as_str(),
                            );
                        }
                    }
                    TypeOfLoadedData::ShowPreviewDuplicates => {
                        let line = line.to_lowercase();
                        if line == "1" || line == "true" {
                            show_previews_duplicates = true;
                        } else if line == "0" || line == "false" {
                            show_previews_duplicates = false;
                        } else {
                            add_text_to_text_view(
                                &text_view_errors,
                                format!("Found invalid data in line {} \"{}\" isn't proper value(0/1/true/false) when loading file {:?}", line_number, line, config_file).as_str(),
                            );
                        }
                    }
                    TypeOfLoadedData::BottomTextPanel => {
                        let line = line.to_lowercase();
                        if line == "1" || line == "true" {
                            bottom_text_panel = true;
                        } else if line == "0" || line == "false" {
                            bottom_text_panel = false;
                        } else {
                            add_text_to_text_view(
                                &text_view_errors,
                                format!("Found invalid data in line {} \"{}\" isn't proper value(0/1/true/false) when loading file {:?}", line_number, line, config_file).as_str(),
                            );
                        }
                    }
                    TypeOfLoadedData::HideHardLinks => {
                        let line = line.to_lowercase();
                        if line == "1" || line == "true" {
                            hide_hard_links = true;
                        } else if line == "0" || line == "false" {
                            hide_hard_links = false;
                        } else {
                            add_text_to_text_view(
                                &text_view_errors,
                                format!("Found invalid data in line {} \"{}\" isn't proper value(0/1/true/false) when loading file {:?}", line_number, line, config_file).as_str(),
                            );
                        }
                    }
                    TypeOfLoadedData::UseCache => {
                        let line = line.to_lowercase();
                        if line == "1" || line == "true" {
                            use_cache = true;
                        } else if line == "0" || line == "false" {
                            use_cache = false;
                        } else {
                            add_text_to_text_view(
                                &text_view_errors,
                                format!("Found invalid data in line {} \"{}\" isn't proper value(0/1/true/false) when loading file {:?}", line_number, line, config_file).as_str(),
                            );
                        }
                    }
                    TypeOfLoadedData::UseTrash => {
                        let line = line.to_lowercase();
                        if line == "1" || line == "true" {
                            use_trash = true;
                        } else if line == "0" || line == "false" {
                            use_trash = false;
                        } else {
                            add_text_to_text_view(
                                &text_view_errors,
                                format!("Found invalid data in line {} \"{}\" isn't proper value(0/1/true/false) when loading file {:?}", line_number, line, config_file).as_str(),
                            );
                        }
                    }
                    TypeOfLoadedData::CacheMinimalSize => {
                        if let Ok(number) = line.parse::<u64>() {
                            cache_minimal_size = number;
                        } else {
                            add_text_to_text_view(
                                &text_view_errors,
                                format!("Found invalid data in line {} \"{}\" isn't proper value(u64) when loading file {:?}", line_number, line, config_file).as_str(),
                            );
                        }
                    }
                    TypeOfLoadedData::DeleteCacheDuplicates => {
                        let line = line.to_lowercase();
                        if line == "1" || line == "true" {
                            delete_outdated_cache_dupliactes = true;
                        } else if line == "0" || line == "false" {
                            delete_outdated_cache_dupliactes = false;
                        } else {
                            add_text_to_text_view(
                                &text_view_errors,
                                format!("Found invalid data in line {} \"{}\" isn't proper value(0/1/true/false) when loading file {:?}", line_number, line, config_file).as_str(),
                            );
                        }
                    }
                    TypeOfLoadedData::DeleteCacheSimilarImages => {
                        let line = line.to_lowercase();
                        if line == "1" || line == "true" {
                            delete_outdated_cache_similar_images = true;
                        } else if line == "0" || line == "false" {
                            delete_outdated_cache_similar_images = false;
                        } else {
                            add_text_to_text_view(
                                &text_view_errors,
                                format!("Found invalid data in line {} \"{}\" isn't proper value(0/1/true/false) when loading file {:?}", line_number, line, config_file).as_str(),
                            );
                        }
                    }
                    TypeOfLoadedData::DeleteCacheSimilarVideos => {
                        let line = line.to_lowercase();
                        if line == "1" || line == "true" {
                            delete_outdated_cache_similar_videos = true;
                        } else if line == "0" || line == "false" {
                            delete_outdated_cache_similar_videos = false;
                        } else {
                            add_text_to_text_view(
                                &text_view_errors,
                                format!("Found invalid data in line {} \"{}\" isn't proper value(0/1/true/false) when loading file {:?}", line_number, line, config_file).as_str(),
                            );
                        }
                    }
                    TypeOfLoadedData::UsePrehashCache => {
                        let line = line.to_lowercase();
                        if line == "1" || line == "true" {
                            use_prehash_cache = true;
                        } else if line == "0" || line == "false" {
                            use_prehash_cache = false;
                        } else {
                            add_text_to_text_view(
                                &text_view_errors,
                                format!("Found invalid data in line {} \"{}\" isn't proper value(0/1/true/false) when loading file {:?}", line_number, line, config_file).as_str(),
                            );
                        }
                    }
                    TypeOfLoadedData::CachePrehashMinimalSize => {
                        if let Ok(number) = line.parse::<u64>() {
                            cache_prehash_minimal_size = number;
                        } else {
                            add_text_to_text_view(
                                &text_view_errors,
                                format!("Found invalid data in line {} \"{}\" isn't proper value(u64) when loading file {:?}", line_number, line, config_file).as_str(),
                            );
                        }
                    }
                    TypeOfLoadedData::Language => {
                        if LANGUAGES_ALL.iter().any(|e| e.short_text == line) {
                            short_language = line;
                        } else {
                            add_text_to_text_view(
                                &text_view_errors,
                                format!("Found invalid data in line {} \"{}\" isn't proper language value when loading file {:?}", line_number, line, config_file).as_str(),
                            );
                        }
                    }
                }
            }
        }

        // Setting data
        if manual_execution || loading_at_start {
            //// Included Directories
            let tree_view_included_directories = upper_notebook.tree_view_included_directories.clone();
            let list_store = get_list_store(&tree_view_included_directories);
            list_store.clear();

            for directory in included_directories {
                let values: [(u32, &dyn ToValue); 1] = [(ColumnsDirectory::Path as u32, &directory)];
                list_store.set(&list_store.append(), &values);
            }

            //// Exclude Directories
            let tree_view_excluded_directories = upper_notebook.tree_view_excluded_directories.clone();
            let list_store = get_list_store(&tree_view_excluded_directories);
            list_store.clear();

            for directory in excluded_directories {
                let values: [(u32, &dyn ToValue); 1] = [(ColumnsDirectory::Path as u32, &directory)];
                list_store.set(&list_store.append(), &values);
            }

            //// Excluded Items
            let entry_excluded_items = upper_notebook.entry_excluded_items.clone();
            entry_excluded_items.set_text(excluded_items.iter().map(|e| e.to_string() + ",").collect::<String>().as_str());

            //// Allowed extensions
            let entry_allowed_extensions = upper_notebook.entry_allowed_extensions.clone();
            entry_allowed_extensions.set_text(allowed_extensions.iter().map(|e| e.to_string() + ",").collect::<String>().as_str());

            //// ComboText
            {
                for (index, lang) in LANGUAGES_ALL.iter().enumerate() {
                    if short_language == lang.short_text {
                        settings.combo_box_settings_language.set_active(Some(index as u32));
                    }
                }
            }

            //// Buttons
            settings.check_button_settings_load_at_start.set_active(loading_at_start);
            settings.check_button_settings_save_at_exit.set_active(saving_at_exit);
            settings.check_button_settings_confirm_deletion.set_active(confirm_deletion);
            settings.check_button_settings_confirm_group_deletion.set_active(confirm_group_deletion);
            settings.check_button_settings_show_preview_similar_images.set_active(show_previews_similar_images);
            settings.check_button_settings_show_preview_duplicates.set_active(show_previews_duplicates);

            settings.check_button_settings_similar_videos_delete_outdated_cache.set_active(delete_outdated_cache_similar_videos);
            settings.check_button_settings_similar_images_delete_outdated_cache.set_active(delete_outdated_cache_similar_images);
            settings.check_button_settings_duplicates_delete_outdated_cache.set_active(delete_outdated_cache_dupliactes);

            settings.check_button_settings_show_text_view.set_active(bottom_text_panel);
            if !bottom_text_panel {
                scrolled_window_errors.hide();
            } else {
                scrolled_window_errors.show();
            }
            settings.check_button_settings_hide_hard_links.set_active(hide_hard_links);
            settings.check_button_settings_use_cache.set_active(use_cache);
            settings.check_button_duplicates_use_prehash_cache.set_active(use_prehash_cache);
            settings.check_button_settings_use_trash.set_active(use_trash);
            settings.entry_settings_cache_file_minimal_size.set_text(cache_minimal_size.to_string().as_str());
            settings.entry_settings_prehash_cache_file_minimal_size.set_text(cache_prehash_minimal_size.to_string().as_str());
        } else {
            settings.check_button_settings_load_at_start.set_active(false);
        }

        if manual_execution {
            add_text_to_text_view(&text_view_errors, format!("{} {:?}", &fl!("saving_loading_reset_configuration"), config_file).as_str());
        }
    } else {
        add_text_to_text_view(&text_view_errors, "Failed to get home directory, so can't load file.");
    }
}

pub fn reset_configuration(manual_clearing: bool, upper_notebook: &GuiUpperNotebook, settings: &GuiSettings, text_view_errors: &TextView) {
    // TODO Maybe add popup dialog to confirm resetting
    let text_view_errors = text_view_errors.clone();

    reset_text_view(&text_view_errors);

    // Resetting included directories
    {
        let tree_view_included_directories = upper_notebook.tree_view_included_directories.clone();
        let list_store = get_list_store(&tree_view_included_directories);
        list_store.clear();

        let current_dir: String = match env::current_dir() {
            Ok(t) => t.to_str().unwrap().to_string(),
            Err(_inspected) => {
                if cfg!(target_family = "unix") {
                    add_text_to_text_view(&text_view_errors, "Failed to read current directory, setting /home instead");
                    "/home".to_string()
                } else if cfg!(target_family = "windows") {
                    add_text_to_text_view(&text_view_errors, "Failed to read current directory, setting C:\\ instead");
                    "C:\\".to_string()
                } else {
                    "".to_string()
                }
            }
        };

        let values: [(u32, &dyn ToValue); 1] = [(ColumnsDirectory::Path as u32, &current_dir)];
        list_store.set(&list_store.append(), &values);
    }
    // Resetting excluded directories
    {
        let tree_view_excluded_directories = upper_notebook.tree_view_excluded_directories.clone();
        let list_store = get_list_store(&tree_view_excluded_directories);
        list_store.clear();
        if cfg!(target_family = "unix") {
            for i in ["/proc", "/dev", "/sys", "/run", "/snap"].iter() {
                let values: [(u32, &dyn ToValue); 1] = [(ColumnsDirectory::Path as u32, &i)];
                list_store.set(&list_store.append(), &values);
            }
        }
    }
    // Resetting excluded items
    {
        let entry_excluded_items = upper_notebook.entry_excluded_items.clone();
        if cfg!(target_family = "unix") {
            entry_excluded_items.set_text("*/.git/*,*/node_modules/*,*/lost+found/*,*/Trash/*,*/.Trash-*/*,*/snap/*,/home/*/.cache/*");
        }
        if cfg!(target_family = "windows") {
            entry_excluded_items.set_text("*\\.git\\*,*\\node_modules\\*,*\\lost+found\\*,*:\\windows\\*");
        }
    }
    // Resetting allowed extensions
    {
        let entry_allowed_extensions = upper_notebook.entry_allowed_extensions.clone();
        entry_allowed_extensions.set_text("");
    }

    // Set default settings
    {
        settings.check_button_settings_save_at_exit.set_active(true);
        settings.check_button_settings_load_at_start.set_active(true);
        settings.check_button_settings_confirm_deletion.set_active(true);
        settings.check_button_settings_confirm_group_deletion.set_active(true);
        settings.check_button_settings_show_preview_similar_images.set_active(true);
        settings.check_button_settings_show_preview_duplicates.set_active(true);
        settings.check_button_settings_show_text_view.set_active(true);
        settings.check_button_settings_hide_hard_links.set_active(true);
        settings.check_button_settings_use_cache.set_active(true);
        settings.check_button_settings_use_trash.set_active(false);
        settings.entry_settings_cache_file_minimal_size.set_text("257144");
        settings.check_button_settings_similar_videos_delete_outdated_cache.set_active(false);
        settings.check_button_settings_similar_images_delete_outdated_cache.set_active(true);
        settings.check_button_settings_duplicates_delete_outdated_cache.set_active(true);
        settings.check_button_duplicates_use_prehash_cache.set_active(false);
        settings.entry_settings_prehash_cache_file_minimal_size.set_text("0");
        settings.combo_box_settings_language.set_active(Some(0));
    }
    if manual_clearing {
        add_text_to_text_view(&text_view_errors, &fl!("saving_loading_reset_configuration"));
    }
}
