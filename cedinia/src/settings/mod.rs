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
    "10000".to_string()
}
fn default_min_file_size() -> String {
    "none".to_string()
}
fn default_max_file_size() -> String {
    "unlimited".to_string()
}
fn default_language() -> String {
    "auto".to_string()
}
fn default_hash_alg() -> String {
    "mean".to_string()
}
fn default_image_filter() -> String {
    "triangle".to_string()
}
fn default_same_music_check_method() -> String {
    "tags".to_string()
}
fn default_similar_videos_audio_preset() -> String {
    "clip_in_longer".to_string()
}
fn default_excluded_items() -> String {
    #[cfg(not(target_os = "android"))]
    {
        "*/.*".to_string()
    }
    #[cfg(target_os = "android")]
    {
        String::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CediniaSettings {
    #[serde(default = "ttrue")]
    pub use_cache: bool,
    #[serde(default = "ttrue")]
    pub ignore_hidden: bool,
    #[serde(default)]
    pub show_notification: bool,
    #[serde(default = "ttrue")]
    pub notify_only_background: bool,
    #[serde(default = "default_min_file_size")]
    pub min_file_size: String,
    #[serde(default = "default_max_file_size")]
    pub max_file_size: String,
    #[serde(default = "default_language")]
    pub language: String,
    #[serde(default = "default_excluded_items")]
    pub excluded_items: String,
    #[serde(default)]
    pub allowed_extensions: String,
    #[serde(default)]
    pub excluded_extensions: String,
    #[serde(default = "ttrue")]
    pub use_dark_theme: bool,

    #[serde(default = "default_check_method")]
    pub duplicates_check_method: String,
    #[serde(default = "default_hash_type")]
    pub duplicates_hash_type: String,

    #[serde(default = "default_similarity_preset")]
    pub similar_images_similarity_preset: String,
    #[serde(default = "default_hash_size")]
    pub similar_images_hash_size: String,
    #[serde(default = "default_hash_alg")]
    pub similar_images_hash_alg: String,
    #[serde(default = "default_image_filter")]
    pub similar_images_image_filter: String,
    #[serde(default)]
    pub similar_images_ignore_same_size: bool,
    #[serde(default)]
    pub similar_images_ignore_same_resolution: bool,
    #[serde(default)]
    pub gallery_image_fit_cover: bool,

    #[serde(default = "default_search_mode")]
    pub big_files_search_mode: String,
    #[serde(default = "default_big_files_count")]
    pub big_files_count: String,

    #[serde(default = "ttrue")]
    pub same_music_title: bool,
    #[serde(default = "ttrue")]
    pub same_music_artist: bool,
    #[serde(default)]
    pub same_music_year: bool,
    #[serde(default)]
    pub same_music_length: bool,
    #[serde(default)]
    pub same_music_genre: bool,
    #[serde(default)]
    pub same_music_bitrate: bool,
    #[serde(default)]
    pub same_music_approximate: bool,
    #[serde(default = "default_same_music_check_method")]
    pub same_music_check_method: String,

    #[serde(default = "ttrue")]
    pub broken_files_audio: bool,
    #[serde(default = "ttrue")]
    pub broken_files_pdf: bool,
    #[serde(default = "ttrue")]
    pub broken_files_archive: bool,
    #[serde(default = "ttrue")]
    pub broken_files_image: bool,
    #[serde(default = "ttrue")]
    pub broken_files_font: bool,
    #[serde(default = "ttrue")]
    pub broken_files_markup: bool,

    #[serde(default = "default_similar_videos_audio_preset")]
    pub similar_videos_audio_preset: String,

    #[serde(default = "ttrue")]
    pub bad_names_uppercase_extension: bool,
    #[serde(default = "ttrue")]
    pub bad_names_emoji_used: bool,
    #[serde(default = "ttrue")]
    pub bad_names_space_at_start_or_end: bool,
    #[serde(default = "ttrue")]
    pub bad_names_non_ascii_graphical: bool,
    #[serde(default = "ttrue")]
    pub bad_names_remove_duplicated_non_alpha: bool,
}

impl Default for CediniaSettings {
    fn default() -> Self {
        serde_json::from_str("{}").expect("Cannot fail creating {} from string")
    }
}

fn get_dirs_file() -> Option<PathBuf> {
    let config_folder = get_config_cache_path()?.config_folder;
    Some(config_folder.join("cedinia_dirs.json"))
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct DirConfig {
    included: Vec<String>,
    excluded: Vec<String>,
    #[serde(default)]
    referenced: Vec<String>,
}

pub fn save_dirs(included: &[PathBuf], excluded: &[PathBuf], referenced: &[PathBuf]) {
    let Some(path) = get_dirs_file() else {
        error!("Cannot determine dirs config path - dirs not saved");
        return;
    };
    if let Some(parent) = path.parent()
        && let Err(e) = std::fs::create_dir_all(parent)
    {
        error!("Cannot create config dir {}: {e}", parent.display());
        return;
    }
    let config = DirConfig {
        included: included.iter().map(|p| p.to_string_lossy().to_string()).collect(),
        excluded: excluded.iter().map(|p| p.to_string_lossy().to_string()).collect(),
        referenced: referenced.iter().map(|p| p.to_string_lossy().to_string()).collect(),
    };
    match serde_json::to_string_pretty(&config) {
        Ok(json) => {
            if let Err(e) = std::fs::write(&path, json) {
                error!("Cannot write dirs to {}: {e}", path.display());
            } else {
                info!("Dirs saved to {}", path.display());
            }
        }
        Err(e) => error!("Cannot serialize dirs: {e}"),
    }
}

pub fn load_dirs() -> (Vec<PathBuf>, Vec<PathBuf>, Vec<PathBuf>) {
    let Some(path) = get_dirs_file() else {
        return (Vec::new(), Vec::new(), Vec::new());
    };
    if !path.is_file() {
        return (Vec::new(), Vec::new(), Vec::new());
    }
    match std::fs::read_to_string(&path) {
        Ok(json) => match serde_json::from_str::<DirConfig>(&json) {
            Ok(c) => {
                let inc = c.included.iter().map(PathBuf::from).collect();
                let exc = c.excluded.iter().map(PathBuf::from).collect();
                let refr = c.referenced.iter().map(PathBuf::from).collect();
                (inc, exc, refr)
            }
            Err(e) => {
                error!("Cannot parse dirs config: {e}");
                (Vec::new(), Vec::new(), Vec::new())
            }
        },
        Err(e) => {
            error!("Cannot read dirs config {}: {e}", path.display());
            (Vec::new(), Vec::new(), Vec::new())
        }
    }
}

fn get_config_file() -> Option<PathBuf> {
    let config_folder = get_config_cache_path()?.config_folder;
    Some(config_folder.join("cedinia_settings.json"))
}

pub fn load_settings() -> CediniaSettings {
    let Some(path) = get_config_file() else {
        info!("Cannot determine config path - using defaults");
        return CediniaSettings::default();
    };

    if !path.is_file() {
        info!("Settings file does not exist yet - using defaults");
        return CediniaSettings::default();
    }

    match std::fs::read_to_string(&path) {
        Ok(json) => match serde_json::from_str::<CediniaSettings>(&json) {
            Ok(s) => {
                info!("Settings loaded from {}", path.display());
                s
            }
            Err(e) => {
                error!("Cannot parse settings from {}: {e} - using defaults", path.display());
                CediniaSettings::default()
            }
        },
        Err(e) => {
            error!("Cannot read settings file {}: {e} - using defaults", path.display());
            CediniaSettings::default()
        }
    }
}

pub fn save_settings(settings: &CediniaSettings) {
    let Some(path) = get_config_file() else {
        error!("Cannot determine config path - settings not saved");
        return;
    };

    if let Some(parent) = path.parent()
        && let Err(e) = std::fs::create_dir_all(parent)
    {
        error!("Cannot create config dir {}: {e}", parent.display());
        return;
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

use crate::{
    BadNamesSettings, BigFilesSettings, BrokenFilesSettings, DuplicateSettings, GeneralSettings, MainWindow, SameMusicSettings, SimilarImagesSettings, SimilarVideosSettings,
};

pub fn apply_settings_to_gui(win: &MainWindow, s: &CediniaSettings) {
    let items = StringComboBoxItems::new();

    win.global::<GeneralSettings>().set_use_cache(s.use_cache);
    win.global::<GeneralSettings>().set_ignore_hidden(s.ignore_hidden);
    win.global::<GeneralSettings>().set_show_notification(s.show_notification);
    win.global::<GeneralSettings>().set_notify_only_background(s.notify_only_background);
    let min_idx = StringComboBoxItems::idx_from_config_name(&s.min_file_size, &items.min_file_size);
    win.global::<GeneralSettings>().set_min_file_size_idx(min_idx as i32);
    let max_idx = StringComboBoxItems::idx_from_config_name(&s.max_file_size, &items.max_file_size);
    win.global::<GeneralSettings>().set_max_file_size_idx(max_idx as i32);
    let lang_idx = crate::localizer_cedinia::LANGUAGE_LIST
        .iter()
        .position(|&(code, _)| code == s.language.as_str())
        .unwrap_or_else(|| crate::localizer_cedinia::detect_os_language_idx() as usize) as i32;
    win.global::<GeneralSettings>().set_language_idx(lang_idx);
    win.global::<GeneralSettings>().set_excluded_items(s.excluded_items.clone().into());
    win.global::<GeneralSettings>().set_allowed_extensions(s.allowed_extensions.clone().into());
    win.global::<GeneralSettings>().set_excluded_extensions(s.excluded_extensions.clone().into());
    win.global::<GeneralSettings>().set_use_dark_theme(s.use_dark_theme);

    let cm_idx = StringComboBoxItems::idx_from_config_name(&s.duplicates_check_method, &items.duplicates_check_method);
    win.global::<DuplicateSettings>().set_check_method(cm_idx as i32);
    win.global::<DuplicateSettings>().set_check_method_value(s.duplicates_check_method.clone().into());

    let ht_idx = StringComboBoxItems::idx_from_config_name(&s.duplicates_hash_type, &items.duplicates_hash_type);
    win.global::<DuplicateSettings>().set_hash_type(ht_idx as i32);
    win.global::<DuplicateSettings>().set_hash_type_value(s.duplicates_hash_type.clone().into());

    let sp_idx = StringComboBoxItems::idx_from_config_name(&s.similar_images_similarity_preset, &items.similarity_preset);
    win.global::<SimilarImagesSettings>().set_similarity_preset(sp_idx as i32);
    win.global::<SimilarImagesSettings>()
        .set_similarity_preset_value(s.similar_images_similarity_preset.clone().into());

    let hs_idx = StringComboBoxItems::idx_from_config_name(&s.similar_images_hash_size, &items.hash_size);
    win.global::<SimilarImagesSettings>().set_hash_size_idx(hs_idx as i32);
    win.global::<SimilarImagesSettings>().set_hash_size_value(s.similar_images_hash_size.clone().into());

    let ha_idx = StringComboBoxItems::idx_from_config_name(&s.similar_images_hash_alg, &items.hash_alg);
    win.global::<SimilarImagesSettings>().set_hash_alg_idx(ha_idx as i32);
    win.global::<SimilarImagesSettings>().set_hash_alg_value(s.similar_images_hash_alg.clone().into());

    let if_idx = StringComboBoxItems::idx_from_config_name(&s.similar_images_image_filter, &items.image_filter);
    win.global::<SimilarImagesSettings>().set_image_filter_idx(if_idx as i32);
    win.global::<SimilarImagesSettings>().set_image_filter_value(s.similar_images_image_filter.clone().into());

    win.global::<SimilarImagesSettings>().set_ignore_same_size(s.similar_images_ignore_same_size);
    win.global::<SimilarImagesSettings>().set_ignore_same_resolution(s.similar_images_ignore_same_resolution);
    win.global::<SimilarImagesSettings>().set_gallery_image_fit_cover(s.gallery_image_fit_cover);

    let sm_idx = StringComboBoxItems::idx_from_config_name(&s.big_files_search_mode, &items.biggest_files_method);
    win.global::<BigFilesSettings>().set_search_mode_idx(sm_idx as i32);
    win.global::<BigFilesSettings>().set_search_mode_value(s.big_files_search_mode.clone().into());

    let cnt_idx = StringComboBoxItems::idx_from_config_name(&s.big_files_count, &items.big_files_count);
    win.global::<BigFilesSettings>().set_count_idx(cnt_idx as i32);
    win.global::<BigFilesSettings>().set_count_value(s.big_files_count.clone().into());

    let sm = win.global::<SameMusicSettings>();
    sm.set_title(s.same_music_title);
    sm.set_artist(s.same_music_artist);
    sm.set_year(s.same_music_year);
    sm.set_length(s.same_music_length);
    sm.set_genre(s.same_music_genre);
    sm.set_bitrate(s.same_music_bitrate);
    sm.set_approximate(s.same_music_approximate);
    let smc_idx = StringComboBoxItems::idx_from_config_name(&s.same_music_check_method, &items.same_music_check_method);
    sm.set_check_method_idx(smc_idx as i32);
    sm.set_check_method_value(s.same_music_check_method.clone().into());

    let bf = win.global::<BrokenFilesSettings>();
    bf.set_check_audio(s.broken_files_audio);
    bf.set_check_pdf(s.broken_files_pdf);
    bf.set_check_archive(s.broken_files_archive);
    bf.set_check_image(s.broken_files_image);
    bf.set_check_font(s.broken_files_font);
    bf.set_check_markup(s.broken_files_markup);

    let bn = win.global::<BadNamesSettings>();
    bn.set_uppercase_extension(s.bad_names_uppercase_extension);
    bn.set_emoji_used(s.bad_names_emoji_used);
    bn.set_space_at_start_or_end(s.bad_names_space_at_start_or_end);
    bn.set_non_ascii_graphical(s.bad_names_non_ascii_graphical);
    bn.set_remove_duplicated_non_alpha(s.bad_names_remove_duplicated_non_alpha);

    let sv_idx = StringComboBoxItems::idx_from_config_name(&s.similar_videos_audio_preset, &items.similar_videos_audio_preset);
    win.global::<SimilarVideosSettings>().set_audio_preset_idx(sv_idx as i32);
    win.global::<SimilarVideosSettings>().set_audio_preset_value(s.similar_videos_audio_preset.clone().into());
}

pub fn collect_settings_from_gui(win: &MainWindow) -> CediniaSettings {
    let items = StringComboBoxItems::new();
    let g = win.global::<GeneralSettings>();
    let d = win.global::<DuplicateSettings>();
    let si = win.global::<SimilarImagesSettings>();
    let bfiles = win.global::<BigFilesSettings>();
    let sm = win.global::<SameMusicSettings>();
    let bf = win.global::<BrokenFilesSettings>();
    let bn = win.global::<BadNamesSettings>();
    let sv = win.global::<SimilarVideosSettings>();

    CediniaSettings {
        use_cache: g.get_use_cache(),
        ignore_hidden: g.get_ignore_hidden(),
        show_notification: g.get_show_notification(),
        notify_only_background: g.get_notify_only_background(),
        min_file_size: StringComboBoxItems::config_name_from_idx(&items.min_file_size, g.get_min_file_size_idx(), "none"),
        max_file_size: StringComboBoxItems::config_name_from_idx(&items.max_file_size, g.get_max_file_size_idx(), "unlimited"),
        language: crate::localizer_cedinia::LANGUAGE_LIST
            .get(g.get_language_idx() as usize)
            .map_or_else(|| "en".to_string(), |&(code, _)| code.to_string()),
        excluded_items: g.get_excluded_items().to_string(),
        allowed_extensions: g.get_allowed_extensions().to_string(),
        excluded_extensions: g.get_excluded_extensions().to_string(),
        use_dark_theme: g.get_use_dark_theme(),
        duplicates_check_method: items
            .duplicates_check_method
            .get(d.get_check_method() as usize)
            .map_or_else(|| panic!("Invalid check_method idx {} in GUI", d.get_check_method()), |e| e.config_name.clone()),
        duplicates_hash_type: items
            .duplicates_hash_type
            .get(d.get_hash_type() as usize)
            .map_or_else(|| panic!("Invalid hash_type idx {} in GUI", d.get_hash_type()), |e| e.config_name.clone()),
        similar_images_similarity_preset: items
            .similarity_preset
            .get(si.get_similarity_preset() as usize)
            .map_or_else(|| panic!("Invalid similarity_preset idx {} in GUI", si.get_similarity_preset()), |e| e.config_name.clone()),
        similar_images_hash_size: items
            .hash_size
            .get(si.get_hash_size_idx() as usize)
            .map_or_else(|| panic!("Invalid hash_size_idx {} in GUI", si.get_hash_size_idx()), |e| e.config_name.clone()),
        similar_images_hash_alg: items
            .hash_alg
            .get(si.get_hash_alg_idx() as usize)
            .map_or_else(|| panic!("Invalid hash_alg_idx {} in GUI", si.get_hash_alg_idx()), |e| e.config_name.clone()),
        similar_images_image_filter: items
            .image_filter
            .get(si.get_image_filter_idx() as usize)
            .map_or_else(|| panic!("Invalid image_filter_idx {} in GUI", si.get_image_filter_idx()), |e| e.config_name.clone()),
        similar_images_ignore_same_size: si.get_ignore_same_size(),
        similar_images_ignore_same_resolution: si.get_ignore_same_resolution(),
        gallery_image_fit_cover: si.get_gallery_image_fit_cover(),
        big_files_search_mode: items
            .biggest_files_method
            .get(bfiles.get_search_mode_idx() as usize)
            .map_or_else(|| panic!("Invalid search_mode_idx {} in GUI", bfiles.get_search_mode_idx()), |e| e.config_name.clone()),
        big_files_count: items
            .big_files_count
            .get(bfiles.get_count_idx() as usize)
            .map_or_else(|| panic!("Invalid count_idx {} in GUI", bfiles.get_count_idx()), |e| e.config_name.clone()),
        same_music_title: sm.get_title(),
        same_music_artist: sm.get_artist(),
        same_music_year: sm.get_year(),
        same_music_length: sm.get_length(),
        same_music_genre: sm.get_genre(),
        same_music_bitrate: sm.get_bitrate(),
        same_music_approximate: sm.get_approximate(),
        same_music_check_method: items.same_music_check_method.get(sm.get_check_method_idx() as usize).map_or_else(
            || panic!("Invalid same_music_check_method_idx {} in GUI", sm.get_check_method_idx()),
            |e| e.config_name.clone(),
        ),
        broken_files_audio: bf.get_check_audio(),
        broken_files_pdf: bf.get_check_pdf(),
        broken_files_archive: bf.get_check_archive(),
        broken_files_image: bf.get_check_image(),
        broken_files_font: bf.get_check_font(),
        broken_files_markup: bf.get_check_markup(),
        bad_names_uppercase_extension: bn.get_uppercase_extension(),
        bad_names_emoji_used: bn.get_emoji_used(),
        bad_names_space_at_start_or_end: bn.get_space_at_start_or_end(),
        bad_names_non_ascii_graphical: bn.get_non_ascii_graphical(),
        bad_names_remove_duplicated_non_alpha: bn.get_remove_duplicated_non_alpha(),
        similar_videos_audio_preset: StringComboBoxItems::config_name_from_idx(&items.similar_videos_audio_preset, sv.get_audio_preset_idx(), "clip_in_longer"),
    }
}
