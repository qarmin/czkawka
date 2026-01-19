use std::fs;
use std::path::Path;
use std::process::Command;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use crate::common::process_utils::run_command_interruptible;
use crate::common::video_utils::VideoMetadata;
use crate::tools::video_optimizer::{VideoTranscodeEntry, VideoTranscodeFixParams};

pub fn check_video(mut entry: VideoTranscodeEntry) -> VideoTranscodeEntry {
    let metadata = match VideoMetadata::from_path(&entry.path) {
        Ok(metadata) => metadata,
        Err(e) => {
            entry.error = Some(format!("Failed to get video metadata for file \"{}\": {}", entry.path.to_string_lossy(), e));
            return entry;
        }
    };

    let Some(current_codec) = metadata.codec.clone() else {
        entry.error = Some(format!("Failed to get video codec for file \"{}\"", entry.path.to_string_lossy()));
        return entry;
    };

    entry.codec = current_codec;
    entry.duration = metadata.duration;
    match (metadata.width, metadata.height) {
        (Some(width), Some(height)) => {
            entry.width = width;
            entry.height = height;
        }
        _ => {
            entry.error = Some(format!("Failed to get video dimensions for file \"{}\"", entry.path.to_string_lossy()));
            return entry;
        }
    }

    entry
}

pub fn process_video(stop_flag: &Arc<AtomicBool>, video_path: &str, original_size: u64, video_transcode_params: VideoTranscodeFixParams) -> Result<(), String> {
    let temp_output = Path::new(video_path).with_extension("czkawka_optimized.mp4");

    let mut command = Command::new("ffmpeg");
    command
        .arg("-i")
        .arg(video_path)
        .arg("-nostdin")
        .arg("-c:v")
        .arg(video_transcode_params.codec.as_str())
        .arg("-crf")
        .arg(video_transcode_params.quality.to_string());

    if video_transcode_params.limit_video_size {
        let scale_filter = format!(
            "scale='min({},iw):min({},ih):force_original_aspect_ratio=decrease'",
            video_transcode_params.max_width, video_transcode_params.max_height
        );
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

    if video_transcode_params.fail_if_not_smaller && new_size >= original_size {
        let _ = fs::remove_file(&temp_output);
        return Err(format!(
            "Optimized file({}) ({new_size} bytes) is larger than original({}) ({original_size} bytes)",
            temp_output.to_string_lossy(),
            video_path
        ));
    }

    if video_transcode_params.overwrite_original {
        fs::rename(&temp_output, video_path).map_err(|e| {
            let _ = fs::remove_file(&temp_output);
            format!("Failed to replace file \"{video_path}\" with optimized version: {e}")
        })?;
        return Ok(());
    }

    Ok(())
}
