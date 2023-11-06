use crate::MainWindow;
use crossbeam_channel::Sender;

pub fn connect_stop_button(app: &MainWindow, stop_sender: Sender<()>) {
    app.on_scan_stopping(move || {
        stop_sender.send(()).unwrap();
    });
}
