use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use slint::{ComponentHandle, Model, SharedString};

use crate::{AppState, MainWindow, VolumeEntry};

pub(crate) fn home_dir() -> PathBuf {
    #[cfg(target_os = "android")]
    {
        PathBuf::from("/sdcard")
    }
    #[cfg(not(target_os = "android"))]
    {
        std::env::var("HOME").map_or_else(|_| PathBuf::from("/"), PathBuf::from)
    }
}

pub(crate) fn detect_storage_volumes() -> Vec<VolumeEntry> {
    let mut result: Vec<VolumeEntry> = Vec::new();

    #[cfg(target_os = "android")]
    let candidates = vec![
        "/sdcard",
        "/storage/emulated/0",
        "/storage/emulated/1",
        "/storage/self/primary",
        "/mnt/sdcard",
        "/mnt/extSdCard",
        "/mnt/external_sd",
        "/mnt/media_rw",
    ];
    #[cfg(not(target_os = "android"))]
    let candidates: Vec<&str> = vec![];

    let mut mounts: Vec<(String, String)> = Vec::new();
    if let Ok(content) = std::fs::read_to_string("/proc/mounts") {
        for line in content.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() < 3 {
                continue;
            }
            let device = parts[0];
            let mountpoint = parts[1];
            let fstype = parts[2];
            if fstype == "vfat"
                || fstype == "exfat"
                || fstype == "ntfs"
                || fstype == "sdcardfs"
                || fstype == "fuse"
                || mountpoint.starts_with("/storage/")
                || mountpoint.starts_with("/sdcard")
                || mountpoint.starts_with("/mnt/")
            {
                if device == "none" && !mountpoint.starts_with("/storage/") && !mountpoint.starts_with("/sdcard") {
                    continue;
                }
                mounts.push((mountpoint.to_string(), fstype.to_string()));
            }
        }
    }

    mounts.sort_by(|a, b| a.0.cmp(&b.0));
    mounts.dedup_by(|a, b| a.0 == b.0);

    for (mountpoint, _fstype) in &mounts {
        if std::fs::read_dir(mountpoint).is_ok() {
            let label = classify_mountpoint(mountpoint);
            result.push(VolumeEntry {
                path: SharedString::from(mountpoint.as_str()),
                label: SharedString::from(label),
                is_included: false,
                is_excluded: false,
            });
        }
    }

    #[cfg(target_os = "android")]
    for path in &candidates {
        let already_listed = result.iter().any(|v| v.path.as_str() == *path);
        if !already_listed && std::fs::read_dir(path).is_ok() {
            let label = classify_mountpoint(path);
            result.push(VolumeEntry {
                path: SharedString::from(*path),
                label: SharedString::from(label),
                is_included: false,
                is_excluded: false,
            });
        }
    }
    #[cfg(not(target_os = "android"))]
    let _ = candidates;

    result
}

pub(crate) fn classify_mountpoint(path: &str) -> &'static str {
    if path.contains("emulated/0") || path == "/sdcard" || path == "/storage/self/primary" || path == "/mnt/sdcard" {
        "💾 Pamięć wbudowana (internal)"
    } else if path.contains("emulated/1")
        || path.contains("extSdCard")
        || path.contains("external_sd")
        || path.contains("sdcard1")
        || path.contains("sdcard2")
        || path.starts_with("/storage/")
        || path.starts_with("/mnt/media_rw/")
    {
        "💳 Karta pamięci (SD card)"
    } else {
        "📦 Wolumin pamięci"
    }
}

pub(crate) fn refresh_volumes_flags(win: &MainWindow, included: &[PathBuf], excluded: &[PathBuf]) {
    let inc_set: Vec<String> = included.iter().map(|p| p.to_string_lossy().to_string()).collect();
    let exc_set: Vec<String> = excluded.iter().map(|p| p.to_string_lossy().to_string()).collect();
    let model = win.global::<AppState>().get_storage_volumes();
    for i in 0..model.row_count() {
        if let Some(mut vol) = model.row_data(i) {
            let path = vol.path.to_string();
            vol.is_included = inc_set.contains(&path);
            vol.is_excluded = exc_set.contains(&path);
            model.set_row_data(i, vol);
        }
    }
}

pub(crate) fn count_files_and_dirs_stoppable(root: &std::path::Path, stop: &Arc<AtomicBool>, stopped: &mut bool) -> (i32, i32) {
    if stop.load(Ordering::Relaxed) {
        *stopped = true;
        return (0, 0);
    }
    let mut files: i32 = 0;
    let mut dirs: i32 = 0;
    let Ok(rd) = std::fs::read_dir(root) else {
        return (0, 0);
    };
    for entry in rd.flatten() {
        if stop.load(Ordering::Relaxed) {
            *stopped = true;
            return (files, dirs);
        }
        let Ok(ft) = entry.file_type() else {
            continue;
        };
        if ft.is_dir() {
            dirs = dirs.saturating_add(1);
            let (f, d) = count_files_and_dirs_stoppable(&entry.path(), stop, stopped);
            files = files.saturating_add(f);
            dirs = dirs.saturating_add(d);
            if *stopped {
                return (files, dirs);
            }
        } else {
            files = files.saturating_add(1);
        }
    }
    (files, dirs)
}
