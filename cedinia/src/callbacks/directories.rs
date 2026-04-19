use std::path::PathBuf;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use slint::{ComponentHandle, ModelRc, SharedString, VecModel};

use crate::volumes::{detect_storage_volumes, refresh_volumes_flags};
use crate::{AppState, DirectoryEntry, MainWindow, VolumeEntry};

pub(crate) fn wire_directories(
    window: &MainWindow,
    included_dirs: Rc<std::cell::RefCell<Vec<PathBuf>>>,
    excluded_dirs: Rc<std::cell::RefCell<Vec<PathBuf>>>,
    referenced_dirs: Rc<std::cell::RefCell<Vec<PathBuf>>>,
) {
    {
        let weak = window.as_weak();
        let inc = included_dirs.clone();
        let exc = excluded_dirs.clone();
        let refr = referenced_dirs.clone();
        window.global::<AppState>().on_pick_include_dir(move || {
            #[cfg(not(target_os = "android"))]
            {
                let win = weak.upgrade().expect("MainWindow dropped while callback was still live");
                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                    let nb = normalize_path(&path);
                    if !inc.borrow().contains(&nb) {
                        inc.borrow_mut().push(nb);
                        win.set_included_dirs_model(build_included_model(&inc.borrow(), &refr.borrow()));
                    }
                    refresh_volumes_flags(&win, &inc.borrow(), &exc.borrow());
                }
            }
            #[cfg(target_os = "android")]
            {
                let _ = (&weak, &inc, &exc, &refr);
                crate::file_picker_android::launch_pick_directory(true, "");
            }
        });
    }

    {
        let weak = window.as_weak();
        let inc = included_dirs.clone();
        let exc = excluded_dirs.clone();
        window.global::<AppState>().on_pick_exclude_dir(move || {
            #[cfg(not(target_os = "android"))]
            {
                let win = weak.upgrade().expect("MainWindow dropped while callback was still live");
                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                    let nb = normalize_path(&path);
                    if !exc.borrow().contains(&nb) {
                        exc.borrow_mut().push(nb);
                        win.set_excluded_dirs_model(build_excluded_model(&exc.borrow()));
                    }
                    refresh_volumes_flags(&win, &inc.borrow(), &exc.borrow());
                }
            }
            #[cfg(target_os = "android")]
            {
                let _ = (&weak, &inc, &exc);
                crate::file_picker_android::launch_pick_directory(false, "");
            }
        });
    }

    {
        let weak = window.as_weak();
        let inc = included_dirs.clone();
        let exc = excluded_dirs.clone();
        let refr = referenced_dirs.clone();
        window.global::<AppState>().on_add_include_dir(move |path| {
            let win = weak.upgrade().expect("MainWindow dropped while callback was still live");
            let nb = normalize_path(path.as_str());
            if !inc.borrow().contains(&nb) {
                inc.borrow_mut().push(nb);
            }
            win.set_included_dirs_model(build_included_model(&inc.borrow(), &refr.borrow()));
            refresh_volumes_flags(&win, &inc.borrow(), &exc.borrow());
        });
    }

    {
        let weak = window.as_weak();
        let inc = included_dirs.clone();
        let exc = excluded_dirs.clone();
        let refr = referenced_dirs.clone();
        window.global::<AppState>().on_remove_include_dir(move |path| {
            let win = weak.upgrade().expect("MainWindow dropped while callback was still live");
            inc.borrow_mut().retain(|p| p.to_string_lossy() != path.as_str());
            refr.borrow_mut().retain(|p| p.to_string_lossy() != path.as_str());
            win.set_included_dirs_model(build_included_model(&inc.borrow(), &refr.borrow()));
            refresh_volumes_flags(&win, &inc.borrow(), &exc.borrow());
        });
    }

    {
        let weak = window.as_weak();
        let inc = included_dirs.clone();
        let exc = excluded_dirs.clone();
        window.global::<AppState>().on_add_exclude_dir(move |path| {
            let win = weak.upgrade().expect("MainWindow dropped while callback was still live");
            let nb = normalize_path(path.as_str());
            if !exc.borrow().contains(&nb) {
                exc.borrow_mut().push(nb);
            }
            win.set_excluded_dirs_model(build_excluded_model(&exc.borrow()));
            refresh_volumes_flags(&win, &inc.borrow(), &exc.borrow());
        });
    }

    {
        let weak = window.as_weak();
        let inc = included_dirs.clone();
        let exc = excluded_dirs.clone();
        window.global::<AppState>().on_remove_exclude_dir(move |path| {
            let win = weak.upgrade().expect("MainWindow dropped while callback was still live");
            exc.borrow_mut().retain(|p| p.to_string_lossy() != path.as_str());
            win.set_excluded_dirs_model(build_excluded_model(&exc.borrow()));
            refresh_volumes_flags(&win, &inc.borrow(), &exc.borrow());
        });
    }

    {
        let weak = window.as_weak();
        let inc = included_dirs.clone();
        let refr = referenced_dirs.clone();
        window.global::<AppState>().on_toggle_referenced_dir(move |path| {
            let win = weak.upgrade().expect("MainWindow dropped while callback was still live");
            let p = PathBuf::from(path.as_str());
            let mut refr_mut = refr.borrow_mut();
            if refr_mut.contains(&p) {
                refr_mut.retain(|x| x != &p);
            } else {
                refr_mut.push(p);
            }
            drop(refr_mut);
            win.set_included_dirs_model(build_included_model(&inc.borrow(), &refr.borrow()));
        });
    }

    {
        let weak = window.as_weak();
        let inc = included_dirs.clone();
        let exc = excluded_dirs.clone();
        window.global::<AppState>().on_list_storage_volumes(move || {
            let raw = detect_storage_volumes();
            let inc_set: Vec<String> = inc.borrow().iter().map(|p| p.to_string_lossy().to_string()).collect();
            let exc_set: Vec<String> = exc.borrow().iter().map(|p| p.to_string_lossy().to_string()).collect();
            let volumes: Vec<VolumeEntry> = raw
                .into_iter()
                .map(|mut v| {
                    let path = v.path.to_string();
                    v.is_included = inc_set.contains(&path);
                    v.is_excluded = exc_set.contains(&path);
                    v
                })
                .collect();
            if let Some(win) = weak.upgrade() {
                win.global::<AppState>().set_storage_volumes(ModelRc::new(VecModel::from(volumes)));
            }
        });
    }

    {
        let weak = window.as_weak();
        let inc = included_dirs.clone();
        let exc = excluded_dirs.clone();
        let refr = referenced_dirs.clone();
        window.global::<AppState>().on_pick_include_from(move |start_path| {
            #[cfg(not(target_os = "android"))]
            {
                let win = weak.upgrade().expect("MainWindow dropped while callback was still live");
                let start = PathBuf::from(start_path.as_str());
                if let Some(path) = rfd::FileDialog::new().set_directory(&start).pick_folder() {
                    let nb = normalize_path(&path);
                    if !inc.borrow().contains(&nb) {
                        inc.borrow_mut().push(nb);
                        win.set_included_dirs_model(build_included_model(&inc.borrow(), &refr.borrow()));
                    }
                    refresh_volumes_flags(&win, &inc.borrow(), &exc.borrow());
                }
            }
            #[cfg(target_os = "android")]
            {
                let _ = (&weak, &inc, &exc, &refr);
                crate::file_picker_android::launch_pick_directory(true, start_path.as_str());
            }
        });
    }

    {
        let weak = window.as_weak();
        let inc = included_dirs.clone();
        let exc = excluded_dirs.clone();
        window.global::<AppState>().on_pick_exclude_from(move |start_path| {
            #[cfg(not(target_os = "android"))]
            {
                let win = weak.upgrade().expect("MainWindow dropped while callback was still live");
                let start = PathBuf::from(start_path.as_str());
                if let Some(path) = rfd::FileDialog::new().set_directory(&start).pick_folder() {
                    let nb = normalize_path(&path);
                    if !exc.borrow().contains(&nb) {
                        exc.borrow_mut().push(nb);
                        win.set_excluded_dirs_model(build_excluded_model(&exc.borrow()));
                    }
                    refresh_volumes_flags(&win, &inc.borrow(), &exc.borrow());
                }
            }
            #[cfg(target_os = "android")]
            {
                let _ = (&weak, &inc, &exc);
                crate::file_picker_android::launch_pick_directory(false, start_path.as_str());
            }
        });
    }

    {
        let weak = window.as_weak();
        window.global::<AppState>().on_path_edit_check(move |path| {
            if let Some(win) = weak.upgrade() {
                let clean: String = path.chars().filter(|&c| c != '\n' && c != '\r').collect();
                if clean.len() != path.len() {
                    win.global::<AppState>().set_path_edit_value(clean.clone().into());
                }
                update_path_edit_status(&win, &clean);
            }
        });
    }

    {
        let weak = window.as_weak();
        let inc = included_dirs.clone();
        let exc = excluded_dirs.clone();
        let refr = referenced_dirs.clone();
        window.global::<AppState>().on_path_edit_confirm(move || {
            let win = weak.upgrade().expect("MainWindow dropped while callback was still live");
            let path = win.global::<AppState>().get_path_edit_value().to_string();
            if path.is_empty() {
                return;
            }
            let nb = normalize_path(path.as_str());
            if win.global::<AppState>().get_path_edit_is_include() {
                if !inc.borrow().contains(&nb) {
                    inc.borrow_mut().push(nb);
                }
                win.set_included_dirs_model(build_included_model(&inc.borrow(), &refr.borrow()));
            } else {
                if !exc.borrow().contains(&nb) {
                    exc.borrow_mut().push(nb);
                }
                win.set_excluded_dirs_model(build_excluded_model(&exc.borrow()));
            }
            win.global::<AppState>().set_path_edit_popup_visible(false);
            refresh_volumes_flags(&win, &inc.borrow(), &exc.borrow());
        });
    }

    {
        let dirs_check_stop: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
        {
            let weak = window.as_weak();
            let inc = included_dirs;
            let exc = excluded_dirs;
            let refr = referenced_dirs;
            let stop = dirs_check_stop.clone();
            window.global::<AppState>().on_start_dirs_check(move || {
                stop.store(false, Ordering::Relaxed);
                let inc_snap = inc.borrow().clone();
                let exc_snap = exc.borrow().clone();
                let refr_snap = refr.borrow().clone();
                let weak = weak.clone();
                let stop = stop.clone();
                if let Some(win) = weak.upgrade() {
                    win.global::<AppState>().set_dirs_check_running(true);
                    win.global::<AppState>().set_dirs_check_done(false);
                    win.global::<AppState>().set_dirs_check_no_processable(false);
                    for setter in [
                        AppState::set_dirs_check_included_text,
                        AppState::set_dirs_check_excluded_text,
                        AppState::set_dirs_check_referenced_text,
                        AppState::set_dirs_check_would_scan_text,
                        AppState::set_dirs_check_processable_text,
                    ] {
                        setter(&win.global::<AppState>(), SharedString::from("-"));
                    }
                }
                let weak_prog = weak.clone();
                std::thread::spawn(move || {
                    let result = compute_dir_stats(&inc_snap, &exc_snap, &refr_snap, &stop, |count| {
                        let w = weak_prog.clone();
                        let txt = SharedString::from(count.to_string());
                        slint::invoke_from_event_loop(move || {
                            if let Some(win) = w.upgrade() {
                                win.global::<AppState>().set_dirs_check_progress_text(txt);
                            }
                        })
                        .expect("Failed to invoke from event loop");
                    });
                    slint::invoke_from_event_loop(move || {
                        let Some(win) = weak.upgrade() else { return };
                        win.global::<AppState>().set_dirs_check_running(false);
                        win.global::<AppState>().set_dirs_check_progress_text(SharedString::from(""));
                        if let Some(s) = result {
                            win.global::<AppState>().set_dirs_check_done(true);
                            win.global::<AppState>().set_dirs_check_no_processable(s.processable_count == 0);
                            win.global::<AppState>()
                                .set_dirs_check_included_text(SharedString::from(fmt_count_size(s.included_count, s.included_size)));
                            win.global::<AppState>()
                                .set_dirs_check_excluded_text(SharedString::from(fmt_count_size(s.excluded_count, s.excluded_size)));
                            win.global::<AppState>()
                                .set_dirs_check_referenced_text(SharedString::from(fmt_count_size(s.referenced_count, s.referenced_size)));
                            win.global::<AppState>()
                                .set_dirs_check_would_scan_text(SharedString::from(fmt_count_size(s.would_scan_count, s.would_scan_size)));
                            win.global::<AppState>()
                                .set_dirs_check_processable_text(SharedString::from(fmt_count_size(s.processable_count, s.processable_size)));
                        }
                    })
                    .expect("Failed to invoke from event loop");
                });
            });
        }
        {
            window.global::<AppState>().on_stop_dirs_check(move || {
                dirs_check_stop.store(true, Ordering::Relaxed);
            });
        }
    }
}

