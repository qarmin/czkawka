use std::rc::Rc;
use std::thread;

use czkawka_core::common::consts::DEFAULT_THREAD_SIZE;
use czkawka_core::common::traits::{ResultEntry, Search};
use czkawka_core::common::{format_time, split_path};
use czkawka_core::tools::exif_remover::{ExifEntry, ExifRemover, ExifRemoverParameters};
use humansize::{BINARY, format_size};
use rayon::prelude::*;
use slint::{ComponentHandle, ModelRc, SharedString, VecModel, Weak};

use crate::audio_player::AudioPlayer;
use crate::common::{MAX_INT_DATA_EXIF_REMOVER, MAX_STR_DATA_EXIF_REMOVER, split_u64_into_i32s};
use crate::connect_scan::{ScanData, get_dt_timestamp_string, get_text_messages, insert_data_to_model, reset_selection_at_end, set_common_settings};
use crate::settings::model::BasicSettings;
use crate::{ActiveTab, GuiState, MainWindow, flk};

pub(crate) fn scan_exif_remover(a: Weak<MainWindow>, sd: ScanData) {
    thread::Builder::new()
        .stack_size(DEFAULT_THREAD_SIZE)
        .spawn(move || {
            // Parse ignored tags from comma-separated string, trimming whitespace
            let ignored_tags: Vec<String> = sd
                .custom_settings
                .ignored_exif_tags
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();

            let params = ExifRemoverParameters::new(ignored_tags);
            let mut tool = ExifRemover::new(params);
            set_common_settings(&mut tool, &sd.custom_settings, &sd.stop_flag);
            tool.search(&sd.stop_flag, Some(&sd.progress_sender));

            let mut vector = tool.get_exif_files().clone();
            let (critical, messages) = get_text_messages(&tool, &sd.basic_settings);

            vector.par_sort_unstable_by(|a, b| b.exif_tags.len().cmp(&a.exif_tags.len()));

            let info = tool.get_information();
            let scanning_time_str = format_time(info.scanning_time);
            let items_found = info.number_of_files_with_exif;
            sd.shared_models.lock().unwrap().shared_exif_remover_state = Some(tool);

            a.upgrade_in_event_loop(move |app| {
                write_exif_remover_results(&app, vector, critical, messages, &scanning_time_str, items_found, &sd.basic_settings, &sd.audio_player);
            })
        })
        .expect("Cannot start thread - not much we can do here");
}
fn write_exif_remover_results(
    app: &MainWindow,
    vector: Vec<ExifEntry>,
    critical: Option<String>,
    messages: String,
    scanning_time_str: &str,
    items_found: usize,
    base_settings: &BasicSettings,
    audio_player: &AudioPlayer,
) {
    let items = Rc::new(VecModel::default());
    for fe in vector {
        let (data_model_str, data_model_int) = prepare_data_model_exif_remover(&fe);
        insert_data_to_model(&items, data_model_str, data_model_int, None);
    }
    app.set_exif_remover_model(items.into());
    if let Some(critical) = critical {
        app.invoke_scan_ended(critical.into());
    } else {
        if base_settings.play_audio_on_scan_completion {
            audio_player.play_scan_completed();
        }
        app.invoke_scan_ended(flk!("rust_found_exif_files", items_found = items_found, time = scanning_time_str).into());
    }
    app.global::<GuiState>().set_info_text(messages.into());
    reset_selection_at_end(app, ActiveTab::ExifRemover);
}

fn prepare_data_model_exif_remover(fe: &ExifEntry) -> (ModelRc<SharedString>, ModelRc<i32>) {
    let (directory, file) = split_path(&fe.path);
    let size_str = format_size(fe.size, BINARY);
    let exif_tags = format!(
        "{} ({})",
        fe.exif_tags.len(),
        fe.exif_tags.iter().map(|item_tag| item_tag.name.clone()).collect::<Vec<String>>().join(", ")
    );
    let exif_groups_name = fe.exif_tags.iter().map(|item_tag| item_tag.group.clone()).collect::<Vec<String>>().join(",");
    let exif_tag_u16 = fe.exif_tags.iter().map(|item_tag| item_tag.code.to_string()).collect::<Vec<String>>().join(",");
    let data_model_str_arr: [SharedString; MAX_STR_DATA_EXIF_REMOVER] = [
        size_str.into(),
        file.into(),
        directory.into(),
        exif_tags.into(),
        get_dt_timestamp_string(fe.get_modified_date()).into(),
        exif_groups_name.into(),
        exif_tag_u16.into(),
    ];
    let data_model_str = VecModel::from_slice(&data_model_str_arr);
    let modification_split = split_u64_into_i32s(fe.get_modified_date());
    let size_split = split_u64_into_i32s(fe.size);
    let data_model_int_arr: [i32; MAX_INT_DATA_EXIF_REMOVER] = [modification_split.0, modification_split.1, size_split.0, size_split.1, fe.exif_tags.len() as i32];
    let data_model_int = VecModel::from_slice(&data_model_int_arr);
    (data_model_str, data_model_int)
}
