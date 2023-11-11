use crate::{MainWindow, ProgressToSend};

use crossbeam_channel::Receiver;
use czkawka_core::common_dir_traversal::{ProgressData, ToolType};
use slint::{ComponentHandle, SharedString};
use std::thread;

pub fn connect_progress_gathering(app: &MainWindow, progress_receiver: Receiver<ProgressData>) {
    let a = app.as_weak();

    thread::spawn(move || loop {
        let Ok(progress_data) = progress_receiver.recv() else {
            return; // Channel closed, so exit the thread since app closing
        };

        a.upgrade_in_event_loop(move |app| {
            let to_send;
            match progress_data.tool_type {
                ToolType::EmptyFiles => {
                    let (all_progress, current_progress) = no_current_stage_get_data(&progress_data);
                    to_send = ProgressToSend {
                        all_progress,
                        current_progress,
                        step_name: SharedString::from(format!("Checked {} files", progress_data.entries_checked)),
                    };
                }
                ToolType::EmptyFolders => {
                    let (all_progress, current_progress) = no_current_stage_get_data(&progress_data);
                    to_send = ProgressToSend {
                        all_progress,
                        current_progress,
                        step_name: SharedString::from(format!("Checked {} folders", progress_data.entries_checked)),
                    };
                }
                ToolType::SimilarImages => {
                    let step_name;
                    let all_progress;
                    let current_progress;
                    match progress_data.current_stage {
                        0 => {
                            (all_progress, current_progress) = no_current_stage_get_data(&progress_data);
                            step_name = format!("Scanning {} file", progress_data.entries_checked);
                        }
                        1 => {
                            (all_progress, current_progress) = common_get_data(&progress_data);
                            step_name = format!("Hashing {}/{} image", progress_data.entries_checked, progress_data.entries_to_check);
                        }
                        2 => {
                            (all_progress, current_progress) = common_get_data(&progress_data);
                            step_name = format!("Comparing {}/{} image hash", progress_data.entries_checked, progress_data.entries_to_check);
                        }
                        _ => panic!(),
                    }

                    to_send = ProgressToSend {
                        all_progress,
                        current_progress,
                        step_name: SharedString::from(step_name),
                    };
                }
                _ => {
                    panic!("Invalid tool type {:?}", progress_data.tool_type);
                }
            }
            app.set_progress_datas(to_send);
        })
        .unwrap();
    });
}

// Used when current stage not have enough data to show status, so we show only all_stages
// Happens if we searching files and we don't know how many files we need to check
fn no_current_stage_get_data(item: &ProgressData) -> (i32, i32) {
    let all_stages = (item.current_stage as f64) / (item.max_stage + 1) as f64;

    ((all_stages * 100.0) as i32, -1)
}

// Used to calculate number of files to check and also to calculate current progress according to number of files to check and checked
fn common_get_data(item: &ProgressData) -> (i32, i32) {
    if item.entries_to_check != 0 {
        let all_stages = (item.current_stage as f64 + (item.entries_checked) as f64 / item.entries_to_check as f64) / (item.max_stage + 1) as f64;
        let all_stages = if all_stages > 0.99 { 0.99 } else { all_stages };

        let current_stage = (item.entries_checked) as f64 / item.entries_to_check as f64;
        let current_stage = if current_stage > 0.99 { 0.99 } else { current_stage };
        ((all_stages * 100.0) as i32, (current_stage * 100.0) as i32)
    } else {
        let all_stages = (item.current_stage as f64) / (item.max_stage + 1) as f64;
        let all_stages = if all_stages > 0.99 { 0.99 } else { all_stages };
        ((all_stages * 100.0) as i32, 0)
    }
}
