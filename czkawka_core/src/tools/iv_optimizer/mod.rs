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

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum OptimizerMode {
    VideoTranscode { codec: VideoCodec, quality: u32 },
    ImageTrim { threshold: u8 },
}

#[derive(Debug, Default, Clone)]
pub struct Info {
    pub number_of_processed_files: usize,
    pub number_of_failed_files: usize,
    pub scanning_time: Duration,
}

#[derive(Clone)]
pub struct IVOptimizerParameters {
    pub mode: OptimizerMode,
    pub excluded_codecs: Vec<String>,
}

impl Default for IVOptimizerParameters {
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

impl IVOptimizerParameters {
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
    pub dimensions: Option<String>,
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
            dimensions: None,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ImageTrimEntry {
    pub path: PathBuf,
    pub size: u64,
    pub modified_date: u64,
    pub error: Option<String>,

    pub bounding_box: Option<BoundingBox>,
    pub new_size: Option<u64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BoundingBox {
    pub top: u32,
    pub bottom: u32,
    pub left: u32,
    pub right: u32,
}

impl ResultEntry for ImageTrimEntry {
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
    fn into_image_trim_entry(self) -> ImageTrimEntry {
        ImageTrimEntry {
            size: self.size,
            path: self.path,
            modified_date: self.modified_date,
            error: None,
            bounding_box: None,
            new_size: None,
        }
    }
}

pub enum IVOptimizerEntry {
    VideoTranscode(VideoTranscodeEntry),
    ImageTrim(ImageTrimEntry),
}

pub struct IVOptimizer {
    common_data: CommonToolData,
    information: Info,
    video_transcode_entries: Vec<VideoTranscodeEntry>,
    image_trim_entries: Vec<ImageTrimEntry>,
    params: IVOptimizerParameters,
}

impl IVOptimizer {
    pub const fn get_video_transcode_entries(&self) -> &Vec<VideoTranscodeEntry> {
        &self.video_transcode_entries
    }

    pub const fn get_image_trim_entries(&self) -> &Vec<ImageTrimEntry> {
        &self.image_trim_entries
    }

    pub const fn get_params(&self) -> &IVOptimizerParameters {
        &self.params
    }

    pub const fn get_information(&self) -> &Info {
        &self.information
    }
}
