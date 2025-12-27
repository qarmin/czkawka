use bitflags::bitflags;
pub mod core;
pub mod traits;

#[cfg(test)]
mod tests;

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::time::Duration;

use rusty_chromaprint::Configuration;
use serde::{Deserialize, Serialize};

use crate::common::model::{CheckingMethod, FileEntry};
use crate::common::tool_data::CommonToolData;
use crate::common::traits::ResultEntry;

bitflags! {
    #[derive(PartialEq, Copy, Clone, Debug)]
    pub struct MusicSimilarity : u32 {
        const NONE = 0;

        const TRACK_TITLE = 0b1;
        const TRACK_ARTIST = 0b10;
        const YEAR = 0b100;
        const LENGTH = 0b1000;
        const GENRE = 0b10000;
        const BITRATE = 0b10_0000;
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MusicEntry {
    pub size: u64,

    pub path: PathBuf,
    pub modified_date: u64,
    pub fingerprint: Vec<u32>,

    pub track_title: String,
    pub track_artist: String,
    pub year: String,
    pub length: String,
    pub genre: String,
    pub bitrate: u32,
}

impl ResultEntry for MusicEntry {
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
    fn into_music_entry(self) -> MusicEntry {
        MusicEntry {
            size: self.size,
            path: self.path,
            modified_date: self.modified_date,

            fingerprint: vec![],
            track_title: String::new(),
            track_artist: String::new(),
            year: String::new(),
            length: String::new(),
            genre: String::new(),
            bitrate: 0,
        }
    }
}

struct GroupedFilesToCheck {
    pub base_files: Vec<MusicEntry>,
    pub files_to_compare: Vec<MusicEntry>,
}

#[derive(Default, Clone)]
pub struct Info {
    pub number_of_duplicates: usize,
    pub number_of_groups: usize,
    pub scanning_time: Duration,
}

#[derive(Clone)]
pub struct SameMusicParameters {
    pub music_similarity: MusicSimilarity,
    pub approximate_comparison: bool,
    pub check_type: CheckingMethod,
    pub minimum_segment_duration: f32,
    pub maximum_difference: f64,
    pub compare_fingerprints_only_with_similar_titles: bool,
}

impl SameMusicParameters {
    pub fn new(
        music_similarity: MusicSimilarity,
        approximate_comparison: bool,
        check_type: CheckingMethod,
        minimum_segment_duration: f32,
        maximum_difference: f64,
        compare_fingerprints_only_with_similar_titles: bool,
    ) -> Self {
        assert!(!music_similarity.is_empty());
        assert!([CheckingMethod::AudioTags, CheckingMethod::AudioContent].contains(&check_type));
        Self {
            music_similarity,
            approximate_comparison,
            check_type,
            minimum_segment_duration,
            maximum_difference,
            compare_fingerprints_only_with_similar_titles,
        }
    }
}

pub struct SameMusic {
    common_data: CommonToolData,
    information: Info,
    music_to_check: BTreeMap<String, MusicEntry>,
    music_entries: Vec<MusicEntry>,
    duplicated_music_entries: Vec<Vec<MusicEntry>>,
    duplicated_music_entries_referenced: Vec<(MusicEntry, Vec<MusicEntry>)>,
    hash_preset_config: Configuration,
    params: SameMusicParameters,
}

impl SameMusic {
    pub const fn get_duplicated_music_entries(&self) -> &Vec<Vec<MusicEntry>> {
        &self.duplicated_music_entries
    }

    pub fn get_params(&self) -> &SameMusicParameters {
        &self.params
    }

    pub const fn get_information(&self) -> &Info {
        &self.information
    }

    pub fn get_similar_music_referenced(&self) -> &Vec<(MusicEntry, Vec<MusicEntry>)> {
        &self.duplicated_music_entries_referenced
    }

    pub fn get_number_of_base_duplicated_files(&self) -> usize {
        if self.common_data.use_reference_folders {
            self.duplicated_music_entries_referenced.len()
        } else {
            self.duplicated_music_entries.len()
        }
    }

    pub fn get_use_reference(&self) -> bool {
        self.common_data.use_reference_folders
    }
}
