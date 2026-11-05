//! Cooperative cancellation that also reaches child processes.
//!
//! One repair operation runs at a time per process, so the flag is process-wide: threading it
//! through every ffmpeg call site would touch dozens of signatures for no extra isolation.

use std::io::{self, Read};
use std::process::{Child, Command, ExitStatus, Output, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

static CURRENT: RwLock<Option<Arc<AtomicBool>>> = RwLock::new(None);

const POLL_INTERVAL: Duration = Duration::from_millis(25);

/// Installs `flag` as the cancellation flag until dropped.
pub struct CancelScope;

impl CancelScope {
    pub fn install(flag: Arc<AtomicBool>) -> Self {
        if let Ok(mut current) = CURRENT.write() {
            *current = Some(flag);
        }
        Self
    }
}

impl Drop for CancelScope {
    fn drop(&mut self) {
        if let Ok(mut current) = CURRENT.write() {
            *current = None;
        }
    }
}

pub fn is_cancelled() -> bool {
    CURRENT.read().is_ok_and(|current| current.as_ref().is_some_and(|flag| flag.load(Ordering::Relaxed)))
}

/// Like [`Command::output`], but kills the child as soon as the operation is cancelled.
pub fn output_cancellable(command: &mut Command) -> io::Result<Output> {
    hide_console_window(command);
    let mut child = command.stdin(Stdio::null()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn()?;
    let stdout = drain(child.stdout.take());
    let stderr = drain(child.stderr.take());

    let status = wait_or_kill(&mut child, is_cancelled)?;
    Ok(Output {
        status,
        stdout: stdout.join().unwrap_or_default(),
        stderr: stderr.join().unwrap_or_default(),
    })
}

#[cfg(windows)]
fn hide_console_window(command: &mut Command) {
    use std::os::windows::process::CommandExt;
    const CREATE_NO_WINDOW: u32 = 0x0800_0000;
    command.creation_flags(CREATE_NO_WINDOW);
}

#[cfg(not(windows))]
fn hide_console_window(_command: &mut Command) {}

fn drain<R: Read + Send + 'static>(pipe: Option<R>) -> thread::JoinHandle<Vec<u8>> {
    thread::spawn(move || {
        let mut buffer = Vec::new();
        if let Some(mut pipe) = pipe {
            let _ = pipe.read_to_end(&mut buffer);
        }
        buffer
    })
}

fn wait_or_kill(child: &mut Child, cancelled: impl Fn() -> bool) -> io::Result<ExitStatus> {
    loop {
        if let Some(status) = child.try_wait()? {
            return Ok(status);
        }
        if cancelled() {
            let _ = child.kill();
            return child.wait();
        }
        thread::sleep(POLL_INTERVAL);
    }
}

#[cfg(all(test, unix))]
mod tests {
    use std::time::Instant;

    use super::*;

    #[test]
    fn cancelling_kills_a_long_running_child_promptly() {
        let flag = Arc::new(AtomicBool::new(false));
        let canceller = {
            let flag = Arc::clone(&flag);
            thread::spawn(move || {
                thread::sleep(Duration::from_millis(100));
                flag.store(true, Ordering::Relaxed);
            })
        };

        let started = Instant::now();
        let mut child = Command::new("sleep").arg("30").spawn().expect("sleep is available");
        let status = wait_or_kill(&mut child, || flag.load(Ordering::Relaxed)).expect("child can be waited on");
        canceller.join().expect("canceller thread");

        assert!(!status.success());
        assert!(started.elapsed() < Duration::from_secs(5));
    }
}
