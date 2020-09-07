#[allow(unused_imports)]
use czkawka_core::{duplicate, empty_folder};

extern crate gtk;
use gtk::prelude::*;
use gtk::Builder;
// use gtk::{ButtonsType, DialogFlags, MessageDialog, MessageType, Window};

fn main() {
    gtk::init().expect("Failed to initialize GTK.");

    // Loading glade file content
    let glade_src = include_str!("../czkawka.glade");
    // Build UI from glade file
    let builder = gtk::Builder::from_string(glade_src);

    // Show first window
    let main_window: gtk::Window = builder.get_object("main_window").unwrap();
    main_window.show_all();

    prepare_buttons_at_start(builder);


    // We start the gtk main loop.
    gtk::main();
}
fn prepare_buttons_at_start(builder : Builder){
    // let buttons_search : gtk::Button = builder.get_object("buttons_search").unwrap();
    // buttons_search.connect_clicked(|| duplicate::);
    let buttons_stop : gtk::Button = builder.get_object("buttons_stop").unwrap();
    buttons_stop.hide();
    let buttons_resume : gtk::Button = builder.get_object("buttons_resume").unwrap();
    buttons_resume.hide();
    let buttons_pause : gtk::Button = builder.get_object("buttons_pause").unwrap();
    buttons_pause.hide();


    let buttons_select : gtk::Button = builder.get_object("buttons_select").unwrap();
    buttons_select.hide();
    let buttons_delete : gtk::Button = builder.get_object("buttons_delete").unwrap();
    buttons_delete.hide();
    let buttons_save : gtk::Button = builder.get_object("buttons_save").unwrap();
    buttons_save.hide();
}