extern crate gtk;
use crate::gui_data::GuiData;
use gtk::prelude::*;

pub fn connect_button_stop(gui_data: &GuiData) {
    let button_stop_in_dialog = gui_data.progress_dialog.button_stop_in_dialog.clone();
    let stop_sender = gui_data.stop_sender.clone();
    button_stop_in_dialog.connect_clicked(move |_| {
        stop_sender.send(()).unwrap();
    });
}
