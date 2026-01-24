use std::io::prelude::*;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::time::Instant;

use crossbeam_channel::Sender;
use fun_time::fun_time;

use crate::common::model::WorkContinueStatus;
use crate::common::progress_data::ProgressData;
use crate::common::tool_data::{CommonData, CommonToolData, DeleteItemType, DeleteMethod};
use crate::common::traits::{AllTraits, DebugPrint, DeletingItems, PrintResults, Search};
use crate::tools::invalid_symlinks::{ErrorType, Info, InvalidSymlinks};

impl AllTraits for InvalidSymlinks {}

impl Search for InvalidSymlinks {
    #[fun_time(message = "find_invalid_links", level = "info")]
    fn search(&mut self, stop_flag: &Arc<AtomicBool>, progress_sender: Option<&Sender<ProgressData>>) {
        let start_time = Instant::now();

        let () = (|| {
            if self.prepare_items(None).is_err() {
                return;
            }
            if self.check_files(stop_flag, progress_sender) == WorkContinueStatus::Stop {
                self.common_data.stopped_search = true;
                return;
            }
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

impl DebugPrint for InvalidSymlinks {
    #[expect(clippy::print_stdout)]
    fn debug_print(&self) {
        if !cfg!(debug_assertions) || cfg!(test) {
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
        self.information.number_of_invalid_symlinks > 0
    }
}
impl DeletingItems for InvalidSymlinks {
    #[fun_time(message = "delete_files", level = "debug")]
    fn delete_files(&mut self, stop_flag: &Arc<AtomicBool>, progress_sender: Option<&Sender<ProgressData>>) -> WorkContinueStatus {
        match self.common_data.delete_method {
            DeleteMethod::Delete => self.delete_simple_elements_and_add_to_messages(stop_flag, progress_sender, DeleteItemType::DeletingFiles(self.invalid_symlinks.clone())),
            DeleteMethod::None => WorkContinueStatus::Continue,
            _ => unreachable!(),
        }
    }
}
