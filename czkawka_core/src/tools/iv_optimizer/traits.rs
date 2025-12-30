use std::io::Write;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::time::Instant;

use crossbeam_channel::Sender;
use fun_time::fun_time;
use humansize::{BINARY, format_size};

use crate::common::model::WorkContinueStatus;
use crate::common::progress_data::ProgressData;
use crate::common::tool_data::{CommonData, CommonToolData};
use crate::common::traits::{AllTraits, DebugPrint, DeletingItems, FixingItems, PrintResults, Search};
use crate::tools::iv_optimizer::{IVOptimizer, IVOptimizerParameters, Info, OptimizerMode};

impl AllTraits for IVOptimizer {}

impl DeletingItems for IVOptimizer {
    #[fun_time(message = "delete_files", level = "debug")]
    fn delete_files(&mut self, _stop_flag: &Arc<AtomicBool>, _progress_sender: Option<&Sender<ProgressData>>) -> WorkContinueStatus {
        unreachable!("IVOptimizer does not support deleting files");
    }
}

impl FixingItems for IVOptimizer {
    #[fun_time(message = "fix_items", level = "debug")]
    fn fix_items(&mut self, stop_flag: &Arc<AtomicBool>, progress_sender: Option<&Sender<ProgressData>>) -> WorkContinueStatus {
        self.fix_files(stop_flag, progress_sender)
    }
}

impl DebugPrint for IVOptimizer {
    #[expect(clippy::print_stdout)]
    fn debug_print(&self) {
        if !cfg!(debug_assertions) || cfg!(test) {
            return;
        }

        println!("### INDIVIDUAL DEBUG PRINT ###");
        println!("Info: {:?}", self.information);
        println!("Mode: {:?}", self.params.mode);
        println!("Video transcode entries: {}", self.video_transcode_entries.len());
        println!("Image trim entries: {}", self.image_trim_entries.len());
        self.debug_print_common();
        println!("-----------------------------------------");
    }
}

impl PrintResults for IVOptimizer {
    fn write_results<T: Write>(&self, writer: &mut T) -> std::io::Result<()> {
        writeln!(writer, "Results of IV Optimizer with mode {:?}", self.params.mode)?;
        writeln!(writer, "Searched in directories: {:?}", self.common_data.directories.included_directories)?;
        writeln!(writer, "Excluded directories: {:?}", self.common_data.directories.excluded_directories)?;

        match self.params.mode {
            OptimizerMode::VideoTranscode { codec, quality } => {
                writeln!(writer, "Target codec: {codec:?}")?;
                writeln!(writer, "Target quality (CRF): {quality}")?;
                writeln!(writer)?;

                let total_entries = self.video_transcode_entries.len();
                let entries_needing_optimization = self.video_transcode_entries.iter().filter(|e| !e.codec.is_empty() && e.error.is_none()).count();
                let failed_entries = self.video_transcode_entries.iter().filter(|e| e.error.is_some()).count();

                writeln!(writer, "Total files found: {total_entries}")?;
                writeln!(writer, "Files needing optimization: {entries_needing_optimization}")?;
                writeln!(writer, "Failed to analyze: {failed_entries}")?;
                writeln!(writer)?;

                for entry in &self.video_transcode_entries {
                    if !entry.codec.is_empty() {
                        if let Some(err) = &entry.error {
                            writeln!(writer, "[FAILED] {} - Codec: {} - Error: {}", entry.path.display(), entry.codec, err)?;
                        } else if let Some(dimensions) = &entry.dimensions {
                            writeln!(
                                writer,
                                "[CANDIDATE] {} - Codec: {} - Dimensions: {} - Size: {}",
                                entry.path.display(),
                                entry.codec,
                                dimensions,
                                format_size(entry.size, BINARY)
                            )?;
                        } else {
                            writeln!(
                                writer,
                                "[CANDIDATE] {} - Codec: {} - Size: {}",
                                entry.path.display(),
                                entry.codec,
                                format_size(entry.size, BINARY)
                            )?;
                        }
                    }
                }
            }
            OptimizerMode::ImageTrim { threshold } => {
                writeln!(writer, "Trim threshold: {threshold}")?;
                writeln!(writer)?;

                let total_entries = self.image_trim_entries.len();
                let entries_needing_optimization = self.image_trim_entries.iter().filter(|e| e.bounding_box.is_some() && e.error.is_none()).count();
                let failed_entries = self.image_trim_entries.iter().filter(|e| e.error.is_some()).count();

                writeln!(writer, "Total files found: {total_entries}")?;
                writeln!(writer, "Files needing optimization: {entries_needing_optimization}")?;
                writeln!(writer, "Failed to analyze: {failed_entries}")?;
                writeln!(writer)?;

                for entry in &self.image_trim_entries {
                    if let Some(bb) = &entry.bounding_box {
                        if let Some(err) = &entry.error {
                            writeln!(writer, "[FAILED] {} - Error: {}", entry.path.display(), err)?;
                        } else {
                            writeln!(
                                writer,
                                "[CANDIDATE] {} - Will trim to {}x{} - Size: {}",
                                entry.path.display(),
                                bb.right - bb.left,
                                bb.bottom - bb.top,
                                format_size(entry.size, BINARY)
                            )?;
                        }
                    } else if let Some(err) = &entry.error {
                        writeln!(writer, "[FAILED] {} - Error: {}", entry.path.display(), err)?;
                    }
                }
            }
        }

        Ok(())
    }

    fn save_results_to_file_as_json(&self, file_name: &str, pretty_print: bool) -> std::io::Result<()> {
        match self.params.mode {
            OptimizerMode::VideoTranscode { .. } => self.save_results_to_file_as_json_internal(file_name, &self.video_transcode_entries, pretty_print),
            OptimizerMode::ImageTrim { .. } => self.save_results_to_file_as_json_internal(file_name, &self.image_trim_entries, pretty_print),
        }
    }
}

impl Search for IVOptimizer {
    #[fun_time(message = "scan_media_files", level = "info")]
    fn search(&mut self, stop_flag: &Arc<AtomicBool>, progress_sender: Option<&Sender<ProgressData>>) {
        let start_time = Instant::now();

        let () = {
            self.prepare_items();
            if self.scan_files(stop_flag, progress_sender) == WorkContinueStatus::Stop {
                self.common_data.stopped_search = true;
            }
        };

        self.information.scanning_time = start_time.elapsed();

        if !self.common_data.stopped_search {
            self.debug_print();
        }
    }
}

impl CommonData for IVOptimizer {
    type Info = Info;
    type Parameters = IVOptimizerParameters;

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
        self.information.number_of_failed_files > 0
    }
}
