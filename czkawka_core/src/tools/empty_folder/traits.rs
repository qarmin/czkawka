use std::io::Write;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::time::Instant;

use crossbeam_channel::Sender;
use fun_time::fun_time;
use rayon::prelude::*;

use crate::common::model::WorkContinueStatus;
use crate::common::progress_data::ProgressData;
use crate::common::tool_data::{CommonData, CommonToolData, DeleteItemType, DeleteMethod};
use crate::common::traits::{AllTraits, DebugPrint, DeletingItems, PrintResults, Search};
use crate::tools::empty_folder::{EmptyFolder, Info};

impl AllTraits for EmptyFolder {}

impl Search for EmptyFolder {
    #[fun_time(message = "find_empty_folders", level = "info")]
    fn search(&mut self, stop_flag: &Arc<AtomicBool>, progress_sender: Option<&Sender<ProgressData>>) {
        let start_time = Instant::now();

        let () = (|| {
            if self.prepare_items(None).is_err() {
                return;
            }
            if self.check_for_empty_folders(stop_flag, progress_sender) == WorkContinueStatus::Stop {
                self.common_data.stopped_search = true;
                return;
            }
            self.optimize_folders();

            if self.delete_files(stop_flag, progress_sender) == WorkContinueStatus::Stop {
                self.common_data.stopped_search = true;
            }
        })();

        self.information.scanning_time = start_time.elapsed();

        if !self.common_data.stopped_search {
            self.debug_print();
        }
    }
}

impl DebugPrint for EmptyFolder {
    #[expect(clippy::print_stdout)]
    fn debug_print(&self) {
        if !cfg!(debug_assertions) || cfg!(test) {
            return;
        }

        println!("---------------DEBUG PRINT---------------");
        println!("Number of empty folders - {}", self.information.number_of_empty_folders);
        self.debug_print_common();
        println!("-----------------------------------------");
    }
}

impl PrintResults for EmptyFolder {
    fn write_results<T: Write>(&self, writer: &mut T) -> std::io::Result<()> {
        if !self.empty_folder_list.is_empty() {
            writeln!(writer, "--------------------------Empty folder list--------------------------")?;
            writeln!(writer, "Found {} empty folders", self.information.number_of_empty_folders)?;
            let mut empty_folder_list = self.empty_folder_list.keys().collect::<Vec<_>>();
            empty_folder_list.par_sort_unstable();
            for name in empty_folder_list {
                writeln!(writer, "{name}")?;
            }
        } else {
            write!(writer, "Not found any empty folders.")?;
        }

        Ok(())
    }

    fn save_results_to_file_as_json(&self, file_name: &str, pretty_print: bool) -> std::io::Result<()> {
        self.save_results_to_file_as_json_internal(file_name, &self.empty_folder_list.keys().collect::<Vec<_>>(), pretty_print)
    }
}

impl CommonData for EmptyFolder {
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
        self.information.number_of_empty_folders > 0
    }
}

impl DeletingItems for EmptyFolder {
    #[fun_time(message = "delete_files", level = "debug")]
    fn delete_files(&mut self, stop_flag: &Arc<AtomicBool>, progress_sender: Option<&Sender<ProgressData>>) -> WorkContinueStatus {
        match self.common_data.delete_method {
            DeleteMethod::Delete => self.delete_simple_elements_and_add_to_messages(
                stop_flag,
                progress_sender,
                DeleteItemType::DeletingFolders(self.empty_folder_list.values().cloned().collect::<Vec<_>>()),
            ),
            DeleteMethod::None => WorkContinueStatus::Continue,
            _ => unreachable!(),
        }
    }
}
