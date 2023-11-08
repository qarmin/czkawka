use crate::MainWindow;
use std::env;
use std::path::PathBuf;

use crate::common::create_string_standard_list_view_from_pathbuf;
use crate::Settings;
use home::home_dir;
use slint::{ComponentHandle, Model, SharedString};

#[cfg(target_family = "unix")]
const DEFAULT_EXCLUDED_DIRECTORIES: &[&str] = &["/proc", "/dev", "/sys", "/run", "/snap"];
#[cfg(not(target_family = "unix"))]
const DEFAULT_EXCLUDED_DIRECTORIES: &[&str] = &["C:\\Windows"];

pub fn reset_settings(app: &MainWindow) {
    app.invoke_set_console_text(SharedString::from(""));

    set_settings_to_gui(app, &SettingsCustom::default());
}

pub fn set_settings_to_gui(app: &MainWindow, custom_settings: &SettingsCustom) {
    let settings = app.global::<Settings>();

    // Included directories
    let included_items = create_string_standard_list_view_from_pathbuf(&custom_settings.included_directories);
    settings.set_included_directories(included_items);

    // Excluded directories
    let excluded_items = create_string_standard_list_view_from_pathbuf(&custom_settings.excluded_directories);
    settings.set_excluded_directories(excluded_items);
}

pub struct SettingsCustom {
    pub included_directories: Vec<PathBuf>,
    pub excluded_directories: Vec<PathBuf>,
}

impl Default for SettingsCustom {
    fn default() -> Self {
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
        let included_directories = included_directories.iter().map(PathBuf::from).collect::<Vec<_>>();

        let mut excluded_directories = DEFAULT_EXCLUDED_DIRECTORIES.iter().map(PathBuf::from).collect::<Vec<_>>();
        excluded_directories.sort();

        Self {
            included_directories,
            excluded_directories,
        }
    }
}

pub fn collect_settings(app: &MainWindow) -> SettingsCustom {
    let settings = app.global::<Settings>();

    let included_directories = settings.get_included_directories();
    let included_directories = included_directories.iter().map(|x| PathBuf::from(x.text.as_str())).collect::<Vec<_>>();

    let excluded_directories = settings.get_excluded_directories();
    let excluded_directories = excluded_directories.iter().map(|x| PathBuf::from(x.text.as_str())).collect::<Vec<_>>();

    SettingsCustom {
        included_directories,
        excluded_directories,
    }
}
