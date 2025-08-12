use std::path::MAIN_SEPARATOR;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;

use crossbeam_channel::Sender;
use czkawka_core::common::progress_data::ProgressData;
use slint::{ComponentHandle, Weak};

use crate::model_operations::model_processor::{MessageType, ModelProcessor};
use crate::simpler_model::{SimplerMainListModel, ToSimplerVec};
use crate::{ActiveTab, Callabler, GuiState, MainWindow, Settings};

pub(crate) fn connect_delete_button(app: &MainWindow, progress_sender: Sender<ProgressData>, stop_flag: Arc<AtomicBool>) {
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

    // let a = app.as_weak();
    // app.on_delete_popup_dialog_show_requested(move|| {
    //    let app = a.upgrade().expect("Failed to upgrade app :(");
    //     let settings = app.global::<Settings>();
    //     let active_tab = app.global::<GuiState>().get_active_tab();
    //     let model = active_tab.get_tool_model(&app);
    // });
}

impl ModelProcessor {
    fn delete_selected_items(self, remove_to_trash: bool, progress_sender: Sender<ProgressData>, weak_app: Weak<MainWindow>, stop_flag: Arc<AtomicBool>) {
        let is_empty_folder_tab = self.active_tab == ActiveTab::EmptyFolders;
        let model = self.active_tab.get_tool_model(&weak_app.upgrade().expect("Failed to upgrade app :("));
        let simpler_model = model.to_simpler_enumerated_vec();
        thread::spawn(move || {
            let path_idx = self.active_tab.get_str_path_idx();
            let name_idx = self.active_tab.get_str_name_idx();

            let dlt_fnc = move |data: &SimplerMainListModel| {
                remove_single_item(
                    &format!("{}{MAIN_SEPARATOR}{}", data.val_str[path_idx], data.val_str[name_idx]),
                    is_empty_folder_tab,
                    remove_to_trash,
                )
            };

            self.process_and_update_gui_state(&weak_app, stop_flag, &progress_sender, simpler_model, dlt_fnc, MessageType::Delete);
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
            return Err(crate::flk!("rust_error_moving_to_trash", error = e.to_string()));
        }
    } else {
        if let Err(e) = std::fs::remove_file(full_path) {
            return Err(crate::flk!("rust_error_removing_file", error = e.to_string()));
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
    use slint::{Model, ModelRc, VecModel};

    use super::*;
    use crate::MainListModel;
    use crate::simpler_model::ToSlintModel;
    use crate::test_common::{create_model_from_model_vec, get_model_vec};

    impl ModelProcessor {
        pub(crate) fn process_deletion_test(
            &self,
            remove_to_trash: bool,
            progress_sender: Sender<ProgressData>,
            model: ModelRc<MainListModel>,
        ) -> Option<(Vec<MainListModel>, Vec<String>, usize, usize)> {
            let is_empty_folder_tab = self.active_tab == ActiveTab::EmptyFolders;

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

            let output = self.process_items(
                simplified_model,
                items_queued_to_delete,
                progress_sender,
                &Arc::default(),
                dlt_fnc,
                MessageType::Delete,
                self.active_tab.get_int_size_opt_idx(),
            );

            let (new_simple_model, errors, items_deleted) = self.remove_deleted_items_from_model(output);

            Some((new_simple_model.to_vec_model(), errors, items_queued_to_delete, items_deleted))
        }
    }

    #[test]
    fn test_no_delete_items() {
        let (progress, _receiver): (Sender<ProgressData>, Receiver<ProgressData>) = unbounded();
        let model = get_model_vec(10);
        let model = create_model_from_model_vec(&model);
        let processor = ModelProcessor::new(ActiveTab::EmptyFolders);
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
        let processor = ModelProcessor::new(ActiveTab::EmptyFolders);
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
