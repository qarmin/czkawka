use crate::gui_data::GuiData;
use gtk::prelude::*;

pub fn connect_button_stop(gui_data: &GuiData) {
    let button_stop_in_dialog = gui_data.progress_window.button_stop_in_dialog.clone();
    let stop_sender = gui_data.stop_sender.clone();
    button_stop_in_dialog.connect_key_release_event(move |_, e| {
        if e.keycode() == Some(36) {
            // Only accept enter key to stop search
            stop_sender.send(()).unwrap();
        }
        gtk::Inhibit(false)
    });

    let button_stop_in_dialog = gui_data.progress_window.button_stop_in_dialog.clone();
    let stop_sender = gui_data.stop_sender.clone();
    button_stop_in_dialog.connect_button_release_event(move |_, _e| {
        stop_sender.send(()).unwrap();
        gtk::Inhibit(false)
    });
}
