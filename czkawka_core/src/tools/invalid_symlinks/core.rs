
use std::fmt::Display;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use crossbeam_channel::Sender;
use fun_time::fun_time;
use log::debug;
use serde::{Deserialize, Serialize};

use crate::common::dir_traversal::{Collect, DirTraversalBuilder, DirTraversalResult};
use crate::common::model::{FileEntry, ToolType, WorkContinueStatus};
use crate::common::progress_data::ProgressData;
use crate::common::tool_data::{CommonData, CommonToolData, DeleteItemType, DeleteMethod};
use crate::common::traits::*;
use crate::flc;
use crate::tools::invalid_symlinks::{ErrorType, Info, InvalidSymlinks, SymlinkInfo, MAX_NUMBER_OF_SYMLINK_JUMPS};

impl InvalidSymlinks {
    pub fn new() -> Self {
        Self {
            common_data: CommonToolData::new(ToolType::InvalidSymlinks),
            information: Info::default(),
            invalid_symlinks: vec![],
        }
    }

    #[fun_time(message = "find_invalid_links", level = "info")]
    pub fn find_invalid_links(&mut self, stop_flag: &Arc<AtomicBool>, progress_sender: Option<&Sender<ProgressData>>) {
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
            .collect(Collect::InvalidSymlinks)
            .build()
            .run();

        match result {
            DirTraversalResult::SuccessFiles { grouped_file_entries, warnings } => {
                self.invalid_symlinks = grouped_file_entries
                    .into_values()
                    .flatten()
                    .filter_map(|e| {
                        let (destination_path, type_of_error) = Self::check_invalid_symlinks(&e.path)?;
                        Some(e.into_symlinks_entry(SymlinkInfo { destination_path, type_of_error }))
                    })
                    .collect();
                self.information.number_of_invalid_symlinks = self.invalid_symlinks.len();
                self.common_data.text_messages.warnings.extend(warnings);
                debug!("Found {} invalid symlinks.", self.information.number_of_invalid_symlinks);
                WorkContinueStatus::Continue
            }
            DirTraversalResult::Stopped => WorkContinueStatus::Stop,
        }
    }

    fn check_invalid_symlinks(current_file_name: &Path) -> Option<(PathBuf, ErrorType)> {
        let mut destination_path = PathBuf::new();
        let type_of_error;

        match current_file_name.read_link() {
            Ok(t) => {
                destination_path.push(t);
                let mut number_of_loop = 0;
                let mut current_path = current_file_name.to_path_buf();
                loop {
                    if number_of_loop == 0 && !current_path.exists() {
                        type_of_error = ErrorType::NonExistentFile;
                        break;
                    }
                    if number_of_loop == MAX_NUMBER_OF_SYMLINK_JUMPS {
                        type_of_error = ErrorType::InfiniteRecursion;
                        break;
                    }

                    current_path = match current_path.read_link() {
                        Ok(t) => t,
                        Err(_inspected) => {
                            // Looks that some next symlinks are broken, but we do nothing with it - TODO why they are broken
                            return None;
                        }
                    };

                    number_of_loop += 1;
                }
            }
            Err(_inspected) => {
                // Failed to load info about it
                type_of_error = ErrorType::NonExistentFile;
            }
        }
        Some((destination_path, type_of_error))
    }
}