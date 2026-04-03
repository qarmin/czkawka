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

pub const LANGUAGE_LIST: &[(&str, &str)] = &[
    ("en", "English"),
    ("pl", "Polski (Polish)"),
    ("fr", "Français (French)"),
    ("it", "Italiano (Italian)"),
    ("ru", "Русский (Russian)"),
    ("uk", "український (Ukrainian)"),
    ("ko", "한국어 (Korean)"),
    ("cs", "Česky (Czech)"),
    ("de", "Deutsch (German)"),
    ("ja", "日本語 (Japanese)"),
    ("pt-PT", "Português (Portuguese)"),
    ("pt-BR", "Português Brasileiro (Brazilian Portuguese)"),
    ("zh-CN", "简体中文 (Simplified Chinese)"),
    ("zh-TW", "繁體中文 (Traditional Chinese)"),
    ("es-ES", "Español (Spanish)"),
    ("no", "Norsk (Norwegian)"),
    ("sv-SE", "Svenska (Swedish)"),
    ("ar", "العربية (Arabic)"),
    ("bg", "Български (Bulgarian)"),
    ("el", "Ελληνικά (Greek)"),
    ("nl", "Nederlands (Dutch)"),
    ("ro", "Română (Romanian)"),
    ("tr", "Türkçe (Turkish)"),
    ("fa", "فارسی (Persian)"),
    ("hi", "हिंदी (Hindi)"),
    ("id", "Bahasa Indonesia (Indonesian)"),
    ("vi", "Tiếng Việt (Vietnamese)"),
];

pub(crate) fn detect_os_language_idx() -> i32 {
    #[cfg(not(target_os = "android"))]
    {
        let requested = i18n_embed::DesktopLanguageRequester::requested_languages();
        if let Some(lang) = requested.first() {
            let short = lang.language.as_str();
            for (idx, &(code, _)) in LANGUAGE_LIST.iter().enumerate() {
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
    if LANGUAGE_LIST.iter().any(|&(code, _)| code == lang) {
        if let Ok(lang_id) = lang.parse::<i18n_embed::unic_langid::LanguageIdentifier>() {
            let _ = localizer.select(&[lang_id]);
        }
    } else {
        #[cfg(not(target_os = "android"))]
        {
            let requested = i18n_embed::DesktopLanguageRequester::requested_languages();
            let _ = localizer.select(&requested);
        }
    }
}
