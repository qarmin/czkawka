use crate::MainWindow;
use std::cmp::{max, min};
use std::env;
use std::path::{Path, PathBuf};

use crate::common::create_string_standard_list_view_from_pathbuf;
use crate::{GuiState, Settings};
use czkawka_core::common_items::{DEFAULT_EXCLUDED_DIRECTORIES, DEFAULT_EXCLUDED_ITEMS};
use directories_next::ProjectDirs;
use home::home_dir;
use log::{error, info};
use serde::{Deserialize, Serialize};
use slint::{ComponentHandle, Model};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingsCustom {
    #[serde(default = "default_included_directories")]
    pub included_directories: Vec<PathBuf>,
    #[serde(default = "default_excluded_directories")]
    pub excluded_directories: Vec<PathBuf>,
    #[serde(default = "default_excluded_items")]
    pub excluded_items: String,
    #[serde(default)]
    pub allowed_extensions: String,
}

impl Default for SettingsCustom {
    fn default() -> Self {
        serde_json::from_str("{}").unwrap()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasicSettings {
    #[serde(default = "default_language")]
    pub language: String,
    #[serde(default)]
    pub default_preset: i32,
    #[serde(default = "default_preset_names")]
    pub preset_names: Vec<String>,
}

impl Default for BasicSettings {
    fn default() -> Self {
        serde_json::from_str("{}").unwrap()
    }
}

pub fn create_default_settings_files() {
    let base_config_file = get_base_config_file();
    if let Some(base_config_file) = base_config_file {
        if !base_config_file.is_file() {
            let _ = save_data_to_file(Some(base_config_file), &BasicSettings::default());
        }
    }
    for i in 0..10 {
        let config_file = get_config_file(i);
        if let Some(config_file) = config_file {
            if !config_file.is_file() {
                let _ = save_data_to_file(Some(config_file), &SettingsCustom::default());
            }
        }
    }
}

pub fn reset_settings(app: &MainWindow) {
    set_settings_to_gui(app, &SettingsCustom::default());
}

pub fn load_settings_from_file(app: &MainWindow) {
    let result_base_settings = load_data_from_file::<BasicSettings>(get_base_config_file());
    let base_settings;
    if let Ok(base_settings_temp) = result_base_settings {
        base_settings = base_settings_temp;
    } else {
        info!("Cannot load base settings, using default instead");
        base_settings = BasicSettings::default();
    }

    let results_custom_settings = load_data_from_file::<SettingsCustom>(get_config_file(base_settings.default_preset));
    let custom_settings;
    if let Ok(custom_settings_temp) = results_custom_settings {
        custom_settings = custom_settings_temp;
    } else {
        info!("Cannot load custom settings, using default instead");
        custom_settings = SettingsCustom::default();
    }

    set_settings_to_gui(app, &custom_settings);
    set_base_settings_to_gui(app, &base_settings);
}

pub fn save_settings_to_file(app: &MainWindow) {
    let current_item = app.global::<Settings>().get_settings_preset_idx();
    let result = save_data_to_file(get_config_file(current_item), &collect_settings(app));

    if let Err(e) = result {
        error!("{e}");
    }
}

pub fn load_data_from_file<T>(config_data: Option<PathBuf>) -> Result<T, String>
where
    for<'de> T: Deserialize<'de>,
{
    let Some(config_data) = config_data else {
        return Err("Cannot get config file".into());
    };
    if !config_data.is_file() {
        return Err("Config file doesn't exists".into());
    }

    match std::fs::read_to_string(config_data) {
        Ok(serialized) => match serde_json::from_str(&serialized) {
            Ok(custom_settings) => Ok(custom_settings),
            Err(e) => {
                return Err(format!("Cannot deserialize settings: {e}"));
            }
        },
        Err(e) => {
            return Err(format!("Cannot read config file: {e}"));
        }
    }
}

pub fn save_data_to_file<T>(config_file: Option<PathBuf>, serializable_data: &T) -> Result<(), String>
where
    T: Serialize,
{
    let Some(config_file) = config_file else {
        return Err("Cannot get config file".into());
    };
    // Create dirs if not exists
    if let Some(parent) = config_file.parent() {
        if let Err(e) = std::fs::create_dir_all(parent) {
            return Err(format!("Cannot create config folder: {e}"));
        }
    }

    match serde_json::to_string_pretty(&serializable_data) {
        Ok(serialized) => {
            if let Err(e) = std::fs::write(config_file, serialized) {
                return Err(format!("Cannot save config file: {e}"));
            }
        }
        Err(e) => {
            return Err(format!("Cannot serialize settings: {e}"));
        }
    }
    Ok(())
}

pub fn get_base_config_file() -> Option<PathBuf> {
    let Some(configs) = ProjectDirs::from("pl", "Qarmin", "Krokiet") else {
        return None;
    };
    let config_folder = configs.config_dir();
    let base_config_file = config_folder.join("config_general.json");
    Some(base_config_file)
}
pub fn get_config_file(number: i32) -> Option<PathBuf> {
    let number = max(min(number, 9), 0);

    let Some(configs) = ProjectDirs::from("pl", "Qarmin", "Krokiet") else {
        return None;
    };
    let config_folder = configs.config_dir();
    let config_file = config_folder.join(format!("config_preset_{number}.json"));
    Some(config_file)
}

pub fn set_base_settings_to_gui(app: &MainWindow, basic_settings: &BasicSettings) {
    let settings = app.global::<Settings>();
    // settings.set_language(basic_settings.language.clone());
    settings.set_settings_preset_idx(basic_settings.default_preset);
}
pub fn set_settings_to_gui(app: &MainWindow, custom_settings: &SettingsCustom) {
    let settings = app.global::<Settings>();

    // Included directories
    let included_directories = create_string_standard_list_view_from_pathbuf(&custom_settings.included_directories);
    settings.set_included_directories(included_directories);

    // Excluded directories
    let excluded_directories = create_string_standard_list_view_from_pathbuf(&custom_settings.excluded_directories);
    settings.set_excluded_directories(excluded_directories);

    settings.set_excluded_items(custom_settings.excluded_items.clone().into());
    settings.set_allowed_extensions(custom_settings.allowed_extensions.clone().into());

    // Clear text
    app.global::<GuiState>().set_info_text("".into());
}

pub fn collect_settings(app: &MainWindow) -> SettingsCustom {
    let settings = app.global::<Settings>();

    let included_directories = settings.get_included_directories();
    let included_directories = included_directories.iter().map(|x| PathBuf::from(x.text.as_str())).collect::<Vec<_>>();

    let excluded_directories = settings.get_excluded_directories();
    let excluded_directories = excluded_directories.iter().map(|x| PathBuf::from(x.text.as_str())).collect::<Vec<_>>();

    let excluded_items = settings.get_excluded_items().to_string();
    let allowed_extensions = settings.get_allowed_extensions().to_string();

    SettingsCustom {
        included_directories,
        excluded_directories,
        excluded_items,
        allowed_extensions,
    }
}

pub fn collect_base_settings(app: &MainWindow) -> BasicSettings {
    let settings = app.global::<Settings>();

    let default_preset = settings.get_settings_preset_idx();
    let preset_names = settings.get_settings_presets().iter().map(|x| x.to_string()).collect::<Vec<_>>();

    assert_eq!(preset_names.len(), 10);
    BasicSettings {
        language: "en".to_string(),
        default_preset,
        preset_names,
    }
}

fn default_included_directories() -> Vec<PathBuf> {
    let mut included_directories = vec![];
    if let Ok(current_dir) = env::current_dir() {
        included_directories.push(current_dir.to_string_lossy().to_string());
    } else if let Some(home_dir) = home_dir() {
        included_directories.push(home_dir.to_string_lossy().to_string());
    } else if cfg!(target_family = "unix") {
        included_directories.push("/".to_string());
    } else {
        // This could be set to default
        included_directories.push("C:\\".to_string());
    };
    included_directories.sort();
    included_directories.iter().map(PathBuf::from).collect::<Vec<_>>()
}

fn default_excluded_directories() -> Vec<PathBuf> {
    let mut excluded_directories = DEFAULT_EXCLUDED_DIRECTORIES.iter().map(PathBuf::from).collect::<Vec<_>>();
    excluded_directories.sort();
    excluded_directories
}

fn default_excluded_items() -> String {
    DEFAULT_EXCLUDED_ITEMS.to_string()
}

fn default_language() -> String {
    "en".to_string()
}

fn default_preset_names() -> Vec<String> {
    vec![
        "Preset 0", "Preset 1", "Preset 2", "Preset 3", "Preset 4", "Preset 5", "Preset 6", "Preset 7", "Preset 8", "Preset 9",
    ]
    .iter()
    .map(|x| x.to_string())
    .collect::<Vec<_>>()
}
