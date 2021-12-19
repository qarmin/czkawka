#[derive(Clone)]
pub struct Language {
    pub combo_box_text: &'static str,
    pub short_text: &'static str,
}

<<<<<<< HEAD
pub const LANGUAGES_ALL: [Language; 3] = [
=======
pub const LANGUAGES_ALL: [Language; 2] = [
    Language { combo_box_text: "English", short_text: "en" },
>>>>>>> d1ac2f3c22bf7be86b121153ff8258f3d09085de
    Language {
        combo_box_text: "Polski (Polish)",
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
