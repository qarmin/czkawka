pub mod core;
pub mod traits;

#[cfg(test)]
mod tests;

use std::collections::BTreeMap;
use std::ops::RangeInclusive;
use std::path::{Path, PathBuf};
use std::time::Duration;

use serde::{Deserialize, Serialize};
use vid_dup_finder_lib::{Cropdetect, VideoHash};

use crate::common::cache::{CACHE_AUDIO_VIDEO_VERSION, CACHE_PERCEPTUAL_VIDEO_VERSION};
use crate::common::model::FileEntry;
use crate::common::tool_data::CommonToolData;
use crate::common::traits::ResultEntry;

pub const MAX_TOLERANCE: i32 = 20;

pub const DEFAULT_CROP_DETECT: Cropdetect = Cropdetect::Letterbox;

pub const ALLOWED_SKIP_FORWARD_AMOUNT: RangeInclusive<u32> = 0..=300;
pub const DEFAULT_SKIP_FORWARD_AMOUNT: u32 = 15;

pub const ALLOWED_VID_HASH_DURATION: RangeInclusive<u32> = 2..=60;
pub const DEFAULT_VID_HASH_DURATION: u32 = 10;

pub const DEFAULT_VIDEO_PERCENTAGE_FOR_THUMBNAIL: u8 = 10;

// ─────────────────────────────────────────────
// New engine / preset types
// ─────────────────────────────────────────────

/// Speed/quality presets for the Perceptual engine.
///
/// | Preset   | frames | min_matched | max_dur_ratio | skip_start | skip_threshold |
/// |----------|--------|-------------|---------------|------------|----------------|
/// | Fastest  |   30   |     10      |    1.05       |    0 s     |    600 s       |
/// | Fast     |  100   |     20      |    1.10       |    0 s     |    600 s       |
/// | Balanced |  300   |     30      |   off (0.0)   |    0 s     |    600 s       |
/// | Thorough |  600   |     30      |   off (0.0)   |  120 s     |    600 s       |
/// | Maximum  | 1200   |     30      |   off (0.0)   |  120 s     |    300 s       |
#[derive(Clone, Copy, Debug, Serialize, Deserialize, Default, PartialEq, Eq)]
pub enum PerceptualSearchPreset {
    Fastest,
    Fast,
    #[default]
    Balanced,
    Thorough,
    Maximum,
}

impl PerceptualSearchPreset {
    /// Returns `(HashConfig, min_matched_frames, max_duration_ratio)`.
    pub fn to_hash_config_and_compare_params(self) -> (similarrio_videoo::HashConfig, usize, f64) {
        use similarrio_videoo::{HashConfig, VideoHashAlg};
        match self {
            Self::Fastest => (
                HashConfig {
                    samples_per_second: Some(1.0),
                    max_frames: 30,
                    skip_start_secs: 0.0,
                    skip_threshold_secs: 600.0,
                    hash_alg: VideoHashAlg::Mean,
                    dct: true,
                    ..HashConfig::default()
                },
                10,
                1.05,
            ),
            Self::Fast => (
                HashConfig {
                    samples_per_second: Some(1.0),
                    max_frames: 100,
                    skip_start_secs: 0.0,
                    skip_threshold_secs: 600.0,
                    hash_alg: VideoHashAlg::Mean,
                    dct: true,
                    ..HashConfig::default()
                },
                20,
                1.10,
            ),
            Self::Balanced => (
                HashConfig {
                    samples_per_second: Some(1.0),
                    max_frames: 300,
                    skip_start_secs: 0.0,
                    skip_threshold_secs: 600.0,
                    hash_alg: VideoHashAlg::Mean,
                    dct: true,
                    ..HashConfig::default()
                },
                30,
                0.0,
            ),
            Self::Thorough => (
                HashConfig {
                    samples_per_second: Some(1.0),
                    max_frames: 600,
                    skip_start_secs: 120.0,
                    skip_threshold_secs: 600.0,
                    hash_alg: VideoHashAlg::Mean,
                    dct: true,
                    ..HashConfig::default()
                },
                30,
                0.0,
            ),
            Self::Maximum => (
                HashConfig {
                    samples_per_second: Some(1.0),
                    max_frames: 1200,
                    skip_start_secs: 120.0,
                    skip_threshold_secs: 300.0,
                    hash_alg: VideoHashAlg::Mean,
                    dct: true,
                    ..HashConfig::default()
                },
                30,
                0.0,
            ),
        }
    }

