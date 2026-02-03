use gtk4::prelude::*;
use gtk4::{Align, Dialog, Orientation, ResponseType};

use crate::flg;

pub fn show_krokiet_info_dialog(window_main: &gtk4::Window) {
    let dialog = Dialog::builder().title("Czkawka Flatpak deprecation notice").transient_for(window_main).modal(true).build();

    let button_ok = dialog.add_button(&flg!("general_ok_button"), ResponseType::Ok);

    dialog.set_default_size(500, 0);

    let info_text = r#"Due to Flathub rules regarding duplicate submissions, the Flatpak version of Czkawka will be removed in the near future and replaced by a new application called Krokiet.

Your currently installed Flatpak version of Czkawka will most likely continue to work and will not be automatically replaced. However, it will be removed from Flathub, which means it will no longer receive updates.

New installations and future updates will be available only through the Krokiet Flatpak package."#;

    let label = gtk4::Label::builder()
        .label(info_text)
        .wrap(true)
        .justify(gtk4::Justification::Center)
        .halign(Align::Center)
        .margin_top(10)
        .margin_bottom(10)
        .margin_start(10)
        .margin_end(10)
        .build();

    button_ok.grab_focus();

    let parent = button_ok
        .parent()
        .expect("Button should have parent")
        .parent()
        .expect("Button parent should have parent")
        .downcast::<gtk4::Box>()
        .expect("Should be a Box");

    parent.set_orientation(Orientation::Vertical);
    parent.set_halign(Align::Fill);
    parent.set_margin_start(10);
    parent.set_margin_end(10);
    parent.set_margin_top(10);
    parent.set_margin_bottom(10);

    parent.insert_child_after(&label, None::<&gtk4::Widget>);

    if let Some(action_area) = button_ok.parent() {
        action_area.set_halign(Align::Center);
    }

    dialog.set_visible(true);

    dialog.connect_response(move |dialog, response_type| {
        if response_type == ResponseType::Ok {
            dialog.close();
        }
    });
}
