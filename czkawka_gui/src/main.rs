// Remove console window in Windows OS
#![windows_subsystem = "windows"]
#![allow(clippy::collapsible_else_if)]
#![allow(clippy::too_many_arguments)]

use gtk::prelude::*;

use czkawka_core::*;

use crate::compute_results::*;
use crate::connect_about_buttons::*;
use crate::connect_button_delete::*;
use crate::connect_button_hardlink::*;
use crate::connect_button_move::*;
use crate::connect_button_save::*;
use crate::connect_button_search::*;
use crate::connect_button_select::*;
use crate::connect_button_stop::*;
use crate::connect_change_language::*;
use crate::connect_duplicate_buttons::*;
use crate::connect_header_buttons::*;
use crate::connect_notebook_tabs::*;
use crate::connect_popovers::*;
use crate::connect_progress_window::*;
use crate::connect_selection_of_directories::*;
use crate::connect_settings::*;
use crate::connect_show_hide_ui::*;
use crate::connect_similar_image_size_change::*;
use crate::gui_data::*;
use crate::initialize_gui::*;
use crate::saving_loading::*;
use crate::tests::validate_notebook_data;

mod compute_results;
mod connect_about_buttons;
mod connect_button_delete;
mod connect_button_hardlink;
mod connect_button_move;
mod connect_button_save;
mod connect_button_search;
mod connect_button_select;
mod connect_button_stop;
mod connect_change_language;
mod connect_duplicate_buttons;
mod connect_header_buttons;
mod connect_notebook_tabs;
mod connect_popovers;
mod connect_progress_window;
mod connect_selection_of_directories;
mod connect_settings;
mod connect_show_hide_ui;
mod connect_similar_image_size_change;
mod create_tree_view;
mod gui_about;
mod gui_bottom_buttons;
mod gui_data;
mod gui_header;
mod gui_main_notebook;
mod gui_popovers;
mod gui_progress_dialog;
mod gui_settings;
mod gui_upper_notebook;
mod help_functions;
mod initialize_gui;
mod language_functions;
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
    let application = gtk::Application::builder().application_id("com.github.qarmin").build();
    application.connect_activate(|application| {
        let mut gui_data: GuiData = GuiData::new_with_application(application);

        // Used for getting data from thread
        let (glib_stop_sender, glib_stop_receiver) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

        // Futures progress report
        let (futures_sender_duplicate_files, futures_receiver_duplicate_files): (futures::channel::mpsc::UnboundedSender<duplicate::ProgressData>, futures::channel::mpsc::UnboundedReceiver<duplicate::ProgressData>) =
            futures::channel::mpsc::unbounded();
        let (futures_sender_empty_files, futures_receiver_empty_files): (futures::channel::mpsc::UnboundedSender<empty_files::ProgressData>, futures::channel::mpsc::UnboundedReceiver<empty_files::ProgressData>) = futures::channel::mpsc::unbounded();
        let (futures_sender_empty_folder, futures_receiver_empty_folder): (futures::channel::mpsc::UnboundedSender<empty_folder::ProgressData>, futures::channel::mpsc::UnboundedReceiver<empty_folder::ProgressData>) =
            futures::channel::mpsc::unbounded();
        let (futures_sender_big_file, futures_receiver_big_file): (futures::channel::mpsc::UnboundedSender<big_file::ProgressData>, futures::channel::mpsc::UnboundedReceiver<big_file::ProgressData>) = futures::channel::mpsc::unbounded();
        let (futures_sender_same_music, futures_receiver_same_music): (futures::channel::mpsc::UnboundedSender<same_music::ProgressData>, futures::channel::mpsc::UnboundedReceiver<same_music::ProgressData>) = futures::channel::mpsc::unbounded();
        let (futures_sender_similar_images, futures_receiver_similar_images): (futures::channel::mpsc::UnboundedSender<similar_images::ProgressData>, futures::channel::mpsc::UnboundedReceiver<similar_images::ProgressData>) =
            futures::channel::mpsc::unbounded();
        let (futures_sender_similar_videos, futures_receiver_similar_videos): (futures::channel::mpsc::UnboundedSender<similar_videos::ProgressData>, futures::channel::mpsc::UnboundedReceiver<similar_videos::ProgressData>) =
            futures::channel::mpsc::unbounded();
        let (futures_sender_temporary, futures_receiver_temporary): (futures::channel::mpsc::UnboundedSender<temporary::ProgressData>, futures::channel::mpsc::UnboundedReceiver<temporary::ProgressData>) = futures::channel::mpsc::unbounded();
        let (futures_sender_invalid_symlinks, futures_receiver_invalid_symlinks): (futures::channel::mpsc::UnboundedSender<invalid_symlinks::ProgressData>, futures::channel::mpsc::UnboundedReceiver<invalid_symlinks::ProgressData>) =
            futures::channel::mpsc::unbounded();
        let (futures_sender_broken_files, futures_receiver_broken_files): (futures::channel::mpsc::UnboundedSender<broken_files::ProgressData>, futures::channel::mpsc::UnboundedReceiver<broken_files::ProgressData>) =
            futures::channel::mpsc::unbounded();

        initialize_gui(&mut gui_data);
        validate_notebook_data(&gui_data); // Must be run after initialization of gui, to check if everything was properly setup
        reset_configuration(false, &gui_data.upper_notebook, &gui_data.settings, &gui_data.text_view_errors); // Fallback for invalid loading setting project
        load_configuration(false, &gui_data.upper_notebook, &gui_data.settings, &gui_data.text_view_errors, &gui_data.scrolled_window_errors);

        // Needs to run when entire GUI is initialized and
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
        connect_duplicate_buttons(&gui_data);
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
            save_configuration(false, &gui_data.upper_notebook, &gui_data.settings, &gui_data.text_view_errors); // Save configuration at exit
            taskbar_state.borrow_mut().release();
            Inhibit(false)
        });
    });

    application.run();
}
