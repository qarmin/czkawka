use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use log::warn;

// Returns None if stopped, Some(Err) if command failed, Some(Ok(output)) if successful.
pub fn run_command_interruptible2(mut command: Command, stop_flag: &Arc<AtomicBool>) -> Option<Result<std::process::Output, String>> {
    if stop_flag.load(Ordering::Relaxed) {
        return None;
    }

    command.stdin(Stdio::null()).stdout(Stdio::piped()).stderr(Stdio::piped());
    // command.stdin(Stdio::null());

    let mut child = match command.spawn() {
        Ok(c) => c,
        Err(e) => return Some(Err(format!("Failed to spawn command: {e}"))),
    };

    let stop_flag = stop_flag.clone();
    let start_time = Instant::now();
    let warning_steps = [50, 250, 1250, 6000];
    let mut next_warning_idx = 0;

    loop {
        if stop_flag.load(Ordering::Relaxed) {
            let _ = child.kill();
            return None;
        }

        let elapsed_secs = start_time.elapsed().as_secs();
        if let Some(warning_time) = warning_steps.get(next_warning_idx)
            && elapsed_secs >= *warning_time
        {
            warn!("Command is still running after {warning_time} seconds, for command: {command:?}");
            next_warning_idx += 1;
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

pub fn run_command_interruptible(mut command: Command, stop_flag: &Arc<AtomicBool>) -> Option<Result<std::process::Output, String>> {
    if stop_flag.load(Ordering::Relaxed) {
        return None;
    }

    command.stdin(Stdio::null()).stdout(Stdio::piped()).stderr(Stdio::piped());

    let mut child = match command.spawn() {
        Ok(c) => c,
        Err(e) => return Some(Err(format!("Failed to spawn command: {e}"))),
    };

    let mut stdout = child.stdout.take().unwrap();
    let mut stderr = child.stderr.take().unwrap();

    let stdout_buf = Arc::new(Mutex::new(Vec::new()));
    let stderr_buf = Arc::new(Mutex::new(Vec::new()));

    let out_buf = stdout_buf.clone();
    let err_buf = stderr_buf.clone();

    let out_handle = thread::spawn(move || {
        let mut buf = Vec::new();
        let _ = std::io::copy(&mut stdout, &mut buf);
        *out_buf.lock().unwrap() = buf;
    });

    let err_handle = thread::spawn(move || {
        let mut buf = Vec::new();
        let _ = std::io::copy(&mut stderr, &mut buf);
        *err_buf.lock().unwrap() = buf;
    });

    let start_time = Instant::now();
    let warning_steps = [50, 250, 1250, 6000];
    let mut next_warning_idx = 0;

    loop {
        if stop_flag.load(Ordering::Relaxed) {
            let _ = child.kill();
            let _ = child.wait();
            break;
        }

        let elapsed_secs = start_time.elapsed().as_secs();
        if let Some(warning_time) = warning_steps.get(next_warning_idx)
            && elapsed_secs >= *warning_time
        {
            warn!("Command is still running after {warning_time} seconds, for command: {command:?}");
            next_warning_idx += 1;
        }

        match child.try_wait() {
            Ok(Some(_)) => break,
            Ok(None) => thread::sleep(Duration::from_millis(100)),
            Err(e) => return Some(Err(format!("Failed to check process status: {e}"))),
        }
    }

    let status = match child.wait() {
        Ok(s) => s,
        Err(e) => return Some(Err(format!("Failed to wait for process: {e}"))),
    };

    let _ = out_handle.join();
    let _ = err_handle.join();

    if stop_flag.load(Ordering::Relaxed) {
        return None;
    }

    let stdout = Arc::try_unwrap(stdout_buf).unwrap().into_inner().unwrap();
    let stderr = Arc::try_unwrap(stderr_buf).unwrap().into_inner().unwrap();

    Some(Ok(std::process::Output { status, stdout, stderr }))
}
