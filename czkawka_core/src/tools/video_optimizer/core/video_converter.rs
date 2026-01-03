use std::fs;
use std::path::Path;
use std::process::Command;

use log::{debug, info, warn};

use crate::common::video_metadata::VideoMetadata;
use crate::tools::video_optimizer::{VideoCodec, VideoTranscodeEntry};

pub fn check_video(mut entry: VideoTranscodeEntry) -> VideoTranscodeEntry {
    debug!("Checking video: {}", entry.path.display());

    let metadata = match VideoMetadata::from_path(&entry.path) {
        Ok(metadata) => metadata,
        Err(e) => {
            entry.error = Some(e);
            return entry;
        }
    };

    let Some(current_codec) = metadata.codec.clone() else {
        entry.error = Some("Failed to get video codec".to_string());
        return entry;
    };

    entry.codec = current_codec;
    match (metadata.width, metadata.height) {
        (Some(width), Some(height)) => {
            entry.width = width;
            entry.height = height;
        }
        _ => {
            entry.error = Some("Failed to get video dimensions".to_string());
            return entry;
        }
    }

    entry
}

pub fn process_video(video_path: &Path, original_size: u64, video_codec: VideoCodec, target_quality: u32) -> Result<u64, String> {
    debug!("Processing video: {}", video_path.display());

    let temp_output = video_path.with_extension(format!("optimized.{}", video_path.extension().and_then(|e| e.to_str()).unwrap_or("mp4")));

    let codec_str = video_codec.as_str();
    let quality = target_quality.to_string();

    let output = Command::new("ffmpeg")
        .arg("-i")
        .arg(video_path)
        .arg("-c:v")
        .arg(codec_str)
        .arg("-crf")
        .arg(&quality)
        .arg("-c:a")
        .arg("copy")
        .arg("-y")
        .arg(&temp_output)
        .output()
        .map_err(|e| format!("Failed to execute ffmpeg: {e}"))?;

    if !output.status.success() {
        let _ = fs::remove_file(&temp_output);
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    let metadata = fs::metadata(&temp_output).map_err(|e| {
        let _ = fs::remove_file(&temp_output);
        format!("Failed to get metadata: {e}")
    })?;

    let new_size = metadata.len();

    if new_size < original_size {
        fs::rename(&temp_output, video_path).map_err(|e| {
            let _ = fs::remove_file(&temp_output);
            format!("Failed to replace file: {e}")
        })?;

        info!("Successfully optimized video: {} ({} -> {} bytes)", video_path.display(), original_size, new_size);
        Ok(new_size)
    } else {
        warn!("Optimized video is larger than original, keeping original: {}", video_path.display());
        let _ = fs::remove_file(&temp_output);
        Err(format!("Optimized file ({new_size} bytes) is larger than original ({original_size} bytes)"))
    }
}
