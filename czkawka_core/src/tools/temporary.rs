use std::fs::DirEntry;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use crossbeam_channel::Sender;
use fun_time::fun_time;
use rayon::prelude::*;
use serde::Serialize;

use crate::common::dir_traversal::{common_read_dir, get_modified_time};
use crate::common::directories::Directories;
use crate::common::items::ExcludedItems;
use crate::common::model::{ToolType, WorkContinueStatus};
use crate::common::progress_data::{CurrentStage, ProgressData};
use crate::common::progress_stop_handler::{check_if_stop_received, prepare_thread_handler_common};
use crate::common::tool_data::{CommonData, CommonToolData, DeleteItemType, DeleteMethod};
use crate::common::traits::*;

const TEMP_EXTENSIONS: &[&str] = &[
    "#",
    "thumbs.db",
    ".bak",
    "~",
    ".tmp",
    ".temp",
    ".ds_store",
    ".crdownload",
    ".part",
    ".cache",
    ".dmp",
    ".download",
    ".partial",
];

#[derive(Clone, Serialize, Debug)]
pub struct TemporaryFileEntry {
    pub path: PathBuf,
    pub modified_date: u64,
    pub size: u64,
}

impl ResultEntry for TemporaryFileEntry {
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

#[derive(Default, Clone)]
pub struct Info {
    pub number_of_temporary_files: usize,
}

pub struct Temporary {
    common_data: CommonToolData,
    information: Info,
    temporary_files: Vec<TemporaryFileEntry>,
}

impl Temporary {
    pub fn new() -> Self {
        Self {
            common_data: CommonToolData::new(ToolType::TemporaryFiles),
            information: Info::default(),
            temporary_files: vec![],
        }
    }

    #[fun_time(message = "find_temporary_files", level = "info")]
    pub fn find_temporary_files(&mut self, stop_flag: &Arc<AtomicBool>, progress_sender: Option<&Sender<ProgressData>>) {
        self.prepare_items();
        if self.check_files(stop_flag, progress_sender) == WorkContinueStatus::Stop {
            self.common_data.stopped_search = true;
            return;
        }
        if self.delete_files(stop_flag, progress_sender) == WorkContinueStatus::Stop {
            self.common_data.stopped_search = true;
            return;
        };
        self.debug_print();
    }

    #[fun_time(message = "check_files", level = "debug")]
    fn check_files(&mut self, stop_flag: &Arc<AtomicBool>, progress_sender: Option<&Sender<ProgressData>>) -> WorkContinueStatus {
        let mut folders_to_check: Vec<PathBuf> = self.common_data.directories.included_directories.clone();

        let progress_handler = prepare_thread_handler_common(progress_sender, CurrentStage::CollectingFiles, 0, self.get_test_type(), 0);

        while !folders_to_check.is_empty() {
            if check_if_stop_received(stop_flag) {
                progress_handler.join_thread();
                return WorkContinueStatus::Stop;
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

                    // Check every sub folder/file/link etc.
                    for entry in read_dir {
                        let Ok(entry_data) = entry else {
                            continue;
                        };
                        let Ok(file_type) = entry_data.file_type() else {
                            continue;
                        };

                        if file_type.is_dir() {
                            check_folder_children(
                                &mut dir_result,
                                &mut warnings,
                                &entry_data,
                                self.common_data.recursive_search,
                                &self.common_data.directories,
                                &self.common_data.excluded_items,
                            );
                        } else if file_type.is_file() {
                            if let Some(file_entry) = self.get_file_entry(progress_handler.items_counter(), &entry_data, &mut warnings) {
                                fe_result.push(file_entry);
                            }
                        }
                    }
                    (dir_result, warnings, fe_result)
                })
                .collect();

            let required_size = segments.iter().map(|(segment, _, _)| segment.len()).sum::<usize>();
            folders_to_check = Vec::with_capacity(required_size);

            // Process collected data
            for (segment, warnings, fe_result) in segments {
                folders_to_check.extend(segment);
                self.common_data.text_messages.warnings.extend(warnings);
                for fe in fe_result {
                    self.temporary_files.push(fe);
                }
            }
        }

        progress_handler.join_thread();
        self.information.number_of_temporary_files = self.temporary_files.len();

