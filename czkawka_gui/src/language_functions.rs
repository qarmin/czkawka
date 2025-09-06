#[derive(Clone)]
pub struct Language {
    pub combo_box_text: &'static str,
    pub short_text: &'static str,
}

pub const LANGUAGES_ALL: &[Language] = &[
    Language {
        combo_box_text: "English",
        short_text: "en",
    },
    Language {
        combo_box_text: "Français (French)",
        short_text: "fr",
    },
    Language {
        combo_box_text: "Italiano (Italian)",
        short_text: "it",
    },
    Language {
        combo_box_text: "Polski (Polish)",
        short_text: "pl",
    },
    Language {
        combo_box_text: "Русский (Russian)",
        short_text: "ru",
    },
    Language {
        combo_box_text: "український (Ukrainian)",
        short_text: "uk",
    },
    Language {
        combo_box_text: "한국어 (Korean)",
        short_text: "ko",
    },
    Language {
        combo_box_text: "Česky (Czech)",
        short_text: "cs",
    },
    Language {
        combo_box_text: "Deutsch (German)",
        short_text: "de",
    },
    Language {
        combo_box_text: "日本語 (Japanese)",
        short_text: "ja",
    },
    Language {
        combo_box_text: "Português (Portuguese)",
        short_text: "pt-PT",
    },
    Language {
        combo_box_text: "Português Brasileiro (Brazilian Portuguese)",
        short_text: "pt-BR",
    },
    Language {
        combo_box_text: "简体中文 (Simplified Chinese)",
        short_text: "zh-CN",
    },
    Language {
        combo_box_text: "繁體中文 (Traditional Chinese)",
        short_text: "zh-TW",
    },
    Language {
        combo_box_text: "Español (Spanish)",
        short_text: "es-ES",
    },
    Language {
        combo_box_text: "Norsk (Norwegian)",
        short_text: "no",
    },
    Language {
        combo_box_text: "Svenska (Swedish)",
        short_text: "sv-SE",
    },
    Language {
        combo_box_text: "العربية (Arabic)",
        short_text: "ar",
    },
    Language {
        combo_box_text: "Български (Bulgarian)",
        short_text: "bg",
    },
    Language {
        combo_box_text: "Ελληνικά (Greek)",
        short_text: "el",
    },
    Language {
        combo_box_text: "Nederlands (Dutch)",
        short_text: "nl",
    },
    Language {
        combo_box_text: "Română (Romanian)",
        short_text: "ro",
    },
];

pub(crate) fn get_language_from_combo_box_text(combo_box_text: &str) -> Language {
    for lang in LANGUAGES_ALL {
        if lang.combo_box_text == combo_box_text {
            return lang.clone();
        }
    }

    panic!("Not found proper text"); // Must be valid, because it is loaded from gui, not from untrusted source
}
