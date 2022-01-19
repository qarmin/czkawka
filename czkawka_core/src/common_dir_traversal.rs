use std::collections::BTreeMap;
use std::fs::Metadata;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread::sleep;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::{fs, thread};

use crate::common::LOOP_DURATION;
use crossbeam_channel::Receiver;
use rayon::prelude::*;

use crate::common_directory::Directories;
use crate::common_extensions::Extensions;
use crate::common_items::ExcludedItems;
use crate::flc;
use crate::localizer_core::generate_translation_hashmap;

#[derive(Debug)]
pub struct ProgressData {
    pub checking_method: CheckingMethod,
    pub current_stage: u8,
    pub max_stage: u8,
    pub entries_checked: usize,
    pub entries_to_check: usize,
}

#[derive(PartialEq, Eq, Clone, Debug, Copy)]
pub enum CheckingMethod {
    None,
    Name,
    Size,
    Hash,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct FileEntry {
    pub path: PathBuf,
    pub size: u64,
    pub modified_date: u64,
    pub hash: String,
    pub symlink_info: Option<SymlinkInfo>,
}

// Symlinks

const MAX_NUMBER_OF_SYMLINK_JUMPS: i32 = 20;

#[derive(Clone, Debug, PartialEq)]
pub struct SymlinkInfo {
    pub destination_path: PathBuf,
    pub type_of_error: ErrorType,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ErrorType {
    InfiniteRecursion,
    NonExistentFile,
}

// Empty folders

/// Enum with values which show if folder is empty.
/// In function "optimize_folders" automatically "Maybe" is changed to "Yes", so it is not necessary to put it here
#[derive(Eq, PartialEq, Copy, Clone)]
pub(crate) enum FolderEmptiness {
    No,
    Maybe,
}

/// Struct assigned to each checked folder with parent path(used to ignore parent if children are not empty) and flag which shows if folder is empty
#[derive(Clone)]
pub struct FolderEntry {
    pub(crate) parent_path: Option<PathBuf>, // Usable only when finding
    pub(crate) is_empty: FolderEmptiness,
    pub modified_date: u64,
}

// Collection mode (files / empty folders)

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Collect {
    EmptyFolders,
    InvalidSymlinks,
    Files,
}

#[derive(Eq, PartialEq)]
enum EntryType {
    File,
    Dir,
    Symlink,
    Other,
}

pub struct DirTraversalBuilder<'a, 'b, F> {
    group_by: Option<F>,
    root_dirs: Vec<PathBuf>,
    stop_receiver: Option<&'a Receiver<()>>,
    progress_sender: Option<&'b futures::channel::mpsc::UnboundedSender<ProgressData>>,
    minimal_file_size: Option<u64>,
    maximal_file_size: Option<u64>,
    checking_method: CheckingMethod,
    max_stage: u8,
    collect: Collect,
    recursive_search: bool,
    directories: Option<Directories>,
    excluded_items: Option<ExcludedItems>,
    allowed_extensions: Option<Extensions>,
}

pub struct DirTraversal<'a, 'b, F> {
    group_by: F,
    root_dirs: Vec<PathBuf>,
    stop_receiver: Option<&'a Receiver<()>>,
    progress_sender: Option<&'b futures::channel::mpsc::UnboundedSender<ProgressData>>,
    recursive_search: bool,
    directories: Directories,
    excluded_items: ExcludedItems,
    allowed_extensions: Extensions,
    minimal_file_size: u64,
    maximal_file_size: u64,
    checking_method: CheckingMethod,
    max_stage: u8,
    collect: Collect,
}

impl<'a, 'b> Default for DirTraversalBuilder<'a, 'b, ()> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, 'b> DirTraversalBuilder<'a, 'b, ()> {
    pub fn new() -> DirTraversalBuilder<'a, 'b, ()> {
        DirTraversalBuilder {
            group_by: None,
            root_dirs: vec![],
            stop_receiver: None,
            progress_sender: None,
            checking_method: CheckingMethod::None,
            max_stage: 0,
            minimal_file_size: None,
            maximal_file_size: None,
            collect: Collect::Files,
            recursive_search: false,
            directories: None,
            allowed_extensions: None,
            excluded_items: None,
        }
    }
}

impl<'a, 'b, F> DirTraversalBuilder<'a, 'b, F> {
    pub fn root_dirs(mut self, dirs: Vec<PathBuf>) -> Self {
        self.root_dirs = dirs;
        self
    }

    pub fn stop_receiver(mut self, stop_receiver: Option<&'a Receiver<()>>) -> Self {
        self.stop_receiver = stop_receiver;
        self
    }

