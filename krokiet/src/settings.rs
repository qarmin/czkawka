use std::cmp::{max, min};
use std::collections::BTreeMap;
use std::env;
use std::fmt::Debug;
use std::path::PathBuf;
use std::sync::{Arc, Mutex, MutexGuard};

use czkawka_core::common::{get_all_available_threads, get_config_cache_path, set_number_of_threads};
use czkawka_core::common_dir_traversal::CheckingMethod;
use czkawka_core::common_items::{DEFAULT_EXCLUDED_DIRECTORIES, DEFAULT_EXCLUDED_ITEMS};
use czkawka_core::tools::big_file::SearchMode;
use czkawka_core::tools::duplicate::HashType;
use home::home_dir;
use image_hasher::{FilterType, HashAlg};
use log::{debug, error, info, warn};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use slint::{ComponentHandle, Model, ModelRc, PhysicalSize, SharedString, VecModel, WindowSize};

use crate::cli::CliResult;
use crate::common::{create_excluded_directories_model_from_pathbuf, create_included_directories_model_from_pathbuf, create_vec_model_from_vec_string};
use crate::connect_translation::{LANGUAGE_LIST, change_language, find_the_closest_language_idx_to_system};
use crate::{Callabler, GuiState, MainWindow, Settings, flk};

pub const DEFAULT_MINIMUM_SIZE_KB: i32 = 16;
pub const DEFAULT_MAXIMUM_SIZE_KB: i32 = i32::MAX / 1024;
pub const DEFAULT_MINIMUM_CACHE_SIZE: i32 = 256;
pub const DEFAULT_MINIMUM_PREHASH_CACHE_SIZE: i32 = 256;
pub const DEFAULT_BIGGEST_FILES: i32 = 50;
pub const DEFAULT_IMAGE_SIMILARITY: i32 = 10;
pub const DEFAULT_VIDEO_SIMILARITY: i32 = 15;
pub const DEFAULT_HASH_SIZE: &str = "16";
pub const DEFAULT_MAXIMUM_DIFFERENCE_VALUE: f32 = 3.0;
pub const DEFAULT_MINIMAL_FRAGMENT_DURATION_VALUE: f32 = 5.0;
pub const MAX_HASH_SIZE: f32 = 40.0;
pub const DEFAULT_WINDOW_WIDTH: u32 = 800;
pub const DEFAULT_WINDOW_HEIGHT: u32 = 600;

pub const PRESET_NUMBER: usize = 11; // 10 normal presets + 1 reserved preset for custom settings
pub const RESERVER_PRESET_IDX: i32 = PRESET_NUMBER as i32 - 1; // 10 normal presets + 1 reserved preset for custom settings
pub const PRESET_NAME_RESERVED: &str = "CLI Folders";

#[derive(Debug, Clone)]
pub struct StringComboBoxItem<T>
where
    T: Clone + Debug,
{
    pub config_name: String,
    pub display_name: String,
    pub value: T,
}

pub struct StringComboBoxItems {
    pub languages: Vec<StringComboBoxItem<String>>,
    pub hash_size: Vec<StringComboBoxItem<u8>>,
    pub resize_algorithm: Vec<StringComboBoxItem<FilterType>>,
    pub image_hash_alg: Vec<StringComboBoxItem<HashAlg>>,
    pub duplicates_hash_type: Vec<StringComboBoxItem<HashType>>,
    pub biggest_files_method: Vec<StringComboBoxItem<SearchMode>>,
    pub audio_check_type: Vec<StringComboBoxItem<CheckingMethod>>,
    pub duplicates_check_method: Vec<StringComboBoxItem<CheckingMethod>>,
}

pub static STRING_COMBO_BOX_ITEMS: Lazy<Arc<Mutex<StringComboBoxItems>>> = Lazy::new(|| {
    let l = StringComboBoxItems::regenerate_items();
    Arc::new(Mutex::new(l))
});

impl StringComboBoxItems {
    pub fn get_item_and_idx_from_config_name<T>(config_name: &str, items: &Vec<StringComboBoxItem<T>>) -> (usize, Vec<SharedString>)
    where
        T: Clone + Debug,
    {
        let position = items.iter().position(|e| e.config_name == config_name).unwrap_or_else(|| {
            warn!("Trying to get non existent item - \"{config_name}\" from {items:?}");
            0
        });
        let display_names = items.iter().map(|e| e.display_name.clone().into()).collect::<Vec<_>>();
        (position, display_names)
    }

    pub fn get_config_name_from_idx<T>(idx: usize, items: &Vec<StringComboBoxItem<T>>) -> String
    where
        T: Clone + Debug,
    {
        if idx < items.len() {
            items[idx].config_name.clone()
        } else {
            warn!("Trying to get non existent item - \"{idx}\" from {items:?}");
            items[0].config_name.clone()
        }
    }

    pub fn get_value_from_config_name<T>(config_name: &str, items: &Vec<StringComboBoxItem<T>>) -> T
    where
        T: Clone + Debug,
    {
        let position = items.iter().position(|e| e.config_name == config_name).unwrap_or_else(|| {
            panic!("Trying to get non existent item - \"{config_name}\" from {items:?}");
        });
        items[position].value.clone()
    }

    // pub fn get_display_names<T>(items: &[StringComboBoxItem<T>]) -> Vec<SharedString>
    // where
    //     T: Clone + Debug,
    // {
    //     items.iter().map(|e| e.display_name.clone().into()).collect()
    // }

