use std::collections::BTreeMap;
use std::fs;
use std::fs::{DirEntry, FileType, Metadata};
#[cfg(target_family = "unix")]
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::time::UNIX_EPOCH;

use crossbeam_channel::Sender;
use fun_time::fun_time;
use log::debug;
use rayon::prelude::*;

use crate::common::directories::Directories;
use crate::common::extensions::Extensions;
use crate::common::items::ExcludedItems;
use crate::common::model::{CheckingMethod, FileEntry, ToolType};
use crate::common::progress_data::{CurrentStage, ProgressData};
use crate::common::progress_stop_handler::{check_if_stop_received, prepare_thread_handler_common};
use crate::common::tool_data::CommonToolData;
use crate::flc;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Collect {
    InvalidSymlinks,
    Files,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum EntryType {
    File,
    Dir,
    Symlink,
    Other,
}

pub struct DirTraversalBuilder<'b, F> {
    group_by: Option<F>,
    root_dirs: Vec<PathBuf>,
    root_files: Vec<PathBuf>,
    stop_flag: Option<Arc<AtomicBool>>,
    progress_sender: Option<&'b Sender<ProgressData>>,
    minimal_file_size: Option<u64>,
    maximal_file_size: Option<u64>,
    checking_method: CheckingMethod,
    collect: Collect,
    recursive_search: bool,
    directories: Option<Directories>,
    excluded_items: Option<ExcludedItems>,
    extensions: Option<Extensions>,
    tool_type: ToolType,
}

#[derive(Debug)]
pub struct DirTraversal<'b, F> {
    group_by: F,
    root_dirs: Vec<PathBuf>,
    root_files: Vec<PathBuf>,
    stop_flag: Arc<AtomicBool>,
    progress_sender: Option<&'b Sender<ProgressData>>,
    recursive_search: bool,
    directories: Directories,
    excluded_items: ExcludedItems,
    extensions: Extensions,
    minimal_file_size: u64,
    maximal_file_size: u64,
    checking_method: CheckingMethod,
    tool_type: ToolType,
    collect: Collect,
}

impl Default for DirTraversalBuilder<'_, ()> {
    fn default() -> Self {
        Self::new()
    }
}

impl DirTraversalBuilder<'_, ()> {
    pub fn new() -> Self {
        DirTraversalBuilder {
            group_by: None,
            root_dirs: Vec::new(),
            root_files: Vec::new(),
            stop_flag: None,
            progress_sender: None,
            checking_method: CheckingMethod::None,
            minimal_file_size: None,
            maximal_file_size: None,
            collect: Collect::Files,
            recursive_search: false,
            directories: None,
            extensions: None,
            excluded_items: None,
            tool_type: ToolType::None,
        }
    }
}

impl<'b, F> DirTraversalBuilder<'b, F> {
    pub(crate) fn common_data(mut self, common_tool_data: &CommonToolData) -> Self {
        self.root_dirs = common_tool_data.directories.included_directories.clone();
        self.root_files = common_tool_data.directories.included_files.clone();
        self.extensions = Some(common_tool_data.extensions.clone());
        self.excluded_items = Some(common_tool_data.excluded_items.clone());
        self.recursive_search = common_tool_data.recursive_search;
        self.minimal_file_size = Some(common_tool_data.minimal_file_size);
        self.maximal_file_size = Some(common_tool_data.maximal_file_size);
        self.tool_type = common_tool_data.tool_type;
        self.directories = Some(common_tool_data.directories.clone());
        self
    }

    pub(crate) fn stop_flag(mut self, stop_flag: &Arc<AtomicBool>) -> Self {
        self.stop_flag = Some(stop_flag.clone());
        self
    }

    pub(crate) fn progress_sender(mut self, progress_sender: Option<&'b Sender<ProgressData>>) -> Self {
        self.progress_sender = progress_sender;
        self
    }

    pub(crate) fn checking_method(mut self, checking_method: CheckingMethod) -> Self {
        self.checking_method = checking_method;
        self
    }

