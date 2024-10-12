use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::atomic::Ordering;
use std::time::Instant;
use std::{mem, panic};

use bk_tree::BKTree;
use crossbeam_channel::{Receiver, Sender};
use fun_time::fun_time;
use humansize::{format_size, BINARY};
use image::GenericImageView;
use image_hasher::{FilterType, HashAlg, HasherConfig};
use log::debug;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};

use crate::common::{
    check_if_stop_received, delete_files_custom, prepare_thread_handler_common, send_info_and_wait_for_ending_all_threads, HEIC_EXTENSIONS, IMAGE_RS_SIMILAR_IMAGES_EXTENSIONS,
    JXL_IMAGE_EXTENSIONS, RAW_IMAGE_EXTENSIONS,
};
use crate::common_cache::{extract_loaded_cache, get_similar_images_cache_file, load_cache_from_file_generalized_by_path, save_cache_to_file_generalized};
use crate::common_dir_traversal::{inode, take_1_per_inode, DirTraversalBuilder, DirTraversalResult, FileEntry, ToolType};
use crate::common_image::get_dynamic_image_from_path;
use crate::common_tool::{CommonData, CommonToolData, DeleteMethod};
use crate::common_traits::{DebugPrint, PrintResults, ResultEntry};
use crate::flc;
use crate::progress_data::{CurrentStage, ProgressData};

type ImHash = Vec<u8>;

// 40 is, similar like previous 20 in 8 hash size is useless
// But since Krowka have problems with proper changing max value in fly
// hardcode 40 as max value
pub const SIMILAR_VALUES: [[u32; 6]; 4] = [
    [1, 2, 5, 7, 14, 40],    // 8
    [2, 5, 15, 30, 40, 40],  // 16
    [4, 10, 20, 40, 40, 40], // 32
    [6, 20, 40, 40, 40, 40], // 64
];

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ImagesEntry {
    pub path: PathBuf,
    pub size: u64,
    pub width: u32,
    pub height: u32,
    pub modified_date: u64,
    pub hash: ImHash,
    pub similarity: u32,
}

impl ResultEntry for ImagesEntry {
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
impl FileEntry {
    fn into_images_entry(self) -> ImagesEntry {
        ImagesEntry {
            size: self.size,
            path: self.path,
            modified_date: self.modified_date,

            width: 0,
            height: 0,
            hash: Vec::new(),
            similarity: 0,
        }
    }
}

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

struct Hamming;

impl bk_tree::Metric<ImHash> for Hamming {
    fn distance(&self, a: &ImHash, b: &ImHash) -> u32 {
        hamming::distance_fast(a, b).expect("Calculating hamming distance, cannot fail") as u32
    }

    fn threshold_distance(&self, a: &ImHash, b: &ImHash, _threshold: u32) -> Option<u32> {
        Some(self.distance(a, b))
    }
}

pub struct SimilarImagesParameters {
    pub similarity: u32,
    pub hash_size: u8,
    pub hash_alg: HashAlg,
    pub image_filter: FilterType,
    pub exclude_images_with_same_size: bool,
    pub ignore_hard_links: bool,
}

impl SimilarImagesParameters {
    pub fn new(similarity: u32, hash_size: u8, hash_alg: HashAlg, image_filter: FilterType, exclude_images_with_same_size: bool, ignore_hard_links: bool) -> Self {
        assert!([8, 16, 32, 64].contains(&hash_size));
        Self {
            similarity,
            hash_size,
            hash_alg,
            image_filter,
            exclude_images_with_same_size,
            ignore_hard_links,
        }
    }
}

pub struct SimilarImages {
    common_data: CommonToolData,
    information: Info,
    bktree: BKTree<ImHash, Hamming>,
    similar_vectors: Vec<Vec<ImagesEntry>>,
    similar_referenced_vectors: Vec<(ImagesEntry, Vec<ImagesEntry>)>,
    // Hashmap with image hashes and Vector with names of files
    image_hashes: HashMap<ImHash, Vec<ImagesEntry>>,
    images_to_check: BTreeMap<String, ImagesEntry>,
    params: SimilarImagesParameters,
}

#[derive(Default)]
pub struct Info {
    pub number_of_duplicates: usize,
    pub number_of_groups: u64,
}

impl SimilarImages {
    pub fn new(params: SimilarImagesParameters) -> Self {
        Self {
            common_data: CommonToolData::new(ToolType::SimilarImages),
            information: Default::default(),
            bktree: BKTree::new(Hamming),
            similar_vectors: vec![],
            similar_referenced_vectors: vec![],
            params,
            images_to_check: Default::default(),
            image_hashes: Default::default(),
        }
    }

    #[fun_time(message = "find_similar_images", level = "info")]
    pub fn find_similar_images(&mut self, stop_receiver: Option<&Receiver<()>>, progress_sender: Option<&Sender<ProgressData>>) {
        self.prepare_items();
        self.common_data.use_reference_folders = !self.common_data.directories.reference_directories.is_empty();
        if !self.check_for_similar_images(stop_receiver, progress_sender) {
            self.common_data.stopped_search = true;
            return;
        }
        if !self.hash_images(stop_receiver, progress_sender) {
            self.common_data.stopped_search = true;
            return;
        }
        if !self.find_similar_hashes(stop_receiver, progress_sender) {
            self.common_data.stopped_search = true;
            return;
        }
        self.delete_files();
        self.debug_print();
    }

    #[fun_time(message = "check_for_similar_images", level = "debug")]
    fn check_for_similar_images(&mut self, stop_receiver: Option<&Receiver<()>>, progress_sender: Option<&Sender<ProgressData>>) -> bool {
        if cfg!(feature = "heif") {
            self.common_data
                .extensions
                .set_and_validate_allowed_extensions(&[IMAGE_RS_SIMILAR_IMAGES_EXTENSIONS, RAW_IMAGE_EXTENSIONS, JXL_IMAGE_EXTENSIONS, HEIC_EXTENSIONS].concat());
        } else {
            self.common_data
                .extensions
                .set_and_validate_allowed_extensions(&[IMAGE_RS_SIMILAR_IMAGES_EXTENSIONS, RAW_IMAGE_EXTENSIONS, JXL_IMAGE_EXTENSIONS].concat());
        }

        if !self.common_data.extensions.set_any_extensions() {
            return true;
        }

        let result = DirTraversalBuilder::new()
            .group_by(inode)
            .stop_receiver(stop_receiver)
            .progress_sender(progress_sender)
            .common_data(&self.common_data)
            .build()
            .run();

        match result {
            DirTraversalResult::SuccessFiles { grouped_file_entries, warnings } => {
                self.images_to_check = grouped_file_entries
                    .into_iter()
                    .flat_map(if self.get_params().ignore_hard_links { |(_, fes)| fes } else { take_1_per_inode })
                    .map(|fe| {
                        let fe_str = fe.path.to_string_lossy().to_string();
                        let image_entry = fe.into_images_entry();

                        (fe_str, image_entry)
                    })
                    .collect();
                self.common_data.text_messages.warnings.extend(warnings);
                debug!("check_files - Found {} image files.", self.images_to_check.len());
                true
            }

            DirTraversalResult::Stopped => false,
        }
    }

