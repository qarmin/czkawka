use std::collections::{BTreeSet, HashMap, HashSet};
use std::fs::{File, Metadata};
use std::io::Write;
use std::io::*;
use std::panic;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread::sleep;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::{fs, mem, thread};

use bk_tree::BKTree;
use crossbeam_channel::Receiver;
use humansize::format_size;
use humansize::BINARY;
use image::GenericImageView;
use image_hasher::{FilterType, HashAlg, HasherConfig};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};

#[cfg(feature = "heif")]
use crate::common::get_dynamic_image_from_heic;
use crate::common::{
    create_crash_message, get_dynamic_image_from_raw_image, get_number_of_threads, open_cache_folder, Common, HEIC_EXTENSIONS, IMAGE_RS_SIMILAR_IMAGES_EXTENSIONS, LOOP_DURATION,
    RAW_IMAGE_EXTENSIONS,
};
use crate::common_directory::Directories;
use crate::common_extensions::Extensions;
use crate::common_items::ExcludedItems;
use crate::common_messages::Messages;
use crate::common_traits::{DebugPrint, PrintResults, SaveResults};
use crate::flc;
use crate::localizer_core::generate_translation_hashmap;

pub const SIMILAR_VALUES: [[u32; 6]; 4] = [
    [1, 2, 5, 7, 14, 20],    // 8
    [2, 5, 15, 30, 40, 40],  // 16
    [4, 10, 20, 40, 40, 40], // 32
    [6, 20, 40, 40, 40, 40], // 64
];

#[derive(Debug)]
pub struct ProgressData {
    pub current_stage: u8,
    pub max_stage: u8,
    pub images_checked: usize,
    pub images_to_check: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FileEntry {
    pub path: PathBuf,
    pub size: u64,
    pub dimensions: String,
    pub modified_date: u64,
    pub hash: Vec<u8>,
    pub similarity: u32,
}

/// Used by CLI tool when we cannot use directly values
#[derive(Clone, Debug, Copy)]
pub enum SimilarityPreset {
    Original,
    VeryHigh,
    High,
    Medium,
    Small,
    VerySmall,
    Minimal,
    None,
}

/// Distance metric to use with the BK-tree.
struct Hamming;

impl bk_tree::Metric<Vec<u8>> for Hamming {
    fn distance(&self, a: &Vec<u8>, b: &Vec<u8>) -> u32 {
        hamming::distance_fast(a, b).unwrap() as u32
    }

    fn threshold_distance(&self, a: &Vec<u8>, b: &Vec<u8>, _threshold: u32) -> Option<u32> {
        Some(self.distance(a, b))
    }
}

/// Struct to store most basics info about all folder
pub struct SimilarImages {
    information: Info,
    text_messages: Messages,
    directories: Directories,
    allowed_extensions: Extensions,
    excluded_items: ExcludedItems,
    bktree: BKTree<Vec<u8>, Hamming>,
    similar_vectors: Vec<Vec<FileEntry>>,
    similar_referenced_vectors: Vec<(FileEntry, Vec<FileEntry>)>,
    recursive_search: bool,
    minimal_file_size: u64,
    maximal_file_size: u64,
    image_hashes: HashMap<Vec<u8>, Vec<FileEntry>>, // Hashmap with image hashes and Vector with names of files
    stopped_search: bool,
    similarity: u32,
    images_to_check: HashMap<String, FileEntry>,
    hash_size: u8,
    hash_alg: HashAlg,
    image_filter: FilterType,
    use_cache: bool,
    delete_outdated_cache: bool,
    exclude_images_with_same_size: bool,
    use_reference_folders: bool,
    save_also_as_json: bool,
}

/// Info struck with helpful information's about results
#[derive(Default)]
pub struct Info {
    pub number_of_duplicates: usize,
    pub number_of_groups: u64,
}

impl Info {
    #[must_use]
    pub fn new() -> Self {
        Default::default()
    }
}

/// Method implementation for `EmptyFolder`
impl SimilarImages {
    /// New function providing basics values
    #[must_use]
    pub fn new() -> Self {
        Self {
            information: Default::default(),
            text_messages: Messages::new(),
            directories: Directories::new(),
            excluded_items: Default::default(),
            allowed_extensions: Extensions::new(),
            bktree: BKTree::new(Hamming),
            similar_vectors: vec![],
            similar_referenced_vectors: Default::default(),
            recursive_search: true,
            minimal_file_size: 1024 * 16, // 16 KB should be enough to exclude too small images from search
            maximal_file_size: u64::MAX,
            image_hashes: Default::default(),
            stopped_search: false,
            similarity: 0,
            images_to_check: Default::default(),
            hash_size: 8,
            hash_alg: HashAlg::Gradient,
            image_filter: FilterType::Lanczos3,
            use_cache: true,
            delete_outdated_cache: true,
            exclude_images_with_same_size: false,
            use_reference_folders: false,
            save_also_as_json: false,
        }
    }

    pub fn set_hash_size(&mut self, hash_size: u8) {
        self.hash_size = match hash_size {
            8 | 16 | 32 | 64 => hash_size,
            e => {
                panic!("Invalid value of hash size {e}");
            }
        }
    }

    pub fn set_delete_outdated_cache(&mut self, delete_outdated_cache: bool) {
        self.delete_outdated_cache = delete_outdated_cache;
    }

    pub fn set_exclude_images_with_same_size(&mut self, exclude_images_with_same_size: bool) {
        self.exclude_images_with_same_size = exclude_images_with_same_size;
    }

    pub fn set_hash_alg(&mut self, hash_alg: HashAlg) {
        self.hash_alg = hash_alg;
    }

    pub fn set_image_filter(&mut self, image_filter: FilterType) {
        self.image_filter = image_filter;
    }

    pub fn set_save_also_as_json(&mut self, save_also_as_json: bool) {
        self.save_also_as_json = save_also_as_json;
    }

    #[must_use]
    pub fn get_stopped_search(&self) -> bool {
        self.stopped_search
    }

    #[must_use]
    pub const fn get_text_messages(&self) -> &Messages {
        &self.text_messages
    }

    #[must_use]
    pub const fn get_similar_images(&self) -> &Vec<Vec<FileEntry>> {
        &self.similar_vectors
    }

    #[must_use]
    pub fn get_similar_images_referenced(&self) -> &Vec<(FileEntry, Vec<FileEntry>)> {
        &self.similar_referenced_vectors
    }

    #[must_use]
    pub fn get_use_reference(&self) -> bool {
        self.use_reference_folders
    }

    #[must_use]
    pub const fn get_information(&self) -> &Info {
        &self.information
    }

    pub fn set_use_cache(&mut self, use_cache: bool) {
        self.use_cache = use_cache;
    }

    pub fn set_recursive_search(&mut self, recursive_search: bool) {
        self.recursive_search = recursive_search;
    }

