use std::fs;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

use crossbeam_channel::{Receiver, Sender};
use fun_time::fun_time;
use log::debug;
use serde::{Deserialize, Serialize};

use crate::common_dir_traversal::{Collect, DirTraversalBuilder, DirTraversalResult, ErrorType, FileEntry, ToolType};
use crate::common_tool::{CommonData, CommonToolData, DeleteMethod};
use crate::common_traits::*;
use crate::progress_data::ProgressData;

#[derive(Default)]
pub struct Info {
    pub number_of_invalid_symlinks: usize,
}

const MAX_NUMBER_OF_SYMLINK_JUMPS: i32 = 20;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct SymlinkInfo {
    pub destination_path: PathBuf,
    pub type_of_error: ErrorType,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SymlinksFileEntry {
    pub path: PathBuf,
    pub size: u64,
    pub modified_date: u64,
    pub symlink_info: SymlinkInfo,
}

impl ResultEntry for SymlinksFileEntry {
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

impl FileEntry {
    fn into_symlinks_entry(self, symlink_info: SymlinkInfo) -> SymlinksFileEntry {
        SymlinksFileEntry {
            size: self.size,
            path: self.path,
            modified_date: self.modified_date,

            symlink_info,
        }
    }
}

pub struct InvalidSymlinks {
    common_data: CommonToolData,
    information: Info,
    invalid_symlinks: Vec<SymlinksFileEntry>,
}
impl InvalidSymlinks {
    pub fn new() -> Self {
        Self {
            common_data: CommonToolData::new(ToolType::InvalidSymlinks),
            information: Info::default(),
            invalid_symlinks: vec![],
        }
    }

    #[fun_time(message = "find_invalid_links", level = "info")]
    pub fn find_invalid_links(&mut self, stop_receiver: Option<&Receiver<()>>, progress_sender: Option<&Sender<ProgressData>>) {
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
        let result = DirTraversalBuilder::new()
            .common_data(&self.common_data)
            .group_by(|_fe| ())
            .stop_receiver(stop_receiver)
            .progress_sender(progress_sender)
            .collect(Collect::InvalidSymlinks)
            .build()
            .run();

        match result {
            DirTraversalResult::SuccessFiles { grouped_file_entries, warnings } => {
                self.invalid_symlinks = grouped_file_entries
                    .into_values()
                    .flatten()
                    .filter_map(|e| {
                        let (destination_path, type_of_error) = Self::check_invalid_symlinks(&e.path)?;
                        Some(e.into_symlinks_entry(SymlinkInfo { destination_path, type_of_error }))
                    })
                    .collect();
                self.information.number_of_invalid_symlinks = self.invalid_symlinks.len();
                self.common_data.text_messages.warnings.extend(warnings);
                debug!("Found {} invalid symlinks.", self.information.number_of_invalid_symlinks);
                true
            }
            DirTraversalResult::Stopped => false,
        }
    }

    fn check_invalid_symlinks(current_file_name: &Path) -> Option<(PathBuf, ErrorType)> {
        let mut destination_path = PathBuf::new();
        let type_of_error;

        match current_file_name.read_link() {
            Ok(t) => {
                destination_path.push(t);
                let mut number_of_loop = 0;
                let mut current_path = current_file_name.to_path_buf();
                loop {
                    if number_of_loop == 0 && !current_path.exists() {
                        type_of_error = ErrorType::NonExistentFile;
                        break;
                    }
                    if number_of_loop == MAX_NUMBER_OF_SYMLINK_JUMPS {
                        type_of_error = ErrorType::InfiniteRecursion;
                        break;
                    }

                    current_path = match current_path.read_link() {
                        Ok(t) => t,
                        Err(_inspected) => {
                            // Looks that some next symlinks are broken, but we do nothing with it - TODO why they are broken
                            return None;
                        }
                    };

                    number_of_loop += 1;
                }
            }
            Err(_inspected) => {
                // Failed to load info about it
                type_of_error = ErrorType::NonExistentFile;
            }
        }
        Some((destination_path, type_of_error))
    }

    #[fun_time(message = "delete_files", level = "debug")]
    fn delete_files(&mut self) {
        match self.common_data.delete_method {
            DeleteMethod::Delete => {
                for file_entry in &self.invalid_symlinks {
                    if fs::remove_file(&file_entry.path).is_err() {
                        self.common_data.text_messages.warnings.push(file_entry.path.to_string_lossy().to_string());
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

impl Default for InvalidSymlinks {
    fn default() -> Self {
        Self::new()
    }
}

impl DebugPrint for InvalidSymlinks {
    fn debug_print(&self) {
        if !cfg!(debug_assertions) {
            return;
        }
        println!("---------------DEBUG PRINT---------------");
        println!("Invalid symlinks list size - {}", self.invalid_symlinks.len());
        self.debug_print_common();
        println!("-----------------------------------------");
    }
}

impl PrintResults for InvalidSymlinks {
    fn write_results<T: Write>(&self, writer: &mut T) -> std::io::Result<()> {
        if !self.invalid_symlinks.is_empty() {
            writeln!(writer, "Found {} invalid symlinks.", self.information.number_of_invalid_symlinks)?;
            for file_entry in &self.invalid_symlinks {
                writeln!(
                    writer,
                    "\"{}\"\t\t\"{}\"\t\t{}",
                    file_entry.path.to_string_lossy(),
                    file_entry.symlink_info.destination_path.to_string_lossy(),
                    match file_entry.symlink_info.type_of_error {
                        ErrorType::InfiniteRecursion => "Infinite Recursion",
                        ErrorType::NonExistentFile => "Non Existent File",
                    }
                )?;
            }
        } else {
            write!(writer, "Not found any invalid symlinks.")?;
        }

        Ok(())
    }

    fn save_results_to_file_as_json(&self, file_name: &str, pretty_print: bool) -> std::io::Result<()> {
        self.save_results_to_file_as_json_internal(file_name, &self.invalid_symlinks, pretty_print)
    }
}

impl CommonData for InvalidSymlinks {
    fn get_cd(&self) -> &CommonToolData {
        &self.common_data
    }
    fn get_cd_mut(&mut self) -> &mut CommonToolData {
        &mut self.common_data
    }
}

impl InvalidSymlinks {
    pub const fn get_invalid_symlinks(&self) -> &Vec<SymlinksFileEntry> {
        &self.invalid_symlinks
    }

    pub const fn get_information(&self) -> &Info {
        &self.information
    }
}
