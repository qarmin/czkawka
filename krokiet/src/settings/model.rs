use std::collections::BTreeMap;
use std::env;
use std::path::PathBuf;

use czkawka_core::common_items::{DEFAULT_EXCLUDED_DIRECTORIES, DEFAULT_EXCLUDED_ITEMS};
use home::home_dir;
use serde::{Deserialize, Serialize};
use czkawka_core::tools::similar_videos::{crop_detect_to_str, DEFAULT_CROP_DETECT, DEFAULT_SKIP_FORWARD_AMOUNT, DEFAULT_VID_HASH_DURATION};
use crate::connect_translation::{LANGUAGE_LIST, find_the_closest_language_idx_to_system};

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
    #[serde(default = "default_similar_videos_skip_forward_amount")]
    pub similar_videos_skip_forward_amount: u32,
    #[serde(default = "default_similar_videos_vid_hash_duration")]
    pub similar_videos_vid_hash_duration: u32,
    #[serde(default = "default_similar_videos_crop_detect")]
    pub similar_videos_crop_detect: String,
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
fn default_similar_videos_skip_forward_amount() -> u32 {
    DEFAULT_SKIP_FORWARD_AMOUNT
}
fn default_similar_videos_vid_hash_duration() -> u32 {
    DEFAULT_VID_HASH_DURATION
}
fn default_similar_videos_crop_detect() -> String {
    crop_detect_to_str(DEFAULT_CROP_DETECT)
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

pub(crate) fn default_image_similarity() -> i32 {
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

pub(crate) fn default_resize_algorithm() -> String {
    "lanczos3".to_string()
}
pub(crate) fn default_hash_type() -> String {
    "mean".to_string()
}
pub(crate) fn default_sub_hash_size() -> String {
    DEFAULT_HASH_SIZE.to_string()
}
fn default_window_width() -> u32 {
    DEFAULT_WINDOW_WIDTH
}
fn default_window_height() -> u32 {
    DEFAULT_WINDOW_HEIGHT
}
