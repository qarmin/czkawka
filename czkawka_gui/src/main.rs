// Remove console window in Windows OS
#![windows_subsystem = "windows"]
#![allow(clippy::collapsible_else_if)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_late_init)]
#![warn(clippy::unwrap_used)]

use std::env;
use std::ffi::OsString;

use crossbeam_channel::{unbounded, Receiver, Sender};
use gtk4::gio::ApplicationFlags;
use gtk4::prelude::*;
use gtk4::Application;
use log::info;

use connect_things::connect_about_buttons::*;
use connect_things::connect_button_compare::*;
use connect_things::connect_button_delete::*;
use connect_things::connect_button_hardlink::*;
use connect_things::connect_button_move::*;
use connect_things::connect_button_save::*;
use connect_things::connect_button_search::*;
use connect_things::connect_button_select::*;
use connect_things::connect_button_stop::*;
use connect_things::connect_change_language::*;
use connect_things::connect_duplicate_buttons::connect_duplicate_combo_box;
use connect_things::connect_header_buttons::*;
use connect_things::connect_notebook_tabs::*;
use connect_things::connect_progress_window::*;
use connect_things::connect_selection_of_directories::*;
use connect_things::connect_settings::*;
use connect_things::connect_show_hide_ui::*;
use connect_things::connect_similar_image_size_change::*;
use czkawka_core::common::{get_number_of_threads, print_version_mode, set_number_of_threads, setup_logger};
use czkawka_core::progress_data::ProgressData;
use czkawka_core::*;
use gui_structs::gui_data::*;

use crate::compute_results::*;
use crate::connect_things::connect_button_sort::connect_button_sort;
use crate::connect_things::connect_popovers_select::connect_popover_select;
use crate::connect_things::connect_popovers_sort::connect_popover_sort;
use crate::connect_things::connect_same_music_mode_changed::connect_same_music_change_mode;
use crate::initialize_gui::*;
use crate::language_functions::LANGUAGES_ALL;
use crate::saving_loading::*;
use crate::tests::validate_notebook_data;

mod compute_results;
mod connect_things;
mod create_tree_view;
mod gui_structs;
mod help_combo_box;
mod help_functions;
mod initialize_gui;
mod language_functions;
mod localizer_gui;
mod notebook_enums;
mod notebook_info;
mod opening_selecting_records;
mod saving_loading;
mod taskbar_progress;
#[cfg(not(target_os = "windows"))]
mod taskbar_progress_dummy;
#[cfg(target_os = "windows")]
mod taskbar_progress_win;
mod tests;

fn main() {
    let application = Application::new(None::<String>, ApplicationFlags::HANDLES_OPEN | ApplicationFlags::HANDLES_COMMAND_LINE);
    application.connect_command_line(move |app, cmdline| {
        setup_logger(false);
        print_version_mode();
        build_ui(app, &cmdline.arguments());
        0
    });
    application.run_with_args(&env::args().collect::<Vec<_>>());
}

fn build_ui(application: &Application, arguments: &[OsString]) {
    let gui_data: GuiData = GuiData::new_with_application(application);

    let (result_sender, result_receiver) = unbounded();

    // Futures progress report
    let (progress_sender, progress_receiver): (Sender<ProgressData>, Receiver<ProgressData>) = unbounded();

    initialize_gui(&gui_data);
    validate_notebook_data(&gui_data); // Must be run after initialization of gui, to check if everything was properly setup
    reset_configuration(false, &gui_data.upper_notebook, &gui_data.main_notebook, &gui_data.settings, &gui_data.text_view_errors); // Fallback for invalid loading setting project
    load_system_language(&gui_data); // Check for default system language, must be loaded after initializing GUI and before loading settings from file
    load_configuration(
        false,
        &gui_data.upper_notebook,
        &gui_data.main_notebook,
        &gui_data.settings,
        &gui_data.text_view_errors,
        &gui_data.scrolled_window_errors,
        arguments,
    );
    set_number_of_threads(gui_data.settings.scale_settings_number_of_threads.value().round() as usize);
    info!("Set thread number to {}", get_number_of_threads());

    // Needs to run when entire GUI is initialized
    connect_change_language(&gui_data);

    connect_button_delete(&gui_data);
    connect_button_save(&gui_data);
    connect_button_search(&gui_data, result_sender, progress_sender);
    connect_button_select(&gui_data);
    connect_button_sort(&gui_data);
    connect_button_stop(&gui_data);
    connect_button_hardlink_symlink(&gui_data);
    connect_button_move(&gui_data);
    connect_button_compare(&gui_data);

    connect_duplicate_combo_box(&gui_data);
    connect_notebook_tabs(&gui_data);
    connect_selection_of_directories(&gui_data);
    connect_popover_select(&gui_data);
    connect_popover_sort(&gui_data);
    connect_compute_results(&gui_data, result_receiver);
    connect_progress_window(&gui_data, progress_receiver);
    connect_show_hide_ui(&gui_data);
    connect_settings(&gui_data);
    connect_button_about(&gui_data);
    connect_about_buttons(&gui_data);
    connect_similar_image_size_change(&gui_data);
    connect_same_music_change_mode(&gui_data);

    let window_main = gui_data.window_main.clone();
    let taskbar_state = gui_data.taskbar_state.clone();
    let used_additional_arguments = arguments.len() > 1;
    window_main.connect_close_request(move |_| {
        // Not save configuration when using non default arguments
        if !used_additional_arguments {
            save_configuration(false, &gui_data.upper_notebook, &gui_data.main_notebook, &gui_data.settings, &gui_data.text_view_errors);
            // Save configuration at exit
        }
        taskbar_state.borrow_mut().release();
        glib::Propagation::Proceed
    });
}