        WorkContinueStatus::Continue
    }
    pub(crate) fn get_file_entry(&self, items_counter: &Arc<AtomicUsize>, entry_data: &DirEntry, warnings: &mut Vec<String>) -> Option<TemporaryFileEntry> {
        items_counter.fetch_add(1, Ordering::Relaxed);

        let current_file_name = entry_data.path();
        if self.common_data.excluded_items.is_excluded(&current_file_name) {
            return None;
        }

        let file_name = entry_data.file_name();
        let file_name_ascii_lowercase = file_name.to_ascii_lowercase();
        let file_name_lowercase = file_name_ascii_lowercase.to_string_lossy();
        if !TEMP_EXTENSIONS.iter().any(|f| file_name_lowercase.ends_with(f)) {
            return None;
        }

        let Ok(metadata) = entry_data.metadata() else {
            return None;
        };

        // Creating new file entry
        Some(TemporaryFileEntry {
            modified_date: get_modified_time(&metadata, warnings, &current_file_name, false),
            size: metadata.len(),
            path: current_file_name,
        })
    }
}

impl DeletingItems for Temporary {
    #[fun_time(message = "delete_files", level = "debug")]
    fn delete_files(&mut self, stop_flag: &Arc<AtomicBool>, progress_sender: Option<&Sender<ProgressData>>) -> WorkContinueStatus {
        if self.get_cd().delete_method == DeleteMethod::None {
            return WorkContinueStatus::Continue;
        }
        let files_to_delete = self.temporary_files.clone();
        self.delete_simple_elements_and_add_to_messages(stop_flag, progress_sender, DeleteItemType::DeletingFiles(files_to_delete))
    }
}

impl PrintResults for Temporary {
    fn write_results<T: Write>(&self, writer: &mut T) -> std::io::Result<()> {
        writeln!(
            writer,
            "Results of searching {:?} with excluded directories {:?} and excluded items {:?}",
            self.common_data.directories.included_directories,
            self.common_data.directories.excluded_directories,
            self.common_data.excluded_items.get_excluded_items()
        )?;
        writeln!(writer, "Found {} temporary files.\n", self.information.number_of_temporary_files)?;

        for file_entry in &self.temporary_files {
            writeln!(writer, "\"{}\"", file_entry.path.to_string_lossy())?;
        }

        Ok(())
    }

    fn save_results_to_file_as_json(&self, file_name: &str, pretty_print: bool) -> std::io::Result<()> {
        self.save_results_to_file_as_json_internal(file_name, &self.temporary_files, pretty_print)
    }
}

impl Default for Temporary {
    fn default() -> Self {
        Self::new()
    }
}

impl DebugPrint for Temporary {
    #[allow(clippy::print_stdout)]
    fn debug_print(&self) {
        if !cfg!(debug_assertions) {
            return;
        }
        println!("### Information's");
        println!("Temporary list size - {}", self.temporary_files.len());
        self.debug_print_common();
    }
}

impl CommonData for Temporary {
    type Info = Info;
    type Parameters = ();

    fn get_information(&self) -> Self::Info {
        self.information.clone()
    }
    fn get_params(&self) -> Self::Parameters {}
    fn get_cd(&self) -> &CommonToolData {
        &self.common_data
    }
    fn get_cd_mut(&mut self) -> &mut CommonToolData {
        &mut self.common_data
    }
    fn found_any_broken_files(&self) -> bool {
        self.information.number_of_temporary_files > 0
    }
}

impl Temporary {
    pub const fn get_temporary_files(&self) -> &Vec<TemporaryFileEntry> {
        &self.temporary_files
    }

    pub const fn get_information(&self) -> &Info {
        &self.information
    }
}

pub(crate) fn check_folder_children(
    dir_result: &mut Vec<PathBuf>,
    warnings: &mut Vec<String>,
    entry_data: &DirEntry,
    recursive_search: bool,
    directories: &Directories,
    excluded_items: &ExcludedItems,
) {
    if !recursive_search {
        return;
    }

    let next_item = entry_data.path();
    if directories.is_excluded(&next_item) {
        return;
    }

    if excluded_items.is_excluded(&next_item) {
        return;
    }

    #[cfg(target_family = "unix")]
    if directories.exclude_other_filesystems() {
        match directories.is_on_other_filesystems(&next_item) {
            Ok(true) => return,
            Err(e) => warnings.push(e),
            _ => (),
        }
    }

    dir_result.push(next_item);
}
