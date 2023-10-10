#[derive(Clone)]
pub struct Language {
    pub combo_box_text: &'static str,
    pub short_text: &'static str,
}

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
        combo_box_text: "Česky (Czech)",
        short_text: "cs",
    },
    Language {
        combo_box_text: "Deutsch (German)",
        short_text: "de",
    },
    Language {
        combo_box_text: "やまと (Japanese)",
        short_text: "ja",
    },
    Language {
        combo_box_text: "Português (Portuguese)",
        short_text: "pt",
    },
    Language {
        combo_box_text: "简体中文 (Simplified Chinese)",
        short_text: "zh",
    },
    Language {
        combo_box_text: "Español (Spanish)",
        short_text: "es",
    },
    Language {
        combo_box_text: "Norsk (Norwegian)",
        short_text: "no",
    },
    Language {
        combo_box_text: "Swedish (Svenska)",
        short_text: "sv",
    },
];

pub fn get_language_from_combo_box_text(combo_box_text: &str) -> Language {
    for lang in LANGUAGES_ALL {
        if lang.combo_box_text == combo_box_text {
            return lang;
        }
    }

    panic!("Not found proper text");
}
