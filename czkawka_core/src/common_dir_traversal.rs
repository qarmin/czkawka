use std::collections::BTreeMap;
use std::fs;
use std::fs::{DirEntry, Metadata, ReadDir};
use std::path::{Path, PathBuf};
use std::sync::atomic::Ordering;
use std::time::UNIX_EPOCH;

use crossbeam_channel::Receiver;
use futures::channel::mpsc::UnboundedSender;
use rayon::prelude::*;

use crate::common::{prepare_thread_handler_common, send_info_and_wait_for_ending_all_threads};
use crate::common_directory::Directories;
use crate::common_extensions::Extensions;
use crate::common_items::ExcludedItems;
use crate::common_traits::ResultEntry;
use crate::flc;
use crate::localizer_core::generate_translation_hashmap;

#[derive(Debug)]
pub struct ProgressData {
    pub checking_method: CheckingMethod,
    pub current_stage: u8,
    pub max_stage: u8,
    pub entries_checked: usize,
    pub entries_to_check: usize,
    pub tool_type: ToolType,
}

#[derive(PartialEq, Eq, Clone, Debug, Copy)]
pub enum ToolType {
    Duplicate,
    EmptyFolders,
    EmptyFiles,
    InvalidSymlinks,
    BrokenFiles,
    BadExtensions,
    BigFile,
    SameMusic,
    SimilarImages,
    SimilarVideos,
    TemporaryFiles,
    None,
}

#[derive(PartialEq, Eq, Clone, Debug, Copy)]
pub enum CheckingMethod {
    None,
    Name,
    SizeName,
    Size,
    Hash,
    AudioTags,
    AudioContent,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct FileEntry {
    pub path: PathBuf,
    pub size: u64,
    pub modified_date: u64,
    pub hash: String,
    pub symlink_info: Option<SymlinkInfo>,
}
impl ResultEntry for FileEntry {
    fn get_path(&self) -> &Path {
        &self.path
    }
}

// Symlinks

const MAX_NUMBER_OF_SYMLINK_JUMPS: i32 = 20;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SymlinkInfo {
    pub destination_path: PathBuf,
    pub type_of_error: ErrorType,
}

#[derive(Clone, Debug, PartialEq, Eq, Copy)]
pub enum ErrorType {
    InfiniteRecursion,
    NonExistentFile,
}

// Empty folders

/// Enum with values which show if folder is empty.
/// In function "`optimize_folders`" automatically "Maybe" is changed to "Yes", so it is not necessary to put it here
#[derive(Eq, PartialEq, Copy, Clone)]
pub(crate) enum FolderEmptiness {
    No,
    Maybe,
}

/// Struct assigned to each checked folder with parent path(used to ignore parent if children are not empty) and flag which shows if folder is empty
#[derive(Clone)]
pub struct FolderEntry {
    pub(crate) parent_path: Option<PathBuf>,
    // Usable only when finding
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

#[derive(Eq, PartialEq, Copy, Clone)]
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
    progress_sender: Option<&'b UnboundedSender<ProgressData>>,
    minimal_file_size: Option<u64>,
    maximal_file_size: Option<u64>,
    checking_method: CheckingMethod,
    max_stage: u8,
    collect: Collect,
    recursive_search: bool,
    directories: Option<Directories>,
    excluded_items: Option<ExcludedItems>,
    allowed_extensions: Option<Extensions>,
    tool_type: ToolType,
}

pub struct DirTraversal<'a, 'b, F> {
    group_by: F,
    root_dirs: Vec<PathBuf>,
    stop_receiver: Option<&'a Receiver<()>>,
    progress_sender: Option<&'b UnboundedSender<ProgressData>>,
    recursive_search: bool,
    directories: Directories,
    excluded_items: ExcludedItems,
    allowed_extensions: Extensions,
    minimal_file_size: u64,
    maximal_file_size: u64,
    checking_method: CheckingMethod,
    max_stage: u8,
    tool_type: ToolType,
    collect: Collect,
}

impl<'a, 'b> Default for DirTraversalBuilder<'a, 'b, ()> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, 'b> DirTraversalBuilder<'a, 'b, ()> {
    #[must_use]
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
            tool_type: ToolType::BadExtensions,
        }
    }
}

impl<'a, 'b, F> DirTraversalBuilder<'a, 'b, F> {
    #[must_use]
    pub fn root_dirs(mut self, dirs: Vec<PathBuf>) -> Self {
        self.root_dirs = dirs;
        self
    }

    #[must_use]
    pub fn stop_receiver(mut self, stop_receiver: Option<&'a Receiver<()>>) -> Self {
        self.stop_receiver = stop_receiver;
        self
    }

