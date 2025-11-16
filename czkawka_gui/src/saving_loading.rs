use std::env;
use std::path::{Path, PathBuf};

use czkawka_core::common::basic_gui_cli::CliResult;
use czkawka_core::common::config_cache_path::get_config_cache_path;
use czkawka_core::common::get_all_available_threads;
use czkawka_core::common::items::DEFAULT_EXCLUDED_ITEMS;
use czkawka_core::common::model::CheckingMethod;
use czkawka_core::tools::similar_images::SIMILAR_VALUES;
use gtk4::prelude::*;
use gtk4::{ListStore, ScrolledWindow, TextView, TreeView};
use serde::{Deserialize, Serialize};

use crate::flg;
use crate::gui_structs::common_tree_view::TreeViewListStoreTrait;
use crate::gui_structs::common_upper_tree_view::UpperTreeViewEnum;
use crate::gui_structs::gui_main_notebook::GuiMainNotebook;
use crate::gui_structs::gui_settings::GuiSettings;
use crate::gui_structs::gui_upper_notebook::GuiUpperNotebook;
use crate::help_combo_box::DUPLICATES_CHECK_METHOD_COMBO_BOX;
use crate::help_functions::{add_text_to_text_view, append_row_to_list_store, get_from_list_store_fnc, get_string_from_list_store, reset_text_view, scale_step_function};
use crate::helpers::enums::{ColumnsExcludedDirectory, ColumnsIncludedDirectory};
use crate::language_functions::{LANGUAGES_ALL, get_language_from_combo_box_text};

const SAVE_FILE_NAME_JSON: &str = "czkawka_gui_config.json";

const DEFAULT_SAVE_ON_EXIT: bool = true;
const DEFAULT_LOAD_AT_START: bool = true;
const DEFAULT_CONFIRM_DELETION: bool = true;
const DEFAULT_CONFIRM_LINK_DELETION: bool = true;
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
}

impl LoadSaveStruct {
    pub(crate) fn with_text_view() -> Self {
        Self {
            settings: SettingsJson::default(),
        }
    }

    fn open_save_file_path() -> Option<PathBuf> {
        let config_dir = get_config_cache_path()?.config_folder;
        Some(config_dir.join(Path::new(SAVE_FILE_NAME_JSON)))
    }

