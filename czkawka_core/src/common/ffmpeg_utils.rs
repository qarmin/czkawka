use std::process::{Command, Stdio};

use crate::common::process_utils::disable_windows_console_window;

pub fn check_if_ffprobe_ffmpeg_exists() -> bool {
    let mut ffmpeg_command = Command::new("ffmpeg");
    disable_windows_console_window(&mut ffmpeg_command);
    let ffmpeg_ok = ffmpeg_command
        .arg("-version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false);

    let mut ffprobe_command = Command::new("ffprobe");
    disable_windows_console_window(&mut ffprobe_command);
    let ffprobe_ok = ffprobe_command
        .arg("-version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false);

    ffprobe_ok && ffmpeg_ok
}
