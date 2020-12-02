mod connect_button_delete;
mod connect_button_save;
mod connect_button_search;
mod connect_button_select;
mod connect_button_stop;
mod connect_compute_results;
mod connect_notebook_tabs;
mod connect_popovers;
mod connect_progress_window;
mod connect_upper_notebook;
mod create_tree_view;
mod double_click_opening;
mod gui_data;
mod help_functions;
mod startup_configuration;

use czkawka_core::*;

extern crate gtk;
use crate::connect_button_delete::*;
use crate::connect_button_save::*;
use crate::connect_button_search::*;
use crate::connect_button_select::*;
use crate::connect_button_stop::*;
use crate::connect_compute_results::*;
use crate::connect_notebook_tabs::*;
use crate::connect_popovers::*;
use crate::connect_progress_window::*;
use crate::connect_upper_notebook::*;
use crate::gui_data::*;
use crate::startup_configuration::*;
use gtk::prelude::*;
use std::{env, process};

fn main() {
    let mut exit_program_after_initialization: bool = false;
    // Printing version
    {
        let all_arguments: Vec<String> = env::args().skip(1).collect(); // Not need to check program name

        for i in all_arguments {
            if i == "-v" || i == "--version" {
                println!("Czkawka GUI {}", CZKAWKA_VERSION);
                process::exit(0);
            }
            if i == "-q" || i == "--quit" {
                exit_program_after_initialization = true;
            }
        }
    }

    gtk::init().expect("Failed to initialize GTK.");

    let gui_data: GuiData = GuiData::new();

    // Used for getting data from thread
    let (glib_stop_sender, glib_stop_receiver) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

    // Futures progress report
    let (futures_sender_duplicate_files, futures_receiver_duplicate_files): (futures::channel::mpsc::Sender<duplicate::ProgressData>, futures::channel::mpsc::Receiver<duplicate::ProgressData>) = futures::channel::mpsc::channel(20);
    let (futures_sender_empty_files, futures_receiver_empty_files): (futures::channel::mpsc::Sender<empty_files::ProgressData>, futures::channel::mpsc::Receiver<empty_files::ProgressData>) = futures::channel::mpsc::channel(20);
    let (futures_sender_empty_folder, futures_receiver_empty_folder): (futures::channel::mpsc::Sender<empty_folder::ProgressData>, futures::channel::mpsc::Receiver<empty_folder::ProgressData>) = futures::channel::mpsc::channel(20);
    let (futures_sender_big_file, futures_receiver_big_file): (futures::channel::mpsc::Sender<big_file::ProgressData>, futures::channel::mpsc::Receiver<big_file::ProgressData>) = futures::channel::mpsc::channel(20);
    let (futures_sender_same_music, futures_receiver_same_music): (futures::channel::mpsc::Sender<same_music::ProgressData>, futures::channel::mpsc::Receiver<same_music::ProgressData>) = futures::channel::mpsc::channel(20);
    let (futures_sender_similar_images, futures_receiver_similar_images): (futures::channel::mpsc::Sender<similar_images::ProgressData>, futures::channel::mpsc::Receiver<similar_images::ProgressData>) = futures::channel::mpsc::channel(20);
    let (futures_sender_temporary, futures_receiver_temporary): (futures::channel::mpsc::Sender<temporary::ProgressData>, futures::channel::mpsc::Receiver<temporary::ProgressData>) = futures::channel::mpsc::channel(20);
    let (futures_sender_zeroed, futures_receiver_zeroed): (futures::channel::mpsc::Sender<zeroed::ProgressData>, futures::channel::mpsc::Receiver<zeroed::ProgressData>) = futures::channel::mpsc::channel(20);

    startup_configuration(&gui_data);
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
        futures_sender_temporary,
        futures_sender_zeroed,
    );
    connect_button_select(&gui_data);
    connect_button_stop(&gui_data);
    connect_notebook_tabs(&gui_data);
    connect_upper_notebook(&gui_data);
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
        futures_receiver_temporary,
        futures_receiver_zeroed,
    );

    // Quit the program when X in main window was clicked
    gui_data.window_main.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    // We start the gtk main loop.
    gtk::main();

    // Quiting if quit flag was provided
    if exit_program_after_initialization {
        gtk::main_quit();
    }
}
