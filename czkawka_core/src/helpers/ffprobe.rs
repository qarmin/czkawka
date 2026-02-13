//! Simple wrapper for the [ffprobe](https://ffmpeg.org/ffprobe.html) CLI utility,
//! which is part of the ffmpeg tool suite.
//!
//! This crate allows retrieving typed information about media files (images and videos)
//! by invoking `ffprobe` with JSON output options and deserializing the data
//! into convenient Rust types.
//!
//!
//!
//! ```rust, no_run
//! use czkawka_core::helpers::ffprobe::ffprobe;
//! match ffprobe("path/to/video.mp4") {
//!    Ok(info) => {
//!        dbg!(info);
//!    },
//!    Err(err) => {
//!        eprintln!("Could not analyze file with ffprobe: {:?}", err);
//!     },
//! }
//! ```
//!
//! CODE IS COPIED FROM https://github.com/theduke/ffprobe-rs
//! I WILL BE ABLE TO AGAIN USE IT AFTER A NEW VERSION IS RELEASED
//! https://github.com/theduke/ffprobe-rs/issues/33
//! LICENSE: MIT

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

/// Execute ffprobe with default settings and return the extracted data.
///
/// See [`ffprobe_config`] if you need to customize settings.
pub fn ffprobe(path: impl AsRef<std::path::Path>) -> Result<FfProbe, FfProbeError> {
    ffprobe_config(
        Config {
            count_frames: false,
            ffprobe_bin: "ffprobe".into(),
        },
        path,
    )
}

/// Run ffprobe with a custom config.
/// See [`ConfigBuilder`] for more details.
pub fn ffprobe_config(config: Config, path: impl AsRef<std::path::Path>) -> Result<FfProbe, FfProbeError> {
    let path = path.as_ref();

    let mut cmd = std::process::Command::new(config.ffprobe_bin);

    // Default args.
    cmd.args(["-v", "error", "-show_format", "-show_streams", "-print_format", "json"]);

    if config.count_frames {
        cmd.arg("-count_frames");
    }

    cmd.arg(path);

    // Prevent CMD popup on Windows.
    #[cfg(target_os = "windows")]
    cmd.creation_flags(0x08000000);

    let out = cmd.output().map_err(FfProbeError::Io)?;

    if !out.status.success() {
        return Err(FfProbeError::Status(out));
    }

    serde_json::from_slice::<FfProbe>(&out.stdout).map_err(FfProbeError::Deserialize)
}

/// ffprobe configuration.
///
/// Use [`Config::builder`] for constructing a new config.
#[derive(Clone, Debug)]
pub struct Config {
    count_frames: bool,
    ffprobe_bin: std::path::PathBuf,
}

impl Config {
    /// Construct a new ConfigBuilder.
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::new()
    }
}

/// Build the ffprobe configuration.
pub struct ConfigBuilder {
    config: Config,
}

impl ConfigBuilder {
    pub fn new() -> Self {
        Self {
            config: Config {
                count_frames: false,
                ffprobe_bin: "ffprobe".into(),
            },
        }
    }

    /// Enable the -count_frames setting.
    /// Will fully decode the file and count the frames.
    /// Frame count will be available in [`Stream::nb_read_frames`].
    pub fn count_frames(mut self, count_frames: bool) -> Self {
        self.config.count_frames = count_frames;
        self
    }

    /// Specify which binary name (e.g. `"ffprobe-6"`) or path (e.g. `"/opt/bin/ffprobe"`) to use
    /// for executing `ffprobe`.
    pub fn ffprobe_bin(mut self, ffprobe_bin: impl AsRef<std::path::Path>) -> Self {
        self.config.ffprobe_bin = ffprobe_bin.as_ref().to_path_buf();
        self
    }

    /// Finalize the builder into a [`Config`].
    pub fn build(self) -> Config {
        self.config
    }

    /// Run ffprobe with the config produced by this builder.
    pub fn run(self, path: impl AsRef<std::path::Path>) -> Result<FfProbe, FfProbeError> {
        ffprobe_config(self.config, path)
    }
}

impl Default for ConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
#[non_exhaustive]
pub enum FfProbeError {
    Io(std::io::Error),
    Status(std::process::Output),
    Deserialize(serde_json::Error),
}

impl std::fmt::Display for FfProbeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(e) => e.fmt(f),
            Self::Status(o) => {
                write!(f, "ffprobe exited with status code {}: {}", o.status, String::from_utf8_lossy(&o.stderr))
            }
            Self::Deserialize(e) => e.fmt(f),
        }
    }
}

impl std::error::Error for FfProbeError {}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]

pub struct FfProbe {
    pub streams: Vec<Stream>,
    pub format: Format,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]

