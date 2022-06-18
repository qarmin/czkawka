use gtk4::prelude::*;

use crate::gui_structs::gui_data::GuiData;

const SPONSOR_SITE: &str = "https://github.com/sponsors/qarmin";
const REPOSITORY_SITE: &str = "https://github.com/qarmin/czkawka";
const INSTRUCTION_SITE: &str = "https://github.com/qarmin/czkawka/blob/master/instructions/Instruction.md";
const TRANSLATION_SITE: &str = "https://crwd.in/czkawka";

pub fn connect_about_buttons(gui_data: &GuiData) {
    let button_donation = gui_data.about.button_donation.clone();
    button_donation.connect_clicked(move |_| {
        if let Err(e) = open::that(SPONSOR_SITE) {
            println!("Failed to open sponsor site: {}, reason {}", SPONSOR_SITE, e)
        };
    });

    let button_instruction = gui_data.about.button_instruction.clone();
    button_instruction.connect_clicked(move |_| {
        if let Err(e) = open::that(INSTRUCTION_SITE) {
            println!("Failed to open instruction site: {}, reason {}", INSTRUCTION_SITE, e)
        };
    });

    let button_repository = gui_data.about.button_repository.clone();
    button_repository.connect_clicked(move |_| {
        if let Err(e) = open::that(REPOSITORY_SITE) {
            println!("Failed to open repository site: {}, reason {}", REPOSITORY_SITE, e)
        };
    });

    let button_translation = gui_data.about.button_translation.clone();
    button_translation.connect_clicked(move |_| {
        if let Err(e) = open::that(TRANSLATION_SITE) {
            println!("Failed to open repository site: {}, reason {}", TRANSLATION_SITE, e)
        };
    });
}
