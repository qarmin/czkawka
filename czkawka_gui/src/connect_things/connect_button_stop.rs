use gtk::prelude::*;

use crate::gui_structs::gui_data::GuiData;
use crate::help_functions::KEY_ENTER;

pub fn connect_button_stop(gui_data: &GuiData) {
    let evk_button_stop_in_dialog = gui_data.progress_window.evk_button_stop_in_dialog.clone();
    let stop_sender = gui_data.stop_sender.clone();
    evk_button_stop_in_dialog.connect_key_released(move |_, _, key_code, _| {
        if key_code == KEY_ENTER {
            // Only accept enter key to stop search
            stop_sender.send(()).unwrap();
        }
    });

    let button_stop_in_dialog = gui_data.progress_window.button_stop_in_dialog.clone();
    let stop_sender = gui_data.stop_sender.clone();
    button_stop_in_dialog.connect_button_release_event(move |_, _e| {
        stop_sender.send(()).unwrap();
        gtk::Inhibit(false)
    });

    // let gc_button_stop_in_dialog = gui_data.progress_window.gc_button_stop_in_dialog.clone();
    // let stop_sender = gui_data.stop_sender.clone();
    // gc_button_stop_in_dialog.connect_button_release_event(move |_, _e| {
    //     stop_sender.send(()).unwrap();
    //     gtk::Inhibit(false)
    // });
}
