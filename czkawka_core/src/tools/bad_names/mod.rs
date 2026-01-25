pub mod core;
#[cfg(test)]
mod tests;
pub mod traits;

use std::path::{Path, PathBuf};
use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::common::model::FileEntry;
use crate::common::tool_data::CommonToolData;
use crate::common::traits::ResultEntry;
use crate::flc;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct BadNameEntry {
    pub path: PathBuf,
    pub modified_date: u64,
    pub size: u64,
    pub new_name: String, // File name - not full path
}

impl ResultEntry for BadNameEntry {
    fn get_path(&self) -> &Path {
        &self.path
    }
    fn get_modified_date(&self) -> u64 {
        self.modified_date
    }
    fn get_size(&self) -> u64 {
        self.size
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct NameIssues {
    pub uppercase_extension: bool,
    pub emoji_used: bool,
    pub space_at_start_or_end: bool,
    pub non_ascii_graphical: bool, // Check if name contains only ASCII graphical characters + space
    pub restricted_charset_allowed: Vec<char>, // Always includes 0-9a-zA-Z, user can add more like '_', '-', ' '
    pub remove_duplicated_non_alphanumeric: bool, // Remove duplicated non-alphanumeric chars like __ or -- or multiple spaces
}

impl NameIssues {
    pub fn all() -> Self {
        Self {
            uppercase_extension: true,
            emoji_used: true,
            space_at_start_or_end: true,
            non_ascii_graphical: true,
            restricted_charset_allowed: vec![], // Empty vec means only 0-9a-zA-Z allowed
            remove_duplicated_non_alphanumeric: true,
        }
    }

    pub fn none() -> Self {
        Self::default()
    }

    pub fn is_empty(&self) -> bool {
        !self.uppercase_extension
            && !self.emoji_used
            && !self.space_at_start_or_end
            && !self.non_ascii_graphical
            && self.restricted_charset_allowed.is_empty()
            && !self.remove_duplicated_non_alphanumeric
    }

    pub fn has_any(&self) -> bool {
        !self.is_empty()
    }

    pub fn to_string_list(&self) -> Vec<String> {
        let mut issues = Vec::new();
        if self.uppercase_extension {
            issues.push(flc!("core_bad_name_uppercase_extension"));
        }
        if self.emoji_used {
            issues.push(flc!("core_bad_name_emoji_used"));
        }
        if self.space_at_start_or_end {
            issues.push(flc!("core_bad_name_space_at_start_end"));
        }
        if self.non_ascii_graphical {
            issues.push(flc!("core_bad_name_non_ascii"));
        }
        if !self.restricted_charset_allowed.is_empty() {
            issues.push(flc!("core_bad_name_restricted_charset"));
        }
        if self.remove_duplicated_non_alphanumeric {
            issues.push("Duplicated non-alphanumeric chars".to_string());
        }
        issues
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct NameFixerParams {
    // Empty - fixing has no parameters
}

#[derive(Default, Clone)]
pub struct Info {
    pub number_of_files_with_bad_names: usize,
    pub scanning_time: Duration,
}

#[derive(Clone)]
pub struct BadNamesParameters {
    pub checked_issues: NameIssues,
}

impl BadNamesParameters {
    pub fn new(checked_issues: NameIssues) -> Self {
        Self { checked_issues }
    }
}

impl Default for BadNamesParameters {
    fn default() -> Self {
        Self {
            checked_issues: NameIssues::all(),
        }
    }
}

pub struct BadNames {
    common_data: CommonToolData,
    information: Info,
    files_to_check: Vec<FileEntry>,
    bad_names_files: Vec<BadNameEntry>,
    params: BadNamesParameters,
}

impl BadNames {
    pub const fn get_bad_names_files(&self) -> &Vec<BadNameEntry> {
        &self.bad_names_files
    }

    pub fn get_params(&self) -> &BadNamesParameters {
        &self.params
    }

    pub const fn get_information(&self) -> &Info {
        &self.information
    }
}