    #[must_use]
    pub fn progress_sender(mut self, progress_sender: Option<&'b UnboundedSender<ProgressData>>) -> Self {
        self.progress_sender = progress_sender;
        self
    }

    #[must_use]
    pub fn checking_method(mut self, checking_method: CheckingMethod) -> Self {
        self.checking_method = checking_method;
        self
    }

    #[must_use]
    pub fn max_stage(mut self, max_stage: u8) -> Self {
        self.max_stage = max_stage;
        self
    }

    #[must_use]
    pub fn minimal_file_size(mut self, minimal_file_size: u64) -> Self {
        self.minimal_file_size = Some(minimal_file_size);
        self
    }

    #[must_use]
    pub fn maximal_file_size(mut self, maximal_file_size: u64) -> Self {
        self.maximal_file_size = Some(maximal_file_size);
        self
    }

    #[must_use]
    pub fn collect(mut self, collect: Collect) -> Self {
        self.collect = collect;
        self
    }

    #[must_use]
    pub fn directories(mut self, directories: Directories) -> Self {
        self.directories = Some(directories);
        self
    }

    #[must_use]
    pub fn allowed_extensions(mut self, allowed_extensions: Extensions) -> Self {
        self.allowed_extensions = Some(allowed_extensions);
        self
    }

    #[must_use]
    pub fn excluded_items(mut self, excluded_items: ExcludedItems) -> Self {
        self.excluded_items = Some(excluded_items);
        self
    }

    #[must_use]
    pub fn recursive_search(mut self, recursive_search: bool) -> Self {
        self.recursive_search = recursive_search;
        self
    }

    #[must_use]
    pub fn tool_type(mut self, tool_type: ToolType) -> Self {
        self.tool_type = tool_type;
        self
    }

    #[cfg(target_family = "unix")]
    #[must_use]
    pub fn exclude_other_filesystems(mut self, exclude_other_filesystems: bool) -> Self {
        match self.directories {
            Some(ref mut directories) => directories.set_exclude_other_filesystems(exclude_other_filesystems),
            None => panic!("Directories is None"),
        }
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
            tool_type: self.tool_type,
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
            tool_type: self.tool_type,
        }
    }
}

pub enum DirTraversalResult<T: Ord + PartialOrd> {
    SuccessFiles {
        warnings: Vec<String>,
        grouped_file_entries: BTreeMap<T, Vec<FileEntry>>,
    },
    SuccessFolders {
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

        let (progress_thread_handle, progress_thread_run, atomic_counter, _check_was_stopped) =
            prepare_thread_handler_common(self.progress_sender, 0, self.max_stage, 0, self.checking_method, self.tool_type);

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
                send_info_and_wait_for_ending_all_threads(&progress_thread_run, progress_thread_handle);
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

                    let Some(read_dir) = common_read_dir(current_folder, &mut warnings) else {
                        return (dir_result, warnings, fe_result, set_as_not_empty_folder_list, folder_entries_list);
                    };

                    let mut counter = 0;
                    // Check every sub folder/file/link etc.
                    'dir: for entry in read_dir {
                        let Some((entry_data, metadata)) = common_get_entry_data_metadata(&entry, &mut warnings, current_folder) else {
                            continue;
                        };

                        match (entry_type(&metadata), collect) {
                            (EntryType::Dir, Collect::Files | Collect::InvalidSymlinks) => {
                                process_dir_in_file_symlink_mode(recursive_search, current_folder, entry_data, &directories, &mut dir_result, &mut warnings, &excluded_items);
                            }
                            (EntryType::Dir, Collect::EmptyFolders) => {
                                counter += 1;
                                process_dir_in_dir_mode(
                                    &metadata,
                                    current_folder,
                                    entry_data,
                                    &directories,
                                    &mut dir_result,
                                    &mut warnings,
                                    &excluded_items,
                                    &mut set_as_not_empty_folder_list,
                                    &mut folder_entries_list,
                                );
                            }
                            (EntryType::File, Collect::Files) => {
                                counter += 1;
                                process_file_in_file_mode(
                                    &metadata,
                                    entry_data,
                                    &mut warnings,
                                    &mut fe_result,
                                    &allowed_extensions,
                                    current_folder,
                                    &directories,
                                    &excluded_items,
                                    minimal_file_size,
                                    maximal_file_size,
                                );
                            }
                            (EntryType::File | EntryType::Symlink, Collect::EmptyFolders) => {
                                #[cfg(target_family = "unix")]
                                if directories.exclude_other_filesystems() {
                                    match directories.is_on_other_filesystems(current_folder) {
                                        Ok(true) => continue 'dir,
                                        Err(e) => warnings.push(e.to_string()),
                                        _ => (),
                                    }
                                }

                                set_as_not_empty_folder_list.push(current_folder.clone());
                            }
                            (EntryType::File, Collect::InvalidSymlinks) => {
                                counter += 1;
                            }
                            (EntryType::Symlink, Collect::InvalidSymlinks) => {
                                counter += 1;
                                process_symlink_in_symlink_mode(
                                    &metadata,
                                    entry_data,
                                    &mut warnings,
                                    &mut fe_result,
                                    &allowed_extensions,
                                    current_folder,
                                    &directories,
                                    &excluded_items,
                                );
                            }
                            (EntryType::Symlink, Collect::Files) | (EntryType::Other, _) => {
                                // nothing to do
                            }
                        }
                    }
                    if counter > 0 {
                        // Increase counter in batch, because usually it may be slow to add multiple times atomic value
                        atomic_counter.fetch_add(counter, Ordering::Relaxed);
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

        send_info_and_wait_for_ending_all_threads(&progress_thread_run, progress_thread_handle);

        match collect {
            Collect::Files | Collect::InvalidSymlinks => DirTraversalResult::SuccessFiles {
                grouped_file_entries,
                warnings: all_warnings,
            },
            Collect::EmptyFolders => DirTraversalResult::SuccessFolders {
                folder_entries,
                warnings: all_warnings,
            },
        }
    }
}

