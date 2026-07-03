use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use slint::ComponentHandle;

use crate::settings::{collect_settings_from_gui, save_settings};
use crate::thumbnail_loader::thumbnail_cache_dir;
use crate::volumes::{count_files_and_dirs_stoppable, detect_storage_volumes};
use crate::{AppState, CollectTestResult, GeneralSettings, MainWindow};

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
        window.global::<AppState>().on_open_path(|path| {
            crate::file_picker_android::open_file(path.as_str());
        });
        window.global::<AppState>().on_open_parent_folder(|path| {
            if !path.is_empty() {
                crate::file_picker_android::open_folder(path.as_str());
            }
        });
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

fn format_duration_ms(ms: u128) -> slint::SharedString {
    let s = if ms < 1_000 {
        format!("{ms} ms")
    } else if ms < 60_000 {
        let secs = ms / 1_000;
        let rem = ms % 1_000;
        if rem == 0 { format!("{secs} s") } else { format!("{secs} s {rem} ms") }
    } else {
        let min = ms / 60_000;
        let secs = (ms % 60_000) / 1_000;
        let rem = ms % 1_000;
        if rem == 0 {
            format!("{min} min {secs} s")
        } else {
            format!("{min} min {secs} s {rem} ms")
        }
    };
    slint::SharedString::from(s)
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
            win.global::<AppState>().set_collect_test_cancelled(false);

            let weak2 = win.as_weak();
            let stop2 = stop.clone();
            std::thread::spawn(move || {
                let start = std::time::Instant::now();
                let volumes = detect_storage_volumes();
                let volume_count = volumes.len() as i32;
                let mut total_files: u64 = 0;
                let mut total_folders: u64 = 0;
                let mut stopped = false;
                'outer: for vol in &volumes {
                    let canonical = std::fs::canonicalize(vol.path.as_str()).unwrap_or_else(|_| std::path::PathBuf::from(vol.path.as_str()));
                    let (f, d) = count_files_and_dirs_stoppable(&canonical, &stop2, &mut stopped);
                    total_files = total_files.saturating_add(f);
                    total_folders = total_folders.saturating_add(d);
                    if stopped {
                        break 'outer;
                    }
                }
                let elapsed_time = format_duration_ms(start.elapsed().as_millis());
                let result = CollectTestResult {
                    volumes: volume_count,
                    files: total_files.min(i32::MAX as u64) as i32,
                    folders: total_folders.min(i32::MAX as u64) as i32,
                    elapsed_time,
                };
                let _ = slint::invoke_from_event_loop(move || {
                    if let Some(win) = weak2.upgrade() {
                        win.global::<AppState>().set_collect_test_result(result);
                        win.global::<AppState>().set_collect_test_running(false);
                        win.global::<AppState>().set_collect_test_done(true);
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
                win.global::<AppState>().set_collect_test_cancelled(true);
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
                if p.is_dir() { dir_size_recursive(&p) } else { e.metadata().map_or(0, |m| m.len()) }
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

    {
        window.global::<AppState>().on_open_thumbnails_folder(move || {
            let dir = thumbnail_cache_dir();
            let _ = std::fs::create_dir_all(&dir);
            open_dir(&dir);
        });
    }

    {
        let weak = window.as_weak();
        window.global::<AppState>().on_clear_app_cache(move || {
            if let Some(cache_path) = czkawka_core::common::config_cache_path::get_config_cache_path() {
                let _ = std::fs::remove_dir_all(&cache_path.cache_folder);
            }
            if let Some(win) = weak.upgrade() {
                win.global::<AppState>().set_diag_app_cache_size("0 B".into());
            }
        });
    }

    {
        window.global::<AppState>().on_open_app_cache_folder(move || {
            if let Some(cache_path) = czkawka_core::common::config_cache_path::get_config_cache_path() {
                let _ = std::fs::create_dir_all(&cache_path.cache_folder);
                open_dir(&cache_path.cache_folder);
            }
        });
    }
}

fn open_dir(path: &Path) {
    #[cfg(not(target_os = "android"))]
    {
        let _ = std::process::Command::new("xdg-open").arg(path).spawn();
    }
    #[cfg(target_os = "android")]
    {
        if let Some(s) = path.to_str() {
            crate::file_picker_android::open_folder(s);
        }
    }
}

fn collect_log_files() -> Vec<PathBuf> {
    let Some(ccp) = czkawka_core::common::config_cache_path::get_config_cache_path() else {
        return Vec::new();
    };
    let mut files = Vec::new();
    if let Ok(entries) = std::fs::read_dir(&ccp.cache_folder) {
        for entry in entries.flatten() {
            // Picks up "cedinia.log" plus the file_rotate suffixes ("cedinia.log.<timestamp>").
            if entry.file_name().to_string_lossy().starts_with("cedinia.log") {
                files.push(entry.path());
            }
        }
    }
    files
}

fn downloads_dir() -> Option<PathBuf> {
    #[cfg(target_os = "android")]
    {
        Some(PathBuf::from("/sdcard/Download"))
    }
    #[cfg(not(target_os = "android"))]
    {
        let home = std::env::var_os("HOME")?;
        let downloads = Path::new(&home).join("Downloads");
        Some(if downloads.is_dir() { downloads } else { PathBuf::from(home) })
    }
}

// Backup alongside cedinia.log: logd already filters logcat to our own UID.
#[cfg(target_os = "android")]
fn dump_logcat(dest: &Path) -> bool {
    let Ok(output) = std::process::Command::new("/system/bin/logcat").args(["-d", "-v", "threadtime"]).output() else {
        return false;
    };
    if output.stdout.is_empty() {
        return false;
    }
    std::fs::write(dest, &output.stdout).is_ok()
}

fn export_logs_to_downloads() -> std::io::Result<PathBuf> {
    log::logger().flush();

    let downloads = downloads_dir().ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "no downloads directory"))?;
    let dest = downloads.join("cedinia_logs");
    std::fs::create_dir_all(&dest)?;

    let mut exported = 0;

    for f in collect_log_files() {
        if std::fs::metadata(&f).map_or(true, |m| m.len() == 0) {
            continue;
        }
        if let Some(name) = f.file_name()
            && std::fs::copy(&f, dest.join(name)).is_ok()
        {
            exported += 1;
        }
    }

    #[cfg(target_os = "android")]
    if dump_logcat(&dest.join("cedinia-logcat.txt")) {
        exported += 1;
    }

    if exported == 0 {
        return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "no log content to export"));
    }
    log::info!("export_logs: exported {exported} log file(s) to \"{}\"", dest.to_string_lossy());
    Ok(dest)
}

