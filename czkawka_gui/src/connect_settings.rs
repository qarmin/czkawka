use crate::gui_data::GuiData;
use crate::saving_loading::{load_configuration, reset_configuration, save_configuration};
use gtk::prelude::*;
use gtk::WindowPosition;

pub fn connect_settings(gui_data: &GuiData) {
    // Connect button settings
    {
        let button_settings = gui_data.header.button_settings.clone();
        let window_main = gui_data.window_main.clone();
        let window_settings = gui_data.settings.window_settings.clone();
        button_settings.connect_clicked(move |_| {
            window_main.set_sensitive(false);
            window_settings.show();
            window_settings.set_position(WindowPosition::Center);
        });

        let window_main = gui_data.window_main.clone();
        let window_settings = gui_data.settings.window_settings.clone();

        window_settings.hide_on_delete();

        window_settings.connect_delete_event(move |window, _y| {
            window.hide();
            window_main.set_sensitive(true);
            gtk::Inhibit(true)
        });
    }

    // Connect save configuration button
    {
        let gui_data = gui_data.clone();
        let button_settings_save_configuration = gui_data.settings.button_settings_save_configuration.clone();
        button_settings_save_configuration.connect_clicked(move |_| {
            save_configuration(&gui_data, true);
        });
    }
    // Connect load configuration button
    {
        let gui_data = gui_data.clone();
        let button_settings_load_configuration = gui_data.settings.button_settings_load_configuration.clone();
        button_settings_load_configuration.connect_clicked(move |_| {
            load_configuration(&gui_data, true);
        });
    }
    // Connect reset configuration button
    {
        let gui_data = gui_data.clone();
        let button_settings_reset_configuration = gui_data.settings.button_settings_reset_configuration.clone();
        button_settings_reset_configuration.connect_clicked(move |_| {
            reset_configuration(&gui_data, true);
        });
    }
}
