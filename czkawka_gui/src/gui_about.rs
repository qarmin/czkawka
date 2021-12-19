use gtk::prelude::*;
use gtk::{Builder, Window};

use crate::fl;

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
        let button_donation: gtk::Button = builder.object("button_donation").unwrap();
        let button_instruction: gtk::Button = builder.object("button_instruction").unwrap();

        Self {
            about_dialog,
            button_repository,
            button_donation,
            button_instruction,
        }
    }
    pub fn update_language(&self) {
        self.button_repository.set_tooltip_text(Some(&fl!("about_repository_button_tooltip")));
        self.button_donation.set_tooltip_text(Some(&fl!("about_donation_button_tooltip")));
        self.button_instruction.set_tooltip_text(Some(&fl!("about_instruction_button_tooltip")));

        self.button_repository.set_label(&fl!("about_repository_button"));
        self.button_donation.set_label(&fl!("about_donation_button"));
        self.button_instruction.set_label(&fl!("about_instruction_button"));
    }
}