fn process_file_in_file_mode(
    metadata: &Metadata,
    entry_data: &DirEntry,
    warnings: &mut Vec<String>,
    fe_result: &mut Vec<FileEntry>,
    allowed_extensions: &Extensions,
    current_folder: &Path,
    directories: &Directories,
    excluded_items: &ExcludedItems,
    minimal_file_size: u64,
    maximal_file_size: u64,
) {
    let Some(file_name_lowercase) = get_lowercase_name(entry_data, warnings) else {
        return;
    };

    if !allowed_extensions.matches_filename(&file_name_lowercase) {
        return;
    }

    if (minimal_file_size..=maximal_file_size).contains(&metadata.len()) {
        let current_file_name = current_folder.join(entry_data.file_name());
        if excluded_items.is_excluded(&current_file_name) {
            return;
        }

        #[cfg(target_family = "unix")]
        if directories.exclude_other_filesystems() {
            match directories.is_on_other_filesystems(&current_file_name) {
                Ok(true) => return,
                Err(e) => warnings.push(e),
                _ => (),
            }
        }

        // Creating new file entry
        let fe: FileEntry = FileEntry {
            path: current_file_name.clone(),
            size: metadata.len(),
            modified_date: get_modified_time(metadata, warnings, &current_file_name, false),
            hash: String::new(),
            symlink_info: None,
        };

        fe_result.push(fe);
    }
}

fn process_dir_in_dir_mode(
    metadata: &Metadata,
    current_folder: &Path,
    entry_data: &DirEntry,
    directories: &Directories,
    dir_result: &mut Vec<PathBuf>,
    warnings: &mut Vec<String>,
    excluded_items: &ExcludedItems,
    set_as_not_empty_folder_list: &mut Vec<PathBuf>,
    folder_entries_list: &mut Vec<(PathBuf, FolderEntry)>,
) {
    let next_folder = current_folder.join(entry_data.file_name());
    if excluded_items.is_excluded(&next_folder) || directories.is_excluded(&next_folder) {
        set_as_not_empty_folder_list.push(current_folder.to_path_buf());
        return;
    }

    #[cfg(target_family = "unix")]
    if directories.exclude_other_filesystems() {
        match directories.is_on_other_filesystems(&next_folder) {
            Ok(true) => return,
            Err(e) => warnings.push(e),
            _ => (),
        }
    }

    dir_result.push(next_folder.clone());
    folder_entries_list.push((
        next_folder,
        FolderEntry {
            parent_path: Some(current_folder.to_path_buf()),
            is_empty: FolderEmptiness::Maybe,
            modified_date: get_modified_time(metadata, warnings, current_folder, true),
        },
    ));
}

fn process_dir_in_file_symlink_mode(
    recursive_search: bool,
    current_folder: &Path,
    entry_data: &DirEntry,
    directories: &Directories,
    dir_result: &mut Vec<PathBuf>,
    warnings: &mut Vec<String>,
    excluded_items: &ExcludedItems,
) {
    if !recursive_search {
        return;
    }

    let next_folder = current_folder.join(entry_data.file_name());
    if directories.is_excluded(&next_folder) {
        return;
    }

    if excluded_items.is_excluded(&next_folder) {
        return;
    }

    #[cfg(target_family = "unix")]
    if directories.exclude_other_filesystems() {
        match directories.is_on_other_filesystems(&next_folder) {
            Ok(true) => return,
            Err(e) => warnings.push(e),
            _ => (),
        }
    }

    dir_result.push(next_folder);
}

