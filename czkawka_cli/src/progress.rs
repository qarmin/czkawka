use std::time::Duration;

use crossbeam_channel::Receiver;
use indicatif::{ProgressBar, ProgressStyle};

use czkawka_core::common_dir_traversal::{CheckingMethod, ProgressData, ToolType};

pub fn connect_progress(progress_receiver: &Receiver<ProgressData>) {
    let mut pb = ProgressBar::new(1);
    let mut latest_id = None;
    while let Ok(progress_data) = progress_receiver.recv() {
        if latest_id != Some(progress_data.current_stage) {
            pb.finish_and_clear();
            if progress_data.current_stage == 0 {
                pb = get_progress_bar_for_collect_files();
            } else if check_if_saving_cache(&progress_data) || check_if_loading_cache(&progress_data) {
                pb = get_progress_loading_saving_cache(check_if_loading_cache(&progress_data));
            } else {
                pb = get_progress_known_values(progress_data.entries_to_check, &get_progress_message(&progress_data));
            }
            latest_id = Some(progress_data.current_stage);
        }

        pb.set_position(progress_data.entries_checked as u64);
        if progress_data.current_stage == 0 && progress_data.tool_type != ToolType::EmptyFolders {
            pb.set_message(format!("Collecting files: {}", progress_data.entries_checked));
        } else if progress_data.current_stage == 0 {
            pb.set_message(format!("Collecting folders: {}", progress_data.entries_checked));
        }
    }
    pb.finish();
}

pub fn get_progress_message(progress_data: &ProgressData) -> String {
    match (progress_data.tool_type, progress_data.current_stage, progress_data.checking_method) {
        (ToolType::SameMusic, 2, CheckingMethod::AudioTags) | (ToolType::SameMusic, 5, CheckingMethod::AudioContent) => "Reading tags",
        (ToolType::SameMusic, 2, CheckingMethod::AudioContent) => "Calculating fingerprint",
        (ToolType::SameMusic, 4, CheckingMethod::AudioTags) => "Comparing tags",
        (ToolType::SameMusic, 4, CheckingMethod::AudioContent) => "Comparing fingerprint",

        (ToolType::Duplicate, 2, CheckingMethod::Hash) => "Reading prehashes",
        (ToolType::Duplicate, 5, CheckingMethod::Hash) => "Reading hashes",
        (ToolType::SimilarImages, 1, _) => "Reading images",
        (ToolType::SimilarImages, 2, _) => "Comparing image hashes",
        (ToolType::SimilarVideos, 1, _) => "Reading similar values",
        (ToolType::BrokenFiles, 1, _) => "Checking broken files",
        (ToolType::BadExtensions, 1, _) => "Checking extensions of files",
        _ => unreachable!(),
    }
    .to_string()
}

pub fn check_if_loading_cache(progress_data: &ProgressData) -> bool {
    matches!(
        (progress_data.tool_type, progress_data.current_stage),
        (ToolType::SameMusic, 1) | (ToolType::Duplicate, 1 | 4)
    )
}
pub fn check_if_saving_cache(progress_data: &ProgressData) -> bool {
    matches!(
        (progress_data.tool_type, progress_data.current_stage),
        (ToolType::SameMusic, 3) | (ToolType::Duplicate, 3 | 6)
    )
}

pub fn get_progress_bar_for_collect_files() -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(120));
    pb.set_style(
        ProgressStyle::with_template("{msg} {spinner:.blue}")
            .unwrap()
            .tick_strings(&["▹▹▹▹▹", "▸▹▹▹▹", "▹▸▹▹▹", "▹▹▸▹▹", "▹▹▹▸▹", "▹▹▹▹▸", "▪▪▪▪▪"]),
    );
    pb
}

pub fn get_progress_known_values(max_value: usize, msg: &str) -> ProgressBar {
    let pb = ProgressBar::new(max_value as u64);
    pb.set_style(ProgressStyle::with_template(&format!("{msg} [{{bar}}] {{pos}}/{{len}} ")).unwrap().progress_chars("=> "));
    pb
}

pub fn get_progress_loading_saving_cache(loading: bool) -> ProgressBar {
    let msg = if loading { "Loading cache" } else { "Saving cache" };
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(120));
    pb.set_style(
        ProgressStyle::with_template(&format!("{msg} {{spinner:.blue}}"))
            .unwrap()
            .tick_strings(&["▹▹▹▹▹", "▸▹▹▹▹", "▹▸▹▹▹", "▹▹▸▹▹", "▹▹▹▸▹", "▹▹▹▹▸", "▪▪▪▪▪"]),
    );
    pb
}
