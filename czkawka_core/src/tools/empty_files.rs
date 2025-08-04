use std::io::prelude::*;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use crossbeam_channel::Sender;
use fun_time::fun_time;
use log::debug;

use crate::common::dir_traversal::{DirTraversalBuilder, DirTraversalResult};
use crate::common::model::{FileEntry, ToolType, WorkContinueStatus};
use crate::common::progress_data::ProgressData;
use crate::common::tool_data::{CommonData, CommonToolData, DeleteItemType, DeleteMethod};
use crate::common::traits::*;

#[derive(Default)]
pub struct Info {
    pub number_of_empty_files: usize,
}

pub struct EmptyFiles {
    common_data: CommonToolData,
    information: Info,
    empty_files: Vec<FileEntry>,
}

impl CommonData for EmptyFiles {
    fn get_cd(&self) -> &CommonToolData {
        &self.common_data
    }
    fn get_cd_mut(&mut self) -> &mut CommonToolData {
        &mut self.common_data
    }
    fn found_any_broken_files(&self) -> bool {
        self.information.number_of_empty_files > 0
    }
}

impl EmptyFiles {
    pub fn new() -> Self {
        Self {
            common_data: CommonToolData::new(ToolType::EmptyFiles),
            information: Info::default(),
            empty_files: vec![],
        }
    }

    #[fun_time(message = "find_empty_files", level = "info")]
    pub fn find_empty_files(&mut self, stop_flag: &Arc<AtomicBool>, progress_sender: Option<&Sender<ProgressData>>) {
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
        let result = DirTraversalBuilder::new()
            .common_data(&self.common_data)
            .group_by(|_fe| ())
            .stop_flag(stop_flag)
            .progress_sender(progress_sender)
            .minimal_file_size(0)
            .maximal_file_size(0)
            .build()
            .run();

        match result {
            DirTraversalResult::SuccessFiles { grouped_file_entries, warnings } => {
                self.empty_files = grouped_file_entries.into_values().flatten().collect();
                self.information.number_of_empty_files = self.empty_files.len();
                self.common_data.text_messages.warnings.extend(warnings);

                debug!("Found {} empty files.", self.information.number_of_empty_files);

                WorkContinueStatus::Continue
            }

            DirTraversalResult::Stopped => WorkContinueStatus::Stop,
        }
    }
}

impl DeletingItems for EmptyFiles {
    #[fun_time(message = "delete_files", level = "debug")]
    fn delete_files(&mut self, stop_flag: &Arc<AtomicBool>, progress_sender: Option<&Sender<ProgressData>>) -> WorkContinueStatus {
        match self.common_data.delete_method {
            DeleteMethod::Delete => self.delete_simple_elements_and_add_to_messages(stop_flag, progress_sender, DeleteItemType::DeletingFiles(self.empty_files.clone())),
            DeleteMethod::None => WorkContinueStatus::Continue,
            _ => unreachable!(),
        }
    }
}

impl Default for EmptyFiles {
    fn default() -> Self {
        Self::new()
    }
}

impl DebugPrint for EmptyFiles {
    #[allow(clippy::print_stdout)]
    fn debug_print(&self) {
        if !cfg!(debug_assertions) {
            return;
        }
        println!("---------------DEBUG PRINT---------------");
        println!("Empty list size - {}", self.empty_files.len());
        self.debug_print_common();
        println!("-----------------------------------------");
    }
}

impl PrintResults for EmptyFiles {
    fn write_results<T: Write>(&self, writer: &mut T) -> std::io::Result<()> {
        writeln!(
            writer,
            "Results of searching {:?} with excluded directories {:?} and excluded items {:?}",
            self.common_data.directories.included_directories,
            self.common_data.directories.excluded_directories,
            self.common_data.excluded_items.get_excluded_items()
        )?;

        if !self.empty_files.is_empty() {
            writeln!(writer, "Found {} empty files.", self.information.number_of_empty_files)?;
            for file_entry in &self.empty_files {
                writeln!(writer, "\"{}\"", file_entry.path.to_string_lossy())?;
            }
        } else {
            write!(writer, "Not found any empty files.")?;
        }

        Ok(())
    }

    fn save_results_to_file_as_json(&self, file_name: &str, pretty_print: bool) -> std::io::Result<()> {
        self.save_results_to_file_as_json_internal(file_name, &self.empty_files, pretty_print)
    }
}

impl EmptyFiles {
    pub const fn get_empty_files(&self) -> &Vec<FileEntry> {
        &self.empty_files
    }

    pub const fn get_information(&self) -> &Info {
        &self.information
    }
}
