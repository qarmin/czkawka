#![cfg(target_os = "windows")]
extern crate winapi;
use std::cell::RefCell;
use std::convert::TryFrom;
use std::ptr;
use winapi::ctypes::c_void;
use winapi::shared::windef::HWND;
use winapi::shared::winerror::{E_POINTER, HRESULT, S_OK};
use winapi::shared::wtypesbase::CLSCTX_INPROC_SERVER;
use winapi::um::shobjidl_core::{CLSID_TaskbarList, ITaskbarList3, TBPFLAG};
use winapi::um::unknwnbase::IUnknown;
use winapi::um::{combaseapi, objbase, winuser};
use winapi::Interface;

pub mod tbp_flags {
	pub use winapi::um::shobjidl_core::{TBPF_ERROR, TBPF_INDETERMINATE, TBPF_NOPROGRESS, TBPF_NORMAL, TBPF_PAUSED};
}

pub struct TaskbarProgress {
	hwnd: HWND,
	taskbar_list: *mut ITaskbarList3,
	current_state: RefCell<TBPFLAG>,
	current_progress: RefCell<(u64, u64)>,
}

impl TaskbarProgress {
	pub fn new() -> Result<TaskbarProgress, HRESULT> {
		let hwnd = unsafe { winuser::GetActiveWindow() };
		TaskbarProgress::try_from(hwnd)
	}

	pub fn set_progress_state(&self, tbp_flags: TBPFLAG) -> Result<(), HRESULT> {
		if tbp_flags == *self.current_state.borrow() {
			return Ok(());
		}
		let result = unsafe {
			if let Some(list) = self.taskbar_list.as_ref() {
				list.SetProgressState(self.hwnd, tbp_flags)
			} else {
				E_POINTER
			}
		};
		if result == S_OK {
			self.current_state.replace(tbp_flags);
			Ok(())
		} else {
			Err(result)
		}
	}

	pub fn set_progress_value(&self, completed: u64, total: u64) -> Result<(), HRESULT> {
		if (completed, total) == *self.current_progress.borrow() {
			return Ok(());
		}
		let result = unsafe {
			if let Some(list) = self.taskbar_list.as_ref() {
				list.SetProgressValue(self.hwnd, completed, total)
			} else {
				E_POINTER
			}
		};
		if result == S_OK {
			self.current_progress.replace((completed, total));
			Ok(())
		} else {
			Err(result)
		}
	}
}

impl TryFrom<HWND> for TaskbarProgress {
	type Error = HRESULT;

	fn try_from(hwnd: HWND) -> Result<Self, Self::Error> {
		if hwnd.is_null() {
			return Err(E_POINTER);
		}

		let init_result = unsafe { combaseapi::CoInitializeEx(ptr::null_mut::<c_void>(), objbase::COINIT_APARTMENTTHREADED) };
		// S_FALSE means that COM library is already initialised for this thread
		// Success codes are not negative, RPC_E_CHANGED_MODE should not be possible and is treated as an error
		if init_result < 0 {
			return Err(init_result);
		}

		let mut taskbar_list: *mut ITaskbarList3 = ptr::null_mut();
		let taskbar_list_ptr: *mut *mut ITaskbarList3 = &mut taskbar_list;

		let create_result = unsafe { combaseapi::CoCreateInstance(&CLSID_TaskbarList, ptr::null_mut::<IUnknown>(), CLSCTX_INPROC_SERVER, &ITaskbarList3::uuidof(), taskbar_list_ptr as *mut *mut c_void) };
		if taskbar_list.is_null() {
			return Err(create_result);
		}

		Ok(TaskbarProgress {
			hwnd,
			taskbar_list,
			current_state: RefCell::new(tbp_flags::TBPF_NOPROGRESS), // Assume no progress
			current_progress: RefCell::new((0, 0)),
		})
	}
}

impl Drop for TaskbarProgress {
	fn drop(&mut self) {
		unsafe {
			if let Some(list) = self.taskbar_list.as_ref() {
				list.Release();
			}
			// A thread must call CoUninitialize once for each successful call it has made to
			// the CoInitialize or CoInitializeEx function, including any call that returns S_FALSE.
			combaseapi::CoUninitialize();
		}
	}
}
