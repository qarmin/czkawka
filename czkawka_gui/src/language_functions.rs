#[derive(Clone)]
pub struct Language {
    pub combo_box_text: &'static str,
    pub short_text: &'static str,
}

pub const LANGUAGES_ALL: [Language; 3] = [
    Language {
        combo_box_text: "English (en)",
        short_text: "en",
    },
    Language {
        combo_box_text: "Polski (pl)",
        short_text: "pl",
    },
    Language {
        combo_box_text: "Italian (it)",
        short_text: "it",
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
