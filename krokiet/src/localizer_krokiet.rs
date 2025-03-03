use i18n_embed::LanguageLoader;
use i18n_embed::fluent::{FluentLanguageLoader, fluent_language_loader};
use once_cell::sync::Lazy;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "i18n/"]
struct Localizations;

pub static LANGUAGE_LOADER_GUI: Lazy<FluentLanguageLoader> = Lazy::new(|| {
    let loader: FluentLanguageLoader = fluent_language_loader!();

    loader.load_fallback_language(&Localizations).expect("Error while loading fallback language");

    loader
});

#[macro_export]
macro_rules! flk {
    ($message_id:literal) => {{
        i18n_embed_fl::fl!($crate::localizer_krokiet::LANGUAGE_LOADER_GUI, $message_id)
    }};

    ($message_id:literal, $($args:expr_2021),*) => {{
        i18n_embed_fl::fl!($crate::localizer_krokiet::LANGUAGE_LOADER_GUI, $message_id, $($args), *)
    }};
}

// // Get the `Localizer` to be used for localizing this library.
// pub fn localizer_krokiet() -> Box<dyn Localizer> {
//     Box::from(DefaultLocalizer::new(&*LANGUAGE_LOADER_GUI, &Localizations))
// }
