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
    pub value: T,
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
    /// Returns `None` for Unlimited (no limit imposed).
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
        let min_file_size = Self::convert(&[
            ("none", "Brak", MinFileSize::None),
            ("1kb", "1 KB", MinFileSize::OneKb),
            ("8kb", "8 KB", MinFileSize::EightKb),
            ("64kb", "64 KB", MinFileSize::SixtyFourKb),
            ("1mb", "1 MB", MinFileSize::OneMb),
        ]);

        let max_file_size = Self::convert(&[
            ("16kb", "16 KB", MaxFileSize::SixteenKb),
            ("1mb", "1 MB", MaxFileSize::OneMb),
            ("10mb", "10 MB", MaxFileSize::TenMb),
            ("100mb", "100 MB", MaxFileSize::HundredMb),
            ("unlimited", "Bez limitu", MaxFileSize::Unlimited),
        ]);

        let duplicates_check_method = Self::convert(&[
            ("hash", "Hash", CheckingMethod::Hash),
            ("name", "Nazwa", CheckingMethod::Name),
            ("size_and_name", "Rozm+Naz", CheckingMethod::SizeName),
            ("size", "Rozmiar", CheckingMethod::Size),
        ]);

        let duplicates_hash_type = Self::convert(&[
            ("blake3", "Blake3", HashType::Blake3),
            ("crc32", "CRC32", HashType::Crc32),
            ("xxh3", "XXH3", HashType::Xxh3),
        ]);

        let hash_size = Self::convert(&[("8", "8", 8u8), ("16", "16", 16), ("32", "32", 32), ("64", "64", 64)]);

        let biggest_files_method = Self::convert(&[("biggest", "Największe", SearchMode::BiggestFiles), ("smallest", "Najmniejsze", SearchMode::SmallestFiles)]);

        let big_files_count = Self::convert(&[("5", "5", 5usize), ("50", "50", 50), ("500", "500", 500), ("5000", "5000", 5000)]);

        let similarity_preset = Self::convert(&[
            ("very_high", "B.Wys.", SimilarityPreset::VeryHigh),
            ("high", "Wysoki", SimilarityPreset::High),
            ("medium", "Średni", SimilarityPreset::Medium),
            ("small", "Niski", SimilarityPreset::Small),
            ("very_small", "B.Niski", SimilarityPreset::VerySmall),
            ("minimal", "Min.", SimilarityPreset::Minimal),
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

        let same_music_check_method = Self::convert(&[("tags", "Tagi", CheckingMethod::AudioTags), ("audio", "Audio", CheckingMethod::AudioContent)]);

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

    fn convert<T>(input: &[(&str, &str, T)]) -> Vec<StringComboBoxItem<T>>
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

    pub fn idx_from_config_name<T: Clone + Debug>(config_name: &str, items: &[StringComboBoxItem<T>]) -> usize {
        items.iter().position(|e| e.config_name == config_name).unwrap_or_else(|| {
            warn!("Unknown config_name \"{config_name}\" in {items:?}, falling back to index 0");
            0
        })
    }

    /// Look up enum value by UI index. Use instead of `value_from_config_name` when only the
    /// SegmentRow idx is available (the `_value` string property may be stale).
    pub fn value_from_idx<T: Clone + Debug>(items: &[StringComboBoxItem<T>], idx: i32, default: T) -> T {
        items.get(idx as usize).map_or_else(
            || {
                warn!("idx {idx} out of range in {items:?}, using default");
                default
            },
            |e| e.value.clone(),
        )
    }

    /// Look up the config_name string by UI index. Use in `collect_settings_from_gui`.
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
