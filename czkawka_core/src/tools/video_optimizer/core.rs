use std::collections::BTreeMap;
use std::mem;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use crossbeam_channel::Sender;
use fun_time::fun_time;
use humansize::{BINARY, format_size};
use log::{debug, info};
use rayon::prelude::*;

use crate::common::cache::{extract_loaded_cache, load_cache_from_file_generalized_by_path, save_cache_to_file_generalized};
use crate::common::dir_traversal::{DirTraversalBuilder, DirTraversalResult};
use crate::common::model::{ToolType, WorkContinueStatus};
use crate::common::progress_data::{CurrentStage, ProgressData};
use crate::common::progress_stop_handler::{check_if_stop_received, prepare_thread_handler_common};
use crate::common::tool_data::{CommonData, CommonToolData};
use crate::tools::video_optimizer::{
    Info, VideoCropEntry, VideoCroppingMechanism, VideoOptimizer, VideoOptimizerFixParams, VideoOptimizerParameters, VideoTranscodeEntry, VideoTranscodeParams,
};

mod video_converter;
mod video_cropper;

pub use video_converter::process_video;
pub use video_cropper::fix_video_crop;

use crate::common::consts::VIDEO_FILES_EXTENSIONS;

pub const CACHE_VIDEO_TRANSCODE_VERSION: u8 = 11;
pub const CACHE_VIDEO_CROP_VERSION: u8 = 11;
pub const CACHE_IMAGE_TRIM_VERSION: u8 = 11;

impl VideoOptimizer {
    pub fn new(params: VideoOptimizerParameters) -> Self {
        Self {
            common_data: CommonToolData::new(ToolType::VideoOptimizer),
            information: Info::default(),
            video_transcode_entries: Default::default(),
            video_crop_entries: Default::default(),
            params,
        }
    }

    #[fun_time(message = "scan_files", level = "debug")]
    pub(crate) fn scan_files(&mut self, stop_flag: &Arc<AtomicBool>, progress_sender: Option<&Sender<ProgressData>>) -> WorkContinueStatus {
        let extensions_string = match &self.params {
            VideoOptimizerParameters::VideoTranscode(_) | VideoOptimizerParameters::VideoCrop(_) => VIDEO_FILES_EXTENSIONS.join(","),
        };
        self.common_data.extensions.set_allowed_extensions(extensions_string);

        let result = DirTraversalBuilder::new()
            .group_by(|_fe| ())
            .stop_flag(stop_flag)
            .progress_sender(progress_sender)
            .common_data(&self.common_data)
            .build()
            .run();

        match result {
            DirTraversalResult::SuccessFiles { grouped_file_entries, warnings } => {
                match &self.params {
                    VideoOptimizerParameters::VideoTranscode(_) => {
                        self.video_transcode_entries = grouped_file_entries.into_values().flatten().map(|fe| fe.into_video_transcode_entry()).collect();
                        info!("Found {} files to check", self.video_transcode_entries.len());
                    }
                    VideoOptimizerParameters::VideoCrop(_) => {
                        self.video_crop_entries = grouped_file_entries.into_values().flatten().map(|fe| fe.into_video_crop_entry()).collect();
                        info!("Found {} files to check", self.video_crop_entries.len());
                    }
                }

                self.common_data.text_messages.warnings.extend(warnings);

                WorkContinueStatus::Continue
            }
            DirTraversalResult::Stopped => WorkContinueStatus::Stop,
        }
    }

    #[fun_time(message = "check_files", level = "debug")]
    pub(crate) fn check_files(&mut self, stop_flag: &Arc<AtomicBool>, progress_sender: Option<&Sender<ProgressData>>) -> WorkContinueStatus {
        match self.params.clone() {
            VideoOptimizerParameters::VideoTranscode(params) => self.process_video_transcode(stop_flag, progress_sender, params),
            VideoOptimizerParameters::VideoCrop(_) => self.process_video_crop(stop_flag, progress_sender),
        }
    }

    #[fun_time(message = "process_video_transcode", level = "debug")]
    fn process_video_transcode(&mut self, stop_flag: &Arc<AtomicBool>, progress_sender: Option<&Sender<ProgressData>>, params: VideoTranscodeParams) -> WorkContinueStatus {
        if self.video_transcode_entries.is_empty() {
            return WorkContinueStatus::Continue;
        }

        let all_files: Vec<VideoTranscodeEntry> = std::mem::take(&mut self.video_transcode_entries);

        let (records_already_cached, non_cached_files_to_check) = self.load_video_transcode_cache(all_files);

        let progress_handler = prepare_thread_handler_common(
            progress_sender,
            CurrentStage::VideoOptimizerProcessingVideos,
            non_cached_files_to_check.len(),
            self.get_test_type(),
            non_cached_files_to_check.values().map(|entry| entry.size).sum(),
        );

        let mut entries: Vec<VideoTranscodeEntry> = non_cached_files_to_check
            .into_par_iter()
            .map(|(_path, entry)| {
                if check_if_stop_received(stop_flag) {
                    return None;
                }
                let size = entry.size;
                let res = video_converter::check_video(entry);
                progress_handler.increase_items(1);
                progress_handler.increase_size(size);
                Some(res)
            })
            .while_some()
            .collect();

        self.common_data.text_messages.warnings.extend(entries.iter().filter_map(|e| e.error.as_ref()).cloned());
        entries.extend(records_already_cached.into_values());

        progress_handler.join_thread();

        self.save_video_transcode_cache(&entries);

        entries.retain(|e| e.error.is_none() && !params.excluded_codecs.contains(&e.codec));

        self.video_transcode_entries = entries;

        WorkContinueStatus::Continue
    }