    #[cfg(target_family = "unix")]
    pub fn set_exclude_other_filesystems(&mut self, exclude_other_filesystems: bool) {
        self.directories.set_exclude_other_filesystems(exclude_other_filesystems);
    }
    #[cfg(not(target_family = "unix"))]
    pub fn set_exclude_other_filesystems(&mut self, _exclude_other_filesystems: bool) {}

    pub fn set_allowed_extensions(&mut self, allowed_extensions: String) {
        self.allowed_extensions.set_allowed_extensions(allowed_extensions, &mut self.text_messages);
    }

    pub fn set_minimal_file_size(&mut self, minimal_file_size: u64) {
        self.minimal_file_size = match minimal_file_size {
            0 => 1,
            t => t,
        };
    }
    pub fn set_maximal_file_size(&mut self, maximal_file_size: u64) {
        self.maximal_file_size = match maximal_file_size {
            0 => 1,
            t => t,
        };
    }
    pub fn set_similarity(&mut self, similarity: u32) {
        self.similarity = similarity;
    }

    /// Public function used by CLI to search for empty folders
    pub fn find_similar_images(&mut self, stop_receiver: Option<&Receiver<()>>, progress_sender: Option<&futures::channel::mpsc::UnboundedSender<ProgressData>>) {
        self.directories.optimize_directories(true, &mut self.text_messages);
        self.use_reference_folders = !self.directories.reference_directories.is_empty();
        if !self.check_for_similar_images(stop_receiver, progress_sender) {
            self.stopped_search = true;
            return;
        }
        if !self.hash_images(stop_receiver, progress_sender) {
            self.stopped_search = true;
            return;
        }
        if !self.find_similar_hashes(stop_receiver, progress_sender) {
            self.stopped_search = true;
            return;
        }
        // if self.delete_folders {
        //     self.delete_empty_folders();
        // }
        self.debug_print();
    }

    // pub fn set_delete_folder(&mut self, delete_folder: bool) {
    //     self.delete_folders = delete_folder;
    // }

    /// Function to check if folder are empty.
    /// Parameter `initial_checking` for second check before deleting to be sure that checked folder is still empty
    fn check_for_similar_images(&mut self, stop_receiver: Option<&Receiver<()>>, progress_sender: Option<&futures::channel::mpsc::UnboundedSender<ProgressData>>) -> bool {
        let start_time: SystemTime = SystemTime::now();
        let mut folders_to_check: Vec<PathBuf> = Vec::with_capacity(1024 * 2); // This should be small enough too not see to big difference and big enough to store most of paths without needing to resize vector

        if !self.allowed_extensions.using_custom_extensions() {
            self.allowed_extensions.extend_allowed_extensions(IMAGE_RS_SIMILAR_IMAGES_EXTENSIONS);
            self.allowed_extensions.extend_allowed_extensions(RAW_IMAGE_EXTENSIONS);
            #[cfg(feature = "heif")]
            self.allowed_extensions.extend_allowed_extensions(HEIC_EXTENSIONS);
        } else {
            self.allowed_extensions
                .validate_allowed_extensions(&[IMAGE_RS_SIMILAR_IMAGES_EXTENSIONS, RAW_IMAGE_EXTENSIONS, HEIC_EXTENSIONS].concat());
            if !self.allowed_extensions.using_custom_extensions() {
                return true;
            }
        }

        // Add root folders for finding
        for id in &self.directories.included_directories {
            folders_to_check.push(id.clone());
        }

        //// PROGRESS THREAD START
        let progress_thread_run = Arc::new(AtomicBool::new(true));

        let atomic_file_counter = Arc::new(AtomicUsize::new(0));

        let progress_thread_handle = if let Some(progress_sender) = progress_sender {
            let progress_send = progress_sender.clone();
            let progress_thread_run = progress_thread_run.clone();
            let atomic_file_counter = atomic_file_counter.clone();
            thread::spawn(move || loop {
                progress_send
                    .unbounded_send(ProgressData {
                        current_stage: 0,
                        max_stage: 3,
                        images_checked: atomic_file_counter.load(Ordering::Relaxed),
                        images_to_check: 0,
                    })
                    .unwrap();
                if !progress_thread_run.load(Ordering::Relaxed) {
                    break;
                }
                sleep(Duration::from_millis(LOOP_DURATION as u64));
            })
        } else {
            thread::spawn(|| {})
        };
        //// PROGRESS THREAD END

        while !folders_to_check.is_empty() {
            if stop_receiver.is_some() && stop_receiver.unwrap().try_recv().is_ok() {
                // End thread which send info to gui
                progress_thread_run.store(false, Ordering::Relaxed);
                progress_thread_handle.join().unwrap();
                return false;
            }

            let segments: Vec<_> = folders_to_check
                .par_iter()
                .map(|current_folder| {
                    let mut dir_result = vec![];
                    let mut warnings = vec![];
                    let mut fe_result = vec![];
                    // Read current dir children
                    let read_dir = match fs::read_dir(current_folder) {
                        Ok(t) => t,
                        Err(e) => {
                            warnings.push(flc!(
                                "core_cannot_open_dir",
                                generate_translation_hashmap(vec![("dir", current_folder.display().to_string()), ("reason", e.to_string())])
                            ));
                            return (dir_result, warnings, fe_result);
                        }
                    };

                    // Check every sub folder/file/link etc.
                    'dir: for entry in read_dir {
                        let entry_data = match entry {
                            Ok(t) => t,
                            Err(e) => {
                                warnings.push(flc!(
                                    "core_cannot_read_entry_dir",
                                    generate_translation_hashmap(vec![("dir", current_folder.display().to_string()), ("reason", e.to_string())])
                                ));
                                continue 'dir;
                            }
                        };
                        let metadata: Metadata = match entry_data.metadata() {
                            Ok(t) => t,
                            Err(e) => {
                                warnings.push(flc!(
                                    "core_cannot_read_metadata_dir",
                                    generate_translation_hashmap(vec![("dir", current_folder.display().to_string()), ("reason", e.to_string())])
                                ));
                                continue 'dir;
                            }
                        };
                        if metadata.is_dir() {
                            if !self.recursive_search {
                                continue 'dir;
                            }

                            let next_folder = current_folder.join(entry_data.file_name());
                            if self.directories.is_excluded(&next_folder) {
                                continue 'dir;
                            }

                            if self.excluded_items.is_excluded(&next_folder) {
                                continue 'dir;
                            }

                            #[cfg(target_family = "unix")]
                            if self.directories.exclude_other_filesystems() {
                                match self.directories.is_on_other_filesystems(&next_folder) {
                                    Ok(true) => continue 'dir,
                                    Err(e) => warnings.push(e.to_string()),
                                    _ => (),
                                }
                            }

                            dir_result.push(next_folder);
                        } else if metadata.is_file() {
                            atomic_file_counter.fetch_add(1, Ordering::Relaxed);

                            let file_name_lowercase: String = match entry_data.file_name().into_string() {
                                Ok(t) => t,
                                Err(_inspected) => {
                                    warnings.push(flc!(
                                        "core_file_not_utf8_name",
                                        generate_translation_hashmap(vec![("name", entry_data.path().display().to_string())])
                                    ));
                                    continue 'dir;
                                }
                            }
                            .to_lowercase();

                            if !self.allowed_extensions.matches_filename(&file_name_lowercase) {
                                continue 'dir;
                            }

                            // Checking files
                            if (self.minimal_file_size..=self.maximal_file_size).contains(&metadata.len()) {
                                let current_file_name = current_folder.join(entry_data.file_name());
                                if self.excluded_items.is_excluded(&current_file_name) {
                                    continue 'dir;
                                }

                                let fe: FileEntry = FileEntry {
                                    path: current_file_name.clone(),
                                    size: metadata.len(),
                                    dimensions: String::new(),
                                    modified_date: match metadata.modified() {
                                        Ok(t) => match t.duration_since(UNIX_EPOCH) {
                                            Ok(d) => d.as_secs(),
                                            Err(_inspected) => {
                                                warnings.push(flc!(
                                                    "core_file_modified_before_epoch",
                                                    generate_translation_hashmap(vec![("name", current_file_name.display().to_string())])
                                                ));
                                                0
                                            }
                                        },
                                        Err(e) => {
                                            warnings.push(flc!(
                                                "core_file_no_modification_date",
                                                generate_translation_hashmap(vec![("name", current_file_name.display().to_string()), ("reason", e.to_string())])
                                            ));
                                            0
                                        }
                                    },

                                    hash: Vec::new(),
                                    similarity: 0,
                                };

                                fe_result.push((current_file_name.to_string_lossy().to_string(), fe));
                            }
                        }
                    }
                    (dir_result, warnings, fe_result)
                })
                .collect();

            // Advance the frontier
            folders_to_check.clear();

            // Process collected data
            for (segment, warnings, fe_result) in segments {
                folders_to_check.extend(segment);
                self.text_messages.warnings.extend(warnings);
                for (name, fe) in fe_result {
                    self.images_to_check.insert(name, fe);
                }
            }
        }

