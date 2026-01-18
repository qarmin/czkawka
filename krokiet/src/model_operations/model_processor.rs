use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};
use std::time::Duration;

use crossbeam_channel::Sender;
use czkawka_core::common::progress_data::{CurrentStage, ProgressData};
use czkawka_core::helpers::delayed_sender::DelayedSender;
use czkawka_core::helpers::messages::{MessageLimit, Messages};
use log::{debug, error};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use slint::{ComponentHandle, ModelRc, VecModel, Weak};

use crate::connect_row_selection::checker::set_number_of_enabled_items;
use crate::connect_row_selection::reset_selection;
use crate::model_operations::ProcessingResult;
use crate::simpler_model::{SimplerMainListModel, ToSlintModel};
use crate::{ActiveTab, GuiState, MainListModel, MainWindow, flk, model_operations};
// This is quite ugly workaround for Slint strange limitation, where model cannot be passed to another thread
// This was needed by me, because I wanted to process deletion without blocking main gui thread, with additional sending progress about entire operation.
// After trying different solutions, looks that the simplest and quite not really efficient solution is to convert slint model, to simpler model, which can be passed to another thread.
// Models are converted multiple times, so this have some big overhead
// ModelRc<MainListModel> --cloning when iterating + converting--> SimplerMainListModel --conversion before setting to model--> ModelRc<MainListModel> --cloning when iterating to remove useless items--> ModelRc<MainListModel>

pub struct ModelProcessor {
    pub active_tab: ActiveTab,
}

#[derive(Clone, Copy)]
pub enum MessageType {
    Delete,
    Rename,
    Move,
    Hardlink,
    Symlink,
    OptimizeVideo,
    CleanExif,
}

impl MessageType {
    fn get_empty_message(self) -> String {
        match self {
            Self::Delete => flk!("rust_no_files_deleted"),
            Self::Rename => flk!("rust_no_files_renamed"),
            Self::Move => flk!("rust_no_files_moved"),
            Self::Hardlink => flk!("rust_no_files_hardlinked"),
            Self::Symlink => flk!("rust_no_files_symlinked"),
            Self::OptimizeVideo => flk!("rust_no_videos_optimized"),
            Self::CleanExif => flk!("rust_no_exif_cleaned"),
        }
    }
    fn get_summary_message(self, deleted: usize, failed: usize, total: usize) -> String {
        match self {
            Self::Delete => flk!("rust_delete_summary", deleted = deleted, failed = failed, total = total),
            Self::Rename => flk!("rust_rename_summary", renamed = deleted, failed = failed, total = total),
            Self::Move => flk!("rust_move_summary", moved = deleted, failed = failed, total = total),
            Self::Hardlink => flk!("rust_hardlink_summary", hardlinked = deleted, failed = failed, total = total),
            Self::Symlink => flk!("rust_symlink_summary", symlinked = deleted, failed = failed, total = total),
            Self::OptimizeVideo => flk!("rust_optimize_video_summary", optimized = deleted, failed = failed, total = total),
            Self::CleanExif => flk!("rust_clean_exif_summary", cleaned = deleted, failed = failed, total = total),
        }
    }
    fn get_base_progress(self) -> ProgressData {
        match self {
            Self::Delete => ProgressData::get_empty_state(CurrentStage::DeletingFiles),
            Self::Rename => ProgressData::get_empty_state(CurrentStage::RenamingFiles),
            Self::Move => ProgressData::get_empty_state(CurrentStage::MovingFiles),
            Self::Hardlink => ProgressData::get_empty_state(CurrentStage::HardlinkingFiles),
            Self::Symlink => ProgressData::get_empty_state(CurrentStage::SymlinkingFiles),
            Self::OptimizeVideo => ProgressData::get_empty_state(CurrentStage::OptimizingVideos),
            Self::CleanExif => ProgressData::get_empty_state(CurrentStage::CleaningExif),
        }
    }
    fn msg_type(self) -> &'static str {
        match self {
            Self::Delete => "delete",
            Self::Rename => "rename",
            Self::Move => "move",
            Self::Hardlink => "hardlink",
            Self::Symlink => "symlink",
            Self::OptimizeVideo => "optimize_video",
            Self::CleanExif => "clean_exif",
        }
    }
}

impl ModelProcessor {
    pub fn new(active_tab: ActiveTab) -> Self {
        Self { active_tab }
    }

    pub(crate) fn remove_single_items_in_groups(&self, items: Vec<MainListModel>) -> Vec<MainListModel> {
        let have_header = self.active_tab.get_is_header_mode();
        model_operations::remove_single_items_in_groups(items, have_header)
    }

