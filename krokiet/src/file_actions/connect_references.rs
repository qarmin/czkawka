use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

use crossbeam_channel::RecvTimeoutError;
use czkawka_core::common::progress_data::ProgressData;
use czkawka_core::tools::file_repair::{ProfileStore, collect_references};
use rfd::FileDialog;
use slint::{ComponentHandle, Model, ModelRc, SharedString, VecModel, Weak};

use crate::connect_rfd::{hide_file_dialog_overlay, show_file_dialog_overlay};
use crate::{Callabler, GuiState, MainWindow, Settings, flk};

const STATE_IDLE: i32 = 0;
const STATE_RUNNING: i32 = 1;
const STATE_FINISHED: i32 = 2;

pub(crate) fn connect_references(app: &MainWindow) {
    let stop_flag = Arc::new(AtomicBool::new(false));
    connect_folder_list(app);
    connect_start_stop(app, stop_flag);
    connect_clear(app);
}

/// Resets the popup to its idle state, starting from the folders of the current scan.
pub(crate) fn prepare_references_popup(app: &MainWindow) {
    let folders = app.global::<Settings>().get_included_paths_model().iter().map(|item| item.path).collect::<Vec<_>>();
    let gui_state = app.global::<GuiState>();
    gui_state.set_references_folders(ModelRc::new(VecModel::from(folders)));
    gui_state.set_references_state(STATE_IDLE);
    gui_state.set_references_progress(0.0);
    gui_state.set_references_status_text(profiles_count_text().into());
}

fn profiles_count_text() -> String {
    flk!("rust_references_profiles_count", count = ProfileStore::ensure_loaded())
}

fn current_folders(app: &MainWindow) -> Vec<SharedString> {
    app.global::<GuiState>().get_references_folders().iter().collect()
}

fn connect_folder_list(app: &MainWindow) {
    let a = app.as_weak();
    app.global::<Callabler>().on_choose_references_folder(move || {
        let app = a.upgrade().expect("Failed to upgrade app :(");
        show_file_dialog_overlay(&app);
        let weak = a.clone();
        thread::spawn(move || {
            let folder = FileDialog::new().pick_folder();
            hide_file_dialog_overlay(&weak);
            if let Some(folder) = folder {
                weak.upgrade_in_event_loop(move |app| {
                    let mut folders = current_folders(&app);
                    folders.push(folder.to_string_lossy().to_string().into());
                    app.global::<GuiState>().set_references_folders(ModelRc::new(VecModel::from(folders)));
                })
                .expect("Failed to add the chosen folder");
            }
        });
    });

    let a = app.as_weak();
    app.global::<Callabler>().on_remove_references_folder(move |index| {
        let app = a.upgrade().expect("Failed to upgrade app :(");
        let mut folders = current_folders(&app);
        if let Ok(index) = usize::try_from(index)
            && index < folders.len()
        {
            folders.remove(index);
        }
        app.global::<GuiState>().set_references_folders(ModelRc::new(VecModel::from(folders)));
    });
}

fn connect_start_stop(app: &MainWindow, stop_flag: Arc<AtomicBool>) {
    let a = app.as_weak();
    let stop_for_start = stop_flag.clone();
    app.global::<Callabler>().on_start_collect_references(move || {
        let app = a.upgrade().expect("Failed to upgrade app :(");
        let folders = current_folders(&app).into_iter().map(|folder| PathBuf::from(folder.as_str())).collect::<Vec<_>>();
        let gui_state = app.global::<GuiState>();
        gui_state.set_references_state(STATE_RUNNING);
        gui_state.set_references_progress(0.0);
        stop_for_start.store(false, Ordering::Relaxed);
        spawn_collecting(a.clone(), folders, stop_for_start.clone());
    });

    app.global::<Callabler>().on_stop_collect_references(move || {
        stop_flag.store(true, Ordering::Relaxed);
    });
}

fn spawn_collecting(weak: Weak<MainWindow>, folders: Vec<PathBuf>, stop_flag: Arc<AtomicBool>) {
    thread::spawn(move || {
        let (progress_sender, progress_receiver) = crossbeam_channel::unbounded::<ProgressData>();
        let progress_weak = weak.clone();
        let progress_thread = thread::spawn(move || {
            loop {
                let mut latest = match progress_receiver.recv_timeout(Duration::from_millis(200)) {
                    Ok(progress) => progress,
                    Err(RecvTimeoutError::Timeout) => continue,
                    Err(RecvTimeoutError::Disconnected) => break,
                };
                while let Ok(next) = progress_receiver.try_recv() {
                    latest = next;
                }
                let fraction = if latest.entries_to_check > 0 {
                    latest.entries_checked as f32 / latest.entries_to_check as f32
                } else {
                    0.0
                };
                let text = flk!("rust_references_progress", checked = latest.entries_checked, total = latest.entries_to_check);
                let _ = progress_weak.upgrade_in_event_loop(move |app| {
                    let gui_state = app.global::<GuiState>();
                    gui_state.set_references_progress(fraction);
                    gui_state.set_references_status_text(text.into());
                });
            }
        });

        let result = collect_references(&folders, &stop_flag, Some(&progress_sender));
        drop(progress_sender);
        progress_thread.join().expect("Failed to join the progress thread");

        let text = match result {
            Ok(Some(profiles)) => flk!("rust_references_result", profiles = profiles),
            Ok(None) => flk!("rust_references_stopped"),
            Err(reason) => flk!("rust_references_failed", reason = reason),
        };
        weak.upgrade_in_event_loop(move |app| {
            let gui_state = app.global::<GuiState>();
            gui_state.set_references_state(STATE_FINISHED);
            gui_state.set_references_status_text(text.into());
        })
        .expect("Failed to show the result of collecting references");
    });
}

fn connect_clear(app: &MainWindow) {
    let a = app.as_weak();
    app.global::<Callabler>().on_clear_video_profiles(move || {
        let app = a.upgrade().expect("Failed to upgrade app :(");
        let text = match ProfileStore::clear() {
            Ok(()) => profiles_count_text(),
            Err(reason) => flk!("rust_references_failed", reason = reason),
        };
        app.global::<GuiState>().set_references_status_text(text.into());
    });
}