    pub fn from_str_opt(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "fastest" => Some(Self::Fastest),
            "fast" => Some(Self::Fast),
            "balanced" => Some(Self::Balanced),
            "thorough" => Some(Self::Thorough),
            "maximum" => Some(Self::Maximum),
            _ => None,
        }
    }
}

/// Speed/quality presets for the Audio (Chromaprint) engine.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, Default, PartialEq, Eq)]
pub enum AudioSearchPreset {
    /// Full audio, no limit, no intro skip.
    #[default]
    Full,
    /// Limit extracted audio to first 120 seconds (fast for large collections).
    Fast2Min,
    /// Skip first 120 seconds of audio for videos longer than 600 seconds.
    SkipIntros,
}

impl AudioSearchPreset {
    pub fn to_audio_config(self) -> similarrio_videoo::AudioConfig {
        use similarrio_videoo::AudioConfig;
        match self {
            Self::Full => AudioConfig {
                skip_start_secs: 0.0,
                skip_threshold_secs: 600.0,
                max_audio_secs: 0.0,
            },
            Self::Fast2Min => AudioConfig {
                skip_start_secs: 0.0,
                skip_threshold_secs: 600.0,
                max_audio_secs: 120.0,
            },
            Self::SkipIntros => AudioConfig {
                skip_start_secs: 120.0,
                skip_threshold_secs: 600.0,
                max_audio_secs: 0.0,
            },
        }
    }

    pub fn from_str_opt(s: &str) -> Option<Self> {
        match s.to_lowercase().replace('-', "_").as_str() {
            "full" => Some(Self::Full),
            "fast_2min" | "fast2min" => Some(Self::Fast2Min),
            "skip_intros" | "skipintros" => Some(Self::SkipIntros),
            _ => None,
        }
    }
}

/// Which algorithm to use for finding similar videos.
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub enum SimilarVideosEngine {
    /// Original vid_dup_finder_lib based engine (default, unchanged behaviour).
    #[default]
    VidDupFinder,
    /// Perceptual hashing via similarrio_videoo (sliding-window pHash).
    Perceptual(PerceptualSearchPreset),
    /// Audio fingerprinting via Chromaprint (similarrio_videoo audio feature).
    Audio(AudioSearchPreset),
}

impl SimilarVideosEngine {
    pub fn from_str_opt(engine: &str, preset: &str, audio_preset: &str) -> Option<Self> {
        match engine.to_lowercase().replace('-', "_").as_str() {
            "vid_dup_finder" | "vid-dup-finder" | "viddup" => Some(Self::VidDupFinder),
            "perceptual" => Some(Self::Perceptual(PerceptualSearchPreset::from_str_opt(preset).unwrap_or_default())),
            "audio" => Some(Self::Audio(AudioSearchPreset::from_str_opt(audio_preset).unwrap_or_default())),
            _ => None,
        }
    }
}

// ─────────────────────────────────────────────
// Cache entry types for the new engines
// ─────────────────────────────────────────────

/// Cache entry used by the Perceptual engine.
///
/// Stores path/size/modified_date (for invalidation) alongside the
/// `VideoFingerprint` from `similarrio_videoo`.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PerceptualCacheEntry {
    pub path: PathBuf,
    pub size: u64,
    pub modified_date: u64,
    pub fingerprint: similarrio_videoo::VideoFingerprint,
}

