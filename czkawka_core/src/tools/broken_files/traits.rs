use std::io::Write;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::time::Instant;

use crossbeam_channel::Sender;
use fun_time::fun_time;

use crate::common::consts::{AUDIO_FILES_EXTENSIONS, IMAGE_RS_BROKEN_FILES_EXTENSIONS, PDF_FILES_EXTENSIONS, VIDEO_FILES_EXTENSIONS, ZIP_FILES_EXTENSIONS};
use crate::common::ffmpeg_utils::check_if_ffprobe_ffmpeg_exists;
use crate::common::model::WorkContinueStatus;
use crate::common::progress_data::ProgressData;
use crate::common::tool_data::{CommonData, CommonToolData, DeleteItemType, DeleteMethod};
use crate::common::traits::{AllTraits, DebugPrint, DeletingItems, PrintResults, Search};
use crate::flc;
use crate::tools::broken_files::{BrokenFiles, BrokenFilesParameters, CheckedTypes, Info};

impl AllTraits for BrokenFiles {}

impl Search for BrokenFiles {
    #[fun_time(message = "find_broken_files", level = "info")]
    fn search(&mut self, stop_flag: &Arc<AtomicBool>, progress_sender: Option<&Sender<ProgressData>>) {
        let start_time = Instant::now();

        let () = (|| {
            if self.params.checked_types.contains(CheckedTypes::VIDEO) && !check_if_ffprobe_ffmpeg_exists() {
                self.common_data.text_messages.critical = Some(flc!("core_ffmpeg_not_found"));
                #[cfg(target_os = "windows")]
                self.common_data.text_messages.errors.push(flc!("core_ffmpeg_not_found_windows"));
                return;
            }

            let extension_types = [
                (CheckedTypes::PDF, PDF_FILES_EXTENSIONS),
                (CheckedTypes::AUDIO, AUDIO_FILES_EXTENSIONS),
                (CheckedTypes::ARCHIVE, ZIP_FILES_EXTENSIONS),
                (CheckedTypes::IMAGE, IMAGE_RS_BROKEN_FILES_EXTENSIONS),
                (CheckedTypes::VIDEO, VIDEO_FILES_EXTENSIONS),
            ];
            let extensions = extension_types
                .into_iter()
                .filter(|(checked_type, _)| self.get_params().checked_types.contains(*checked_type))
                .flat_map(|(_, exts)| exts.to_vec())
                .collect::<Vec<&str>>();

            if extensions.is_empty() {
                self.common_data.text_messages.critical = Some(flc!("core_needs_to_set_at_least_one_broken_option"));
                return;
            }

            if self.prepare_items(Some(&extensions)).is_err() {
                return;
            }
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
            }
        })();

        self.information.scanning_time = start_time.elapsed();

        if !self.common_data.stopped_search {
            self.debug_print();
        }
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
