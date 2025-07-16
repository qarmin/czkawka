use crossbeam_channel::Sender;
use czkawka_core::common_dir_traversal::{CheckingMethod, ToolType};
use czkawka_core::common_messages::Messages;
use czkawka_core::progress_data::{CurrentStage, ProgressData};
use log::error;
use rayon::prelude::*;
use slint::{ComponentHandle, Model, ModelRc, VecModel, Weak};
use std::path::MAIN_SEPARATOR;
use std::thread;
use std::time::Duration;

use crate::common::{get_str_name_idx, get_str_path_idx, get_tool_model, set_tool_model};
use crate::connect_row_selection::reset_selection;
use crate::model_operations::{ModelProcessor, get_shared_str_item};
use crate::simpler_model::{SimplerMainListModel, ToSimplerVec, ToSlintModel};
use crate::{Callabler, CurrentTab, GuiState, MainListModel, MainWindow, Settings, flk};

pub fn connect_delete_button(app: &MainWindow, progress_sender: Sender<ProgressData>) {
    let a = app.as_weak();
    app.global::<Callabler>().on_delete_selected_items(move || {
        let weak_app = a.clone();
        let progress_sender = progress_sender.clone();
        let app = a.upgrade().expect("Failed to upgrade app :(");

        let active_tab = app.global::<GuiState>().get_active_tab();

        let settings = app.global::<Settings>();

        let processor = ModelProcessor::new(active_tab);
        processor.delete_selected_items(settings.get_move_to_trash(), progress_sender, weak_app);
    });
}

// This is quite ugly workaround for Slint strange limitation, where model cannot be passed to another thread
// This was needed by me, because I wanted to process deletion without blocking main gui thread, with additional sending progress about entire operation.
// After trying different solutions, looks that the simplest and quite not really efficient solution is to convert slint model, to simpler model, which can be passed to another thread.
// Models are converted multiple times, so this have some big overhead
// ModelRc<MainListModel> --cloning when iterating + converting--> SimplerMainListModel --conversion before setting to model--> ModelRc<MainListModel> --cloning when iterating to remove useless items--> ModelRc<MainListModel>

impl ModelProcessor {
    fn delete_selected_items(self, remove_to_trash: bool, progress_sender: Sender<ProgressData>, weak_app: Weak<MainWindow>) {
        let is_empty_folder_tab = self.active_tab == CurrentTab::EmptyFolders;
        let model = get_tool_model(&weak_app.upgrade().expect("Failed to upgrade app :("), self.active_tab);
        let simpler_model = model.to_simpler_enumerated_vec();
        thread::spawn(move || {
            let mut base_progress = ProgressData {
                sstage: CurrentStage::DeletingFiles,
                checking_method: CheckingMethod::None,
                current_stage_idx: 0,
                max_stage_idx: 0,
                entries_checked: 0,
                entries_to_check: 0,
                bytes_checked: 0,
                bytes_to_check: 0,
                tool_type: ToolType::None,
            };

            let items_queued_to_delete = simpler_model.iter().filter(|(_idx, e)| e.checked).count();
            if items_queued_to_delete == 0 {
                weak_app
                    .upgrade_in_event_loop(move |app| {
                        app.global::<GuiState>().set_info_text("".into()); // TODO NOT DELETE ANYTHING message should be added here
                    })
                    .expect("Failed to update app info text");
                return;
            }

            weak_app
                .upgrade_in_event_loop(move |app| {
                    app.set_deleting(true);
                })
                .expect("Failed to update app info text");

            // Sending progress data about how many items are queued to delete
            base_progress.entries_to_check = items_queued_to_delete;
            let _ = progress_sender.send(base_progress).map_err(|e| error!("Failed to send progress data: {e}"));

            thread::sleep(Duration::from_secs(2));

            let results = self.delete_items(simpler_model, is_empty_folder_tab, remove_to_trash);
            let (new_simple_model, errors, items_deleted) = self.remove_deleted_items_from_model(results);

            // Sending progress data at the end of deletion, to indicate that deletion is finished
            base_progress.entries_checked = items_deleted + errors.len();
            let _ = progress_sender.send(base_progress).map_err(|e| error!("Failed to send progress data: {e}"));

            weak_app
                .upgrade_in_event_loop(move |app| {
                    // app.set_text_summary_text(flk!("rust_delete_summary", deleted = items_deleted, failed = errors.len()).into());

                    let new_model_after_removing_useless_items = self.remove_single_items_in_groups(new_simple_model.to_vec_model());
                    set_tool_model(&app, self.active_tab, ModelRc::new(VecModel::from(new_model_after_removing_useless_items)));

                    app.global::<GuiState>().set_info_text(Messages::new_from_errors(errors).create_messages_text().into());

                    app.global::<GuiState>().set_preview_visible(false);

                    reset_selection(&app, true);

                    app.set_deleting(false);
                })
                .expect("Failed to update app after deletion");
        });
    }

