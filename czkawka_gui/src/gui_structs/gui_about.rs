use gdk4::gdk_pixbuf::Pixbuf;
use gtk4::prelude::*;
use gtk4::{Builder, Button, Orientation, Picture, Window};

use crate::flg;
use crate::help_functions::get_all_boxes_from_widget;

#[derive(Clone)]
pub struct GuiAbout {
    pub about_dialog: gtk4::AboutDialog,

    pub button_repository: Button,
    pub button_donation: Button,
    pub button_instruction: Button,
    pub button_translation: Button,
}

impl GuiAbout {
    pub fn create_from_builder(window_main: &Window, logo: &Pixbuf) -> Self {
        let glade_src = include_str!("../../ui/about_dialog.ui").to_string();
        let builder = Builder::from_string(glade_src.as_str());

        let about_dialog: gtk4::AboutDialog = builder.object("about_dialog").unwrap();
        about_dialog.set_modal(true);
        about_dialog.set_transient_for(Some(window_main));

        about_dialog.set_logo(Picture::for_pixbuf(logo).paintable().as_ref());

        // Taken from command - "git shortlog -s -n -e" - remember to remove duplicates
        // This should be updated only before releasing new version
        about_dialog.set_authors(&[
            "Rafał Mikrut",
            "Alexis Lefebvre",
            "Thomas Andreas Jung",
            "Peter Blackson",
            "TheEvilSkeleton",
            "Ben Bodenmiller",
            "ChihWei Wang",
            "Dan Dascalescu",
            "Igor",
            "Kerfuffle",
            "Shriraj Hegde",
            "krzysdz",
            "0xflotus",
            "Adam Boguszewski",
            "Caduser2020",
            "Danny Kirkham",
            "Dariusz Niedoba",
            "Dominik Piątkowski",
            "Douman",
            "Elazar Fine",
            "Farmadupe",
            "Gitoffthelawn",
            "Ivan Habernal",
            "Jan Jurec",
            "Joey Babcock",
            "Jona",
            "Jonathan Hult",
            "Meir Klemfner",
            "Mek101",
            "Michael Grigoryan",
            "Nikita Karamov",
            "Sbgodin",
            "Spirit",
            "Stefan Seering",
            "Syfaro",
            "Yuri Slobodyanyuk",
            "bakeromso",
            "bellrise",
            "cyqsimon",
            "endolith",
            "jann",
            "kamilek96",
            "kuskov",
            "tecome",
            "tenninjas",
        ]);

        let custom_box = get_all_boxes_from_widget(&about_dialog)[2].clone(); // TODO may not be stable enough between GTK versions
        let new_box = gtk4::Box::new(Orientation::Horizontal, 5);

        let button_repository = Button::builder().label("Repository").build();
        let button_donation = Button::builder().label("Donation").build();
        let button_instruction = Button::builder().label("Instruction").build();
        let button_translation = Button::builder().label("Translation").build();

        new_box.append(&button_repository);
        new_box.append(&button_donation);
        new_box.append(&button_instruction);
        new_box.append(&button_translation);

        custom_box.append(&new_box);

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
