use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::time::Duration;

use crossbeam_channel::Sender;
use humansize::{BINARY, format_size};
use log::info;
use rayon::prelude::*;

use crate::common::directories::Directories;
use crate::common::extensions::Extensions;
use crate::common::items::ExcludedItems;
use crate::common::model::{CheckingMethod, ToolType, WorkContinueStatus};
use crate::common::progress_data::{CurrentStage, ProgressData};
use crate::common::progress_stop_handler::check_if_stop_received;
use crate::common::traits::ResultEntry;
use crate::common::{make_hard_link, remove_folder_if_contains_only_empty_folders, remove_single_file};
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
    pub(crate) move_to_trash: bool,
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
            minimal_file_size: 0,
            stopped_search: false,
            use_cache: true,
            delete_outdated_cache: true,
            save_also_as_json: false,
            use_reference_folders: false,
            dry_run: false,
            move_to_trash: false,
        }
    }
}

pub trait CommonData {
    type Info;
    type Parameters;

    fn get_information(&self) -> Self::Info;
    fn get_params(&self) -> Self::Parameters;

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

    // Only used for internal deleting - probably only useful in CLI, but not in GUI which probably uses its own delete method selection
    fn set_move_to_trash(&mut self, move_to_trash: bool) {
        self.get_cd_mut().move_to_trash = move_to_trash;
    }
    fn get_move_to_trash(&self) -> bool {
        self.get_cd().move_to_trash
    }

    fn set_included_paths(&mut self, included_paths: Vec<PathBuf>) {
        let messages = self.get_cd_mut().directories.set_included_paths(included_paths);
        self.get_cd_mut().text_messages.extend_with_another_messages(messages);
    }

    fn set_excluded_paths(&mut self, excluded_paths: Vec<PathBuf>) {
        let messages = self.get_cd_mut().directories.set_excluded_paths(excluded_paths);
        self.get_cd_mut().text_messages.extend_with_another_messages(messages);
    }

    fn set_reference_paths(&mut self, reference_paths: Vec<PathBuf>) {
        let messages = self.get_cd_mut().directories.set_reference_paths(reference_paths);
        self.get_cd_mut().text_messages.extend_with_another_messages(messages);
    }

    fn set_allowed_extensions(&mut self, allowed_extensions: Vec<String>) {
        let messages = self.get_cd_mut().extensions.set_allowed_extensions(allowed_extensions);
        self.get_cd_mut().text_messages.extend_with_another_messages(messages);
    }

    fn set_excluded_extensions(&mut self, excluded_extensions: Vec<String>) {
        let messages = self.get_cd_mut().extensions.set_excluded_extensions(excluded_extensions);
        self.get_cd_mut().text_messages.extend_with_another_messages(messages);
    }

    fn set_excluded_items(&mut self, excluded_items: Vec<String>) {
        let messages = self.get_cd_mut().excluded_items.set_excluded_items(excluded_items);
        self.get_cd_mut().text_messages.extend_with_another_messages(messages);
    }

    fn get_extensions_mut(&mut self) -> &mut Extensions {
        &mut self.get_cd_mut().extensions
    }

    #[expect(clippy::result_unit_err)]
    fn prepare_items(&mut self, tool_extensions: Option<&[&str]>) -> Result<(), ()> {
        let recursive_search = self.get_cd().recursive_search;
        // Optimizes directories and removes recursive calls
        match self.get_cd_mut().directories.optimize_directories(recursive_search) {
            Ok(messages) => {
                self.get_cd_mut().text_messages.extend_with_another_messages(messages);
            }
            Err(messages) => {
                self.get_cd_mut().text_messages.extend_with_another_messages(messages);
                return Err(());
            }
        }

        if let Err(e) = self.get_extensions_mut().set_and_validate_extensions(tool_extensions) {
            self.get_cd_mut().text_messages.critical = Some(e);
            return Err(());
        }

        Ok(())
    }

