#[cfg(not(target_os = "windows"))]
pub use crate::taskbar_progress_dummy::{TaskbarProgress, tbp_flags};
#[cfg(target_os = "windows")]
pub use crate::taskbar_progress_win::{TaskbarProgress, tbp_flags};
