use std::io::Write;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::time::Instant;

use crossbeam_channel::Sender;
use fun_time::fun_time;

use crate::common::model::WorkContinueStatus;
use crate::common::progress_data::ProgressData;
use crate::common::tool_data::{CommonData, CommonToolData, DeleteItemType, DeleteMethod};
use crate::common::traits::{AllTraits, DebugPrint, DeletingItems, FixingItems, PrintResults, Search};
use crate::flc;
use crate::tools::bad_names::{BadNames, BadNamesParameters, Info, NameFixerParams};

impl AllTraits for BadNames {}

impl Search for BadNames {
    #[fun_time(message = "find_bad_names", level = "info")]
    fn search(&mut self, stop_flag: &Arc<AtomicBool>, progress_sender: Option<&Sender<ProgressData>>) {
        let start_time = Instant::now();

        let () = (|| {
            if self.params.checked_issues.is_empty() {
                self.common_data.text_messages.critical = Some(flc!("core_needs_to_set_at_least_one_bad_name_option"));
                return;
            }

            if self.prepare_items(None).is_err() {
                return;
            }
            if self.check_files(stop_flag, progress_sender) == WorkContinueStatus::Stop {
                self.common_data.stopped_search = true;
                return;
            }
            if self.look_for_bad_names_files(stop_flag, progress_sender) == WorkContinueStatus::Stop {
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

impl DebugPrint for BadNames {
    fn debug_print(&self) {
        if !cfg!(debug_assertions) || cfg!(test) {
            return;
        }
        self.debug_print_common();
    }
}

impl PrintResults for BadNames {
    fn write_results<T: Write>(&self, writer: &mut T) -> std::io::Result<()> {
        writeln!(
            writer,
            "Results of searching {:?} with excluded directories {:?} and excluded items {:?}",
            self.common_data.directories.included_directories,
            self.common_data.directories.excluded_directories,
            self.common_data.excluded_items.get_excluded_items()
        )?;

        if !self.bad_names_files.is_empty() {
            writeln!(writer, "Found {} files with bad names.", self.information.number_of_files_with_bad_names)?;
            for file_entry in &self.bad_names_files {
                writeln!(
                    writer,
                    "\"{}\" -> \"{}\"",
                    file_entry.path.to_string_lossy(),
                    file_entry.new_name
                )?;
            }
        } else {
            write!(writer, "Not found any files with bad names.")?;
        }

        Ok(())
    }

    fn save_results_to_file_as_json(&self, file_name: &str, pretty_print: bool) -> std::io::Result<()> {
        self.save_results_to_file_as_json_internal(file_name, &self.bad_names_files, pretty_print)
    }
}

impl DeletingItems for BadNames {
    #[fun_time(message = "delete_files", level = "debug")]
    fn delete_files(&mut self, stop_flag: &Arc<AtomicBool>, progress_sender: Option<&Sender<ProgressData>>) -> WorkContinueStatus {
        match self.common_data.delete_method {
            DeleteMethod::Delete => self.delete_simple_elements_and_add_to_messages(stop_flag, progress_sender, DeleteItemType::DeletingFiles(self.bad_names_files.clone())),
            DeleteMethod::None => WorkContinueStatus::Continue,
            _ => unreachable!(),
        }
    }
}

impl FixingItems for BadNames {
    type FixParams = NameFixerParams;
    #[fun_time(message = "fix_items", level = "debug")]
    fn fix_items(&mut self, stop_flag: &Arc<AtomicBool>, _progress_sender: Option<&Sender<ProgressData>>, fix_params: Self::FixParams) -> WorkContinueStatus {
        self.fix_bad_names(fix_params, stop_flag)
    }
}

impl CommonData for BadNames {
    type Info = Info;
    type Parameters = BadNamesParameters;

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
        self.information.number_of_files_with_bad_names > 0
    }
}
