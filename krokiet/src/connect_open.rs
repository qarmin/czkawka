use directories_next::ProjectDirs;
use log::error;
use slint::ComponentHandle;

use crate::{Callabler, MainWindow};

pub fn connect_open_items(app: &MainWindow) {
    app.global::<Callabler>().on_item_opened(move |path| {
        match open::that(&*path) {
            Ok(()) => {}
            Err(e) => {
                eprintln!("Failed to open file: {e}");
            }
        };
        // TODO - this should be added to line edit
    });

    app.global::<Callabler>().on_open_config_folder(move || {
        let Some(dirs) = ProjectDirs::from("pl", "Qarmin", "Krokiet") else {
            error!("Failed to open config folder");
            return;
        };
        let config_folder = dirs.config_dir();
        if let Err(e) = open::that(config_folder) {
            error!("Failed to open config folder {:?}: {e}", config_folder);
        }
    });

    // Cache uses Czkawka name to easily change between apps
    app.global::<Callabler>().on_open_cache_folder(move || {
        let Some(dirs) = ProjectDirs::from("pl", "Qarmin", "Czkawka") else {
            error!("Failed to open cache folder");
            return;
        };
        let cache_folder = dirs.cache_dir();
        if let Err(e) = open::that(cache_folder) {
            error!("Failed to open cache folder {:?}: {e}", cache_folder);
        }
    });

    app.global::<Callabler>().on_open_link(move |link| {
        match open::that(link.as_str()) {
            Ok(()) => {}
            Err(e) => {
                eprintln!("Failed to open link: {e}");
            }
        };
    });
}
