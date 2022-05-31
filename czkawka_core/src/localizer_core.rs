use std::collections::HashMap;

use i18n_embed::{
    fluent::{fluent_language_loader, FluentLanguageLoader},
    DefaultLocalizer, LanguageLoader, Localizer,
};
use once_cell::sync::Lazy;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "i18n/"]
struct Localizations;

pub static LANGUAGE_LOADER_CORE: Lazy<FluentLanguageLoader> = Lazy::new(|| {
    let loader: FluentLanguageLoader = fluent_language_loader!();

    loader.load_fallback_language(&Localizations).expect("Error while loading fallback language");

    loader
});

#[macro_export]
macro_rules! flc {
    ($message_id:literal) => {{
        i18n_embed_fl::fl!($crate::localizer_core::LANGUAGE_LOADER_CORE, $message_id)
    }};

    ($message_id:literal, $($args:expr),*) => {{
        i18n_embed_fl::fl!($crate::localizer_core::LANGUAGE_LOADER_CORE, $message_id, $($args), *)
    }};
}

// Get the `Localizer` to be used for localizing this library.
pub fn localizer_core() -> Box<dyn Localizer> {
    Box::from(DefaultLocalizer::new(&*LANGUAGE_LOADER_CORE, &Localizations))
}

pub fn generate_translation_hashmap(vec: Vec<(&'static str, String)>) -> HashMap<&'static str, String> {
    let mut hashmap: HashMap<&'static str, String> = Default::default();
    for (key, value) in vec {
        hashmap.insert(key, value);
    }
    hashmap
}

pub fn fnc_get_similarity_very_high() -> String {
    flc!("core_similarity_very_high")
}

pub fn fnc_get_similarity_minimal() -> String {
    flc!("core_similarity_minimal")
}
