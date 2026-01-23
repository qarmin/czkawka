use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use crossbeam_channel::Sender;
use fun_time::fun_time;
use log::debug;

use crate::common::dir_traversal::{Collect, DirTraversalBuilder, DirTraversalResult};
use crate::common::model::{ToolType, WorkContinueStatus};
use crate::common::progress_data::ProgressData;
use crate::common::tool_data::CommonToolData;
use crate::tools::invalid_symlinks::{ErrorType, Info, InvalidSymlinks, MAX_NUMBER_OF_SYMLINK_JUMPS, SymlinkInfo};

impl InvalidSymlinks {
    pub fn new() -> Self {
        Self {
            common_data: CommonToolData::new(ToolType::InvalidSymlinks),
            information: Info::default(),
            invalid_symlinks: Vec::new(),
        }
    }

    #[fun_time(message = "check_files", level = "debug")]
    pub(crate) fn check_files(&mut self, stop_flag: &Arc<AtomicBool>, progress_sender: Option<&Sender<ProgressData>>) -> WorkContinueStatus {
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
                let mut loop_count = 0;
                let mut current_path = current_file_name.to_path_buf();
                loop {
                    if loop_count == 0 && !current_path.exists() {
                        type_of_error = ErrorType::NonExistentFile;
                        break;
                    }
                    if loop_count == MAX_NUMBER_OF_SYMLINK_JUMPS {
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

                    loop_count += 1;
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
