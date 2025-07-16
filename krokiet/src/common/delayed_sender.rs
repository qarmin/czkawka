use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

pub struct DelayedSender<T: Send + 'static> {
    slot: Arc<Mutex<Option<T>>>,
}

impl<T: Send + 'static> DelayedSender<T> {
    pub fn new(sender: crossbeam_channel::Sender<T>, wait_time: Duration) -> Self {
        let slot = Arc::new(Mutex::new(None));
        let slot_clone = Arc::clone(&slot);
        let _join = thread::spawn(move || {
            let mut last_send_time: Option<Instant> = None;
            let duration_between_checks = Duration::from_secs_f64(wait_time.as_secs_f64() / 5.0);

            loop {
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

                if let Err(e) = sender.send(value) {
                    log::error!("Failed to send value: {e:?}");
                };
                last_send_time = Some(Instant::now());
            }
        });

        Self { slot }
    }

    pub fn send(&self, value: T) {
        let mut slot = self.slot.lock().expect("Failed to lock slot in DelayedSender");
        *slot = Some(value);
    }
}
