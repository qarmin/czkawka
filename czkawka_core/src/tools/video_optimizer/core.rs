use std::collections::BTreeMap;
use std::mem;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use crossbeam_channel::Sender;
use fun_time::fun_time;
use log::{debug, info};
use rayon::prelude::*;

use crate::common::cache::{load_and_split_cache_generalized_by_path, save_and_connect_cache_generalized_by_path};
use crate::common::config_cache_path::get_config_cache_path;
use crate::common::dir_traversal::{DirTraversalBuilder, DirTraversalResult};
use crate::common::model::{ToolType, WorkContinueStatus};
use crate::common::progress_data::{CurrentStage, ProgressData};
use crate::common::progress_stop_handler::{check_if_stop_received, prepare_thread_handler_common};
use crate::common::tool_data::{CommonData, CommonToolData};
use crate::common::video_utils::{VIDEO_THUMBNAILS_SUBFOLDER, generate_thumbnail};
use crate::tools::video_optimizer::{
    Info, VideoCropEntry, VideoCropParams, VideoCropSingleFixParams, VideoOptimizer, VideoOptimizerFixParams, VideoOptimizerParameters, VideoTranscodeEntry, VideoTranscodeParams,
};

mod video_converter;
mod video_cropper;

pub use video_converter::process_video;
pub use video_cropper::fix_video_crop;

use crate::common::cache::CACHE_VIDEO_OPTIMIZE_VERSION;
use crate::common::traits::ResultEntry;
use crate::flc;

impl VideoOptimizer {
    pub fn new(params: VideoOptimizerParameters) -> Self {
        Self {
            common_data: CommonToolData::new(ToolType::VideoOptimizer),
            information: Info::default(),
            video_transcode_test_entries: Default::default(),
            video_crop_test_entries: Default::default(),
            video_transcode_result_entries: Vec::new(),
            video_crop_result_entries: Vec::new(),
            params,
        }
    }

    #[fun_time(message = "scan_files", level = "debug")]
    pub(crate) fn scan_files(&mut self, stop_flag: &Arc<AtomicBool>, progress_sender: Option<&Sender<ProgressData>>) -> WorkContinueStatus {
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
                        self.video_transcode_test_entries = grouped_file_entries
                            .into_values()
                            .flatten()
                            .map(|fe| (fe.get_path().to_string_lossy().to_string(), fe.into_video_transcode_entry()))
                            .collect();
                        info!("Found {} files to check", self.video_transcode_test_entries.len());
                    }
                    VideoOptimizerParameters::VideoCrop(_) => {
                        self.video_crop_test_entries = grouped_file_entries
                            .into_values()
                            .flatten()
                            .map(|fe| (fe.get_path().to_string_lossy().to_string(), fe.into_video_crop_entry()))
                            .collect();
                        info!("Found {} files to check", self.video_crop_test_entries.len());
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
        if self.video_transcode_test_entries.is_empty() {
            return WorkContinueStatus::Continue;
        }

        let (loaded_hash_map, records_already_cached, non_cached_files_to_check) = self.load_video_transcode_cache();

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

        self.save_video_transcode_cache(&entries, loaded_hash_map);

        entries.retain(|e| e.error.is_none() && !params.excluded_codecs.contains(&e.codec));

        self.video_transcode_result_entries = entries;
        self.information.number_of_videos_to_transcode = self.video_transcode_result_entries.len();

        if self.create_transcode_thumbnails(progress_sender, stop_flag, &params) == WorkContinueStatus::Stop {
            return WorkContinueStatus::Stop;
        }

        WorkContinueStatus::Continue
    }

    #[fun_time(message = "process_video_crop", level = "debug")]
    fn process_video_crop(&mut self, stop_flag: &Arc<AtomicBool>, progress_sender: Option<&Sender<ProgressData>>) -> WorkContinueStatus {
        if self.video_crop_test_entries.is_empty() {
            return WorkContinueStatus::Continue;
        }

        let VideoOptimizerParameters::VideoCrop(params) = self.params.clone() else {
            unreachable!("process_video_crop called with non VideoCrop parameters, caller is responsible for that");
        };

        let (loaded_hash_map, records_already_cached, non_cached_files_to_check) = self.load_video_crop_cache(&params);

        let progress_handler = prepare_thread_handler_common(
            progress_sender,
            CurrentStage::VideoOptimizerProcessingVideos,
            non_cached_files_to_check.len(),
            self.get_test_type(),
            non_cached_files_to_check.values().map(|entry| entry.size).sum(),
        );

        let mut vec_file_entry: Vec<VideoCropEntry> = non_cached_files_to_check
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

        self.common_data
            .text_messages
            .warnings
            .extend(vec_file_entry.iter().filter_map(|e| e.error.as_ref()).cloned());
        vec_file_entry.extend(records_already_cached.into_values());

        progress_handler.join_thread();

        self.save_video_crop_cache(&vec_file_entry, &params, loaded_hash_map);

        vec_file_entry.retain(|e| e.error.is_none() && e.new_image_dimensions != (0, 0, 0, 0));

        self.video_crop_result_entries = vec_file_entry;
        self.information.number_of_videos_to_crop = self.video_crop_result_entries.len();

        if self.create_crop_thumbnails(progress_sender, stop_flag, &params) == WorkContinueStatus::Stop {
            return WorkContinueStatus::Stop;
        }

        WorkContinueStatus::Continue
    }

