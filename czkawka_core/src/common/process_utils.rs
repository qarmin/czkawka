use std::collections::HashMap;
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, OnceLock};
use std::thread;
use std::time::{Duration, Instant};

use log::{error, warn};

use crate::flc;

#[cfg_attr(not(target_os = "windows"), expect(clippy::needless_pass_by_ref_mut))]
pub fn disable_windows_console_window(command: &mut Command) {
    #[cfg(target_os = "windows")]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x08000000;
        command.creation_flags(CREATE_NO_WINDOW);
    }
    #[cfg(not(target_os = "windows"))]
    {
        let _ = command;
    }
}

pub struct CommandOutput {
    pub status: std::process::ExitStatus,
    pub stdout: String,
    pub stderr: String,
}

// Seconds after which a still-running operation logs a warning - shared by the subprocess
// watchdog (`run_command_interruptible`) and the in-process watchdog below.
const WARNING_STEPS_SECS: [u64; 4] = [50, 250, 1250, 6000];

// A single shared watcher thread that logs a warning when any registered in-process
// operation runs past the WARNING_STEPS_SECS thresholds. Replaces the old "one watchdog
// thread per file" approach: callers register an operation with `start`/`watch` and the
// lone background thread, woken every 100 ms, scans the registry, removes nothing on its
// own (entries leave via `stop`/the RAII guard) and emits escalating warnings so a hang on
// a single file is visible in the logs instead of silently freezing the scan.
struct OperationRecord {
    operation: String,
    start: Instant,
    next_warning_idx: usize,
}

static WATCHER_OPERATIONS: OnceLock<Mutex<HashMap<String, OperationRecord>>> = OnceLock::new();
static WATCHER_THREAD_STARTED: OnceLock<()> = OnceLock::new();

fn watcher_operations() -> &'static Mutex<HashMap<String, OperationRecord>> {
    WATCHER_OPERATIONS.get_or_init(|| Mutex::new(HashMap::new()))
}

fn ensure_watcher_thread() {
    if WATCHER_THREAD_STARTED.set(()).is_err() {
        return; // Already running - one thread for the whole process lifetime.
    }
    let spawn_result = thread::Builder::new().name("long-op-watcher".to_string()).spawn(|| {
        loop {
            thread::sleep(Duration::from_millis(100));
            let mut guard = watcher_operations().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            for (key, record) in guard.iter_mut() {
                let elapsed_secs = record.start.elapsed().as_secs();
                while let Some(warning_time) = WARNING_STEPS_SECS.get(record.next_warning_idx).copied() {
                    if elapsed_secs < warning_time {
                        break;
                    }
                    warn!("Operation \"{}\" is still running after {warning_time} seconds, for: {key}", record.operation);
                    record.next_warning_idx += 1;
                }
            }
        }
    });
    if let Err(e) = spawn_result {
        error!("Failed to spawn long-operation watcher thread: {e}");
    }
}

pub struct LongOperationWatcher;

impl LongOperationWatcher {
    // Register a running operation under `key` (use a stable, unique-at-a-time key such as the
    // file path). Prefer `watch`/`run_with_long_operation_warnings` so `stop` cannot be missed.
    pub fn start(operation: &str, key: &str) {
        ensure_watcher_thread();
        watcher_operations().lock().unwrap_or_else(std::sync::PoisonError::into_inner).insert(
            key.to_string(),
            OperationRecord {
                operation: operation.to_string(),
                start: Instant::now(),
                next_warning_idx: 0,
            },
        );
    }

    pub fn stop(key: &str) {
        if let Some(operations) = WATCHER_OPERATIONS.get() {
            operations.lock().unwrap_or_else(std::sync::PoisonError::into_inner).remove(key);
        }
    }

    // RAII variant: `stop` runs on drop, so it fires on every exit path (early return, `?`, panic unwind).
    #[must_use]
    pub fn watch(operation: &str, key: &str) -> OperationWatch {
        Self::start(operation, key);
        OperationWatch { key: key.to_string() }
    }
}

pub struct OperationWatch {
    key: String,
}

impl Drop for OperationWatch {
    fn drop(&mut self) {
        LongOperationWatcher::stop(&self.key);
    }
}

