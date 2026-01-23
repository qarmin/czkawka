use std::collections::BTreeSet;
use std::mem;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use crossbeam_channel::Sender;
use fun_time::fun_time;
use indexmap::IndexMap;
use log::debug;
use mime_guess::get_mime_extensions;
use rayon::prelude::*;

use crate::common::dir_traversal::{DirTraversalBuilder, DirTraversalResult};
use crate::common::model::{FileEntry, ToolType, WorkContinueStatus};
use crate::common::progress_data::{CurrentStage, ProgressData};
use crate::common::progress_stop_handler::{check_if_stop_received, prepare_thread_handler_common};
use crate::common::tool_data::{CommonData, CommonToolData};
use crate::tools::bad_extensions::workarounds::{DISABLED_EXTENSIONS, WORKAROUNDS};
use crate::tools::bad_extensions::{BadExtensions, BadExtensionsParameters, BadFileEntry, Info};

// Text longer than 10 characters is not considered as extension
const MAX_EXTENSION_LENGTH: usize = 10;

impl BadExtensions {
    pub fn new(params: BadExtensionsParameters) -> Self {
        Self {
            common_data: CommonToolData::new(ToolType::BadExtensions),
            information: Info::default(),
            files_to_check: Default::default(),
            bad_extensions_files: Default::default(),
            params,
        }
    }

    #[fun_time(message = "check_files", level = "debug")]
    pub(crate) fn check_files(&mut self, stop_flag: &Arc<AtomicBool>, progress_sender: Option<&Sender<ProgressData>>) -> WorkContinueStatus {
        let result = DirTraversalBuilder::new()
            .common_data(&self.common_data)
            .group_by(|_fe| ())
            .stop_flag(stop_flag)
            .progress_sender(progress_sender)
            .build()
            .run();

        match result {
            DirTraversalResult::SuccessFiles { grouped_file_entries, warnings } => {
                self.files_to_check = grouped_file_entries.into_values().flatten().collect();
                self.common_data.text_messages.warnings.extend(warnings);

                WorkContinueStatus::Continue
            }

            DirTraversalResult::Stopped => WorkContinueStatus::Stop,
        }
    }

    #[fun_time(message = "look_for_bad_extensions_files", level = "debug")]
    pub(crate) fn look_for_bad_extensions_files(&mut self, stop_flag: &Arc<AtomicBool>, progress_sender: Option<&Sender<ProgressData>>) -> WorkContinueStatus {
        if self.files_to_check.is_empty() {
            return WorkContinueStatus::Continue;
        }

        let progress_handler = prepare_thread_handler_common(progress_sender, CurrentStage::BadExtensionsChecking, self.files_to_check.len(), self.get_test_type(), 0);

        let files_to_check = mem::take(&mut self.files_to_check);

        let mut workarounds: IndexMap<&str, Vec<&str>> = Default::default();
        for (proper, found) in WORKAROUNDS {
            workarounds.entry(found).or_default().push(proper);
        }

        self.bad_extensions_files = self.verify_extensions(files_to_check, progress_handler.items_counter(), stop_flag, &workarounds);

        progress_handler.join_thread();

        // Break if stop was clicked
        if check_if_stop_received(stop_flag) {
            return WorkContinueStatus::Stop;
        }

        self.information.number_of_files_with_bad_extension = self.bad_extensions_files.len();

        debug!("Found {} files with invalid extension.", self.information.number_of_files_with_bad_extension);

        WorkContinueStatus::Continue
    }

    fn verify_extension_of_file(&self, file_entry: FileEntry, workarounds: &IndexMap<&str, Vec<&str>>) -> Option<BadFileEntry> {
        // Check what exactly content file contains
        let kind = match infer::get_from_path(&file_entry.path) {
            Ok(k) => k?,
            Err(_) => return None,
        };
        let proper_extension = kind.extension();

        let current_extension = Self::get_and_validate_extension(&file_entry, proper_extension)?;

        // Check for all extensions that file can use(not sure if it is worth to do it)
        let (mut all_available_extensions, valid_extensions) = Self::check_for_all_extensions_that_file_can_use(workarounds, &current_extension, proper_extension);

        if all_available_extensions.is_empty() {
            // Not found any extension
            return None;
        } else if current_extension.is_empty() {
            if !self.params.include_files_without_extension {
                return None;
            }
        } else if all_available_extensions.take(&current_extension).is_some() {
            // Found proper extension
            return None;
        }

        Some(BadFileEntry {
            path: file_entry.path,
            modified_date: file_entry.modified_date,
            size: file_entry.size,
            current_extension,
            proper_extensions_group: valid_extensions,
            proper_extension: proper_extension.to_string(),
        })
    }

    #[fun_time(message = "verify_extensions", level = "debug")]
    fn verify_extensions(
        &self,
        files_to_check: Vec<FileEntry>,
        items_counter: &Arc<AtomicUsize>,
        stop_flag: &Arc<AtomicBool>,
        workarounds: &IndexMap<&str, Vec<&str>>,
    ) -> Vec<BadFileEntry> {
        files_to_check
            .into_par_iter()
            .map(|file_entry| {
                if check_if_stop_received(stop_flag) {
                    return None;
                }
                let res = self.verify_extension_of_file(file_entry, workarounds);
                items_counter.fetch_add(1, Ordering::Relaxed);
                Some(res)
            })
            .while_some()
            .flatten()
            .collect::<Vec<_>>()
    }

    fn get_and_validate_extension(file_entry: &FileEntry, proper_extension: &str) -> Option<String> {
        let current_extension;
        // Extract current extension from file
        if let Some(extension) = file_entry.path.extension() {
            let extension = extension.to_string_lossy().to_lowercase();
            if DISABLED_EXTENSIONS.contains(&extension.as_str()) {
                return None;
            }
            if extension.len() > MAX_EXTENSION_LENGTH {
                current_extension = String::new();
            } else {
                current_extension = extension;
            }
        } else {
            current_extension = String::new();
        }

        // Already have proper extension, no need to do more things
        if current_extension == proper_extension {
            return None;
        }
        Some(current_extension)
    }

    fn check_for_all_extensions_that_file_can_use(workarounds: &IndexMap<&str, Vec<&str>>, current_extension: &str, proper_extension: &str) -> (BTreeSet<String>, String) {
        let mut all_available_extensions: BTreeSet<String> = Default::default();
        for mim in mime_guess::from_ext(proper_extension) {
            if let Some(all_ext) = get_mime_extensions(&mim) {
                for ext in all_ext {
                    all_available_extensions.insert((*ext).to_string());
                }
            }
        }

        // Workarounds:
        if !current_extension.is_empty()
            && let Some(vec_pre) = workarounds.get(current_extension)
        {
            for pre in vec_pre {
                if all_available_extensions.contains(*pre) {
                    all_available_extensions.insert(current_extension.to_string());
                    break;
                }
            }
        }

        let valid_extensions = if all_available_extensions.is_empty() {
            String::new()
        } else {
            let mut guessed_multiple_extensions = format!("({proper_extension}) - ");
            for ext in &all_available_extensions {
                guessed_multiple_extensions.push_str(ext);
                guessed_multiple_extensions.push(',');
            }
            guessed_multiple_extensions.pop();
            guessed_multiple_extensions
        };

        (all_available_extensions, valid_extensions)
    }
}
