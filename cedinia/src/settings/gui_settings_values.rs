use std::fmt::Debug;

use czkawka_core::common::model::{CheckingMethod, HashType};
use czkawka_core::re_exported::{FilterType, HashAlg};
use czkawka_core::tools::big_file::SearchMode;
use czkawka_core::tools::similar_images::SimilarityPreset;
use log::warn;

pub enum DisplaySpec {
    Const(&'static str),
    Translatable(&'static str),
}

#[derive(Debug, Clone)]
pub struct StringComboBoxItem<T>
where
    T: Clone + Debug,
{
    pub config_name: String,
    pub display_name: String,

    pub i18n_key: Option<String>,
    pub value: T,
}

impl<T: Clone + Debug> StringComboBoxItem<T> {
    pub fn translated_display_name(&self) -> String {
        if let Some(key) = &self.i18n_key {
            crate::localizer_cedinia::LANGUAGE_LOADER_CEDINIA.get(key)
        } else {
            self.display_name.clone()
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MinFileSize {
    None,
    OneKb,
    EightKb,
    SixtyFourKb,
    OneMb,
}

impl MinFileSize {
    pub fn to_bytes(self) -> u64 {
        match self {
            Self::None => 0,
            Self::OneKb => 1_024,
            Self::EightKb => 8 * 1_024,
            Self::SixtyFourKb => 64 * 1_024,
            Self::OneMb => 1_024 * 1_024,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MaxFileSize {
    SixteenKb,
    OneMb,
    TenMb,
    HundredMb,
    Unlimited,
}

impl MaxFileSize {
    pub fn to_bytes(self) -> Option<u64> {
        match self {
            Self::SixteenKb => Some(16 * 1_024),
            Self::OneMb => Some(1_024 * 1_024),
            Self::TenMb => Some(10 * 1_024 * 1_024),
            Self::HundredMb => Some(100 * 1_024 * 1_024),
            Self::Unlimited => None,
        }
    }
}

/// Parameters corresponding to one of the audio-similarity presets for Similar Videos.
#[derive(Debug, Clone)]
pub struct AudioPresetParams {
    pub similarity_percent: f64,
    pub maximum_difference: f64,
    pub length_ratio: f64,
    pub min_duration_seconds: u32,
}

pub struct StringComboBoxItems {
    pub min_file_size: Vec<StringComboBoxItem<MinFileSize>>,
    pub max_file_size: Vec<StringComboBoxItem<MaxFileSize>>,
    pub duplicates_check_method: Vec<StringComboBoxItem<CheckingMethod>>,
    pub duplicates_hash_type: Vec<StringComboBoxItem<HashType>>,
    pub hash_size: Vec<StringComboBoxItem<u8>>,
    pub biggest_files_method: Vec<StringComboBoxItem<SearchMode>>,
    pub big_files_count: Vec<StringComboBoxItem<usize>>,
    pub similarity_preset: Vec<StringComboBoxItem<SimilarityPreset>>,
    pub hash_alg: Vec<StringComboBoxItem<HashAlg>>,
    pub image_filter: Vec<StringComboBoxItem<FilterType>>,
    pub same_music_check_method: Vec<StringComboBoxItem<CheckingMethod>>,
    pub similar_videos_audio_preset: Vec<StringComboBoxItem<AudioPresetParams>>,
}

impl Default for StringComboBoxItems {
    fn default() -> Self {
        Self::new()
    }
}

impl StringComboBoxItems {
    pub fn new() -> Self {
        let min_file_size = Self::convert_i18n(&[
            ("none", MinFileSize::None, DisplaySpec::Translatable("option_min_size_none")),
            ("1kb", MinFileSize::OneKb, DisplaySpec::Const("1 KB")),
            ("8kb", MinFileSize::EightKb, DisplaySpec::Const("8 KB")),
            ("64kb", MinFileSize::SixtyFourKb, DisplaySpec::Const("64 KB")),
            ("1mb", MinFileSize::OneMb, DisplaySpec::Const("1 MB")),
        ]);

        let max_file_size = Self::convert_i18n(&[
            ("16kb", MaxFileSize::SixteenKb, DisplaySpec::Const("16 KB")),
            ("1mb", MaxFileSize::OneMb, DisplaySpec::Const("1 MB")),
            ("10mb", MaxFileSize::TenMb, DisplaySpec::Const("10 MB")),
            ("100mb", MaxFileSize::HundredMb, DisplaySpec::Const("100 MB")),
            ("unlimited", MaxFileSize::Unlimited, DisplaySpec::Translatable("option_max_size_unlimited")),
        ]);

        let duplicates_check_method = Self::convert_i18n(&[
            ("hash", CheckingMethod::Hash, DisplaySpec::Translatable("option_check_method_hash")),
            ("name", CheckingMethod::Name, DisplaySpec::Translatable("option_check_method_name")),
            ("size_and_name", CheckingMethod::SizeName, DisplaySpec::Translatable("option_check_method_size_and_name")),
            ("size", CheckingMethod::Size, DisplaySpec::Translatable("option_check_method_size")),
        ]);

        let duplicates_hash_type = Self::convert(&[
            ("blake3", "Blake3", HashType::Blake3),
            ("crc32", "CRC32", HashType::Crc32),
            ("xxh3", "XXH3", HashType::Xxh3),
        ]);

        let hash_size = Self::convert(&[("8", "8", 8u8), ("16", "16", 16), ("32", "32", 32), ("64", "64", 64)]);

        let biggest_files_method = Self::convert_i18n(&[
            ("biggest", SearchMode::BiggestFiles, DisplaySpec::Translatable("option_search_mode_biggest")),
            ("smallest", SearchMode::SmallestFiles, DisplaySpec::Translatable("option_search_mode_smallest")),
        ]);

        let big_files_count = Self::convert(&[("10", "10", 10usize), ("100", "100", 100), ("1000", "1000", 1000), ("10000", "10000", 10000)]);

        let similarity_preset = Self::convert_i18n(&[
            ("very_high", SimilarityPreset::VeryHigh, DisplaySpec::Translatable("option_similarity_very_high")),
            ("high", SimilarityPreset::High, DisplaySpec::Translatable("option_similarity_high")),
            ("medium", SimilarityPreset::Medium, DisplaySpec::Translatable("option_similarity_medium")),
            ("low", SimilarityPreset::Small, DisplaySpec::Translatable("option_similarity_low")),
            ("very_low", SimilarityPreset::VerySmall, DisplaySpec::Translatable("option_similarity_very_low")),
            ("minimal", SimilarityPreset::Minimal, DisplaySpec::Translatable("option_similarity_minimal")),
        ]);

        let hash_alg = Self::convert(&[
            ("mean", "Mean", HashAlg::Mean),
            ("gradient", "Gradient", HashAlg::Gradient),
            ("double_gradient", "D.Grad.", HashAlg::DoubleGradient),
            ("vert_gradient", "V.Grad.", HashAlg::VertGradient),
            ("median", "Median", HashAlg::Median),
            ("blockhash", "Blockhash", HashAlg::Blockhash),
        ]);

        let image_filter = Self::convert(&[
            ("nearest", "Nearest", FilterType::Nearest),
            ("triangle", "Triangle", FilterType::Triangle),
            ("catmull_rom", "CatmullRom", FilterType::CatmullRom),
            ("gaussian", "Gaussian", FilterType::Gaussian),
            ("lanczos3", "Lanczos3", FilterType::Lanczos3),
        ]);

        let same_music_check_method = Self::convert_i18n(&[
            ("tags", CheckingMethod::AudioTags, DisplaySpec::Translatable("option_music_method_tags")),
            ("audio", CheckingMethod::AudioContent, DisplaySpec::Translatable("option_music_method_audio")),
        ]);

        // Preset values match krokiet's apply_similar_videos_audio_preset().
        let similar_videos_audio_preset = Self::convert_i18n(&[
            (
                "identical",
                AudioPresetParams {
                    similarity_percent: 90.0,
                    maximum_difference: 2.0,
                    length_ratio: 0.85,
                    min_duration_seconds: 5,
                },
                DisplaySpec::Translatable("option_audio_preset_identical"),
            ),
            (
                "clip_in_longer",
                AudioPresetParams {
                    similarity_percent: 90.0,
                    maximum_difference: 4.0,
                    length_ratio: 0.05,
                    min_duration_seconds: 10,
                },
                DisplaySpec::Translatable("option_audio_preset_clip"),
            ),
            (
                "similar",
                AudioPresetParams {
                    similarity_percent: 25.0,
                    maximum_difference: 6.0,
                    length_ratio: 0.4,
                    min_duration_seconds: 10,
                },
                DisplaySpec::Translatable("option_audio_preset_similar"),
            ),
        ]);

        Self {
            min_file_size,
            max_file_size,
            duplicates_check_method,
            duplicates_hash_type,
            hash_size,
            biggest_files_method,
            big_files_count,
            similarity_preset,
            hash_alg,
            image_filter,
            same_music_check_method,
            similar_videos_audio_preset,
        }
    }

    fn convert<T>(input: &[(&str, &str, T)]) -> Vec<StringComboBoxItem<T>>
    where
        T: Clone + Debug,
    {
        input
            .iter()
            .map(|(config_name, display_name, value)| StringComboBoxItem {
                config_name: config_name.to_string(),
                display_name: display_name.to_string(),
                i18n_key: None,
                value: value.clone(),
            })
            .collect()
    }

    fn convert_i18n<T>(input: &[(&str, T, DisplaySpec)]) -> Vec<StringComboBoxItem<T>>
    where
        T: Clone + Debug,
    {
        input
            .iter()
            .map(|(config_name, value, spec)| StringComboBoxItem {
                config_name: config_name.to_string(),
                display_name: match spec {
                    DisplaySpec::Const(s) => s.to_string(),
                    DisplaySpec::Translatable(_) => config_name.to_string(),
                },
                i18n_key: match spec {
                    DisplaySpec::Const(_) => None,
                    DisplaySpec::Translatable(key) => Some(key.to_string()),
                },
                value: value.clone(),
            })
            .collect()
    }

    pub fn idx_from_config_name<T: Clone + Debug>(config_name: &str, items: &[StringComboBoxItem<T>]) -> usize {
        items.iter().position(|e| e.config_name == config_name).unwrap_or_else(|| {
            warn!("Unknown config_name \"{config_name}\" in {items:?}, falling back to index 0");
            0
        })
    }

    pub fn value_from_idx<T: Clone + Debug>(items: &[StringComboBoxItem<T>], idx: i32, default: T) -> T {
        items.get(idx as usize).map_or_else(
            || {
                warn!("idx {idx} out of range in {items:?}, using default");
                default
            },
            |e| e.value.clone(),
        )
    }

    pub fn config_name_from_idx<T: Clone + Debug>(items: &[StringComboBoxItem<T>], idx: i32, default: &str) -> String {
        items.get(idx as usize).map_or_else(
            || {
                warn!("idx {idx} out of range in {items:?}, defaulting to \"{default}\"");
                default.to_string()
            },
            |e| e.config_name.clone(),
        )
    }

    pub fn value_from_config_name<T: Clone + Debug>(config_name: &str, items: &[StringComboBoxItem<T>], default: T) -> T {
        items.iter().find(|e| e.config_name == config_name).map_or_else(
            || {
                warn!("Unknown config_name \"{config_name}\" in {items:?}, using default");
                default
            },
            |e| e.value.clone(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn idx_from_config_name_found() {
        let items = StringComboBoxItems::new();
        assert_eq!(StringComboBoxItems::idx_from_config_name("blake3", &items.duplicates_hash_type), 0);
        assert_eq!(StringComboBoxItems::idx_from_config_name("crc32", &items.duplicates_hash_type), 1);
        assert_eq!(StringComboBoxItems::idx_from_config_name("xxh3", &items.duplicates_hash_type), 2);
    }

    #[test]
    fn idx_from_config_name_unknown_falls_back_to_zero() {
        let items = StringComboBoxItems::new();
        assert_eq!(StringComboBoxItems::idx_from_config_name("nonexistent", &items.duplicates_hash_type), 0);
    }

    #[test]
    fn const_items_have_no_i18n_key() {
        let items = StringComboBoxItems::new();
        let blake3 = &items.duplicates_hash_type[0];
        assert_eq!(blake3.i18n_key, None);
        assert_eq!(blake3.display_name, "Blake3");
    }

    #[test]
    fn translatable_items_have_explicit_i18n_key() {
        let items = StringComboBoxItems::new();
        let hash_method = &items.duplicates_check_method[0];
        assert_eq!(hash_method.i18n_key.as_deref(), Some("option_check_method_hash"));
        assert_eq!(hash_method.config_name, "hash");
    }

    #[test]
    fn value_from_idx_in_range() {
        let items = StringComboBoxItems::new();
        assert_eq!(StringComboBoxItems::value_from_idx(&items.hash_size, 0, 0u8), 8u8);
        assert_eq!(StringComboBoxItems::value_from_idx(&items.hash_size, 2, 0u8), 32u8);
        assert_eq!(StringComboBoxItems::value_from_idx(&items.hash_size, 3, 0u8), 64u8);
    }

    #[test]
    fn value_from_idx_out_of_range_returns_default() {
        let items = StringComboBoxItems::new();
        assert_eq!(StringComboBoxItems::value_from_idx(&items.hash_size, 100, 99u8), 99u8);
    }

    #[test]
    fn config_name_from_idx_roundtrip() {
        let items = StringComboBoxItems::new();
        assert_eq!(StringComboBoxItems::config_name_from_idx(&items.duplicates_hash_type, 1, ""), "crc32");
    }

    #[test]
    fn config_name_from_idx_out_of_range_returns_default() {
        let items = StringComboBoxItems::new();
        assert_eq!(StringComboBoxItems::config_name_from_idx(&items.hash_size, 99, "fallback"), "fallback");
    }

    #[test]
    fn value_from_config_name_found() {
        let items = StringComboBoxItems::new();
        assert_eq!(StringComboBoxItems::value_from_config_name("8", &items.hash_size, 0u8), 8u8);
        assert_eq!(StringComboBoxItems::value_from_config_name("64", &items.hash_size, 0u8), 64u8);
    }

    #[test]
    fn value_from_config_name_unknown_returns_default() {
        let items = StringComboBoxItems::new();
        assert_eq!(StringComboBoxItems::value_from_config_name("999kb", &items.hash_size, 0u8), 0u8);
    }

    #[test]
    fn min_file_size_none_is_zero_bytes() {
        assert_eq!(MinFileSize::None.to_bytes(), 0);
    }

    #[test]
    fn max_file_size_unlimited_is_none() {
        assert_eq!(MaxFileSize::Unlimited.to_bytes(), None);
    }

    #[test]
    fn all_combo_boxes_have_at_least_one_entry() {
        let items = StringComboBoxItems::new();
        assert!(!items.min_file_size.is_empty());
        assert!(!items.max_file_size.is_empty());
        assert!(!items.duplicates_check_method.is_empty());
        assert!(!items.duplicates_hash_type.is_empty());
        assert!(!items.hash_size.is_empty());
        assert!(!items.similarity_preset.is_empty());
        assert!(!items.hash_alg.is_empty());
        assert!(!items.image_filter.is_empty());
        assert!(!items.same_music_check_method.is_empty());
        assert!(!items.similar_videos_audio_preset.is_empty());
    }

    #[test]
    fn similar_videos_audio_preset_config_names_are_stable() {
        let items = StringComboBoxItems::new();
        let names: Vec<&str> = items.similar_videos_audio_preset.iter().map(|e| e.config_name.as_str()).collect();
        assert_eq!(names, &["identical", "clip_in_longer", "similar"]);
    }
}
