use i18n_embed::fluent::{FluentLanguageLoader, fluent_language_loader};
use i18n_embed::{DefaultLocalizer, LanguageLoader, Localizer};
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "i18n/"]
struct Localizations;

pub static LANGUAGE_LOADER_CEDINIA: std::sync::LazyLock<FluentLanguageLoader> = std::sync::LazyLock::new(|| {
    let loader: FluentLanguageLoader = fluent_language_loader!();
    loader.load_fallback_language(&Localizations).expect("Error while loading fallback language for cedinia");
    loader
});

#[macro_export]
macro_rules! flc {
    ( $($tt:tt)* ) => {{
        i18n_embed_fl::fl!($crate::localizer_cedinia::LANGUAGE_LOADER_CEDINIA, $($tt)*)
    }};
}

#[expect(dead_code)]
pub(crate) fn localizer_cedinia() -> Box<dyn Localizer> {
    Box::from(DefaultLocalizer::new(&*LANGUAGE_LOADER_CEDINIA, &Localizations))
}
