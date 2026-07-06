use std::path::PathBuf;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};

use czkawka_core::common::config_cache_path::{print_infos_and_warnings, set_config_cache_path};
use czkawka_core::common::image::register_image_decoding_hooks;
#[cfg(not(target_os = "android"))]
use czkawka_core::common::logger::setup_logger;
use czkawka_core::common::logger::{filtering_messages, print_version_mode};
use slint::{ComponentHandle, Model, ModelRc, SharedString, Timer, TimerMode, VecModel, Weak};

use crate::callbacks::{
    DeleteEvent, build_excluded_model, build_included_model, get_model_for_tool, wire_cache_info, wire_collect_test, wire_directories, wire_export_logs, wire_language_change,
    wire_licenses_popup, wire_notification_settings, wire_open_path, wire_open_url, wire_permission, wire_save_settings_now, wire_scan, wire_selection,
};
use crate::compare::wire_compare;
use crate::model::make_file_model;
use crate::scan_runner::{FileItem, ScanResult, ScanResultHandler, start_worker};
use crate::set_initial_gui_infos::set_initial_gui_infos;
use crate::settings::{apply_settings_to_gui, collect_settings_from_gui, load_dirs, load_settings, save_dirs, save_settings};
use crate::thumbnail_loader::{ThumbnailData, collect_thumb_tasks, make_placeholder_image, rgba_to_slint_image, spawn_thumbnail_loader};
use crate::translations::translate_items;
use crate::volumes::home_dir;
use crate::{AppState, FileEntry, MainWindow, ProgressData, ScanState};

#[cfg(target_os = "android")]
thread_local! {
    static DIR_STATE: std::cell::RefCell<Option<(
        slint::Weak<MainWindow>,
        Rc<std::cell::RefCell<Vec<PathBuf>>>,
        Rc<std::cell::RefCell<Vec<PathBuf>>>,
        Rc<std::cell::RefCell<Vec<PathBuf>>>,
    )>> = const { std::cell::RefCell::new(None) };
}

