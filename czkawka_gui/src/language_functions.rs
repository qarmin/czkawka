#[derive(Clone)]
pub struct Language {
    pub combo_box_text: &'static str,
    pub short_text: &'static str,
}

pub const LANGUAGES_ALL: [Language; 2] = [
    Language { combo_box_text: "English", short_text: "en" },
    Language {
        combo_box_text: "Polski (Polish)",
        short_text: "pl",
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
