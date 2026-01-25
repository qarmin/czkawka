use std::rc::Rc;
use std::thread;

use czkawka_core::common::consts::DEFAULT_THREAD_SIZE;
use czkawka_core::common::traits::{ResultEntry, Search};
use czkawka_core::common::{format_time, split_path, split_path_compare};
use czkawka_core::tools::empty_folder::{EmptyFolder, FolderEntry};
use rayon::prelude::*;
use slint::{ComponentHandle, ModelRc, SharedString, VecModel, Weak};

use crate::audio_player::AudioPlayer;
use crate::common::{MAX_INT_DATA_EMPTY_FOLDERS, MAX_STR_DATA_EMPTY_FOLDERS, split_u64_into_i32s};
use crate::connect_scan::{ScanData, get_dt_timestamp_string, get_text_messages, insert_data_to_model, reset_selection_at_end, set_common_settings};
use crate::settings::model::BasicSettings;
use crate::{ActiveTab, GuiState, MainWindow, flk};

pub(crate) fn scan_empty_folders(a: Weak<MainWindow>, sd: ScanData) {
    thread::Builder::new()
        .stack_size(DEFAULT_THREAD_SIZE)
        .spawn(move || {
            let mut tool = EmptyFolder::new();
            set_common_settings(&mut tool, &sd.custom_settings, &sd.stop_flag);
            tool.search(&sd.stop_flag, Some(&sd.progress_sender));

            let mut vector = tool.get_empty_folder_list().values().cloned().collect::<Vec<_>>();
            let (critical, messages) = get_text_messages(&tool, &sd.basic_settings);

            vector.par_sort_unstable_by(|a, b| split_path_compare(a.path.as_path(), b.path.as_path()));

            let info = tool.get_information();
            let scanning_time_str = format_time(info.scanning_time);
            let items_found = info.number_of_empty_folders;
            sd.shared_models.lock().unwrap().shared_empty_folders_state = Some(tool);

            a.upgrade_in_event_loop(move |app| {
                write_empty_folders_results(&app, vector, critical, messages, &scanning_time_str, items_found, &sd.basic_settings, &sd.audio_player);
            })
        })
        .expect("Cannot start thread - not much we can do here");
}
fn write_empty_folders_results(
    app: &MainWindow,
    vector: Vec<FolderEntry>,
    critical: Option<String>,
    messages: String,
    scanning_time_str: &str,
    items_found: usize,
    base_settings: &BasicSettings,
    audio_player: &AudioPlayer,
) {
    let items = Rc::new(VecModel::default());
    for fe in vector {
        let (data_model_str, data_model_int) = prepare_data_model_empty_folders(&fe);
        insert_data_to_model(&items, data_model_str, data_model_int, None);
    }
    app.set_empty_folder_model(items.into());
    if let Some(critical) = critical {
        app.invoke_scan_ended(critical.into());
    } else {
        if base_settings.play_audio_on_scan_completion {
            audio_player.play_scan_completed();
        }
        app.invoke_scan_ended(flk!("rust_found_empty_folders", items_found = items_found, time = scanning_time_str).into());
    }
    app.global::<GuiState>().set_info_text(messages.into());
    reset_selection_at_end(app, ActiveTab::EmptyFolders);
}

fn prepare_data_model_empty_folders(fe: &FolderEntry) -> (ModelRc<SharedString>, ModelRc<i32>) {
    let (directory, file) = split_path(&fe.path);
    let data_model_str_arr: [SharedString; MAX_STR_DATA_EMPTY_FOLDERS] = [file.into(), directory.into(), get_dt_timestamp_string(fe.modified_date).into()];
    let data_model_str = VecModel::from_slice(&data_model_str_arr);
    let modification_split = split_u64_into_i32s(fe.get_modified_date());
    let data_model_int_arr: [i32; MAX_INT_DATA_EMPTY_FOLDERS] = [modification_split.0, modification_split.1];
    let data_model_int = VecModel::from_slice(&data_model_int_arr);
    (data_model_str, data_model_int)
}
