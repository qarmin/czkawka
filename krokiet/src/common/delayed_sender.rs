use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

/// A sender that delays sending values until a specified wait time has passed since the last sent value.
/// This is useful for batching updates or reducing the frequency of sending messages in a multi-threaded environment.
/// It is not ideal - using mutexes in send function from multiple threads can lead to performance issues(waiting for), but at least for now I don't see too much performance impact.
pub struct DelayedSender<T: Send + 'static> {
    slot: Arc<Mutex<Option<T>>>,
    stop_flag: Arc<AtomicBool>,
}

impl<T: Send + 'static> DelayedSender<T> {
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
                if let Some(last_send_time) = last_send_time {
                    if last_send_time.elapsed() < wait_time {
                        thread::sleep(duration_between_checks);
                        continue;
                    }
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
                };
                last_send_time = Some(Instant::now());
            }
        });

        Self { slot, stop_flag }
    }

    pub fn send(&self, value: T) {
        let mut slot = self.slot.lock().expect("Failed to lock slot in DelayedSender");
        *slot = Some(value);
    }
}

impl<T: Send + 'static> Drop for DelayedSender<T> {
    fn drop(&mut self) {
        // We need to know, that after dropping DelayedSender, no more values will be sent
        // Previously some values were cached and sent after other later operations
        self.stop_flag.store(true, std::sync::atomic::Ordering::Relaxed);
    }
}
