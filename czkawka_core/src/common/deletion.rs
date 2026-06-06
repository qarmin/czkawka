use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::time::Duration;

use crossbeam_channel::Sender;
use humansize::{BINARY, format_size};
use log::info;
use rayon::prelude::*;

use crate::common::model::WorkContinueStatus;
use crate::common::progress_data::{CurrentStage, ProgressData};
use crate::common::progress_stop_handler::check_if_stop_received;
use crate::common::tool_data::{CommonToolData, DeleteMethod};
use crate::common::traits::ResultEntry;
use crate::common::{make_hard_link, remove_folder_if_contains_only_empty_folders, remove_single_file};
use crate::helpers::delayed_sender::DelayedSender;
use crate::helpers::messages::Messages;

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

pub(crate) fn delete_simple_elements_and_add_to_messages<T: ResultEntry + Sized + Send + Sync>(
    cd: &mut CommonToolData,
    stop_flag: &Arc<AtomicBool>,
    progress_sender: Option<&Sender<ProgressData>>,
    delete_item_type: DeleteItemType<T>,
) -> WorkContinueStatus {
    let delete_results = delete_elements(cd, stop_flag, progress_sender, delete_item_type);

    if check_if_stop_received(stop_flag) {
        WorkContinueStatus::Stop
    } else {
        delete_results.add_to_messages(&mut cd.text_messages);
        WorkContinueStatus::Continue
    }
}

pub(crate) fn delete_advanced_elements_and_add_to_messages<T: ResultEntry + Sized + Send + Sync + Clone>(
    cd: &mut CommonToolData,
    stop_flag: &Arc<AtomicBool>,
    progress_sender: Option<&Sender<ProgressData>>,
    files_to_process: Vec<Vec<T>>,
) -> WorkContinueStatus {
    let delete_method = cd.delete_method;
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
                let mut all_values = values;
                let original;
                if cd.use_reference_folders {
                    // The reference should be the first item.
                    original = all_values.remove(0);
                    all_values = sort_items(all_values);
                } else {
                    all_values = sort_items(all_values);
                    original = all_values.remove(0);
                }
                (original, all_values)
            })
            .collect::<Vec<_>>();
        delete_elements(cd, stop_flag, progress_sender, DeleteItemType::HardlinkingFiles(res))
    } else {
        let res = files_to_process
            .into_iter()
            .flat_map(|values| {
                let mut all_values = values;
                if cd.use_reference_folders {
                    match all_values.len() {
                        0 | 1 => unreachable!("Using reference folders you should not get less than 2 items"),
                        2 => {
                            // The reference should be the first item, and should not be deleted.
                            all_values.remove(0);
                            return all_values;
                        }
                        _ => {
                            // The reference should be the first item, and should not be deleted.
                            all_values.remove(0);
                            all_values = sort_items(all_values);
                        }
                    }
                } else {
                    all_values = sort_items(all_values);
                }
                let len = all_values.len();
                match delete_method {
                    DeleteMethod::Delete => all_values,
                    DeleteMethod::AllExceptNewest | DeleteMethod::AllExceptBiggest => {
                        all_values.truncate(len - 1);
                        all_values
                    }
                    DeleteMethod::AllExceptOldest | DeleteMethod::AllExceptSmallest => {
                        all_values.remove(0);
                        all_values
                    }
                    DeleteMethod::OneOldest | DeleteMethod::OneSmallest => {
                        all_values.truncate(1);
                        all_values
                    }
                    DeleteMethod::OneNewest | DeleteMethod::OneBiggest => {
                        all_values.drain(..len - 1);
                        all_values
                    }
                    DeleteMethod::HardLink | DeleteMethod::None => unreachable!("HardLink and None should be handled before"),
                }
            })
            .collect::<Vec<_>>();
        delete_elements(cd, stop_flag, progress_sender, DeleteItemType::DeletingFiles(res))
    };

    if check_if_stop_received(stop_flag) {
        WorkContinueStatus::Stop
    } else {
        delete_results.add_to_messages(&mut cd.text_messages);
        WorkContinueStatus::Continue
    }
}

#[expect(clippy::needless_pass_by_value)]
pub(crate) fn delete_elements<T: ResultEntry + Sized + Send + Sync>(
    cd: &CommonToolData,
    stop_flag: &Arc<AtomicBool>,
    progress_sender: Option<&Sender<ProgressData>>,
    delete_item_type: DeleteItemType<T>,
) -> DeleteResult {
    let dry_run = cd.dry_run;
    let move_to_trash = cd.move_to_trash;
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
                    return Some(vec![(e, None, None)]);
                }

                let delete_res = if matches!(delete_item_type, DeleteItemType::DeletingFiles(_)) {
                    remove_single_file(e.get_path(), move_to_trash)
                } else {
                    remove_folder_if_contains_only_empty_folders(e.get_path(), move_to_trash)
                };

                match delete_res {
                    Ok(()) => Some(vec![(e, None, None)]),
                    Err(err) => Some(vec![(e, None, Some(err))]),
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
                    return Some(files.iter().map(|e| (e, Some(original.get_path()), None)).collect::<Vec<_>>());
                }

                let res = files
                    .iter()
                    .map(|file| {
                        let err = match make_hard_link(original.get_path(), file.get_path()) {
                            Ok(()) => None,
                            Err(err) => Some(format!(
                                "Failed to hardlink \"{}\" to \"{}\": {err}",
                                file.get_path().to_string_lossy(),
                                original.get_path().to_string_lossy(),
                            )),
                        };
                        (file, Some(original.get_path()), err)
                    })
                    .collect::<Vec<_>>();

                Some(res)
            })
            .while_some()
            .flatten()
            .collect::<Vec<_>>(),
    };

    let mut delete_result = DeleteResult::default();

    for (file_entry, maybe_original, delete_err) in res {
        if let Some(err) = delete_err {
            delete_result.errors.push(err);
            delete_result.failed_to_delete_files += 1;
        } else {
            if dry_run {
                if is_hardlinking {
                    let original = maybe_original.expect("Should be defined");
                    delete_result.infos.push(format!(
                        "Would hardlink: \"{}\" to \"{}\"",
                        file_entry.get_path().to_string_lossy(),
                        original.to_string_lossy()
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

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;

    use tempfile::TempDir;

    use super::*;
    use crate::common::model::{FileEntry, ToolType};
    use crate::common::tool_data::{CommonData, CommonToolData};

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
        fn found_any_items(&self) -> bool {
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