impl ResultEntry for PerceptualCacheEntry {
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

/// Cache entry used by the Audio engine.
///
/// Stores path/size/modified_date alongside the Chromaprint `AudioFingerprint`.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AudioCacheEntry {
    pub path: PathBuf,
    pub size: u64,
    pub modified_date: u64,
    pub fingerprint: similarrio_videoo::AudioFingerprint,
}

impl ResultEntry for AudioCacheEntry {
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

// ─────────────────────────────────────────────
// Cache file name helpers
// ─────────────────────────────────────────────

pub fn get_perceptual_cache_file(preset: PerceptualSearchPreset) -> String {
    let preset_str = match preset {
        PerceptualSearchPreset::Fastest => "fastest",
        PerceptualSearchPreset::Fast => "fast",
        PerceptualSearchPreset::Balanced => "balanced",
        PerceptualSearchPreset::Thorough => "thorough",
        PerceptualSearchPreset::Maximum => "maximum",
    };
    format!("cache_similar_videos_perceptual_{CACHE_PERCEPTUAL_VIDEO_VERSION}__preset_{preset_str}.bin")
}

pub fn get_audio_cache_file(preset: AudioSearchPreset) -> String {
    let preset_str = match preset {
        AudioSearchPreset::Full => "full",
        AudioSearchPreset::Fast2Min => "fast2min",
        AudioSearchPreset::SkipIntros => "skipintros",
    };
    format!("cache_similar_videos_audio_{CACHE_AUDIO_VIDEO_VERSION}__preset_{preset_str}.bin")
}

// ─────────────────────────────────────────────
// Original types (unchanged)
// ─────────────────────────────────────────────

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VideosEntry {
    pub path: PathBuf,
    pub size: u64,
    pub modified_date: u64,
    pub vhash: VideoHash,
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
            fps: None,
            codec: None,
            bitrate: None,
            width: None,
            height: None,
            duration: None,
            thumbnail_path: None,
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
    pub crop_detect: Cropdetect,
    pub generate_thumbnails: bool,
    pub thumbnail_video_percentage_from_start: u8,
    pub generate_thumbnail_grid_instead_of_single: bool,
    pub thumbnail_grid_tiles_per_side: u8,
    /// Which algorithm to use for similarity detection.
    pub engine: SimilarVideosEngine,
}

pub fn crop_detect_from_str_opt(s: &str) -> Option<Cropdetect> {
    match s.to_lowercase().as_str() {
        "none" => Some(Cropdetect::None),
        "letterbox" => Some(Cropdetect::Letterbox),
        "motion" => Some(Cropdetect::Motion),
        _ => None,
    }
}

impl SimilarVideosParameters {
    #[expect(clippy::too_many_arguments)]
    pub fn new(
        tolerance: i32,
        exclude_videos_with_same_size: bool,
        exclude_videos_with_same_resolution: bool,
        skip_forward_amount: u32,
        duration: u32,
        crop_detect: Cropdetect,
        generate_thumbnails: bool,
        thumbnail_video_percentage_from_start: u8,
        generate_thumbnail_grid_instead_of_single: bool,
        thumbnail_grid_tiles_per_side: u8,
        engine: SimilarVideosEngine,
    ) -> Self {
        assert!((0..=MAX_TOLERANCE).contains(&tolerance));
        assert!(ALLOWED_SKIP_FORWARD_AMOUNT.contains(&skip_forward_amount));
        assert!(ALLOWED_VID_HASH_DURATION.contains(&duration));
        Self {
            tolerance,
            exclude_videos_with_same_size,
            exclude_videos_with_same_resolution,
            skip_forward_amount,
            duration,
            crop_detect,
            generate_thumbnails,
            thumbnail_video_percentage_from_start,
            generate_thumbnail_grid_instead_of_single,
            thumbnail_grid_tiles_per_side,
            engine,
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
