use std::path::{Path, MAIN_SEPARATOR};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;

use crossbeam_channel::Sender;
use czkawka_core::common::progress_data::ProgressData;
use czkawka_core::tools::video_optimizer::{VideoCodec, VideoCropFixParams, VideoCroppingMechanism, VideoOptimizerParameters, VideoTranscodeFixParams};
use slint::{ComponentHandle, SharedString, Weak};

use crate::common::IntDataVideoOptimizer;
use crate::model_operations::model_processor::{MessageType, ModelProcessor};
use crate::settings::collect_settings;
use crate::settings::combo_box::StringComboBoxItems;
use crate::shared_models::SharedModels;
use crate::simpler_model::{SimplerMainListModel, ToSimplerVec};
use crate::{Callabler, GuiState, MainWindow};

pub(crate) fn connect_optimize_video(app: &MainWindow, progress_sender: Sender<ProgressData>, stop_flag: Arc<AtomicBool>, shared_models: Arc<Mutex<SharedModels>>) {
    let a = app.as_weak();
    app.global::<Callabler>().on_optimize_items(
        move |codec: SharedString, fail_if_bigger: bool, overwrite_files: bool, video_quality: f32, limit_video_size: bool, max_width: i32, max_height: i32| {
            let weak_app = a.clone();
            let progress_sender = progress_sender.clone();
            let stop_flag = stop_flag.clone();
            stop_flag.store(false, Ordering::Relaxed);
            let app = a.upgrade().expect("Failed to upgrade app :(");
            let active_tab = app.global::<GuiState>().get_active_tab();

            let settings = collect_settings(&app);

            let collected_items = StringComboBoxItems::get_items();
            let crop_mechanism = StringComboBoxItems::get_value_from_config_name(&settings.video_optimizer_crop_type, &collected_items.video_optimizer_crop_type);

            let shared_model = shared_models.lock();
            let shared_model = shared_model.as_ref().expect("Failed to lock shared models");
            let shared_model = shared_model.shared_video_optimizer_state.as_ref().expect("Item should be present for video optimizer");
            let crop_mode = matches!(shared_model.get_params(), VideoOptimizerParameters::VideoCrop(_));

            let processor = ModelProcessor::new(active_tab);

            if crop_mode {
                let video_codec = if codec.is_empty() {
                    None
                } else {
                    Some(StringComboBoxItems::get_value_from_config_name(&codec, &collected_items.video_optimizer_video_codec))
                };
                processor.crop_selected_videos(progress_sender, weak_app, stop_flag, video_codec, overwrite_files, video_quality, crop_mechanism);
            } else {
                let video_codec = StringComboBoxItems::get_value_from_config_name(&codec, &collected_items.video_optimizer_video_codec);
                processor.optimize_selected_videos(
                    progress_sender,
                    weak_app,
                    stop_flag,
                    video_codec,
                    fail_if_bigger,
                    overwrite_files,
                    video_quality,
                    limit_video_size,
                    max_width.max(0) as u32,
                    max_height.max(0) as u32,
                );
            }
        },
    );
}

impl ModelProcessor {
    fn optimize_selected_videos(
        self,
        progress_sender: Sender<ProgressData>,
        weak_app: Weak<MainWindow>,
        stop_flag: Arc<AtomicBool>,
        requested_video_codec: VideoCodec,
        fail_if_bigger: bool,
        overwrite_files: bool,
        video_quality: f32,
        limit_video_size: bool,
        max_width: u32,
        max_height: u32,
    ) {
        let codec_str = requested_video_codec.as_ffprobe_codec_name().to_string();

        let model = self.active_tab.get_tool_model(&weak_app.upgrade().expect("Failed to upgrade app :("));
        let simpler_model = model.to_simpler_enumerated_vec();
        thread::spawn(move || {
            let path_idx = self.active_tab.get_str_path_idx();
            let name_idx = self.active_tab.get_str_name_idx();
            let size_idx = self.active_tab.get_int_size_idx();
            let codec_idx = self.active_tab.get_str_video_codec_idx();

            let stop_flag_clone = stop_flag.clone();
            let optimize_fnc = move |data: &SimplerMainListModel| {
                let file_codec = &data.val_str[codec_idx];
                if codec_str == *file_codec {
                    return Ok(()); // No need to transcode if codec is the same
                }

                let full_path = format!("{}{MAIN_SEPARATOR}{}", data.val_str[path_idx], data.val_str[name_idx]);
                let original_size = data.get_size(size_idx);
                let target_quality = video_quality as u32;

                optimize_single_video(
                    &stop_flag_clone,
                    &full_path,
                    original_size,
                    VideoTranscodeFixParams {
                        codec: requested_video_codec,
                        quality: target_quality,
                        fail_if_not_smaller: fail_if_bigger,
                        overwrite_original: overwrite_files,
                        limit_video_size,
                        max_width,
                        max_height,
                    },
                )
            };

            self.process_and_update_gui_state(&weak_app, stop_flag, &progress_sender, simpler_model, optimize_fnc, MessageType::OptimizeVideo, true);
        });
    }

