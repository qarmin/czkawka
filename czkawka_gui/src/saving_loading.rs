use std::collections::HashMap;
use std::ffi::OsString;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::{env, fs};

use directories_next::ProjectDirs;
use gtk4::prelude::*;
use gtk4::{ComboBoxText, ScrolledWindow, TextView};

use czkawka_core::common_dir_traversal::CheckingMethod;
use czkawka_core::similar_images::SIMILAR_VALUES;

use crate::flg;
use crate::gui_structs::gui_main_notebook::GuiMainNotebook;
use crate::gui_structs::gui_settings::GuiSettings;
use crate::gui_structs::gui_upper_notebook::GuiUpperNotebook;
use crate::help_combo_box::DUPLICATES_CHECK_METHOD_COMBO_BOX;
use crate::help_functions::*;
use crate::language_functions::{get_language_from_combo_box_text, LANGUAGES_ALL};
use crate::localizer_core::generate_translation_hashmap;

const SAVE_FILE_NAME: &str = "czkawka_gui_config_4.txt";

const DEFAULT_SAVE_ON_EXIT: bool = true;
const DEFAULT_LOAD_AT_START: bool = true;
const DEFAULT_CONFIRM_DELETION: bool = true;
const DEFAULT_CONFIRM_GROUP_DELETION: bool = true;
const DEFAULT_SHOW_IMAGE_PREVIEW: bool = true;
const DEFAULT_SHOW_DUPLICATE_IMAGE_PREVIEW: bool = true;
const DEFAULT_BOTTOM_TEXT_VIEW: bool = true;
const DEFAULT_USE_CACHE: bool = true;
const DEFAULT_SAVE_ALSO_AS_JSON: bool = false;
const DEFAULT_HIDE_HARD_LINKS: bool = true;
const DEFAULT_USE_PRECACHE: bool = false;
const DEFAULT_USE_TRASH: bool = false;
pub const DEFAULT_MINIMAL_CACHE_SIZE: &str = "257144";
const DEFAULT_PREHASH_MINIMAL_CACHE_SIZE: &str = "0";
const DEFAULT_VIDEO_REMOVE_AUTO_OUTDATED_CACHE: bool = false;
const DEFAULT_IMAGE_REMOVE_AUTO_OUTDATED_CACHE: bool = true;
const DEFAULT_DUPLICATE_REMOVE_AUTO_OUTDATED_CACHE: bool = true;
const DEFAULT_DUPLICATE_CASE_SENSITIVE_NAME_CHECKING: bool = false;

const DEFAULT_NUMBER_OF_BIGGEST_FILES: &str = "50";
const DEFAULT_SIMILAR_IMAGES_SIMILARITY: i32 = 0;
const DEFAULT_SIMILAR_IMAGES_IGNORE_SAME_SIZE: bool = false;
const DEFAULT_SIMILAR_IMAGES_FAST_COMPARE: bool = false;
const DEFAULT_SIMILAR_VIDEOS_SIMILARITY: i32 = 15;
const DEFAULT_SIMILAR_VIDEOS_IGNORE_SAME_SIZE: bool = false;

pub const DEFAULT_MINIMAL_FILE_SIZE: &str = "16384";
pub const DEFAULT_MAXIMAL_FILE_SIZE: &str = "999999999999";

#[cfg(target_family = "unix")]
const DEFAULT_EXCLUDED_ITEMS: &str = "*/.git/*,*/node_modules/*,*/lost+found/*,*/Trash/*,*/.Trash-*/*,*/snap/*,/home/*/.cache/*";
#[cfg(not(target_family = "unix"))]
const DEFAULT_EXCLUDED_ITEMS: &str = "*\\.git\\*,*\\node_modules\\*,*\\lost+found\\*,*:\\windows\\*";

#[cfg(target_family = "unix")]
const DEFAULT_EXCLUDED_DIRECTORIES: &[&str] = &["/proc", "/dev", "/sys", "/run", "/snap"];
#[cfg(not(target_family = "unix"))]
const DEFAULT_EXCLUDED_DIRECTORIES: &[&str] = &["C:\\Windows"];

struct LoadSaveStruct {
    loaded_items: HashMap<String, Vec<String>>,
    text_view: TextView,
}

impl LoadSaveStruct {
    pub fn with_text_view(text_view: TextView) -> Self {
        Self {
            loaded_items: Default::default(),
            text_view,
        }
    }

    pub fn get_vector_string(&self, key: String, default_value: Vec<String>) -> Vec<String> {
        if self.loaded_items.contains_key(&key) {
            let mut new_vector = Vec::new();
            for i in self.loaded_items.get(&key).unwrap() {
                if !i.trim().is_empty() {
                    new_vector.push(i.trim().to_string());
                }
            }
            return new_vector;
        }

        default_value
    }
    pub fn get_integer_string(&self, key: String, default_value: String) -> String {
        if default_value.parse::<i64>().is_err() {
            println!("Default value {} can't be convert to integer value", default_value);
            panic!();
        }
        assert!(default_value.parse::<i64>().is_ok());
        let mut returned_value = self.get_string(key, default_value.clone());
        if returned_value.parse::<i64>().is_err() {
            returned_value = default_value;
        }
        returned_value
    }
    pub fn get_string(&self, key: String, default_value: String) -> String {
        if self.loaded_items.contains_key(&key) {
            let item = self.loaded_items.get(&key).unwrap().clone().into_iter().filter(|e| !e.is_empty()).collect::<Vec<String>>();
            return if item.len() == 1 {
                item[0].clone()
            } else if item.is_empty() {
                "".to_string()
            } else {
                add_text_to_text_view(
                    &self.text_view,
                    &flg!(
                        "saving_loading_invalid_string",
                        generate_translation_hashmap(vec![("key", key), ("result", format!("{:?}", item))])
                    ),
                );
                default_value
            };
        }

        default_value
    }
    pub fn get_integer<T: std::str::FromStr>(&self, key: String, default_value: T) -> T {
        if self.loaded_items.contains_key(&key) {
            let item = self.loaded_items.get(&key).unwrap().clone().into_iter().filter(|e| !e.is_empty()).collect::<Vec<String>>();

            return if item.len() == 1 {
                match item[0].parse::<T>() {
                    Ok(t) => t,
                    Err(_) => {
                        println!("Failed to decode integer from \"{}\", found {:?}", key, item[0]);
                        default_value
                    }
                }
            } else {
                add_text_to_text_view(
                    &self.text_view,
                    &flg!(
                        "saving_loading_invalid_int",
                        generate_translation_hashmap(vec![("key", key), ("result", format!("{:?}", item))])
                    ),
                );
                default_value
            };
        }

        default_value
    }
    pub fn get_bool(&self, key: String, default_value: bool) -> bool {
        if self.loaded_items.contains_key(&key) {
            let item = self.loaded_items.get(&key).unwrap().clone().into_iter().filter(|e| !e.is_empty()).collect::<Vec<String>>();
            return if item.len() == 1 {
                let text = item[0].clone().trim().to_lowercase();
                if text == "false" || text == "0" {
                    false
                } else if text == "true" || text == "1" {
                    true
                } else {
                    add_text_to_text_view(
                        &self.text_view,
                        &flg!(
                            "saving_loading_decode_problem_bool",
                            generate_translation_hashmap(vec![("key", key), ("result", item[0].to_string())])
                        ),
                    );
                    default_value
                }
            } else {
                add_text_to_text_view(
                    &self.text_view,
                    &flg!(
                        "saving_loading_invalid_bool",
                        generate_translation_hashmap(vec![("key", key), ("result", format!("{:?}", item))])
                    ),
                );
                default_value
            };
        }

        default_value
    }

