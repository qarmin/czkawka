use std::io::BufReader;

use gdk4::gdk_pixbuf::Pixbuf;
use gtk4::prelude::*;
use gtk4::{Align, Dialog, Orientation, ResponseType};

use crate::flg;
use crate::gui_structs::gui_data::KROKIET_LOGO_PNG;

pub fn show_krokiet_promo_dialog(window_main: &gtk4::Window) {
    let dialog = Dialog::builder()
        .title(flg!("krokiet_promo_title"))
        .transient_for(window_main)
        .modal(true)
        .build();

    let button_ok = dialog.add_button(&flg!("general_ok_button"), ResponseType::Ok);

    dialog.set_default_size(520, 0);

    // Load Krokiet logo
    let logo_widget = if let Ok(pixbuf) = Pixbuf::from_read(BufReader::new(KROKIET_LOGO_PNG)) {
        if let Some(scaled) = pixbuf.scale_simple(128, 128, gdk4::gdk_pixbuf::InterpType::Bilinear) {
            let image = gtk4::Image::from_pixbuf(Some(&scaled));
            image.set_margin_top(15);
            image.set_margin_bottom(5);
            Some(image)
        } else {
            None
        }
    } else {
        None
    };

    let label = gtk4::Label::builder()
        .label(&flg!("krokiet_promo_message"))
        .wrap(true)
        .justify(gtk4::Justification::Center)
        .halign(Align::Center)
        .margin_top(10)
        .margin_bottom(10)
        .margin_start(15)
        .margin_end(15)
        .build();

    let link = gtk4::Label::builder()
        .label("<a href=\"https://github.com/qarmin/czkawka/releases\">Pobierz Krokieta</a>  |  <a href=\"https://github.com/qarmin/czkawka/tree/master/krokiet\">Strona projektu</a>")
        .use_markup(true)
        .halign(Align::Center)
        .margin_top(5)
        .margin_bottom(15)
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

    // Insert widgets: logo first, then label, then link
    if let Some(logo) = &logo_widget {
        parent.insert_child_after(logo, None::<&gtk4::Widget>);
        parent.insert_child_after(&label, Some(logo.upcast_ref::<gtk4::Widget>()));
    } else {
        parent.insert_child_after(&label, None::<&gtk4::Widget>);
    }
    parent.insert_child_after(&link, Some(&label));

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