        // End thread which send info to gui
        progress_thread_run.store(false, Ordering::Relaxed);
        progress_thread_handle.join().unwrap();
        Common::print_time(start_time, SystemTime::now(), "check_for_similar_images");
        true
    }

    // Cache algorithm:
    // - Load data from file
    // - Remove from data to search, already loaded entries from cache(size and modified datamust match)
    // - Check hash of files which doesn't have saved entry
    // - Join already read hashes with hashes which were read from file
    // - Join all hashes and save it to file

    fn hash_images(&mut self, stop_receiver: Option<&Receiver<()>>, progress_sender: Option<&futures::channel::mpsc::UnboundedSender<ProgressData>>) -> bool {
        let hash_map_modification = SystemTime::now();

        let loaded_hash_map;

        let mut records_already_cached: HashMap<String, FileEntry> = Default::default();
        let mut non_cached_files_to_check: HashMap<String, FileEntry> = Default::default();

        if self.use_cache {
            loaded_hash_map = match load_hashes_from_file(&mut self.text_messages, self.delete_outdated_cache, self.hash_size, self.hash_alg, self.image_filter) {
                Some(t) => t,
                None => Default::default(),
            };

            for (name, file_entry) in &self.images_to_check {
                #[allow(clippy::if_same_then_else)]
                if !loaded_hash_map.contains_key(name) {
                    // If loaded data doesn't contains current image info
                    non_cached_files_to_check.insert(name.clone(), file_entry.clone());
                } else if file_entry.size != loaded_hash_map.get(name).unwrap().size || file_entry.modified_date != loaded_hash_map.get(name).unwrap().modified_date {
                    // When size or modification date of image changed, then it is clear that is different image
                    non_cached_files_to_check.insert(name.clone(), file_entry.clone());
                } else {
                    // Checking may be omitted when already there is entry with same size and modification date
                    records_already_cached.insert(name.clone(), loaded_hash_map.get(name).unwrap().clone());
                }
            }
        } else {
            loaded_hash_map = Default::default();
            mem::swap(&mut self.images_to_check, &mut non_cached_files_to_check);
        }

        Common::print_time(hash_map_modification, SystemTime::now(), "sort_images - reading data from cache and preparing them");
        let hash_map_modification = SystemTime::now();

        //// PROGRESS THREAD START
        let check_was_stopped = AtomicBool::new(false); // Used for breaking from GUI and ending check thread
        let progress_thread_run = Arc::new(AtomicBool::new(true));

        let atomic_file_counter = Arc::new(AtomicUsize::new(0));

        let progress_thread_handle = if let Some(progress_sender) = progress_sender {
            let progress_send = progress_sender.clone();
            let progress_thread_run = progress_thread_run.clone();
            let atomic_file_counter = atomic_file_counter.clone();
            let images_to_check = non_cached_files_to_check.len();
            thread::spawn(move || loop {
                progress_send
                    .unbounded_send(ProgressData {
                        current_stage: 1,
                        max_stage: 3,
                        images_checked: atomic_file_counter.load(Ordering::Relaxed),
                        images_to_check,
                    })
                    .unwrap();
                if !progress_thread_run.load(Ordering::Relaxed) {
                    break;
                }
                sleep(Duration::from_millis(LOOP_DURATION as u64));
            })
        } else {
            thread::spawn(|| {})
        };

        //// PROGRESS THREAD END
        let mut vec_file_entry: Vec<(FileEntry, Vec<u8>)> = non_cached_files_to_check
            .into_par_iter()
            .map(|(_s, mut file_entry)| {
                atomic_file_counter.fetch_add(1, Ordering::Relaxed);
                if stop_receiver.is_some() && stop_receiver.unwrap().try_recv().is_ok() {
                    check_was_stopped.store(true, Ordering::Relaxed);
                    return None;
                }
                let file_name_lowercase = file_entry.path.to_string_lossy().to_lowercase();

                let image;

                #[allow(clippy::never_loop)] // Required to implement nice if/else
                'krztyna: loop {
                    if RAW_IMAGE_EXTENSIONS.iter().any(|e| file_name_lowercase.ends_with(e)) {
                        image = match get_dynamic_image_from_raw_image(&file_entry.path) {
                            Some(t) => t,
                            None => return Some(Some((file_entry, Vec::new()))),
                        };
                        break 'krztyna;
                    }

                    #[cfg(feature = "heif")]
                    if HEIC_EXTENSIONS.iter().any(|e| file_name_lowercase.ends_with(e)) {
                        image = match get_dynamic_image_from_heic(&file_entry.path.to_string_lossy()) {
                            Ok(t) => t,
                            Err(_) => {
                                return Some(Some((file_entry, Vec::new())));
                            }
                        };
                        break 'krztyna;
                    }

                    // Normal image extension, when any other fail, not using if/else
                    let result = panic::catch_unwind(|| {
                        match image::open(file_entry.path.clone()) {
                            Ok(t) => Ok(t),
                            // Err(_inspected) => return Some(None), // Something is wrong with image,
                            // For broken images empty hash is used, because without it will try to resecan files each time when it is called(missing cache file is responsible for it)
                            // This may cause problems(very rarely), when e.g. file was not available due lack of permissions, but it is available now
                            Err(_inspected) => Err(()),
                        }
                    });

                    // If image crashed during opening, we just skip checking its hash and go on
                    if let Ok(image_result) = result {
                        if let Ok(image2) = image_result {
                            image = image2;
                        } else {
                            return Some(Some((file_entry, Vec::new())));
                        }
                    } else {
                        let message = create_crash_message("Image-rs", &file_entry.path.to_string_lossy(), "https://github.com/image-rs/image/issues");
                        println!("{message}");
                        return Some(Some((file_entry, Vec::new())));
                    }

                    break 'krztyna;
                }

                let dimensions = image.dimensions();

                file_entry.dimensions = format!("{}x{}", dimensions.0, dimensions.1);

                let hasher_config = HasherConfig::new()
                    .hash_size(self.hash_size as u32, self.hash_size as u32)
                    .hash_alg(self.hash_alg)
                    .resize_filter(self.image_filter);
                let hasher = hasher_config.to_hasher();

                let hash = hasher.hash_image(&image);
                let buf: Vec<u8> = hash.as_bytes().to_vec();

                file_entry.hash = buf.clone();

                Some(Some((file_entry, buf)))
            })
            .while_some()
            .filter(Option::is_some)
            .map(Option::unwrap)
            .collect::<Vec<(FileEntry, Vec<u8>)>>();

