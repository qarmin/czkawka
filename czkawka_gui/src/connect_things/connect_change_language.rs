use gtk4::prelude::*;
use i18n_embed::unic_langid::LanguageIdentifier;
use i18n_embed::DesktopLanguageRequester;

use crate::language_functions::get_language_from_combo_box_text;
use crate::{localizer_gui, GuiData, LANGUAGES_ALL};

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
    let localizers = vec![
        ("czkawka_core", czkawka_core::localizer_core::localizer_core()),
        ("czkawka_gui", localizer_gui::localizer_gui()),
    ];

    let lang_short = get_language_from_combo_box_text(&gui_data.settings.combo_box_settings_language.active_text().expect("No active text")).short_text;

    let lang_identifier = vec![LanguageIdentifier::from_bytes(lang_short.as_bytes()).expect("Failed to create LanguageIdentifier")];
    for (lib, localizer) in localizers {
        if let Err(error) = localizer.select(&lang_identifier) {
            eprintln!("Error while loadings languages for {lib} {error:?}");
        }
    }
    gui_data.update_language();
}

pub fn load_system_language(gui_data: &GuiData) {
    let requested_languages = DesktopLanguageRequester::requested_languages();

    if let Some(language) = requested_languages.first() {
        let old_short_lang = language.to_string();
        let mut short_lang = String::new();
        // removes from e.g. en_zb, ending _zd since Czkawka don't support this(maybe could add this in future, but only when)
        for i in old_short_lang.chars() {
            if i.is_ascii_alphabetic() {
                short_lang.push(i);
            } else {
                break;
            }
        }
        // let mut found: bool = false;
        for (index, lang) in LANGUAGES_ALL.iter().enumerate() {
            if lang.short_text == short_lang {
                // found = true;
                gui_data.settings.combo_box_settings_language.set_active(Some(index as u32));
                break;
            }
        }
        // if found {
        //     println!("INFO: Default system language {} is available, so choosing them", short_lang);
        // } else {
        //     println!("INFO: Default system language {} is not available, using English(en) instead", short_lang);
        // }
    }
}
