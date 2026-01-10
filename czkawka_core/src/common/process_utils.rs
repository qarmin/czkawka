use std::process::{Command, Stdio};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

// Returns None if stopped, Some(Err) if command failed, Some(Ok(output)) if successful.
pub fn run_command_interruptible(mut command: Command, stop_flag: &Arc<AtomicBool>) -> Option<Result<std::process::Output, String>> {
    if stop_flag.load(Ordering::Relaxed) {
        return None;
    }

    command.stdin(Stdio::null()).stdout(Stdio::piped()).stderr(Stdio::piped());

    let mut child = match command.spawn() {
        Ok(c) => c,
        Err(e) => return Some(Err(format!("Failed to spawn command: {e}"))),
    };

    let stop_flag = stop_flag.clone();
    loop {
        if stop_flag.load(Ordering::Relaxed) {
            let _ = child.kill();
            return None;
        }

        match child.try_wait() {
            Ok(Some(_status)) => {
                return match child.wait_with_output() {
                    Ok(output) => Some(Ok(output)),
                    Err(e) => Some(Err(format!("Failed to get output: {e}"))),
                };
            }
            Ok(None) => {
                thread::sleep(Duration::from_millis(100));
            }
            Err(e) => {
                return Some(Err(format!("Failed to check process status: {e}")));
            }
        }
    }
}
