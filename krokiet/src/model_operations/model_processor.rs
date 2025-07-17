use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::time::Duration;

use crossbeam_channel::Sender;
use czkawka_core::common_messages::Messages;
use czkawka_core::progress_data::{CurrentStage, ProgressData};
use log::error;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use slint::{ComponentHandle, ModelRc, VecModel, Weak};

use crate::common::delayed_sender::DelayedSender;
use crate::common::{get_is_header_mode, set_tool_model};
use crate::connect_row_selection::reset_selection;
use crate::model_operations::ProcessingResult;
use crate::simpler_model::{SimplerMainListModel, ToSlintModel};
use crate::{CurrentTab, GuiState, MainListModel, MainWindow, flk, model_operations};

// This is quite ugly workaround for Slint strange limitation, where model cannot be passed to another thread
// This was needed by me, because I wanted to process deletion without blocking main gui thread, with additional sending progress about entire operation.
// After trying different solutions, looks that the simplest and quite not really efficient solution is to convert slint model, to simpler model, which can be passed to another thread.
// Models are converted multiple times, so this have some big overhead
// ModelRc<MainListModel> --cloning when iterating + converting--> SimplerMainListModel --conversion before setting to model--> ModelRc<MainListModel> --cloning when iterating to remove useless items--> ModelRc<MainListModel>

pub struct ModelProcessor {
    pub active_tab: CurrentTab,
}

#[derive(Clone, Copy)]
pub enum MessageType {
    Delete,
    Rename,
    Move,
}

impl MessageType {
    fn get_empty_message(&self) -> String {
        match self {
            Self::Delete => flk!("rust_no_files_deleted"),
            Self::Rename => flk!("rust_no_files_renamed"),
            Self::Move => flk!("rust_no_files_moved"),
        }
    }
    fn get_summary_message(&self, deleted: usize, failed: usize, total: usize) -> String {
        match self {
            Self::Delete => flk!("rust_delete_summary", deleted = deleted, failed = failed, total = total),
            Self::Rename => flk!("rust_rename_summary", renamed = deleted, failed = failed, total = total),
            Self::Move => flk!("rust_move_summary", moved = deleted, failed = failed, total = total),
        }
    }
    fn get_base_progress(&self) -> ProgressData {
        match self {
            Self::Delete => ProgressData::get_empty_state(CurrentStage::DeletingFiles),
            Self::Rename => ProgressData::get_empty_state(CurrentStage::RenamingFiles),
            Self::Move => ProgressData::get_empty_state(CurrentStage::MovingFiles),
        }
    }
}

impl ModelProcessor {
    pub fn new(active_tab: CurrentTab) -> Self {
        Self { active_tab }
    }

    pub fn remove_single_items_in_groups(&self, items: Vec<MainListModel>) -> Vec<MainListModel> {
        let have_header = get_is_header_mode(self.active_tab);
        model_operations::remove_single_items_in_groups(items, have_header)
    }

    pub fn remove_deleted_items_from_model(&self, results: ProcessingResult) -> (Vec<SimplerMainListModel>, Vec<String>, usize) {
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
    pub fn process_items(
        &self,
        items_simplified: Vec<(usize, SimplerMainListModel)>,
        items_queued_to_delete: usize,
        sender: Sender<ProgressData>,
        stop_flag: &Arc<AtomicBool>,
        process_function: impl Fn(&SimplerMainListModel) -> Result<(), String> + Send + Sync + 'static,
        message_type: MessageType,
    ) -> ProcessingResult {
        let rm_idx = Arc::new(AtomicUsize::new(0));
        let delayed_sender = DelayedSender::new(sender, Duration::from_millis(200));

        let mut output: Vec<_> = items_simplified
            .into_par_iter()
            .map(|(idx, data)| {
                if !data.checked {
                    return (idx, data, None);
                }

                // Stop requested, so just return items
                if stop_flag.load(Ordering::Relaxed) {
                    return (idx, data, None);
                }

                let rm_idx = rm_idx.fetch_add(1, Ordering::Relaxed);
                let mut progress = message_type.get_base_progress();
                progress.entries_to_check = items_queued_to_delete;
                progress.entries_checked = rm_idx;
                delayed_sender.send(progress);

                let res = process_function(&data);
                (idx, data, Some(res))
            })
            .collect();
        output.sort_by_key(|(idx, _, _)| *idx);

        output
    }

    #[allow(clippy::too_many_arguments)]
    pub fn process_and_update_gui_state(
        self,
        weak_app: &Weak<MainWindow>,
        stop_flag: Arc<AtomicBool>,
        progress_sender: &Sender<ProgressData>,
        simpler_model: Vec<(usize, SimplerMainListModel)>,
        dlt_fnc: impl Fn(&SimplerMainListModel) -> Result<(), String> + Send + Sync + 'static,
        message_type: MessageType,
    ) {
        weak_app
            .upgrade_in_event_loop(move |app| {
                app.set_processing(true); // TODO processing should be probably set in gui
            })
            .expect("Failed to update app info text");

        let items_queued_to_delete = simpler_model.iter().filter(|(_idx, e)| e.checked).count();
        if items_queued_to_delete == 0 {
            weak_app
                .upgrade_in_event_loop(move |app| {
                    app.global::<GuiState>().set_info_text(message_type.get_empty_message().into());
                    stop_flag.store(false, Ordering::Relaxed);
                    app.set_stop_requested(false);
                    app.set_processing(false);
                })
                .expect("Failed to update app info text");
            return;
        }

        // Sending progress data about how many items are queued to delete
        let mut base_progress = message_type.get_base_progress();
        base_progress.entries_to_check = items_queued_to_delete;
        let _ = progress_sender.send(base_progress).map_err(|e| error!("Failed to send progress data: {e}"));

        let results = self.process_items(simpler_model, items_queued_to_delete, progress_sender.clone(), &stop_flag, dlt_fnc, message_type);
        let (new_simple_model, errors, items_deleted) = self.remove_deleted_items_from_model(results);
        let errors_len = errors.len();

        // Sending progress data at the end of deletion, to indicate that deletion is finished
        base_progress.entries_checked = items_deleted + errors_len;

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
                app.invoke_processing_ended(message_type.get_summary_message(items_deleted, errors_len, items_queued_to_delete).into());
            })
            .expect("Failed to update app after deletion");
    }
}
