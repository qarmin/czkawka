use std::collections::BTreeMap;
use std::fs;
use std::fs::{DirEntry, File, Metadata};
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use crossbeam_channel::Receiver;
use futures::channel::mpsc::UnboundedSender;
use humansize::format_size;
use humansize::BINARY;
use rayon::prelude::*;

use crate::common::{check_folder_children, prepare_thread_handler_common, send_info_and_wait_for_ending_all_threads, split_path};
use crate::common_dir_traversal::{common_get_entry_data_metadata, common_read_dir, get_lowercase_name, get_modified_time, CheckingMethod, ProgressData, ToolType};
use crate::common_directory::Directories;
use crate::common_extensions::Extensions;
use crate::common_items::ExcludedItems;
use crate::common_messages::Messages;
use crate::common_traits::{DebugPrint, PrintResults, SaveResults};

#[derive(Clone)]
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

#[derive(Eq, PartialEq, Clone, Debug, Copy)]
pub enum DeleteMethod {
    None,
    Delete,
}

/// Info struck with helpful information's about results
#[derive(Default)]
pub struct Info {
    pub number_of_real_files: usize,
}

impl Info {
    #[must_use]
    pub fn new() -> Self {
        Default::default()
    }
}

/// Struct with required information's to work
pub struct BigFile {
    tool_type: ToolType,
    text_messages: Messages,
    information: Info,
    big_files: Vec<(u64, FileEntry)>,
    excluded_items: ExcludedItems,
    directories: Directories,
    allowed_extensions: Extensions,
    recursive_search: bool,
    number_of_files_to_check: usize,
    delete_method: DeleteMethod,
    stopped_search: bool,
    search_mode: SearchMode,
}

impl BigFile {
    #[must_use]
    pub fn new() -> Self {
        Self {
            tool_type: ToolType::BigFile,
            text_messages: Default::default(),
            information: Info::new(),
            big_files: Default::default(),
            excluded_items: ExcludedItems::new(),
            directories: Directories::new(),
            allowed_extensions: Extensions::new(),
            recursive_search: true,
            number_of_files_to_check: 50,
            delete_method: DeleteMethod::None,
            stopped_search: false,
            search_mode: SearchMode::BiggestFiles,
        }
    }

    pub fn find_big_files(&mut self, stop_receiver: Option<&Receiver<()>>, progress_sender: Option<&UnboundedSender<ProgressData>>) {
        self.optimize_directories();
        if !self.look_for_big_files(stop_receiver, progress_sender) {
            self.stopped_search = true;
            return;
        }
        self.delete_files();
        self.debug_print();
    }
    #[must_use]
    pub fn get_stopped_search(&self) -> bool {
        self.stopped_search
    }

    pub fn set_search_mode(&mut self, search_mode: SearchMode) {
        self.search_mode = search_mode;
    }

    #[must_use]
    pub const fn get_big_files(&self) -> &Vec<(u64, FileEntry)> {
        &self.big_files
    }

    #[must_use]
    pub const fn get_text_messages(&self) -> &Messages {
        &self.text_messages
    }

    #[must_use]
    pub const fn get_information(&self) -> &Info {
        &self.information
    }

    pub fn set_delete_method(&mut self, delete_method: DeleteMethod) {
        self.delete_method = delete_method;
    }

    pub fn set_recursive_search(&mut self, recursive_search: bool) {
        self.recursive_search = recursive_search;
    }

    #[cfg(target_family = "unix")]
    pub fn set_exclude_other_filesystems(&mut self, exclude_other_filesystems: bool) {
        self.directories.set_exclude_other_filesystems(exclude_other_filesystems);
    }
    #[cfg(not(target_family = "unix"))]
    pub fn set_exclude_other_filesystems(&mut self, _exclude_other_filesystems: bool) {}

    /// List of allowed extensions, only files with this extensions will be checking if are duplicates
    pub fn set_allowed_extensions(&mut self, allowed_extensions: String) {
        self.allowed_extensions.set_allowed_extensions(allowed_extensions, &mut self.text_messages);
    }

    fn look_for_big_files(&mut self, stop_receiver: Option<&Receiver<()>>, progress_sender: Option<&UnboundedSender<ProgressData>>) -> bool {
        let mut folders_to_check: Vec<PathBuf> = Vec::with_capacity(1024 * 2); // This should be small enough too not see to big difference and big enough to store most of paths without needing to resize vector
        let mut old_map: BTreeMap<u64, Vec<FileEntry>> = Default::default();

        // Add root folders for finding
        for id in &self.directories.included_directories {
            folders_to_check.push(id.clone());
        }

        let (progress_thread_handle, progress_thread_run, atomic_counter, _check_was_stopped) =
            prepare_thread_handler_common(progress_sender, 0, 0, 0, CheckingMethod::None, self.tool_type);

        while !folders_to_check.is_empty() {
            if stop_receiver.is_some() && stop_receiver.unwrap().try_recv().is_ok() {
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
                                self.recursive_search,
                                &self.directories,
                                &self.excluded_items,
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
                self.text_messages.warnings.extend(warnings);
                for (size, fe) in fe_result {
                    old_map.entry(size).or_insert_with(Vec::new).push(fe);
                }
            }
        }

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

        if !self.allowed_extensions.matches_filename(&file_name_lowercase) {
            return;
        }

        let current_file_name = current_folder.join(entry_data.file_name());
        if self.excluded_items.is_excluded(&current_file_name) {
            return;
        }

        let fe: FileEntry = FileEntry {
            path: current_file_name.clone(),
            size: metadata.len(),
            modified_date: get_modified_time(metadata, warnings, &current_file_name, false),
        };

        fe_result.push((fe.size, fe));
    }