    pub fn progress_sender(mut self, progress_sender: Option<&'b futures::channel::mpsc::UnboundedSender<ProgressData>>) -> Self {
        self.progress_sender = progress_sender;
        self
    }

    pub fn checking_method(mut self, checking_method: CheckingMethod) -> Self {
        self.checking_method = checking_method;
        self
    }

    pub fn max_stage(mut self, max_stage: u8) -> Self {
        self.max_stage = max_stage;
        self
    }

    pub fn minimal_file_size(mut self, minimal_file_size: u64) -> Self {
        self.minimal_file_size = Some(minimal_file_size);
        self
    }

    pub fn maximal_file_size(mut self, maximal_file_size: u64) -> Self {
        self.maximal_file_size = Some(maximal_file_size);
        self
    }

    pub fn collect(mut self, collect: Collect) -> Self {
        self.collect = collect;
        self
    }

    pub fn directories(mut self, directories: Directories) -> Self {
        self.directories = Some(directories);
        self
    }

    pub fn allowed_extensions(mut self, allowed_extensions: Extensions) -> Self {
        self.allowed_extensions = Some(allowed_extensions);
        self
    }

    pub fn excluded_items(mut self, excluded_items: ExcludedItems) -> Self {
        self.excluded_items = Some(excluded_items);
        self
    }

    pub fn recursive_search(mut self, recursive_search: bool) -> Self {
        self.recursive_search = recursive_search;
        self
    }

    pub fn group_by<G, T>(self, group_by: G) -> DirTraversalBuilder<'a, 'b, G>
    where
        G: Fn(&FileEntry) -> T,
    {
        DirTraversalBuilder {
            group_by: Some(group_by),
            root_dirs: self.root_dirs,
            stop_receiver: self.stop_receiver,
            progress_sender: self.progress_sender,
            directories: self.directories,
            allowed_extensions: self.allowed_extensions,
            excluded_items: self.excluded_items,
            recursive_search: self.recursive_search,
            maximal_file_size: self.maximal_file_size,
            minimal_file_size: self.minimal_file_size,
            collect: self.collect,
            checking_method: self.checking_method,
            max_stage: self.max_stage,
        }
    }

