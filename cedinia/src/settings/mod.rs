pub mod gui_settings_values;

use std::path::PathBuf;

use czkawka_core::common::config_cache_path::get_config_cache_path;
use log::{error, info};
use serde::{Deserialize, Serialize};

use crate::settings::gui_settings_values::StringComboBoxItems;

fn default_check_method() -> String {
    "hash".to_string()
}
fn default_hash_type() -> String {
    "blake3".to_string()
}
fn default_hash_size() -> String {
    "16".to_string()
}
fn default_min_size_kb_idx() -> i32 {
    1
}
fn ttrue() -> bool {
    true
}
fn default_similarity_preset() -> String {
    "medium".to_string()
}
fn default_search_mode() -> String {
    "biggest".to_string()
}
fn default_big_files_count() -> String {
    "50".to_string()
}
fn default_min_file_size_idx() -> i32 {
    0
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CediniaSettings {
    #[serde(default = "ttrue")]
    pub use_cache: bool,
    #[serde(default = "ttrue")]
    pub ignore_hidden: bool,
    #[serde(default = "default_min_file_size_idx")]
    pub min_file_size_idx: i32,
    #[serde(default)]
    pub excluded_items: String,
    #[serde(default)]
    pub allowed_extensions: String,
    #[serde(default)]
    pub excluded_extensions: String,

    #[serde(default = "default_check_method")]
    pub duplicates_check_method: String,
    #[serde(default = "default_hash_type")]
    pub duplicates_hash_type: String,
    #[serde(default = "default_min_size_kb_idx")]
    pub duplicates_min_size_kb_idx: i32,

    #[serde(default = "default_similarity_preset")]
    pub similar_images_similarity_preset: String,
    #[serde(default = "default_hash_size")]
    pub similar_images_hash_size: String,

    #[serde(default = "default_search_mode")]
    pub big_files_search_mode: String,
    #[serde(default = "default_big_files_count")]
    pub big_files_count: String,
}

impl Default for CediniaSettings {
    fn default() -> Self {
        Self {
            use_cache: true,
            ignore_hidden: true,
            min_file_size_idx: default_min_file_size_idx(),
            excluded_items: String::new(),
            allowed_extensions: String::new(),
            excluded_extensions: String::new(),
            duplicates_check_method: default_check_method(),
            duplicates_hash_type: default_hash_type(),
            duplicates_min_size_kb_idx: default_min_size_kb_idx(),
            similar_images_similarity_preset: default_similarity_preset(),
            similar_images_hash_size: default_hash_size(),
            big_files_search_mode: default_search_mode(),
            big_files_count: default_big_files_count(),
        }
    }
}

fn get_config_file() -> Option<PathBuf> {
    let config_folder = get_config_cache_path()?.config_folder;
    Some(config_folder.join("cedinia_settings.json"))
}

pub fn load_settings() -> CediniaSettings {
    let Some(path) = get_config_file() else {
        info!("Cannot determine config path – using defaults");
        return CediniaSettings::default();
    };

    if !path.is_file() {
        info!("Settings file does not exist yet – using defaults");
        return CediniaSettings::default();
    }

    match std::fs::read_to_string(&path) {
        Ok(json) => match serde_json::from_str::<CediniaSettings>(&json) {
            Ok(s) => {
                info!("Settings loaded from {}", path.display());
                s
            }
            Err(e) => {
                error!("Cannot parse settings from {}: {e} – using defaults", path.display());
                CediniaSettings::default()
            }
        },
        Err(e) => {
            error!("Cannot read settings file {}: {e} – using defaults", path.display());
            CediniaSettings::default()
        }
    }
}

pub fn save_settings(settings: &CediniaSettings) {
    let Some(path) = get_config_file() else {
        error!("Cannot determine config path – settings not saved");
        return;
    };

    if let Some(parent) = path.parent() {
        if let Err(e) = std::fs::create_dir_all(parent) {
            error!("Cannot create config dir {}: {e}", parent.display());
            return;
        }
    }

    match serde_json::to_string_pretty(settings) {
        Ok(json) => {
            if let Err(e) = std::fs::write(&path, json) {
                error!("Cannot write settings to {}: {e}", path.display());
            } else {
                info!("Settings saved to {}", path.display());
            }
        }
        Err(e) => error!("Cannot serialize settings: {e}"),
    }
}

use slint::ComponentHandle;

use crate::{BigFilesSettings, DuplicateSettings, GeneralSettings, MainWindow, SimilarImagesSettings};

pub fn apply_settings_to_gui(win: &MainWindow, s: &CediniaSettings) {
    let items = StringComboBoxItems::new();

    win.global::<GeneralSettings>().set_use_cache(s.use_cache);
    win.global::<GeneralSettings>().set_ignore_hidden(s.ignore_hidden);
    win.global::<GeneralSettings>().set_min_file_size_idx(s.min_file_size_idx);
    win.global::<GeneralSettings>().set_excluded_items(s.excluded_items.clone().into());
    win.global::<GeneralSettings>().set_allowed_extensions(s.allowed_extensions.clone().into());
    win.global::<GeneralSettings>().set_excluded_extensions(s.excluded_extensions.clone().into());

    let cm_idx = StringComboBoxItems::idx_from_config_name(&s.duplicates_check_method, &items.duplicates_check_method);
    win.global::<DuplicateSettings>().set_check_method(cm_idx as i32);
    win.global::<DuplicateSettings>().set_check_method_value(s.duplicates_check_method.clone().into());

    let ht_idx = StringComboBoxItems::idx_from_config_name(&s.duplicates_hash_type, &items.duplicates_hash_type);
    win.global::<DuplicateSettings>().set_hash_type(ht_idx as i32);
    win.global::<DuplicateSettings>().set_hash_type_value(s.duplicates_hash_type.clone().into());

    win.global::<DuplicateSettings>().set_min_size_kb_idx(s.duplicates_min_size_kb_idx);

    let sp_idx = StringComboBoxItems::idx_from_config_name(&s.similar_images_similarity_preset, &items.similarity_preset);
    win.global::<SimilarImagesSettings>().set_similarity_preset(sp_idx as i32);
    win.global::<SimilarImagesSettings>()
        .set_similarity_preset_value(s.similar_images_similarity_preset.clone().into());

    let hs_idx = StringComboBoxItems::idx_from_config_name(&s.similar_images_hash_size, &items.hash_size);
    win.global::<SimilarImagesSettings>().set_hash_size_idx(hs_idx as i32);
    win.global::<SimilarImagesSettings>().set_hash_size_value(s.similar_images_hash_size.clone().into());

    let sm_idx = StringComboBoxItems::idx_from_config_name(&s.big_files_search_mode, &items.biggest_files_method);
    win.global::<BigFilesSettings>().set_search_mode_idx(sm_idx as i32);
    win.global::<BigFilesSettings>().set_search_mode_value(s.big_files_search_mode.clone().into());

    let cnt_idx = StringComboBoxItems::idx_from_config_name(&s.big_files_count, &items.big_files_count);
    win.global::<BigFilesSettings>().set_count_idx(cnt_idx as i32);
    win.global::<BigFilesSettings>().set_count_value(s.big_files_count.clone().into());
}

pub fn collect_settings_from_gui(win: &MainWindow) -> CediniaSettings {
    let g = win.global::<GeneralSettings>();
    let d = win.global::<DuplicateSettings>();
    let si = win.global::<SimilarImagesSettings>();
    let bf = win.global::<BigFilesSettings>();

    CediniaSettings {
        use_cache: g.get_use_cache(),
        ignore_hidden: g.get_ignore_hidden(),
        min_file_size_idx: g.get_min_file_size_idx(),
        excluded_items: g.get_excluded_items().to_string(),
        allowed_extensions: g.get_allowed_extensions().to_string(),
        excluded_extensions: g.get_excluded_extensions().to_string(),
        duplicates_check_method: d.get_check_method_value().to_string(),
        duplicates_hash_type: d.get_hash_type_value().to_string(),
        duplicates_min_size_kb_idx: d.get_min_size_kb_idx(),
        similar_images_similarity_preset: si.get_similarity_preset_value().to_string(),
        similar_images_hash_size: si.get_hash_size_value().to_string(),
        big_files_search_mode: bf.get_search_mode_value().to_string(),
        big_files_count: bf.get_count_value().to_string(),
    }
}