    pub fn extract_n_biggest_files(&mut self, old_map: BTreeMap<u64, Vec<FileEntry>>) {
        let iter: Box<dyn Iterator<Item = _>>;
        if self.search_mode == SearchMode::SmallestFiles {
            iter = Box::new(old_map.into_iter());
        } else {
            iter = Box::new(old_map.into_iter().rev());
        }

        for (size, mut vector) in iter {
            if self.information.number_of_real_files < self.number_of_files_to_check {
                if vector.len() > 1 {
                    vector.sort_unstable_by_key(|e| {
                        let t = split_path(e.path.as_path());
                        (t.0, t.1)
                    });
                }
                for file in vector {
                    if self.information.number_of_real_files < self.number_of_files_to_check {
                        self.big_files.push((size, file));
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

    pub fn set_number_of_files_to_check(&mut self, number_of_files_to_check: usize) {
        self.number_of_files_to_check = number_of_files_to_check;
    }

    /// Setting excluded items which needs to contains * wildcard
    /// Are a lot of slower than absolute path, so it should be used to heavy
    pub fn set_excluded_items(&mut self, excluded_items: Vec<String>) {
        self.excluded_items.set_excluded_items(excluded_items, &mut self.text_messages);
    }

    fn optimize_directories(&mut self) {
        self.directories.optimize_directories(self.recursive_search, &mut self.text_messages);
    }

    pub fn set_included_directory(&mut self, included_directory: Vec<PathBuf>) {
        self.directories.set_included_directory(included_directory, &mut self.text_messages);
    }

    pub fn set_excluded_directory(&mut self, excluded_directory: Vec<PathBuf>) {
        self.directories.set_excluded_directory(excluded_directory, &mut self.text_messages);
    }

    /// Function to delete files, from filed Vector
    fn delete_files(&mut self) {
        match self.delete_method {
            DeleteMethod::Delete => {
                for (_, file_entry) in &self.big_files {
                    if fs::remove_file(&file_entry.path).is_err() {
                        self.text_messages.warnings.push(file_entry.path.display().to_string());
                    }
                }
            }
            DeleteMethod::None => {
                //Just do nothing
            }
        }
    }
}

impl Default for BigFile {
    fn default() -> Self {
        Self::new()
    }
}

impl DebugPrint for BigFile {
    #[allow(dead_code)]
    #[allow(unreachable_code)]
    /// Debugging printing - only available on debug build
    fn debug_print(&self) {
        #[cfg(not(debug_assertions))]
        {
            return;
        }
        println!("---------------DEBUG PRINT---------------");
        println!("### Information's");

        println!("Errors size - {}", self.text_messages.errors.len());
        println!("Warnings size - {}", self.text_messages.warnings.len());
        println!("Messages size - {}", self.text_messages.messages.len());

        println!("### Other");
        println!("Big files size {} in {} groups", self.information.number_of_real_files, self.big_files.len());
        println!("Excluded items - {:?}", self.excluded_items.items);
        println!("Included directories - {:?}", self.directories.included_directories);
        println!("Excluded directories - {:?}", self.directories.excluded_directories);
        println!("Recursive search - {}", self.recursive_search);
        #[cfg(target_family = "unix")]
        println!("Skip other filesystems - {}", self.directories.exclude_other_filesystems());
        println!("Number of files to check - {:?}", self.number_of_files_to_check);
        println!("-----------------------------------------");
    }
}

impl SaveResults for BigFile {
    /// Saving results to provided file
    fn save_results_to_file(&mut self, file_name: &str) -> bool {
        let file_name: String = match file_name {
            "" => "results.txt".to_string(),
            k => k.to_string(),
        };

        let file_handler = match File::create(&file_name) {
            Ok(t) => t,
            Err(e) => {
                self.text_messages.errors.push(format!("Failed to create file {file_name}, reason {e}"));
                return false;
            }
        };
        let mut writer = BufWriter::new(file_handler);

        if let Err(e) = writeln!(
            writer,
            "Results of searching {:?} with excluded directories {:?} and excluded items {:?}",
            self.directories.included_directories, self.directories.excluded_directories, self.excluded_items.items
        ) {
            self.text_messages.errors.push(format!("Failed to save results to file {file_name}, reason {e}"));
            return false;
        }

        if self.information.number_of_real_files != 0 {
            if self.search_mode == SearchMode::BiggestFiles {
                write!(writer, "{} the biggest files.\n\n", self.information.number_of_real_files).unwrap();
            } else {
                write!(writer, "{} the smallest files.\n\n", self.information.number_of_real_files).unwrap();
            }
            for (size, file_entry) in &self.big_files {
                writeln!(writer, "{} ({}) - {}", format_size(*size, BINARY), size, file_entry.path.display()).unwrap();
            }
        } else {
            write!(writer, "Not found any files.").unwrap();
        }

        true
    }
}

impl PrintResults for BigFile {
    fn print_results(&self) {
        if self.search_mode == SearchMode::BiggestFiles {
            println!("{} the biggest files.\n\n", self.information.number_of_real_files);
        } else {
            println!("{} the smallest files.\n\n", self.information.number_of_real_files);
        }
        for (size, file_entry) in &self.big_files {
            println!("{} ({}) - {}", format_size(*size, BINARY), size, file_entry.path.display());
        }
    }
}