// Convenience wrapper around the shared watcher: registers `key`, runs `f`, then deregisters
// (even on panic). For blocking work that cannot be interrupted (e.g. in-process video hashing
// or a blocking ffmpeg read).
pub fn run_with_long_operation_warnings<T, F: FnOnce() -> T>(operation: &str, key: &str, f: F) -> T {
    let _watch = LongOperationWatcher::watch(operation, key);
    f()
}

// Remember - Ok returned by this function does not necessarily mean that the command executed successfully
// it only means that the command was executed and its output was captured.
// The actual success of the command should be determined by checking the `status` field of the returned `CommandOutput`.
pub fn run_command_interruptible(mut command: Command, stop_flag: &Arc<AtomicBool>) -> Option<Result<CommandOutput, String>> {
    if stop_flag.load(Ordering::Relaxed) {
        return None;
    }

    disable_windows_console_window(&mut command);

    command.stdin(Stdio::null()).stdout(Stdio::piped()).stderr(Stdio::piped());

    let mut child = match command.spawn() {
        Ok(c) => c,
        Err(e) => return Some(Err(flc!("core_failed_to_spawn_command", reason = e.to_string()))),
    };

    let Some(mut stdout) = child.stdout.take() else {
        error!("Failed to take stdout from child process");
        return Some(Err("Failed to take stdout from child process".to_string()));
    };
    let Some(mut stderr) = child.stderr.take() else {
        error!("Failed to take stderr from child process");
        return Some(Err("Failed to take stderr from child process".to_string()));
    };

    let stdout_buf = Arc::new(Mutex::new(Vec::new()));
    let stderr_buf = Arc::new(Mutex::new(Vec::new()));

    let out_buf = stdout_buf.clone();
    let err_buf = stderr_buf.clone();

    let out_handle = thread::spawn(move || {
        let mut buf = Vec::new();
        let _ = std::io::copy(&mut stdout, &mut buf);
        match out_buf.lock() {
            Ok(mut lock) => *lock = buf,
            Err(e) => error!("Failed to lock stdout buffer: {e}"),
        }
    });

    let err_handle = thread::spawn(move || {
        let mut buf = Vec::new();
        let _ = std::io::copy(&mut stderr, &mut buf);
        match err_buf.lock() {
            Ok(mut lock) => *lock = buf,
            Err(e) => error!("Failed to lock stderr buffer: {e}"),
        }
    });

    let start_time = Instant::now();
    let warning_steps = WARNING_STEPS_SECS;
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
            Err(e) => return Some(Err(flc!("core_failed_to_check_process_status", reason = e.to_string()))),
        }
    }

    let status = match child.wait() {
        Ok(s) => s,
        Err(e) => return Some(Err(flc!("core_failed_to_wait_for_process", reason = e.to_string()))),
    };

    let _ = out_handle.join();
    let _ = err_handle.join();

    if stop_flag.load(Ordering::Relaxed) {
        return None;
    }

    let stdout = match Arc::try_unwrap(stdout_buf) {
        Ok(mutex) => match mutex.into_inner() {
            Ok(buf) => buf,
            Err(e) => {
                error!("Failed to get stdout inner buffer: {e}");
                return Some(Err("Failed to get stdout inner buffer".to_string()));
            }
        },
        Err(_) => {
            error!("Failed to unwrap stdout Arc - multiple references still exist");
            return Some(Err("Failed to unwrap stdout Arc".to_string()));
        }
    };

    let stderr = match Arc::try_unwrap(stderr_buf) {
        Ok(mutex) => match mutex.into_inner() {
            Ok(buf) => buf,
            Err(e) => {
                error!("Failed to get stderr inner buffer: {e}");
                return Some(Err("Failed to get stderr inner buffer".to_string()));
            }
        },
        Err(_) => {
            error!("Failed to unwrap stderr Arc - multiple references still exist");
            return Some(Err("Failed to unwrap stderr Arc".to_string()));
        }
    };

    Some(Ok(CommandOutput {
        status,
        stdout: String::from_utf8_lossy(&stdout).to_string(),
        stderr: String::from_utf8_lossy(&stderr).to_string(),
    }))
}
