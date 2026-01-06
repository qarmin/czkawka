use log::debug;

use crate::common::video_metadata::VideoMetadata;
use crate::tools::video_optimizer::VideoCropEntry;

pub fn check_video_crop(mut entry: VideoCropEntry) -> VideoCropEntry {
    debug!("Checking video for crop: {}", entry.path.display());

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

    // TODO: Implement crop detection logic
    // For now, just return the entry with basic metadata

    entry
}

