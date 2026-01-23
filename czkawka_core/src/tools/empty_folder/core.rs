use std::fs::DirEntry;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use crossbeam_channel::Sender;
use fun_time::fun_time;
use indexmap::IndexMap;
use log::debug;
use rayon::prelude::*;

use crate::common::dir_traversal::{common_get_entry_data, common_get_metadata_dir, common_read_dir, get_modified_time};
use crate::common::directories::Directories;
use crate::common::items::ExcludedItems;
use crate::common::model::{ToolType, WorkContinueStatus};
use crate::common::progress_data::{CurrentStage, ProgressData};
use crate::common::progress_stop_handler::{check_if_stop_received, prepare_thread_handler_common};
use crate::common::tool_data::{CommonData, CommonToolData};
use crate::tools::empty_folder::{EmptyFolder, FolderEmptiness, FolderEntry, Info};

impl EmptyFolder {
    pub fn new() -> Self {
        Self {
            common_data: CommonToolData::new(ToolType::EmptyFolders),
            information: Default::default(),
            empty_folder_list: Default::default(),
        }
    }

    pub const fn get_empty_folder_list(&self) -> &IndexMap<String, FolderEntry> {
        &self.empty_folder_list
    }

    pub const fn get_information(&self) -> &Info {
        &self.information
    }

    pub(crate) fn optimize_folders(&mut self) {
        let mut new_directory_folders: IndexMap<String, FolderEntry> = Default::default();

        for (name, folder_entry) in &self.empty_folder_list {
            match &folder_entry.parent_path {
                Some(t) => {
                    if !self.empty_folder_list.contains_key(t) {
                        new_directory_folders.insert(name.clone(), folder_entry.clone());
                    }
                }
                None => {
                    new_directory_folders.insert(name.clone(), folder_entry.clone());
                }
            }
        }
        self.empty_folder_list = new_directory_folders;
        self.information.number_of_empty_folders = self.empty_folder_list.len();
    }

    #[fun_time(message = "check_for_empty_folders", level = "debug")]
    pub(crate) fn check_for_empty_folders(&mut self, stop_flag: &Arc<AtomicBool>, progress_sender: Option<&Sender<ProgressData>>) -> WorkContinueStatus {
        let mut folders_to_check: Vec<PathBuf> = self.common_data.directories.included_directories.clone();

        let progress_handler = prepare_thread_handler_common(progress_sender, CurrentStage::CollectingFiles, 0, self.get_test_type(), 0);

        let excluded_items = self.common_data.excluded_items.clone();
        let directories = self.common_data.directories.clone();

        let mut non_empty_folders: Vec<String> = Vec::new();

        let mut start_folder_entries = Vec::with_capacity(folders_to_check.len());
        let mut new_folder_entries_list = Vec::new();
        for dir in &folders_to_check {
            start_folder_entries.push(FolderEntry {
                path: dir.clone(),
                parent_path: None,
                is_empty: FolderEmptiness::Maybe,
                modified_date: 0,
            });
        }

        while !folders_to_check.is_empty() {
            if check_if_stop_received(stop_flag) {
                progress_handler.join_thread();
                return WorkContinueStatus::Stop;
            }

            let segments: Vec<_> = folders_to_check
                .into_par_iter()
                .map(|current_folder| {
                    let mut dir_result = Vec::new();
                    let mut warnings = Vec::new();
                    let mut non_empty_folder = None;
                    let mut folder_entries_list = Vec::new();

                    let current_folder_as_string = current_folder.to_string_lossy().to_string();

                    let Some(read_dir) = common_read_dir(&current_folder, &mut warnings) else {
                        return (dir_result, warnings, Some(current_folder_as_string), folder_entries_list);
                    };

                    let mut counter = 0;
                    // Check every sub folder/file/link etc.
                    for entry in read_dir {
                        let Some(entry_data) = common_get_entry_data(&entry, &mut warnings, &current_folder) else {
                            continue;
                        };
                        let Ok(file_type) = entry_data.file_type() else { continue };

                        if file_type.is_dir() {
                            counter += 1;
                            Self::process_dir_in_dir_mode(
                                &current_folder,
                                &current_folder_as_string,
                                entry_data,
                                &directories,
                                &mut dir_result,
                                &mut warnings,
                                &excluded_items,
                                &mut non_empty_folder,
                                &mut folder_entries_list,
                            );
                        } else if non_empty_folder.is_none() {
                            non_empty_folder = Some(current_folder_as_string.clone());
                        }
                    }
                    if counter > 0 {
                        // Increase counter in batch, because usually it may be slow to add multiple times atomic value
                        progress_handler.increase_items(counter);
                    }

                    (dir_result, warnings, non_empty_folder, folder_entries_list)
                })
                .collect();

            let required_size = segments.iter().map(|(segment, _, _, _)| segment.len()).sum::<usize>();
            folders_to_check = Vec::with_capacity(required_size);

            // Process collected data
            for (segment, warnings, non_empty_folder, fe_list) in segments {
                folders_to_check.extend(segment);
                if !warnings.is_empty() {
                    self.common_data.text_messages.warnings.extend(warnings);
                }
                if let Some(non_empty_folder) = non_empty_folder {
                    non_empty_folders.push(non_empty_folder);
                }
                new_folder_entries_list.push(fe_list);
            }
        }

        let mut folder_entries: IndexMap<String, FolderEntry> = IndexMap::with_capacity(start_folder_entries.len() + new_folder_entries_list.iter().map(Vec::len).sum::<usize>());
        for fe in start_folder_entries {
            folder_entries.insert(fe.path.to_string_lossy().to_string(), fe);
        }
        for fe_list in new_folder_entries_list {
            for fe in fe_list {
                folder_entries.insert(fe.path.to_string_lossy().to_string(), fe);
            }
        }

        for current_folder in non_empty_folders.into_iter().rev() {
            Self::set_as_not_empty_folder(&mut folder_entries, &current_folder);
        }

        for (name, folder_entry) in folder_entries {
            if folder_entry.is_empty != FolderEmptiness::No {
                self.empty_folder_list.insert(name, folder_entry);
            }
        }

        debug!("Found {} empty folders.", self.empty_folder_list.len());
        progress_handler.join_thread();
        WorkContinueStatus::Continue
    }

