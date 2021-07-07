use gtk::prelude::*;
use gtk::{Builder, WindowPosition};

#[derive(Clone)]
pub struct GuiAbout {
    pub about_dialog: gtk::AboutDialog,

    pub button_repository: gtk::Button,
    pub button_donation: gtk::Button,
    pub button_instruction: gtk::Button,
}

impl GuiAbout {
    pub fn create_from_builder() -> Self {
        let glade_src = include_str!("../ui/about_dialog.glade").to_string();
        let builder = Builder::from_string(glade_src.as_str());

        let about_dialog: gtk::AboutDialog = builder.object("about_dialog").unwrap();
        about_dialog.set_position(WindowPosition::Center);

        let button_repository: gtk::Button = builder.object("button_repository").unwrap();
        let button_donation: gtk::Button = builder.object("button_donation").unwrap();
        let button_instruction: gtk::Button = builder.object("button_instruction").unwrap();

        Self {
            about_dialog,
            button_repository,
            button_donation,
            button_instruction,
        }
    }
}
