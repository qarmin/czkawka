// Remove console window in Windows OS
#![windows_subsystem = "windows"]

use i18n_embed::unic_langid::LanguageIdentifier;
use i18n_embed::DesktopLanguageRequester;

use std::collections::HashMap;

use i18n_embed::{
    fluent::{fluent_language_loader, FluentLanguageLoader},
    DefaultLocalizer, LanguageLoader, Localizer,
};
use once_cell::sync::Lazy;
use rust_embed::RustEmbed;

fn main() {
    load_system_language(); // Check for default system language, must be loaded after initializing GUI and before loading settings from file
    connect_change_language();
}
pub fn connect_change_language() {
    change_language();
}

fn change_language() {
    println!("Change language");
    let localizers = vec![("czkawka_gui", localizer())];

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

#[derive(RustEmbed)]
#[folder = "i18n/"]
struct Localizations;

pub static LANGUAGE_LOADER: Lazy<FluentLanguageLoader> = Lazy::new(|| {
    let loader: FluentLanguageLoader = fluent_language_loader!();

    loader.load_fallback_language(&Localizations).expect("Error while loading fallback language");

    loader
});

#[macro_export]
macro_rules! fl {
    ($message_id:literal) => {{
        i18n_embed_fl::fl!($crate::localizer::LANGUAGE_LOADER, $message_id)
    }};

    ($message_id:literal, $($args:expr),*) => {{
        i18n_embed_fl::fl!($crate::localizer::LANGUAGE_LOADER, $message_id, $($args), *)
    }};
}

// Get the `Localizer` to be used for localizing this library.
pub fn localizer() -> Box<dyn Localizer> {
    Box::from(DefaultLocalizer::new(&*LANGUAGE_LOADER, &Localizations))
}

pub fn generate_translation_hashmap(vec: Vec<(&'static str, String)>) -> HashMap<&'static str, String> {
    let mut hashmap: HashMap<&'static str, String> = Default::default();
    for (key, value) in vec {
        hashmap.insert(key, value);
    }
    hashmap
}