        // End thread which send info to gui
        progress_thread_run.store(false, Ordering::Relaxed);
        progress_thread_handle.join().unwrap();

        Common::print_time(hash_map_modification, SystemTime::now(), "sort_images - reading data from files in parallel");
        let hash_map_modification = SystemTime::now();

        // Just connect loaded results with already calculated hashes
        for (_name, file_entry) in records_already_cached {
            vec_file_entry.push((file_entry.clone(), file_entry.hash));
        }

        // All valid entries are used to create bktree used to check for hash similarity
        for (file_entry, buf) in &vec_file_entry {
            // Only use to comparing, non broken hashes(all 0 or 255 hashes means that algorithm fails to decode them because e.g. contains a log of alpha channel)
            if !(buf.is_empty() || buf.iter().all(|e| *e == 0) || buf.iter().all(|e| *e == 255)) {
                self.image_hashes.entry(buf.clone()).or_insert_with(Vec::<FileEntry>::new).push(file_entry.clone());
            }
        }

        if self.use_cache {
            // Must save all results to file, old loaded from file with all currently counted results
            let mut all_results: HashMap<String, FileEntry> = loaded_hash_map;
            for (file_entry, _hash) in vec_file_entry {
                all_results.insert(file_entry.path.to_string_lossy().to_string(), file_entry);
            }
            save_hashes_to_file(
                &all_results,
                &mut self.text_messages,
                self.save_also_as_json,
                self.hash_size,
                self.hash_alg,
                self.image_filter,
            );
        }

        // Break if stop was clicked after saving to cache
        if check_was_stopped.load(Ordering::Relaxed) {
            return false;
        }

