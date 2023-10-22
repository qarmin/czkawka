use crate::{MainWindow, ProgressToSend};
use crossbeam_channel::Receiver;
use czkawka_core::common_dir_traversal::ProgressData;
use slint::{ComponentHandle, SharedString};
use std::thread;

pub fn connect_progress_gathering(app: &MainWindow, progress_receiver: Receiver<ProgressData>) {
    let a = app.as_weak();

    thread::spawn(move || loop {
        let progress_data = progress_receiver.recv().unwrap();

        a.upgrade_in_event_loop(move |app| {
            let (all_stages, current_stage) = common_get_data(&progress_data);
            let to_send = ProgressToSend {
                all_progress: (all_stages * 100.0) as i32,
                current_progress: (current_stage * 100.0) as i32,
                step_name: SharedString::from(format!("Checked {} folders", progress_data.entries_checked)),
            };

            app.set_progress_datas(to_send);
        })
        .unwrap();
    });
}
fn common_get_data(item: &ProgressData) -> (f64, f64) {
    if item.entries_to_check != 0 {
        let all_stages = (item.current_stage as f64 + (item.entries_checked) as f64 / item.entries_to_check as f64) / (item.max_stage + 1) as f64;
        let all_stages = if all_stages > 0.99 { 0.99 } else { all_stages };

        let current_stage = (item.entries_checked) as f64 / item.entries_to_check as f64;
        let current_stage = if current_stage > 0.99 { 0.99 } else { current_stage };
        (all_stages, current_stage)
    } else {
        let all_stages = (item.current_stage as f64) / (item.max_stage + 1) as f64;
        let all_stages = if all_stages > 0.99 { 0.99 } else { all_stages };
        (all_stages, 0f64)
    }
}
