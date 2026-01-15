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

    pub const fn as_ffprobe_codec_name(&self) -> &str {
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
            "h265" | "libx265" => Ok(Self::H265),
            "av1" | "libaom-av1" => Ok(Self::Av1),
            "vp9" | "libvpx-vp9" => Ok(Self::Vp9),
            _ => Err(format!("Unknown codec: {codec}")),
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
pub struct VideoCropFixParams {
    pub overwrite_original: bool,
    pub target_codec: Option<VideoCodec>,
    pub quality: Option<u32>,
    pub crop_rectangle: (u32, u32, u32, u32),
    pub crop_mechanism: VideoCroppingMechanism,
}

#[derive(Debug, Default, Clone)]
pub struct Info {
    pub number_of_processed_files: usize,
    pub number_of_failed_files: usize,
    pub scanning_time: Duration,
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum VideoOptimizerParameters {
    VideoTranscode(VideoTranscodeParams),
    VideoCrop(VideoCropParams),
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct VideoTranscodeParams {
    pub excluded_codecs: Vec<String>,
}
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct VideoCropParams {
    pub crop_detect: VideoCroppingMechanism,
}

impl VideoTranscodeParams {
    pub fn new() -> Self {
        Self {
            excluded_codecs: vec!["h265".to_string(), "av1".to_string(), "vp9".to_string()],
        }
    }
}
impl Default for VideoTranscodeParams {
    fn default() -> Self {
        Self::new()
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
    pub new_image_dimensions: Option<(u32, u32, u32, u32)>,
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
            new_image_dimensions: None,
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

    pub const fn get_information(&self) -> &Info {
        &self.information
    }
}