    #[fun_time(message = "hash_images_load_cache", level = "debug")]
    fn hash_images_load_cache(&mut self) -> (BTreeMap<String, ImagesEntry>, BTreeMap<String, ImagesEntry>, BTreeMap<String, ImagesEntry>) {
        let loaded_hash_map;

        let mut records_already_cached: BTreeMap<String, ImagesEntry> = Default::default();
        let mut non_cached_files_to_check: BTreeMap<String, ImagesEntry> = Default::default();

        if self.common_data.use_cache {
            let (messages, loaded_items) = load_cache_from_file_generalized_by_path::<ImagesEntry>(
                &get_similar_images_cache_file(&self.get_params().hash_size, &self.get_params().hash_alg, &self.get_params().image_filter),
                self.get_delete_outdated_cache(),
                &self.images_to_check,
            );
            self.get_text_messages_mut().extend_with_another_messages(messages);
            loaded_hash_map = loaded_items.unwrap_or_default();

            debug!("hash_images-load_cache - starting calculating diff");
            extract_loaded_cache(
                &loaded_hash_map,
                mem::take(&mut self.images_to_check),
                &mut records_already_cached,
                &mut non_cached_files_to_check,
            );
            debug!(
                "hash_images_load_cache - completed diff between loaded and prechecked files, {}({}) - non cached, {}({}) - already cached",
                non_cached_files_to_check.len(),
                format_size(non_cached_files_to_check.values().map(|e| e.size).sum::<u64>(), BINARY),
                records_already_cached.len(),
                format_size(records_already_cached.values().map(|e| e.size).sum::<u64>(), BINARY),
            );
        } else {
            loaded_hash_map = Default::default();
            mem::swap(&mut self.images_to_check, &mut non_cached_files_to_check);
        }
        (loaded_hash_map, records_already_cached, non_cached_files_to_check)
    }

    // Cache algorithm:
    // - Load data from file
    // - Remove from data to search, already loaded entries from cache(size and modified date must match)
    // - Check hash of files which doesn't have saved entry
    // - Join already read hashes with hashes which were read from file
    // - Join all hashes and save it to file

    #[fun_time(message = "hash_images", level = "debug")]
    fn hash_images(&mut self, stop_receiver: Option<&Receiver<()>>, progress_sender: Option<&Sender<ProgressData>>) -> bool {
        if self.images_to_check.is_empty() {
            return true;
        }

        let (loaded_hash_map, records_already_cached, non_cached_files_to_check) = self.hash_images_load_cache();

        let (progress_thread_handle, progress_thread_run, atomic_counter, check_was_stopped) = prepare_thread_handler_common(
            progress_sender,
            CurrentStage::SimilarImagesCalculatingHashes,
            non_cached_files_to_check.len(),
            self.get_test_type(),
        );

        debug!("hash_images - start hashing images");
        let (mut vec_file_entry, errors): (Vec<ImagesEntry>, Vec<String>) = non_cached_files_to_check
            .into_par_iter()
            .map(|(_s, mut file_entry)| {
                atomic_counter.fetch_add(1, Ordering::Relaxed);
                if check_if_stop_received(stop_receiver) {
                    check_was_stopped.store(true, Ordering::Relaxed);
                    return None;
                }
                if let Err(e) = self.collect_image_file_entry(&mut file_entry) {
                    return Some(Err(e));
                }

                Some(Ok(file_entry))
            })
            .while_some()
            .partition_map(|res| match res {
                Ok(entry) => itertools::Either::Left(entry),
                Err(err) => itertools::Either::Right(err),
            });

        self.common_data.text_messages.errors.extend(errors);
        debug!("hash_images - end hashing {} images", vec_file_entry.len());

        send_info_and_wait_for_ending_all_threads(&progress_thread_run, progress_thread_handle);

        // Just connect loaded results with already calculated hashes
        for file_entry in records_already_cached.into_values() {
            vec_file_entry.push(file_entry);
        }

        // All valid entries are used to create bktree used to check for hash similarity
        for file_entry in &vec_file_entry {
            // Only use to comparing, non broken hashes(all 0 or 255 hashes means that algorithm fails to decode them because e.g. contains a lot of alpha channel)
            if !(file_entry.hash.is_empty() || file_entry.hash.iter().all(|e| *e == 0) || file_entry.hash.iter().all(|e| *e == 255)) {
                self.image_hashes.entry(file_entry.hash.clone()).or_default().push(file_entry.clone());
            }
        }

        self.save_to_cache(vec_file_entry, loaded_hash_map);

        // Break if stop was clicked after saving to cache
        if check_was_stopped.load(Ordering::Relaxed) {
            return false;
        }

        true
    }

    #[fun_time(message = "save_to_cache", level = "debug")]
    fn save_to_cache(&mut self, vec_file_entry: Vec<ImagesEntry>, loaded_hash_map: BTreeMap<String, ImagesEntry>) {
        if self.common_data.use_cache {
            // Must save all results to file, old loaded from file with all currently counted results
            let mut all_results: BTreeMap<String, ImagesEntry> = loaded_hash_map;
            for file_entry in vec_file_entry {
                all_results.insert(file_entry.path.to_string_lossy().to_string(), file_entry);
            }

            let messages = save_cache_to_file_generalized(
                &get_similar_images_cache_file(&self.get_params().hash_size, &self.get_params().hash_alg, &self.get_params().image_filter),
                &all_results,
                self.common_data.save_also_as_json,
                0,
            );
            self.get_text_messages_mut().extend_with_another_messages(messages);
        }
    }

    fn collect_image_file_entry(&self, file_entry: &mut ImagesEntry) -> Result<(), String> {
        let img = get_dynamic_image_from_path(&file_entry.path.to_string_lossy())?;

        let dimensions = img.dimensions();

        file_entry.width = dimensions.0;
        file_entry.height = dimensions.1;

        let hasher_config = HasherConfig::new()
            .hash_size(self.get_params().hash_size as u32, self.get_params().hash_size as u32)
            .hash_alg(self.get_params().hash_alg)
            .resize_filter(self.get_params().image_filter);
        let hasher = hasher_config.to_hasher();
        let hash = hasher.hash_image(&img);
        file_entry.hash = hash.as_bytes().to_vec();

        Ok(())
    }

