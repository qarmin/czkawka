// Remove console window in Windows OS
#![windows_subsystem = "windows"]
#![allow(clippy::collapsible_else_if)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]

use gtk::prelude::*;

use czkawka_core::*;

use crate::compute_results::*;
use crate::initialize_gui::*;
use crate::language_functions::LANGUAGES_ALL;
use crate::saving_loading::*;
use crate::tests::validate_notebook_data;
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
use connect_things::connect_popovers::*;
use connect_things::connect_progress_window::*;
use connect_things::connect_selection_of_directories::*;
use connect_things::connect_settings::*;
use connect_things::connect_show_hide_ui::*;
use connect_things::connect_similar_image_size_change::*;
use gui_structs::gui_data::*;

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
mod opening_selecting_records;
mod saving_loading;
mod taskbar_progress;
#[cfg(not(target_os = "windows"))]
mod taskbar_progress_dummy;
#[cfg(target_os = "windows")]
mod taskbar_progress_win;
mod tests;

fn main() {
    let application = gtk::Application::builder().build();
    application.connect_activate(move |application| {
        let mut gui_data: GuiData = GuiData::new_with_application(application);

        // Used for getting data from thread
        let (glib_stop_sender, glib_stop_receiver) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

        // Futures progress report
        let (futures_sender_duplicate_files, futures_receiver_duplicate_files): (
            futures::channel::mpsc::UnboundedSender<common_dir_traversal::ProgressData>,
            futures::channel::mpsc::UnboundedReceiver<common_dir_traversal::ProgressData>,
        ) = futures::channel::mpsc::unbounded();
        let (futures_sender_empty_files, futures_receiver_empty_files): (
            futures::channel::mpsc::UnboundedSender<common_dir_traversal::ProgressData>,
            futures::channel::mpsc::UnboundedReceiver<common_dir_traversal::ProgressData>,
        ) = futures::channel::mpsc::unbounded();
        let (futures_sender_empty_folder, futures_receiver_empty_folder): (
            futures::channel::mpsc::UnboundedSender<common_dir_traversal::ProgressData>,
            futures::channel::mpsc::UnboundedReceiver<common_dir_traversal::ProgressData>,
        ) = futures::channel::mpsc::unbounded();
        let (futures_sender_big_file, futures_receiver_big_file): (
            futures::channel::mpsc::UnboundedSender<big_file::ProgressData>,
            futures::channel::mpsc::UnboundedReceiver<big_file::ProgressData>,
        ) = futures::channel::mpsc::unbounded();
        let (futures_sender_same_music, futures_receiver_same_music): (
            futures::channel::mpsc::UnboundedSender<common_dir_traversal::ProgressData>,
            futures::channel::mpsc::UnboundedReceiver<common_dir_traversal::ProgressData>,
        ) = futures::channel::mpsc::unbounded();
        let (futures_sender_similar_images, futures_receiver_similar_images): (
            futures::channel::mpsc::UnboundedSender<similar_images::ProgressData>,
            futures::channel::mpsc::UnboundedReceiver<similar_images::ProgressData>,
        ) = futures::channel::mpsc::unbounded();
        let (futures_sender_similar_videos, futures_receiver_similar_videos): (
            futures::channel::mpsc::UnboundedSender<similar_videos::ProgressData>,
            futures::channel::mpsc::UnboundedReceiver<similar_videos::ProgressData>,
        ) = futures::channel::mpsc::unbounded();
        let (futures_sender_temporary, futures_receiver_temporary): (
            futures::channel::mpsc::UnboundedSender<temporary::ProgressData>,
            futures::channel::mpsc::UnboundedReceiver<temporary::ProgressData>,
        ) = futures::channel::mpsc::unbounded();
        let (futures_sender_invalid_symlinks, futures_receiver_invalid_symlinks): (
            futures::channel::mpsc::UnboundedSender<common_dir_traversal::ProgressData>,
            futures::channel::mpsc::UnboundedReceiver<common_dir_traversal::ProgressData>,
        ) = futures::channel::mpsc::unbounded();
        let (futures_sender_broken_files, futures_receiver_broken_files): (
            futures::channel::mpsc::UnboundedSender<broken_files::ProgressData>,
            futures::channel::mpsc::UnboundedReceiver<broken_files::ProgressData>,
        ) = futures::channel::mpsc::unbounded();

        initialize_gui(&mut gui_data);
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
        );

        // Needs to run when entire GUI is initialized
        connect_change_language(&gui_data);

        connect_button_delete(&gui_data);
        connect_button_save(&gui_data);
        connect_button_search(
            &gui_data,
            glib_stop_sender,
            futures_sender_duplicate_files,
            futures_sender_empty_files,
            futures_sender_empty_folder,
            futures_sender_big_file,
            futures_sender_same_music,
            futures_sender_similar_images,
            futures_sender_similar_videos,
            futures_sender_temporary,
            futures_sender_invalid_symlinks,
            futures_sender_broken_files,
        );
        connect_button_select(&gui_data);
        connect_button_stop(&gui_data);
        connect_button_hardlink_symlink(&gui_data);
        connect_button_move(&gui_data);
        connect_button_compare(&gui_data);

        connect_duplicate_combo_box(&gui_data);
        connect_notebook_tabs(&gui_data);
        connect_selection_of_directories(&gui_data);
        connect_popovers(&gui_data);
        connect_compute_results(&gui_data, glib_stop_receiver);
        connect_progress_window(
            &gui_data,
            futures_receiver_duplicate_files,
            futures_receiver_empty_files,
            futures_receiver_empty_folder,
            futures_receiver_big_file,
            futures_receiver_same_music,
            futures_receiver_similar_images,
            futures_receiver_similar_videos,
            futures_receiver_temporary,
            futures_receiver_invalid_symlinks,
            futures_receiver_broken_files,
        );
        connect_show_hide_ui(&gui_data);
        connect_settings(&gui_data);
        connect_button_about(&gui_data);
        connect_about_buttons(&gui_data);
        connect_similar_image_size_change(&gui_data);

        let window_main = gui_data.window_main.clone();
        let taskbar_state = gui_data.taskbar_state.clone();
        window_main.connect_delete_event(move |_, _| {
            save_configuration(false, &gui_data.upper_notebook, &gui_data.main_notebook, &gui_data.settings, &gui_data.text_view_errors); // Save configuration at exit
            taskbar_state.borrow_mut().release();
            Inhibit(false)
        });
    });

    application.run();
}
