extern crate gtk;
use crate::gui_data::GuiData;
use gtk::prelude::*;

const SPONSOR_SITE: &str = "https://github.com/sponsors/qarmin";
const REPOSITORY_SITE: &str = "https://github.com/qarmin/czkawka";
const INSTRUCTION_SITE: &str = "https://github.com/qarmin/czkawka/blob/master/instructions/Instruction.md";

pub fn connect_about_buttons(gui_data: &GuiData) {
    let button_donation = gui_data.about.button_donation.clone();
    button_donation.connect_clicked(move |_| {
        if open::that(SPONSOR_SITE).is_err() {
            println!("Failed to open sponsor site: {}", SPONSOR_SITE)
        };
    });

    let button_instruction = gui_data.about.button_instruction.clone();
    button_instruction.connect_clicked(move |_| {
        if open::that(INSTRUCTION_SITE).is_err() {
            println!("Failed to open instruction site: {}", INSTRUCTION_SITE)
        };
    });

    let button_repository = gui_data.about.button_repository.clone();
    button_repository.connect_clicked(move |_| {
        if open::that(REPOSITORY_SITE).is_err() {
            println!("Failed to open repository site: {}", REPOSITORY_SITE)
        };
    });
}
