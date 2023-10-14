use std::collections::BTreeMap;
use std::fs;
use std::fs::{DirEntry, Metadata};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use crossbeam_channel::Receiver;
use fun_time::fun_time;
use futures::channel::mpsc::UnboundedSender;
use humansize::{format_size, BINARY};
use log::debug;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};

use crate::common::{check_folder_children, check_if_stop_received, prepare_thread_handler_common, send_info_and_wait_for_ending_all_threads, split_path};
use crate::common_dir_traversal::{common_get_entry_data_metadata, common_read_dir, get_lowercase_name, get_modified_time, CheckingMethod, ProgressData, ToolType};
use crate::common_tool::{CommonData, CommonToolData, DeleteMethod};
use crate::common_traits::{DebugPrint, PrintResults};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FileEntry {
    pub path: PathBuf,
    pub size: u64,
    pub modified_date: u64,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum SearchMode {
    BiggestFiles,
    SmallestFiles,
}

#[derive(Default)]
pub struct Info {
    pub number_of_real_files: usize,
}

pub struct BigFile {
    common_data: CommonToolData,
    information: Info,
    big_files: Vec<FileEntry>,
    number_of_files_to_check: usize,
    search_mode: SearchMode,
}

impl BigFile {
    pub fn new() -> Self {
        Self {
            common_data: CommonToolData::new(ToolType::BigFile),
            information: Info::default(),
            big_files: Default::default(),
            number_of_files_to_check: 50,
            search_mode: SearchMode::BiggestFiles,
        }
    }

    #[fun_time(message = "find_big_files", level = "info")]
    pub fn find_big_files(&mut self, stop_receiver: Option<&Receiver<()>>, progress_sender: Option<&UnboundedSender<ProgressData>>) {
        self.optimize_dirs_before_start();
        if !self.look_for_big_files(stop_receiver, progress_sender) {
            self.common_data.stopped_search = true;
            return;
        }
        self.delete_files();
        self.debug_print();
    }

    #[fun_time(message = "look_for_big_files", level = "debug")]
    fn look_for_big_files(&mut self, stop_receiver: Option<&Receiver<()>>, progress_sender: Option<&UnboundedSender<ProgressData>>) -> bool {
        let mut folders_to_check: Vec<PathBuf> = Vec::with_capacity(1024 * 2); // This should be small enough too not see to big difference and big enough to store most of paths without needing to resize vector
        let mut old_map: BTreeMap<u64, Vec<FileEntry>> = Default::default();

        // Add root folders for finding
        for id in &self.common_data.directories.included_directories {
            folders_to_check.push(id.clone());
        }

        let (progress_thread_handle, progress_thread_run, atomic_counter, _check_was_stopped) =
            prepare_thread_handler_common(progress_sender, 0, 0, 0, CheckingMethod::None, self.common_data.tool_type);

        debug!("Starting to search for big files");
        while !folders_to_check.is_empty() {
            if check_if_stop_received(stop_receiver) {
                send_info_and_wait_for_ending_all_threads(&progress_thread_run, progress_thread_handle);
                return false;
            }

            let segments: Vec<_> = folders_to_check
                .par_iter()
                .map(|current_folder| {
                    let mut dir_result = vec![];
                    let mut warnings = vec![];
                    let mut fe_result = vec![];

                    let Some(read_dir) = common_read_dir(current_folder, &mut warnings) else {
                        return (dir_result, warnings, fe_result);
                    };

                    // Check every sub folder/file/link etc.
                    for entry in read_dir {
                        let Some((entry_data, metadata)) = common_get_entry_data_metadata(&entry, &mut warnings, current_folder) else {
                            continue;
                        };

                        if metadata.is_dir() {
                            check_folder_children(
                                &mut dir_result,
                                &mut warnings,
                                current_folder,
                                entry_data,
                                self.common_data.recursive_search,
                                &self.common_data.directories,
                                &self.common_data.excluded_items,
                            );
                        } else if metadata.is_file() {
                            self.collect_file_entry(&atomic_counter, &metadata, entry_data, &mut fe_result, &mut warnings, current_folder);
                        }
                    }
                    (dir_result, warnings, fe_result)
                })
                .collect();

            // Advance the frontier
            folders_to_check.clear();

            // Process collected data
            for (segment, warnings, fe_result) in segments {
                folders_to_check.extend(segment);
                self.common_data.text_messages.warnings.extend(warnings);
                for (size, fe) in fe_result {
                    old_map.entry(size).or_default().push(fe);
                }
            }
        }

        debug!("Collected {} files", old_map.len());

        send_info_and_wait_for_ending_all_threads(&progress_thread_run, progress_thread_handle);

        self.extract_n_biggest_files(old_map);

        true
    }

    pub fn collect_file_entry(
        &self,
        atomic_counter: &Arc<AtomicUsize>,
        metadata: &Metadata,
        entry_data: &DirEntry,
        fe_result: &mut Vec<(u64, FileEntry)>,
        warnings: &mut Vec<String>,
        current_folder: &Path,
    ) {
        atomic_counter.fetch_add(1, Ordering::Relaxed);

        if metadata.len() == 0 {
            return;
        }

        let Some(file_name_lowercase) = get_lowercase_name(entry_data, warnings) else {
            return;
        };

        if !self.common_data.allowed_extensions.matches_filename(&file_name_lowercase) {
            return;
        }

        let current_file_name = current_folder.join(entry_data.file_name());
        if self.common_data.excluded_items.is_excluded(&current_file_name) {
            return;
        }

        let fe: FileEntry = FileEntry {
            path: current_file_name.clone(),
            size: metadata.len(),
            modified_date: get_modified_time(metadata, warnings, &current_file_name, false),
        };

        fe_result.push((fe.size, fe));
    }

    #[fun_time(message = "extract_n_biggest_files", level = "debug")]
    pub fn extract_n_biggest_files(&mut self, old_map: BTreeMap<u64, Vec<FileEntry>>) {
        let iter: Box<dyn Iterator<Item = _>>;
        if self.search_mode == SearchMode::SmallestFiles {
            iter = Box::new(old_map.into_iter());
        } else {
            iter = Box::new(old_map.into_iter().rev());
        }

        for (_size, mut vector) in iter {
            if self.information.number_of_real_files < self.number_of_files_to_check {
                if vector.len() > 1 {
                    vector.sort_unstable_by_key(|e| {
                        let t = split_path(e.path.as_path());
                        (t.0, t.1)
                    });
                }
                for file in vector {
                    if self.information.number_of_real_files < self.number_of_files_to_check {
                        self.big_files.push(file);
                        self.information.number_of_real_files += 1;
                    } else {
                        break;
                    }
                }
            } else {
                break;
            }
        }
    }

    fn delete_files(&mut self) {
        match self.common_data.delete_method {
            DeleteMethod::Delete => {
                for file_entry in &self.big_files {
                    if fs::remove_file(&file_entry.path).is_err() {
                        self.common_data.text_messages.warnings.push(file_entry.path.display().to_string());
                    }
                }
            }
            DeleteMethod::None => {
                //Just do nothing
            }
            _ => unreachable!(),
        }
    }
}

impl Default for BigFile {
    fn default() -> Self {
        Self::new()
    }
}

impl DebugPrint for BigFile {
    fn debug_print(&self) {
        if !cfg!(debug_assertions) {
            return;
        }

        println!("### INDIVIDUAL DEBUG PRINT ###");
        println!("Big files size {} in {} groups", self.information.number_of_real_files, self.big_files.len());
        println!("Number of files to check - {:?}", self.number_of_files_to_check);
        self.debug_print_common();
        println!("-----------------------------------------");
    }
}

impl PrintResults for BigFile {
    fn write_results<T: Write>(&self, writer: &mut T) -> std::io::Result<()> {
        writeln!(
            writer,
            "Results of searching {:?} with excluded directories {:?} and excluded items {:?}",
            self.common_data.directories.included_directories, self.common_data.directories.excluded_directories, self.common_data.excluded_items.items
        )?;

        if self.information.number_of_real_files != 0 {
            if self.search_mode == SearchMode::BiggestFiles {
                writeln!(writer, "{} the biggest files.\n\n", self.information.number_of_real_files)?;
            } else {
                writeln!(writer, "{} the smallest files.\n\n", self.information.number_of_real_files)?;
            }
            for file_entry in &self.big_files {
                writeln!(writer, "{} ({}) - {}", format_size(file_entry.size, BINARY), file_entry.size, file_entry.path.display())?;
            }
        } else {
            write!(writer, "Not found any files.").unwrap();
        }

        Ok(())
    }

    fn save_results_to_file_as_json(&self, file_name: &str, pretty_print: bool) -> std::io::Result<()> {
        self.save_results_to_file_as_json_internal(file_name, &self.big_files, pretty_print)
    }
}

impl CommonData for BigFile {
    fn get_cd(&self) -> &CommonToolData {
        &self.common_data
    }
    fn get_cd_mut(&mut self) -> &mut CommonToolData {
        &mut self.common_data
    }
}

impl BigFile {
    pub fn set_search_mode(&mut self, search_mode: SearchMode) {
        self.search_mode = search_mode;
    }

    pub const fn get_big_files(&self) -> &Vec<FileEntry> {
        &self.big_files
    }

    pub const fn get_information(&self) -> &Info {
        &self.information
    }

    pub fn set_number_of_files_to_check(&mut self, number_of_files_to_check: usize) {
        self.number_of_files_to_check = number_of_files_to_check;
    }
}
