use std::collections::BTreeMap;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use crossbeam_channel::Sender;
use fun_time::fun_time;
use humansize::{BINARY, format_size};
use log::{debug, info};
use rayon::prelude::*;

use crate::common::cache::{extract_loaded_cache, load_cache_from_file_generalized_by_path, save_cache_to_file_generalized};
use crate::common::dir_traversal::{DirTraversalBuilder, DirTraversalResult};
use crate::common::model::{FileEntry, ToolType, WorkContinueStatus};
use crate::common::progress_data::{CurrentStage, ProgressData};
use crate::common::progress_stop_handler::{check_if_stop_received, prepare_thread_handler_common};
use crate::common::tool_data::{CommonData, CommonToolData};
use crate::tools::video_optimizer::{Info, OptimizerMode, VideoCodec, VideoOptimizer, VideoOptimizerParameters, VideoTranscodeEntry};

mod video_converter;

// Re-export public functions for GUI usage
pub use video_converter::process_video;

use crate::common::consts::VIDEO_FILES_EXTENSIONS;

pub const CACHE_VIDEO_TRANSCODE_VERSION: u8 = 1;
pub const CACHE_IMAGE_TRIM_VERSION: u8 = 1;

impl VideoOptimizer {
    pub fn new(params: VideoOptimizerParameters) -> Self {
        Self {
            common_data: CommonToolData::new(ToolType::VideoOptimizer),
            information: Info::default(),
            video_transcode_entries: Default::default(),
            params,
        }
    }

    #[fun_time(message = "scan_files", level = "debug")]
    pub(crate) fn scan_files(&mut self, stop_flag: &Arc<AtomicBool>, progress_sender: Option<&Sender<ProgressData>>) -> WorkContinueStatus {
        let extensions_string = match self.params.mode {
            OptimizerMode::VideoTranscode { .. } => VIDEO_FILES_EXTENSIONS.join(","),
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
                let all_files: Vec<FileEntry> = grouped_file_entries.into_values().flatten().collect();

                info!("Found {} files to check", all_files.len());

                match self.params.mode {
                    OptimizerMode::VideoTranscode { codec, quality } => {
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

                        entries.extend(records_already_cached.into_values());

                        progress_handler.join_thread();

                        if check_if_stop_received(stop_flag) {
                            return WorkContinueStatus::Stop;
                        }

                        self.save_video_transcode_cache(&entries, codec, quality);

                        let mut disallowed_codecs = self.params.excluded_codecs.clone();
                        disallowed_codecs.push(codec.as_ffprobe_codec_name().to_string());
                        entries.retain(|e| e.error.is_none() && !disallowed_codecs.contains(&e.codec));

                        self.video_transcode_entries = entries;
                    }
                }

                self.common_data.text_messages.warnings.extend(warnings);

                WorkContinueStatus::Continue
            }
            DirTraversalResult::Stopped => WorkContinueStatus::Stop,
        }
    }

    #[fun_time(message = "load_video_transcode_cache", level = "debug")]
    fn load_video_transcode_cache(&mut self, all_files: Vec<FileEntry>) -> (BTreeMap<String, VideoTranscodeEntry>, BTreeMap<String, VideoTranscodeEntry>) {
        let mut records_already_cached: BTreeMap<String, VideoTranscodeEntry> = Default::default();
        let mut non_cached_files_to_check: BTreeMap<String, VideoTranscodeEntry> = Default::default();

        let preliminary_files: BTreeMap<String, VideoTranscodeEntry> = all_files
            .into_iter()
            .map(|file_entry| {
                let path = file_entry.path.to_string_lossy().to_string();
                (path, file_entry.into_video_transcode_entry())
            })
            .collect();

        if self.common_data.use_cache {
            let OptimizerMode::VideoTranscode { codec, quality } = self.params.mode;
            let (messages, loaded_items) = load_cache_from_file_generalized_by_path::<VideoTranscodeEntry>(
                &get_video_transcode_cache_file(&codec, quality),
                self.get_delete_outdated_cache(),
                &preliminary_files,
            );
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

    #[fun_time(message = "save_video_transcode_cache", level = "debug")]
    fn save_video_transcode_cache(&mut self, entries: &[VideoTranscodeEntry], _codec: VideoCodec, quality: u32) {
        if self.common_data.use_cache {
            let OptimizerMode::VideoTranscode { codec, .. } = self.params.mode;
            let entries_map: BTreeMap<String, VideoTranscodeEntry> = entries.iter().map(|entry| (entry.path.to_string_lossy().to_string(), entry.clone())).collect();
            let messages = save_cache_to_file_generalized(&get_video_transcode_cache_file(&codec, quality), &entries_map, self.get_save_also_as_json(), 0);
            self.get_cd_mut().text_messages.messages.extend(messages.messages);
            self.get_cd_mut().text_messages.warnings.extend(messages.warnings);
        }
    }

    #[fun_time(message = "fix_files", level = "debug")]
    pub(crate) fn fix_files(&mut self, stop_flag: &Arc<AtomicBool>, _progress_sender: Option<&Sender<ProgressData>>) -> WorkContinueStatus {
        match self.params.mode {
            OptimizerMode::VideoTranscode { codec, quality } => {
                info!("Starting optimization of {} video files", self.video_transcode_entries.len());

                self.video_transcode_entries.par_iter_mut().for_each(|entry| {
                    if check_if_stop_received(stop_flag) {
                        return;
                    }
                    match process_video(&entry.path, entry.size, codec, quality) {
                        Ok(_new_size) => {}
                        Err(e) => {
                            entry.error = Some(e);
                        }
                    }
                });

                let successful_files = self.video_transcode_entries.iter().filter(|e| e.error.is_none() && !e.codec.is_empty()).count();
                let failed_files = self.video_transcode_entries.iter().filter(|e| e.error.is_some()).count();

                self.information.number_of_processed_files = successful_files;
                self.information.number_of_failed_files = failed_files;

                debug!("Optimization complete - Processed: {successful_files}, Failed: {failed_files}");
            }
        }

        WorkContinueStatus::Continue
    }
}

pub fn get_video_transcode_cache_file(codec: &VideoCodec, quality: u32) -> String {
    format!("cache_video_transcode_{codec:?}_{quality}_{CACHE_VIDEO_TRANSCODE_VERSION}.bin")
}
