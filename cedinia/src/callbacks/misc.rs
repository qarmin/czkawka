use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use slint::ComponentHandle;

use crate::settings::{collect_settings_from_gui, save_settings};
use crate::thumbnail_loader::thumbnail_cache_dir;
use crate::volumes::{count_files_and_dirs_stoppable, detect_storage_volumes};
use crate::{AppState, CollectTestResult, MainWindow};

pub(crate) fn wire_open_path(window: &MainWindow) {
    #[cfg(not(target_os = "android"))]
    {
        window.global::<AppState>().on_open_path(|path| {
            let _ = std::process::Command::new("xdg-open").arg(path.as_str()).spawn();
        });
        window.global::<AppState>().on_open_parent_folder(|path| {
            if !path.is_empty() {
                let _ = std::process::Command::new("xdg-open").arg(path.as_str()).spawn();
            }
        });
    }
    #[cfg(target_os = "android")]
    {
        window.global::<AppState>().on_open_path(|_| {});
        window.global::<AppState>().on_open_parent_folder(|_| {});
    }
}

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

pub(crate) fn wire_collect_test(window: &MainWindow) {
    let collect_stop_flag: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));

    {
        let weak = window.as_weak();
        let stop = collect_stop_flag.clone();
        window.global::<AppState>().on_run_collect_test(move || {
            let win = weak.upgrade().expect("Failed to upgrade app :(");
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
        let stop = collect_stop_flag;
        window.global::<AppState>().on_stop_collect_test(move || {
            stop.store(true, Ordering::Relaxed);
            if let Some(win) = weak.upgrade() {
                win.global::<AppState>().set_collect_test_running(false);
            }
        });
    }
}

fn dir_size_recursive(path: &Path) -> u64 {
    std::fs::read_dir(path).ok().map_or(0, |entries| {
        entries
            .flatten()
            .map(|e| {
                let p = e.path();
                if p.is_dir() {
                    dir_size_recursive(&p)
                } else {
                    e.metadata().map(|m| m.len()).unwrap_or(0)
                }
            })
            .sum()
    })
}

pub(crate) fn wire_cache_info(window: &MainWindow) {
    {
        let weak = window.as_weak();
        window.global::<AppState>().on_refresh_diag_cache_info(move || {
            let win = weak.upgrade().expect("Failed to upgrade app :(");

            if win.global::<AppState>().get_diag_refresh_running() {
                return;
            }
            win.global::<AppState>().set_diag_refresh_running(true);

            let weak2 = win.as_weak();
            std::thread::spawn(move || {
                let thumb_dir = thumbnail_cache_dir();
                let thumb_size = dir_size_recursive(&thumb_dir);

                let app_cache_size = czkawka_core::common::config_cache_path::get_config_cache_path().map_or(0, |p| dir_size_recursive(&p.cache_folder));

                let _ = slint::invoke_from_event_loop(move || {
                    if let Some(win) = weak2.upgrade() {
                        win.global::<AppState>()
                            .set_diag_thumbnails_size(humansize::format_size(thumb_size, humansize::BINARY).into());
                        win.global::<AppState>()
                            .set_diag_app_cache_size(humansize::format_size(app_cache_size, humansize::BINARY).into());
                        win.global::<AppState>().set_diag_refresh_running(false);
                    }
                });
            });
        });
    }

    {
        let weak = window.as_weak();
        window.global::<AppState>().on_clear_thumbnails_cache(move || {
            let thumb_dir = thumbnail_cache_dir();
            if let Ok(entries) = std::fs::read_dir(&thumb_dir) {
                for entry in entries.flatten() {
                    let _ = std::fs::remove_file(entry.path());
                }
            }
            if let Some(win) = weak.upgrade() {
                win.global::<AppState>().set_diag_thumbnails_size("0 B".into());
            }
        });
    }
}

pub(crate) fn wire_save_settings_now(window: &MainWindow, included_dirs: Rc<std::cell::RefCell<Vec<PathBuf>>>, excluded_dirs: Rc<std::cell::RefCell<Vec<PathBuf>>>) {
    let weak = window.as_weak();
    window.global::<AppState>().on_save_settings_now(move || {
        let win = weak.upgrade().expect("Failed to upgrade app :(");
        let settings = collect_settings_from_gui(&win);
        save_settings(&settings);
        crate::settings::save_dirs(&included_dirs.borrow(), &excluded_dirs.borrow());
    });
}
