use std::path::PathBuf;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};

use czkawka_core::common::config_cache_path::{print_infos_and_warnings, set_config_cache_path};
use czkawka_core::common::image::register_image_decoding_hooks;
use czkawka_core::common::logger::{filtering_messages, print_version_mode, setup_logger};
use slint::{ComponentHandle, Model, ModelRc, SharedString, Timer, TimerMode, VecModel, Weak};

use crate::callbacks::{DeleteEvent, build_dir_model, get_model_for_tool, wire_collect_test, wire_directories, wire_permission, wire_scan, wire_selection};
use crate::model::make_file_model;
use crate::scan_runner::{FileItem, ScanResult, ScanResultHandler, start_worker};
use crate::settings::{apply_settings_to_gui, collect_settings_from_gui, load_settings, save_settings};
use crate::thumbnail_loader::{ThumbnailData, collect_thumb_tasks, make_placeholder_image, rgba_to_slint_image, spawn_thumbnail_loader};
use crate::volumes::home_dir;
use crate::{AppState, FileEntry, MainWindow, ProgressData, ScanState, SimilarGroupCard, SimilarImageItem};

#[cfg(target_os = "android")]
thread_local! {
    static DIR_STATE: std::cell::RefCell<Option<(
        slint::Weak<MainWindow>,
        Rc<std::cell::RefCell<Vec<PathBuf>>>,
        Rc<std::cell::RefCell<Vec<PathBuf>>>,
    )>> = const { std::cell::RefCell::new(None) };
}

#[cfg(target_os = "android")]
pub fn on_directory_picked(path: String, is_include: bool) {
    log::info!("on_directory_picked: path='{}' is_include={}", path, is_include);
    DIR_STATE.with(|cell| {
        let guard = cell.borrow();
        if let Some((weak, inc, exc)) = guard.as_ref() {
            if let Some(win) = weak.upgrade() {
                if is_include {
                    inc.borrow_mut().push(PathBuf::from(&path));
                } else {
                    exc.borrow_mut().push(PathBuf::from(&path));
                }
                win.set_directories_model(build_dir_model(&inc.borrow(), &exc.borrow()));
            }
        }
    });
}

pub fn run_app() {
    setup_logger_cache();

    #[cfg(target_os = "android")]
    unreachable!("use android_main");
    #[cfg(not(target_os = "android"))]
    run_app_with_insets(0.0, 1.0, ());
}

#[cfg(target_os = "android")]
pub fn run_app_with_insets(inset_bottom_px: f32, scale: f32, android_app: slint::android::AndroidApp) {
    run_app_inner(inset_bottom_px, scale, Some(android_app));
}

#[cfg(not(target_os = "android"))]
pub fn run_app_with_insets(inset_bottom_px: f32, scale: f32, _unused: ()) {
    run_app_inner(inset_bottom_px, scale, None::<()>);
}

fn build_gallery_groups(items: &[FileItem]) -> Vec<SimilarGroupCard> {
    use slint::{ModelRc, SharedString, VecModel};
    let mut groups: Vec<SimilarGroupCard> = Vec::new();
    let mut cur_label = String::new();
    let mut cur_items: Vec<SimilarImageItem> = Vec::new();

    for (flat_idx, item) in items.iter().enumerate() {
        if item.is_header {
            if !cur_items.is_empty() {
                groups.push(SimilarGroupCard {
                    label: SharedString::from(&cur_label),
                    items: ModelRc::new(VecModel::from(std::mem::take(&mut cur_items))),
                });
            }
            cur_label = item.name.clone();
        } else {
            let full_path = if item.path.is_empty() {
                item.name.clone()
            } else {
                format!("{}/{}", item.path, item.name)
            };
            cur_items.push(SimilarImageItem {
                full_path: SharedString::from(full_path),
                name: SharedString::from(&item.name),
                size: SharedString::from(&item.size),
                extra: SharedString::from(&item.extra),
                flat_idx: flat_idx as i32,
                thumbnail: Default::default(),
                checked: false,
            });
        }
    }
    if !cur_items.is_empty() {
        groups.push(SimilarGroupCard {
            label: SharedString::from(&cur_label),
            items: ModelRc::new(VecModel::from(cur_items)),
        });
    }
    groups
}

