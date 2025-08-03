use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize};
use std::sync::{Arc, atomic};
use std::thread;
use std::thread::{JoinHandle, sleep};
use std::time::{Duration, Instant};

use crossbeam_channel::Sender;
use fun_time::fun_time;

use crate::common::consts::{LOOP_DURATION, SEND_PROGRESS_DATA_TIME_BETWEEN};
use crate::common::progress_data::{CurrentStage, ProgressData};
use crate::common_dir_traversal::{CheckingMethod, ToolType};

pub(crate) fn prepare_thread_handler_common(
    progress_sender: Option<&Sender<ProgressData>>,
    sstage: CurrentStage,
    max_items: usize,
    test_type: (ToolType, CheckingMethod),
    max_size: u64,
) -> (JoinHandle<()>, Arc<AtomicBool>, Arc<AtomicUsize>, AtomicBool, Arc<AtomicU64>) {
    let (tool_type, checking_method) = test_type;
    assert_ne!(tool_type, ToolType::None, "Cannot send progress data for ToolType::None");
    let progress_thread_run = Arc::new(AtomicBool::new(true));
    let items_counter = Arc::new(AtomicUsize::new(0));
    let size_counter = Arc::new(AtomicU64::new(0));
    let check_was_stopped = AtomicBool::new(false);
    let progress_thread_sender = if let Some(progress_sender) = progress_sender {
        let progress_send = progress_sender.clone();
        let progress_thread_run = progress_thread_run.clone();
        let items_counter = items_counter.clone();
        let size_counter = size_counter.clone();
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
                        entries_checked: items_counter.load(atomic::Ordering::Relaxed),
                        entries_to_check: max_items,
                        bytes_checked: size_counter.load(atomic::Ordering::Relaxed),
                        bytes_to_check: max_size,
                        tool_type,
                    };

                    progress_data.validate();

                    progress_send.send(progress_data).expect("Cannot send progress data");
                    time_since_last_send = Instant::now();
                }
                if !progress_thread_run.load(atomic::Ordering::Relaxed) {
                    break;
                }
                sleep(Duration::from_millis(LOOP_DURATION as u64));
            }
        })
    } else {
        thread::spawn(|| {})
    };
    (progress_thread_sender, progress_thread_run, items_counter, check_was_stopped, size_counter)
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
