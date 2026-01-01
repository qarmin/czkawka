use std::path::MAIN_SEPARATOR;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;

use crossbeam_channel::Sender;
use czkawka_core::common::progress_data::ProgressData;
use slint::{ComponentHandle, Weak};

use crate::model_operations::model_processor::{MessageType, ModelProcessor};
use crate::simpler_model::{SimplerMainListModel, ToSimplerVec};
use crate::{Callabler, GuiState, MainWindow};

pub(crate) fn connect_optimize_video(app: &MainWindow, progress_sender: Sender<ProgressData>, stop_flag: Arc<AtomicBool>) {
    let a = app.as_weak();
    app.global::<Callabler>().on_optimize_items(move || {
        let weak_app = a.clone();
        let progress_sender = progress_sender.clone();
        let stop_flag = stop_flag.clone();
        stop_flag.store(false, Ordering::Relaxed);
        let app = a.upgrade().expect("Failed to upgrade app :(");
        let active_tab = app.global::<GuiState>().get_active_tab();

        let processor = ModelProcessor::new(active_tab);
        processor.optimize_selected_videos(progress_sender, weak_app, stop_flag);
    });
}

impl ModelProcessor {
    fn optimize_selected_videos(self, progress_sender: Sender<ProgressData>, weak_app: Weak<MainWindow>, stop_flag: Arc<AtomicBool>) {
        let model = self.active_tab.get_tool_model(&weak_app.upgrade().expect("Failed to upgrade app :("));
        let simpler_model = model.to_simpler_enumerated_vec();
        thread::spawn(move || {
            let path_idx = self.active_tab.get_str_path_idx();
            let name_idx = self.active_tab.get_str_name_idx();

            let optimize_fnc = move |data: &SimplerMainListModel| optimize_single_video(&format!("{}{MAIN_SEPARATOR}{}", data.val_str[path_idx], data.val_str[name_idx]));

            self.process_and_update_gui_state(&weak_app, stop_flag, &progress_sender, simpler_model, optimize_fnc, MessageType::OptimizeVideo);
        });
    }
}

#[cfg(not(test))]
#[expect(clippy::unnecessary_wraps)]
fn optimize_single_video(_full_path: &str) -> Result<(), String> {
    // TODO - should be an option to remove original file or not
    // Not sure how exactly this should work - needs to rethink

    Ok(())
}

#[cfg(test)]
fn optimize_single_video(full_path: &str) -> Result<(), String> {
    if full_path.contains("test_error") {
        return Err(format!("Test error for item: {full_path}"));
    }
    Ok(())
}
