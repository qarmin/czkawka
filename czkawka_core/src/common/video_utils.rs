use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use blake3::Hasher;
use ffprobe::ffprobe;
use image::{GenericImage, RgbImage};
use serde::{Deserialize, Serialize};

use crate::common::consts::VIDEO_RESOLUTION_LIMIT;
use crate::common::process_utils::disable_windows_console_window;
use crate::common::progress_stop_handler::check_if_stop_received;

pub const VIDEO_THUMBNAILS_SUBFOLDER: &str = "video_thumbnails";

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
    // This function returns strange status 234, when path contains non default UTF-8 characters, not sure why
    if !video_path.exists() {
        return Err(format!(
            "Video file does not exist(could be removed between scan/later steps): \"{}\"",
            video_path.to_string_lossy()
        ));
    }

    let mut command = Command::new("ffmpeg");
    let command_mut = &mut command;

    disable_windows_console_window(command_mut);

    command_mut.arg("-threads").arg("1").arg("-ss").arg(timestamp.to_string()).arg("-i").arg(video_path);

    if let Some((max_width, max_height)) = max_values {
        let vf_filter = format!("scale='min({max_width},iw)':'min({max_height},ih)':force_original_aspect_ratio=decrease");
        command_mut.arg("-vf").arg(&vf_filter);
    }

    let output = command_mut
        .arg("-vframes")
        .arg("1")
        .arg("-f")
        .arg("image2pipe")
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
        return Err(format!("ffmpeg failed with status: {} - {stderr} - command {command:?} ", output.status));
    }

    let img = image::load_from_memory(&output.stdout).map_err(|e| format!("Failed to load image: {e}"))?;

    Ok(img.into_rgb8())
}

pub fn generate_thumbnail(
    stop_flag: &Arc<AtomicBool>,
    video_path: &Path,
    size: u64,
    modified_date: u64,
    duration: Option<f64>,
    thumbnails_dir: &Path,
    thumbnail_video_percentage_from_start: u8,
    generate_grid_instead_of_single: bool,
) -> Result<PathBuf, String> {
    let mut hasher = Hasher::new();
    hasher.update(
        format!(
            "{thumbnail_video_percentage_from_start}___{}___{}___{}___{generate_grid_instead_of_single}",
            size,
            modified_date,
            video_path.to_string_lossy()
        )
        .as_bytes(),
    );
    let hash = hasher.finalize();
    let thumbnail_filename = format!("{}.jpg", hash.to_hex());
    let thumbnail_path = thumbnails_dir.join(thumbnail_filename);

    if thumbnail_path.exists() {
        let _ = filetime::set_file_mtime(&thumbnail_path, filetime::FileTime::now());
        return Ok(thumbnail_path);
    }

    let seek_time = duration.map_or(5.0, |d| d * (thumbnail_video_percentage_from_start as f64) / 100.0);
    let duration_per_11_items = duration.map_or(0.5, |d| d / 11.0);

    let max_height = 1080;
    let max_width = 1920;
    let tiles_size = 3;

    if generate_grid_instead_of_single {
        let frame_times = (0..(tiles_size * tiles_size)).map(|i| duration_per_11_items as f32 * (i + 1) as f32).collect::<Vec<f32>>();
        let mut imgs = Vec::new();
        for ft in frame_times {
            if check_if_stop_received(stop_flag) {
                return Err(String::from("Thumbnail generation was stopped by user"));
            }

            match extract_frame_ffmpeg(video_path, ft, Some((max_width, max_height))) {
                Ok(img) => imgs.push(img),
                Err(e) => {
                    let _ = fs::write(&thumbnail_path, b"");
                    return Err(format!("Failed to extract frame at {ft} seconds from \"{}\": {e}", video_path.to_string_lossy()));
                }
            }
        }
        assert_eq!(imgs.len(), tiles_size * tiles_size);

        let first_img = &imgs.first().expect("Cannot be empty here, because at least tiles_size^2 images are extracted");

        if imgs.iter().any(|img| img.height() != first_img.height() || img.width() != first_img.width()) {
            let _ = fs::write(&thumbnail_path, b"");
            return Err(format!(
                "Failed to generate thumbnail for \"{}\": extracted frames have different dimensions",
                video_path.to_string_lossy()
            ));
        }
        let mut new_thumbnail = RgbImage::new(first_img.width() * tiles_size as u32, first_img.height() * tiles_size as u32);
        for (idx, img) in imgs.iter().enumerate() {
            let x = (idx % tiles_size) as u32 * img.width();
            let y = (idx / tiles_size) as u32 * img.height();
            new_thumbnail
                .copy_from(img, x, y)
                .map_err(|e| format!("Failed to generate thumbnail for \"{}\": {e}", video_path.to_string_lossy()))?;
        }

        if let Err(e) = new_thumbnail.save(&thumbnail_path) {
            let _ = fs::write(&thumbnail_path, b"");
            return Err(format!("Failed to save thumbnail for \"{}\": {e}", video_path.to_string_lossy()));
        }
    } else {
        match extract_frame_ffmpeg(video_path, seek_time as f32, Some((max_width, max_height))) {
            Ok(img) => {
                if let Err(e) = img.save(&thumbnail_path) {
                    let _ = fs::write(&thumbnail_path, b"");
                    return Err(format!("Failed to save thumbnail for \"{}\": {e}", video_path.to_string_lossy()));
                }
            }
            Err(e) => {
                let _ = fs::write(&thumbnail_path, b"");
                return Err(format!("Failed to extract frame at {seek_time} seconds from \"{}\" - {e}", video_path.to_string_lossy()));
            }
        }
    }
    Ok(thumbnail_path)
}
