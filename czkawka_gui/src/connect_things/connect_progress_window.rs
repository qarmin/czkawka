use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use futures::channel::mpsc::UnboundedReceiver;
use futures::StreamExt;
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

#[allow(clippy::too_many_arguments)]
pub fn connect_progress_window(gui_data: &GuiData, mut progress_receiver: UnboundedReceiver<ProgressData>) {
    let main_context = MainContext::default();
    let _guard = main_context.acquire().unwrap();

    let gui_data = gui_data.clone();
    let future = async move {
        while let Some(item) = progress_receiver.next().await {
            match item.tool_type {
                ToolType::Duplicate => process_bar_duplicates(&gui_data, &item),
                ToolType::EmptyFiles => process_bar_empty_files(&gui_data, &item),
                ToolType::EmptyFolders => process_bar_empty_folder(&gui_data, &item),
                ToolType::BigFile => process_bar_big_files(&gui_data, &item),
                ToolType::SameMusic => process_bar_same_music(&gui_data, &item),
                ToolType::SimilarImages => process_bar_similar_images(&gui_data, &item),
                ToolType::SimilarVideos => process_bar_similar_videos(&gui_data, &item),
                ToolType::TemporaryFiles => process_bar_temporary(&gui_data, &item),
                ToolType::InvalidSymlinks => process_bar_invalid_symlinks(&gui_data, &item),
                ToolType::BrokenFiles => process_bar_broken_files(&gui_data, &item),
                ToolType::BadExtensions => process_bar_bad_extensions(&gui_data, &item),
                ToolType::None => panic!(),
            }
        }
    };
    main_context.spawn_local(future);
}
fn process_bar_empty_files(gui_data: &GuiData, item: &ProgressData) {
    let label_stage = gui_data.progress_window.label_stage.clone();
    let taskbar_state = gui_data.taskbar_state.clone();

    label_stage.set_text(&flg!("progress_scanning_general_file", file_number_tm(item)));
    taskbar_state.borrow().set_progress_state(TBPF_INDETERMINATE);
}
fn process_bar_empty_folder(gui_data: &GuiData, item: &ProgressData) {
    let label_stage = gui_data.progress_window.label_stage.clone();
    let taskbar_state = gui_data.taskbar_state.clone();

    label_stage.set_text(&flg!(
        "progress_scanning_empty_folders",
        generate_translation_hashmap(vec![("folder_number", item.entries_checked.to_string())])
    ));
    taskbar_state.borrow().set_progress_state(TBPF_INDETERMINATE);
}
fn process_bar_big_files(gui_data: &GuiData, item: &ProgressData) {
    let label_stage = gui_data.progress_window.label_stage.clone();
    let taskbar_state = gui_data.taskbar_state.clone();

    label_stage.set_text(&flg!("progress_scanning_general_file", file_number_tm(item)));
    taskbar_state.borrow().set_progress_state(TBPF_INDETERMINATE);
}
fn process_bar_same_music(gui_data: &GuiData, item: &ProgressData) {
    let label_stage = gui_data.progress_window.label_stage.clone();
    let progress_bar_current_stage = gui_data.progress_window.progress_bar_current_stage.clone();
    let progress_bar_all_stages = gui_data.progress_window.progress_bar_all_stages.clone();
    let taskbar_state = gui_data.taskbar_state.clone();

    match item.current_stage {
        0 => {
            progress_bar_current_stage.hide();
            label_stage.set_text(&flg!("progress_scanning_general_file", file_number_tm(item)));
            taskbar_state.borrow().set_progress_state(TBPF_INDETERMINATE);
        }
        1 => {
            progress_bar_current_stage.show();
            common_set_data(item, &progress_bar_all_stages, &progress_bar_current_stage, &taskbar_state);

            match item.checking_method {
                CheckingMethod::AudioTags => label_stage.set_text(&flg!("progress_scanning_music_tags", progress_ratio_tm(item))),
                CheckingMethod::AudioContent => label_stage.set_text(&flg!("progress_scanning_music_content", progress_ratio_tm(item))),
                _ => panic!(),
            }
        }
        2 => {
            common_set_data(item, &progress_bar_all_stages, &progress_bar_current_stage, &taskbar_state);

            match item.checking_method {
                CheckingMethod::AudioTags => label_stage.set_text(&flg!("progress_scanning_music_tags_end", progress_ratio_tm(item))),
                CheckingMethod::AudioContent => label_stage.set_text(&flg!("progress_scanning_music_content_end", progress_ratio_tm(item))),
                _ => panic!(),
            }
        }
        3 => {
            common_set_data(item, &progress_bar_all_stages, &progress_bar_current_stage, &taskbar_state);

            match item.checking_method {
                CheckingMethod::AudioContent => label_stage.set_text(&flg!("progress_scanning_music_tags", progress_ratio_tm(item))),
                _ => panic!(),
            }
        }
        _ => panic!(),
    }
}
fn process_bar_similar_images(gui_data: &GuiData, item: &ProgressData) {
    let label_stage = gui_data.progress_window.label_stage.clone();
    let progress_bar_current_stage = gui_data.progress_window.progress_bar_current_stage.clone();
    let progress_bar_all_stages = gui_data.progress_window.progress_bar_all_stages.clone();
    let taskbar_state = gui_data.taskbar_state.clone();

    match item.current_stage {
        0 => {
            progress_bar_current_stage.hide();
            label_stage.set_text(&flg!("progress_scanning_general_file", file_number_tm(item)));
            taskbar_state.borrow().set_progress_state(TBPF_INDETERMINATE);
        }
        1 => {
            progress_bar_current_stage.show();
            common_set_data(item, &progress_bar_all_stages, &progress_bar_current_stage, &taskbar_state);
            label_stage.set_text(&flg!("progress_scanning_image", progress_ratio_tm(item)));
        }
        2 => {
            progress_bar_current_stage.show();
            common_set_data(item, &progress_bar_all_stages, &progress_bar_current_stage, &taskbar_state);
            label_stage.set_text(&flg!("progress_comparing_image_hashes", progress_ratio_tm(item)));
        }
        _ => panic!(),
    }
}
fn process_bar_similar_videos(gui_data: &GuiData, item: &ProgressData) {
    let label_stage = gui_data.progress_window.label_stage.clone();
    let progress_bar_current_stage = gui_data.progress_window.progress_bar_current_stage.clone();
    let progress_bar_all_stages = gui_data.progress_window.progress_bar_all_stages.clone();
    let taskbar_state = gui_data.taskbar_state.clone();

    match item.current_stage {
        0 => {
            progress_bar_current_stage.hide();
            label_stage.set_text(&flg!("progress_scanning_general_file", file_number_tm(item)));
            taskbar_state.borrow().set_progress_state(TBPF_INDETERMINATE);
        }
        1 => {
            progress_bar_current_stage.show();
            common_set_data(item, &progress_bar_all_stages, &progress_bar_current_stage, &taskbar_state);
            label_stage.set_text(&flg!("progress_scanning_video", progress_ratio_tm(item)));
        }
        _ => panic!(),
    }
}
fn process_bar_temporary(gui_data: &GuiData, item: &ProgressData) {
    let label_stage = gui_data.progress_window.label_stage.clone();
    let taskbar_state = gui_data.taskbar_state.clone();

    label_stage.set_text(&flg!("progress_scanning_general_file", file_number_tm(item)));
    taskbar_state.borrow().set_progress_state(TBPF_INDETERMINATE);
}
fn process_bar_invalid_symlinks(gui_data: &GuiData, item: &ProgressData) {
    let label_stage = gui_data.progress_window.label_stage.clone();
    let taskbar_state = gui_data.taskbar_state.clone();

    label_stage.set_text(&flg!("progress_scanning_general_file", file_number_tm(item)));
    taskbar_state.borrow().set_progress_state(TBPF_INDETERMINATE);
}
fn process_bar_broken_files(gui_data: &GuiData, item: &ProgressData) {
    let label_stage = gui_data.progress_window.label_stage.clone();
    let progress_bar_current_stage = gui_data.progress_window.progress_bar_current_stage.clone();
    let progress_bar_all_stages = gui_data.progress_window.progress_bar_all_stages.clone();
    let taskbar_state = gui_data.taskbar_state.clone();

    match item.current_stage {
        0 => {
            progress_bar_current_stage.hide();
            label_stage.set_text(&flg!("progress_scanning_general_file", file_number_tm(item)));
            taskbar_state.borrow().set_progress_state(TBPF_INDETERMINATE);
        }
        1 => {
            progress_bar_current_stage.show();
            common_set_data(item, &progress_bar_all_stages, &progress_bar_current_stage, &taskbar_state);
            label_stage.set_text(&flg!("progress_scanning_broken_files", progress_ratio_tm(item)));
        }
        _ => panic!(),
    }
}
fn process_bar_bad_extensions(gui_data: &GuiData, item: &ProgressData) {
    let label_stage = gui_data.progress_window.label_stage.clone();
    let progress_bar_current_stage = gui_data.progress_window.progress_bar_current_stage.clone();
    let progress_bar_all_stages = gui_data.progress_window.progress_bar_all_stages.clone();
    let taskbar_state = gui_data.taskbar_state.clone();

    match item.current_stage {
        0 => {
            progress_bar_current_stage.hide();
            label_stage.set_text(&flg!("progress_scanning_general_file", file_number_tm(item)));
            taskbar_state.borrow().set_progress_state(TBPF_INDETERMINATE);
        }
        1 => {
            progress_bar_current_stage.show();
            common_set_data(item, &progress_bar_all_stages, &progress_bar_current_stage, &taskbar_state);
            label_stage.set_text(&flg!("progress_scanning_extension_of_files", progress_ratio_tm(item)));
        }
        _ => panic!(),
    }
}
fn process_bar_duplicates(gui_data: &GuiData, item: &ProgressData) {
    let label_stage = gui_data.progress_window.label_stage.clone();
    let progress_bar_current_stage = gui_data.progress_window.progress_bar_current_stage.clone();
    let progress_bar_all_stages = gui_data.progress_window.progress_bar_all_stages.clone();
    let grid_progress_stages = gui_data.progress_window.grid_progress_stages.clone();
    let taskbar_state = gui_data.taskbar_state.clone();

    match item.checking_method {
        CheckingMethod::Hash => {
            label_stage.show();
            match item.current_stage {
                // Checking Size
                0 => {
                    progress_bar_current_stage.hide();
                    // progress_bar_all_stages.hide();
                    progress_bar_all_stages.set_fraction(0 as f64);
                    label_stage.set_text(&flg!("progress_scanning_size", file_number_tm(item)));
                    taskbar_state.borrow().set_progress_state(TBPF_INDETERMINATE);
                }
                // Hash - first 1KB file
                1 => {
                    progress_bar_current_stage.show();
                    // progress_bar_all_stages.show();
                    common_set_data(item, &progress_bar_all_stages, &progress_bar_current_stage, &taskbar_state);

                    label_stage.set_text(&flg!("progress_analyzed_partial_hash", progress_ratio_tm(item)));
                }
                // Hash - normal hash
                2 => {
                    common_set_data(item, &progress_bar_all_stages, &progress_bar_current_stage, &taskbar_state);
                    label_stage.set_text(&flg!("progress_analyzed_full_hash", progress_ratio_tm(item)));
                }
                _ => {
                    panic!("Not available current_stage");
                }
            }
        }
        CheckingMethod::Name => {
            label_stage.show();
            grid_progress_stages.hide();

            label_stage.set_text(&flg!("progress_scanning_name", file_number_tm(item)));
            taskbar_state.borrow().set_progress_state(TBPF_INDETERMINATE);
        }
        CheckingMethod::SizeName => {
            label_stage.show();
            grid_progress_stages.hide();

            label_stage.set_text(&flg!("progress_scanning_size_name", file_number_tm(item)));
            taskbar_state.borrow().set_progress_state(TBPF_INDETERMINATE);
        }
        CheckingMethod::Size => {
            label_stage.show();
            grid_progress_stages.hide();

            label_stage.set_text(&flg!("progress_scanning_size", file_number_tm(item)));
            taskbar_state.borrow().set_progress_state(TBPF_INDETERMINATE);
        }
        _ => panic!(),
    };
}

fn common_set_data(item: &ProgressData, progress_bar_all_stages: &ProgressBar, progress_bar_current_stage: &ProgressBar, taskbar_state: &Rc<RefCell<TaskbarProgress>>) {
    if item.entries_to_check != 0 {
        progress_bar_all_stages.set_fraction((item.current_stage as f64 + (item.entries_checked) as f64 / item.entries_to_check as f64) / (item.max_stage + 1) as f64);
        progress_bar_current_stage.set_fraction((item.entries_checked) as f64 / item.entries_to_check as f64);
        taskbar_state.borrow().set_progress_value(
            ((item.current_stage as usize) * item.entries_to_check + item.entries_checked) as u64,
            item.entries_to_check as u64 * (item.max_stage + 1) as u64,
        );
    } else {
        progress_bar_all_stages.set_fraction((item.current_stage as f64) / (item.max_stage + 1) as f64);
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