    #[fun_time(message = "create_transcode_thumbnails", level = "debug")]
    fn create_transcode_thumbnails(&mut self, progress_sender: Option<&Sender<ProgressData>>, stop_flag: &Arc<AtomicBool>, params: &VideoTranscodeParams) -> WorkContinueStatus {
        if !params.generate_thumbnails {
            return WorkContinueStatus::Continue;
        }

        let progress_handler = prepare_thread_handler_common(
            progress_sender,
            CurrentStage::VideoOptimizerCreatingThumbnails,
            self.video_transcode_result_entries.len(),
            self.get_test_type(),
            0,
        );

        let Some(config_cache_path) = get_config_cache_path() else {
            return WorkContinueStatus::Continue;
        };

        let thumbnails_dir = config_cache_path.cache_folder.join(VIDEO_THUMBNAILS_SUBFOLDER);
        if let Err(e) = std::fs::create_dir_all(&thumbnails_dir) {
            debug!("Failed to create thumbnails directory: {e}");
            return WorkContinueStatus::Continue;
        }

        let thumbnail_video_percentage_from_start = params.thumbnail_video_percentage_from_start;
        let generate_grid_instead_of_single = params.generate_thumbnail_grid_instead_of_single;
        let thumbnail_grid_tiles_per_side = params.thumbnail_grid_tiles_per_side;

        let errors = self
            .video_transcode_result_entries
            .par_iter_mut()
            .map(|entry| {
                if check_if_stop_received(stop_flag) {
                    return None;
                }

                match generate_thumbnail(
                    stop_flag,
                    &entry.path,
                    entry.size,
                    entry.modified_date,
                    Some(entry.duration),
                    &thumbnails_dir,
                    thumbnail_video_percentage_from_start,
                    generate_grid_instead_of_single,
                    thumbnail_grid_tiles_per_side,
                ) {
                    Ok(thumbnail_path) => {
                        entry.thumbnail_path = Some(thumbnail_path);
                        progress_handler.increase_items(1);
                        Some(None)
                    }
                    Err(e) => {
                        progress_handler.increase_items(1);
                        Some(Some(e))
                    }
                }
            })
            .while_some()
            .flatten()
            .collect::<Vec<String>>();

        self.common_data.text_messages.warnings.extend(errors);

        progress_handler.join_thread();
        if check_if_stop_received(stop_flag) {
            return WorkContinueStatus::Stop;
        }

        WorkContinueStatus::Continue
    }

    #[fun_time(message = "create_crop_thumbnails", level = "debug")]
    fn create_crop_thumbnails(&mut self, progress_sender: Option<&Sender<ProgressData>>, stop_flag: &Arc<AtomicBool>, params: &VideoCropParams) -> WorkContinueStatus {
        if !params.generate_thumbnails {
            return WorkContinueStatus::Continue;
        }

        let progress_handler = prepare_thread_handler_common(
            progress_sender,
            CurrentStage::VideoOptimizerCreatingThumbnails,
            self.video_crop_result_entries.len(),
            self.get_test_type(),
            self.video_crop_result_entries.iter().map(|e| e.size).sum(),
        );

        let Some(config_cache_path) = get_config_cache_path() else {
            return WorkContinueStatus::Continue;
        };

        let thumbnails_dir = config_cache_path.cache_folder.join(VIDEO_THUMBNAILS_SUBFOLDER);
        if let Err(e) = std::fs::create_dir_all(&thumbnails_dir) {
            debug!("Failed to create thumbnails directory: {e}");
            return WorkContinueStatus::Continue;
        }

        let thumbnail_video_percentage_from_start = params.thumbnail_video_percentage_from_start;
        let generate_grid_instead_of_single = params.generate_thumbnail_grid_instead_of_single;
        let thumbnail_grid_tiles_per_side = params.thumbnail_grid_tiles_per_side;

        let errors = self
            .video_crop_result_entries
            .par_iter_mut()
            .map(|entry| {
                if check_if_stop_received(stop_flag) {
                    return None;
                }

                let result = generate_thumbnail(
                    stop_flag,
                    &entry.path,
                    entry.size,
                    entry.modified_date,
                    Some(entry.duration),
                    &thumbnails_dir,
                    thumbnail_video_percentage_from_start,
                    generate_grid_instead_of_single,
                    thumbnail_grid_tiles_per_side,
                );

                match result {
                    Ok(thumbnail_path) => {
                        entry.thumbnail_path = Some(thumbnail_path);
                        progress_handler.increase_items(1);
                        Some(None)
                    }
                    Err(e) => {
                        progress_handler.increase_items(1);
                        Some(Some(e))
                    }
                }
            })
            .while_some()
            .flatten()
            .collect::<Vec<String>>();

        self.common_data.text_messages.warnings.extend(errors);

        progress_handler.join_thread();
        if check_if_stop_received(stop_flag) {
            return WorkContinueStatus::Stop;
        }

        WorkContinueStatus::Continue
    }

