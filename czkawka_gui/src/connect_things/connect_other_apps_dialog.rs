use gtk4::prelude::*;
use gtk4::{Dialog, Orientation, ResponseType, ScrolledWindow};
use log::error;

use crate::flg;

struct OtherApp {
    name: &'static str,
    url: &'static str,
}

const OTHER_APPS: &[OtherApp] = &[
    OtherApp {
        name: "Szyszka",
        url: "https://github.com/qarmin/szyszka",
    },
    OtherApp {
        name: "Mykrut",
        url: "https://github.com/qarmin/mykrut",
    },
    OtherApp {
        name: "Dcmki Viewer",
        url: "https://github.com/qarmin/dcmki_viewer",
    },
    OtherApp {
        name: "Video Thumbnailer",
        url: "https://github.com/qarmin/video_thumbnailer",
    },
    OtherApp {
        name: "Space Finder",
        url: "https://github.com/qarmin/space_finder",
    },
    OtherApp {
        name: "System Info Collector",
        url: "https://github.com/qarmin/system-info-collector",
    },
];

fn app_descriptions() -> Vec<String> {
    vec![
        flg!("about_other_apps_szyszka_desc"),
        flg!("about_other_apps_mykrut_desc"),
        flg!("about_other_apps_dcmki_viewer_desc"),
        flg!("about_other_apps_video_thumbnailer_desc"),
        flg!("about_other_apps_space_finder_desc"),
        flg!("about_other_apps_system_info_collector_desc"),
    ]
}

pub fn show_other_apps_dialog(window_main: &gtk4::Window) {
    let dialog = Dialog::builder()
        .title(&flg!("about_other_apps_dialog_title"))
        .transient_for(window_main)
        .modal(true)
        .build();

    dialog.set_default_size(520, 420);

    let button_close = dialog.add_button(&flg!("general_close_button"), ResponseType::Close);

    let note_label = gtk4::Label::new(Some(&flg!("about_other_apps_open_source_note")));
    note_label.set_halign(gtk4::Align::Center);
    note_label.set_margin_top(6);
    note_label.set_margin_bottom(6);

    let list_box = gtk4::ListBox::new();
    list_box.set_selection_mode(gtk4::SelectionMode::None);

    let descriptions = app_descriptions();
    for (app, desc) in OTHER_APPS.iter().zip(descriptions.iter()) {
        let row = gtk4::ListBoxRow::new();
        let row_box = gtk4::Box::new(Orientation::Horizontal, 10);
        row_box.set_margin_top(8);
        row_box.set_margin_bottom(8);
        row_box.set_margin_start(12);
        row_box.set_margin_end(12);

        let info_box = gtk4::Box::new(Orientation::Vertical, 4);
        info_box.set_hexpand(true);

        let name_label = gtk4::Label::new(None);
        name_label.set_markup(&format!("<b>{}</b>", app.name));
        name_label.set_halign(gtk4::Align::Start);

        let desc_label = gtk4::Label::new(Some(desc));
        desc_label.set_halign(gtk4::Align::Start);
        desc_label.set_wrap(true);

        info_box.append(&name_label);
        info_box.append(&desc_label);

        let url = app.url.to_string();
        let open_button = gtk4::Button::with_label(&flg!("about_other_apps_open_button"));
        open_button.set_valign(gtk4::Align::Center);
        open_button.connect_clicked(move |_| {
            if let Err(e) = open::that(&url) {
                error!("Failed to open URL: {url}, reason {e}");
            }
        });

        row_box.append(&info_box);
        row_box.append(&open_button);
        row.set_child(Some(&row_box));
        list_box.append(&row);
    }

    let scrolled = ScrolledWindow::builder().child(&list_box).vexpand(true).hexpand(true).min_content_height(300).build();

    let parent = button_close
        .parent()
        .expect("Button should have parent")
        .parent()
        .expect("Button parent should have parent")
        .downcast::<gtk4::Box>()
        .expect("Should be a Box");

    parent.set_orientation(Orientation::Vertical);
    parent.set_halign(gtk4::Align::Fill);
    parent.set_margin_start(10);
    parent.set_margin_end(10);
    parent.set_margin_top(10);
    parent.set_margin_bottom(10);

    parent.insert_child_after(&note_label, None::<&gtk4::Widget>);
    parent.insert_child_after(&scrolled, Some(&note_label));

    if let Some(action_area) = button_close.parent() {
        action_area.set_halign(gtk4::Align::Center);
    }

    button_close.grab_focus();
    dialog.set_visible(true);

    dialog.connect_response(move |dialog, _| {
        dialog.close();
    });
}
