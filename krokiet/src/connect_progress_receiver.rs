use std::thread;

use crossbeam_channel::Receiver;
use czkawka_core::common_dir_traversal::ToolType;
use czkawka_core::progress_data::{CurrentStage, ProgressData};
use slint::ComponentHandle;

use crate::{MainWindow, ProgressToSend};

pub fn connect_progress_gathering(app: &MainWindow, progress_receiver: Receiver<ProgressData>) {
    let a = app.as_weak();

    thread::spawn(move || loop {
        let Ok(progress_data) = progress_receiver.recv() else {
            return; // Channel closed, so exit the thread since app closing
        };

        a.upgrade_in_event_loop(move |app| {
            let to_send = if progress_data.current_stage_idx == 0 {
                progress_collect_items(&progress_data, progress_data.tool_type != ToolType::EmptyFolders)
            } else if progress_data.sstage.check_if_loading_saving_cache() {
                progress_save_load_cache(&progress_data)
            } else {
                progress_default(&progress_data)
            };

            app.set_progress_datas(to_send);
        })
        .expect("Failed to spawn thread for progress gathering");
    });
}

fn progress_save_load_cache(item: &ProgressData) -> ProgressToSend {
    let step_name = match item.sstage {
        CurrentStage::SameMusicCacheLoadingTags => "Loading tags cache",
        CurrentStage::SameMusicCacheLoadingFingerprints => "Loading fingerprints cache",
        CurrentStage::SameMusicCacheSavingTags => "Saving tags cache",
        CurrentStage::SameMusicCacheSavingFingerprints => "Saving fingerprints cache",
        CurrentStage::DuplicatePreHashCacheLoading => "Loading prehash cache",
        CurrentStage::DuplicatePreHashCacheSaving => "Saving prehash cache",
        CurrentStage::DuplicateCacheLoading => "Loading hash cache",
        CurrentStage::DuplicateCacheSaving => "Saving hash cache",
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
    let step_name = match item.sstage {
        CurrentStage::DuplicateScanningName => {
            format!("Scanning name of {} file", item.entries_checked)
        }
        CurrentStage::DuplicateScanningSizeName => {
            format!("Scanning size and name of {} file", item.entries_checked)
        }
        CurrentStage::DuplicateScanningSize => {
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
    let step_name = match item.sstage {
        CurrentStage::SameMusicReadingTags => {
            format!("Checking tags of {}/{} audio file", item.entries_checked, item.entries_to_check)
        }
        CurrentStage::SameMusicCalculatingFingerprints => {
            format!("Checking content of {}/{} audio file", item.entries_checked, item.entries_to_check)
        }
        CurrentStage::SameMusicComparingTags => {
            format!("Comparing tags of {}/{} audio file", item.entries_checked, item.entries_to_check)
        }
        CurrentStage::SameMusicComparingFingerprints => {
            format!("Comparing content of {}/{} audio file", item.entries_checked, item.entries_to_check)
        }
        CurrentStage::SimilarImagesCalculatingHashes => {
            format!("Hashing of {}/{} image", item.entries_checked, item.entries_to_check)
        }
        CurrentStage::SimilarImagesComparingHashes => {
            format!("Comparing {}/{} image hash", item.entries_checked, item.entries_to_check)
        }
        CurrentStage::SimilarVideosCalculatingHashes => {
            format!("Hashing of {}/{} video", item.entries_checked, item.entries_to_check)
        }
        CurrentStage::BrokenFilesChecking => {
            format!("Checking {}/{} file", item.entries_checked, item.entries_to_check)
        }
        CurrentStage::BadExtensionsChecking => {
            format!("Checking {}/{} file", item.entries_checked, item.entries_to_check)
        }
        CurrentStage::DuplicatePreHashing => {
            format!("Analyzing partial hash of {}/{} files", item.entries_checked, item.entries_to_check)
        }
        CurrentStage::DuplicateFullHashing => {
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
// Happens if we are searching files and we don't know how many files we need to check
fn no_current_stage_get_data(item: &ProgressData) -> (i32, i32) {
    let all_stages = (item.current_stage_idx as f64) / (item.max_stage_idx + 1) as f64;

    ((all_stages * 100.0) as i32, -1)
}

// Used to calculate number of files to check and also to calculate current progress according to number of files to check and checked
fn common_get_data(item: &ProgressData) -> (i32, i32) {
    if item.entries_to_check != 0 {
        let all_stages = (item.current_stage_idx as f64 + item.entries_checked as f64 / item.entries_to_check as f64) / (item.max_stage_idx + 1) as f64;
        let all_stages = all_stages.min(0.99);

        let current_stage = item.entries_checked as f64 / item.entries_to_check as f64;
        let current_stage = current_stage.min(0.99);
        ((all_stages * 100.0) as i32, (current_stage * 100.0) as i32)
    } else {
        let all_stages = (item.current_stage_idx as f64) / (item.max_stage_idx + 1) as f64;
        let all_stages = all_stages.min(0.99);
        ((all_stages * 100.0) as i32, 0)
    }
}
