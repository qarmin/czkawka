use crate::{Callabler, MainWindow};
use slint::ComponentHandle;

pub fn connect_open_items(app: &MainWindow) {
    app.global::<Callabler>().on_item_opened(move |path| {
        match open::that(&*path) {
            Ok(()) => {}
            Err(e) => {
                eprintln!("Failed to open file: {e}");
            }
        };
        // TODO - this should be added to line edit
    });
}