fn rebuild_similar_images_after_delete(win: &MainWindow, deleted: &std::collections::HashSet<String>) {
    let groups = win.get_similar_images_groups();
    let mut new_groups: Vec<SimilarGroupCard> = Vec::new();
    let mut new_flat: Vec<FileEntry> = Vec::new();

    for gi in 0..groups.row_count() {
        if let Some(group) = groups.row_data(gi) {
            let surviving: Vec<_> = (0..group.items.row_count())
                .filter_map(|ii| group.items.row_data(ii))
                .filter(|item| !deleted.contains(item.full_path.as_str()))
                .map(|mut item| {
                    item.checked = false;
                    item
                })
                .collect();

            if surviving.is_empty() {
                continue;
            }

            new_flat.push(FileEntry {
                checked: false,
                is_header: true,
                name: group.label.clone(),
                path: SharedString::default(),
                size: SharedString::default(),
                modified: SharedString::default(),
                extra: SharedString::default(),
            });

            let mut final_items: Vec<SimilarImageItem> = Vec::new();
            for mut item in surviving {
                item.flat_idx = new_flat.len() as i32;
                let full = item.full_path.to_string();
                let (dir, fname) = full.rfind('/').map(|p| (&full[..p], &full[p + 1..])).unwrap_or(("", &full));
                new_flat.push(FileEntry {
                    checked: false,
                    is_header: false,
                    name: SharedString::from(fname),
                    path: SharedString::from(dir),
                    size: item.size.clone(),
                    modified: SharedString::default(),
                    extra: item.extra.clone(),
                });
                final_items.push(item);
            }

            new_groups.push(SimilarGroupCard {
                label: group.label.clone(),
                items: ModelRc::new(VecModel::from(final_items)),
            });
        }
    }

    win.set_similar_images_model(ModelRc::new(VecModel::from(new_flat)));
    win.set_similar_images_groups(ModelRc::new(VecModel::from(new_groups)));
    win.global::<AppState>().set_selected_count(0);
}

struct SendWeak(Weak<MainWindow>);
unsafe impl Send for SendWeak {}
unsafe impl Sync for SendWeak {}

struct SendableTx(std::sync::mpsc::Sender<crate::thumbnail_loader::ThumbnailResult>);
unsafe impl Send for SendableTx {}
unsafe impl Sync for SendableTx {}

struct SendableThumbCancel(Arc<std::sync::Mutex<Arc<AtomicBool>>>);
unsafe impl Send for SendableThumbCancel {}
unsafe impl Sync for SendableThumbCancel {}

struct GuiHandler {
    weak: SendWeak,
    scan_gen: Arc<AtomicU32>,
    thumb_tx: SendableTx,
    thumb_cancel: SendableThumbCancel,
}

impl ScanResultHandler for GuiHandler {
    fn on_result(&self, result: ScanResult) {
        let weak = self.weak.0.clone();
        let current_gen = self.scan_gen.load(Ordering::SeqCst);

        match result {
            ScanResult::Progress(p) => {
                slint::invoke_from_event_loop(move || {
                    let Some(win) = weak.upgrade() else { return };
                    if p.scan_id != current_gen {
                        return;
                    }
                    let pd = ProgressData {
                        step_name: SharedString::from(p.step_name),
                        current_progress: p.current,
                        all_progress: p.all,
                        is_indeterminate: p.is_indeterminate,
                    };
                    win.global::<AppState>().set_progress(pd);
                })
                .ok();
            }

            ScanResult::DuplicateFiles(items) => {
                slint::invoke_from_event_loop(move || {
                    let Some(win) = weak.upgrade() else { return };
                    win.set_duplicate_files_model(make_file_model(items));
                })
                .ok();
            }
            ScanResult::EmptyFolders(items) => {
                slint::invoke_from_event_loop(move || {
                    let Some(win) = weak.upgrade() else { return };
                    win.set_empty_folder_model(make_file_model(items));
                })
                .ok();
            }
            ScanResult::SimilarImages(items) => {
                let thumb_tx = self.thumb_tx.0.clone();
                let thumb_cancel = Arc::clone(&self.thumb_cancel.0);
                slint::invoke_from_event_loop(move || {
                    let Some(win) = weak.upgrade() else { return };
                    let tasks = collect_thumb_tasks(&items);
                    let groups = build_gallery_groups(&items);
                    win.set_similar_images_model(make_file_model(items));
                    win.set_similar_images_groups(ModelRc::new(VecModel::from(groups)));

                    let mut cancel_guard = thumb_cancel.lock().unwrap();
                    cancel_guard.store(true, Ordering::Relaxed);
                    let new_cancel = Arc::new(AtomicBool::new(false));
                    *cancel_guard = new_cancel.clone();
                    drop(cancel_guard);
                    spawn_thumbnail_loader(tasks, thumb_tx, new_cancel, current_gen);
                })
                .ok();
            }
            ScanResult::EmptyFiles(items) => {
                slint::invoke_from_event_loop(move || {
                    let Some(win) = weak.upgrade() else { return };
                    win.set_empty_files_model(make_file_model(items));
                })
                .ok();
            }
            ScanResult::TemporaryFiles(items) => {
                slint::invoke_from_event_loop(move || {
                    let Some(win) = weak.upgrade() else { return };
                    win.set_temporary_files_model(make_file_model(items));
                })
                .ok();
            }
            ScanResult::BigFiles(items) => {
                slint::invoke_from_event_loop(move || {
                    let Some(win) = weak.upgrade() else { return };
                    win.set_big_files_model(make_file_model(items));
                })
                .ok();
            }
            ScanResult::BrokenFiles(items) => {
                slint::invoke_from_event_loop(move || {
                    let Some(win) = weak.upgrade() else { return };
                    win.set_broken_files_model(make_file_model(items));
                })
                .ok();
            }
            ScanResult::InvalidSymlinks(items) => {
                slint::invoke_from_event_loop(move || {
                    let Some(win) = weak.upgrade() else { return };
                    win.set_invalid_symlinks_model(make_file_model(items));
                })
                .ok();
            }
            ScanResult::BadExtensions(items) => {
                slint::invoke_from_event_loop(move || {
                    let Some(win) = weak.upgrade() else { return };
                    win.set_bad_extensions_model(make_file_model(items));
                })
                .ok();
            }
            ScanResult::SameMusic(items) => {
                slint::invoke_from_event_loop(move || {
                    let Some(win) = weak.upgrade() else { return };
                    win.set_same_music_model(make_file_model(items));
                })
                .ok();
            }
            ScanResult::Finished(id) => {
                slint::invoke_from_event_loop(move || {
                    let Some(win) = weak.upgrade() else { return };
                    if id != current_gen {
                        return;
                    }
                    let was_stopping = win.global::<AppState>().get_scan_state() == ScanState::Stopping;
                    if was_stopping {
                        win.global::<AppState>().set_scan_state(ScanState::Stopped);
                        win.global::<AppState>().set_status_message(SharedString::from("Zatrzymano"));
                    } else {
                        win.global::<AppState>().set_scan_state(ScanState::Done);
                        let tool = win.global::<AppState>().get_active_tool();
                        let model = get_model_for_tool(&win, tool);
                        let file_count = (0..model.row_count()).filter(|&i| model.row_data(i).map(|e| !e.is_header).unwrap_or(false)).count();
                        let status = if file_count > 0 {
                            format!("Znaleziono {file_count} elementów")
                        } else {
                            "Brak wyników".to_string()
                        };
                        win.global::<AppState>().set_status_message(SharedString::from(status));
                    }
                })
                .ok();
            }
        }
    }
}

