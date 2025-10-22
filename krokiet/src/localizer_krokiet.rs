use i18n_embed::fluent::{FluentLanguageLoader, fluent_language_loader};
use i18n_embed::{DefaultLocalizer, LanguageLoader, Localizer};
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "i18n/"]
struct Localizations;

pub static LANGUAGE_LOADER_KROKIET: std::sync::LazyLock<FluentLanguageLoader> = std::sync::LazyLock::new(|| {
    let loader: FluentLanguageLoader = fluent_language_loader!();

    loader.load_fallback_language(&Localizations).expect("Error while loading fallback language");

    loader
});

#[macro_export]
macro_rules! flk {
    ( $($tt:tt)* ) => {{
        i18n_embed_fl::fl!($crate::localizer_krokiet::LANGUAGE_LOADER_KROKIET, $($tt)*)
    }};
}

// Get the `Localizer` to be used for localizing this library.
pub(crate) fn localizer_krokiet() -> Box<dyn Localizer> {
    Box::from(DefaultLocalizer::new(&*LANGUAGE_LOADER_KROKIET, &Localizations))
}
