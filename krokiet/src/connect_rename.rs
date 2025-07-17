use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;

use crossbeam_channel::Sender;
use czkawka_core::progress_data::ProgressData;
use slint::{ComponentHandle, Weak};

use crate::model_operations::model_processor::{MessageType, ModelProcessor};
use crate::simpler_model::{SimplerMainListModel, ToSimplerVec};
use crate::{Callabler, GuiState, MainWindow};

pub fn connect_rename(app: &MainWindow, progress_sender: Sender<ProgressData>, stop_flag: Arc<AtomicBool>) {
    let a = app.as_weak();
    app.global::<Callabler>().on_rename_files(move || {
        let weak_app = a.clone();
        let progress_sender = progress_sender.clone();
        let stop_flag = stop_flag.clone();
        stop_flag.store(false, Ordering::Relaxed);
        let app = a.upgrade().expect("Failed to upgrade app :(");
        let active_tab = app.global::<GuiState>().get_active_tab();

        let processor = ModelProcessor::new(active_tab);
        processor.rename_selected_items(progress_sender, weak_app, stop_flag);
    });
}

impl ModelProcessor {
    fn rename_selected_items(self, progress_sender: Sender<ProgressData>, weak_app: Weak<MainWindow>, stop_flag: Arc<AtomicBool>) {
        let model = self.active_tab.get_tool_model(&weak_app.upgrade().expect("Failed to upgrade app :("));
        let simpler_model = model.to_simpler_enumerated_vec();
        thread::spawn(move || {
            let path_idx = self.active_tab.get_str_path_idx();
            let name_idx = self.active_tab.get_str_name_idx();
            let ext_idx = self.active_tab.get_str_proper_extension();

            let rm_fnc = move |data: &SimplerMainListModel| rename_single_item(data, path_idx, name_idx, ext_idx);

            self.process_and_update_gui_state(&weak_app, stop_flag, &progress_sender, simpler_model, rm_fnc, MessageType::Rename);
        });
    }
}

#[cfg(not(test))]
fn rename_single_item(data: &SimplerMainListModel, path_idx: usize, name_idx: usize, ext_idx: usize) -> Result<(), String> {
    use std::path::MAIN_SEPARATOR;
    let folder = &data.val_str[path_idx];
    let file_name = &data.val_str[name_idx];
    let new_extension = &data.val_str[ext_idx];

    let file_stem = std::path::Path::new(&file_name).file_stem().map(|e| e.to_string_lossy().to_string()).unwrap_or_default();
    let new_full_path = format!("{folder}{MAIN_SEPARATOR}{file_stem}.{new_extension}");
    let old_full_path = format!("{folder}{MAIN_SEPARATOR}{file_name}");

    if let Err(e) = std::fs::rename(&old_full_path, &new_full_path) {
        Err(crate::flk!(
            "rust_failed_to_rename_file",
            old_path = old_full_path,
            new_path = new_full_path,
            error = e.to_string()
        ))
    } else {
        Ok(())
    }
}

#[cfg(test)]
fn rename_single_item(data: &SimplerMainListModel, path_idx: usize, _name_idx: usize, _ext_idx: usize) -> Result<(), String> {
    let full_path = &data.val_str[path_idx];
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
    use crate::simpler_model::ToSlintModel;
    use crate::test_common::{create_model_from_model_vec, get_model_vec};
    use crate::{CurrentTab, MainListModel};

    impl ModelProcessor {
        pub fn process_rename_test(&self, progress_sender: Sender<ProgressData>, model: ModelRc<MainListModel>) -> Option<(Vec<MainListModel>, Vec<String>, usize, usize)> {
            let items_queued_to_delete = model.iter().filter(|e| e.checked).count();
            if items_queued_to_delete == 0 {
                return None; // No items to delete
            }
            let simplified_model = model.to_simpler_enumerated_vec();

            let path_idx = 0;
            let name_idx = 0;
            let ext_idx = 0;

            let rm_fnc = move |data: &SimplerMainListModel| rename_single_item(data, path_idx, name_idx, ext_idx);

            let output = self.process_items(
                simplified_model,
                items_queued_to_delete,
                progress_sender,
                &Arc::new(AtomicBool::new(false)),
                rm_fnc,
                MessageType::Rename,
                self.active_tab.get_int_size_opt_idx(),
            );

            let (new_simple_model, errors, items_deleted) = self.remove_deleted_items_from_model(output);

            Some((new_simple_model.to_vec_model(), errors, items_queued_to_delete, items_deleted))
        }
    }

    #[test]
    fn test_no_rename_items() {
        let (progress, _receiver): (Sender<ProgressData>, Receiver<ProgressData>) = unbounded();
        let model = get_model_vec(10);
        let model = create_model_from_model_vec(&model);
        let processor = ModelProcessor::new(CurrentTab::EmptyFolders);
        assert!(processor.process_rename_test(progress, model).is_none());
    }

    #[test]
    fn test_rename_selected_items() {
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
        let (new_model, errors, items_queued_to_delete, items_deleted) = processor.process_rename_test(progress, model).unwrap();

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
