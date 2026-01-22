use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::{mem, panic};

use bk_tree::BKTree;
use crossbeam_channel::Sender;
use fun_time::fun_time;
use image::GenericImageView;
use image_hasher::{FilterType, HashAlg, HasherConfig};
use indexmap::{IndexMap, IndexSet};
use log::{debug, error};
use rayon::prelude::*;

use crate::common::cache::{CACHE_IMAGE_VERSION, load_and_split_cache_generalized_by_path, save_and_connect_cache_generalized_by_path};
use crate::common::dir_traversal::{DirTraversalBuilder, DirTraversalResult, inode, take_1_per_inode};
use crate::common::image::get_dynamic_image_from_path;
use crate::common::model::{ToolType, WorkContinueStatus};
use crate::common::progress_data::{CurrentStage, ProgressData};
use crate::common::progress_stop_handler::{check_if_stop_received, prepare_thread_handler_common};
use crate::common::tool_data::{CommonData, CommonToolData};
use crate::common::traits::ResultEntry;
use crate::flc;
use crate::tools::similar_images::{Hamming, ImHash, ImagesEntry, SIMILAR_VALUES, SimilarImages, SimilarImagesParameters, SimilarityPreset};

impl SimilarImages {
    pub fn new(params: SimilarImagesParameters) -> Self {
        Self {
            common_data: CommonToolData::new(ToolType::SimilarImages),
            information: Default::default(),
            bktree: BKTree::new(Hamming),
            similar_vectors: Vec::new(),
            similar_referenced_vectors: Vec::new(),
            params,
            images_to_check: Default::default(),
            image_hashes: Default::default(),
        }
    }

    #[fun_time(message = "check_for_similar_images", level = "debug")]
    pub(crate) fn check_for_similar_images(&mut self, stop_flag: &Arc<AtomicBool>, progress_sender: Option<&Sender<ProgressData>>) -> WorkContinueStatus {
        let result = DirTraversalBuilder::new()
            .group_by(inode)
            .stop_flag(stop_flag)
            .progress_sender(progress_sender)
            .common_data(&self.common_data)
            .build()
            .run();

        match result {
            DirTraversalResult::SuccessFiles { grouped_file_entries, warnings } => {
                self.images_to_check = grouped_file_entries
                    .into_par_iter()
                    .flat_map(if self.get_params().ignore_hard_links { |(_, fes)| fes } else { take_1_per_inode })
                    .map(|fe| {
                        let fe_str = fe.path.to_string_lossy().to_string();
                        let image_entry = fe.into_images_entry();

                        (fe_str, image_entry)
                    })
                    .collect();

                self.information.initial_found_files = self.images_to_check.len();

                self.common_data.text_messages.warnings.extend(warnings);
                debug!("check_files - Found {} image files.", self.images_to_check.len());
                WorkContinueStatus::Continue
            }

            DirTraversalResult::Stopped => WorkContinueStatus::Stop,
        }
    }

    #[fun_time(message = "hash_images_load_cache", level = "debug")]
    fn hash_images_load_cache(&mut self) -> (BTreeMap<String, ImagesEntry>, BTreeMap<String, ImagesEntry>, BTreeMap<String, ImagesEntry>) {
        load_and_split_cache_generalized_by_path(
            &get_similar_images_cache_file(self.get_params().hash_size, self.get_params().hash_alg, self.get_params().image_filter),
            mem::take(&mut self.images_to_check),
            self,
        )
    }

    #[fun_time(message = "save_to_cache", level = "debug")]
    fn save_to_cache(&mut self, vec_file_entry: &[ImagesEntry], loaded_hash_map: BTreeMap<String, ImagesEntry>) {
        save_and_connect_cache_generalized_by_path(
            &get_similar_images_cache_file(self.get_params().hash_size, self.get_params().hash_alg, self.get_params().image_filter),
            vec_file_entry,
            loaded_hash_map,
            self,
        );
    }

