use std::rc::Rc;
use std::thread;

use czkawka_core::common::consts::DEFAULT_THREAD_SIZE;
use czkawka_core::common::tool_data::CommonData;
use czkawka_core::common::traits::Search;
use czkawka_core::common::{format_time, split_path};
use czkawka_core::tools::video_optimizer;
use czkawka_core::tools::video_optimizer::{
    VideoCropEntry, VideoCropParams, VideoOptimizer, VideoOptimizerMode, VideoOptimizerParameters, VideoTranscodeEntry, VideoTranscodeParams,
};
use humansize::{BINARY, format_size};
use log::error;
use slint::{ComponentHandle, ModelRc, SharedString, VecModel, Weak};

use crate::common::{MAX_INT_DATA_VIDEO_OPTIMIZER, MAX_STR_DATA_VIDEO_OPTIMIZER, split_u64_into_i32s};
use crate::connect_scan::{MessagesData, ScanData, get_dt_timestamp_string, get_text_messages, insert_data_to_model, reset_selection_at_end, set_common_settings};
use crate::{ActiveTab, GuiState, MainWindow, flk};

pub(crate) fn scan_video_optimizer(a: Weak<MainWindow>, sd: ScanData) {
    thread::Builder::new()
        .stack_size(DEFAULT_THREAD_SIZE)
        .spawn(move || {
            let video_optimizer_mode = sd.combo_box_items.video_optimizer_mode.value;
            let params = if video_optimizer_mode == VideoOptimizerMode::VideoCrop {
                let crop_detect = sd.combo_box_items.video_optimizer_crop_type.value;
                let params = VideoCropParams::with_custom_params(
                    crop_detect,
                    sd.custom_settings.video_optimizer_black_pixel_threshold,
                    sd.custom_settings.video_optimizer_black_bar_min_percentage,
                    sd.custom_settings.video_optimizer_max_samples,
                    sd.custom_settings.video_optimizer_min_crop_size,
                    sd.custom_settings.video_thumbnails_generate,
                    sd.custom_settings.video_thumbnails_percentage,
                    sd.custom_settings.video_thumbnails_generate_grid,
                    sd.custom_settings.video_thumbnails_grid_tiles_per_side,
                );
                VideoOptimizerParameters::VideoCrop(params)
            } else {
                let excluded_codecs: Vec<String> = sd
                    .custom_settings
                    .video_optimizer_excluded_codecs
                    .split(',')
                    .map(|s| s.trim().to_lowercase())
                    .filter(|s| !s.is_empty())
                    .collect();
                let params = VideoTranscodeParams::new(
                    excluded_codecs,
                    sd.custom_settings.video_thumbnails_generate,
                    sd.custom_settings.video_thumbnails_percentage,
                    sd.custom_settings.video_thumbnails_generate_grid,
                    sd.custom_settings.video_thumbnails_grid_tiles_per_side,
                );
                VideoOptimizerParameters::VideoTranscode(params)
            };

            let is_crop_mode = matches!(params, VideoOptimizerParameters::VideoCrop(_));

            let mut tool = VideoOptimizer::new(params);
            set_common_settings(&mut tool, &sd.custom_settings, &sd.stop_flag);

            tool.search(&sd.stop_flag, Some(&sd.progress_sender));

            let (critical, messages) = get_text_messages(&tool, &sd.basic_settings);

            let info = tool.get_information();
            let stopped_search = tool.get_stopped_search();

            if is_crop_mode {
                let video_crop_entries = tool.get_video_crop_entries().clone();
                sd.shared_models.lock().unwrap().shared_video_optimizer_state = Some(tool);

                let messages_data = MessagesData { critical, messages };

                a.upgrade_in_event_loop(move |app| {
                    write_video_optimizer_crop_results(&app, video_crop_entries, messages_data, info, sd, stopped_search);
                })
            } else {
                let video_transcode_entries = tool.get_video_transcode_entries().clone();
                sd.shared_models.lock().unwrap().shared_video_optimizer_state = Some(tool);

                let messages_data = MessagesData { critical, messages };

                a.upgrade_in_event_loop(move |app| {
                    write_video_optimizer_transcode_results(&app, video_transcode_entries, messages_data, info, sd, stopped_search);
                })
            }
        })
        .expect("Cannot start thread - not much we can do here");
}

fn write_video_optimizer_transcode_results(
    app: &MainWindow,
    video_transcode_entries: Vec<VideoTranscodeEntry>,
    messages_data: MessagesData,
    info: video_optimizer::Info,
    sd: ScanData,
    stopped_search: bool,
) {
    let scanning_time_str = format_time(info.scanning_time);
    let items_found = info.number_of_videos_to_transcode;

    let items = Rc::new(VecModel::default());

    for fe in video_transcode_entries {
        let (data_model_str, data_model_int) = prepare_data_model_video_optimizer_transcode(fe);
        insert_data_to_model(&items, data_model_str, data_model_int, None);
    }

    app.set_video_optimizer_model(items.into());
    if let Some(critical) = messages_data.critical {
        app.invoke_scan_ended(critical.into());
    } else {
        if !stopped_search && sd.basic_settings.play_audio_on_scan_completion {
            sd.audio_player.play_scan_completed();
        }
        app.invoke_scan_ended(flk!("rust_found_video_optimizer", items_found = items_found, time = scanning_time_str).into());
    }
    app.global::<GuiState>().set_info_text(messages_data.messages.into());
    reset_selection_at_end(app, ActiveTab::VideoOptimizer);
}

