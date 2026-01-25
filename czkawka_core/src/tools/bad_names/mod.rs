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

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct BadNameEntry {
    pub path: PathBuf,
    pub modified_date: u64,
    pub size: u64,
    pub new_name: String,
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
    pub non_ascii_graphical: bool,
    pub restricted_charset_allowed: Option<Vec<char>>,
    pub remove_duplicated_non_alphanumeric: bool,
}

impl NameIssues {
    pub fn all() -> Self {
        Self {
            uppercase_extension: true,
            emoji_used: true,
            space_at_start_or_end: true,
            non_ascii_graphical: true,
            restricted_charset_allowed: Some(Vec::new()),
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
            && self.restricted_charset_allowed.is_none()
            && !self.remove_duplicated_non_alphanumeric
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
