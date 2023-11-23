use crate::MainWindow;
use std::env;
use std::path::PathBuf;

use crate::common::create_string_standard_list_view_from_pathbuf;
use crate::{GuiState, Settings};
use directories_next::ProjectDirs;
use home::home_dir;
use log::error;
use serde::{Deserialize, Serialize};
use slint::{ComponentHandle, Model};

#[cfg(target_family = "unix")]
const DEFAULT_EXCLUDED_DIRECTORIES: &[&str] = &["/proc", "/dev", "/sys", "/run", "/snap"];
#[cfg(not(target_family = "unix"))]
const DEFAULT_EXCLUDED_DIRECTORIES: &[&str] = &["C:\\Windows"];

#[cfg(target_family = "unix")]
pub const DEFAULT_EXCLUDED_ITEMS: &str = "*/.git/*,*/node_modules/*,*/lost+found/*,*/Trash/*,*/.Trash-*/*,*/snap/*,/home/*/.cache/*";
#[cfg(not(target_family = "unix"))]
pub const DEFAULT_EXCLUDED_ITEMS: &str = "*\\.git\\*,*\\node_modules\\*,*\\lost+found\\*,*:\\windows\\*,*:\\$RECYCLE.BIN\\*,*:\\$SysReset\\*,*:\\System Volume Information\\*,*:\\OneDriveTemp\\*,*:\\hiberfil.sys,*:\\pagefile.sys,*:\\swapfile.sys";

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
        Self {
            included_directories: default_included_directories(),
            excluded_directories: default_excluded_directories(),
            excluded_items: default_excluded_items(),
            allowed_extensions: String::new(),
        }
    }
}

pub fn reset_settings(app: &MainWindow) {
    set_settings_to_gui(app, &SettingsCustom::default());
}

pub fn load_settings_from_file(app: &MainWindow) {
    let Some(config_file) = get_config_file() else {
        error!("Cannot get config file");
        return;
    };
    if !config_file.is_file() {
        error!("Config file doesn't exists");
        return;
    }

    match std::fs::read_to_string(config_file) {
        Ok(serialized) => match serde_json::from_str(&serialized) {
            Ok(custom_settings) => {
                set_settings_to_gui(app, &custom_settings);
            }
            Err(e) => {
                error!("Cannot deserialize settings: {e}");
            }
        },
        Err(e) => {
            error!("Cannot read config file: {e}");
        }
    }
}

pub fn save_settings_to_file(app: &MainWindow) {
    let Some(config_file) = get_config_file() else {
        error!("Cannot get config file");
        return;
    };
    // Create dirs if not exists
    if let Some(parent) = config_file.parent() {
        if let Err(e) = std::fs::create_dir_all(parent) {
            error!("Cannot create config folder: {e}");
            return;
        }
    }

    let collected_settings = collect_settings(app);
    match serde_json::to_string_pretty(&collected_settings) {
        Ok(serialized) => {
            if let Err(e) = std::fs::write(config_file, serialized) {
                error!("Cannot save config file: {e}");
            }
        }
        Err(e) => {
            error!("Cannot serialize settings: {e}");
        }
    }
}

pub fn get_config_file() -> Option<PathBuf> {
    let Some(configs) = ProjectDirs::from("pl", "Qarmin", "Krokiet") else {
        return None;
    };
    let config_folder = configs.config_dir();
    let config_file = config_folder.join("config.json");
    Some(config_file)
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
