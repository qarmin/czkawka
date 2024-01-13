use std::time::Duration;

use crossbeam_channel::Receiver;
use indicatif::{ProgressBar, ProgressStyle};

use czkawka_core::common_dir_traversal::{ProgressData, ToolType};

pub fn connect_progress(progress_receiver: Receiver<ProgressData>) {
    let mut pb = ProgressBar::new(1);
    let mut latest_id = None;
    while let Ok(progress_data) = progress_receiver.recv() {
        if latest_id != Some(progress_data.current_stage) {
            pb.finish_and_clear();
            if progress_data.current_stage == 0 {
                pb = get_progress_bar_for_collect_files();
            } else {
                pb = ProgressBar::new_spinner();
            }
            latest_id = Some(progress_data.current_stage);
        }

        if progress_data.current_stage == 0 && progress_data.tool_type != ToolType::EmptyFolders {
            pb.set_message(format!("Collecting files: {}", progress_data.entries_checked));
        } else if progress_data.current_stage == 0 {
            pb.set_message(format!("Collecting folders: {}", progress_data.entries_checked));
        } else {
            pb.set_message(format!("Loading cache: {}", progress_data.entries_checked));
        }
        // println!("{:?}", progress_data);
    }
    pb.finish();
    println!("AAA");
}

pub fn check_if_loading_saving_cache(progress_data: &ProgressData) -> bool {}

pub fn get_progress_bar_for_collect_files() -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(120));
    pb.set_style(
        ProgressStyle::with_template("{spinner:.blue} {msg}")
            .unwrap()
            .tick_strings(&["▹▹▹▹▹", "▸▹▹▹▹", "▹▸▹▹▹", "▹▹▸▹▹", "▹▹▹▸▹", "▹▹▹▹▸", "▪▪▪▪▪"]),
    );
    pb
}

pub fn get_progress_known_values(max_value: usize) -> ProgressBar {
    let pb = ProgressBar::new(max_value as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.blue} {msg} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} ({eta})")
            .progress_chars("##-"),
    );
    pb
}

// pub fn get_progress_loading_saving_cache() -> ProgressBar {
//
// }
