use slint::{ComponentHandle, Weak};

use crate::{GuiState, MainWindow};

pub(crate) fn show_file_dialog_overlay(app: &MainWindow) {
    app.global::<GuiState>().set_file_dialog_open(true);
}

pub(crate) fn hide_file_dialog_overlay(app: &Weak<MainWindow>) {
    let app = app.clone();
    app.upgrade_in_event_loop(move |app| {
        app.global::<GuiState>().set_file_dialog_open(false);
    })
    .expect("Failed to hide file dialog overlay");
}
