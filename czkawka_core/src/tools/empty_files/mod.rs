mod core;
mod traits;

use std::io::prelude::*;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use crossbeam_channel::Sender;
use fun_time::fun_time;
use log::debug;

use crate::common::dir_traversal::{DirTraversalBuilder, DirTraversalResult};
use crate::common::model::{FileEntry, ToolType, WorkContinueStatus};
use crate::common::progress_data::ProgressData;
use crate::common::tool_data::{CommonData, CommonToolData, DeleteItemType, DeleteMethod};
use crate::common::traits::*;

#[derive(Default, Clone)]
pub struct Info {
    pub number_of_empty_files: usize,
}

pub struct EmptyFiles {
    common_data: CommonToolData,
    information: Info,
    empty_files: Vec<FileEntry>,
}




impl Default for EmptyFiles {
    fn default() -> Self {
        Self::new()
    }
}


impl EmptyFiles {
    pub const fn get_empty_files(&self) -> &Vec<FileEntry> {
        &self.empty_files
    }

    pub const fn get_information(&self) -> &Info {
        &self.information
    }
}
