use crate::gui_data::GuiData;
use crate::saving_loading::{load_configuration, reset_configuration, save_configuration};
use gtk::prelude::*;


pub fn connect_settings(gui_data: &GuiData) {
    // Connect button settings
    {
        let button_settings = gui_data.header.button_settings.clone();
        let window_main = gui_data.window_main.clone();
        let window_settings = gui_data.settings.window_settings.clone();
        button_settings.connect_clicked(move |_| {
            window_main.set_sensitive(false);
            window_settings.show();
        });

        let window_main = gui_data.window_main.clone();
        let window_settings = gui_data.settings.window_settings.clone();

        window_settings.connect_delete_event(move |window, _y| {
            window.hide();
            window_main.set_sensitive(true);
            gtk::Inhibit(true)
        });
    }

    // Connect save configuration button
    {
        let upper_notebook = gui_data.upper_notebook.clone();
        let settings = gui_data.settings.clone();
        let text_view_errors = gui_data.text_view_errors.clone();
        let button_settings_save_configuration = gui_data.settings.button_settings_save_configuration.clone();
        button_settings_save_configuration.connect_clicked(move |_| {
            save_configuration(true, &upper_notebook, &settings, &text_view_errors);
        });
    }
    // Connect load configuration button
    {
        let upper_notebook = gui_data.upper_notebook.clone();
        let settings = gui_data.settings.clone();
        let text_view_errors = gui_data.text_view_errors.clone();
        let button_settings_load_configuration = gui_data.settings.button_settings_load_configuration.clone();
        let scrolled_window_errors = gui_data.scrolled_window_errors.clone();
        button_settings_load_configuration.connect_clicked(move |_| {
            load_configuration(true, &upper_notebook, &settings, &text_view_errors, &scrolled_window_errors);
        });
    }
    // Connect reset configuration button
    {
        let upper_notebook = gui_data.upper_notebook.clone();
        let settings = gui_data.settings.clone();
        let text_view_errors = gui_data.text_view_errors.clone();
        let button_settings_reset_configuration = gui_data.settings.button_settings_reset_configuration.clone();
        button_settings_reset_configuration.connect_clicked(move |_| {
            reset_configuration(true, &upper_notebook, &settings, &text_view_errors);
        });
    }
}
