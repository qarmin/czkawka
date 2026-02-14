// Remove console window in Windows OS
#![windows_subsystem = "windows"]
#![allow(clippy::unwrap_used)] // Cannot use due unwrap used in a lot of places in generated code
#![allow(clippy::indexing_slicing)] // Cannot use due unwrap used in a lot of places in generated code
#![allow(clippy::todo)] // Cannot use due unwrap used in a lot of places in generated code

use std::rc::Rc;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use crossbeam_channel::{Receiver, Sender, unbounded};
use czkawka_core::common::basic_gui_cli::process_cli_args;
use czkawka_core::common::config_cache_path::{print_infos_and_warnings, set_config_cache_path};
use czkawka_core::common::image::register_image_decoding_hooks;
use czkawka_core::common::logger::{filtering_messages, print_version_mode, setup_logger};
use czkawka_core::common::progress_data::ProgressData;
use file_actions::connect_clean_exif::connect_clean;
use file_actions::connect_delete::connect_delete_button;
use file_actions::connect_hardlink::connect_hardlink;
use file_actions::connect_move::connect_move;
use file_actions::connect_optimize_video::connect_optimize_video;
use file_actions::connect_rename::connect_rename;
use file_actions::connect_symlink::connect_symlink;
use log::{error, info};
use slint::VecModel;

use crate::clear_outdated_video_thumbnails::clear_outdated_video_thumbnails;
use crate::connect_clean_cache::connect_clean_cache;
use crate::connect_directories_changes::connect_add_remove_directories;
use crate::connect_open::connect_open_items;
use crate::connect_progress_receiver::connect_progress_gathering;
use crate::connect_row_selection::connect_row_selections;
use crate::connect_save::connect_save;
use crate::connect_scan::connect_scan_button;
use crate::connect_select::connect_select;
use crate::connect_show_confirmation::connect_show_confirmation;
use crate::connect_show_preview::connect_show_preview;
use crate::connect_sort::{connect_sort, connect_sort_column};
use crate::connect_stop::connect_stop_button;
use crate::connect_tab_changed::connect_tab_changed;
use crate::connect_translation::connect_translations;
use crate::create_calculate_task_size::create_calculate_task_size;
use crate::set_initial_gui_info::set_initial_gui_infos;
use crate::set_initial_scroll_list_data_indexes::set_initial_scroll_list_data_indexes;
// TODO - at start this should be used, to be sure that rust models are in sync with slint models
// currently I need to do this manually - https://github.com/slint-ui/slint/issues/7632
// use crate::set_initial_gui_info::set_initial_gui_infos;
use crate::settings::{connect_changing_settings_preset, create_default_settings_files, load_initial_settings_from_file, save_all_settings_to_file, set_initial_settings_to_gui};
use crate::shared_models::SharedModels;

mod audio_player;
mod clear_outdated_video_thumbnails;
mod common;
mod connect_clean_cache;
mod connect_directories_changes;
mod connect_open;
mod connect_progress_receiver;
mod connect_row_selection;
mod connect_save;
mod connect_scan;
mod connect_select;
mod connect_show_confirmation;
mod connect_show_preview;
mod connect_sort;
mod connect_stop;
mod connect_tab_changed;
mod connect_translation;
mod create_calculate_task_size;
mod file_actions;
mod localizer_krokiet;
mod model_operations;
mod set_initial_gui_info;
mod set_initial_scroll_list_data_indexes;
mod settings;
mod shared_models;
mod simpler_model;
#[cfg(test)]
mod test_common;

slint::include_modules!();

