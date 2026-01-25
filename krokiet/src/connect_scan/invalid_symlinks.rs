use std::rc::Rc;
use std::thread;

use czkawka_core::common::consts::DEFAULT_THREAD_SIZE;
use czkawka_core::common::tool_data::CommonData;
use czkawka_core::common::traits::{ResultEntry, Search};
use czkawka_core::common::{format_time, split_path, split_path_compare};
use czkawka_core::tools::invalid_symlinks;
use czkawka_core::tools::invalid_symlinks::{InvalidSymlinks, SymlinksFileEntry};
use rayon::prelude::*;
use slint::{ComponentHandle, ModelRc, SharedString, VecModel, Weak};

use crate::common::{MAX_INT_DATA_INVALID_SYMLINKS, MAX_STR_DATA_INVALID_SYMLINKS, split_u64_into_i32s};
use crate::connect_scan::{MessagesData, ScanData, get_dt_timestamp_string, get_text_messages, insert_data_to_model, reset_selection_at_end, set_common_settings};
use crate::{ActiveTab, GuiState, MainWindow, flk};

pub(crate) fn scan_invalid_symlinks(a: Weak<MainWindow>, sd: ScanData) {
    thread::Builder::new()
        .stack_size(DEFAULT_THREAD_SIZE)
        .spawn(move || {
            let mut tool = InvalidSymlinks::new();
            set_common_settings(&mut tool, &sd.custom_settings, &sd.stop_flag);

            tool.search(&sd.stop_flag, Some(&sd.progress_sender));

            let mut vector = tool.get_invalid_symlinks().clone();
            let (critical, messages) = get_text_messages(&tool, &sd.basic_settings);

            vector.par_sort_unstable_by(|a, b| split_path_compare(a.path.as_path(), b.path.as_path()));

            let info = tool.get_information();
            let stopped_search = tool.get_stopped_search();
            sd.shared_models.lock().unwrap().shared_same_invalid_symlinks = Some(tool);

            let messages_data = MessagesData { critical, messages };

            a.upgrade_in_event_loop(move |app| {
                write_invalid_symlinks_results(&app, vector, messages_data, info, sd, stopped_search);
            })
        })
        .expect("Cannot start thread - not much we can do here");
}
fn write_invalid_symlinks_results(app: &MainWindow, vector: Vec<SymlinksFileEntry>, messages_data: MessagesData, info: invalid_symlinks::Info, sd: ScanData, stopped_search: bool) {
    let scanning_time_str = format_time(info.scanning_time);
    let items_found = info.number_of_invalid_symlinks;

    let items = Rc::new(VecModel::default());
    for fe in vector {
        let (data_model_str, data_model_int) = prepare_data_model_invalid_symlinks(fe);
        insert_data_to_model(&items, data_model_str, data_model_int, None);
    }
    app.set_invalid_symlinks_model(items.into());
    if let Some(critical) = messages_data.critical {
        app.invoke_scan_ended(critical.into());
    } else {
        if !stopped_search && sd.basic_settings.play_audio_on_scan_completion {
            sd.audio_player.play_scan_completed();
        }
        app.invoke_scan_ended(flk!("rust_found_invalid_symlinks", items_found = items_found, time = scanning_time_str).into());
    }
    app.global::<GuiState>().set_info_text(messages_data.messages.into());
    reset_selection_at_end(app, ActiveTab::InvalidSymlinks);
}

fn prepare_data_model_invalid_symlinks(fe: SymlinksFileEntry) -> (ModelRc<SharedString>, ModelRc<i32>) {
    let (directory, file) = split_path(fe.get_path());
    let data_model_str_arr: [SharedString; MAX_STR_DATA_INVALID_SYMLINKS] = [
        file.into(),
        directory.into(),
        fe.symlink_info.destination_path.to_string_lossy().to_string().into(),
        fe.symlink_info.type_of_error.to_string().into(),
        get_dt_timestamp_string(fe.get_modified_date()).into(),
    ];
    let data_model_str = VecModel::from_slice(&data_model_str_arr);
    let modification_split = split_u64_into_i32s(fe.get_modified_date());
    let data_model_int_arr: [i32; MAX_INT_DATA_INVALID_SYMLINKS] = [modification_split.0, modification_split.1];
    let data_model_int = VecModel::from_slice(&data_model_int_arr);
    (data_model_str, data_model_int)
}
