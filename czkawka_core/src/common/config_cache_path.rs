use std::fs::{File, OpenOptions};
use std::path::PathBuf;
use std::{env, fs};

use directories_next::ProjectDirs;
use log::{info, warn};
use once_cell::sync::OnceCell;

static CONFIG_CACHE_PATH: OnceCell<Option<ConfigCachePath>> = OnceCell::new();

#[derive(Debug, Clone)]
pub struct ConfigCachePath {
    pub config_folder: PathBuf,
    pub cache_folder: PathBuf,
}

pub fn get_config_cache_path() -> Option<ConfigCachePath> {
    CONFIG_CACHE_PATH.get().expect("Cannot fail if set_config_cache_path was called before").clone()
}

fn resolve_folder(env_var: &str, default_folder: Option<PathBuf>, name: &'static str, warnings: &mut Vec<String>) -> Option<PathBuf> {
    let default_folder_str = default_folder.as_ref().map_or("<not available>".to_string(), |t| t.to_string_lossy().to_string());

    if env_var.is_empty() {
        default_folder
    } else {
        let folder_path = PathBuf::from(env_var);
        let _ = fs::create_dir_all(&folder_path);
        if !folder_path.exists() {
            warnings.push(format!(
                "{name} folder \"{}\" does not exist, using default folder \"{}\"",
                folder_path.to_string_lossy(),
                default_folder_str
            ));
            return default_folder;
        }
        if !folder_path.is_dir() {
            warnings.push(format!(
                "{name} folder \"{}\" is not a directory, using default folder \"{}\"",
                folder_path.to_string_lossy(),
                default_folder_str
            ));
            return default_folder;
        }

        match dunce::canonicalize(folder_path) {
            Ok(t) => Some(t),
            Err(_e) => {
                warnings.push(format!(
                    "Cannot canonicalize {} folder \"{}\", using default folder \"{}\"",
                    name.to_ascii_lowercase(),
                    env_var,
                    default_folder_str
                ));
                default_folder
            }
        }
    }
}
#[cfg(test)]
pub fn set_config_cache_path_test(cache_path: PathBuf, config_path: PathBuf) {
    CONFIG_CACHE_PATH
        .set(Some(ConfigCachePath {
            cache_folder: cache_path,
            config_folder: config_path,
        }))
        .expect("Cannot set config cache path");
}

pub struct ConfigCachePathSetResult {
    pub infos: Vec<String>,
    pub warnings: Vec<String>,
    pub config_env_set: bool,
    pub cache_env_set: bool,
    pub default_cache_path_exists: bool,
    pub default_config_path_exists: bool,
}

// This function must be executed, to not crash, when gathering config/cache path
pub fn set_config_cache_path(cache_name: &'static str, config_name: &'static str) -> ConfigCachePathSetResult {
    // By default, such folders are used:
    // Lin: /home/username/.config/czkawka
    // LinFlatpak: /home/username/.var/app/com.github.qarmin.czkawka/config/czkawka
    // Win: C:\Users\Username\AppData\Roaming\Qarmin\Czkawka\config
    // Mac: /Users/Username/Library/Application Support/pl.Qarmin.Czkawka

    let mut infos = Vec::new();
    let mut warnings = Vec::new();

    let config_folder_env = env::var("CZKAWKA_CONFIG_PATH").unwrap_or_default().trim().to_string();
    let cache_folder_env = env::var("CZKAWKA_CACHE_PATH").unwrap_or_default().trim().to_string();

    let default_cache_folder = ProjectDirs::from("pl", "Qarmin", cache_name).map(|proj_dirs| proj_dirs.cache_dir().to_path_buf());
    let default_config_folder = ProjectDirs::from("pl", "Qarmin", config_name).map(|proj_dirs| proj_dirs.config_dir().to_path_buf());

    let default_config_path_exists = default_config_folder.as_ref().is_some_and(|t| t.exists());
    let default_cache_path_exists = default_cache_folder.as_ref().is_some_and(|t| t.exists());

    let config_folder = resolve_folder(&config_folder_env, default_config_folder, "Config", &mut warnings);
    let cache_folder = resolve_folder(&cache_folder_env, default_cache_folder, "Cache", &mut warnings);

    let config_cache_path = if let (Some(config_folder), Some(cache_folder)) = (config_folder, cache_folder) {
        infos.push(format!(
            "Config folder set to \"{}\" and cache folder set to \"{}\"",
            config_folder.to_string_lossy(),
            cache_folder.to_string_lossy()
        ));
        if !config_folder.exists()
            && let Err(e) = fs::create_dir_all(&config_folder)
        {
            warnings.push(format!("Cannot create config folder \"{}\", reason {e}", config_folder.to_string_lossy()));
        }
        if !cache_folder.exists()
            && let Err(e) = fs::create_dir_all(&cache_folder)
        {
            warnings.push(format!("Cannot create cache folder \"{}\", reason {e}", cache_folder.to_string_lossy()));
        }
        Some(ConfigCachePath { config_folder, cache_folder })
    } else {
        warnings.push("Cannot set config/cache path - config and cache will not be used.".to_string());
        None
    };

    CONFIG_CACHE_PATH.set(config_cache_path).expect("Cannot set config/cache path twice");

    ConfigCachePathSetResult {
        infos,
        warnings,
        config_env_set: !config_folder_env.is_empty(),
        cache_env_set: !cache_folder_env.is_empty(),
        default_cache_path_exists,
        default_config_path_exists,
    }
}

pub(crate) fn open_cache_folder(
    cache_file_name: &str,
    save_to_cache: bool,
    use_json: bool,
    warnings: &mut Vec<String>,
) -> Option<((Option<File>, PathBuf), (Option<File>, PathBuf))> {
    let cache_dir = get_config_cache_path()?.cache_folder;
    let cache_file = cache_dir.join(cache_file_name);
    let cache_file_json = cache_dir.join(cache_file_name.replace(".bin", ".json"));

    let mut file_handler_default = None;
    let mut file_handler_json = None;

    if save_to_cache {
        file_handler_default = Some(match OpenOptions::new().truncate(true).write(true).create(true).open(&cache_file) {
            Ok(t) => t,
            Err(e) => {
                warnings.push(format!("Cannot create or open cache file \"{}\", reason {e}", cache_file.to_string_lossy()));
                return None;
            }
        });
        if use_json {
            file_handler_json = Some(match OpenOptions::new().truncate(true).write(true).create(true).open(&cache_file_json) {
                Ok(t) => t,
                Err(e) => {
                    warnings.push(format!("Cannot create or open cache file \"{}\", reason {e}", cache_file_json.to_string_lossy()));
                    return None;
                }
            });
        }
    } else if let Ok(t) = OpenOptions::new().read(true).open(&cache_file) {
        file_handler_default = Some(t);
    } else if use_json {
        file_handler_json = Some(OpenOptions::new().read(true).open(&cache_file_json).ok()?);
    } else {
        return None;
    }
    Some(((file_handler_default, cache_file), (file_handler_json, cache_file_json)))
}

// When initializing logger or settings config/cache folders, logger is not yet initialized,
// so we need to delay them until logger is initialized
pub fn print_infos_and_warnings(infos: Vec<String>, warnings: Vec<String>) {
    for info in infos {
        info!("{info}");
    }
    for warning in warnings {
        warn!("{warning}");
    }
}
