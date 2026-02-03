#![allow(clippy::useless_let_if_seq)]

use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::io::{BufReader, BufWriter};
use std::path::Path;

use bincode::Options;
use fun_time::fun_time;
use humansize::{BINARY, format_size};
use log::{debug, error};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use serde::{Deserialize, Serialize};

use crate::common::config_cache_path::open_cache_folder;
use crate::common::traits::ResultEntry;
use crate::helpers::messages::Messages;

pub(crate) const CACHE_VERSION: &str = "100";
pub(crate) const CACHE_DUPLICATE_VERSION: &str = "100";
pub(crate) const CACHE_IMAGE_VERSION: &str = "100";
pub(crate) const CACHE_VIDEO_VERSION: &str = "100";

const MEMORY_LIMIT: u64 = 8 * 1024 * 1024 * 1024;

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
                text_messages.warnings.push(format!("Cannot write data to cache file {cache_file:?}, reason {e}"));
                debug!("Failed to save cache to file {cache_file:?} - {e}");
                return text_messages;
            }
            debug!("Saved cache to binary file {cache_file:?} with size {}", get_cache_size(&cache_file));
        }
        if save_also_as_json && let Some(file_handler_json) = file_handler_json {
            let writer = BufWriter::new(file_handler_json);
            if let Err(e) = serde_json::to_writer(writer, &hashmap_to_save) {
                text_messages.warnings.push(format!("Cannot write data to cache file {cache_file_json:?}, reason {e}"));
                debug!("Failed to save cache to file {cache_file_json:?} - {e}");
                return text_messages;
            }
            debug!("Saved cache to json file {cache_file_json:?} with size {}", get_cache_size(&cache_file_json));
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
        let file_entry_path_str: &str = file_entry_path_str.as_ref();
        if let Some(used_file) = used_files.get(file_entry_path_str) {
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
    let map_loaded_entries: BTreeMap<String, T> = vec_loaded_entries
        .into_iter()
        .map(|file_entry| (file_entry.get_path().to_string_lossy().into_owned(), file_entry))
        .collect();
    debug!("Converted cache Vec<T> into BTreeMap<String, T>");

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
    debug!("Converting cache BtreeMap<u64, Vec<T>> into HashMap<String, (u64, u64)>");
    let used_files: HashMap<String, (u64, u64)> = cache_not_converted
        .iter()
        .flat_map(|(size, vec)| {
            vec.iter()
                .map(move |file_entry| (file_entry.get_path().to_string_lossy().into_owned(), (*size, file_entry.get_modified_date())))
        })
        .collect();
    debug!("Converted cache BtreeMap<u64, Vec<T>> into HashMap<String, (u64, u64)>");

    let check_file = |file_entry: &T| {
        let file_entry_path_str = file_entry.get_path().to_string_lossy();
        let file_entry_path_str: &str = file_entry_path_str.as_ref();
        if let Some((size, modification_date)) = used_files.get(file_entry_path_str) {
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
    let mut map_loaded_entries: BTreeMap<u64, Vec<T>> = Default::default();
    for file_entry in vec_loaded_entries {
        map_loaded_entries.entry(file_entry.get_size()).or_default().push(file_entry);
    }
    debug!("Converted cache Vec<T> into BTreeMap<u64, Vec<T>>");

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
                    text_messages.warnings.push(format!("Failed to load data from cache file {cache_file:?}, reason {e}"));
                    error!("Failed to load cache from file {cache_file:?} - {e}");
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
                        .push(format!("Failed to load data from json cache file {cache_file_json:?}, reason {e}"));
                    debug!("Failed to load cache from file {cache_file:?} - {e}");
                    return (text_messages, None);
                }
            };
        }

        debug!("Starting removing outdated cache entries (removing non existent files from cache - {delete_outdated_cache})");
        let initial_number_of_entries = vec_loaded_entries.len();
        vec_loaded_entries = vec_loaded_entries
            .into_par_iter()
            .filter(|file_entry| {
                if !check_func(file_entry) {
                    return false;
                }

                if delete_outdated_cache && !file_entry.get_path().exists() {
                    return false;
                }

                true
            })
            .collect();
        debug!(
            "Completed removing outdated cache entries, removed {} out of all {} entries",
            initial_number_of_entries - vec_loaded_entries.len(),
            initial_number_of_entries
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
