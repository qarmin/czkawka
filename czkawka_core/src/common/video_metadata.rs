use std::path::Path;

use ffprobe::ffprobe;
use serde::{Deserialize, Serialize};

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
                metadata.width = Some(w as u32);
            }
            if let Some(h) = stream.height
                && h >= 0
            {
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

    pub fn dimensions_string(&self) -> Option<String> {
        match (self.width, self.height) {
            (Some(w), Some(h)) => Some(format!("{w}x{h}")),
            _ => None,
        }
    }
}
