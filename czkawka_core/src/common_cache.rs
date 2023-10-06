use crate::common;
use crate::common_messages::Messages;
use crate::common_traits::ResultEntry;
use crate::similar_images::{convert_algorithm_to_string, convert_filters_to_string};
use image::imageops::FilterType;
use image_hasher::HashAlg;
use log::debug;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::io::{BufReader, BufWriter};

pub fn get_broken_files_cache_file() -> String {
    "cache_broken_files_61.bin".to_string()
}

pub fn get_similar_images_cache_file(hash_size: &u8, hash_alg: &HashAlg, image_filter: &FilterType) -> String {
    format!(
        "cache_similar_images_{}_{}_{}_61.bin",
        hash_size,
        convert_algorithm_to_string(hash_alg),
        convert_filters_to_string(image_filter),
    )
}

pub fn get_similar_videos_cache_file() -> String {
    "cache_similar_videos_61.bin".to_string()
}
pub fn get_similar_music_cache_file(checking_tags: bool) -> &'static str {
    if checking_tags {
        "cache_same_music_tags_61.bin"
    } else {
        "cache_same_music_fingerprints_61.bin"
    }
}

pub fn save_cache_to_file_generalized<T>(cache_file_name: &str, hashmap: &BTreeMap<String, T>, save_also_as_json: bool) -> Messages
where
    T: Serialize + ResultEntry + Sized + Send + Sync,
{
    debug!("Saving cache to file {} (or also json alternative) - {} results", cache_file_name, hashmap.len());
    let mut text_messages = Messages::new();
    if let Some(((file_handler, cache_file), (file_handler_json, cache_file_json))) =
        common::open_cache_folder(cache_file_name, true, save_also_as_json, &mut text_messages.warnings)
    {
        {
            let writer = BufWriter::new(file_handler.unwrap()); // Unwrap because cannot fail here
            if let Err(e) = bincode::serialize_into(writer, &hashmap.values().collect::<Vec<_>>()) {
                text_messages
                    .warnings
                    .push(format!("Cannot write data to cache file {}, reason {}", cache_file.display(), e));
                debug!("Failed to save cache to file {:?}", cache_file);
                return text_messages;
            }
            debug!("Saved binary to file {:?}", cache_file);
        }
        if save_also_as_json {
            if let Some(file_handler_json) = file_handler_json {
                let writer = BufWriter::new(file_handler_json);
                if let Err(e) = serde_json::to_writer(writer, &hashmap.values().collect::<Vec<_>>()) {
                    text_messages
                        .warnings
                        .push(format!("Cannot write data to cache file {}, reason {}", cache_file_json.display(), e));
                    debug!("Failed to save cache to file {:?}", cache_file_json);
                    return text_messages;
                }
                debug!("Saved json to file {:?}", cache_file_json);
            }
        }

        text_messages.messages.push(format!("Properly saved to file {} cache entries.", hashmap.len()));
    } else {
        debug!("Failed to save cache to file {cache_file_name} because not exists");
    }
    text_messages
}

pub fn load_cache_from_file_generalized<T>(cache_file_name: &str, delete_outdated_cache: bool, used_files: &BTreeMap<String, T>) -> (Messages, Option<BTreeMap<String, T>>)
where
    for<'a> T: Deserialize<'a> + ResultEntry + Sized + Send + Sync,
{
    debug!("Loading cache from file {} (or json alternative)", cache_file_name);
    let mut text_messages = Messages::new();

    if let Some(((file_handler, cache_file), (file_handler_json, cache_file_json))) = common::open_cache_folder(cache_file_name, false, true, &mut text_messages.warnings) {
        let mut vec_loaded_entries: Vec<T>;
        if let Some(file_handler) = file_handler {
            let reader = BufReader::new(file_handler);

            vec_loaded_entries = match bincode::deserialize_from(reader) {
                Ok(t) => t,
                Err(e) => {
                    text_messages
                        .warnings
                        .push(format!("Failed to load data from cache file {}, reason {}", cache_file.display(), e));
                    debug!("Failed to load cache from file {:?}", cache_file);
                    return (text_messages, None);
                }
            };
        } else {
            let reader = BufReader::new(file_handler_json.unwrap()); // Unwrap cannot fail, because at least one file must be valid
            vec_loaded_entries = match serde_json::from_reader(reader) {
                Ok(t) => t,
                Err(e) => {
                    text_messages
                        .warnings
                        .push(format!("Failed to load data from cache file {}, reason {}", cache_file_json.display(), e));
                    debug!("Failed to load cache from file {:?}", cache_file);
                    return (text_messages, None);
                }
            };
        }

        // Don't load cache data if destination file not exists
        if delete_outdated_cache {
            debug!("Starting to removing outdated cache entries");
            let initial_number_of_entries = vec_loaded_entries.len();
            vec_loaded_entries = vec_loaded_entries
                .into_par_iter()
                .filter(|file_entry| {
                    if delete_outdated_cache && !file_entry.get_path().exists() {
                        return false;
                    }

                    let file_entry_path_str = file_entry.get_path().to_string_lossy().to_string();
                    if let Some(used_file) = used_files.get(&file_entry_path_str) {
                        if file_entry.get_size() != used_file.get_size() {
                            return false;
                        }
                        if file_entry.get_modified_date() != used_file.get_modified_date() {
                            return false;
                        }
                    }

                    true
                })
                .collect();
            debug!(
                "Completed removing outdated cache entries, removed {} out of all {} entries",
                initial_number_of_entries - vec_loaded_entries.len(),
                initial_number_of_entries
            );
        }

        text_messages.messages.push(format!("Properly loaded {} cache entries.", vec_loaded_entries.len()));

        let map_loaded_entries: BTreeMap<_, _> = vec_loaded_entries
            .into_iter()
            .map(|file_entry| (file_entry.get_path().to_string_lossy().into_owned(), file_entry))
            .collect();
        debug!("Loaded cache from file {cache_file_name} (or json alternative) - {} results", map_loaded_entries.len());
        return (text_messages, Some(map_loaded_entries));
    }
    debug!("Failed to load cache from file {cache_file_name} because not exists");
    (text_messages, None)
}
