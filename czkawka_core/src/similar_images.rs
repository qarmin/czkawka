use crate::common::Common;
use crate::common_directory::Directories;
use crate::common_items::ExcludedItems;
use crate::common_messages::Messages;
use crate::common_traits::{DebugPrint, PrintResults, SaveResults};
use bk_tree::BKTree;
use crossbeam_channel::Receiver;
use directories_next::ProjectDirs;
use humansize::{file_size_opts as options, FileSize};
use image::GenericImageView;
use img_hash::{FilterType, HashAlg, HasherConfig};
use rayon::prelude::*;
use std::collections::{BTreeMap, BTreeSet};
use std::fs::OpenOptions;
use std::fs::{File, Metadata};
use std::io::Write;
use std::io::*;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread::sleep;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::{fs, mem, thread};

// TODO check for better values
pub const SIMILAR_VALUES: [[u32; 6]; 3] = [
    [0, 1, 2, 3, 4, 5],     // 4 - Max 16
    [0, 2, 5, 7, 14, 20],   // 8 - Max 256
    [2, 5, 10, 20, 40, 80], // 16 - Max 65536
];

#[derive(Debug)]
pub struct ProgressData {
    pub current_stage: u8,
    pub max_stage: u8,
    pub images_checked: usize,
    pub images_to_check: usize,
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub enum Similarity {
    None,
    Similar(u32),
}

#[derive(Clone, Debug)]
pub struct FileEntry {
    pub path: PathBuf,
    pub size: u64,
    pub dimensions: String,
    pub modified_date: u64,
    pub hash: Vec<u8>,
    pub similarity: Similarity,
}

// This is used by CLI tool when we cann
#[derive(Clone, Debug)]
pub enum SimilarityPreset {
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
    excluded_items: ExcludedItems,
    bktree: BKTree<Vec<u8>, Hamming>,
    similar_vectors: Vec<Vec<FileEntry>>,
    recursive_search: bool,
    minimal_file_size: u64,
    maximal_file_size: u64,
    image_hashes: BTreeMap<Vec<u8>, Vec<FileEntry>>, // Hashmap with image hashes and Vector with names of files
    stopped_search: bool,
    similarity: Similarity,
    images_to_check: BTreeMap<String, FileEntry>,
    hash_size: u8,
    hash_alg: HashAlg,
    image_filter: FilterType,
    use_cache: bool,
}

/// Info struck with helpful information's about results
#[derive(Default)]
pub struct Info {
    pub number_of_removed_files: usize,
    pub number_of_failed_to_remove_files: usize,
    pub gained_space: u64,
}
impl Info {
    pub fn new() -> Self {
        Default::default()
    }
}

/// Method implementation for EmptyFolder
impl SimilarImages {
    /// New function providing basics values
    pub fn new() -> Self {
        Self {
            information: Default::default(),
            text_messages: Messages::new(),
            directories: Directories::new(),
            excluded_items: Default::default(),
            bktree: BKTree::new(Hamming),
            similar_vectors: vec![],
            recursive_search: true,
            minimal_file_size: 1024 * 16, // 16 KB should be enough to exclude too small images from search
            maximal_file_size: u64::MAX,
            image_hashes: Default::default(),
            stopped_search: false,
            similarity: Similarity::Similar(1),
            images_to_check: Default::default(),
            hash_size: 8,
            hash_alg: HashAlg::Gradient,
            image_filter: FilterType::Lanczos3,
            use_cache: true,
        }
    }

    pub fn set_hash_size(&mut self, hash_size: u8) {
        self.hash_size = match hash_size {
            4 | 8 | 16 => hash_size,
            e => {
                panic!("Invalid value of hash size {}", e);
            }
        }
    }

    pub fn set_hash_alg(&mut self, hash_alg: HashAlg) {
        self.hash_alg = hash_alg;
    }

    pub fn set_image_filter(&mut self, image_filter: FilterType) {
        self.image_filter = image_filter;
    }