    fn delete_items(
        &self,
        items_simplified: Vec<(usize, SimplerMainListModel)>,
        is_empty_folder_tab: bool,
        remove_to_trash: bool,
    ) -> Vec<(usize, SimplerMainListModel, Option<Result<(), String>>)> {
        let path_idx = get_str_path_idx(self.active_tab);
        let name_idx = get_str_name_idx(self.active_tab);
        let mut output: Vec<_> = items_simplified
            .into_par_iter()
            .map(|(idx, data)| {
                // TODO missing logic of sending progress data about how many items are queued to delete
                if !data.checked {
                    return (idx, data, None);
                }

                let res = remove_single_item(
                    &format!("{}{MAIN_SEPARATOR}{}", data.val_str[path_idx], data.val_str[name_idx]),
                    is_empty_folder_tab,
                    remove_to_trash,
                );
                (idx, data, Some(res))
            })
            .collect();
        output.sort_by_key(|(idx, _, _)| *idx);

        output
    }

    fn remove_deleted_items_from_model(&self, results: Vec<(usize, SimplerMainListModel, Option<Result<(), String>>)>) -> (Vec<SimplerMainListModel>, Vec<String>, usize) {
        let mut errors = vec![];
        let mut items_deleted = 0;

        let new_model: Vec<SimplerMainListModel> = results
            .into_iter()
            .filter_map(|(_idx, item, delete_res)| match delete_res {
                Some(Ok(())) => {
                    items_deleted += 1;
                    None
                }
                Some(Err(err)) => {
                    errors.push(err);
                    Some(item)
                }
                None => Some(item),
            })
            .collect();

        (new_model, errors, items_deleted)
    }
}

// #[cfg(not(test))]
// fn remove_single_item(full_path: &str, is_folder_tab: bool, remove_to_trash: bool) -> Result<(), String> {
//     if is_folder_tab {
//         return czkawka_core::common::remove_folder_if_contains_only_empty_folders(full_path, remove_to_trash);
//     }
//     if remove_to_trash {
//         if let Err(e) = trash::delete(full_path) {
//             return Err(flk!("rust_error_moving_to_trash", error = e.to_string()));
//         }
//     } else {
//         if let Err(e) = std::fs::remove_file(full_path) {
//             return Err(flk!("rust_error_removing_file", error = e.to_string()));
//         }
//     }
//     Ok(())
// }
//
// #[cfg(test)]
fn remove_single_item(full_path: &str, _is_folder_tab: bool, _remove_to_trash: bool) -> Result<(), String> {
    dbg!(full_path);
    if full_path.contains("test_error") {
        return Err(format!("Test error for item: {full_path}"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crossbeam_channel::{Receiver, unbounded};

    use super::*;
    use crate::test_common::{create_model_from_model_vec, get_model_vec};

    impl ModelProcessor {
        pub fn process_deletion_test(
            &self,
            remove_to_trash: bool,
            progress_sender: Sender<ProgressData>,
            model: &ModelRc<MainListModel>,
        ) -> Option<(Vec<MainListModel>, Vec<String>, usize, usize)> {
            let is_empty_folder_tab = self.active_tab == CurrentTab::EmptyFolders;

            let (items, items_simplified, items_queued_to_delete) = self.prepare_delete_models(model)?;

            let output = self.delete_items(items_simplified, is_empty_folder_tab, remove_to_trash);

            let (new_model, errors, items_deleted) = self.connect_new_model_after_deletion(output, items);

            Some((new_model, errors, items_queued_to_delete, items_deleted))
        }
    }

    #[test]
    fn test_no_delete_items() {
        let (progress, _receiver): (Sender<ProgressData>, Receiver<ProgressData>) = unbounded();
        let model = get_model_vec(10);
        let model = create_model_from_model_vec(&model);
        let processor = ModelProcessor::new(CurrentTab::EmptyFolders);
        assert!(processor.process_deletion_test(false, progress, &model).is_none());
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
        let (new_model, errors, items_queued_to_delete, items_deleted) = processor.process_deletion_test(false, progress, &model).unwrap();

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