    pub(crate) fn regenerate_items() -> Self {
        let languages = LANGUAGE_LIST
            .iter()
            .map(|e| StringComboBoxItem {
                config_name: e.short_name.to_string(),
                display_name: e.long_name.to_string(),
                value: e.short_name.to_string(),
            })
            .collect();

        let hash_size = Self::convert_to_combobox_items(&[("8", "8", 8), ("16", "16", 16), ("32", "32", 32), ("64", "64", 64)]);
        let resize_algorithm = Self::convert_to_combobox_items(&[
            ("lanczos3", "Lanczos3", FilterType::Lanczos3),
            ("gaussian", "Gaussian", FilterType::Gaussian),
            ("catmullrom", "CatmullRom", FilterType::CatmullRom),
            ("triangle", "Triangle", FilterType::Triangle),
            ("nearest", "Nearest", FilterType::Nearest),
        ]);

        let image_hash_alg = Self::convert_to_combobox_items(&[
            ("mean", "Mean", HashAlg::Mean),
            ("gradient", "Gradient", HashAlg::Gradient),
            ("blockhash", "BlockHash", HashAlg::Blockhash),
            ("vertgradient", "VertGradient", HashAlg::VertGradient),
            ("doublegradient", "DoubleGradient", HashAlg::DoubleGradient),
            ("median", "Median", HashAlg::Median),
        ]);

        let duplicates_hash_type = Self::convert_to_combobox_items(&[
            ("blake3", "Blake3", HashType::Blake3),
            ("crc32", "CRC32", HashType::Crc32),
            ("xxh3", "XXH3", HashType::Xxh3),
        ]);

        let biggest_files_method = Self::convert_to_combobox_items(&[
            ("biggest", "The Biggest", SearchMode::BiggestFiles),
            ("smallest", "The Smallest", SearchMode::SmallestFiles),
        ]);

        let audio_check_type = Self::convert_to_combobox_items(&[("tags", "Tags", CheckingMethod::AudioTags), ("fingerprint", "Fingerprint", CheckingMethod::AudioContent)]);

        let duplicates_check_method = Self::convert_to_combobox_items(&[
            ("hash", "Hash", CheckingMethod::Hash),
            ("size", "Size", CheckingMethod::Size),
            ("name", "Name", CheckingMethod::Name),
            ("size_and_name", "Size and Name", CheckingMethod::SizeName),
        ]);

        Self {
            languages,
            hash_size,
            resize_algorithm,
            image_hash_alg,
            duplicates_hash_type,
            biggest_files_method,
            audio_check_type,
            duplicates_check_method,
        }
    }

    fn convert_to_combobox_items<T>(input: &[(&str, &str, T)]) -> Vec<StringComboBoxItem<T>>
    where
        T: Clone + Debug,
    {
        input
            .iter()
            .map(|(config_name, display_name, value)| StringComboBoxItem {
                config_name: config_name.to_string(),
                display_name: display_name.to_string(),
                value: value.clone(),
            })
            .collect()
    }

