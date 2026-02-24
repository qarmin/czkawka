use std::sync::{Arc, Mutex};

use rfd::FileDialog;
use slint::ComponentHandle;

use crate::connect_rfd::{hide_file_dialog_overlay, show_file_dialog_overlay};
use crate::shared_models::SharedModels;
use crate::{Callabler, GuiState, MainWindow};

pub(crate) fn connect_save(app: &MainWindow, shared_models: Arc<Mutex<SharedModels>>) {
    let a = app.as_weak();
    app.global::<Callabler>().on_save_results(move || {
        let app = a.upgrade().expect("Failed to upgrade app :(");
        let active_tab = app.global::<GuiState>().get_active_tab();

        show_file_dialog_overlay(&app);

        let weak = a.clone();
        let shared_models = Arc::clone(&shared_models);
        std::thread::spawn(move || {
            let folder = FileDialog::new().pick_folder();

            hide_file_dialog_overlay(&weak);

            if let Some(folder) = folder {
                let folder_str = folder.to_string_lossy().to_string();
                weak.upgrade_in_event_loop(move |app| {
                    if let Err(e) = shared_models.lock().unwrap().save_results(active_tab, &folder_str) {
                        app.global::<GuiState>().set_info_text(e.into());
                    }
                })
                .expect("Failed to save results");
            }
        });
    });
}
