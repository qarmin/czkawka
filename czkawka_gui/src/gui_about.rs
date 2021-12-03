use gtk::prelude::*;
use gtk::{Builder, Window};

#[derive(Clone)]
pub struct GuiAbout {
    pub about_dialog: gtk::AboutDialog,

    pub button_repository: gtk::Button,
    pub button_donation: gtk::Button,
    pub button_instruction: gtk::Button,
}

impl GuiAbout {
    pub fn create_from_builder(window_main: &Window) -> Self {
        let glade_src = include_str!("../ui/about_dialog.glade").to_string();
        let builder = Builder::from_string(glade_src.as_str());

        let about_dialog: gtk::AboutDialog = builder.object("about_dialog").unwrap();
        about_dialog.set_modal(true);
        about_dialog.set_transient_for(Some(window_main));

        let button_repository: gtk::Button = builder.object("button_repository").unwrap();
        button_repository.set_tooltip_text(Some("Link to repository page with source code."));
        let button_donation: gtk::Button = builder.object("button_donation").unwrap();
        button_donation.set_tooltip_text(Some("Link to donation page."));
        let button_instruction: gtk::Button = builder.object("button_instruction").unwrap();
        button_instruction.set_tooltip_text(Some("Link to instruction page."));

        Self {
            about_dialog,
            button_repository,
            button_donation,
            button_instruction,
        }
    }
}