pub(crate) fn wire_export_logs(window: &MainWindow) {
    let weak = window.as_weak();
    window.global::<AppState>().on_export_logs(move || {
        let win = weak.upgrade().expect("Failed to upgrade app :(");
        if win.global::<AppState>().get_log_export_running() {
            return;
        }
        win.global::<AppState>().set_log_export_running(true);

        let weak2 = win.as_weak();
        std::thread::spawn(move || {
            let result = export_logs_to_downloads();
            let _ = slint::invoke_from_event_loop(move || {
                if let Some(win) = weak2.upgrade() {
                    let st = win.global::<AppState>();
                    st.set_log_export_running(false);
                    match result {
                        Ok(dest) => {
                            st.set_log_export_ok(true);
                            st.set_log_export_result(dest.to_string_lossy().to_string().into());
                        }
                        Err(e) => {
                            log::error!("export_logs: failed: {e}");
                            st.set_log_export_ok(false);
                            st.set_log_export_result(slint::SharedString::new());
                        }
                    }
                    st.set_log_export_done(true);
                }
            });
        });
    });
}

pub(crate) fn wire_language_change(window: &MainWindow) {
    let weak = window.as_weak();
    window.global::<AppState>().on_apply_language_change(move || {
        let win = weak.upgrade().expect("MainWindow dropped in on_apply_language_change");
        let idx = win.global::<GeneralSettings>().get_language_idx() as usize;
        let lang = czkawka_core::localizer_core::LANGUAGE_LIST.get(idx).map_or("en", |l| l.short_name);
        crate::localizer_cedinia::apply_language_preference(lang);
        crate::translations::translate_items(&win);
    });
}

pub(crate) fn wire_notification_settings(window: &MainWindow) {
    let blocked = !crate::notifications::are_system_notifications_enabled();
    window.global::<AppState>().set_system_notifications_blocked(blocked);

    window.global::<AppState>().on_open_notification_settings(|| {
        crate::notifications::open_system_notification_settings();
    });
}

pub(crate) fn wire_open_url(window: &MainWindow) {
    #[cfg(not(target_os = "android"))]
    {
        window.global::<AppState>().on_open_url(|url| {
            let _ = std::process::Command::new("xdg-open").arg(url.as_str()).spawn();
        });
    }
    #[cfg(target_os = "android")]
    {
        window.global::<AppState>().on_open_url(|url| {
            crate::file_picker_android::open_url(url.as_str());
        });
    }
}

pub(crate) fn wire_licenses_popup(window: &MainWindow) {
    let licenses_text = include_str!("../../THIRD_PARTY_LICENSES.txt");
    window.global::<AppState>().set_licenses_text(slint::SharedString::from(licenses_text));

    let weak = window.as_weak();
    window.global::<AppState>().on_show_third_party_licenses(move || {
        if let Some(win) = weak.upgrade() {
            win.global::<AppState>().set_licenses_popup_visible(true);
        }
    });
}

pub(crate) fn wire_save_settings_now(
    window: &MainWindow,
    included_dirs: Rc<std::cell::RefCell<Vec<PathBuf>>>,
    excluded_dirs: Rc<std::cell::RefCell<Vec<PathBuf>>>,
    referenced_dirs: Rc<std::cell::RefCell<Vec<PathBuf>>>,
) {
    let weak = window.as_weak();
    window.global::<AppState>().on_save_settings_now(move || {
        let win = weak.upgrade().expect("Failed to upgrade app :(");
        let settings = collect_settings_from_gui(&win);
        save_settings(&settings);
        crate::settings::save_dirs(&included_dirs.borrow(), &excluded_dirs.borrow(), &referenced_dirs.borrow());

        #[cfg(target_os = "android")]
        crate::file_picker_android::apply_theme_to_system_bars(settings.use_dark_theme);
    });
}