fn main() {
    register_image_decoding_hooks();
    let config_cache_path_set_result = set_config_cache_path("Czkawka", "Krokiet");
    let cli_args = process_cli_args("Krokiet", "krokiet_gui", std::env::args().skip(1).collect());

    let (base_settings, custom_settings, preset_to_load) = load_initial_settings_from_file(cli_args.as_ref());
    if base_settings.use_manual_application_scale {
        // SAFETY:
        // set_var is safe when using on single threaded context
        unsafe {
            std::env::set_var("SLINT_SCALE_FACTOR", format!("{:.2}", base_settings.manual_application_scale));
        }
    }

    setup_logger(false, "krokiet", filtering_messages);
    print_version_mode("Krokiet");
    print_infos_and_warnings(config_cache_path_set_result.infos, config_cache_path_set_result.warnings);
    print_krokiet_features();

    create_default_settings_files();

    let app = match MainWindow::new() {
        Ok(app) => app,
        Err(e) => {
            error!("Error during creating main window: {e}");
            show_critical_error(e.to_string());
            return;
        }
    };

    #[cfg(feature = "audio")]
    app.global::<GuiState>().set_audio_feature_enabled(true);

    let (progress_sender, progress_receiver): (Sender<ProgressData>, Receiver<ProgressData>) = unbounded();
    let stop_flag: Arc<AtomicBool> = Arc::default();

    zeroing_all_models(&app);

    let shared_models = SharedModels::new_shared();

    // Create audio player for scan completion notifications
    let audio_player = Arc::new(crate::audio_player::AudioPlayer::new());

    // Disabled for now, due invalid settings model at start
    set_initial_gui_infos(&app);

    set_initial_scroll_list_data_indexes(&app);

    let original_preset_idx = base_settings.default_preset;
    set_initial_settings_to_gui(&app, &base_settings, &custom_settings, cli_args, preset_to_load);

    connect_delete_button(&app, progress_sender.clone(), stop_flag.clone());
    connect_scan_button(&app, progress_sender.clone(), stop_flag.clone(), Arc::clone(&shared_models), Arc::clone(&audio_player));
    connect_stop_button(&app, stop_flag.clone());
    connect_open_items(&app);
    connect_progress_gathering(&app, progress_receiver);
    connect_add_remove_directories(&app);
    connect_show_preview(&app, Arc::clone(&shared_models));
    connect_translations(&app);
    connect_changing_settings_preset(&app);
    connect_select(&app);
    connect_move(&app, progress_sender.clone(), stop_flag.clone());
    connect_rename(&app, progress_sender.clone(), stop_flag.clone());
    connect_optimize_video(&app, progress_sender.clone(), stop_flag.clone());
    connect_clean(&app, progress_sender.clone(), stop_flag.clone());
    connect_hardlink(&app, progress_sender.clone(), stop_flag.clone());
    connect_symlink(&app, progress_sender, stop_flag);
    connect_save(&app, Arc::clone(&shared_models));
    connect_row_selections(&app);
    connect_sort(&app);
    connect_sort_column(&app);
    let (task_sender, task_receiver) = std::sync::mpsc::channel();
    connect_tab_changed(&app, task_sender.clone());
    create_calculate_task_size(task_receiver);
    connect_clean_cache(&app, task_sender);
    connect_show_confirmation(&app, Arc::clone(&shared_models));

    clear_outdated_video_thumbnails(&app);

    // Popups gather their size, after starting/closing popup at least once
    // This is simpler solution, than setting sizes of popups manually for each language
    app.invoke_initialize_popup_sizes();

    match app.run() {
        Ok(()) => {
            save_all_settings_to_file(&app, original_preset_idx);
        }
        Err(e) => {
            error!("Error during running the application: {e}");
            show_critical_error(e.to_string());
        }
    }
}

pub fn show_critical_error(error: String) {
    rfd::MessageDialog::new()
        .set_title(flk!("rust_init_error_title"))
        .set_description(&flk!("rust_init_error_message", error_message = error))
        .show();
}

pub(crate) fn zeroing_all_models(app: &MainWindow) {
    app.set_empty_folder_model(Rc::new(VecModel::default()).into());
    app.set_empty_files_model(Rc::new(VecModel::default()).into());
    app.set_similar_images_model(Rc::new(VecModel::default()).into());
    app.set_duplicate_files_model(Rc::new(VecModel::default()).into());
    app.set_similar_music_model(Rc::new(VecModel::default()).into());
    app.set_big_files_model(Rc::new(VecModel::default()).into());
    app.set_bad_extensions_model(Rc::new(VecModel::default()).into());
    app.set_bad_names_model(Rc::new(VecModel::default()).into());
    app.set_broken_files_model(Rc::new(VecModel::default()).into());
    app.set_similar_videos_model(Rc::new(VecModel::default()).into());
    app.set_invalid_symlinks_model(Rc::new(VecModel::default()).into());
    app.set_temporary_files_model(Rc::new(VecModel::default()).into());
    app.set_video_optimizer_model(Rc::new(VecModel::default()).into());
}

#[allow(clippy::allow_attributes)]
#[allow(unfulfilled_lint_expectations)] // Happens only on release build
#[expect(clippy::vec_init_then_push)]
#[expect(unused_mut)]
pub(crate) fn print_krokiet_features() {
    let mut features: Vec<&str> = Vec::new();

    #[cfg(feature = "audio")]
    features.push("audio");
    #[cfg(feature = "skia_opengl")]
    features.push("skia_opengl");
    #[cfg(feature = "skia_vulkan")]
    features.push("skia_vulkan");
    #[cfg(feature = "software")]
    features.push("software");
    #[cfg(feature = "femtovg")]
    features.push("femtovg");
    #[cfg(feature = "winit_femtovg")]
    features.push("winit_femtovg");
    #[cfg(feature = "winit_skia_opengl")]
    features.push("winit_skia_opengl");
    #[cfg(feature = "winit_skia_vulkan")]
    features.push("winit_skia_vulkan");
    #[cfg(feature = "winit_software")]
    features.push("winit_software");

    info!("Krokiet features({}): [{}]", features.len(), features.join(", "));
}
