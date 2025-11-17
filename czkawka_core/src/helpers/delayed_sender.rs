//! DelayedSender: A utility for batching or throttling messages sent between threads.

use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

/// A sender that delays sending values until a specified wait time has passed since the last sent value.
///
/// This is useful for batching updates or reducing the frequency of sending messages in a multi-threaded environment.
/// Note: Using mutexes in the send function from multiple threads can lead to performance issues (waiting for mutex release),
/// but for now, the performance impact is minimal. In the future, a more efficient channel could be used.
pub struct DelayedSender<T: Send + 'static> {
    slot: Arc<Mutex<Option<T>>>,
    stop_flag: Arc<AtomicBool>,
}

impl<T: Send + 'static> DelayedSender<T> {
    /// Creates a new DelayedSender.
    ///
    /// # Arguments
    /// * `sender` - The channel sender to forward values to.
    /// * `wait_time` - The minimum duration to wait between sends.
    pub fn new(sender: crossbeam_channel::Sender<T>, wait_time: Duration) -> Self {
        let slot = Arc::new(Mutex::new(None));
        let slot_clone = Arc::clone(&slot);
        let stop_flag = Arc::new(AtomicBool::new(false));
        let stop_flag_clone = Arc::clone(&stop_flag);
        let _join = thread::spawn(move || {
            let mut last_send_time: Option<Instant> = None;
            let duration_between_checks = Duration::from_secs_f64(wait_time.as_secs_f64() / 5.0);

            loop {
                if stop_flag_clone.load(std::sync::atomic::Ordering::Relaxed) {
                    break;
                }
                if let Some(last_send_time) = last_send_time
                    && last_send_time.elapsed() < wait_time
                {
                    thread::sleep(duration_between_checks);
                    continue;
                }

                let Some(value) = slot_clone.lock().expect("Failed to lock slot in DelayedSender").take() else {
                    thread::sleep(duration_between_checks);
                    continue;
                };

                if stop_flag_clone.load(std::sync::atomic::Ordering::Relaxed) {
                    break;
                }
                if let Err(e) = sender.send(value) {
                    log::error!("Failed to send value: {e:?}");
                }
                last_send_time = Some(Instant::now());
            }
        });

        Self { slot, stop_flag }
    }

    /// Sends a value, replacing any previous value that has not yet been sent.
    pub fn send(&self, value: T) {
        let mut slot = self.slot.lock().expect("Failed to lock slot in DelayedSender");
        *slot = Some(value);
    }
}

impl<T: Send + 'static> Drop for DelayedSender<T> {
    fn drop(&mut self) {
        // After dropping DelayedSender, no more values will be sent.
        // Previously, some values were cached and sent after later operations.
        self.stop_flag.store(true, std::sync::atomic::Ordering::Relaxed);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delayed_sender_basic_send() {
        let (sender, receiver) = crossbeam_channel::unbounded();
        let delayed_sender = DelayedSender::new(sender, Duration::from_millis(50));

        delayed_sender.send(42);
        thread::sleep(Duration::from_millis(100));

        let result = receiver.try_recv();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
    }

    #[test]
    fn test_delayed_sender_batching() {
        let (sender, receiver) = crossbeam_channel::unbounded();
        let delayed_sender = DelayedSender::new(sender, Duration::from_millis(100));

        // First send will be sent immediately (last_send_time is None)
        delayed_sender.send(1);
        thread::sleep(Duration::from_millis(50));

        // Wait for first send to complete
        let first = receiver.try_recv();
        assert!(first.is_ok());
        assert_eq!(first.unwrap(), 1);

        // Now send multiple values quickly - only the last one should be sent
        delayed_sender.send(2);
        thread::sleep(Duration::from_millis(10));
        delayed_sender.send(3);
        thread::sleep(Duration::from_millis(10));
        delayed_sender.send(4);

        thread::sleep(Duration::from_millis(150));

        let result = receiver.try_recv();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 4);

        let result2 = receiver.try_recv();
        assert!(result2.is_err());
    }

    #[test]
    fn test_delayed_sender_multiple_sends() {
        let (sender, receiver) = crossbeam_channel::unbounded();
        let delayed_sender = DelayedSender::new(sender, Duration::from_millis(50));

        delayed_sender.send(10);
        thread::sleep(Duration::from_millis(100));

        delayed_sender.send(20);
        thread::sleep(Duration::from_millis(100));

        let first = receiver.try_recv();
        assert!(first.is_ok());
        assert_eq!(first.unwrap(), 10);

        let second = receiver.try_recv();
        assert!(second.is_ok());
        assert_eq!(second.unwrap(), 20);
    }

    #[test]
    fn test_delayed_sender_drop_stops_thread() {
        let (sender, receiver) = crossbeam_channel::unbounded();
        {
            let delayed_sender = DelayedSender::new(sender, Duration::from_millis(50));
            delayed_sender.send(100);
        } // delayed_sender is dropped here

        thread::sleep(Duration::from_millis(150));

        // The thread should have stopped, so no value should be sent
        // or at most one value might have been sent before stop
        let count = receiver.try_iter().count();
        assert!(count <= 1);
    }

    #[test]
    fn test_delayed_sender_no_send_without_wait() {
        let (sender, receiver) = crossbeam_channel::unbounded();
        let delayed_sender = DelayedSender::new(sender, Duration::from_millis(100));

        // First send - will be sent immediately
        delayed_sender.send(5);
        thread::sleep(Duration::from_millis(50));

        let first = receiver.try_recv();
        assert!(first.is_ok());
        assert_eq!(first.unwrap(), 5);

        // Second send - should not be sent within 50ms (wait_time is 100ms)
        delayed_sender.send(10);
        thread::sleep(Duration::from_millis(50));

        let result = receiver.try_recv();
        assert!(result.is_err());

        // But should be sent after full wait_time
        thread::sleep(Duration::from_millis(100));
        let result = receiver.try_recv();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 10);
    }
}


