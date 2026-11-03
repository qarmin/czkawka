use std::io::Write;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::time::Instant;

use crossbeam_channel::Sender;
use fun_time::fun_time;

use crate::common::model::WorkContinueStatus;
use crate::common::progress_data::ProgressData;
use crate::common::tool_data::{CommonData, CommonToolData};
use crate::common::traits::{AllTraits, DebugPrint, DeletingItems, PrintResults, Search};
use crate::tools::disk_usage::report::{build_report, write_report};
use crate::tools::disk_usage::{DiskUsage, DiskUsageParameters, Info};

impl AllTraits for DiskUsage {}

impl DeletingItems for DiskUsage {
    fn delete_files(&mut self, _stop_flag: &Arc<AtomicBool>, _progress_sender: Option<&Sender<ProgressData>>) -> WorkContinueStatus {
        WorkContinueStatus::Continue
    }
}

impl DebugPrint for DiskUsage {
    #[expect(clippy::print_stdout)]
    fn debug_print(&self) {
        if !cfg!(debug_assertions) || cfg!(test) {
            return;
        }

        println!("### INDIVIDUAL DEBUG PRINT ###");
        println!("Info: {:?}", self.information);
        println!("Params: {:?}", self.params);
        self.debug_print_common();
        println!("-----------------------------------------");
    }
}

impl PrintResults for DiskUsage {
    fn write_results<T: Write>(&self, writer: &mut T) -> std::io::Result<()> {
        self.write_base_search_paths(writer)?;

        if self.found_any_items() {
            writeln!(
                writer,
                "{} files in {} directories.\n",
                self.information.number_of_files, self.information.number_of_directories
            )?;
            write_report(writer, &build_report(&self.tree, self.params.report_depth, self.params.report_entries_per_directory))
        } else {
            writeln!(writer, "Not found any files.")
        }
    }

    fn save_results_to_file_as_json(&self, file_name: &str, pretty_print: bool) -> std::io::Result<()> {
        let report = build_report(&self.tree, self.params.report_depth, self.params.report_entries_per_directory);
        self.save_results_to_file_as_json_internal(file_name, &report, pretty_print)
    }
}

impl Search for DiskUsage {
    #[fun_time(message = "find_disk_usage", level = "info")]
    fn search(&mut self, stop_flag: &Arc<AtomicBool>, progress_sender: Option<&Sender<ProgressData>>) {
        let start_time = Instant::now();

        let () = (|| {
            if self.prepare_items(None).is_err() {
                return;
            }
            if self.scan_disk_usage(stop_flag, progress_sender) == WorkContinueStatus::Stop {
                self.common_data.stopped_search = true;
            }
        })();

        self.information.scanning_time = start_time.elapsed();

        if !self.common_data.stopped_search {
            self.debug_print();
        }
    }
}

impl CommonData for DiskUsage {
    type Info = Info;
    type Parameters = DiskUsageParameters;

    fn get_information(&self) -> Self::Info {
        self.information.clone()
    }
    fn get_params(&self) -> Self::Parameters {
        self.params
    }
    fn get_cd(&self) -> &CommonToolData {
        &self.common_data
    }
    fn get_cd_mut(&mut self) -> &mut CommonToolData {
        &mut self.common_data
    }
    fn found_any_items(&self) -> bool {
        self.information.number_of_files > 0
    }
}
