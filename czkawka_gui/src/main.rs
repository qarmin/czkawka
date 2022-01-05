// Remove console window in Windows OS
#![windows_subsystem = "windows"]
#![allow(clippy::collapsible_else_if)]
#![allow(clippy::too_many_arguments)]

use gtk::prelude::*;
use gtk::WindowType;

use crate::connect_change_language::*;


mod connect_change_language;
mod language_functions;

fn main() {
    let application = gtk::Application::builder().build();
    application.connect_activate(move |_application| {
        load_system_language(); // Check for default system language, must be loaded after initializing GUI and before loading settings from file

        connect_change_language();

        let window_main = gtk::Window::new(WindowType::Toplevel);
        window_main.connect_delete_event(move |_, _| Inhibit(false));
    });

    application.run();
}
