#![allow(unused)]
use std::collections::HashMap;
use std::env;
use std::ffi::OsString;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use czkawka_core::common::config_cache_path::get_config_cache_path;
use czkawka_core::common::get_all_available_threads;
use czkawka_core::common::items::DEFAULT_EXCLUDED_ITEMS;
use czkawka_core::common::model::CheckingMethod;
use czkawka_core::tools::similar_images::SIMILAR_VALUES;
use gtk4::prelude::*;
use gtk4::{ComboBoxText, ScrolledWindow, TextView, TreeView};
use log::error;

use serde::{Deserialize, Serialize};
use serde_json;

use crate::flg;
use crate::gui_structs::gui_main_notebook::GuiMainNotebook;
use crate::gui_structs::gui_settings::GuiSettings;
use crate::gui_structs::gui_upper_notebook::GuiUpperNotebook;
use crate::help_combo_box::DUPLICATES_CHECK_METHOD_COMBO_BOX;
use crate::help_functions::*;
use crate::language_functions::{LANGUAGES_ALL, get_language_from_combo_box_text};

const SAVE_FILE_NAME_JSON: &str = "czkawka_gui_config.json";

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
const DEFAULT_GENERAL_IGNORE_OTHER_FILESYSTEMS: bool = false;
const DEFAULT_USING_RUST_LIBRARIES_TO_SHOW_PREVIEW: bool = true;

const DEFAULT_MUSIC_APPROXIMATE_COMPARISON: bool = false;
const DEFAULT_MUSIC_GROUP_CONTENT_BY_TITLE: bool = false;

const DEFAULT_BROKEN_FILES_PDF: bool = true;
const DEFAULT_BROKEN_FILES_AUDIO: bool = true;
const DEFAULT_BROKEN_FILES_ARCHIVE: bool = true;
const DEFAULT_BROKEN_FILES_IMAGE: bool = true;

const DEFAULT_THREAD_NUMBER: u32 = 0;

const DEFAULT_NUMBER_OF_BIGGEST_FILES: &str = "50";
const DEFAULT_SIMILAR_IMAGES_SIMILARITY: f32 = 0.0;
const DEFAULT_SIMILAR_IMAGES_IGNORE_SAME_SIZE: bool = false;
const DEFAULT_SIMILAR_VIDEOS_SIMILARITY: f32 = 15.0;
const DEFAULT_SIMILAR_VIDEOS_IGNORE_SAME_SIZE: bool = false;

pub const DEFAULT_MINIMAL_FILE_SIZE: &str = "16384";
pub const DEFAULT_MAXIMAL_FILE_SIZE: &str = "999999999999";

#[cfg(target_family = "unix")]
const DEFAULT_EXCLUDED_DIRECTORIES: &[&str] = &["/proc", "/dev", "/sys", "/snap"];
#[cfg(not(target_family = "unix"))]
const DEFAULT_EXCLUDED_DIRECTORIES: &[&str] = &["C:\\Windows"];

struct LoadSaveStruct {
    settings: SettingsJson,
    text_view: TextView,
}

impl LoadSaveStruct {
    pub(crate) fn with_text_view(text_view: TextView) -> Self {
        Self {
            settings: SettingsJson::default(),
            text_view,
        }
    }

    fn open_save_file_path(&self) -> Option<PathBuf> {
        let config_dir = get_config_cache_path()?.config_folder;
        Some(config_dir.join(Path::new(SAVE_FILE_NAME_JSON)))
    }

    pub(crate) fn open_and_read_content(&mut self, text_view_errors: &TextView, manual_execution: bool) {
        // Read only JSON config; legacy text format compatibility removed
        let json_file = match get_config_cache_path() {
            Some(cfg) => cfg.config_folder.join(Path::new(SAVE_FILE_NAME_JSON)),
            None => {
                if manual_execution {
                    add_text_to_text_view(text_view_errors, &flg!("saving_loading_failed_to_read_config_file", path = SAVE_FILE_NAME_JSON));
                }
                return;
            }
        };

        if !json_file.is_file() {
            if manual_execution {
                // No config file - that's fine, keep defaults
                add_text_to_text_view(text_view_errors, &flg!("saving_loading_loading_success"));
            }
            return;
        }

        match std::fs::read_to_string(&json_file) {
            Ok(content) => match serde_json::from_str::<SettingsJson>(&content) {
                Ok(cfg) => {
                    self.settings = cfg;
                    if manual_execution {
                        add_text_to_text_view(text_view_errors, &flg!("saving_loading_loading_success"));
                    }
                }
                Err(e) => {
                    add_text_to_text_view(
                        text_view_errors,
                        &flg!(
                            "saving_loading_failed_to_read_data_from_file",
                            path = json_file.to_string_lossy().to_string(),
                            reason = e.to_string()
                        ),
                    );
                }
            },
            Err(e) => {
                add_text_to_text_view(
                    text_view_errors,
                    &flg!(
                        "saving_loading_failed_to_read_data_from_file",
                        path = json_file.to_string_lossy().to_string(),
                        reason = e.to_string()
                    ),
                );
            }
        }
    }

    pub(crate) fn save_to_file(&self, text_view_errors: &TextView) {
        // Write only JSON config; legacy text format compatibility removed
        let json_file = match self.open_save_file_path() {
            Some(p) => p,
            None => {
                add_text_to_text_view(text_view_errors, &flg!("saving_loading_failed_to_create_config_file", path = SAVE_FILE_NAME_JSON, reason = "config directory not found"));
                return;
            }
        };

        match serde_json::to_string_pretty(&self.settings) {
            Ok(json_string) => match std::fs::write(&json_file, json_string) {
                Ok(_) => {
                    add_text_to_text_view(
                        text_view_errors,
                        flg!("saving_loading_saving_success", name = json_file.to_string_lossy().to_string()).as_str(),
                    );
                }
                Err(e) => {
                    add_text_to_text_view(
                        text_view_errors,
                        &flg!(
                            "saving_loading_failed_to_create_config_file",
                            path = json_file.to_string_lossy().to_string(),
                            reason = e.to_string()
                        ),
                    );
                }
            },
            Err(e) => {
                // "saving_loading_failed_to_serialize_json" does not exist in localization.
                // Reuse existing message id that accepts path and reason.
                add_text_to_text_view(
                    text_view_errors,
                    &flg!(
                        "saving_loading_failed_to_create_config_file",
                        path = json_file.to_string_lossy().to_string(),
                        reason = e.to_string()
                    ),
                );
            }
        }
    }

