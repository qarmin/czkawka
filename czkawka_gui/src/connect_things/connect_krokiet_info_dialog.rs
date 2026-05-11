use gtk4::prelude::*;
use gtk4::{Align, Dialog, Orientation, Picture, ResponseType};

use crate::flg;
use crate::gui_structs::gui_data::CZK_ICON_KROKIET;
use crate::helpers::image_operations::svg_to_pixbuf;

pub fn show_krokiet_info_dialog(window_main: &gtk4::Window) {
    let dialog = Dialog::builder().title(flg!("krokiet_info_title")).transient_for(window_main).modal(true).build();

    let button_ok = dialog.add_button(&flg!("general_ok_button"), ResponseType::Ok);

    dialog.set_default_size(500, 1);

    // Load Krokiet logo from SVG at 96x96
    let logo_widget = svg_to_pixbuf(CZK_ICON_KROKIET, 96).map(|pixbuf| {
        let picture = Picture::for_pixbuf(&pixbuf);
        picture.set_can_shrink(false);
        let wrapper = gtk4::Box::new(Orientation::Vertical, 0);
        wrapper.set_size_request(96, 96);
        wrapper.set_halign(Align::Center);
        wrapper.set_hexpand(false);
        wrapper.set_vexpand(false);
        wrapper.set_margin_top(15);
        wrapper.set_margin_bottom(5);
        wrapper.append(&picture);
        wrapper
    });

    let label = gtk4::Label::builder()
        .label(&flg!("krokiet_info_message"))
        .wrap(true)
        .justify(gtk4::Justification::Center)
        .halign(Align::Center)
        .margin_top(10)
        .margin_bottom(10)
        .margin_start(10)
        .margin_end(10)
        .build();

    let link_text = format!(
        "<a href=\"https://github.com/qarmin/czkawka/releases\">{}</a>  |  <a href=\"https://github.com/qarmin/czkawka/tree/master/krokiet\">{}</a>",
        flg!("krokiet_promo_link_download"),
        flg!("krokiet_promo_link_project")
    );
    let link = gtk4::Label::builder()
        .label(&link_text)
        .use_markup(true)
        .halign(Align::Center)
        .margin_top(5)
        .margin_bottom(10)
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

    if let Some(logo) = &logo_widget {
        parent.insert_child_after(logo, None::<&gtk4::Widget>);
        parent.insert_child_after(&label, Some(logo));
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
