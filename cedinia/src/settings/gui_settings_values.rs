use std::fmt::Debug;

use czkawka_core::common::model::{CheckingMethod, HashType};
use czkawka_core::re_exported::{FilterType, HashAlg};
use czkawka_core::tools::big_file::SearchMode;
use czkawka_core::tools::similar_images::SimilarityPreset;
use log::warn;

#[derive(Debug, Clone)]
pub struct StringComboBoxItem<T>
where
    T: Clone + Debug,
{
    pub config_name: String,
    pub display_name: String,
    /// When set, `translated_display_name()` does a runtime i18n lookup.
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
}

impl Default for StringComboBoxItems {
    fn default() -> Self {
        Self::new()
    }
}

impl StringComboBoxItems {
    pub fn new() -> Self {
        // Translated via i18n runtime lookup
        let min_file_size = Self::convert_i18n(
            &[
                ("none", MinFileSize::None),
                ("1kb", MinFileSize::OneKb),
                ("8kb", MinFileSize::EightKb),
                ("64kb", MinFileSize::SixtyFourKb),
                ("1mb", MinFileSize::OneMb),
            ],
            "option_min_size",
        );

        let max_file_size = Self::convert_i18n(
            &[
                ("16kb", MaxFileSize::SixteenKb),
                ("1mb", MaxFileSize::OneMb),
                ("10mb", MaxFileSize::TenMb),
                ("100mb", MaxFileSize::HundredMb),
                ("unlimited", MaxFileSize::Unlimited),
            ],
            "option_max_size",
        );

        let duplicates_check_method = Self::convert_i18n(
            &[
                ("hash", CheckingMethod::Hash),
                ("name", CheckingMethod::Name),
                ("size_and_name", CheckingMethod::SizeName),
                ("size", CheckingMethod::Size),
            ],
            "option_check_method",
        );

        // Hash type names are proper nouns – not translated
        let duplicates_hash_type = Self::convert(&[
            ("blake3", "Blake3", HashType::Blake3),
            ("crc32", "CRC32", HashType::Crc32),
            ("xxh3", "XXH3", HashType::Xxh3),
        ]);

        // Sizes are numbers – not translated
        let hash_size = Self::convert(&[("8", "8", 8u8), ("16", "16", 16), ("32", "32", 32), ("64", "64", 64)]);

        let biggest_files_method = Self::convert_i18n(&[("biggest", SearchMode::BiggestFiles), ("smallest", SearchMode::SmallestFiles)], "option_search_mode");

        // Counts are numbers – not translated
        let big_files_count = Self::convert(&[("5", "5", 5usize), ("50", "50", 50), ("500", "500", 500), ("5000", "5000", 5000)]);

        // Config names aligned with FTL keys: "low" / "very_low" instead of "small" / "very_small"
        let similarity_preset = Self::convert_i18n(
            &[
                ("very_high", SimilarityPreset::VeryHigh),
                ("high", SimilarityPreset::High),
                ("medium", SimilarityPreset::Medium),
                ("low", SimilarityPreset::Small),
                ("very_low", SimilarityPreset::VerySmall),
                ("minimal", SimilarityPreset::Minimal),
            ],
            "option_similarity",
        );

        // Algorithm names are proper nouns – not translated
        let hash_alg = Self::convert(&[
            ("mean", "Mean", HashAlg::Mean),
            ("gradient", "Gradient", HashAlg::Gradient),
            ("double_gradient", "D.Grad.", HashAlg::DoubleGradient),
            ("vert_gradient", "V.Grad.", HashAlg::VertGradient),
            ("median", "Median", HashAlg::Median),
            ("blockhash", "Blockhash", HashAlg::Blockhash),
        ]);

        // Filter names are proper nouns – not translated
        let image_filter = Self::convert(&[
            ("nearest", "Nearest", FilterType::Nearest),
            ("triangle", "Triangle", FilterType::Triangle),
            ("catmull_rom", "CatmullRom", FilterType::CatmullRom),
            ("gaussian", "Gaussian", FilterType::Gaussian),
            ("lanczos3", "Lanczos3", FilterType::Lanczos3),
        ]);

        let same_music_check_method = Self::convert_i18n(&[("tags", CheckingMethod::AudioTags), ("audio", CheckingMethod::AudioContent)], "option_music_method");

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
        }
    }

    /// Build items with static (non-translated) display names.
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

    /// Build items whose display names are looked up at runtime via `{key_prefix}_{config_name}`.
    fn convert_i18n<T>(input: &[(&str, T)], key_prefix: &str) -> Vec<StringComboBoxItem<T>>
    where
        T: Clone + Debug,
    {
        input
            .iter()
            .map(|(config_name, value)| StringComboBoxItem {
                config_name: config_name.to_string(),
                display_name: config_name.to_string(),
                i18n_key: Some(format!("{key_prefix}_{config_name}")),
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
