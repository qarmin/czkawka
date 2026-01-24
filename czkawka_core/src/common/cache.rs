#![allow(clippy::useless_let_if_seq)]

mod cleaning;

use std::collections::BTreeMap;
use std::io::{BufReader, BufWriter};
use std::path::Path;
use std::{fs, mem};

use bincode::Options;
pub use cleaning::{CacheCleaningStatistics, CacheProgressCleaning, clean_all_cache_files};
use fun_time::fun_time;
use humansize::{BINARY, format_size};
use indexmap::IndexMap;
use log::{debug, error};
use once_cell::sync::Lazy;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use serde::{Deserialize, Serialize};

use crate::common::cache::cleaning::{should_clean_cache, update_cleaning_timestamp};
use crate::common::config_cache_path::open_cache_folder;
use crate::common::tool_data::CommonData;
use crate::common::traits::ResultEntry;
use crate::helpers::messages::Messages;

pub(crate) const CACHE_VERSION: u8 = 100;
pub(crate) const CACHE_DUPLICATE_VERSION: u8 = 100;
pub(crate) const CACHE_IMAGE_VERSION: u8 = 100;
pub(crate) const CACHE_VIDEO_VERSION: u8 = 110;
pub(crate) const CACHE_BROKEN_FILES_VERSION: u8 = 110;
pub(crate) const CACHE_VIDEO_OPTIMIZE_VERSION: u8 = 110;

const MEMORY_LIMIT: u64 = 8 * 1024 * 1024 * 1024;
const CLEANING_TIMESTAMPS_FILE: &str = "cleaning_timestamps.json";

static CACHE_CLEANING_INTERVAL_SECONDS: Lazy<u64> = Lazy::new(|| {
    option_env!("CZKAWKA_CACHE_CLEANING_INTERVAL_SECONDS")
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(7 * 24 * 60 * 60)
});

fn get_cache_size(file_name: &Path) -> String {
    fs::metadata(file_name).map_or_else(|_| "<unknown size>".to_string(), |metadata| format_size(metadata.len(), BINARY))
}

#[fun_time(message = "save_cache_to_file_generalized", level = "debug")]
pub fn save_cache_to_file_generalized<T>(cache_file_name: &str, hashmap: &BTreeMap<String, T>, save_also_as_json: bool, minimum_file_size: u64) -> Messages
where
    T: Serialize + ResultEntry + Sized + Send + Sync,
{
    let mut text_messages = Messages::new();
    if let Some(((file_handler, cache_file), (file_handler_json, cache_file_json))) = open_cache_folder(cache_file_name, true, save_also_as_json, &mut text_messages.warnings) {
        let hashmap_to_save = hashmap.values().filter(|t| t.get_size() >= minimum_file_size).collect::<Vec<_>>();

        {
            let writer = BufWriter::new(file_handler.expect("Cannot fail, because for saving, this always exists"));
            let options = bincode::DefaultOptions::new().with_limit(MEMORY_LIMIT);
            if let Err(e) = options.serialize_into(writer, &hashmap_to_save) {
                text_messages
                    .warnings
                    .push(format!("Cannot write data to cache file \"{}\", reason {e}", cache_file.to_string_lossy()));
                debug!("Failed to save cache to file \"{}\" - {e}", cache_file.to_string_lossy());
                return text_messages;
            }
            debug!("Saved cache to binary file \"{}\" with size {}", cache_file.to_string_lossy(), get_cache_size(&cache_file));
        }
        if save_also_as_json && let Some(file_handler_json) = file_handler_json {
            let writer = BufWriter::new(file_handler_json);
            if let Err(e) = serde_json::to_writer(writer, &hashmap_to_save) {
                text_messages
                    .warnings
                    .push(format!("Cannot write data to cache file \"{}\", reason {e}", cache_file_json.to_string_lossy()));
                debug!("Failed to save cache to file \"{}\" - {e}", cache_file_json.to_string_lossy());
                return text_messages;
            }
            debug!(
                "Saved cache to json file \"{}\" with size {}",
                cache_file_json.to_string_lossy(),
                get_cache_size(&cache_file_json)
            );
        }

        text_messages.messages.push(format!("Properly saved to file {} cache entries.", hashmap.len()));
        debug!("Properly saved to file {} cache entries.", hashmap.len());
    } else {
        debug!("Failed to save cache to file {cache_file_name} because not exists");
    }
    text_messages
}

