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
use img_hash::HasherConfig;
use rayon::prelude::*;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::fs::{File, Metadata};
use std::io::Write;
use std::io::*;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread::sleep;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::{fs, thread};

/// Type to store for each entry in the similarity BK-tree.
type Node = [u8; 8];

const CACHE_FILE_NAME: &str = "cache_similar_image.txt";

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
    Minimal,
    VerySmall,
    Small,
    Medium,
    High,
    VeryHigh,
}

#[derive(Clone, Debug)]
pub struct FileEntry {
    pub path: PathBuf,
    pub size: u64,
    pub dimensions: String,
    pub modified_date: u64,
    pub hash: Node,
    pub similarity: Similarity,
}

/// Distance metric to use with the BK-tree.
struct Hamming;

impl bk_tree::Metric<Node> for Hamming {
    fn distance(&self, a: &Node, b: &Node) -> u64 {
        hamming::distance_fast(a, b).unwrap()
    }
}

/// Struct to store most basics info about all folder
pub struct SimilarImages {
    information: Info,
    text_messages: Messages,
    directories: Directories,
    excluded_items: ExcludedItems,
    bktree: BKTree<Node, Hamming>,
    similar_vectors: Vec<Vec<FileEntry>>,
    recursive_search: bool,
    minimal_file_size: u64,
    image_hashes: HashMap<Node, Vec<FileEntry>>, // Hashmap with image hashes and Vector with names of files
    stopped_search: bool,
    similarity: Similarity,
    images_to_check: HashMap<String, FileEntry>,
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
            image_hashes: Default::default(),
            stopped_search: false,
            similarity: Similarity::High,
            images_to_check: Default::default(),
        }
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

    pub fn set_recursive_search(&mut self, recursive_search: bool) {
        self.recursive_search = recursive_search;
    }

    pub fn set_minimal_file_size(&mut self, minimal_file_size: u64) {
        self.minimal_file_size = match minimal_file_size {
            0 => 1,
            t => t,
        };
    }
    pub fn set_similarity(&mut self, similarity: Similarity) {
        self.similarity = similarity;
    }

    /// Public function used by CLI to search for empty folders
    pub fn find_similar_images(&mut self, stop_receiver: Option<&Receiver<()>>, progress_sender: Option<&futures::channel::mpsc::Sender<ProgressData>>) {
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
    fn check_for_similar_images(&mut self, stop_receiver: Option<&Receiver<()>>, progress_sender: Option<&futures::channel::mpsc::Sender<ProgressData>>) -> bool {
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
            let mut progress_send = progress_sender.clone();
            let progress_thread_run = progress_thread_run.clone();
            let atomic_file_counter = atomic_file_counter.clone();
            progress_thread_handle = thread::spawn(move || loop {
                progress_send
                    .try_send(ProgressData {
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
                Err(_) => {
                    self.text_messages.warnings.push(format!("Cannot open dir {}", current_folder.display()));
                    continue;
                } // Permissions denied
            };

            // Check every sub folder/file/link etc.
            'dir: for entry in read_dir {
                let entry_data = match entry {
                    Ok(t) => t,
                    Err(_) => {
                        self.text_messages.warnings.push(format!("Cannot read entry in dir {}", current_folder.display()));
                        continue;
                    } //Permissions denied
                };
                let metadata: Metadata = match entry_data.metadata() {
                    Ok(t) => t,
                    Err(_) => {
                        self.text_messages.warnings.push(format!("Cannot read metadata in dir {}", current_folder.display()));
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
                        Err(_) => continue,
                    }
                    .to_lowercase();

                    // Checking allowed image extensions
                    let allowed_image_extensions = ["jpg", "jpeg", "png", "bmp", "ico", "tiff", "pnm", "tga", "ff", "gif"];
                    if !allowed_image_extensions.iter().any(|e| file_name_lowercase.ends_with(format!(".{}", e).as_str())) {
                        continue 'dir;
                    }

                    // Checking files
                    if metadata.len() >= self.minimal_file_size {
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
                                    Err(_) => {
                                        self.text_messages.warnings.push(format!("File {} seems to be modified before Unix Epoch.", current_file_name.display()));
                                        0
                                    }
                                },
                                Err(_) => {
                                    self.text_messages.warnings.push(format!("Unable to get modification date from file {}", current_file_name.display()));
                                    continue;
                                } // Permissions Denied
                            },

                            hash: [0; 8],
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

    fn sort_images(&mut self, stop_receiver: Option<&Receiver<()>>, progress_sender: Option<&futures::channel::mpsc::Sender<ProgressData>>) -> bool {
        let hash_map_modification = SystemTime::now();

        let loaded_hash_map = match load_hashes_from_file(&mut self.text_messages) {
            Some(t) => t,
            None => Default::default(),
        };

        let mut records_already_cached: HashMap<String, FileEntry> = Default::default();
        let mut non_cached_files_to_check: HashMap<String, FileEntry> = Default::default();
        for (name, file_entry) in &self.images_to_check {
            #[allow(clippy::collapsible_if)]
            if !loaded_hash_map.contains_key(name) {
                // If loaded data doesn't contains current image info
                non_cached_files_to_check.insert(name.clone(), file_entry.clone());
            } else {
                if file_entry.size != loaded_hash_map.get(name).unwrap().size || file_entry.modified_date != loaded_hash_map.get(name).unwrap().modified_date {
                    // When size or modification date of image changed, then it is clear that is different image
                    non_cached_files_to_check.insert(name.clone(), file_entry.clone());
                } else {
                    // Checking may be omitted when already there is entry with same size and modification date
                    records_already_cached.insert(name.clone(), loaded_hash_map.get(name).unwrap().clone());
                }
            }
        }

        Common::print_time(hash_map_modification, SystemTime::now(), "sort_images - reading data from cache and preparing them".to_string());
        let hash_map_modification = SystemTime::now();

        //// PROGRESS THREAD START
        const LOOP_DURATION: u32 = 200; //in ms
        let progress_thread_run = Arc::new(AtomicBool::new(true));

        let atomic_file_counter = Arc::new(AtomicUsize::new(0));

        let progress_thread_handle;
        if let Some(progress_sender) = progress_sender {
            let mut progress_send = progress_sender.clone();
            let progress_thread_run = progress_thread_run.clone();
            let atomic_file_counter = atomic_file_counter.clone();
            let images_to_check = non_cached_files_to_check.len();
            progress_thread_handle = thread::spawn(move || loop {
                progress_send
                    .try_send(ProgressData {
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
        let mut vec_file_entry: Vec<(FileEntry, Node)> = non_cached_files_to_check
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
                    Err(_) => return Some(None), // Something is wrong with image
                };
                let dimensions = image.dimensions();

                file_entry.dimensions = format!("{}x{}", dimensions.0, dimensions.1);
                let hasher = HasherConfig::with_bytes_type::<Node>().to_hasher();

                let hash = hasher.hash_image(&image);
                let mut buf = [0u8; 8];
                buf.copy_from_slice(&hash.as_bytes());
                file_entry.hash = buf;

                Some(Some((file_entry, buf)))
            })
            .while_some()
            .filter(|file_entry| file_entry.is_some())
            .map(|file_entry| file_entry.unwrap())
            .collect::<Vec<(FileEntry, Node)>>();

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
            self.bktree.add(*buf);
            self.image_hashes.entry(*buf).or_insert_with(Vec::<FileEntry>::new);
            self.image_hashes.get_mut(buf).unwrap().push(file_entry.clone());
        }

        // Must save all results to file, old loaded from file with all currently counted results
        let mut all_results: HashMap<String, FileEntry> = loaded_hash_map;
        for (file_entry, _hash) in vec_file_entry {
            all_results.insert(file_entry.path.to_string_lossy().to_string(), file_entry);
        }
        save_hashes_to_file(&all_results, &mut self.text_messages);

        Common::print_time(hash_map_modification, SystemTime::now(), "sort_images - saving data to files".to_string());
        let hash_map_modification = SystemTime::now();

        let similarity: u64 = match self.similarity {
            Similarity::VeryHigh => 0,
            Similarity::High => 1,
            Similarity::Medium => 2,
            Similarity::Small => 3,
            Similarity::VerySmall => 4,
            Similarity::Minimal => 5,
            _ => panic!("0-5 similarity levels are allowed, check if not added more."),
        };

        // TODO
        // Now is A is similar to B with VeryHigh and C with Medium
        // And D is similar with C with High
        // And Similarity is set to Medium(or lower)
        // And A is checked before D
        // Then C is shown that is similar group A, not D

        // TODO
        // Maybe also add here progress report

        let mut new_vector: Vec<Vec<FileEntry>> = Vec::new();
        let mut non_cached_files_to_check = self.image_hashes.clone();
        for (hash, vec_file_entry) in &self.image_hashes {
            if stop_receiver.is_some() && stop_receiver.unwrap().try_recv().is_ok() {
                return false;
            }
            if !non_cached_files_to_check.contains_key(hash) {
                continue;
            }
            non_cached_files_to_check.remove(hash);

            let vector_with_found_similar_hashes = self.bktree.find(hash, similarity).collect::<Vec<_>>();
            if vector_with_found_similar_hashes.len() == 1 && vec_file_entry.len() == 1 {
                // This one picture doesn't have similar pictures, so there is no go
                continue;
            }

            let mut vector_of_similar_images: Vec<FileEntry> = vec_file_entry
                .iter()
                .map(|fe| FileEntry {
                    path: fe.path.clone(),
                    size: fe.size,
                    dimensions: fe.dimensions.clone(),
                    modified_date: fe.modified_date,
                    hash: fe.hash,
                    similarity: Similarity::VeryHigh,
                })
                .collect();

            for (similarity, similar_hash) in vector_with_found_similar_hashes.iter() {
                if *similarity == 0 && hash == *similar_hash {
                    // This was already read before
                    continue;
                } else if hash == *similar_hash {
                    panic!("I'm not sure if same hash can have distance > 0");
                }

                if let Some(vec_file_entry) = non_cached_files_to_check.get(*similar_hash) {
                    vector_of_similar_images.append(
                        &mut (vec_file_entry
                            .iter()
                            .map(|fe| FileEntry {
                                path: fe.path.clone(),
                                size: fe.size,
                                dimensions: fe.dimensions.clone(),
                                modified_date: fe.modified_date,
                                hash: [0; 8],
                                similarity: match similarity {
                                    0 => Similarity::VeryHigh,
                                    1 => Similarity::High,
                                    2 => Similarity::Medium,
                                    3 => Similarity::Small,
                                    4 => Similarity::VerySmall,
                                    5 => Similarity::Minimal,
                                    _ => panic!("0-5 similarity levels are allowed, check if not added more."),
                                },
                            })
                            .collect::<Vec<_>>()),
                    );
                    non_cached_files_to_check.remove(*similar_hash);
                }
            }
            if vector_of_similar_images.len() > 1 {
                // Not sure why it may happens
                new_vector.push((*vector_of_similar_images).to_owned());
            }
        }

        self.similar_vectors = new_vector;

        Common::print_time(hash_map_modification, SystemTime::now(), "sort_images - selecting data from BtreeMap".to_string());
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
            Err(_) => {
                self.text_messages.errors.push("Failed to create file ".to_string() + file_name.as_str());
                return false;
            }
        };
        let mut writer = BufWriter::new(file_handler);

        if writeln!(
            writer,
            "Results of searching {:?} with excluded directories {:?} and excluded items {:?}",
            self.directories.included_directories, self.directories.excluded_directories, self.excluded_items.items
        )
        .is_err()
        {
            self.text_messages.errors.push(format!("Failed to save results to file {}", file_name));
            return false;
        }

        if !self.similar_vectors.is_empty() {
            write!(writer, "{} images which have similar friends\n\n", self.similar_vectors.len()).unwrap();

        // for struct_similar in self.similar_vectors.iter() {
        //     writeln!(writer, "Image {:?} have {} similar images", struct_similar.base_image.path, struct_similar.similar_images.len()).unwrap();
        //     for similar_picture in struct_similar.similar_images.iter() {
        //         writeln!(writer, "{:?} - Similarity Level: {}", similar_picture.path, get_string_from_similarity(&similar_picture.similarity)).unwrap();
        //     }
        //     writeln!(writer).unwrap();
        // }
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
                        get_string_from_similarity(&file_entry.similarity)
                    );
                }
                println!();
            }
        }
    }
}

fn get_string_from_similarity(similarity: &Similarity) -> &str {
    match similarity {
        Similarity::Minimal => "Minimal",
        Similarity::VerySmall => "Very Small",
        Similarity::Small => "Small",
        Similarity::Medium => "Medium",
        Similarity::High => "High",
        Similarity::VeryHigh => "Very High",
        Similarity::None => panic!(),
    }
}

fn save_hashes_to_file(hashmap: &HashMap<String, FileEntry>, text_messages: &mut Messages) {
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
        } else if fs::create_dir_all(&cache_dir).is_err() {
            text_messages.messages.push(format!("Cannot create config dir {}", cache_dir.display()));
            return;
        }
        let cache_file = cache_dir.join(CACHE_FILE_NAME);
        let file_handler = match OpenOptions::new().truncate(true).write(true).create(true).open(&cache_file) {
            Ok(t) => t,
            Err(_) => {
                text_messages.messages.push(format!("Cannot create or open cache file {}", cache_file.display()));
                return;
            }
        };
        let mut writer = BufWriter::new(file_handler);

        for file_entry in hashmap.values() {
            let mut string: String = "".to_string();

            string += format!("{}//{}//{}//{}//", file_entry.path.display(), file_entry.size, file_entry.dimensions, file_entry.modified_date).as_str();

            for i in 0..file_entry.hash.len() - 1 {
                string += format!("{}//", file_entry.hash[i]).as_str();
            }
            string += file_entry.hash[file_entry.hash.len() - 1].to_string().as_str();

            if writeln!(writer, "{}", string).is_err() {
                text_messages.messages.push(format!("Failed to save some data to cache file {}", cache_file.display()));
                return;
            };
        }
    }
}
fn load_hashes_from_file(text_messages: &mut Messages) -> Option<HashMap<String, FileEntry>> {
    if let Some(proj_dirs) = ProjectDirs::from("pl", "Qarmin", "Czkawka") {
        let cache_dir = PathBuf::from(proj_dirs.cache_dir());
        let cache_file = cache_dir.join(CACHE_FILE_NAME);
        let file_handler = match OpenOptions::new().read(true).open(&cache_file) {
            Ok(t) => t,
            Err(_) => {
                // text_messages.messages.push(format!("Cannot find or open cache file {}", cache_file.display())); // This shouldn't be write to output
                return None;
            }
        };

        let reader = BufReader::new(file_handler);

        let mut hashmap_loaded_entries: HashMap<String, FileEntry> = Default::default();

        // Read the file line by line using the lines() iterator from std::io::BufRead.
        for (index, line) in reader.lines().enumerate() {
            let line = match line {
                Ok(t) => t,
                Err(_) => {
                    text_messages.warnings.push(format!("Failed to load line number {} from cache file {}", index + 1, cache_file.display()));
                    return None;
                }
            };
            let uuu = line.split("//").collect::<Vec<&str>>();
            if uuu.len() != 12 {
                text_messages.warnings.push(format!("Found invalid data in line {} - ({}) in cache file {}", index + 1, line, cache_file.display()));
                continue;
            }
            // Don't load cache data if destination file not exists
            if Path::new(uuu[0]).exists() {
                let mut hash: Node = [0u8; 8];
                for i in 0..hash.len() {
                    hash[i] = match uuu[4 + i].parse::<u8>() {
                        Ok(t) => t,
                        Err(_) => {
                            text_messages.warnings.push(format!("Found invalid hash value in line {} - ({}) in cache file {}", index + 1, line, cache_file.display()));
                            continue;
                        }
                    };
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
                            Err(_) => {
                                text_messages.warnings.push(format!("Found invalid size value in line {} - ({}) in cache file {}", index + 1, line, cache_file.display()));
                                continue;
                            }
                        },
                        dimensions: uuu[2].to_string(),
                        modified_date: match uuu[3].parse::<u64>() {
                            Ok(t) => t,
                            Err(_) => {
                                text_messages.warnings.push(format!("Found invalid modified date value in line {} - ({}) in cache file {}", index + 1, line, cache_file.display()));
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
