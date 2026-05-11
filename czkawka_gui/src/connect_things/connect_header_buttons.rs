use gtk4::prelude::*;

use crate::connect_things::connect_krokiet_promo_dialog::show_krokiet_promo_dialog;
use crate::gui_structs::gui_data::GuiData;

pub(crate) fn connect_button_about(gui_data: &GuiData) {
    let about_dialog = gui_data.about.about_dialog.clone();
    let button_app_info = gui_data.header.button_app_info.clone();
    button_app_info.connect_clicked(move |_| {
        about_dialog.set_visible(true);

        // Prevent from deleting dialog after close
        about_dialog.connect_close_request(|dialog| {
            dialog.set_visible(false);
            glib::Propagation::Stop
        });
    });

    let window_main = gui_data.window_main.clone();
    let button_krokiet = gui_data.header.button_krokiet.clone();
    button_krokiet.connect_clicked(move |_| {
        show_krokiet_promo_dialog(&window_main);
    });
}