    // Split hashes at 2 parts, base hashes and hashes to compare, 3 argument is set of hashes with multiple images
    #[fun_time(message = "split_hashes", level = "debug")]
    fn split_hashes(&mut self, all_hashed_images: &HashMap<ImHash, Vec<ImagesEntry>>) -> (Vec<ImHash>, HashSet<ImHash>) {
        let hashes_with_multiple_images: HashSet<ImHash> = all_hashed_images
            .iter()
            .filter_map(|(hash, vec_file_entry)| {
                if vec_file_entry.len() >= 2 {
                    return Some(hash.clone());
                };
                None
            })
            .collect();
        let mut base_hashes = Vec::new(); // Initial hashes
        if self.common_data.use_reference_folders {
            let mut files_from_referenced_folders: HashMap<ImHash, Vec<ImagesEntry>> = HashMap::new();
            let mut normal_files: HashMap<ImHash, Vec<ImagesEntry>> = HashMap::new();

            all_hashed_images.clone().into_iter().for_each(|(hash, vec_file_entry)| {
                for file_entry in vec_file_entry {
                    if is_in_reference_folder(&self.common_data.directories.reference_directories, &file_entry.path) {
                        files_from_referenced_folders.entry(hash.clone()).or_default().push(file_entry);
                    } else {
                        normal_files.entry(hash.clone()).or_default().push(file_entry);
                    }
                }
            });

            for hash in normal_files.into_keys() {
                self.bktree.add(hash);
            }

            for hash in files_from_referenced_folders.into_keys() {
                base_hashes.push(hash);
            }
        } else {
            for original_hash in all_hashed_images.keys() {
                self.bktree.add(original_hash.clone());
            }
            base_hashes = all_hashed_images.keys().cloned().collect::<Vec<_>>();
        }
        (base_hashes, hashes_with_multiple_images)
    }

    #[fun_time(message = "collect_hash_compare_result", level = "debug")]
    fn collect_hash_compare_result(
        &self,
        hashes_parents: HashMap<ImHash, u32>,
        hashes_with_multiple_images: &HashSet<ImHash>,
        all_hashed_images: &HashMap<ImHash, Vec<ImagesEntry>>,
        collected_similar_images: &mut HashMap<ImHash, Vec<ImagesEntry>>,
        hashes_similarity: HashMap<ImHash, (ImHash, u32)>,
    ) {
        // Collecting results to vector
        for (parent_hash, child_number) in hashes_parents {
            // If hash contains other hasher OR multiple images are available for checked hash
            if child_number > 0 || hashes_with_multiple_images.contains(&parent_hash) {
                let vec_fe = all_hashed_images[&parent_hash].clone();
                collected_similar_images.insert(parent_hash.clone(), vec_fe);
            }
        }

        for (child_hash, (parent_hash, similarity)) in hashes_similarity {
            let mut vec_fe = all_hashed_images[&child_hash].clone();
            for fe in &mut vec_fe {
                fe.similarity = similarity;
            }
            collected_similar_images
                .get_mut(&parent_hash)
                .expect("Cannot find parent hash - this should be added in previous step")
                .append(&mut vec_fe);
        }
    }

    #[fun_time(message = "compare_hashes_with_non_zero_tolerance", level = "debug")]
    fn compare_hashes_with_non_zero_tolerance(
        &mut self,
        all_hashed_images: &HashMap<ImHash, Vec<ImagesEntry>>,
        collected_similar_images: &mut HashMap<ImHash, Vec<ImagesEntry>>,
        progress_sender: Option<&Sender<ProgressData>>,
        stop_receiver: Option<&Receiver<()>>,
        tolerance: u32,
    ) -> bool {
        // Don't use hashes with multiple images in bktree, because they will always be master of group and cannot be find by other hashes
        let (base_hashes, hashes_with_multiple_images) = self.split_hashes(all_hashed_images);

        let (progress_thread_handle, progress_thread_run, atomic_counter, check_was_stopped) =
            prepare_thread_handler_common(progress_sender, CurrentStage::SimilarImagesComparingHashes, base_hashes.len(), self.get_test_type());

        let mut hashes_parents: HashMap<ImHash, u32> = Default::default(); // Hashes used as parent (hash, children_number_of_hash)
        let mut hashes_similarity: HashMap<ImHash, (ImHash, u32)> = Default::default(); // Hashes used as child, (parent_hash, similarity)

        // Check them in chunks, to decrease number of used memory
        // println!();
        let base_hashes_chunks = base_hashes.chunks(1000);
        for chunk in base_hashes_chunks {
            let partial_results = chunk
                .into_par_iter()
                .map(|hash_to_check| {
                    atomic_counter.fetch_add(1, Ordering::Relaxed);

                    if check_if_stop_received(stop_receiver) {
                        check_was_stopped.store(true, Ordering::Relaxed);
                        return None;
                    }
                    let mut found_items = self
                        .bktree
                        .find(hash_to_check, tolerance)
                        .filter(|(similarity, compared_hash)| {
                            *similarity != 0 && !hashes_parents.contains_key(*compared_hash) && !hashes_with_multiple_images.contains(*compared_hash)
                        })
                        .filter(|(similarity, compared_hash)| {
                            if let Some((_, other_similarity_with_parent)) = hashes_similarity.get(*compared_hash) {
                                // If current hash is more similar to other hash than to current parent hash, then skip check earlier
                                // Because there is no way to be more similar to other hash than to current parent hash
                                if *similarity >= *other_similarity_with_parent {
                                    return false;
                                }
                            }
                            true
                        })
                        .collect::<Vec<_>>();

                    // Sort by tolerance
                    found_items.sort_unstable_by_key(|f| f.0);
                    Some((hash_to_check, found_items))
                })
                .while_some()
                .filter(|(original_hash, vec_similar_hashes)| !vec_similar_hashes.is_empty() || hashes_with_multiple_images.contains(*original_hash))
                .collect::<Vec<_>>();

            // for (hash, vec) in &partial_results {
            //     println!("{hash:?} --- {:?}", vec.iter().map(|e| e.1).collect::<Vec<_>>());
            // }

            if check_was_stopped.load(Ordering::Relaxed) {
                send_info_and_wait_for_ending_all_threads(&progress_thread_run, progress_thread_handle);
                return false;
            }

            self.connect_results(partial_results, &mut hashes_parents, &mut hashes_similarity, &hashes_with_multiple_images);
        }

        send_info_and_wait_for_ending_all_threads(&progress_thread_run, progress_thread_handle);

        debug_check_for_duplicated_things(self.common_data.use_reference_folders, &hashes_parents, &hashes_similarity, all_hashed_images, "LATTER");
        self.collect_hash_compare_result(hashes_parents, &hashes_with_multiple_images, all_hashed_images, collected_similar_images, hashes_similarity);

        true
    }

