use std::collections::BTreeMap;
use std::io::{BufReader, BufWriter};

use fun_time::fun_time;
use image::imageops::FilterType;
use image_hasher::HashAlg;
use log::debug;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use serde::{Deserialize, Serialize};

use crate::common;
use crate::common_messages::Messages;
use crate::common_traits::ResultEntry;
use crate::duplicate::HashType;
use crate::similar_images::{convert_algorithm_to_string, convert_filters_to_string};

const CACHE_VERSION: &str = "70";

pub fn get_broken_files_cache_file() -> String {
    format!("cache_broken_files_{CACHE_VERSION}.bin")
}

pub fn get_similar_images_cache_file(hash_size: &u8, hash_alg: &HashAlg, image_filter: &FilterType) -> String {
    format!(
        "cache_similar_images_{hash_size}_{}_{}_{CACHE_VERSION}.bin",
        convert_algorithm_to_string(hash_alg),
        convert_filters_to_string(image_filter),
    )
}

pub fn get_similar_videos_cache_file() -> String {
    format!("cache_similar_videos_{CACHE_VERSION}.bin")
}
pub fn get_similar_music_cache_file(checking_tags: bool) -> String {
    if checking_tags {
        format!("cache_same_music_tags_{CACHE_VERSION}.bin")
    } else {
        format!("cache_same_music_fingerprints_{CACHE_VERSION}.bin")
    }
}

pub fn get_duplicate_cache_file(type_of_hash: &HashType, is_prehash: bool) -> String {
    let prehash_str = if is_prehash { "_prehash" } else { "" };
    format!("cache_duplicates_{type_of_hash:?}{prehash_str}_{CACHE_VERSION}.bin")
}

#[fun_time(message = "save_cache_to_file_generalized", level = "debug")]
pub fn save_cache_to_file_generalized<T>(cache_file_name: &str, hashmap: &BTreeMap<String, T>, save_also_as_json: bool, minimum_file_size: u64) -> Messages
where
    T: Serialize + ResultEntry + Sized + Send + Sync,
{
    let mut text_messages = Messages::new();
    if let Some(((file_handler, cache_file), (file_handler_json, cache_file_json))) =
        common::open_cache_folder(cache_file_name, true, save_also_as_json, &mut text_messages.warnings)
    {
        let hashmap_to_save = hashmap.values().filter(|t| t.get_size() >= minimum_file_size).collect::<Vec<_>>();

        {
            let writer = BufWriter::new(file_handler.expect("Cannot fail, because for saving, this always exists"));
            if let Err(e) = bincode::serialize_into(writer, &hashmap_to_save) {
                text_messages.warnings.push(format!("Cannot write data to cache file {cache_file:?}, reason {e}"));
                debug!("Failed to save cache to file {cache_file:?}");
                return text_messages;
            }
            debug!("Saved binary to file {cache_file:?}");
        }
        if save_also_as_json {
            if let Some(file_handler_json) = file_handler_json {
                let writer = BufWriter::new(file_handler_json);
                if let Err(e) = serde_json::to_writer(writer, &hashmap_to_save) {
                    text_messages.warnings.push(format!("Cannot write data to cache file {cache_file_json:?}, reason {e}"));
                    debug!("Failed to save cache to file {cache_file_json:?}");
                    return text_messages;
                }
                debug!("Saved json to file {cache_file_json:?}");
            }
        }

        text_messages.messages.push(format!("Properly saved to file {} cache entries.", hashmap.len()));
        debug!("Properly saved to file {} cache entries.", hashmap.len());
    } else {
        debug!("Failed to save cache to file {cache_file_name} because not exists");
    }
    text_messages
}

