use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::time::Duration;

use crossbeam_channel::Receiver;
use glib::MainContext;
use gtk4::prelude::*;
use gtk4::ProgressBar;

use common_dir_traversal::CheckingMethod;
use czkawka_core::common_dir_traversal;
use czkawka_core::common_dir_traversal::{ProgressData, ToolType};

use crate::flg;
use crate::gui_structs::gui_data::GuiData;
use crate::localizer_core::generate_translation_hashmap;
use crate::taskbar_progress::tbp_flags::TBPF_INDETERMINATE;
use crate::taskbar_progress::TaskbarProgress;

// Empty files
// 0 - Collecting files

// Empty folders
// 0 - Collecting folders

// Big files
// 0 - Collecting files

// Same music
// 0 - Collecting files
// 1 - Loading cache
// 2 - Checking tags / content
// 3 - Saving cache
// 4 - Checking tags / content - progress
// 5 - Only content - ending

// Similar images
// 0 - Collecting files
// 1 - Scanning images
// 2 - Comparing hashes

// Similar videos
// 0 - Collecting files
// 1 - Scanning videos

// Temporary files
// 0 - Collecting files

// Invalid symlinks
// 0 - Collecting files

// Broken files
// 0 - Collecting files
// 1 - Scanning files

// Bad extensions
// 0 - Collecting files
// 1 - Scanning files

// Duplicates - Hash
// 0 - Collecting files
// 1 - Loading cache
// 2 - Hash - first 1KB file
// 3 - Saving cache
// 4 - Loading cache
// 5 - Hash - normal hash
// 6 - Saving cache

// Duplicates - Name or SizeName or Size
// 0 - Collecting files

