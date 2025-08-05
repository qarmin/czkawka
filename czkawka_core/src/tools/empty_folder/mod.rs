mod core;
mod traits;

use std::collections::HashMap;
use std::fs::DirEntry;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use crossbeam_channel::Sender;
use fun_time::fun_time;
use log::debug;
use rayon::prelude::*;

use crate::common::dir_traversal::{common_get_entry_data, common_get_metadata_dir, common_read_dir, get_modified_time};
use crate::common::directories::Directories;
use crate::common::items::ExcludedItems;
use crate::common::model::{ToolType, WorkContinueStatus};
use crate::common::progress_data::{CurrentStage, ProgressData};
use crate::common::progress_stop_handler::{check_if_stop_received, prepare_thread_handler_common};
use crate::common::tool_data::{CommonData, CommonToolData, DeleteItemType, DeleteMethod};
use crate::common::traits::{DebugPrint, DeletingItems, PrintResults, ResultEntry};

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