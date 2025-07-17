use std::path::MAIN_SEPARATOR;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;

use crossbeam_channel::Sender;
use czkawka_core::common_messages::Messages;
use czkawka_core::progress_data::ProgressData;
use log::error;
use slint::{ComponentHandle, ModelRc, VecModel, Weak};

use crate::common::{get_str_name_idx, get_str_path_idx, get_tool_model, set_tool_model};
use crate::connect_row_selection::reset_selection;
use crate::model_operations::model_processor::ModelProcessor;
use crate::simpler_model::{SimplerMainListModel, ToSimplerVec, ToSlintModel};
use crate::{Callabler, CurrentTab, GuiState, MainWindow, Settings, flk};

pub fn connect_delete_button(app: &MainWindow, progress_sender: Sender<ProgressData>, stop_flag: Arc<AtomicBool>) {
    let a = app.as_weak();
    app.global::<Callabler>().on_delete_selected_items(move || {
        let weak_app = a.clone();
        let progress_sender = progress_sender.clone();
        let stop_flag = stop_flag.clone();
        stop_flag.store(false, Ordering::Relaxed);
        let app = a.upgrade().expect("Failed to upgrade app :(");

        let active_tab = app.global::<GuiState>().get_active_tab();

        let settings = app.global::<Settings>();

        let processor = ModelProcessor::new(active_tab);
        processor.delete_selected_items(settings.get_move_to_trash(), progress_sender, weak_app, stop_flag);
    });
}

impl ModelProcessor {
    fn delete_selected_items(self, remove_to_trash: bool, progress_sender: Sender<ProgressData>, weak_app: Weak<MainWindow>, stop_flag: Arc<AtomicBool>) {
        let is_empty_folder_tab = self.active_tab == CurrentTab::EmptyFolders;
        let model = get_tool_model(&weak_app.upgrade().expect("Failed to upgrade app :("), self.active_tab);
        let simpler_model = model.to_simpler_enumerated_vec();
        thread::spawn(move || {
            let mut base_progress = ProgressData::get_base_deleting_state();

            weak_app
                .upgrade_in_event_loop(move |app| {
                    app.set_processing(true);
                })
                .expect("Failed to update app info text");

            let items_queued_to_delete = simpler_model.iter().filter(|(_idx, e)| e.checked).count();
            if items_queued_to_delete == 0 {
                weak_app
                    .upgrade_in_event_loop(move |app| {
                        app.global::<GuiState>().set_info_text(flk!("rust_no_files_deleted").into());
                        stop_flag.store(false, Ordering::Relaxed);
                        app.set_stop_requested(false);
                        app.set_processing(false);
                    })
                    .expect("Failed to update app info text");
                return;
            }

            // Sending progress data about how many items are queued to delete
            base_progress.entries_to_check = items_queued_to_delete;
            let _ = progress_sender.send(base_progress).map_err(|e| error!("Failed to send progress data: {e}"));

            let path_idx = get_str_path_idx(self.active_tab);
            let name_idx = get_str_name_idx(self.active_tab);

            let dlt_fnc = move |data: &SimplerMainListModel| {
                remove_single_item(
                    &format!("{}{MAIN_SEPARATOR}{}", data.val_str[path_idx], data.val_str[name_idx]),
                    is_empty_folder_tab,
                    remove_to_trash,
                )
            };

            let results = self.process_items(simpler_model, items_queued_to_delete, progress_sender.clone(), &stop_flag, dlt_fnc);
            let (new_simple_model, errors, items_deleted) = self.remove_deleted_items_from_model(results);

            // Sending progress data at the end of deletion, to indicate that deletion is finished
            base_progress.entries_checked = items_deleted + errors.len();

            let _ = progress_sender.send(base_progress).map_err(|e| error!("Failed to send progress data: {e}"));

            weak_app
                .upgrade_in_event_loop(move |app| {
                    let mut new_model_after_removing_useless_items = self.remove_single_items_in_groups(new_simple_model.to_vec_model());
                    // Selection cache was invalidated, so we need to reset it
                    new_model_after_removing_useless_items.iter_mut().for_each(|e| e.selected_row = false);
                    set_tool_model(&app, self.active_tab, ModelRc::new(VecModel::from(new_model_after_removing_useless_items)));

                    app.global::<GuiState>()
                        .set_info_text(Messages::new_from_errors(errors.clone()).create_messages_text().into());

                    app.global::<GuiState>().set_preview_visible(false);

                    reset_selection(&app, true);
                    stop_flag.store(false, Ordering::Relaxed);
                    app.invoke_processing_ended(flk!("rust_delete_summary", deleted = items_deleted, failed = errors.len(), total = items_queued_to_delete).into());
                })
                .expect("Failed to update app after deletion");
        });
    }
}

