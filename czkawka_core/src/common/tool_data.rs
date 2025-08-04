use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::time::Duration;

use crossbeam_channel::Sender;
use humansize::{format_size, BINARY};
use log::info;
use rayon::prelude::*;

use crate::common::directories::Directories;
use crate::common::extensions::Extensions;
use crate::common::items::ExcludedItems;
use crate::common::model::{CheckingMethod, ToolType, WorkContinueStatus};
use crate::common::progress_data::{CurrentStage, ProgressData};
use crate::common::remove_folder_if_contains_only_empty_folders;
use crate::common_traits::ResultEntry;
use crate::helpers::delayed_sender::DelayedSender;
use crate::helpers::messages::Messages;

#[derive(Debug, Clone, Default)]
pub struct CommonToolData {
    pub(crate) tool_type: ToolType,
    pub(crate) text_messages: Messages,
    pub(crate) directories: Directories,
    pub(crate) extensions: Extensions,
    pub(crate) excluded_items: ExcludedItems,
    pub(crate) recursive_search: bool,
    pub(crate) delete_method: DeleteMethod,
    pub(crate) maximal_file_size: u64,
    pub(crate) minimal_file_size: u64,
    pub(crate) stopped_search: bool,
    pub(crate) use_cache: bool,
    pub(crate) delete_outdated_cache: bool,
    pub(crate) save_also_as_json: bool,
    pub(crate) use_reference_folders: bool,
    pub(crate) dry_run: bool,
}

#[derive(Debug, Clone, Default)]
pub struct DeleteResult {
    deleted_files: usize,
    gained_bytes: u64,
    failed_to_delete_files: usize,
    errors: Vec<String>,
    infos: Vec<String>,
}

