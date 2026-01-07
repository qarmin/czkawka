use std::path::MAIN_SEPARATOR;
use std::str::FromStr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;

use crossbeam_channel::Sender;
use czkawka_core::common::progress_data::ProgressData;
use czkawka_core::tools::video_optimizer::VideoCodec;
use slint::{ComponentHandle, SharedString, Weak};

use crate::model_operations::model_processor::{MessageType, ModelProcessor};
use crate::shared_models::SharedModels;
use crate::simpler_model::{SimplerMainListModel, ToSimplerVec};
use crate::{Callabler, GuiState, MainWindow};

pub(crate) fn connect_optimize_video(app: &MainWindow, progress_sender: Sender<ProgressData>, stop_flag: Arc<AtomicBool>, _shared_models: Arc<Mutex<SharedModels>>) {
    let a = app.as_weak();
    app.global::<Callabler>().on_optimize_items(
        move |codec: SharedString, fail_if_bigger: bool, overwrite_files: bool, video_quality: f32, limit_video_size: bool, max_width: i32, max_height: i32| {
            let weak_app = a.clone();
            let progress_sender = progress_sender.clone();
            let stop_flag = stop_flag.clone();
            stop_flag.store(false, Ordering::Relaxed);
            let app = a.upgrade().expect("Failed to upgrade app :(");
            let active_tab = app.global::<GuiState>().get_active_tab();

            let processor = ModelProcessor::new(active_tab);
            processor.optimize_selected_videos(
                progress_sender,
                weak_app,
                stop_flag,
                codec.to_string(),
                fail_if_bigger,
                overwrite_files,
                video_quality,
                limit_video_size,
                max_width.max(0) as u32,
                max_height.max(0) as u32,
            );
        },
    );
}

impl ModelProcessor {
    fn optimize_selected_videos(
        self,
        progress_sender: Sender<ProgressData>,
        weak_app: Weak<MainWindow>,
        stop_flag: Arc<AtomicBool>,
        requested_video_codec_str: String,
        fail_if_bigger: bool,
        overwrite_files: bool,
        video_quality: f32,
        limit_video_size: bool,
        max_width: u32,
        max_height: u32,
    ) {
        let model = self.active_tab.get_tool_model(&weak_app.upgrade().expect("Failed to upgrade app :("));
        let simpler_model = model.to_simpler_enumerated_vec();
        thread::spawn(move || {
            let path_idx = self.active_tab.get_str_path_idx();
            let name_idx = self.active_tab.get_str_name_idx();
            let size_idx = self.active_tab.get_int_size_idx();
            let codec_idx = self.active_tab.get_str_video_codec_idx();

            let requested_video_codec_str = requested_video_codec_str.clone();
            let stop_flag_clone = stop_flag.clone();
            let optimize_fnc = move |data: &SimplerMainListModel| {
                let requested_video_codec = VideoCodec::from_str(&requested_video_codec_str)
                    .unwrap_or_else(|_err| panic!("Unsupported codec: {}(This should be validated before)", &requested_video_codec_str));
                let file_codec = &data.val_str[codec_idx];
                if &requested_video_codec_str == file_codec {
                    return Ok(()); // No need to transcode if codec is the same
                }

                let full_path = format!("{}{MAIN_SEPARATOR}{}", data.val_str[path_idx], data.val_str[name_idx]);
                let original_size = data.get_size(size_idx);
                let target_quality = video_quality as u32;

                optimize_single_video(
                    &stop_flag_clone,
                    &full_path,
                    original_size,
                    requested_video_codec,
                    target_quality,
                    fail_if_bigger,
                    overwrite_files,
                    limit_video_size,
                    max_width,
                    max_height,
                )
            };

            self.process_and_update_gui_state(&weak_app, stop_flag, &progress_sender, simpler_model, optimize_fnc, MessageType::OptimizeVideo, true);
        });
    }
}

#[cfg(not(test))]
fn optimize_single_video(
    stop_flag: &Arc<AtomicBool>,
    video_path: &str,
    original_size: u64,
    requested_video_codec: VideoCodec,
    target_quality: u32,
    fail_if_not_smaller: bool,
    overwrite_original: bool,
    limit_video_size: bool,
    max_width: u32,
    max_height: u32,
) -> Result<(), String> {
    czkawka_core::tools::video_optimizer::core::process_video(
        stop_flag,
        video_path,
        original_size,
        requested_video_codec,
        target_quality,
        fail_if_not_smaller,
        overwrite_original,
        limit_video_size,
        max_width,
        max_height,
    )
}

#[cfg(test)]
fn optimize_single_video(
    _stop_flag: &Arc<AtomicBool>,
    video_path: &str,
    _original_size: u64,
    _requested_video_codec: VideoCodec,
    _target_quality: u32,
    _fail_if_not_smaller: bool,
    _overwrite_original: bool,
    _limit_video_size: bool,
    _max_width: u32,
    _max_height: u32,
) -> Result<(), String> {
    if video_path.contains("test_error") {
        return Err(format!("Test error for item: {video_path}"));
    }
    Ok(())
}
