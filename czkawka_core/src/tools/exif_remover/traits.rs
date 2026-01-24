use std::io::Write;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::time::Instant;

use crossbeam_channel::Sender;
use fun_time::fun_time;
use humansize::BINARY;

use crate::common::consts::EXIF_FILES_EXTENSIONS;
use crate::common::model::WorkContinueStatus;
use crate::common::progress_data::ProgressData;
use crate::common::tool_data::{CommonData, CommonToolData, DeleteItemType, DeleteMethod};
use crate::common::traits::{AllTraits, DebugPrint, DeletingItems, FixingItems, PrintResults, Search};
use crate::tools::exif_remover::{ExifEntry, ExifRemover, ExifRemoverParameters, ExifTagsFixerParams, Info};

impl AllTraits for ExifRemover {}

impl DeletingItems for ExifRemover {
    #[fun_time(message = "delete_files", level = "debug")]
    fn delete_files(&mut self, stop_flag: &Arc<AtomicBool>, progress_sender: Option<&Sender<ProgressData>>) -> WorkContinueStatus {
        match self.common_data.delete_method {
            DeleteMethod::Delete => {
                let files_to_delete: Vec<ExifEntry> = self.exif_files.clone();
                self.delete_simple_elements_and_add_to_messages(stop_flag, progress_sender, DeleteItemType::DeletingFiles(files_to_delete))
            }
            DeleteMethod::None => WorkContinueStatus::Continue,
            _ => unreachable!(),
        }
    }
}

impl FixingItems for ExifRemover {
    type FixParams = ExifTagsFixerParams;
    #[fun_time(message = "fix_items", level = "debug")]
    fn fix_items(&mut self, stop_flag: &Arc<AtomicBool>, progress_sender: Option<&Sender<ProgressData>>, fix_params: Self::FixParams) -> WorkContinueStatus {
        self.fix_files(stop_flag, progress_sender, fix_params)
    }
}

impl DebugPrint for ExifRemover {
    #[expect(clippy::print_stdout)]
    fn debug_print(&self) {
        if !cfg!(debug_assertions) || cfg!(test) {
            return;
        }

        println!("### INDIVIDUAL DEBUG PRINT ###");
        println!("Info: {:?}", self.information);
        println!("Number of files with EXIF: {}", self.information.number_of_files_with_exif);
        self.debug_print_common();
        println!("-----------------------------------------");
    }
}

impl PrintResults for ExifRemover {
    fn write_results<T: Write>(&self, writer: &mut T) -> std::io::Result<()> {
        writeln!(
            writer,
            "Results of searching EXIF data with excluded directories {:?} and excluded items {:?}",
            self.common_data.directories.included_directories, self.common_data.directories.excluded_directories
        )?;

        if self.information.number_of_files_with_exif != 0 {
            writeln!(writer, "Found {} files with EXIF data.\n", self.information.number_of_files_with_exif)?;

            for exif_entry in &self.exif_files {
                writeln!(
                    writer,
                    "\nFile: \"{}\" - {} - {} - {:?}",
                    exif_entry.path.to_string_lossy(),
                    humansize::format_size(exif_entry.size, BINARY),
                    exif_entry.modified_date,
                    exif_entry.exif_tags.iter().map(|item_tag| item_tag.name.clone()).collect::<Vec<_>>()
                )?;
            }
        } else {
            writeln!(writer, "Not found any files with EXIF data.")?;
        }

        Ok(())
    }

    fn save_results_to_file_as_json(&self, file_name: &str, pretty_print: bool) -> std::io::Result<()> {
        self.save_results_to_file_as_json_internal(file_name, &self.exif_files, pretty_print)
    }
}

impl Search for ExifRemover {
    #[fun_time(message = "find_exif_data", level = "info")]
    fn search(&mut self, stop_flag: &Arc<AtomicBool>, progress_sender: Option<&Sender<ProgressData>>) {
        let start_time = Instant::now();

        let () = (|| {
            if self.prepare_items(Some(EXIF_FILES_EXTENSIONS)).is_err() {
                return;
            }
            if self.find_exif_files(stop_flag, progress_sender) == WorkContinueStatus::Stop {
                self.common_data.stopped_search = true;
                return;
            }

            if self.check_exif_in_files(stop_flag, progress_sender) == WorkContinueStatus::Stop {
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

impl CommonData for ExifRemover {
    type Info = Info;
    type Parameters = ExifRemoverParameters;

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
        self.information.number_of_files_with_exif > 0
    }
}