    #[fun_time(message = "process_video_crop", level = "debug")]
    fn process_video_crop(&mut self, stop_flag: &Arc<AtomicBool>, progress_sender: Option<&Sender<ProgressData>>) -> WorkContinueStatus {
        if self.video_crop_entries.is_empty() {
            return WorkContinueStatus::Continue;
        }

        let VideoOptimizerParameters::VideoCrop(params) = self.params.clone() else {
            unreachable!("process_video_crop called with non VideoCrop parameters, caller is responsible for that");
        };

        let all_files: Vec<VideoCropEntry> = std::mem::take(&mut self.video_crop_entries);

        let (records_already_cached, non_cached_files_to_check) = self.load_video_crop_cache(all_files, params.crop_detect);

        let progress_handler = prepare_thread_handler_common(
            progress_sender,
            CurrentStage::VideoOptimizerProcessingVideos,
            non_cached_files_to_check.len(),
            self.get_test_type(),
            non_cached_files_to_check.values().map(|entry| entry.size).sum(),
        );

        let mut entries: Vec<VideoCropEntry> = non_cached_files_to_check
            .into_par_iter()
            .map(|(_path, entry)| {
                if check_if_stop_received(stop_flag) {
                    return None;
                }
                let size = entry.size;
                let res = video_cropper::check_video_crop(entry, &params, stop_flag);
                progress_handler.increase_items(1);
                progress_handler.increase_size(size);
                res
            })
            .while_some()
            .collect();

        self.common_data.text_messages.warnings.extend(entries.iter().filter_map(|e| e.error.as_ref()).cloned());
        entries.extend(records_already_cached.into_values());

        progress_handler.join_thread();

        self.save_video_crop_cache(&entries, params.crop_detect);

        entries.retain(|e| e.error.is_none() && e.new_image_dimensions.is_some());

        self.video_crop_entries = entries;

        WorkContinueStatus::Continue
    }

    #[fun_time(message = "load_video_transcode_cache", level = "debug")]
    fn load_video_transcode_cache(&mut self, all_files: Vec<VideoTranscodeEntry>) -> (BTreeMap<String, VideoTranscodeEntry>, BTreeMap<String, VideoTranscodeEntry>) {
        let mut records_already_cached: BTreeMap<String, VideoTranscodeEntry> = Default::default();
        let mut non_cached_files_to_check: BTreeMap<String, VideoTranscodeEntry> = Default::default();

        let preliminary_files: BTreeMap<String, VideoTranscodeEntry> = all_files
            .into_iter()
            .map(|entry| {
                let path = entry.path.to_string_lossy().to_string();
                (path, entry)
            })
            .collect();

        if self.common_data.use_cache {
            let (messages, loaded_items) =
                load_cache_from_file_generalized_by_path::<VideoTranscodeEntry>(&get_video_transcode_cache_file(), self.get_delete_outdated_cache(), &preliminary_files);
            self.get_cd_mut().text_messages.messages.extend(messages.messages);
            self.get_cd_mut().text_messages.warnings.extend(messages.warnings);

            if let Some(loaded_items) = loaded_items {
                extract_loaded_cache(&loaded_items, preliminary_files, &mut records_already_cached, &mut non_cached_files_to_check);

                info!(
                    "load_video_transcode_cache - {}({}) non cached, {}({}) already cached",
                    non_cached_files_to_check.len(),
                    format_size(non_cached_files_to_check.values().map(|e| e.size).sum::<u64>(), BINARY),
                    records_already_cached.len(),
                    format_size(records_already_cached.values().map(|e| e.size).sum::<u64>(), BINARY),
                );
            } else {
                non_cached_files_to_check = preliminary_files;
            }
        } else {
            non_cached_files_to_check = preliminary_files;
        }

        (records_already_cached, non_cached_files_to_check)
    }

