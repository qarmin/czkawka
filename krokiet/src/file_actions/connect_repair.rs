use std::path::{MAIN_SEPARATOR, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

use crossbeam_channel::Sender;
use czkawka_core::common::progress_data::ProgressData;
use czkawka_core::tools::file_repair::{FixParameters, RepairPreset, cancel_scope};
use rfd::FileDialog;
use slint::{ComponentHandle, Weak};

use crate::connect_rfd::{hide_file_dialog_overlay, show_file_dialog_overlay};
use crate::model_operations::model_processor::{MessageType, ModelProcessor, ProcessFunction};
use crate::simpler_model::{SimplerSingleMainListModel, ToSimplerVec};
use crate::{Callabler, GuiState, MainWindow};

const FIX_TIME_BUDGET: Duration = Duration::from_secs(120);

pub(crate) fn connect_repair(app: &MainWindow, progress_sender: Sender<ProgressData>, stop_flag: Arc<AtomicBool>) {
    connect_repair_items(app, progress_sender, stop_flag);
    connect_path_choosers(app);
}

fn connect_repair_items(app: &MainWindow, progress_sender: Sender<ProgressData>, stop_flag: Arc<AtomicBool>) {
    let a = app.as_weak();
    app.global::<Callabler>().on_repair_items(move |level_index, replace_original| {
        stop_flag.store(false, Ordering::Relaxed);
        let app = a.upgrade().expect("Failed to upgrade app :(");
        let gui_state = app.global::<GuiState>();
        let output_dir = non_empty_path(gui_state.get_repair_output_dir().as_str());
        let reference = non_empty_path(gui_state.get_repair_reference_file().as_str());
        let preset = RepairPreset::ALL
            .get(usize::try_from(level_index).unwrap_or_default())
            .copied()
            .unwrap_or(RepairPreset::Safe);
        let params = FixParameters {
            preset,
            references: reference.into_iter().collect(),
            output_dir,
            replace_original,
            time_budget: FIX_TIME_BUDGET,
        };

        ModelProcessor::new(gui_state.get_active_tab()).repair_selected_files(progress_sender.clone(), a.clone(), stop_flag.clone(), params);
    });
}

fn non_empty_path(text: &str) -> Option<PathBuf> {
    let trimmed = text.trim();
    (!trimmed.is_empty()).then(|| PathBuf::from(trimmed))
}

fn connect_path_choosers(app: &MainWindow) {
    let a = app.as_weak();
    app.global::<Callabler>().on_choose_repair_output_dir(move || {
        choose_path(&a, |dialog| dialog.pick_folder(), |app, path| app.global::<GuiState>().set_repair_output_dir(path.into()));
    });
    let a = app.as_weak();
    app.global::<Callabler>().on_choose_repair_reference_file(move || {
        choose_path(&a, |dialog| dialog.pick_file(), |app, path| app.global::<GuiState>().set_repair_reference_file(path.into()));
    });
}

fn choose_path(weak: &Weak<MainWindow>, pick: fn(FileDialog) -> Option<PathBuf>, apply: fn(&MainWindow, String)) {
    let app = weak.upgrade().expect("Failed to upgrade app :(");
    show_file_dialog_overlay(&app);
    let weak = weak.clone();
    thread::spawn(move || {
        let chosen = pick(FileDialog::new());
        hide_file_dialog_overlay(&weak);
        if let Some(path) = chosen {
            weak.upgrade_in_event_loop(move |app| apply(&app, path.to_string_lossy().to_string()))
                .expect("Failed to set the chosen path");
        }
    });
}

impl ModelProcessor {
    fn repair_selected_files(self, progress_sender: Sender<ProgressData>, weak_app: Weak<MainWindow>, stop_flag: Arc<AtomicBool>, params: FixParameters) {
        let model = self.active_tab.get_tool_model(&weak_app.upgrade().expect("Failed to upgrade app :("));
        let simpler_model = model.to_simpler_enumerated_vec();
        thread::spawn(move || {
            let path_idx = self.active_tab.get_str_path_idx();
            let name_idx = self.active_tab.get_str_name_idx();
            let stop_for_repair = stop_flag.clone();
            let _cancel = cancel_scope(&stop_flag);

            let repair_fnc = move |data: &SimplerSingleMainListModel| {
                let path = PathBuf::from(format!("{}{MAIN_SEPARATOR}{}", data.val_str[path_idx], data.val_str[name_idx]));
                repair_single_file(&path, &params, &stop_for_repair)
            };

            self.process_and_update_gui_state(
                &weak_app,
                stop_flag,
                &progress_sender,
                simpler_model,
                &ProcessFunction::Simple(Box::new(repair_fnc)),
                MessageType::Repair,
                true,
            );
        });
    }
}

#[cfg(not(test))]
fn repair_single_file(path: &std::path::Path, params: &FixParameters, stop_flag: &Arc<AtomicBool>) -> Result<(), String> {
    use czkawka_core::tools::file_repair::{FixOutcome, fix_file};

    match fix_file(path, params, stop_flag) {
        FixOutcome::Fixed { .. } => Ok(()),
        FixOutcome::Failed(reason) => Err(format!("Failed to repair {}, reason: {reason}", path.display())),
        FixOutcome::Cancelled => Err(format!("Repair of {} was cancelled", path.display())),
    }
}

#[cfg(test)]
fn repair_single_file(path: &std::path::Path, _params: &FixParameters, _stop_flag: &Arc<AtomicBool>) -> Result<(), String> {
    if path.to_string_lossy().contains("test_error") {
        return Err(format!("Test error for item: {}", path.display()));
    }
    Ok(())
}
