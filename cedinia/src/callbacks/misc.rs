use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use slint::ComponentHandle;

use crate::volumes::{count_files_and_dirs_stoppable, detect_storage_volumes};
use crate::{AppState, CollectTestResult, MainWindow};

pub(crate) fn wire_permission(window: &MainWindow) {
    #[cfg(target_os = "android")]
    {
        let perm = crate::file_picker_android::check_storage_permission();
        window.global::<AppState>().set_storage_permission_granted(perm);
        if !perm {
            window.global::<AppState>().set_show_permission_popup(true);
        }
        window.global::<AppState>().on_request_storage_permission(move || {
            crate::file_picker_android::request_storage_permission();
        });
    }
    #[cfg(not(target_os = "android"))]
    {
        window.global::<AppState>().on_request_storage_permission(|| {});
    }
}

pub(crate) fn wire_collect_test(window: &MainWindow) -> Arc<AtomicBool> {
    let collect_stop_flag: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));

    {
        let weak = window.as_weak();
        let stop = collect_stop_flag.clone();
        window.global::<AppState>().on_run_collect_test(move || {
            let win = match weak.upgrade() {
                Some(w) => w,
                None => return,
            };
            stop.store(false, Ordering::Relaxed);
            win.global::<AppState>().set_collect_test_running(true);
            win.global::<AppState>().set_collect_test_done(false);

            let weak2 = win.as_weak();
            let stop2 = stop.clone();
            std::thread::spawn(move || {
                let start = std::time::Instant::now();
                let volumes = detect_storage_volumes();
                let volume_count = volumes.len() as i32;
                let mut total_files: i32 = 0;
                let mut total_folders: i32 = 0;
                let mut stopped = false;
                'outer: for vol in &volumes {
                    let root = std::path::Path::new(vol.path.as_str());
                    let (f, d) = count_files_and_dirs_stoppable(root, &stop2, &mut stopped);
                    total_files = total_files.saturating_add(f);
                    total_folders = total_folders.saturating_add(d);
                    if stopped {
                        break 'outer;
                    }
                }
                let elapsed_ms = start.elapsed().as_millis() as i32;
                let result = CollectTestResult {
                    volumes: volume_count,
                    files: total_files,
                    folders: total_folders,
                    elapsed_ms,
                };
                let _ = slint::invoke_from_event_loop(move || {
                    if let Some(win) = weak2.upgrade() {
                        win.global::<AppState>().set_collect_test_result(result);
                        win.global::<AppState>().set_collect_test_running(false);
                        if !stopped {
                            win.global::<AppState>().set_collect_test_done(true);
                        }
                    }
                });
            });
        });
    }

    {
        let weak = window.as_weak();
        let stop = collect_stop_flag.clone();
        window.global::<AppState>().on_stop_collect_test(move || {
            stop.store(true, Ordering::Relaxed);
            if let Some(win) = weak.upgrade() {
                win.global::<AppState>().set_collect_test_running(false);
            }
        });
    }

    collect_stop_flag
}
