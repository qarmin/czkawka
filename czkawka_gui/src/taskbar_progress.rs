#[cfg(target_os = "windows")]
pub use crate::taskbar_progress_win::{tbp_flags, TaskbarProgress};

#[cfg(not(target_os = "windows"))]
pub use crate::taskbar_progress_dummy::{tbp_flags, TaskbarProgress};
