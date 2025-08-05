pub mod core;
pub mod traits;

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::common::tool_data::CommonToolData;
use crate::common::traits::ResultEntry;

#[derive(Clone, Debug)]
pub struct FolderEntry {
    pub path: PathBuf,
    pub(crate) parent_path: Option<String>,
    // Usable only when finding
    pub(crate) is_empty: FolderEmptiness,
    pub modified_date: u64,
}

impl ResultEntry for FolderEntry {
    fn get_path(&self) -> &Path {
        &self.path
    }

    fn get_modified_date(&self) -> u64 {
        self.modified_date
    }

    fn get_size(&self) -> u64 {
        0
    }
}

pub struct EmptyFolder {
    common_data: CommonToolData,
    information: Info,
    empty_folder_list: HashMap<String, FolderEntry>, // Path, FolderEntry
}

/// Enum with values which show if folder is empty.
/// In function "`optimize_folders`" automatically "Maybe" is changed to "Yes", so it is not necessary to put it here
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub(crate) enum FolderEmptiness {
    No,
    Maybe,
}

#[derive(Default, Clone)]
pub struct Info {
    pub number_of_empty_folders: usize,
}

impl Default for EmptyFolder {
    fn default() -> Self {
        Self::new()
    }
}
