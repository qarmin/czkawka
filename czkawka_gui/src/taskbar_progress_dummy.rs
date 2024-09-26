#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::needless_pass_by_value)]
#![allow(clippy::pedantic)]
#![cfg(not(target_os = "windows"))]

use std::convert::From;

enum HWND__ {}

type HWND = *mut HWND__;

#[allow(non_camel_case_types, dead_code)]
pub enum TBPFLAG {
    TBPF_NOPROGRESS = 0,
    TBPF_INDETERMINATE = 0x1,
    TBPF_NORMAL = 0x2,
    TBPF_ERROR = 0x4,
    TBPF_PAUSED = 0x8,
}

pub mod tbp_flags {
    pub use super::TBPFLAG::*;
}

pub struct TaskbarProgress {}

impl TaskbarProgress {
    pub fn new() -> TaskbarProgress {
        TaskbarProgress {}
    }

    pub fn set_progress_state(&self, _tbp_flags: TBPFLAG) {}

    pub fn set_progress_value(&self, _completed: u64, _total: u64) {}

    pub fn hide(&self) {}

    pub fn show(&self) {}

    pub fn release(&mut self) {}
}

impl From<HWND> for TaskbarProgress {
    fn from(_hwnd: HWND) -> Self {
        TaskbarProgress {}
    }
}

impl Drop for TaskbarProgress {
    fn drop(&mut self) {}
}
