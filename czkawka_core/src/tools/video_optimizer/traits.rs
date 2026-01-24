use std::io::Write;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::time::Instant;

use crossbeam_channel::Sender;
use fun_time::fun_time;
use humansize::{BINARY, format_size};

use crate::common::consts::VIDEO_FILES_EXTENSIONS;
use crate::common::ffmpeg_utils::check_if_ffprobe_ffmpeg_exists;
use crate::common::model::WorkContinueStatus;
use crate::common::progress_data::ProgressData;
use crate::common::tool_data::{CommonData, CommonToolData};
use crate::common::traits::{AllTraits, DebugPrint, DeletingItems, FixingItems, PrintResults, Search};
use crate::flc;
use crate::tools::video_optimizer::{Info, VideoOptimizer, VideoOptimizerFixParams, VideoOptimizerParameters};

impl AllTraits for VideoOptimizer {}

impl DeletingItems for VideoOptimizer {
    #[fun_time(message = "delete_files", level = "debug")]
    fn delete_files(&mut self, _stop_flag: &Arc<AtomicBool>, _progress_sender: Option<&Sender<ProgressData>>) -> WorkContinueStatus {
        unreachable!("VideoOptimizer does not support deleting files");
    }
}

impl FixingItems for VideoOptimizer {
    type FixParams = VideoOptimizerFixParams;
    #[fun_time(message = "fix_items", level = "debug")]
    fn fix_items(&mut self, stop_flag: &Arc<AtomicBool>, progress_sender: Option<&Sender<ProgressData>>, fix_params: Self::FixParams) -> WorkContinueStatus {
        self.fix_files(stop_flag, progress_sender, fix_params)
    }
}

impl DebugPrint for VideoOptimizer {
    #[expect(clippy::print_stdout)]
    fn debug_print(&self) {
        if !cfg!(debug_assertions) || cfg!(test) {
            return;
        }

        println!("### INDIVIDUAL DEBUG PRINT ###");
        println!("Info: {:?}", self.information);
        println!("Mode: {:?}", self.params);
        println!("Video transcode entries: {}", self.video_transcode_result_entries.len());
        println!("Video crop entries: {}", self.video_crop_result_entries.len());
        self.debug_print_common();
        println!("-----------------------------------------");
    }
}

impl PrintResults for VideoOptimizer {
    fn write_results<T: Write>(&self, writer: &mut T) -> std::io::Result<()> {
        writeln!(writer, "Results of Video Optimizer with mode {:?}", self.params)?;
        writeln!(writer, "Searched in directories: {:?}", self.common_data.directories.included_directories)?;
        writeln!(writer, "Excluded directories: {:?}", self.common_data.directories.excluded_directories)?;

        match self.params.clone() {
            VideoOptimizerParameters::VideoTranscode(_) => {
                writeln!(writer)?;

                let total_entries = self.video_transcode_result_entries.len();
                let entries_needing_optimization = self.video_transcode_result_entries.iter().filter(|e| !e.codec.is_empty() && e.error.is_none()).count();
                let failed_entries = self.video_transcode_result_entries.iter().filter(|e| e.error.is_some()).count();

                writeln!(writer, "Total files found: {total_entries}")?;
                writeln!(writer, "Files needing optimization: {entries_needing_optimization}")?;
                writeln!(writer, "Failed to analyze: {failed_entries}")?;
                writeln!(writer)?;

                for entry in &self.video_transcode_result_entries {
                    if !entry.codec.is_empty() {
                        writeln!(
                            writer,
                            "\"{}\" - Codec: {} - Dimensions: {}x{} - Size: {}",
                            entry.path.to_string_lossy(),
                            entry.codec,
                            entry.width,
                            entry.height,
                            format_size(entry.size, BINARY)
                        )?;
                    }
                }
            }
            VideoOptimizerParameters::VideoCrop(_) => {
                writeln!(writer)?;

                let total_entries = self.video_crop_result_entries.len();
                let entries_with_crop_info = self.video_crop_result_entries.iter().filter(|e| !e.codec.is_empty() && e.error.is_none()).count();
                let failed_entries = self.video_crop_result_entries.iter().filter(|e| e.error.is_some()).count();

                writeln!(writer, "Total files found: {total_entries}")?;
                writeln!(writer, "Files with crop information: {entries_with_crop_info}")?;
                writeln!(writer, "Failed to analyze: {failed_entries}")?;
                writeln!(writer)?;

                for entry in &self.video_crop_result_entries {
                    if !entry.codec.is_empty() {
                        let new_image_dimensions: String = if let Some((lt, rt, rb, lb)) = entry.new_image_dimensions {
                            format!("  New dimensions: LT:{lt}, RT:{rt}, RB:{rb}, LB:{lb}")
                        } else {
                            String::new()
                        };
                        writeln!(
                            writer,
                            "{} - Codec: {} - Dimensions: {}x{} - Size: {}{new_image_dimensions}",
                            entry.path.display(),
                            entry.codec,
                            entry.width,
                            entry.height,
                            format_size(entry.size, BINARY)
                        )?;
                    }
                }
            }
        }

        Ok(())
    }

    fn save_results_to_file_as_json(&self, file_name: &str, pretty_print: bool) -> std::io::Result<()> {
        match &self.params {
            VideoOptimizerParameters::VideoTranscode(_) => self.save_results_to_file_as_json_internal(file_name, &self.video_transcode_result_entries, pretty_print),
            VideoOptimizerParameters::VideoCrop(_) => self.save_results_to_file_as_json_internal(file_name, &self.video_crop_result_entries, pretty_print),
        }
    }
}

impl Search for VideoOptimizer {
    #[fun_time(message = "scan_media_files", level = "info")]
    fn search(&mut self, stop_flag: &Arc<AtomicBool>, progress_sender: Option<&Sender<ProgressData>>) {
        let start_time = Instant::now();

        let () = (|| {
            if !check_if_ffprobe_ffmpeg_exists() {
                self.common_data.text_messages.critical = Some(flc!("core_ffmpeg_not_found"));
                #[cfg(target_os = "windows")]
                self.common_data.text_messages.errors.push(flc!("core_ffmpeg_not_found_windows"));
                return;
            }

            if self.prepare_items(Some(VIDEO_FILES_EXTENSIONS)).is_err() {
                return;
            }
            if self.scan_files(stop_flag, progress_sender) == WorkContinueStatus::Stop {
                self.common_data.stopped_search = true;
                return;
            }
            if self.check_files(stop_flag, progress_sender) == WorkContinueStatus::Stop {
                self.common_data.stopped_search = true;
            }
        })();

        self.information.scanning_time = start_time.elapsed();

        if !self.common_data.stopped_search {
            self.debug_print();
        }
    }
}

impl CommonData for VideoOptimizer {
    type Info = Info;
    type Parameters = VideoOptimizerParameters;

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
