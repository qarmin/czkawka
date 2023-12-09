use std::collections::HashMap;
use std::fs;
use std::fs::DirEntry;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::atomic::Ordering;

use crate::common::{check_if_stop_received, prepare_thread_handler_common, send_info_and_wait_for_ending_all_threads};
use crossbeam_channel::{Receiver, Sender};
use fun_time::fun_time;
use log::debug;
use rayon::prelude::*;

use crate::common_dir_traversal::{common_get_entry_data, common_get_metadata_dir, common_read_dir, get_modified_time, CheckingMethod, ProgressData, ToolType};
use crate::common_directory::Directories;
use crate::common_items::ExcludedItems;
use crate::common_tool::{CommonData, CommonToolData, DeleteMethod};
use crate::common_traits::{DebugPrint, PrintResults};

#[derive(Clone, Debug)]
pub struct FolderEntry {
    pub path: PathBuf,
    pub(crate) parent_path: Option<String>,
    // Usable only when finding
    pub(crate) is_empty: FolderEmptiness,
    pub modified_date: u64,
}

pub struct EmptyFolder {
    common_data: CommonToolData,
    information: Info,
    empty_folder_list: HashMap<String, FolderEntry>, // Path, FolderEntry
}

/// Enum with values which show if folder is empty.
/// In function "`optimize_folders`" automatically "Maybe" is changed to "Yes", so it is not necessary to put it here
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub(crate) enum FolderEmptiness {
    No,
    Maybe,
}

#[derive(Default)]
pub struct Info {
    pub number_of_empty_folders: usize,
}

impl EmptyFolder {
    pub fn new() -> Self {
        Self {
            common_data: CommonToolData::new(ToolType::EmptyFolders),
            information: Default::default(),
            empty_folder_list: Default::default(),
        }
    }

    pub const fn get_empty_folder_list(&self) -> &HashMap<String, FolderEntry> {
        &self.empty_folder_list
    }

    pub const fn get_information(&self) -> &Info {
        &self.information
    }

    #[fun_time(message = "find_empty_folders", level = "info")]
    pub fn find_empty_folders(&mut self, stop_receiver: Option<&Receiver<()>>, progress_sender: Option<&Sender<ProgressData>>) {
        self.optimize_dirs_before_start();
        if !self.check_for_empty_folders(stop_receiver, progress_sender) {
            self.common_data.stopped_search = true;
            return;
        }
        self.optimize_folders();

        self.delete_files();
        self.debug_print();
    }

    fn optimize_folders(&mut self) {
        let mut new_directory_folders: HashMap<String, FolderEntry> = Default::default();

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

    // #[fun_time(message = "check_for_empty_folders", level = "debug")]
    fn check_for_empty_folders(&mut self, stop_receiver: Option<&Receiver<()>>, progress_sender: Option<&Sender<ProgressData>>) -> bool {
        let mut folders_to_check: Vec<PathBuf> = self.common_data.directories.included_directories.clone();

        let (progress_thread_handle, progress_thread_run, atomic_counter, _check_was_stopped) =
            prepare_thread_handler_common(progress_sender, 0, 0, 0, CheckingMethod::None, self.common_data.tool_type);

        let excluded_items = self.common_data.excluded_items.clone();
        let directories = self.common_data.directories.clone();

        let mut folder_entries: HashMap<String, FolderEntry> = HashMap::new();
        let mut non_empty_folders: Vec<String> = vec![];

        for dir in &folders_to_check {
            folder_entries.insert(
                dir.to_string_lossy().to_string(),
                FolderEntry {
                    path: dir.clone(),
                    parent_path: None,
                    is_empty: FolderEmptiness::Maybe,
                    modified_date: 0,
                },
            );
        }

        while !folders_to_check.is_empty() {
            if check_if_stop_received(stop_receiver) {
                send_info_and_wait_for_ending_all_threads(&progress_thread_run, progress_thread_handle);
                return false;
            }

            let segments: Vec<_> = folders_to_check
                .into_par_iter()
                .map(|current_folder| {
                    let mut dir_result = vec![];
                    let mut warnings = vec![];
                    let mut set_as_not_empty_folder_list = vec![];
                    let mut folder_entries_list = vec![];

                    let Some(read_dir) = common_read_dir(&current_folder, &mut warnings) else {
                        set_as_not_empty_folder_list.push(current_folder.to_string_lossy().to_string());
                        return (dir_result, warnings, set_as_not_empty_folder_list, folder_entries_list);
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
                                entry_data,
                                &directories,
                                &mut dir_result,
                                &mut warnings,
                                &excluded_items,
                                &mut set_as_not_empty_folder_list,
                                &mut folder_entries_list,
                            );
                        } else {
                            set_as_not_empty_folder_list.push(current_folder.to_string_lossy().to_string());
                        }
                    }
                    if counter > 0 {
                        // Increase counter in batch, because usually it may be slow to add multiple times atomic value
                        atomic_counter.fetch_add(counter, Ordering::Relaxed);
                    }
                    (dir_result, warnings, set_as_not_empty_folder_list, folder_entries_list)
                })
                .collect();

            let required_size = segments.iter().map(|(segment, _, _, _)| segment.len()).sum::<usize>();
            folders_to_check = Vec::with_capacity(required_size);

            // Process collected data
            for (segment, warnings, set_as_not_empty_folder_list, fe_list) in segments {
                folders_to_check.extend(segment);
                self.common_data.text_messages.warnings.extend(warnings);
                non_empty_folders.extend(set_as_not_empty_folder_list);
                for (path, entry) in fe_list {
                    folder_entries.insert(path, entry);
                }
            }
        }

