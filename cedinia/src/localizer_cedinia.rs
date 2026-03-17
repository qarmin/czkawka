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

/// Returns 1 for Polish, 0 for English – determined by the OS locale.
/// Used to pick the default UI index when no explicit language has been saved.
pub(crate) fn detect_os_language_idx() -> i32 {
    #[cfg(not(target_os = "android"))]
    {
        let requested = i18n_embed::DesktopLanguageRequester::requested_languages();
        if requested.iter().any(|l| l.language.as_str() == "pl") {
            return 1;
        }
    }
    0
}

/// Load the given language preference. "auto" uses the OS locale; "pl"/"en" forces a specific
/// language. Call this before `translate_items`.
pub(crate) fn apply_language_preference(lang: &str) {
    let localizer = localizer_cedinia();
    match lang {
        "pl" | "en" => {
            if let Ok(lang_id) = lang.parse::<i18n_embed::unic_langid::LanguageIdentifier>() {
                let _ = localizer.select(&[lang_id]);
            }
        }
        _ => {
            // "auto" – use the OS-requested languages on desktop
            #[cfg(not(target_os = "android"))]
            {
                let requested = i18n_embed::DesktopLanguageRequester::requested_languages();
                let _ = localizer.select(&requested);
            }
        }
    }
}