    pub fn get_stopped_search(&self) -> bool {
        self.stopped_search
    }

    pub const fn get_text_messages(&self) -> &Messages {
        &self.text_messages
    }

    pub const fn get_similar_images(&self) -> &Vec<Vec<FileEntry>> {
        &self.similar_vectors
    }

    pub const fn get_information(&self) -> &Info {
        &self.information
    }

    pub fn set_use_cache(&mut self, use_cache: bool) {
        self.use_cache = use_cache;
    }

    pub fn set_recursive_search(&mut self, recursive_search: bool) {
        self.recursive_search = recursive_search;
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
    pub fn set_similarity(&mut self, similarity: Similarity) {
        self.similarity = similarity;
    }

    /// Public function used by CLI to search for empty folders
    pub fn find_similar_images(&mut self, stop_receiver: Option<&Receiver<()>>, progress_sender: Option<&futures::channel::mpsc::UnboundedSender<ProgressData>>) {
        self.directories.optimize_directories(true, &mut self.text_messages);
        if !self.check_for_similar_images(stop_receiver, progress_sender) {
            self.stopped_search = true;
            return;
        }
        if !self.sort_images(stop_receiver, progress_sender) {
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
    /// Parameter initial_checking for second check before deleting to be sure that checked folder is still empty
    fn check_for_similar_images(&mut self, stop_receiver: Option<&Receiver<()>>, progress_sender: Option<&futures::channel::mpsc::UnboundedSender<ProgressData>>) -> bool {
        let start_time: SystemTime = SystemTime::now();
        let mut folders_to_check: Vec<PathBuf> = Vec::with_capacity(1024 * 2); // This should be small enough too not see to big difference and big enough to store most of paths without needing to resize vector

        // Add root folders for finding
        for id in &self.directories.included_directories {
            folders_to_check.push(id.clone());
        }

        //// PROGRESS THREAD START
        const LOOP_DURATION: u32 = 200; //in ms
        let progress_thread_run = Arc::new(AtomicBool::new(true));

        let atomic_file_counter = Arc::new(AtomicUsize::new(0));

        let progress_thread_handle;
        if let Some(progress_sender) = progress_sender {
            let progress_send = progress_sender.clone();
            let progress_thread_run = progress_thread_run.clone();
            let atomic_file_counter = atomic_file_counter.clone();
            progress_thread_handle = thread::spawn(move || loop {
                progress_send
                    .unbounded_send(ProgressData {
                        current_stage: 0,
                        max_stage: 1,
                        images_checked: atomic_file_counter.load(Ordering::Relaxed) as usize,
                        images_to_check: 0,
                    })
                    .unwrap();
                if !progress_thread_run.load(Ordering::Relaxed) {
                    break;
                }
                sleep(Duration::from_millis(LOOP_DURATION as u64));
            });
        } else {
            progress_thread_handle = thread::spawn(|| {});
        }
        //// PROGRESS THREAD END

        while !folders_to_check.is_empty() {
            if stop_receiver.is_some() && stop_receiver.unwrap().try_recv().is_ok() {
                // End thread which send info to gui
                progress_thread_run.store(false, Ordering::Relaxed);
                progress_thread_handle.join().unwrap();
                return false;
            }
            let current_folder = folders_to_check.pop().unwrap();

            // Read current dir, if permission are denied just go to next
            let read_dir = match fs::read_dir(&current_folder) {
                Ok(t) => t,
                Err(e) => {
                    self.text_messages.warnings.push(format!("Cannot open dir {}, reason {}", current_folder.display(), e));
                    continue;
                } // Permissions denied
            };

            // Check every sub folder/file/link etc.
            'dir: for entry in read_dir {
                let entry_data = match entry {
                    Ok(t) => t,
                    Err(e) => {
                        self.text_messages.warnings.push(format!("Cannot read entry in dir {}, reason {}", current_folder.display(), e));
                        continue;
                    } //Permissions denied
                };
                let metadata: Metadata = match entry_data.metadata() {
                    Ok(t) => t,
                    Err(e) => {
                        self.text_messages.warnings.push(format!("Cannot read metadata in dir {}, reason {}", current_folder.display(), e));
                        continue;
                    } //Permissions denied
                };
                if metadata.is_dir() {
                    if !self.recursive_search {
                        continue;
                    }

                    let next_folder = current_folder.join(entry_data.file_name());
                    if self.directories.is_excluded(&next_folder) {
                        continue 'dir;
                    }

                    if self.excluded_items.is_excluded(&next_folder) {
                        continue 'dir;
                    }

                    folders_to_check.push(next_folder);
                } else if metadata.is_file() {
                    atomic_file_counter.fetch_add(1, Ordering::Relaxed);

                    let file_name_lowercase: String = match entry_data.file_name().into_string() {
                        Ok(t) => t,
                        Err(_inspected) => {
                            println!("File {:?} has not valid UTF-8 name", entry_data);
                            continue 'dir;
                        }
                    }
                    .to_lowercase();

                    // Checking allowed image extensions
                    let allowed_image_extensions = [".jpg", ".jpeg", ".png" /*, ".bmp"*/, ".tiff", ".tif", ".tga", ".ff" /*, ".gif"*/, ".jif", ".jfi" /*, ".webp"*/]; // webp cannot be seen in preview, gif needs to be enabled after releasing image crate 0.24.0, bmp needs to be fixed in image crate
                    if !allowed_image_extensions.iter().any(|e| file_name_lowercase.ends_with(e)) {
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
                            dimensions: "".to_string(),
                            modified_date: match metadata.modified() {
                                Ok(t) => match t.duration_since(UNIX_EPOCH) {
                                    Ok(d) => d.as_secs(),
                                    Err(_inspected) => {
                                        self.text_messages.warnings.push(format!("File {} seems to be modified before Unix Epoch.", current_file_name.display()));
                                        0
                                    }
                                },
                                Err(e) => {
                                    self.text_messages.warnings.push(format!("Unable to get modification date from file {}, reason {}", current_file_name.display(), e));
                                    0
                                } // Permissions Denied
                            },

                            hash: Vec::new(),
                            similarity: Similarity::None,
                        };

                        self.images_to_check.insert(current_file_name.to_string_lossy().to_string(), fe);
                    }
                }
            }
        }
        // End thread which send info to gui
        progress_thread_run.store(false, Ordering::Relaxed);
        progress_thread_handle.join().unwrap();
        Common::print_time(start_time, SystemTime::now(), "check_for_similar_images".to_string());
        true
    }

    // Cache algorithm:
    // - Load data from file
    // - Remove from data to search this already loaded entries(size of image must match)
    // - Check hash of files which doesn't have saved entry
    // - Join already read hashes with hashes which were read from file
    // - Join all hashes and save it to file

    fn sort_images(&mut self, stop_receiver: Option<&Receiver<()>>, progress_sender: Option<&futures::channel::mpsc::UnboundedSender<ProgressData>>) -> bool {
        let hash_map_modification = SystemTime::now();

        let loaded_hash_map;

        let mut records_already_cached: BTreeMap<String, FileEntry> = Default::default();
        let mut non_cached_files_to_check: BTreeMap<String, FileEntry> = Default::default();

        if self.use_cache {
            loaded_hash_map = match load_hashes_from_file(&mut self.text_messages, self.hash_size, self.hash_alg, self.image_filter) {
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

        Common::print_time(hash_map_modification, SystemTime::now(), "sort_images - reading data from cache and preparing them".to_string());
        let hash_map_modification = SystemTime::now();

        //// PROGRESS THREAD START
        const LOOP_DURATION: u32 = 200; //in ms
        let progress_thread_run = Arc::new(AtomicBool::new(true));

        let atomic_file_counter = Arc::new(AtomicUsize::new(0));

        let progress_thread_handle;
        if let Some(progress_sender) = progress_sender {
            let progress_send = progress_sender.clone();
            let progress_thread_run = progress_thread_run.clone();
            let atomic_file_counter = atomic_file_counter.clone();
            let images_to_check = non_cached_files_to_check.len();
            progress_thread_handle = thread::spawn(move || loop {
                progress_send
                    .unbounded_send(ProgressData {
                        current_stage: 1,
                        max_stage: 1,
                        images_checked: atomic_file_counter.load(Ordering::Relaxed) as usize,
                        images_to_check,
                    })
                    .unwrap();
                if !progress_thread_run.load(Ordering::Relaxed) {
                    break;
                }
                sleep(Duration::from_millis(LOOP_DURATION as u64));
            });
        } else {
            progress_thread_handle = thread::spawn(|| {});
        }
        //// PROGRESS THREAD END
        let mut vec_file_entry: Vec<(FileEntry, Vec<u8>)> = non_cached_files_to_check
            .par_iter()
            .map(|file_entry| {
                atomic_file_counter.fetch_add(1, Ordering::Relaxed);
                if stop_receiver.is_some() && stop_receiver.unwrap().try_recv().is_ok() {
                    // This will not break
                    return None;
                }
                let mut file_entry = file_entry.1.clone();

                let image = match image::open(file_entry.path.clone()) {
                    Ok(t) => t,
                    Err(_inspected) => return Some(None), // Something is wrong with image
                };
                let dimensions = image.dimensions();

                file_entry.dimensions = format!("{}x{}", dimensions.0, dimensions.1);

                let hasher_config = HasherConfig::new().hash_size(self.hash_size as u32, self.hash_size as u32).hash_alg(self.hash_alg).resize_filter(self.image_filter);
                let hasher = hasher_config.to_hasher();

                let hash = hasher.hash_image(&image);
                let buf: Vec<u8> = hash.as_bytes().to_vec();

                // Images with hashes with full of 0 or 255 usually means that algorithm fails to decode them because e.g. contains a log of alpha channel
                {
                    if buf.iter().all(|e| *e == 0) {
                        return Some(None);
                    }
                    if buf.iter().all(|e| *e == 255) {
                        return Some(None);
                    }
                }

                file_entry.hash = buf.clone();

                Some(Some((file_entry, buf)))
            })
            .while_some()
            .filter(|file_entry| file_entry.is_some())
            .map(|file_entry| file_entry.unwrap())
            .collect::<Vec<(FileEntry, Vec<u8>)>>();

        // End thread which send info to gui
        progress_thread_run.store(false, Ordering::Relaxed);
        progress_thread_handle.join().unwrap();

        Common::print_time(hash_map_modification, SystemTime::now(), "sort_images - reading data from files in parallel".to_string());
        let hash_map_modification = SystemTime::now();

        // Just connect loaded results with already calculated hashes
        for (_name, file_entry) in records_already_cached {
            vec_file_entry.push((file_entry.clone(), file_entry.hash));
        }

        for (file_entry, buf) in &vec_file_entry {
            self.bktree.add(buf.clone());
            self.image_hashes.entry(buf.clone()).or_insert_with(Vec::<FileEntry>::new);
            self.image_hashes.get_mut(buf).unwrap().push(file_entry.clone());
        }

        if self.use_cache {
            // Must save all results to file, old loaded from file with all currently counted results
            let mut all_results: BTreeMap<String, FileEntry> = loaded_hash_map;
            for (file_entry, _hash) in vec_file_entry {
                all_results.insert(file_entry.path.to_string_lossy().to_string(), file_entry);
            }
            save_hashes_to_file(&all_results, &mut self.text_messages, self.hash_size, self.hash_alg, self.image_filter);
        }

        Common::print_time(hash_map_modification, SystemTime::now(), "sort_images - saving data to files".to_string());
        let hash_map_modification = SystemTime::now();

        let similarity: u32 = match self.similarity {
            Similarity::Similar(k) => k,
            _ => panic!(),
        };

        // TODO
        // Maybe also add here progress report

        let mut collected_similar_images: BTreeMap<Vec<u8>, Vec<FileEntry>> = Default::default();

        let mut available_hashes = self.image_hashes.clone();
        let mut this_time_check_hashes;
        let mut master_of_group: BTreeSet<Vec<u8>> = Default::default(); // Lista wszystkich głównych hashy, które odpowiadają za porównywanie

        // TODO optimize this for big temp_max_similarity values
        // TODO maybe Simialar(u32) is enough instead SIMILAR_VALUES value?
        let temp_max_similarity = match self.hash_size {
            4 => SIMILAR_VALUES[0][5],
            8 => SIMILAR_VALUES[1][5],
            16 => SIMILAR_VALUES[2][5],
            _ => panic!(),
        };

        for current_similarity in 0..=temp_max_similarity {
            this_time_check_hashes = available_hashes.clone();

            if stop_receiver.is_some() && stop_receiver.unwrap().try_recv().is_ok() {
                return false;
            }

            for (hash, vec_file_entry) in this_time_check_hashes.iter() {
                let vector_with_found_similar_hashes = self
                    .bktree
                    .find(hash, similarity)
                    .filter(|r| (r.0 == current_similarity) && !master_of_group.contains(r.1) && available_hashes.contains_key(r.1))
                    .collect::<Vec<_>>();

                // Not found any hash with specific distance
                if vector_with_found_similar_hashes.is_empty() {
                    continue;
                }

                // This one picture doesn't have similar pictures except self in similarity 0
                if current_similarity == 0 && vector_with_found_similar_hashes.len() == 1 {
                    continue;
                }

                // Jeśli jeszcze nie dodał, to dodaje teraz grupę główną do już obrobionych
                if !master_of_group.contains(hash) {
                    master_of_group.insert(hash.clone());
                    collected_similar_images.insert(hash.clone(), Vec::new());

                    let mut things: Vec<FileEntry> = vec_file_entry
                        .iter()
                        .map(|fe| FileEntry {
                            path: fe.path.clone(),
                            size: fe.size,
                            dimensions: fe.dimensions.clone(),
                            modified_date: fe.modified_date,
                            hash: fe.hash.clone(),
                            similarity: Similarity::Similar(0),
                        })
                        .collect();
                    collected_similar_images.get_mut(hash).unwrap().append(&mut things);
                }

                // Since we checked hash, we don't need to check it again
                if current_similarity != 0 {
                    vector_with_found_similar_hashes.iter().for_each(|e| {
                        let mut things: Vec<FileEntry> = available_hashes
                            .get_mut(e.1)
                            .unwrap()
                            .iter()
                            .map(|fe| FileEntry {
                                path: fe.path.clone(),
                                size: fe.size,
                                dimensions: fe.dimensions.clone(),
                                modified_date: fe.modified_date,
                                hash: Vec::new(),
                                similarity: Similarity::Similar(current_similarity),
                            })
                            .collect::<Vec<_>>();
                        collected_similar_images.get_mut(hash).unwrap().append(&mut things);
                        available_hashes.remove(e.1);
                    });
                }
            }
        }

        self.similar_vectors = collected_similar_images.values().cloned().collect();

        Common::print_time(hash_map_modification, SystemTime::now(), "sort_images - selecting data from BtreeMap".to_string());

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

    pub fn set_excluded_directory(&mut self, excluded_directory: Vec<PathBuf>) {
        self.directories.set_excluded_directory(excluded_directory, &mut self.text_messages);
    }

    pub fn set_excluded_items(&mut self, excluded_items: Vec<String>) {
        self.excluded_items.set_excluded_items(excluded_items, &mut self.text_messages);
    }
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
                self.text_messages.errors.push(format!("Failed to create file {}, reason {}", file_name, e));
                return false;
            }
        };
        let mut writer = BufWriter::new(file_handler);

        if let Err(e) = writeln!(
            writer,
            "Results of searching {:?} with excluded directories {:?} and excluded items {:?}",
            self.directories.included_directories, self.directories.excluded_directories, self.excluded_items.items
        ) {
            self.text_messages.errors.push(format!("Failed to save results to file {}, reason {}", file_name, e));
            return false;
        }

        if !self.similar_vectors.is_empty() {
            write!(writer, "{} images which have similar friends\n\n", self.similar_vectors.len()).unwrap();

            for struct_similar in self.similar_vectors.iter() {
                writeln!(writer, "Found {} images which have similar friends", self.similar_vectors.len()).unwrap();
                for file_entry in struct_similar {
                    writeln!(
                        writer,
                        "{} - {} - {} - {}",
                        file_entry.path.display(),
                        file_entry.dimensions,
                        file_entry.size.file_size(options::BINARY).unwrap(),
                        get_string_from_similarity(&file_entry.similarity, self.hash_size)
                    )
                    .unwrap();
                }
                writeln!(writer).unwrap();
            }
        } else {
            write!(writer, "Not found any similar images.").unwrap();
        }

        Common::print_time(start_time, SystemTime::now(), "save_results_to_file".to_string());
        true
    }
}
impl PrintResults for SimilarImages {
    /// Prints basic info about empty folders // TODO print better
    fn print_results(&self) {
        if !self.similar_vectors.is_empty() {
            println!("Found {} images which have similar friends", self.similar_vectors.len());

            for vec_file_entry in &self.similar_vectors {
                for file_entry in vec_file_entry {
                    println!(
                        "{} - {} - {} - {}",
                        file_entry.path.display(),
                        file_entry.dimensions,
                        file_entry.size.file_size(options::BINARY).unwrap(),
                        get_string_from_similarity(&file_entry.similarity, self.hash_size)
                    );
                }
                println!();
            }
        }
    }
}

fn save_hashes_to_file(hashmap: &BTreeMap<String, FileEntry>, text_messages: &mut Messages, hash_size: u8, hash_alg: HashAlg, image_filter: FilterType) {
    if let Some(proj_dirs) = ProjectDirs::from("pl", "Qarmin", "Czkawka") {
        // Lin: /home/username/.cache/czkawka
        // Win: C:\Users\Username\AppData\Local\Qarmin\Czkawka\cache
        // Mac: /Users/Username/Library/Caches/pl.Qarmin.Czkawka

        let cache_dir = PathBuf::from(proj_dirs.cache_dir());
        if cache_dir.exists() {
            if !cache_dir.is_dir() {
                text_messages.messages.push(format!("Config dir {} is a file!", cache_dir.display()));
                return;
            }
        } else if let Err(e) = fs::create_dir_all(&cache_dir) {
            text_messages.messages.push(format!("Cannot create config dir {}, reason {}", cache_dir.display(), e));
            return;
        }
        let cache_file = cache_dir.join(get_cache_file(&hash_size, &hash_alg, &image_filter));
        let file_handler = match OpenOptions::new().truncate(true).write(true).create(true).open(&cache_file) {
            Ok(t) => t,
            Err(e) => {
                text_messages.messages.push(format!("Cannot create or open cache file {}, reason {}", cache_file.display(), e));
                return;
            }
        };
        let mut writer = BufWriter::new(file_handler);

        for file_entry in hashmap.values() {
            let mut string: String = String::with_capacity(100);

            string += format!("{}//{}//{}//{}//", file_entry.path.display(), file_entry.size, file_entry.dimensions, file_entry.modified_date).as_str();

            for i in 0..file_entry.hash.len() - 1 {
                string.push_str(file_entry.hash[i].to_string().as_str());
                string.push_str("//");
            }
            string += file_entry.hash[file_entry.hash.len() - 1].to_string().as_str();

            if let Err(e) = writeln!(writer, "{}", string) {
                text_messages.messages.push(format!("Failed to save some data to cache file {}, reason {}", cache_file.display(), e));
                return;
            };
        }
    }
}
fn load_hashes_from_file(text_messages: &mut Messages, hash_size: u8, hash_alg: HashAlg, image_filter: FilterType) -> Option<BTreeMap<String, FileEntry>> {
    if let Some(proj_dirs) = ProjectDirs::from("pl", "Qarmin", "Czkawka") {
        let cache_dir = PathBuf::from(proj_dirs.cache_dir());
        let cache_file = cache_dir.join(get_cache_file(&hash_size, &hash_alg, &image_filter));
        let file_handler = match OpenOptions::new().read(true).open(&cache_file) {
            Ok(t) => t,
            Err(_inspected) => {
                // text_messages.messages.push(format!("Cannot find or open cache file {}", cache_file.display())); // This shouldn't be write to output
                return None;
            }
        };

        let reader = BufReader::new(file_handler);

        let mut hashmap_loaded_entries: BTreeMap<String, FileEntry> = Default::default();

        let number_of_results: usize = hash_size as usize * hash_size as usize / 8;

        // Read the file line by line using the lines() iterator from std::io::BufRead.
        for (index, line) in reader.lines().enumerate() {
            let line = match line {
                Ok(t) => t,
                Err(e) => {
                    text_messages.warnings.push(format!("Failed to load line number {} from cache file {}, reason {}", index + 1, cache_file.display(), e));
                    return None;
                }
            };
            let uuu = line.split("//").collect::<Vec<&str>>();
            if uuu.len() != (number_of_results + 4) {
                text_messages.warnings.push(format!(
                    "Found invalid data in line {} - ({}) in cache file {}, expected {} values, found {}",
                    index + 1,
                    line,
                    cache_file.display(),
                    uuu.len(),
                    number_of_results + 4
                ));
                continue;
            }
            // Don't load cache data if destination file not exists
            if Path::new(uuu[0]).exists() {
                let mut hash: Vec<u8> = Vec::new();
                for i in 0..number_of_results {
                    hash.push(match uuu[4 + i as usize].parse::<u8>() {
                        Ok(t) => t,
                        Err(e) => {
                            text_messages
                                .warnings
                                .push(format!("Found invalid hash value in line {} - ({}) in cache file {}, reason {}", index + 1, line, cache_file.display(), e));
                            continue;
                        }
                    });
                }

                #[cfg(debug_assertions)]
                {
                    let mut have_at_least: u8 = 0;
                    for i in hash.iter() {
                        if *i == 0 {
                            have_at_least += 1;
                        }
                    }
                    if have_at_least == hash.len() as u8 {
                        println!("ERROR START - {}", line);
                        println!("have_at_least == hash.len() as u8");
                        println!("ERROR END hash.len() - {} == have_at_least - {}", hash.len(), have_at_least);
                        continue; // Just skip this entry, it is very very unlikelly that something have this hash, but if it has, then just ignore it
                    }
                }

                hashmap_loaded_entries.insert(
                    uuu[0].to_string(),
                    FileEntry {
                        path: PathBuf::from(uuu[0]),
                        size: match uuu[1].parse::<u64>() {
                            Ok(t) => t,
                            Err(e) => {
                                text_messages
                                    .warnings
                                    .push(format!("Found invalid size value in line {} - ({}) in cache file {}, reason {}", index + 1, line, cache_file.display(), e));
                                continue;
                            }
                        },
                        dimensions: uuu[2].to_string(),
                        modified_date: match uuu[3].parse::<u64>() {
                            Ok(t) => t,
                            Err(e) => {
                                text_messages
                                    .warnings
                                    .push(format!("Found invalid modified date value in line {} - ({}) in cache file {}, reason {}", index + 1, line, cache_file.display(), e));
                                continue;
                            }
                        },
                        hash,
                        similarity: Similarity::None,
                    },
                );
            }
        }

        return Some(hashmap_loaded_entries);
    }

    text_messages.messages.push("Cannot find or open system config dir to save cache file".to_string());
    None
}

fn get_cache_file(hash_size: &u8, hash_alg: &HashAlg, image_filter: &FilterType) -> String {
    format!("cache_similar_images_{}_{}_{}.txt", hash_size, convert_algorithm_to_string(hash_alg), convert_filters_to_string(image_filter))
}

pub fn get_string_from_similarity(similarity: &Similarity, hash_size: u8) -> String {
    let index_preset = match hash_size {
        4 => 0,
        8 => 1,
        16 => 2,
        _ => panic!(),
    };

    match similarity {
        Similarity::None => {
            panic!()
        }
        Similarity::Similar(h) => {
            #[cfg(debug_assertions)]
            {
                if *h <= SIMILAR_VALUES[index_preset][0] {
                    format!("Very High {}", *h)
                } else if *h <= SIMILAR_VALUES[index_preset][1] {
                    format!("High {}", *h)
                } else if *h <= SIMILAR_VALUES[index_preset][2] {
                    format!("Medium {}", *h)
                } else if *h <= SIMILAR_VALUES[index_preset][3] {
                    format!("Small {}", *h)
                } else if *h <= SIMILAR_VALUES[index_preset][4] {
                    format!("Very Small {}", *h)
                } else if *h <= SIMILAR_VALUES[index_preset][5] {
                    format!("Minimal {}", *h)
                } else {
                    panic!();
                }
            }
            #[cfg(not(debug_assertions))]
            {
                if *h <= SIMILAR_VALUES[index_preset][0] {
                    format!("Very High")
                } else if *h <= SIMILAR_VALUES[index_preset][1] {
                    format!("High")
                } else if *h <= SIMILAR_VALUES[index_preset][2] {
                    format!("Medium")
                } else if *h <= SIMILAR_VALUES[index_preset][3] {
                    format!("Small")
                } else if *h <= SIMILAR_VALUES[index_preset][4] {
                    format!("Very Small")
                } else if *h <= SIMILAR_VALUES[index_preset][5] {
                    format!("Minimal")
                } else {
                    panic!();
                }
            }
        }
    }
}

pub fn return_similarity_from_similarity_preset(similarity_preset: &SimilarityPreset, hash_size: u8) -> Similarity {
    let index_preset = match hash_size {
        4 => 0,
        8 => 1,
        16 => 2,
        _ => panic!(),
    };
    match similarity_preset {
        SimilarityPreset::VeryHigh => Similarity::Similar(SIMILAR_VALUES[index_preset][0]),
        SimilarityPreset::High => Similarity::Similar(SIMILAR_VALUES[index_preset][1]),
        SimilarityPreset::Medium => Similarity::Similar(SIMILAR_VALUES[index_preset][2]),
        SimilarityPreset::Small => Similarity::Similar(SIMILAR_VALUES[index_preset][3]),
        SimilarityPreset::VerySmall => Similarity::Similar(SIMILAR_VALUES[index_preset][4]),
        SimilarityPreset::Minimal => Similarity::Similar(SIMILAR_VALUES[index_preset][5]),
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
        HashAlg::__Nonexhaustive => panic!(),
    }
    .to_string()
}

pub fn test_image_conversion_speed() {
    let file_name: &str = "test.jpg";
    let file_path = Path::new(file_name);
    match image::open(file_path) {
        Ok(img_open) => {
            for alg in [HashAlg::Blockhash, HashAlg::Gradient, HashAlg::DoubleGradient, HashAlg::VertGradient, HashAlg::Mean] {
                for filter in [FilterType::Lanczos3, FilterType::CatmullRom, FilterType::Gaussian, FilterType::Nearest, FilterType::Triangle] {
                    for size in [2, 4, 8, 16, 32, 64] {
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