#[allow(clippy::too_many_arguments)]
pub fn connect_progress_window(gui_data: &GuiData, progress_receiver: Receiver<ProgressData>) {
    let main_context = MainContext::default();
    let _guard = main_context.acquire().unwrap();

    let gui_data = gui_data.clone();

    let future = async move {
        loop {
            loop {
                let item = progress_receiver.try_recv();
                if let Ok(item) = item {
                    if item.current_stage == 0 {
                        progress_collect_items(&gui_data, &item, item.tool_type != ToolType::EmptyFolders);
                    } else if check_if_loading_saving_cache(&item) {
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
pub fn check_if_loading_saving_cache(progress_data: &ProgressData) -> bool {
    matches!(
        (progress_data.tool_type, progress_data.current_stage),
        (ToolType::SameMusic, 1 | 3) | (ToolType::Duplicate, 1 | 3 | 4 | 6)
    )
}
fn progress_save_load_cache(gui_data: &GuiData, item: &ProgressData) {
    let label_stage = gui_data.progress_window.label_stage.clone();
    let progress_bar_current_stage = gui_data.progress_window.progress_bar_current_stage.clone();
    let taskbar_state = gui_data.taskbar_state.clone();

    progress_bar_current_stage.hide();
    taskbar_state.borrow().set_progress_state(TBPF_INDETERMINATE);
    let text = match (item.tool_type, item.checking_method, item.current_stage) {
        (ToolType::SameMusic, CheckingMethod::AudioTags | CheckingMethod::AudioContent, 1) => {
            flg!("progress_cache_loading")
        }
        (ToolType::SameMusic, CheckingMethod::AudioTags | CheckingMethod::AudioContent, 3) => {
            flg!("progress_cache_saving")
        }
        (ToolType::Duplicate, CheckingMethod::Hash, 1) => {
            flg!("progress_prehash_cache_loading")
        }
        (ToolType::Duplicate, CheckingMethod::Hash, 3) => {
            flg!("progress_prehash_cache_saving")
        }
        (ToolType::Duplicate, CheckingMethod::Hash, 4) => {
            flg!("progress_hash_cache_loading")
        }
        (ToolType::Duplicate, CheckingMethod::Hash, 6) => {
            flg!("progress_hash_cache_saving")
        }
        _ => panic!(),
    };
    label_stage.set_text(&text);
}

fn progress_collect_items(gui_data: &GuiData, item: &ProgressData, files: bool) {
    let label_stage = gui_data.progress_window.label_stage.clone();
    let progress_bar_current_stage = gui_data.progress_window.progress_bar_current_stage.clone();
    let taskbar_state = gui_data.taskbar_state.clone();

    progress_bar_current_stage.hide();
    taskbar_state.borrow().set_progress_state(TBPF_INDETERMINATE);

    match (item.tool_type, item.checking_method) {
        (ToolType::Duplicate, CheckingMethod::Name) => {
            label_stage.set_text(&flg!("progress_scanning_name", file_number_tm(item)));
            return;
        }
        (ToolType::Duplicate, CheckingMethod::SizeName) => {
            label_stage.set_text(&flg!("progress_scanning_size_name", file_number_tm(item)));
            return;
        }
        (ToolType::Duplicate, CheckingMethod::Size | CheckingMethod::Hash) => {
            label_stage.set_text(&flg!("progress_scanning_size", file_number_tm(item)));
            return;
        }
        _ => {}
    }

    if files {
        label_stage.set_text(&flg!("progress_scanning_general_file", file_number_tm(item)));
    } else {
        label_stage.set_text(&flg!("progress_scanning_empty_folders", folder_number = item.entries_checked));
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

    match (item.tool_type, item.checking_method, item.current_stage) {
        (ToolType::SameMusic, CheckingMethod::AudioTags, 2) | (ToolType::SameMusic, CheckingMethod::AudioContent, 5) => {
            label_stage.set_text(&flg!("progress_scanning_music_tags", progress_ratio_tm(item)));
        }
        (ToolType::SameMusic, CheckingMethod::AudioContent, 2) => {
            label_stage.set_text(&flg!("progress_scanning_music_content", progress_ratio_tm(item)));
        }
        (ToolType::SameMusic, CheckingMethod::AudioTags, 4) => {
            label_stage.set_text(&flg!("progress_scanning_music_tags_end", progress_ratio_tm(item)));
        }
        (ToolType::SameMusic, CheckingMethod::AudioContent, 4) => {
            label_stage.set_text(&flg!("progress_scanning_music_content_end", progress_ratio_tm(item)));
        }
        (ToolType::SimilarImages, _, 1) => {
            label_stage.set_text(&flg!("progress_scanning_image", progress_ratio_tm(item)));
        }
        (ToolType::SimilarImages, _, 2) => {
            label_stage.set_text(&flg!("progress_comparing_image_hashes", progress_ratio_tm(item)));
        }
        (ToolType::SimilarVideos, _, 1) => {
            label_stage.set_text(&flg!("progress_scanning_video", progress_ratio_tm(item)));
        }
        (ToolType::BrokenFiles, _, 1) => {
            label_stage.set_text(&flg!("progress_scanning_broken_files", progress_ratio_tm(item)));
        }
        (ToolType::BadExtensions, _, 1) => {
            label_stage.set_text(&flg!("progress_scanning_extension_of_files", progress_ratio_tm(item)));
        }
        (ToolType::Duplicate, CheckingMethod::Hash, 2) => {
            label_stage.set_text(&flg!("progress_analyzed_partial_hash", progress_ratio_tm(item)));
        }
        (ToolType::Duplicate, CheckingMethod::Hash, 5) => {
            label_stage.set_text(&flg!("progress_analyzed_full_hash", progress_ratio_tm(item)));
        }
        _ => unreachable!(),
    }
}

fn common_set_data(item: &ProgressData, progress_bar_all_stages: &ProgressBar, progress_bar_current_stage: &ProgressBar, taskbar_state: &Rc<RefCell<TaskbarProgress>>) {
    if item.entries_to_check != 0 {
        let all_stages = (item.current_stage as f64 + item.entries_checked as f64 / item.entries_to_check as f64) / (item.max_stage + 1) as f64;
        let all_stages = if all_stages > 0.99 { 0.99 } else { all_stages };
        progress_bar_all_stages.set_fraction(all_stages);
        progress_bar_current_stage.set_fraction(item.entries_checked as f64 / item.entries_to_check as f64);
        taskbar_state.borrow().set_progress_value(
            ((item.current_stage as usize) * item.entries_to_check + item.entries_checked) as u64,
            item.entries_to_check as u64 * (item.max_stage + 1) as u64,
        );
    } else {
        let all_stages = (item.current_stage as f64) / (item.max_stage + 1) as f64;
        let all_stages = if all_stages > 0.99 { 0.99 } else { all_stages };
        progress_bar_all_stages.set_fraction(all_stages);
        progress_bar_current_stage.set_fraction(0f64);
        taskbar_state.borrow().set_progress_value(item.current_stage as u64, 1 + item.max_stage as u64);
    }
}

fn file_number_tm(item: &ProgressData) -> HashMap<&'static str, String> {
    generate_translation_hashmap(vec![("file_number", item.entries_checked.to_string())])
}

fn progress_ratio_tm(item: &ProgressData) -> HashMap<&'static str, String> {
    generate_translation_hashmap(vec![("file_checked", item.entries_checked.to_string()), ("all_files", item.entries_to_check.to_string())])
}
