extern crate gtk;
use crate::gui_data::GuiData;
use crate::saving_loading::{load_configuration, reset_configuration, save_configuration};
use gtk::prelude::*;

pub fn connect_settings(gui_data: &GuiData) {
    // Connect save configuration button
    {
        let gui_data = gui_data.clone();
        let button_settings_save_configuration = gui_data.button_settings_save_configuration.clone();
        button_settings_save_configuration.connect_clicked(move |_| {
            save_configuration(&gui_data, true);
        });
    }
    // Connect load configuration button
    {
        let gui_data = gui_data.clone();
        let button_settings_load_configuration = gui_data.button_settings_load_configuration.clone();
        button_settings_load_configuration.connect_clicked(move |_| {
            load_configuration(&gui_data, true);
        });
    }
    // Connect reset configuration button
    {
        let gui_data = gui_data.clone();
        let button_settings_reset_configuration = gui_data.button_settings_reset_configuration.clone();
        button_settings_reset_configuration.connect_clicked(move |_| {
            reset_configuration(&gui_data, true);
        });
    }
}