    #[fun_time(message = "hash_images", level = "debug")]
    pub(crate) fn hash_images(&mut self, stop_flag: &Arc<AtomicBool>, progress_sender: Option<&Sender<ProgressData>>) -> WorkContinueStatus {
        if self.images_to_check.is_empty() {
            return WorkContinueStatus::Continue;
        }

        let (loaded_hash_map, records_already_cached, non_cached_files_to_check) = self.hash_images_load_cache();

        let progress_handler = prepare_thread_handler_common(
            progress_sender,
            CurrentStage::SimilarImagesCalculatingHashes,
            non_cached_files_to_check.len(),
            self.get_test_type(),
            non_cached_files_to_check.values().map(|entry| entry.size).sum(),
        );

        debug!("hash_images - start hashing images");
        let (mut vec_file_entry, errors): (Vec<ImagesEntry>, Vec<String>) = non_cached_files_to_check
            .into_par_iter()
            .map(|(_s, file_entry)| {
                if check_if_stop_received(stop_flag) {
                    return None;
                }
                let size = file_entry.size;
                let res = self.collect_image_file_entry(file_entry);
                progress_handler.increase_items(1);
                progress_handler.increase_size(size);

                Some(res)
            })
            .while_some()
            .partition_map(|res| match res {
                Ok(entry) => itertools::Either::Left(entry),
                Err(err) => itertools::Either::Right(err),
            });

        self.common_data.text_messages.errors.extend(errors);
        debug!("hash_images - end hashing {} images", vec_file_entry.len());

        progress_handler.join_thread();

        vec_file_entry.extend(records_already_cached.into_values());

        self.save_to_cache(&vec_file_entry, loaded_hash_map);

        // All valid entries are used to create bktree used to check for hash similarity
        for file_entry in vec_file_entry {
            // Only use to comparing, non broken hashes(all 0 or 255 hashes means that algorithm fails to decode them because e.g. contains a lot of alpha channel)
            if !(file_entry.hash.is_empty() || file_entry.hash.iter().all(|e| *e == 0) || file_entry.hash.iter().all(|e| *e == 255)) {
                self.image_hashes.entry(file_entry.hash.clone()).or_default().push(file_entry);
            }
        }

        // Break if stop was clicked after saving to cache
        if check_if_stop_received(stop_flag) {
            return WorkContinueStatus::Stop;
        }

        WorkContinueStatus::Continue
    }

    fn collect_image_file_entry(&self, mut file_entry: ImagesEntry) -> Result<ImagesEntry, String> {
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

        Ok(file_entry)
    }

