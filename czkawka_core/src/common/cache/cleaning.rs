use std::fs;
use std::io::{BufReader, BufWriter};
use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use bincode::Options;
use crossbeam_channel::Sender;
use fun_time::fun_time;
use log::{debug, error};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};

use crate::common::cache::{
    CACHE_BROKEN_FILES_VERSION, CACHE_CLEANING_INTERVAL_SECONDS, CACHE_DUPLICATE_VERSION, CACHE_IMAGE_VERSION, CACHE_VERSION, CACHE_VIDEO_OPTIMIZE_VERSION, CACHE_VIDEO_VERSION,
    CLEANING_TIMESTAMPS_FILE, MEMORY_LIMIT,
};
use crate::common::config_cache_path::get_config_cache_path;
use crate::common::traits::ResultEntry;
use crate::tools::broken_files::BrokenEntry;
use crate::tools::duplicate::DuplicateEntry;
use crate::tools::exif_remover::ExifEntry;
use crate::tools::same_music::MusicEntry;
use crate::tools::similar_images::ImagesEntry;
use crate::tools::similar_videos::VideosEntry;
use crate::tools::video_optimizer::{VideoCropEntry, VideoTranscodeEntry};

#[derive(Debug, Clone, Default)]
pub struct CacheCleaningStatistics {
    pub total_files_found: usize,
    pub successfully_cleaned: usize,
    pub files_with_errors: usize,
    pub total_entries_before: usize,
    pub total_entries_removed: usize,
    pub total_entries_left: usize,
    pub total_size_before: u64,
    pub total_size_after: u64,
    pub errors: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct CacheProgressCleaning {
    pub current_cache_file: usize,
    pub total_cache_files: usize,
    pub current_file_name: String,
    pub checked_entries: usize,
    pub all_entries: usize,
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
        Err(e) => {
            error!(
                "Failed to parse cleaning timestamps file \"{}\" while processing cache file \"{cache_file_name}\" - {e:?}",
                timestamps_file.to_string_lossy()
            );
            return true;
        }
    };

    let current_time = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs();

    if let Some(timestamp) = cleaning_timestamps.timestamps.iter().find(|t| t.cache_file_name == cache_file_name) {
        let time_since_last_cleaning = current_time.saturating_sub(timestamp.last_cleaned_timestamp);
        if time_since_last_cleaning < *CACHE_CLEANING_INTERVAL_SECONDS {
            debug!(
                "Last cleaning for {} was {} seconds ago, which is less than the configured interval of {} seconds. Skipping cleaning.",
                cache_file_name, time_since_last_cleaning, *CACHE_CLEANING_INTERVAL_SECONDS
            );
            return false;
        }
        debug!(
            "Last cleaning for {} was {} seconds ago, which exceeds the configured interval of {} seconds. Proceeding with cleaning.",
            cache_file_name, time_since_last_cleaning, *CACHE_CLEANING_INTERVAL_SECONDS
        );
        return true;
    }

    debug!("No cleaning timestamp found for {cache_file_name}, cache cleaning should run");
    true
}

pub(crate) fn update_cleaning_timestamp(cache_file_name: &str) {
    let Some(timestamps_file) = get_timestamps_file_path() else {
        return;
    };

    let mut cleaning_timestamps = if let Ok(content) = fs::read_to_string(&timestamps_file) {
        serde_json::from_str::<CleaningTimestamps>(&content).unwrap_or_else(|e| {
            error!("Failed to parse cleaning timestamps file \"{}\" content - {e:?}", timestamps_file.to_string_lossy());
            CleaningTimestamps { timestamps: vec![] }
        })
    } else {
        CleaningTimestamps { timestamps: vec![] }
    };

    let current_time = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs();

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
            error!("Failed to write cleaning timestamps to file {}: {e}", timestamps_file.to_string_lossy());
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
        if filename.starts_with("cache_duplicates_") && filename.ends_with(&format!("_{CACHE_DUPLICATE_VERSION}.bin")) {
            Some(Self::Duplicates)
        } else if filename == format!("cache_same_music_tags_{CACHE_VERSION}.bin") {
            Some(Self::MusicTags)
        } else if filename == format!("cache_same_music_fingerprints_{CACHE_VERSION}.bin") {
            Some(Self::MusicFingerprints)
        } else if filename.starts_with("cache_similar_images_") && filename.ends_with(&format!("_{CACHE_IMAGE_VERSION}.bin")) {
            Some(Self::SimilarImages)
        } else if filename.starts_with(&format!("cache_similar_videos_{CACHE_VIDEO_VERSION}__")) && filename.ends_with(".bin") {
            Some(Self::SimilarVideos)
        } else if filename == format!("cache_broken_files_{CACHE_BROKEN_FILES_VERSION}.bin") {
            Some(Self::BrokenFiles)
        } else if filename == format!("cache_exif_remover_{CACHE_VERSION}.bin") {
            Some(Self::ExifRemover)
        } else if filename == format!("cache_video_transcode_{CACHE_VIDEO_OPTIMIZE_VERSION}.bin") {
            Some(Self::VideoTranscode)
        } else if filename.starts_with(&format!("cache_video_crop_{CACHE_VIDEO_OPTIMIZE_VERSION}_")) && filename.ends_with(".bin") {
            Some(Self::VideoCrop)
        } else {
            None
        }
    }
}

