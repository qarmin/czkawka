use std::thread;

use czkawka_core::common::consts::DEFAULT_THREAD_SIZE;
use czkawka_core::common::format_time;
use czkawka_core::common::tool_data::CommonData;
use czkawka_core::common::traits::Search;
use czkawka_core::tools::disk_usage::{DiskUsage, DiskUsageParameters, Info};
use humansize::{BINARY, format_size};
use slint::{ComponentHandle, Weak};

use crate::connect_scan::{MessagesData, ScanData, get_text_messages, set_common_settings};
use crate::{DiskUsageState, GuiState, MainWindow, flk};

pub(crate) fn scan_disk_usage(a: Weak<MainWindow>, sd: ScanData) {
    thread::Builder::new()
        .stack_size(DEFAULT_THREAD_SIZE)
        .spawn(move || {
            let params = DiskUsageParameters {
                ignore_excluded_items: sd.disk_usage_ignore_excluded_items,
                ignore_excluded_directories: sd.disk_usage_ignore_excluded_directories,
                ..DiskUsageParameters::new(sd.disk_usage_content_detection)
            };
            let mut tool = DiskUsage::new(params);

            set_common_settings(&mut tool, &sd.custom_settings, &sd.stop_flag);
            tool.search(&sd.stop_flag, Some(&sd.progress_sender));

            let (critical, messages) = get_text_messages(&tool);
            let info = tool.get_information();
            let stopped_search = tool.get_stopped_search();
            sd.shared_models.lock().expect("Mutex poisoned").shared_disk_usage_state = Some(tool);

            let messages_data = MessagesData { critical, messages };

            a.upgrade_in_event_loop(move |app| {
                write_disk_usage_results(&app, messages_data, info, sd, stopped_search);
            })
        })
        .expect("Cannot start thread - not much we can do here");
}

fn write_disk_usage_results(app: &MainWindow, messages_data: MessagesData, info: Info, sd: ScanData, stopped_search: bool) {
    let state = app.global::<DiskUsageState>();
    state.set_has_results(messages_data.critical.is_none() && info.number_of_files > 0);
    state.invoke_results_changed();

    if let Some(critical) = messages_data.critical {
        app.invoke_scan_ended(critical.into());
    } else {
        if !stopped_search && sd.basic_settings.play_audio_on_scan_completion {
            sd.audio_player.play_scan_completed();
        }
        let result_message = flk!(
            "rust_found_disk_usage",
            items_found = info.number_of_files,
            time = format_time(info.scanning_time),
            size = format_size(info.total_size, BINARY)
        );
        if !stopped_search && sd.basic_settings.show_notification_on_scan_completion {
            crate::notification_manager::send_scan_completed_notification("Disk usage", &result_message);
        }
        app.invoke_scan_ended(result_message.into());
    }
    app.global::<GuiState>().set_info_text(messages_data.messages.into());
}
