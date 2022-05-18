use crossbeam_channel::{Sender, TrySendError};

use crate::gui_structs::gui_data::GuiData;
use crate::help_functions::KEY_ENTER;

fn send_stop_message(stop_sender: &Sender<()>) {
    stop_sender
        .try_send(())
        .map_or_else(|e| if matches!(e, TrySendError::Full(_)) { Ok(()) } else { Err(e) }, |_| Ok(()))
        .unwrap();
}

pub fn connect_button_stop(gui_data: &GuiData) {
    let evk_button_stop_in_dialog = gui_data.progress_window.evk_button_stop_in_dialog.clone();
    let stop_sender = gui_data.stop_sender.clone();
    evk_button_stop_in_dialog.connect_key_released(move |_, _, key_code, _| {
        if key_code == KEY_ENTER {
            // Only accept enter key to stop search
            send_stop_message(&stop_sender);
        }
    });

    // TODO GTK 4
    // let gc_button_stop_in_dialog = gui_data.progress_window.gc_button_stop_in_dialog.clone();
    // let stop_sender = gui_data.stop_sender.clone();
    // // gc_button_stop_in_dialog.connect_released(move |_, _e| {
    // gc_button_stop_in_dialog.connect_button_release_event(move |_, _e| {
    //     send_stop_message(&stop_sender);
    //     gtk4::Inhibit(false)
    // });
}
