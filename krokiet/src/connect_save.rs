use std::sync::{Arc, Mutex};

use rfd::FileDialog;
use slint::ComponentHandle;

use crate::shared_models::SharedModels;
use crate::{Callabler, GuiState, MainWindow};

pub fn connect_save(app: &MainWindow, shared_models: Arc<Mutex<SharedModels>>) {
    let a = app.as_weak();
    app.global::<Callabler>().on_save_results(move || {
        let app = a.upgrade().expect("Failed to upgrade app :(");
        let active_tab = app.global::<GuiState>().get_active_tab();

        let file_dialog = FileDialog::new();
        let Some(folder) = file_dialog.pick_folder() else {
            return;
        };
        let folder_str = folder.to_string_lossy();
        if let Err(e) = shared_models.lock().unwrap().save_results(active_tab, &folder_str) {
            app.global::<GuiState>().set_info_text(e.into());
        }
    });
}