#[cfg(target_os = "android")]
pub fn on_directory_picked(path: String, is_include: bool) {
    log::info!("on_directory_picked: path='{}' is_include={}", path, is_include);
    DIR_STATE.with(|cell| {
        let guard = cell.borrow();
        if let Some((weak, inc, exc, refr)) = guard.as_ref() {
            if let Some(win) = weak.upgrade() {
                if is_include {
                    inc.borrow_mut().push(PathBuf::from(&path));
                } else {
                    exc.borrow_mut().push(PathBuf::from(&path));
                }
                win.set_included_dirs_model(build_included_model(&inc.borrow(), &refr.borrow()));
                win.set_excluded_dirs_model(build_excluded_model(&exc.borrow()));

                let settings = crate::settings::collect_settings_from_gui(&win);
                crate::settings::save_settings(&settings);
                crate::settings::save_dirs(&inc.borrow(), &exc.borrow(), &refr.borrow());
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

fn set_tool_model(weak: Weak<MainWindow>, items: Vec<FileItem>, setter: fn(&MainWindow, ModelRc<FileEntry>)) {
    slint::invoke_from_event_loop(move || {
        let win = weak.upgrade().expect("Failed to upgrade app :(");
        setter(&win, make_file_model(items));
    })
    .expect("Failed to invoke progress update in event loop");
}

struct GuiHandler {
    weak: Weak<MainWindow>,
    scan_gen: Arc<AtomicU32>,
    thumb_tx: std::sync::mpsc::Sender<crate::thumbnail_loader::ThumbnailResult>,
    thumb_cancel: Arc<std::sync::Mutex<Arc<AtomicBool>>>,
}

impl ScanResultHandler for GuiHandler {
    fn on_result(&self, result: ScanResult) {
        let weak = self.weak.clone();
        let current_gen = self.scan_gen.load(Ordering::SeqCst);

        match result {
            ScanResult::Progress(p) => {
                slint::invoke_from_event_loop(move || {
                    let win = weak.upgrade().expect("Failed to upgrade app :(");
                    if p.scan_id != current_gen {
                        return;
                    }
                    let pd = ProgressData {
                        step_name: SharedString::from(p.step_name),
                        all_progress: p.all_progress,
                        current_progress: p.current_progress,
                        is_indeterminate: p.is_indeterminate,
                    };
                    win.global::<AppState>().set_progress(pd);
                })
                .expect("Failed to invoke progress update in event loop");
            }

            ScanResult::DuplicateFiles(items) => set_tool_model(weak, items, MainWindow::set_duplicate_files_model),
            ScanResult::EmptyFolders(items) => set_tool_model(weak, items, MainWindow::set_empty_folder_model),
            ScanResult::SimilarImages(items) => {
                let thumb_tx = self.thumb_tx.clone();
                let thumb_cancel = Arc::clone(&self.thumb_cancel);
                slint::invoke_from_event_loop(move || {
                    let win = weak.upgrade().expect("Failed to upgrade app :(");
                    let tasks = collect_thumb_tasks(&items);
                    let ph = make_placeholder_image();
                    let groups = crate::model::build_gallery_groups(&items, &ph);
                    win.set_similar_images_model(make_file_model(items));
                    win.set_similar_images_groups(ModelRc::new(VecModel::from(groups)));

                    let mut cancel_guard = thumb_cancel.lock().unwrap();
                    cancel_guard.store(true, Ordering::Relaxed);
                    let new_cancel = Arc::new(AtomicBool::new(false));
                    *cancel_guard = new_cancel.clone();
                    drop(cancel_guard);
                    spawn_thumbnail_loader(tasks, thumb_tx, new_cancel, current_gen);
                })
                .expect("Failed to invoke progress update in event loop");
            }
            ScanResult::EmptyFiles(items) => set_tool_model(weak, items, MainWindow::set_empty_files_model),
            ScanResult::TemporaryFiles(items) => set_tool_model(weak, items, MainWindow::set_temporary_files_model),
            ScanResult::BigFiles(items) => set_tool_model(weak, items, MainWindow::set_big_files_model),
            ScanResult::BrokenFiles(items) => set_tool_model(weak, items, MainWindow::set_broken_files_model),
            ScanResult::BadExtensions(items) => set_tool_model(weak, items, MainWindow::set_bad_extensions_model),
            ScanResult::SameMusic(items) => set_tool_model(weak, items, MainWindow::set_same_music_model),
            ScanResult::BadNames(items) => set_tool_model(weak, items, MainWindow::set_bad_names_model),
            ScanResult::ExifRemover(items) => set_tool_model(weak, items, MainWindow::set_exif_remover_model),
            ScanResult::SimilarVideos(items) => set_tool_model(weak, items, MainWindow::set_similar_videos_model),
            ScanResult::Finished(id) => {
                slint::invoke_from_event_loop(move || {
                    let win = weak.upgrade().expect("Failed to upgrade app :(");
                    if id != current_gen {
                        return;
                    }
                    let was_stopping = win.global::<AppState>().get_scan_state() == ScanState::Stopping;
                    if was_stopping {
                        win.global::<AppState>().set_scan_state(ScanState::Stopped);
                        win.global::<AppState>().set_status_message(SharedString::from(crate::flc!("status_stopped")));
                    } else {
                        win.global::<AppState>().set_scan_state(ScanState::Done);
                        let tool = win.global::<AppState>().get_active_tool();
                        let model = get_model_for_tool(&win, tool);
                        let file_count = (0..model.row_count()).filter(|&i| model.row_data(i).is_some_and(|e| !e.is_header)).count();
                        let status = if file_count > 0 {
                            format!("{} {file_count} {}", crate::flc!("found_items_prefix"), crate::flc!("found_items_suffix"))
                        } else {
                            crate::flc!("status_no_results")
                        };
                        win.global::<AppState>().set_status_message(SharedString::from(status));
                        if win.global::<crate::GeneralSettings>().get_show_notification() {
                            let only_bg = win.global::<crate::GeneralSettings>().get_notify_only_background();
                            crate::notifications::send_scan_completed(file_count, only_bg);
                        }
                    }
                })
                .expect("Failed to invoke progress update in event loop");
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
    std::thread::spawn(crate::thumbnail_loader::cleanup_old_thumbnails);

    let window = MainWindow::new().expect("Failed to create MainWindow");

    let loaded_settings = load_settings();
    crate::localizer_cedinia::apply_language_preference(&loaded_settings.language);
    apply_settings_to_gui(&window, &loaded_settings);

    #[cfg(target_os = "android")]
    crate::file_picker_android::apply_theme_to_system_bars(loaded_settings.use_dark_theme);

    translate_items(&window);
    set_initial_gui_infos(&window);
    window.global::<AppState>().set_status_message(SharedString::from(crate::flc!("status_ready")));

    let bot_lp = inset_bottom_px / scale;
    window.global::<AppState>().set_inset_bottom(bot_lp);

    #[cfg(target_os = "android")]
    window.global::<AppState>().set_is_desktop(false);

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

    let (saved_included, saved_excluded, saved_referenced) = load_dirs();
    let included_dirs = Rc::new(std::cell::RefCell::new(if saved_included.is_empty() { vec![home_dir()] } else { saved_included }));
    let excluded_dirs: Rc<std::cell::RefCell<Vec<PathBuf>>> = Rc::new(std::cell::RefCell::new(saved_excluded));
    let referenced_dirs: Rc<std::cell::RefCell<Vec<PathBuf>>> = Rc::new(std::cell::RefCell::new(saved_referenced));
    let scan_gen: Arc<AtomicU32> = Arc::new(AtomicU32::new(0));

    let (thumb_tx, thumb_rx) = std::sync::mpsc::channel::<crate::thumbnail_loader::ThumbnailResult>();
    let thumb_cancel: Arc<std::sync::Mutex<Arc<AtomicBool>>> = Arc::new(std::sync::Mutex::new(Arc::new(AtomicBool::new(false))));
    let placeholder: Rc<std::cell::OnceCell<slint::Image>> = Rc::new(std::cell::OnceCell::new());

    let handler = GuiHandler {
        weak: window.as_weak(),
        scan_gen: Arc::clone(&scan_gen),
        thumb_tx,
        thumb_cancel: Arc::clone(&thumb_cancel),
    };
    let (scan_tx_inner, stop_flag) = start_worker(handler);
    let scan_tx = Rc::new(scan_tx_inner);

    #[cfg(target_os = "android")]
    DIR_STATE.with(|cell| {
        *cell.borrow_mut() = Some((window.as_weak(), included_dirs.clone(), excluded_dirs.clone(), referenced_dirs.clone()));
    });

    window.set_included_dirs_model(build_included_model(&included_dirs.borrow(), &referenced_dirs.borrow()));
    window.set_excluded_dirs_model(build_excluded_model(&excluded_dirs.borrow()));

    let (delete_tx, delete_rx) = std::sync::mpsc::channel::<DeleteEvent>();
    let delete_rx = Rc::new(std::cell::RefCell::new(delete_rx));
    let delete_stop: Rc<std::cell::RefCell<Arc<AtomicBool>>> = Rc::new(std::cell::RefCell::new(Arc::new(AtomicBool::new(false))));

    wire_scan(
        &window,
        stop_flag,
        scan_tx,
        included_dirs.clone(),
        excluded_dirs.clone(),
        referenced_dirs.clone(),
        scan_gen.clone(),
    );
    wire_permission(&window);
    wire_notification_settings(&window);
    wire_selection(&window, delete_tx, Rc::clone(&delete_stop));
    wire_directories(&window, included_dirs.clone(), excluded_dirs.clone(), referenced_dirs.clone());
    wire_collect_test(&window);
    wire_open_path(&window);
    wire_language_change(&window);
    wire_open_url(&window);
    wire_cache_info(&window);
    wire_export_logs(&window);
    wire_licenses_popup(&window);
    wire_save_settings_now(&window, included_dirs.clone(), excluded_dirs.clone(), referenced_dirs.clone());
    wire_compare(&window);

    let weak = window.as_weak();
    let thumb_rx = Rc::new(std::cell::RefCell::new(thumb_rx));
    let scan_gen_poll = scan_gen;
    let delete_rx_poll = delete_rx;
    let timer = Timer::default();
    #[cfg(target_os = "android")]
    let mut perm_poll_counter: u32 = 0;
    timer.start(TimerMode::Repeated, std::time::Duration::from_millis(50), move || {
        let win = weak.upgrade().expect("Failed to upgrade MainWindow weak reference in timer");
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
                if let Some(group) = groups.row_data(tr.group_idx)
                    && let Some(mut item) = group.items.row_data(tr.item_idx)
                {
                    item.thumbnail = img;
                    group.items.set_row_data(tr.item_idx, item);
                }
            }
        }
        {
            let rx = delete_rx_poll.borrow();
            while let Ok(event) = rx.try_recv() {
                crate::file_actions::handle_delete_event(&win, event);
            }
        }

        #[cfg(target_os = "android")]
        {
            perm_poll_counter += 1;
            if perm_poll_counter >= 40 {
                perm_poll_counter = 0;
                let granted = crate::file_picker_android::check_storage_permission();
                win.global::<AppState>().set_storage_permission_granted(granted);
                let blocked = !crate::notifications::are_system_notifications_enabled();
                win.global::<AppState>().set_system_notifications_blocked(blocked);
            }
        }
    });

    window.run().expect("Failed to run MainWindow");

    let current_settings = collect_settings_from_gui(&window);
    save_settings(&current_settings);
    save_dirs(&included_dirs.borrow(), &excluded_dirs.borrow(), &referenced_dirs.borrow());
}

pub(crate) fn setup_logger_cache() {
    static INIT_DONE: std::sync::OnceLock<()> = std::sync::OnceLock::new();
    if INIT_DONE.set(()).is_err() {
        log::info!("setup_logger_cache: already initialized, skipping");
        return;
    }

    register_image_decoding_hooks();
    czkawka_core::common::build_runtime_info::BuildRuntimeInfo::get();
    let config_cache_path_set_result = set_config_cache_path("cedinia", "cedinia");

    #[cfg(not(target_os = "android"))]
    setup_logger(false, "cedinia", filtering_messages);
    #[cfg(target_os = "android")]
    setup_android_logger();

    print_version_mode("Cedinia");
    print_infos_and_warnings(config_cache_path_set_result.infos, config_cache_path_set_result.warnings);
}

#[cfg(target_os = "android")]
struct DualLogger {
    android: android_logger::AndroidLogger,
    file: std::sync::Mutex<Option<std::fs::File>>,
    level: log::LevelFilter,
}

#[cfg(target_os = "android")]
impl log::Log for DualLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &log::Record) {
        self.android.log(record);

        if !self.enabled(record.metadata()) || !filtering_messages(record) {
            return;
        }
        if let Ok(mut guard) = self.file.lock()
            && let Some(f) = guard.as_mut()
        {
            use std::io::Write;
            let ts = chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
            let _ = writeln!(f, "{ts} {:<5} [{}] {}", record.level(), record.target(), record.args());
        }
    }

    fn flush(&self) {
        self.android.flush();
        if let Ok(mut guard) = self.file.lock()
            && let Some(f) = guard.as_mut()
        {
            use std::io::Write;
            let _ = f.flush();
        }
    }
}

#[cfg(target_os = "android")]
fn setup_android_logger() {
    let android = android_logger::AndroidLogger::new(android_logger::Config::default().with_max_level(log::LevelFilter::Debug).with_tag("cedinia"));

    let file = czkawka_core::common::config_cache_path::get_config_cache_path().and_then(|p| {
        let path = p.cache_folder.join("cedinia.log");
        let truncate = std::fs::metadata(&path).map_or(false, |m| m.len() > 50 * 1024 * 1024);
        let mut opts = std::fs::OpenOptions::new();
        opts.create(true);
        if truncate {
            opts.write(true).truncate(true);
        } else {
            opts.append(true);
        }
        opts.open(&path).ok()
    });

    let logger = DualLogger {
        android,
        file: std::sync::Mutex::new(file),
        level: log::LevelFilter::Debug,
    };
    if log::set_boxed_logger(Box::new(logger)).is_ok() {
        log::set_max_level(log::LevelFilter::Debug);
    }

    // Route panics into the logger/file without pulling in the log_panics crate.
    std::panic::set_hook(Box::new(|info| {
        log::error!("PANIC: {info}");
    }));
}
