use std::{fs, thread};

use czkawka_core::common::config_cache_path::get_config_cache_path;
use czkawka_core::tools::similar_videos::core::VIDEO_THUMBNAILS_SUBFOLDER;
use humansize::{BINARY, format_size};
use log::{error, info};
use slint::ComponentHandle;

use crate::{ActiveTab, Callabler, GuiState, MainWindow, Translations, flk};
#[derive(Debug)]
pub struct SizeCountResult {
    pub video_thumbnails_size_bytes: u64,
    pub video_thumbnails_count: u64,
    pub cache_files_size_bytes: u64,
    pub cache_files_count: u64,
    pub log_file_size_bytes: u64,
    pub log_file_count: u64,
}

fn collect_file_size_and_count(path: &std::path::Path, extensions: Option<&[&str]>) -> (u64, u64) {
    let mut total_size: u64 = 0;
    let mut total_count: u64 = 0;

    let Ok(dir_entry) = fs::read_dir(path) else {
        return (0, 0);
    };

    for entry in dir_entry.flatten() {
        let entry_path = entry.path();
        if !entry_path.is_file() {
            continue;
        }
        let Ok(metadata) = entry.metadata() else {
            continue;
        };

        if let Some(extensions) = &extensions {
            let Some(extension) = entry_path.extension().map(|e| e.to_string_lossy().to_lowercase()) else {
                continue;
            };

            if extensions.iter().any(|&ext| ext == extension) {
                total_size += metadata.len();
                total_count += 1;
            }
        } else {
            total_size += metadata.len();
            total_count += 1;
        }
    }

    (total_size, total_count)
}

pub fn cache_size_count_task(task_receiver: &std::sync::mpsc::Receiver<std::sync::mpsc::Sender<SizeCountResult>>) {
    let Some(cache_dir) = get_config_cache_path().map(|p| p.cache_folder) else {
        info!("Cannot get config cache path, skipping size of config cache calculation.");
        return;
    };
    let thumbnails_dir = cache_dir.join(VIDEO_THUMBNAILS_SUBFOLDER);
    while let Ok(sender) = task_receiver.recv() {
        let (video_thumbnails_size_bytes, video_thumbnails_count) = collect_file_size_and_count(&thumbnails_dir, Some(&["jpg"]));
        let (cache_files_size_bytes, cache_files_count) = collect_file_size_and_count(&cache_dir, Some(&["bin", "json"]));
        let (log_file_size_bytes, log_file_count) = collect_file_size_and_count(&cache_dir, Some(&["log"]));

        let result = SizeCountResult {
            video_thumbnails_size_bytes,
            video_thumbnails_count,
            cache_files_size_bytes,
            cache_files_count,
            log_file_size_bytes,
            log_file_count,
        };

        let _ = sender.send(result).inspect_err(|e| {
            error!("Failed to send size count result: {e}");
        });
    }
}

pub(crate) fn connect_size_of_config_cache(app: &MainWindow) {
    let a = app.as_weak();

    let (task_sender, task_receiver) = std::sync::mpsc::channel();

    let _join_handler = std::thread::spawn(move || {
        cache_size_count_task(&task_receiver);
    });

    app.global::<Callabler>().on_tab_changed(move || {
        let a_cloned = a.clone();
        let app = a_cloned.upgrade().expect("Failed to upgrade app :(");
        let active_tab = app.global::<GuiState>().get_active_tab();
        if active_tab != ActiveTab::Settings {
            return;
        }

        let task_sender = task_sender.clone();
        let a = a.clone();
        thread::spawn(move || {
            let (result_sender, result_receiver) = std::sync::mpsc::channel();

            let _ = task_sender.send(result_sender).inspect_err(|e| {
                error!("Failed to send size count task: {e}");
            });

            let Ok(res) = result_receiver.recv().inspect_err(|e| {
                error!("Failed to receive size count task: {e}");
            }) else {
                return;
            };
            a.upgrade_in_event_loop(move |app| {
                let translations = app.global::<Translations>();
                translations.set_settings_cache_number_size_text(
                    flk!(
                        "settings_cache_number_size_text",
                        size = format_size(res.cache_files_size_bytes, BINARY)
                        number = res.cache_files_count
                    )
                    .into(),
                );
                translations.set_settings_video_thumbnails_number_size_text(
                    flk!(
                        "settings_video_thumbnails_number_size_text",
                        size = format_size(res.video_thumbnails_size_bytes, BINARY)
                        number = res.video_thumbnails_count
                    )
                    .into(),
                );
                translations.set_settings_log_number_size_text(
                    flk!(
                        "settings_log_number_size_text",
                        size = format_size(res.log_file_size_bytes, BINARY)
                        number = res.log_file_count
                    )
                    .into(),
                );
            })
            .expect("Failed to update app info text");
        });
    });
}
