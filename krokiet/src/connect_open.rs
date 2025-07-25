use czkawka_core::common::get_config_cache_path;
use log::error;
use slint::ComponentHandle;

use crate::{Callabler, MainWindow};

pub(crate) fn connect_open_items(app: &MainWindow) {
    app.global::<Callabler>().on_open_config_folder(move || {
        let Some(config_cache) = get_config_cache_path() else {
            error!("Failed to open config folder");
            return;
        };
        if let Err(e) = open::that(&config_cache.config_folder) {
            error!("Failed to open config folder \"{}\": {e}", config_cache.config_folder.to_string_lossy());
        }
    });

    app.global::<Callabler>().on_open_cache_folder(move || {
        let Some(config_cache) = get_config_cache_path() else {
            error!("Failed to open cache folder");
            return;
        };
        if let Err(e) = open::that(&config_cache.cache_folder) {
            error!("Failed to open cache folder \"{}\": {e}", config_cache.cache_folder.to_string_lossy());
        }
    });

    app.global::<Callabler>().on_open_link(move |link| {
        match open::that(link.as_str()) {
            Ok(()) => {}
            Err(e) => {
                error!("Failed to open link: {e}");
            }
        };
    });
}
