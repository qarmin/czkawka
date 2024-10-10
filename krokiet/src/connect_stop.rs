use crossbeam_channel::Sender;

use crate::MainWindow;

pub fn connect_stop_button(app: &MainWindow, stop_sender: Sender<()>) {
    app.on_scan_stopping(move || {
        stop_sender.send(()).expect("Failed to send stop signal, no much to do");
    });
}
