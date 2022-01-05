// Remove console window in Windows OS
#![windows_subsystem = "windows"]

use gtk::prelude::*;

use crate::connect_change_language::*;

mod connect_change_language;

fn main() {
    let application = gtk::Application::builder().build();
    application.connect_activate(move |application| {
        load_system_language(); // Check for default system language, must be loaded after initializing GUI and before loading settings from file

        connect_change_language();

        let window_main = gtk::Window::builder().application(application).build();
        window_main.show_all();
        window_main.connect_delete_event(move |_, _| Inhibit(false));
    });

    application.run();
}
