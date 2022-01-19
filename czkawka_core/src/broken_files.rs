use std::collections::BTreeMap;
use std::fs::{File, Metadata};
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread::sleep;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::{fs, mem, panic, thread};

use crossbeam_channel::Receiver;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};

use crate::common::{open_cache_folder, Common, LOOP_DURATION};
use crate::common_directory::Directories;
use crate::common_extensions::Extensions;
use crate::common_items::ExcludedItems;
use crate::common_messages::Messages;
use crate::common_traits::*;
use crate::flc;
use crate::localizer_core::generate_translation_hashmap;
use crate::similar_images::{AUDIO_FILES_EXTENSIONS, IMAGE_RS_BROKEN_FILES_EXTENSIONS, ZIP_FILES_EXTENSIONS};

#[derive(Debug)]
pub struct ProgressData {
    pub current_stage: u8,
    pub max_stage: u8,
    pub files_checked: usize,
    pub files_to_check: usize,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum DeleteMethod {
    None,
    Delete,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct FileEntry {
    pub path: PathBuf,
    pub modified_date: u64,
    pub size: u64,
    pub type_of_file: TypeOfFile,
    pub error_string: String,
}

#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TypeOfFile {
    Unknown = -1,
    Image = 0,
    ArchiveZip,
    #[cfg(feature = "broken_audio")]
    Audio,
}

/// Info struck with helpful information's about results
#[derive(Default)]
pub struct Info {
    pub number_of_broken_files: usize,
}

impl Info {
    pub fn new() -> Self {
        Default::default()
    }
}

pub struct BrokenFiles {
    text_messages: Messages,
    information: Info,
    files_to_check: BTreeMap<String, FileEntry>,
    broken_files: Vec<FileEntry>,
    directories: Directories,
    allowed_extensions: Extensions,
    excluded_items: ExcludedItems,
    recursive_search: bool,
    delete_method: DeleteMethod,
    stopped_search: bool,
    use_cache: bool,
    delete_outdated_cache: bool, // TODO add this to GUI
    save_also_as_json: bool,
}

impl BrokenFiles {
    pub fn new() -> Self {
        Self {
            text_messages: Messages::new(),
            information: Info::new(),
            recursive_search: true,
            allowed_extensions: Extensions::new(),
            directories: Directories::new(),
            excluded_items: ExcludedItems::new(),
            files_to_check: Default::default(),
            delete_method: DeleteMethod::None,
            stopped_search: false,
            broken_files: Default::default(),
            use_cache: true,
            delete_outdated_cache: true,
            save_also_as_json: false,
        }
    }

    pub fn find_broken_files(&mut self, stop_receiver: Option<&Receiver<()>>, progress_sender: Option<&futures::channel::mpsc::UnboundedSender<ProgressData>>) {
        self.directories.optimize_directories(self.recursive_search, &mut self.text_messages);
        if !self.check_files(stop_receiver, progress_sender) {
            self.stopped_search = true;
            return;
        }
        if !self.look_for_broken_files(stop_receiver, progress_sender) {
            self.stopped_search = true;
            return;
        }
        self.delete_files();
        self.debug_print();
    }

    pub fn get_stopped_search(&self) -> bool {
        self.stopped_search
    }

    pub const fn get_broken_files(&self) -> &Vec<FileEntry> {
        &self.broken_files
    }

    pub const fn get_text_messages(&self) -> &Messages {
        &self.text_messages
    }

    pub const fn get_information(&self) -> &Info {
        &self.information
    }

    pub fn set_delete_method(&mut self, delete_method: DeleteMethod) {
        self.delete_method = delete_method;
    }

    pub fn set_save_also_as_json(&mut self, save_also_as_json: bool) {
        self.save_also_as_json = save_also_as_json;
    }

    pub fn set_use_cache(&mut self, use_cache: bool) {
        self.use_cache = use_cache;
    }

    pub fn set_recursive_search(&mut self, recursive_search: bool) {
        self.recursive_search = recursive_search;
    }

    pub fn set_included_directory(&mut self, included_directory: Vec<PathBuf>) -> bool {
        self.directories.set_included_directory(included_directory, &mut self.text_messages)
    }