pub(crate) fn extract_loaded_cache<T>(
    loaded_hash_map: &BTreeMap<String, T>,
    files_to_check: BTreeMap<String, T>,
    records_already_cached: &mut BTreeMap<String, T>,
    non_cached_files_to_check: &mut BTreeMap<String, T>,
) where
    T: Clone,
{
    for (name, file_entry) in files_to_check {
        if let Some(cached_file_entry) = loaded_hash_map.get(&name) {
            records_already_cached.insert(name, cached_file_entry.clone());
        } else {
            non_cached_files_to_check.insert(name, file_entry);
        }
    }
}

#[fun_time(message = "load_cache_from_file_generalized_by_path", level = "debug")]
pub fn load_cache_from_file_generalized_by_path<T>(cache_file_name: &str, delete_outdated_cache: bool, used_files: &BTreeMap<String, T>) -> (Messages, Option<BTreeMap<String, T>>)
where
    for<'a> T: Deserialize<'a> + ResultEntry + Sized + Send + Sync + Clone,
{
    let check_file = |file_entry: &T| {
        let file_entry_path_str = file_entry.get_path().to_string_lossy();
        let key: &str = file_entry_path_str.as_ref();
        if let Some(used_file) = used_files.get(key) {
            if file_entry.get_size() != used_file.get_size() {
                return false;
            }
            if file_entry.get_modified_date() != used_file.get_modified_date() {
                return false;
            }
        }
        true
    };

    let (text_messages, vec_loaded_cache) = load_cache_from_file_generalized(cache_file_name, delete_outdated_cache, check_file);
    let Some(vec_loaded_entries) = vec_loaded_cache else {
        return (text_messages, None);
    };

    debug!("Converting cache Vec<T> into BTreeMap<String, T>");
    let number_of_entries = vec_loaded_entries.len();
    let start_time = std::time::Instant::now();
    let map_loaded_entries: BTreeMap<String, T> = vec_loaded_entries
        .into_iter()
        .map(|file_entry| (file_entry.get_path().to_string_lossy().into_owned(), file_entry))
        .collect();
    debug!("Converted cache Vec<T>({number_of_entries} results) into BTreeMap<String, T> in {:?}", start_time.elapsed());

    (text_messages, Some(map_loaded_entries))
}

#[fun_time(message = "load_cache_from_file_generalized_by_size", level = "debug")]
pub fn load_cache_from_file_generalized_by_size<T>(
    cache_file_name: &str,
    delete_outdated_cache: bool,
    cache_not_converted: &BTreeMap<u64, Vec<T>>,
) -> (Messages, Option<BTreeMap<u64, Vec<T>>>)
where
    for<'a> T: Deserialize<'a> + ResultEntry + Sized + Send + Sync + Clone,
{
    debug!("Converting cache BtreeMap<u64, Vec<T>> into IndexMap<String, (u64, u64)>");
    let used_files: IndexMap<String, (u64, u64)> = cache_not_converted
        .iter()
        .flat_map(|(size, vec)| {
            vec.iter()
                .map(move |file_entry| (file_entry.get_path().to_string_lossy().into_owned(), (*size, file_entry.get_modified_date())))
        })
        .collect();
    debug!("Converted cache BtreeMap<u64, Vec<T>> into IndexMap<String, (u64, u64)>");

    let check_file = |file_entry: &T| {
        let file_entry_path_str = file_entry.get_path().to_string_lossy();
        let key: &str = file_entry_path_str.as_ref();
        if let Some((size, modification_date)) = used_files.get(key) {
            if file_entry.get_size() != *size {
                return false;
            }
            if file_entry.get_modified_date() != *modification_date {
                return false;
            }
        }
        true
    };

    let (text_messages, vec_loaded_cache) = load_cache_from_file_generalized(cache_file_name, delete_outdated_cache, check_file);
    let Some(vec_loaded_entries) = vec_loaded_cache else {
        return (text_messages, None);
    };

    debug!("Converting cache Vec<T> into BTreeMap<u64, Vec<T>>");
    let number_of_entries = vec_loaded_entries.len();
    let start_time = std::time::Instant::now();
    let mut map_loaded_entries: BTreeMap<u64, Vec<T>> = Default::default();
    for file_entry in vec_loaded_entries {
        map_loaded_entries.entry(file_entry.get_size()).or_default().push(file_entry);
    }
    debug!(
        "Converted cache Vec<T>({number_of_entries} results) into BTreeMap<u64, Vec<T>> in {:?}",
        start_time.elapsed()
    );

    (text_messages, Some(map_loaded_entries))
}

