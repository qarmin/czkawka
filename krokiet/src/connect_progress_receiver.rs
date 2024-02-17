use std::thread;

use crossbeam_channel::Receiver;
use slint::ComponentHandle;

use czkawka_core::common_dir_traversal::{CheckingMethod, ProgressData, ToolType};

use crate::{MainWindow, ProgressToSend};

pub fn connect_progress_gathering(app: &MainWindow, progress_receiver: Receiver<ProgressData>) {
    let a = app.as_weak();

    thread::spawn(move || loop {
        let Ok(progress_data) = progress_receiver.recv() else {
            return; // Channel closed, so exit the thread since app closing
        };

        a.upgrade_in_event_loop(move |app| {
            let to_send = if progress_data.current_stage == 0 {
                progress_collect_items(&progress_data, progress_data.tool_type != ToolType::EmptyFolders)
            } else if check_if_loading_saving_cache(&progress_data) {
                progress_save_load_cache(&progress_data)
            } else {
                progress_default(&progress_data)
            };

            app.set_progress_datas(to_send);
        })
        .unwrap();
    });
}

pub fn check_if_loading_saving_cache(progress_data: &ProgressData) -> bool {
    matches!(
        (progress_data.tool_type, progress_data.current_stage),
        (ToolType::SameMusic, 1 | 3) | (ToolType::Duplicate, 1 | 3 | 4 | 6)
    )
}

fn progress_save_load_cache(item: &ProgressData) -> ProgressToSend {
    let step_name = match (item.tool_type, item.checking_method, item.current_stage) {
        (ToolType::SameMusic, CheckingMethod::AudioTags | CheckingMethod::AudioContent, 1) => "Loading cache",
        (ToolType::SameMusic, CheckingMethod::AudioTags | CheckingMethod::AudioContent, 3) => "Saving cache",
        (ToolType::Duplicate, CheckingMethod::Hash, 1) => "Loading prehash cache",
        (ToolType::Duplicate, CheckingMethod::Hash, 3) => "Saving prehash cache",
        (ToolType::Duplicate, CheckingMethod::Hash, 4) => "Loading hash cache",
        (ToolType::Duplicate, CheckingMethod::Hash, 6) => "Saving hash cache",
        _ => unreachable!(),
    };
    let (all_progress, current_progress) = common_get_data(item);
    ProgressToSend {
        all_progress,
        current_progress,
        step_name: step_name.into(),
    }
}

fn progress_collect_items(item: &ProgressData, files: bool) -> ProgressToSend {
    let step_name = match (item.tool_type, item.checking_method) {
        (ToolType::Duplicate, CheckingMethod::Name) => {
            format!("Scanning name of {} file", item.entries_checked)
        }
        (ToolType::Duplicate, CheckingMethod::SizeName) => {
            format!("Scanning size and name of {} file", item.entries_checked)
        }
        (ToolType::Duplicate, CheckingMethod::Size | CheckingMethod::Hash) => {
            format!("Scanning size of {} file", item.entries_checked)
        }
        _ => {
            if files {
                format!("Scanning {} file", item.entries_checked)
            } else {
                format!("Scanning {} folder", item.entries_checked)
            }
        }
    };
    let (all_progress, current_progress) = no_current_stage_get_data(item);
    ProgressToSend {
        all_progress,
        current_progress,
        step_name: step_name.into(),
    }
}

fn progress_default(item: &ProgressData) -> ProgressToSend {
    let step_name = match (item.tool_type, item.checking_method, item.current_stage) {
        (ToolType::SameMusic, CheckingMethod::AudioTags, 2) | (ToolType::SameMusic, CheckingMethod::AudioContent, 5) => {
            format!("Checking tags of {}/{} audio file", item.entries_checked, item.entries_to_check)
        }
        (ToolType::SameMusic, CheckingMethod::AudioContent, 2) => {
            format!("Checking content of {}/{} audio file", item.entries_checked, item.entries_to_check)
        }
        (ToolType::SameMusic, CheckingMethod::AudioTags, 4) => {
            format!("Scanning tags of {}/{} audio file", item.entries_checked, item.entries_to_check)
        }
        (ToolType::SameMusic, CheckingMethod::AudioContent, 4) => {
            format!("Scanning content of {}/{} audio file", item.entries_checked, item.entries_to_check)
        }
        (ToolType::SimilarImages, _, 1) => {
            format!("Hashing of {}/{} image", item.entries_checked, item.entries_to_check)
        }
        (ToolType::SimilarImages, _, 2) => {
            format!("Comparing {}/{} image hash", item.entries_checked, item.entries_to_check)
        }
        (ToolType::SimilarVideos, _, 1) => {
            format!("Hashing of {}/{} video", item.entries_checked, item.entries_to_check)
        }
        (ToolType::BrokenFiles, _, 1) => {
            format!("Checking {}/{} file", item.entries_checked, item.entries_to_check)
        }
        (ToolType::BadExtensions, _, 1) => {
            format!("Checking {}/{} file", item.entries_checked, item.entries_to_check)
        }
        (ToolType::Duplicate, CheckingMethod::Hash, 2) => {
            format!("Analyzing partial hash of {}/{} files", item.entries_checked, item.entries_to_check)
        }
        (ToolType::Duplicate, CheckingMethod::Hash, 5) => {
            format!("Analyzing full hash of {}/{} files", item.entries_checked, item.entries_to_check)
        }
        _ => unreachable!(),
    };
    let (all_progress, current_progress) = common_get_data(item);
    ProgressToSend {
        all_progress,
        current_progress,
        step_name: step_name.into(),
    }
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
        let all_stages = (item.current_stage as f64 + item.entries_checked as f64 / item.entries_to_check as f64) / (item.max_stage + 1) as f64;
        let all_stages = if all_stages > 0.99 { 0.99 } else { all_stages };

        let current_stage = item.entries_checked as f64 / item.entries_to_check as f64;
        let current_stage = if current_stage > 0.99 { 0.99 } else { current_stage };
        ((all_stages * 100.0) as i32, (current_stage * 100.0) as i32)
    } else {
        let all_stages = (item.current_stage as f64) / (item.max_stage + 1) as f64;
        let all_stages = if all_stages > 0.99 { 0.99 } else { all_stages };
        ((all_stages * 100.0) as i32, 0)
    }
}
