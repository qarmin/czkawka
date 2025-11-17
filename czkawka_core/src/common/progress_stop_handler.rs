use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize};
use std::sync::{Arc, atomic};
use std::thread;
use std::thread::{JoinHandle, sleep};
use std::time::{Duration, Instant};

use crossbeam_channel::Sender;
use fun_time::fun_time;

use crate::common::model::{CheckingMethod, ToolType};
use crate::common::progress_data::{CurrentStage, ProgressData};
pub const LOOP_DURATION: u32 = 20;
pub const SEND_PROGRESS_DATA_TIME_BETWEEN: u32 = 200;

pub(crate) struct ProgressThreadHandler {
    progress_thread_handle: JoinHandle<()>,
    progress_thread_running: Arc<AtomicBool>,
    progress_status: ProgressStatus,
}
impl ProgressThreadHandler {
    pub fn new(progress_thread_handle: JoinHandle<()>, progress_thread_running: Arc<AtomicBool>, progress_status: ProgressStatus) -> Self {
        Self {
            progress_thread_handle,
            progress_thread_running,
            progress_status,
        }
    }
    pub fn join_thread(self) {
        self.progress_thread_running.store(false, atomic::Ordering::Relaxed);
        self.progress_thread_handle
            .join()
            .expect("Cannot join progress thread - quite fatal error, but I hope, that it will never happen :)");
    }
    pub fn increase_items(&self, count: usize) {
        self.progress_status.items_counter.fetch_add(count, atomic::Ordering::Relaxed);
    }
    pub fn increase_size(&self, size: u64) {
        self.progress_status.size_counter.fetch_add(size, atomic::Ordering::Relaxed);
    }
    pub fn items_counter(&self) -> &Arc<AtomicUsize> {
        &self.progress_status.items_counter
    }
    pub fn size_counter(&self) -> &Arc<AtomicU64> {
        &self.progress_status.size_counter
    }
}

#[derive(Clone)]
pub(crate) struct ProgressStatus {
    items_counter: Arc<AtomicUsize>,
    size_counter: Arc<AtomicU64>,
}
impl ProgressStatus {
    pub fn new() -> Self {
        Self {
            items_counter: Arc::new(AtomicUsize::new(0)),
            size_counter: Arc::new(AtomicU64::new(0)),
        }
    }
}

pub(crate) fn prepare_thread_handler_common(
    progress_sender: Option<&Sender<ProgressData>>,
    sstage: CurrentStage,
    max_items: usize,
    test_type: (ToolType, CheckingMethod),
    max_size: u64,
) -> ProgressThreadHandler {
    let (tool_type, checking_method) = test_type;
    assert_ne!(tool_type, ToolType::None, "Cannot send progress data for ToolType::None");
    let progress_status = ProgressStatus::new();
    let progress_thread_running = Arc::new(AtomicBool::new(true));

    let progress_thread_sender = if let Some(progress_sender) = progress_sender.cloned() {
        let progress_status = progress_status.clone();
        let progress_thread_running = progress_thread_running.clone();
        thread::spawn(move || {
            // Use earlier time, to send immediately first message
            let mut time_since_last_send = Instant::now().checked_sub(Duration::from_secs(10u64)).unwrap_or_else(Instant::now);

            loop {
                if time_since_last_send.elapsed().as_millis() > SEND_PROGRESS_DATA_TIME_BETWEEN as u128 {
                    let progress_data = ProgressData {
                        sstage,
                        checking_method,
                        current_stage_idx: sstage.get_current_stage(),
                        max_stage_idx: tool_type.get_max_stage(checking_method),
                        entries_checked: progress_status.items_counter.load(atomic::Ordering::Relaxed),
                        entries_to_check: max_items,
                        bytes_checked: progress_status.size_counter.load(atomic::Ordering::Relaxed),
                        bytes_to_check: max_size,
                        tool_type,
                    };

                    progress_data.validate();

                    progress_sender.send(progress_data).expect("Cannot send progress data");
                    time_since_last_send = Instant::now();
                }
                if !progress_thread_running.load(atomic::Ordering::Relaxed) {
                    break;
                }
                sleep(Duration::from_millis(LOOP_DURATION as u64));
            }
        })
    } else {
        thread::spawn(|| {})
    };
    ProgressThreadHandler::new(progress_thread_sender, progress_thread_running, progress_status)
}

#[inline]
pub(crate) fn check_if_stop_received(stop_flag: &Arc<AtomicBool>) -> bool {
    stop_flag.load(atomic::Ordering::Relaxed)
}

#[fun_time(message = "send_info_and_wait_for_ending_all_threads", level = "debug")]
pub(crate) fn send_info_and_wait_for_ending_all_threads(progress_thread_run: &Arc<AtomicBool>, progress_thread_handle: JoinHandle<()>) {
    progress_thread_run.store(false, atomic::Ordering::Relaxed);
    progress_thread_handle.join().expect("Cannot join progress thread - quite fatal error, but happens rarely");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_progress_status_and_stop_flag() {
        let status = ProgressStatus::new();
        assert_eq!(status.items_counter.load(atomic::Ordering::Relaxed), 0);
        assert_eq!(status.size_counter.load(atomic::Ordering::Relaxed), 0);

        status.items_counter.fetch_add(10, atomic::Ordering::Relaxed);
        status.size_counter.fetch_add(1024, atomic::Ordering::Relaxed);

        assert_eq!(status.items_counter.load(atomic::Ordering::Relaxed), 10);
        assert_eq!(status.size_counter.load(atomic::Ordering::Relaxed), 1024);

        let stop_flag = Arc::new(AtomicBool::new(false));
        assert!(!check_if_stop_received(&stop_flag));
        stop_flag.store(true, atomic::Ordering::Relaxed);
        assert!(check_if_stop_received(&stop_flag));
    }

    #[test]
    fn test_progress_thread_handler_with_sender() {
        let (sender, _receiver) = crossbeam_channel::unbounded();
        let handler = prepare_thread_handler_common(
            Some(&sender),
            CurrentStage::DuplicateFullHashing,
            100,
            (ToolType::Duplicate, CheckingMethod::Hash),
            10000,
        );

        assert_eq!(handler.items_counter().load(atomic::Ordering::Relaxed), 0);
        assert_eq!(handler.size_counter().load(atomic::Ordering::Relaxed), 0);

        handler.increase_items(5);
        handler.increase_size(512);
        handler.increase_items(3);
        handler.increase_size(256);

        assert_eq!(handler.items_counter().load(atomic::Ordering::Relaxed), 8);
        assert_eq!(handler.size_counter().load(atomic::Ordering::Relaxed), 768);

        handler.join_thread();
    }

    #[test]
    fn test_progress_thread_handler_without_sender() {
        let handler = prepare_thread_handler_common(None, CurrentStage::CollectingFiles, 50, (ToolType::EmptyFiles, CheckingMethod::None), 5000);

        handler.increase_items(10);
        handler.increase_size(1000);

        assert_eq!(handler.items_counter().load(atomic::Ordering::Relaxed), 10);
        assert_eq!(handler.size_counter().load(atomic::Ordering::Relaxed), 1000);

        handler.join_thread();
    }

    #[test]
    #[should_panic(expected = "Cannot send progress data for ToolType::None")]
    fn test_panics_on_none_tool_type() {
        prepare_thread_handler_common(None, CurrentStage::CollectingFiles, 50, (ToolType::None, CheckingMethod::None), 5000);
    }
}