    fn delete_simple_elements_and_add_to_messages<T: ResultEntry + Sized + Send + Sync>(
        &mut self,
        stop_flag: &Arc<AtomicBool>,
        progress_sender: Option<&Sender<ProgressData>>,
        delete_item_type: DeleteItemType<T>,
    ) -> WorkContinueStatus {
        let delete_results = self.delete_elements(stop_flag, progress_sender, delete_item_type);

        if check_if_stop_received(stop_flag) {
            WorkContinueStatus::Stop
        } else {
            delete_results.add_to_messages(self.get_text_messages_mut());
            WorkContinueStatus::Continue
        }
    }

    #[expect(clippy::indexing_slicing)] // Safe, because input is always checked to have at least 1 element
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

        if check_if_stop_received(stop_flag) {
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
        let move_to_trash = self.get_cd().move_to_trash;
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
                    if check_if_stop_received(stop_flag) {
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
                        remove_single_file(e.get_path(), move_to_trash)
                    } else {
                        remove_folder_if_contains_only_empty_folders(e.get_path(), move_to_trash)
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
                    if check_if_stop_received(stop_flag) {
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
                            let err = match make_hard_link(original.get_path(), file.get_path()) {
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
            let action = if is_hardlinking { "hardlink" } else { "delete" };
            let action2 = if is_hardlinking { "hardlinked" } else { "deleted" };
            info!(
                "{} items {action2}, {} gained, {} failed to {action}",
                delete_result.deleted_files,
                format_size(delete_result.gained_bytes, BINARY),
                delete_result.failed_to_delete_files
            );
        }

        delete_result
    }

    #[expect(clippy::print_stdout)]
    fn debug_print_common(&self) {
        println!("---------------DEBUG PRINT COMMON---------------");
        println!("Included directories(optimized) - {:?}", self.get_cd().directories.included_directories);
        println!("Included files(optimized) - {:?}", self.get_cd().directories.included_files);
        println!("Excluded directories(optimized) - {:?}", self.get_cd().directories.excluded_directories);
        println!("Excluded files(optimized) - {:?}", self.get_cd().directories.excluded_files);
        println!("Reference directories(optimized) - {:?}", self.get_cd().directories.reference_directories);
        println!("Reference files(optimized) - {:?}", self.get_cd().directories.reference_files);
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

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::TempDir;

    use super::*;
    use crate::common::model::FileEntry;

    // Mock implementation for testing
    struct MockTool {
        common_data: CommonToolData,
    }

    impl CommonData for MockTool {
        type Info = ();
        type Parameters = ();

        fn get_information(&self) -> Self::Info {}
        fn get_params(&self) -> Self::Parameters {}
        fn get_cd(&self) -> &CommonToolData {
            &self.common_data
        }
        fn get_cd_mut(&mut self) -> &mut CommonToolData {
            &mut self.common_data
        }
        fn found_any_broken_files(&self) -> bool {
            false
        }
    }

    impl MockTool {
        fn new() -> Self {
            Self {
                common_data: CommonToolData::new(ToolType::Duplicate),
            }
        }
    }

    #[test]
    fn test_delete_result_add_to_messages() {
        let delete_result = DeleteResult {
            deleted_files: 5,
            gained_bytes: 1024,
            failed_to_delete_files: 2,
            errors: vec!["Error 1".to_string(), "Error 2".to_string()],
            infos: vec!["Info 1".to_string()],
        };

        let mut messages = Messages::new();
        delete_result.add_to_messages(&mut messages);

        assert_eq!(messages.errors.len(), 2);
        assert_eq!(messages.messages.len(), 1);
        assert!(messages.errors.contains(&"Error 1".to_string()));
        assert!(messages.messages.contains(&"Info 1".to_string()));
    }

    #[test]
    fn test_delete_item_type_calculate_size_and_entries() {
        let files = vec![
            FileEntry {
                path: PathBuf::from("/a"),
                size: 100,
                modified_date: 1,
            },
            FileEntry {
                path: PathBuf::from("/b"),
                size: 200,
                modified_date: 2,
            },
            FileEntry {
                path: PathBuf::from("/c"),
                size: 300,
                modified_date: 3,
            },
        ];

        let delete_files = DeleteItemType::DeletingFiles(files.clone());
        assert_eq!(delete_files.calculate_size_to_delete(), 600);
        assert_eq!(delete_files.calculate_entries_to_delete(), 3);

        let delete_folders = DeleteItemType::DeletingFolders(files.clone());
        assert_eq!(delete_folders.calculate_size_to_delete(), 600);
        assert_eq!(delete_folders.calculate_entries_to_delete(), 3);

        let hardlink_files = DeleteItemType::HardlinkingFiles(vec![
            (files[0].clone(), vec![files[1].clone()]),
            (files[2].clone(), vec![files[0].clone(), files[1].clone()]),
        ]);
        assert_eq!(hardlink_files.calculate_size_to_delete(), 400);
        assert_eq!(hardlink_files.calculate_entries_to_delete(), 3);
    }

    #[test]
    fn test_common_tool_data_new() {
        let tool_data = CommonToolData::new(ToolType::Duplicate);
        assert_eq!(tool_data.tool_type, ToolType::Duplicate);
        assert_eq!(tool_data.delete_method, DeleteMethod::None);
        assert_eq!(tool_data.maximal_file_size, u64::MAX);
        assert_eq!(tool_data.minimal_file_size, 0);
        assert!(tool_data.recursive_search);
        assert!(!tool_data.stopped_search);
        assert!(tool_data.use_cache);
        assert!(tool_data.delete_outdated_cache);
        assert!(!tool_data.save_also_as_json);
        assert!(!tool_data.use_reference_folders);
        assert!(!tool_data.dry_run);
    }

    #[test]
    fn test_delete_elements_dry_run() {
        let temp_dir = TempDir::new().unwrap();
        let file1 = temp_dir.path().join("file1.txt");
        let file2 = temp_dir.path().join("file2.txt");
        fs::write(&file1, "test content 1").unwrap();
        fs::write(&file2, "test content 2").unwrap();

        let files = vec![
            FileEntry {
                path: file1.clone(),
                size: 14,
                modified_date: 1,
            },
            FileEntry {
                path: file2.clone(),
                size: 14,
                modified_date: 2,
            },
        ];

        let mut tool = MockTool::new();
        tool.common_data.dry_run = true;

        let stop_flag = Arc::new(AtomicBool::new(false));
        let delete_result = tool.delete_elements(&stop_flag, None, DeleteItemType::DeletingFiles(files));

        assert_eq!(delete_result.deleted_files, 2, "Should mark 2 files as deleted");
        assert_eq!(delete_result.failed_to_delete_files, 0, "Should have no failed deletions");
        assert_eq!(delete_result.gained_bytes, 28, "Should calculate gained bytes");
        assert_eq!(delete_result.infos.len(), 2, "Should have 2 info messages in dry run");
        assert!(file1.exists(), "File should still exist in dry run");
        assert!(file2.exists(), "File should still exist in dry run");
    }

    #[test]
    fn test_delete_elements_actual_deletion() {
        let temp_dir = TempDir::new().unwrap();
        let file1 = temp_dir.path().join("file1.txt");
        let file2 = temp_dir.path().join("file2.txt");
        fs::write(&file1, "test content 1").unwrap();
        fs::write(&file2, "test content 2").unwrap();

        let files = vec![
            FileEntry {
                path: file1.clone(),
                size: 14,
                modified_date: 1,
            },
            FileEntry {
                path: file2.clone(),
                size: 14,
                modified_date: 2,
            },
        ];

        let tool = MockTool::new();
        let stop_flag = Arc::new(AtomicBool::new(false));
        let delete_result = tool.delete_elements(&stop_flag, None, DeleteItemType::DeletingFiles(files));

        assert_eq!(delete_result.deleted_files, 2, "Should delete 2 files");
        assert_eq!(delete_result.failed_to_delete_files, 0, "Should have no failed deletions");
        assert_eq!(delete_result.gained_bytes, 28, "Should gain 28 bytes");
        assert!(!file1.exists(), "File 1 should be deleted");
        assert!(!file2.exists(), "File 2 should be deleted");
    }

    #[test]
    fn test_delete_elements_with_stop_flag() {
        let temp_dir = TempDir::new().unwrap();
        let file1 = temp_dir.path().join("file1.txt");
        fs::write(&file1, "test content").unwrap();

        let files = vec![FileEntry {
            path: file1.clone(),
            size: 12,
            modified_date: 1,
        }];

        let tool = MockTool::new();
        let stop_flag = Arc::new(AtomicBool::new(true)); // Stop flag set to true
        let delete_result = tool.delete_elements(&stop_flag, None, DeleteItemType::DeletingFiles(files));

        assert_eq!(delete_result.deleted_files, 0, "Should not delete any files when stopped");
        assert!(file1.exists(), "File should still exist");
    }

    #[test]
    fn test_delete_elements_nonexistent_file() {
        let temp_dir = TempDir::new().unwrap();
        let nonexistent_file = temp_dir.path().join("nonexistent.txt");

        let files = vec![FileEntry {
            path: nonexistent_file,
            size: 100,
            modified_date: 1,
        }];

        let tool = MockTool::new();
        let stop_flag = Arc::new(AtomicBool::new(false));
        let delete_result = tool.delete_elements(&stop_flag, None, DeleteItemType::DeletingFiles(files));

        assert_eq!(delete_result.deleted_files, 0, "Should not delete nonexistent file");
        assert_eq!(delete_result.failed_to_delete_files, 1, "Should report 1 failed deletion");
        assert_eq!(delete_result.errors.len(), 1, "Should have 1 error message");
    }

    #[test]
    fn test_delete_simple_elements_and_add_to_messages() {
        let temp_dir = TempDir::new().unwrap();
        let file1 = temp_dir.path().join("file1.txt");
        let file2 = temp_dir.path().join("file2.txt");
        fs::write(&file1, "content1").unwrap();
        fs::write(&file2, "content2").unwrap();

        let files = vec![
            FileEntry {
                path: file1.clone(),
                size: 8,
                modified_date: 1,
            },
            FileEntry {
                path: file2.clone(),
                size: 8,
                modified_date: 2,
            },
        ];

        let mut tool = MockTool::new();
        let stop_flag = Arc::new(AtomicBool::new(false));
        let status = tool.delete_simple_elements_and_add_to_messages(&stop_flag, None, DeleteItemType::DeletingFiles(files));

        assert_eq!(status, WorkContinueStatus::Continue, "Should continue");
        assert!(!file1.exists(), "File 1 should be deleted");
        assert!(!file2.exists(), "File 2 should be deleted");
        assert_eq!(tool.common_data.text_messages.errors.len(), 0, "Should have no errors");
    }

    #[test]
    fn test_delete_simple_elements_with_stop_flag() {
        let temp_dir = TempDir::new().unwrap();
        let file1 = temp_dir.path().join("file1.txt");
        fs::write(&file1, "content").unwrap();

        let files = vec![FileEntry {
            path: file1.clone(),
            size: 7,
            modified_date: 1,
        }];

        let mut tool = MockTool::new();
        let stop_flag = Arc::new(AtomicBool::new(true));
        let status = tool.delete_simple_elements_and_add_to_messages(&stop_flag, None, DeleteItemType::DeletingFiles(files));

        assert_eq!(status, WorkContinueStatus::Stop, "Should stop");
        assert!(file1.exists(), "File should still exist");
    }

    #[test]
    fn test_delete_advanced_elements_all_except_newest() {
        let temp_dir = TempDir::new().unwrap();
        let file1 = temp_dir.path().join("file1.txt");
        let file2 = temp_dir.path().join("file2.txt");
        let file3 = temp_dir.path().join("file3.txt");
        fs::write(&file1, "a").unwrap();
        fs::write(&file2, "b").unwrap();
        fs::write(&file3, "c").unwrap();

        let files_group = vec![vec![
            FileEntry {
                path: file1.clone(),
                size: 1,
                modified_date: 1,
            },
            FileEntry {
                path: file2.clone(),
                size: 1,
                modified_date: 2,
            },
            FileEntry {
                path: file3.clone(),
                size: 1,
                modified_date: 3,
            },
        ]];

        let mut tool = MockTool::new();
        tool.common_data.delete_method = DeleteMethod::AllExceptNewest;

        let stop_flag = Arc::new(AtomicBool::new(false));
        let status = tool.delete_advanced_elements_and_add_to_messages(&stop_flag, None, files_group);

        assert_eq!(status, WorkContinueStatus::Continue, "Should continue");
        assert!(!file1.exists(), "Oldest file should be deleted");
        assert!(!file2.exists(), "Middle file should be deleted");
        assert!(file3.exists(), "Newest file should be kept");
    }

    #[test]
    fn test_delete_advanced_elements_all_except_oldest() {
        let temp_dir = TempDir::new().unwrap();
        let file1 = temp_dir.path().join("file1.txt");
        let file2 = temp_dir.path().join("file2.txt");
        let file3 = temp_dir.path().join("file3.txt");
        fs::write(&file1, "a").unwrap();
        fs::write(&file2, "b").unwrap();
        fs::write(&file3, "c").unwrap();

        let files_group = vec![vec![
            FileEntry {
                path: file1.clone(),
                size: 1,
                modified_date: 1,
            },
            FileEntry {
                path: file2.clone(),
                size: 1,
                modified_date: 2,
            },
            FileEntry {
                path: file3.clone(),
                size: 1,
                modified_date: 3,
            },
        ]];

        let mut tool = MockTool::new();
        tool.common_data.delete_method = DeleteMethod::AllExceptOldest;

        let stop_flag = Arc::new(AtomicBool::new(false));
        let status = tool.delete_advanced_elements_and_add_to_messages(&stop_flag, None, files_group);

        assert_eq!(status, WorkContinueStatus::Continue, "Should continue");
        assert!(file1.exists(), "Oldest file should be kept");
        assert!(!file2.exists(), "Middle file should be deleted");
        assert!(!file3.exists(), "Newest file should be deleted");
    }

    #[test]
    fn test_delete_advanced_elements_one_oldest() {
        let temp_dir = TempDir::new().unwrap();
        let file1 = temp_dir.path().join("file1.txt");
        let file2 = temp_dir.path().join("file2.txt");
        let file3 = temp_dir.path().join("file3.txt");
        fs::write(&file1, "a").unwrap();
        fs::write(&file2, "b").unwrap();
        fs::write(&file3, "c").unwrap();

        let files_group = vec![vec![
            FileEntry {
                path: file1.clone(),
                size: 1,
                modified_date: 1,
            },
            FileEntry {
                path: file2.clone(),
                size: 1,
                modified_date: 2,
            },
            FileEntry {
                path: file3.clone(),
                size: 1,
                modified_date: 3,
            },
        ]];

        let mut tool = MockTool::new();
        tool.common_data.delete_method = DeleteMethod::OneOldest;

        let stop_flag = Arc::new(AtomicBool::new(false));
        let status = tool.delete_advanced_elements_and_add_to_messages(&stop_flag, None, files_group);

        assert_eq!(status, WorkContinueStatus::Continue, "Should continue");
        assert!(!file1.exists(), "Oldest file should be deleted");
        assert!(file2.exists(), "Middle file should be kept");
        assert!(file3.exists(), "Newest file should be kept");
    }

    #[test]
    fn test_delete_advanced_elements_one_newest() {
        let temp_dir = TempDir::new().unwrap();
        let file1 = temp_dir.path().join("file1.txt");
        let file2 = temp_dir.path().join("file2.txt");
        let file3 = temp_dir.path().join("file3.txt");
        fs::write(&file1, "a").unwrap();
        fs::write(&file2, "b").unwrap();
        fs::write(&file3, "c").unwrap();

        let files_group = vec![vec![
            FileEntry {
                path: file1.clone(),
                size: 1,
                modified_date: 1,
            },
            FileEntry {
                path: file2.clone(),
                size: 1,
                modified_date: 2,
            },
            FileEntry {
                path: file3.clone(),
                size: 1,
                modified_date: 3,
            },
        ]];

        let mut tool = MockTool::new();
        tool.common_data.delete_method = DeleteMethod::OneNewest;

        let stop_flag = Arc::new(AtomicBool::new(false));
        let status = tool.delete_advanced_elements_and_add_to_messages(&stop_flag, None, files_group);

        assert_eq!(status, WorkContinueStatus::Continue, "Should continue");
        assert!(file1.exists(), "Oldest file should be kept");
        assert!(file2.exists(), "Middle file should be kept");
        assert!(!file3.exists(), "Newest file should be deleted");
    }

    #[test]
    fn test_delete_advanced_elements_all_except_biggest() {
        let temp_dir = TempDir::new().unwrap();
        let file1 = temp_dir.path().join("file1.txt");
        let file2 = temp_dir.path().join("file2.txt");
        let file3 = temp_dir.path().join("file3.txt");
        fs::write(&file1, "a").unwrap();
        fs::write(&file2, "bb").unwrap();
        fs::write(&file3, "ccc").unwrap();

        let files_group = vec![vec![
            FileEntry {
                path: file1.clone(),
                size: 1,
                modified_date: 1,
            },
            FileEntry {
                path: file2.clone(),
                size: 2,
                modified_date: 1,
            },
            FileEntry {
                path: file3.clone(),
                size: 3,
                modified_date: 1,
            },
        ]];

        let mut tool = MockTool::new();
        tool.common_data.delete_method = DeleteMethod::AllExceptBiggest;

        let stop_flag = Arc::new(AtomicBool::new(false));
        let status = tool.delete_advanced_elements_and_add_to_messages(&stop_flag, None, files_group);

        assert_eq!(status, WorkContinueStatus::Continue, "Should continue");
        assert!(!file1.exists(), "Smallest file should be deleted");
        assert!(!file2.exists(), "Middle file should be deleted");
        assert!(file3.exists(), "Biggest file should be kept");
    }

    #[test]
    fn test_delete_advanced_elements_all_except_smallest() {
        let temp_dir = TempDir::new().unwrap();
        let file1 = temp_dir.path().join("file1.txt");
        let file2 = temp_dir.path().join("file2.txt");
        let file3 = temp_dir.path().join("file3.txt");
        fs::write(&file1, "a").unwrap();
        fs::write(&file2, "bb").unwrap();
        fs::write(&file3, "ccc").unwrap();

        let files_group = vec![vec![
            FileEntry {
                path: file1.clone(),
                size: 1,
                modified_date: 1,
            },
            FileEntry {
                path: file2.clone(),
                size: 2,
                modified_date: 1,
            },
            FileEntry {
                path: file3.clone(),
                size: 3,
                modified_date: 1,
            },
        ]];

        let mut tool = MockTool::new();
        tool.common_data.delete_method = DeleteMethod::AllExceptSmallest;

        let stop_flag = Arc::new(AtomicBool::new(false));
        let status = tool.delete_advanced_elements_and_add_to_messages(&stop_flag, None, files_group);

        assert_eq!(status, WorkContinueStatus::Continue, "Should continue");
        assert!(file1.exists(), "Smallest file should be kept");
        assert!(!file2.exists(), "Middle file should be deleted");
        assert!(!file3.exists(), "Biggest file should be deleted");
    }

    #[test]
    fn test_delete_advanced_elements_delete_all() {
        let temp_dir = TempDir::new().unwrap();
        let file1 = temp_dir.path().join("file1.txt");
        let file2 = temp_dir.path().join("file2.txt");
        fs::write(&file1, "a").unwrap();
        fs::write(&file2, "b").unwrap();

        let files_group = vec![vec![
            FileEntry {
                path: file1.clone(),
                size: 1,
                modified_date: 1,
            },
            FileEntry {
                path: file2.clone(),
                size: 1,
                modified_date: 2,
            },
        ]];

        let mut tool = MockTool::new();
        tool.common_data.delete_method = DeleteMethod::Delete;

        let stop_flag = Arc::new(AtomicBool::new(false));
        let status = tool.delete_advanced_elements_and_add_to_messages(&stop_flag, None, files_group);

        assert_eq!(status, WorkContinueStatus::Continue, "Should continue");
        assert!(!file1.exists(), "All files should be deleted");
        assert!(!file2.exists(), "All files should be deleted");
    }

    #[test]
    fn test_delete_advanced_elements_multiple_groups() {
        let temp_dir = TempDir::new().unwrap();
        let file1 = temp_dir.path().join("file1.txt");
        let file2 = temp_dir.path().join("file2.txt");
        let file3 = temp_dir.path().join("file3.txt");
        let file4 = temp_dir.path().join("file4.txt");
        fs::write(&file1, "a").unwrap();
        fs::write(&file2, "b").unwrap();
        fs::write(&file3, "c").unwrap();
        fs::write(&file4, "d").unwrap();

        let files_group = vec![
            vec![
                FileEntry {
                    path: file1.clone(),
                    size: 1,
                    modified_date: 1,
                },
                FileEntry {
                    path: file2.clone(),
                    size: 1,
                    modified_date: 2,
                },
            ],
            vec![
                FileEntry {
                    path: file3.clone(),
                    size: 1,
                    modified_date: 1,
                },
                FileEntry {
                    path: file4.clone(),
                    size: 1,
                    modified_date: 2,
                },
            ],
        ];

        let mut tool = MockTool::new();
        tool.common_data.delete_method = DeleteMethod::AllExceptNewest;

        let stop_flag = Arc::new(AtomicBool::new(false));
        let status = tool.delete_advanced_elements_and_add_to_messages(&stop_flag, None, files_group);

        assert_eq!(status, WorkContinueStatus::Continue, "Should continue");
        assert!(!file1.exists(), "Oldest from group 1 should be deleted");
        assert!(file2.exists(), "Newest from group 1 should be kept");
        assert!(!file3.exists(), "Oldest from group 2 should be deleted");
        assert!(file4.exists(), "Newest from group 2 should be kept");
    }

    #[test]
    fn test_delete_advanced_elements_with_stop_flag() {
        let temp_dir = TempDir::new().unwrap();
        let file1 = temp_dir.path().join("file1.txt");
        let file2 = temp_dir.path().join("file2.txt");
        fs::write(&file1, "a").unwrap();
        fs::write(&file2, "b").unwrap();

        let files_group = vec![vec![
            FileEntry {
                path: file1,
                size: 1,
                modified_date: 1,
            },
            FileEntry {
                path: file2,
                size: 1,
                modified_date: 2,
            },
        ]];

        let mut tool = MockTool::new();
        tool.common_data.delete_method = DeleteMethod::AllExceptNewest;

        let stop_flag = Arc::new(AtomicBool::new(true));
        let status = tool.delete_advanced_elements_and_add_to_messages(&stop_flag, None, files_group);

        assert_eq!(status, WorkContinueStatus::Stop, "Should stop");
    }
}