        Common::print_time(hash_map_modification, SystemTime::now(), "sort_images - saving data to files");
        true
    }

    fn find_similar_hashes(&mut self, stop_receiver: Option<&Receiver<()>>, progress_sender: Option<&futures::channel::mpsc::UnboundedSender<ProgressData>>) -> bool {
        if self.image_hashes.is_empty() {
            return true;
        }

        let hash_map_modification = SystemTime::now();
        let tolerance = self.similarity;

        // Results
        let mut collected_similar_images: HashMap<Vec<u8>, Vec<FileEntry>> = Default::default();

        let mut all_hashed_images = Default::default();
        mem::swap(&mut all_hashed_images, &mut self.image_hashes);

        let all_hashes: Vec<_> = all_hashed_images.keys().collect();

        // Checking entries with tolerance 0 is really easy and fast, because only entries with same hashes needs to be checked
        if tolerance == 0 {
            for (hash, vec_file_entry) in all_hashed_images.clone() {
                if vec_file_entry.len() >= 2 {
                    collected_similar_images.insert(hash, vec_file_entry);
                }
            }
        } else {
            //// PROGRESS THREAD START
            let check_was_stopped = AtomicBool::new(false); // Used for breaking from GUI and ending check thread
            let progress_thread_run = Arc::new(AtomicBool::new(true));
            let atomic_mode_counter = Arc::new(AtomicUsize::new(0));

            let progress_thread_handle = if let Some(progress_sender) = progress_sender {
                let progress_send = progress_sender.clone();
                let progress_thread_run = progress_thread_run.clone();
                let atomic_mode_counter = atomic_mode_counter.clone();
                let all_combinations_to_check = all_hashes.len();
                thread::spawn(move || loop {
                    progress_send
                        .unbounded_send(ProgressData {
                            current_stage: 2,
                            max_stage: 2,
                            images_checked: atomic_mode_counter.load(Ordering::Relaxed),
                            images_to_check: all_combinations_to_check,
                        })
                        .unwrap();
                    if !progress_thread_run.load(Ordering::Relaxed) {
                        break;
                    }
                    sleep(Duration::from_millis(LOOP_DURATION as u64));
                })
            } else {
                thread::spawn(|| {})
            };
            //// PROGRESS THREAD END

            // Don't use hashes with multiple images in bktree, because they will always be master of group and cannot be find by other hashes
            let mut hashes_with_multiple_images: HashSet<_> = Default::default(); // Fast way to check if hash have multiple images

            let mut files_from_referenced_folders = HashMap::new();
            let mut normal_files = HashMap::new();

            let number_of_processors = get_number_of_threads();
            let chunk_size;
            let mut chunks: Vec<&[&Vec<u8>]>;

            let mut initial_hashes: Vec<&Vec<u8>> = Vec::new();
            let mut additional_chunk_to_check: Vec<&Vec<u8>> = Default::default();

            if self.use_reference_folders {
                let reference_directories = self.directories.reference_directories.clone();
                all_hashed_images.clone().into_iter().for_each(|(hash, vec_file_entry)| {
                    for file_entry in vec_file_entry {
                        if reference_directories.iter().any(|e| file_entry.path.starts_with(e)) {
                            files_from_referenced_folders.entry(hash.clone()).or_insert_with(Vec::new).push(file_entry);
                        } else {
                            normal_files.entry(hash.clone()).or_insert_with(Vec::new).push(file_entry);
                        }
                    }
                });

                for (hash, vec_files) in &normal_files {
                    if vec_files.len() >= 2 {
                        hashes_with_multiple_images.insert(hash);
                    }
                    self.bktree.add(hash.clone());
                }
                for (hash, vec_files) in &files_from_referenced_folders {
                    if vec_files.len() >= 2 {
                        hashes_with_multiple_images.insert(hash);
                    }
                    initial_hashes.push(hash);
                }
                chunk_size = initial_hashes.len() / number_of_processors;

                chunks = if chunk_size > 0 {
                    initial_hashes.chunks(chunk_size).collect::<Vec<_>>()
                } else {
                    vec![&initial_hashes]
                };
            } else {
                for (hash, vec_files) in &all_hashed_images {
                    if vec_files.len() >= 2 {
                        additional_chunk_to_check.push(hash);
                        hashes_with_multiple_images.insert(hash);
                    } else {
                        self.bktree.add(hash.clone());
                    }
                }
                chunk_size = all_hashes.len() / number_of_processors;
                chunks = if chunk_size > 0 {
                    all_hashes.chunks(chunk_size).collect::<Vec<_>>()
                } else {
                    vec![&all_hashes]
                };
                chunks.push(&additional_chunk_to_check);
            }

            let parts: Vec<_> = chunks
                .into_par_iter()
                .map(|hashes_to_check| {
                    let mut hashes_parents: HashMap<&Vec<u8>, u32> = Default::default(); // Hashes used as parent (hash, children_number_of_hash)
                    let mut hashes_similarity: HashMap<&Vec<u8>, (&Vec<u8>, u32)> = Default::default(); // Hashes used as child, (parent_hash, similarity)

                    // Sprawdź czy hash nie jest użyty jako master gdzie indziej
                    // Jeśli tak to przejdź do sprawdzania kolejnego elementu
                    // Zweryfikuj czy sprawdzany element ma rodzica
                    // Jeśli ma to sprawdź czy similarity nowego rodzica jest mniejsze niż starego
                    // // Jeśli tak to zmniejsz ilość dzieci starego rodzica, dodaj ilość dzieci w nowym rodzicu i podmień rekord hashes_similarity
                    // // Jeśli nie to dodaj nowy rekord w hashes_similarity jak i hashes_parents z liczbą dzieci równą 1

                    for (index, hash_to_check) in hashes_to_check.iter().enumerate() {
                        // Don't check for user stop too often
                        // Also don't add too often data to atomic variable
                        const CYCLES_COUNTER: usize = 0b11_1111;
                        if ((index & CYCLES_COUNTER) == CYCLES_COUNTER) && index != 0 {
                            atomic_mode_counter.fetch_add(CYCLES_COUNTER, Ordering::Relaxed);
                            if stop_receiver.is_some() && stop_receiver.unwrap().try_recv().is_ok() {
                                check_was_stopped.store(true, Ordering::Relaxed);
                                return None;
                            }
                        }
                        hashes_parents.insert(hash_to_check, 0);

                        let mut found_items = self
                            .bktree
                            .find(hash_to_check, tolerance)
                            .filter(|(similarity, _hash)| if self.use_reference_folders { true } else { *similarity != 0 })
                            .collect::<Vec<_>>();

                        found_items.sort_unstable_by_key(|f| f.0);

                        for (similarity, compared_hash) in found_items {
                            image_to_check(
                                &mut hashes_parents,
                                &mut hashes_similarity,
                                &hashes_with_multiple_images,
                                hash_to_check,
                                compared_hash,
                                similarity,
                            );
                        }
                    }

                    #[cfg(debug_assertions)]
                    if !self.use_reference_folders {
                        debug_check_for_duplicated_things(&hashes_parents, &hashes_similarity, &all_hashed_images, "BEFORE");
                    }

                    Some((hashes_parents, hashes_similarity))
                })
                .while_some()
                .collect();

            // End thread which send info to gui
            progress_thread_run.store(false, Ordering::Relaxed);
            progress_thread_handle.join().unwrap();

            if check_was_stopped.load(Ordering::Relaxed) {
                return false;
            }

            {
                let mut hashes_parents: HashMap<&Vec<u8>, u32> = Default::default();
                let mut hashes_similarity: HashMap<&Vec<u8>, (&Vec<u8>, u32)> = Default::default();
                let mut iter = parts.into_iter();
                // At start fill arrays with first item
                // Normal algorithm would do exactly same thing, but slower, one record after one
                if let Some((first_hashes_parents, first_hashes_similarity)) = iter.next() {
                    hashes_parents = first_hashes_parents;
                    hashes_similarity = first_hashes_similarity;
                }

                for (partial_hashes_with_parents, partial_hashes_with_similarity) in iter {
                    for (parent_hash, _child_number) in partial_hashes_with_parents {
                        if !hashes_parents.contains_key(parent_hash) && !hashes_similarity.contains_key(parent_hash) {
                            hashes_parents.insert(parent_hash, 0);
                        }
                    }

                    for (hash_to_check, (compared_hash, similarity)) in partial_hashes_with_similarity {
                        image_to_check(
                            &mut hashes_parents,
                            &mut hashes_similarity,
                            &hashes_with_multiple_images,
                            hash_to_check,
                            compared_hash,
                            similarity,
                        );
                    }
                }

                #[cfg(debug_assertions)]
                if !self.use_reference_folders {
                    debug_check_for_duplicated_things(&hashes_parents, &hashes_similarity, &all_hashed_images, "LATTER");
                }

                // Just simple check if all original hashes with multiple entries are available in end results
                let original_hashes_at_start = hashes_with_multiple_images.len();
                let original_hashes_in_end_results = hashes_parents
                    .iter()
                    .filter(|(parent_hash, _child_number)| hashes_with_multiple_images.contains(*parent_hash))
                    .count();
                if !self.use_reference_folders {
                    assert_eq!(original_hashes_at_start, original_hashes_in_end_results);
                }

                if self.use_reference_folders {
                    // This is same step as without reference folders, but also checks if children are inside/outside reference directories, because may happen, that one file is inside reference folder and other outside

                    // Collecting results to vector
                    for (parent_hash, child_number) in hashes_parents {
                        // If hash contains other hasher OR multiple images are available for checked hash
                        if child_number > 0 || hashes_with_multiple_images.contains(parent_hash) {
                            let vec_fe = all_hashed_images
                                .get(parent_hash)
                                .unwrap()
                                .iter()
                                .filter(|e| is_in_reference_folder(&self.directories.reference_directories, &e.path))
                                .cloned()
                                .collect();
                            collected_similar_images.insert(parent_hash.clone(), vec_fe);
                        }
                    }

                    for (child_hash, (parent_hash, similarity)) in hashes_similarity {
                        let mut vec_fe: Vec<_> = all_hashed_images
                            .get(child_hash)
                            .unwrap()
                            .iter()
                            .filter(|e| !is_in_reference_folder(&self.directories.reference_directories, &e.path))
                            .cloned()
                            .collect();
                        for mut fe in &mut vec_fe {
                            fe.similarity = similarity;
                        }
                        collected_similar_images.get_mut(parent_hash).unwrap().append(&mut vec_fe);
                    }
                } else {
                    // Collecting results to vector
                    for (parent_hash, child_number) in hashes_parents {
                        // If hash contains other hasher OR multiple images are available for checked hash
                        if child_number > 0 || hashes_with_multiple_images.contains(parent_hash) {
                            let vec_fe = all_hashed_images.get(parent_hash).unwrap().clone();
                            collected_similar_images.insert(parent_hash.clone(), vec_fe);
                        }
                    }

                    for (child_hash, (parent_hash, similarity)) in hashes_similarity {
                        let mut vec_fe = all_hashed_images.get(child_hash).unwrap().clone();
                        for mut fe in &mut vec_fe {
                            fe.similarity = similarity;
                        }
                        collected_similar_images.get_mut(parent_hash).unwrap().append(&mut vec_fe);
                    }
                }
            }
        }

        // Validating if group contains duplicated results
        #[cfg(debug_assertions)]
        {
            let mut result_hashset: HashSet<String> = Default::default();
            let mut found = false;
            for vec_file_entry in collected_similar_images.values() {
                if vec_file_entry.is_empty() {
                    println!("Empty Element {vec_file_entry:?}");
                    found = true;
                    continue;
                }
                if vec_file_entry.len() == 1 {
                    println!("Single Element {vec_file_entry:?}");
                    found = true;
                    continue;
                }
                for file_entry in vec_file_entry {
                    let st = file_entry.path.to_string_lossy().to_string();
                    if result_hashset.contains(&st) {
                        found = true;
                        println!("Duplicated Element {st}");
                    } else {
                        result_hashset.insert(st);
                    }
                }
            }
            assert!(!found, "Found Invalid entries");
        }
        self.similar_vectors = collected_similar_images.into_values().collect();

        if self.exclude_images_with_same_size {
            let mut new_vector = Default::default();
            mem::swap(&mut self.similar_vectors, &mut new_vector);
            for vec_file_entry in new_vector {
                let mut bt_sizes: BTreeSet<u64> = Default::default();
                let mut vec_values = Vec::new();
                for file_entry in vec_file_entry {
                    if !bt_sizes.contains(&file_entry.size) {
                        bt_sizes.insert(file_entry.size);
                        vec_values.push(file_entry);
                    }
                }
                if vec_values.len() > 1 {
                    self.similar_vectors.push(vec_values);
                }
            }
        }

        if self.use_reference_folders {
            let mut similar_vector = Default::default();
            mem::swap(&mut self.similar_vectors, &mut similar_vector);
            let reference_directories = self.directories.reference_directories.clone();
            self.similar_referenced_vectors = similar_vector
                .into_iter()
                .filter_map(|vec_file_entry| {
                    let mut files_from_referenced_folders = Vec::new();
                    let mut normal_files = Vec::new();
                    for file_entry in vec_file_entry {
                        if reference_directories.iter().any(|e| file_entry.path.starts_with(e)) {
                            files_from_referenced_folders.push(file_entry);
                        } else {
                            normal_files.push(file_entry);
                        }
                    }

                    if files_from_referenced_folders.is_empty() || normal_files.is_empty() {
                        None
                    } else {
                        Some((files_from_referenced_folders.pop().unwrap(), normal_files))
                    }
                })
                .collect::<Vec<(FileEntry, Vec<FileEntry>)>>();
        }

        Common::print_time(hash_map_modification, SystemTime::now(), "sort_images - selecting data from HashMap");

        if self.use_reference_folders {
            for (_fe, vector) in &self.similar_referenced_vectors {
                self.information.number_of_duplicates += vector.len();
                self.information.number_of_groups += 1;
            }
        } else {
            for vector in &self.similar_vectors {
                self.information.number_of_duplicates += vector.len() - 1;
                self.information.number_of_groups += 1;
            }
        }

        // Clean unused data
        self.image_hashes = Default::default();
        self.images_to_check = Default::default();
        self.bktree = BKTree::new(Hamming);

        true
    }

    /// Set included dir which needs to be relative, exists etc.
    pub fn set_included_directory(&mut self, included_directory: Vec<PathBuf>) {
        self.directories.set_included_directory(included_directory, &mut self.text_messages);
    }

    pub fn set_reference_directory(&mut self, reference_directory: Vec<PathBuf>) {
        self.directories.set_reference_directory(reference_directory);
    }

    pub fn set_excluded_directory(&mut self, excluded_directory: Vec<PathBuf>) {
        self.directories.set_excluded_directory(excluded_directory, &mut self.text_messages);
    }

    pub fn set_excluded_items(&mut self, excluded_items: Vec<String>) {
        self.excluded_items.set_excluded_items(excluded_items, &mut self.text_messages);
    }
}