impl DeleteResult {
    pub(crate) fn add_to_messages(&self, messages: &mut Messages) {
        messages.errors.extend(self.errors.clone());
        messages.messages.extend(self.infos.clone());
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum DeleteItemType<T: ResultEntry + Sized + Send + Sync> {
    DeletingFiles(Vec<T>),
    DeletingFolders(Vec<T>),
    HardlinkingFiles(Vec<(T, Vec<T>)>),
}

impl<T: ResultEntry + Sized + Send + Sync> DeleteItemType<T> {
    fn calculate_size_to_delete(&self) -> u64 {
        match &self {
            Self::DeletingFiles(items) | Self::DeletingFolders(items) => items.iter().map(|item| item.get_size()).sum(),
            Self::HardlinkingFiles(items) => items.iter().map(|(item, _)| item.get_size()).sum(),
        }
    }

    fn calculate_entries_to_delete(&self) -> usize {
        match &self {
            Self::DeletingFiles(items) | Self::DeletingFolders(items) => items.len(),
            Self::HardlinkingFiles(items) => items.iter().map(|(_original, files)| files.len()).sum(),
        }
    }
}

#[derive(Eq, PartialEq, Clone, Debug, Copy, Default)]
pub enum DeleteMethod {
    #[default]
    None,
    Delete, // Just delete items
    AllExceptNewest,
    AllExceptOldest,
    OneOldest,
    OneNewest,
    HardLink,
    AllExceptBiggest,
    AllExceptSmallest,
    OneBiggest,
    OneSmallest,
}

impl CommonToolData {
    pub fn new(tool_type: ToolType) -> Self {
        Self {
            tool_type,
            text_messages: Messages::new(),
            directories: Directories::new(),
            extensions: Extensions::new(),
            excluded_items: ExcludedItems::new(),
            recursive_search: true,
            delete_method: DeleteMethod::None,
            maximal_file_size: u64::MAX,
            minimal_file_size: 8192,
            stopped_search: false,
            use_cache: true,
            delete_outdated_cache: true,
            save_also_as_json: false,
            use_reference_folders: false,
            dry_run: false,
        }
    }
}

pub trait CommonData {
    fn get_cd(&self) -> &CommonToolData;
    fn get_cd_mut(&mut self) -> &mut CommonToolData;
    fn get_check_method(&self) -> CheckingMethod {
        CheckingMethod::None
    }
    fn get_test_type(&self) -> (ToolType, CheckingMethod) {
        (self.get_cd().tool_type, self.get_check_method())
    }
    fn found_any_broken_files(&self) -> bool;

    fn get_tool_type(&self) -> ToolType {
        self.get_cd().tool_type
    }

    fn set_dry_run(&mut self, dry_run: bool) {
        self.get_cd_mut().dry_run = dry_run;
    }
    fn get_dry_run(&self) -> bool {
        self.get_cd().dry_run
    }

    fn set_use_cache(&mut self, use_cache: bool) {
        self.get_cd_mut().use_cache = use_cache;
    }
    fn get_use_cache(&self) -> bool {
        self.get_cd().use_cache
    }

    fn set_delete_outdated_cache(&mut self, delete_outdated_cache: bool) {
        self.get_cd_mut().delete_outdated_cache = delete_outdated_cache;
    }
    fn get_delete_outdated_cache(&self) -> bool {
        self.get_cd().delete_outdated_cache
    }

    fn get_stopped_search(&self) -> bool {
        self.get_cd().stopped_search
    }
    fn set_stopped_search(&mut self, stopped_search: bool) {
        self.get_cd_mut().stopped_search = stopped_search;
    }

    fn set_maximal_file_size(&mut self, maximal_file_size: u64) {
        self.get_cd_mut().maximal_file_size = match maximal_file_size {
            0 => 1,
            t => t,
        };
    }
    fn get_maximal_file_size(&self) -> u64 {
        self.get_cd().maximal_file_size
    }

    fn set_minimal_file_size(&mut self, minimal_file_size: u64) {
        self.get_cd_mut().minimal_file_size = match minimal_file_size {
            0 => 1,
            t => t,
        };
    }
    fn get_minimal_file_size(&self) -> u64 {
        self.get_cd().minimal_file_size
    }

    fn set_reference_directory(&mut self, reference_directory: Vec<PathBuf>) {
        let messages = self.get_cd_mut().directories.set_reference_directory(&reference_directory);
        self.get_cd_mut().text_messages.extend_with_another_messages(messages);
    }

    #[cfg(target_family = "unix")]
    fn set_exclude_other_filesystems(&mut self, exclude_other_filesystems: bool) {
        self.get_cd_mut().directories.set_exclude_other_filesystems(exclude_other_filesystems);
    }
    #[cfg(not(target_family = "unix"))]
    fn set_exclude_other_filesystems(&mut self, _exclude_other_filesystems: bool) {}

    fn get_text_messages(&self) -> &Messages {
        &self.get_cd().text_messages
    }
    fn get_text_messages_mut(&mut self) -> &mut Messages {
        &mut self.get_cd_mut().text_messages
    }

    fn set_save_also_as_json(&mut self, save_also_as_json: bool) {
        self.get_cd_mut().save_also_as_json = save_also_as_json;
    }
    fn get_save_also_as_json(&self) -> bool {
        self.get_cd().save_also_as_json
    }

    fn set_recursive_search(&mut self, recursive_search: bool) {
        self.get_cd_mut().recursive_search = recursive_search;
    }
    fn get_recursive_search(&self) -> bool {
        self.get_cd().recursive_search
    }

    fn set_use_reference_folders(&mut self, use_reference_folders: bool) {
        self.get_cd_mut().use_reference_folders = use_reference_folders;
    }
    fn get_use_reference_folders(&self) -> bool {
        self.get_cd().use_reference_folders
    }

    fn set_delete_method(&mut self, delete_method: DeleteMethod) {
        self.get_cd_mut().delete_method = delete_method;
    }
    fn get_delete_method(&self) -> DeleteMethod {
        self.get_cd().delete_method
    }

    fn set_included_directory(&mut self, included_directory: Vec<PathBuf>) {
        let messages = self.get_cd_mut().directories.set_included_directory(included_directory);
        self.get_cd_mut().text_messages.extend_with_another_messages(messages);
    }

    fn set_excluded_directory(&mut self, excluded_directory: Vec<PathBuf>) {
        let messages = self.get_cd_mut().directories.set_excluded_directory(excluded_directory);
        self.get_cd_mut().text_messages.extend_with_another_messages(messages);
    }
    fn set_allowed_extensions(&mut self, allowed_extensions: String) {
        let messages = self.get_cd_mut().extensions.set_allowed_extensions(allowed_extensions);
        self.get_cd_mut().text_messages.extend_with_another_messages(messages);
    }
    fn set_excluded_extensions(&mut self, excluded_extensions: String) {
        let messages = self.get_cd_mut().extensions.set_excluded_extensions(excluded_extensions);
        self.get_cd_mut().text_messages.extend_with_another_messages(messages);
    }

    fn set_excluded_items(&mut self, excluded_items: Vec<String>) {
        let messages = self.get_cd_mut().excluded_items.set_excluded_items(excluded_items);
        self.get_cd_mut().text_messages.extend_with_another_messages(messages);
    }

    fn prepare_items(&mut self) {
        let recursive_search = self.get_cd().recursive_search;
        // Optimizes directories and removes recursive calls
        let messages = self.get_cd_mut().directories.optimize_directories(recursive_search);
        self.get_cd_mut().text_messages.extend_with_another_messages(messages);
    }

    fn delete_simple_elements_and_add_to_messages<T: ResultEntry + Sized + Send + Sync>(
        &mut self,
        stop_flag: &Arc<AtomicBool>,
        progress_sender: Option<&Sender<ProgressData>>,
        delete_item_type: DeleteItemType<T>,
    ) -> WorkContinueStatus {
        let delete_results = self.delete_elements(stop_flag, progress_sender, delete_item_type);

        if stop_flag.load(std::sync::atomic::Ordering::Relaxed) {
            WorkContinueStatus::Stop
        } else {
            delete_results.add_to_messages(self.get_text_messages_mut());
            WorkContinueStatus::Continue
        }
    }

    fn delete_advanced_elements_and_add_to_messages<T: ResultEntry + Sized + Send + Sync + Clone>(
        &mut self,
        stop_flag: &Arc<AtomicBool>,
        progress_sender: Option<&Sender<ProgressData>>,
        files_to_process: Vec<Vec<T>>,
    ) -> WorkContinueStatus {
        let delete_method = self.get_cd().delete_method;
        let sorting_by_size = matches!(
            delete_method,
            DeleteMethod::AllExceptBiggest | DeleteMethod::AllExceptSmallest | DeleteMethod::OneBiggest | DeleteMethod::OneSmallest
        );
        let sort_items = |mut input: Vec<T>| -> Vec<T> {
            input.sort_unstable_by_key(if sorting_by_size { ResultEntry::get_size } else { ResultEntry::get_modified_date });
            input
        };

        let delete_results = if delete_method == DeleteMethod::HardLink {
            let res = files_to_process
                .into_iter()
                .map(|values| {
                    let mut all_values = sort_items(values);
                    let original = all_values.remove(0);
                    (original, all_values)
                })
                .collect::<Vec<_>>();
            self.delete_elements(stop_flag, progress_sender, DeleteItemType::HardlinkingFiles(res))
        } else {
            let res = files_to_process
                .into_iter()
                .flat_map(|values| {
                    // TODO - probably a little too much cloning, so later could be this optimized
                    let len = values.len();
                    let all_values = sort_items(values);
                    match delete_method {
                        DeleteMethod::Delete => &all_values,
                        DeleteMethod::AllExceptNewest | DeleteMethod::AllExceptBiggest => &all_values[..(len - 1)],
                        DeleteMethod::AllExceptOldest | DeleteMethod::AllExceptSmallest => &all_values[1..],
                        DeleteMethod::OneOldest | DeleteMethod::OneSmallest => &all_values[..1],
                        DeleteMethod::OneNewest | DeleteMethod::OneBiggest => &all_values[(len - 1)..],
                        DeleteMethod::HardLink | DeleteMethod::None => unreachable!("HardLink and None should be handled before"),
                    }
                    .to_vec()
                })
                .collect::<Vec<_>>();
            self.delete_elements(stop_flag, progress_sender, DeleteItemType::DeletingFiles(res))
        };

        if stop_flag.load(std::sync::atomic::Ordering::Relaxed) {
            WorkContinueStatus::Stop
        } else {
            delete_results.add_to_messages(self.get_text_messages_mut());
            WorkContinueStatus::Continue
        }
    }

    fn delete_elements<T: ResultEntry + Sized + Send + Sync>(
        &self,
        stop_flag: &Arc<AtomicBool>,
        progress_sender: Option<&Sender<ProgressData>>,
        delete_item_type: DeleteItemType<T>,
    ) -> DeleteResult {
        let dry_run = self.get_cd().dry_run;
        let mut progress = ProgressData::get_empty_state(CurrentStage::DeletingFiles);
        progress.bytes_to_check = delete_item_type.calculate_size_to_delete();
        progress.entries_to_check = delete_item_type.calculate_entries_to_delete();

        let is_hardlinking = matches!(delete_item_type, DeleteItemType::HardlinkingFiles(_));

        let msg_common = format!(
            "{} items, total size: {} bytes, dry_run: {dry_run}",
            progress.entries_to_check,
            format_size(progress.bytes_to_check, BINARY)
        );
        if is_hardlinking {
            info!("Hardlinking {msg_common}");
        } else {
            info!("Deleting {msg_common}");
        }

        let delayed_sender = progress_sender.map(|e| DelayedSender::new(e.clone(), Duration::from_millis(200)));

        let bytes_processed = Arc::new(std::sync::atomic::AtomicU64::new(0));
        let files_processed = Arc::new(std::sync::atomic::AtomicUsize::new(0));

        let res = match delete_item_type {
            DeleteItemType::DeletingFiles(ref items) | DeleteItemType::DeletingFolders(ref items) => items
                .into_par_iter()
                .map(|e| {
                    if stop_flag.load(std::sync::atomic::Ordering::Relaxed) {
                        return None;
                    }

                    let mut progress_tmp = progress;
                    progress_tmp.bytes_checked = bytes_processed.fetch_add(e.get_size(), std::sync::atomic::Ordering::Relaxed);
                    progress_tmp.entries_checked = files_processed.fetch_add(1, std::sync::atomic::Ordering::Relaxed);

                    if let Some(e) = delayed_sender.as_ref() {
                        e.send(progress_tmp);
                    }

                    if dry_run {
                        return Some(vec![(e, None)]);
                    }

                    let delete_res = if matches!(delete_item_type, DeleteItemType::DeletingFiles(_)) {
                        fs::remove_file(e.get_path()).map_err(|err| format!("Failed to delete \"{}\": {err}", e.get_path().to_string_lossy()))
                    } else {
                        remove_folder_if_contains_only_empty_folders(e.get_path(), false) // TODO remove to trash should be an option
                    };

                    match delete_res {
                        Ok(()) => Some(vec![(e, None)]),
                        Err(err) => Some(vec![(e, Some(err))]),
                    }
                })
                .while_some()
                .flatten()
                .collect::<Vec<_>>(),
            DeleteItemType::HardlinkingFiles(ref items) => items
                .into_par_iter()
                .map(|(original, files)| {
                    if stop_flag.load(std::sync::atomic::Ordering::Relaxed) {
                        return None;
                    }

                    let mut progress_tmp = progress;
                    progress_tmp.bytes_checked = bytes_processed.fetch_add(files.iter().map(|e| e.get_size()).sum(), std::sync::atomic::Ordering::Relaxed);
                    progress_tmp.entries_checked = files_processed.fetch_add(1, std::sync::atomic::Ordering::Relaxed);

                    if let Some(e) = delayed_sender.as_ref() {
                        e.send(progress_tmp);
                    }

                    if dry_run {
                        return Some(files.iter().map(|e| (e, None)).collect::<Vec<_>>());
                    }

                    let res = files
                        .iter()
                        .map(|file| {
                            let err = match fs::hard_link(original.get_path(), file.get_path()) {
                                Ok(()) => None,
                                Err(err) => Some(format!(
                                    "Failed to hardlink \"{}\" to \"{}\": {err}",
                                    original.get_path().to_string_lossy(),
                                    file.get_path().to_string_lossy()
                                )),
                            };
                            (file, err)
                        })
                        .collect::<Vec<_>>();

                    Some(res)
                })
                .while_some()
                .flatten()
                .collect::<Vec<_>>(),
        };

        let mut delete_result = DeleteResult::default();

        for (file_entry, delete_err) in res {
            if let Some(err) = delete_err {
                delete_result.errors.push(err);
                delete_result.failed_to_delete_files += 1;
            } else {
                if dry_run {
                    if is_hardlinking {
                        delete_result.infos.push(format!(
                            "Would hardlink: \"{}\" to \"{}\"",
                            file_entry.get_path().to_string_lossy(),
                            file_entry.get_path().to_string_lossy()
                        ));
                    } else {
                        delete_result.infos.push(format!("Would delete: \"{}\"", file_entry.get_path().to_string_lossy()));
                    }
                }
                delete_result.deleted_files += 1;
                delete_result.gained_bytes += file_entry.get_size();
            }
        }

        if !dry_run {
            info!(
                "{} items deleted, {} bytes gained, {} failed to delete",
                delete_result.deleted_files,
                format_size(delete_result.gained_bytes, BINARY),
                delete_result.failed_to_delete_files
            );
        }

        delete_result
    }

    #[allow(clippy::print_stdout)]
    fn debug_print_common(&self) {
        println!("---------------DEBUG PRINT COMMON---------------");
        println!("Tool type: {:?}", self.get_cd().tool_type);
        println!("Directories: {:?}", self.get_cd().directories);
        println!("Extensions: {:?}", self.get_cd().extensions);
        println!("Excluded items: {:?}", self.get_cd().excluded_items);
        println!("Recursive search: {}", self.get_cd().recursive_search);
        println!("Maximal file size: {}", self.get_cd().maximal_file_size);
        println!("Minimal file size: {}", self.get_cd().minimal_file_size);
        println!("Stopped search: {}", self.get_cd().stopped_search);
        println!("Use cache: {}", self.get_cd().use_cache);
        println!("Delete outdated cache: {}", self.get_cd().delete_outdated_cache);
        println!("Save also as json: {}", self.get_cd().save_also_as_json);
        println!("Delete method: {:?}", self.get_cd().delete_method);
        println!("Use reference folders: {}", self.get_cd().use_reference_folders);
        println!("Dry run: {}", self.get_cd().dry_run);

        println!("---------------DEBUG PRINT MESSAGES---------------");
        println!("Errors size - {}", self.get_cd().text_messages.errors.len());
        println!("Warnings size - {}", self.get_cd().text_messages.warnings.len());
        println!("Messages size - {}", self.get_cd().text_messages.messages.len());
    }
}
