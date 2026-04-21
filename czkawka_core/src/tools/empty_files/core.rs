use std::fs;
use std::mem;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use crossbeam_channel::Sender;
use fun_time::fun_time;
use log::debug;
use rayon::prelude::*;

use crate::common::dir_traversal::{DirTraversalBuilder, DirTraversalResult};
use crate::common::model::{CheckingMethod, FileEntry, ToolType, WorkContinueStatus};
use crate::common::progress_data::{CurrentStage, ProgressData};
use crate::common::progress_stop_handler::{check_if_stop_received, prepare_thread_handler_common};
use crate::common::tool_data::CommonToolData;
use crate::tools::empty_files::{EmptyFiles, EmptyFilesParameters, Info};

impl EmptyFiles {
    pub fn new(params: EmptyFilesParameters) -> Self {
        Self {
            common_data: CommonToolData::new(ToolType::EmptyFiles),
            information: Info::default(),
            empty_files: Vec::new(),
            files_to_check: Vec::new(),
            params,
        }
    }

    fn effective_checking_method(&self) -> CheckingMethod {
        if self.params.search_zero_byte_content_files || self.params.search_whitespace_content_files {
            CheckingMethod::EmptyFilesContent
        } else {
            CheckingMethod::None
        }
    }

    /// Stage 0 – traverses the directory tree and splits results:
    /// - zero-size files go directly into `self.empty_files`
    /// - non-zero files are stored in `self.files_to_check` for stage 1 (content check mode only)
    #[fun_time(message = "collect_files", level = "debug")]
    pub(crate) fn collect_files(&mut self, stop_flag: &Arc<AtomicBool>, progress_sender: Option<&Sender<ProgressData>>) -> WorkContinueStatus {
        let checking_method = self.effective_checking_method();
        let max_size = if checking_method == CheckingMethod::EmptyFilesContent { u64::MAX } else { 0 };

        let result = DirTraversalBuilder::new()
            .common_data(&self.common_data)
            .group_by(|_fe| ())
            .stop_flag(stop_flag)
            .progress_sender(progress_sender)
            .minimal_file_size(0)
            .maximal_file_size(max_size)
            .checking_method(checking_method)
            .build()
            .run();

        match result {
            DirTraversalResult::SuccessFiles { grouped_file_entries, warnings } => {
                for fe in grouped_file_entries.into_values().flatten() {
                    if fe.size == 0 {
                        self.empty_files.push(fe);
                    } else {
                        self.files_to_check.push(fe);
                    }
                }
                self.common_data.text_messages.warnings.extend(warnings);
                debug!("collect_files – {} zero-size, {} queued for content check", self.empty_files.len(), self.files_to_check.len());
                WorkContinueStatus::Continue
            }
            DirTraversalResult::Stopped => WorkContinueStatus::Stop,
        }
    }

    /// Stage 1 – reads every file from `self.files_to_check` and keeps only those whose
    /// entire content consists of null bytes (zero-byte mode) or ASCII whitespace characters
    /// (whitespace mode).  Clears `self.files_to_check` when done.
    #[fun_time(message = "check_content", level = "debug")]
    pub(crate) fn check_content(&mut self, stop_flag: &Arc<AtomicBool>, progress_sender: Option<&Sender<ProgressData>>) -> WorkContinueStatus {
        let files = mem::take(&mut self.files_to_check);
        if files.is_empty() {
            return WorkContinueStatus::Continue;
        }

        let total_size: u64 = files.iter().map(|fe| fe.size).sum();
        let progress_handler = prepare_thread_handler_common(
            progress_sender,
            CurrentStage::EmptyFilesCheckingContent,
            files.len(),
            (ToolType::EmptyFiles, CheckingMethod::EmptyFilesContent),
            total_size,
        );

        let search_whitespace = self.params.search_whitespace_content_files;
        let stopped = AtomicBool::new(false);

        let mut matches: Vec<FileEntry> = files
            .into_par_iter()
            .map(|fe| -> Option<Option<FileEntry>> {
                if check_if_stop_received(stop_flag) {
                    stopped.store(true, Ordering::Relaxed);
                    return None;
                }
                let size = fe.size;
                let Ok(content) = fs::read(&fe.path) else {
                    progress_handler.increase_items(1);
                    progress_handler.increase_size(size);
                    return Some(None);
                };
                let is_match = if search_whitespace {
                    content.iter().all(|&b| matches!(b, 0x00 | 0x09 | 0x0A | 0x0B | 0x0C | 0x0D | 0x20))
                } else {
                    // search_zero_byte_content_files is guaranteed true here
                    content.iter().all(|&b| b == 0x00)
                };
                progress_handler.increase_items(1);
                progress_handler.increase_size(size);
                if is_match { Some(Some(fe)) } else { Some(None) }
            })
            .while_some()
            .flatten()
            .collect();

        progress_handler.join_thread();

        if stopped.load(Ordering::Relaxed) {
            return WorkContinueStatus::Stop;
        }

        self.empty_files.append(&mut matches);
        WorkContinueStatus::Continue
    }

    #[fun_time(message = "check_files", level = "debug")]
    pub(crate) fn check_files(&mut self, stop_flag: &Arc<AtomicBool>, progress_sender: Option<&Sender<ProgressData>>) -> WorkContinueStatus {
        if self.collect_files(stop_flag, progress_sender) == WorkContinueStatus::Stop {
            return WorkContinueStatus::Stop;
        }
        if self.params.search_zero_byte_content_files || self.params.search_whitespace_content_files {
            if self.check_content(stop_flag, progress_sender) == WorkContinueStatus::Stop {
                return WorkContinueStatus::Stop;
            }
        }
        self.information.number_of_empty_files = self.empty_files.len();
        debug!("Found {} empty files total.", self.information.number_of_empty_files);
        WorkContinueStatus::Continue
    }
}