    pub fn get_items() -> MutexGuard<'static, Self> {
        STRING_COMBO_BOX_ITEMS.lock().expect("Can't lock string combobox items")
    }

    fn regenerate_and_set() {
        *STRING_COMBO_BOX_ITEMS.lock().expect("Can't lock string combobox items") = Self::regenerate_items();
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingsCustom {
    #[serde(default = "default_included_directories")]
    pub included_directories: Vec<PathBuf>,
    #[serde(default)]
    pub included_directories_referenced: Vec<PathBuf>,
    #[serde(default = "default_excluded_directories")]
    pub excluded_directories: Vec<PathBuf>,
    #[serde(default = "default_excluded_items")]
    pub excluded_items: String,
    #[serde(default)]
    pub allowed_extensions: String,
    #[serde(default)]
    pub excluded_extensions: String,
    #[serde(default = "minimum_file_size")]
    pub minimum_file_size: i32,
    #[serde(default = "maximum_file_size")]
    pub maximum_file_size: i32,
    #[serde(default = "ttrue")]
    pub recursive_search: bool,
    #[serde(default = "ttrue")]
    pub use_cache: bool,
    #[serde(default)]
    pub save_also_as_json: bool,
    #[serde(default)]
    pub move_deleted_files_to_trash: bool,
    #[serde(default)]
    pub ignore_other_file_systems: bool,
    #[serde(default)]
    pub thread_number: i32,
    #[serde(default = "ttrue")]
    pub duplicate_image_preview: bool,
    #[serde(default = "ttrue")]
    pub duplicate_hide_hard_links: bool,
    #[serde(default = "ttrue")]
    pub duplicate_use_prehash: bool,
    #[serde(default = "minimal_hash_cache_size")]
    pub duplicate_minimal_hash_cache_size: i32,
    #[serde(default = "minimal_prehash_cache_size")]
    pub duplicate_minimal_prehash_cache_size: i32,
    #[serde(default = "ttrue")]
    pub duplicate_delete_outdated_entries: bool,
    #[serde(default = "ttrue")]
    pub similar_images_hide_hard_links: bool,
    #[serde(default = "ttrue")]
    pub similar_images_show_image_preview: bool,
    #[serde(default = "ttrue")]
    pub similar_images_delete_outdated_entries: bool,
    #[serde(default = "ttrue")]
    pub similar_videos_delete_outdated_entries: bool,
    #[serde(default = "ttrue")]
    pub similar_music_delete_outdated_entries: bool,
    #[serde(default = "default_sub_hash_size")]
    pub similar_images_sub_hash_size: String,
    #[serde(default = "default_hash_type")]
    pub similar_images_sub_hash_alg: String,
    #[serde(default = "default_resize_algorithm")]
    pub similar_images_sub_resize_algorithm: String,
    #[serde(default)]
    pub similar_images_sub_ignore_same_size: bool,
    #[serde(default = "default_image_similarity")]
    pub similar_images_sub_similarity: i32,
    #[serde(default = "default_duplicates_check_method")]
    pub duplicates_sub_check_method: String,
    #[serde(default = "default_duplicates_hash_type")]
    pub duplicates_sub_available_hash_type: String,
    #[serde(default)]
    pub duplicates_sub_name_case_sensitive: bool,
    #[serde(default = "default_biggest_method")]
    pub biggest_files_sub_method: String,
    #[serde(default = "default_biggest_files")]
    pub biggest_files_sub_number_of_files: i32,
    #[serde(default = "ttrue")]
    pub similar_videos_hide_hard_links: bool,
    #[serde(default)]
    pub similar_videos_sub_ignore_same_size: bool,
    #[serde(default = "default_video_similarity")]
    pub similar_videos_sub_similarity: i32,
    #[serde(default = "default_audio_check_type")]
    pub similar_music_sub_audio_check_type: String,
    #[serde(default)]
    pub similar_music_sub_approximate_comparison: bool,
    #[serde(default)]
    pub similar_music_compare_fingerprints_only_with_similar_titles: bool,
    #[serde(default = "ttrue")]
    pub similar_music_sub_title: bool,
    #[serde(default = "ttrue")]
    pub similar_music_sub_artist: bool,
    #[serde(default)]
    pub similar_music_sub_year: bool,
    #[serde(default)]
    pub similar_music_sub_bitrate: bool,
    #[serde(default)]
    pub similar_music_sub_genre: bool,
    #[serde(default)]
    pub similar_music_sub_length: bool,
    #[serde(default = "default_maximum_difference_value")]
    pub similar_music_sub_maximum_difference_value: f32,
    #[serde(default = "default_minimal_fragment_duration_value")]
    pub similar_music_sub_minimal_fragment_duration_value: f32,
    #[serde(default = "ttrue")]
    pub broken_files_sub_audio: bool,
    #[serde(default)]
    pub broken_files_sub_pdf: bool,
    #[serde(default)]
    pub broken_files_sub_archive: bool,
    #[serde(default)]
    pub broken_files_sub_image: bool,
    #[serde(default)]
    pub column_sizes: BTreeMap<String, Vec<f32>>,
}

impl Default for SettingsCustom {
    fn default() -> Self {
        serde_json::from_str("{}").expect("Cannot fail creating {} from string")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasicSettings {
    #[serde(default)]
    pub default_preset: i32,
    #[serde(default = "default_preset_names")]
    pub preset_names: Vec<String>,
    #[serde(default = "default_window_width")]
    pub window_width: u32,
    #[serde(default = "default_window_height")]
    pub window_height: u32,
    #[serde(default = "detect_language")]
    pub language: String,
    #[serde(default = "ttrue")]
    pub dark_theme: bool,
    #[serde(default)]
    pub show_only_icons: bool,
}

impl Default for BasicSettings {
    fn default() -> Self {
        serde_json::from_str("{}").expect("Cannot fail creating {} from string")
    }
}

pub fn connect_changing_settings_preset(app: &MainWindow) {
    let a = app.as_weak();
    app.global::<Callabler>().on_changed_settings_preset(move || {
        let app = a.upgrade().expect("Failed to upgrade app :(");
        let current_item = app.global::<Settings>().get_settings_preset_idx();
        let loaded_data = load_data_from_file::<SettingsCustom>(get_config_file(current_item));
        match loaded_data {
            Ok(loaded_data) => {
                set_settings_to_gui(&app, &loaded_data, None);
                app.set_text_summary_text(flk!("rust_loaded_preset", preset_idx = (current_item + 1)).into());
            }
            Err(e) => {
                set_settings_to_gui(&app, &SettingsCustom::default(), None);
                app.set_text_summary_text(flk!("rust_cannot_load_preset", preset_idx = (current_item + 1), reason = (&e)).into());
                error!("Failed to change preset - {e}, using default instead");
            }
        }
    });
    let a = app.as_weak();
    app.global::<Callabler>().on_save_current_preset(move || {
        let app = a.upgrade().expect("Failed to upgrade app :(");
        let settings = app.global::<Settings>();
        let current_item = settings.get_settings_preset_idx();
        let result = save_data_to_file(get_config_file(current_item), &collect_settings(&app));
        match result {
            Ok(()) => {
                app.set_text_summary_text(flk!("rust_saved_preset", preset_idx = (current_item + 1)).into());
            }
            Err(e) => {
                app.set_text_summary_text(flk!("rust_cannot_save_preset", preset_idx = (current_item + 1), reason = (&e)).into());
                error!("Failed to save preset - {e}");
            }
        }
    });
    let a = app.as_weak();
    app.global::<Callabler>().on_reset_current_preset(move || {
        let app = a.upgrade().expect("Failed to upgrade app :(");
        let settings = app.global::<Settings>();
        let current_item = settings.get_settings_preset_idx();
        set_settings_to_gui(&app, &SettingsCustom::default(), None);
        app.set_text_summary_text(flk!("rust_reset_preset", preset_idx = (current_item + 1)).into());
    });
    let a = app.as_weak();
    app.global::<Callabler>().on_load_current_preset(move || {
        let app = a.upgrade().expect("Failed to upgrade app :(");
        let settings = app.global::<Settings>();
        let current_item = settings.get_settings_preset_idx();
        let loaded_data = load_data_from_file::<SettingsCustom>(get_config_file(current_item));
        match loaded_data {
            Ok(loaded_data) => {
                set_settings_to_gui(&app, &loaded_data, None);
                app.set_text_summary_text(flk!("rust_loaded_preset", preset_idx = (current_item + 1)).into());
            }
            Err(e) => {
                set_settings_to_gui(&app, &SettingsCustom::default(), None);
                let err_message = flk!("rust_cannot_load_preset", preset_idx = (current_item + 1), reason = (&e));
                app.set_text_summary_text(err_message.into());
                error!("Failed to load preset - {e}, using default instead");
            }
        }
    });
}

pub fn create_default_settings_files() {
    let base_config_file = get_base_config_file();
    if let Some(base_config_file) = base_config_file {
        if !base_config_file.is_file() {
            let _ = save_data_to_file(Some(base_config_file), &BasicSettings::default());
        }
    }

    for i in 0..PRESET_NUMBER {
        let config_file = get_config_file(i as i32);
        if let Some(config_file) = config_file {
            if !config_file.is_file() {
                let _ = save_data_to_file(Some(config_file), &SettingsCustom::default());
            }
        }
    }
}

pub fn load_settings_from_file(app: &MainWindow, cli_result: Option<CliResult>) -> i32 {
    StringComboBoxItems::regenerate_and_set();

    let result_base_settings = load_data_from_file::<BasicSettings>(get_base_config_file());

    let mut base_settings;
    if let Ok(base_settings_temp) = result_base_settings {
        base_settings = base_settings_temp;
    } else {
        info!("Cannot load base settings, using default instead");
        base_settings = BasicSettings::default();
    }

    let preset_to_load = if cli_result.is_some() { RESERVER_PRESET_IDX } else { base_settings.default_preset };

    let mut custom_settings = load_data_from_file::<SettingsCustom>(get_config_file(preset_to_load)).unwrap_or_else(|e| {
        error!("Cannot load custom settings for preset {preset_to_load} - {e}, using default instead");
        SettingsCustom::default()
    });

    #[allow(clippy::comparison_chain)]
    if base_settings.preset_names.len() > PRESET_NUMBER {
        base_settings.preset_names.truncate(PRESET_NUMBER);
    } else if base_settings.preset_names.len() < PRESET_NUMBER {
        while base_settings.preset_names.len() < PRESET_NUMBER - 1 {
            base_settings.preset_names.push(format!("Preset {}", base_settings.preset_names.len() + 1));
        }
        base_settings.preset_names.push(PRESET_NAME_RESERVED.to_string());
    }
    base_settings.default_preset = base_settings.default_preset.clamp(0, PRESET_NUMBER as i32 - 2);
    custom_settings.thread_number = max(min(custom_settings.thread_number, get_all_available_threads() as i32), 0);

    // Ended validating
    set_settings_to_gui(app, &custom_settings, cli_result);
    set_base_settings_to_gui(app, &base_settings, preset_to_load);
    set_number_of_threads(custom_settings.thread_number as usize);

    base_settings.default_preset
}

pub fn save_all_settings_to_file(app: &MainWindow, original_preset_idx: i32) {
    save_base_settings_to_file(app, original_preset_idx);
    save_custom_settings_to_file(app);
}

pub fn save_base_settings_to_file(app: &MainWindow, original_preset_idx: i32) {
    let mut collected_config_from_file = collect_base_settings(app);

    // We cannot normally start app with disallowed preset, so we restore it to original value
    if collected_config_from_file.default_preset == PRESET_NUMBER as i32 - 1 {
        collected_config_from_file.default_preset = original_preset_idx;
    }

    let result = save_data_to_file(get_base_config_file(), &collected_config_from_file);

    if let Err(e) = result {
        error!("Failed to save base settings - {e}");
    }
}

pub fn save_custom_settings_to_file(app: &MainWindow) {
    let current_item = app.global::<Settings>().get_settings_preset_idx();
    let result = save_data_to_file(get_config_file(current_item), &collect_settings(app));

    if let Err(e) = result {
        error!("Failed to save custom settings - {e}");
    }
}

pub fn load_data_from_file<T>(config_file: Option<PathBuf>) -> Result<T, String>
where
    for<'de> T: Deserialize<'de>,
{
    let current_time = std::time::Instant::now();
    let Some(config_file) = config_file else {
        return Err("Cannot get config file".into());
    };
    if !config_file.is_file() {
        return Err(format!("Config file \"{}\" doesn't exist", config_file.to_string_lossy()));
    }

    let result = match std::fs::read_to_string(&config_file) {
        Ok(serialized) => {
            debug!("Loading data from file {:?} took {:?}", config_file, current_time.elapsed());

            match serde_json::from_str(&serialized) {
                Ok(custom_settings) => Ok(custom_settings),
                Err(e) => Err(format!("Cannot deserialize settings: {e}")),
            }
        }
        Err(e) => Err(format!("Cannot read config file: {e}")),
    };

    debug!("Loading and converting data from file {:?} took {:?}", config_file, current_time.elapsed());

    result
}

pub fn save_data_to_file<T>(config_file: Option<PathBuf>, serializable_data: &T) -> Result<(), String>
where
    T: Serialize,
{
    let current_time = std::time::Instant::now();
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
            if let Err(e) = std::fs::write(&config_file, serialized) {
                return Err(format!("Cannot save config file: {e}"));
            }
        }
        Err(e) => {
            return Err(format!("Cannot serialize settings: {e}"));
        }
    }

    debug!("Saving data to file {:?} took {:?}", config_file, current_time.elapsed());
    Ok(())
}

pub fn get_base_config_file() -> Option<PathBuf> {
    let config_folder = get_config_cache_path()?.config_folder;
    let base_config_file = config_folder.join("config_general.json");
    Some(base_config_file)
}
pub fn get_config_file(number: i32) -> Option<PathBuf> {
    let config_folder = get_config_cache_path()?.config_folder;
    let config_file = config_folder.join(format!("config_preset_{number}.json"));
    Some(config_file)
}

pub fn set_base_settings_to_gui(app: &MainWindow, basic_settings: &BasicSettings, preset_idx: i32) {
    let settings = app.global::<Settings>();
    change_language(app);

    settings.set_settings_preset_idx(preset_idx);
    settings.set_settings_presets(ModelRc::new(create_vec_model_from_vec_string(basic_settings.preset_names.clone())));

    let width = basic_settings.window_width.clamp(100, 1920 * 4);
    let height = basic_settings.window_height.clamp(100, 1080 * 4);

    app.window().set_size(WindowSize::Physical(PhysicalSize { width, height }));

    settings.set_dark_theme(basic_settings.dark_theme);
    settings.set_show_only_icons(basic_settings.show_only_icons);
    app.global::<Callabler>().invoke_theme_changed();

    set_combobox_basic_settings_items(&settings, basic_settings);
}

pub fn set_combobox_basic_settings_items(settings: &Settings, basic_settings: &BasicSettings) {
    let collected_items = StringComboBoxItems::get_items();

    // Language
    let (idx, display_names) = StringComboBoxItems::get_item_and_idx_from_config_name(&basic_settings.language, &collected_items.languages);
    // settings.set_language_model(display_names); // TODO - replace with
    settings.set_language_index(idx as i32);
    settings.set_language_value(display_names[idx].clone());
}

pub fn set_combobox_custom_settings_items(settings: &Settings, custom_settings: &SettingsCustom) {
    let collected_items = StringComboBoxItems::get_items();

    // Hash size
    let (idx, display_names) = StringComboBoxItems::get_item_and_idx_from_config_name(&custom_settings.similar_images_sub_hash_size.to_string(), &collected_items.hash_size);
    // settings.set_similar_images_sub_hash_size_model(display_names); // TODO - replace with
    settings.set_similar_images_sub_hash_size_index(idx as i32);
    settings.set_similar_images_sub_hash_size_value(display_names[idx].clone());

    // Hash type
    let (idx, display_names) = StringComboBoxItems::get_item_and_idx_from_config_name(&custom_settings.similar_images_sub_hash_alg, &collected_items.image_hash_alg);
    // settings.set_similar_images_sub_hash_alg_model(display_names);
    settings.set_similar_images_sub_hash_alg_index(idx as i32);
    settings.set_similar_images_sub_hash_alg_value(display_names[idx].clone());

    // Resize algorithm
    let (idx, display_names) = StringComboBoxItems::get_item_and_idx_from_config_name(&custom_settings.similar_images_sub_resize_algorithm, &collected_items.resize_algorithm);
    // settings.set_similar_images_sub_resize_algorithm_model(display_names);
    settings.set_similar_images_sub_resize_algorithm_index(idx as i32);
    settings.set_similar_images_sub_resize_algorithm_value(display_names[idx].clone());

    // Duplicates check method
    let (idx, display_names) = StringComboBoxItems::get_item_and_idx_from_config_name(&custom_settings.duplicates_sub_check_method, &collected_items.duplicates_check_method);
    // settings.set_duplicates_sub_check_method_model(display_names);
    settings.set_duplicates_sub_check_method_index(idx as i32);
    settings.set_duplicates_sub_check_method_value(display_names[idx].clone());

    // Duplicates hash type
    let (idx, display_names) = StringComboBoxItems::get_item_and_idx_from_config_name(&custom_settings.duplicates_sub_available_hash_type, &collected_items.duplicates_hash_type);
    // settings.set_duplicates_sub_available_hash_type_model(display_names);
    settings.set_duplicates_sub_available_hash_type_index(idx as i32);
    settings.set_duplicates_sub_available_hash_type_value(display_names[idx].clone());

    // Biggest files method
    let (idx, display_names) = StringComboBoxItems::get_item_and_idx_from_config_name(&custom_settings.biggest_files_sub_method, &collected_items.biggest_files_method);
    // settings.set_biggest_files_sub_method_model(display_names);
    settings.set_biggest_files_sub_method_index(idx as i32);
    settings.set_biggest_files_sub_method_value(display_names[idx].clone());

    // Audio check type
    let (idx, display_names) = StringComboBoxItems::get_item_and_idx_from_config_name(&custom_settings.similar_music_sub_audio_check_type, &collected_items.audio_check_type);
    // settings.set_duplicates_sub_available_hash_type_model(display_names);
    settings.set_similar_music_sub_audio_check_type_index(idx as i32);
    settings.set_similar_music_sub_audio_check_type_value(display_names[idx].clone());
}

pub fn set_settings_to_gui(app: &MainWindow, custom_settings: &SettingsCustom, cli_args: Option<CliResult>) {
    let settings = app.global::<Settings>();

    let (included, referenced, excluded) = if let Some(cli_args) = cli_args {
        let vs_to_vp = |vec: Vec<String>| vec.into_iter().map(PathBuf::from).collect::<Vec<_>>();
        (vs_to_vp(cli_args.included_items), vs_to_vp(cli_args.referenced_items), vs_to_vp(cli_args.excluded_items))
    } else {
        (
            custom_settings.included_directories.clone(),
            custom_settings.included_directories_referenced.clone(),
            custom_settings.excluded_directories.clone(),
        )
    };
    // Included directories
    let included_directories = create_included_directories_model_from_pathbuf(&included, &referenced);
    settings.set_included_directories_model(included_directories);

    // Excluded directories
    let excluded_directories = create_excluded_directories_model_from_pathbuf(&excluded);
    settings.set_excluded_directories_model(excluded_directories);

    settings.set_excluded_items(custom_settings.excluded_items.clone().into());
    settings.set_allowed_extensions(custom_settings.allowed_extensions.clone().into());
    settings.set_excluded_extensions(custom_settings.excluded_extensions.clone().into());
    settings.set_minimum_file_size(custom_settings.minimum_file_size.to_string().into());
    settings.set_maximum_file_size(custom_settings.maximum_file_size.to_string().into());
    settings.set_use_cache(custom_settings.use_cache);
    settings.set_save_as_json(custom_settings.save_also_as_json);
    settings.set_move_to_trash(custom_settings.move_deleted_files_to_trash);
    settings.set_ignore_other_filesystems(custom_settings.ignore_other_file_systems);
    settings.set_thread_number(custom_settings.thread_number as f32);

    settings.set_recursive_search(custom_settings.recursive_search);
    settings.set_duplicate_image_preview(custom_settings.duplicate_image_preview);
    settings.set_duplicate_hide_hard_links(custom_settings.duplicate_hide_hard_links);
    settings.set_duplicate_use_prehash(custom_settings.duplicate_use_prehash);
    settings.set_duplicate_minimal_hash_cache_size(custom_settings.duplicate_minimal_hash_cache_size.to_string().into());
    settings.set_duplicate_minimal_prehash_cache_size(custom_settings.duplicate_minimal_prehash_cache_size.to_string().into());
    settings.set_duplicate_delete_outdated_entries(custom_settings.duplicate_delete_outdated_entries);
    settings.set_duplicates_sub_name_case_sensitive(custom_settings.duplicates_sub_name_case_sensitive);
    settings.set_similar_images_hide_hard_links(custom_settings.similar_images_hide_hard_links);
    settings.set_similar_images_show_image_preview(custom_settings.similar_images_show_image_preview);
    settings.set_similar_images_delete_outdated_entries(custom_settings.similar_images_delete_outdated_entries);
    settings.set_similar_videos_hide_hard_links(custom_settings.similar_videos_hide_hard_links);
    settings.set_similar_videos_delete_outdated_entries(custom_settings.similar_videos_delete_outdated_entries);
    settings.set_similar_music_compare_fingerprints_only_with_similar_titles(custom_settings.similar_music_compare_fingerprints_only_with_similar_titles);
    settings.set_similar_music_delete_outdated_entries(custom_settings.similar_music_delete_outdated_entries);

    set_combobox_custom_settings_items(&settings, custom_settings);

    settings.set_similar_images_sub_ignore_same_size(custom_settings.similar_images_sub_ignore_same_size);
    settings.set_similar_images_sub_max_similarity(MAX_HASH_SIZE);
    settings.set_similar_images_sub_current_similarity(custom_settings.similar_images_sub_similarity as f32);

    settings.set_biggest_files_sub_number_of_files(custom_settings.biggest_files_sub_number_of_files.to_string().into());

    settings.set_similar_videos_sub_ignore_same_size(custom_settings.similar_videos_sub_ignore_same_size);
    settings.set_similar_videos_sub_current_similarity(custom_settings.similar_videos_sub_similarity as f32);
    settings.set_similar_videos_sub_max_similarity(20.0);

    settings.set_similar_music_sub_approximate_comparison(custom_settings.similar_music_sub_approximate_comparison);
    settings.set_similar_music_sub_title(custom_settings.similar_music_sub_title);
    settings.set_similar_music_sub_artist(custom_settings.similar_music_sub_artist);
    settings.set_similar_music_sub_year(custom_settings.similar_music_sub_year);
    settings.set_similar_music_sub_bitrate(custom_settings.similar_music_sub_bitrate);
    settings.set_similar_music_sub_genre(custom_settings.similar_music_sub_genre);
    settings.set_similar_music_sub_length(custom_settings.similar_music_sub_length);
    settings.set_similar_music_sub_maximum_difference_value(custom_settings.similar_music_sub_maximum_difference_value);
    settings.set_similar_music_sub_minimal_fragment_duration_value(custom_settings.similar_music_sub_minimal_fragment_duration_value);

    settings.set_broken_files_sub_audio(custom_settings.broken_files_sub_audio);
    settings.set_broken_files_sub_pdf(custom_settings.broken_files_sub_pdf);
    settings.set_broken_files_sub_archive(custom_settings.broken_files_sub_archive);
    settings.set_broken_files_sub_image(custom_settings.broken_files_sub_image);

    let sel_px = 35.0;
    let path_px = 350.0;
    let name_px = 100.0;
    let mod_px = 150.0;
    let size_px = 75.0;

    let fnm = |default_model: &[f32], name: &str| {
        let model = default_model.iter().map(|s| (*s).clamp(30.0, 2500.0));
        let model = model
            .into_iter()
            .enumerate()
            .map(|(idx, data)| *custom_settings.column_sizes.get(name).cloned().unwrap_or_default().get(idx).unwrap_or(&data))
            .collect::<Vec<_>>();

        ModelRc::new(VecModel::from(model))
    };

    settings.set_duplicates_column_size(fnm(&[sel_px, size_px, name_px, path_px, mod_px], "duplicates"));
    settings.set_empty_folders_column_size(fnm(&[sel_px, name_px, path_px, mod_px], "empty_folders"));
    settings.set_empty_files_column_size(fnm(&[sel_px, name_px, path_px, mod_px], "empty_files"));
    settings.set_temporary_files_column_size(fnm(&[sel_px, name_px, path_px, mod_px], "temporary_files"));
    settings.set_big_files_column_size(fnm(&[sel_px, size_px, name_px, path_px, mod_px], "big_files"));
    settings.set_similar_images_column_size(fnm(&[sel_px, 80.0, 80.0, 80.0, name_px, path_px, mod_px], "similar_images"));
    settings.set_similar_videos_column_size(fnm(&[sel_px, size_px, name_px, path_px, mod_px], "similar_videos"));
    settings.set_similar_music_column_size(fnm(&[sel_px, size_px, name_px, 80.0, 80.0, 80.0, 80.0, 80.0, 80.0, path_px, mod_px], "similar_music"));
    settings.set_invalid_symlink_column_size(fnm(&[sel_px, name_px, path_px, path_px, mod_px], "invalid_symlink"));
    settings.set_broken_files_column_size(fnm(&[sel_px, name_px, path_px, 200.0, size_px, mod_px], "broken_files"));
    settings.set_bad_extensions_column_size(fnm(&[sel_px, name_px, path_px, 40.0, 200.0], "bad_extensions"));

    // Clear text
    app.global::<GuiState>().set_info_text("".into());
}

pub fn collect_settings(app: &MainWindow) -> SettingsCustom {
    let settings = app.global::<Settings>();

    let collected_items = StringComboBoxItems::regenerate_items();

    let included_directories_model = settings.get_included_directories_model();
    let included_directories = included_directories_model.iter().map(|model| PathBuf::from(model.path.as_str())).collect::<Vec<_>>();
    let included_directories_referenced = included_directories_model
        .iter()
        .filter(|model| model.referenced_folder)
        .map(|model| PathBuf::from(model.path.as_str()))
        .collect::<Vec<_>>();

    let excluded_directories_model = settings.get_excluded_directories_model();
    let excluded_directories = excluded_directories_model.iter().map(|model| PathBuf::from(model.path.as_str())).collect::<Vec<_>>();

    let excluded_items = settings.get_excluded_items().to_string();
    let allowed_extensions = settings.get_allowed_extensions().to_string();
    let excluded_extensions = settings.get_excluded_extensions().to_string();
    let minimum_file_size = settings.get_minimum_file_size().parse::<i32>().unwrap_or(DEFAULT_MINIMUM_SIZE_KB);
    let maximum_file_size = settings.get_maximum_file_size().parse::<i32>().unwrap_or(DEFAULT_MAXIMUM_SIZE_KB);

    let recursive_search = settings.get_recursive_search();
    let use_cache = settings.get_use_cache();
    let save_also_as_json = settings.get_save_as_json();
    let move_deleted_files_to_trash = settings.get_move_to_trash();
    let ignore_other_file_systems = settings.get_ignore_other_filesystems();
    let thread_number = settings.get_thread_number().round() as i32;

    let duplicate_image_preview = settings.get_duplicate_image_preview();
    let duplicate_hide_hard_links = settings.get_duplicate_hide_hard_links();
    let duplicate_use_prehash = settings.get_duplicate_use_prehash();
    let duplicate_minimal_hash_cache_size = settings.get_duplicate_minimal_hash_cache_size().parse::<i32>().unwrap_or(DEFAULT_MINIMUM_CACHE_SIZE);
    let duplicate_minimal_prehash_cache_size = settings
        .get_duplicate_minimal_prehash_cache_size()
        .parse::<i32>()
        .unwrap_or(DEFAULT_MINIMUM_PREHASH_CACHE_SIZE);
    let duplicate_delete_outdated_entries = settings.get_duplicate_delete_outdated_entries();
    let duplicates_sub_name_case_sensitive = settings.get_duplicates_sub_name_case_sensitive();

    let similar_images_hide_hard_links = settings.get_similar_images_hide_hard_links();
    let similar_images_show_image_preview = settings.get_similar_images_show_image_preview();
    let similar_images_delete_outdated_entries = settings.get_similar_images_delete_outdated_entries();

    let similar_videos_hide_hard_links = settings.get_similar_videos_hide_hard_links();
    let similar_videos_delete_outdated_entries = settings.get_similar_videos_delete_outdated_entries();

    let similar_music_compare_fingerprints_only_with_similar_titles = settings.get_similar_music_compare_fingerprints_only_with_similar_titles();
    let similar_music_delete_outdated_entries = settings.get_similar_music_delete_outdated_entries();

    let similar_images_sub_hash_size_idx = settings.get_similar_images_sub_hash_size_index();
    let similar_images_sub_hash_size = StringComboBoxItems::get_config_name_from_idx(similar_images_sub_hash_size_idx as usize, &collected_items.hash_size);
    let similar_images_sub_hash_alg_idx = settings.get_similar_images_sub_hash_alg_index();
    let similar_images_sub_hash_alg = StringComboBoxItems::get_config_name_from_idx(similar_images_sub_hash_alg_idx as usize, &collected_items.image_hash_alg);
    let similar_images_sub_resize_algorithm_idx = settings.get_similar_images_sub_resize_algorithm_index();
    let similar_images_sub_resize_algorithm = StringComboBoxItems::get_config_name_from_idx(similar_images_sub_resize_algorithm_idx as usize, &collected_items.resize_algorithm);
    let similar_images_sub_ignore_same_size = settings.get_similar_images_sub_ignore_same_size();
    let similar_images_sub_similarity = settings.get_similar_images_sub_current_similarity().round() as i32;

    let duplicates_sub_check_method_idx = settings.get_duplicates_sub_check_method_index();
    let duplicates_sub_check_method = StringComboBoxItems::get_config_name_from_idx(duplicates_sub_check_method_idx as usize, &collected_items.duplicates_check_method);
    let duplicates_sub_available_hash_type_idx = settings.get_duplicates_sub_available_hash_type_index();
    let duplicates_sub_available_hash_type = StringComboBoxItems::get_config_name_from_idx(duplicates_sub_available_hash_type_idx as usize, &collected_items.duplicates_hash_type);

    let biggest_files_sub_method_idx = settings.get_biggest_files_sub_method_index();
    let biggest_files_sub_method = StringComboBoxItems::get_config_name_from_idx(biggest_files_sub_method_idx as usize, &collected_items.biggest_files_method);
    let biggest_files_sub_number_of_files = settings.get_biggest_files_sub_number_of_files().parse().unwrap_or(DEFAULT_BIGGEST_FILES);

    let similar_videos_sub_ignore_same_size = settings.get_similar_videos_sub_ignore_same_size();
    let similar_videos_sub_similarity = settings.get_similar_videos_sub_current_similarity().round() as i32;

    let similar_music_sub_audio_check_type_idx = settings.get_similar_music_sub_audio_check_type_index();
    let similar_music_sub_audio_check_type = StringComboBoxItems::get_config_name_from_idx(similar_music_sub_audio_check_type_idx as usize, &collected_items.audio_check_type);
    let similar_music_sub_approximate_comparison = settings.get_similar_music_sub_approximate_comparison();
    let similar_music_sub_title = settings.get_similar_music_sub_title();
    let similar_music_sub_artist = settings.get_similar_music_sub_artist();
    let similar_music_sub_year = settings.get_similar_music_sub_year();
    let similar_music_sub_bitrate = settings.get_similar_music_sub_bitrate();
    let similar_music_sub_genre = settings.get_similar_music_sub_genre();
    let similar_music_sub_length = settings.get_similar_music_sub_length();
    let similar_music_sub_maximum_difference_value = settings.get_similar_music_sub_maximum_difference_value();
    let similar_music_sub_minimal_fragment_duration_value = settings.get_similar_music_sub_minimal_fragment_duration_value();

    let broken_files_sub_audio = settings.get_broken_files_sub_audio();
    let broken_files_sub_pdf = settings.get_broken_files_sub_pdf();
    let broken_files_sub_archive = settings.get_broken_files_sub_archive();
    let broken_files_sub_image = settings.get_broken_files_sub_image();

    let column_sizes = BTreeMap::from([
        ("duplicates".to_string(), settings.get_duplicates_column_size().iter().collect::<Vec<_>>()),
        ("empty_folders".to_string(), settings.get_empty_folders_column_size().iter().collect::<Vec<_>>()),
        ("empty_files".to_string(), settings.get_empty_files_column_size().iter().collect::<Vec<_>>()),
        ("temporary_files".to_string(), settings.get_temporary_files_column_size().iter().collect::<Vec<_>>()),
        ("big_files".to_string(), settings.get_big_files_column_size().iter().collect::<Vec<_>>()),
        ("similar_images".to_string(), settings.get_similar_images_column_size().iter().collect::<Vec<_>>()),
        ("similar_videos".to_string(), settings.get_similar_videos_column_size().iter().collect::<Vec<_>>()),
        ("similar_music".to_string(), settings.get_similar_music_column_size().iter().collect::<Vec<_>>()),
        ("invalid_symlink".to_string(), settings.get_invalid_symlink_column_size().iter().collect::<Vec<_>>()),
        ("broken_files".to_string(), settings.get_broken_files_column_size().iter().collect::<Vec<_>>()),
        ("bad_extensions".to_string(), settings.get_bad_extensions_column_size().iter().collect::<Vec<_>>()),
    ]);

    SettingsCustom {
        included_directories,
        included_directories_referenced,
        excluded_directories,
        excluded_items,
        allowed_extensions,
        excluded_extensions,
        minimum_file_size,
        maximum_file_size,
        recursive_search,
        use_cache,
        save_also_as_json,
        move_deleted_files_to_trash,
        ignore_other_file_systems,
        thread_number,
        duplicate_image_preview,
        duplicate_hide_hard_links,
        duplicate_use_prehash,
        duplicate_minimal_hash_cache_size,
        duplicate_minimal_prehash_cache_size,
        duplicate_delete_outdated_entries,
        similar_images_hide_hard_links,
        similar_images_show_image_preview,
        similar_images_delete_outdated_entries,
        similar_videos_delete_outdated_entries,
        similar_music_delete_outdated_entries,
        similar_images_sub_hash_size,
        similar_images_sub_hash_alg,
        similar_images_sub_resize_algorithm,
        similar_images_sub_ignore_same_size,
        similar_images_sub_similarity,
        duplicates_sub_check_method,
        duplicates_sub_available_hash_type,
        duplicates_sub_name_case_sensitive,
        biggest_files_sub_method,
        biggest_files_sub_number_of_files,
        similar_videos_hide_hard_links,
        similar_videos_sub_ignore_same_size,
        similar_videos_sub_similarity,
        similar_music_sub_audio_check_type,
        similar_music_sub_approximate_comparison,
        similar_music_compare_fingerprints_only_with_similar_titles,
        similar_music_sub_title,
        similar_music_sub_artist,
        similar_music_sub_year,
        similar_music_sub_bitrate,
        similar_music_sub_genre,
        similar_music_sub_length,
        similar_music_sub_maximum_difference_value,
        similar_music_sub_minimal_fragment_duration_value,
        broken_files_sub_audio,
        broken_files_sub_pdf,
        broken_files_sub_archive,
        broken_files_sub_image,
        column_sizes,
    }
}

pub fn collect_base_settings(app: &MainWindow) -> BasicSettings {
    let settings = app.global::<Settings>();
    let collected_items = StringComboBoxItems::regenerate_items();

    let default_preset = settings.get_settings_preset_idx();
    let preset_names = settings.get_settings_presets().iter().map(|x| x.to_string()).collect::<Vec<_>>();
    let window_width = (app.window().size().width as f32 / app.window().scale_factor()) as u32;
    let window_height = (app.window().size().height as f32 / app.window().scale_factor()) as u32;

    assert_eq!(preset_names.len(), PRESET_NUMBER);
    let lang_idx = settings.get_language_index();
    let language = StringComboBoxItems::get_config_name_from_idx(lang_idx as usize, &collected_items.languages);
    // let language = LANGUAGE_LIST[lang_idx as usize].short_name.to_string();
    let dark_theme = settings.get_dark_theme();
    let show_only_icons = settings.get_show_only_icons();
    BasicSettings {
        language,
        default_preset,
        preset_names,
        window_width,
        window_height,
        dark_theme,
        show_only_icons,
    }
}

fn detect_language() -> String {
    let lang_idx = find_the_closest_language_idx_to_system();
    LANGUAGE_LIST[lang_idx].short_name.to_string()
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
fn default_duplicates_check_method() -> String {
    "hash".to_string()
}
fn default_maximum_difference_value() -> f32 {
    DEFAULT_MAXIMUM_DIFFERENCE_VALUE
}
fn default_minimal_fragment_duration_value() -> f32 {
    DEFAULT_MINIMAL_FRAGMENT_DURATION_VALUE
}
fn default_duplicates_hash_type() -> String {
    "blake3".to_string()
}
fn default_biggest_method() -> String {
    "biggest".to_string()
}
fn default_audio_check_type() -> String {
    "tags".to_string()
}
fn default_video_similarity() -> i32 {
    DEFAULT_VIDEO_SIMILARITY
}
fn default_biggest_files() -> i32 {
    DEFAULT_BIGGEST_FILES
}

pub fn default_image_similarity() -> i32 {
    DEFAULT_IMAGE_SIMILARITY
}
fn default_excluded_items() -> String {
    DEFAULT_EXCLUDED_ITEMS.to_string()
}

fn default_preset_names() -> Vec<String> {
    let mut v = (0..(PRESET_NUMBER - 1)).map(|x| format!("Preset {}", x + 1)).collect::<Vec<_>>();
    v.push(PRESET_NAME_RESERVED.to_string());
    v
}

fn minimum_file_size() -> i32 {
    DEFAULT_MINIMUM_SIZE_KB
}
fn maximum_file_size() -> i32 {
    DEFAULT_MAXIMUM_SIZE_KB
}
fn ttrue() -> bool {
    true
}
fn minimal_hash_cache_size() -> i32 {
    DEFAULT_MINIMUM_CACHE_SIZE
}
fn minimal_prehash_cache_size() -> i32 {
    DEFAULT_MINIMUM_PREHASH_CACHE_SIZE
}

pub fn default_resize_algorithm() -> String {
    "lanczos3".to_string()
}
pub fn default_hash_type() -> String {
    "mean".to_string()
}
pub fn default_sub_hash_size() -> String {
    DEFAULT_HASH_SIZE.to_string()
}
fn default_window_width() -> u32 {
    DEFAULT_WINDOW_WIDTH
}
fn default_window_height() -> u32 {
    DEFAULT_WINDOW_HEIGHT
}