fn image_to_check<'a>(
    hashes_parents: &mut HashMap<&'a Vec<u8>, u32>,
    hashes_similarity: &mut HashMap<&'a Vec<u8>, (&'a Vec<u8>, u32)>,
    hashes_with_multiple_images: &HashSet<&'a Vec<u8>>,
    hash_to_check: &'a Vec<u8>,
    compared_hash: &'a Vec<u8>,
    similarity: u32,
) {
    if let Some(children_number) = hashes_parents.get(compared_hash) {
        if *children_number > 0 || hashes_with_multiple_images.contains(compared_hash) {
            return;
        }
    }

    // If there is already record, with smaller sensitivity, then replace it
    let mut need_to_add = false;
    let mut need_to_check = false;

    // TODO consider to replace variables from above with closures
    // If current checked hash, have parent, first we must check if similarity between them is lower than checked item
    if let Some((current_parent_hash, current_similarity_with_parent)) = hashes_similarity.get(hash_to_check) {
        if *current_similarity_with_parent > similarity {
            need_to_check = true;

            *hashes_parents.get_mut(current_parent_hash).unwrap() -= 1;
            hashes_similarity.remove(hash_to_check).unwrap();
        }
    } else {
        need_to_check = true;
    }

    if need_to_check {
        if let Some((other_parent_hash, other_similarity)) = hashes_similarity.get(compared_hash) {
            if *other_similarity > similarity {
                need_to_add = true;
                *hashes_parents.get_mut(other_parent_hash).unwrap() -= 1;
            }
        }
        // But when there is no record, just add it
        else {
            need_to_add = true;
        }
    }

    if need_to_add {
        hashes_similarity.insert(compared_hash, (hash_to_check, similarity));

        if let Some(number_of_children) = hashes_parents.get_mut(hash_to_check) {
            *number_of_children += 1;
        } else {
            hashes_parents.insert(hash_to_check, 1);
        }
    }
}

