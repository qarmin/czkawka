pub mod core;
pub mod traits;

#[cfg(test)]
mod tests;

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::time::Duration;

use bk_tree::BKTree;
use hamming_bitwise_fast::hamming_bitwise_fast;
use image_hasher::{FilterType, HashAlg};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::common::model::FileEntry;
use crate::common::tool_data::CommonToolData;
use crate::common::traits::ResultEntry;

type ImHash = Vec<u8>;

// 40 is, similar like previous 20 in 8 hash size is useless
// But since Krowka have problems with proper changing max value in fly
// hardcode 40 as max value
pub const SIMILAR_VALUES: [[u32; 6]; 4] = [
    [1, 2, 5, 7, 14, 40],    // 8
    [2, 5, 15, 30, 40, 40],  // 16
    [4, 10, 20, 40, 40, 40], // 32
    [6, 20, 40, 40, 40, 40], // 64
];

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ImagesEntry {
    pub path: PathBuf,
    pub size: u64,
    pub width: u32,
    pub height: u32,
    pub modified_date: u64,
    pub hash: ImHash,
    pub similarity: u32,
}

impl ResultEntry for ImagesEntry {
    fn get_path(&self) -> &Path {
        &self.path
    }
    fn get_modified_date(&self) -> u64 {
        self.modified_date
    }
    fn get_size(&self) -> u64 {
        self.size
    }
}
impl FileEntry {
    fn into_images_entry(self) -> ImagesEntry {
        ImagesEntry {
            size: self.size,
            path: self.path,
            modified_date: self.modified_date,

            width: 0,
            height: 0,
            hash: Vec::new(),
            similarity: 0,
        }
    }
}

#[derive(Clone, Debug, Copy)]
pub enum SimilarityPreset {
    Original,
    VeryHigh,
    High,
    Medium,
    Small,
    VerySmall,
    Minimal,
    None,
}

struct Hamming;

impl bk_tree::Metric<ImHash> for Hamming {
    fn distance(&self, a: &ImHash, b: &ImHash) -> u32 {
        hamming_bitwise_fast(a, b)
    }

    fn threshold_distance(&self, a: &ImHash, b: &ImHash, _threshold: u32) -> Option<u32> {
        Some(self.distance(a, b))
    }
}

#[derive(Clone)]
pub struct SimilarImagesParameters {
    pub similarity: u32,
    pub hash_size: u8,
    pub hash_alg: HashAlg,
    pub image_filter: FilterType,
    pub exclude_images_with_same_size: bool,
    pub ignore_hard_links: bool,
}

impl SimilarImagesParameters {
    pub fn new(similarity: u32, hash_size: u8, hash_alg: HashAlg, image_filter: FilterType, exclude_images_with_same_size: bool, ignore_hard_links: bool) -> Self {
        assert!([8, 16, 32, 64].contains(&hash_size));
        Self {
            similarity,
            hash_size,
            hash_alg,
            image_filter,
            exclude_images_with_same_size,
            ignore_hard_links,
        }
    }
}

pub struct SimilarImages {
    common_data: CommonToolData,
    information: Info,
    bktree: BKTree<ImHash, Hamming>,
    similar_vectors: Vec<Vec<ImagesEntry>>,
    similar_referenced_vectors: Vec<(ImagesEntry, Vec<ImagesEntry>)>,
    // Hashmap with image hashes and Vector with names of files
    image_hashes: IndexMap<ImHash, Vec<ImagesEntry>>,
    images_to_check: BTreeMap<String, ImagesEntry>,
    params: SimilarImagesParameters,
}

#[derive(Default, Clone)]
pub struct Info {
    pub initial_found_files: usize,
    pub number_of_duplicates: usize,
    pub number_of_groups: usize,
    pub scanning_time: Duration,
}

impl SimilarImages {
    pub fn get_params(&self) -> &SimilarImagesParameters {
        &self.params
    }

    pub const fn get_similar_images(&self) -> &Vec<Vec<ImagesEntry>> {
        &self.similar_vectors
    }

    pub fn get_similar_images_referenced(&self) -> &Vec<(ImagesEntry, Vec<ImagesEntry>)> {
        &self.similar_referenced_vectors
    }

    pub fn get_use_reference(&self) -> bool {
        self.common_data.use_reference_folders
    }

    pub const fn get_information(&self) -> &Info {
        &self.information
    }
}