fn write_video_optimizer_crop_results(
    app: &MainWindow,
    video_crop_entries: Vec<VideoCropEntry>,
    messages_data: MessagesData,
    info: video_optimizer::Info,
    sd: ScanData,
    stopped_search: bool,
) {
    let scanning_time_str = format_time(info.scanning_time);
    let items_found = info.number_of_videos_to_crop;

    let items = Rc::new(VecModel::default());

    for fe in video_crop_entries {
        let Some((data_model_str, data_model_int)) = prepare_data_model_video_optimizer_crop(fe) else {
            continue;
        };
        insert_data_to_model(&items, data_model_str, data_model_int, None);
    }

    app.set_video_optimizer_model(items.into());
    if let Some(critical) = messages_data.critical {
        app.invoke_scan_ended(critical.into());
    } else {
        if !stopped_search && sd.basic_settings.play_audio_on_scan_completion {
            sd.audio_player.play_scan_completed();
        }
        app.invoke_scan_ended(flk!("rust_found_video_optimizer", items_found = items_found, time = scanning_time_str).into());
    }
    app.global::<GuiState>().set_info_text(messages_data.messages.into());
    reset_selection_at_end(app, ActiveTab::VideoOptimizer);
}

fn prepare_data_model_video_optimizer_transcode(fe: VideoTranscodeEntry) -> (ModelRc<SharedString>, ModelRc<i32>) {
    let (directory, file) = split_path(&fe.path);
    let data_model_str_arr: [SharedString; MAX_STR_DATA_VIDEO_OPTIMIZER] = [
        format_size(fe.size, BINARY).into(),
        file.into(),
        directory.into(),
        fe.codec.into(),
        format!("{}x{}", fe.width, fe.height).into(),
        "-".into(),
        get_dt_timestamp_string(fe.modified_date).into(),
        fe.thumbnail_path.as_ref().map(|e| e.to_string_lossy().to_string()).unwrap_or_default().into(),
    ];
    let data_model_str = VecModel::from_slice(&data_model_str_arr);
    let modification_split = split_u64_into_i32s(fe.modified_date);
    let size_split = split_u64_into_i32s(fe.size);
    let dimension = fe.width as i32 * fe.height as i32; // Video dimension, limited to 16K vs 16K, so no overflow
    let data_model_int_arr: [i32; MAX_INT_DATA_VIDEO_OPTIMIZER] = [
        modification_split.0,
        modification_split.1,
        size_split.0,
        size_split.1,
        dimension,
        0,
        fe.width as i32,
        fe.height as i32,
        0,
        0,
        0,
        0,
    ];
    let data_model_int = VecModel::from_slice(&data_model_int_arr);
    (data_model_str, data_model_int)
}

fn prepare_data_model_video_optimizer_crop(fe: VideoCropEntry) -> Option<(ModelRc<SharedString>, ModelRc<i32>)> {
    let (directory, file) = split_path(&fe.path);
    let (left, top, right, bottom) = fe.new_image_dimensions;

    let (_width, _height, pixels_diff, dim_string) = if left > right || top > bottom {
        error!(
            "ERROR: Invalid rectangle coordinates in cache for file \"{}\": left={}, top={}, right={}, bottom={}. Skipping dimensions display.",
            fe.path.to_string_lossy(),
            left,
            top,
            right,
            bottom
        );
        return None;
    } else {
        let new_width = (right - left) as i32;
        let new_height = (bottom - top) as i32;
        let pixels_diff = fe.width * fe.height - new_width as u32 * new_height as u32;
        (
            new_width,
            new_height,
            pixels_diff,
            format!("{}x{} ({}x{})", new_width, new_height, fe.width as i32 - new_width, fe.height as i32 - new_height),
        )
    };

    let data_model_str_arr: [SharedString; MAX_STR_DATA_VIDEO_OPTIMIZER] = [
        format_size(fe.size, BINARY).into(),
        file.into(),
        directory.into(),
        fe.codec.into(),
        format!("{}x{}", fe.width, fe.height).into(),
        dim_string.into(),
        get_dt_timestamp_string(fe.modified_date).into(),
        fe.thumbnail_path.as_ref().map(|e| e.to_string_lossy().to_string()).unwrap_or_default().into(),
    ];
    let data_model_str = VecModel::from_slice(&data_model_str_arr);
    let modification_split = split_u64_into_i32s(fe.modified_date);
    let size_split = split_u64_into_i32s(fe.size);
    let dimension = fe.width as i32 * fe.height as i32;
    let data_model_int_arr: [i32; MAX_INT_DATA_VIDEO_OPTIMIZER] = [
        modification_split.0,
        modification_split.1,
        size_split.0,
        size_split.1,
        dimension,
        pixels_diff as i32,
        fe.width as i32,
        fe.height as i32,
        left as i32,
        top as i32,
        right as i32,
        bottom as i32,
    ];
    let data_model_int = VecModel::from_slice(&data_model_int_arr);
    Some((data_model_str, data_model_int))
}
