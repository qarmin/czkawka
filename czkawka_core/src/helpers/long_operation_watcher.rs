//! LongOperationWatcher: logs an escalating warning when a registered operation runs long.
//!
//! A single shared background thread (started lazily, once per process) polls a registry of
//! in-flight operations and emits a warning when one crosses each threshold in
//! `WARNING_STEPS_SECS`. This replaces a "one watchdog thread per operation" approach: callers
//! register an operation with `start`/`watch`, and the lone thread - woken every 100 ms - scans
//! the registry and emits warnings so a hang on a single file/command is visible in the logs
//! instead of silently freezing the scan.

use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};
use std::thread;
use std::time::{Duration, Instant};

use log::{error, warn};

/// Seconds after which a still-running operation logs a warning.
pub const WARNING_STEPS_SECS: [u64; 4] = [50, 250, 1250, 6000];

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
    // file path or a process id). Prefer `watch`/`run_with_long_operation_warnings` so `stop`
    // cannot be missed.
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