    // Bool, int, string
    pub fn save_var<T: ToString>(&mut self, key: String, value: T) {
        if self.loaded_items.contains_key(&key) {
            add_text_to_text_view(
                &self.text_view,
                &flg!("saving_loading_saving_same_keys", generate_translation_hashmap(vec![("key", key.clone())])),
            );
        }

        self.loaded_items.insert(key, vec![value.to_string()]);
    }

    pub fn save_list_store(&mut self, key: String, tree_view: &gtk4::TreeView, column_path: i32) {
        let mut vec_string = vec![];
        let list_store = get_list_store(tree_view);
        if let Some(iter) = list_store.iter_first() {
            loop {
                // TODO maybe save also here reference directories?
                vec_string.push(list_store.get::<String>(&iter, column_path));
                if !list_store.iter_next(&iter) {
                    break;
                }
            }
        }
        self.loaded_items.insert(key, vec_string);
    }

    pub fn open_save_file(&self, text_view_errors: &TextView, save_configuration: bool, manual_execution: bool) -> Option<(File, PathBuf)> {
        if let Some(proj_dirs) = ProjectDirs::from("pl", "Qarmin", "Czkawka") {
            // Lin: /home/username/.config/czkawka
            // Win: C:\Users\Username\AppData\Roaming\Qarmin\Czkawka\config
            // Mac: /Users/Username/Library/Application Support/pl.Qarmin.Czkawka

            let config_dir = proj_dirs.config_dir();
            let config_file = config_dir.join(Path::new(SAVE_FILE_NAME));

            if save_configuration {
                if config_dir.exists() {
                    if !config_dir.is_dir() {
                        add_text_to_text_view(
                            text_view_errors,
                            &flg!(
                                "saving_loading_folder_config_instead_file",
                                generate_translation_hashmap(vec![("path", config_dir.display().to_string())])
                            ),
                        );
                        return None;
                    }
                } else if let Err(e) = fs::create_dir_all(config_dir) {
                    add_text_to_text_view(
                        text_view_errors,
                        &flg!(
                            "saving_loading_failed_to_create_configuration_folder",
                            generate_translation_hashmap(vec![("path", config_dir.display().to_string()), ("reason", e.to_string())])
                        ),
                    );
                    return None;
                }

                let config_file_handler = match File::create(&config_file) {
                    Ok(t) => t,
                    Err(e) => {
                        add_text_to_text_view(
                            text_view_errors,
                            &flg!(
                                "saving_loading_failed_to_create_config_file",
                                generate_translation_hashmap(vec![("path", config_file.display().to_string()), ("reason", e.to_string())])
                            ),
                        );
                        return None;
                    }
                };
                return Some((config_file_handler, config_file));
            } else {
                if !config_file.exists() || !config_file.is_file() {
                    if manual_execution {
                        // Don't show errors when there is no configuration file when starting app
                        add_text_to_text_view(
                            text_view_errors,
                            &flg!(
                                "saving_loading_failed_to_read_config_file",
                                generate_translation_hashmap(vec![("path", config_file.display().to_string())])
                            ),
                        );
                    }
                    return None;
                }

                let config_file_handler = match File::open(&config_file) {
                    Ok(t) => t,
                    Err(e) => {
                        add_text_to_text_view(
                            text_view_errors,
                            &flg!(
                                "saving_loading_failed_to_create_config_file",
                                generate_translation_hashmap(vec![("path", config_file.display().to_string()), ("reason", e.to_string())])
                            ),
                        );
                        return None;
                    }
                };
                return Some((config_file_handler, config_file));
            }
        } else {
            add_text_to_text_view(text_view_errors, flg!("saving_loading_failed_to_get_home_directory").as_str());
        }
        None
    }

    pub fn open_and_read_content(&mut self, text_view_errors: &TextView, manual_execution: bool) {
        if let Some((mut config_file_handler, config_file)) = self.open_save_file(text_view_errors, false, manual_execution) {
            let mut loaded_data: String = String::new();
            if let Err(e) = config_file_handler.read_to_string(&mut loaded_data) {
                add_text_to_text_view(
                    text_view_errors,
                    &flg!(
                        "saving_loading_failed_to_read_data_from_file",
                        generate_translation_hashmap(vec![("path", config_file.display().to_string()), ("reason", e.to_string())])
                    ),
                );
                return;
            }

            let mut header: String = "".to_string();
            let lines: Vec<String> = loaded_data.replace('\r', "").split('\n').map(String::from).collect::<Vec<String>>();
            for (index, line) in lines.iter().enumerate() {
                let line = line.trim();
                if line.starts_with("--") {
                    header = line.to_string();
                } else if !header.is_empty() {
                    self.loaded_items.entry(header.clone()).or_insert_with(Vec::new);
                    self.loaded_items.get_mut(&header).unwrap().push(line.to_string());
                } else {
                    add_text_to_text_view(
                        text_view_errors,
                        &flg!(
                            "saving_loading_orphan_data",
                            generate_translation_hashmap(vec![("data", line.to_string()), ("index", index.to_string())])
                        ),
                    );
                }
            }

            let (_, hashmap_sl) = create_hash_map();
            for setting in self.loaded_items.keys() {
                if !hashmap_sl.contains_key(setting) {
                    add_text_to_text_view(
                        text_view_errors,
                        &flg!("saving_loading_not_valid", generate_translation_hashmap(vec![("data", setting.to_string())])),
                    );
                }
            }

            if manual_execution {
                add_text_to_text_view(text_view_errors, &flg!("saving_loading_loading_success"));
            }
        }
    }