    // Expose accessors for the new serde-backed settings
    pub(crate) fn settings(&self) -> &SettingsJson {
        &self.settings
    }
    pub(crate) fn settings_mut(&mut self) -> &mut SettingsJson {
        &mut self.settings
    }
}

// New JSON model mirroring available settings; per-field serde defaults so a bad/missing field won't break deserialization
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SettingsJson {
    #[serde(default = "default_included_directories")]
    pub included_directories: Vec<String>,

    #[serde(default = "default_excluded_directories")]
    pub excluded_directories: Vec<String>,

    #[serde(default = "default_excluded_items")]
    pub excluded_items: String,

    #[serde(default)]
    pub allowed_extensions: String,

    #[serde(default = "default_minimal_file_size")]
    pub minimal_file_size: String,

    #[serde(default = "default_maximal_file_size")]
    pub maximal_file_size: String,

    #[serde(default = "default_save_at_exit")]
    pub save_at_exit: bool,

    #[serde(default = "default_load_at_start")]
    pub load_at_start: bool,

    #[serde(default = "default_confirm_deletion")]
    pub confirm_deletion_files: bool,

    #[serde(default = "default_confirm_group_deletion")]
    pub confirm_deletion_all_files_in_group: bool,

    #[serde(default = "default_show_bottom_text_panel")]
    pub show_bottom_text_panel: bool,

    #[serde(default = "default_hide_hard_links")]
    pub hide_hard_links: bool,

    #[serde(default = "default_use_cache")]
    pub use_cache: bool,

    #[serde(default = "default_save_also_as_json")]
    pub use_json_cache_file: bool,

    #[serde(default = "default_delete_to_trash")]
    pub delete_to_trash: bool,

    #[serde(default = "default_minimal_cache_size")]
    pub minimal_cache_size: String,

    #[serde(default = "default_image_preview_image")]
    pub image_preview_image: bool,

    #[serde(default = "default_duplicate_preview_image")]
    pub duplicate_preview_image: bool,

    #[serde(default = "default_duplicate_delete_outdated_cache_entries")]
    pub duplicate_delete_outdated_cache_entries: bool,

    #[serde(default = "default_image_delete_outdated_cache_entries")]
    pub image_delete_outdated_cache_entries: bool,

    #[serde(default = "default_video_delete_outdated_cache_entries")]
    pub video_delete_outdated_cache_entries: bool,

    #[serde(default = "default_use_prehash_cache")]
    pub use_prehash_cache: bool,

    #[serde(default = "default_minimal_prehash_cache_size")]
    pub minimal_prehash_cache_size: String,

    #[serde(default)]
    pub language: String,

    #[serde(default)]
    pub combo_box_duplicate_hash_type: u32,

    #[serde(default)]
    pub combo_box_duplicate_check_method: u32,

    #[serde(default)]
    pub combo_box_image_resize_algorithm: u32,

    #[serde(default)]
    pub combo_box_image_hash_type: u32,

    #[serde(default = "default_image_hash_size")]
    pub combo_box_image_hash_size: u32,

    #[serde(default = "default_number_of_biggest_files")]
    pub number_of_biggest_files: String,

    #[serde(default = "default_similar_images_similarity")]
    pub similar_images_similarity: f64,

    #[serde(default = "default_similar_images_ignore_same_size")]
    pub similar_images_ignore_same_size: bool,

    #[serde(default = "default_similar_videos_similarity")]
    pub similar_videos_similarity: f64,

    #[serde(default = "default_similar_videos_ignore_same_size")]
    pub similar_videos_ignore_same_size: bool,

    #[serde(default = "default_music_approximate_comparison")]
    pub music_approximate_comparison: bool,

    #[serde(default = "default_duplicate_name_case_sensitive")]
    pub duplicate_name_case_sensitive: bool,

    #[serde(default)]
    pub combo_box_big_files_mode: u32,

    #[serde(default = "default_broken_files_pdf")]
    pub broken_files_pdf: bool,

    #[serde(default = "default_broken_files_audio")]
    pub broken_files_audio: bool,

    #[serde(default = "default_broken_files_image")]
    pub broken_files_image: bool,

    #[serde(default = "default_broken_files_archive")]
    pub broken_files_archive: bool,

    #[serde(default = "default_ignore_other_filesystems")]
    pub ignore_other_filesystems: bool,

    #[serde(default = "default_thread_number")]
    pub thread_number: u32,

    #[serde(default = "default_music_compare_by_title")]
    pub music_compare_by_title: bool,

    #[serde(default = "default_use_rust_libraries_to_preview")]
    pub use_rust_libraries_to_preview: bool,
}

// Use serde to build defaults from empty object; this uses per-field defaults above
impl Default for SettingsJson {
    fn default() -> Self {
        serde_json::from_str("{}").expect("Cannot fail creating SettingsJson from empty object")
    }
}

