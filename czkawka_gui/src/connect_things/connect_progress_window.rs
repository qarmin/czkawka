use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::time::Duration;

use crossbeam_channel::Receiver;
use czkawka_core::common_dir_traversal::ToolType;
use czkawka_core::progress_data::{CurrentStage, ProgressData};
use glib::MainContext;
use gtk4::prelude::*;
use gtk4::ProgressBar;

use crate::flg;
use crate::gui_structs::gui_data::GuiData;
use crate::localizer_core::generate_translation_hashmap;
use crate::taskbar_progress::tbp_flags::TBPF_INDETERMINATE;
use crate::taskbar_progress::TaskbarProgress;

#[allow(clippy::too_many_arguments)]
pub fn connect_progress_window(gui_data: &GuiData, progress_receiver: Receiver<ProgressData>) {
    let main_context = MainContext::default();
    let _guard = main_context.acquire().expect("Failed to acquire main context");

    let gui_data = gui_data.clone();

    let future = async move {
        loop {
            loop {
                let item = progress_receiver.try_recv();
                if let Ok(item) = item {
                    if item.current_stage_idx == 0 {
                        progress_collect_items(&gui_data, &item, item.tool_type != ToolType::EmptyFolders);
                    } else if item.sstage.check_if_loading_saving_cache() {
                        progress_save_load_cache(&gui_data, &item);
                    } else {
                        progress_default(&gui_data, &item);
                    }
                } else {
                    break;
                }
            }
            glib::timeout_future(Duration::from_millis(300)).await;
        }
    };

    main_context.spawn_local(future);
}

fn progress_save_load_cache(gui_data: &GuiData, item: &ProgressData) {
    let label_stage = gui_data.progress_window.label_stage.clone();
    let progress_bar_current_stage = gui_data.progress_window.progress_bar_current_stage.clone();
    let taskbar_state = gui_data.taskbar_state.clone();

    progress_bar_current_stage.hide();
    taskbar_state.borrow().set_progress_state(TBPF_INDETERMINATE);

    let text = match item.sstage {
        CurrentStage::SameMusicCacheLoadingFingerprints | CurrentStage::SameMusicCacheLoadingTags => {
            flg!("progress_cache_loading")
        }
        CurrentStage::SameMusicCacheSavingFingerprints | CurrentStage::SameMusicCacheSavingTags => {
            flg!("progress_cache_saving")
        }
        CurrentStage::DuplicateCacheLoading => {
            flg!("progress_hash_cache_loading")
        }
        CurrentStage::DuplicateCacheSaving => {
            flg!("progress_hash_cache_saving")
        }
        CurrentStage::DuplicatePreHashCacheLoading => {
            flg!("progress_prehash_cache_loading")
        }
        CurrentStage::DuplicatePreHashCacheSaving => {
            flg!("progress_prehash_cache_saving")
        }
        _ => panic!("Invalid stage {:?}", item.sstage),
    };

    label_stage.set_text(&text);
}

fn progress_collect_items(gui_data: &GuiData, item: &ProgressData, files: bool) {
    let label_stage = gui_data.progress_window.label_stage.clone();
    let progress_bar_current_stage = gui_data.progress_window.progress_bar_current_stage.clone();
    let taskbar_state = gui_data.taskbar_state.clone();

    progress_bar_current_stage.hide();
    taskbar_state.borrow().set_progress_state(TBPF_INDETERMINATE);

    match item.sstage {
        CurrentStage::DuplicateScanningName => {
            label_stage.set_text(&flg!("progress_scanning_name", file_number_tm(item)));
        }
        CurrentStage::DuplicateScanningSizeName => {
            label_stage.set_text(&flg!("progress_scanning_size_name", file_number_tm(item)));
        }
        CurrentStage::DuplicateScanningSize => {
            label_stage.set_text(&flg!("progress_scanning_size", file_number_tm(item)));
        }
        _ => {
            if files {
                label_stage.set_text(&flg!("progress_scanning_general_file", file_number_tm(item)));
            } else {
                label_stage.set_text(&flg!("progress_scanning_empty_folders", folder_number = item.entries_checked));
            }
        }
    }
}

