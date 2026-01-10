use std::fs;
use std::path::Path;
use std::process::Command;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use log::debug;

use crate::common::process_utils::run_command_interruptible;
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

pub fn process_video(
    stop_flag: &Arc<AtomicBool>,
    video_path: &str,
    original_size: u64,
    video_codec: VideoCodec,
    target_quality: u32,
    fail_if_not_smaller: bool,
    overwrite_original: bool,
    limit_video_size: bool,
    max_width: u32,
    max_height: u32,
) -> Result<(), String> {
    let temp_output = Path::new(video_path).with_extension("czkawka_optimized.mp4");

    let mut command = Command::new("ffmpeg");
    command
        .arg("-i")
        .arg(video_path)
        .arg("-nostdin")
        .arg("-c:v")
        .arg(video_codec.as_str())
        .arg("-crf")
        .arg(target_quality.to_string());

    if limit_video_size {
        let scale_filter = format!("scale='min({max_width},iw):min({max_height},ih):force_original_aspect_ratio=decrease'");
        command.arg("-vf").arg(scale_filter);
    }

    command.arg("-c:a").arg("copy").arg("-y").arg(&temp_output);

    match run_command_interruptible(command, stop_flag) {
        None => {
            let _ = fs::remove_file(&temp_output);
            return Err(String::from("Video processing was stopped by user"));
        }
        Some(Err(e)) => {
            let _ = fs::remove_file(&temp_output);
            return Err(format!("Failed to process video file {video_path}: {e}"));
        }
        Some(Ok(_)) => {
            // Command succeeded, continue with validation
        }
    }

    let metadata = fs::metadata(&temp_output).map_err(|e| {
        let _ = fs::remove_file(&temp_output);
        format!("Failed to get metadata of optimized file \"{}\": {}", temp_output.to_string_lossy(), e)
    })?;

    let new_size = metadata.len();

    if fail_if_not_smaller && new_size >= original_size {
        let _ = fs::remove_file(&temp_output);
        return Err(format!(
            "Optimized file({}) ({new_size} bytes) is larger than original({}) ({original_size} bytes)",
            temp_output.to_string_lossy(),
            video_path
        ));
    }

    if overwrite_original {
        fs::rename(&temp_output, video_path).map_err(|e| {
            let _ = fs::remove_file(&temp_output);
            format!("Failed to replace file \"{video_path}\" with optimized version: {e}")
        })?;
        return Ok(());
    }

    Ok(())
}
