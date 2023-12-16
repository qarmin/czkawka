use std::collections::BTreeMap;
use std::fs;
use std::fs::{DirEntry, FileType, Metadata, ReadDir};
use std::path::{Path, PathBuf};
use std::sync::atomic::Ordering;
use std::time::UNIX_EPOCH;

use crossbeam_channel::{Receiver, Sender};
use fun_time::fun_time;
use log::debug;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};

use crate::common::{check_if_stop_received, prepare_thread_handler_common, send_info_and_wait_for_ending_all_threads};
use crate::common_directory::Directories;
use crate::common_extensions::Extensions;
use crate::common_items::ExcludedItems;
use crate::common_tool::CommonToolData;
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

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
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
    #[default]
    None,
}

#[derive(PartialEq, Eq, Clone, Debug, Copy, Default, Deserialize, Serialize)]
pub enum CheckingMethod {
    #[default]
    None,
    Name,
    SizeName,
    Size,
    Hash,
    AudioTags,
    AudioContent,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct FileEntry {
    pub path: PathBuf,
    pub size: u64,
    pub modified_date: u64,
}

impl ResultEntry for FileEntry {
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

// Symlinks

#[derive(Clone, Debug, PartialEq, Eq, Copy, Deserialize, Serialize)]
pub enum ErrorType {
    InfiniteRecursion,
    NonExistentFile,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Collect {
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
    progress_sender: Option<&'b Sender<ProgressData>>,
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
    progress_sender: Option<&'b Sender<ProgressData>>,
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
            tool_type: ToolType::None,
        }
    }
}

impl<'a, 'b, F> DirTraversalBuilder<'a, 'b, F> {
    pub fn root_dirs(mut self, dirs: Vec<PathBuf>) -> Self {
        self.root_dirs = dirs;
        self
    }

    pub fn common_data(mut self, common_tool_data: &CommonToolData) -> Self {
        self.root_dirs = common_tool_data.directories.included_directories.clone();
        self.allowed_extensions = Some(common_tool_data.allowed_extensions.clone());
        self.excluded_items = Some(common_tool_data.excluded_items.clone());
        self.recursive_search = common_tool_data.recursive_search;
        self.minimal_file_size = Some(common_tool_data.minimal_file_size);
        self.maximal_file_size = Some(common_tool_data.maximal_file_size);
        self.tool_type = common_tool_data.tool_type;
        self.directories = Some(common_tool_data.directories.clone());
        self
    }

    pub fn stop_receiver(mut self, stop_receiver: Option<&'a Receiver<()>>) -> Self {
        self.stop_receiver = stop_receiver;
        self
    }