    // Split hashes at 2 parts, base hashes and hashes to compare, 3 argument is set of hashes with multiple images
    #[fun_time(message = "split_hashes", level = "debug")]
    fn split_hashes(&mut self, all_hashed_images: &IndexMap<ImHash, Vec<ImagesEntry>>) -> (Vec<ImHash>, IndexSet<ImHash>) {
        let hashes_with_multiple_images: IndexSet<ImHash> = all_hashed_images
            .iter()
            .filter_map(|(hash, vec_file_entry)| {
                if vec_file_entry.len() >= 2 {
                    return Some(hash.clone());
                }
                None
            })
            .collect();
        let mut base_hashes = Vec::new(); // Initial hashes
        if self.common_data.use_reference_folders {
            let mut files_from_referenced_folders: IndexMap<ImHash, Vec<ImagesEntry>> = IndexMap::new();
            let mut normal_files: IndexMap<ImHash, Vec<ImagesEntry>> = IndexMap::new();

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
        hashes_parents: IndexMap<ImHash, u32>,
        hashes_with_multiple_images: &IndexSet<ImHash>,
        all_hashed_images: &IndexMap<ImHash, Vec<ImagesEntry>>,
        collected_similar_images: &mut IndexMap<ImHash, Vec<ImagesEntry>>,
        hashes_similarity: IndexMap<ImHash, (ImHash, u32)>,
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
        all_hashed_images: &IndexMap<ImHash, Vec<ImagesEntry>>,
        collected_similar_images: &mut IndexMap<ImHash, Vec<ImagesEntry>>,
        progress_sender: Option<&Sender<ProgressData>>,
        stop_flag: &Arc<AtomicBool>,
        tolerance: u32,
    ) -> WorkContinueStatus {
        // Don't use hashes with multiple images in bktree, because they will always be master of group and cannot be find by other hashes
        let (base_hashes, hashes_with_multiple_images) = self.split_hashes(all_hashed_images);

        let progress_handler = prepare_thread_handler_common(progress_sender, CurrentStage::SimilarImagesComparingHashes, base_hashes.len(), self.get_test_type(), 0);

        let mut hashes_parents: IndexMap<ImHash, u32> = Default::default(); // Hashes used as parent (hash, children_number_of_hash)
        let mut hashes_similarity: IndexMap<ImHash, (ImHash, u32)> = Default::default(); // Hashes used as child, (parent_hash, similarity)

        // Check them in chunks, to decrease number of used memory
        // Without chunks, every single hash would be compared to every other hash and generate really big amount of results
        // With chunks we can save results to variables and later use such variables, to skip ones with too big difference
        // Not really helpful, when not finding almost any duplicates, but with bigger amount of them, this should help a lot
        let base_hashes_chunks = base_hashes.chunks(1000);
        for chunk in base_hashes_chunks {
            let partial_results = chunk
                .into_par_iter()
                .map(|hash_to_check| {
                    progress_handler.increase_items(1);

                    if check_if_stop_received(stop_flag) {
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
                // TODO - this filter move to into_par_iter above
                .filter(|(original_hash, vec_similar_hashes)| !vec_similar_hashes.is_empty() || hashes_with_multiple_images.contains(*original_hash))
                .collect::<Vec<_>>();

            if check_if_stop_received(stop_flag) {
                progress_handler.join_thread();
                return WorkContinueStatus::Stop;
            }

            SimilarImages::connect_results_simplified(partial_results, &mut hashes_parents, &mut hashes_similarity, &hashes_with_multiple_images);
        }
        // To avoid situations in simplified connector we don't add such hashes to results
        for multiple_image_hash in &hashes_with_multiple_images {
            if !hashes_parents.contains_key(multiple_image_hash) {
                hashes_parents.insert(multiple_image_hash.clone(), 0);
            }
        }

        progress_handler.join_thread();

        debug_check_for_duplicated_things(self.common_data.use_reference_folders, &hashes_parents, &hashes_similarity, all_hashed_images, "LATTER");
        self.collect_hash_compare_result(hashes_parents, &hashes_with_multiple_images, all_hashed_images, collected_similar_images, hashes_similarity);

        WorkContinueStatus::Continue
    }

    fn connect_results_simplified<'a>(
        partial_results: Vec<(&'a ImHash, Vec<(u32, &'a ImHash)>)>,
        hashes_parents: &mut IndexMap<ImHash, u32>,
        hashes_similarity: &mut IndexMap<ImHash, (ImHash, u32)>,
        hashes_with_multiple_images: &IndexSet<ImHash>,
    ) {
        // To simplify later logic, we sort all results by similarity
        // To be able to do this, we need to flatten structure, which will increase memory usage a bit, but should improve a little logic(algorithm is a little broken and works better with sorted data)
        // There can be hashes with multiple similar images, without any similar hashes, so we need to keep them too and add to final results without even checking for parents etc.
        let mut flattened_partial_results: Vec<(&'a ImHash, (u32, &'a ImHash))> = partial_results
            .into_iter()
            .filter_map(|(parent, similar)| {
                if similar.is_empty() {
                    assert!(hashes_with_multiple_images.contains(parent)); // We expect, that only hashes with multiple images can have no similar hashes
                    assert!(!hashes_parents.contains_key(parent)); // We expect, that this hash is not already in parents list - this would be strange, because it have no similar hashes
                    None
                } else {
                    Some(similar.into_iter().map(move |sim| (parent, sim)))
                }
            })
            .flatten()
            .collect::<Vec<_>>();

        flattened_partial_results.sort_by_key(|(_parent, (similarity, _compared_hash))| *similarity);

        // Original hash means, that we check this hash and we can easily find this hash a new parent
        // Compared hash cannot be changed if it is already parent to different hash, because it would be too complex to handle this properly
        for (original_hash, (similarity, compared_hash)) in flattened_partial_results {
            // If compared hash already is parent to different hash, skip it
            // This may be not optimal, because we may miss better parent for such hash, but I have no idea how to properly reparent it
            // This would be hard, because we would need to track all similar hashes for reparented childrens, to find them better parents
            if hashes_parents.contains_key(compared_hash) {
                continue;
            }

            let compared_hash_parent = if let Some((other_parent_hash, other_similarity)) = hashes_similarity.get(compared_hash) {
                if *other_similarity > similarity {
                    Some(other_parent_hash.clone())
                } else {
                    // Have parent, but with lower similarity, so skipping this one
                    continue;
                }
            } else {
                None
            };

            // If current checked hash, have parent, first we must check if similarity between them is lower than checked item
            if let Some((current_parent_hash, current_similarity_with_parent)) = hashes_similarity.get(original_hash) {
                if *current_similarity_with_parent <= similarity {
                    // Have more similar parent, so skip this one
                    continue;
                }

                let children_count = hashes_parents.get_mut(current_parent_hash).expect("Cannot find parent hash");
                *children_count -= 1;
                let left_any_children = *children_count != 0;

                // We can remove entirely previous parent from hashes_parents if it will not have any other children
                // Of course, only if hash applies to single image, because hashes with multiple images must stay in parents list
                if !left_any_children && !hashes_with_multiple_images.contains(current_parent_hash) {
                    hashes_parents.swap_remove(current_parent_hash);
                }
                hashes_similarity
                    .swap_remove(original_hash)
                    .expect("This should never fail, because we are iterating over this hash");

                let parent = hashes_parents.insert((*original_hash).clone(), 1);
                assert!(parent.is_none(), "Parent hash should not exist here");
            } else {
                *hashes_parents.entry(original_hash.clone()).or_insert(0) += 1;
            }

            // This overwrites parent hash if there was any
            // or just adds new record if there was no parent
            hashes_similarity.insert(compared_hash.clone(), (original_hash.clone(), similarity));

            if let Some(compared_hash_parent) = compared_hash_parent {
                *hashes_parents.get_mut(&compared_hash_parent).expect("Cannot find parent hash") -= 1;
            }
        }
    }

    #[fun_time(message = "find_similar_hashes", level = "debug")]
    pub(crate) fn find_similar_hashes(&mut self, stop_flag: &Arc<AtomicBool>, progress_sender: Option<&Sender<ProgressData>>) -> WorkContinueStatus {
        if self.image_hashes.is_empty() {
            return WorkContinueStatus::Continue;
        }

        let tolerance = self.get_params().similarity;

        // Results
        let mut collected_similar_images: IndexMap<ImHash, Vec<ImagesEntry>> = Default::default();

        let all_hashed_images = mem::take(&mut self.image_hashes);

        // Checking entries with tolerance 0 is really easy and fast, because only entries with same hashes needs to be checked
        if tolerance == 0 {
            for (hash, vec_file_entry) in all_hashed_images {
                if vec_file_entry.len() >= 2 {
                    collected_similar_images.insert(hash, vec_file_entry);
                }
            }
        } else if self.compare_hashes_with_non_zero_tolerance(&all_hashed_images, &mut collected_similar_images, progress_sender, stop_flag, tolerance) == WorkContinueStatus::Stop
        {
            return WorkContinueStatus::Stop;
        }

        Self::verify_duplicated_items(&collected_similar_images);

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

        WorkContinueStatus::Continue
    }

    #[fun_time(message = "exclude_items_with_same_size", level = "debug")]
    fn exclude_items_with_same_size(&mut self) {
        if self.get_params().exclude_images_with_same_size {
            for vec_file_entry in mem::take(&mut self.similar_vectors) {
                let mut bt_sizes: BTreeSet<u64> = Default::default();
                let mut vec_values = Vec::new();
                for file_entry in vec_file_entry {
                    if bt_sizes.insert(file_entry.size) {
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

    // TODO this probably not works good when reference folders are used
    pub(crate) fn verify_duplicated_items(collected_similar_images: &IndexMap<ImHash, Vec<ImagesEntry>>) {
        if !cfg!(debug_assertions) {
            return;
        }
        // Validating if group contains duplicated results
        let mut result_hashset: IndexSet<String> = Default::default();
        let mut found = false;

        for vec_file_entry in collected_similar_images.values() {
            if vec_file_entry.is_empty() {
                error!("Found empty group");
                found = true;
                continue;
            }
            if vec_file_entry.len() == 1 {
                error!("Found simple element {vec_file_entry:?}");
                found = true;
                continue;
            }
            for file_entry in vec_file_entry {
                let st = file_entry.path.to_string_lossy().to_string();
                if result_hashset.contains(&st) {
                    found = true;
                    error!("Duplicated Element {st}");
                } else {
                    result_hashset.insert(st);
                }
            }
        }
        assert!(!found, "Found Invalid entries, verify errors before");
    }
}

fn is_in_reference_folder(reference_directories: &[PathBuf], path: &Path) -> bool {
    reference_directories.iter().any(|e| path.starts_with(e))
}

#[expect(clippy::indexing_slicing)] // Because hash size is validated before
pub fn get_string_from_similarity(similarity: u32, hash_size: u8) -> String {
    let index_preset = match hash_size {
        8 => 0,
        16 => 1,
        32 => 2,
        64 => 3,
        _ => panic!("Invalid hash size {hash_size}"),
    };

    if similarity == 0 {
        flc!("core_similarity_original")
    } else if similarity <= SIMILAR_VALUES[index_preset][0] {
        flc!("core_similarity_very_high")
    } else if similarity <= SIMILAR_VALUES[index_preset][1] {
        flc!("core_similarity_high")
    } else if similarity <= SIMILAR_VALUES[index_preset][2] {
        flc!("core_similarity_medium")
    } else if similarity <= SIMILAR_VALUES[index_preset][3] {
        flc!("core_similarity_small")
    } else if similarity <= SIMILAR_VALUES[index_preset][4] {
        flc!("core_similarity_very_small")
    } else if similarity <= SIMILAR_VALUES[index_preset][5] {
        flc!("core_similarity_minimal")
    } else {
        panic!("Invalid similarity value {similarity} for hash size {hash_size} (index {index_preset})");
    }
}

#[expect(clippy::indexing_slicing)] // Because hash size is validated before
pub fn return_similarity_from_similarity_preset(similarity_preset: SimilarityPreset, hash_size: u8) -> u32 {
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

pub(crate) fn convert_filters_to_string(image_filter: FilterType) -> String {
    match image_filter {
        FilterType::Lanczos3 => "Lanczos3",
        FilterType::Nearest => "Nearest",
        FilterType::Triangle => "Triangle",
        FilterType::Gaussian => "Gaussian",
        FilterType::CatmullRom => "CatmullRom",
    }
    .to_string()
}

pub(crate) fn convert_algorithm_to_string(hash_alg: HashAlg) -> String {
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

#[allow(clippy::allow_attributes)]
#[allow(unfulfilled_lint_expectations)] // Happens only on release build
#[expect(dead_code)]
#[expect(unreachable_code)]
#[expect(unused_variables)]
// Function to validate if after first check there are any duplicated entries
// E.g. /a.jpg is used also as master and similar image which is forbidden, because may
// cause accidentally delete more pictures that user wanted
fn debug_check_for_duplicated_things(
    use_reference_folders: bool,
    hashes_parents: &IndexMap<ImHash, u32>,
    hashes_similarity: &IndexMap<ImHash, (ImHash, u32)>,
    all_hashed_images: &IndexMap<ImHash, Vec<ImagesEntry>>,
    numm: &str,
) {
    if !cfg!(debug_assertions) {
        return;
    }

    if use_reference_folders {
        return;
    }

    let mut found_broken_thing = false;
    let mut hashmap_hashes: IndexSet<_> = Default::default();
    let mut hashmap_names: IndexSet<_> = Default::default();
    for (hash, number_of_children) in hashes_parents {
        if *number_of_children > 0 {
            if hashmap_hashes.contains(hash) {
                debug!("------1--HASH--{}  {:?}", numm, all_hashed_images[hash]);
                found_broken_thing = true;
            }
            hashmap_hashes.insert((*hash).clone());

            for i in &all_hashed_images[hash] {
                let name = i.path.to_string_lossy().to_string();
                if hashmap_names.contains(&name) {
                    debug!("------1--NAME--{numm}  {name:?}");
                    found_broken_thing = true;
                }
                hashmap_names.insert(name);
            }
        }
    }
    for hash in hashes_similarity.keys() {
        if hashmap_hashes.contains(hash) {
            debug!("------2--HASH--{}  {:?}", numm, all_hashed_images[hash]);
            found_broken_thing = true;
        }
        hashmap_hashes.insert((*hash).clone());

        for i in &all_hashed_images[hash] {
            let name = i.path.to_string_lossy().to_string();
            if hashmap_names.contains(&name) {
                debug!("------2--NAME--{numm}  {name:?}");
                found_broken_thing = true;
            }
            hashmap_names.insert(name);
        }
    }

    assert!(!found_broken_thing);
}

pub fn get_similar_images_cache_file(hash_size: u8, hash_alg: HashAlg, image_filter: FilterType) -> String {
    format!(
        "cache_similar_images_{hash_size}_{}_{}_{CACHE_IMAGE_VERSION}.bin",
        convert_algorithm_to_string(hash_alg),
        convert_filters_to_string(image_filter),
    )
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use bk_tree::BKTree;
    use image::imageops::FilterType;
    use image_hasher::HashAlg;
    use indexmap::IndexMap;

    use super::*;
    use crate::common::tool_data::CommonData;
    use crate::tools::similar_images::{Hamming, ImHash, ImagesEntry, SimilarImages, SimilarImagesParameters};

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

    // Just to debug changes to algorithms
    // #[test]
    // fn test_fuzzer() {
    //     for _ in 0..100 {
    //         let mut parameters = get_default_parameters();
    //         parameters.similarity = rand::random::<u32>() % 40;
    //         let mut similar_images = SimilarImages::new(parameters);
    //
    //         for i in 0..(rand::random::<u32>() % 2000) {
    //             let mut entry = vec![1u8; 8];
    //             entry[1] = rand::random::<u8>();
    //             if rand::random::<bool>() {
    //                 entry[2] = rand::random::<u8>();
    //             }
    //             if rand::random::<bool>() {
    //                 entry[3] = rand::random::<u8>();
    //             }
    //             if rand::random::<bool>() {
    //                 entry[4] = rand::random::<u8>();
    //             }
    //             let fe = create_random_file_entry(entry, &format!("file_{i}.txt"));
    //             add_hashes(&mut similar_images.image_hashes, vec![fe]);
    //         }
    //
    //         similar_images.find_similar_hashes(&Arc::default(), None);
    //     }
    // }

    #[test]
    fn test_compare_no_images() {
        use crate::common::traits::Search;
        for _ in 0..100 {
            let mut similar_images = SimilarImages::new(get_default_parameters());
            similar_images.search(&Arc::default(), None);
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

            similar_images.find_similar_hashes(&Arc::default(), None);
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

            similar_images.find_similar_hashes(&Arc::default(), None);
            assert_eq!(similar_images.get_similar_images().len(), 1);
        }
    }

    #[test]
    fn test_2000_hashes() {
        let mut parameters = get_default_parameters();
        parameters.similarity = 10;
        let mut similar_images = SimilarImages::new(parameters);

        for i in 0..2000 {
            let mut entry = vec![1u8; 8];
            entry[7] = (i as u32 % 256) as u8;
            entry[6] = (i as u32 / 256 % 256) as u8;
            entry[5] = (i as u32 / 256 / 256 % 256) as u8;
            let fe = create_random_file_entry(entry, &format!("file_{i}.txt"));
            add_hashes(&mut similar_images.image_hashes, vec![fe]);
        }

        similar_images.find_similar_hashes(&Arc::default(), None);
        assert!(!similar_images.get_similar_images().is_empty());
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

            similar_images.find_similar_hashes(&Arc::default(), None);
            assert_eq!(similar_images.get_similar_images().len(), 1);
            assert_eq!(similar_images.get_similar_images()[0].len(), 3);
        }
    }

    #[test]
    fn test_simple_normal_one_group_extended2() {
        for _ in 0..100 {
            let mut parameters = get_default_parameters();
            parameters.similarity = 222222;
            let mut similar_images = SimilarImages::new(parameters);
            similar_images.set_use_reference_folders(false);

            let fe1 = create_random_file_entry(vec![59, 41, 53, 27, 19, 143, 228, 228], "abc.txt");
            let fe2 = create_random_file_entry(vec![57, 41, 60, 155, 51, 173, 204, 228], "bcd.txt");
            let fe3 = create_random_file_entry(vec![28, 222, 206, 192, 203, 157, 25, 24], "rrd.txt");

            add_hashes(&mut similar_images.image_hashes, vec![fe1, fe2, fe3]);

            similar_images.find_similar_hashes(&Arc::default(), None);
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

            similar_images.find_similar_hashes(&Arc::default(), None);
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

            similar_images.find_similar_hashes(&Arc::default(), None);
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

            similar_images.find_similar_hashes(&Arc::default(), None);
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

            similar_images.find_similar_hashes(&Arc::default(), None);
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

            similar_images.find_similar_hashes(&Arc::default(), None);
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

            similar_images.find_similar_hashes(&Arc::default(), None);
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

            similar_images.find_similar_hashes(&Arc::default(), None);
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

            similar_images.find_similar_hashes(&Arc::default(), None);
            let res = similar_images.get_similar_images_referenced();
            assert_eq!(res.len(), 2);
            assert_eq!(res[0].1.len(), 1);
            assert_eq!(res[1].1.len(), 1);
            #[allow(clippy::allow_attributes)]
            #[allow(clippy::cmp_owned)] // TODO Bug in nightly
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

            similar_images.find_similar_hashes(&Arc::default(), None);
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

            similar_images.find_similar_hashes(&Arc::default(), None);
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

    fn add_hashes(hashmap: &mut IndexMap<ImHash, Vec<ImagesEntry>>, file_entries: Vec<ImagesEntry>) {
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

#[cfg(test)]
mod connect_results_tests {
    use image_hasher::{FilterType, HashAlg};
    use indexmap::{IndexMap, IndexSet};

    use super::*;

    #[test]
    fn test_connect_results_real_case() {
        let params = SimilarImagesParameters::new(10, 8, HashAlg::Gradient, FilterType::Lanczos3, false, true);
        let _finder = SimilarImages::new(params);

        let hash1: ImHash = vec![59, 41, 53, 27, 19, 143, 228, 228];
        let hash2: ImHash = vec![57, 41, 60, 155, 51, 173, 204, 228];
        let hash3: ImHash = vec![28, 222, 206, 192, 203, 157, 25, 24];

        let partial_results = vec![
            (&hash1, vec![(9, &hash2), (43, &hash3)]),
            (&hash2, vec![(9, &hash1), (38, &hash3)]),
            (&hash3, vec![(38, &hash2), (43, &hash1)]),
        ];

        let mut hashes_parents: IndexMap<ImHash, u32> = IndexMap::new();
        let mut hashes_similarity: IndexMap<ImHash, (ImHash, u32)> = IndexMap::new();
        let hashes_with_multiple_images: IndexSet<ImHash> = IndexSet::new();

        assert_eq!(hashes_parents.len(), 0);
        assert_eq!(hashes_similarity.len(), 0);

        SimilarImages::connect_results_simplified(partial_results, &mut hashes_parents, &mut hashes_similarity, &hashes_with_multiple_images);

        assert_eq!(hashes_parents.len(), 1);
        assert_eq!(hashes_similarity.len(), 2);
    }
}
