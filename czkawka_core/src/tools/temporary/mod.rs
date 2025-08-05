mod core;
mod traits;

use std::fs::DirEntry;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use crossbeam_channel::Sender;
use fun_time::fun_time;
use rayon::prelude::*;
use serde::Serialize;

use crate::common::dir_traversal::{common_read_dir, get_modified_time};
use crate::common::directories::Directories;
use crate::common::items::ExcludedItems;
use crate::common::model::{ToolType, WorkContinueStatus};
use crate::common::progress_data::{CurrentStage, ProgressData};
use crate::common::progress_stop_handler::{check_if_stop_received, prepare_thread_handler_common};
use crate::common::tool_data::{CommonData, CommonToolData, DeleteItemType, DeleteMethod};
use crate::common::traits::*;

const TEMP_EXTENSIONS: &[&str] = &[
    "#",
    "thumbs.db",
    ".bak",
    "~",
    ".tmp",
    ".temp",
    ".ds_store",
    ".crdownload",
    ".part",
    ".cache",
    ".dmp",
    ".download",
    ".partial",
];

#[derive(Clone, Serialize, Debug)]
pub struct TemporaryFileEntry {
    pub path: PathBuf,
    pub modified_date: u64,
    pub size: u64,
}

impl ResultEntry for TemporaryFileEntry {
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

#[derive(Default, Clone)]
pub struct Info {
    pub number_of_temporary_files: usize,
}

pub struct Temporary {
    common_data: CommonToolData,
    information: Info,
    temporary_files: Vec<TemporaryFileEntry>,
}


impl Default for Temporary {
    fn default() -> Self {
        Self::new()
    }
}



impl Temporary {
    pub const fn get_temporary_files(&self) -> &Vec<TemporaryFileEntry> {
        &self.temporary_files
    }

    pub const fn get_information(&self) -> &Info {
        &self.information
    }
}