#[fun_time(message = "load_cache_from_file_generalized", level = "debug")]
fn load_cache_from_file_generalized<T, F>(cache_file_name: &str, delete_outdated_cache: bool, check_func: F) -> (Messages, Option<Vec<T>>)
where
    for<'a> T: Deserialize<'a> + ResultEntry + Sized + Send + Sync + Clone,
    F: Fn(&T) -> bool + Send + Sync,
{
    let mut text_messages = Messages::new();

    if let Some(((file_handler, cache_file), (file_handler_json, cache_file_json))) = open_cache_folder(cache_file_name, false, true, &mut text_messages.warnings) {
        let cache_full_name;
        let mut vec_loaded_entries: Vec<T>;
        if let Some(file_handler) = file_handler {
            cache_full_name = cache_file.clone();
            let reader = BufReader::new(file_handler);

            let options = bincode::DefaultOptions::new().with_limit(MEMORY_LIMIT);
            vec_loaded_entries = match options.deserialize_from(reader) {
                Ok(t) => t,
                Err(e) => {
                    text_messages
                        .warnings
                        .push(format!("Failed to load data from cache file {}, reason {e}", cache_file.to_string_lossy()));
                    error!("Failed to load cache from file {} - {e}", cache_file.to_string_lossy());
                    return (text_messages, None);
                }
            };
        } else {
            cache_full_name = cache_file_json.clone();
            let reader = BufReader::new(file_handler_json.expect("This cannot fail, because if file_handler is None, then this cannot be None"));
            vec_loaded_entries = match serde_json::from_reader(reader) {
                Ok(t) => t,
                Err(e) => {
                    text_messages
                        .warnings
                        .push(format!("Failed to load data from json cache file {}, reason {e}", cache_file_json.to_string_lossy()));
                    debug!("Failed to load cache from file {} - {e}", cache_file_json.to_string_lossy());
                    return (text_messages, None);
                }
            };
        }

        let should_clean = should_clean_cache(cache_file_name);
        debug!("Starting removing outdated cache entries (removing non existent files from cache - {delete_outdated_cache}, should_clean - {should_clean})");
        let initial_number_of_entries = vec_loaded_entries.len();
        let deleting_start_time = std::time::Instant::now();

        let effective_delete_outdated = delete_outdated_cache && should_clean;

        vec_loaded_entries = vec_loaded_entries
            .into_par_iter()
            .filter(|file_entry| {
                if !check_func(file_entry) {
                    return false;
                }

                if effective_delete_outdated && !file_entry.get_path().exists() {
                    return false;
                }

                true
            })
            .collect();

        if effective_delete_outdated {
            update_cleaning_timestamp(cache_file_name);
        }

        debug!(
            "Completed removing outdated cache entries, removed {} out of all {} entries in {:?}",
            initial_number_of_entries - vec_loaded_entries.len(),
            initial_number_of_entries,
            deleting_start_time.elapsed()
        );

        text_messages.messages.push(format!("Properly loaded {} cache entries.", vec_loaded_entries.len()));

        debug!(
            "Loaded cache from file {cache_file_name} (or json alternative) - {} results - size {}",
            vec_loaded_entries.len(),
            get_cache_size(&cache_full_name)
        );
        return (text_messages, Some(vec_loaded_entries));
    }
    debug!("Failed to load cache from file {cache_file_name} because not exists");
    (text_messages, None)
}

