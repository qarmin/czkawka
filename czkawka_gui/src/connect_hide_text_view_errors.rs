use crate::gui_data::GuiData;
use gtk::prelude::*;

pub fn connect_hide_text_view_errors(gui_data: &GuiData) {
    let check_button_settings_show_text_view = gui_data.settings.check_button_settings_show_text_view.clone();
    let buttons_show_errors = gui_data.bottom_buttons.buttons_show_errors.clone();
    let scrolled_window_errors = gui_data.scrolled_window_errors.clone();

    buttons_show_errors.connect_clicked(move |_| {
        if scrolled_window_errors.is_visible() {
            scrolled_window_errors.hide();
            check_button_settings_show_text_view.set_active(false);
        } else {
            scrolled_window_errors.show();
            check_button_settings_show_text_view.set_active(true);
        }
    });
}
