#![cfg(not(target_os = "windows"))]

type HRESULT = i32;
enum HWND__ {}
type HWND = *mut HWND__;

enum TBPFLAG {
	TBPF_NOPROGRESS = 0,
	TBPF_INDETERMINATE = 0x1,
	TBPF_NORMAL = 0x2,
	TBPF_ERROR = 0x4,
	TBPF_PAUSED = 0x8,
}

pub mod tbp_flags {
	use TBPFLAG::*;
}

pub struct TaskbarProgress {}

impl TaskbarProgress {
	pub fn new() -> Result<TaskbarProgress, HRESULT> {
		TaskbarProgress {}
	}

	pub fn set_progress_state(&self, tbp_flags: TBPFLAG) -> Result<(), HRESULT> {}

	pub fn set_progress_value(&self, completed: u64, total: u64) -> Result<(), HRESULT> {}
}

impl TryFrom<HWND> for TaskbarProgress {
	type Error = HRESULT;

	fn try_from(hwnd: HWND) -> Result<Self, Self::Error> {
		Ok(TaskbarProgress {})
	}
}

impl Drop for TaskbarProgress {
	fn drop(&mut self) {}
}
