#![cfg(target_os = "windows")]
extern crate winapi;

use std::cell::RefCell;
use std::convert::From;
use std::ptr;

use winapi::ctypes::c_void;
use winapi::shared::windef::HWND;
use winapi::shared::winerror::{E_POINTER, S_OK};
use winapi::shared::wtypesbase::CLSCTX_INPROC_SERVER;
use winapi::um::shobjidl_core::{CLSID_TaskbarList, ITaskbarList3, TBPFLAG};
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
    must_uninit_com: bool,
    is_active: RefCell<bool>,
}

impl TaskbarProgress {
    pub fn new() -> TaskbarProgress {
        let hwnd = unsafe { winuser::GetActiveWindow() };
        TaskbarProgress::from(hwnd)
    }

    pub fn set_progress_state(&self, tbp_flags: TBPFLAG) {
        if tbp_flags == *self.current_state.borrow() || !*self.is_active.borrow() {
            return ();
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
        }
    }

    pub fn set_progress_value(&self, completed: u64, total: u64) {
        // Don't change the value if the is_active flag is false or the value has not changed.
        // If is_active is true and the value has not changed, but the progress indicator was in NOPROGRESS or INDETERMINATE state, set the value (and NORMAL state).
        if ((completed, total) == *self.current_progress.borrow()
            && *self.current_state.borrow() != tbp_flags::TBPF_NOPROGRESS
            && *self.current_state.borrow() != tbp_flags::TBPF_INDETERMINATE)
            || !*self.is_active.borrow()
        {
            return ();
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
            if *self.current_state.borrow() == tbp_flags::TBPF_NOPROGRESS || *self.current_state.borrow() == tbp_flags::TBPF_INDETERMINATE {
                self.current_state.replace(tbp_flags::TBPF_NORMAL);
            }
        }
    }

    pub fn hide(&self) {
        self.set_progress_state(tbp_flags::TBPF_NOPROGRESS);
        *self.is_active.borrow_mut() = false;
    }

    pub fn show(&self) {
        *self.is_active.borrow_mut() = true;
    }

    /// Releases the ITaskbarList3 pointer, uninitialises the COM API and sets the struct to a valid "empty" state.
    /// It's required for proper use of the COM API, because `drop` is never called (objects moved to GTK closures have `static` lifetime).
    pub fn release(&mut self) {
        unsafe {
            if let Some(list) = self.taskbar_list.as_ref() {
                list.Release();
                self.taskbar_list = ptr::null_mut();
                self.hwnd = ptr::null_mut();
            }
            // A thread must call CoUninitialize once for each successful call it has made to
            // the CoInitialize or CoInitializeEx function, including any call that returns S_FALSE.
            if self.must_uninit_com {
                combaseapi::CoUninitialize();
                self.must_uninit_com = false;
            }
        }
    }
}

impl From<HWND> for TaskbarProgress {
    fn from(hwnd: HWND) -> Self {
        if hwnd.is_null() {
            return TaskbarProgress {
                hwnd,
                taskbar_list: ptr::null_mut(),
                current_state: RefCell::new(tbp_flags::TBPF_NOPROGRESS),
                current_progress: RefCell::new((0, 0)),
                must_uninit_com: false,
                is_active: RefCell::new(false),
            };
        }

        let init_result = unsafe { combaseapi::CoInitializeEx(ptr::null_mut(), objbase::COINIT_APARTMENTTHREADED) };
        // S_FALSE means that COM library is already initialised for this thread
        // Success codes are not negative, RPC_E_CHANGED_MODE should not be possible and is treated as an error
        if init_result < 0 {
            return TaskbarProgress {
                hwnd: ptr::null_mut(),
                taskbar_list: ptr::null_mut(),
                current_state: RefCell::new(tbp_flags::TBPF_NOPROGRESS),
                current_progress: RefCell::new((0, 0)),
                must_uninit_com: false,
                is_active: RefCell::new(false),
            };
        }

        let mut taskbar_list: *mut ITaskbarList3 = ptr::null_mut();
        let taskbar_list_ptr: *mut *mut ITaskbarList3 = &mut taskbar_list;

        unsafe {
            combaseapi::CoCreateInstance(
                &CLSID_TaskbarList,
                ptr::null_mut(),
                CLSCTX_INPROC_SERVER,
                &ITaskbarList3::uuidof(),
                taskbar_list_ptr as *mut *mut c_void,
            )
        };

        TaskbarProgress {
            hwnd: if taskbar_list.is_null() { ptr::null_mut() } else { hwnd },
            taskbar_list,
            current_state: RefCell::new(tbp_flags::TBPF_NOPROGRESS), // Assume no progress
            current_progress: RefCell::new((0, 0)),
            must_uninit_com: true,
            is_active: RefCell::new(false),
        }
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
            if self.must_uninit_com {
                combaseapi::CoUninitialize();
            }
        }
    }
}
