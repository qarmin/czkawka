use std::io::Write;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use crossbeam_channel::Sender;
use fun_time::fun_time;

use crate::common::model::WorkContinueStatus;
use crate::common::progress_data::ProgressData;
use crate::common::tool_data::{CommonData, CommonToolData, DeleteItemType, DeleteMethod};
use crate::common::traits::{AllTraits, DebugPrint, DeletingItems, PrintResults, Search};
use crate::tools::broken_files::{BrokenFiles, BrokenFilesParameters, Info};

impl AllTraits for BrokenFiles {}

impl Search for BrokenFiles {
    #[fun_time(message = "find_broken_files", level = "info")]
    fn search(&mut self, stop_flag: &Arc<AtomicBool>, progress_sender: Option<&Sender<ProgressData>>) {
        self.prepare_items();
        if self.check_files(stop_flag, progress_sender) == WorkContinueStatus::Stop {
            self.common_data.stopped_search = true;
            return;
        }
        if self.look_for_broken_files(stop_flag, progress_sender) == WorkContinueStatus::Stop {
            self.common_data.stopped_search = true;
            return;
        }
        if self.delete_files(stop_flag, progress_sender) == WorkContinueStatus::Stop {
            self.common_data.stopped_search = true;
            return;
        }
        self.debug_print();
    }
}

impl DebugPrint for BrokenFiles {
    fn debug_print(&self) {
        if !cfg!(debug_assertions) || cfg!(test) {
            return;
        }
        self.debug_print_common();
    }
}

impl PrintResults for BrokenFiles {
    fn write_results<T: Write>(&self, writer: &mut T) -> std::io::Result<()> {
        writeln!(
            writer,
            "Results of searching {:?} with excluded directories {:?} and excluded items {:?}",
            self.common_data.directories.included_directories,
            self.common_data.directories.excluded_directories,
            self.common_data.excluded_items.get_excluded_items()
        )?;

        if !self.broken_files.is_empty() {
            writeln!(writer, "Found {} broken files.", self.information.number_of_broken_files)?;
            for file_entry in &self.broken_files {
                writeln!(writer, "\"{}\" - {}", file_entry.path.to_string_lossy(), file_entry.error_string)?;
            }
        } else {
            write!(writer, "Not found any broken files.")?;
        }

        Ok(())
    }

    fn save_results_to_file_as_json(&self, file_name: &str, pretty_print: bool) -> std::io::Result<()> {
        self.save_results_to_file_as_json_internal(file_name, &self.broken_files, pretty_print)
    }
}
impl DeletingItems for BrokenFiles {
    #[fun_time(message = "delete_files", level = "debug")]
    fn delete_files(&mut self, stop_flag: &Arc<AtomicBool>, progress_sender: Option<&Sender<ProgressData>>) -> WorkContinueStatus {
        match self.common_data.delete_method {
            DeleteMethod::Delete => self.delete_simple_elements_and_add_to_messages(stop_flag, progress_sender, DeleteItemType::DeletingFiles(self.broken_files.clone())),
            DeleteMethod::None => WorkContinueStatus::Continue,
            _ => unreachable!(),
        }
    }
}

impl CommonData for BrokenFiles {
    type Info = Info;
    type Parameters = BrokenFilesParameters;

    fn get_information(&self) -> Self::Info {
        self.information.clone()
    }
    fn get_params(&self) -> Self::Parameters {
        self.params.clone()
    }
    fn get_cd(&self) -> &CommonToolData {
        &self.common_data
    }
    fn get_cd_mut(&mut self) -> &mut CommonToolData {
        &mut self.common_data
    }
    fn found_any_broken_files(&self) -> bool {
        self.information.number_of_broken_files > 0
    }
}
