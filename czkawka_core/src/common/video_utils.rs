use std::path::Path;
use std::process::{Command, Stdio};

use ffprobe::ffprobe;
use image::RgbImage;
use serde::{Deserialize, Serialize};

use crate::common::consts::VIDEO_RESOLUTION_LIMIT;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct VideoMetadata {
    pub fps: Option<f64>,
    pub codec: Option<String>,
    pub bitrate: Option<u64>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub duration: Option<f64>,
}

impl VideoMetadata {
    pub fn from_path(path: &Path) -> Result<Self, String> {
        let info = ffprobe(path).map_err(|e| format!("Failed to read video properties: {e}"))?;

        let mut metadata = Self::default();

        if let Some(duration_str) = &info.format.duration
            && let Ok(d) = duration_str.parse::<f64>()
        {
            metadata.duration = Some(d);
        }

        if let Some(stream) = info.streams.into_iter().find(|s| s.codec_type.as_deref() == Some("video")) {
            metadata.codec = stream.codec_name;

            if let Some(bit_rate_str) = stream.bit_rate.or(info.format.bit_rate)
                && let Ok(b) = bit_rate_str.parse::<u64>()
            {
                metadata.bitrate = Some(b);
            }

            if let Some(w) = stream.width
                && w >= 0
            {
                if w > VIDEO_RESOLUTION_LIMIT as i64 {
                    return Err(format!("Video width {w} exceeds the limit of {VIDEO_RESOLUTION_LIMIT}"));
                }
                metadata.width = Some(w as u32);
            }
            if let Some(h) = stream.height
                && h >= 0
            {
                if h > VIDEO_RESOLUTION_LIMIT as i64 {
                    return Err(format!("Video height {h} exceeds the limit of {VIDEO_RESOLUTION_LIMIT}"));
                }
                metadata.height = Some(h as u32);
            }

            let fps_opt = if !stream.avg_frame_rate.is_empty() && stream.avg_frame_rate != "0/0" {
                Some(stream.avg_frame_rate)
            } else if !stream.r_frame_rate.is_empty() && stream.r_frame_rate != "0/0" {
                Some(stream.r_frame_rate)
            } else {
                None
            };

            if let Some(fps_str) = fps_opt {
                let fps_val = if fps_str.contains('/') {
                    let mut parts = fps_str.splitn(2, '/');
                    if let (Some(n), Some(d)) = (parts.next(), parts.next()) {
                        if let (Ok(nv), Ok(dv)) = (n.parse::<f64>(), d.parse::<f64>()) {
                            if dv != 0.0 { Some(nv / dv) } else { None }
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    fps_str.parse::<f64>().ok()
                };

                if let Some(fps_v) = fps_val {
                    metadata.fps = Some(fps_v);
                }
            }
        }

        Ok(metadata)
    }
}

pub(crate) fn extract_frame_ffmpeg(video_path: &Path, timestamp: f32, max_values: Option<(u32, u32)>) -> Result<RgbImage, String> {
    let mut command = Command::new("ffmpeg");
    let command_mut = &mut command;
    if let Some((max_width, max_height)) = max_values {
        let vf_filter = format!("scale='min({max_width},iw)':'min({max_height},ih)':force_original_aspect_ratio=decrease");
        command_mut.arg("-vf").arg(&vf_filter);
    }

    let output = command_mut
        .arg("-threads")
        .arg("1")
        .arg("-ss")
        .arg(timestamp.to_string())
        .arg("-i")
        .arg(video_path)
        .arg("-vframes")
        .arg("1")
        .arg("-f")
        .arg("image2pipe")
        .arg("-pix_fmt")
        .arg("rgb24")
        .arg("-vcodec")
        .arg("png")
        .arg("pipe:1")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .output()
        .map_err(|e| format!("Failed to execute ffmpeg: {e}"))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).replace("\r\n", "\n").replace("\n", " ");
        return Err(format!("ffmpeg failed with status: {} - {stderr}, ", output.status));
    }

    let img = image::load_from_memory(&output.stdout).map_err(|e| format!("Failed to load image: {e}"))?;

    Ok(img.to_rgb8())
}