fn normalize_path(path: impl AsRef<std::path::Path>) -> PathBuf {
    let s = path.as_ref().to_string_lossy();
    let trimmed = s.trim_end_matches(['/', '\\']);
    PathBuf::from(if trimmed.is_empty() { "/" } else { trimmed })
}

fn update_path_edit_status(win: &MainWindow, path: &str) {
    let pb = std::path::Path::new(path);
    win.global::<AppState>().set_path_edit_path_exists(pb.exists());
    win.global::<AppState>().set_path_edit_path_is_dir(pb.is_dir());
}

pub(crate) fn build_included_model(included: &[PathBuf], referenced: &[PathBuf]) -> ModelRc<DirectoryEntry> {
    let ref_set: std::collections::HashSet<String> = referenced.iter().map(|p| p.to_string_lossy().to_string()).collect();
    let entries: Vec<DirectoryEntry> = included
        .iter()
        .map(|p| {
            let path_str = p.to_string_lossy().to_string();
            let is_ref = ref_set.contains(&path_str);
            DirectoryEntry {
                path: SharedString::from(path_str),
                is_included: true,
                is_referenced: is_ref,
                exists: p.exists(),
            }
        })
        .collect();
    ModelRc::new(VecModel::from(entries))
}

pub(crate) fn build_excluded_model(excluded: &[PathBuf]) -> ModelRc<DirectoryEntry> {
    let entries: Vec<DirectoryEntry> = excluded
        .iter()
        .map(|p| DirectoryEntry {
            path: SharedString::from(p.to_string_lossy().to_string()),
            is_included: false,
            is_referenced: false,
            exists: p.exists(),
        })
        .collect();
    ModelRc::new(VecModel::from(entries))
}

