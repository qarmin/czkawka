use crate::MainWindow;
use slint::{ComponentHandle, Model, ModelRc, SharedString, VecModel};
use std::borrow::BorrowMut;

pub fn connect_open_items(app: &MainWindow) {
    app.on_item_opened(move |path| {
        match open::that(&*path) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Failed to open file: {}", e);
            }
        };
        // TODO - this should be added to line edit
    });
}