        // Start to
        for current_folder in non_empty_folders.into_iter().rev() {
            Self::set_as_not_empty_folder(&mut folder_entries, &current_folder);
        }

        for (name, folder_entry) in folder_entries {
            if folder_entry.is_empty != FolderEmptiness::No {
                self.empty_folder_list.insert(name, folder_entry);
            }
        }

        debug!("Found {} empty folders.", self.empty_folder_list.len());
        send_info_and_wait_for_ending_all_threads(&progress_thread_run, progress_thread_handle);
        true
    }

    pub(crate) fn set_as_not_empty_folder(folder_entries: &mut HashMap<String, FolderEntry>, current_folder: &str) {
        let mut d = folder_entries.get_mut(current_folder).unwrap();
        if d.is_empty == FolderEmptiness::No {
            return; // Already set as non empty by one of his child
        }

        // Loop to recursively set as non empty this and all his parent folders
        loop {
            d.is_empty = FolderEmptiness::No;
            if d.parent_path.is_some() {
                let cf = d.parent_path.clone().unwrap();
                d = folder_entries.get_mut(&cf).unwrap();
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
        entry_data: &DirEntry,
        directories: &Directories,
        dir_result: &mut Vec<PathBuf>,
        warnings: &mut Vec<String>,
        excluded_items: &ExcludedItems,
        set_as_not_empty_folder_list: &mut Vec<String>,
        folder_entries_list: &mut Vec<(String, FolderEntry)>,
    ) {
        let current_folder_str = current_folder.to_string_lossy().to_string();
        let next_folder = current_folder.join(entry_data.file_name());
        if excluded_items.is_excluded(&next_folder) || directories.is_excluded(&next_folder) {
            set_as_not_empty_folder_list.push(current_folder_str);
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

        let Some(metadata) = common_get_metadata_dir(entry_data, warnings, current_folder) else {
            set_as_not_empty_folder_list.push(current_folder_str);
            return;
        };

        dir_result.push(next_folder.clone());
        folder_entries_list.push((
            next_folder.to_string_lossy().to_string(),
            FolderEntry {
                path: next_folder,
                parent_path: Some(current_folder_str),
                is_empty: FolderEmptiness::Maybe,
                modified_date: get_modified_time(&metadata, warnings, current_folder, true),
            },
        ));
    }

    #[fun_time(message = "delete_files", level = "debug")]
    fn delete_files(&mut self) {
        if self.get_delete_method() == DeleteMethod::None {
            return;
        }
        let folders_to_remove = self.empty_folder_list.keys().collect::<Vec<_>>();

        let errors: Vec<_> = folders_to_remove
            .into_par_iter()
            .filter_map(|name| {
                if let Err(e) = fs::remove_dir_all(name) {
                    Some(format!("Failed to remove folder {name:?}, reason {e}"))
                } else {
                    None
                }
            })
            .collect();
        self.get_text_messages_mut().errors.extend(errors);
    }
}

impl Default for EmptyFolder {
    fn default() -> Self {
        Self::new()
    }
}

impl DebugPrint for EmptyFolder {
    fn debug_print(&self) {
        if !cfg!(debug_assertions) {
            return;
        }

        println!("---------------DEBUG PRINT---------------");
        println!("Number of empty folders - {}", self.information.number_of_empty_folders);
        self.debug_print_common();
        println!("-----------------------------------------");
    }
}

impl PrintResults for EmptyFolder {
    fn write_results<T: Write>(&self, writer: &mut T) -> std::io::Result<()> {
        if !self.empty_folder_list.is_empty() {
            writeln!(writer, "--------------------------Empty folder list--------------------------")?;
            writeln!(writer, "Found {} empty folders", self.information.number_of_empty_folders)?;
            let mut empty_folder_list = self.empty_folder_list.keys().collect::<Vec<_>>();
            empty_folder_list.sort_unstable();
            for name in empty_folder_list {
                writeln!(writer, "{name}")?;
            }
        } else {
            write!(writer, "Not found any empty folders.")?;
        }

        Ok(())
    }

    fn save_results_to_file_as_json(&self, file_name: &str, pretty_print: bool) -> std::io::Result<()> {
        self.save_results_to_file_as_json_internal(file_name, &self.empty_folder_list.keys().collect::<Vec<_>>(), pretty_print)
    }
}

impl CommonData for EmptyFolder {
    fn get_cd(&self) -> &CommonToolData {
        &self.common_data
    }
    fn get_cd_mut(&mut self) -> &mut CommonToolData {
        &mut self.common_data
    }
}
