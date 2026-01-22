use std::io::Write;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::time::Instant;

use crossbeam_channel::Sender;
use fun_time::fun_time;

use crate::common::model::WorkContinueStatus;
use crate::common::progress_data::ProgressData;
use crate::common::tool_data::{CommonData, CommonToolData, DeleteItemType, DeleteMethod};
use crate::common::traits::{AllTraits, DebugPrint, DeletingItems, PrintResults, Search};
use crate::tools::bad_extensions::{BadExtensions, BadExtensionsParameters, Info};

impl AllTraits for BadExtensions {}

impl Search for BadExtensions {
    #[fun_time(message = "find_bad_extensions_files", level = "info")]
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
            if self.look_for_bad_extensions_files(stop_flag, progress_sender) == WorkContinueStatus::Stop {
                self.common_data.stopped_search = true;
            }
        })();

        self.information.scanning_time = start_time.elapsed();

        if !self.common_data.stopped_search {
            self.debug_print();
        }
    }
}

impl DeletingItems for BadExtensions {
    fn delete_files(&mut self, stop_flag: &Arc<AtomicBool>, progress_sender: Option<&Sender<ProgressData>>) -> WorkContinueStatus {
        match self.common_data.delete_method {
            DeleteMethod::Delete => self.delete_simple_elements_and_add_to_messages(stop_flag, progress_sender, DeleteItemType::DeletingFolders(self.bad_extensions_files.clone())),
            DeleteMethod::None => WorkContinueStatus::Continue,
            _ => unreachable!(),
        }
    }
}

impl DebugPrint for BadExtensions {
    #[expect(clippy::print_stdout)]
    fn debug_print(&self) {
        if !cfg!(debug_assertions) || cfg!(test) {
            return;
        }
        println!("---------------DEBUG PRINT---------------");
        self.debug_print_common();
        println!("-----------------------------------------");
    }
}

impl PrintResults for BadExtensions {
    fn write_results<T: Write>(&self, writer: &mut T) -> std::io::Result<()> {
        writeln!(
            writer,
            "Results of searching {:?} with excluded directories {:?} and excluded items {:?}",
            self.common_data.directories.included_directories,
            self.common_data.directories.excluded_directories,
            self.common_data.excluded_items.get_excluded_items()
        )?;
        writeln!(writer, "Found {} files with invalid extension.\n", self.information.number_of_files_with_bad_extension)?;

        for file_entry in &self.bad_extensions_files {
            writeln!(writer, "\"{}\" ----- {}", file_entry.path.to_string_lossy(), file_entry.proper_extensions_group)?;
        }

        Ok(())
    }

    fn save_results_to_file_as_json(&self, file_name: &str, pretty_print: bool) -> std::io::Result<()> {
        self.save_results_to_file_as_json_internal(file_name, &self.bad_extensions_files, pretty_print)
    }
}

impl CommonData for BadExtensions {
    type Info = Info;
    type Parameters = BadExtensionsParameters;

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
        self.get_information().number_of_files_with_bad_extension > 0
    }
}
