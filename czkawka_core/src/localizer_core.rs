use std::collections::HashMap;

use i18n_embed::fluent::{FluentLanguageLoader, fluent_language_loader};
use i18n_embed::{DefaultLocalizer, LanguageLoader, Localizer};
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "i18n/"]
struct Localizations;

pub static LANGUAGE_LOADER_CORE: std::sync::LazyLock<FluentLanguageLoader> = std::sync::LazyLock::new(|| {
    let loader: FluentLanguageLoader = fluent_language_loader!();

    loader.load_fallback_language(&Localizations).expect("Error while loading fallback language");

    loader
});

#[macro_export]
macro_rules! flc {
    ( $($tt:tt)* ) => {{
        i18n_embed_fl::fl!($crate::localizer_core::LANGUAGE_LOADER_CORE, $($tt)*)
    }};
}

pub fn localizer_core() -> Box<dyn Localizer> {
    Box::from(DefaultLocalizer::new(&*LANGUAGE_LOADER_CORE, &Localizations))
}

/// A supported UI language.
pub struct Language {
    pub long_name: &'static str,
    pub short_name: &'static str,
}

pub const LANGUAGE_LIST: &[Language] = &[
    Language {
        long_name: "English",
        short_name: "en",
    },
    Language {
        long_name: "Polski (Polish)",
        short_name: "pl",
    },
    Language {
        long_name: "Français (French)",
        short_name: "fr",
    },
    Language {
        long_name: "Italiano (Italian)",
        short_name: "it",
    },
    Language {
        long_name: "Русский (Russian)",
        short_name: "ru",
    },
    Language {
        long_name: "український (Ukrainian)",
        short_name: "uk",
    },
    Language {
        long_name: "한국어 (Korean)",
        short_name: "ko",
    },
    Language {
        long_name: "Česky (Czech)",
        short_name: "cs",
    },
    Language {
        long_name: "Deutsch (German)",
        short_name: "de",
    },
    Language {
        long_name: "日本語 (Japanese)",
        short_name: "ja",
    },
    Language {
        long_name: "Português (Portuguese)",
        short_name: "pt-PT",
    },
    Language {
        long_name: "Português Brasileiro (Brazilian Portuguese)",
        short_name: "pt-BR",
    },
    Language {
        long_name: "简体中文 (Simplified Chinese)",
        short_name: "zh-CN",
    },
    Language {
        long_name: "繁體中文 (Traditional Chinese)",
        short_name: "zh-TW",
    },
    Language {
        long_name: "Español (Spanish)",
        short_name: "es-ES",
    },
    Language {
        long_name: "Norsk (Norwegian)",
        short_name: "no",
    },
    Language {
        long_name: "Svenska (Swedish)",
        short_name: "sv-SE",
    },
    Language {
        long_name: "العربية (Arabic)",
        short_name: "ar",
    },
    Language {
        long_name: "Български (Bulgarian)",
        short_name: "bg",
    },
    Language {
        long_name: "Ελληνικά (Greek)",
        short_name: "el",
    },
    Language {
        long_name: "Nederlands (Dutch)",
        short_name: "nl",
    },
    Language {
        long_name: "Română (Romanian)",
        short_name: "ro",
    },
    Language {
        long_name: "Türkçe (Turkish)",
        short_name: "tr",
    },
    Language {
        long_name: "فارسی (Persian)",
        short_name: "fa",
    },
    Language {
        long_name: "हिंदी (Hindi)",
        short_name: "hi",
    },
    Language {
        long_name: "Bahasa Indonesia (Indonesian)",
        short_name: "id",
    },
    Language {
        long_name: "Tiếng Việt (Vietnamese)",
        short_name: "vi",
    },
];

/// Find the best-matching index in [`LANGUAGE_LIST`] for a BCP 47 tag.
///
/// Pass 1: exact match (handles `pt-BR`, `pt-PT`, `es-ES`, `sv-SE`, …).
/// Pass 2: Chinese script disambiguation (`zh-Hans-*` → `zh-CN`, `zh-Hant-*` → `zh-TW`).
/// Pass 3: language subtag only (`pl-PL` → `pl`, `de-AT` → `de`).
/// Returns 0 (English) if no match is found.
pub fn find_language_idx(tag: &str) -> usize {
    for (idx, lang) in LANGUAGE_LIST.iter().enumerate() {
        if tag.eq_ignore_ascii_case(lang.short_name) {
            return idx;
        }
    }

    let lang_only = tag.split('-').next().unwrap_or(tag);

    if lang_only.eq_ignore_ascii_case("zh") {
        let target = if tag.contains("Hans") || tag.contains("CN") || tag.contains("SG") {
            "zh-CN"
        } else {
            "zh-TW"
        };
        if let Some(idx) = LANGUAGE_LIST.iter().position(|l| l.short_name == target) {
            return idx;
        }
    }

    for (idx, lang) in LANGUAGE_LIST.iter().enumerate() {
        let name_lang = lang.short_name.split('-').next().unwrap_or(lang.short_name);
        if lang_only.eq_ignore_ascii_case(name_lang) {
            return idx;
        }
    }

    0
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