    pub fn set_excluded_directory(&mut self, excluded_directory: Vec<PathBuf>) {
        self.directories.set_excluded_directory(excluded_directory, &mut self.text_messages);
    }
    pub fn set_allowed_extensions(&mut self, allowed_extensions: String) {
        self.allowed_extensions.set_allowed_extensions(allowed_extensions, &mut self.text_messages);
    }

    pub fn set_excluded_items(&mut self, excluded_items: Vec<String>) {
        self.excluded_items.set_excluded_items(excluded_items, &mut self.text_messages);
    }

    fn check_files(&mut self, stop_receiver: Option<&Receiver<()>>, progress_sender: Option<&futures::channel::mpsc::UnboundedSender<ProgressData>>) -> bool {
        let start_time: SystemTime = SystemTime::now();
        let mut folders_to_check: Vec<PathBuf> = Vec::with_capacity(1024 * 2); // This should be small enough too not see to big difference and big enough to store most of paths without needing to resize vector

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
                        max_stage: 1,
                        files_checked: atomic_file_counter.load(Ordering::Relaxed) as usize,
                        files_to_check: 0,
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
                    // Read current dir childrens
                    let read_dir = match fs::read_dir(&current_folder) {
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

                            let type_of_file = check_extension_avaibility(&file_name_lowercase);
                            if type_of_file == TypeOfFile::Unknown {
                                continue 'dir;
                            }

                            let current_file_name = current_folder.join(entry_data.file_name());
                            if self.excluded_items.is_excluded(&current_file_name) {
                                continue 'dir;
                            }

                            let fe: FileEntry = FileEntry {
                                path: current_file_name.clone(),
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
                                size: metadata.len(),
                                type_of_file,
                                error_string: "".to_string(),
                            };

                            fe_result.push((current_file_name.to_string_lossy().to_string(), fe));
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
                    self.files_to_check.insert(name, fe);
                }
            }
        }

        // End thread which send info to gui
        progress_thread_run.store(false, Ordering::Relaxed);
        progress_thread_handle.join().unwrap();