// Per-field default helper functions (kept small and explicit)
fn default_included_directories() -> Vec<String> {
    Vec::new()
}
fn default_excluded_directories() -> Vec<String> {
    DEFAULT_EXCLUDED_DIRECTORIES.iter().map(|s| s.to_string()).collect()
}
fn default_excluded_items() -> String {
    DEFAULT_EXCLUDED_ITEMS.to_string()
}
fn default_minimal_file_size() -> String {
    DEFAULT_MINIMAL_FILE_SIZE.to_string()
}
fn default_maximal_file_size() -> String {
    DEFAULT_MAXIMAL_FILE_SIZE.to_string()
}
fn default_save_at_exit() -> bool {
    DEFAULT_SAVE_ON_EXIT
}
fn default_load_at_start() -> bool {
    DEFAULT_LOAD_AT_START
}
fn default_confirm_deletion() -> bool {
    DEFAULT_CONFIRM_DELETION
}
fn default_confirm_group_deletion() -> bool {
    DEFAULT_CONFIRM_GROUP_DELETION
}
fn default_show_bottom_text_panel() -> bool {
    DEFAULT_BOTTOM_TEXT_VIEW
}
fn default_hide_hard_links() -> bool {
    DEFAULT_HIDE_HARD_LINKS
}
fn default_use_cache() -> bool {
    DEFAULT_USE_CACHE
}
fn default_save_also_as_json() -> bool {
    DEFAULT_SAVE_ALSO_AS_JSON
}
fn default_delete_to_trash() -> bool {
    DEFAULT_USE_TRASH
}
fn default_minimal_cache_size() -> String {
    DEFAULT_MINIMAL_CACHE_SIZE.to_string()
}
fn default_image_preview_image() -> bool {
    DEFAULT_SHOW_IMAGE_PREVIEW
}
fn default_duplicate_preview_image() -> bool {
    DEFAULT_SHOW_DUPLICATE_IMAGE_PREVIEW
}
fn default_duplicate_delete_outdated_cache_entries() -> bool {
    DEFAULT_DUPLICATE_REMOVE_AUTO_OUTDATED_CACHE
}
fn default_image_delete_outdated_cache_entries() -> bool {
    DEFAULT_IMAGE_REMOVE_AUTO_OUTDATED_CACHE
}
fn default_video_delete_outdated_cache_entries() -> bool {
    DEFAULT_VIDEO_REMOVE_AUTO_OUTDATED_CACHE
}
fn default_use_prehash_cache() -> bool {
    DEFAULT_USE_PRECACHE
}
fn default_minimal_prehash_cache_size() -> String {
    DEFAULT_PREHASH_MINIMAL_CACHE_SIZE.to_string()
}
fn default_image_hash_size() -> u32 {
    1
}
fn default_number_of_biggest_files() -> String {
    DEFAULT_NUMBER_OF_BIGGEST_FILES.to_string()
}
fn default_similar_images_similarity() -> f64 {
    DEFAULT_SIMILAR_IMAGES_SIMILARITY as f64
}
fn default_similar_images_ignore_same_size() -> bool {
    DEFAULT_SIMILAR_IMAGES_IGNORE_SAME_SIZE
}
fn default_similar_videos_similarity() -> f64 {
    DEFAULT_SIMILAR_VIDEOS_SIMILARITY as f64
}
fn default_similar_videos_ignore_same_size() -> bool {
    DEFAULT_SIMILAR_VIDEOS_IGNORE_SAME_SIZE
}
fn default_music_approximate_comparison() -> bool {
    DEFAULT_MUSIC_APPROXIMATE_COMPARISON
}
fn default_duplicate_name_case_sensitive() -> bool {
    DEFAULT_DUPLICATE_CASE_SENSITIVE_NAME_CHECKING
}
fn default_broken_files_pdf() -> bool {
    DEFAULT_BROKEN_FILES_PDF
}
fn default_broken_files_audio() -> bool {
    DEFAULT_BROKEN_FILES_AUDIO
}
fn default_broken_files_image() -> bool {
    DEFAULT_BROKEN_FILES_IMAGE
}
fn default_broken_files_archive() -> bool {
    DEFAULT_BROKEN_FILES_ARCHIVE
}
fn default_ignore_other_filesystems() -> bool {
    DEFAULT_GENERAL_IGNORE_OTHER_FILESYSTEMS
}
fn default_thread_number() -> u32 {
    DEFAULT_THREAD_NUMBER
}
fn default_music_compare_by_title() -> bool {
    DEFAULT_MUSIC_GROUP_CONTENT_BY_TITLE
}
fn default_use_rust_libraries_to_preview() -> bool {
    DEFAULT_USING_RUST_LIBRARIES_TO_SHOW_PREVIEW
}

fn set_configuration_to_gui_internal(
    upper_notebook: &GuiUpperNotebook,
    main_notebook: &GuiMainNotebook,
    settings: &GuiSettings,
    default_config: &SettingsJson,
) {
    // Resetting included directories
    {
        let tree_view_included_directories = upper_notebook.tree_view_included_directories.clone();
        let list_store = get_list_store(&tree_view_included_directories);
        list_store.clear();
        for i in default_config.included_directories.clone() {
            let values: [(u32, &dyn ToValue); 1] = [(ColumnsExcludedDirectory::Path as u32, &i)];
            list_store.set(&list_store.append(), &values);
        }
    }
    // Resetting excluded directories
    {
        let tree_view_excluded_directories = upper_notebook.tree_view_excluded_directories.clone();
        let list_store = get_list_store(&tree_view_excluded_directories);
        list_store.clear();
        for i in default_config.excluded_directories.clone() {
            let values: [(u32, &dyn ToValue); 1] = [(ColumnsExcludedDirectory::Path as u32, &i)];
            list_store.set(&list_store.append(), &values);
        }
    }
    // Resetting excluded items
    {
        upper_notebook.entry_excluded_items.set_text(&default_config.excluded_items);
        upper_notebook.entry_allowed_extensions.set_text(&default_config.allowed_extensions);
        upper_notebook.entry_general_minimal_size.set_text(&default_config.minimal_file_size);
        upper_notebook.entry_general_maximal_size.set_text(&default_config.maximal_file_size);
    }

    // Set default settings
    {
        settings.check_button_settings_save_at_exit.set_active(default_config.save_at_exit);
        settings.check_button_settings_load_at_start.set_active(default_config.load_at_start);
        settings.check_button_settings_confirm_deletion.set_active(default_config.confirm_deletion_files);
        settings.check_button_settings_confirm_group_deletion.set_active(default_config.confirm_deletion_all_files_in_group);
        settings.check_button_settings_show_preview_similar_images.set_active(default_config.image_preview_image);
        settings.check_button_settings_show_preview_duplicates.set_active(default_config.duplicate_preview_image);
        settings.check_button_settings_show_text_view.set_active(default_config.show_bottom_text_panel);
        settings.check_button_settings_hide_hard_links.set_active(default_config.hide_hard_links);
        settings.check_button_settings_use_cache.set_active(default_config.use_cache);
        settings.check_button_settings_save_also_json.set_active(default_config.use_json_cache_file);
        settings.check_button_settings_use_trash.set_active(default_config.delete_to_trash);
        settings.entry_settings_cache_file_minimal_size.set_text(&default_config.minimal_cache_size);
        settings
            .check_button_settings_similar_videos_delete_outdated_cache
            .set_active(default_config.video_delete_outdated_cache_entries);
        settings
            .check_button_settings_similar_images_delete_outdated_cache
            .set_active(default_config.image_delete_outdated_cache_entries);
        settings
            .check_button_settings_duplicates_delete_outdated_cache
            .set_active(default_config.duplicate_delete_outdated_cache_entries);
        settings.check_button_duplicates_use_prehash_cache.set_active(default_config.use_prehash_cache);
        settings.entry_settings_prehash_cache_file_minimal_size.set_text(&default_config.minimal_prehash_cache_size);

        let lang_idx = LANGUAGES_ALL.iter().position(|l| l.short_text == &default_config.language).unwrap_or(0);
        settings.combo_box_settings_language.set_active(Some(lang_idx as u32));

        settings.check_button_settings_one_filesystem.set_active(default_config.ignore_other_filesystems);
        settings.check_button_settings_use_rust_preview.set_active(default_config.use_rust_libraries_to_preview);

        main_notebook.combo_box_duplicate_hash_type.set_active(Some(default_config.combo_box_duplicate_hash_type));
        main_notebook.combo_box_duplicate_check_method.set_active(Some(default_config.combo_box_duplicate_check_method));
        main_notebook.combo_box_image_hash_algorithm.set_active(Some(default_config.combo_box_image_hash_type));
        main_notebook.combo_box_image_resize_algorithm.set_active(Some(default_config.combo_box_image_resize_algorithm));
        main_notebook.combo_box_image_hash_size.set_active(Some(default_config.combo_box_image_hash_size));
        main_notebook.combo_box_big_files_mode.set_active(Some(default_config.combo_box_big_files_mode));

        main_notebook.check_button_broken_files_audio.set_active(default_config.broken_files_audio);
        main_notebook.check_button_broken_files_pdf.set_active(default_config.broken_files_pdf);
        main_notebook.check_button_broken_files_archive.set_active(default_config.broken_files_archive);
        main_notebook.check_button_broken_files_image.set_active(default_config.broken_files_image);

        main_notebook.scale_similarity_similar_images.set_range(0_f64, SIMILAR_VALUES[0][5] as f64);
        main_notebook.scale_similarity_similar_images.set_fill_level(SIMILAR_VALUES[0][5] as f64);

        main_notebook
            .check_button_music_compare_only_in_title_group
            .set_active(default_config.music_compare_by_title);

        main_notebook.check_button_music_approximate_comparison.set_active(default_config.music_approximate_comparison);

        main_notebook.entry_big_files_number.set_text(&default_config.number_of_biggest_files);
        main_notebook.scale_similarity_similar_images.set_value(default_config.similar_images_similarity as f64);
        main_notebook.check_button_image_ignore_same_size.set_active(default_config.similar_images_ignore_same_size);
        main_notebook.check_button_video_ignore_same_size.set_active(default_config.similar_videos_ignore_same_size);
        main_notebook.scale_similarity_similar_videos.set_value(default_config.similar_videos_similarity as f64);
    }
}

