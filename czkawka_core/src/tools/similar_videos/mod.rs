mod core;
mod traits;

use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::io::Write;
use std::mem;
use std::ops::RangeInclusive;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use crossbeam_channel::Sender;
use fun_time::fun_time;
use humansize::{BINARY, format_size};
use log::debug;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use vid_dup_finder_lib::{CreationOptions, Cropdetect, VideoHash, VideoHashBuilder};

use crate::common::cache::{extract_loaded_cache,  load_cache_from_file_generalized_by_path, save_cache_to_file_generalized};
use crate::common::consts::VIDEO_FILES_EXTENSIONS;
use crate::common::dir_traversal::{DirTraversalBuilder, DirTraversalResult, inode, take_1_per_inode};
use crate::common::model::{FileEntry, ToolType, WorkContinueStatus};
use crate::common::progress_data::{CurrentStage, ProgressData};
use crate::common::progress_stop_handler::{check_if_stop_received, prepare_thread_handler_common};
use crate::common::tool_data::{CommonData, CommonToolData, DeleteMethod};
use crate::common::traits::{DebugPrint, DeletingItems, PrintResults, ResultEntry};
use crate::flc;

pub const MAX_TOLERANCE: i32 = 20;

pub const DEFAULT_CROP_DETECT: Cropdetect = Cropdetect::Letterbox;

pub const ALLOWED_SKIP_FORWARD_AMOUNT: RangeInclusive<u32> = 0..=300;
pub const DEFAULT_SKIP_FORWARD_AMOUNT: u32 = 15;

pub const ALLOWED_VID_HASH_DURATION: RangeInclusive<u32> = 2..=60;
pub const DEFAULT_VID_HASH_DURATION: u32 = 10;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VideosEntry {
    pub path: PathBuf,
    pub size: u64,
    pub modified_date: u64,
    pub vhash: VideoHash,
    pub error: String,
}

impl ResultEntry for VideosEntry {
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
    fn into_videos_entry(self) -> VideosEntry {
        VideosEntry {
            size: self.size,
            path: self.path,
            modified_date: self.modified_date,

            vhash: Default::default(),
            error: String::new(),
        }
    }
}

#[derive(Clone)]
pub struct SimilarVideosParameters {
    pub tolerance: i32,
    pub exclude_videos_with_same_size: bool,
    pub ignore_hard_links: bool,
    pub skip_forward_amount: u32,
    pub duration: u32,
    pub crop_detect: Cropdetect,
}

pub fn crop_detect_from_str_opt(s: &str) -> Option<Cropdetect> {
    match s.to_lowercase().as_str() {
        "none" => Some(Cropdetect::None),
        "letterbox" => Some(Cropdetect::Letterbox),
        "motion" => Some(Cropdetect::Motion),
        _ => None,
    }
}

pub fn crop_detect_from_str(s: &str) -> Cropdetect {
    crop_detect_from_str_opt(s).unwrap_or(DEFAULT_CROP_DETECT)
}
pub fn crop_detect_to_str(crop_detect: Cropdetect) -> String {
    match crop_detect {
        Cropdetect::None => "none".to_string(),
        Cropdetect::Letterbox => "letterbox".to_string(),
        Cropdetect::Motion => "motion".to_string(),
    }
}

impl SimilarVideosParameters {
    pub fn new(tolerance: i32, exclude_videos_with_same_size: bool, ignore_hard_links: bool, skip_forward_amount: u32, duration: u32, crop_detect: Cropdetect) -> Self {
        assert!((0..=MAX_TOLERANCE).contains(&tolerance));
        assert!(ALLOWED_SKIP_FORWARD_AMOUNT.contains(&skip_forward_amount));
        assert!(ALLOWED_VID_HASH_DURATION.contains(&duration));
        Self {
            tolerance,
            exclude_videos_with_same_size,
            ignore_hard_links,
            skip_forward_amount,
            duration,
            crop_detect,
        }
    }
}

pub struct SimilarVideos {
    common_data: CommonToolData,
    information: Info,
    similar_vectors: Vec<Vec<VideosEntry>>,
    similar_referenced_vectors: Vec<(VideosEntry, Vec<VideosEntry>)>,
    videos_hashes: BTreeMap<Vec<u8>, Vec<VideosEntry>>,
    videos_to_check: BTreeMap<String, VideosEntry>,
    params: SimilarVideosParameters,
}


#[derive(Default, Clone)]
pub struct Info {
    pub number_of_duplicates: usize,
    pub number_of_groups: u64,
}



impl SimilarVideos {
    pub fn get_params(&self) -> &SimilarVideosParameters {
        &self.params
    }

    pub const fn get_similar_videos(&self) -> &Vec<Vec<VideosEntry>> {
        &self.similar_vectors
    }

    pub const fn get_information(&self) -> &Info {
        &self.information
    }

    pub fn get_similar_videos_referenced(&self) -> &Vec<(VideosEntry, Vec<VideosEntry>)> {
        &self.similar_referenced_vectors
    }

    pub fn get_number_of_base_duplicated_files(&self) -> usize {
        if self.common_data.use_reference_folders {
            self.similar_referenced_vectors.len()
        } else {
            self.similar_vectors.len()
        }
    }

    pub fn get_use_reference(&self) -> bool {
        self.common_data.use_reference_folders
    }
}