fn process_symlink_in_symlink_mode(
    metadata: &Metadata,
    entry_data: &DirEntry,
    warnings: &mut Vec<String>,
    fe_result: &mut Vec<FileEntry>,
    allowed_extensions: &Extensions,
    current_folder: &Path,
    directories: &Directories,
    excluded_items: &ExcludedItems,
) {
    let Some(file_name_lowercase) = get_lowercase_name(entry_data, warnings) else {
        return;
    };

    if !allowed_extensions.matches_filename(&file_name_lowercase) {
        return;
    }

    let current_file_name = current_folder.join(entry_data.file_name());
    if excluded_items.is_excluded(&current_file_name) {
        return;
    }

    #[cfg(target_family = "unix")]
    if directories.exclude_other_filesystems() {
        match directories.is_on_other_filesystems(current_folder) {
            Ok(true) => return,
            Err(e) => warnings.push(e),
            _ => (),
        }
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
                        return;
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
        modified_date: get_modified_time(metadata, warnings, &current_file_name, false),
        size: 0,
        hash: String::new(),
        symlink_info: Some(SymlinkInfo { destination_path, type_of_error }),
    };

    // Adding files to Vector
    fe_result.push(fe);
}

pub fn common_read_dir(current_folder: &Path, warnings: &mut Vec<String>) -> Option<ReadDir> {
    match fs::read_dir(current_folder) {
        Ok(t) => Some(t),
        Err(e) => {
            warnings.push(flc!(
                "core_cannot_open_dir",
                generate_translation_hashmap(vec![("dir", current_folder.display().to_string()), ("reason", e.to_string())])
            ));
            None
        }
    }
}

pub fn common_get_entry_data_metadata<'a>(entry: &'a Result<DirEntry, std::io::Error>, warnings: &mut Vec<String>, current_folder: &Path) -> Option<(&'a DirEntry, Metadata)> {
    let entry_data = match entry {
        Ok(t) => t,
        Err(e) => {
            warnings.push(flc!(
                "core_cannot_read_entry_dir",
                generate_translation_hashmap(vec![("dir", current_folder.display().to_string()), ("reason", e.to_string())])
            ));
            return None;
        }
    };
    let metadata: Metadata = match entry_data.metadata() {
        Ok(t) => t,
        Err(e) => {
            warnings.push(flc!(
                "core_cannot_read_metadata_dir",
                generate_translation_hashmap(vec![("dir", current_folder.display().to_string()), ("reason", e.to_string())])
            ));
            return None;
        }
    };
    Some((entry_data, metadata))
}

pub fn get_modified_time(metadata: &Metadata, warnings: &mut Vec<String>, current_file_name: &Path, is_folder: bool) -> u64 {
    match metadata.modified() {
        Ok(t) => match t.duration_since(UNIX_EPOCH) {
            Ok(d) => d.as_secs(),
            Err(_inspected) => {
                let translation_hashmap = generate_translation_hashmap(vec![("name", current_file_name.display().to_string())]);
                if is_folder {
                    warnings.push(flc!("core_folder_modified_before_epoch", translation_hashmap));
                } else {
                    warnings.push(flc!("core_file_modified_before_epoch", translation_hashmap));
                }
                0
            }
        },
        Err(e) => {
            let translation_hashmap = generate_translation_hashmap(vec![("name", current_file_name.display().to_string()), ("reason", e.to_string())]);
            if is_folder {
                warnings.push(flc!("core_folder_no_modification_date", translation_hashmap));
            } else {
                warnings.push(flc!("core_file_no_modification_date", translation_hashmap));
            }
            0
        }
    }
}

pub fn get_lowercase_name(entry_data: &DirEntry, warnings: &mut Vec<String>) -> Option<String> {
    let name = match entry_data.file_name().into_string() {
        Ok(t) => t,
        Err(_inspected) => {
            warnings.push(flc!(
                "core_file_not_utf8_name",
                generate_translation_hashmap(vec![("name", entry_data.path().display().to_string())])
            ));
            return None;
        }
    }
    .to_lowercase();
    Some(name)
}

fn set_as_not_empty_folder(folder_entries: &mut BTreeMap<PathBuf, FolderEntry>, current_folder: &Path) {
    let mut d = folder_entries.get_mut(current_folder).unwrap();
    // Not folder so it may be a file or symbolic link so it isn't empty
    d.is_empty = FolderEmptiness::No;
    // Loop to recursively set as non empty this and all his parent folders
    loop {
        d.is_empty = FolderEmptiness::No;
        if d.parent_path.is_some() {
            let cf = d.parent_path.clone().unwrap();
            d = folder_entries.get_mut(&cf).unwrap();
        } else {
            break;
        }
    }
}
