use std::path::{MAIN_SEPARATOR, Path};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;

use crossbeam_channel::Sender;
use czkawka_core::common::progress_data::ProgressData;
use slint::{ComponentHandle, Weak};

use crate::model_operations::model_processor::{MessageType, ModelProcessor, ProcessFunction};
use crate::simpler_model::{SimplerSingleMainListModel, ToSimplerVec};
use crate::{ActiveTab, Callabler, GuiState, MainWindow};

pub(crate) fn connect_deduplicate(app: &MainWindow, progress_sender: Sender<ProgressData>, stop_flag: Arc<AtomicBool>) {
    app.global::<GuiState>().set_cow_deduplication_available(cfg!(target_os = "linux"));
    let a = app.as_weak();
    app.global::<Callabler>().on_deduplicate_items(move || {
        let weak_app = a.clone();
        let progress_sender = progress_sender.clone();
        let stop_flag = stop_flag.clone();
        stop_flag.store(false, Ordering::Relaxed);
        let app = a.upgrade().expect("Failed to upgrade app :(");
        let active_tab = app.global::<GuiState>().get_active_tab();

        if active_tab != ActiveTab::DuplicateFiles || !cfg!(target_os = "linux") || app.get_processing() {
            return;
        }
        app.set_processing(true);
        let processor = ModelProcessor::new(active_tab);
        processor.deduplicate_selected_items(progress_sender, weak_app, stop_flag);
    });
}

impl ModelProcessor {
    fn deduplicate_selected_items(self, progress_sender: Sender<ProgressData>, weak_app: Weak<MainWindow>, stop_flag: Arc<AtomicBool>) {
        let model = self.active_tab.get_tool_model(&weak_app.upgrade().expect("Failed to upgrade app :("));
        let simpler_model = model.to_simpler_enumerated_vec();
        thread::spawn(move || {
            let path_idx = self.active_tab.get_str_path_idx();
            let name_idx = self.active_tab.get_str_name_idx();

            let operation_stop = stop_flag.clone();
            let deduplicate_fnc = move |original: &SimplerSingleMainListModel, derived: &SimplerSingleMainListModel| {
                deduplicate_single_item(
                    &format!("{}{MAIN_SEPARATOR}{}", original.val_str[path_idx], original.val_str[name_idx]),
                    &format!("{}{MAIN_SEPARATOR}{}", derived.val_str[path_idx], derived.val_str[name_idx]),
                    &operation_stop,
                )
            };
            self.process_and_update_gui_state(
                &weak_app,
                stop_flag,
                &progress_sender,
                simpler_model,
                &ProcessFunction::Related(Box::new(deduplicate_fnc)),
                MessageType::Deduplicate,
                false,
            );
        });
    }
}

fn deduplicate_single_item(original_path: &str, derived_path: &str, stop: &AtomicBool) -> Result<(), String> {
    czkawka_core::common::deduplication::deduplicate_files(Path::new(original_path), Path::new(derived_path), stop)
        .map_err(|e| crate::flk!("rust_deduplicate_failed", name = original_path, target = derived_path, reason = e.to_string()))
}
