use czkawka_core::common::get_available_threads;
use slint::ComponentHandle;

use crate::GuiState;
use crate::MainWindow;

// Some info needs to be send to gui at the start like available thread number in OS.
//
pub fn set_initial_gui_infos(app: &MainWindow) {
    let threads = get_available_threads();
    app.global::<GuiState>().set_maximum_threads(threads as f32);
}