#[derive(Default)]
struct DirStats {
    included_count: u64,
    included_size: u64,
    excluded_count: u64,
    excluded_size: u64,
    referenced_count: u64,
    referenced_size: u64,
    would_scan_count: u64,
    would_scan_size: u64,
    processable_count: u64,
    processable_size: u64,
}

fn walk_dir(root: &std::path::Path, stop: &Arc<AtomicBool>, cb: &mut impl FnMut(&std::path::Path, u64)) -> bool {
    if stop.load(Ordering::Relaxed) {
        return false;
    }
    let Ok(rd) = std::fs::read_dir(root) else {
        return true;
    };
    for entry in rd.flatten() {
        if stop.load(Ordering::Relaxed) {
            return false;
        }
        let Ok(file_type) = entry.file_type() else { continue };
        let path = entry.path();
        if file_type.is_dir() {
            if !walk_dir(&path, stop, cb) {
                return false;
            }
        } else if file_type.is_file() {
            let Ok(meta) = entry.metadata() else { continue };
            cb(&path, meta.len());
        }
    }
    true
}

fn canonical_or_original(p: &PathBuf) -> PathBuf {
    std::fs::canonicalize(p).unwrap_or_else(|_| p.clone())
}

fn deduplicate_dirs(dirs: &[PathBuf]) -> Vec<PathBuf> {
    let canonical: Vec<PathBuf> = dirs.iter().map(canonical_or_original).collect();
    canonical
        .iter()
        .enumerate()
        .filter(|(i, path)| !canonical.iter().enumerate().any(|(j, other)| *i != j && path.starts_with(other) && *path != other))
        .map(|(_, path)| path.clone())
        .collect()
}

