use crate::gui_data::GuiData;
use gtk::prelude::*;
use gtk::{ResponseType, WindowPosition};

pub fn connect_button_about(gui_data: &GuiData) {
    let about_dialog = gui_data.about.about_dialog.clone();
    let button_app_info = gui_data.header.button_app_info.clone();
    button_app_info.connect_clicked(move |_| {
        about_dialog.set_position(WindowPosition::Center);
        about_dialog.show();
        let response = about_dialog.run();
        if response != ResponseType::None {
            about_dialog.hide();
        }
    });
}
