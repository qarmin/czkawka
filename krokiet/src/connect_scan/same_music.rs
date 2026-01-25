use std::rc::Rc;
use std::thread;

use czkawka_core::common::consts::DEFAULT_THREAD_SIZE;
use czkawka_core::common::tool_data::CommonData;
use czkawka_core::common::traits::{ResultEntry, Search};
use czkawka_core::common::{format_time, split_path};
use czkawka_core::tools::same_music;
use czkawka_core::tools::same_music::core::format_audio_duration;
use czkawka_core::tools::same_music::{MusicEntry, MusicSimilarity, SameMusic, SameMusicParameters};
use humansize::{BINARY, format_size};
use slint::{ComponentHandle, ModelRc, SharedString, VecModel, Weak};

use crate::common::{MAX_INT_DATA_SIMILAR_MUSIC, MAX_STR_DATA_SIMILAR_MUSIC, split_u64_into_i32s};
use crate::connect_scan::{MessagesData, ScanData, get_dt_timestamp_string, get_text_messages, insert_data_to_model, reset_selection_at_end, set_common_settings};
use crate::{ActiveTab, GuiState, MainWindow, flk};

pub(crate) fn scan_similar_music(a: Weak<MainWindow>, sd: ScanData) {
    thread::Builder::new()
        .stack_size(DEFAULT_THREAD_SIZE)
        .spawn(move || {
            let mut music_similarity: MusicSimilarity = MusicSimilarity::NONE;
            if sd.custom_settings.similar_music_sub_title {
                music_similarity |= MusicSimilarity::TRACK_TITLE;
            }
            if sd.custom_settings.similar_music_sub_artist {
                music_similarity |= MusicSimilarity::TRACK_ARTIST;
            }
            if sd.custom_settings.similar_music_sub_bitrate {
                music_similarity |= MusicSimilarity::BITRATE;
            }
            if sd.custom_settings.similar_music_sub_length {
                music_similarity |= MusicSimilarity::LENGTH;
            }
            if sd.custom_settings.similar_music_sub_year {
                music_similarity |= MusicSimilarity::YEAR;
            }
            if sd.custom_settings.similar_music_sub_genre {
                music_similarity |= MusicSimilarity::GENRE;
            }

            let params = SameMusicParameters::new(
                music_similarity,
                sd.custom_settings.similar_music_sub_approximate_comparison,
                sd.combo_box_items.audio_check_type.value,
                sd.custom_settings.similar_music_sub_minimal_fragment_duration_value,
                sd.custom_settings.similar_music_sub_maximum_difference_value as f64,
                sd.custom_settings.similar_music_compare_fingerprints_only_with_similar_titles,
            );
            let mut tool = SameMusic::new(params);
            set_common_settings(&mut tool, &sd.custom_settings, &sd.stop_flag);

            tool.search(&sd.stop_flag, Some(&sd.progress_sender));

            let (critical, messages) = get_text_messages(&tool, &sd.basic_settings);

            let mut vector: Vec<_> = if tool.get_use_reference() {
                tool.get_similar_music_referenced()
                    .iter()
                    .cloned()
                    .map(|(original, others)| (Some(original), others))
                    .collect()
            } else {
                tool.get_duplicated_music_entries().iter().cloned().map(|items| (None, items)).collect()
            };

            vector.sort_by_cached_key(|(_, a)| u64::MAX - a.iter().map(|e| e.size).sum::<u64>());
            for (_first_entry, vec_fe) in &mut vector {
                vec_fe.sort_unstable_by_key(|a| u64::MAX - a.size);
            }

            let info = tool.get_information();
            let stopped_search = tool.get_stopped_search();
            let items_found = info.number_of_duplicates;
            let groups = info.number_of_groups;
            sd.shared_models.lock().unwrap().shared_same_music_state = Some(tool);

            let messages_data = MessagesData { critical, messages };

            a.upgrade_in_event_loop(move |app| {
                write_similar_music_results(&app, vector, messages_data, info, sd, stopped_search, items_found, groups);
            })
        })
        .expect("Cannot start thread - not much we can do here");
}
fn write_similar_music_results(
    app: &MainWindow,
    vector: Vec<(Option<MusicEntry>, Vec<MusicEntry>)>,
    messages_data: MessagesData,
    info: same_music::Info,
    sd: ScanData,
    stopped_search: bool,
    items_found: usize,
    groups: usize,
) {
    let scanning_time_str = format_time(info.scanning_time);

    let items = Rc::new(VecModel::default());
    for (ref_fe, vec_fe) in vector {
        if let Some(ref_fe) = ref_fe {
            let (data_model_str, data_model_int) = prepare_data_model_similar_music(ref_fe);
            insert_data_to_model(&items, data_model_str, data_model_int, Some(true));
        } else {
            insert_data_to_model(&items, ModelRc::new(VecModel::default()), ModelRc::new(VecModel::default()), Some(false));
        }

        for fe in vec_fe {
            let (data_model_str, data_model_int) = prepare_data_model_similar_music(fe);
            insert_data_to_model(&items, data_model_str, data_model_int, None);
        }
    }
    app.set_similar_music_model(items.into());
    if let Some(critical) = messages_data.critical {
        app.invoke_scan_ended(critical.into());
    } else {
        if !stopped_search && sd.basic_settings.play_audio_on_scan_completion {
            sd.audio_player.play_scan_completed();
        }
        app.invoke_scan_ended(flk!("rust_found_similar_music_files", items_found = items_found, groups = groups, time = scanning_time_str).into());
    }
    app.global::<GuiState>().set_info_text(messages_data.messages.into());
    reset_selection_at_end(app, ActiveTab::SimilarMusic);
}
fn prepare_data_model_similar_music(fe: MusicEntry) -> (ModelRc<SharedString>, ModelRc<i32>) {
    let (directory, file) = split_path(fe.get_path());
    let data_model_str_arr: [SharedString; MAX_STR_DATA_SIMILAR_MUSIC] = [
        format_size(fe.size, BINARY).into(),
        file.into(),
        fe.track_title.clone().into(),
        fe.track_artist.clone().into(),
        fe.year.clone().into(),
        fe.bitrate.to_string().into(),
        format_audio_duration(fe.length).into(),
        fe.genre.clone().into(),
        directory.into(),
        get_dt_timestamp_string(fe.get_modified_date()).into(),
    ];
    let data_model_str = VecModel::from_slice(&data_model_str_arr);
    let modification_split = split_u64_into_i32s(fe.get_modified_date());
    let size_split = split_u64_into_i32s(fe.size);
    let data_model_int_arr: [i32; MAX_INT_DATA_SIMILAR_MUSIC] = [modification_split.0, modification_split.1, size_split.0, size_split.1, fe.bitrate as i32, fe.length as i32];
    let data_model_int = VecModel::from_slice(&data_model_int_arr);
    (data_model_str, data_model_int)
}
