pub mod core;
#[cfg(test)]
mod tests;
pub mod traits;

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
            Self::H265 => "hevc",
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
            _ => Err(format!("Unknown codec: {codec}")),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum OptimizerMode {
    VideoTranscode { codec: VideoCodec, quality: u32 },
}

#[derive(Debug, Default, Clone)]
pub struct Info {
    pub number_of_processed_files: usize,
    pub number_of_failed_files: usize,
    pub scanning_time: Duration,
}

#[derive(Clone)]
pub struct VideoOptimizerParameters {
    pub mode: OptimizerMode,
    pub excluded_codecs: Vec<String>,
}

impl Default for VideoOptimizerParameters {
    fn default() -> Self {
        Self {
            mode: OptimizerMode::VideoTranscode {
                codec: VideoCodec::H265,
                quality: 23,
            },
            excluded_codecs: vec!["hevc".to_string(), "av1".to_string()],
        }
    }
}

impl VideoOptimizerParameters {
    pub fn new(mode: OptimizerMode) -> Self {
        Self {
            mode,
            excluded_codecs: vec!["hevc".to_string(), "av1".to_string()],
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
}

pub enum VideoOptimizerEntry {
    VideoTranscode(VideoTranscodeEntry),
}

pub struct VideoOptimizer {
    common_data: CommonToolData,
    information: Info,
    video_transcode_entries: Vec<VideoTranscodeEntry>,
    params: VideoOptimizerParameters,
}

impl VideoOptimizer {
    pub const fn get_video_transcode_entries(&self) -> &Vec<VideoTranscodeEntry> {
        &self.video_transcode_entries
    }

    pub const fn get_params(&self) -> &VideoOptimizerParameters {
        &self.params
    }

    pub const fn get_information(&self) -> &Info {
        &self.information
    }
}