    pub fn save_to_file(&self, text_view_errors: &TextView) {
        if let Some((mut config_file_handler, config_file)) = self.open_save_file(text_view_errors, true, false) {
            let mut data_saved: bool = false;
            for (key, vec_string) in &self.loaded_items {
                match writeln!(config_file_handler, "{}", key) {
                    Ok(_inspected) => {
                        data_saved = true;
                    }
                    Err(_inspected) => {
                        data_saved = false;
                        break;
                    }
                }
                for data in vec_string {
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
            }
            if data_saved {
                add_text_to_text_view(
                    text_view_errors,
                    flg!(
                        "saving_loading_saving_success",
                        generate_translation_hashmap(vec![("name", config_file.display().to_string())])
                    )
                    .as_str(),
                );
            } else {
                add_text_to_text_view(
                    text_view_errors,
                    flg!(
                        "saving_loading_saving_failure",
                        generate_translation_hashmap(vec![("name", config_file.display().to_string())])
                    )
                    .as_str(),
                );
            }
        }
    }
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
enum LoadText {
    IncludedDirectories,
    ExcludedDirectories,
    ExcludedItems,
    AllowedExtensions,
    MinimalFileSize,
    MaximalFileSize,
    SaveAtExit,
    LoadAtStart,
    ConfirmDeletionFiles,
    ConfirmDeletionAllFilesInGroup,
    ShowBottomTextPanel,
    HideHardLinks,
    UseCache,
    UseJsonCacheFile,
    DeleteToTrash,
    MinimalCacheSize,
    ImagePreviewImage,
    DuplicatePreviewImage,
    DuplicateDeleteOutdatedCacheEntries,
    ImageDeleteOutdatedCacheEntries,
    VideoDeleteOutdatedCacheEntries,
    UsePrehashCache,
    MinimalPrehashCacheSize,
    Language,
    ComboBoxDuplicateHashType,
    ComboBoxDuplicateCheckMethod,
    ComboBoxImageResizeAlgorithm,
    ComboBoxImageHashType,
    ComboBoxImageHashSize,
    NumberOfBiggestFiles,
    SimilarImagesSimilarity,
    SimilarImagesIgnoreSameSize,
    SimilarImagesFastCompare,
    SimilarVideosSimilarity,
    SimilarVideosIgnoreSameSize,
    MusicApproximateComparison,
    DuplicateNameCaseSensitive,
}

fn create_hash_map() -> (HashMap<LoadText, String>, HashMap<String, LoadText>) {
    let values = [
        (LoadText::IncludedDirectories, "included_directories"),
        (LoadText::ExcludedDirectories, "excluded_directories"),
        (LoadText::ExcludedItems, "excluded_items"),
        (LoadText::AllowedExtensions, "allowed_extensions"),
        (LoadText::MinimalFileSize, "minimal_file_size"),
        (LoadText::MaximalFileSize, "maximal_file_size"),
        (LoadText::SaveAtExit, "save_at_exit"),
        (LoadText::LoadAtStart, "load_at_start"),
        (LoadText::ConfirmDeletionFiles, "confirm_deletion_files"),
        (LoadText::ConfirmDeletionAllFilesInGroup, "confirm_deletion_all_files_in_group"),
        (LoadText::ShowBottomTextPanel, "show_bottom_text_panel"),
        (LoadText::HideHardLinks, "hide_hard_links"),
        (LoadText::UseCache, "use_cache"),
        (LoadText::UseJsonCacheFile, "use_json_cache_file"),
        (LoadText::DeleteToTrash, "delete_to_trash"),
        (LoadText::MinimalCacheSize, "minimal_cache_size"),
        (LoadText::ImagePreviewImage, "image_preview_image"),
        (LoadText::DuplicatePreviewImage, "duplicate_preview_image"),
        (LoadText::DuplicateDeleteOutdatedCacheEntries, "duplicate_delete_outdated_cache_entries"),
        (LoadText::ImageDeleteOutdatedCacheEntries, "image_delete_outdated_cache_entries"),
        (LoadText::VideoDeleteOutdatedCacheEntries, "video_delete_outdated_cache_entries"),
        (LoadText::UsePrehashCache, "use_prehash_cache"),
        (LoadText::MinimalPrehashCacheSize, "minimal_prehash_cache_size"),
        (LoadText::Language, "language"),
        (LoadText::ComboBoxDuplicateHashType, "combo_box_duplicate_hash_type"),
        (LoadText::ComboBoxDuplicateCheckMethod, "combo_box_duplicate_check_method"),
        (LoadText::ComboBoxImageResizeAlgorithm, "combo_box_image_resize_algorithm"),
        (LoadText::ComboBoxImageHashType, "combo_box_image_hash_type"),
        (LoadText::ComboBoxImageHashSize, "combo_box_image_hash_size"),
        (LoadText::NumberOfBiggestFiles, "number_of_biggest_files"),
        (LoadText::SimilarImagesSimilarity, "similar_images_similarity"),
        (LoadText::SimilarImagesIgnoreSameSize, "similar_images_ignore_same_size"),
        (LoadText::SimilarImagesFastCompare, "similar_images_fast_compare"),
        (LoadText::SimilarVideosSimilarity, "similar_videos_similarity"),
        (LoadText::SimilarVideosIgnoreSameSize, "similar_videos_ignore_same_size"),
        (LoadText::MusicApproximateComparison, "music_approximate_comparison"),
        (LoadText::DuplicateNameCaseSensitive, "duplicate_name_case_sensitive"),
    ];
    let mut hashmap_ls: HashMap<LoadText, String> = Default::default();
    let mut hashmap_sl: HashMap<String, LoadText> = Default::default();

    for (load_text, string) in values {
        hashmap_ls.insert(load_text, format!("--{}", string));
        hashmap_sl.insert(format!("--{}", string), load_text);
    }

    (hashmap_ls, hashmap_sl)
}

pub fn save_configuration(manual_execution: bool, upper_notebook: &GuiUpperNotebook, main_notebook: &GuiMainNotebook, settings: &GuiSettings, text_view_errors: &TextView) {
    let check_button_settings_save_at_exit = settings.check_button_settings_save_at_exit.clone();
    let text_view_errors = text_view_errors.clone();

    reset_text_view(&text_view_errors);

    if !manual_execution && !check_button_settings_save_at_exit.is_active() {
        // When check button is deselected, not save configuration at exit
        return;
    }

    let mut saving_struct = LoadSaveStruct::with_text_view(text_view_errors.clone());

    let (hashmap_ls, _hashmap_sl) = create_hash_map();

    // Upper notebook
    saving_struct.save_list_store(
        hashmap_ls.get(&LoadText::IncludedDirectories).unwrap().to_string(),
        &upper_notebook.tree_view_included_directories.clone(),
        ColumnsIncludedDirectory::Path as i32,
    );
    saving_struct.save_list_store(
        hashmap_ls.get(&LoadText::ExcludedDirectories).unwrap().to_string(),
        &upper_notebook.tree_view_excluded_directories.clone(),
        ColumnsExcludedDirectory::Path as i32,
    );
    saving_struct.save_var(hashmap_ls.get(&LoadText::ExcludedItems).unwrap().to_string(), upper_notebook.entry_excluded_items.text());
    saving_struct.save_var(
        hashmap_ls.get(&LoadText::AllowedExtensions).unwrap().to_string(),
        upper_notebook.entry_allowed_extensions.text(),
    );
    saving_struct.save_var(
        hashmap_ls.get(&LoadText::MinimalFileSize).unwrap().to_string(),
        upper_notebook.entry_general_minimal_size.text(),
    );
    saving_struct.save_var(
        hashmap_ls.get(&LoadText::MaximalFileSize).unwrap().to_string(),
        upper_notebook.entry_general_maximal_size.text(),
    );

    // Check buttons
    saving_struct.save_var(
        hashmap_ls.get(&LoadText::SaveAtExit).unwrap().to_string(),
        settings.check_button_settings_save_at_exit.is_active(),
    );
    saving_struct.save_var(
        hashmap_ls.get(&LoadText::LoadAtStart).unwrap().to_string(),
        settings.check_button_settings_load_at_start.is_active(),
    );
    saving_struct.save_var(
        hashmap_ls.get(&LoadText::ConfirmDeletionFiles).unwrap().to_string(),
        settings.check_button_settings_confirm_deletion.is_active(),
    );
    saving_struct.save_var(
        hashmap_ls.get(&LoadText::ConfirmDeletionAllFilesInGroup).unwrap().to_string(),
        settings.check_button_settings_confirm_group_deletion.is_active(),
    );
    saving_struct.save_var(
        hashmap_ls.get(&LoadText::ImagePreviewImage).unwrap().to_string(),
        settings.check_button_settings_show_preview_similar_images.is_active(),
    );
    saving_struct.save_var(
        hashmap_ls.get(&LoadText::DuplicatePreviewImage).unwrap().to_string(),
        settings.check_button_settings_show_preview_duplicates.is_active(),
    );
    saving_struct.save_var(
        hashmap_ls.get(&LoadText::HideHardLinks).unwrap().to_string(),
        settings.check_button_settings_hide_hard_links.is_active(),
    );
    saving_struct.save_var(
        hashmap_ls.get(&LoadText::UseCache).unwrap().to_string(),
        settings.check_button_settings_use_cache.is_active(),
    );
    saving_struct.save_var(
        hashmap_ls.get(&LoadText::UseJsonCacheFile).unwrap().to_string(),
        settings.check_button_settings_save_also_json.is_active(),
    );
    saving_struct.save_var(
        hashmap_ls.get(&LoadText::DeleteToTrash).unwrap().to_string(),
        settings.check_button_settings_use_trash.is_active(),
    );
    saving_struct.save_var(
        hashmap_ls.get(&LoadText::ImageDeleteOutdatedCacheEntries).unwrap().to_string(),
        settings.check_button_settings_similar_images_delete_outdated_cache.is_active(),
    );
    saving_struct.save_var(
        hashmap_ls.get(&LoadText::DuplicateDeleteOutdatedCacheEntries).unwrap().to_string(),
        settings.check_button_settings_duplicates_delete_outdated_cache.is_active(),
    );
    saving_struct.save_var(
        hashmap_ls.get(&LoadText::VideoDeleteOutdatedCacheEntries).unwrap().to_string(),
        settings.check_button_settings_similar_videos_delete_outdated_cache.is_active(),
    );
    saving_struct.save_var(
        hashmap_ls.get(&LoadText::UsePrehashCache).unwrap().to_string(),
        settings.check_button_duplicates_use_prehash_cache.is_active(),
    );
    saving_struct.save_var(
        hashmap_ls.get(&LoadText::ShowBottomTextPanel).unwrap().to_string(),
        settings.check_button_settings_show_text_view.is_active(),
    );

    // Others
    saving_struct.save_var(
        hashmap_ls.get(&LoadText::MinimalCacheSize).unwrap().to_string(),
        settings.entry_settings_cache_file_minimal_size.text(),
    );
    saving_struct.save_var(
        hashmap_ls.get(&LoadText::MinimalPrehashCacheSize).unwrap().to_string(),
        settings.entry_settings_prehash_cache_file_minimal_size.text(),
    );
    saving_struct.save_var(
        hashmap_ls.get(&LoadText::Language).unwrap().to_string(),
        get_language_from_combo_box_text(settings.combo_box_settings_language.active_text().unwrap().to_string()).short_text,
    );

    // Comboboxes main notebook
    saving_struct.save_var(
        hashmap_ls.get(&LoadText::ComboBoxDuplicateHashType).unwrap().to_string(),
        main_notebook.combo_box_duplicate_hash_type.active().unwrap_or(0),
    );
    saving_struct.save_var(
        hashmap_ls.get(&LoadText::ComboBoxDuplicateCheckMethod).unwrap().to_string(),
        main_notebook.combo_box_duplicate_check_method.active().unwrap_or(0),
    );
    saving_struct.save_var(
        hashmap_ls.get(&LoadText::ComboBoxImageResizeAlgorithm).unwrap().to_string(),
        main_notebook.combo_box_image_resize_algorithm.active().unwrap_or(0),
    );
    saving_struct.save_var(
        hashmap_ls.get(&LoadText::ComboBoxImageHashType).unwrap().to_string(),
        main_notebook.combo_box_image_hash_algorithm.active().unwrap_or(0),
    );
    saving_struct.save_var(
        hashmap_ls.get(&LoadText::ComboBoxImageHashSize).unwrap().to_string(),
        main_notebook.combo_box_image_hash_size.active().unwrap_or(0),
    );

    // Other2
    saving_struct.save_var(
        hashmap_ls.get(&LoadText::DuplicateNameCaseSensitive).unwrap().to_string(),
        main_notebook.check_button_duplicate_case_sensitive_name.is_active(),
    );
    saving_struct.save_var(
        hashmap_ls.get(&LoadText::NumberOfBiggestFiles).unwrap().to_string(),
        main_notebook.entry_big_files_number.text(),
    );
    saving_struct.save_var(
        hashmap_ls.get(&LoadText::SimilarImagesSimilarity).unwrap().to_string(),
        main_notebook.scale_similarity_similar_images.value(),
    );
    saving_struct.save_var(
        hashmap_ls.get(&LoadText::SimilarImagesIgnoreSameSize).unwrap().to_string(),
        main_notebook.check_button_image_ignore_same_size.is_active(),
    );
    saving_struct.save_var(
        hashmap_ls.get(&LoadText::SimilarImagesFastCompare).unwrap().to_string(),
        main_notebook.check_button_image_fast_compare.is_active(),
    );
    saving_struct.save_var(
        hashmap_ls.get(&LoadText::SimilarVideosSimilarity).unwrap().to_string(),
        main_notebook.scale_similarity_similar_videos.value(),
    );
    saving_struct.save_var(
        hashmap_ls.get(&LoadText::SimilarVideosIgnoreSameSize).unwrap().to_string(),
        main_notebook.check_button_video_ignore_same_size.is_active(),
    );
    saving_struct.save_var(
        hashmap_ls.get(&LoadText::MusicApproximateComparison).unwrap().to_string(),
        main_notebook.check_button_music_approximate_comparison.is_active(),
    );

    saving_struct.save_to_file(&text_view_errors);
}

pub fn load_configuration(
    manual_execution: bool,
    upper_notebook: &GuiUpperNotebook,
    main_notebook: &GuiMainNotebook,
    settings: &GuiSettings,
    text_view_errors: &TextView,
    scrolled_window_errors: &ScrolledWindow,
    arguments: Vec<OsString>,
) {
    let text_view_errors = text_view_errors.clone();

    reset_text_view(&text_view_errors);

    let mut loaded_entries = LoadSaveStruct::with_text_view(text_view_errors.clone());
    loaded_entries.open_and_read_content(&text_view_errors, manual_execution);

    // Load here language, default system language could change value in settings so we don't want to lose this value
    let short_language = get_language_from_combo_box_text(settings.combo_box_settings_language.active_text().unwrap().to_string())
        .short_text
        .to_string();

    let included_directories = get_string_from_list_store(&upper_notebook.tree_view_included_directories, ColumnsIncludedDirectory::Path as i32, None);
    let excluded_directories = get_string_from_list_store(&upper_notebook.tree_view_excluded_directories, ColumnsExcludedDirectory::Path as i32, None);

    // Loading data from hashmaps
    let (hashmap_ls, _hashmap_sl) = create_hash_map();

    let mut included_directories: Vec<String> = loaded_entries.get_vector_string(hashmap_ls.get(&LoadText::IncludedDirectories).unwrap().clone(), included_directories);
    let mut excluded_directories: Vec<String> = loaded_entries.get_vector_string(hashmap_ls.get(&LoadText::ExcludedDirectories).unwrap().clone(), excluded_directories);
    let excluded_items: String = loaded_entries.get_string(
        hashmap_ls.get(&LoadText::ExcludedItems).unwrap().clone(),
        upper_notebook.entry_excluded_items.text().to_string(),
    );
    let allowed_extensions: String = loaded_entries.get_string(hashmap_ls.get(&LoadText::AllowedExtensions).unwrap().clone(), "".to_string());
    let minimal_file_size: String = loaded_entries.get_integer_string(hashmap_ls.get(&LoadText::MinimalFileSize).unwrap().clone(), DEFAULT_MINIMAL_FILE_SIZE.to_string());
    let maximal_file_size: String = loaded_entries.get_integer_string(hashmap_ls.get(&LoadText::MaximalFileSize).unwrap().clone(), DEFAULT_MAXIMAL_FILE_SIZE.to_string());

    let loading_at_start: bool = loaded_entries.get_bool(hashmap_ls.get(&LoadText::LoadAtStart).unwrap().clone(), DEFAULT_LOAD_AT_START);
    let saving_at_exit: bool = loaded_entries.get_bool(hashmap_ls.get(&LoadText::SaveAtExit).unwrap().clone(), DEFAULT_SAVE_ON_EXIT);
    let confirm_deletion: bool = loaded_entries.get_bool(hashmap_ls.get(&LoadText::ConfirmDeletionFiles).unwrap().clone(), DEFAULT_CONFIRM_DELETION);
    let confirm_group_deletion: bool = loaded_entries.get_bool(hashmap_ls.get(&LoadText::ConfirmDeletionAllFilesInGroup).unwrap().clone(), DEFAULT_CONFIRM_GROUP_DELETION);
    let show_previews_similar_images: bool = loaded_entries.get_bool(hashmap_ls.get(&LoadText::ImagePreviewImage).unwrap().clone(), DEFAULT_SHOW_IMAGE_PREVIEW);
    let show_previews_duplicates: bool = loaded_entries.get_bool(hashmap_ls.get(&LoadText::DuplicatePreviewImage).unwrap().clone(), DEFAULT_SHOW_DUPLICATE_IMAGE_PREVIEW);
    let bottom_text_panel: bool = loaded_entries.get_bool(hashmap_ls.get(&LoadText::ShowBottomTextPanel).unwrap().clone(), DEFAULT_BOTTOM_TEXT_VIEW);
    let hide_hard_links: bool = loaded_entries.get_bool(hashmap_ls.get(&LoadText::HideHardLinks).unwrap().clone(), DEFAULT_HIDE_HARD_LINKS);
    let use_cache: bool = loaded_entries.get_bool(hashmap_ls.get(&LoadText::UseCache).unwrap().clone(), DEFAULT_USE_CACHE);
    let use_json_cache: bool = loaded_entries.get_bool(hashmap_ls.get(&LoadText::UseJsonCacheFile).unwrap().clone(), DEFAULT_SAVE_ALSO_AS_JSON);
    let use_trash: bool = loaded_entries.get_bool(hashmap_ls.get(&LoadText::DeleteToTrash).unwrap().clone(), DEFAULT_USE_TRASH);
    let delete_outdated_cache_duplicates: bool = loaded_entries.get_bool(
        hashmap_ls.get(&LoadText::DuplicateDeleteOutdatedCacheEntries).unwrap().clone(),
        DEFAULT_DUPLICATE_REMOVE_AUTO_OUTDATED_CACHE,
    );
    let delete_outdated_cache_similar_images: bool = loaded_entries.get_bool(
        hashmap_ls.get(&LoadText::ImageDeleteOutdatedCacheEntries).unwrap().clone(),
        DEFAULT_IMAGE_REMOVE_AUTO_OUTDATED_CACHE,
    );
    let delete_outdated_cache_similar_videos: bool = loaded_entries.get_bool(
        hashmap_ls.get(&LoadText::VideoDeleteOutdatedCacheEntries).unwrap().clone(),
        DEFAULT_VIDEO_REMOVE_AUTO_OUTDATED_CACHE,
    );
    let use_prehash_cache: bool = loaded_entries.get_bool(hashmap_ls.get(&LoadText::UsePrehashCache).unwrap().clone(), DEFAULT_USE_PRECACHE);

    let cache_prehash_minimal_size: String = loaded_entries.get_integer_string(
        hashmap_ls.get(&LoadText::MinimalPrehashCacheSize).unwrap().clone(),
        DEFAULT_PREHASH_MINIMAL_CACHE_SIZE.to_string(),
    );
    let cache_minimal_size: String = loaded_entries.get_integer_string(hashmap_ls.get(&LoadText::MinimalCacheSize).unwrap().clone(), DEFAULT_MINIMAL_CACHE_SIZE.to_string());
    let short_language = loaded_entries.get_string(hashmap_ls.get(&LoadText::Language).unwrap().clone(), short_language);

    let combo_box_duplicate_hash_type = loaded_entries.get_integer(hashmap_ls.get(&LoadText::ComboBoxDuplicateHashType).unwrap().clone(), 0);
    let combo_box_duplicate_checking_method = loaded_entries.get_integer(hashmap_ls.get(&LoadText::ComboBoxDuplicateCheckMethod).unwrap().clone(), 0);
    let combo_box_image_hash_size = loaded_entries.get_integer(hashmap_ls.get(&LoadText::ComboBoxImageHashSize).unwrap().clone(), 0);
    let combo_box_image_hash_algorithm = loaded_entries.get_integer(hashmap_ls.get(&LoadText::ComboBoxImageHashType).unwrap().clone(), 0);
    let combo_box_image_resize_algorithm = loaded_entries.get_integer(hashmap_ls.get(&LoadText::ComboBoxImageResizeAlgorithm).unwrap().clone(), 0);

    let number_of_biggest_files = loaded_entries.get_integer_string(
        hashmap_ls.get(&LoadText::NumberOfBiggestFiles).unwrap().clone(),
        DEFAULT_NUMBER_OF_BIGGEST_FILES.to_string(),
    );
    let similar_images_similarity = loaded_entries.get_integer(hashmap_ls.get(&LoadText::SimilarImagesSimilarity).unwrap().clone(), DEFAULT_SIMILAR_IMAGES_SIMILARITY);
    let similar_images_ignore_same_size = loaded_entries.get_bool(
        hashmap_ls.get(&LoadText::SimilarImagesIgnoreSameSize).unwrap().clone(),
        DEFAULT_SIMILAR_IMAGES_IGNORE_SAME_SIZE,
    );
    let similar_images_fast_compare = loaded_entries.get_bool(hashmap_ls.get(&LoadText::SimilarImagesFastCompare).unwrap().clone(), DEFAULT_SIMILAR_IMAGES_FAST_COMPARE);
    let similar_videos_similarity = loaded_entries.get_integer(hashmap_ls.get(&LoadText::SimilarVideosSimilarity).unwrap().clone(), DEFAULT_SIMILAR_VIDEOS_SIMILARITY);
    let similar_videos_ignore_same_size = loaded_entries.get_bool(
        hashmap_ls.get(&LoadText::SimilarVideosIgnoreSameSize).unwrap().clone(),
        DEFAULT_SIMILAR_VIDEOS_IGNORE_SAME_SIZE,
    );
    let check_button_case_sensitive_name = loaded_entries.get_integer(
        hashmap_ls.get(&LoadText::DuplicateNameCaseSensitive).unwrap().clone(),
        DEFAULT_DUPLICATE_CASE_SENSITIVE_NAME_CHECKING,
    );

    // Setting data
    if manual_execution || loading_at_start {
        {
            // Handle here arguments that were added to app e.g. czkawka_gui /home --/home/roman
            if loading_at_start && arguments.len() > 1 {
                let iter_i = arguments.iter().skip(1);
                let iter_e = iter_i.clone();
                included_directories = iter_i
                    .filter_map(|e| {
                        let r = e.to_string_lossy().to_string();
                        if !r.starts_with("--") {
                            let path = Path::new(&r);
                            if !path.exists() {
                                return None;
                            }
                            match path.canonicalize() {
                                Ok(r) => Some(r.to_string_lossy().to_string()),
                                Err(_) => None,
                            }
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>();
                excluded_directories = iter_e
                    .filter_map(|e| {
                        let r = e.to_string_lossy().to_string();
                        if let Some(r) = r.strip_prefix("--") {
                            let path = Path::new(&r);
                            if !path.exists() {
                                return None;
                            }
                            match path.canonicalize() {
                                Ok(r) => Some(r.to_string_lossy().to_string()),
                                Err(_) => None,
                            }
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>();
            }

            // Include Directories
            let tree_view_included_directories = upper_notebook.tree_view_included_directories.clone();
            let list_store = get_list_store(&tree_view_included_directories);
            list_store.clear();

            for directory in included_directories {
                let values: [(u32, &dyn ToValue); 2] = [
                    (ColumnsIncludedDirectory::Path as u32, &directory),
                    (ColumnsIncludedDirectory::ReferenceButton as u32, &false),
                ];
                list_store.set(&list_store.append(), &values);
            }

            //// Exclude Directories
            let tree_view_excluded_directories = upper_notebook.tree_view_excluded_directories.clone();
            let list_store = get_list_store(&tree_view_excluded_directories);
            list_store.clear();

            for directory in excluded_directories {
                let values: [(u32, &dyn ToValue); 1] = [(ColumnsExcludedDirectory::Path as u32, &directory)];
                list_store.set(&list_store.append(), &values);
            }
        }
        //// Language ComboBoxText
        {
            for (index, lang) in LANGUAGES_ALL.iter().enumerate() {
                if short_language == lang.short_text {
                    settings.combo_box_settings_language.set_active(Some(index as u32));
                }
            }
        }

        upper_notebook.entry_excluded_items.set_text(&excluded_items);
        upper_notebook.entry_allowed_extensions.set_text(&allowed_extensions);
        upper_notebook.entry_general_minimal_size.set_text(&minimal_file_size);
        upper_notebook.entry_general_maximal_size.set_text(&maximal_file_size);

        //// Buttons
        settings.check_button_settings_load_at_start.set_active(loading_at_start);
        settings.check_button_settings_save_at_exit.set_active(saving_at_exit);
        settings.check_button_settings_confirm_deletion.set_active(confirm_deletion);
        settings.check_button_settings_confirm_group_deletion.set_active(confirm_group_deletion);
        settings.check_button_settings_show_preview_similar_images.set_active(show_previews_similar_images);
        settings.check_button_settings_show_preview_duplicates.set_active(show_previews_duplicates);

        settings
            .check_button_settings_similar_videos_delete_outdated_cache
            .set_active(delete_outdated_cache_similar_videos);
        settings
            .check_button_settings_similar_images_delete_outdated_cache
            .set_active(delete_outdated_cache_similar_images);
        settings.check_button_settings_duplicates_delete_outdated_cache.set_active(delete_outdated_cache_duplicates);

        settings.check_button_settings_show_text_view.set_active(bottom_text_panel);
        if !bottom_text_panel {
            scrolled_window_errors.hide();
        } else {
            scrolled_window_errors.show();
        }
        settings.check_button_settings_hide_hard_links.set_active(hide_hard_links);
        settings.check_button_settings_use_cache.set_active(use_cache);
        settings.check_button_settings_save_also_json.set_active(use_json_cache);
        settings.check_button_duplicates_use_prehash_cache.set_active(use_prehash_cache);
        settings.check_button_settings_use_trash.set_active(use_trash);
        settings.entry_settings_cache_file_minimal_size.set_text(&cache_minimal_size);
        settings.entry_settings_prehash_cache_file_minimal_size.set_text(&cache_prehash_minimal_size);

        save_proper_value_to_combo_box(&main_notebook.combo_box_duplicate_hash_type, combo_box_duplicate_hash_type);
        save_proper_value_to_combo_box(&main_notebook.combo_box_duplicate_check_method, combo_box_duplicate_checking_method);
        save_proper_value_to_combo_box(&main_notebook.combo_box_image_hash_algorithm, combo_box_image_hash_algorithm);
        save_proper_value_to_combo_box(&main_notebook.combo_box_image_hash_size, combo_box_image_hash_size);
        save_proper_value_to_combo_box(&main_notebook.combo_box_image_resize_algorithm, combo_box_image_resize_algorithm);

        main_notebook.check_button_duplicate_case_sensitive_name.set_active(check_button_case_sensitive_name);
        main_notebook.entry_big_files_number.set_text(&number_of_biggest_files);
        main_notebook.check_button_image_ignore_same_size.set_active(similar_images_ignore_same_size);
        main_notebook.check_button_image_fast_compare.set_active(similar_images_fast_compare);
        main_notebook.check_button_video_ignore_same_size.set_active(similar_videos_ignore_same_size);
        main_notebook.scale_similarity_similar_videos.set_value(similar_videos_similarity as f64);

        {
            let combo_chosen_index = main_notebook.combo_box_duplicate_check_method.active().unwrap();

            if DUPLICATES_CHECK_METHOD_COMBO_BOX[combo_chosen_index as usize].check_method == CheckingMethod::Hash {
                main_notebook.combo_box_duplicate_hash_type.set_visible(true);
                main_notebook.label_duplicate_hash_type.set_visible(true);
            } else {
                main_notebook.combo_box_duplicate_hash_type.set_visible(false);
                main_notebook.label_duplicate_hash_type.set_visible(false);
            }

            if DUPLICATES_CHECK_METHOD_COMBO_BOX[combo_chosen_index as usize].check_method == CheckingMethod::Name {
                main_notebook.check_button_duplicate_case_sensitive_name.set_visible(true);
            } else {
                main_notebook.check_button_duplicate_case_sensitive_name.set_visible(false);
            }
        }

        // Set size of similarity scale gtk node, must be set BEFORE setting value of this
        let index = main_notebook.combo_box_image_hash_size.active().unwrap() as usize;

        main_notebook.scale_similarity_similar_images.set_range(0_f64, SIMILAR_VALUES[index][5] as f64);
        main_notebook.scale_similarity_similar_images.set_fill_level(SIMILAR_VALUES[index][5] as f64);

        main_notebook.scale_similarity_similar_images.set_value(similar_images_similarity as f64);
    } else {
        settings.check_button_settings_load_at_start.set_active(false);
    }
}

/// Function do not allow to set invalid index to combobox because this would cause to show empty value and function would crash
fn save_proper_value_to_combo_box(combo_box: &ComboBoxText, what_to_save: u32) {
    combo_box.set_active(Some(what_to_save));
    if combo_box.active().is_none() {
        combo_box.set_active(Some(0));
    }
}

/// Reset configuration to defaults
pub fn reset_configuration(manual_clearing: bool, upper_notebook: &GuiUpperNotebook, main_notebook: &GuiMainNotebook, settings: &GuiSettings, text_view_errors: &TextView) {
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

        let values: [(u32, &dyn ToValue); 2] = [
            (ColumnsIncludedDirectory::Path as u32, &current_dir),
            (ColumnsIncludedDirectory::ReferenceButton as u32, &false),
        ];
        list_store.set(&list_store.append(), &values);
    }
    // Resetting excluded directories
    {
        let tree_view_excluded_directories = upper_notebook.tree_view_excluded_directories.clone();
        let list_store = get_list_store(&tree_view_excluded_directories);
        list_store.clear();
        for i in DEFAULT_EXCLUDED_DIRECTORIES.iter() {
            let values: [(u32, &dyn ToValue); 1] = [(ColumnsExcludedDirectory::Path as u32, &i)];
            list_store.set(&list_store.append(), &values);
        }
    }
    // Resetting excluded items
    {
        upper_notebook.entry_excluded_items.set_text(DEFAULT_EXCLUDED_ITEMS);
        upper_notebook.entry_allowed_extensions.set_text("");
        upper_notebook.entry_general_minimal_size.set_text(DEFAULT_MINIMAL_FILE_SIZE);
        upper_notebook.entry_general_maximal_size.set_text(DEFAULT_MAXIMAL_FILE_SIZE);
    }

    // Set default settings
    {
        settings.check_button_settings_save_at_exit.set_active(DEFAULT_SAVE_ON_EXIT);
        settings.check_button_settings_load_at_start.set_active(DEFAULT_LOAD_AT_START);
        settings.check_button_settings_confirm_deletion.set_active(DEFAULT_CONFIRM_DELETION);
        settings.check_button_settings_confirm_group_deletion.set_active(DEFAULT_CONFIRM_GROUP_DELETION);
        settings.check_button_settings_show_preview_similar_images.set_active(DEFAULT_SHOW_IMAGE_PREVIEW);
        settings.check_button_settings_show_preview_duplicates.set_active(DEFAULT_SHOW_DUPLICATE_IMAGE_PREVIEW);
        settings.check_button_settings_show_text_view.set_active(DEFAULT_BOTTOM_TEXT_VIEW);
        settings.check_button_settings_hide_hard_links.set_active(DEFAULT_HIDE_HARD_LINKS);
        settings.check_button_settings_use_cache.set_active(DEFAULT_USE_CACHE);
        settings.check_button_settings_save_also_json.set_active(DEFAULT_SAVE_ALSO_AS_JSON);
        settings.check_button_settings_use_trash.set_active(DEFAULT_USE_TRASH);
        settings.entry_settings_cache_file_minimal_size.set_text(DEFAULT_MINIMAL_CACHE_SIZE);
        settings
            .check_button_settings_similar_videos_delete_outdated_cache
            .set_active(DEFAULT_VIDEO_REMOVE_AUTO_OUTDATED_CACHE);
        settings
            .check_button_settings_similar_images_delete_outdated_cache
            .set_active(DEFAULT_IMAGE_REMOVE_AUTO_OUTDATED_CACHE);
        settings
            .check_button_settings_duplicates_delete_outdated_cache
            .set_active(DEFAULT_DUPLICATE_REMOVE_AUTO_OUTDATED_CACHE);
        settings.check_button_duplicates_use_prehash_cache.set_active(DEFAULT_USE_PRECACHE);
        settings.entry_settings_prehash_cache_file_minimal_size.set_text(DEFAULT_PREHASH_MINIMAL_CACHE_SIZE);
        settings.combo_box_settings_language.set_active(Some(0));

        main_notebook.combo_box_duplicate_hash_type.set_active(Some(0));
        main_notebook.combo_box_duplicate_check_method.set_active(Some(0));
        main_notebook.combo_box_image_hash_algorithm.set_active(Some(0));
        main_notebook.combo_box_image_resize_algorithm.set_active(Some(0));
        main_notebook.combo_box_image_hash_size.set_active(Some(0));

        main_notebook.scale_similarity_similar_images.set_range(0_f64, SIMILAR_VALUES[0][5] as f64); // DEFAULT FOR MAX of 8
        main_notebook.scale_similarity_similar_images.set_fill_level(SIMILAR_VALUES[0][5] as f64);

        main_notebook.entry_big_files_number.set_text(DEFAULT_NUMBER_OF_BIGGEST_FILES);
        main_notebook.scale_similarity_similar_images.set_value(DEFAULT_SIMILAR_IMAGES_SIMILARITY as f64);
        main_notebook.check_button_image_ignore_same_size.set_active(DEFAULT_SIMILAR_IMAGES_IGNORE_SAME_SIZE);
        main_notebook.check_button_image_fast_compare.set_active(DEFAULT_SIMILAR_IMAGES_FAST_COMPARE);
        main_notebook.check_button_video_ignore_same_size.set_active(DEFAULT_SIMILAR_VIDEOS_IGNORE_SAME_SIZE);
        main_notebook.scale_similarity_similar_videos.set_value(DEFAULT_SIMILAR_VIDEOS_SIMILARITY as f64);
    }
    if manual_clearing {
        add_text_to_text_view(&text_view_errors, &flg!("saving_loading_reset_configuration"));
    }
}