pub struct Stream {
    pub index: i64,
    pub codec_name: Option<String>,
    pub sample_aspect_ratio: Option<String>,
    pub display_aspect_ratio: Option<String>,
    pub color_range: Option<String>,
    pub color_space: Option<String>,
    pub bits_per_raw_sample: Option<String>,
    pub channel_layout: Option<String>,
    pub max_bit_rate: Option<String>,
    pub nb_frames: Option<String>,
    /// Number of frames seen by the decoder.
    /// Requires full decoding and is only available if the 'count_frames'
    /// setting was enabled.
    pub nb_read_frames: Option<String>,
    pub codec_long_name: Option<String>,
    pub codec_type: Option<String>,
    pub codec_time_base: Option<String>,
    pub codec_tag_string: String,
    pub codec_tag: String,
    pub sample_fmt: Option<String>,
    pub sample_rate: Option<String>,
    pub channels: Option<i64>,
    pub bits_per_sample: Option<i64>,
    pub r_frame_rate: String,
    pub avg_frame_rate: String,
    pub time_base: String,
    pub start_pts: Option<i64>,
    pub start_time: Option<String>,
    pub duration_ts: Option<i64>,
    pub duration: Option<String>,
    pub bit_rate: Option<String>,
    pub disposition: Disposition,
    pub tags: Option<StreamTags>,
    pub profile: Option<String>,
    pub width: Option<i64>,
    pub height: Option<i64>,
    pub coded_width: Option<i64>,
    pub coded_height: Option<i64>,
    pub closed_captions: Option<i64>,
    pub has_b_frames: Option<i64>,
    pub pix_fmt: Option<String>,
    pub level: Option<i64>,
    pub chroma_location: Option<String>,
    pub refs: Option<i64>,
    pub is_avc: Option<String>,
    pub nal_length: Option<String>,
    pub nal_length_size: Option<String>,
    pub field_order: Option<String>,
    pub id: Option<String>,
    #[serde(default)]
    pub side_data_list: Vec<SideData>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
// Allowed to prevent having to break compatibility of float fields are added.
#[expect(clippy::derive_partial_eq_without_eq)]
pub struct SideData {
    pub side_data_type: String,
    pub rotation: Option<i16>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
// Allowed to prevent having to break compatibility of float fields are added.
#[expect(clippy::derive_partial_eq_without_eq)]
pub struct Disposition {
    pub default: i64,
    pub dub: i64,
    pub original: i64,
    pub comment: i64,
    pub lyrics: i64,
    pub karaoke: i64,
    pub forced: i64,
    pub hearing_impaired: i64,
    pub visual_impaired: i64,
    pub clean_effects: i64,
    pub attached_pic: i64,
    pub timed_thumbnails: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
// Allowed to prevent having to break compatibility of float fields are added.
#[expect(clippy::derive_partial_eq_without_eq)]
pub struct StreamTags {
    pub language: Option<String>,
    pub creation_time: Option<String>,
    pub handler_name: Option<String>,
    pub encoder: Option<String>,
    pub timecode: Option<String>,
    pub reel_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]

pub struct Format {
    pub filename: String,
    pub nb_streams: i64,
    pub nb_programs: i64,
    pub format_name: String,
    pub format_long_name: Option<String>,
    pub start_time: Option<String>,
    pub duration: Option<String>,
    pub size: Option<String>,
    pub bit_rate: Option<String>,
    pub probe_score: i64,
    pub tags: Option<FormatTags>,
}

impl Format {
    /// Get the duration parsed into a [`std::time::Duration`].
    pub fn try_get_duration(&self) -> Option<Result<std::time::Duration, std::num::ParseFloatError>> {
        self.duration.as_ref().map(|duration| match duration.parse::<f64>() {
            Ok(num) => Ok(std::time::Duration::from_secs_f64(num)),
            Err(error) => Err(error),
        })
    }

    /// Get the duration parsed into a [`std::time::Duration`].
    ///
    /// Will return [`None`] if no duration is available, or if parsing fails.
    /// See [`Self::try_get_duration`] for a method that returns an error.
    pub fn get_duration(&self) -> Option<std::time::Duration> {
        self.try_get_duration()?.ok()
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FormatTags {
    #[serde(rename = "WMFSDKNeeded")]
    pub wmfsdkneeded: Option<String>,
    #[serde(rename = "DeviceConformanceTemplate")]
    pub device_conformance_template: Option<String>,
    #[serde(rename = "WMFSDKVersion")]
    pub wmfsdkversion: Option<String>,
    #[serde(rename = "IsVBR")]
    pub is_vbr: Option<String>,
    pub major_brand: Option<String>,
    pub minor_version: Option<String>,
    pub compatible_brands: Option<String>,
    pub creation_time: Option<String>,
    pub encoder: Option<String>,

    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}