    pub fn progress_sender(mut self, progress_sender: Option<&'b Sender<ProgressData>>) -> Self {
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

    pub fn tool_type(mut self, tool_type: ToolType) -> Self {
        self.tool_type = tool_type;
        self
    }

    #[cfg(target_family = "unix")]
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
    Stopped,
}

fn entry_type(file_type: FileType) -> EntryType {
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
    #[fun_time(message = "run(collecting files/dirs)", level = "debug")]
    pub fn run(self) -> DirTraversalResult<T> {
        assert!(self.tool_type != ToolType::None, "Tool type cannot be None");

        let mut all_warnings = vec![];
        let mut grouped_file_entries: BTreeMap<T, Vec<FileEntry>> = BTreeMap::new();

        // Add root folders for finding
        let mut folders_to_check: Vec<PathBuf> = self.root_dirs.clone();

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
            if check_if_stop_received(stop_receiver) {
                send_info_and_wait_for_ending_all_threads(&progress_thread_run, progress_thread_handle);
                return DirTraversalResult::Stopped;
            }

            let segments: Vec<_> = folders_to_check
                .into_par_iter()
                .map(|current_folder| {
                    let mut dir_result = vec![];
                    let mut warnings = vec![];
                    let mut fe_result = vec![];

                    let Some(read_dir) = common_read_dir(&current_folder, &mut warnings) else {
                        return (dir_result, warnings, fe_result);
                    };

                    let mut counter = 0;
                    // Check every sub folder/file/link etc.
                    for entry in read_dir {
                        let Some(entry_data) = common_get_entry_data(&entry, &mut warnings, &current_folder) else {
                            continue;
                        };
                        let Ok(file_type) = entry_data.file_type() else { continue };

                        match (entry_type(file_type), collect) {
                            (EntryType::Dir, Collect::Files | Collect::InvalidSymlinks) => {
                                process_dir_in_file_symlink_mode(recursive_search, entry_data, &directories, &mut dir_result, &mut warnings, &excluded_items);
                            }
                            (EntryType::File, Collect::Files) => {
                                counter += 1;
                                process_file_in_file_mode(
                                    entry_data,
                                    &mut warnings,
                                    &mut fe_result,
                                    &allowed_extensions,
                                    &directories,
                                    &excluded_items,
                                    minimal_file_size,
                                    maximal_file_size,
                                );
                            }
                            (EntryType::File, Collect::InvalidSymlinks) => {
                                counter += 1;
                            }
                            (EntryType::Symlink, Collect::InvalidSymlinks) => {
                                counter += 1;
                                process_symlink_in_symlink_mode(entry_data, &mut warnings, &mut fe_result, &allowed_extensions, &directories, &excluded_items);
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
                    (dir_result, warnings, fe_result)
                })
                .collect();

            let required_size = segments.iter().map(|(segment, _, _)| segment.len()).sum::<usize>();
            folders_to_check = Vec::with_capacity(required_size);

            // Process collected data
            for (segment, warnings, fe_result) in segments {
                folders_to_check.extend(segment);
                all_warnings.extend(warnings);
                for fe in fe_result {
                    let key = (self.group_by)(&fe);
                    grouped_file_entries.entry(key).or_default().push(fe);
                }
            }
        }

        send_info_and_wait_for_ending_all_threads(&progress_thread_run, progress_thread_handle);

        debug!("Collected {} files", grouped_file_entries.values().map(Vec::len).sum::<usize>());

        match collect {
            Collect::Files | Collect::InvalidSymlinks => DirTraversalResult::SuccessFiles {
                grouped_file_entries,
                warnings: all_warnings,
            },
        }
    }
}

fn process_file_in_file_mode(
    entry_data: &DirEntry,
    warnings: &mut Vec<String>,
    fe_result: &mut Vec<FileEntry>,
    allowed_extensions: &Extensions,
    directories: &Directories,
    excluded_items: &ExcludedItems,
    minimal_file_size: u64,
    maximal_file_size: u64,
) {
    if !allowed_extensions.check_if_entry_ends_with_extension(entry_data) {
        return;
    }

    let current_file_name = entry_data.path();
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

    let Some(metadata) = common_get_metadata_dir(entry_data, warnings, &current_file_name) else {
        return;
    };

    if (minimal_file_size..=maximal_file_size).contains(&metadata.len()) {
        // Creating new file entry
        let fe: FileEntry = FileEntry {
            size: metadata.len(),
            modified_date: get_modified_time(&metadata, warnings, &current_file_name, false),
            path: current_file_name,
        };

        fe_result.push(fe);
    }
}

fn process_dir_in_file_symlink_mode(
    recursive_search: bool,
    entry_data: &DirEntry,
    directories: &Directories,
    dir_result: &mut Vec<PathBuf>,
    warnings: &mut Vec<String>,
    excluded_items: &ExcludedItems,
) {
    if !recursive_search {
        return;
    }

    let dir_path = entry_data.path();
    if directories.is_excluded(&dir_path) {
        return;
    }

    if excluded_items.is_excluded(&dir_path) {
        return;
    }

    #[cfg(target_family = "unix")]
    if directories.exclude_other_filesystems() {
        match directories.is_on_other_filesystems(&dir_path) {
            Ok(true) => return,
            Err(e) => warnings.push(e),
            _ => (),
        }
    }

    dir_result.push(dir_path);
}

fn process_symlink_in_symlink_mode(
    entry_data: &DirEntry,
    warnings: &mut Vec<String>,
    fe_result: &mut Vec<FileEntry>,
    allowed_extensions: &Extensions,
    directories: &Directories,
    excluded_items: &ExcludedItems,
) {
    if !allowed_extensions.check_if_entry_ends_with_extension(entry_data) {
        return;
    }

    let current_file_name = entry_data.path();
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

    let Some(metadata) = common_get_metadata_dir(entry_data, warnings, &current_file_name) else {
        return;
    };

    // Creating new file entry
    let fe: FileEntry = FileEntry {
        size: metadata.len(),
        modified_date: get_modified_time(&metadata, warnings, &current_file_name, false),
        path: current_file_name,
    };

    fe_result.push(fe);
}

pub fn common_read_dir(current_folder: &Path, warnings: &mut Vec<String>) -> Option<ReadDir> {
    match fs::read_dir(current_folder) {
        Ok(t) => Some(t),
        Err(e) => {
            warnings.push(flc!(
                "core_cannot_open_dir",
                generate_translation_hashmap(vec![("dir", current_folder.to_string_lossy().to_string()), ("reason", e.to_string())])
            ));
            None
        }
    }
}
pub fn common_get_entry_data<'a>(entry: &'a Result<DirEntry, std::io::Error>, warnings: &mut Vec<String>, current_folder: &Path) -> Option<&'a DirEntry> {
    let entry_data = match entry {
        Ok(t) => t,
        Err(e) => {
            warnings.push(flc!(
                "core_cannot_read_entry_dir",
                generate_translation_hashmap(vec![("dir", current_folder.to_string_lossy().to_string()), ("reason", e.to_string())])
            ));
            return None;
        }
    };
    Some(entry_data)
}
pub fn common_get_metadata_dir(entry_data: &DirEntry, warnings: &mut Vec<String>, current_folder: &Path) -> Option<Metadata> {
    let metadata: Metadata = match entry_data.metadata() {
        Ok(t) => t,
        Err(e) => {
            warnings.push(flc!(
                "core_cannot_read_metadata_dir",
                generate_translation_hashmap(vec![("dir", current_folder.to_string_lossy().to_string()), ("reason", e.to_string())])
            ));
            return None;
        }
    };
    Some(metadata)
}

pub fn common_get_entry_data_metadata<'a>(entry: &'a Result<DirEntry, std::io::Error>, warnings: &mut Vec<String>, current_folder: &Path) -> Option<(&'a DirEntry, Metadata)> {
    let entry_data = match entry {
        Ok(t) => t,
        Err(e) => {
            warnings.push(flc!(
                "core_cannot_read_entry_dir",
                generate_translation_hashmap(vec![("dir", current_folder.to_string_lossy().to_string()), ("reason", e.to_string())])
            ));
            return None;
        }
    };
    let metadata: Metadata = match entry_data.metadata() {
        Ok(t) => t,
        Err(e) => {
            warnings.push(flc!(
                "core_cannot_read_metadata_dir",
                generate_translation_hashmap(vec![("dir", current_folder.to_string_lossy().to_string()), ("reason", e.to_string())])
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
                let translation_hashmap = generate_translation_hashmap(vec![("name", current_file_name.to_string_lossy().to_string())]);
                if is_folder {
                    warnings.push(flc!("core_folder_modified_before_epoch", translation_hashmap));
                } else {
                    warnings.push(flc!("core_file_modified_before_epoch", translation_hashmap));
                }
                0
            }
        },
        Err(e) => {
            let translation_hashmap = generate_translation_hashmap(vec![("name", current_file_name.to_string_lossy().to_string()), ("reason", e.to_string())]);
            if is_folder {
                warnings.push(flc!("core_folder_no_modification_date", translation_hashmap));
            } else {
                warnings.push(flc!("core_file_no_modification_date", translation_hashmap));
            }
            0
        }
    }
}