    #[fun_time(message = "connect_results", level = "debug")]
    fn connect_results(
        &self,
        partial_results: Vec<(&ImHash, Vec<(u32, &ImHash)>)>,
        hashes_parents: &mut HashMap<ImHash, u32>,
        hashes_similarity: &mut HashMap<ImHash, (ImHash, u32)>,
        hashes_with_multiple_images: &HashSet<ImHash>,
    ) {
        for (original_hash, vec_compared_hashes) in partial_results {
            let mut number_of_added_child_items = 0;
            for (similarity, compared_hash) in vec_compared_hashes {
                // If hash is already in results skip it
                // This check duplicates check from bktree.find, but it is needed to because when iterating over elements, this structure can change
                if hashes_parents.contains_key(compared_hash) {
                    continue;
                }

                // If there is already record, with smaller sensitivity, then replace it
                let mut need_to_add = false;
                let mut need_to_check = false;

                // TODO consider to replace variables from above with closures
                // If current checked hash, have parent, first we must check if similarity between them is lower than checked item
                if let Some((current_parent_hash, current_similarity_with_parent)) = hashes_similarity.get(original_hash) {
                    if *current_similarity_with_parent > similarity {
                        need_to_check = true;

                        *hashes_parents.get_mut(current_parent_hash).expect("Cannot find parent hash") -= 1;
                        if hashes_parents.get(current_parent_hash) == Some(&0) && !hashes_with_multiple_images.contains(current_parent_hash) {
                            hashes_parents.remove(current_parent_hash);
                        }
                        hashes_similarity
                            .remove(original_hash)
                            .expect("This should never fail, because we are iterating over this hash");
                    }
                } else {
                    need_to_check = true;
                }

                if need_to_check {
                    if let Some((other_parent_hash, other_similarity)) = hashes_similarity.get(compared_hash) {
                        if *other_similarity > similarity {
                            need_to_add = true;
                            *hashes_parents.get_mut(other_parent_hash).expect("Cannot find parent hash") -= 1;
                        }
                    }
                    // But when there is no record, just add it
                    else {
                        need_to_add = true;
                    }
                }

                if need_to_add {
                    hashes_similarity.insert(compared_hash.clone(), (original_hash.clone(), similarity));
                    number_of_added_child_items += 1;
                }
            }

            if number_of_added_child_items > 0 || hashes_with_multiple_images.contains(original_hash) {
                hashes_parents.insert((*original_hash).clone(), number_of_added_child_items);
            }
        }
    }

