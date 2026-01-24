use std::time::Duration;

use crossbeam_channel::Receiver;
use czkawka_core::common::model::ToolType;
use czkawka_core::common::progress_data::{CurrentStage, ProgressData};
use humansize::{BINARY, format_size};
use indicatif::{ProgressBar, ProgressStyle};

pub(crate) fn connect_progress(progress_receiver: &Receiver<ProgressData>) {
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
            } else if progress_data.bytes_to_check != 0 {
                pb = get_progress_known_values(progress_data.bytes_to_check);
            } else {
                pb = get_progress_known_values(progress_data.entries_to_check as u64);
            }
            latest_id = Some(progress_data.current_stage_idx);
        }

        if progress_data.sstage == CurrentStage::CollectingFiles && progress_data.tool_type != ToolType::EmptyFolders {
            pb.set_message(format!("Collecting files: {}", progress_data.entries_checked));
        } else if progress_data.sstage == CurrentStage::CollectingFiles {
            pb.set_message(format!("Collecting folders: {}", progress_data.entries_checked));
        } else if !progress_data.sstage.check_if_loading_saving_cache() {
            if progress_data.bytes_to_check != 0 {
                pb.set_position(progress_data.bytes_checked);
                pb.set_message(format!(
                    "{}: {}/{} ({}/{})",
                    get_progress_message(&progress_data),
                    progress_data.entries_checked,
                    progress_data.entries_to_check,
                    format_size(progress_data.bytes_checked, BINARY),
                    format_size(progress_data.bytes_to_check, BINARY)
                ));
            } else {
                pb.set_position(progress_data.entries_checked as u64);
                pb.set_message(format!(
                    "{}: {}/{}",
                    get_progress_message(&progress_data),
                    progress_data.entries_checked,
                    progress_data.entries_to_check
                ));
            }
        }
    }
    pb.finish_and_clear();
}

pub(crate) fn get_progress_message(progress_data: &ProgressData) -> String {
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
        CurrentStage::SimilarVideosCreatingThumbnails | CurrentStage::VideoOptimizerCreatingThumbnails => "Creating video thumbnails",
        CurrentStage::BrokenFilesChecking => "Checking broken files",
        CurrentStage::BadExtensionsChecking => "Checking extensions of files",
        CurrentStage::DeletingFiles => "Deleting files/folders",
        CurrentStage::RenamingFiles => "Renaming files",
        CurrentStage::MovingFiles => "Moving files",
        CurrentStage::HardlinkingFiles => "Creating hardlinks",
        CurrentStage::SymlinkingFiles => "Creating symlinks",
        CurrentStage::OptimizingVideos => "Optimizing videos",
        CurrentStage::CleaningExif => "Cleaning EXIF data",
        CurrentStage::ExifRemoverExtractingTags => "Extracting EXIF tags",
        CurrentStage::VideoOptimizerProcessingVideos => "Processing videos",

        CurrentStage::CollectingFiles
        | CurrentStage::DuplicateCacheSaving
        | CurrentStage::DuplicateCacheLoading
        | CurrentStage::DuplicatePreHashCacheSaving
        | CurrentStage::DuplicatePreHashCacheLoading
        | CurrentStage::DuplicateScanningName
        | CurrentStage::DuplicateScanningSizeName
        | CurrentStage::DuplicateScanningSize
        | CurrentStage::SameMusicCacheSavingTags
        | CurrentStage::SameMusicCacheLoadingTags
        | CurrentStage::SameMusicCacheSavingFingerprints
        | CurrentStage::SameMusicCacheLoadingFingerprints
        | CurrentStage::ExifRemoverCacheLoading
        | CurrentStage::ExifRemoverCacheSaving => unreachable!("This stages(caches, initial files scanning) should be handled somewhere else"),
    }
    .to_string()
}

pub(crate) fn get_progress_bar_for_collect_files() -> ProgressBar {
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

pub(crate) fn get_progress_loading_saving_cache(loading: bool) -> ProgressBar {
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