    #[fun_time(message = "load_video_transcode_cache", level = "debug")]
    fn load_video_transcode_cache(
        &mut self,
    ) -> (
        BTreeMap<String, VideoTranscodeEntry>,
        BTreeMap<String, VideoTranscodeEntry>,
        BTreeMap<String, VideoTranscodeEntry>,
    ) {
        load_and_split_cache_generalized_by_path(&get_video_transcode_cache_file(), mem::take(&mut self.video_transcode_test_entries), self)
    }

    #[fun_time(message = "load_video_crop_cache", level = "debug")]
    fn load_video_crop_cache(&mut self, params: &VideoCropParams) -> (BTreeMap<String, VideoCropEntry>, BTreeMap<String, VideoCropEntry>, BTreeMap<String, VideoCropEntry>) {
        load_and_split_cache_generalized_by_path(&get_video_crop_cache_file(params), mem::take(&mut self.video_crop_test_entries), self)
    }

    #[fun_time(message = "save_video_transcode_cache", level = "debug")]
    fn save_video_transcode_cache(&mut self, vec_file_entry: &[VideoTranscodeEntry], loaded_hash_map: BTreeMap<String, VideoTranscodeEntry>) {
        save_and_connect_cache_generalized_by_path(&get_video_transcode_cache_file(), vec_file_entry, loaded_hash_map, self);
    }

    #[fun_time(message = "save_video_crop_cache", level = "debug")]
    fn save_video_crop_cache(&mut self, vec_file_entry: &[VideoCropEntry], params: &VideoCropParams, loaded_hash_map: BTreeMap<String, VideoCropEntry>) {
        save_and_connect_cache_generalized_by_path(&get_video_crop_cache_file(params), vec_file_entry, loaded_hash_map, self);
    }

    #[fun_time(message = "fix_files", level = "debug")]
    pub(crate) fn fix_files(&mut self, stop_flag: &Arc<AtomicBool>, _progress_sender: Option<&Sender<ProgressData>>, fix_params: VideoOptimizerFixParams) {
        match self.params.clone() {
            VideoOptimizerParameters::VideoTranscode(_) => {
                let VideoOptimizerFixParams::VideoTranscode(video_transcode_params) = fix_params else {
                    unreachable!("VideoTranscode mode should have VideoTranscode fix_params(caller is responsible for that)");
                };

                let transcode_warnings: Vec<_> = mem::take(&mut self.video_transcode_result_entries)
                    .into_par_iter()
                    .map(|entry| {
                        if check_if_stop_received(stop_flag) {
                            return None;
                        }

                        match process_video(stop_flag, &entry.path.to_string_lossy(), entry.size, video_transcode_params) {
                            Ok(_new_size) => Some(None),
                            Err(e) => Some(Some(flc!("core_failed_to_optimize_video", file = entry.path.to_string_lossy(), reason = e))),
                        }
                    })
                    .while_some()
                    .flatten()
                    .collect();

                self.common_data.text_messages.warnings.extend(transcode_warnings);
            }
            VideoOptimizerParameters::VideoCrop(_) => {
                let VideoOptimizerFixParams::VideoCrop(video_crop_params) = fix_params else {
                    unreachable!("VideoCrop mode should have VideoCrop fix_params(caller is responsible for that)");
                };

                let crop_warnings: Vec<_> = mem::take(&mut self.video_crop_result_entries)
                    .into_par_iter()
                    .map(|entry| {
                        if check_if_stop_received(stop_flag) {
                            return None;
                        }

                        let (left, top, right, bottom) = entry.new_image_dimensions;
                        let entry_crop_params = VideoCropSingleFixParams {
                            overwrite_original: video_crop_params.overwrite_original,
                            target_codec: video_crop_params.target_codec,
                            quality: video_crop_params.quality,
                            crop_rectangle: (left, top, right, bottom),
                            crop_mechanism: video_crop_params.crop_mechanism,
                        };

                        match fix_video_crop(&entry.path, &entry_crop_params, stop_flag, &entry.codec) {
                            Ok(()) => Some(None),
                            Err(e) => Some(Some(flc!("core_failed_to_crop_video", file = entry.path.to_string_lossy(), reason = e))),
                        }
                    })
                    .while_some()
                    .flatten()
                    .collect();

                self.common_data.text_messages.warnings.extend(crop_warnings);
            }
        }
    }
}

pub fn get_video_transcode_cache_file() -> String {
    format!("cache_video_transcode_{CACHE_VIDEO_OPTIMIZE_VERSION}.bin")
}

pub fn get_video_crop_cache_file(params: &VideoCropParams) -> String {
    format!(
        "cache_video_crop_{CACHE_VIDEO_OPTIMIZE_VERSION}_{:?}_t{}_p{}_s{}_c{}.bin",
        params.crop_detect, params.black_pixel_threshold, params.black_bar_min_percentage, params.max_samples, params.min_crop_size
    )
}
