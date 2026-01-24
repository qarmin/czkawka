use std::thread;

use crossbeam_channel::Receiver;
use czkawka_core::common::model::ToolType;
use czkawka_core::common::progress_data::{CurrentStage, ProgressData};
use humansize::{BINARY, format_size};
use slint::ComponentHandle;

use crate::{MainWindow, ProgressToSend, flk};

pub(crate) fn connect_progress_gathering(app: &MainWindow, progress_receiver: Receiver<ProgressData>) {
    let a = app.as_weak();

    thread::spawn(move || {
        loop {
            let Ok(progress_data) = progress_receiver.recv() else {
                return; // Channel closed, so exit the thread since app closing
            };

            a.upgrade_in_event_loop(move |app| {
                let removing_empty_folders = progress_data.tool_type == ToolType::EmptyFolders;

                let to_send = if progress_data.current_stage_idx == 0 && !progress_data.sstage.is_special_non_tool_stage() {
                    progress_collect_items(&progress_data, !removing_empty_folders)
                } else if progress_data.sstage.check_if_loading_saving_cache() {
                    progress_save_load_cache(&progress_data)
                } else {
                    progress_default(&progress_data)
                };

                app.set_progress_datas(to_send);
            })
            .expect("Failed to spawn thread for progress gathering");
        }
    });
}

fn progress_save_load_cache(item: &ProgressData) -> ProgressToSend {
    let step_name = match item.sstage {
        CurrentStage::SameMusicCacheLoadingTags => flk!("rust_loading_tags_cache"),
        CurrentStage::SameMusicCacheLoadingFingerprints => flk!("rust_loading_fingerprints_cache"),
        CurrentStage::SameMusicCacheSavingTags => flk!("rust_saving_tags_cache"),
        CurrentStage::SameMusicCacheSavingFingerprints => flk!("rust_saving_fingerprints_cache"),
        CurrentStage::DuplicatePreHashCacheLoading => flk!("rust_loading_prehash_cache"),
        CurrentStage::DuplicatePreHashCacheSaving => flk!("rust_saving_prehash_cache"),
        CurrentStage::DuplicateCacheLoading => flk!("rust_loading_hash_cache"),
        CurrentStage::DuplicateCacheSaving => flk!("rust_saving_hash_cache"),
        CurrentStage::ExifRemoverCacheLoading => flk!("rust_loading_exif_cache"),
        CurrentStage::ExifRemoverCacheSaving => flk!("rust_saving_exif_cache"),
        CurrentStage::DeletingFiles
        | CurrentStage::RenamingFiles
        | CurrentStage::MovingFiles
        | CurrentStage::HardlinkingFiles
        | CurrentStage::SymlinkingFiles
        | CurrentStage::OptimizingVideos
        | CurrentStage::CleaningExif
        | CurrentStage::CollectingFiles
        | CurrentStage::DuplicateScanningName
        | CurrentStage::DuplicateScanningSizeName
        | CurrentStage::DuplicateScanningSize
        | CurrentStage::DuplicatePreHashing
        | CurrentStage::DuplicateFullHashing
        | CurrentStage::SameMusicReadingTags
        | CurrentStage::SameMusicCalculatingFingerprints
        | CurrentStage::SameMusicComparingTags
        | CurrentStage::SameMusicComparingFingerprints
        | CurrentStage::SimilarImagesCalculatingHashes
        | CurrentStage::SimilarImagesComparingHashes
        | CurrentStage::SimilarVideosCalculatingHashes
        | CurrentStage::SimilarVideosCreatingThumbnails
        | CurrentStage::BrokenFilesChecking
        | CurrentStage::BadExtensionsChecking
        | CurrentStage::ExifRemoverExtractingTags
        | CurrentStage::VideoOptimizerCreatingThumbnails
        | CurrentStage::VideoOptimizerProcessingVideos => unreachable!(),
    };
    let (all_progress, current_progress, current_progress_size) = common_get_data(item);
    ProgressToSend {
        all_progress,
        current_progress,
        current_progress_size,
        step_name: step_name.into(),
    }
}

fn progress_collect_items(item: &ProgressData, files: bool) -> ProgressToSend {
    let step_name = match item.sstage {
        CurrentStage::DuplicateScanningName => flk!("rust_scanning_name", entries_checked = item.entries_checked),
        CurrentStage::DuplicateScanningSizeName => flk!("rust_scanning_size_name", entries_checked = item.entries_checked),
        CurrentStage::DuplicateScanningSize => flk!("rust_scanning_size", entries_checked = item.entries_checked),
        _ => {
            if files {
                flk!("rust_scanning_file", entries_checked = item.entries_checked)
            } else {
                flk!("rust_scanning_folder", entries_checked = item.entries_checked)
            }
        }
    };
    let (all_progress, current_progress) = no_current_stage_get_data(item);
    ProgressToSend {
        all_progress,
        current_progress,
        current_progress_size: -1,
        step_name: step_name.into(),
    }
}

