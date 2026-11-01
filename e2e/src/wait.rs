use std::thread::sleep;
use std::time::{Duration, Instant};

const POLL_INTERVAL: Duration = Duration::from_millis(200);

/// Polls until `probe` returns a value, failing with a message about `what` after the timeout
pub fn until<T>(timeout: Duration, what: &str, mut probe: impl FnMut() -> Result<Option<T>, String>) -> Result<T, String> {
    let start = Instant::now();
    loop {
        if let Some(value) = probe()? {
            return Ok(value);
        }
        if start.elapsed() > timeout {
            return Err(format!("timed out after {timeout:?} waiting for {what}"));
        }
        sleep(POLL_INTERVAL);
    }
}
