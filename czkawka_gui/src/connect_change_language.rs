use crate::GuiData;
use gtk::prelude::*;
use i18n_embed::unic_langid::LanguageIdentifier;
// use i18n_embed::{DesktopLanguageRequester, Localizer};

pub fn connect_change_language(gui_data: &GuiData) {
    change_language(gui_data);

    let check_button_language = gui_data.header.check_button_language.clone();
    let gui_data = gui_data.clone();
    check_button_language.connect_clicked(move |_| {
        change_language(&gui_data);
    });
}

fn change_language(gui_data: &GuiData) {
    // Alg
    // Use
    let localizers = vec![("czkawka_gui", crate::localizer::localizer())];
    let lang_byte = match gui_data.header.check_button_language.is_active() {
        true => "pl",
        false => "en",
    }
    .as_bytes();
    let lang_identifier = vec![LanguageIdentifier::from_bytes(lang_byte).unwrap()];
    // let available_languages = Localizer::available_languages();
    // println!("{:?}", available_languages);
    for (lib, localizer) in localizers {
        if let Err(error) = localizer.select(&lang_identifier) {
            eprintln!("Error while loadings languages for {} {:?}", lib, error);
        }
    }
    gui_data.update_language();

    // Try to use default OS
    // let requested_languages = DesktopLanguageRequester::requested_languages();
    // let localizers = vec![("czkawka_gui", crate::localizer::localizer())];
    //
    // println!("Requested Languages{:?}", requested_languages);
    //
    // let lang_identifier = LanguageIdentifier::from_bytes("pl".as_bytes());
    // // let available_languages = Localizer::available_languages();
    // // println!("{:?}", available_languages);
    // for (lib, localizer) in localizers {
    //     if let Err(error) = localizer.select(&requested_languages) {
    //         eprintln!("Error while loadings languages for {} {:?}", lib, error);
    //     }
    // }
}