fn progress_default(item: &ProgressData) -> ProgressToSend {
    let items_stats = format!("{}/{}", item.entries_checked, item.entries_to_check);
    let size_stats = format!("{}/{}", format_size(item.bytes_checked, BINARY), format_size(item.bytes_to_check, BINARY));
    let step_name = match item.sstage {
        CurrentStage::SameMusicReadingTags => flk!("rust_checked_tags", items_stats = items_stats),
        CurrentStage::SameMusicCalculatingFingerprints => flk!("rust_checked_content", items_stats = items_stats, size_stats = size_stats),
        CurrentStage::SameMusicComparingTags => flk!("rust_compared_tags", items_stats = items_stats),
        CurrentStage::SameMusicComparingFingerprints => flk!("rust_compared_content", items_stats = items_stats),
        CurrentStage::SimilarImagesCalculatingHashes => flk!("rust_hashed_images", items_stats = items_stats, size_stats = size_stats),
        CurrentStage::SimilarImagesComparingHashes => flk!("rust_compared_image_hashes", items_stats = items_stats),
        CurrentStage::SimilarVideosCalculatingHashes => flk!("rust_hashed_videos", items_stats = items_stats),
        CurrentStage::SimilarVideosCreatingThumbnails => flk!("rust_created_thumbnails", items_stats = items_stats),
        CurrentStage::BrokenFilesChecking => flk!("rust_checked_files", items_stats = items_stats, size_stats = size_stats),
        CurrentStage::BadExtensionsChecking => flk!("rust_checked_files_bad_extensions", items_stats = items_stats),
        CurrentStage::VideoOptimizerCreatingThumbnails => flk!("rust_checked_images", items_stats = items_stats, size_stats = size_stats),
        CurrentStage::VideoOptimizerProcessingVideos => flk!("rust_checked_videos", items_stats = items_stats, size_stats = size_stats),
        CurrentStage::DuplicatePreHashing => flk!("rust_analyzed_partial_hash", items_stats = items_stats, size_stats = size_stats),
        CurrentStage::DuplicateFullHashing => flk!("rust_analyzed_full_hash", items_stats = items_stats, size_stats = size_stats),

        CurrentStage::DeletingFiles if item.bytes_to_check != 0 => flk!("rust_deleting_files", items_stats = items_stats, size_stats = size_stats),
        CurrentStage::DeletingFiles => flk!("rust_deleting_no_size_files", items_stats = items_stats),
        CurrentStage::RenamingFiles => flk!("rust_renaming_files", items_stats = items_stats),
        CurrentStage::MovingFiles if item.bytes_to_check != 0 => flk!("rust_moving_files", items_stats = items_stats, size_stats = size_stats),
        CurrentStage::MovingFiles => flk!("rust_moving_no_size_files", items_stats = items_stats),
        CurrentStage::HardlinkingFiles if item.bytes_to_check != 0 => flk!("rust_hardlinking_files", items_stats = items_stats, size_stats = size_stats),
        CurrentStage::HardlinkingFiles => flk!("rust_hardlinking_no_size_files", items_stats = items_stats),
        CurrentStage::SymlinkingFiles if item.bytes_to_check != 0 => flk!("rust_symlinking_files", items_stats = items_stats, size_stats = size_stats),
        CurrentStage::SymlinkingFiles => flk!("rust_symlinking_no_size_files", items_stats = items_stats),
        CurrentStage::OptimizingVideos if item.bytes_to_check != 0 => flk!("rust_optimizing_videos", items_stats = items_stats, size_stats = size_stats),
        CurrentStage::OptimizingVideos => flk!("rust_optimizing_no_size_videos", items_stats = items_stats),
        CurrentStage::CleaningExif if item.bytes_to_check != 0 => flk!("rust_cleaning_exif", items_stats = items_stats, size_stats = size_stats),
        CurrentStage::CleaningExif => flk!("rust_cleaning_no_size_exif", items_stats = items_stats),

        CurrentStage::ExifRemoverExtractingTags => flk!("rust_extracted_exif_tags", items_stats = items_stats, size_stats = size_stats),

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
    };
    let (all_progress, current_progress, current_progress_size) = common_get_data(item);

    // Deleting is a single operation, so we don't need to show two same progress bars
    let all_progress = if item.sstage.is_special_non_tool_stage() { -1 } else { all_progress };

    ProgressToSend {
        all_progress,
        current_progress,
        current_progress_size,
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
fn common_get_data(item: &ProgressData) -> (i32, i32, i32) {
    let (current_items_checked, current_stage_items_to_check) = if item.bytes_to_check > 0 {
        (item.bytes_checked, item.bytes_to_check)
    } else {
        (item.entries_checked as u64, item.entries_to_check as u64)
    };

    if item.entries_to_check != 0 {
        let all_stages = (item.current_stage_idx as f64 + current_items_checked as f64 / current_stage_items_to_check as f64) / (item.max_stage_idx + 1) as f64;
        let all_stages = all_stages.min(0.99);

        let current_stage = current_items_checked as f64 / current_stage_items_to_check as f64;
        let current_stage = current_stage.min(0.99);

        let current_stage_size = if item.bytes_to_check != 0 {
            ((item.bytes_checked as f64 / item.bytes_to_check as f64).min(0.99) * 100.0) as i32
        } else {
            -1
        };

        ((all_stages * 100.0) as i32, (current_stage * 100.0) as i32, current_stage_size)
    } else {
        let all_stages = (item.current_stage_idx as f64) / (item.max_stage_idx + 1) as f64;
        let all_stages = all_stages.min(0.99);
        ((all_stages * 100.0) as i32, 0, -1)
    }
}
