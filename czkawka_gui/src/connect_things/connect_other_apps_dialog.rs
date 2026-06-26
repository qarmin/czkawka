use gtk4::prelude::*;
use gtk4::{Dialog, Orientation, ResponseType, ScrolledWindow};
use log::error;

struct OtherApp {
    name: &'static str,
    description: &'static str,
    url: &'static str,
}

const OTHER_APPS: &[OtherApp] = &[
    OtherApp {
        name: "Szyszka",
        description: "Fast and powerful file renamer.",
        url: "https://github.com/qarmin/szyszka",
    },
    OtherApp {
        name: "Mykrut",
        description: "Simple fast and opinionated Linux file manager.",
        url: "https://github.com/qarmin/mykrut",
    },
    OtherApp {
        name: "Dcmki Viewer",
        description: "Simple DICOM viewer.",
        url: "https://github.com/qarmin/dcmki_viewer",
    },
    OtherApp {
        name: "Video Thumbnailer",
        description: "Wrapper around the video thumbnail generator used in Czkawka.",
        url: "https://github.com/qarmin/video_thumbnailer",
    },
    OtherApp {
        name: "Space Finder",
        description: "Simple finder of the biggest files on your system.",
        url: "https://github.com/qarmin/space_finder",
    },
    OtherApp {
        name: "System Info Collector",
        description: "Collects RAM/CPU usage from the OS and shows it as graphs.",
        url: "https://github.com/qarmin/system-info-collector",
    },
];

pub fn show_other_apps_dialog(window_main: &gtk4::Window) {
    let dialog = Dialog::builder().title("Other Apps").transient_for(window_main).modal(true).build();

    dialog.set_default_size(520, 380);

    let button_close = dialog.add_button("Close", ResponseType::Close);

    let list_box = gtk4::ListBox::new();
    list_box.set_selection_mode(gtk4::SelectionMode::None);

    for app in OTHER_APPS {
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

        let desc_label = gtk4::Label::new(Some(app.description));
        desc_label.set_halign(gtk4::Align::Start);
        desc_label.set_wrap(true);

        info_box.append(&name_label);
        info_box.append(&desc_label);

        let url = app.url.to_string();
        let open_button = gtk4::Button::with_label("Open");
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

    let scrolled = ScrolledWindow::builder()
        .child(&list_box)
        .vexpand(true)
        .hexpand(true)
        .min_content_height(300)
        .build();

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

    parent.insert_child_after(&scrolled, None::<&gtk4::Widget>);

    if let Some(action_area) = button_close.parent() {
        action_area.set_halign(gtk4::Align::Center);
    }

    button_close.grab_focus();
    dialog.set_visible(true);

    dialog.connect_response(move |dialog, _| {
        dialog.close();
    });
}