    pub(crate) fn minimal_file_size(mut self, minimal_file_size: u64) -> Self {
        self.minimal_file_size = Some(minimal_file_size);
        self
    }

    pub(crate) fn maximal_file_size(mut self, maximal_file_size: u64) -> Self {
        self.maximal_file_size = Some(maximal_file_size);
        self
    }

    pub(crate) fn collect(mut self, collect: Collect) -> Self {
        self.collect = collect;
        self
    }

    pub(crate) fn group_by<G, T>(self, group_by: G) -> DirTraversalBuilder<'b, G>
    where
        G: Fn(&FileEntry) -> T,
    {
        DirTraversalBuilder {
            group_by: Some(group_by),
            root_dirs: self.root_dirs,
            root_files: self.root_files,
            stop_flag: self.stop_flag,
            progress_sender: self.progress_sender,
            directories: self.directories,
            extensions: self.extensions,
            excluded_items: self.excluded_items,
            recursive_search: self.recursive_search,
            maximal_file_size: self.maximal_file_size,
            minimal_file_size: self.minimal_file_size,
            collect: self.collect,
            checking_method: self.checking_method,
            tool_type: self.tool_type,
        }
    }

    pub(crate) fn build(self) -> DirTraversal<'b, F> {
        DirTraversal {
            group_by: self.group_by.expect("could not build"),
            root_dirs: self.root_dirs,
            root_files: self.root_files,
            stop_flag: self.stop_flag.expect("Stop flag must be always initialized"),
            progress_sender: self.progress_sender,
            checking_method: self.checking_method,
            minimal_file_size: self.minimal_file_size.unwrap_or(0),
            maximal_file_size: self.maximal_file_size.unwrap_or(u64::MAX),
            collect: self.collect,
            directories: self.directories.expect("could not build"),
            excluded_items: self.excluded_items.expect("could not build"),
            extensions: self.extensions.unwrap_or_default(),
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

impl<F, T> DirTraversal<'_, F>
where
    F: Fn(&FileEntry) -> T,
    T: Ord + PartialOrd,
{
    #[fun_time(message = "run(collecting files/dirs)", level = "debug")]
    pub(crate) fn run(self) -> DirTraversalResult<T> {
        assert_ne!(self.tool_type, ToolType::None, "Tool type cannot be None");

        let mut all_warnings = Vec::new();
        let mut grouped_file_entries: BTreeMap<T, Vec<FileEntry>> = BTreeMap::new();

        // Add root folders and files for finding
        let mut folders_to_check: Vec<PathBuf> = self.root_dirs.clone();
        let mut files_to_check: Vec<PathBuf> = self.root_files.clone();

        let progress_handler = prepare_thread_handler_common(self.progress_sender, CurrentStage::CollectingFiles, 0, (self.tool_type, self.checking_method), 0);

        let DirTraversal {
            collect,
            directories,
            excluded_items,
            extensions,
            recursive_search,
            minimal_file_size,
            maximal_file_size,
            stop_flag,
            ..
        } = self;

        let mut file_results = Vec::new();
        // File traversal
        while let Some(current_file) = files_to_check.pop() {
            let Some(metadata) = common_get_metadata_from_path(&current_file, &mut all_warnings) else {
                continue;
            };
            let file_type = metadata.file_type();
            match (entry_type(file_type), collect) {
                (EntryType::File, Collect::Files) => {
                    progress_handler.increase_items(1);
                    process_file_in_file_mode_path_check(
                        &current_file,
                        &metadata,
                        &mut all_warnings,
                        &mut file_results,
                        &extensions,
                        &excluded_items,
                        &directories,
                        minimal_file_size,
                        maximal_file_size,
                    );
                }
                (EntryType::File, Collect::InvalidSymlinks) => {
                    progress_handler.increase_items(1);
                }
                (EntryType::Symlink, Collect::InvalidSymlinks) => {
                    progress_handler.increase_items(1);
                    process_symlink_in_symlink_mode_path_check(&current_file, &metadata, &mut all_warnings, &mut file_results, &extensions, &excluded_items);
                }
                (EntryType::Symlink | EntryType::Dir | EntryType::Other, _) => {
                    // nothing to do
                }
            }
        }
        file_results.sort_by_cached_key(|fe| fe.path.to_string_lossy().to_string());
        for fe in file_results {
            let key = (self.group_by)(&fe);
            grouped_file_entries.entry(key).or_default().push(fe);
        }

        // Folder traversal
        while !folders_to_check.is_empty() {
            if check_if_stop_received(&stop_flag) {
                progress_handler.join_thread();
                return DirTraversalResult::Stopped;
            }

            let segments: Vec<_> = folders_to_check
                .into_par_iter()
                .with_max_len(2) // Avoiding checking too many folders in batch
                .map(|current_folder| {
                    let mut dir_result = Vec::new();
                    let mut warnings = Vec::new();
                    let mut fe_result = Vec::new();

                    let Some(read_dir) = common_read_dir(&current_folder, &mut warnings) else {
                        return Some((dir_result, warnings, fe_result));
                    };

                    let mut counter = 0;
                    // Check every sub folder/file/link etc.
                    for entry in read_dir {
                        if check_if_stop_received(&stop_flag) {
                            return None;
                        }

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
                                    &extensions,
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
                                process_symlink_in_symlink_mode(entry_data, &mut warnings, &mut fe_result, &extensions, &directories, &excluded_items);
                            }
                            (EntryType::Symlink, Collect::Files) | (EntryType::Other, _) => {
                                // nothing to do
                            }
                        }
                    }
                    if counter > 0 {
                        // Increase counter in batch, because usually it may be slow to add multiple times atomic value
                        progress_handler.increase_items(counter);
                    }
                    Some((dir_result, warnings, fe_result))
                })
                .while_some()
                .collect();

            let required_size = segments.iter().map(|(segment, _, _)| segment.len()).sum::<usize>();
            folders_to_check = Vec::with_capacity(required_size);

            // Process collected data
            for (segment, warnings, mut fe_result) in segments {
                folders_to_check.extend(segment);
                all_warnings.extend(warnings);
                fe_result.sort_by_cached_key(|fe| fe.path.to_string_lossy().to_string());
                for fe in fe_result {
                    let key = (self.group_by)(&fe);
                    grouped_file_entries.entry(key).or_default().push(fe);
                }
            }
        }

        progress_handler.join_thread();

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
    extensions: &Extensions,
    directories: &Directories,
    excluded_items: &ExcludedItems,
    minimal_file_size: u64,
    maximal_file_size: u64,
) {
    if !extensions.check_if_entry_have_valid_extension(&entry_data.file_name()) {
        return;
    }

    let current_file_name = entry_data.path();
    if excluded_items.is_excluded(&current_file_name) {
        return;
    }

    if directories.is_excluded_file(&current_file_name) {
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

    #[cfg(windows)]
    let _ = directories; // Silence unused variable warning on Windows

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
// Same as above, but working with Path instead of DirEntry
// Sadly this cannot be merged, due to a little crazy optimizations done in this functions
fn process_file_in_file_mode_path_check(
    path: &Path,
    metadata: &Metadata,
    warnings: &mut Vec<String>,
    fe_result: &mut Vec<FileEntry>,
    extensions: &Extensions,
    excluded_items: &ExcludedItems,
    directories: &Directories,
    minimal_file_size: u64,
    maximal_file_size: u64,
) {
    let Some(file_name) = path.file_name() else {
        return;
    };
    if !extensions.check_if_entry_have_valid_extension(file_name) {
        return;
    }

    if directories.is_excluded_file(path) {
        return;
    }
    if directories.is_excluded_item_in_dir(path) {
        return;
    }

    if excluded_items.is_excluded(path) {
        return;
    }

    if (minimal_file_size..=maximal_file_size).contains(&metadata.len()) {
        // Creating new file entry
        let fe: FileEntry = FileEntry {
            size: metadata.len(),
            modified_date: get_modified_time(metadata, warnings, path, false),
            path: path.to_path_buf(),
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
    if directories.is_excluded_dir(&dir_path) {
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

    #[cfg(windows)]
    let _ = warnings; // Silence unused variable warning on Windows

    dir_result.push(dir_path);
}

fn process_symlink_in_symlink_mode(
    entry_data: &DirEntry,
    warnings: &mut Vec<String>,
    fe_result: &mut Vec<FileEntry>,
    extensions: &Extensions,
    directories: &Directories,
    excluded_items: &ExcludedItems,
) {
    if !extensions.check_if_entry_have_valid_extension(&entry_data.file_name()) {
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

    #[cfg(windows)]
    let _ = directories; // Silence unused variable warning on Windows

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
fn process_symlink_in_symlink_mode_path_check(
    path: &Path,
    metadata: &Metadata,
    warnings: &mut Vec<String>,
    fe_result: &mut Vec<FileEntry>,
    extensions: &Extensions,
    excluded_items: &ExcludedItems,
) {
    let Some(file_name) = path.file_name() else {
        return;
    };
    if !extensions.check_if_entry_have_valid_extension(file_name) {
        return;
    }

    if excluded_items.is_excluded(path) {
        return;
    }

    // Creating new file entry
    let fe: FileEntry = FileEntry {
        size: metadata.len(),
        modified_date: get_modified_time(metadata, warnings, path, false),
        path: path.to_path_buf(),
    };

    fe_result.push(fe);
}

pub(crate) fn common_read_dir(current_folder: &Path, warnings: &mut Vec<String>) -> Option<Vec<Result<DirEntry, std::io::Error>>> {
    match fs::read_dir(current_folder) {
        Ok(t) => Some(t.collect()),
        Err(e) => {
            warnings.push(flc!("core_cannot_open_dir", dir = current_folder.to_string_lossy().to_string(), reason = e.to_string()));
            None
        }
    }
}
pub(crate) fn common_get_entry_data<'a>(entry: &'a Result<DirEntry, std::io::Error>, warnings: &mut Vec<String>, current_folder: &Path) -> Option<&'a DirEntry> {
    let entry_data = match entry {
        Ok(t) => t,
        Err(e) => {
            warnings.push(flc!(
                "core_cannot_read_entry_dir",
                dir = current_folder.to_string_lossy().to_string(),
                reason = e.to_string()
            ));
            return None;
        }
    };
    Some(entry_data)
}
pub(crate) fn common_get_metadata_dir(entry_data: &DirEntry, warnings: &mut Vec<String>, current_folder: &Path) -> Option<Metadata> {
    let metadata: Metadata = match entry_data.metadata() {
        Ok(t) => t,
        Err(e) => {
            warnings.push(flc!(
                "core_cannot_read_metadata_dir",
                dir = current_folder.to_string_lossy().to_string(),
                reason = e.to_string()
            ));
            return None;
        }
    };
    Some(metadata)
}

pub(crate) fn common_get_metadata_from_path(path: &Path, warnings: &mut Vec<String>) -> Option<Metadata> {
    let metadata: Metadata = match fs::metadata(path) {
        Ok(t) => t,
        Err(e) => {
            warnings.push(flc!("core_cannot_read_metadata_file", file = path.to_string_lossy().to_string(), reason = e.to_string()));
            return None;
        }
    };
    Some(metadata)
}

pub(crate) fn get_modified_time(metadata: &Metadata, warnings: &mut Vec<String>, current_file_name: &Path, is_folder: bool) -> u64 {
    match metadata.modified() {
        Ok(t) => match t.duration_since(UNIX_EPOCH) {
            Ok(d) => d.as_secs(),
            Err(_inspected) => {
                if is_folder {
                    warnings.push(flc!("core_folder_modified_before_epoch", name = current_file_name.to_string_lossy().to_string()));
                } else {
                    warnings.push(flc!("core_file_modified_before_epoch", name = current_file_name.to_string_lossy().to_string()));
                }
                0
            }
        },
        Err(e) => {
            if is_folder {
                warnings.push(flc!(
                    "core_folder_no_modification_date",
                    name = current_file_name.to_string_lossy().to_string(),
                    reason = e.to_string()
                ));
            } else {
                warnings.push(flc!(
                    "core_file_no_modification_date",
                    name = current_file_name.to_string_lossy().to_string(),
                    reason = e.to_string()
                ));
            }
            0
        }
    }
}

#[cfg(target_family = "windows")]
pub(crate) fn inode(_fe: &FileEntry) -> Option<u64> {
    None
}

#[cfg(target_family = "unix")]
pub(crate) fn inode(fe: &FileEntry) -> Option<u64> {
    if let Ok(meta) = fs::metadata(&fe.path) { Some(meta.ino()) } else { None }
}

pub(crate) fn take_1_per_inode((k, mut v): (Option<u64>, Vec<FileEntry>)) -> Vec<FileEntry> {
    if k.is_some() {
        v.drain(1..);
    }
    v
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::prelude::*;
    use std::time::{Duration, SystemTime};
    use std::{fs, io};

    use indexmap::IndexSet;
    use tempfile::TempDir;

    use super::*;
    use crate::common::tool_data::*;

    impl CommonData for CommonToolData {
        type Info = ();
        type Parameters = ();
        fn get_information(&self) -> Self::Info {}
        fn get_params(&self) -> Self::Parameters {}
        fn get_cd(&self) -> &CommonToolData {
            self
        }
        fn get_cd_mut(&mut self) -> &mut CommonToolData {
            self
        }
        fn found_any_items(&self) -> bool {
            false
        }
    }

    static NOW: std::sync::LazyLock<SystemTime> = std::sync::LazyLock::new(|| SystemTime::UNIX_EPOCH + Duration::new(100, 0));
    const CONTENT: &[u8; 1] = b"a";

    fn create_files(dir: &TempDir) -> io::Result<(PathBuf, PathBuf, PathBuf)> {
        let (src, hard, other_file) = (dir.path().join("a"), dir.path().join("b"), dir.path().join("c"));

        let mut file = File::create(&src)?;
        file.write_all(CONTENT)?;
        fs::hard_link(&src, &hard)?;
        file.set_modified(*NOW)?;

        let mut file = File::create(&other_file)?;
        file.write_all(CONTENT)?;
        file.set_modified(*NOW)?;
        Ok((src, hard, other_file))
    }

    #[test]
    fn test_traversal() -> io::Result<()> {
        let dir = tempfile::Builder::new().tempdir()?;
        let (src, hard, other_file) = create_files(&dir)?;
        let secs = NOW.duration_since(SystemTime::UNIX_EPOCH).expect("Cannot fail calculating duration since epoch").as_secs();

        let mut common_data = CommonToolData::new(ToolType::SimilarImages);
        common_data.directories.set_included_paths([dir.path().to_owned()].to_vec());
        common_data.set_minimal_file_size(0);

        match DirTraversalBuilder::new()
            .group_by(|_fe| ())
            .stop_flag(&Arc::default())
            .common_data(&common_data)
            .build()
            .run()
        {
            DirTraversalResult::SuccessFiles {
                warnings: _,
                grouped_file_entries,
            } => {
                let actual: IndexSet<_> = grouped_file_entries.into_values().flatten().collect();
                assert_eq!(
                    IndexSet::from([
                        FileEntry {
                            path: src,
                            size: 1,
                            modified_date: secs,
                        },
                        FileEntry {
                            path: hard,
                            size: 1,
                            modified_date: secs,
                        },
                        FileEntry {
                            path: other_file,
                            size: 1,
                            modified_date: secs,
                        },
                    ]),
                    actual
                );
            }
            DirTraversalResult::Stopped => {
                panic!("Expect SuccessFiles.");
            }
        }
        Ok(())
    }

    fn create_temp_structure(dir: &TempDir) -> io::Result<(PathBuf, PathBuf, PathBuf)> {
        let global_file = dir.path().join("global_file.txt");
        let other_dir = dir.path().join("other_file");
        fs::create_dir_all(&other_dir)?;
        let other_file = other_dir.join("other_file.txt");

        let mut f = File::create(&global_file)?;
        f.write_all(b"global_file")?;
        f.set_modified(*NOW)?;

        let mut f2 = File::create(&other_file)?;
        f2.write_all(b"other_file")?;
        f2.set_modified(*NOW)?;

        Ok((global_file, other_file, other_dir))
    }

    fn run_traversal(common_data: &CommonToolData) -> Vec<FileEntry> {
        match DirTraversalBuilder::new()
            .group_by(|_fe| ())
            .stop_flag(&Arc::default())
            .common_data(common_data)
            .build()
            .run()
        {
            DirTraversalResult::SuccessFiles { grouped_file_entries, .. } => grouped_file_entries.into_values().flatten().collect(),
            DirTraversalResult::Stopped => panic!("Expect SuccessFiles."),
        }
    }

    #[test]
    fn test_traversal_with_and_without_excluded_dir() -> io::Result<()> {
        let dir = tempfile::Builder::new().tempdir()?;
        let (global_file, other_file, other_dir) = create_temp_structure(&dir)?;
        let secs = NOW.duration_since(SystemTime::UNIX_EPOCH).expect("Cannot fail calculating duration since epoch").as_secs();

        let mut common_data = CommonToolData::new(ToolType::SimilarImages);
        common_data.directories.set_included_paths([dir.path().to_owned()].to_vec());
        common_data.set_minimal_file_size(0);

        let actual: IndexSet<_> = run_traversal(&common_data).into_iter().collect();
        assert_eq!(2, actual.len());
        assert!(actual.contains(&FileEntry {
            path: global_file.clone(),
            size: 11,
            modified_date: secs
        }));
        assert!(actual.contains(&FileEntry {
            path: other_file.clone(),
            size: 10,
            modified_date: secs
        }));

        let mut common_data2 = CommonToolData::new(ToolType::SimilarImages);
        common_data2.directories.set_included_paths([dir.path().to_owned()].to_vec());
        common_data2.directories.set_excluded_paths([other_dir].to_vec());
        common_data2.set_minimal_file_size(0);

        let actual: IndexSet<_> = run_traversal(&common_data2).into_iter().collect();
        assert_eq!(1, actual.len());
        assert!(actual.contains(&FileEntry {
            path: global_file.clone(),
            size: 11,
            modified_date: secs
        }));

        let mut common_data3 = CommonToolData::new(ToolType::SimilarImages);
        common_data3.directories.set_included_paths([dir.path().to_owned()].to_vec());
        common_data3.directories.set_excluded_paths([other_file.clone()].to_vec());
        common_data3.set_minimal_file_size(0);

        let actual: IndexSet<_> = run_traversal(&common_data3).into_iter().collect();
        assert_eq!(1, actual.len());
        assert!(actual.contains(&FileEntry {
            path: global_file.clone(),
            size: 11,
            modified_date: secs
        }));

        let mut common_data4 = CommonToolData::new(ToolType::SimilarImages);
        common_data4.directories.set_included_paths([global_file.clone()].to_vec());
        common_data4.set_minimal_file_size(0);

        let actual: IndexSet<_> = run_traversal(&common_data4).into_iter().collect();
        assert_eq!(1, actual.len());
        assert!(actual.contains(&FileEntry {
            path: global_file.clone(),
            size: 11,
            modified_date: secs
        }));

        let mut common_data5 = CommonToolData::new(ToolType::SimilarImages);
        common_data5.directories.set_included_paths([global_file.clone(), other_file.clone()].to_vec());
        common_data5.set_minimal_file_size(0);

        let actual: IndexSet<_> = run_traversal(&common_data5).into_iter().collect();
        assert_eq!(2, actual.len());
        assert!(actual.contains(&FileEntry {
            path: global_file.clone(),
            size: 11,
            modified_date: secs
        }));
        assert!(actual.contains(&FileEntry {
            path: other_file.clone(),
            size: 10,
            modified_date: secs
        }));

        let mut common_data6 = CommonToolData::new(ToolType::SimilarImages);
        common_data6.directories.set_included_paths([global_file.clone(), other_file.clone()].to_vec());
        common_data6.directories.set_excluded_paths([other_file].to_vec());
        common_data6.set_minimal_file_size(0);

        let actual: IndexSet<_> = run_traversal(&common_data6).into_iter().collect();
        assert_eq!(1, actual.len());
        assert!(actual.contains(&FileEntry {
            path: global_file,
            size: 11,
            modified_date: secs
        }));

        // This test is invalid - other dir should be removed by optimizer
        // let mut common_data7 = CommonToolData::new(ToolType::SimilarImages);
        // common_data7.directories.set_included_paths([other_file.clone()].to_vec());
        // common_data7.directories.set_excluded_paths([other_dir.clone()].to_vec());
        // common_data7.set_minimal_file_size(0);
        //
        // let actual: IndexSet<_> = run_traversal(&common_data7).into_iter().collect();
        // assert_eq!(0, actual.len());

        Ok(())
    }

    #[cfg(target_family = "unix")]
    #[test]
    fn test_traversal_group_by_inode() -> io::Result<()> {
        let dir = tempfile::Builder::new().tempdir()?;
        let (src, _, other) = create_files(&dir)?;
        let secs = NOW.duration_since(SystemTime::UNIX_EPOCH).expect("Cannot fail calculating duration since epoch").as_secs();

        let mut common_data = CommonToolData::new(ToolType::SimilarImages);
        common_data.directories.set_included_paths([dir.path().to_owned()].to_vec());
        common_data.set_minimal_file_size(0);

        match DirTraversalBuilder::new()
            .group_by(inode)
            .stop_flag(&Arc::default())
            .common_data(&common_data)
            .build()
            .run()
        {
            DirTraversalResult::SuccessFiles {
                warnings: _,
                grouped_file_entries,
            } => {
                let actual: IndexSet<_> = grouped_file_entries.into_iter().flat_map(take_1_per_inode).collect();
                assert_eq!(
                    IndexSet::from([
                        FileEntry {
                            path: src,
                            size: 1,
                            modified_date: secs,
                        },
                        FileEntry {
                            path: other,
                            size: 1,
                            modified_date: secs,
                        },
                    ]),
                    actual
                );
            }
            DirTraversalResult::Stopped => {
                panic!("Expect SuccessFiles.");
            }
        }
        Ok(())
    }

    #[cfg(target_family = "windows")]
    #[test]
    fn test_traversal_group_by_inode() -> io::Result<()> {
        let dir = tempfile::Builder::new().tempdir()?;
        let (src, hard, other) = create_files(&dir)?;
        let secs = NOW.duration_since(SystemTime::UNIX_EPOCH).expect("Cannot fail duration from epoch").as_secs();

        let mut common_data = CommonToolData::new(ToolType::SimilarImages);
        common_data.directories.set_included_paths([dir.path().to_owned()].to_vec());
        common_data.set_minimal_file_size(0);

        match DirTraversalBuilder::new()
            .group_by(inode)
            .stop_flag(&Arc::default())
            .common_data(&common_data)
            .build()
            .run()
        {
            DirTraversalResult::SuccessFiles {
                warnings: _,
                grouped_file_entries,
            } => {
                let actual: IndexSet<_> = grouped_file_entries.into_iter().flat_map(take_1_per_inode).collect();
                assert_eq!(
                    IndexSet::from([
                        FileEntry {
                            path: src,
                            size: 1,
                            modified_date: secs,
                        },
                        FileEntry {
                            path: hard,
                            size: 1,
                            modified_date: secs,
                        },
                        FileEntry {
                            path: other,
                            size: 1,
                            modified_date: secs,
                        },
                    ]),
                    actual
                );
            }
            _ => {
                panic!("Expect SuccessFiles.");
            }
        };
        Ok(())
    }
}
