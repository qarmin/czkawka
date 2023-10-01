use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;

use crossbeam_channel::Receiver;
use futures::channel::mpsc::UnboundedSender;
use log::{debug, info};

use crate::common_dir_traversal::{DirTraversalBuilder, DirTraversalResult, FileEntry, ProgressData, ToolType};
use crate::common_tool::{CommonData, CommonToolData};
use crate::common_traits::*;

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum DeleteMethod {
    None,
    Delete,
}

/// Info struck with helpful information's about results
#[derive(Default)]
pub struct Info {
    pub number_of_empty_files: usize,
}

impl Info {
    pub fn new() -> Self {
        Default::default()
    }
}

/// Struct with required information's to work
pub struct EmptyFiles {
    common_data: CommonToolData,
    information: Info,
    empty_files: Vec<FileEntry>,
    delete_method: DeleteMethod,
}

impl CommonData for EmptyFiles {
    fn get_cd(&self) -> &CommonToolData {
        &self.common_data
    }
    fn get_cd_mut(&mut self) -> &mut CommonToolData {
        &mut self.common_data
    }
}

impl EmptyFiles {
    pub fn new() -> Self {
        Self {
            common_data: CommonToolData::new(ToolType::EmptyFiles),
            information: Info::new(),
            empty_files: vec![],
            delete_method: DeleteMethod::None,
        }
    }

    /// Finding empty files, save results to internal struct variables
    pub fn find_empty_files(&mut self, stop_receiver: Option<&Receiver<()>>, progress_sender: Option<&UnboundedSender<ProgressData>>) {
        info!("Starting finding empty files");
        self.optimize_dirs_before_start();
        if !self.check_files(stop_receiver, progress_sender) {
            self.common_data.stopped_search = true;
            return;
        }
        self.delete_files();
        self.debug_print();
    }

    pub const fn get_empty_files(&self) -> &Vec<FileEntry> {
        &self.empty_files
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
            .minimal_file_size(0)
            .maximal_file_size(0)
            .directories(self.common_data.directories.clone())
            .allowed_extensions(self.common_data.allowed_extensions.clone())
            .excluded_items(self.common_data.excluded_items.clone())
            .recursive_search(self.common_data.recursive_search)
            .build()
            .run();
        debug!("check_files - collected files to check");
        let res = match result {
            DirTraversalResult::SuccessFiles { grouped_file_entries, warnings } => {
                if let Some(empty_files) = grouped_file_entries.get(&()) {
                    self.empty_files = empty_files.clone();
                }
                self.information.number_of_empty_files = self.empty_files.len();
                self.common_data.text_messages.warnings.extend(warnings);

                true
            }
            DirTraversalResult::SuccessFolders { .. } => {
                unreachable!()
            }
            DirTraversalResult::Stopped => false,
        };
        debug!("check_files - end");
        res
    }

    /// Function to delete files, from filed Vector
    fn delete_files(&mut self) {
        match self.delete_method {
            DeleteMethod::Delete => {
                for file_entry in &self.empty_files {
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

impl Default for EmptyFiles {
    fn default() -> Self {
        Self::new()
    }
}

impl DebugPrint for EmptyFiles {
    #[allow(dead_code)]
    #[allow(unreachable_code)]
    /// Debugging printing - only available on debug build
    fn debug_print(&self) {
        #[cfg(not(debug_assertions))]
        {
            return;
        }
        println!("---------------DEBUG PRINT---------------");
        println!("Empty list size - {}", self.empty_files.len());
        println!("Delete Method - {:?}", self.delete_method);
        self.debug_print_common();
        println!("-----------------------------------------");
    }
}

impl SaveResults for EmptyFiles {
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

        if !self.empty_files.is_empty() {
            writeln!(writer, "Found {} empty files.", self.information.number_of_empty_files).unwrap();
            for file_entry in &self.empty_files {
                writeln!(writer, "{}", file_entry.path.display()).unwrap();
            }
        } else {
            write!(writer, "Not found any empty files.").unwrap();
        }

        true
    }
}

impl PrintResults for EmptyFiles {
    /// Print information's about duplicated entries
    /// Only needed for CLI
    fn print_results(&self) {
        println!("Found {} empty files.\n", self.information.number_of_empty_files);
        for file_entry in &self.empty_files {
            println!("{}", file_entry.path.display());
        }
    }
}
