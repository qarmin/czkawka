pub mod core;
#[cfg(test)]
mod tests;
pub mod traits;

use std::path::{Path, PathBuf};
use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::common::tool_data::CommonToolData;
use crate::common::traits::ResultEntry;
use crate::flc;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct BadNameEntry {
    pub path: PathBuf,
    pub modified_date: u64,
    pub size: u64,
    pub issues: NameIssues,
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

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct  NameIssues {
    pub uppercase_extension: bool,
    pub emoji_used: bool,
    pub space_at_start_or_end: bool,
    pub non_ascii_name: Option<CharsetFixMethod>,
    pub restricted_charset: Option<CharsetFixMethod>,
}

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub enum CharsetFixMethod {
    ReplaceWithUnderscore,
    ReplaceWithSpace,
    Delete,
    #[default]
    Transliterate,
}

impl NameIssues {
    pub fn all() -> Self {
        Self {
            uppercase_extension: true,
            emoji_used: true,
            space_at_start_or_end: true,
            non_ascii_name: Some(CharsetFixMethod::default()),
            restricted_charset: Some(CharsetFixMethod::default()),
        }
    }

    pub fn none() -> Self {
        Self::default()
    }

    pub fn is_empty(&self) -> bool {
        !self.uppercase_extension
            && !self.emoji_used
            && !self.space_at_start_or_end
            && self.non_ascii_name.is_none()
            && self.restricted_charset.is_none()
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
        if self.non_ascii_name.is_some() {
            issues.push(flc!("core_bad_name_non_ascii"));
        }
        if self.restricted_charset.is_some() {
            issues.push(flc!("core_bad_name_restricted_charset"));
        }
        issues
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct NameFixerParams {
    pub fix_uppercase_extension: bool,
    pub fix_emoji: bool,
    pub fix_space_at_start_or_end: bool,
    pub fix_non_ascii: Option<CharsetFixMethod>,
    pub fix_restricted_charset: Option<CharsetFixMethod>,
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
