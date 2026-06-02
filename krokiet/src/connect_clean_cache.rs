use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;

use crossbeam_channel::Sender;
use czkawka_core::common::cache::{CacheProgressCleaning, clean_all_cache_files};
use humansize::{BINARY, format_size};
use slint::ComponentHandle;

use crate::create_calculate_task_size::{SizeCountResult, update_cache_sizes};
use crate::{CacheCleaningProgress, CacheCleaningResult, Callabler, GuiState, MainWindow, flk};

pub(crate) fn connect_clean_cache(app: &MainWindow, cache_size_task_sender: std::sync::mpsc::Sender<std::sync::mpsc::Sender<SizeCountResult>>) {
    let app_weak = app.as_weak();
    let stop_flag: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
    let stop_flag_start: Arc<AtomicBool> = stop_flag.clone();

    app.global::<Callabler>().on_start_cache_cleaning(move || {
        let app_weak = app_weak.clone();
        let stop_flag = stop_flag_start.clone();
        let cache_size_task_sender = cache_size_task_sender.clone();
        stop_flag.store(false, Ordering::Relaxed);

        thread::spawn(move || {
            let start_time = std::time::Instant::now();

            let (progress_sender, progress_receiver): (Sender<CacheProgressCleaning>, _) = crossbeam_channel::unbounded();

            let app_weak_progress = app_weak.clone();
            let stop_flag_progress = stop_flag.clone();
            let progress_thread = thread::spawn(move || {
                while !stop_flag_progress.load(Ordering::Relaxed) {
                    // Block until the next message (or stop tick), then drain the channel
                    // so a producer running faster than the 200ms poll doesn't back up.
                    let mut latest = match progress_receiver.recv_timeout(std::time::Duration::from_millis(200)) {
                        Ok(msg) => msg,
                        Err(crossbeam_channel::RecvTimeoutError::Disconnected) => break,
                        Err(crossbeam_channel::RecvTimeoutError::Timeout) => continue,
                    };
                    while let Ok(next) = progress_receiver.try_recv() {
                        latest = next;
                    }
                    app_weak_progress
                        .upgrade_in_event_loop(move |app| {
                            let slint_progress = CacheCleaningProgress {
                                current_cache_file: latest.current_cache_file as i32,
                                total_cache_files: latest.total_cache_files as i32,
                                current_file_name: latest.current_file_name.into(),
                                checked_entries: latest.checked_entries as i32,
                                all_entries: latest.all_entries as i32,
                            };
                            app.global::<GuiState>().set_cache_cleaning_progress(slint_progress);
                        })
                        .expect("Failed to update progress in event loop");
                }
            });

            let result = clean_all_cache_files(&stop_flag, Some(&progress_sender));
            // Drop the sender so the progress thread sees Disconnected and exits its loop.
            drop(progress_sender);

            progress_thread.join().expect("Failed to join progress thread");

            let elapsed = format!("{:?}", start_time.elapsed());

            app_weak
                .upgrade_in_event_loop(move |app| {
                    let gui_state = app.global::<GuiState>();
                    gui_state.set_cache_cleaning_is_cleaning(false);
                    gui_state.set_cache_cleaning_finished(true);

                    match result {
                        Ok(stats) => {
                            let processed_files_text = flk!("rust_cache_processed_files", files = stats.total_files_found);
                            let entries_stats_text = flk!(
                                "rust_cache_entries_stats",
                                removed = stats.total_entries_removed,
                                all = stats.total_entries_before,
                                left = stats.total_entries_left
                            );
                            let size_reduced = stats.total_size_before.saturating_sub(stats.total_size_after);
                            let size_stats_text = flk!("rust_cache_size_reduced", size = format_size(size_reduced, BINARY));
                            let time_text = flk!("rust_cache_time_elapsed", time = elapsed);

                            let slint_result = CacheCleaningResult {
                                processed_files_text: processed_files_text.into(),
                                entries_stats_text: entries_stats_text.into(),
                                size_stats_text: size_stats_text.into(),
                                time_text: time_text.into(),
                                errors_count: stats.files_with_errors as i32,
                                errors: stats.errors.join("\n").into(),
                            };
                            gui_state.set_cache_cleaning_result(slint_result);
                        }
                        Err(e) => {
                            let time_text = flk!("rust_cache_time_elapsed", time = elapsed);
                            let slint_result = CacheCleaningResult {
                                processed_files_text: "".into(),
                                entries_stats_text: "".into(),
                                size_stats_text: "".into(),
                                time_text: time_text.into(),
                                errors_count: 0,
                                errors: e.into(),
                            };
                            gui_state.set_cache_cleaning_result(slint_result);
                        }
                    }

                    update_cache_sizes(&app, &cache_size_task_sender);
                })
                .expect("Failed to update final result in event loop");
        });
    });

    let stop_flag_stop = stop_flag;
    app.global::<Callabler>().on_stop_cache_cleaning(move || {
        stop_flag_stop.store(true, Ordering::Relaxed);
    });
}