pub(crate) fn load_and_split_cache_generalized_by_path<C: CommonData, K>(
    cache_file_name: &str,
    mut items_to_check: BTreeMap<String, K>,
    common_data: &mut C,
) -> (BTreeMap<String, K>, BTreeMap<String, K>, BTreeMap<String, K>)
where
    for<'a> K: Deserialize<'a> + ResultEntry + Sized + Send + Sync + Clone,
{
    if !common_data.get_use_cache() {
        return (Default::default(), Default::default(), items_to_check);
    }

    let loaded_hash_map;

    let mut records_already_cached: BTreeMap<String, K> = Default::default();
    let mut non_cached_files_to_check: BTreeMap<String, K> = Default::default();

    let (messages, loaded_items) = load_cache_from_file_generalized_by_path::<K>(cache_file_name, common_data.get_delete_outdated_cache(), &items_to_check);
    common_data.get_text_messages_mut().extend_with_another_messages(messages);
    loaded_hash_map = loaded_items.unwrap_or_default();

    debug!("load_cache - Starting to check for differences");
    extract_loaded_cache(
        &loaded_hash_map,
        mem::take(&mut items_to_check),
        &mut records_already_cached,
        &mut non_cached_files_to_check,
    );
    debug!(
        "load_cache - completed diff between loaded and prechecked files, {}({}) - non cached, {}({}) - already cached",
        non_cached_files_to_check.len(),
        format_size(non_cached_files_to_check.values().map(|e| e.get_size()).sum::<u64>(), BINARY),
        records_already_cached.len(),
        format_size(records_already_cached.values().map(|e| e.get_size()).sum::<u64>(), BINARY),
    );
    (loaded_hash_map, records_already_cached, non_cached_files_to_check)
}