#[fun_time(message = "clean_all_cache_files", level = "debug")]
pub fn clean_all_cache_files(stop_flag: &Arc<AtomicBool>, cache_progress_sender: Option<&Sender<CacheProgressCleaning>>) -> Result<CacheCleaningStatistics, String> {
    let mut stats = CacheCleaningStatistics::default();

    let Some(config_cache_path) = get_config_cache_path() else {
        return Err("Cannot get cache folder path".to_string());
    };

    let cache_folder = &config_cache_path.cache_folder;

    let entries = fs::read_dir(cache_folder).map_err(|e| format!("Cannot read cache folder \"{}\": {}", cache_folder.to_string_lossy(), e))?;

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
    let checked_entries = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let all_entries = Arc::new(std::sync::atomic::AtomicUsize::new(0));

    let progress_thread = cache_progress_sender.map(|sender| {
        let sender = sender.clone();
        let stop_flag = stop_flag.clone();
        let current_file = current_file.clone();
        let current_file_name = current_file_name.clone();
        let checked_entries = checked_entries.clone();
        let all_entries = all_entries.clone();

        std::thread::spawn(move || {
            while !stop_flag.load(Ordering::Relaxed) {
                std::thread::sleep(std::time::Duration::from_millis(100));

                let current = current_file.load(Ordering::Relaxed);
                let name = current_file_name.lock().expect("Mutex poisoned").clone();
                let checked = checked_entries.load(Ordering::Relaxed);
                let all = all_entries.load(Ordering::Relaxed);

                if current > 0 {
                    let _ = sender.send(CacheProgressCleaning {
                        current_cache_file: current,
                        total_cache_files: total_files,
                        current_file_name: name,
                        checked_entries: checked,
                        all_entries: all,
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
        debug!("Found cache file to clean: {file_name} (type: {cache_type:?})");

        current_file.store(current_file_idx + 1, Ordering::Relaxed);
        *current_file_name.lock().expect("Lock poisoned") = file_name.clone();

        checked_entries.store(0, Ordering::Relaxed);
        all_entries.store(0, Ordering::Relaxed);

        let result = match cache_type {
            CacheType::Duplicates => clean_cache_file_typed::<DuplicateEntry>(&path, stop_flag, &checked_entries, &all_entries),
            CacheType::MusicTags | CacheType::MusicFingerprints => clean_cache_file_typed::<MusicEntry>(&path, stop_flag, &checked_entries, &all_entries),
            CacheType::SimilarImages => clean_cache_file_typed::<ImagesEntry>(&path, stop_flag, &checked_entries, &all_entries),
            CacheType::SimilarVideos => clean_cache_file_typed::<VideosEntry>(&path, stop_flag, &checked_entries, &all_entries),
            CacheType::BrokenFiles => clean_cache_file_typed::<BrokenEntry>(&path, stop_flag, &checked_entries, &all_entries),
            CacheType::ExifRemover => clean_cache_file_typed::<ExifEntry>(&path, stop_flag, &checked_entries, &all_entries),
            CacheType::VideoTranscode => clean_cache_file_typed::<VideoTranscodeEntry>(&path, stop_flag, &checked_entries, &all_entries),
            CacheType::VideoCrop => clean_cache_file_typed::<VideoCropEntry>(&path, stop_flag, &checked_entries, &all_entries),
        };

        match result {
            Ok(Some((before, after, size_before, size_after))) => {
                stats.successfully_cleaned += 1;
                stats.total_entries_before += before;
                stats.total_entries_left += after;
                stats.total_entries_removed += before - after;
                stats.total_size_before += size_before;
                stats.total_size_after += size_after;

                update_cleaning_timestamp(&file_name);
            }
            Ok(None) => {
                debug!("Cleaning of cache file {file_name} was skipped due to stop flag");
                return Err("Operation stopped by user".to_string());
            }
            Err(e) => {
                stats.files_with_errors += 1;
                stats.errors.push(format!("{file_name}: {e}"));
            }
        }
    }
    stop_flag.store(true, Ordering::Relaxed);
    if let Some(handle) = progress_thread {
        let _ = handle.join();
    }

    Ok(stats)
}

fn clean_cache_file_typed<T>(
    cache_path: &Path,
    stop_flag: &Arc<AtomicBool>,
    checked_entries: &Arc<std::sync::atomic::AtomicUsize>,
    all_entries: &Arc<std::sync::atomic::AtomicUsize>,
) -> Result<Option<(usize, usize, u64, u64)>, String>
where
    for<'a> T: Deserialize<'a> + ResultEntry + Serialize + Clone + Send,
{
    let size_before = fs::metadata(cache_path).map(|m| m.len()).unwrap_or(0);

    let file = fs::File::open(cache_path).map_err(|e| format!("Cannot open file: {e}"))?;
    let reader = BufReader::new(file);

    let options = bincode::DefaultOptions::new().with_limit(MEMORY_LIMIT);
    let entries: Vec<T> = options.deserialize_from(reader).map_err(|e| format!("Cannot deserialize file: {e}"))?;

    let original_count = entries.len();

    all_entries.store(original_count, Ordering::Relaxed);

    let checked_entries_clone = checked_entries.clone();

    let filtered_entries: Vec<T> = entries
        .into_par_iter()
        .map(|cached_entry| {
            if stop_flag.load(Ordering::Relaxed) {
                return None;
            }

            checked_entries_clone.fetch_add(1, Ordering::Relaxed);

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

    let remaining_count = filtered_entries.len();
    let removed_count = original_count - remaining_count;

    let size_after = if removed_count > 0 {
        let tmp_file_path = cache_path.with_extension("tmp");

        let tmp_file = fs::File::create(&tmp_file_path).map_err(|e| format!("Cannot create temporary file: {e}"))?;
        let writer = BufWriter::new(tmp_file);
        let options = bincode::DefaultOptions::new().with_limit(MEMORY_LIMIT);
        options
            .serialize_into(writer, &filtered_entries)
            .map_err(|e| format!("Cannot serialize cleaned data to temporary file: {e}"))?;

        let new_size = fs::metadata(&tmp_file_path).map(|m| m.len()).unwrap_or(size_before);

        fs::rename(&tmp_file_path, cache_path).map_err(|e| format!("Cannot replace original cache file: {e}"))?;

        debug!(
            "Cleaned cache file \"{}\": removed {} entries, {} remaining, size reduced from {} to {} bytes",
            cache_path.to_string_lossy(),
            removed_count,
            filtered_entries.len(),
            size_before,
            new_size
        );

        new_size
    } else {
        size_before
    };

    Ok(Some((original_count, remaining_count, size_before, size_after)))
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use std::sync::Arc;
    use std::sync::atomic::AtomicBool;
    use std::time::UNIX_EPOCH;

    use bincode::Options;
    use serde::{Deserialize, Serialize};
    use tempfile::TempDir;

    use super::*;
    use crate::common::cache::tests::setup_cache_path;

    #[derive(Clone, Debug, Serialize, Deserialize)]
    struct TestCacheEntry {
        path: PathBuf,
        size: u64,
        modified_date: u64,
        data: String,
    }

    impl ResultEntry for TestCacheEntry {
        fn get_path(&self) -> &Path {
            &self.path
        }
        fn get_size(&self) -> u64 {
            self.size
        }
        fn get_modified_date(&self) -> u64 {
            self.modified_date
        }
    }

    fn setup_test_env() -> (PathBuf, PathBuf) {
        setup_cache_path();
        let config_cache = get_config_cache_path().unwrap();
        (config_cache.cache_folder.clone(), config_cache.config_folder)
    }

    fn create_test_file(dir: &Path, name: &str, content: &str) -> (PathBuf, u64, u64) {
        let path = dir.join(name);
        fs::write(&path, content).unwrap();
        let metadata = fs::metadata(&path).unwrap();
        let modified = metadata.modified().unwrap().duration_since(UNIX_EPOCH).unwrap().as_secs();
        (path, metadata.len(), modified)
    }

    fn create_cache_file(cache_dir: &Path, name: &str, entries: &[TestCacheEntry]) -> PathBuf {
        let cache_path = cache_dir.join(name);
        let file = fs::File::create(&cache_path).unwrap();
        let writer = BufWriter::new(file);
        let options = bincode::DefaultOptions::new().with_limit(MEMORY_LIMIT);
        options.serialize_into(writer, entries).unwrap();
        cache_path
    }

    #[test]
    fn test_timestamp_operations_and_should_clean() {
        let (_cache_dir, _config_dir) = setup_test_env();
        let cache_name = format!("test_cache_{}", std::process::id());

        assert!(should_clean_cache(&cache_name));

        update_cleaning_timestamp(&cache_name);
        assert!(!should_clean_cache(&cache_name));

        update_cleaning_timestamp(&cache_name);
        assert!(!should_clean_cache(&cache_name));

        let different_cache = format!("different_cache_{}", std::process::id());
        assert!(should_clean_cache(&different_cache));

        update_cleaning_timestamp(&different_cache);
        assert!(!should_clean_cache(&different_cache));
        assert!(!should_clean_cache(&cache_name));
    }

    #[test]
    fn test_clean_cache_file_typed_mixed_scenarios() {
        let (cache_dir, _config_dir) = setup_test_env();
        let data_dir = TempDir::new().unwrap();

        let (valid_path, valid_size, valid_modified) = create_test_file(data_dir.path(), "valid.txt", "valid content");
        let (modified_path, _, old_modified) = create_test_file(data_dir.path(), "modified.txt", "old content");
        std::thread::sleep(std::time::Duration::from_millis(100));
        fs::write(&modified_path, "new content with different size").unwrap();
        let (deleted_path, deleted_size, deleted_modified) = create_test_file(data_dir.path(), "deleted.txt", "to be deleted");
        fs::remove_file(&deleted_path).unwrap();

        let entries = vec![
            TestCacheEntry {
                path: valid_path.clone(),
                size: valid_size,
                modified_date: valid_modified,
                data: "valid".to_string(),
            },
            TestCacheEntry {
                path: modified_path,
                size: 11,
                modified_date: old_modified,
                data: "modified".to_string(),
            },
            TestCacheEntry {
                path: deleted_path,
                size: deleted_size,
                modified_date: deleted_modified,
                data: "deleted".to_string(),
            },
        ];

        let cache_path = create_cache_file(cache_dir.as_path(), "test_cache.bin", &entries);
        let stop_flag = Arc::new(AtomicBool::new(false));
        let checked = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let all = Arc::new(std::sync::atomic::AtomicUsize::new(0));

        let result = clean_cache_file_typed::<TestCacheEntry>(&cache_path, &stop_flag, &checked, &all).unwrap();

        assert!(result.is_some());
        let (original, remaining, _, _) = result.unwrap();
        assert_eq!(original, 3);
        assert_eq!(remaining, 1);
        assert_eq!(checked.load(Ordering::Relaxed), 3);
        assert_eq!(all.load(Ordering::Relaxed), 3);

        let file = fs::File::open(&cache_path).unwrap();
        let reader = BufReader::new(file);
        let options = bincode::DefaultOptions::new().with_limit(MEMORY_LIMIT);
        let cleaned_entries: Vec<TestCacheEntry> = options.deserialize_from(reader).unwrap();
        assert_eq!(cleaned_entries.len(), 1);
        assert_eq!(cleaned_entries[0].path, valid_path);
    }

    #[test]
    fn test_clean_cache_file_with_stop_flag() {
        let (cache_dir, _config_dir) = setup_test_env();
        let data_dir = TempDir::new().unwrap();

        const ENTRIES_NUMBER: usize = 100;

        let mut entries = Vec::new();
        for i in 0..ENTRIES_NUMBER {
            let (path, size, modified) = create_test_file(data_dir.path(), &format!("file_{i}.txt"), &format!("content {i}"));
            entries.push(TestCacheEntry {
                path,
                size,
                modified_date: modified,
                data: format!("data {i}"),
            });
        }

        let cache_path = create_cache_file(cache_dir.as_path(), "test_stop.bin", &entries);
        let stop_flag = Arc::new(AtomicBool::new(false));
        let checked = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let all = Arc::new(std::sync::atomic::AtomicUsize::new(0));

        let stop_flag_clone = stop_flag.clone();
        std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_millis(1));
            stop_flag_clone.store(true, Ordering::Relaxed);
        });

        // Well - it may fail in any place, so we just cannot check exact number of checked entries
        let result = clean_cache_file_typed::<TestCacheEntry>(&cache_path, &stop_flag, &checked, &all).unwrap();
        if result.is_some() {
            assert!(checked.load(Ordering::Relaxed) <= ENTRIES_NUMBER);
        }
    }

    #[test]
    fn test_cache_type_from_filename_all_variants() {
        assert!(matches!(
            CacheType::from_filename(&format!("cache_duplicates_hash_{CACHE_DUPLICATE_VERSION}.bin")),
            Some(CacheType::Duplicates)
        ));
        assert!(matches!(
            CacheType::from_filename(&format!("cache_duplicates_size_{CACHE_DUPLICATE_VERSION}.bin")),
            Some(CacheType::Duplicates)
        ));
        assert!(matches!(
            CacheType::from_filename(&format!("cache_same_music_tags_{CACHE_VERSION}.bin")),
            Some(CacheType::MusicTags)
        ));
        assert!(matches!(
            CacheType::from_filename(&format!("cache_same_music_fingerprints_{CACHE_VERSION}.bin")),
            Some(CacheType::MusicFingerprints)
        ));
        assert!(matches!(
            CacheType::from_filename(&format!("cache_similar_images_8_{CACHE_IMAGE_VERSION}.bin")),
            Some(CacheType::SimilarImages)
        ));
        assert!(matches!(
            CacheType::from_filename(&format!("cache_similar_videos_{CACHE_VIDEO_VERSION}__10.bin")),
            Some(CacheType::SimilarVideos)
        ));
        assert!(matches!(
            CacheType::from_filename(&format!("cache_broken_files_{CACHE_BROKEN_FILES_VERSION}.bin")),
            Some(CacheType::BrokenFiles)
        ));
        assert!(matches!(
            CacheType::from_filename(&format!("cache_exif_remover_{CACHE_VERSION}.bin")),
            Some(CacheType::ExifRemover)
        ));
        assert!(matches!(
            CacheType::from_filename(&format!("cache_video_transcode_{CACHE_VIDEO_OPTIMIZE_VERSION}.bin")),
            Some(CacheType::VideoTranscode)
        ));
        assert!(matches!(
            CacheType::from_filename(&format!("cache_video_crop_{CACHE_VIDEO_OPTIMIZE_VERSION}_test.bin")),
            Some(CacheType::VideoCrop)
        ));

        assert!(CacheType::from_filename("invalid_cache.bin").is_none());
        assert!(CacheType::from_filename("cache_duplicates_99.bin").is_none());
        assert!(CacheType::from_filename("random_file.txt").is_none());
    }

    #[test]
    fn test_clean_cache_file_no_changes_needed() {
        let (cache_dir, _config_dir) = setup_test_env();
        let data_dir = TempDir::new().unwrap();

        let mut entries = Vec::new();
        for i in 0..5 {
            let (path, size, modified) = create_test_file(data_dir.path(), &format!("valid_{i}.txt"), &format!("valid content {i}"));
            entries.push(TestCacheEntry {
                path,
                size,
                modified_date: modified,
                data: format!("data {i}"),
            });
        }

        let cache_path = create_cache_file(cache_dir.as_path(), "test_no_changes.bin", &entries);
        let size_before = fs::metadata(&cache_path).unwrap().len();

        let stop_flag = Arc::new(AtomicBool::new(false));
        let checked = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let all = Arc::new(std::sync::atomic::AtomicUsize::new(0));

        let result = clean_cache_file_typed::<TestCacheEntry>(&cache_path, &stop_flag, &checked, &all).unwrap();

        assert!(result.is_some());
        let (original, remaining, size_before_result, size_after) = result.unwrap();
        assert_eq!(original, 5);
        assert_eq!(remaining, 5);
        assert_eq!(size_before_result, size_before);
        assert_eq!(size_after, size_before);
    }

    #[test]
    fn test_clean_cache_file_all_entries_invalid() {
        let (cache_dir, _config_dir) = setup_test_env();
        let data_dir = TempDir::new().unwrap();

        let (deleted1, size1, mod1) = create_test_file(data_dir.path(), "del1.txt", "content 1");
        let (deleted2, size2, mod2) = create_test_file(data_dir.path(), "del2.txt", "content 2");
        let (deleted3, size3, mod3) = create_test_file(data_dir.path(), "del3.txt", "content 3");

        fs::remove_file(&deleted1).unwrap();
        fs::remove_file(&deleted2).unwrap();
        fs::remove_file(&deleted3).unwrap();

        let entries = vec![
            TestCacheEntry {
                path: deleted1,
                size: size1,
                modified_date: mod1,
                data: "1".to_string(),
            },
            TestCacheEntry {
                path: deleted2,
                size: size2,
                modified_date: mod2,
                data: "2".to_string(),
            },
            TestCacheEntry {
                path: deleted3,
                size: size3,
                modified_date: mod3,
                data: "3".to_string(),
            },
        ];

        let cache_path = create_cache_file(cache_dir.as_path(), "test_all_invalid.bin", &entries);
        let stop_flag = Arc::new(AtomicBool::new(false));
        let checked = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let all = Arc::new(std::sync::atomic::AtomicUsize::new(0));

        let result = clean_cache_file_typed::<TestCacheEntry>(&cache_path, &stop_flag, &checked, &all).unwrap();

        assert!(result.is_some());
        let (original, remaining, _, _) = result.unwrap();
        assert_eq!(original, 3);
        assert_eq!(remaining, 0);

        let file = fs::File::open(&cache_path).unwrap();
        let reader = BufReader::new(file);
        let options = bincode::DefaultOptions::new().with_limit(MEMORY_LIMIT);
        let cleaned_entries: Vec<TestCacheEntry> = options.deserialize_from(reader).unwrap();
        assert_eq!(cleaned_entries.len(), 0);
    }

    #[test]
    fn test_cache_progress_cleaning_struct() {
        let progress = CacheProgressCleaning {
            current_cache_file: 3,
            total_cache_files: 10,
            current_file_name: "test_cache.bin".to_string(),
            checked_entries: 50,
            all_entries: 100,
        };

        assert_eq!(progress.current_cache_file, 3);
        assert_eq!(progress.total_cache_files, 10);
        assert_eq!(progress.current_file_name, "test_cache.bin");
        assert_eq!(progress.checked_entries, 50);
        assert_eq!(progress.all_entries, 100);
    }

    #[test]
    fn test_cleaning_timestamps_serialization() {
        let timestamps = CleaningTimestamps {
            timestamps: vec![
                SingleCleaningTimestamp {
                    cache_file_name: "cache1.bin".to_string(),
                    last_cleaned_timestamp: 1000,
                },
                SingleCleaningTimestamp {
                    cache_file_name: "cache2.bin".to_string(),
                    last_cleaned_timestamp: 2000,
                },
            ],
        };

        let serialized = serde_json::to_string(&timestamps).unwrap();
        let deserialized: CleaningTimestamps = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized.timestamps.len(), 2);
        assert_eq!(deserialized.timestamps[0].cache_file_name, "cache1.bin");
        assert_eq!(deserialized.timestamps[0].last_cleaned_timestamp, 1000);
        assert_eq!(deserialized.timestamps[1].cache_file_name, "cache2.bin");
        assert_eq!(deserialized.timestamps[1].last_cleaned_timestamp, 2000);
    }
}
