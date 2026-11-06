use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use crate::common::cache::{load_cache_from_file_generalized_by_path, save_cache_to_file_generalized};
use crate::common::dir_traversal::get_modified_time;
use crate::common::model::HashType;
use crate::helpers::messages::Messages;
use crate::tools::duplicate::DuplicateEntry;
use crate::tools::duplicate::core::get_duplicate_cache_file;

/// Smallest file whose new hash is stored; the same default as Duplicates, because tiny files are cheaper to hash than to look up.
const MINIMAL_CACHED_FILE_SIZE: u64 = 257_144;

#[derive(Debug, Clone, Copy)]
pub(super) struct CacheSettings {
    pub enabled: bool,
    pub delete_outdated: bool,
    pub save_also_as_json: bool,
}

/// Full hashes shared with the Duplicates tool, which keeps them in the same file in the same format.
pub(super) struct HashCache {
    settings: CacheSettings,
    file_name: String,
    loaded: BTreeMap<String, DuplicateEntry>,
    /// Size and modification date of every file that is looked up, the key of a valid cache entry.
    looked_up: BTreeMap<String, DuplicateEntry>,
}

impl HashCache {
    pub fn load(settings: CacheSettings, hash_type: HashType, files: &[(PathBuf, u64)], messages: &mut Messages) -> Self {
        let file_name = get_duplicate_cache_file(hash_type, false);
        let mut cache = Self {
            settings,
            file_name,
            loaded: BTreeMap::new(),
            looked_up: BTreeMap::new(),
        };
        if !settings.enabled {
            return cache;
        }
        cache.looked_up = files
            .iter()
            .map(|(path, size)| (path_key(path), describe_file(path, *size, &mut messages.warnings)))
            .collect();
        let (load_messages, loaded) = load_cache_from_file_generalized_by_path(&cache.file_name, settings.delete_outdated, &cache.looked_up);
        messages.extend_with_another_messages(load_messages);
        cache.loaded = loaded.unwrap_or_default();
        cache
    }

    pub fn cached_hash(&self, path: &Path) -> Option<&str> {
        self.loaded.get(&path_key(path)).map(|entry| entry.hash.as_str())
    }

    /// Adds the newly computed hashes to what was loaded, so entries of other tools and runs are kept.
    pub fn save(mut self, computed: &[(PathBuf, u64, String)], messages: &mut Messages) {
        let mut any_new = false;
        for (path, size, hash) in computed.iter().filter(|(_, size, _)| *size >= MINIMAL_CACHED_FILE_SIZE) {
            let key = path_key(path);
            let modified_date = self.looked_up.get(&key).map_or(0, |entry| entry.modified_date);
            self.loaded.insert(
                key,
                DuplicateEntry {
                    path: path.clone(),
                    modified_date,
                    size: *size,
                    hash: hash.clone(),
                },
            );
            any_new = true;
        }
        if self.settings.enabled && any_new {
            messages.extend_with_another_messages(save_cache_to_file_generalized(&self.file_name, &self.loaded, self.settings.save_also_as_json, 0));
        }
    }
}

fn path_key(path: &Path) -> String {
    path.to_string_lossy().into_owned()
}

fn describe_file(path: &Path, size: u64, warnings: &mut Vec<String>) -> DuplicateEntry {
    let modified_date = std::fs::metadata(path).map_or(0, |metadata| get_modified_time(&metadata, warnings, path, false));
    DuplicateEntry {
        path: path.to_path_buf(),
        modified_date,
        size,
        hash: String::new(),
    }
}
