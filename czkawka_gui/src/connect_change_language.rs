use crate::language_functions::get_language_from_combo_box_text;
use crate::GuiData;
use gtk::prelude::*;
use i18n_embed::unic_langid::LanguageIdentifier;
// use i18n_embed::{DesktopLanguageRequester, Localizer};

pub fn connect_change_language(gui_data: &GuiData) {
    change_language(gui_data);

    let combo_box_settings_language = gui_data.settings.combo_box_settings_language.clone();
    let gui_data = gui_data.clone();
    combo_box_settings_language.connect_changed(move |_| {
        change_language(&gui_data);
    });
}

fn change_language(gui_data: &GuiData) {
    let localizers = vec![("czkawka_gui", czkawka_core::localizer::localizer())];

    let lang_short = get_language_from_combo_box_text(gui_data.settings.combo_box_settings_language.active_text().unwrap().to_string()).short_text;

    let lang_identifier = vec![LanguageIdentifier::from_bytes(lang_short.as_bytes()).unwrap()];
    // let available_languages = Localizer::available_languages();
    // println!("{:?}", available_languages);
    for (lib, localizer) in localizers {
        if let Err(error) = localizer.select(&lang_identifier) {
            eprintln!("Error while loadings languages for {} {:?}", lib, error);
        }
    }
    gui_data.update_language();

    // Try to use default OS language
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
