use std::fs;
use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use crossbeam_channel::Sender;
use fun_time::fun_time;
use log::debug;
use rayon::prelude::*;

use crate::common::dir_traversal::{DirTraversalBuilder, DirTraversalResult};
use crate::common::model::{ToolType, WorkContinueStatus};
use crate::common::progress_data::{CurrentStage, ProgressData};
use crate::common::progress_stop_handler::{check_if_stop_received, prepare_thread_handler_common};
use crate::common::tool_data::{CommonData, CommonToolData};
use crate::tools::bad_names::{BadNameEntry, BadNames, BadNamesParameters, Info, NameFixerParams, NameIssues};

impl BadNames {
    pub fn new(params: BadNamesParameters) -> Self {
        Self {
            common_data: CommonToolData::new(ToolType::BadNames),
            information: Info::default(),
            files_to_check: Default::default(),
            bad_names_files: Default::default(),
            params,
        }
    }

    #[fun_time(message = "check_files", level = "debug")]
    pub(crate) fn check_files(&mut self, stop_flag: &Arc<AtomicBool>, progress_sender: Option<&Sender<ProgressData>>) -> WorkContinueStatus {
        let result = DirTraversalBuilder::new()
            .group_by(|_fe| ())
            .stop_flag(stop_flag)
            .progress_sender(progress_sender)
            .common_data(&self.common_data)
            .build()
            .run();

        match result {
            DirTraversalResult::SuccessFiles { grouped_file_entries, warnings } => {
                self.files_to_check = grouped_file_entries.into_values().flatten().collect();
                self.common_data.text_messages.warnings.extend(warnings);
                debug!("check_files - Found {} files to check.", self.files_to_check.len());

                WorkContinueStatus::Continue
            }
            DirTraversalResult::Stopped => WorkContinueStatus::Stop,
        }
    }

    #[fun_time(message = "look_for_bad_names_files", level = "debug")]
    pub(crate) fn look_for_bad_names_files(&mut self, stop_flag: &Arc<AtomicBool>, progress_sender: Option<&Sender<ProgressData>>) -> WorkContinueStatus {
        if self.files_to_check.is_empty() {
            return WorkContinueStatus::Continue;
        }

        let progress_handler = prepare_thread_handler_common(
            progress_sender,
            CurrentStage::BadNamesChecking,
            self.files_to_check.len(),
            self.get_test_type(),
            self.files_to_check.iter().map(|item| item.size).sum::<u64>(),
        );

        let files_to_check = std::mem::take(&mut self.files_to_check);
        let checked_issues = self.params.checked_issues.clone();

        debug!("look_for_bad_names_files - started checking for bad names");
        let bad_names_files: Vec<BadNameEntry> = files_to_check
            .into_par_iter()
            .filter_map(|file_entry| {
                if check_if_stop_received(stop_flag) {
                    return None;
                }

                let size = file_entry.size;
                let result = check_and_generate_new_name(&file_entry.path, &checked_issues).map(|new_name| BadNameEntry {
                    path: file_entry.path,
                    modified_date: file_entry.modified_date,
                    size: file_entry.size,
                    new_name,
                });

                progress_handler.increase_items(1);
                progress_handler.increase_size(size);

                result
            })
            .collect();

        debug!("look_for_bad_names_files - ended checking for bad names");
        progress_handler.join_thread();

        if check_if_stop_received(stop_flag) {
            return WorkContinueStatus::Stop;
        }

        self.bad_names_files = bad_names_files;
        self.information.number_of_files_with_bad_names = self.bad_names_files.len();
        debug!("Found {} files with bad names.", self.information.number_of_files_with_bad_names);

        WorkContinueStatus::Continue
    }

    #[fun_time(message = "fix_bad_names", level = "debug")]
    pub fn fix_bad_names(&mut self, _fix_params: NameFixerParams, stop_flag: &Arc<AtomicBool>) -> WorkContinueStatus {
        let mut fixed_count = 0;
        let mut failed_renames = Vec::new();

        for entry in &self.bad_names_files {
            if check_if_stop_received(stop_flag) {
                return WorkContinueStatus::Stop;
            }

            let new_path = entry.path.with_file_name(&entry.new_name);

            match fs::rename(&entry.path, &new_path) {
                Ok(()) => {
                    fixed_count += 1;
                    debug!("Renamed {:?} to {:?}", entry.path, new_path);
                }
                Err(e) => {
                    failed_renames.push(format!("Failed to rename {:?}: {}", entry.path, e));
                }
            }
        }

        if !failed_renames.is_empty() {
            self.common_data.text_messages.warnings.extend(failed_renames);
        }

        debug!("Fixed {fixed_count} file names");
        WorkContinueStatus::Continue
    }
}

