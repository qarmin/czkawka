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
use crate::tools::bad_names::{BadNameEntry, BadNames, BadNamesParameters, CharsetFixMethod, Info, NameFixerParams, NameIssues};

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

        debug!("look_for_bad_names_files - started checking for bad names");
        let bad_names_files: Vec<BadNameEntry> = files_to_check
            .into_par_iter()
            .filter_map(|file_entry| {
                if check_if_stop_received(stop_flag) {
                    return None;
                }

                let size = file_entry.size;
                let result = check_file_name(&file_entry.path, self.params.checked_issues).map(|issues| BadNameEntry {
                    path: file_entry.path,
                    modified_date: file_entry.modified_date,
                    size: file_entry.size,
                    issues,
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
    pub fn fix_bad_names(&mut self, fix_params: NameFixerParams, stop_flag: &Arc<AtomicBool>) -> WorkContinueStatus {
        let mut fixed_count = 0;
        let mut failed_renames = Vec::new();

        for entry in &self.bad_names_files {
            if check_if_stop_received(stop_flag) {
                return WorkContinueStatus::Stop;
            }

            let has_issues_to_fix = (entry.issues.uppercase_extension && fix_params.fix_uppercase_extension)
                || (entry.issues.non_ascii_name && fix_params.fix_non_ascii.is_some())
                || (entry.issues.emoji_used && fix_params.fix_emoji)
                || (entry.issues.space_at_start_or_end && fix_params.fix_space_at_start_or_end)
                || (entry.issues.restricted_charset && fix_params.fix_restricted_charset.is_some());

            if has_issues_to_fix && let Some(new_name) = generate_fixed_name(&entry.path, &fix_params) {
                let new_path = entry.path.with_file_name(new_name);

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
        }

        if !failed_renames.is_empty() {
            self.common_data.text_messages.warnings.extend(failed_renames);
        }

        debug!("Fixed {fixed_count} file names");
        WorkContinueStatus::Continue
    }
}

pub fn check_file_name(path: &Path, checked_issues: NameIssues) -> Option<NameIssues> {
    let file_name = path.file_name()?.to_string_lossy();
    let mut issues = NameIssues::none();

    if checked_issues.uppercase_extension
        && let Some(extension) = path.extension()
    {
        let ext_str = extension.to_string_lossy();
        if ext_str.chars().any(|c| c.is_uppercase()) {
            issues.uppercase_extension = true;
        }
    }

    if checked_issues.non_ascii_name && !file_name.is_ascii() {
        issues.non_ascii_name = true;
    }

    if checked_issues.emoji_used && file_name.chars().any(is_emoji) {
        issues.emoji_used = true;
    }

    if checked_issues.space_at_start_or_end {
        if let Some(stem) = path.file_stem() {
            let stem_str = stem.to_string_lossy();
            if stem_str.starts_with(' ') || stem_str.ends_with(' ') {
                issues.space_at_start_or_end = true;
            }
        }
        if let Some(extension) = path.extension() {
            let ext_str = extension.to_string_lossy();
            if ext_str.starts_with(' ') || ext_str.ends_with(' ') {
                issues.space_at_start_or_end = true;
            }
        }
    }

    if checked_issues.restricted_charset && file_name.chars().any(|c| !is_allowed_char(c)) {
        issues.restricted_charset = true;
    }

    if issues.is_empty() { None } else { Some(issues) }
}

fn is_allowed_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == ' ' || c == '_' || c == '.'
}

pub fn generate_fixed_name(path: &Path, fix_params: &NameFixerParams) -> Option<String> {
    let file_name = path.file_name()?.to_string_lossy();
    let mut stem = path.file_stem()?.to_string_lossy().to_string();
    let mut extension = path.extension().map(|e| e.to_string_lossy().to_string());

    if fix_params.fix_space_at_start_or_end {
        stem = stem.trim().to_string();
        if let Some(ref mut ext) = extension {
            *ext = ext.trim().to_string();
        }
    }

    if fix_params.fix_emoji {
        stem = stem.chars().filter(|c| !is_emoji(*c)).collect();
        if let Some(ref mut ext) = extension {
            *ext = ext.chars().filter(|c| !is_emoji(*c)).collect();
        }
    }

    if let Some(method) = fix_params.fix_non_ascii {
        stem = fix_non_ascii(&stem, method);
        if let Some(ref mut ext) = extension {
            *ext = fix_non_ascii(ext, method);
        }
    }

    if let Some(method) = fix_params.fix_restricted_charset {
        stem = fix_restricted_charset(&stem, method);
        if let Some(ref mut ext) = extension {
            *ext = fix_restricted_charset(ext, method);
        }
    }

    if fix_params.fix_uppercase_extension
        && let Some(ref mut ext) = extension
        && ext.chars().any(|c| c.is_uppercase())
    {
        *ext = ext.to_lowercase();
    }

    let new_name = if let Some(ext) = extension {
        if ext.is_empty() { stem } else { format!("{stem}.{ext}") }
    } else {
        stem
    };

    if new_name != file_name.as_ref() as &str { Some(new_name) } else { None }
}

fn fix_non_ascii(s: &str, method: CharsetFixMethod) -> String {
    match method {
        CharsetFixMethod::ReplaceWithUnderscore => s.chars().map(|c| if c.is_ascii() { c } else { '_' }).collect(),
        CharsetFixMethod::ReplaceWithSpace => s.chars().map(|c| if c.is_ascii() { c } else { ' ' }).collect(),
        CharsetFixMethod::Delete => s.chars().filter(|c| c.is_ascii()).collect(),
        CharsetFixMethod::Transliterate => s
            .chars()
            .flat_map(|c| {
                if c.is_ascii() {
                    vec![c]
                } else {
                    deunicode::deunicode_char(c)
                        .map(|s| {
                            let chars: Vec<char> = s.chars().collect();
                            if chars.is_empty() || (chars.len() == 1 && !chars[0].is_ascii_graphic()) {
                                vec![' ']
                            } else {
                                chars
                            }
                        })
                        .unwrap_or_else(|| vec![' '])
                }
            })
            .collect(),
    }
}

fn fix_restricted_charset(s: &str, method: CharsetFixMethod) -> String {
    match method {
        CharsetFixMethod::ReplaceWithUnderscore => s.chars().map(|c| if is_allowed_char(c) { c } else { '_' }).collect(),
        CharsetFixMethod::ReplaceWithSpace => s.chars().map(|c| if is_allowed_char(c) { c } else { ' ' }).collect(),
        CharsetFixMethod::Delete => s.chars().filter(|c| is_allowed_char(*c)).collect(),
        CharsetFixMethod::Transliterate => s
            .chars()
            .flat_map(|c| {
                if is_allowed_char(c) {
                    vec![c]
                } else if !c.is_ascii() {
                    deunicode::deunicode_char(c)
                        .map(|s| {
                            let chars: Vec<char> = s.chars().filter(|ch| is_allowed_char(*ch)).collect();
                            if chars.is_empty() { vec!['_'] } else { chars }
                        })
                        .unwrap_or_else(|| vec!['_'])
                } else {
                    vec!['_']
                }
            })
            .collect(),
    }
}

fn is_emoji(c: char) -> bool {
    let code = c as u32;
    matches!(code,
        0x1F600..=0x1F64F |
        0x1F300..=0x1F5FF |
        0x1F680..=0x1F6FF |
        0x1F1E0..=0x1F1FF |
        0x2600..=0x26FF   |
        0x2700..=0x27BF   |
        0xFE00..=0xFE0F   |
        0x1F900..=0x1F9FF |
        0x1F018..=0x1F270 |
        0x238C..=0x2454   |
        0x20D0..=0x20FF
    )
}
