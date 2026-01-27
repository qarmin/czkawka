use std::path::MAIN_SEPARATOR;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;

use crossbeam_channel::Sender;
use czkawka_core::common::progress_data::ProgressData;
use slint::{ComponentHandle, Weak};

use crate::model_operations::model_processor::{MessageType, ModelProcessor, ProcessFunction};
use crate::simpler_model::{SimplerSingleMainListModel, ToSimplerVec};
use crate::{Callabler, GuiState, MainWindow};

pub(crate) fn connect_hardlink(app: &MainWindow, progress_sender: Sender<ProgressData>, stop_flag: Arc<AtomicBool>) {
    let a = app.as_weak();
    app.global::<Callabler>().on_hardlink_items(move || {
        let weak_app = a.clone();
        let progress_sender = progress_sender.clone();
        let stop_flag = stop_flag.clone();
        stop_flag.store(false, Ordering::Relaxed);
        let app = a.upgrade().expect("Failed to upgrade app :(");
        let active_tab = app.global::<GuiState>().get_active_tab();

        let processor = ModelProcessor::new(active_tab);
        processor.hardlink_selected_items(progress_sender, weak_app, stop_flag);
    });
}

impl ModelProcessor {
    fn hardlink_selected_items(self, progress_sender: Sender<ProgressData>, weak_app: Weak<MainWindow>, stop_flag: Arc<AtomicBool>) {
        let model = self.active_tab.get_tool_model(&weak_app.upgrade().expect("Failed to upgrade app :("));
        let simpler_model = model.to_simpler_enumerated_vec();
        thread::spawn(move || {
            let path_idx = self.active_tab.get_str_path_idx();
            let name_idx = self.active_tab.get_str_name_idx();

            let hardlink_fnc = move |original: &SimplerSingleMainListModel, derived: &SimplerSingleMainListModel| {
                hardlink_single_item(
                    &format!("{}{MAIN_SEPARATOR}{}", original.val_str[path_idx], original.val_str[name_idx]),
                    &format!("{}{MAIN_SEPARATOR}{}", derived.val_str[path_idx], derived.val_str[name_idx]),
                )
            };
            self.process_and_update_gui_state(
                &weak_app,
                stop_flag,
                &progress_sender,
                simpler_model,
                &ProcessFunction::Related(Box::new(hardlink_fnc)),
                MessageType::Hardlink,
                false,
            );
        });
    }
}

#[cfg(not(test))]
fn hardlink_single_item(original_path: &str, derived_path: &str) -> Result<(), String> {
    czkawka_core::common::make_hard_link(original_path, derived_path)
        .map_err(|e| crate::flk!("rust_hardlink_failed", name = original_path, target = derived_path, reason = e.to_string()))
}

#[cfg(test)]
fn hardlink_single_item(original_path: &str, _derived_path: &str) -> Result<(), String> {
    if original_path.contains("test_error") {
        return Err(format!("Test error for item: {original_path}"));
    }
    Ok(())
}