    pub(crate) fn remove_deleted_items_from_model(results: ProcessingResult) -> (Vec<SimplerMainListModel>, Vec<String>, usize) {
        let mut errors = Vec::new();
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

    pub(crate) fn process_items(
        items_simplified: Vec<(usize, SimplerMainListModel)>,
        items_queued_to_delete: usize,
        sender: Sender<ProgressData>,
        stop_flag: &Arc<AtomicBool>,
        process_function: impl Fn(&SimplerMainListModel) -> Result<(), String> + Send + Sync + 'static,
        message_type: MessageType,
        size_idx: Option<usize>,
        force_single_threaded: bool,
    ) -> ProcessingResult {
        let rm_idx = Arc::new(AtomicUsize::new(0));
        let size = Arc::new(AtomicU64::new(0));
        let delayed_sender = DelayedSender::new(sender, Duration::from_millis(100));

        let fnc = |(idx, data): (usize, SimplerMainListModel)| {
            if !data.checked {
                return (idx, data, None);
            }

            // Stop requested, so just return items
            if stop_flag.load(Ordering::Relaxed) {
                return (idx, data, None);
            }

            let rm_idx = rm_idx.fetch_add(1, Ordering::Relaxed);
            let size = size.fetch_add(size_idx.map(|size_idx| data.get_size(size_idx)).unwrap_or_default(), Ordering::Relaxed);
            let mut progress = message_type.get_base_progress();
            progress.entries_to_check = items_queued_to_delete;
            progress.entries_checked = rm_idx;
            progress.bytes_checked = size;
            delayed_sender.send(progress);

            let res = process_function(&data);
            (idx, data, Some(res))
        };

        let mut output: Vec<_> = if force_single_threaded {
            items_simplified.into_iter().map(fnc).collect()
        } else {
            items_simplified.into_par_iter().map(fnc).collect()
        };
        output.sort_by_key(|(idx, _, _)| *idx);

        output
    }

    pub(crate) fn process_and_update_gui_state(
        self,
        weak_app: &Weak<MainWindow>,
        stop_flag: Arc<AtomicBool>,
        progress_sender: &Sender<ProgressData>,
        simpler_model: Vec<(usize, SimplerMainListModel)>,
        dlt_fnc: impl Fn(&SimplerMainListModel) -> Result<(), String> + Send + Sync + 'static,
        message_type: MessageType,
        force_single_threaded: bool,
    ) {
        weak_app
            .upgrade_in_event_loop(move |app| {
                app.set_processing(true); // TODO processing should be probably set in gui
            })
            .expect("Failed to update app info text");

        let items_queued_to_delete = simpler_model.iter().filter(|(_idx, e)| e.checked).count();
        debug!("Processing {} items for {}", items_queued_to_delete, message_type.msg_type());
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

        let size_idx = self.active_tab.get_int_size_opt_idx();

        // Sending progress data about how many items are queued to delete
        let mut base_progress = message_type.get_base_progress();
        base_progress.entries_to_check = items_queued_to_delete;
        base_progress.bytes_to_check = size_idx
            .map(|size_idx| simpler_model.iter().map(|(_idx, m)| if m.checked { m.get_size(size_idx) } else { 0 }).sum())
            .unwrap_or_default();
        let _ = progress_sender.send(base_progress).map_err(|e| error!("Failed to send progress data: {e}"));

        let start_time = std::time::Instant::now();
        let results = Self::process_items(
            simpler_model,
            items_queued_to_delete,
            progress_sender.clone(),
            &stop_flag,
            dlt_fnc,
            message_type,
            size_idx,
            force_single_threaded,
        );
        let processing_time = start_time.elapsed();
        let removing_items_from_model = std::time::Instant::now();
        let (new_simple_model, errors, items_deleted) = Self::remove_deleted_items_from_model(results);
        debug!(
            "Items processed in {processing_time:?}, removing items from model took {:?}, from all {} items, removed from list {}, failed to process {}",
            removing_items_from_model.elapsed(),
            items_queued_to_delete,
            items_deleted,
            errors.len()
        );
        let errors_len = errors.len();

        // Sending progress data at the end of deletion, to indicate that deletion is finished
        base_progress.entries_checked = items_deleted + errors_len;

        let _ = progress_sender.send(base_progress).map_err(|e| error!("Failed to send progress data: {e}"));

        weak_app
            .upgrade_in_event_loop(move |app| {
                let mut new_model_after_removing_useless_items = self.remove_single_items_in_groups(new_simple_model.to_vec_model());
                // Selection cache was invalidated, so we need to reset it
                for e in &mut new_model_after_removing_useless_items {
                    e.selected_row = false;
                }
                self.active_tab.set_tool_model(&app, ModelRc::new(VecModel::from(new_model_after_removing_useless_items)));

                app.global::<GuiState>()
                    .set_info_text(Messages::new_from_errors(errors.clone()).create_messages_text(MessageLimit::NoLimit).into());

                app.global::<GuiState>().set_preview_visible(false);

                reset_selection(&app, self.active_tab, true);
                set_number_of_enabled_items(&app, self.active_tab, errors_len as u64);
                stop_flag.store(false, Ordering::Relaxed);
                app.invoke_processing_ended(message_type.get_summary_message(items_deleted, errors_len, items_queued_to_delete).into());
            })
            .expect("Failed to update app after deletion");
    }
}
