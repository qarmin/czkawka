// Remove console window in Windows OS
#![windows_subsystem = "windows"]
#![allow(clippy::indexing_slicing)] // Too much used, to be able to ignore it in every place

use std::env;

use connect_things::connect_about_buttons::connect_about_buttons;
use connect_things::connect_button_compare::connect_button_compare;
use connect_things::connect_button_delete::connect_button_delete;
use connect_things::connect_button_hardlink::connect_button_hardlink_symlink;
use connect_things::connect_button_move::connect_button_move;
use connect_things::connect_button_save::connect_button_save;
use connect_things::connect_button_search::connect_button_search;
use connect_things::connect_button_select::connect_button_select;
use connect_things::connect_button_stop::connect_button_stop;
use connect_things::connect_change_language::{connect_change_language, load_system_language};
use connect_things::connect_duplicate_buttons::connect_duplicate_combo_box;
use connect_things::connect_header_buttons::connect_button_about;
use connect_things::connect_notebook_tabs::connect_notebook_tabs;
use connect_things::connect_progress_window::connect_progress_window;
use connect_things::connect_selection_of_directories::connect_selection_of_directories;
use connect_things::connect_settings::connect_settings;
use connect_things::connect_show_hide_ui::connect_show_hide_ui;
use connect_things::connect_similar_image_size_change::connect_similar_image_size_change;
use crossbeam_channel::{Receiver, Sender, unbounded};
use czkawka_core::common::config_cache_path::{print_infos_and_warnings, set_config_cache_path};
use czkawka_core::common::logger::{filtering_messages, print_version_mode, setup_logger};
use czkawka_core::common::progress_data::ProgressData;
use czkawka_core::common::{get_number_of_threads, set_number_of_threads};
use czkawka_core::localizer_core;
use glib::ExitCode;
use gtk4::Application;
use gtk4::gio::ApplicationFlags;
use gtk4::prelude::*;
use gui_structs::gui_data::{
    CZK_ICON_ADD, CZK_ICON_COMPARE, CZK_ICON_DELETE, CZK_ICON_HARDLINK, CZK_ICON_HIDE_DOWN, CZK_ICON_HIDE_UP, CZK_ICON_INFO, CZK_ICON_LEFT, CZK_ICON_MANUAL_ADD, CZK_ICON_MOVE,
    CZK_ICON_RIGHT, CZK_ICON_SAVE, CZK_ICON_SEARCH, CZK_ICON_SELECT, CZK_ICON_SETTINGS, CZK_ICON_STOP, CZK_ICON_SYMLINK, CZK_ICON_TRASH, GuiData,
};
use log::info;

use crate::cli::{CliResult, process_cli_args};
use crate::compute_results::connect_compute_results;
use crate::connect_things::connect_button_sort::connect_button_sort;
use crate::connect_things::connect_popovers_select::connect_popover_select;
use crate::connect_things::connect_popovers_sort::connect_popover_sort;
use crate::connect_things::connect_same_music_mode_changed::connect_same_music_change_mode;
use crate::initialize_gui::initialize_gui;
use crate::language_functions::LANGUAGES_ALL;
use crate::saving_loading::{DEFAULT_MAXIMAL_FILE_SIZE, DEFAULT_MINIMAL_CACHE_SIZE, DEFAULT_MINIMAL_FILE_SIZE, load_configuration, reset_configuration, save_configuration};

mod cli;
mod compute_results;
mod connect_things;
mod create_tree_view;
mod dicom_traits;
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

fn main() {
    let (infos, warnings) = set_config_cache_path("Czkawka", "Czkawka");
    setup_logger(false, "czkawka_gui", filtering_messages);
    print_version_mode("Czkawka gtk");
    print_infos_and_warnings(infos, warnings);

    let application = Application::new(None::<String>, ApplicationFlags::HANDLES_OPEN | ApplicationFlags::HANDLES_COMMAND_LINE);

    #[cfg(target_os = "linux")]
    glib::set_prgname(Some("com.github.qarmin.czkawka"));

    application.connect_command_line(move |app, cmdline| {
        let cli_args = process_cli_args(cmdline.arguments().into_iter().skip(1).map(|x| x.to_string_lossy().to_string()).collect());
        build_ui(app, cli_args.as_ref());
        ExitCode::new(0)
    });
    application.run_with_args(&env::args().collect::<Vec<_>>());
}

fn build_ui(application: &Application, cli_args: Option<&CliResult>) {
    let gui_data: GuiData = GuiData::new_with_application(application);
    gui_data.setup();

    let (result_sender, result_receiver) = unbounded();

    // Futures progress report
    let (progress_sender, progress_receiver): (Sender<ProgressData>, Receiver<ProgressData>) = unbounded();

    initialize_gui(&gui_data);
    reset_configuration(false, &gui_data.upper_notebook, &gui_data.main_notebook, &gui_data.settings, &gui_data.text_view_errors); // Fallback for invalid loading setting project
    load_system_language(&gui_data); // Check for default system language, must be loaded after initializing GUI and before loading settings from file
    load_configuration(
        false,
        &gui_data.upper_notebook,
        &gui_data.main_notebook,
        &gui_data.settings,
        &gui_data.text_view_errors,
        &gui_data.scrolled_window_errors,
        cli_args,
    );
    set_number_of_threads(gui_data.settings.scale_settings_number_of_threads.value().round() as usize);

    print_czkawka_gui_info(get_number_of_threads());

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
    let used_additional_arguments = cli_args.is_some();
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

pub(crate) fn print_czkawka_gui_info(thread_number: usize) {
    let gtk_version = format!("{}.{}.{}", gtk4::major_version(), gtk4::minor_version(), gtk4::micro_version());

    info!("Czkawka Gui - used thread number: {thread_number}, gtk version {gtk_version}");
}
