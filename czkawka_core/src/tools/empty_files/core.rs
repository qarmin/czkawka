

use std::io::prelude::*;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use crossbeam_channel::Sender;
use fun_time::fun_time;
use log::debug;

use crate::common::dir_traversal::{DirTraversalBuilder, DirTraversalResult};
use crate::common::model::{FileEntry, ToolType, WorkContinueStatus};
use crate::common::progress_data::ProgressData;
use crate::common::tool_data::{CommonData, CommonToolData, DeleteItemType, DeleteMethod};
use crate::common::traits::*;
use crate::tools::empty_files::{EmptyFiles, Info};

impl EmptyFiles {
    pub fn new() -> Self {
        Self {
            common_data: CommonToolData::new(ToolType::EmptyFiles),
            information: Info::default(),
            empty_files: vec![],
        }
    }

    #[fun_time(message = "find_empty_files", level = "info")]
    pub fn find_empty_files(&mut self, stop_flag: &Arc<AtomicBool>, progress_sender: Option<&Sender<ProgressData>>) {
        self.prepare_items();
        if self.check_files(stop_flag, progress_sender) == WorkContinueStatus::Stop {
            self.common_data.stopped_search = true;
            return;
        }
        if self.delete_files(stop_flag, progress_sender) == WorkContinueStatus::Stop {
            self.common_data.stopped_search = true;
            return;
        };
        self.debug_print();
    }

    #[fun_time(message = "check_files", level = "debug")]
    fn check_files(&mut self, stop_flag: &Arc<AtomicBool>, progress_sender: Option<&Sender<ProgressData>>) -> WorkContinueStatus {
        let result = DirTraversalBuilder::new()
            .common_data(&self.common_data)
            .group_by(|_fe| ())
            .stop_flag(stop_flag)
            .progress_sender(progress_sender)
            .minimal_file_size(0)
            .maximal_file_size(0)
            .build()
            .run();

        match result {
            DirTraversalResult::SuccessFiles { grouped_file_entries, warnings } => {
                self.empty_files = grouped_file_entries.into_values().flatten().collect();
                self.information.number_of_empty_files = self.empty_files.len();
                self.common_data.text_messages.warnings.extend(warnings);

                debug!("Found {} empty files.", self.information.number_of_empty_files);

                WorkContinueStatus::Continue
            }

            DirTraversalResult::Stopped => WorkContinueStatus::Stop,
        }
    }
}