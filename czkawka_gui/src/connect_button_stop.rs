extern crate gtk;
use crate::gui_data::GuiData;
use gtk::prelude::*;

pub fn connect_button_stop(gui_data: &GuiData) {
    // TODO remove it when it will not be used
    {
        let buttons_stop = gui_data.buttons_stop.clone();
        let stop_sender = gui_data.stop_sender.clone();
        buttons_stop.connect_clicked(move |_| {
            stop_sender.send(()).unwrap();
        });
    }

    let button_stop_in_dialog = gui_data.button_stop_in_dialog.clone();
    let stop_sender = gui_data.stop_sender.clone();
    button_stop_in_dialog.connect_clicked(move |_| {
        stop_sender.send(()).unwrap();
    });
}
