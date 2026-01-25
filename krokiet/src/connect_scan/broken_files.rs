use std::rc::Rc;
use std::thread;

use czkawka_core::common::consts::DEFAULT_THREAD_SIZE;
use czkawka_core::common::traits::{ResultEntry, Search};
use czkawka_core::common::{format_time, split_path, split_path_compare};
use czkawka_core::tools::broken_files::{BrokenEntry, BrokenFiles, BrokenFilesParameters, CheckedTypes};
use humansize::{BINARY, format_size};
use rayon::prelude::*;
use slint::{ComponentHandle, ModelRc, SharedString, VecModel, Weak};

use crate::audio_player::AudioPlayer;
use crate::common::{MAX_INT_DATA_BROKEN_FILES, MAX_STR_DATA_BROKEN_FILES, split_u64_into_i32s};
use crate::connect_scan::{ScanData, get_dt_timestamp_string, get_text_messages, insert_data_to_model, reset_selection_at_end, set_common_settings};
use crate::settings::model::BasicSettings;
use crate::{ActiveTab, GuiState, MainWindow, flk};

pub(crate) fn scan_broken_files(a: Weak<MainWindow>, sd: ScanData) {
    thread::Builder::new()
        .stack_size(DEFAULT_THREAD_SIZE)
        .spawn(move || {
            let mut checked_types: CheckedTypes = CheckedTypes::NONE;
            if sd.custom_settings.broken_files_sub_audio {
                checked_types |= CheckedTypes::AUDIO;
            }
            if sd.custom_settings.broken_files_sub_pdf {
                checked_types |= CheckedTypes::PDF;
            }
            if sd.custom_settings.broken_files_sub_image {
                checked_types |= CheckedTypes::IMAGE;
            }
            if sd.custom_settings.broken_files_sub_archive {
                checked_types |= CheckedTypes::ARCHIVE;
            }
            if sd.custom_settings.broken_files_sub_video {
                checked_types |= CheckedTypes::VIDEO;
            }

            if checked_types == CheckedTypes::NONE {
                a.upgrade_in_event_loop(move |app| {
                    app.invoke_scan_ended(flk!("rust_no_file_type_selected").into());
                })
                .expect("Cannot upgrade in event loop :(");
                return Ok(());
            }

            let params = BrokenFilesParameters::new(checked_types);
            let mut tool = BrokenFiles::new(params);
            set_common_settings(&mut tool, &sd.custom_settings, &sd.stop_flag);

            tool.search(&sd.stop_flag, Some(&sd.progress_sender));

            let mut vector = tool.get_broken_files().clone();
            let (critical, messages) = get_text_messages(&tool, &sd.basic_settings);

            vector.par_sort_unstable_by(|a, b| split_path_compare(a.path.as_path(), b.path.as_path()));

            let info = tool.get_information();
            let scanning_time_str = format_time(info.scanning_time);
            let items_found = info.number_of_broken_files;
            let size = tool.get_broken_files().iter().map(|e| e.size).sum::<u64>();
            sd.shared_models.lock().unwrap().shared_broken_files_state = Some(tool);

            a.upgrade_in_event_loop(move |app| {
                write_broken_files_results(
                    &app,
                    vector,
                    critical,
                    messages,
                    &scanning_time_str,
                    items_found,
                    size,
                    &sd.basic_settings,
                    &sd.audio_player,
                );
            })
        })
        .expect("Cannot start thread - not much we can do here");
}
fn write_broken_files_results(
    app: &MainWindow,
    vector: Vec<BrokenEntry>,
    critical: Option<String>,
    messages: String,
    scanning_time_str: &str,
    items_found: usize,
    size: u64,
    base_settings: &BasicSettings,
    audio_player: &AudioPlayer,
) {
    let items = Rc::new(VecModel::default());
    for fe in vector {
        let (data_model_str, data_model_int) = prepare_data_model_broken_files(&fe);
        insert_data_to_model(&items, data_model_str, data_model_int, None);
    }
    app.set_broken_files_model(items.into());
    if let Some(critical) = critical {
        app.invoke_scan_ended(critical.into());
    } else {
        if base_settings.play_audio_on_scan_completion {
            audio_player.play_scan_completed();
        }
        app.invoke_scan_ended(
            flk!(
                "rust_found_broken_files",
                items_found = items_found,
                time = scanning_time_str,
                size = format_size(size, BINARY)
            )
            .into(),
        );
    }
    app.global::<GuiState>().set_info_text(messages.into());
    reset_selection_at_end(app, ActiveTab::BrokenFiles);
}

fn prepare_data_model_broken_files(fe: &BrokenEntry) -> (ModelRc<SharedString>, ModelRc<i32>) {
    let (directory, file) = split_path(&fe.path);
    let data_model_str_arr: [SharedString; MAX_STR_DATA_BROKEN_FILES] = [
        file.into(),
        directory.into(),
        fe.error_string.clone().into(),
        format_size(fe.size, BINARY).into(),
        get_dt_timestamp_string(fe.modified_date).into(),
    ];
    let data_model_str = VecModel::from_slice(&data_model_str_arr);
    let modification_split = split_u64_into_i32s(fe.get_modified_date());
    let size_split = split_u64_into_i32s(fe.size);
    let data_model_int_arr: [i32; MAX_INT_DATA_BROKEN_FILES] = [modification_split.0, modification_split.1, size_split.0, size_split.1];
    let data_model_int = VecModel::from_slice(&data_model_int_arr);
    (data_model_str, data_model_int)
}
