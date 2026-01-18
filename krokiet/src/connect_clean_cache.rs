use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;

use crossbeam_channel::Sender;
use czkawka_core::common::cache::{CacheProgressCleaning, clean_all_cache_files};
use humansize::{BINARY, format_size};
use slint::ComponentHandle;

use crate::{Callabler, CacheCleaningProgress, CacheCleaningResult, GuiState, MainWindow};

pub(crate) fn connect_clean_cache(app: &MainWindow) {
    let app_weak = app.as_weak();
    let stop_flag: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
    let stop_flag_start: Arc<AtomicBool> = stop_flag.clone();

    app.global::<Callabler>().on_start_cache_cleaning(move || {
        let app_weak = app_weak.clone();
        let stop_flag = stop_flag_start.clone();
        stop_flag.store(false, Ordering::Relaxed);

        thread::spawn(move || {
            let start_time = std::time::Instant::now();

            let (progress_sender, progress_receiver): (Sender<CacheProgressCleaning>, _) = crossbeam_channel::unbounded();

            let app_weak_progress = app_weak.clone();
            let stop_flag_progress = stop_flag.clone();
            let progress_thread = thread::spawn(move || {
                while !stop_flag_progress.load(Ordering::Relaxed) {
                    if let Ok(progress) = progress_receiver.recv_timeout(std::time::Duration::from_millis(200)) {
                        app_weak_progress.upgrade_in_event_loop(move |app| {
                            let slint_progress = CacheCleaningProgress {
                                current_cache_file: progress.current_cache_file as i32,
                                total_cache_files: progress.total_cache_files as i32,
                                current_file_name: progress.current_file_name.into(),
                                checked_entries: progress.checked_entries as i32,
                                all_entries: progress.all_entries as i32,
                            };
                            app.global::<GuiState>().set_cache_cleaning_progress(slint_progress);
                        }).expect("Failed to update progress in event loop");
                    }
                }
            });

            let result = clean_all_cache_files(&stop_flag, Some(&progress_sender));

            progress_thread.join().expect("Failed to join progress thread");

            let elapsed_ms = start_time.elapsed().as_millis() as i32;

            app_weak.upgrade_in_event_loop(move |app| {
                let gui_state = app.global::<GuiState>();
                gui_state.set_cache_cleaning_is_cleaning(false);
                gui_state.set_cache_cleaning_finished(true);

                match result {
                    Ok(stats) => {
                        let slint_result = CacheCleaningResult {
                            total_files_found: stats.total_files_found as i32,
                            successfully_cleaned: stats.successfully_cleaned as i32,
                            files_with_errors: stats.files_with_errors as i32,
                            total_entries_removed: stats.total_entries_removed as i32,
                            total_bytes_saved: format_size(0u64, BINARY).into(),
                            elapsed_time_ms: elapsed_ms,
                            errors: stats.errors.join("\n").into(),
                        };
                        gui_state.set_cache_cleaning_result(slint_result);
                    }
                    Err(e) => {
                        let slint_result = CacheCleaningResult {
                            total_files_found: 0,
                            successfully_cleaned: 0,
                            files_with_errors: 0,
                            total_entries_removed: 0,
                            total_bytes_saved: "".into(),
                            elapsed_time_ms: elapsed_ms,
                            errors: e.clone().into(),
                        };
                        gui_state.set_cache_cleaning_result(slint_result);
                    }
                }
            }).expect("Failed to update final result in event loop");
        });
    });

    let stop_flag_stop = stop_flag.clone();
    app.global::<Callabler>().on_stop_cache_cleaning(move || {
        stop_flag_stop.store(true, Ordering::Relaxed);
    });
}
