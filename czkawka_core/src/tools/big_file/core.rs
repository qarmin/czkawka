use std::cmp::Reverse;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use crossbeam_channel::Sender;
use fun_time::fun_time;
use log::debug;
use rayon::prelude::*;

use crate::common::dir_traversal::{DirTraversalBuilder, DirTraversalResult};
use crate::common::model::{ToolType, WorkContinueStatus};
use crate::common::progress_data::ProgressData;
use crate::common::tool_data::{CommonData, CommonToolData};
use crate::tools::big_file::{BigFile, BigFileParameters, Info, SearchMode};

impl BigFile {
    pub fn new(params: BigFileParameters) -> Self {
        Self {
            common_data: CommonToolData::new(ToolType::BigFile),
            information: Info::default(),
            big_files: Default::default(),
            params,
        }
    }

    #[fun_time(message = "look_for_big_files", level = "debug")]
    pub(crate) fn look_for_big_files(&mut self, stop_flag: &Arc<AtomicBool>, progress_sender: Option<&Sender<ProgressData>>) -> WorkContinueStatus {
        let result = DirTraversalBuilder::new()
            .group_by(|_fe| ())
            .stop_flag(stop_flag)
            .progress_sender(progress_sender)
            .common_data(&self.common_data)
            .minimal_file_size(1)
            .maximal_file_size(u64::MAX)
            .build()
            .run();

        match result {
            DirTraversalResult::SuccessFiles { grouped_file_entries, warnings } => {
                let mut all_files = grouped_file_entries.into_values().flatten().collect::<Vec<_>>();

                if self.get_params().search_mode == SearchMode::BiggestFiles {
                    all_files.par_sort_unstable_by_key(|fe| Reverse(fe.size));
                } else {
                    all_files.par_sort_unstable_by_key(|fe| fe.size);
                }

                all_files.truncate(self.get_params().number_of_files_to_check);

                self.big_files = all_files;

                self.common_data.text_messages.warnings.extend(warnings);
                self.information.number_of_real_files = self.big_files.len();
                debug!("check_files - Found {} biggest/smallest files.", self.big_files.len());
                WorkContinueStatus::Continue
            }

            DirTraversalResult::Stopped => WorkContinueStatus::Stop,
        }
    }
}
