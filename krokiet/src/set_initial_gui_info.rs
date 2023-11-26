use crate::GuiState;
use crate::{Callabler, CurrentTab, MainListModel, MainWindow};
use log::info;
use slint::{ComponentHandle, Model, ModelRc, VecModel};
use std::thread;

fn set_initial_gui_infos(app: &MainWindow) {
    let threads = match thread::available_parallelism() {
        Ok(t) => t.get(),
        Err(_) => 1,
    };
    app.global::<GuiState>().set_maximum_threads(threads as f32);
}