fn run_app_inner(
    inset_bottom_px: f32,
    scale: f32,
    #[cfg(target_os = "android")] android_app: Option<slint::android::AndroidApp>,
    #[cfg(not(target_os = "android"))] _android_app: Option<()>,
) {
    let window = MainWindow::new().expect("Failed to create MainWindow");

    let loaded_settings = load_settings();
    apply_settings_to_gui(&window, &loaded_settings);

    let bot_lp = inset_bottom_px / scale;
    window.global::<AppState>().set_inset_bottom(bot_lp);

    #[cfg(target_os = "android")]
    {
        if let Some(app) = android_app {
            let weak = window.as_weak();
            let inset_timer = Rc::new(Timer::default());
            let inset_timer_clone = inset_timer.clone();
            inset_timer.start(TimerMode::Repeated, std::time::Duration::from_millis(50), move || {
                let rect = app.content_rect();
                if rect.bottom > 0 {
                    if let Some(win) = weak.upgrade() {
                        let window_height = win.window().size().height as f32;
                        let nav_bar_px = window_height - rect.bottom as f32;
                        if nav_bar_px > 0.0 {
                            win.global::<AppState>().set_inset_bottom(nav_bar_px / scale);
                        }
                    }
                    inset_timer_clone.stop();
                }
            });
        }
    }

    let included_dirs = Rc::new(std::cell::RefCell::new(vec![home_dir()]));
    let excluded_dirs: Rc<std::cell::RefCell<Vec<PathBuf>>> = Rc::new(std::cell::RefCell::new(vec![]));
    let scan_gen: Arc<AtomicU32> = Arc::new(AtomicU32::new(0));

    let (thumb_tx, thumb_rx) = std::sync::mpsc::channel::<crate::thumbnail_loader::ThumbnailResult>();
    let thumb_cancel: Arc<std::sync::Mutex<Arc<AtomicBool>>> = Arc::new(std::sync::Mutex::new(Arc::new(AtomicBool::new(false))));
    let placeholder: Rc<std::cell::OnceCell<slint::Image>> = Rc::new(std::cell::OnceCell::new());

    let handler = GuiHandler {
        weak: SendWeak(window.as_weak()),
        scan_gen: Arc::clone(&scan_gen),
        thumb_tx: SendableTx(thumb_tx),
        thumb_cancel: SendableThumbCancel(Arc::clone(&thumb_cancel)),
    };
    let (scan_tx_inner, stop_flag) = start_worker(handler);
    let scan_tx = Rc::new(scan_tx_inner);

    #[cfg(target_os = "android")]
    DIR_STATE.with(|cell| {
        *cell.borrow_mut() = Some((window.as_weak(), included_dirs.clone(), excluded_dirs.clone()));
    });

    window.set_directories_model(build_dir_model(&included_dirs.borrow(), &excluded_dirs.borrow()));

    let (delete_tx, delete_rx) = std::sync::mpsc::channel::<DeleteEvent>();
    let delete_rx = Rc::new(std::cell::RefCell::new(delete_rx));
    let delete_stop: Rc<std::cell::RefCell<Arc<AtomicBool>>> = Rc::new(std::cell::RefCell::new(Arc::new(AtomicBool::new(false))));

    wire_scan(&window, stop_flag, scan_tx, included_dirs.clone(), scan_gen.clone());
    wire_permission(&window);
    wire_selection(&window, delete_tx, Rc::clone(&delete_stop));
    wire_directories(&window, included_dirs.clone(), excluded_dirs.clone());
    wire_collect_test(&window);

    let weak = window.as_weak();
    let thumb_rx = Rc::new(std::cell::RefCell::new(thumb_rx));
    let scan_gen_poll = scan_gen;
    let delete_rx_poll = delete_rx;
    let timer = Timer::default();
    #[cfg(target_os = "android")]
    let mut perm_poll_counter: u32 = 0;
    timer.start(TimerMode::Repeated, std::time::Duration::from_millis(50), move || {
        let win = match weak.upgrade() {
            Some(w) => w,
            None => return,
        };
        let current_gen = scan_gen_poll.load(Ordering::SeqCst);

        {
            let rx = thumb_rx.borrow();
            while let Ok(tr) = rx.try_recv() {
                if tr.scan_id != current_gen {
                    continue;
                }
                let img = match tr.data {
                    ThumbnailData::Loaded(rgba, w, h) => rgba_to_slint_image(&rgba, w, h),
                    ThumbnailData::Placeholder => placeholder.get_or_init(make_placeholder_image).clone(),
                };
                let groups = win.get_similar_images_groups();
                if let Some(group) = groups.row_data(tr.group_idx) {
                    if let Some(mut item) = group.items.row_data(tr.item_idx) {
                        item.thumbnail = img;
                        group.items.set_row_data(tr.item_idx, item);
                    }
                }
            }
        }
        {
            let rx = delete_rx_poll.borrow();
            while let Ok(event) = rx.try_recv() {
                match event {
                    DeleteEvent::Progress(done, total) => {
                        win.global::<AppState>().set_delete_progress_text(SharedString::from(format!("{} / {}", done, total)));
                    }
                    DeleteEvent::Finished(deleted, errors) => {
                        win.global::<AppState>().set_delete_running(false);

                        if !deleted.is_empty() {
                            let del_set: std::collections::HashSet<String> = deleted.into_iter().collect();
                            rebuild_similar_images_after_delete(&win, &del_set);
                        }

                        let status = if errors.is_empty() { "Usunięto zaznaczone" } else { "Usunięto z błędami" };
                        win.global::<AppState>().set_status_message(SharedString::from(status));

                        if !errors.is_empty() {
                            let displayed: Vec<&String> = errors.iter().take(10).collect();
                            let mut msg = displayed.iter().map(|s| s.as_str()).collect::<Vec<_>>().join("\n\n");
                            if errors.len() > 10 {
                                msg.push_str(&format!("\n\n…i {} więcej", errors.len() - 10));
                            }
                            win.global::<AppState>().set_delete_errors_text(SharedString::from(msg));
                            win.global::<AppState>().set_delete_errors_visible(true);
                        }
                    }
                }
            }
        }
        #[cfg(target_os = "android")]
        {
            perm_poll_counter += 1;
            if perm_poll_counter >= 40 {
                perm_poll_counter = 0;
                let granted = crate::file_picker_android::check_storage_permission();
                win.global::<AppState>().set_storage_permission_granted(granted);
            }
        }
    });

    window.run().expect("Failed to run MainWindow");

    let current_settings = collect_settings_from_gui(&window);
    save_settings(&current_settings);
}

pub(crate) fn setup_logger_cache() {
    unsafe {
        std::env::set_var("RUST_LOG", "debug");
    }

    register_image_decoding_hooks();
    let config_cache_path_set_result = set_config_cache_path("Czkawka", "Krokiet");

    setup_logger(false, "cedinia", filtering_messages);
    print_version_mode("Cedinia");
    print_infos_and_warnings(config_cache_path_set_result.infos, config_cache_path_set_result.warnings);
}
