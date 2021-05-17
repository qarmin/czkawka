use gtk::prelude::*;
use gtk::WindowPosition;

#[derive(Clone)]
pub struct GuiAbout {
    pub about_dialog: gtk::AboutDialog,

    pub button_repository: gtk::Button,
    pub button_donation: gtk::Button,
    pub button_instruction: gtk::Button,
}

impl GuiAbout {
    pub fn create_from_builder(builder: &gtk::Builder) -> Self {
        let about_dialog: gtk::AboutDialog = builder.get_object("about_dialog").unwrap();
        about_dialog.set_position(WindowPosition::Center);

        let button_repository: gtk::Button = builder.get_object("button_repository").unwrap();
        let button_donation: gtk::Button = builder.get_object("button_donation").unwrap();
        let button_instruction: gtk::Button = builder.get_object("button_instruction").unwrap();

        Self {
            about_dialog,
            button_repository,
            button_donation,
            button_instruction,
        }
    }
}