#[cfg(not(test))]
fn remove_single_item(full_path: &str, is_folder_tab: bool, remove_to_trash: bool) -> Result<(), String> {
    if is_folder_tab {
        return czkawka_core::common::remove_folder_if_contains_only_empty_folders(full_path, remove_to_trash);
    }
    if remove_to_trash {
        if let Err(e) = trash::delete(full_path) {
            return Err(flk!("rust_error_moving_to_trash", error = e.to_string()));
        }
    } else {
        if let Err(e) = std::fs::remove_file(full_path) {
            return Err(flk!("rust_error_removing_file", error = e.to_string()));
        }
    }
    Ok(())
}

#[cfg(test)]
fn remove_single_item(full_path: &str, _is_folder_tab: bool, _remove_to_trash: bool) -> Result<(), String> {
    if full_path.contains("test_error") {
        return Err(format!("Test error for item: {full_path}"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crossbeam_channel::{Receiver, unbounded};
    use slint::Model;

    use super::*;
    use crate::MainListModel;
    use crate::test_common::{create_model_from_model_vec, get_model_vec};

    impl ModelProcessor {
        pub fn process_deletion_test(
            &self,
            remove_to_trash: bool,
            progress_sender: Sender<ProgressData>,
            model: ModelRc<MainListModel>,
        ) -> Option<(Vec<MainListModel>, Vec<String>, usize, usize)> {
            let is_empty_folder_tab = self.active_tab == CurrentTab::EmptyFolders;

            let items_queued_to_delete = model.iter().filter(|e| e.checked).count();
            if items_queued_to_delete == 0 {
                return None; // No items to delete
            }
            let simplified_model = model.to_simpler_enumerated_vec();

            let path_idx = 0;
            let name_idx = 0;
            let dlt_fnc = move |data: &SimplerMainListModel| {
                remove_single_item(
                    &format!("{}{MAIN_SEPARATOR}{}", data.val_str[path_idx], data.val_str[name_idx]),
                    is_empty_folder_tab,
                    remove_to_trash,
                )
            };

            let output = self.process_items(simplified_model, items_queued_to_delete, progress_sender, &Arc::new(AtomicBool::new(false)), dlt_fnc);

            let (new_simple_model, errors, items_deleted) = self.remove_deleted_items_from_model(output);

            Some((new_simple_model.to_vec_model(), errors, items_queued_to_delete, items_deleted))
        }
    }

    #[test]
    fn test_no_delete_items() {
        let (progress, _receiver): (Sender<ProgressData>, Receiver<ProgressData>) = unbounded();
        let model = get_model_vec(10);
        let model = create_model_from_model_vec(&model);
        let processor = ModelProcessor::new(CurrentTab::EmptyFolders);
        assert!(processor.process_deletion_test(false, progress, model).is_none());
    }

    #[test]
    fn test_delete_selected_items() {
        let (progress, _receiver): (Sender<ProgressData>, Receiver<ProgressData>) = unbounded();
        let mut model = get_model_vec(10);
        model[0].checked = true;
        model[0].val_str = ModelRc::new(VecModel::from(vec!["normal1".to_string().into(); 10]));
        model[1].checked = true;
        model[1].val_str = ModelRc::new(VecModel::from(vec!["normal2".to_string().into(); 10]));
        model[3].checked = true;
        model[3].val_str = ModelRc::new(VecModel::from(vec!["test_error".to_string().into(); 10]));
        let model = create_model_from_model_vec(&model);
        let processor = ModelProcessor::new(CurrentTab::EmptyFolders);
        let (new_model, errors, items_queued_to_delete, items_deleted) = processor.process_deletion_test(false, progress, model).unwrap();

        assert_eq!(new_model.len(), 8);
        assert_eq!(errors.len(), 1);
        assert_eq!(items_queued_to_delete, 3);
        assert_eq!(items_deleted, 2);

        assert!(new_model[1].checked);
        assert!(new_model[1].val_str.iter().all(|s| s == "test_error"));
        assert!(!new_model[0].checked);
        assert!(new_model.iter().skip(2).all(|model| !model.checked));
    }
}
