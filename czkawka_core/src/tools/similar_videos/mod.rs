pub mod core;
pub mod traits;

#[cfg(test)]
mod tests;

use std::collections::BTreeMap;
use std::ops::RangeInclusive;
use std::path::{Path, PathBuf};
use std::time::Duration;

use rusty_chromaprint::Configuration;
use serde::{Deserialize, Serialize};
use similario_core::VideoSignature;

use crate::common::model::FileEntry;
use crate::common::tool_data::CommonToolData;
use crate::common::traits::ResultEntry;

pub const MAX_TOLERANCE: i32 = 20;

pub const DEFAULT_CROP_DETECT: bool = true;

pub const ALLOWED_SKIP_FORWARD_AMOUNT: RangeInclusive<u32> = 0..=300;
pub const DEFAULT_SKIP_FORWARD_AMOUNT: u32 = 15;

pub const ALLOWED_VID_HASH_DURATION: RangeInclusive<u32> = 2..=60;
pub const DEFAULT_VID_HASH_DURATION: u32 = 10;

pub const ALLOWED_WINDOW_COUNT: RangeInclusive<u32> = 1..=20;
pub const DEFAULT_WINDOW_COUNT: u32 = 5;

pub const ALLOWED_DURATION_TOLERANCE_PCT: RangeInclusive<f64> = 0.0..=100.0;
pub const DEFAULT_DURATION_TOLERANCE_PCT: f64 = 20.0;

pub const ALLOWED_MATCH_FRACTION: RangeInclusive<f64> = 0.0..=1.0;
pub const DEFAULT_MIN_MATCHING_WINDOWS: f64 = 0.6;
pub const DEFAULT_SUBCLIP_MIN_MATCH: f64 = 0.5;

pub const DEFAULT_VIDEO_PERCENTAGE_FOR_THUMBNAIL: u8 = 10;
pub const DEFAULT_THUMBNAIL_GRID_TILES_PER_SIDE: u8 = 2;

// Audio fingerprint mode constants
pub const ALLOWED_AUDIO_SIMILARITY_PERCENT: RangeInclusive<f64> = 0.0..=100.0;
pub const DEFAULT_AUDIO_SIMILARITY_PERCENT: f64 = 80.0;
pub const ALLOWED_AUDIO_LENGTH_RATIO: RangeInclusive<f64> = 0.0..=1.0;
pub const DEFAULT_AUDIO_LENGTH_RATIO: f64 = 0.1;
pub const DEFAULT_AUDIO_MIN_DURATION_SECONDS: u32 = 10;
pub const DEFAULT_AUDIO_MAXIMUM_DIFFERENCE: f64 = 3.0;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VideosEntry {
    pub path: PathBuf,
    pub size: u64,
    pub modified_date: u64,
    pub signature: Option<VideoSignature>,
    pub error: String,

    // Properties extracted from video
    pub fps: Option<f64>,
    pub codec: Option<String>,
    pub bitrate: Option<u64>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub duration: Option<f64>,

    #[serde(skip)] // Saving it to cache is bad idea, because cache can be moved to another locations
    pub thumbnail_path: Option<PathBuf>,
}

/// Minimal cache entry used to persist audio fingerprint data for video files.
/// Kept separate from `VideosEntry` so the visual-hash cache format is not affected.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VideoAudioEntry {
    pub path: PathBuf,
    pub size: u64,
    pub modified_date: u64,
    pub fingerprint: Vec<u32>,
    pub audio_duration_seconds: u32,
}

impl ResultEntry for VideoAudioEntry {
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

            signature: None,
            error: String::new(),
            fps: None,
            codec: None,
            bitrate: None,
            width: None,
            height: None,
            duration: None,
            thumbnail_path: None,
        }
    }

    pub(crate) fn into_video_audio_entry(self) -> VideoAudioEntry {
        VideoAudioEntry {
            size: self.size,
            path: self.path,
            modified_date: self.modified_date,
            fingerprint: Vec::new(),
            audio_duration_seconds: 0,
        }
    }
}