#[fun_time(message = "load_cache_from_file_generalized_by_path", level = "debug")]
pub fn load_cache_from_file_generalized_by_path<T>(cache_file_name: &str, delete_outdated_cache: bool, used_files: &BTreeMap<String, T>) -> (Messages, Option<BTreeMap<String, T>>)
where
    for<'a> T: Deserialize<'a> + ResultEntry + Sized + Send + Sync + Clone,
{
    let (text_messages, vec_loaded_cache) = load_cache_from_file_generalized(cache_file_name, delete_outdated_cache, used_files);
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
    debug!("Converting cache BtreeMap<u64, Vec<T>> into BTreeMap<String, T>");
    let mut used_files: BTreeMap<String, T> = Default::default();
    for file_entry in cache_not_converted.values().flatten() {
        used_files.insert(file_entry.get_path().to_string_lossy().into_owned(), file_entry.clone());
    }
    debug!("Converted cache BtreeMap<u64, Vec<T>> into BTreeMap<String, T>");

    let (text_messages, vec_loaded_cache) = load_cache_from_file_generalized(cache_file_name, delete_outdated_cache, &used_files);
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

#[fun_time(message = "load_cache_from_file_generalized_by_path_from_size", level = "debug")]
pub fn load_cache_from_file_generalized_by_path_from_size<T>(
    cache_file_name: &str,
    delete_outdated_cache: bool,
    cache_not_converted: &BTreeMap<u64, Vec<T>>,
) -> (Messages, Option<BTreeMap<String, T>>)
where
    for<'a> T: Deserialize<'a> + ResultEntry + Sized + Send + Sync + Clone,
{
    debug!("Converting cache BtreeMap<u64, Vec<T>> into BTreeMap<String, T>");
    let mut used_files: BTreeMap<String, T> = Default::default();
    for file_entry in cache_not_converted.values().flatten() {
        used_files.insert(file_entry.get_path().to_string_lossy().into_owned(), file_entry.clone());
    }
    debug!("Converted cache BtreeMap<u64, Vec<T>> into BTreeMap<String, T>");

    let (text_messages, vec_loaded_cache) = load_cache_from_file_generalized(cache_file_name, delete_outdated_cache, &used_files);
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

#[fun_time(message = "load_cache_from_file_generalized", level = "debug")]
fn load_cache_from_file_generalized<T>(cache_file_name: &str, delete_outdated_cache: bool, used_files: &BTreeMap<String, T>) -> (Messages, Option<Vec<T>>)
where
    for<'a> T: Deserialize<'a> + ResultEntry + Sized + Send + Sync + Clone,
{
    let mut text_messages = Messages::new();

    if let Some(((file_handler, cache_file), (file_handler_json, cache_file_json))) = common::open_cache_folder(cache_file_name, false, true, &mut text_messages.warnings) {
        let mut vec_loaded_entries: Vec<T>;
        if let Some(file_handler) = file_handler {
            let reader = BufReader::new(file_handler);

            vec_loaded_entries = match bincode::deserialize_from(reader) {
                Ok(t) => t,
                Err(e) => {
                    text_messages.warnings.push(format!("Failed to load data from cache file {cache_file:?}, reason {e}"));
                    debug!("Failed to load cache from file {cache_file:?}");
                    return (text_messages, None);
                }
            };
        } else {
            let reader = BufReader::new(file_handler_json.expect("This cannot fail, because if file_handler is None, then this cannot be None"));
            vec_loaded_entries = match serde_json::from_reader(reader) {
                Ok(t) => t,
                Err(e) => {
                    text_messages.warnings.push(format!("Failed to load data from cache file {cache_file_json:?}, reason {e}"));
                    debug!("Failed to load cache from file {cache_file:?}");
                    return (text_messages, None);
                }
            };
        }

        debug!(
            "Starting removing outdated cache entries (removing non existent files from cache - {})",
            delete_outdated_cache
        );
        let initial_number_of_entries = vec_loaded_entries.len();
        vec_loaded_entries = vec_loaded_entries
            .into_par_iter()
            .filter(|file_entry| {
                let path = file_entry.get_path();

                let file_entry_path_str = path.to_string_lossy().to_string();
                if let Some(used_file) = used_files.get(&file_entry_path_str) {
                    if file_entry.get_size() != used_file.get_size() {
                        return false;
                    }
                    if file_entry.get_modified_date() != used_file.get_modified_date() {
                        return false;
                    }
                }

                if delete_outdated_cache && !path.exists() {
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

        debug!("Loaded cache from file {cache_file_name} (or json alternative) - {} results", vec_loaded_entries.len());
        return (text_messages, Some(vec_loaded_entries));
    }
    debug!("Failed to load cache from file {cache_file_name} because not exists");
    (text_messages, None)
}
