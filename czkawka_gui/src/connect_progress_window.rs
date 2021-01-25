use crate::gui_data::GuiData;

use czkawka_core::{big_file, broken_files, duplicate, empty_files, empty_folder, invalid_symlinks, same_music, similar_images, temporary, zeroed};

use futures::StreamExt;
use gtk::{LabelExt, ProgressBarExt, WidgetExt};

#[allow(clippy::too_many_arguments)]
pub fn connect_progress_window(
    gui_data: &GuiData,
    mut futures_receiver_duplicate_files: futures::channel::mpsc::Receiver<duplicate::ProgressData>,
    mut futures_receiver_empty_files: futures::channel::mpsc::Receiver<empty_files::ProgressData>,
    mut futures_receiver_empty_folder: futures::channel::mpsc::Receiver<empty_folder::ProgressData>,
    mut futures_receiver_big_files: futures::channel::mpsc::Receiver<big_file::ProgressData>,
    mut futures_receiver_same_music: futures::channel::mpsc::Receiver<same_music::ProgressData>,
    mut futures_receiver_similar_images: futures::channel::mpsc::Receiver<similar_images::ProgressData>,
    mut futures_receiver_temporary: futures::channel::mpsc::Receiver<temporary::ProgressData>,
    mut futures_receiver_zeroed: futures::channel::mpsc::Receiver<zeroed::ProgressData>,
    mut futures_receiver_invalid_symlinks: futures::channel::mpsc::Receiver<invalid_symlinks::ProgressData>,
    mut futures_receiver_broken_files: futures::channel::mpsc::Receiver<broken_files::ProgressData>,
) {
    let main_context = glib::MainContext::default();

    {
        // Duplicate Files
        let label_stage = gui_data.progress_window.label_stage.clone();
        let progress_bar_current_stage = gui_data.progress_window.progress_bar_current_stage.clone();
        let progress_bar_all_stages = gui_data.progress_window.progress_bar_all_stages.clone();
        let grid_progress_stages = gui_data.progress_window.grid_progress_stages.clone();
        let future = async move {
            while let Some(item) = futures_receiver_duplicate_files.next().await {
                match item.checking_method {
                    duplicate::CheckingMethod::Hash | duplicate::CheckingMethod::HashMB => {
                        label_stage.show();
                        match item.current_stage {
                            // Checking Size
                            0 => {
                                progress_bar_current_stage.hide();
                                // progress_bar_all_stages.hide();
                                progress_bar_all_stages.set_fraction(0 as f64);
                                label_stage.set_text(format!("Scanned size of {} files", item.files_checked).as_str());
                            }
                            // Hash - first 1KB file
                            1 => {
                                progress_bar_current_stage.show();
                                // progress_bar_all_stages.show();
                                if item.files_to_check != 0 {
                                    progress_bar_all_stages.set_fraction((1f64 + (item.files_checked) as f64 / item.files_to_check as f64) / (item.max_stage + 1) as f64);
                                    progress_bar_current_stage.set_fraction((item.files_checked) as f64 / item.files_to_check as f64);
                                } else {
                                    progress_bar_all_stages.set_fraction((1f64) / (item.max_stage + 1) as f64);
                                    progress_bar_current_stage.set_fraction(0f64);
                                }
                                label_stage.set_text(format!("Analyzed partial hash of {}/{} files", item.files_checked, item.files_to_check).as_str());
                            }
                            // Hash - first 1MB of file or normal hash
                            2 => {
                                if item.files_to_check != 0 {
                                    progress_bar_all_stages.set_fraction((2f64 + (item.files_checked) as f64 / item.files_to_check as f64) / (item.max_stage + 1) as f64);
                                    progress_bar_current_stage.set_fraction((item.files_checked) as f64 / item.files_to_check as f64);
                                } else {
                                    progress_bar_all_stages.set_fraction((2f64) / (item.max_stage + 1) as f64);
                                    progress_bar_current_stage.set_fraction(0f64);
                                }
                                label_stage.set_text(format!("Analyzed full hash of {}/{} files", item.files_checked, item.files_to_check).as_str());
                            }
                            _ => {
                                panic!("Not available current_stage");
                            }
                        }
                    }
                    duplicate::CheckingMethod::Name => {
                        label_stage.show();
                        grid_progress_stages.hide();

                        label_stage.set_text(format!("Scanned name of {} files", item.files_checked).as_str());
                    }
                    duplicate::CheckingMethod::Size => {
                        label_stage.show();
                        grid_progress_stages.hide();

                        label_stage.set_text(format!("Scanned size {} files", item.files_checked).as_str());
                    }
                    duplicate::CheckingMethod::None => {
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
        let future = async move {
            while let Some(item) = futures_receiver_empty_files.next().await {
                label_stage.set_text(format!("Scanned {} files", item.files_checked).as_str());
            }
        };
        main_context.spawn_local(future);
    }
    {
        // Empty Folder
        let label_stage = gui_data.progress_window.label_stage.clone();
        let future = async move {
            while let Some(item) = futures_receiver_empty_folder.next().await {
                label_stage.set_text(format!("Scanned {} folders", item.folders_checked).as_str());
            }
        };
        main_context.spawn_local(future);
    }
    {
        // Big Files
        let label_stage = gui_data.progress_window.label_stage.clone();
        let future = async move {
            while let Some(item) = futures_receiver_big_files.next().await {
                label_stage.set_text(format!("Scanned {} files", item.files_checked).as_str());
            }
        };
        main_context.spawn_local(future);
    }
    {
        // Same Music
        let label_stage = gui_data.progress_window.label_stage.clone();
        let progress_bar_current_stage = gui_data.progress_window.progress_bar_current_stage.clone();
        let progress_bar_all_stages = gui_data.progress_window.progress_bar_all_stages.clone();
        let future = async move {
            while let Some(item) = futures_receiver_same_music.next().await {
                match item.current_stage {
                    0 => {
                        progress_bar_current_stage.hide();
                        label_stage.set_text(format!("Scanned {} files", item.music_checked).as_str());
                    }
                    1 => {
                        progress_bar_current_stage.show();
                        if item.music_to_check != 0 {
                            progress_bar_all_stages.set_fraction((1f64 + (item.music_checked) as f64 / item.music_to_check as f64) / (item.max_stage + 1) as f64);
                            progress_bar_current_stage.set_fraction((item.music_checked) as f64 / item.music_to_check as f64);
                        } else {
                            progress_bar_all_stages.set_fraction((1f64) / (item.max_stage + 1) as f64);
                            progress_bar_current_stage.set_fraction(0f64);
                        }
                        label_stage.set_text(format!("Reading tags of {}/{} music files", item.music_checked, item.music_to_check).as_str());
                    }
                    2 => {
                        if item.music_to_check != 0 {
                            progress_bar_all_stages.set_fraction((2f64 + (item.music_checked) as f64 / item.music_to_check as f64) / (item.max_stage + 1) as f64);
                            progress_bar_current_stage.set_fraction((item.music_checked) as f64 / item.music_to_check as f64);
                        } else {
                            progress_bar_all_stages.set_fraction((2f64) / (item.max_stage + 1) as f64);
                            progress_bar_current_stage.set_fraction(0f64);
                        }
                        label_stage.set_text(format!("Checking for duplicates of {}/{} music files", item.music_checked, item.music_to_check).as_str());
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
        let future = async move {
            while let Some(item) = futures_receiver_similar_images.next().await {
                match item.current_stage {
                    0 => {
                        progress_bar_current_stage.hide();
                        label_stage.set_text(format!("Scanned {} files", item.images_checked).as_str());
                    }
                    1 => {
                        progress_bar_current_stage.show();
                        if item.images_to_check != 0 {
                            progress_bar_all_stages.set_fraction((1f64 + (item.images_checked) as f64 / item.images_to_check as f64) / (item.max_stage + 1) as f64);
                            progress_bar_current_stage.set_fraction((item.images_checked) as f64 / item.images_to_check as f64);
                        } else {
                            progress_bar_all_stages.set_fraction((1f64) / (item.max_stage + 1) as f64);
                            progress_bar_current_stage.set_fraction(0f64);
                        }
                        label_stage.set_text(format!("Hashing {}/{} image", item.images_checked, item.images_to_check).as_str());
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
        let future = async move {
            while let Some(item) = futures_receiver_temporary.next().await {
                label_stage.set_text(format!("Scanned {} files", item.files_checked).as_str());
            }
        };
        main_context.spawn_local(future);
    }
    {
        // Zeroed Files
        let label_stage = gui_data.progress_window.label_stage.clone();
        let progress_bar_current_stage = gui_data.progress_window.progress_bar_current_stage.clone();
        let progress_bar_all_stages = gui_data.progress_window.progress_bar_all_stages.clone();
        let future = async move {
            while let Some(item) = futures_receiver_zeroed.next().await {
                match item.current_stage {
                    0 => {
                        progress_bar_current_stage.hide();
                        label_stage.set_text(format!("Scanned {} files", item.files_checked).as_str());
                    }
                    1 => {
                        progress_bar_current_stage.show();
                        if item.files_to_check != 0 {
                            progress_bar_all_stages.set_fraction((1f64 + (item.files_checked) as f64 / item.files_to_check as f64) / (item.max_stage + 1) as f64);
                            progress_bar_current_stage.set_fraction((item.files_checked) as f64 / item.files_to_check as f64);
                        } else {
                            progress_bar_all_stages.set_fraction((1f64) / (item.max_stage + 1) as f64);
                            progress_bar_current_stage.set_fraction(0f64);
                        }
                        label_stage.set_text(format!("Checking {}/{} file", item.files_checked, item.files_to_check).as_str());
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
        // Invalid Symlinks
        let label_stage = gui_data.progress_window.label_stage.clone();
        let future = async move {
            while let Some(item) = futures_receiver_invalid_symlinks.next().await {
                label_stage.set_text(format!("Scanned {} files", item.files_checked).as_str());
            }
        };
        main_context.spawn_local(future);
    }
    {
        // Broken Files
        let label_stage = gui_data.progress_window.label_stage.clone();
        let progress_bar_current_stage = gui_data.progress_window.progress_bar_current_stage.clone();
        let progress_bar_all_stages = gui_data.progress_window.progress_bar_all_stages.clone();
        let future = async move {
            while let Some(item) = futures_receiver_broken_files.next().await {
                match item.current_stage {
                    0 => {
                        progress_bar_current_stage.hide();
                        label_stage.set_text(format!("Scanned {} files", item.files_checked).as_str());
                    }
                    1 => {
                        progress_bar_current_stage.show();
                        if item.files_to_check != 0 {
                            progress_bar_all_stages.set_fraction((1f64 + (item.files_checked) as f64 / item.files_to_check as f64) / (item.max_stage + 1) as f64);
                            progress_bar_current_stage.set_fraction((item.files_checked) as f64 / item.files_to_check as f64);
                        } else {
                            progress_bar_all_stages.set_fraction((1f64) / (item.max_stage + 1) as f64);
                            progress_bar_current_stage.set_fraction(0f64);
                        }
                        label_stage.set_text(format!("Checking {}/{} files", item.files_checked, item.files_to_check).as_str());
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
