use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use crate::MainWindow;

pub fn connect_stop_button(app: &MainWindow, stop_sender: Arc<AtomicBool>) {
    app.on_scan_stopping(move || {
        stop_sender.store(true, std::sync::atomic::Ordering::Relaxed);
    });
}