    pub(crate) fn open_and_read_content(&mut self, text_view_errors: &TextView, manual_execution: bool) {
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
        let Some(json_file) = Self::open_save_file_path() else {
            add_text_to_text_view(
                text_view_errors,
                &flg!(
                    "saving_loading_failed_to_create_config_file",
                    path = SAVE_FILE_NAME_JSON,
                    reason = "config directory not found"
                ),
            );
            return;
        };

        match serde_json::to_string_pretty(&self.settings) {
            Ok(json_string) => match std::fs::write(&json_file, json_string) {
                Ok(()) => {
                    add_text_to_text_view(text_view_errors, &flg!("saving_loading_saving_success", name = json_file.to_string_lossy().to_string()));
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
                add_text_to_text_view(
                    text_view_errors,
                    &flg!("saving_loading_saving_failure", name = json_file.to_string_lossy().to_string(), reason = e.to_string()),
                );
            }
        }
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

    #[serde(default)]
    pub reference_directories: Vec<String>,

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

    #[serde(default = "default_confirm_link_deletion")]
    pub confirm_deletion_links: bool,

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
fn default_confirm_link_deletion() -> bool {
    DEFAULT_CONFIRM_LINK_DELETION
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

fn set_included_reference_folders(tree_view_included_directories: &TreeView, included_directories: &[String], referenced_directories: &[String]) {
    let list_store = tree_view_included_directories.get_model();
    list_store.clear();

    // Referenced directories must be also in included directories
    let referenced_directories: Vec<String> = referenced_directories.iter().filter(|s| included_directories.contains(s)).cloned().collect();

    let only_included_directories: Vec<String> = included_directories.iter().filter(|s| !referenced_directories.contains(s)).cloned().collect();

    for (directories, is_referenced) in [(only_included_directories, false), (referenced_directories, true)] {
        for directory in directories {
            let values: [(u32, &dyn ToValue); 2] = [
                (ColumnsIncludedDirectory::Path as u32, &directory),
                (ColumnsIncludedDirectory::ReferenceButton as u32, &is_referenced),
            ];
            append_row_to_list_store(&list_store, &values);
        }
    }
}

fn set_configuration_to_gui_internal(upper_notebook: &GuiUpperNotebook, main_notebook: &GuiMainNotebook, settings: &GuiSettings, default_config: &SettingsJson) {
    let tree_view_included_directories = upper_notebook.common_upper_tree_views.get_tree_view(UpperTreeViewEnum::IncludedDirectories);
    let tree_view_excluded_directories = upper_notebook.common_upper_tree_views.get_tree_view(UpperTreeViewEnum::ExcludedDirectories);

    // Resetting included directories
    {
        set_included_reference_folders(tree_view_included_directories, &default_config.included_directories, &default_config.reference_directories);
    }
    // Resetting excluded directories
    {
        let list_store = tree_view_excluded_directories.get_model();
        list_store.clear();
        for i in default_config.excluded_directories.clone() {
            let values: [(u32, &dyn ToValue); 1] = [(ColumnsExcludedDirectory::Path as u32, &i)];
            append_row_to_list_store(&list_store, &values);
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
        settings
            .check_button_settings_confirm_group_deletion
            .set_active(default_config.confirm_deletion_all_files_in_group);
        settings.check_button_settings_confirm_link.set_active(default_config.confirm_deletion_links);
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

        let lang_idx = LANGUAGES_ALL.iter().position(|l| l.short_text == default_config.language).unwrap_or(0);
        settings.combo_box_settings_language.set_active(Some(lang_idx as u32));

        settings.check_button_settings_one_filesystem.set_active(default_config.ignore_other_filesystems);
        settings.check_button_settings_use_rust_preview.set_active(default_config.use_rust_libraries_to_preview);

        // Set combo boxes and check buttons as before
        main_notebook.combo_box_duplicate_hash_type.set_active(Some(default_config.combo_box_duplicate_hash_type));
        main_notebook
            .combo_box_duplicate_check_method
            .set_active(Some(default_config.combo_box_duplicate_check_method));
        main_notebook.combo_box_image_hash_algorithm.set_active(Some(default_config.combo_box_image_hash_type));
        main_notebook
            .combo_box_image_resize_algorithm
            .set_active(Some(default_config.combo_box_image_resize_algorithm));
        main_notebook.combo_box_image_hash_size.set_active(Some(default_config.combo_box_image_hash_size));
        main_notebook.combo_box_big_files_mode.set_active(Some(default_config.combo_box_big_files_mode));

        main_notebook.check_button_broken_files_audio.set_active(default_config.broken_files_audio);
        main_notebook.check_button_broken_files_pdf.set_active(default_config.broken_files_pdf);
        main_notebook.check_button_broken_files_archive.set_active(default_config.broken_files_archive);
        main_notebook.check_button_broken_files_image.set_active(default_config.broken_files_image);

        // Set similarity scale range/value based on chosen image hash size index
        let index = default_config.combo_box_image_hash_size as usize;
        let max_similar = SIMILAR_VALUES[index][5] as f64;
        main_notebook.scale_similarity_similar_images.set_range(0_f64, max_similar);
        main_notebook.scale_similarity_similar_images.set_fill_level(max_similar);
        main_notebook.scale_similarity_similar_images.connect_change_value(scale_step_function);
        main_notebook.scale_similarity_similar_images.set_value(default_config.similar_images_similarity);

        // Set similar videos scale value
        main_notebook.scale_similarity_similar_videos.set_value(default_config.similar_videos_similarity);

        // Update duplicate-related widget visibility according to chosen method
        {
            let combo_chosen_index = main_notebook.combo_box_duplicate_check_method.active().unwrap_or(0) as usize;
            if DUPLICATES_CHECK_METHOD_COMBO_BOX[combo_chosen_index].check_method == CheckingMethod::Hash {
                main_notebook.combo_box_duplicate_hash_type.set_visible(true);
                main_notebook.label_duplicate_hash_type.set_visible(true);
            } else {
                main_notebook.combo_box_duplicate_hash_type.set_visible(false);
                main_notebook.label_duplicate_hash_type.set_visible(false);
            }

            if [CheckingMethod::Name, CheckingMethod::SizeName].contains(&DUPLICATES_CHECK_METHOD_COMBO_BOX[combo_chosen_index].check_method) {
                main_notebook.check_button_duplicate_case_sensitive_name.set_visible(true);
            } else {
                main_notebook.check_button_duplicate_case_sensitive_name.set_visible(false);
            }
        }

        // Threads slider
        settings.scale_settings_number_of_threads.set_range(0_f64, get_all_available_threads() as f64);
        settings.scale_settings_number_of_threads.set_fill_level(get_all_available_threads() as f64);
        settings.scale_settings_number_of_threads.connect_change_value(scale_step_function);
        settings.scale_settings_number_of_threads.set_value(default_config.thread_number as f64);
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

fn gui_to_settings(upper_notebook: &GuiUpperNotebook, main_notebook: &GuiMainNotebook, settings: &GuiSettings) -> SettingsJson {
    let tree_view_included_directories = &upper_notebook.common_upper_tree_views.get_tree_view(UpperTreeViewEnum::IncludedDirectories);
    let tree_view_excluded_directories = &upper_notebook.common_upper_tree_views.get_tree_view(UpperTreeViewEnum::ExcludedDirectories);

    // Gather directories
    let included_directories = get_string_from_list_store(tree_view_included_directories, ColumnsIncludedDirectory::Path as i32, None);
    let excluded_directories = get_string_from_list_store(tree_view_excluded_directories, ColumnsExcludedDirectory::Path as i32, None);

    let ref_fnc: &dyn Fn(&ListStore, &gtk4::TreeIter, &mut Vec<String>) = &|list_store, tree_iter, vec| {
        if list_store.get::<bool>(tree_iter, ColumnsIncludedDirectory::ReferenceButton as i32) {
            vec.push(list_store.get::<String>(tree_iter, ColumnsIncludedDirectory::Path as i32));
        }
    };

    let reference_directories = get_from_list_store_fnc(tree_view_included_directories, ref_fnc);

    // Language short text
    let language_text = match settings.combo_box_settings_language.active_text() {
        Some(t) => get_language_from_combo_box_text(&t).short_text.to_string(),
        None => "en".to_string(),
    };

    SettingsJson {
        included_directories,
        reference_directories,
        excluded_directories,
        excluded_items: upper_notebook.entry_excluded_items.text().to_string(),
        allowed_extensions: upper_notebook.entry_allowed_extensions.text().to_string(),
        minimal_file_size: upper_notebook.entry_general_minimal_size.text().to_string(),
        maximal_file_size: upper_notebook.entry_general_maximal_size.text().to_string(),
        save_at_exit: settings.check_button_settings_save_at_exit.is_active(),
        load_at_start: settings.check_button_settings_load_at_start.is_active(),
        confirm_deletion_files: settings.check_button_settings_confirm_deletion.is_active(),
        confirm_deletion_all_files_in_group: settings.check_button_settings_confirm_group_deletion.is_active(),
        confirm_deletion_links: settings.check_button_settings_confirm_link.is_active(),
        show_bottom_text_panel: settings.check_button_settings_show_text_view.is_active(),
        hide_hard_links: settings.check_button_settings_hide_hard_links.is_active(),
        use_cache: settings.check_button_settings_use_cache.is_active(),
        use_json_cache_file: settings.check_button_settings_save_also_json.is_active(),
        delete_to_trash: settings.check_button_settings_use_trash.is_active(),
        minimal_cache_size: settings.entry_settings_cache_file_minimal_size.text().to_string(),
        image_preview_image: settings.check_button_settings_show_preview_similar_images.is_active(),
        duplicate_preview_image: settings.check_button_settings_show_preview_duplicates.is_active(),
        duplicate_delete_outdated_cache_entries: settings.check_button_settings_duplicates_delete_outdated_cache.is_active(),
        image_delete_outdated_cache_entries: settings.check_button_settings_similar_images_delete_outdated_cache.is_active(),
        video_delete_outdated_cache_entries: settings.check_button_settings_similar_videos_delete_outdated_cache.is_active(),
        use_prehash_cache: settings.check_button_duplicates_use_prehash_cache.is_active(),
        minimal_prehash_cache_size: settings.entry_settings_prehash_cache_file_minimal_size.text().to_string(),
        language: language_text,
        combo_box_duplicate_hash_type: main_notebook.combo_box_duplicate_hash_type.active().unwrap_or(0),
        combo_box_duplicate_check_method: main_notebook.combo_box_duplicate_check_method.active().unwrap_or(0),
        combo_box_image_resize_algorithm: main_notebook.combo_box_image_resize_algorithm.active().unwrap_or(0),
        combo_box_image_hash_type: main_notebook.combo_box_image_hash_algorithm.active().unwrap_or(0),
        combo_box_image_hash_size: main_notebook.combo_box_image_hash_size.active().unwrap_or(1),
        number_of_biggest_files: main_notebook.entry_big_files_number.text().to_string(),
        similar_images_similarity: main_notebook.scale_similarity_similar_images.value(),
        similar_images_ignore_same_size: main_notebook.check_button_image_ignore_same_size.is_active(),
        similar_videos_similarity: main_notebook.scale_similarity_similar_videos.value(),
        similar_videos_ignore_same_size: main_notebook.check_button_video_ignore_same_size.is_active(),
        music_approximate_comparison: main_notebook.check_button_music_approximate_comparison.is_active(),
        duplicate_name_case_sensitive: main_notebook.check_button_duplicate_case_sensitive_name.is_active(),
        combo_box_big_files_mode: main_notebook.combo_box_big_files_mode.active().unwrap_or(0),
        broken_files_pdf: main_notebook.check_button_broken_files_pdf.is_active(),
        broken_files_audio: main_notebook.check_button_broken_files_audio.is_active(),
        broken_files_image: main_notebook.check_button_broken_files_image.is_active(),
        broken_files_archive: main_notebook.check_button_broken_files_archive.is_active(),
        ignore_other_filesystems: settings.check_button_settings_one_filesystem.is_active(),
        thread_number: settings.scale_settings_number_of_threads.value() as u32,
        music_compare_by_title: main_notebook.check_button_music_compare_only_in_title_group.is_active(),
        use_rust_libraries_to_preview: settings.check_button_settings_use_rust_preview.is_active(),
    }
}

#[allow(clippy::allow_attributes)]
#[allow(clippy::useless_let_if_seq)] // TODO - rust with some version shows this
pub fn load_configuration(
    manual_execution: bool,
    upper_notebook: &GuiUpperNotebook,
    main_notebook: &GuiMainNotebook,
    settings: &GuiSettings,
    text_view_errors: &TextView,
    scrolled_window_errors: &ScrolledWindow,
    cli_result: Option<&CliResult>,
) {
    let mut loader = LoadSaveStruct::with_text_view();
    loader.open_and_read_content(text_view_errors, manual_execution);

    // Determine folders from CLI args (if any)
    let set_start_folders = cli_result.is_some();

    // Loaded settings (from file or defaults)
    let loaded_settings = loader.settings_mut();

    // Show/hide bottom text panel
    if !loaded_settings.show_bottom_text_panel {
        scrolled_window_errors.set_visible(false);
    } else {
        scrolled_window_errors.set_visible(true);
    }

    reset_text_view(text_view_errors);

    let included_directories;
    let excluded_directories;
    let referenced_directories;
    if let Some(cli) = cli_result {
        included_directories = cli.included_items.clone();
        excluded_directories = cli.excluded_items.clone();
        referenced_directories = cli.referenced_items.clone();
    } else {
        included_directories = if !loaded_settings.included_directories.is_empty() {
            loaded_settings.included_directories.clone()
        } else {
            vec![get_current_directory()]
        };
        excluded_directories = if !loaded_settings.excluded_directories.is_empty() {
            loaded_settings.excluded_directories.clone()
        } else {
            DEFAULT_EXCLUDED_DIRECTORIES.iter().map(|s| s.to_string()).collect()
        };
        referenced_directories = loaded_settings
            .reference_directories
            .clone()
            .into_iter()
            .filter(|s| included_directories.contains(s))
            .collect();
    }

    // When we manually load configuration, then we want them to be set, so allow it
    // When we start app with load_at_start option, then we want to load them too
    if manual_execution || loaded_settings.load_at_start {
        set_configuration_to_gui_internal(upper_notebook, main_notebook, settings, loaded_settings);
    }

    // When starting app wtih arguments, we want to set folders
    if set_start_folders {
        set_directories(
            upper_notebook.common_upper_tree_views.get_tree_view(UpperTreeViewEnum::IncludedDirectories),
            upper_notebook.common_upper_tree_views.get_tree_view(UpperTreeViewEnum::ExcludedDirectories),
            &included_directories,
            &referenced_directories,
            &excluded_directories,
        );
        // When using CLI args, disable saving at exit by default
        // User still may enable it manually
        settings.check_button_settings_save_at_exit.set_active(false);
    }
}

fn set_directories(
    tree_view_included_directories: &TreeView,
    tree_view_excluded_directories: &TreeView,
    included_directories: &[String],
    referenced_directories: &[String],
    excluded_directories: &[String],
) {
    set_included_reference_folders(tree_view_included_directories, included_directories, referenced_directories);

    //// Exclude Directories
    let list_store = tree_view_excluded_directories.get_model();
    list_store.clear();

    for directory in excluded_directories {
        let values: [(u32, &dyn ToValue); 1] = [(ColumnsExcludedDirectory::Path as u32, &directory)];
        append_row_to_list_store(&list_store, &values);
    }
}

pub fn save_configuration(manual_execution: bool, upper_notebook: &GuiUpperNotebook, main_notebook: &GuiMainNotebook, settings: &GuiSettings, text_view_errors: &TextView) {
    let check_button_settings_save_at_exit = settings.check_button_settings_save_at_exit.clone();
    let text_view_errors = text_view_errors.clone();

    reset_text_view(&text_view_errors);

    if !manual_execution && !check_button_settings_save_at_exit.is_active() {
        // When check button is deselected, not save configuration at exit
        return;
    }

    let mut saver = LoadSaveStruct::with_text_view();
    saver.settings = gui_to_settings(upper_notebook, main_notebook, settings);
    saver.save_to_file(&text_view_errors);
}
pub(crate) fn reset_configuration(manual_clearing: bool, upper_notebook: &GuiUpperNotebook, main_notebook: &GuiMainNotebook, settings: &GuiSettings, text_view_errors: &TextView) {
    // TODO Maybe add popup dialog to confirm resetting
    let text_view_errors = text_view_errors.clone();

    let default_config = SettingsJson {
        included_directories: vec![get_current_directory()],
        ..Default::default()
    };

    reset_text_view(&text_view_errors);

    set_configuration_to_gui_internal(upper_notebook, main_notebook, settings, &default_config);

    if manual_clearing {
        add_text_to_text_view(&text_view_errors, &flg!("saving_loading_reset_configuration"));
    }
}
