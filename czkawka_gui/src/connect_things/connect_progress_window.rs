use futures::StreamExt;
use gtk::prelude::*;

use czkawka_core::{big_file, broken_files, common_dir_traversal, similar_images, similar_videos, temporary};

use crate::flg;
use crate::gui_structs::gui_data::GuiData;
use crate::localizer_core::generate_translation_hashmap;
use crate::taskbar_progress::tbp_flags::TBPF_INDETERMINATE;

#[allow(clippy::too_many_arguments)]
pub fn connect_progress_window(
    gui_data: &GuiData,
    mut futures_receiver_duplicate_files: futures::channel::mpsc::UnboundedReceiver<common_dir_traversal::ProgressData>,
    mut futures_receiver_empty_files: futures::channel::mpsc::UnboundedReceiver<common_dir_traversal::ProgressData>,
    mut futures_receiver_empty_folder: futures::channel::mpsc::UnboundedReceiver<common_dir_traversal::ProgressData>,
    mut futures_receiver_big_files: futures::channel::mpsc::UnboundedReceiver<big_file::ProgressData>,
    mut futures_receiver_same_music: futures::channel::mpsc::UnboundedReceiver<common_dir_traversal::ProgressData>,
    mut futures_receiver_similar_images: futures::channel::mpsc::UnboundedReceiver<similar_images::ProgressData>,
    mut futures_receiver_similar_videos: futures::channel::mpsc::UnboundedReceiver<similar_videos::ProgressData>,
    mut futures_receiver_temporary: futures::channel::mpsc::UnboundedReceiver<temporary::ProgressData>,
    mut futures_receiver_invalid_symlinks: futures::channel::mpsc::UnboundedReceiver<common_dir_traversal::ProgressData>,
    mut futures_receiver_broken_files: futures::channel::mpsc::UnboundedReceiver<broken_files::ProgressData>,
) {
    let main_context = glib::MainContext::default();
    let _guard = main_context.acquire().unwrap();

    {
        // Duplicate Files
        let label_stage = gui_data.progress_window.label_stage.clone();
        let progress_bar_current_stage = gui_data.progress_window.progress_bar_current_stage.clone();
        let progress_bar_all_stages = gui_data.progress_window.progress_bar_all_stages.clone();
        let grid_progress_stages = gui_data.progress_window.grid_progress_stages.clone();
        let taskbar_state = gui_data.taskbar_state.clone();
        let future = async move {
            while let Some(item) = futures_receiver_duplicate_files.next().await {
                match item.checking_method {
                    common_dir_traversal::CheckingMethod::Hash => {
                        label_stage.show();
                        match item.current_stage {
                            // Checking Size
                            0 => {
                                progress_bar_current_stage.hide();
                                // progress_bar_all_stages.hide();
                                progress_bar_all_stages.set_fraction(0 as f64);
                                label_stage.set_text(&flg!(
                                    "progress_scanning_size",
                                    generate_translation_hashmap(vec![("file_number", item.entries_checked.to_string())])
                                ));
                                taskbar_state.borrow().set_progress_state(TBPF_INDETERMINATE);
                            }
                            // Hash - first 1KB file
                            1 => {
                                progress_bar_current_stage.show();
                                // progress_bar_all_stages.show();
                                if item.entries_to_check != 0 {
                                    progress_bar_all_stages.set_fraction((1f64 + (item.entries_checked) as f64 / item.entries_to_check as f64) / (item.max_stage + 1) as f64);
                                    progress_bar_current_stage.set_fraction((item.entries_checked) as f64 / item.entries_to_check as f64);
                                    taskbar_state.borrow().set_progress_value(
                                        (item.entries_to_check + item.entries_checked) as u64,
                                        item.entries_to_check as u64 * (item.max_stage + 1) as u64,
                                    );
                                } else {
                                    progress_bar_all_stages.set_fraction((1f64) / (item.max_stage + 1) as f64);
                                    progress_bar_current_stage.set_fraction(0f64);
                                    taskbar_state.borrow().set_progress_value(1, 1 + item.max_stage as u64);
                                }

                                label_stage.set_text(&flg!(
                                    "progress_analyzed_partial_hash",
                                    generate_translation_hashmap(vec![("file_checked", item.entries_checked.to_string()), ("all_files", item.entries_to_check.to_string())])
                                ));
                            }
                            // Hash - normal hash
                            2 => {
                                if item.entries_to_check != 0 {
                                    progress_bar_all_stages.set_fraction((2f64 + (item.entries_checked) as f64 / item.entries_to_check as f64) / (item.max_stage + 1) as f64);
                                    progress_bar_current_stage.set_fraction((item.entries_checked) as f64 / item.entries_to_check as f64);
                                    taskbar_state.borrow().set_progress_value(
                                        (2 * item.entries_to_check + item.entries_checked) as u64,
                                        item.entries_to_check as u64 * (item.max_stage + 1) as u64,
                                    );
                                } else {
                                    progress_bar_all_stages.set_fraction((2f64) / (item.max_stage + 1) as f64);
                                    progress_bar_current_stage.set_fraction(0f64);
                                    taskbar_state.borrow().set_progress_value(2, 1 + item.max_stage as u64);
                                }

                                label_stage.set_text(&flg!(
                                    "progress_analyzed_full_hash",
                                    generate_translation_hashmap(vec![("file_checked", item.entries_checked.to_string()), ("all_files", item.entries_to_check.to_string())])
                                ));
                            }
                            _ => {
                                panic!("Not available current_stage");
                            }
                        }
                    }
                    common_dir_traversal::CheckingMethod::Name => {
                        label_stage.show();
                        grid_progress_stages.hide();

                        label_stage.set_text(&flg!(
                            "progress_scanning_name",
                            generate_translation_hashmap(vec![("file_number", item.entries_checked.to_string())])
                        ));
                        taskbar_state.borrow().set_progress_state(TBPF_INDETERMINATE);
                    }
                    common_dir_traversal::CheckingMethod::Size => {
                        label_stage.show();
                        grid_progress_stages.hide();

                        label_stage.set_text(&flg!(
                            "progress_scanning_size",
                            generate_translation_hashmap(vec![("file_number", item.entries_checked.to_string())])
                        ));
                        taskbar_state.borrow().set_progress_state(TBPF_INDETERMINATE);
                    }
                    common_dir_traversal::CheckingMethod::None => {
                        panic!();
                    }
                };
            }
        };
        main_context.spawn_local(future);
    }
    {
        // Empty Files
        let label_stage = gui_data.progress_window.label_stage.clone();
        let taskbar_state = gui_data.taskbar_state.clone();
        let future = async move {
            while let Some(item) = futures_receiver_empty_files.next().await {
                label_stage.set_text(&flg!(
                    "progress_scanning_general_file",
                    generate_translation_hashmap(vec![("file_number", item.entries_checked.to_string())])
                ));
                taskbar_state.borrow().set_progress_state(TBPF_INDETERMINATE);
            }
        };
        main_context.spawn_local(future);
    }
    {
        // Empty Folder
        let label_stage = gui_data.progress_window.label_stage.clone();
        let taskbar_state = gui_data.taskbar_state.clone();
        let future = async move {
            while let Some(item) = futures_receiver_empty_folder.next().await {
                label_stage.set_text(&flg!(
                    "progress_scanning_empty_folders",
                    generate_translation_hashmap(vec![("folder_number", item.entries_checked.to_string())])
                ));
                taskbar_state.borrow().set_progress_state(TBPF_INDETERMINATE);
            }
        };
        main_context.spawn_local(future);
    }
    {
        // Big Files
        let label_stage = gui_data.progress_window.label_stage.clone();
        let taskbar_state = gui_data.taskbar_state.clone();
        let future = async move {
            while let Some(item) = futures_receiver_big_files.next().await {
                label_stage.set_text(&flg!(
                    "progress_scanning_general_file",
                    generate_translation_hashmap(vec![("file_number", item.files_checked.to_string())])
                ));
                taskbar_state.borrow().set_progress_state(TBPF_INDETERMINATE);
            }
        };
        main_context.spawn_local(future);
    }
    {
        // Same Music
        let label_stage = gui_data.progress_window.label_stage.clone();
        let progress_bar_current_stage = gui_data.progress_window.progress_bar_current_stage.clone();
        let progress_bar_all_stages = gui_data.progress_window.progress_bar_all_stages.clone();
        let taskbar_state = gui_data.taskbar_state.clone();
        let future = async move {
            while let Some(item) = futures_receiver_same_music.next().await {
                match item.current_stage {
                    0 => {
                        progress_bar_current_stage.hide();
                        label_stage.set_text(&flg!(
                            "progress_scanning_general_file",
                            generate_translation_hashmap(vec![("file_number", item.entries_checked.to_string())])
                        ));
                        taskbar_state.borrow().set_progress_state(TBPF_INDETERMINATE);
                    }
                    1 => {
                        progress_bar_current_stage.show();
                        if item.entries_to_check != 0 {
                            progress_bar_all_stages.set_fraction((1f64 + (item.entries_checked) as f64 / item.entries_to_check as f64) / (item.max_stage + 1) as f64);
                            progress_bar_current_stage.set_fraction((item.entries_checked) as f64 / item.entries_to_check as f64);
                            taskbar_state.borrow().set_progress_value(
                                (item.entries_to_check + item.entries_checked) as u64,
                                item.entries_to_check as u64 * (item.max_stage + 1) as u64,
                            );
                        } else {
                            progress_bar_all_stages.set_fraction((1f64) / (item.max_stage + 1) as f64);
                            progress_bar_current_stage.set_fraction(0f64);
                            taskbar_state.borrow().set_progress_value(1, (item.max_stage + 1) as u64);
                        }
                        label_stage.set_text(&flg!(
                            "progress_scanning_music_tags",
                            generate_translation_hashmap(vec![("file_checked", item.entries_checked.to_string()), ("all_files", item.entries_to_check.to_string())])
                        ));
                    }
                    2 => {
                        if item.entries_to_check != 0 {
                            progress_bar_all_stages.set_fraction((2f64 + (item.entries_checked) as f64 / item.entries_to_check as f64) / (item.max_stage + 1) as f64);
                            progress_bar_current_stage.set_fraction((item.entries_checked) as f64 / item.entries_to_check as f64);
                            taskbar_state.borrow().set_progress_value(
                                (2 * item.entries_to_check + item.entries_checked) as u64,
                                item.entries_to_check as u64 * (item.max_stage + 1) as u64,
                            );
                        } else {
                            progress_bar_all_stages.set_fraction((2f64) / (item.max_stage + 1) as f64);
                            progress_bar_current_stage.set_fraction(0f64);
                            taskbar_state.borrow().set_progress_value(2, (item.max_stage + 1) as u64);
                        }
                        label_stage.set_text(&flg!(
                            "progress_scanning_music_tags_end",
                            generate_translation_hashmap(vec![("file_checked", item.entries_checked.to_string()), ("all_files", item.entries_to_check.to_string())])
                        ));
                    }
                    _ => {
                        panic!();
                    }
                }
            }
        };
        main_context.spawn_local(future);
    }
    {
        // Similar Images
        let label_stage = gui_data.progress_window.label_stage.clone();
        let progress_bar_current_stage = gui_data.progress_window.progress_bar_current_stage.clone();
        let progress_bar_all_stages = gui_data.progress_window.progress_bar_all_stages.clone();
        let taskbar_state = gui_data.taskbar_state.clone();
        let future = async move {
            while let Some(item) = futures_receiver_similar_images.next().await {
                match item.current_stage {
                    0 => {
                        progress_bar_current_stage.hide();
                        label_stage.set_text(&flg!(
                            "progress_scanning_general_file",
                            generate_translation_hashmap(vec![("file_number", item.images_checked.to_string())])
                        ));
                        taskbar_state.borrow().set_progress_state(TBPF_INDETERMINATE);
                    }
                    1 => {
                        progress_bar_current_stage.show();
                        if item.images_to_check != 0 {
                            progress_bar_all_stages.set_fraction((1f64 + (item.images_checked) as f64 / item.images_to_check as f64) / (item.max_stage + 1) as f64);
                            progress_bar_current_stage.set_fraction((item.images_checked) as f64 / item.images_to_check as f64);
                            taskbar_state.borrow().set_progress_value(
                                (item.images_to_check + item.images_checked) as u64,
                                item.images_to_check as u64 * (item.max_stage + 1) as u64,
                            );
                        } else {
                            progress_bar_all_stages.set_fraction((item.current_stage as f64) / (item.max_stage + 1) as f64);
                            progress_bar_current_stage.set_fraction(0f64);
                            taskbar_state.borrow().set_progress_value(1, (item.max_stage + 1) as u64);
                        }
                        label_stage.set_text(&flg!(
                            "progress_scanning_image",
                            generate_translation_hashmap(vec![("file_checked", item.images_checked.to_string()), ("all_files", item.images_to_check.to_string())])
                        ));
                    }
                    2 => {
                        progress_bar_current_stage.show();
                        if item.images_to_check != 0 {
                            progress_bar_all_stages.set_fraction((2f64 + (item.images_checked) as f64 / item.images_to_check as f64) / (item.max_stage + 1) as f64);
                            progress_bar_current_stage.set_fraction((item.images_checked) as f64 / item.images_to_check as f64);
                            taskbar_state.borrow().set_progress_value(
                                (item.images_to_check + item.images_checked) as u64,
                                item.images_to_check as u64 * (item.max_stage + 1) as u64,
                            );
                        } else {
                            progress_bar_all_stages.set_fraction((item.current_stage as f64) / (item.max_stage + 1) as f64);
                            progress_bar_current_stage.set_fraction(0f64);
                            taskbar_state.borrow().set_progress_value(2, (item.max_stage + 1) as u64);
                        }
                        label_stage.set_text(&flg!(
                            "progress_comparing_image_hashes",
                            generate_translation_hashmap(vec![("file_checked", item.images_checked.to_string()), ("all_files", item.images_to_check.to_string())])
                        ));
                    }
                    _ => {
                        panic!();
                    }
                }
            }
        };
        main_context.spawn_local(future);
    }
    {
        // Similar Videos
        let label_stage = gui_data.progress_window.label_stage.clone();
        let progress_bar_current_stage = gui_data.progress_window.progress_bar_current_stage.clone();
        let progress_bar_all_stages = gui_data.progress_window.progress_bar_all_stages.clone();
        let taskbar_state = gui_data.taskbar_state.clone();
        let future = async move {
            while let Some(item) = futures_receiver_similar_videos.next().await {
                match item.current_stage {
                    0 => {
                        progress_bar_current_stage.hide();
                        label_stage.set_text(&flg!(
                            "progress_scanning_general_file",
                            generate_translation_hashmap(vec![("file_number", item.videos_checked.to_string())])
                        ));
                        taskbar_state.borrow().set_progress_state(TBPF_INDETERMINATE);
                    }
                    1 => {
                        progress_bar_current_stage.show();
                        if item.videos_to_check != 0 {
                            progress_bar_all_stages.set_fraction((1f64 + (item.videos_checked) as f64 / item.videos_to_check as f64) / (item.max_stage + 1) as f64);
                            progress_bar_current_stage.set_fraction((item.videos_checked) as f64 / item.videos_to_check as f64);
                            taskbar_state.borrow().set_progress_value(
                                (item.videos_to_check + item.videos_checked) as u64,
                                item.videos_to_check as u64 * (item.max_stage + 1) as u64,
                            );
                        } else {
                            progress_bar_all_stages.set_fraction((1f64) / (item.max_stage + 1) as f64);
                            progress_bar_current_stage.set_fraction(0f64);
                            taskbar_state.borrow().set_progress_value(1, (item.max_stage + 1) as u64);
                        }
                        label_stage.set_text(&flg!(
                            "progress_scanning_video",
                            generate_translation_hashmap(vec![("file_checked", item.videos_checked.to_string()), ("all_files", item.videos_to_check.to_string())])
                        ));
                    }
                    _ => {
                        panic!();
                    }
                }
            }
        };
        main_context.spawn_local(future);
    }
    {
        // Temporary
        let label_stage = gui_data.progress_window.label_stage.clone();
        let taskbar_state = gui_data.taskbar_state.clone();
        let future = async move {
            while let Some(item) = futures_receiver_temporary.next().await {
                label_stage.set_text(&flg!(
                    "progress_scanning_general_file",
                    generate_translation_hashmap(vec![("file_number", item.files_checked.to_string())])
                ));
                taskbar_state.borrow().set_progress_state(TBPF_INDETERMINATE);
            }
        };
        main_context.spawn_local(future);
    }
    {
        // Invalid Symlinks
        let label_stage = gui_data.progress_window.label_stage.clone();
        let taskbar_state = gui_data.taskbar_state.clone();
        let future = async move {
            while let Some(item) = futures_receiver_invalid_symlinks.next().await {
                label_stage.set_text(&flg!(
                    "progress_scanning_general_file",
                    generate_translation_hashmap(vec![("file_number", item.entries_checked.to_string())])
                ));
                taskbar_state.borrow().set_progress_state(TBPF_INDETERMINATE);
            }
        };
        main_context.spawn_local(future);
    }
    {
        // Broken Files
        let label_stage = gui_data.progress_window.label_stage.clone();
        let progress_bar_current_stage = gui_data.progress_window.progress_bar_current_stage.clone();
        let progress_bar_all_stages = gui_data.progress_window.progress_bar_all_stages.clone();
        let taskbar_state = gui_data.taskbar_state.clone();
        let future = async move {
            while let Some(item) = futures_receiver_broken_files.next().await {
                match item.current_stage {
                    0 => {
                        progress_bar_current_stage.hide();
                        label_stage.set_text(&flg!(
                            "progress_scanning_general_file",
                            generate_translation_hashmap(vec![("file_number", item.files_checked.to_string())])
                        ));
                        taskbar_state.borrow().set_progress_state(TBPF_INDETERMINATE);
                    }
                    1 => {
                        progress_bar_current_stage.show();
                        if item.files_to_check != 0 {
                            progress_bar_all_stages.set_fraction((1f64 + (item.files_checked) as f64 / item.files_to_check as f64) / (item.max_stage + 1) as f64);
                            progress_bar_current_stage.set_fraction((item.files_checked) as f64 / item.files_to_check as f64);
                            taskbar_state
                                .borrow()
                                .set_progress_value((item.files_to_check + item.files_checked) as u64, item.files_to_check as u64 * (item.max_stage + 1) as u64);
                        } else {
                            progress_bar_all_stages.set_fraction((1f64) / (item.max_stage + 1) as f64);
                            progress_bar_current_stage.set_fraction(0f64);
                            taskbar_state.borrow().set_progress_value(1, (item.max_stage + 1) as u64);
                        }
                        label_stage.set_text(&flg!(
                            "progress_scanning_broken_files",
                            generate_translation_hashmap(vec![("file_checked", item.files_checked.to_string()), ("all_files", item.files_to_check.to_string())])
                        ));
                    }
                    _ => {
                        panic!();
                    }
                }
            }
        };
        main_context.spawn_local(future);
    }
}