// Check file name against NameIssues and generate a new fixed name if issues are found
pub fn check_and_generate_new_name(path: &Path, checked_issues: &NameIssues) -> Option<String> {
    let file_name = path.file_name()?.to_string_lossy();
    let mut stem = path.file_stem()?.to_string_lossy().to_string();
    let mut extension = path.extension().map(|e| e.to_string_lossy().to_string());

    let mut has_issues = false;

    // Check and fix uppercase extension
    if checked_issues.uppercase_extension {
        if let Some(ref mut ext) = extension {
            if ext.chars().any(|c| c.is_uppercase()) {
                has_issues = true;
                *ext = ext.to_lowercase();
            }
        }
    }

    // Check and fix emoji
    if checked_issues.emoji_used {
        let original_stem_len = stem.len();
        stem = stem.chars().filter(|c| !is_emoji(*c)).collect();
        if stem.len() != original_stem_len {
            has_issues = true;
        }

        if let Some(ref mut ext) = extension {
            let original_ext_len = ext.len();
            *ext = ext.chars().filter(|c| !is_emoji(*c)).collect();
            if ext.len() != original_ext_len {
                has_issues = true;
            }
        }
    }

    // Check and fix spaces at start or end
    if checked_issues.space_at_start_or_end {
        let trimmed_stem = stem.trim();
        if trimmed_stem != stem {
            has_issues = true;
            stem = trimmed_stem.to_string();
        }

        if let Some(ref mut ext) = extension {
            let trimmed_ext = ext.trim();
            if trimmed_ext != ext.as_str() {
                has_issues = true;
                *ext = trimmed_ext.to_string();
            }
        }
    }

    // Check and fix non-ascii graphical (ascii_graphical + space only)
    if checked_issues.non_ascii_graphical {
        let original_stem = stem.clone();
        stem = stem.chars().filter(|c| is_ascii_graphical_or_space(*c)).collect();
        if stem != original_stem {
            has_issues = true;
        }

        if let Some(ref mut ext) = extension {
            let original_ext = ext.clone();
            *ext = ext.chars().filter(|c| is_ascii_graphical_or_space(*c)).collect();
            if ext != &original_ext {
                has_issues = true;
            }
        }
    }

    // Check and fix restricted charset
    if !checked_issues.restricted_charset_allowed.is_empty() {
        let original_stem = stem.clone();
        stem = stem.chars()
            .filter(|c| is_alphanumeric(*c) || checked_issues.restricted_charset_allowed.contains(c))
            .collect();
        if stem != original_stem {
            has_issues = true;
        }

        if let Some(ref mut ext) = extension {
            let original_ext = ext.clone();
            *ext = ext.chars()
                .filter(|c| is_alphanumeric(*c) || checked_issues.restricted_charset_allowed.contains(c))
                .collect();
            if ext != &original_ext {
                has_issues = true;
            }
        }
    }

    // Check and fix duplicated non-alphanumeric chars
    if checked_issues.remove_duplicated_non_alphanumeric {
        let original_stem = stem.clone();
        stem = remove_duplicated_non_alphanumeric(&stem);
        if stem != original_stem {
            has_issues = true;
        }

        if let Some(ref mut ext) = extension {
            let original_ext = ext.clone();
            *ext = remove_duplicated_non_alphanumeric(ext);
            if ext != &original_ext {
                has_issues = true;
            }
        }
    }

    // If no issues found, return None
    if !has_issues {
        return None;
    }

    // Construct new file name
    let new_name = if let Some(ext) = extension {
        if ext.is_empty() {
            stem
        } else {
            format!("{stem}.{ext}")
        }
    } else {
        stem
    };

    // Return new name only if it's different from original
    if new_name != file_name.as_ref() as &str {
        Some(new_name)
    } else {
        None
    }
}

fn is_ascii_graphical_or_space(c: char) -> bool {
    c.is_ascii_graphic() || c == ' '
}

fn is_alphanumeric(c: char) -> bool {
    c.is_ascii_alphanumeric()
}

fn remove_duplicated_non_alphanumeric(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();

    while let Some(c) = chars.next() {
        result.push(c);

        if !c.is_ascii_alphanumeric() {
            // Skip consecutive identical non-alphanumeric characters
            while let Some(&next_c) = chars.peek() {
                if next_c == c {
                    chars.next();
                } else {
                    break;
                }
            }
        }
    }

    result
}

fn is_emoji(c: char) -> bool {
    let code = c as u32;
    matches!(code,
        // Misc symbols + pictographs
        0x231A..=0x231B |
        0x23E9..=0x23EC |
        0x23F0 |
        0x23F3 |
        0x25FD..=0x25FE |
        0x2614..=0x2615 |
        0x2648..=0x2653 |
        0x267F |
        0x2693 |
        0x26A1 |
        0x26AA..=0x26AB |
        0x26BD..=0x26BE |
        0x26C4..=0x26C5 |
        0x26CE |
        0x26D4 |
        0x26EA |
        0x26F2..=0x26F3 |
        0x26F5 |
        0x26FA |
        0x26FD |
        0x2705 |
        0x270A..=0x270B |
        0x2728 |
        0x274C |
        0x274E |
        0x2753..=0x2757 |
        0x2795..=0x2797 |
        0x27B0 |
        0x27BF |
        0x2B1B..=0x2B1C |
        0x2B50 |
        0x2B55 |

        // Enclosed characters
        0x1F004 |
        0x1F0CF |
        0x1F18E |
        0x1F191..=0x1F19A |
        0x1F201 |
        0x1F21A |
        0x1F22F |
        0x1F232..=0x1F23A |
        0x1F250..=0x1F251 |

        // Main emoji blocks
        0x1F300..=0x1F5FF |
        0x1F600..=0x1F64F |
        0x1F680..=0x1F6FF |
        0x1F900..=0x1F9FF |

        // Regional indicator symbols (flags)
        0x1F1E6..=0x1F1FF
    )
}
