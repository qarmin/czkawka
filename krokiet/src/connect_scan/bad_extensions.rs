use std::rc::Rc;
use std::thread;

use czkawka_core::common::consts::DEFAULT_THREAD_SIZE;
use czkawka_core::common::tool_data::CommonData;
use czkawka_core::common::traits::{ResultEntry, Search};
use czkawka_core::common::{format_time, split_path, split_path_compare};
use czkawka_core::tools::bad_extensions::{BadExtensions, BadExtensionsParameters, BadFileEntry};
use rayon::prelude::*;
use slint::{ComponentHandle, ModelRc, SharedString, VecModel, Weak};

use crate::audio_player::AudioPlayer;
use crate::common::{MAX_INT_DATA_BAD_EXTENSIONS, MAX_STR_DATA_BAD_EXTENSIONS, split_u64_into_i32s};
use crate::connect_scan::{ScanData, get_text_messages, insert_data_to_model, reset_selection_at_end, set_common_settings};
use crate::settings::model::BasicSettings;
use crate::{ActiveTab, GuiState, MainWindow, flk};

pub(crate) fn scan_bad_extensions(a: Weak<MainWindow>, sd: ScanData) {
    thread::Builder::new()
        .stack_size(DEFAULT_THREAD_SIZE)
        .spawn(move || {
            let params = BadExtensionsParameters::new();
            let mut tool = BadExtensions::new(params);
            set_common_settings(&mut tool, &sd.custom_settings, &sd.stop_flag);
            tool.search(&sd.stop_flag, Some(&sd.progress_sender));

            let mut vector = tool.get_bad_extensions_files().clone();
            let (critical, messages) = get_text_messages(&tool, &sd.basic_settings);

            vector.par_sort_unstable_by(|a, b| split_path_compare(a.path.as_path(), b.path.as_path()));

            let info = tool.get_information();
            let scanning_time_str = format_time(info.scanning_time);
            let items_found = info.number_of_files_with_bad_extension;
            sd.shared_models.lock().unwrap().shared_bad_extensions_state = Some(tool);

            a.upgrade_in_event_loop(move |app| {
                write_bad_extensions_results(&app, vector, critical, messages, &scanning_time_str, items_found, &sd.basic_settings, &sd.audio_player);
            })
        })
        .expect("Cannot start thread - not much we can do here");
}
fn write_bad_extensions_results(
    app: &MainWindow,
    vector: Vec<BadFileEntry>,
    critical: Option<String>,
    messages: String,
    scanning_time_str: &str,
    items_found: usize,
    base_settings: &BasicSettings,
    audio_player: &AudioPlayer,
) {
    let items = Rc::new(VecModel::default());
    for fe in vector {
        let (data_model_str, data_model_int) = prepare_data_model_bad_extensions(&fe);
        insert_data_to_model(&items, data_model_str, data_model_int, None);
    }
    app.set_bad_extensions_model(items.into());
    if let Some(critical) = critical {
        app.invoke_scan_ended(critical.into());
    } else {
        if base_settings.play_audio_on_scan_completion {
            audio_player.play_scan_completed();
        }
        app.invoke_scan_ended(flk!("rust_found_bad_extensions", items_found = items_found, time = scanning_time_str).into());
    }
    app.global::<GuiState>().set_info_text(messages.into());
    reset_selection_at_end(app, ActiveTab::BadExtensions);
}

fn prepare_data_model_bad_extensions(fe: &BadFileEntry) -> (ModelRc<SharedString>, ModelRc<i32>) {
    let (directory, file) = split_path(&fe.path);
    let data_model_str_arr: [SharedString; MAX_STR_DATA_BAD_EXTENSIONS] = [
        file.into(),
        directory.into(),
        fe.current_extension.clone().into(),
        fe.proper_extensions_group.clone().into(),
        fe.proper_extension.clone().into(),
    ];
    let data_model_str = VecModel::from_slice(&data_model_str_arr);
    let modification_split = split_u64_into_i32s(fe.get_modified_date());
    let size_split = split_u64_into_i32s(fe.size);
    let data_model_int_arr: [i32; MAX_INT_DATA_BAD_EXTENSIONS] = [modification_split.0, modification_split.1, size_split.0, size_split.1];
    let data_model_int = VecModel::from_slice(&data_model_int_arr);
    (data_model_str, data_model_int)
}
