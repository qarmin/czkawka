mod core;
mod traits;

use std::collections::{BTreeMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::{mem, panic};

use anyhow::Context;
use crossbeam_channel::Sender;
use fun_time::fun_time;
use humansize::{BINARY, format_size};
use lofty::file::{AudioFile, TaggedFileExt};
use lofty::prelude::*;
use lofty::read_from;
use log::{debug, error};
use rayon::prelude::*;
use rusty_chromaprint::{Configuration, Fingerprinter, match_fingerprints};
use serde::{Deserialize, Serialize};
use symphonia::core::audio::SampleBuffer;
use symphonia::core::codecs::{CODEC_TYPE_NULL, DecoderOptions};
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;

use crate::common::cache::{extract_loaded_cache, load_cache_from_file_generalized_by_path, save_cache_to_file_generalized};
use crate::common::consts::AUDIO_FILES_EXTENSIONS;
use crate::common::create_crash_message;
use crate::common::dir_traversal::{DirTraversalBuilder, DirTraversalResult};
use crate::common::model::{CheckingMethod, FileEntry, ToolType, WorkContinueStatus};
use crate::common::progress_data::{CurrentStage, ProgressData};
use crate::common::progress_stop_handler::{check_if_stop_received, prepare_thread_handler_common};
use crate::common::tool_data::{CommonData, CommonToolData, DeleteMethod};
use crate::common::traits::*;

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
    pub number_of_groups: u64,
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


