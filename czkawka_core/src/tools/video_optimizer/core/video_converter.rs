use std::fs;
use std::path::Path;
use std::process::Command;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use log::error;

use crate::common::process_utils::run_command_interruptible;
use crate::common::video_utils::VideoMetadata;
use crate::flc;
use crate::tools::video_optimizer::{VideoTranscodeEntry, VideoTranscodeFixParams};

pub fn check_video(mut entry: VideoTranscodeEntry) -> VideoTranscodeEntry {
    let metadata = match VideoMetadata::from_path(&entry.path) {
        Ok(metadata) => metadata,
        Err(e) => {
            entry.error = Some(flc!("core_failed_to_get_video_metadata", file = entry.path.to_string_lossy(), reason = e));
            return entry;
        }
    };

    let Some(current_codec) = metadata.codec.clone() else {
        entry.error = Some(flc!("core_failed_to_get_video_codec", file = entry.path.to_string_lossy()));
        return entry;
    };

    let Some(duration) = metadata.duration else {
        entry.error = Some(flc!("core_failed_to_get_video_duration", file = entry.path.to_string_lossy()));
        return entry;
    };

    entry.codec = current_codec;
    entry.duration = duration;
    match (metadata.width, metadata.height) {
        (Some(width), Some(height)) => {
            entry.width = width;
            entry.height = height;
        }
        _ => {
            entry.error = Some(flc!("core_failed_to_get_video_dimensions", file = entry.path.to_string_lossy()));
            return entry;
        }
    }

    entry
}

pub fn process_video(stop_flag: &Arc<AtomicBool>, video_path: &str, original_size: u64, params: VideoTranscodeFixParams) -> Result<(), String> {
    let temp_output = Path::new(video_path).with_extension("czkawka_optimized.mp4");

    let mut command = Command::new("ffmpeg");
    command
        .arg("-i")
        .arg(video_path)
        .arg("-nostdin")
        .arg("-c:v")
        .arg(params.codec.as_str())
        .arg("-crf")
        .arg(params.quality.to_string());

    if params.limit_video_size {
        let scale_filter = format!("scale='min({},iw):min({},ih):force_original_aspect_ratio=decrease'", params.max_width, params.max_height);
        command.arg("-vf").arg(scale_filter);
    }

    command.arg("-c:a").arg("copy").arg("-y").arg(&temp_output);

    match run_command_interruptible(command, stop_flag) {
        None => {
            let _ = fs::remove_file(&temp_output);
            return Err(flc!("core_video_processing_stopped_by_user"));
        }
        Some(Err(e)) => {
            let _ = fs::remove_file(&temp_output);
            return Err(flc!("core_failed_to_process_video", file = video_path, reason = e));
        }
        Some(Ok(output)) => {
            if !output.status.success() {
                let connected = format!("{} - {}", output.stdout, output.stderr);
                if connected.to_lowercase().contains("unknown encoder") {
                    return Err(flc!("core_ffmpeg_unknown_encoder", file = video_path, encoder = params.codec.as_ffprobe_codec_name()));
                }
                error!(
                    "FFmpeg failed to transcode video \"{}\" with status {}. Stdout: {}, Stderr: {}",
                    video_path, output.status, output.stdout, output.stderr
                );
                return Err(flc!("core_ffmpeg_error", file = video_path, code = output.status.to_string(), reason = output.stderr));
            }
        }
    }

    let metadata = fs::metadata(&temp_output).map_err(|e| {
        let _ = fs::remove_file(&temp_output);
        flc!(
            "core_failed_to_get_metadata_of_optimized_file",
            file = temp_output.to_string_lossy(),
            reason = e.to_string()
        )
    })?;

    let new_size = metadata.len();

    if params.fail_if_not_smaller && new_size >= original_size {
        let _ = fs::remove_file(&temp_output);
        return Err(flc!(
            "core_optimized_file_larger",
            optimized = temp_output.to_string_lossy(),
            new_size = new_size,
            original = video_path,
            original_size = original_size
        ));
    }

    if params.overwrite_original {
        fs::rename(&temp_output, video_path).map_err(|e| {
            let _ = fs::remove_file(&temp_output);
            flc!("core_failed_to_replace_with_optimized", file = video_path, reason = e.to_string())
        })?;
        return Ok(());
    }

    Ok(())
}
