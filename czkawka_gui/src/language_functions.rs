#[derive(Clone)]
pub struct Language {
    pub combo_box_text: &'static str,
    pub short_text: &'static str,
}

/// Languages should be alphabetically sorted
pub const LANGUAGES_ALL: [Language; 5] = [
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
        combo_box_text: "Deutsch (German) - Computer translation",
        short_text: "de",
    },
    Language {
        combo_box_text: "Português (Portuguese) - Computer translation",
        short_text: "pt",
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
