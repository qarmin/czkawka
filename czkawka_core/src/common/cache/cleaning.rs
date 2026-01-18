use std::fs;
use std::io::{BufReader, BufWriter};
use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use bincode::Options;
use crossbeam_channel::Sender;
use fun_time::fun_time;
use itertools::Itertools;
use log::{debug, error};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use crate::common::cache::{CACHE_BROKEN_FILES_VERSION, CACHE_CLEANING_INTERVAL_SECONDS, CACHE_DUPLICATE_VERSION, CACHE_IMAGE_VERSION, CACHE_VERSION, CACHE_VIDEO_OPTIMIZE_VERSION, CACHE_VIDEO_VERSION, CLEANING_TIMESTAMPS_FILE, MEMORY_LIMIT};
use crate::common::traits::ResultEntry;

#[derive(Debug, Clone, Default)]
pub struct CacheCleaningStatistics {
    pub total_files_found: usize,
    pub successfully_cleaned: usize,
    pub files_with_errors: usize,
    pub total_entries_removed: usize,
    pub errors: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct CacheProgressCleaning {
    pub current_file: usize,
    pub total_files: usize,
    pub current_file_name: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct CleaningTimestamps {
    timestamps: Vec<SingleCleaningTimestamp>,
}
#[derive(Deserialize, Serialize, Debug)]
struct SingleCleaningTimestamp {
    cache_file_name: String,
    last_cleaned_timestamp: u64,
}

fn get_timestamps_file_path() -> Option<std::path::PathBuf> {
    use crate::common::config_cache_path::get_config_cache_path;
    get_config_cache_path().map(|config| config.cache_folder.join(CLEANING_TIMESTAMPS_FILE))
}

pub(crate) fn should_clean_cache(cache_file_name: &str) -> bool {
    let Some(timestamps_file) = get_timestamps_file_path() else {
        return true;
    };

    let Ok(content) = fs::read_to_string(&timestamps_file) else {
        return true;
    };

    let cleaning_timestamps = match serde_json::from_str::<CleaningTimestamps>(&content) {
        Ok(t) => t,
        Err(_) => {
            error!("Failed to parse cache file content");
            return true
        },
    };

    let current_time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    if let Some(timestamp) = cleaning_timestamps.timestamps.iter().find(|t| t.cache_file_name == cache_file_name) {
        let time_since_last_cleaning = current_time.saturating_sub(timestamp.last_cleaned_timestamp);
        if time_since_last_cleaning < *CACHE_CLEANING_INTERVAL_SECONDS {
            debug!(
                "Last cleaning for {} was {} seconds ago, which is less than the configured interval of {} seconds. Skipping cleaning.",
                cache_file_name, time_since_last_cleaning, *CACHE_CLEANING_INTERVAL_SECONDS
            );
            return false;
        } else {
            debug!(
                "Last cleaning for {} was {} seconds ago, which exceeds the configured interval of {} seconds. Proceeding with cleaning.",
                cache_file_name, time_since_last_cleaning, *CACHE_CLEANING_INTERVAL_SECONDS
            );
            return true;
        }
    }

    debug!("No cleaning timestamp found for {}, cache cleaning should run", cache_file_name);
    true
}

pub(crate) fn update_cleaning_timestamp(cache_file_name: &str) {
    let Some(timestamps_file) = get_timestamps_file_path() else {
        return;
    };

    let mut cleaning_timestamps = if let Ok(content) = fs::read_to_string(&timestamps_file) {
        serde_json::from_str::<CleaningTimestamps>(&content).unwrap_or_else(|e| {
            error!("Failed to parse cache file \"{cache_file_name}\" content - {e:?}");
            CleaningTimestamps { timestamps: vec![] }
        })
    } else {
        CleaningTimestamps { timestamps: vec![] }
    };

    let current_time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    if let Some(timestamp) = cleaning_timestamps.timestamps.iter_mut().find(|t| t.cache_file_name == cache_file_name) {
        timestamp.last_cleaned_timestamp = current_time;
    } else {
        cleaning_timestamps.timestamps.push(SingleCleaningTimestamp {
            cache_file_name: cache_file_name.to_string(),
            last_cleaned_timestamp: current_time,
        });
    }

    if let Ok(serialized) = serde_json::to_string_pretty(&cleaning_timestamps) {
        if let Err(e) = fs::write(&timestamps_file, serialized) {
            error!("Failed to write cleaning timestamps to file {}: {}", timestamps_file.to_string_lossy(), e);
        }
    } else {
        error!("Failed to serialize cleaning timestamps");
    }
}

#[derive(Debug)]
enum CacheType {
    Duplicates,
    MusicTags,
    MusicFingerprints,
    SimilarImages,
    SimilarVideos,
    BrokenFiles,
    ExifRemover,
    VideoTranscode,
    VideoCrop,
}



impl CacheType {
    fn from_filename(filename: &str) -> Option<Self> {
        if filename.starts_with("cache_duplicates_") && filename.ends_with(&format!("_{}.bin", CACHE_DUPLICATE_VERSION)) {
            Some(Self::Duplicates)
        } else if filename == format!("cache_same_music_tags_{}.bin", CACHE_VERSION) {
            Some(Self::MusicTags)
        } else if filename == format!("cache_same_music_fingerprints_{}.bin", CACHE_VERSION) {
            Some(Self::MusicFingerprints)
        } else if filename.starts_with("cache_similar_images_") && filename.ends_with(&format!("_{}.bin", CACHE_IMAGE_VERSION)) {
            Some(Self::SimilarImages)
        } else if filename.starts_with(&format!("cache_similar_videos_{}__", CACHE_VIDEO_VERSION)) && filename.ends_with(".bin") {
            Some(Self::SimilarVideos)
        } else if filename == format!("cache_broken_files_{}.bin", CACHE_BROKEN_FILES_VERSION) {
            Some(Self::BrokenFiles)
        } else if filename == format!("cache_exif_remover_{}.bin", CACHE_VERSION) {
            Some(Self::ExifRemover)
        } else if filename == format!("cache_video_transcode_{}.bin", CACHE_VIDEO_OPTIMIZE_VERSION) {
            Some(Self::VideoTranscode)
        } else if filename.starts_with(&format!("cache_video_crop_{}_", CACHE_VIDEO_OPTIMIZE_VERSION)) && filename.ends_with(".bin") {
            Some(Self::VideoCrop)
        } else {
            None
        }
    }
}

#[fun_time(message = "clean_all_cache_files", level = "debug")]
pub fn clean_all_cache_files(stop_flag: &Arc<AtomicBool>, cache_progress_sender: Option<&Sender<CacheProgressCleaning>>) -> Result<CacheCleaningStatistics, String> {
    use crate::common::config_cache_path::get_config_cache_path;
    use crate::tools::duplicate::DuplicateEntry;
    use crate::tools::broken_files::BrokenEntry;
    use crate::tools::same_music::MusicEntry;
    use crate::tools::similar_images::ImagesEntry;
    use crate::tools::similar_videos::VideosEntry;
    use crate::tools::exif_remover::ExifEntry;
    use crate::tools::video_optimizer::{VideoTranscodeEntry, VideoCropEntry};

    let mut stats = CacheCleaningStatistics::default();

    let Some(config_cache_path) = get_config_cache_path() else {
        return Err("Cannot get cache folder path".to_string());
    };

    let cache_folder = &config_cache_path.cache_folder;

    let entries = fs::read_dir(cache_folder)
        .map_err(|e| format!("Cannot read cache folder \"{}\": {}", cache_folder.to_string_lossy(), e))?;

    let cache_files: Vec<_> = entries
        .flatten()
        .filter_map(|entry| {
            let path = entry.path();
            if !path.is_file() {
                return None;
            }
            let file_name = path.file_name()?.to_str()?.to_string();
            let cache_type = CacheType::from_filename(&file_name)?;
            Some((path, file_name, cache_type))
        })
        .collect();

    let total_files = cache_files.len();

    let current_file = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let current_file_name = Arc::new(std::sync::Mutex::new(String::new()));

    let progress_thread = cache_progress_sender.map(|sender| {
        let sender = sender.clone();
        let stop_flag = stop_flag.clone();
        let current_file = current_file.clone();
        let current_file_name = current_file_name.clone();

        std::thread::spawn(move || {
            while !stop_flag.load(Ordering::Relaxed) {
                std::thread::sleep(std::time::Duration::from_millis(200));

                let current = current_file.load(Ordering::Relaxed);
                let name = current_file_name.lock().expect("Mutex poisoned").clone();

                if current > 0 {
                    let _ = sender.send(CacheProgressCleaning {
                        current_file: current,
                        total_files,
                        current_file_name: name,
                    });
                }
            }
        })
    });

    for (current_file_idx, (path, file_name, cache_type)) in cache_files.into_iter().enumerate() {
        if stop_flag.load(Ordering::Relaxed) {
            return Err("Operation stopped by user".to_string());
        }

        stats.total_files_found += 1;
        debug!("Found cache file to clean: {} (type: {:?})", file_name, cache_type);

        current_file.store(current_file_idx + 1, Ordering::Relaxed);
        *current_file_name.lock().unwrap() = file_name.clone();

        let result = match cache_type {
            CacheType::Duplicates => clean_cache_file_typed::<DuplicateEntry>(&path, stop_flag),
            CacheType::MusicTags | CacheType::MusicFingerprints => clean_cache_file_typed::<MusicEntry>(&path, stop_flag),
            CacheType::SimilarImages => clean_cache_file_typed::<ImagesEntry>(&path, stop_flag),
            CacheType::SimilarVideos => clean_cache_file_typed::<VideosEntry>(&path, stop_flag),
            CacheType::BrokenFiles => clean_cache_file_typed::<BrokenEntry>(&path, stop_flag),
            CacheType::ExifRemover => clean_cache_file_typed::<ExifEntry>(&path, stop_flag),
            CacheType::VideoTranscode => clean_cache_file_typed::<VideoTranscodeEntry>(&path, stop_flag),
            CacheType::VideoCrop => clean_cache_file_typed::<VideoCropEntry>(&path, stop_flag),
        };

        match result {
            Ok(Some(removed)) => {
                stats.successfully_cleaned += 1;
                stats.total_entries_removed += removed;
            }
            Ok(None) => {
                debug!("Cleaning of cache file {} was skipped due to stop flag", file_name);
                return Err("Operation stopped by user".to_string());
            }
            Err(e) => {
                stats.files_with_errors += 1;
                stats.errors.push(format!("{}: {}", file_name, e));
            }
        }
    }
    if let Some(handle) = progress_thread {
        let _ = handle.join();
    }

    Ok(stats)
}

fn clean_cache_file_typed<T>(
    cache_path: &Path,
    stop_flag: &Arc<AtomicBool>,
) -> Result<Option<usize>, String>
where
        for<'a> T: Deserialize<'a> + ResultEntry + Serialize + Clone + Send,
{

    let file = fs::File::open(cache_path).map_err(|e| format!("Cannot open file: {}", e))?;
    let reader = BufReader::new(file);

    let options = bincode::DefaultOptions::new().with_limit(MEMORY_LIMIT);
    let entries: Vec<T> = options
        .deserialize_from(reader)
        .map_err(|e| format!("Cannot deserialize file: {}", e))?;

    let original_count = entries.len();

    let filtered_entries: Vec<T> = entries
        .into_par_iter()
        .map(|cached_entry| {
            if stop_flag.load(Ordering::Relaxed) {
                return None;
            }
            let Ok(metadata) = fs::metadata(cached_entry.get_path()) else {
                return Some(None);
            };
            if metadata.len() != cached_entry.get_size() {
                return Some(None);
            }
            if let Ok(modified_time) = metadata.modified() {
                if let Ok(duration_since_epoch) = modified_time.duration_since(std::time::UNIX_EPOCH) {
                    if duration_since_epoch.as_secs() != cached_entry.get_modified_date() {
                        return Some(None);
                    }
                } else {
                    return Some(None);
                }
            }

            Some(Some(cached_entry))
        })
        .while_some()
        .flatten()
        .collect();

    if stop_flag.load(Ordering::Relaxed) {
        return Ok(None);
    }

    let removed_count = original_count - filtered_entries.len();

    if removed_count > 0 {
        let tmp_file_path = cache_path.with_extension("tmp");

        let tmp_file = fs::File::create(&tmp_file_path).map_err(|e| format!("Cannot create temporary file: {}", e))?;
        let writer = BufWriter::new(tmp_file);
        let options = bincode::DefaultOptions::new().with_limit(MEMORY_LIMIT);
        options
            .serialize_into(writer, &filtered_entries)
            .map_err(|e| format!("Cannot serialize cleaned data to temporary file: {}", e
            ))?;
        fs::rename(&tmp_file_path, cache_path).map_err(|e| format!("Cannot replace original cache file: {}", e))?;

        debug!(
            "Cleaned cache file \"{}\": removed {} entries, {} remaining",
            cache_path.to_string_lossy(),
            removed_count,
            filtered_entries.len()
        );
    }

    Ok(Some(removed_count))
}