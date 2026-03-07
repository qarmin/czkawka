use std::fmt::Debug;

use czkawka_core::common::model::{CheckingMethod, HashType};
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

pub struct StringComboBoxItems {
    pub duplicates_check_method: Vec<StringComboBoxItem<CheckingMethod>>,
    pub duplicates_hash_type: Vec<StringComboBoxItem<HashType>>,
    pub hash_size: Vec<StringComboBoxItem<u8>>,
    pub biggest_files_method: Vec<StringComboBoxItem<SearchMode>>,
    pub big_files_count: Vec<StringComboBoxItem<usize>>,
    pub similarity_preset: Vec<StringComboBoxItem<SimilarityPreset>>,
}

impl Default for StringComboBoxItems {
    fn default() -> Self {
        Self::new()
    }
}

impl StringComboBoxItems {
    pub fn new() -> Self {
        let duplicates_check_method = Self::convert(&[
            ("hash", "Hash", CheckingMethod::Hash),
            ("name", "Name", CheckingMethod::Name),
            ("size_and_name", "Size and Name", CheckingMethod::SizeName),
            ("size", "Size", CheckingMethod::Size),
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

        Self {
            duplicates_check_method,
            duplicates_hash_type,
            hash_size,
            biggest_files_method,
            big_files_count,
            similarity_preset,
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

    pub fn value_from_config_name<T: Clone + Debug>(config_name: &str, items: &[StringComboBoxItem<T>], default: T) -> T {
        items.iter().find(|e| e.config_name == config_name).map(|e| e.value.clone()).unwrap_or_else(|| {
            warn!("Unknown config_name \"{config_name}\" in {items:?}, using default");
            default
        })
    }
}