fn compute_dir_stats(included: &[PathBuf], excluded: &[PathBuf], referenced: &[PathBuf], stop: &Arc<AtomicBool>, on_progress: impl Fn(u64)) -> Option<DirStats> {
    let mut stats = DirStats::default();
    let mut total: u64 = 0;
    let mut last_progress = std::time::Instant::now();

    let dedup_included = deduplicate_dirs(included);
    let dedup_excluded = deduplicate_dirs(excluded);
    let dedup_referenced = deduplicate_dirs(referenced);

    let mut seen_files: std::collections::HashSet<PathBuf> = std::collections::HashSet::new();

    for dir in &dedup_included {
        let cont = walk_dir(dir, stop, &mut |path, size| {
            total += 1;
            if last_progress.elapsed().as_millis() >= 300 {
                on_progress(total);
                last_progress = std::time::Instant::now();
            }
            let canonical_path = canonical_or_original(&path.to_path_buf());
            if seen_files.insert(canonical_path) {
                stats.included_count += 1;
                stats.included_size += size;
                if !dedup_excluded.iter().any(|e| path.starts_with(e)) {
                    stats.would_scan_count += 1;
                    stats.would_scan_size += size;
                    if !dedup_referenced.iter().any(|r| path.starts_with(r)) {
                        stats.processable_count += 1;
                        stats.processable_size += size;
                    }
                }
            }
        });
        if !cont {
            return None;
        }
    }

    for dir in &dedup_excluded {
        let cont = walk_dir(dir, stop, &mut |_, size| {
            total += 1;
            if last_progress.elapsed().as_millis() >= 300 {
                on_progress(total);
                last_progress = std::time::Instant::now();
            }
            stats.excluded_count += 1;
            stats.excluded_size += size;
        });
        if !cont {
            return None;
        }
    }

    for dir in &dedup_referenced {
        let cont = walk_dir(dir, stop, &mut |_, size| {
            total += 1;
            if last_progress.elapsed().as_millis() >= 300 {
                on_progress(total);
                last_progress = std::time::Instant::now();
            }
            stats.referenced_count += 1;
            stats.referenced_size += size;
        });
        if !cont {
            return None;
        }
    }

    Some(stats)
}

fn fmt_count_size(count: u64, bytes: u64) -> String {
    let size_str = if bytes < 1024 {
        format!("{bytes} B")
    } else if bytes < 1024 * 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else if bytes < 1024 * 1024 * 1024 {
        format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
    } else {
        format!("{:.2} GB", bytes as f64 / (1024.0 * 1024.0 * 1024.0))
    };
    format!("{count}  ({size_str})")
}
