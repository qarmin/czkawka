#[derive(Clone)]
pub struct Language {
    pub combo_box_text: &'static str,
    pub short_text: &'static str,
}

/// Languages should be alphabetically sorted
pub const LANGUAGES_ALL: [Language; 11] = [
    Language {
        combo_box_text: "English",
        short_text: "en",
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
        combo_box_text: "عربى (Arabic) - Computer translation",
        short_text: "ar",
    },
    Language {
        combo_box_text: "Deutsch (German) - Computer translation",
        short_text: "de",
    },
    Language {
        combo_box_text: "Français (French) - Computer translation",
        short_text: "fr",
    },
    Language {
        combo_box_text: "やまと (Japanese) - Computer translation",
        short_text: "ja",
    },
    Language {
        combo_box_text: "Português (Portuguese) - Computer translation",
        short_text: "pt",
    },
    Language {
        combo_box_text: "Русский (Russian) - Computer translation",
        short_text: "ru",
    },
    Language {
        combo_box_text: "简体中文 (Simplified Chinese) - Computer translation",
        short_text: "zh",
    },
    Language {
        combo_box_text: "Español (Spanish) - Computer translation",
        short_text: "es",
    },
];

pub fn get_language_from_combo_box_text(combo_box_text: String) -> Language {
    for lang in LANGUAGES_ALL {
        if lang.combo_box_text == combo_box_text {
            return lang;
        }
    }

    panic!("Not found proper text");
}
