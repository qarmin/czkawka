use std::path::MAIN_SEPARATOR;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;

use crossbeam_channel::Sender;
use czkawka_core::common::progress_data::ProgressData;
use czkawka_core::tools::video_optimizer::{VideoCodec, VideoCropSingleFixParams, VideoCroppingMechanism, VideoTranscodeFixParams};
use slint::{ComponentHandle, Weak};

use crate::common::IntDataVideoOptimizer;
use crate::model_operations::model_processor::{MessageType, ModelProcessor, ProcessFunction};
use crate::settings::collect_combo_box_settings;
use crate::simpler_model::{SimplerSingleMainListModel, ToSimplerVec};
use crate::{Callabler, GuiState, MainWindow, Settings};

pub(crate) fn connect_optimize_video(app: &MainWindow, progress_sender: Sender<ProgressData>, stop_flag: Arc<AtomicBool>) {
    let a = app.as_weak();

    let progress_sender_crop = progress_sender.clone();
    let stop_flag_crop = stop_flag.clone();
    app.global::<Callabler>().on_crop_video_items(move || {
        let weak_app = a.clone();
        let progress_sender = progress_sender_crop.clone();
        let stop_flag = stop_flag_crop.clone();
        stop_flag.store(false, Ordering::Relaxed);
        let app = a.upgrade().expect("Failed to upgrade app :(");
        let active_tab = app.global::<GuiState>().get_active_tab();

        let settings = app.global::<Settings>();
        let reencode = settings.get_popup_crop_video_reencode();
        let video_quality = settings.get_video_optimizer_sub_video_quality();
        let overwrite_files = settings.get_popup_crop_video_overwrite_files();

        let crop_mechanism = collect_combo_box_settings(&app).video_optimizer_crop_type.value;

        let processor = ModelProcessor::new(active_tab);

        let requested_codec = if reencode {
            Some(collect_combo_box_settings(&app).video_optimizer_video_codec.value)
        } else {
            None
        };

        processor.crop_selected_videos(progress_sender, weak_app, stop_flag, requested_codec, overwrite_files, video_quality, crop_mechanism);
    });

    let a2 = app.as_weak();
    app.global::<Callabler>().on_reencode_video_items(move || {
        let weak_app = a2.clone();
        let progress_sender = progress_sender.clone();
        let stop_flag = stop_flag.clone();
        stop_flag.store(false, Ordering::Relaxed);
        let app = a2.upgrade().expect("Failed to upgrade app :(");
        let active_tab = app.global::<GuiState>().get_active_tab();

        let settings = app.global::<Settings>();
        let codec = collect_combo_box_settings(&app).video_optimizer_video_codec.value;
        let fail_if_bigger = settings.get_popup_reencode_video_fail_if_bigger();
        let overwrite_files = settings.get_popup_reencode_video_overwrite_files();
        let video_quality = settings.get_popup_reencode_video_quality();
        let limit_video_size = settings.get_popup_reencode_video_limit_video_size();

        let max_width_str = settings.get_popup_reencode_video_max_width();
        let max_height_str = settings.get_popup_reencode_video_max_height();

        let max_width = max_width_str.parse::<i32>().unwrap_or(0).max(0) as u32;
        let max_height = max_height_str.parse::<i32>().unwrap_or(0).max(0) as u32;

        let max_width = if max_width > 0 { max_width } else { 1920 };
        let max_height = if max_height > 0 { max_height } else { 1920 };

        let processor = ModelProcessor::new(active_tab);

        processor.optimize_selected_videos(
            progress_sender,
            weak_app,
            stop_flag,
            codec,
            fail_if_bigger,
            overwrite_files,
            video_quality,
            limit_video_size,
            max_width,
            max_height,
        );
    });
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
            let optimize_fnc = move |data: &SimplerSingleMainListModel| {
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

            self.process_and_update_gui_state(
                &weak_app,
                stop_flag,
                &progress_sender,
                simpler_model,
                &ProcessFunction::Simple(Box::new(optimize_fnc)),
                MessageType::OptimizeVideo,
                true,
            );
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
            let codec_idx = self.active_tab.get_str_video_codec_idx();

            let rect_left_idx = IntDataVideoOptimizer::RectLeft as usize;
            let rect_top_idx = IntDataVideoOptimizer::RectTop as usize;
            let rect_right_idx = IntDataVideoOptimizer::RectRight as usize;
            let rect_bottom_idx = IntDataVideoOptimizer::RectBottom as usize;

            let quality = if requested_codec.is_some() { Some(video_quality as u32) } else { None };

            let stop_flag_clone = stop_flag.clone();
            let crop_fnc = move |data: &SimplerSingleMainListModel| {
                let full_path = format!("{}{MAIN_SEPARATOR}{}", data.val_str[path_idx], data.val_str[name_idx]);
                let original_size = data.get_size(size_idx);
                let codec = &data.val_str[codec_idx];

                let left = data.val_int[rect_left_idx] as u32;
                let top = data.val_int[rect_top_idx] as u32;
                let right = data.val_int[rect_right_idx] as u32;
                let bottom = data.val_int[rect_bottom_idx] as u32;

                crop_single_video(
                    &stop_flag_clone,
                    &full_path,
                    original_size,
                    VideoCropSingleFixParams {
                        overwrite_original: overwrite_files,
                        target_codec: requested_codec,
                        quality,
                        crop_rectangle: (left, top, right, bottom),
                        crop_mechanism: video_crop_mechanism,
                    },
                    codec,
                )
            };

            self.process_and_update_gui_state(
                &weak_app,
                stop_flag,
                &progress_sender,
                simpler_model,
                &ProcessFunction::Simple(Box::new(crop_fnc)),
                MessageType::OptimizeVideo,
                true,
            );
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
fn crop_single_video(stop_flag: &Arc<AtomicBool>, full_path: &str, _original_size: u64, params: VideoCropSingleFixParams, codec: &str) -> Result<(), String> {
    czkawka_core::tools::video_optimizer::core::fix_video_crop(std::path::Path::new(full_path), &params, stop_flag, codec)
}

#[cfg(test)]
fn crop_single_video(_stop_flag: &Arc<AtomicBool>, video_path: &str, _original_size: u64, _params: VideoCropSingleFixParams, _codec: &str) -> Result<(), String> {
    if video_path.contains("test_error") {
        return Err(format!("Test error for item: {video_path}"));
    }
    Ok(())
}
