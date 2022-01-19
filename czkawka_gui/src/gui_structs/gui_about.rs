use gdk::gdk_pixbuf::Pixbuf;
use gtk::prelude::*;
use gtk::{Builder, Window};

use crate::flg;

#[derive(Clone)]
pub struct GuiAbout {
    pub about_dialog: gtk::AboutDialog,

    pub button_repository: gtk::Button,
    pub button_donation: gtk::Button,
    pub button_instruction: gtk::Button,
    pub button_translation: gtk::Button,
}

impl GuiAbout {
    pub fn create_from_builder(window_main: &Window, logo: &Pixbuf) -> Self {
        let glade_src = include_str!("../../ui/about_dialog.glade").to_string();
        let builder = Builder::from_string(glade_src.as_str());

        let about_dialog: gtk::AboutDialog = builder.object("about_dialog").unwrap();
        about_dialog.set_modal(true);
        about_dialog.set_transient_for(Some(window_main));

        about_dialog.set_logo(Some(logo));

        // Taken from command - "git shortlog -s -n -e" - remember to remove duplicates
        // This should be updated only before releasing new version
        about_dialog.set_authors(&vec![
            "Rafał Mikrut",
            "Thomas Andreas Jung",
            "Alexis Lefebvre",
            "Ben Bodenmiller",
            "Dan Dascalescu",
            "Igor",
            "Peter Blackson",
            "Shriraj Hegde",
            "krzysdz",
            "0xflotus",
            "Adam Boguszewski",
            "Caduser2020",
            "Danny Kirkham",
            "Dariusz Niedoba",
            "Douman",
            "Elazar Fine",
            "Farmadupe",
            "Jan Jurec",
            "Jona",
            "Meir Klemfner",
            "Mek101",
            "Michael Grigoryan",
            "Nikita Karamov",
            "Proprietary Chrome-chan",
            "Sbgodin",
            "Spirit",
            "Stefan Seering",
            "Syfaro",
            "Yuri Slobodyanyuk",
            "bellrise",
            "endolith",
            "jann",
            "kamilek96",
            "kuskov",
            "tecome",
            "tenninjas",
        ]);

        let button_repository: gtk::Button = builder.object("button_repository").unwrap();
        let button_donation: gtk::Button = builder.object("button_donation").unwrap();
        let button_instruction: gtk::Button = builder.object("button_instruction").unwrap();
        let button_translation: gtk::Button = builder.object("button_translation").unwrap();

        Self {
            about_dialog,
            button_repository,
            button_donation,
            button_instruction,
            button_translation,
        }
    }
    pub fn update_language(&self) {
        let mut comment_text: String = "2020 - 2022  Rafał Mikrut(qarmin)\n\n".to_string();
        comment_text += &flg!("about_window_motto");
        self.about_dialog.set_comments(Some(&comment_text));

        self.button_repository.set_tooltip_text(Some(&flg!("about_repository_button_tooltip")));
        self.button_donation.set_tooltip_text(Some(&flg!("about_donation_button_tooltip")));
        self.button_instruction.set_tooltip_text(Some(&flg!("about_instruction_button_tooltip")));
        self.button_translation.set_tooltip_text(Some(&flg!("about_translation_button_tooltip")));

        self.button_repository.set_label(&flg!("about_repository_button"));
        self.button_donation.set_label(&flg!("about_donation_button"));
        self.button_instruction.set_label(&flg!("about_instruction_button"));
        self.button_translation.set_label(&flg!("about_translation_button"));
    }
}
