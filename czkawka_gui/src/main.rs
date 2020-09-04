#[allow(unused_imports)]
use czkawka_core::{duplicate, empty_folder};

extern crate gtk;
use gtk::prelude::*;
use gtk::{ButtonsType, DialogFlags, MessageDialog, MessageType, Window};

fn main() {
    gtk::init().expect("Failed to initialize GTK.");

    // Loading glade file content
    let glade_src = include_str!("../czkawka.glade");
    // Build UI from glade file
    let builder = gtk::Builder::from_string(glade_src);

    // Show first window
    let window : gtk::Window = builder.get_object("window1").unwrap();
    window.show_all();


    let button: gtk::Button = builder.get_object("button1").unwrap();
    let dialog: gtk::MessageDialog = builder.get_object("messagedialog1").unwrap();

    button.connect_clicked(move |_| {
        // We make the dialog window blocks all other windows.
        dialog.run();
        // When it finished running, we hide it again.
        dialog.hide();
    });

// We start the gtk main loop.
    gtk::main();

}
