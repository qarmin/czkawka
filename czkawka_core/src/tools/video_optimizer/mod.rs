pub mod core;
#[cfg(test)]
mod tests;
pub mod traits;

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::common::model::FileEntry;
use crate::common::tool_data::CommonToolData;
use crate::common::traits::ResultEntry;
use crate::flc;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum VideoCodec {
    H264,
    H265,
    Av1,
    Vp9,
}

impl VideoCodec {
    pub const fn as_str(&self) -> &str {
        match self {
            Self::H264 => "libx264",
            Self::H265 => "libx265",
            Self::Av1 => "libaom-av1",
            Self::Vp9 => "libvpx-vp9",
        }
    }

    pub const fn as_ffprobe_codec_name(self) -> &'static str {
        match self {
            Self::H264 => "h264",
            Self::H265 => "h265",
            Self::Av1 => "av1",
            Self::Vp9 => "vp9",
        }
    }
}

impl std::str::FromStr for VideoCodec {
    type Err = String;

    fn from_str(codec: &str) -> Result<Self, Self::Err> {
        match codec.to_lowercase().as_str() {
            "h264" | "libx264" => Ok(Self::H264),
            "h265" | "hevc" | "libx265" => Ok(Self::H265),
            "av1" | "libaom-av1" => Ok(Self::Av1),
            "vp9" | "libvpx-vp9" => Ok(Self::Vp9),
            _ => Err(flc!("core_unknown_codec", codec = codec)),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum VideoCroppingMechanism {
    BlackBars,
    StaticContent,
}
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum VideoOptimizerMode {
    VideoTranscode,
    VideoCrop,
}

impl std::str::FromStr for VideoOptimizerMode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "transcode" | "videotranscode" => Ok(Self::VideoTranscode),
            "crop" | "videocrop" => Ok(Self::VideoCrop),
            _ => Err(flc!("core_invalid_video_optimizer_mode", mode = s)),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum VideoOptimizerFixParams {
    VideoTranscode(VideoTranscodeFixParams),
    VideoCrop(VideoCropFixParams),
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct VideoTranscodeFixParams {
    pub codec: VideoCodec,
    pub quality: u32,
    pub fail_if_not_smaller: bool,
    pub overwrite_original: bool,
    pub limit_video_size: bool,
    pub max_width: u32,
    pub max_height: u32,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct VideoCropSingleFixParams {
    pub overwrite_original: bool,
    pub target_codec: Option<VideoCodec>,
    pub quality: Option<u32>,
    pub crop_rectangle: (u32, u32, u32, u32),
    pub crop_mechanism: VideoCroppingMechanism,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct VideoCropFixParams {
    pub overwrite_original: bool,
    pub target_codec: Option<VideoCodec>,
    pub quality: Option<u32>,
    pub crop_mechanism: VideoCroppingMechanism,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Info {
    pub scanning_time: Duration,
    pub number_of_videos_to_transcode: usize,
    pub number_of_videos_to_crop: usize,
}

#[derive(Clone, PartialEq, Debug)]
pub enum VideoOptimizerParameters {
    VideoTranscode(VideoTranscodeParams),
    VideoCrop(VideoCropParams),
}

impl VideoOptimizerParameters {
    pub fn get_generate_number_of_items_in_thumbnail_grid(&self) -> u8 {
        let (generate_thumbnail_grid_instead_of_single, thumbnail_grid_tiles_per_side) = match self {
            Self::VideoTranscode(params) => (params.generate_thumbnail_grid_instead_of_single, params.thumbnail_grid_tiles_per_side),
            Self::VideoCrop(params) => (params.generate_thumbnail_grid_instead_of_single, params.thumbnail_grid_tiles_per_side),
        };

        if generate_thumbnail_grid_instead_of_single { thumbnail_grid_tiles_per_side } else { 1 }
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct VideoTranscodeParams {
    pub(crate) excluded_codecs: Vec<String>,
    pub(crate) generate_thumbnails: bool,
    pub(crate) thumbnail_video_percentage_from_start: u8,
    pub(crate) generate_thumbnail_grid_instead_of_single: bool,
    pub(crate) thumbnail_grid_tiles_per_side: u8,
}
#[derive(Clone, PartialEq, Debug)]
pub struct VideoCropParams {
    pub(crate) crop_detect: VideoCroppingMechanism,
    pub(crate) black_pixel_threshold: u8,
    pub(crate) black_bar_min_percentage: u8,
    pub(crate) max_samples: usize,
    pub(crate) min_crop_size: u32,
    pub(crate) generate_thumbnails: bool,
    pub(crate) thumbnail_video_percentage_from_start: u8,
    pub(crate) generate_thumbnail_grid_instead_of_single: bool,
    pub(crate) thumbnail_grid_tiles_per_side: u8,
}

impl VideoTranscodeParams {
    pub fn new(
        excluded_codecs: Vec<String>,
        generate_thumbnails: bool,
        thumbnail_video_percentage_from_start: u8,
        generate_thumbnail_grid_instead_of_single: bool,
        thumbnail_grid_tiles_per_side: u8,
    ) -> Self {
        Self {
            excluded_codecs,
            generate_thumbnails,
            thumbnail_video_percentage_from_start,
            generate_thumbnail_grid_instead_of_single,
            thumbnail_grid_tiles_per_side,
        }
    }
}
impl Default for VideoTranscodeParams {
    fn default() -> Self {
        Self {
            excluded_codecs: vec!["hevc".to_string(), "h265".to_string(), "av1".to_string(), "vp9".to_string()],
            generate_thumbnails: false,
            thumbnail_video_percentage_from_start: 10,
            generate_thumbnail_grid_instead_of_single: false,
            thumbnail_grid_tiles_per_side: 2,
        }
    }
}

impl VideoCropParams {
    pub fn with_custom_params(
        crop_detect: VideoCroppingMechanism,
        black_pixel_threshold: u8,
        black_bar_min_percentage: u8,
        max_samples: usize,
        min_crop_size: u32,
        generate_thumbnails: bool,
        thumbnail_video_percentage_from_start: u8,
        generate_thumbnail_grid_instead_of_single: bool,
        thumbnail_grid_tiles_per_side: u8,
    ) -> Self {
        assert!(black_pixel_threshold <= 128, "black_pixel_threshold must be 0-128, got {black_pixel_threshold}");
        assert!(
            (50..=100).contains(&black_bar_min_percentage),
            "black_bar_min_percentage must be 50-100, got {black_bar_min_percentage}"
        );
        assert!((5..=1000).contains(&max_samples), "max_samples must be 5-1000, got {max_samples}");
        assert!((1..=1000).contains(&min_crop_size), "min_crop_size must be 1-1000, got {min_crop_size}");

        Self {
            crop_detect,
            black_pixel_threshold,
            black_bar_min_percentage,
            max_samples,
            min_crop_size,
            generate_thumbnails,
            thumbnail_video_percentage_from_start,
            generate_thumbnail_grid_instead_of_single,
            thumbnail_grid_tiles_per_side,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VideoTranscodeEntry {
    pub path: PathBuf,
    pub size: u64,
    pub modified_date: u64,
    pub error: Option<String>,

    pub codec: String,
    pub width: u32,
    pub height: u32,
    pub duration: f64,

    #[serde(skip)] // Saving it to cache is bad idea, because cache can be moved to another locations
    pub thumbnail_path: Option<PathBuf>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VideoCropEntry {
    pub path: PathBuf,
    pub size: u64,
    pub modified_date: u64,
    pub error: Option<String>,

    pub codec: String,
    pub width: u32,
    pub height: u32,
    pub new_image_dimensions: (u32, u32, u32, u32),
    pub duration: f64,

    #[serde(skip)] // Saving it to cache is bad idea, because cache can be moved to another locations
    pub thumbnail_path: Option<PathBuf>,
}

impl ResultEntry for VideoTranscodeEntry {
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

impl ResultEntry for VideoCropEntry {
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
    fn into_video_transcode_entry(self) -> VideoTranscodeEntry {
        VideoTranscodeEntry {
            size: self.size,
            path: self.path,
            modified_date: self.modified_date,
            error: None,
            codec: String::new(),
            width: 0,
            height: 0,
            duration: 0.0,
            thumbnail_path: None,
        }
    }

    fn into_video_crop_entry(self) -> VideoCropEntry {
        VideoCropEntry {
            size: self.size,
            path: self.path,
            modified_date: self.modified_date,
            error: None,
            codec: String::new(),
            width: 0,
            height: 0,
            new_image_dimensions: (0, 0, 0, 0),
            duration: 0.0,
            thumbnail_path: None,
        }
    }
}

pub enum VideoOptimizerEntry {
    VideoTranscode(VideoTranscodeEntry),
    VideoCrop(VideoCropEntry),
}

pub struct VideoOptimizer {
    common_data: CommonToolData,
    information: Info,
    video_transcode_test_entries: BTreeMap<String, VideoTranscodeEntry>,
    video_crop_test_entries: BTreeMap<String, VideoCropEntry>,
    video_transcode_result_entries: Vec<VideoTranscodeEntry>,
    video_crop_result_entries: Vec<VideoCropEntry>,
    params: VideoOptimizerParameters,
}

impl VideoOptimizer {
    pub const fn get_video_transcode_entries(&self) -> &Vec<VideoTranscodeEntry> {
        &self.video_transcode_result_entries
    }

    pub const fn get_video_crop_entries(&self) -> &Vec<VideoCropEntry> {
        &self.video_crop_result_entries
    }

    pub const fn get_params(&self) -> &VideoOptimizerParameters {
        &self.params
    }

    pub const fn get_information(&self) -> Info {
        self.information
    }
}
