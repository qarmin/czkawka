// Remove console window in Windows OS
#![windows_subsystem = "windows"]

use i18n_embed::unic_langid::LanguageIdentifier;
use i18n_embed::DesktopLanguageRequester;

fn main() {
    load_system_language(); // Check for default system language, must be loaded after initializing GUI and before loading settings from file
    connect_change_language();
}
pub fn connect_change_language() {
    change_language();
}

fn change_language() {
    println!("Change language");
    let localizers = vec![("czkawka_gui", czkawka_core::localizer::localizer())];

    let lang_identifier = vec![LanguageIdentifier::from_bytes("en".as_bytes()).unwrap()];
    for (lib, localizer) in localizers {
        if let Err(error) = localizer.select(&lang_identifier) {
            eprintln!("Error while loadings languages for {} {:?}", lib, error);
        }
    }
}

pub fn load_system_language() {
    println!("Load system language");
    let requested_languages = DesktopLanguageRequester::requested_languages();

    println!("requested_languages - {:?} ({})", requested_languages, requested_languages.len());

    if let Some(language) = requested_languages.get(0) {
        let old_short_lang = language.to_string();
        let mut short_lang = "".to_string();
        // removes from e.g. en_zb, ending _zd since Czkawka don't support this(maybe could add this in future, but only when)
        for i in old_short_lang.chars() {
            if i.is_ascii_alphabetic() {
                short_lang.push(i)
            } else {
                break;
            }
        }
    }
}