        Common::print_time(start_time, SystemTime::now(), "check_files".to_string());
        true
    }
    fn look_for_broken_files(&mut self, stop_receiver: Option<&Receiver<()>>, progress_sender: Option<&futures::channel::mpsc::UnboundedSender<ProgressData>>) -> bool {
        let system_time = SystemTime::now();

        let loaded_hash_map;

        let mut records_already_cached: BTreeMap<String, FileEntry> = Default::default();
        let mut non_cached_files_to_check: BTreeMap<String, FileEntry> = Default::default();

        if self.use_cache {
            loaded_hash_map = match load_cache_from_file(&mut self.text_messages, self.delete_outdated_cache) {
                Some(t) => t,
                None => Default::default(),
            };

            for (name, file_entry) in &self.files_to_check {
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
            mem::swap(&mut self.files_to_check, &mut non_cached_files_to_check);
        }

        let check_was_breaked = AtomicBool::new(false); // Used for breaking from GUI and ending check thread

        //// PROGRESS THREAD START
        let progress_thread_run = Arc::new(AtomicBool::new(true));
        let atomic_file_counter = Arc::new(AtomicUsize::new(0));

        let progress_thread_handle = if let Some(progress_sender) = progress_sender {
            let progress_send = progress_sender.clone();
            let progress_thread_run = progress_thread_run.clone();
            let atomic_file_counter = atomic_file_counter.clone();
            let files_to_check = non_cached_files_to_check.len();
            thread::spawn(move || loop {
                progress_send
                    .unbounded_send(ProgressData {
                        current_stage: 1,
                        max_stage: 1,
                        files_checked: atomic_file_counter.load(Ordering::Relaxed) as usize,
                        files_to_check,
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
        let mut vec_file_entry: Vec<FileEntry> = non_cached_files_to_check
            .into_par_iter()
            .map(|(_, mut file_entry)| {
                atomic_file_counter.fetch_add(1, Ordering::Relaxed);
                if stop_receiver.is_some() && stop_receiver.unwrap().try_recv().is_ok() {
                    check_was_breaked.store(true, Ordering::Relaxed);
                    return None;
                }

                match file_entry.type_of_file {
                    TypeOfFile::Image => {
                        let file_entry_clone = file_entry.clone();

                        let result = panic::catch_unwind(|| {
                            match image::open(&file_entry.path) {
                                Ok(_) => Some(None),
                                Err(t) => {
                                    let error_string = t.to_string();
                                    // This error is a problem with image library, remove check when https://github.com/image-rs/jpeg-decoder/issues/130 will be fixed
                                    if !error_string.contains("spectral selection is not allowed in non-progressive scan") {
                                        file_entry.error_string = error_string;
                                        Some(Some(file_entry))
                                    } else {
                                        Some(None)
                                    }
                                }
                            }
                        });

                        // If image crashed during opening, we just skip checking its hash and go on
                        if let Ok(image_result) = result {
                             image_result
                        } else {
                            println!("Image-rs library crashed when opening \"{:?}\" image, please check if problem happens with latest image-rs version(this can be checked via https://github.com/qarmin/ImageOpening tool) and if it is not reported, please report bug here - https://github.com/image-rs/image/issues", file_entry_clone.path);
                             Some(Some(file_entry_clone))
                        }
                    }
                    TypeOfFile::ArchiveZip => match fs::File::open(&file_entry.path) {
                        Ok(file) => match zip::ZipArchive::new(file) {
                            Ok(_) => Some(None),
                            Err(e) => {
                                // TODO Maybe filter out unnecessary types of errors
                                file_entry.error_string = e.to_string();
                                Some(Some(file_entry))
                            }
                        },
                        Err(_inspected) => Some(None), // TODO maybe throw error or something
                    },
                    #[cfg(feature = "broken_audio")]
                    TypeOfFile::Audio => match fs::File::open(&file_entry.path) {
                        Ok(file) => match rodio::Decoder::new(BufReader::new(file)) {
                            Ok(_) => Some(None),
                            Err(e) => {
                                file_entry.error_string = e.to_string();
                                Some(Some(file_entry))
                            }
                        },
                        Err(_inspected) => Some(None), // TODO maybe throw error or something
                    },

                    // This means that cache read invalid value because maybe cache comes from different czkawka version
                    TypeOfFile::Unknown => Some(None),
                }
            })
            .while_some()
            .filter(|file_entry| file_entry.is_some())
            .map(|file_entry| file_entry.unwrap())
            .collect::<Vec<FileEntry>>();

        // End thread which send info to gui
        progress_thread_run.store(false, Ordering::Relaxed);
        progress_thread_handle.join().unwrap();

        // Break if stop was clicked
        if check_was_breaked.load(Ordering::Relaxed) {
            return false;
        }

        // Just connect loaded results with already calculated
        for (_name, file_entry) in records_already_cached {
            vec_file_entry.push(file_entry.clone());
        }

        self.broken_files = vec_file_entry
            .iter()
            .filter_map(|f| if f.error_string.is_empty() { None } else { Some(f.clone()) })
            .collect();

        if self.use_cache {
            // Must save all results to file, old loaded from file with all currently counted results
            let mut all_results: BTreeMap<String, FileEntry> = self.files_to_check.clone();

            for file_entry in vec_file_entry {
                all_results.insert(file_entry.path.to_string_lossy().to_string(), file_entry);
            }
            for (_name, file_entry) in loaded_hash_map {
                all_results.insert(file_entry.path.to_string_lossy().to_string(), file_entry);
            }
            save_cache_to_file(&all_results, &mut self.text_messages, self.save_also_as_json);
        }

        self.information.number_of_broken_files = self.broken_files.len();

        Common::print_time(system_time, SystemTime::now(), "sort_images - reading data from files in parallel".to_string());

        // Clean unused data
        self.files_to_check = Default::default();

        true
    }
    /// Function to delete files, from filed Vector
    fn delete_files(&mut self) {
        let start_time: SystemTime = SystemTime::now();

        match self.delete_method {
            DeleteMethod::Delete => {
                for file_entry in self.broken_files.iter() {
                    if fs::remove_file(&file_entry.path).is_err() {
                        self.text_messages.warnings.push(file_entry.path.display().to_string());
                    }
                }
            }
            DeleteMethod::None => {
                //Just do nothing
            }
        }

        Common::print_time(start_time, SystemTime::now(), "delete_files".to_string());
    }
}

impl Default for BrokenFiles {
    fn default() -> Self {
        Self::new()
    }
}

impl DebugPrint for BrokenFiles {
    #[allow(dead_code)]
    #[allow(unreachable_code)]
    /// Debugging printing - only available on debug build
    fn debug_print(&self) {
        #[cfg(not(debug_assertions))]
        {
            return;
        }
        println!("---------------DEBUG PRINT---------------");
        println!("### Information's");

        println!("Errors size - {}", self.text_messages.errors.len());
        println!("Warnings size - {}", self.text_messages.warnings.len());
        println!("Messages size - {}", self.text_messages.messages.len());

        println!("### Other");

        println!("Excluded items - {:?}", self.excluded_items.items);
        println!("Included directories - {:?}", self.directories.included_directories);
        println!("Excluded directories - {:?}", self.directories.excluded_directories);
        println!("Recursive search - {}", self.recursive_search);
        println!("Delete Method - {:?}", self.delete_method);
        println!("-----------------------------------------");
    }
}

impl SaveResults for BrokenFiles {
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

        if !self.broken_files.is_empty() {
            writeln!(writer, "Found {} broken files.", self.information.number_of_broken_files).unwrap();
            for file_entry in self.broken_files.iter() {
                writeln!(writer, "{} - {}", file_entry.path.display(), file_entry.error_string).unwrap();
            }
        } else {
            write!(writer, "Not found any broken files.").unwrap();
        }
        Common::print_time(start_time, SystemTime::now(), "save_results_to_file".to_string());
        true
    }
}

impl PrintResults for BrokenFiles {
    /// Print information's about duplicated entries
    /// Only needed for CLI
    fn print_results(&self) {
        let start_time: SystemTime = SystemTime::now();
        println!("Found {} broken files.\n", self.information.number_of_broken_files);
        for file_entry in self.broken_files.iter() {
            println!("{} - {}", file_entry.path.display(), file_entry.error_string);
        }

        Common::print_time(start_time, SystemTime::now(), "print_entries".to_string());
    }
}

fn save_cache_to_file(old_hashmap: &BTreeMap<String, FileEntry>, text_messages: &mut Messages, save_also_as_json: bool) {
    let mut hashmap: BTreeMap<String, FileEntry> = Default::default();
    for (path, fe) in old_hashmap {
        if fe.size > 1024 {
            hashmap.insert(path.clone(), fe.clone());
        }
    }
    let hashmap = &hashmap;

    if let Some(((file_handler, cache_file), (file_handler_json, cache_file_json))) = open_cache_folder(&get_cache_file(), true, save_also_as_json, &mut text_messages.warnings) {
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

fn load_cache_from_file(text_messages: &mut Messages, delete_outdated_cache: bool) -> Option<BTreeMap<String, FileEntry>> {
    if let Some(((file_handler, cache_file), (file_handler_json, cache_file_json))) = open_cache_folder(&get_cache_file(), false, true, &mut text_messages.warnings) {
        let mut hashmap_loaded_entries: BTreeMap<String, FileEntry>;
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

fn get_cache_file() -> String {
    "cache_broken_files.bin".to_string()
}

fn check_extension_avaibility(file_name_lowercase: &str) -> TypeOfFile {
    if IMAGE_RS_BROKEN_FILES_EXTENSIONS.iter().any(|e| file_name_lowercase.ends_with(e)) {
        TypeOfFile::Image
    } else if ZIP_FILES_EXTENSIONS.iter().any(|e| file_name_lowercase.ends_with(e)) {
        TypeOfFile::ArchiveZip
    } else if AUDIO_FILES_EXTENSIONS.iter().any(|e| file_name_lowercase.ends_with(e)) {
        #[cfg(feature = "broken_audio")]
        {
            TypeOfFile::Audio
        }

        #[cfg(not(feature = "broken_audio"))]
        {
            TypeOfFile::Unknown
        }
    } else {
        TypeOfFile::Unknown
    }
}
