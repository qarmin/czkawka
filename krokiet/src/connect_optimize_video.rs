use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;

use crossbeam_channel::Sender;
use czkawka_core::common::progress_data::ProgressData;
use slint::ComponentHandle;

use crate::{Callabler, GuiState, MainWindow};

pub(crate) fn connect_optimize_video(app: &MainWindow, _progress_sender: Sender<ProgressData>, stop_flag: Arc<AtomicBool>) {
    let a = app.as_weak();
    app.global::<Callabler>().on_optimize_items(move || {
        let weak_app = a.clone();
        let stop_flag = stop_flag.clone();
        stop_flag.store(false, Ordering::Relaxed);
        let app = a.upgrade().expect("Failed to upgrade app :(");
        let active_tab = app.global::<GuiState>().get_active_tab();

        thread::spawn(move || {
            // Placeholder implementation - will be implemented later
            if weak_app.upgrade().is_some() {
                println!("Optimize button clicked for tab: {active_tab:?}");
                // TODO: Implement actual optimization logic
            }
        });
    });
}
