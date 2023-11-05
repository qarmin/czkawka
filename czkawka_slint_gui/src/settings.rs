use crate::MainWindow;
use std::env;

use crate::common::create_string_standard_list_view;
use crate::Settings;
use home::home_dir;
use slint::{ComponentHandle, SharedString};

#[cfg(target_family = "unix")]
const DEFAULT_EXCLUDED_DIRECTORIES: &[&str] = &["/proc", "/dev", "/sys", "/run", "/snap"];
#[cfg(not(target_family = "unix"))]
const DEFAULT_EXCLUDED_DIRECTORIES: &[&str] = &["C:\\Windows"];

pub fn reset_settings(app: &MainWindow) {
    let settings = app.global::<Settings>();

    // app.width(1000);
    app.invoke_set_console_text(SharedString::from(""));

    // Included directories
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
    let included_items = create_string_standard_list_view(&included_directories);
    settings.set_included_directories(included_items);

    // Excluded directories
    let mut excluded_directories = DEFAULT_EXCLUDED_DIRECTORIES.iter().map(|x| (*x).to_string()).collect::<Vec<_>>();
    excluded_directories.sort();
    let excluded_items = create_string_standard_list_view(&excluded_directories);
    settings.set_excluded_directories(excluded_items);
}