    fn crop_selected_videos(
        self,
        progress_sender: Sender<ProgressData>,
        weak_app: Weak<MainWindow>,
        stop_flag: Arc<AtomicBool>,
        requested_codec: Option<VideoCodec>,
        overwrite_files: bool,
        video_quality: f32,
        video_crop_mechanism: VideoCroppingMechanism,
    ) {
        let model = self.active_tab.get_tool_model(&weak_app.upgrade().expect("Failed to upgrade app :("));
        let simpler_model = model.to_simpler_enumerated_vec();

        thread::spawn(move || {
            let path_idx = self.active_tab.get_str_path_idx();
            let name_idx = self.active_tab.get_str_name_idx();
            let size_idx = self.active_tab.get_int_size_idx();

            let rect_left_idx = IntDataVideoOptimizer::RectLeft as usize;
            let rect_top_idx = IntDataVideoOptimizer::RectTop as usize;
            let rect_right_idx = IntDataVideoOptimizer::RectRight as usize;
            let rect_bottom_idx = IntDataVideoOptimizer::RectBottom as usize;

            let quality = if video_quality > 0.0 { Some(video_quality as u32) } else { None };

            let stop_flag_clone = stop_flag.clone();
            let crop_fnc = move |data: &SimplerMainListModel| {
                let full_path = format!("{}{MAIN_SEPARATOR}{}", data.val_str[path_idx], data.val_str[name_idx]);
                let original_size = data.get_size(size_idx);

                let left = data.val_int[rect_left_idx] as u32;
                let top = data.val_int[rect_top_idx] as u32;
                let right = data.val_int[rect_right_idx] as u32;
                let bottom = data.val_int[rect_bottom_idx] as u32;

                crop_single_video(
                    &stop_flag_clone,
                    &full_path,
                    original_size,
                    VideoCropFixParams {
                        overwrite_original: overwrite_files,
                        target_codec: requested_codec,
                        quality,
                        crop_rectangle: (left, top, right, bottom),
                        crop_mechanism: video_crop_mechanism,
                    },
                )
            };

            self.process_and_update_gui_state(&weak_app, stop_flag, &progress_sender, simpler_model, crop_fnc, MessageType::OptimizeVideo, true);
        });
    }
}

#[cfg(not(test))]
fn optimize_single_video(stop_flag: &Arc<AtomicBool>, video_path: &str, original_size: u64, transcode_params: VideoTranscodeFixParams) -> Result<(), String> {
    czkawka_core::tools::video_optimizer::core::process_video(stop_flag, video_path, original_size, transcode_params)
}

#[cfg(test)]
fn optimize_single_video(_stop_flag: &Arc<AtomicBool>, video_path: &str, _original_size: u64, _transcode_params: VideoTranscodeFixParams) -> Result<(), String> {
    if video_path.contains("test_error") {
        return Err(format!("Test error for item: {video_path}"));
    }
    Ok(())
}

#[cfg(not(test))]
fn crop_single_video(stop_flag: &Arc<AtomicBool>, full_path: &str, _original_size: u64, params: VideoCropFixParams) -> Result<(), String> {
    czkawka_core::tools::video_optimizer::core::fix_video_crop(std::path::Path::new(full_path), &params, stop_flag)
}

#[cfg(test)]
fn crop_single_video(_stop_flag: &Arc<AtomicBool>, video_path: &str, _original_size: u64, _params: VideoCropFixParams) -> Result<(), String> {
    if video_path.contains("test_error") {
        return Err(format!("Test error for item: {video_path}"));
    }
    Ok(())
}