fn progress_default(gui_data: &GuiData, item: &ProgressData) {
    let label_stage = gui_data.progress_window.label_stage.clone();
    let progress_bar_current_stage = gui_data.progress_window.progress_bar_current_stage.clone();
    let progress_bar_all_stages = gui_data.progress_window.progress_bar_all_stages.clone();
    let taskbar_state = gui_data.taskbar_state.clone();

    progress_bar_current_stage.show();
    common_set_data(item, &progress_bar_all_stages, &progress_bar_current_stage, &taskbar_state);
    taskbar_state.borrow().set_progress_state(TBPF_INDETERMINATE);

    match item.sstage {
        CurrentStage::SameMusicReadingTags => {
            label_stage.set_text(&flg!("progress_scanning_music_tags", progress_ratio_tm(item)));
        }
        CurrentStage::SameMusicCalculatingFingerprints => {
            label_stage.set_text(&flg!("progress_scanning_music_content", progress_ratio_tm(item)));
        }
        CurrentStage::SameMusicComparingTags => {
            label_stage.set_text(&flg!("progress_scanning_music_tags_end", progress_ratio_tm(item)));
        }
        CurrentStage::SameMusicComparingFingerprints => {
            label_stage.set_text(&flg!("progress_scanning_music_content_end", progress_ratio_tm(item)));
        }
        CurrentStage::SimilarImagesCalculatingHashes => {
            label_stage.set_text(&flg!("progress_scanning_image", progress_ratio_tm(item)));
        }
        CurrentStage::SimilarImagesComparingHashes => {
            label_stage.set_text(&flg!("progress_comparing_image_hashes", progress_ratio_tm(item)));
        }
        CurrentStage::SimilarVideosCalculatingHashes => {
            label_stage.set_text(&flg!("progress_scanning_video", progress_ratio_tm(item)));
        }
        CurrentStage::BrokenFilesChecking => {
            label_stage.set_text(&flg!("progress_scanning_broken_files", progress_ratio_tm(item)));
        }
        CurrentStage::BadExtensionsChecking => {
            label_stage.set_text(&flg!("progress_scanning_extension_of_files", progress_ratio_tm(item)));
        }
        CurrentStage::DuplicatePreHashing => {
            label_stage.set_text(&flg!("progress_analyzed_partial_hash", progress_ratio_tm(item)));
        }
        CurrentStage::DuplicateFullHashing => {
            label_stage.set_text(&flg!("progress_analyzed_full_hash", progress_ratio_tm(item)));
        }
        _ => unreachable!("Invalid stage {:?}", item.sstage),
    }
}

fn common_set_data(item: &ProgressData, progress_bar_all_stages: &ProgressBar, progress_bar_current_stage: &ProgressBar, taskbar_state: &Rc<RefCell<TaskbarProgress>>) {
    if item.entries_to_check != 0 {
        let all_stages = (item.current_stage_idx as f64 + item.entries_checked as f64 / item.entries_to_check as f64) / (item.max_stage_idx + 1) as f64;
        let all_stages = all_stages.min(0.99);
        progress_bar_all_stages.set_fraction(all_stages);
        progress_bar_current_stage.set_fraction(item.entries_checked as f64 / item.entries_to_check as f64);
        taskbar_state.borrow().set_progress_value(
            ((item.current_stage_idx as usize) * item.entries_to_check + item.entries_checked) as u64,
            item.entries_to_check as u64 * (item.max_stage_idx + 1) as u64,
        );
    } else {
        let all_stages = (item.current_stage_idx as f64) / (item.max_stage_idx + 1) as f64;
        let all_stages = all_stages.min(0.99);
        progress_bar_all_stages.set_fraction(all_stages);
        progress_bar_current_stage.set_fraction(0f64);
        taskbar_state.borrow().set_progress_value(item.current_stage_idx as u64, 1 + item.max_stage_idx as u64);
    }
}

fn file_number_tm(item: &ProgressData) -> HashMap<&'static str, String> {
    generate_translation_hashmap(vec![("file_number", item.entries_checked.to_string())])
}

fn progress_ratio_tm(item: &ProgressData) -> HashMap<&'static str, String> {
    generate_translation_hashmap(vec![("file_checked", item.entries_checked.to_string()), ("all_files", item.entries_to_check.to_string())])
}