    pub fn build(self) -> DirTraversal<'a, 'b, F> {
        DirTraversal {
            group_by: self.group_by.expect("could not build"),
            root_dirs: self.root_dirs,
            stop_receiver: self.stop_receiver,
            progress_sender: self.progress_sender,
            checking_method: self.checking_method,
            max_stage: self.max_stage,
            minimal_file_size: self.minimal_file_size.unwrap_or(0),
            maximal_file_size: self.maximal_file_size.unwrap_or(u64::MAX),
            collect: self.collect,
            directories: self.directories.expect("could not build"),
            excluded_items: self.excluded_items.expect("could not build"),
            allowed_extensions: self.allowed_extensions.unwrap_or_default(),
            recursive_search: self.recursive_search,
        }
    }
}

pub enum DirTraversalResult<T: Ord + PartialOrd> {
    SuccessFiles {
        start_time: SystemTime,
        warnings: Vec<String>,
        grouped_file_entries: BTreeMap<T, Vec<FileEntry>>,
    },
    SuccessFolders {
        start_time: SystemTime,
        warnings: Vec<String>,
        folder_entries: BTreeMap<PathBuf, FolderEntry>, // Path, FolderEntry
    },
    Stopped,
}

fn entry_type(metadata: &Metadata) -> EntryType {
    let file_type = metadata.file_type();
    if file_type.is_dir() {
        EntryType::Dir
    } else if file_type.is_symlink() {
        EntryType::Symlink
    } else if file_type.is_file() {
        EntryType::File
    } else {
        EntryType::Other
    }
}

impl<'a, 'b, F, T> DirTraversal<'a, 'b, F>
where
    F: Fn(&FileEntry) -> T,
    T: Ord + PartialOrd,
{
    pub fn run(self) -> DirTraversalResult<T> {
        let mut all_warnings = vec![];
        let mut grouped_file_entries: BTreeMap<T, Vec<FileEntry>> = BTreeMap::new();
        let mut folder_entries: BTreeMap<PathBuf, FolderEntry> = BTreeMap::new();
        let start_time: SystemTime = SystemTime::now();

        // Add root folders into result (only for empty folder collection)
        let mut folders_to_check: Vec<PathBuf> = Vec::with_capacity(1024 * 2); // This should be small enough too not see to big difference and big enough to store most of paths without needing to resize vector
        if self.collect == Collect::EmptyFolders {
            for dir in &self.root_dirs {
                folder_entries.insert(
                    dir.clone(),
                    FolderEntry {
                        parent_path: None,
                        is_empty: FolderEmptiness::Maybe,
                        modified_date: 0,
                    },
                );
            }
        }
        // Add root folders for finding
        folders_to_check.extend(self.root_dirs);

        //// PROGRESS THREAD START
        let progress_thread_run = Arc::new(AtomicBool::new(true));

        let atomic_entry_counter = Arc::new(AtomicUsize::new(0));

        let progress_thread_handle = if let Some(progress_sender) = self.progress_sender {
            let progress_send = progress_sender.clone();
            let progress_thread_run = progress_thread_run.clone();
            let atomic_entry_counter = atomic_entry_counter.clone();
            let checking_method = self.checking_method;
            let max_stage = self.max_stage;
            thread::spawn(move || loop {
                progress_send
                    .unbounded_send(ProgressData {
                        checking_method,
                        current_stage: 0,
                        max_stage,
                        entries_checked: atomic_entry_counter.load(Ordering::Relaxed) as usize,
                        entries_to_check: 0,
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

        let DirTraversal {
            collect,
            directories,
            excluded_items,
            allowed_extensions,
            recursive_search,
            minimal_file_size,
            maximal_file_size,
            stop_receiver,
            ..
        } = self;

        while !folders_to_check.is_empty() {
            if stop_receiver.is_some() && stop_receiver.unwrap().try_recv().is_ok() {
                // End thread which send info to gui
                progress_thread_run.store(false, Ordering::Relaxed);
                progress_thread_handle.join().unwrap();
                return DirTraversalResult::Stopped;
            }

            let segments: Vec<_> = folders_to_check
                .par_iter()
                .map(|current_folder| {
                    let mut dir_result = vec![];
                    let mut warnings = vec![];
                    let mut fe_result = vec![];
                    let mut set_as_not_empty_folder_list = vec![];
                    let mut folder_entries_list = vec![];
                    // Read current dir childrens
                    let read_dir = match fs::read_dir(&current_folder) {
                        Ok(t) => t,
                        Err(e) => {
                            warnings.push(flc!(
                                "core_cannot_open_dir",
                                generate_translation_hashmap(vec![("dir", current_folder.display().to_string()), ("reason", e.to_string())])
                            ));
                            return (dir_result, warnings, fe_result, set_as_not_empty_folder_list, folder_entries_list);
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
                        match (entry_type(&metadata), collect) {
                            (EntryType::Dir, Collect::Files) | (EntryType::Dir, Collect::InvalidSymlinks) => {
                                if !recursive_search {
                                    continue 'dir;
                                }

                                let next_folder = current_folder.join(entry_data.file_name());
                                if directories.is_excluded(&next_folder) {
                                    continue 'dir;
                                }

                                if excluded_items.is_excluded(&next_folder) {
                                    continue 'dir;
                                }

                                dir_result.push(next_folder);
                            }
                            (EntryType::Dir, Collect::EmptyFolders) => {
                                atomic_entry_counter.fetch_add(1, Ordering::Relaxed);
                                let next_folder = current_folder.join(entry_data.file_name());
                                if excluded_items.is_excluded(&next_folder) || directories.is_excluded(&next_folder) {
                                    set_as_not_empty_folder_list.push(current_folder.clone());
                                    continue 'dir;
                                }
                                dir_result.push(next_folder.clone());
                                folder_entries_list.push((
                                    next_folder.clone(),
                                    FolderEntry {
                                        parent_path: Some(current_folder.clone()),
                                        is_empty: FolderEmptiness::Maybe,
                                        modified_date: match metadata.modified() {
                                            Ok(t) => match t.duration_since(UNIX_EPOCH) {
                                                Ok(d) => d.as_secs(),
                                                Err(_inspected) => {
                                                    warnings.push(flc!(
                                                        "core_folder_modified_before_epoch",
                                                        generate_translation_hashmap(vec![("name", current_folder.display().to_string())])
                                                    ));
                                                    0
                                                }
                                            },
                                            Err(e) => {
                                                warnings.push(flc!(
                                                    "core_folder_no_modification_date",
                                                    generate_translation_hashmap(vec![("name", current_folder.display().to_string()), ("reason", e.to_string())])
                                                ));
                                                0
                                            }
                                        },
                                    },
                                ));
                            }
                            (EntryType::File, Collect::Files) => {
                                atomic_entry_counter.fetch_add(1, Ordering::Relaxed);

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

                                if !allowed_extensions.matches_filename(&file_name_lowercase) {
                                    continue 'dir;
                                }

                                if (minimal_file_size..=maximal_file_size).contains(&metadata.len()) {
                                    let current_file_name = current_folder.join(entry_data.file_name());
                                    if excluded_items.is_excluded(&current_file_name) {
                                        continue 'dir;
                                    }

                                    // Creating new file entry
                                    let fe: FileEntry = FileEntry {
                                        path: current_file_name.clone(),
                                        size: metadata.len(),
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
                                        hash: "".to_string(),
                                        symlink_info: None,
                                    };

                                    fe_result.push(fe);
                                }
                            }
                            (EntryType::File, Collect::EmptyFolders) | (EntryType::Symlink, Collect::EmptyFolders) => {
                                set_as_not_empty_folder_list.push(current_folder.clone());
                            }
                            (EntryType::File, Collect::InvalidSymlinks) => {
                                atomic_entry_counter.fetch_add(1, Ordering::Relaxed);
                            }
                            (EntryType::Symlink, Collect::InvalidSymlinks) => {
                                atomic_entry_counter.fetch_add(1, Ordering::Relaxed);

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

                                if !allowed_extensions.matches_filename(&file_name_lowercase) {
                                    continue 'dir;
                                }

                                let current_file_name = current_folder.join(entry_data.file_name());
                                if excluded_items.is_excluded(&current_file_name) {
                                    continue 'dir;
                                }

                                let mut destination_path = PathBuf::new();
                                let type_of_error;

                                match current_file_name.read_link() {
                                    Ok(t) => {
                                        destination_path.push(t);
                                        let mut number_of_loop = 0;
                                        let mut current_path = current_file_name.clone();
                                        loop {
                                            if number_of_loop == 0 && !current_path.exists() {
                                                type_of_error = ErrorType::NonExistentFile;
                                                break;
                                            }
                                            if number_of_loop == MAX_NUMBER_OF_SYMLINK_JUMPS {
                                                type_of_error = ErrorType::InfiniteRecursion;
                                                break;
                                            }

                                            current_path = match current_path.read_link() {
                                                Ok(t) => t,
                                                Err(_inspected) => {
                                                    // Looks that some next symlinks are broken, but we do nothing with it - TODO why they are broken
                                                    continue 'dir;
                                                }
                                            };

                                            number_of_loop += 1;
                                        }
                                    }
                                    Err(_inspected) => {
                                        // Failed to load info about it
                                        type_of_error = ErrorType::NonExistentFile;
                                    }
                                }

                                // Creating new file entry
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
                                    size: 0,
                                    hash: "".to_string(),
                                    symlink_info: Some(SymlinkInfo { destination_path, type_of_error }),
                                };

                                // Adding files to Vector
                                fe_result.push(fe);
                            }
                            (EntryType::Symlink, Collect::Files) | (EntryType::Other, _) => {
                                // nothing to do
                            }
                        }
                    }
                    (dir_result, warnings, fe_result, set_as_not_empty_folder_list, folder_entries_list)
                })
                .collect();

            // Advance the frontier
            folders_to_check.clear();

            // Process collected data
            for (segment, warnings, fe_result, set_as_not_empty_folder_list, fe_list) in segments {
                folders_to_check.extend(segment);
                all_warnings.extend(warnings);
                for fe in fe_result {
                    let key = (self.group_by)(&fe);
                    grouped_file_entries.entry(key).or_insert_with(Vec::new).push(fe);
                }
                for current_folder in &set_as_not_empty_folder_list {
                    set_as_not_empty_folder(&mut folder_entries, current_folder);
                }
                for (path, entry) in fe_list {
                    folder_entries.insert(path, entry);
                }
            }
        }

        // End thread which send info to gui
        progress_thread_run.store(false, Ordering::Relaxed);
        progress_thread_handle.join().unwrap();

        match collect {
            Collect::Files | Collect::InvalidSymlinks => DirTraversalResult::SuccessFiles {
                start_time,
                grouped_file_entries,
                warnings: all_warnings,
            },
            Collect::EmptyFolders => DirTraversalResult::SuccessFolders {
                start_time,
                folder_entries,
                warnings: all_warnings,
            },
        }
    }
}

fn set_as_not_empty_folder(folder_entries: &mut BTreeMap<PathBuf, FolderEntry>, current_folder: &Path) {
    // Not folder so it may be a file or symbolic link so it isn't empty
    folder_entries.get_mut(current_folder).unwrap().is_empty = FolderEmptiness::No;
    let mut d = folder_entries.get_mut(current_folder).unwrap();
    // Loop to recursively set as non empty this and all his parent folders
    loop {
        d.is_empty = FolderEmptiness::No;
        if d.parent_path != None {
            let cf = d.parent_path.clone().unwrap();
            d = folder_entries.get_mut(&cf).unwrap();
        } else {
            break;
        }
    }
}
