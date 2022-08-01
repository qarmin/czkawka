#[derive(Clone)]
pub struct Language {
    pub combo_box_text: &'static str,
    pub short_text: &'static str,
}

/// Languages should be alphabetically sorted
pub const LANGUAGES_ALL: [Language; 15] = [
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
        combo_box_text: "한국인 (Korean)",
        short_text: "ko",
    },
    Language {
        combo_box_text: "Česky (Czech) - Computer translation",
        short_text: "cs",
    },
    Language {
        combo_box_text: "Deutsch (German) - Computer translation",
        short_text: "de",
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
        combo_box_text: "简体中文 (Simplified Chinese) - Computer translation",
        short_text: "zh",
    },
    Language {
        combo_box_text: "Español (Spanish) - Computer translation",
        short_text: "es",
    },
    Language {
        combo_box_text: "Norsk (Norwegian) - Computer translation",
        short_text: "no",
    },
    Language {
        combo_box_text: "Swedish (Svenska) - Computer translation",
        short_text: "sv",
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
