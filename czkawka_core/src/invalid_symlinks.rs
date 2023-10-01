use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;

use crossbeam_channel::Receiver;
use futures::channel::mpsc::UnboundedSender;
use log::{debug, info};

use crate::common_dir_traversal::{Collect, DirTraversalBuilder, DirTraversalResult, ErrorType, FileEntry, ProgressData, ToolType};
use crate::common_tool::{CommonData, CommonToolData};
use crate::common_traits::*;

#[derive(Eq, PartialEq, Clone, Debug, Copy)]
pub enum DeleteMethod {
    None,
    Delete,
}

/// Info struck with helpful information's about results
#[derive(Default)]
pub struct Info {
    pub number_of_invalid_symlinks: usize,
}

impl Info {
    pub fn new() -> Self {
        Default::default()
    }
}

/// Struct with required information's to work
pub struct InvalidSymlinks {
    common_data: CommonToolData,
    information: Info,
    invalid_symlinks: Vec<FileEntry>,
    delete_method: DeleteMethod,
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
    pub fn new() -> Self {
        Self {
            common_data: CommonToolData::new(ToolType::InvalidSymlinks),
            information: Info::new(),
            invalid_symlinks: vec![],
            delete_method: DeleteMethod::None,
        }
    }

    pub fn find_invalid_links(&mut self, stop_receiver: Option<&Receiver<()>>, progress_sender: Option<&UnboundedSender<ProgressData>>) {
        info!("Starting finding invalid links");
        self.optimize_dirs_before_start();
        if !self.check_files(stop_receiver, progress_sender) {
            self.common_data.stopped_search = true;
            return;
        }
        self.delete_files();
        self.debug_print();
    }

    pub const fn get_invalid_symlinks(&self) -> &Vec<FileEntry> {
        &self.invalid_symlinks
    }

    pub const fn get_information(&self) -> &Info {
        &self.information
    }

    pub fn set_delete_method(&mut self, delete_method: DeleteMethod) {
        self.delete_method = delete_method;
    }

    /// Check files for any with size == 0
    fn check_files(&mut self, stop_receiver: Option<&Receiver<()>>, progress_sender: Option<&UnboundedSender<ProgressData>>) -> bool {
        debug!("check_files - start");
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
        debug!("check_files - collected files");
        let res = match result {
            DirTraversalResult::SuccessFiles { grouped_file_entries, warnings } => {
                if let Some(((), invalid_symlinks)) = grouped_file_entries.into_iter().next() {
                    self.invalid_symlinks = invalid_symlinks;
                }
                self.information.number_of_invalid_symlinks = self.invalid_symlinks.len();
                self.common_data.text_messages.warnings.extend(warnings);
                true
            }
            DirTraversalResult::SuccessFolders { .. } => unreachable!(),
            DirTraversalResult::Stopped => false,
        };
        debug!("check_files - end");
        res
    }

    /// Function to delete files, from filed Vector
    fn delete_files(&mut self) {
        match self.delete_method {
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
        }
    }
}

impl Default for InvalidSymlinks {
    fn default() -> Self {
        Self::new()
    }
}

impl DebugPrint for InvalidSymlinks {
    #[allow(dead_code)]
    #[allow(unreachable_code)]
    /// Debugging printing - only available on debug build
    fn debug_print(&self) {
        #[cfg(not(debug_assertions))]
        {
            return;
        }
        println!("---------------DEBUG PRINT---------------");
        println!("Invalid symlinks list size - {}", self.invalid_symlinks.len());
        println!("Delete Method - {:?}", self.delete_method);
        self.debug_print_common();
        println!("-----------------------------------------");
    }
}

impl SaveResults for InvalidSymlinks {
    fn save_results_to_file(&mut self, file_name: &str) -> bool {
        let file_name: String = match file_name {
            "" => "results.txt".to_string(),
            k => k.to_string(),
        };

        let file_handler = match File::create(&file_name) {
            Ok(t) => t,
            Err(e) => {
                self.common_data.text_messages.errors.push(format!("Failed to create file {file_name}, reason {e}"));
                return false;
            }
        };
        let mut writer = BufWriter::new(file_handler);

        if let Err(e) = writeln!(
            writer,
            "Results of searching {:?} with excluded directories {:?} and excluded items {:?}",
            self.common_data.directories.included_directories, self.common_data.directories.excluded_directories, self.common_data.excluded_items.items
        ) {
            self.common_data
                .text_messages
                .errors
                .push(format!("Failed to save results to file {file_name}, reason {e}"));
            return false;
        }

        if !self.invalid_symlinks.is_empty() {
            writeln!(writer, "Found {} invalid symlinks.", self.information.number_of_invalid_symlinks).unwrap();
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
                )
                .unwrap();
            }
        } else {
            write!(writer, "Not found any invalid symlinks.").unwrap();
        }
        true
    }
}

impl PrintResults for InvalidSymlinks {
    /// Print information's about duplicated entries
    /// Only needed for CLI
    fn print_results(&self) {
        println!("Found {} invalid symlinks.\n", self.information.number_of_invalid_symlinks);
        for file_entry in &self.invalid_symlinks {
            println!(
                "{}\t\t{}\t\t{}",
                file_entry.path.display(),
                file_entry.symlink_info.clone().expect("invalid traversal result").destination_path.display(),
                match file_entry.symlink_info.clone().expect("invalid traversal result").type_of_error {
                    ErrorType::InfiniteRecursion => "Infinite Recursion",
                    ErrorType::NonExistentFile => "Non Existent File",
                }
            );
        }
    }
}