#[derive(Clone, Debug)]
pub struct SimilarVideosParameters {
    pub tolerance: i32,
    pub exclude_videos_with_same_size: bool,
    pub exclude_videos_with_same_resolution: bool,
    pub skip_forward_amount: u32,
    pub duration: u32,
    pub crop_detect: bool,
    pub window_count: u32,
    pub duration_tolerance_pct: f64,
    pub min_matching_windows: f64,
    pub subclip_min_match: f64,
    pub generate_thumbnails: bool,
    pub thumbnail_video_percentage_from_start: u8,
    pub generate_thumbnail_grid_instead_of_single: bool,
    pub thumbnail_grid_tiles_per_side: u8,

    pub check_audio_content: bool,
    pub audio_similarity_percent: f64,
    pub maximum_difference: f64,
    pub audio_length_ratio: f64,
    pub audio_min_duration_seconds: u32,
}

impl SimilarVideosParameters {
    pub fn new(
        tolerance: i32,
        exclude_videos_with_same_size: bool,
        exclude_videos_with_same_resolution: bool,
        skip_forward_amount: u32,
        duration: u32,
        crop_detect: bool,
        window_count: u32,
        duration_tolerance_pct: f64,
        min_matching_windows: f64,
        subclip_min_match: f64,
        generate_thumbnails: bool,
        thumbnail_video_percentage_from_start: u8,
        generate_thumbnail_grid_instead_of_single: bool,
        thumbnail_grid_tiles_per_side: u8,
        check_audio_content: bool,
        audio_similarity_percent: f64,
        maximum_difference: f64,
        audio_length_ratio: f64,
        audio_min_duration_seconds: u32,
    ) -> Self {
        assert!((0..=MAX_TOLERANCE).contains(&tolerance));
        assert!(ALLOWED_SKIP_FORWARD_AMOUNT.contains(&skip_forward_amount));
        assert!(ALLOWED_VID_HASH_DURATION.contains(&duration));
        assert!(ALLOWED_WINDOW_COUNT.contains(&window_count));
        assert!(ALLOWED_DURATION_TOLERANCE_PCT.contains(&duration_tolerance_pct));
        assert!(ALLOWED_MATCH_FRACTION.contains(&min_matching_windows));
        assert!(ALLOWED_MATCH_FRACTION.contains(&subclip_min_match));
        assert!(ALLOWED_AUDIO_SIMILARITY_PERCENT.contains(&audio_similarity_percent));
        assert!(ALLOWED_AUDIO_LENGTH_RATIO.contains(&audio_length_ratio));
        Self {
            tolerance,
            exclude_videos_with_same_size,
            exclude_videos_with_same_resolution,
            skip_forward_amount,
            duration,
            crop_detect,
            window_count,
            duration_tolerance_pct,
            min_matching_windows,
            subclip_min_match,
            generate_thumbnails,
            thumbnail_video_percentage_from_start,
            generate_thumbnail_grid_instead_of_single,
            thumbnail_grid_tiles_per_side,
            check_audio_content,
            audio_similarity_percent,
            maximum_difference,
            audio_length_ratio,
            audio_min_duration_seconds,
        }
    }
}

pub struct SimilarVideos {
    pub(crate) common_data: CommonToolData,
    pub(crate) information: Info,
    pub(crate) similar_vectors: Vec<Vec<VideosEntry>>,
    pub(crate) similar_referenced_vectors: Vec<(VideosEntry, Vec<VideosEntry>)>,
    pub(crate) videos_to_check: BTreeMap<String, VideosEntry>,
    /// Entries for the audio fingerprint pass, keyed by path string.
    pub(crate) audio_to_check: BTreeMap<String, VideoAudioEntry>,
    pub(crate) params: SimilarVideosParameters,
    pub(crate) audio_config: Configuration,
}

#[derive(Default, Clone, Copy)]
pub struct Info {
    pub number_of_duplicates: usize,
    pub number_of_groups: usize,
    pub scanning_time: Duration,
}

impl SimilarVideos {
    pub fn get_params(&self) -> &SimilarVideosParameters {
        &self.params
    }

    pub const fn get_similar_videos(&self) -> &Vec<Vec<VideosEntry>> {
        &self.similar_vectors
    }

    pub const fn get_information(&self) -> Info {
        self.information
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