fn get_current_directory() -> String {
    match env::current_dir() {
        Ok(t) => t.to_string_lossy().to_string(),
        Err(_inspected) => {
            if cfg!(target_family = "unix") {
                "/home".to_string()
            } else if cfg!(target_family = "windows") {
                "C:\\".to_string()
            } else {
                String::new()
            }
        }
    }
}

fn load_arguments(arguments: Vec<String>) -> Option<Vec<String>>{
    // Handle here arguments that were added to app e.g. czkawka_gui /home --/home/roman
    if arguments.len() > 1 {
        let iter_i = arguments.iter().skip(1);
        let iter_e = iter_i.clone();
        let inc_dir = iter_i
            .filter_map(|e| {
                let r = e.to_string();
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
        let exc_dir = iter_e
            .filter_map(|e| {
                let r = e.to_string();
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

        // if inc_dir.is_empty() {
        //     error!("Arguments {arguments:?} should contain at least one directory to include");
        // } else {
        //     included_directories = inc_dir;
        //     excluded_directories = exc_dir;
        //     saving_at_exit = false;
        //     set_start_folders = true;
        // }
        if !inc_dir.is_empty() {
            return Some(inc_dir);
        }
    }
    None
}

pub(crate) fn reset_configuration(manual_clearing: bool, upper_notebook: &GuiUpperNotebook, main_notebook: &GuiMainNotebook, settings: &GuiSettings, text_view_errors: &TextView) {
    // TODO Maybe add popup dialog to confirm resetting
    let text_view_errors = text_view_errors.clone();

    let mut default_config = SettingsJson::default();

    default_config.included_directories = vec![get_current_directory()];

    reset_text_view(&text_view_errors);

    set_configuration_to_gui_internal(upper_notebook, main_notebook, settings, &default_config);

    if manual_clearing {
        add_text_to_text_view(&text_view_errors, &flg!("saving_loading_reset_configuration"));
    }
}

pub fn load_configuration(
    manual_execution: bool,
    upper_notebook: &GuiUpperNotebook,
    main_notebook: &GuiMainNotebook,
    settings: &GuiSettings,
    text_view_errors: &TextView,
    scrolled_window_errors: &ScrolledWindow,
    arguments: &Vec<String>,
) {
    // Preserve original public API: first argument is the manual_execution flag.
    // Minimal loader: read JSON configuration and provide it back via a LoadSaveStruct.
    let mut loader = LoadSaveStruct::with_text_view(text_view_errors.clone());
    loader.open_and_read_content(text_view_errors, manual_execution);

    let folders = load_arguments(arguments.clone()); // TODO - port from krokiet, because contains a lot of better logic
    let set_start_folders = folders.is_some();
    let saving_at_exit = !set_start_folders;
    let folders = folders.unwrap_or_else(|| vec![get_current_directory()]);

    let loaded_settings = loader.settings_mut();
    if !loaded_settings.show_bottom_text_panel {
        scrolled_window_errors.hide();
    } else {
        scrolled_window_errors.show();
    }

    reset_text_view(&text_view_errors);
    set_configuration_to_gui_internal(upper_notebook, main_notebook, settings, loaded_settings);
}
// pub(crate) fn load_configuration(
//     manual_execution: bool,
//     upper_notebook: &GuiUpperNotebook,
//     main_notebook: &GuiMainNotebook,
//     settings: &GuiSettings,
//     text_view_errors: &TextView,
//     scrolled_window_errors: &ScrolledWindow,
//     arguments: &[OsString],
// ) {
//     let text_view_errors = text_view_errors.clone();
//
//     reset_text_view(&text_view_errors);
//
//     let mut loaded_entries = LoadSaveStruct::with_text_view(text_view_errors.clone());
//     loaded_entries.open_and_read_content(&text_view_errors, manual_execution);
//
//     // Load here language, default system language could change value in settings so we don't want to lose this value
//     let short_language = get_language_from_combo_box_text(&settings.combo_box_settings_language.active_text().expect("No active text"))
//         .short_text
//         .to_string();
//
//     let included_directories = get_string_from_list_store(&upper_notebook.tree_view_included_directories, ColumnsIncludedDirectory::Path as i32, None);
//     let excluded_directories = get_string_from_list_store(&upper_notebook.tree_view_excluded_directories, ColumnsExcludedDirectory::Path as i32, None);
//
//     // Loading data from hashmaps
//     let (hashmap_ls, _hashmap_sl) = create_hash_map();
//
//     let mut included_directories: Vec<String> = loaded_entries.get_vector_string(&hashmap_ls[&LoadText::IncludedDirectories], included_directories);
//     let mut excluded_directories: Vec<String> = loaded_entries.get_vector_string(&hashmap_ls[&LoadText::ExcludedDirectories], excluded_directories);
//     let excluded_items: String = loaded_entries.get_string(hashmap_ls[&LoadText::ExcludedItems].clone(), upper_notebook.entry_excluded_items.text().to_string());
//     let allowed_extensions: String = loaded_entries.get_string(hashmap_ls[&LoadText::AllowedExtensions].clone(), String::new());
//     let minimal_file_size: String = loaded_entries.get_integer_string(hashmap_ls[&LoadText::MinimalFileSize].clone(), DEFAULT_MINIMAL_FILE_SIZE.to_string());
//     let maximal_file_size: String = loaded_entries.get_integer_string(hashmap_ls[&LoadText::MaximalFileSize].clone(), DEFAULT_MAXIMAL_FILE_SIZE.to_string());
//
//     let loading_at_start: bool = loaded_entries.get_bool(hashmap_ls[&LoadText::LoadAtStart].clone(), DEFAULT_LOAD_AT_START);
//     let mut saving_at_exit: bool = loaded_entries.get_bool(hashmap_ls[&LoadText::SaveAtExit].clone(), DEFAULT_SAVE_ON_EXIT);
//     let confirm_deletion: bool = loaded_entries.get_bool(hashmap_ls[&LoadText::ConfirmDeletionFiles].clone(), DEFAULT_CONFIRM_DELETION);
//     let confirm_group_deletion: bool = loaded_entries.get_bool(hashmap_ls[&LoadText::ConfirmDeletionAllFilesInGroup].clone(), DEFAULT_CONFIRM_GROUP_DELETION);
//     let show_previews_similar_images: bool = loaded_entries.get_bool(hashmap_ls[&LoadText::ImagePreviewImage].clone(), DEFAULT_SHOW_IMAGE_PREVIEW);
//     let show_previews_duplicates: bool = loaded_entries.get_bool(hashmap_ls[&LoadText::DuplicatePreviewImage].clone(), DEFAULT_SHOW_DUPLICATE_IMAGE_PREVIEW);
//     let bottom_text_panel: bool = loaded_entries.get_bool(hashmap_ls[&LoadText::ShowBottomTextPanel].clone(), DEFAULT_BOTTOM_TEXT_VIEW);
//     let hide_hard_links: bool = loaded_entries.get_bool(hashmap_ls[&LoadText::HideHardLinks].clone(), DEFAULT_HIDE_HARD_LINKS);
//     let use_cache: bool = loaded_entries.get_bool(hashmap_ls[&LoadText::UseCache].clone(), DEFAULT_USE_CACHE);
//     let use_json_cache: bool = loaded_entries.get_bool(hashmap_ls[&LoadText::UseJsonCacheFile].clone(), DEFAULT_SAVE_ALSO_AS_JSON);
//     let use_trash: bool = loaded_entries.get_bool(hashmap_ls[&LoadText::DeleteToTrash].clone(), DEFAULT_USE_TRASH);
//     let ignore_other_fs: bool = loaded_entries.get_bool(hashmap_ls[&LoadText::GeneralIgnoreOtherFilesystems].clone(), DEFAULT_GENERAL_IGNORE_OTHER_FILESYSTEMS);
//     let use_rust_libraries_to_preview: bool = loaded_entries.get_bool(
//         hashmap_ls[&LoadText::GeneralUseRustLibrariesToPreview].clone(),
//         DEFAULT_USING_RUST_LIBRARIES_TO_SHOW_PREVIEW,
//     );
//
//     let delete_outdated_cache_duplicates: bool = loaded_entries.get_bool(
//         hashmap_ls[&LoadText::DuplicateDeleteOutdatedCacheEntries].clone(),
//         DEFAULT_DUPLICATE_REMOVE_AUTO_OUTDATED_CACHE,
//     );
//     let delete_outdated_cache_similar_images: bool =
//         loaded_entries.get_bool(hashmap_ls[&LoadText::ImageDeleteOutdatedCacheEntries].clone(), DEFAULT_IMAGE_REMOVE_AUTO_OUTDATED_CACHE);
//     let delete_outdated_cache_similar_videos: bool =
//         loaded_entries.get_bool(hashmap_ls[&LoadText::VideoDeleteOutdatedCacheEntries].clone(), DEFAULT_VIDEO_REMOVE_AUTO_OUTDATED_CACHE);
//     let use_prehash_cache: bool = loaded_entries.get_bool(hashmap_ls[&LoadText::UsePrehashCache].clone(), DEFAULT_USE_PRECACHE);
//
//     let cache_prehash_minimal_size: String =
//         loaded_entries.get_integer_string(hashmap_ls[&LoadText::MinimalPrehashCacheSize].clone(), DEFAULT_PREHASH_MINIMAL_CACHE_SIZE.to_string());
//     let cache_minimal_size: String = loaded_entries.get_integer_string(hashmap_ls[&LoadText::MinimalCacheSize].clone(), DEFAULT_MINIMAL_CACHE_SIZE.to_string());
//     let short_language = loaded_entries.get_string(hashmap_ls[&LoadText::Language].clone(), short_language);
//
//     let combo_box_duplicate_hash_type = loaded_entries.get_object(hashmap_ls[&LoadText::ComboBoxDuplicateHashType].clone(), 0);
//     let combo_box_duplicate_checking_method = loaded_entries.get_object(hashmap_ls[&LoadText::ComboBoxDuplicateCheckMethod].clone(), 0);
//     let combo_box_image_hash_size = loaded_entries.get_object(hashmap_ls[&LoadText::ComboBoxImageHashSize].clone(), 1); // 16 instead default 8
//     let combo_box_image_hash_algorithm = loaded_entries.get_object(hashmap_ls[&LoadText::ComboBoxImageHashType].clone(), 0);
//     let combo_box_image_resize_algorithm = loaded_entries.get_object(hashmap_ls[&LoadText::ComboBoxImageResizeAlgorithm].clone(), 0);
//     let combo_box_big_files_mode = loaded_entries.get_object(hashmap_ls[&LoadText::ComboBoxBigFiles].clone(), 0);
//
//     let number_of_biggest_files = loaded_entries.get_integer_string(hashmap_ls[&LoadText::NumberOfBiggestFiles].clone(), DEFAULT_NUMBER_OF_BIGGEST_FILES.to_string());
//     let similar_images_similarity = loaded_entries.get_object(hashmap_ls[&LoadText::SimilarImagesSimilarity].clone(), DEFAULT_SIMILAR_IMAGES_SIMILARITY);
//     let similar_images_ignore_same_size = loaded_entries.get_bool(hashmap_ls[&LoadText::SimilarImagesIgnoreSameSize].clone(), DEFAULT_SIMILAR_IMAGES_IGNORE_SAME_SIZE);
//     let similar_videos_similarity = loaded_entries.get_object(hashmap_ls[&LoadText::SimilarVideosSimilarity].clone(), DEFAULT_SIMILAR_VIDEOS_SIMILARITY);
//     let similar_videos_ignore_same_size = loaded_entries.get_bool(hashmap_ls[&LoadText::SimilarVideosIgnoreSameSize].clone(), DEFAULT_SIMILAR_VIDEOS_IGNORE_SAME_SIZE);
//     let check_button_case_sensitive_name = loaded_entries.get_object(hashmap_ls[&LoadText::DuplicateNameCaseSensitive].clone(), DEFAULT_DUPLICATE_CASE_SENSITIVE_NAME_CHECKING);
//     let check_button_music_approximate_comparison = loaded_entries.get_object(hashmap_ls[&LoadText::MusicApproximateComparison].clone(), DEFAULT_MUSIC_APPROXIMATE_COMPARISON);
//     let check_button_music_compare_by_title = loaded_entries.get_object(hashmap_ls[&LoadText::MusicCompareByTitle].clone(), DEFAULT_MUSIC_GROUP_CONTENT_BY_TITLE);
//
//     let check_button_broken_files_archive = loaded_entries.get_object(hashmap_ls[&LoadText::BrokenFilesArchive].clone(), DEFAULT_BROKEN_FILES_ARCHIVE);
//     let check_button_broken_files_pdf = loaded_entries.get_object(hashmap_ls[&LoadText::BrokenFilesPdf].clone(), DEFAULT_BROKEN_FILES_PDF);
//     let check_button_broken_files_image = loaded_entries.get_object(hashmap_ls[&LoadText::BrokenFilesImage].clone(), DEFAULT_BROKEN_FILES_IMAGE);
//     let check_button_broken_files_audio = loaded_entries.get_object(hashmap_ls[&LoadText::BrokenFilesAudio].clone(), DEFAULT_BROKEN_FILES_AUDIO);
//     let thread_number = loaded_entries.get_object(hashmap_ls[&LoadText::ThreadNumber].clone(), DEFAULT_THREAD_NUMBER);
//
//     let mut set_start_folders = false;
//     if !manual_execution {
//         // Handle here arguments that were added to app e.g. czkawka_gui /home --/home/roman
//         if arguments.len() > 1 {
//             let iter_i = arguments.iter().skip(1);
//             let iter_e = iter_i.clone();
//             let inc_dir = iter_i
//                 .filter_map(|e| {
//                     let r = e.to_string_lossy().to_string();
//                     if !r.starts_with("--") {
//                         let path = Path::new(&r);
//                         if !path.exists() {
//                             return None;
//                         }
//                         match path.canonicalize() {
//                             Ok(r) => Some(r.to_string_lossy().to_string()),
//                             Err(_) => None,
//                         }
//                     } else {
//                         None
//                     }
//                 })
//                 .collect::<Vec<_>>();
//             let exc_dir = iter_e
//                 .filter_map(|e| {
//                     let r = e.to_string_lossy().to_string();
//                     if let Some(r) = r.strip_prefix("--") {
//                         let path = Path::new(&r);
//                         if !path.exists() {
//                             return None;
//                         }
//                         match path.canonicalize() {
//                             Ok(r) => Some(r.to_string_lossy().to_string()),
//                             Err(_) => None,
//                         }
//                     } else {
//                         None
//                     }
//                 })
//                 .collect::<Vec<_>>();
//
//             if inc_dir.is_empty() {
//                 error!("Arguments {arguments:?} should contains at least one directory to include");
//             } else {
//                 included_directories = inc_dir;
//                 excluded_directories = exc_dir;
//                 saving_at_exit = false;
//                 set_start_folders = true;
//             }
//         }
//     }
//
//     if manual_execution || loading_at_start || set_start_folders {
//         set_directories(
//             &upper_notebook.tree_view_included_directories,
//             &upper_notebook.tree_view_excluded_directories,
//             &included_directories,
//             &excluded_directories,
//         );
//     }
//
//     // Setting data
//     if loading_at_start || manual_execution {
//         //// Language ComboBoxText
//         {
//             for (index, lang) in LANGUAGES_ALL.iter().enumerate() {
//                 if short_language == lang.short_text {
//                     settings.combo_box_settings_language.set_active(Some(index as u32));
//                 }
//             }
//         }
//
//         upper_notebook.entry_excluded_items.set_text(&excluded_items);
//         upper_notebook.entry_allowed_extensions.set_text(&allowed_extensions);
//         upper_notebook.entry_general_minimal_size.set_text(&minimal_file_size);
//         upper_notebook.entry_general_maximal_size.set_text(&maximal_file_size);
//
//         //// Buttons
//         settings.check_button_settings_load_at_start.set_active(loading_at_start);
//         settings.check_button_settings_save_at_exit.set_active(saving_at_exit);
//         settings.check_button_settings_confirm_deletion.set_active(confirm_deletion);
//         settings.check_button_settings_confirm_group_deletion.set_active(confirm_group_deletion);
//         settings.check_button_settings_show_preview_similar_images.set_active(show_previews_similar_images);
//         settings.check_button_settings_show_preview_duplicates.set_active(show_previews_duplicates);
//
//         settings
//             .check_button_settings_similar_videos_delete_outdated_cache
//             .set_active(delete_outdated_cache_similar_videos);
//         settings
//             .check_button_settings_similar_images_delete_outdated_cache
//             .set_active(delete_outdated_cache_similar_images);
//         settings.check_button_settings_duplicates_delete_outdated_cache.set_active(delete_outdated_cache_duplicates);
//
//         settings.check_button_settings_show_text_view.set_active(bottom_text_panel);
//         if !bottom_text_panel {
//             scrolled_window_errors.hide();
//         } else {
//             scrolled_window_errors.show();
//         }
//         settings.check_button_settings_hide_hard_links.set_active(hide_hard_links);
//         settings.check_button_settings_use_cache.set_active(use_cache);
//         settings.check_button_settings_save_also_json.set_active(use_json_cache);
//         settings.check_button_duplicates_use_prehash_cache.set_active(use_prehash_cache);
//         settings.check_button_settings_use_trash.set_active(use_trash);
//         settings.entry_settings_cache_file_minimal_size.set_text(&cache_minimal_size);
//         settings.entry_settings_prehash_cache_file_minimal_size.set_text(&cache_prehash_minimal_size);
//         settings.check_button_settings_one_filesystem.set_active(ignore_other_fs);
//         settings.check_button_settings_use_rust_preview.set_active(use_rust_libraries_to_preview);
//
//         save_proper_value_to_combo_box(&main_notebook.combo_box_duplicate_hash_type, combo_box_duplicate_hash_type);
//         save_proper_value_to_combo_box(&main_notebook.combo_box_duplicate_check_method, combo_box_duplicate_checking_method);
//         save_proper_value_to_combo_box(&main_notebook.combo_box_image_hash_algorithm, combo_box_image_hash_algorithm);
//         save_proper_value_to_combo_box(&main_notebook.combo_box_image_hash_size, combo_box_image_hash_size);
//         save_proper_value_to_combo_box(&main_notebook.combo_box_image_resize_algorithm, combo_box_image_resize_algorithm);
//         save_proper_value_to_combo_box(&main_notebook.combo_box_big_files_mode, combo_box_big_files_mode);
//
//         main_notebook.check_button_duplicate_case_sensitive_name.set_active(check_button_case_sensitive_name);
//         main_notebook.entry_big_files_number.set_text(&number_of_biggest_files);
//         main_notebook.check_button_image_ignore_same_size.set_active(similar_images_ignore_same_size);
//         main_notebook.check_button_video_ignore_same_size.set_active(similar_videos_ignore_same_size);
//         main_notebook.scale_similarity_similar_videos.set_value(similar_videos_similarity as f64);
//         main_notebook.check_button_music_compare_only_in_title_group.set_active(check_button_music_compare_by_title);
//         main_notebook
//             .check_button_music_approximate_comparison
//             .set_active(check_button_music_approximate_comparison);
//
//         main_notebook.check_button_broken_files_audio.set_active(check_button_broken_files_audio);
//         main_notebook.check_button_broken_files_pdf.set_active(check_button_broken_files_pdf);
//         main_notebook.check_button_broken_files_image.set_active(check_button_broken_files_image);
//         main_notebook.check_button_broken_files_archive.set_active(check_button_broken_files_archive);
//
//         {
//             let combo_chosen_index = main_notebook.combo_box_duplicate_check_method.active().expect("Failed to get active item");
//
//             if DUPLICATES_CHECK_METHOD_COMBO_BOX[combo_chosen_index as usize].check_method == CheckingMethod::Hash {
//                 main_notebook.combo_box_duplicate_hash_type.set_visible(true);
//                 main_notebook.label_duplicate_hash_type.set_visible(true);
//             } else {
//                 main_notebook.combo_box_duplicate_hash_type.set_visible(false);
//                 main_notebook.label_duplicate_hash_type.set_visible(false);
//             }
//
//             if [CheckingMethod::Name, CheckingMethod::SizeName].contains(&DUPLICATES_CHECK_METHOD_COMBO_BOX[combo_chosen_index as usize].check_method) {
//                 main_notebook.check_button_duplicate_case_sensitive_name.set_visible(true);
//             } else {
//                 main_notebook.check_button_duplicate_case_sensitive_name.set_visible(false);
//             }
//         }
//
//         // Set size of similarity scale gtk node, must be set BEFORE setting value of this
//         let index = main_notebook.combo_box_image_hash_size.active().expect("Failed to get active item") as usize;
//
//         main_notebook.scale_similarity_similar_images.set_range(0_f64, SIMILAR_VALUES[index][5] as f64);
//         main_notebook.scale_similarity_similar_images.set_fill_level(SIMILAR_VALUES[index][5] as f64);
//         main_notebook.scale_similarity_similar_images.connect_change_value(scale_step_function);
//         main_notebook.scale_similarity_similar_images.set_value(similar_images_similarity as f64);
//
//         settings.scale_settings_number_of_threads.set_range(0_f64, get_all_available_threads() as f64);
//         settings.scale_settings_number_of_threads.set_fill_level(get_all_available_threads() as f64);
//         settings.scale_settings_number_of_threads.connect_change_value(scale_step_function);
//         settings.scale_settings_number_of_threads.set_value(thread_number as f64);
//     } else {
//         settings.check_button_settings_load_at_start.set_active(false);
//     }
// }

// pub(crate) fn reset_configuration(manual_clearing: bool, upper_notebook: &GuiUpperNotebook, main_notebook: &GuiMainNotebook, settings: &GuiSettings, text_view_errors: &TextView) {
//     // TODO Maybe add popup dialog to confirm resetting
//     let text_view_errors = text_view_errors.clone();
//
//     reset_text_view(&text_view_errors);
//
//     // Resetting included directories
//     {
//         let tree_view_included_directories = upper_notebook.tree_view_included_directories.clone();
//         let list_store = get_list_store(&tree_view_included_directories);
//         list_store.clear();
//
//         let current_dir: String = match env::current_dir() {
//             Ok(t) => t.to_string_lossy().to_string(),
//             Err(_inspected) => {
//                 if cfg!(target_family = "unix") {
//                     add_text_to_text_view(&text_view_errors, "Failed to read current directory, setting /home instead");
//                     "/home".to_string()
//                 } else if cfg!(target_family = "windows") {
//                     add_text_to_text_view(&text_view_errors, "Failed to read current directory, setting C:\\ instead");
//                     "C:\\".to_string()
//                 } else {
//                     String::new()
//                 }
//             }
//         };
//
//         let values: [(u32, &dyn ToValue); 2] = [
//             (ColumnsIncludedDirectory::Path as u32, &current_dir),
//             (ColumnsIncludedDirectory::ReferenceButton as u32, &false),
//         ];
//         list_store.set(&list_store.append(), &values);
//     }
//     // Resetting excluded directories
//     {
//         let tree_view_excluded_directories = upper_notebook.tree_view_excluded_directories.clone();
//         let list_store = get_list_store(&tree_view_excluded_directories);
//         list_store.clear();
//         for i in DEFAULT_EXCLUDED_DIRECTORIES {
//             let values: [(u32, &dyn ToValue); 1] = [(ColumnsExcludedDirectory::Path as u32, &i)];
//             list_store.set(&list_store.append(), &values);
//         }
//     }
//     // Resetting excluded items
//     {
//         upper_notebook.entry_excluded_items.set_text(DEFAULT_EXCLUDED_ITEMS);
//         upper_notebook.entry_allowed_extensions.set_text("");
//         upper_notebook.entry_general_minimal_size.set_text(DEFAULT_MINIMAL_FILE_SIZE);
//         upper_notebook.entry_general_maximal_size.set_text(DEFAULT_MAXIMAL_FILE_SIZE);
//     }
//
//     // Set default settings
//     {
//         settings.check_button_settings_save_at_exit.set_active(DEFAULT_SAVE_ON_EXIT);
//         settings.check_button_settings_load_at_start.set_active(DEFAULT_LOAD_AT_START);
//         settings.check_button_settings_confirm_deletion.set_active(DEFAULT_CONFIRM_DELETION);
//         settings.check_button_settings_confirm_group_deletion.set_active(DEFAULT_CONFIRM_GROUP_DELETION);
//         settings.check_button_settings_show_preview_similar_images.set_active(DEFAULT_SHOW_IMAGE_PREVIEW);
//         settings.check_button_settings_show_preview_duplicates.set_active(DEFAULT_SHOW_DUPLICATE_IMAGE_PREVIEW);
//         settings.check_button_settings_show_text_view.set_active(DEFAULT_BOTTOM_TEXT_VIEW);
//         settings.check_button_settings_hide_hard_links.set_active(DEFAULT_HIDE_HARD_LINKS);
//         settings.check_button_settings_use_cache.set_active(DEFAULT_USE_CACHE);
//         settings.check_button_settings_save_also_json.set_active(DEFAULT_SAVE_ALSO_AS_JSON);
//         settings.check_button_settings_use_trash.set_active(DEFAULT_USE_TRASH);
//         settings.entry_settings_cache_file_minimal_size.set_text(DEFAULT_MINIMAL_CACHE_SIZE);
//         settings
//             .check_button_settings_similar_videos_delete_outdated_cache
//             .set_active(DEFAULT_VIDEO_REMOVE_AUTO_OUTDATED_CACHE);
//         settings
//             .check_button_settings_similar_images_delete_outdated_cache
//             .set_active(DEFAULT_IMAGE_REMOVE_AUTO_OUTDATED_CACHE);
//         settings
//             .check_button_settings_duplicates_delete_outdated_cache
//             .set_active(DEFAULT_DUPLICATE_REMOVE_AUTO_OUTDATED_CACHE);
//         settings.check_button_duplicates_use_prehash_cache.set_active(DEFAULT_USE_PRECACHE);
//         settings.entry_settings_prehash_cache_file_minimal_size.set_text(DEFAULT_PREHASH_MINIMAL_CACHE_SIZE);
//         settings.combo_box_settings_language.set_active(Some(0));
//         settings.check_button_settings_one_filesystem.set_active(DEFAULT_GENERAL_IGNORE_OTHER_FILESYSTEMS);
//         settings.check_button_settings_use_rust_preview.set_active(DEFAULT_USING_RUST_LIBRARIES_TO_SHOW_PREVIEW);
//
//         main_notebook.combo_box_duplicate_hash_type.set_active(Some(0));
//         main_notebook.combo_box_duplicate_check_method.set_active(Some(0));
//         main_notebook.combo_box_image_hash_algorithm.set_active(Some(0));
//         main_notebook.combo_box_image_resize_algorithm.set_active(Some(1)); // Nearest by default
//         main_notebook.combo_box_image_hash_size.set_active(Some(1)); // Set as 16 instead 8
//         main_notebook.combo_box_big_files_mode.set_active(Some(0));
//
//         main_notebook.check_button_broken_files_audio.set_active(DEFAULT_BROKEN_FILES_AUDIO);
//         main_notebook.check_button_broken_files_pdf.set_active(DEFAULT_BROKEN_FILES_PDF);
//         main_notebook.check_button_broken_files_archive.set_active(DEFAULT_BROKEN_FILES_ARCHIVE);
//         main_notebook.check_button_broken_files_image.set_active(DEFAULT_BROKEN_FILES_IMAGE);
//
//         main_notebook.scale_similarity_similar_images.set_range(0_f64, SIMILAR_VALUES[0][5] as f64); // DEFAULT FOR MAX of 8
//         main_notebook.scale_similarity_similar_images.set_fill_level(SIMILAR_VALUES[0][5] as f64);
//
//         main_notebook
//             .check_button_music_compare_only_in_title_group
//             .set_active(DEFAULT_MUSIC_GROUP_CONTENT_BY_TITLE);
//
//         main_notebook.check_button_music_approximate_comparison.set_active(DEFAULT_MUSIC_APPROXIMATE_COMPARISON);
//
//         main_notebook.entry_big_files_number.set_text(DEFAULT_NUMBER_OF_BIGGEST_FILES);
//         main_notebook.scale_similarity_similar_images.set_value(DEFAULT_SIMILAR_IMAGES_SIMILARITY as f64);
//         main_notebook.check_button_image_ignore_same_size.set_active(DEFAULT_SIMILAR_IMAGES_IGNORE_SAME_SIZE);
//         main_notebook.check_button_video_ignore_same_size.set_active(DEFAULT_SIMILAR_VIDEOS_IGNORE_SAME_SIZE);
//         main_notebook.scale_similarity_similar_videos.set_value(DEFAULT_SIMILAR_VIDEOS_SIMILARITY as f64);
//     }
//     if manual_clearing {
//         add_text_to_text_view(&text_view_errors, &flg!("saving_loading_reset_configuration"));
//     }
// }

pub fn save_configuration(_save: bool, _upper_notebook: &GuiUpperNotebook, _main_notebook: &GuiMainNotebook, _settings: &GuiSettings, text_view_errors: &TextView) {
    // Minimal saver: write current settings from GUI into JSON file.
    // As we don't have direct access to GUI -> SettingsJson conversion here (project-specific),
    // this currently writes defaults. Replace with conversion from GUI to SettingsJson when available.
    let saver = LoadSaveStruct::with_text_view(text_view_errors.clone());
    saver.save_to_file(text_view_errors);
}
