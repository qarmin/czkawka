extern crate gtk;
use crate::gui_data::GuiData;
use gtk::prelude::*;

pub fn connect_button_stop(gui_data: &GuiData) {
    let buttons_stop = gui_data.buttons_stop.clone();
    let stop_sender = gui_data.stop_sender.clone();
    buttons_stop.connect_clicked(move |_| {
        stop_sender.send(()).unwrap();
    });
}
