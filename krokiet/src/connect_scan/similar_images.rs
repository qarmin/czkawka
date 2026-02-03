use std::rc::Rc;
use std::thread;

use czkawka_core::common::consts::DEFAULT_THREAD_SIZE;
use czkawka_core::common::tool_data::CommonData;
use czkawka_core::common::traits::{ResultEntry, Search};
use czkawka_core::common::{format_time, split_path};
use czkawka_core::tools::similar_images;
use czkawka_core::tools::similar_images::core::get_string_from_similarity;
use czkawka_core::tools::similar_images::{ImagesEntry, SimilarImages, SimilarImagesParameters};
use humansize::{BINARY, format_size};
use rayon::prelude::*;
use slint::{ComponentHandle, ModelRc, SharedString, VecModel, Weak};

use crate::common::{MAX_INT_DATA_SIMILAR_IMAGES, MAX_STR_DATA_SIMILAR_IMAGES, split_u64_into_i32s};
use crate::connect_scan::{MessagesData, ScanData, get_dt_timestamp_string, get_text_messages, insert_data_to_model, reset_selection_at_end, set_common_settings};
use crate::{ActiveTab, GuiState, MainWindow, flk};

pub(crate) fn scan_similar_images(a: Weak<MainWindow>, sd: ScanData) {
    thread::Builder::new()
        .stack_size(DEFAULT_THREAD_SIZE)
        .spawn(move || {
            let hash_alg = sd.combo_box_items.image_hash_alg.value;
            let resize_algorithm = sd.combo_box_items.resize_algorithm.value;
            let hash_size = sd
                .custom_settings
                .similar_images_sub_hash_size
                .parse()
                .unwrap_or_else(|_| panic!("Cannot parse hash size {}", sd.custom_settings.similar_images_sub_hash_size));

            let params = SimilarImagesParameters::new(
                sd.custom_settings.similar_images_sub_similarity as u32,
                hash_size,
                hash_alg,
                resize_algorithm,
                sd.custom_settings.similar_images_sub_ignore_same_size,
            );
            let mut tool = SimilarImages::new(params);

            set_common_settings(&mut tool, &sd.custom_settings, &sd.stop_flag);

            tool.search(&sd.stop_flag, Some(&sd.progress_sender));

            let (critical, messages) = get_text_messages(&tool, &sd.basic_settings);

            let mut vector: Vec<_> = if tool.get_use_reference() {
                tool.get_similar_images_referenced()
                    .iter()
                    .cloned()
                    .map(|(original, others)| (Some(original), others))
                    .collect()
            } else {
                tool.get_similar_images().iter().cloned().map(|items| (None, items)).collect()
            };

            for (_first_entry, vec_fe) in &mut vector {
                vec_fe.par_sort_unstable_by_key(|e| (e.difference, u64::MAX - e.size));
            }
            vector.sort_by_key(|(_header, vc)| u64::MAX - vc.iter().map(|e| e.size).sum::<u64>()); // Also sorts by size, to show the biggest groups first

            let info = tool.get_information();
            let stopped_search = tool.get_stopped_search();
            let items_found = info.number_of_duplicates;
            let groups = info.number_of_groups;
            sd.shared_models.lock().unwrap().shared_similar_images_state = Some(tool);

            let messages_data = MessagesData { critical, messages };

            a.upgrade_in_event_loop(move |app| {
                write_similar_images_results(&app, vector, messages_data, info, sd, stopped_search, hash_size, items_found, groups);
            })
        })
        .expect("Cannot start thread - not much we can do here");
}
fn write_similar_images_results(
    app: &MainWindow,
    vector: Vec<(Option<ImagesEntry>, Vec<ImagesEntry>)>,
    messages_data: MessagesData,
    info: similar_images::Info,
    sd: ScanData,
    stopped_search: bool,
    hash_size: u8,
    items_found: usize,
    groups: usize,
) {
    let scanning_time_str = format_time(info.scanning_time);

    let items = Rc::new(VecModel::default());
    for (ref_fe, vec_fe) in vector {
        if let Some(ref_fe) = ref_fe {
            let (data_model_str, data_model_int) = prepare_data_model_similar_images(ref_fe, hash_size);
            insert_data_to_model(&items, data_model_str, data_model_int, Some(true));
        } else {
            insert_data_to_model(&items, ModelRc::new(VecModel::default()), ModelRc::new(VecModel::default()), Some(false));
        }

        for fe in vec_fe {
            let (data_model_str, data_model_int) = prepare_data_model_similar_images(fe, hash_size);
            insert_data_to_model(&items, data_model_str, data_model_int, None);
        }
    }
    app.set_similar_images_model(items.into());
    if let Some(critical) = messages_data.critical {
        app.invoke_scan_ended(critical.into());
    } else {
        if !stopped_search && sd.basic_settings.play_audio_on_scan_completion {
            sd.audio_player.play_scan_completed();
        }
        app.invoke_scan_ended(flk!("rust_found_similar_images", items_found = items_found, groups = groups, time = scanning_time_str).into());
    }
    app.global::<GuiState>().set_info_text(messages_data.messages.into());
    reset_selection_at_end(app, ActiveTab::SimilarImages);
}
fn prepare_data_model_similar_images(fe: ImagesEntry, hash_size: u8) -> (ModelRc<SharedString>, ModelRc<i32>) {
    let (directory, file) = split_path(fe.get_path());
    let data_model_str_arr: [SharedString; MAX_STR_DATA_SIMILAR_IMAGES] = [
        get_string_from_similarity(fe.difference, hash_size).into(),
        format_size(fe.size, BINARY).into(),
        format!("{}x{}", fe.width, fe.height).into(),
        file.into(),
        directory.into(),
        get_dt_timestamp_string(fe.get_modified_date()).into(),
    ];
    let data_model_str = VecModel::from_slice(&data_model_str_arr);
    let modification_split = split_u64_into_i32s(fe.get_modified_date());
    let size_split = split_u64_into_i32s(fe.size);
    let data_model_int_arr: [i32; MAX_INT_DATA_SIMILAR_IMAGES] = [
        modification_split.0,
        modification_split.1,
        size_split.0,
        size_split.1,
        fe.width as i32,
        fe.height as i32,
        (fe.width as u64 * fe.height as u64) as i32, // Limited to 2000MP, but using u64, because in cache it can exceed i32
    ];
    let data_model_int = VecModel::from_slice(&data_model_int_arr);
    (data_model_str, data_model_int)
}
