// Remove console window in Windows OS
#![windows_subsystem = "windows"]
#![allow(unknown_lints)] // May be disabled, but locally I use nightly clippy
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_else_if)]
// #![warn(clippy::unwrap_used)] // Cannot use due unwrap used in a lot of places in generated code
#![warn(clippy::print_stderr)]
#![warn(clippy::print_stdout)]
#![warn(clippy::dbg_macro)]

use std::rc::Rc;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use crossbeam_channel::{Receiver, Sender, unbounded};
use czkawka_core::common::{print_infos_and_warnings, print_version_mode, set_config_cache_path, setup_logger};
use czkawka_core::progress_data::ProgressData;
use log::{info, warn};
use slint::VecModel;

use crate::connect_delete::connect_delete_button;
use crate::connect_directories_changes::connect_add_remove_directories;
use crate::connect_move::connect_move;
use crate::connect_open::connect_open_items;
use crate::connect_progress_receiver::connect_progress_gathering;
use crate::connect_rename::connect_rename;
use crate::connect_row_selection::connect_row_selections;
use crate::connect_save::connect_save;
use crate::connect_scan::connect_scan_button;
use crate::connect_select::{connect_select, connect_showing_proper_select_buttons};
use crate::connect_show_preview::connect_show_preview;
use crate::connect_sort::connect_sort;
use crate::connect_stop::connect_stop_button;
use crate::connect_translation::connect_translations;
// TODO - at start this should be used, to be sure that rust models are in sync with slint models
// currently I need to do this manually - https://github.com/slint-ui/slint/issues/7632
// use crate::set_initial_gui_info::set_initial_gui_infos;
use crate::settings::{connect_changing_settings_preset, create_default_settings_files, load_settings_from_file, save_all_settings_to_file};
use crate::shared_models::SharedModels;

mod common;
mod connect_delete;
mod connect_directories_changes;
mod connect_move;
mod connect_open;
mod connect_progress_receiver;
mod connect_rename;
mod connect_row_selection;
mod connect_save;
mod connect_scan;
mod connect_select;
mod connect_show_preview;
mod connect_sort;
mod connect_stop;
mod connect_translation;
mod localizer_krokiet;
mod model_operations;
mod set_initial_gui_info;
mod settings;
mod shared_models;
mod simpler_model;
#[cfg(test)]
mod test_common;

slint::include_modules!();
fn main() {
    let (infos, warnings) = set_config_cache_path("Czkawka", "Krokiet");
    setup_logger(false, "krokiet");
    print_version_mode("Krokiet");
    print_infos_and_warnings(infos, warnings);
    print_krokiet_features();

    let app = MainWindow::new().expect("Failed to create main window");

    let (progress_sender, progress_receiver): (Sender<ProgressData>, Receiver<ProgressData>) = unbounded();
    let stop_flag: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));

    zeroing_all_models(&app);

    let shared_models = SharedModels::new_shared();

    // Disabled for now, due invalid settings model at start
    // set_initial_gui_infos(&app);

    create_default_settings_files();
    load_settings_from_file(&app);

    connect_delete_button(&app, progress_sender.clone());
    connect_scan_button(&app, progress_sender, stop_flag.clone(), Arc::clone(&shared_models));
    connect_stop_button(&app, stop_flag);
    connect_open_items(&app);
    connect_progress_gathering(&app, progress_receiver);
    connect_add_remove_directories(&app);
    connect_show_preview(&app);
    connect_translations(&app);
    connect_changing_settings_preset(&app);
    connect_select(&app);
    connect_showing_proper_select_buttons(&app);
    connect_move(&app);
    connect_rename(&app);
    connect_save(&app, Arc::clone(&shared_models));
    connect_row_selections(&app);
    connect_sort(&app);

    // Popups gather their size, after starting/closing popup at least once
    // This is simpler solution, than setting sizes of popups manually for each language
    app.invoke_initialize_popup_sizes();

    app.run().expect("Failed to run app :(");

    save_all_settings_to_file(&app);
}

pub fn zeroing_all_models(app: &MainWindow) {
    app.set_empty_folder_model(Rc::new(VecModel::default()).into());
    app.set_empty_files_model(Rc::new(VecModel::default()).into());
    app.set_similar_images_model(Rc::new(VecModel::default()).into());
    app.set_duplicate_files_model(Rc::new(VecModel::default()).into());
    app.set_similar_music_model(Rc::new(VecModel::default()).into());
    app.set_big_files_model(Rc::new(VecModel::default()).into());
    app.set_bad_extensions_model(Rc::new(VecModel::default()).into());
    app.set_broken_files_model(Rc::new(VecModel::default()).into());
    app.set_similar_videos_model(Rc::new(VecModel::default()).into());
    app.set_invalid_symlinks_model(Rc::new(VecModel::default()).into());
    app.set_temporary_files_model(Rc::new(VecModel::default()).into());
}

#[allow(clippy::vec_init_then_push)]
#[allow(unused_mut)]
pub fn print_krokiet_features() {
    let mut features: Vec<&str> = vec![];

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
