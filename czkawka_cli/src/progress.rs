use std::time::Duration;

use crossbeam_channel::Receiver;
use czkawka_core::common::progress_data::{ProgressData, ToolStage};
use indicatif::{ProgressBar, ProgressStyle};

pub(crate) fn connect_progress(progress_receiver: &Receiver<ProgressData>) {
    let mut pb = ProgressBar::new(1);
    let mut latest_stage: Option<ToolStage> = None;
    while let Ok(progress_data) = progress_receiver.recv() {
        if latest_stage != Some(progress_data.stage) {
            pb.finish_and_clear();
            pb = if progress_data.stage.is_indeterminate() {
                get_progress_spinner()
            } else if progress_data.bytes_to_check != 0 {
                get_progress_known_values(progress_data.bytes_to_check)
            } else {
                get_progress_known_values(progress_data.entries_to_check as u64)
            };
            latest_stage = Some(progress_data.stage);
        }

        if !progress_data.stage.is_indeterminate() {
            let position = if progress_data.bytes_to_check != 0 {
                progress_data.bytes_checked
            } else {
                progress_data.entries_checked as u64
            };
            pb.set_position(position);
        }
        pb.set_message(progress_data.to_display().label);
    }
    pb.finish_and_clear();
}

pub(crate) fn get_progress_spinner() -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(120));
    #[expect(clippy::literal_string_with_formatting_args)]
    pb.set_style(
        ProgressStyle::with_template("{msg} {spinner:.blue}")
            .expect("Failed to create progress bar style")
            .tick_strings(&["▹▹▹▹▹", "▸▹▹▹▹", "▹▸▹▹▹", "▹▹▸▹▹", "▹▹▹▸▹", "▹▹▹▹▸", "▪▪▪▪▪"]),
    );
    pb
}

pub(crate) fn get_progress_known_values(max_value: u64) -> ProgressBar {
    let pb = ProgressBar::new(max_value);
    pb.set_style(
        ProgressStyle::with_template("{msg} [{bar}]")
            .expect("Failed to create progress bar style")
            .progress_chars("=> "),
    );
    pb
}