    #[fun_time(message = "find_similar_hashes", level = "debug")]
    fn find_similar_hashes(&mut self, stop_receiver: Option<&Receiver<()>>, progress_sender: Option<&Sender<ProgressData>>) -> bool {
        if self.image_hashes.is_empty() {
            return true;
        }

        let tolerance = self.get_params().similarity;

        // Results
        let mut collected_similar_images: HashMap<ImHash, Vec<ImagesEntry>> = Default::default();

        let all_hashed_images = mem::take(&mut self.image_hashes);

        // Checking entries with tolerance 0 is really easy and fast, because only entries with same hashes needs to be checked
        if tolerance == 0 {
            for (hash, vec_file_entry) in all_hashed_images {
                if vec_file_entry.len() >= 2 {
                    collected_similar_images.insert(hash, vec_file_entry);
                }
            }
        } else if !self.compare_hashes_with_non_zero_tolerance(&all_hashed_images, &mut collected_similar_images, progress_sender, stop_receiver, tolerance) {
            return false;
        }

        self.verify_duplicated_items(&collected_similar_images);

        // Info about hashes is not needed anymore, so we drop this info
        self.similar_vectors = collected_similar_images.into_values().collect();

        self.exclude_items_with_same_size();

        self.remove_multiple_records_from_reference_folders();

        if self.common_data.use_reference_folders {
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

        // Clean unused data to save ram
        self.image_hashes = Default::default();
        self.images_to_check = Default::default();
        self.bktree = BKTree::new(Hamming);

        true
    }

    #[fun_time(message = "exclude_items_with_same_size", level = "debug")]
    fn exclude_items_with_same_size(&mut self) {
        if self.get_params().exclude_images_with_same_size {
            for vec_file_entry in mem::take(&mut self.similar_vectors) {
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
    }

    #[fun_time(message = "remove_multiple_records_from_reference_folders", level = "debug")]
    fn remove_multiple_records_from_reference_folders(&mut self) {
        if self.common_data.use_reference_folders {
            self.similar_referenced_vectors = mem::take(&mut self.similar_vectors)
                .into_iter()
                .filter_map(|vec_file_entry| {
                    let (mut files_from_referenced_folders, normal_files): (Vec<_>, Vec<_>) = vec_file_entry
                        .into_iter()
                        .partition(|e| self.common_data.directories.is_in_referenced_directory(e.get_path()));

                    if normal_files.is_empty() {
                        None
                    } else {
                        files_from_referenced_folders.pop().map(|file| (file, normal_files))
                    }
                })
                .collect::<Vec<(ImagesEntry, Vec<ImagesEntry>)>>();
        }
    }

    #[allow(unused_variables)]
    // TODO this probably not works good when reference folders are used
    pub fn verify_duplicated_items(&self, collected_similar_images: &HashMap<ImHash, Vec<ImagesEntry>>) {
        if !cfg!(debug_assertions) {
            return;
        }
        // Validating if group contains duplicated results
        let mut result_hashset: HashSet<String> = Default::default();
        let mut found = false;

        for vec_file_entry in collected_similar_images.values() {
            if vec_file_entry.is_empty() {
                println!("Empty group");
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
        assert!(!found, "Found Invalid entries, verify errors before");
    }

    fn delete_files(&mut self) {
        if self.common_data.delete_method == DeleteMethod::None {
            return;
        }

        let vec_files = self.similar_vectors.iter().collect::<Vec<_>>();
        delete_files_custom(&vec_files, &self.common_data.delete_method, &mut self.common_data.text_messages, self.common_data.dry_run);
    }
}

fn is_in_reference_folder(reference_directories: &[PathBuf], path: &Path) -> bool {
    reference_directories.iter().any(|e| path.starts_with(e))
}

impl DebugPrint for SimilarImages {
    fn debug_print(&self) {
        if !cfg!(debug_assertions) {
            return;
        }

        println!("---------------DEBUG PRINT---------------");
        self.debug_print_common();
        println!("-----------------------------------------");
    }
}

impl PrintResults for SimilarImages {
    fn write_results<T: Write>(&self, writer: &mut T) -> std::io::Result<()> {
        if !self.similar_vectors.is_empty() {
            write!(writer, "{} images which have similar friends\n\n", self.similar_vectors.len())?;

            for struct_similar in &self.similar_vectors {
                writeln!(writer, "Found {} images which have similar friends", struct_similar.len())?;
                for file_entry in struct_similar {
                    writeln!(
                        writer,
                        "\"{}\" - {}x{} - {} - {}",
                        file_entry.path.to_string_lossy(),
                        file_entry.width,
                        file_entry.height,
                        format_size(file_entry.size, BINARY),
                        get_string_from_similarity(&file_entry.similarity, self.get_params().hash_size)
                    )?;
                }
                writeln!(writer)?;
            }
        } else if !self.similar_referenced_vectors.is_empty() {
            writeln!(writer, "{} images which have similar friends\n\n", self.similar_referenced_vectors.len())?;

            for (file_entry, vec_file_entry) in &self.similar_referenced_vectors {
                writeln!(writer, "Found {} images which have similar friends", vec_file_entry.len())?;
                writeln!(writer)?;
                writeln!(
                    writer,
                    "\"{}\" - {}x{} - {} - {}",
                    file_entry.path.to_string_lossy(),
                    file_entry.width,
                    file_entry.height,
                    format_size(file_entry.size, BINARY),
                    get_string_from_similarity(&file_entry.similarity, self.get_params().hash_size)
                )?;
                for file_entry in vec_file_entry {
                    writeln!(
                        writer,
                        "{:?} - {}x{} - {} - {}",
                        file_entry.path,
                        file_entry.width,
                        file_entry.height,
                        format_size(file_entry.size, BINARY),
                        get_string_from_similarity(&file_entry.similarity, self.get_params().hash_size)
                    )?;
                }
                writeln!(writer)?;
            }
        } else {
            write!(writer, "Not found any similar images.")?;
        }

        Ok(())
    }

    fn save_results_to_file_as_json(&self, file_name: &str, pretty_print: bool) -> std::io::Result<()> {
        if self.get_use_reference() {
            self.save_results_to_file_as_json_internal(file_name, &self.similar_referenced_vectors, pretty_print)
        } else {
            self.save_results_to_file_as_json_internal(file_name, &self.similar_vectors, pretty_print)
        }
    }
}

pub fn get_string_from_similarity(similarity: &u32, hash_size: u8) -> String {
    let index_preset = match hash_size {
        8 => 0,
        16 => 1,
        32 => 2,
        64 => 3,
        _ => panic!("Invalid hash size {hash_size}"),
    };

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
        panic!("Invalid similarity value {similarity} for hash size {hash_size} (index {index_preset})");
    }
}

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

pub fn convert_filters_to_string(image_filter: &FilterType) -> String {
    match image_filter {
        FilterType::Lanczos3 => "Lanczos3",
        FilterType::Nearest => "Nearest",
        FilterType::Triangle => "Triangle",
        FilterType::Gaussian => "Gaussian",
        FilterType::CatmullRom => "CatmullRom",
    }
    .to_string()
}

pub fn convert_algorithm_to_string(hash_alg: &HashAlg) -> String {
    match hash_alg {
        HashAlg::Mean => "Mean",
        HashAlg::Gradient => "Gradient",
        HashAlg::Blockhash => "Blockhash",
        HashAlg::VertGradient => "VertGradient",
        HashAlg::DoubleGradient => "DoubleGradient",
        HashAlg::Median => "Median",
    }
    .to_string()
}

pub fn test_image_conversion_speed() {
    let file_name: &str = "test.jpg";
    let file_path = Path::new(file_name);
    match image::open(file_path) {
        Ok(img_open) => {
            for alg in [
                HashAlg::Blockhash,
                HashAlg::Gradient,
                HashAlg::DoubleGradient,
                HashAlg::VertGradient,
                HashAlg::Mean,
                HashAlg::Median,
            ] {
                for filter in [
                    FilterType::Lanczos3,
                    FilterType::CatmullRom,
                    FilterType::Gaussian,
                    FilterType::Nearest,
                    FilterType::Triangle,
                ] {
                    for size in [8, 16, 32, 64] {
                        let hasher_config = HasherConfig::new().hash_alg(alg).resize_filter(filter).hash_size(size, size);

                        let start = Instant::now();

                        let hasher = hasher_config.to_hasher();
                        let _hash = hasher.hash_image(&img_open);

                        println!("{:?} us {:?} {:?} {}x{}", start.elapsed().as_micros(), alg, filter, size, size);
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
#[allow(unreachable_code)]
#[allow(unused_variables)]
// Function to validate if after first check there are any duplicated entries
// E.g. /a.jpg is used also as master and similar image which is forbidden, because may
// cause accidentally delete more pictures that user wanted
fn debug_check_for_duplicated_things(
    use_reference_folders: bool,
    hashes_parents: &HashMap<ImHash, u32>,
    hashes_similarity: &HashMap<ImHash, (ImHash, u32)>,
    all_hashed_images: &HashMap<ImHash, Vec<ImagesEntry>>,
    numm: &str,
) {
    if !cfg!(debug_assertions) {
        return;
    }

    if use_reference_folders {
        return;
    }

    let mut found_broken_thing = false;
    let mut hashmap_hashes: HashSet<_> = Default::default();
    let mut hashmap_names: HashSet<_> = Default::default();
    for (hash, number_of_children) in hashes_parents {
        if *number_of_children > 0 {
            if hashmap_hashes.contains(hash) {
                println!("------1--HASH--{}  {:?}", numm, all_hashed_images[hash]);
                found_broken_thing = true;
            }
            hashmap_hashes.insert((*hash).clone());

            for i in &all_hashed_images[hash] {
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
        if hashmap_hashes.contains(hash) {
            println!("------2--HASH--{}  {:?}", numm, all_hashed_images[hash]);
            found_broken_thing = true;
        }
        hashmap_hashes.insert((*hash).clone());

        for i in &all_hashed_images[hash] {
            let name = i.path.to_string_lossy().to_string();
            if hashmap_names.contains(&name) {
                println!("------2--NAME--{numm}  {name:?}");
                found_broken_thing = true;
            }
            hashmap_names.insert(name);
        }
    }

    assert!(!found_broken_thing);
}

impl CommonData for SimilarImages {
    fn get_cd(&self) -> &CommonToolData {
        &self.common_data
    }
    fn get_cd_mut(&mut self) -> &mut CommonToolData {
        &mut self.common_data
    }
}

impl SimilarImages {
    pub fn get_params(&self) -> &SimilarImagesParameters {
        &self.params
    }

    pub const fn get_similar_images(&self) -> &Vec<Vec<ImagesEntry>> {
        &self.similar_vectors
    }

    pub fn get_similar_images_referenced(&self) -> &Vec<(ImagesEntry, Vec<ImagesEntry>)> {
        &self.similar_referenced_vectors
    }

    pub fn get_use_reference(&self) -> bool {
        self.common_data.use_reference_folders
    }

    pub const fn get_information(&self) -> &Info {
        &self.information
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::path::PathBuf;

    use bk_tree::BKTree;
    use image::imageops::FilterType;
    use image_hasher::HashAlg;

    use crate::common_tool::CommonData;
    use crate::similar_images::{Hamming, ImHash, ImagesEntry, SimilarImages, SimilarImagesParameters};

    fn get_default_parameters() -> SimilarImagesParameters {
        SimilarImagesParameters {
            hash_alg: HashAlg::Gradient,
            hash_size: 8,
            similarity: 0,
            image_filter: FilterType::Lanczos3,
            exclude_images_with_same_size: false,
            ignore_hard_links: false,
        }
    }

    #[test]
    fn test_compare_no_images() {
        for _ in 0..100 {
            let mut similar_images = SimilarImages::new(get_default_parameters());
            similar_images.find_similar_images(None, None);
            assert_eq!(similar_images.get_similar_images().len(), 0);
        }
    }

    #[test]
    fn test_compare_tolerance_0_normal_mode() {
        for _ in 0..100 {
            let mut parameters = get_default_parameters();
            parameters.similarity = 0;
            let mut similar_images = SimilarImages::new(parameters);

            let fe1 = create_random_file_entry(vec![1, 1, 1, 1, 1, 1, 1, 1], "abc.txt");
            let fe2 = create_random_file_entry(vec![1, 1, 1, 1, 1, 1, 1, 1], "bcd.txt");
            let fe3 = create_random_file_entry(vec![1, 1, 1, 1, 1, 1, 1, 2], "cde.txt");
            let fe4 = create_random_file_entry(vec![1, 1, 1, 1, 1, 1, 1, 2], "rrt.txt");
            let fe5 = create_random_file_entry(vec![1, 1, 1, 1, 1, 1, 1, 2], "bld.txt");

            add_hashes(&mut similar_images.image_hashes, vec![fe1.clone(), fe2.clone(), fe3.clone(), fe4.clone(), fe5.clone()]);

            similar_images.find_similar_hashes(None, None);
            assert_eq!(similar_images.get_similar_images().len(), 2);
            let first_group = similar_images.get_similar_images()[0].iter().map(|e| &e.path).collect::<Vec<_>>();
            let second_group = similar_images.get_similar_images()[1].iter().map(|e| &e.path).collect::<Vec<_>>();
            // Initial order is not guaranteed, so we need to check both options
            if similar_images.get_similar_images()[0][0].hash == fe1.hash {
                assert_eq!(first_group, vec![&fe1.path, &fe2.path]);
                assert_eq!(second_group, vec![&fe3.path, &fe4.path, &fe5.path]);
            } else {
                assert_eq!(first_group, vec![&fe3.path, &fe4.path, &fe5.path]);
                assert_eq!(second_group, vec![&fe1.path, &fe2.path]);
            }
        }
    }

    #[test]
    fn test_simple_normal_one_group() {
        for _ in 0..100 {
            let mut parameters = get_default_parameters();
            parameters.similarity = 1;
            let mut similar_images = SimilarImages::new(parameters);

            let fe1 = create_random_file_entry(vec![1, 1, 1, 1, 1, 1, 1, 1], "abc.txt");
            let fe2 = create_random_file_entry(vec![1, 1, 1, 1, 1, 1, 1, 1], "bcd.txt");

            add_hashes(&mut similar_images.image_hashes, vec![fe1, fe2]);

            similar_images.find_similar_hashes(None, None);
            assert_eq!(similar_images.get_similar_images().len(), 1);
        }
    }

    #[test]
    fn test_simple_normal_one_group_extended() {
        for _ in 0..100 {
            let mut parameters = get_default_parameters();
            parameters.similarity = 2;
            let mut similar_images = SimilarImages::new(parameters);
            similar_images.set_use_reference_folders(false);

            let fe1 = create_random_file_entry(vec![1, 1, 1, 1, 1, 1, 1, 1], "abc.txt");
            let fe2 = create_random_file_entry(vec![1, 1, 1, 1, 1, 1, 1, 1], "bcd.txt");
            let fe3 = create_random_file_entry(vec![1, 1, 1, 1, 1, 1, 1, 2], "rrd.txt");

            add_hashes(&mut similar_images.image_hashes, vec![fe1, fe2, fe3]);

            similar_images.find_similar_hashes(None, None);
            assert_eq!(similar_images.get_similar_images().len(), 1);
            assert_eq!(similar_images.get_similar_images()[0].len(), 3);
        }
    }

    #[test]
    fn test_simple_referenced_same_group() {
        for _ in 0..100 {
            let mut parameters = get_default_parameters();
            parameters.similarity = 0;
            let mut similar_images = SimilarImages::new(parameters);
            similar_images.set_use_reference_folders(true);
            // Not using special method, because it validates if path exists
            similar_images.common_data.directories.reference_directories = vec![PathBuf::from("/home/rr/")];

            let fe1 = create_random_file_entry(vec![1, 1, 1, 1, 1, 1, 1, 1], "/home/rr/abc.txt");
            let fe2 = create_random_file_entry(vec![1, 1, 1, 1, 1, 1, 1, 1], "/home/rr/bcd.txt");

            add_hashes(&mut similar_images.image_hashes, vec![fe1, fe2]);

            similar_images.find_similar_hashes(None, None);
            assert_eq!(similar_images.get_similar_images().len(), 0);
        }
    }

    #[test]
    fn test_simple_referenced_group_extended() {
        for _ in 0..100 {
            let mut parameters = get_default_parameters();
            parameters.similarity = 0;
            let mut similar_images = SimilarImages::new(parameters);
            similar_images.set_use_reference_folders(true);
            // Not using special method, because it validates if path exists
            similar_images.common_data.directories.reference_directories = vec![PathBuf::from("/home/rr/")];

            let fe1 = create_random_file_entry(vec![1, 1, 1, 1, 1, 1, 1, 1], "/home/rr/abc.txt");
            let fe2 = create_random_file_entry(vec![1, 1, 1, 1, 1, 1, 1, 1], "/home/kk/bcd.txt");

            add_hashes(&mut similar_images.image_hashes, vec![fe1, fe2]);

            similar_images.find_similar_hashes(None, None);
            assert_eq!(similar_images.get_similar_images_referenced().len(), 1);
            assert_eq!(similar_images.get_similar_images_referenced()[0].1.len(), 1);
        }
    }

    #[test]
    fn test_simple_referenced_group_extended2() {
        for _ in 0..100 {
            let mut parameters = get_default_parameters();
            parameters.similarity = 0;
            let mut similar_images = SimilarImages::new(parameters);
            similar_images.set_use_reference_folders(true);
            // Not using special method, because it validates if path exists
            similar_images.common_data.directories.reference_directories = vec![PathBuf::from("/home/rr/")];

            let fe1 = create_random_file_entry(vec![1, 1, 1, 1, 1, 1, 1, 1], "/home/rr/abc.txt");
            let fe2 = create_random_file_entry(vec![1, 1, 1, 1, 1, 1, 1, 1], "/home/rr/abc2.txt");
            let fe3 = create_random_file_entry(vec![1, 1, 1, 1, 1, 1, 1, 1], "/home/kk/bcd.txt");
            let fe4 = create_random_file_entry(vec![1, 1, 1, 1, 1, 1, 1, 1], "/home/kk/bcd2.txt");

            add_hashes(&mut similar_images.image_hashes, vec![fe1, fe2, fe3, fe4]);

            similar_images.find_similar_hashes(None, None);
            let res = similar_images.get_similar_images_referenced();
            assert_eq!(res.len(), 1);
            assert_eq!(res[0].1.len(), 2);
            assert!(res[0].1.iter().all(|e| e.path.starts_with("/home/kk/")));
        }
    }

    #[test]
    fn test_simple_normal_too_small_similarity() {
        for _ in 0..100 {
            let mut parameters = get_default_parameters();
            parameters.similarity = 1;
            let mut similar_images = SimilarImages::new(parameters);
            similar_images.set_use_reference_folders(false);

            let fe1 = create_random_file_entry(vec![1, 1, 1, 1, 1, 1, 1, 0b00001], "abc.txt");
            let fe2 = create_random_file_entry(vec![1, 1, 1, 1, 1, 1, 1, 0b00100], "bcd.txt");
            let fe3 = create_random_file_entry(vec![1, 1, 1, 1, 1, 1, 1, 0b10000], "rrd.txt");

            add_hashes(&mut similar_images.image_hashes, vec![fe1, fe2, fe3]);

            similar_images.find_similar_hashes(None, None);
            let res = similar_images.get_similar_images();
            assert!(res.is_empty());
        }
    }

    #[test]
    fn test_simple_normal_union_of_similarity() {
        for _ in 0..100 {
            let mut parameters = get_default_parameters();
            parameters.similarity = 4;
            let mut similar_images = SimilarImages::new(parameters);
            similar_images.set_use_reference_folders(false);

            let fe1 = create_random_file_entry(vec![1, 1, 1, 1, 1, 1, 1, 0b0000_0001], "abc.txt");
            let fe2 = create_random_file_entry(vec![1, 1, 1, 1, 1, 1, 1, 0b0000_1111], "bcd.txt");
            let fe3 = create_random_file_entry(vec![1, 1, 1, 1, 1, 1, 1, 0b0111_1111], "rrd.txt");

            add_hashes(&mut similar_images.image_hashes, vec![fe1, fe2, fe3]);

            similar_images.find_similar_hashes(None, None);
            let res = similar_images.get_similar_images();
            assert_eq!(res.len(), 1);
            let mut path = res[0].iter().map(|e| e.path.to_string_lossy().to_string()).collect::<Vec<_>>();
            path.sort();
            if res[0].len() == 3 {
                assert_eq!(path, vec!["abc.txt".to_string(), "bcd.txt".to_string(), "rrd.txt".to_string()]);
            } else if res[0].len() == 2 {
                assert!(path == vec!["abc.txt".to_string(), "bcd.txt".to_string()] || path == vec!["bcd.txt".to_string(), "rrd.txt".to_string()]);
            } else {
                panic!("Invalid number of items");
            }
        }
    }

    #[test]
    fn test_reference_similarity_only_one() {
        for _ in 0..100 {
            let mut parameters = get_default_parameters();
            parameters.similarity = 1;
            let mut similar_images = SimilarImages::new(parameters);
            similar_images.set_use_reference_folders(true);
            // Not using special method, because it validates if path exists
            similar_images.common_data.directories.reference_directories = vec![PathBuf::from("/home/rr/")];

            let fe1 = create_random_file_entry(vec![1, 1, 1, 1, 1, 1, 1, 0b0001], "/home/rr/abc.txt");
            let fe2 = create_random_file_entry(vec![1, 1, 1, 1, 1, 1, 1, 0b0011], "/home/kk/bcd.txt");

            add_hashes(&mut similar_images.image_hashes, vec![fe1, fe2]);

            similar_images.find_similar_hashes(None, None);
            let res = similar_images.get_similar_images_referenced();
            assert_eq!(res.len(), 1);
            assert_eq!(res[0].1.len(), 1);
            assert_eq!(res[0].0.path, PathBuf::from("/home/rr/abc.txt"));
            assert_eq!(res[0].1[0].path, PathBuf::from("/home/kk/bcd.txt"));
        }
    }

    #[test]
    fn test_reference_too_small_similarity() {
        for _ in 0..100 {
            let mut parameters = get_default_parameters();
            parameters.similarity = 1;
            let mut similar_images = SimilarImages::new(parameters);
            similar_images.set_use_reference_folders(true);
            // Not using special method, because it validates if path exists
            similar_images.common_data.directories.reference_directories = vec![PathBuf::from("/home/rr/")];

            let fe1 = create_random_file_entry(vec![1, 1, 1, 1, 1, 1, 1, 0b0001], "/home/rr/abc.txt");
            let fe2 = create_random_file_entry(vec![1, 1, 1, 1, 1, 1, 1, 0b0010], "/home/kk/bcd.txt");

            add_hashes(&mut similar_images.image_hashes, vec![fe1, fe2]);

            similar_images.find_similar_hashes(None, None);
            let res = similar_images.get_similar_images_referenced();
            assert_eq!(res.len(), 0);
        }
    }

    #[test]
    fn test_reference_minimal() {
        for _ in 0..100 {
            let mut parameters = get_default_parameters();
            parameters.similarity = 1;
            let mut similar_images = SimilarImages::new(parameters);
            similar_images.set_use_reference_folders(true);
            // Not using special method, because it validates if path exists
            similar_images.common_data.directories.reference_directories = vec![PathBuf::from("/home/rr/")];

            let fe1 = create_random_file_entry(vec![1, 1, 1, 1, 1, 1, 1, 0b0001], "/home/rr/abc.txt");
            let fe2 = create_random_file_entry(vec![1, 1, 1, 1, 1, 1, 1, 0b0011], "/home/kk/bcd.txt");
            let fe3 = create_random_file_entry(vec![1, 1, 1, 1, 1, 1, 1, 0b0100], "/home/kk/bcd2.txt");
            let fe4 = create_random_file_entry(vec![1, 1, 1, 1, 1, 1, 1, 0b1100], "/home/rr/krkr.txt");

            add_hashes(&mut similar_images.image_hashes, vec![fe1, fe2, fe3, fe4]);

            similar_images.find_similar_hashes(None, None);
            let res = similar_images.get_similar_images_referenced();
            assert_eq!(res.len(), 2);
            assert_eq!(res[0].1.len(), 1);
            assert_eq!(res[1].1.len(), 1);
            if res[0].1[0].path == PathBuf::from("/home/kk/bcd.txt") {
                assert_eq!(res[0].0.path, PathBuf::from("/home/rr/abc.txt"));
                assert_eq!(res[1].0.path, PathBuf::from("/home/rr/krkr.txt"));
            } else if res[0].1[0].path == PathBuf::from("/home/kk/bcd2.txt") {
                assert_eq!(res[0].0.path, PathBuf::from("/home/rr/krkr.txt"));
                assert_eq!(res[1].0.path, PathBuf::from("/home/rr/abc.txt"));
            }
        }
    }

    #[test]
    fn test_reference_same() {
        for _ in 0..100 {
            let mut parameters = get_default_parameters();
            parameters.similarity = 1;
            let mut similar_images = SimilarImages::new(parameters);
            similar_images.set_use_reference_folders(true);
            // Not using special method, because it validates if path exists
            similar_images.common_data.directories.reference_directories = vec![PathBuf::from("/home/rr/")];

            let fe1 = create_random_file_entry(vec![1, 1, 1, 1, 1, 1, 1, 1], "/home/rr/abc.txt");
            let fe2 = create_random_file_entry(vec![1, 1, 1, 1, 1, 1, 1, 1], "/home/kk/bcd.txt");

            add_hashes(&mut similar_images.image_hashes, vec![fe1, fe2]);

            similar_images.find_similar_hashes(None, None);
            let res = similar_images.get_similar_images_referenced();
            assert_eq!(res.len(), 1);
            assert_eq!(res[0].1.len(), 1);
        }
    }

    #[test]
    fn test_reference_union() {
        for _ in 0..100 {
            let mut parameters = get_default_parameters();
            parameters.similarity = 10;
            let mut similar_images = SimilarImages::new(parameters);
            similar_images.set_use_reference_folders(true);
            // Not using special method, because it validates if path exists
            similar_images.common_data.directories.reference_directories = vec![PathBuf::from("/home/rr/")];

            let fe0 = create_random_file_entry(vec![1, 1, 1, 1, 1, 1, 1, 0b1000], "/home/rr/abc2.txt");
            let fe1 = create_random_file_entry(vec![1, 1, 1, 1, 1, 1, 1, 0b0001], "/home/rr/abc.txt");
            let fe2 = create_random_file_entry(vec![1, 1, 1, 1, 1, 1, 1, 0b1110], "/home/kk/bcd.txt");
            let fe3 = create_random_file_entry(vec![1, 1, 1, 1, 1, 1, 1, 0b0100], "/home/kk/bcd2.txt");
            let fe4 = create_random_file_entry(vec![1, 1, 1, 1, 1, 1, 1, 0b1100], "/home/rr/krkr.txt");

            add_hashes(&mut similar_images.image_hashes, vec![fe0, fe1, fe2, fe3, fe4]);

            similar_images.find_similar_hashes(None, None);
            let res = similar_images.get_similar_images_referenced();
            assert_eq!(res.len(), 1);
            assert_eq!(res[0].1.len(), 2);
            assert_eq!(res[0].0.path, PathBuf::from("/home/rr/krkr.txt"));
        }
    }

    #[test]
    fn test_tolerance() {
        // This test not really tests anything, but shows that current hamming distance works
        // in bits instead of bytes
        // I tried to make it work in bytes, but it was terrible, so Hamming should be really Ok

        let fe1 = vec![1, 1, 1, 1, 1, 1, 1, 1];
        let fe2 = vec![1, 1, 1, 1, 1, 1, 1, 2];
        let mut bktree = BKTree::new(Hamming);
        bktree.add(fe1);
        let (similarity, _hash) = bktree.find(&fe2, 100).next().expect("No similar images found");
        assert_eq!(similarity, 2);

        let fe1 = vec![1, 1, 1, 1, 1, 1, 1, 1];
        let fe2 = vec![1, 1, 1, 1, 1, 1, 1, 3];
        let mut bktree = BKTree::new(Hamming);
        bktree.add(fe1);
        let (similarity, _hash) = bktree.find(&fe2, 100).next().expect("No similar images found");
        assert_eq!(similarity, 1);

        let fe1 = vec![1, 1, 1, 1, 1, 1, 1, 0b0000_0000];
        let fe2 = vec![1, 1, 1, 1, 1, 1, 1, 0b0000_1000];
        let mut bktree = BKTree::new(Hamming);
        bktree.add(fe1);
        let (similarity, _hash) = bktree.find(&fe2, 100).next().expect("No similar images found");
        assert_eq!(similarity, 1);
    }

    fn add_hashes(hashmap: &mut HashMap<ImHash, Vec<ImagesEntry>>, file_entries: Vec<ImagesEntry>) {
        for fe in file_entries {
            hashmap.entry(fe.hash.clone()).or_default().push(fe);
        }
    }

    fn create_random_file_entry(hash: Vec<u8>, name: &str) -> ImagesEntry {
        ImagesEntry {
            path: PathBuf::from(name.to_string()),
            size: 0,
            width: 100,
            height: 100,
            modified_date: 0,
            hash,
            similarity: 0,
        }
    }
}