fn is_in_reference_folder(reference_directories: &[PathBuf], path: &Path) -> bool {
    reference_directories.iter().any(|e| path.starts_with(e))
}

impl Default for SimilarImages {
    fn default() -> Self {
        Self::new()
    }
}

impl DebugPrint for SimilarImages {
    #[allow(dead_code)]
    #[allow(unreachable_code)]
    fn debug_print(&self) {
        #[cfg(not(debug_assertions))]
        {
            return;
        }

        println!("---------------DEBUG PRINT---------------");
        println!("Included directories - {:?}", self.directories.included_directories);
        println!("-----------------------------------------");
    }
}

impl SaveResults for SimilarImages {
    fn save_results_to_file(&mut self, file_name: &str) -> bool {
        let start_time: SystemTime = SystemTime::now();
        let file_name: String = match file_name {
            "" => "results.txt".to_string(),
            k => k.to_string(),
        };

        let file_handler = match File::create(&file_name) {
            Ok(t) => t,
            Err(e) => {
                self.text_messages.errors.push(format!("Failed to create file {file_name}, reason {e}"));
                return false;
            }
        };
        let mut writer = BufWriter::new(file_handler);

        if let Err(e) = writeln!(
            writer,
            "Results of searching {:?} with excluded directories {:?} and excluded items {:?}",
            self.directories.included_directories, self.directories.excluded_directories, self.excluded_items.items
        ) {
            self.text_messages.errors.push(format!("Failed to save results to file {file_name}, reason {e}"));
            return false;
        }

        if !self.similar_vectors.is_empty() {
            write!(writer, "{} images which have similar friends\n\n", self.similar_vectors.len()).unwrap();

            for struct_similar in &self.similar_vectors {
                writeln!(writer, "Found {} images which have similar friends", self.similar_vectors.len()).unwrap();
                for file_entry in struct_similar {
                    writeln!(
                        writer,
                        "{} - {} - {} - {}",
                        file_entry.path.display(),
                        file_entry.dimensions,
                        format_size(file_entry.size, BINARY),
                        get_string_from_similarity(&file_entry.similarity, self.hash_size)
                    )
                    .unwrap();
                }
                writeln!(writer).unwrap();
            }
        } else {
            write!(writer, "Not found any similar images.").unwrap();
        }

        Common::print_time(start_time, SystemTime::now(), "save_results_to_file");
        true
    }
}

impl PrintResults for SimilarImages {
    fn print_results(&self) {
        if !self.similar_vectors.is_empty() {
            println!("Found {} images which have similar friends", self.similar_vectors.len());

            for vec_file_entry in &self.similar_vectors {
                for file_entry in vec_file_entry {
                    println!(
                        "{} - {} - {} - {}",
                        file_entry.path.display(),
                        file_entry.dimensions,
                        format_size(file_entry.size, BINARY),
                        get_string_from_similarity(&file_entry.similarity, self.hash_size)
                    );
                }
                println!();
            }
        }
    }
}

pub fn save_hashes_to_file(
    hashmap: &HashMap<String, FileEntry>,
    text_messages: &mut Messages,
    save_also_as_json: bool,
    hash_size: u8,
    hash_alg: HashAlg,
    image_filter: FilterType,
) {
    if let Some(((file_handler, cache_file), (file_handler_json, cache_file_json))) =
        open_cache_folder(&get_cache_file(&hash_size, &hash_alg, &image_filter), true, save_also_as_json, &mut text_messages.warnings)
    {
        {
            let writer = BufWriter::new(file_handler.unwrap()); // Unwrap because cannot fail here
            if let Err(e) = bincode::serialize_into(writer, hashmap) {
                text_messages
                    .warnings
                    .push(format!("Cannot write data to cache file {}, reason {}", cache_file.display(), e));
                return;
            }
        }
        if save_also_as_json {
            if let Some(file_handler_json) = file_handler_json {
                let writer = BufWriter::new(file_handler_json);
                if let Err(e) = serde_json::to_writer(writer, hashmap) {
                    text_messages
                        .warnings
                        .push(format!("Cannot write data to cache file {}, reason {}", cache_file_json.display(), e));
                    return;
                }
            }
        }

        text_messages.messages.push(format!("Properly saved to file {} cache entries.", hashmap.len()));
    }
}

pub fn load_hashes_from_file(
    text_messages: &mut Messages,
    delete_outdated_cache: bool,
    hash_size: u8,
    hash_alg: HashAlg,
    image_filter: FilterType,
) -> Option<HashMap<String, FileEntry>> {
    if let Some(((file_handler, cache_file), (file_handler_json, cache_file_json))) =
        open_cache_folder(&get_cache_file(&hash_size, &hash_alg, &image_filter), false, true, &mut text_messages.warnings)
    {
        let mut hashmap_loaded_entries: HashMap<String, FileEntry>;
        if let Some(file_handler) = file_handler {
            let reader = BufReader::new(file_handler);
            hashmap_loaded_entries = match bincode::deserialize_from(reader) {
                Ok(t) => t,
                Err(e) => {
                    text_messages
                        .warnings
                        .push(format!("Failed to load data from cache file {}, reason {}", cache_file.display(), e));
                    return None;
                }
            };
        } else {
            let reader = BufReader::new(file_handler_json.unwrap()); // Unwrap cannot fail, because at least one file must be valid
            hashmap_loaded_entries = match serde_json::from_reader(reader) {
                Ok(t) => t,
                Err(e) => {
                    text_messages
                        .warnings
                        .push(format!("Failed to load data from cache file {}, reason {}", cache_file_json.display(), e));
                    return None;
                }
            };
        }

        // Don't load cache data if destination file not exists
        if delete_outdated_cache {
            hashmap_loaded_entries.retain(|src_path, _file_entry| Path::new(src_path).exists());
        }

        text_messages.messages.push(format!("Properly loaded {} cache entries.", hashmap_loaded_entries.len()));

        return Some(hashmap_loaded_entries);
    }
    None
}

fn get_cache_file(hash_size: &u8, hash_alg: &HashAlg, image_filter: &FilterType) -> String {
    format!(
        "cache_similar_images_{}_{}_{}_50.bin",
        hash_size,
        convert_algorithm_to_string(hash_alg),
        convert_filters_to_string(image_filter),
    )
}

