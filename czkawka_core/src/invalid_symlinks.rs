use std::fs;

use std::io::prelude::*;

use crossbeam_channel::Receiver;
use fun_time::fun_time;
use futures::channel::mpsc::UnboundedSender;
use log::debug;

use crate::common_dir_traversal::{Collect, DirTraversalBuilder, DirTraversalResult, ErrorType, FileEntry, ProgressData, ToolType};
use crate::common_tool::{CommonData, CommonToolData, DeleteMethod};
use crate::common_traits::*;

#[derive(Default)]
pub struct Info {
    pub number_of_invalid_symlinks: usize,
}

pub struct InvalidSymlinks {
    common_data: CommonToolData,
    information: Info,
    invalid_symlinks: Vec<FileEntry>,
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
    pub fn find_invalid_links(&mut self, stop_receiver: Option<&Receiver<()>>, progress_sender: Option<&UnboundedSender<ProgressData>>) {
        self.optimize_dirs_before_start();
        if !self.check_files(stop_receiver, progress_sender) {
            self.common_data.stopped_search = true;
            return;
        }
        self.delete_files();
        self.debug_print();
    }

    #[fun_time(message = "check_files", level = "debug")]
    fn check_files(&mut self, stop_receiver: Option<&Receiver<()>>, progress_sender: Option<&UnboundedSender<ProgressData>>) -> bool {
        let result = DirTraversalBuilder::new()
            .root_dirs(self.common_data.directories.included_directories.clone())
            .group_by(|_fe| ())
            .stop_receiver(stop_receiver)
            .progress_sender(progress_sender)
            .collect(Collect::InvalidSymlinks)
            .directories(self.common_data.directories.clone())
            .allowed_extensions(self.common_data.allowed_extensions.clone())
            .excluded_items(self.common_data.excluded_items.clone())
            .recursive_search(self.common_data.recursive_search)
            .build()
            .run();

        match result {
            DirTraversalResult::SuccessFiles { grouped_file_entries, warnings } => {
                self.invalid_symlinks = grouped_file_entries.into_values().flatten().collect();
                self.information.number_of_invalid_symlinks = self.invalid_symlinks.len();
                self.common_data.text_messages.warnings.extend(warnings);
                debug!("Found {} invalid symlinks.", self.information.number_of_invalid_symlinks);
                true
            }
            DirTraversalResult::SuccessFolders { .. } => unreachable!(),
            DirTraversalResult::Stopped => false,
        }
    }

    #[fun_time(message = "delete_files", level = "debug")]
    fn delete_files(&mut self) {
        match self.common_data.delete_method {
            DeleteMethod::Delete => {
                for file_entry in &self.invalid_symlinks {
                    if fs::remove_file(file_entry.path.clone()).is_err() {
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
                    "{}\t\t{}\t\t{}",
                    file_entry.path.display(),
                    file_entry.symlink_info.clone().expect("invalid traversal result").destination_path.display(),
                    match file_entry.symlink_info.clone().expect("invalid traversal result").type_of_error {
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
    pub const fn get_invalid_symlinks(&self) -> &Vec<FileEntry> {
        &self.invalid_symlinks
    }

    pub const fn get_information(&self) -> &Info {
        &self.information
    }
}
