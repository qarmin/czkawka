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

pub(crate) fn localizer_cedinia() -> Box<dyn Localizer> {
    Box::from(DefaultLocalizer::new(&*LANGUAGE_LOADER_CEDINIA, &Localizations))
}

/// All supported UI languages in display order. Index 0 is the fallback (English).
pub const LANGUAGE_LIST: &[&str] = &["en", "pl"];

pub(crate) fn detect_os_language_idx() -> i32 {
    #[cfg(not(target_os = "android"))]
    {
        let requested = i18n_embed::DesktopLanguageRequester::requested_languages();
        if let Some(lang) = requested.first() {
            let short = lang.language.as_str();
            for (idx, &code) in LANGUAGE_LIST.iter().enumerate() {
                if short == code {
                    return idx as i32;
                }
            }
        }
    }
    0
}

pub(crate) fn apply_language_preference(lang: &str) {
    let localizer = localizer_cedinia();
    if LANGUAGE_LIST.contains(&lang) {
        if let Ok(lang_id) = lang.parse::<i18n_embed::unic_langid::LanguageIdentifier>() {
            let _ = localizer.select(&[lang_id]);
        }
    } else {
        // "auto" or unknown → use system language
        #[cfg(not(target_os = "android"))]
        {
            let requested = i18n_embed::DesktopLanguageRequester::requested_languages();
            let _ = localizer.select(&requested);
        }
    }
}
