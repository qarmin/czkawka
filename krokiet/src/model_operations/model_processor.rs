use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::time::Duration;

use crossbeam_channel::Sender;
use czkawka_core::progress_data::ProgressData;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::common::delayed_sender::DelayedSender;
use crate::common::get_is_header_mode;
use crate::model_operations::ProcessingResult;
use crate::simpler_model::SimplerMainListModel;
use crate::{CurrentTab, MainListModel, model_operations};

// This is quite ugly workaround for Slint strange limitation, where model cannot be passed to another thread
// This was needed by me, because I wanted to process deletion without blocking main gui thread, with additional sending progress about entire operation.
// After trying different solutions, looks that the simplest and quite not really efficient solution is to convert slint model, to simpler model, which can be passed to another thread.
// Models are converted multiple times, so this have some big overhead
// ModelRc<MainListModel> --cloning when iterating + converting--> SimplerMainListModel --conversion before setting to model--> ModelRc<MainListModel> --cloning when iterating to remove useless items--> ModelRc<MainListModel>

pub struct ModelProcessor {
    pub active_tab: CurrentTab,
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
                let mut progress = ProgressData::get_base_deleting_state();
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
}
