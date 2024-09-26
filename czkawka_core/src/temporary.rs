use std::fs;
use std::fs::DirEntry;
use std::io::prelude::*;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use crossbeam_channel::{Receiver, Sender};
use fun_time::fun_time;
use rayon::prelude::*;
use serde::Serialize;

use crate::common::{check_folder_children, check_if_stop_received, prepare_thread_handler_common, send_info_and_wait_for_ending_all_threads};
use crate::common_dir_traversal::{common_read_dir, get_modified_time, ToolType};
use crate::common_tool::{CommonData, CommonToolData, DeleteMethod};
use crate::common_traits::*;
use crate::progress_data::{CurrentStage, ProgressData};

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
}

impl TemporaryFileEntry {
    pub fn get_path(&self) -> &PathBuf {
        &self.path
    }
    pub fn get_modified_date(&self) -> u64 {
        self.modified_date
    }
}

#[derive(Default)]
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
    pub fn find_temporary_files(&mut self, stop_receiver: Option<&Receiver<()>>, progress_sender: Option<&Sender<ProgressData>>) {
        self.prepare_items();
        if !self.check_files(stop_receiver, progress_sender) {
            self.common_data.stopped_search = true;
            return;
        }
        self.delete_files();
        self.debug_print();
    }

    #[fun_time(message = "check_files", level = "debug")]
    fn check_files(&mut self, stop_receiver: Option<&Receiver<()>>, progress_sender: Option<&Sender<ProgressData>>) -> bool {
        let mut folders_to_check: Vec<PathBuf> = self.common_data.directories.included_directories.clone();

        let (progress_thread_handle, progress_thread_run, atomic_counter, _check_was_stopped) =
            prepare_thread_handler_common(progress_sender, CurrentStage::CollectingFiles, 0, self.get_test_type());

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
                            if let Some(file_entry) = self.get_file_entry(&atomic_counter, &entry_data, &mut warnings) {
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

        send_info_and_wait_for_ending_all_threads(&progress_thread_run, progress_thread_handle);
        self.information.number_of_temporary_files = self.temporary_files.len();

        true
    }
    pub fn get_file_entry(&self, atomic_counter: &Arc<AtomicUsize>, entry_data: &DirEntry, warnings: &mut Vec<String>) -> Option<TemporaryFileEntry> {
        atomic_counter.fetch_add(1, Ordering::Relaxed);

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
            path: current_file_name,
        })
    }

    #[fun_time(message = "delete_files", level = "debug")]
    fn delete_files(&mut self) {
        match self.common_data.delete_method {
            DeleteMethod::Delete => {
                let mut warnings = Vec::new();
                for file_entry in &self.temporary_files {
                    if fs::remove_file(file_entry.path.clone()).is_err() {
                        warnings.push(file_entry.path.to_string_lossy().to_string());
                    }
                }
                self.common_data.text_messages.warnings.extend(warnings);
            }
            DeleteMethod::None => {
                //Just do nothing
            }
            _ => unreachable!(),
        }
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
            writeln!(writer, "{:?}", file_entry.path)?;
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
    fn get_cd(&self) -> &CommonToolData {
        &self.common_data
    }
    fn get_cd_mut(&mut self) -> &mut CommonToolData {
        &mut self.common_data
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
