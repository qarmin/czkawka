use std::time::Duration;

use crossbeam_channel::Receiver;
use indicatif::{ProgressBar, ProgressStyle};

use czkawka_core::common_dir_traversal::ToolType;
use czkawka_core::progress_data::{CurrentStage, ProgressData};

pub fn connect_progress(progress_receiver: &Receiver<ProgressData>) {
    let mut pb = ProgressBar::new(1);
    let mut latest_id = None;
    while let Ok(progress_data) = progress_receiver.recv() {
        // We only need to recreate progress bar if stage changed
        if latest_id != Some(progress_data.current_stage_idx) {
            pb.finish_and_clear();
            if progress_data.current_stage_idx == 0 {
                pb = get_progress_bar_for_collect_files();
            } else if progress_data.sstage.check_if_loading_saving_cache() {
                pb = get_progress_loading_saving_cache(progress_data.sstage.check_if_loading_cache());
            } else {
                pb = get_progress_known_values(progress_data.entries_to_check, &get_progress_message(&progress_data));
            }
            latest_id = Some(progress_data.current_stage_idx);
        }

        pb.set_position(progress_data.entries_checked as u64);
        if progress_data.sstage == CurrentStage::CollectingFiles && progress_data.tool_type != ToolType::EmptyFolders {
            pb.set_message(format!("Collecting files: {}", progress_data.entries_checked));
        } else if progress_data.sstage == CurrentStage::CollectingFiles {
            pb.set_message(format!("Collecting folders: {}", progress_data.entries_checked));
        }
    }
    pb.finish();
}

pub fn get_progress_message(progress_data: &ProgressData) -> String {
    match progress_data.sstage {
        CurrentStage::SameMusicReadingTags => "Reading tags",
        CurrentStage::SameMusicCalculatingFingerprints => "Calculating fingerprints",
        CurrentStage::SameMusicComparingTags => "Comparing tags",
        CurrentStage::SameMusicComparingFingerprints => "Comparing fingerprints",
        CurrentStage::DuplicatePreHashing => "Calculating prehashes",
        CurrentStage::DuplicateFullHashing => "Calculating hashes",
        CurrentStage::SimilarImagesCalculatingHashes => "Calculating image hashes",
        CurrentStage::SimilarImagesComparingHashes => "Comparing image hashes",
        CurrentStage::SimilarVideosCalculatingHashes => "Reading similar values",
        CurrentStage::BrokenFilesChecking => "Checking broken files",
        CurrentStage::BadExtensionsChecking => "Checking extensions of files",
        _ => unreachable!(),
    }
    .to_string()
}

pub fn get_progress_bar_for_collect_files() -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(120));
    pb.set_style(
        ProgressStyle::with_template("{msg} {spinner:.blue}")
            .expect("Failed to create progress bar style")
            .tick_strings(&["▹▹▹▹▹", "▸▹▹▹▹", "▹▸▹▹▹", "▹▹▸▹▹", "▹▹▹▸▹", "▹▹▹▹▸", "▪▪▪▪▪"]),
    );
    pb
}

pub fn get_progress_known_values(max_value: usize, msg: &str) -> ProgressBar {
    let pb = ProgressBar::new(max_value as u64);
    pb.set_style(
        ProgressStyle::with_template(&format!("{msg} [{{bar}}] {{pos}}/{{len}} "))
            .expect("Failed to create progress bar style")
            .progress_chars("=> "),
    );
    pb
}

pub fn get_progress_loading_saving_cache(loading: bool) -> ProgressBar {
    let msg = if loading { "Loading cache" } else { "Saving cache" };
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(120));
    pb.set_style(
        ProgressStyle::with_template(&format!("{msg} {{spinner:.blue}}"))
            .expect("Failed to create progress bar style")
            .tick_strings(&["▹▹▹▹▹", "▸▹▹▹▹", "▹▸▹▹▹", "▹▹▸▹▹", "▹▹▹▸▹", "▹▹▹▹▸", "▪▪▪▪▪"]),
    );
    pb
}