pub(crate) fn save_and_connect_cache_generalized_by_path<C: CommonData, K>(cache_file_name: &str, vec_file_entry: &[K], loaded_hash_map: BTreeMap<String, K>, common_data: &mut C)
where
    K: Serialize + ResultEntry + Sized + Send + Sync + Clone,
{
    if !common_data.get_use_cache() {
        return;
    }
    let mut all_results: BTreeMap<String, K> = Default::default();

    for file_entry in vec_file_entry.iter().cloned() {
        all_results.insert(file_entry.get_path().to_string_lossy().to_string(), file_entry);
    }
    for (name, file_entry) in loaded_hash_map {
        all_results.insert(name, file_entry);
    }

    let messages = save_cache_to_file_generalized(cache_file_name, &all_results, common_data.get_save_also_as_json(), 0);
    common_data.get_text_messages_mut().extend_with_another_messages(messages);
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use std::fs;
    use std::path::PathBuf;
    use std::sync::Once;

    use tempfile::TempDir;

    use super::*;
    use crate::common::config_cache_path::set_config_cache_path_test;

    static INIT: Once = Once::new();

    pub(crate) fn setup_cache_path() {
        INIT.call_once(|| {
            let temp_cache_dir = TempDir::new().expect("Failed to create temp cache dir");
            let temp_config_dir = TempDir::new().expect("Failed to create temp config dir");

            let cache_path = temp_cache_dir.path().to_path_buf();
            let config_path = temp_config_dir.path().to_path_buf();

            set_config_cache_path_test(cache_path, config_path);

            // Leak the TempDir to keep directories alive for the duration of tests
            std::mem::forget(temp_cache_dir);
            std::mem::forget(temp_config_dir);
        });
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    struct TestEntry {
        path: PathBuf,
        size: u64,
        modified_date: u64,
        value: u32,
    }

    impl ResultEntry for TestEntry {
        fn get_path(&self) -> &Path {
            &self.path
        }
        fn get_modified_date(&self) -> u64 {
            self.modified_date
        }
        fn get_size(&self) -> u64 {
            self.size
        }
    }

    impl TestEntry {
        fn new(path: &str, size: u64, modified_date: u64, value: u32) -> Self {
            Self {
                path: PathBuf::from(path),
                size,
                modified_date,
                value,
            }
        }
    }

    #[test]
    fn test_extract_loaded_cache() {
        let mut loaded_cache = BTreeMap::new();
        loaded_cache.insert("file1".to_string(), TestEntry::new("/tmp/file1", 100, 1000, 10));
        loaded_cache.insert("file2".to_string(), TestEntry::new("/tmp/file2", 200, 2000, 20));

        let mut files_to_check = BTreeMap::new();
        files_to_check.insert("file1".to_string(), TestEntry::new("/tmp/file1", 100, 1000, 10));
        files_to_check.insert("file3".to_string(), TestEntry::new("/tmp/file3", 300, 3000, 30));
        files_to_check.insert("file2".to_string(), TestEntry::new("/tmp/file2", 200, 2000, 20));

        let mut records_already_cached = BTreeMap::new();
        let mut non_cached_files_to_check = BTreeMap::new();

        extract_loaded_cache(&loaded_cache, files_to_check, &mut records_already_cached, &mut non_cached_files_to_check);

        assert_eq!(records_already_cached.len(), 2);
        assert_eq!(non_cached_files_to_check.len(), 1);
        assert!(records_already_cached.contains_key("file1"));
        assert!(records_already_cached.contains_key("file2"));
        assert!(non_cached_files_to_check.contains_key("file3"));
        assert_eq!(records_already_cached.get("file1").unwrap().value, 10);
        assert_eq!(non_cached_files_to_check.get("file3").unwrap().value, 30);
    }

    #[test]
    fn test_extract_loaded_cache_empty() {
        let loaded_cache: BTreeMap<String, TestEntry> = BTreeMap::new();
        let mut files_to_check = BTreeMap::new();
        files_to_check.insert("file1".to_string(), TestEntry::new("/tmp/file1", 100, 1000, 10));
        files_to_check.insert("file2".to_string(), TestEntry::new("/tmp/file2", 200, 2000, 20));

        let mut records_already_cached = BTreeMap::new();
        let mut non_cached_files_to_check = BTreeMap::new();

        extract_loaded_cache(&loaded_cache, files_to_check, &mut records_already_cached, &mut non_cached_files_to_check);

        assert_eq!(records_already_cached.len(), 0, "No entries should be cached");
        assert_eq!(non_cached_files_to_check.len(), 2, "All entries should be non-cached");
    }

    #[test]
    fn test_extract_loaded_cache_all_cached() {
        let mut loaded_cache = BTreeMap::new();
        loaded_cache.insert("file1".to_string(), TestEntry::new("/tmp/file1", 100, 1000, 10));
        loaded_cache.insert("file2".to_string(), TestEntry::new("/tmp/file2", 200, 2000, 20));

        let mut files_to_check = BTreeMap::new();
        files_to_check.insert("file1".to_string(), TestEntry::new("/tmp/file1", 100, 1000, 10));
        files_to_check.insert("file2".to_string(), TestEntry::new("/tmp/file2", 200, 2000, 20));

        let mut records_already_cached = BTreeMap::new();
        let mut non_cached_files_to_check = BTreeMap::new();

        extract_loaded_cache(&loaded_cache, files_to_check, &mut records_already_cached, &mut non_cached_files_to_check);

        assert_eq!(records_already_cached.len(), 2, "All entries should be cached");
        assert_eq!(non_cached_files_to_check.len(), 0, "No entries should be non-cached");
    }

    #[test]
    fn test_save_and_load_cache_by_path() {
        setup_cache_path();
        let temp_dir = TempDir::new().unwrap();
        let temp_file = temp_dir.path().join("test_file.txt");
        fs::write(&temp_file, "test content").unwrap();
        let metadata = fs::metadata(&temp_file).unwrap();

        let mut cache_to_save = BTreeMap::new();
        cache_to_save.insert(
            temp_file.to_string_lossy().to_string(),
            TestEntry::new(temp_file.to_str().unwrap(), metadata.len(), metadata.modified().unwrap().elapsed().unwrap().as_secs(), 42),
        );

        // Save cache
        let cache_name = format!("test_cache_by_path_{}", std::process::id());
        let messages = save_cache_to_file_generalized(&cache_name, &cache_to_save, false, 0);
        assert!(messages.warnings.is_empty(), "Should not have warnings when saving");
        assert!(!messages.messages.is_empty(), "Should have success messages when saving");

        // Load cache
        let (load_messages, loaded_cache) = load_cache_from_file_generalized_by_path::<TestEntry>(&cache_name, false, &cache_to_save);
        assert!(load_messages.warnings.is_empty(), "Should not have warnings when loading");
        assert!(!load_messages.messages.is_empty(), "Should have success messages when loading");
        assert!(loaded_cache.is_some(), "Should load cache successfully");

        let loaded = loaded_cache.unwrap();
        assert_eq!(loaded.len(), 1, "Should load 1 entry");
        assert!(loaded.contains_key(temp_file.to_str().unwrap()), "Should contain the test file");
    }

    #[test]
    fn test_save_and_load_cache_by_size() {
        setup_cache_path();
        let temp_dir = TempDir::new().unwrap();
        let temp_file1 = temp_dir.path().join("test_file1.txt");
        let temp_file2 = temp_dir.path().join("test_file2.txt");
        fs::write(&temp_file1, "test content 1").unwrap();
        fs::write(&temp_file2, "test content 2").unwrap();

        let metadata1 = fs::metadata(&temp_file1).unwrap();
        let metadata2 = fs::metadata(&temp_file2).unwrap();

        let mut cache_to_save: BTreeMap<u64, Vec<TestEntry>> = BTreeMap::new();
        cache_to_save.entry(metadata1.len()).or_default().push(TestEntry::new(
            temp_file1.to_str().unwrap(),
            metadata1.len(),
            metadata1.modified().unwrap().elapsed().unwrap().as_secs(),
            10,
        ));
        cache_to_save.entry(metadata2.len()).or_default().push(TestEntry::new(
            temp_file2.to_str().unwrap(),
            metadata2.len(),
            metadata2.modified().unwrap().elapsed().unwrap().as_secs(),
            20,
        ));

        // Convert to flat map for saving
        let mut flat_cache = BTreeMap::new();
        for entries in cache_to_save.values() {
            for entry in entries {
                flat_cache.insert(entry.path.to_string_lossy().to_string(), entry.clone());
            }
        }

        // Save cache
        let cache_name = format!("test_cache_by_size_{}", std::process::id());
        let messages = save_cache_to_file_generalized(&cache_name, &flat_cache, false, 0);
        assert!(messages.warnings.is_empty(), "Should not have warnings when saving");

        // Load cache
        let (load_messages, loaded_cache) = load_cache_from_file_generalized_by_size::<TestEntry>(&cache_name, false, &cache_to_save);
        assert!(load_messages.warnings.is_empty(), "Should not have warnings when loading");
        assert!(loaded_cache.is_some(), "Should load cache successfully");

        let loaded = loaded_cache.unwrap();
        assert!(!loaded.is_empty(), "Should load entries");
    }

    #[test]
    fn test_save_cache_with_minimum_file_size() {
        setup_cache_path();
        let temp_dir = TempDir::new().unwrap();
        let temp_file = temp_dir.path().join("test_file.txt");
        fs::write(&temp_file, "test").unwrap();

        let mut cache_to_save = BTreeMap::new();
        cache_to_save.insert("small_file".to_string(), TestEntry::new("/tmp/small", 10, 1000, 1));
        cache_to_save.insert("large_file".to_string(), TestEntry::new("/tmp/large", 1000, 2000, 2));

        // Save cache with minimum file size of 100 bytes
        let cache_name = format!("test_cache_min_size_{}", std::process::id());
        let messages = save_cache_to_file_generalized(&cache_name, &cache_to_save, false, 100);
        assert!(messages.warnings.is_empty(), "Should not have warnings");

        // Load cache - should only contain large file
        let files_to_check = cache_to_save.clone();
        let (_, loaded_cache) = load_cache_from_file_generalized_by_path::<TestEntry>(&cache_name, false, &files_to_check);

        if let Some(loaded) = loaded_cache {
            // Only the large file should be saved (size >= 100)
            for (_, entry) in loaded {
                assert!(entry.size >= 100, "All loaded entries should be >= minimum size");
            }
        }
    }

    #[test]
    fn test_load_cache_with_outdated_entries() {
        setup_cache_path();
        let temp_dir = TempDir::new().unwrap();
        let temp_file = temp_dir.path().join("test_file.txt");
        fs::write(&temp_file, "test content").unwrap();
        let metadata = fs::metadata(&temp_file).unwrap();

        let mut cache_to_save = BTreeMap::new();
        cache_to_save.insert(
            temp_file.to_string_lossy().to_string(),
            TestEntry::new(temp_file.to_str().unwrap(), metadata.len(), metadata.modified().unwrap().elapsed().unwrap().as_secs(), 42),
        );

        // Save cache
        let cache_name = format!("test_cache_outdated_{}", std::process::id());
        save_cache_to_file_generalized(&cache_name, &cache_to_save, false, 0);

        // Modify the file
        std::thread::sleep(std::time::Duration::from_millis(100));
        fs::write(&temp_file, "modified content").unwrap();

        // Create new files_to_check with updated metadata
        let new_metadata = fs::metadata(&temp_file).unwrap();
        let mut files_to_check = BTreeMap::new();
        files_to_check.insert(
            temp_file.to_string_lossy().to_string(),
            TestEntry::new(
                temp_file.to_str().unwrap(),
                new_metadata.len(),
                new_metadata.modified().unwrap().elapsed().unwrap().as_secs(),
                42,
            ),
        );

        // Load cache - should filter out the outdated entry
        let (_, loaded_cache) = load_cache_from_file_generalized_by_path::<TestEntry>(&cache_name, false, &files_to_check);

        if let Some(loaded) = loaded_cache {
            // Should be empty because size/modified date changed
            assert!(loaded.is_empty() || loaded.len() < cache_to_save.len(), "Outdated entries should be filtered");
        }
    }

    #[test]
    fn test_load_nonexistent_cache() {
        setup_cache_path();
        let cache_name = format!("nonexistent_cache_{}", std::process::id());
        let files_to_check: BTreeMap<String, TestEntry> = BTreeMap::new();

        let (messages, loaded_cache) = load_cache_from_file_generalized_by_path::<TestEntry>(&cache_name, false, &files_to_check);

        assert!(loaded_cache.is_none(), "Should return None for nonexistent cache");
        assert!(messages.warnings.is_empty(), "Should not have warnings for nonexistent cache");
    }

    #[test]
    fn test_save_cache_with_json() {
        setup_cache_path();
        let temp_dir = TempDir::new().unwrap();
        let temp_file = temp_dir.path().join("test_file.txt");
        fs::write(&temp_file, "test content").unwrap();

        let mut cache_to_save = BTreeMap::new();
        cache_to_save.insert("test_key".to_string(), TestEntry::new("/tmp/test", 100, 1000, 42));

        // Save cache with JSON enabled
        let cache_name = format!("test_cache_json_{}", std::process::id());
        let messages = save_cache_to_file_generalized(&cache_name, &cache_to_save, true, 0);
        assert!(messages.warnings.is_empty(), "Should not have warnings when saving with JSON");
    }

    #[test]
    fn test_get_cache_size_nonexistent() {
        let nonexistent_path = Path::new("/nonexistent/path/to/cache.bin");
        let size_str = get_cache_size(nonexistent_path);
        assert_eq!(size_str, "<unknown size>", "Should return unknown size for nonexistent file");
    }
}