    pub(crate) fn set_as_not_empty_folder(folder_entries: &mut IndexMap<String, FolderEntry>, current_folder: &str) {
        let mut d = folder_entries
            .get_mut(current_folder)
            .unwrap_or_else(|| panic!("Folder {current_folder} not found in folder_entries"));
        if d.is_empty == FolderEmptiness::No {
            return; // Already set as non empty by one of its child
        }

        // Loop to recursively set as non empty this and all its parent folders
        loop {
            d.is_empty = FolderEmptiness::No;

            if let Some(parent_path) = &d.parent_path {
                let cf = parent_path.clone();
                d = folder_entries.get_mut(&cf).unwrap_or_else(|| panic!("Folder {cf} not found in folder_entries"));
                if d.is_empty == FolderEmptiness::No {
                    break; // Already set as non empty, so one of child already set it to non empty
                }
            } else {
                break;
            }
        }
    }

    fn process_dir_in_dir_mode(
        current_folder: &Path,
        current_folder_as_str: &str,
        entry_data: &DirEntry,
        directories: &Directories,
        dir_result: &mut Vec<PathBuf>,
        warnings: &mut Vec<String>,
        excluded_items: &ExcludedItems,
        non_empty_folder: &mut Option<String>,
        folder_entries_list: &mut Vec<FolderEntry>,
    ) {
        let next_folder = entry_data.path();
        if excluded_items.is_excluded(&next_folder) || directories.is_excluded(&next_folder) {
            if non_empty_folder.is_none() {
                *non_empty_folder = Some(current_folder_as_str.to_string());
            }
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

        let Some(metadata) = common_get_metadata_dir(entry_data, warnings, &next_folder) else {
            if non_empty_folder.is_none() {
                *non_empty_folder = Some(current_folder_as_str.to_string());
            }
            return;
        };

        dir_result.push(next_folder.clone());
        folder_entries_list.push(FolderEntry {
            path: next_folder,
            parent_path: Some(current_folder_as_str.to_string()),
            is_empty: FolderEmptiness::Maybe,
            modified_date: get_modified_time(&metadata, warnings, current_folder, true),
        });
    }
}
