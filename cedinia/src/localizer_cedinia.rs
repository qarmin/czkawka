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

// CJK languages are excluded from auto-detection; the user must select them manually.
const CJK_CODES: &[&str] = &["ko", "ja", "zh-CN", "zh-TW"];

pub(crate) fn detect_os_language_idx() -> i32 {
    #[cfg(not(target_os = "android"))]
    {
        let requested = i18n_embed::DesktopLanguageRequester::requested_languages();
        if let Some(lang) = requested.first() {
            let tag = lang.to_string();
            log::debug!("detect_os_language_idx: OS language tag={tag}");
            return matched_non_cjk_idx(&tag);
        }
    }
    #[cfg(target_os = "android")]
    {
        if let Some(tag) = crate::file_picker_android::get_android_language_tag() {
            log::debug!("detect_os_language_idx: Android language tag={tag}");
            return matched_non_cjk_idx(&tag);
        }
        log::debug!("detect_os_language_idx: could not get Android language, falling back to English");
    }
    0
}

fn matched_non_cjk_idx(tag: &str) -> i32 {
    let idx = czkawka_core::localizer_core::find_language_idx(tag);
    let code = czkawka_core::localizer_core::LANGUAGE_LIST.get(idx).map_or("en", |l| l.short_name);
    if CJK_CODES.contains(&code) {
        log::debug!("detect_os_language_idx: CJK language '{code}' excluded from auto-detection, falling back to English");
        return 0;
    }
    log::debug!("detect_os_language_idx: matched '{code}' at index {idx}");
    idx as i32
}

pub(crate) fn apply_language_preference(lang: &str) {
    let localizer = localizer_cedinia();
    let core_localizer = czkawka_core::localizer_core::localizer_core();
    if czkawka_core::localizer_core::LANGUAGE_LIST.iter().any(|l| l.short_name == lang) {
        log::debug!("apply_language_preference: applying saved language '{lang}'");
        if let Ok(lang_id) = lang.parse::<i18n_embed::unic_langid::LanguageIdentifier>() {
            let _ = localizer.select(std::slice::from_ref(&lang_id));
            let _ = core_localizer.select(&[lang_id]);
        }
    } else {
        log::debug!("apply_language_preference: '{lang}' not in list, detecting OS language");
        #[cfg(not(target_os = "android"))]
        {
            let requested = i18n_embed::DesktopLanguageRequester::requested_languages();
            let _ = localizer.select(&requested);
            let _ = core_localizer.select(&requested);
        }
        #[cfg(target_os = "android")]
        {
            if let Some(tag) = crate::file_picker_android::get_android_language_tag() {
                let idx = czkawka_core::localizer_core::find_language_idx(&tag);
                let lang_code = czkawka_core::localizer_core::LANGUAGE_LIST.get(idx).map(|l| l.short_name).unwrap_or("en");
                log::debug!("apply_language_preference: Android tag={tag}, applying '{lang_code}'");
                if let Ok(lang_id) = lang_code.parse::<i18n_embed::unic_langid::LanguageIdentifier>() {
                    let _ = localizer.select(std::slice::from_ref(&lang_id));
                    let _ = core_localizer.select(&[lang_id]);
                }
            } else {
                log::debug!("apply_language_preference: could not get Android language, staying with English");
            }
        }
    }
}