#[must_use]
pub fn get_string_from_similarity(similarity: &u32, hash_size: u8) -> String {
    let index_preset = match hash_size {
        8 => 0,
        16 => 1,
        32 => 2,
        64 => 3,
        _ => panic!(),
    };

    // #[cfg(debug_assertions)]
    // {
    //     if *similarity <= SIMILAR_VALUES[index_preset][0] {
    //         format!("{} {}", flc!("core_similarity_very_high"), *similarity)
    //     } else if *similarity <= SIMILAR_VALUES[index_preset][1] {
    //         format!("{} {}", flc!("core_similarity_high"), *similarity)
    //     } else if *similarity <= SIMILAR_VALUES[index_preset][2] {
    //         format!("{} {}", flc!("core_similarity_medium"), *similarity)
    //     } else if *similarity <= SIMILAR_VALUES[index_preset][3] {
    //         format!("{} {}", flc!("core_similarity_small"), *similarity)
    //     } else if *similarity <= SIMILAR_VALUES[index_preset][4] {
    //         format!("{} {}", flc!("core_similarity_very_small"), *similarity)
    //     } else if *similarity <= SIMILAR_VALUES[index_preset][5] {
    //         format!("{} {}", flc!("core_similarity_minimal"), *similarity)
    //     } else {
    //         panic!();
    //     }
    // }
    // #[cfg(not(debug_assertions))]

    if *similarity == 0 {
        flc!("core_similarity_original")
    } else if *similarity <= SIMILAR_VALUES[index_preset][0] {
        flc!("core_similarity_very_high")
    } else if *similarity <= SIMILAR_VALUES[index_preset][1] {
        flc!("core_similarity_high")
    } else if *similarity <= SIMILAR_VALUES[index_preset][2] {
        flc!("core_similarity_medium")
    } else if *similarity <= SIMILAR_VALUES[index_preset][3] {
        flc!("core_similarity_small")
    } else if *similarity <= SIMILAR_VALUES[index_preset][4] {
        flc!("core_similarity_very_small")
    } else if *similarity <= SIMILAR_VALUES[index_preset][5] {
        flc!("core_similarity_minimal")
    } else {
        panic!();
    }
}

#[must_use]
pub fn return_similarity_from_similarity_preset(similarity_preset: &SimilarityPreset, hash_size: u8) -> u32 {
    let index_preset = match hash_size {
        8 => 0,
        16 => 1,
        32 => 2,
        64 => 3,
        _ => panic!(),
    };
    match similarity_preset {
        SimilarityPreset::Original => 0,
        SimilarityPreset::VeryHigh => SIMILAR_VALUES[index_preset][0],
        SimilarityPreset::High => SIMILAR_VALUES[index_preset][1],
        SimilarityPreset::Medium => SIMILAR_VALUES[index_preset][2],
        SimilarityPreset::Small => SIMILAR_VALUES[index_preset][3],
        SimilarityPreset::VerySmall => SIMILAR_VALUES[index_preset][4],
        SimilarityPreset::Minimal => SIMILAR_VALUES[index_preset][5],
        SimilarityPreset::None => panic!(""),
    }
}

fn convert_filters_to_string(image_filter: &FilterType) -> String {
    match image_filter {
        FilterType::Lanczos3 => "Lanczos3",
        FilterType::Nearest => "Nearest",
        FilterType::Triangle => "Triangle",
        FilterType::Gaussian => "Gaussian",
        FilterType::CatmullRom => "CatmullRom",
    }
    .to_string()
}

fn convert_algorithm_to_string(hash_alg: &HashAlg) -> String {
    match hash_alg {
        HashAlg::Mean => "Mean",
        HashAlg::Gradient => "Gradient",
        HashAlg::Blockhash => "Blockhash",
        HashAlg::VertGradient => "VertGradient",
        HashAlg::DoubleGradient => "DoubleGradient",
    }
    .to_string()
}

pub fn test_image_conversion_speed() {
    let file_name: &str = "test.jpg";
    let file_path = Path::new(file_name);
    match image::open(file_path) {
        Ok(img_open) => {
            for alg in [HashAlg::Blockhash, HashAlg::Gradient, HashAlg::DoubleGradient, HashAlg::VertGradient, HashAlg::Mean] {
                for filter in [
                    FilterType::Lanczos3,
                    FilterType::CatmullRom,
                    FilterType::Gaussian,
                    FilterType::Nearest,
                    FilterType::Triangle,
                ] {
                    for size in [8, 16, 32, 64] {
                        let hasher_config = HasherConfig::new().hash_alg(alg).resize_filter(filter).hash_size(size, size);

                        let start = SystemTime::now();

                        let hasher = hasher_config.to_hasher();
                        let _hash = hasher.hash_image(&img_open);

                        let end = SystemTime::now();

                        println!("{:?} us {:?} {:?} {}x{}", end.duration_since(start).unwrap().as_micros(), alg, filter, size, size);
                    }
                }
            }
        }
        Err(e) => {
            println!(
                "Failed to open test file {}, reason {}",
                match file_path.canonicalize() {
                    Ok(t) => t.to_string_lossy().to_string(),
                    Err(_inspected) => file_name.to_string(),
                },
                e
            );
        }
    }
}

#[allow(dead_code)]
// Function to validate if after first check there are any duplicated entries
// E.g. /a.jpg is used also as master and similar image which is forbidden, because may
// cause accidentally delete more pictures that user wanted
fn debug_check_for_duplicated_things(
    hashes_parents: &HashMap<&Vec<u8>, u32>,
    hashes_similarity: &HashMap<&Vec<u8>, (&Vec<u8>, u32)>,
    all_hashed_images: &HashMap<Vec<u8>, Vec<FileEntry>>,
    numm: &str,
) {
    let mut found_broken_thing = false;
    let mut hashmap_hashes: HashSet<_> = Default::default();
    let mut hashmap_names: HashSet<_> = Default::default();
    for (hash, number_of_children) in hashes_parents {
        if *number_of_children > 0 {
            if hashmap_hashes.contains(*hash) {
                println!("------1--HASH--{}  {:?}", numm, all_hashed_images.get(*hash).unwrap());
                found_broken_thing = true;
            }
            hashmap_hashes.insert((*hash).clone());

            for i in all_hashed_images.get(*hash).unwrap() {
                let name = i.path.to_string_lossy().to_string();
                if hashmap_names.contains(&name) {
                    println!("------1--NAME--{numm}  {name:?}");
                    found_broken_thing = true;
                }
                hashmap_names.insert(name);
            }
        }
    }
    for hash in hashes_similarity.keys() {
        if hashmap_hashes.contains(*hash) {
            println!("------2--HASH--{}  {:?}", numm, all_hashed_images.get(*hash).unwrap());
            found_broken_thing = true;
        }
        hashmap_hashes.insert((*hash).clone());

        for i in all_hashed_images.get(*hash).unwrap() {
            let name = i.path.to_string_lossy().to_string();
            if hashmap_names.contains(&name) {
                println!("------2--NAME--{numm}  {name:?}");
                found_broken_thing = true;
            }
            hashmap_names.insert(name);
        }
    }

    if found_broken_thing {
        panic!();
    }
}