    #[fun_time(message = "load_video_crop_cache", level = "debug")]
    fn load_video_crop_cache(&mut self, all_files: Vec<VideoCropEntry>, params: VideoCroppingMechanism) -> (BTreeMap<String, VideoCropEntry>, BTreeMap<String, VideoCropEntry>) {
        let mut records_already_cached: BTreeMap<String, VideoCropEntry> = Default::default();
        let mut non_cached_files_to_check: BTreeMap<String, VideoCropEntry> = Default::default();

        let preliminary_files: BTreeMap<String, VideoCropEntry> = all_files
            .into_iter()
            .map(|entry| {
                let path = entry.path.to_string_lossy().to_string();
                (path, entry)
            })
            .collect();

        if self.common_data.use_cache {
            let (messages, loaded_items) =
                load_cache_from_file_generalized_by_path::<VideoCropEntry>(&get_video_crop_cache_file(params), self.get_delete_outdated_cache(), &preliminary_files);
            self.get_cd_mut().text_messages.messages.extend(messages.messages);
            self.get_cd_mut().text_messages.warnings.extend(messages.warnings);

            if let Some(loaded_items) = loaded_items {
                extract_loaded_cache(&loaded_items, preliminary_files, &mut records_already_cached, &mut non_cached_files_to_check);

                info!(
                    "load_video_crop_cache - {}({}) non cached, {}({}) already cached",
                    non_cached_files_to_check.len(),
                    format_size(non_cached_files_to_check.values().map(|e| e.size).sum::<u64>(), BINARY),
                    records_already_cached.len(),
                    format_size(records_already_cached.values().map(|e| e.size).sum::<u64>(), BINARY),
                );
            } else {
                non_cached_files_to_check = preliminary_files;
            }
        } else {
            non_cached_files_to_check = preliminary_files;
        }

        (records_already_cached, non_cached_files_to_check)
    }

    #[fun_time(message = "save_video_transcode_cache", level = "debug")]
    fn save_video_transcode_cache(&mut self, entries: &[VideoTranscodeEntry]) {
        if self.common_data.use_cache {
            let entries_map: BTreeMap<String, VideoTranscodeEntry> = entries.iter().map(|entry| (entry.path.to_string_lossy().to_string(), entry.clone())).collect();
            let messages = save_cache_to_file_generalized(&get_video_transcode_cache_file(), &entries_map, self.get_save_also_as_json(), 0);
            self.get_cd_mut().text_messages.messages.extend(messages.messages);
            self.get_cd_mut().text_messages.warnings.extend(messages.warnings);
        }
    }

    #[fun_time(message = "save_video_crop_cache", level = "debug")]
    fn save_video_crop_cache(&mut self, entries: &[VideoCropEntry], video_cropping_mechanism: VideoCroppingMechanism) {
        if self.common_data.use_cache {
            let entries_map: BTreeMap<String, VideoCropEntry> = entries.iter().map(|entry| (entry.path.to_string_lossy().to_string(), entry.clone())).collect();
            let messages = save_cache_to_file_generalized(&get_video_crop_cache_file(video_cropping_mechanism), &entries_map, self.get_save_also_as_json(), 0);
            self.get_cd_mut().text_messages.messages.extend(messages.messages);
            self.get_cd_mut().text_messages.warnings.extend(messages.warnings);
        }
    }

    #[fun_time(message = "fix_files", level = "debug")]
    pub(crate) fn fix_files(&mut self, stop_flag: &Arc<AtomicBool>, _progress_sender: Option<&Sender<ProgressData>>, fix_params: VideoOptimizerFixParams) -> WorkContinueStatus {
        match self.params.clone() {
            VideoOptimizerParameters::VideoTranscode(_) => {
                info!("Starting optimization of {} video files", self.video_transcode_entries.len());

                let VideoOptimizerFixParams::VideoTranscode(video_transcode_params) = fix_params else {
                    unreachable!("VideoTranscode mode should have VideoTranscode fix_params(caller is responsible for that)");
                };

                // TODO this should use same mechanism as deleting files - this currently do not save progress to CLI
                self.video_transcode_entries = mem::take(&mut self
                    .video_transcode_entries)
                    .into_par_iter()
                    .map(|mut entry| {
                        if check_if_stop_received(stop_flag) {
                            return None;
                        }

                        match process_video(stop_flag, &entry.path.to_string_lossy(), entry.size, video_transcode_params) {
                            Ok(_new_size) => {}
                            Err(e) => {
                                entry.error = Some(e); // TODO
                            }
                        }

                        Some(entry)
                    })
                    .while_some()
                    .collect();

                // TODO save errors/warnings to text messages

                let successful_files = self.video_transcode_entries.iter().filter(|e| e.error.is_none() && !e.codec.is_empty()).count();
                let failed_files = self.video_transcode_entries.iter().filter(|e| e.error.is_some()).count();

                self.information.number_of_processed_files = successful_files;
                self.information.number_of_failed_files = failed_files;

                debug!("Optimization complete - Processed: {successful_files}, Failed: {failed_files}");
            }
            VideoOptimizerParameters::VideoCrop(_) => {
                // TODO: Implement video cropping logic
                info!("Video crop mode - logic not yet implemented for {} files", self.video_crop_entries.len());
            }
        }

        WorkContinueStatus::Continue
    }
}

pub fn get_video_transcode_cache_file() -> String {
    format!("cache_video_transcode_{CACHE_VIDEO_TRANSCODE_VERSION}.bin")
}

pub fn get_video_crop_cache_file(vide_cropping_mechanism: VideoCroppingMechanism) -> String {
    format!("cache_video_crop_{CACHE_VIDEO_CROP_VERSION}_{vide_cropping_mechanism:?}.bin")
}
