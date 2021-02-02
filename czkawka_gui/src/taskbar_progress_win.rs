#![cfg(target_os = "windows")]
extern crate winapi;
use std::{cell::RefCell, convert::TryFrom, ptr};
use winapi::ctypes::c_void;
use winapi::shared::guiddef::GUID;
use winapi::shared::windef::HWND;
use winapi::shared::winerror::{E_POINTER, HRESULT, RPC_E_CHANGED_MODE, S_FALSE, S_OK};
use winapi::shared::wtypesbase::CLSCTX_INPROC_SERVER;
use winapi::um::shobjidl_core::{CLSID_TaskbarList, ITaskbarList3, TBPFLAG};
use winapi::um::unknwnbase::IUnknown;
use winapi::um::{combaseapi, objbase, winuser};

pub mod tbp_flags {
	pub use winapi::um::shobjidl_core::{TBPF_ERROR, TBPF_INDETERMINATE, TBPF_NOPROGRESS, TBPF_NORMAL, TBPF_PAUSED};
}

pub struct TaskbarProgress {
	hwnd: HWND,
	taskbar_list: *mut ITaskbarList3,
	com_was_init: bool,
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

		// IIDs are not listed in winapi, but I found them here:
		// https://referencesource.microsoft.com/#PresentationFramework/src/Framework/System/Windows/Standard/ComGuids.cs,44
		// IID_ITaskbarList3 "ea1afb91-9e28-4b86-90e9-9e9f8a5eefaf"
		#[allow(non_upper_case_globals)]
		const IID_ITaskbarList3: GUID = GUID {
			Data1: 0xea1afb91,
			Data2: 0x9e28,
			Data3: 0x4b86,
			Data4: [0x90, 0xe9, 0x9e, 0x9f, 0x8a, 0x5e, 0xef, 0xaf],
		};

		let init_result = unsafe { combaseapi::CoInitializeEx(ptr::null_mut::<c_void>(), objbase::COINIT_APARTMENTTHREADED) };
		// S_FALSE means that COM library is already initialised for this thread
		// RPC_E_CHANGED_MODE means that COM was initialised in multithreaded mode, this call may have changed it
		if init_result != S_OK && init_result != S_FALSE && init_result != RPC_E_CHANGED_MODE {
			return Err(init_result);
		}

		let com_was_init = init_result != S_OK;
		let mut taskbar_list: *mut ITaskbarList3 = ptr::null_mut();
		let taskbar_list_ptr: *mut *mut ITaskbarList3 = &mut taskbar_list;

		let create_result = unsafe { combaseapi::CoCreateInstance(&CLSID_TaskbarList, ptr::null_mut::<IUnknown>(), CLSCTX_INPROC_SERVER, &IID_ITaskbarList3, taskbar_list_ptr as *mut *mut c_void) };
		if taskbar_list.is_null() {
			return Err(create_result);
		}

		Ok(TaskbarProgress {
			hwnd,
			taskbar_list,
			com_was_init,
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
			if !self.com_was_init {
				combaseapi::CoUninitialize();
			}
		}
	}
}
