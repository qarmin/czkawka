use std::rc::Rc;
use std::thread;

use czkawka_core::common::consts::DEFAULT_THREAD_SIZE;
use czkawka_core::common::tool_data::CommonData;
use czkawka_core::common::traits::{ResultEntry, Search};
use czkawka_core::common::{format_time, split_path, split_path_compare};
use czkawka_core::tools::similar_videos;
use czkawka_core::tools::similar_videos::core::{format_bitrate_opt, format_duration_opt};
use czkawka_core::tools::similar_videos::{SimilarVideos, SimilarVideosParameters, VideosEntry};
use humansize::{BINARY, format_size};
use rayon::prelude::*;
use slint::{ComponentHandle, ModelRc, SharedString, VecModel, Weak};

use crate::common::{MAX_INT_DATA_SIMILAR_VIDEOS, MAX_STR_DATA_SIMILAR_VIDEOS, split_u64_into_i32s};
use crate::connect_scan::{MessagesData, ScanData, get_dt_timestamp_string, get_text_messages, insert_data_to_model, reset_selection_at_end, set_common_settings};
use crate::{ActiveTab, GuiState, MainWindow, flk};

pub(crate) fn scan_similar_videos(a: Weak<MainWindow>, sd: ScanData) {
    thread::Builder::new()
        .stack_size(DEFAULT_THREAD_SIZE)
        .spawn(move || {
            let params = SimilarVideosParameters::new(
                sd.custom_settings.similar_videos_sub_similarity,
                sd.custom_settings.similar_videos_sub_ignore_same_size,
                sd.custom_settings.similar_videos_skip_forward_amount,
                sd.custom_settings.similar_videos_vid_hash_duration,
                sd.combo_box_items.videos_crop_detect.value,
                sd.custom_settings.video_thumbnails_preview,
                sd.custom_settings.video_thumbnails_percentage,
                sd.custom_settings.video_thumbnails_generate_grid,
                sd.custom_settings.video_thumbnails_grid_tiles_per_side,
            );
            let mut tool = SimilarVideos::new(params);
            set_common_settings(&mut tool, &sd.custom_settings, &sd.stop_flag);

            tool.search(&sd.stop_flag, Some(&sd.progress_sender));

            let (critical, messages) = get_text_messages(&tool, &sd.basic_settings);

            let mut vector: Vec<_> = if tool.get_use_reference() {
                tool.get_similar_videos_referenced()
                    .iter()
                    .cloned()
                    .map(|(original, others)| (Some(original), others))
                    .collect()
            } else {
                tool.get_similar_videos().iter().cloned().map(|items| (None, items)).collect()
            };
            for (_first_entry, vec_fe) in &mut vector {
                vec_fe.par_sort_unstable_by(|a, b| match a.size.cmp(&b.size) {
                    std::cmp::Ordering::Equal => split_path_compare(a.path.as_path(), b.path.as_path()),
                    std::cmp::Ordering::Less => std::cmp::Ordering::Greater,
                    std::cmp::Ordering::Greater => std::cmp::Ordering::Less,
                });
            }
            vector.sort_by_key(|(_header, vc)| u64::MAX - vc.iter().map(|e| e.size).sum::<u64>()); // Also sorts by size, to show the biggest groups first

            let info = tool.get_information();
            let stopped_search = tool.get_stopped_search();
            let items_found = info.number_of_duplicates;
            let groups = info.number_of_groups;
            sd.shared_models.lock().unwrap().shared_similar_videos_state = Some(tool);

            let messages_data = MessagesData { critical, messages };

            a.upgrade_in_event_loop(move |app| {
                write_similar_videos_results(&app, vector, messages_data, info, sd, stopped_search, items_found, groups);
            })
        })
        .expect("Cannot start thread - not much we can do here");
}
fn write_similar_videos_results(
    app: &MainWindow,
    vector: Vec<(Option<VideosEntry>, Vec<VideosEntry>)>,
    messages_data: MessagesData,
    info: similar_videos::Info,
    sd: ScanData,
    stopped_search: bool,
    items_found: usize,
    groups: usize,
) {
    let scanning_time_str = format_time(info.scanning_time);

    let items = Rc::new(VecModel::default());
    for (ref_fe, vec_fe) in vector {
        if let Some(ref_fe) = ref_fe {
            let (data_model_str, data_model_int) = prepare_data_model_similar_videos(ref_fe);
            insert_data_to_model(&items, data_model_str, data_model_int, Some(true));
        } else {
            insert_data_to_model(&items, ModelRc::new(VecModel::default()), ModelRc::new(VecModel::default()), Some(false));
        }

        for fe in vec_fe {
            let (data_model_str, data_model_int) = prepare_data_model_similar_videos(fe);
            insert_data_to_model(&items, data_model_str, data_model_int, None);
        }
    }
    app.set_similar_videos_model(items.into());
    if let Some(critical) = messages_data.critical {
        app.invoke_scan_ended(critical.into());
    } else {
        if !stopped_search && sd.basic_settings.play_audio_on_scan_completion {
            sd.audio_player.play_scan_completed();
        }
        app.invoke_scan_ended(flk!("rust_found_similar_videos", items_found = items_found, groups = groups, time = scanning_time_str).into());
    }
    app.global::<GuiState>().set_info_text(messages_data.messages.into());
    reset_selection_at_end(app, ActiveTab::SimilarVideos);
}
fn prepare_data_model_similar_videos(fe: VideosEntry) -> (ModelRc<SharedString>, ModelRc<i32>) {
    let (directory, file) = split_path(fe.get_path());
    let bitrate = format_bitrate_opt(fe.bitrate);
    let fps = fe.fps.map(|e| format!("{e:.2}")).unwrap_or_default();
    let codec = fe.codec.clone().unwrap_or_default();
    let dimensions = if let (Some(w), Some(h)) = (fe.width, fe.height) {
        format!("{w}x{h}")
    } else {
        "".to_string()
    };
    let preview_path = fe.thumbnail_path.as_ref().map(|e| e.to_string_lossy().to_string()).unwrap_or_default();
    let duration = format_duration_opt(fe.duration);
    let data_model_str_arr: [SharedString; MAX_STR_DATA_SIMILAR_VIDEOS] = [
        format_size(fe.size, BINARY).into(),
        file.into(),
        directory.into(),
        dimensions.into(),
        duration.into(),
        bitrate.into(),
        fps.into(),
        codec.into(),
        get_dt_timestamp_string(fe.get_modified_date()).into(),
        preview_path.into(),
    ];
    let data_model_str = VecModel::from_slice(&data_model_str_arr);
    let modification_split = split_u64_into_i32s(fe.get_modified_date());
    let size_split = split_u64_into_i32s(fe.size);
    let bitrate_split = split_u64_into_i32s(fe.bitrate.unwrap_or(0));
    let duration_i32 = fe.duration.map_or(0, |d| (d * 100.0) as i32);
    let fps_i32 = fe.fps.map_or(0, |f| (f * 100.0) as i32);
    let dimension = fe.width.and_then(|w| fe.height.map(|h| w as i32 * h as i32)).unwrap_or_default();
    let data_model_int_arr: [i32; MAX_INT_DATA_SIMILAR_VIDEOS] = [
        modification_split.0,
        modification_split.1,
        size_split.0,
        size_split.1,
        bitrate_split.0,
        bitrate_split.1,
        duration_i32,
        fps_i32,
        dimension,
    ];
    let data_model_int = VecModel::from_slice(&data_model_int_arr);
    (data_model_str, data_model_int)
}
