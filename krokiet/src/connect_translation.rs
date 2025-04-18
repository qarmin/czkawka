use i18n_embed::unic_langid::LanguageIdentifier;
use log::error;
use slint::{ComponentHandle, ModelRc, SharedString, VecModel};

use crate::{Callabler, MainWindow, Settings, Translations, flk, localizer_krokiet};

struct Language {
    long_name: &'static str,
    short_name: &'static str,
}

const LANGUAGE_LIST: &[Language] = &[
    Language {
        long_name: "English",
        short_name: "en",
    },
    Language {
        long_name: "Polski (Polish)",
        short_name: "pl",
    },
];

pub fn connect_translations(app: &MainWindow) {
    init_languages(app);
    translate_items(app);

    let a = app.as_weak();
    app.global::<Callabler>().on_changed_language(move || {
        let app = a.upgrade().unwrap();
        change_language(&app);
    });
}

fn change_language(app: &MainWindow) {
    let localizers = vec![
        ("czkawka_core", czkawka_core::localizer_core::localizer_core()),
        ("krokiet", localizer_krokiet::localizer_krokiet()),
    ];

    let lang = app.global::<Settings>().get_language_index();
    let language = LANGUAGE_LIST[lang as usize].short_name;

    let lang_identifier = vec![LanguageIdentifier::from_bytes(language.as_bytes()).expect("Failed to create LanguageIdentifier")];
    for (lib, localizer) in localizers {
        if let Err(error) = localizer.select(&lang_identifier) {
            error!("Error while loadings languages for {lib} {error:?}");
        }
    }

    translate_items(app);
}

fn init_languages(app: &MainWindow) {
    let new_languages_model: Vec<SharedString> = LANGUAGE_LIST.iter().map(|e| e.long_name.into()).collect::<Vec<_>>();

    app.global::<Settings>().set_languages_list(ModelRc::new(VecModel::from(new_languages_model)));
    app.global::<Settings>().set_language_index(0); // TODO loaded from settings
}

// ([a-z_]+):
// translation.set_\1_text(flk!("\1").into());
fn translate_items(app: &MainWindow) {
    let translation = app.global::<Translations>();

    translation.set_yes_button_text(flk!("yes_button").into());
    translation.set_no_button_text(flk!("no_button").into());

    translation.set_scan_button_text(flk!("scan_button").into());
    translation.set_stop_button_text(flk!("stop_button").into());
    translation.set_select_button_text(flk!("select_button").into());
    translation.set_move_button_text(flk!("move_button").into());
    translation.set_delete_button_text(flk!("delete_button").into());
    translation.set_save_button_text(flk!("save_button").into());
    translation.set_sort_button_text(flk!("sort_button").into());
    translation.set_rename_button_text(flk!("rename_button").into());

    translation.set_motto_text(flk!("motto").into());
    translation.set_unicorn_text(flk!("unicorn").into());
    translation.set_repository_text(flk!("repository").into());
    translation.set_instruction_text(flk!("instruction").into());
    translation.set_donation_text(flk!("donation").into());
    translation.set_translation_text(flk!("translation").into());

    translation.set_add_button_text(flk!("add_button").into());
    translation.set_remove_button_text(flk!("remove_button").into());
    translation.set_manual_add_button_text(flk!("manual_add_button").into());
    translation.set_included_directories_text(flk!("included_directories").into());
    translation.set_excluded_directories_text(flk!("excluded_directories").into());
    translation.set_ref_text(flk!("ref").into());
    translation.set_path_text(flk!("path").into());

    translation.set_delete_text(flk!("delete").into());
    translation.set_delete_confirmation_text(flk!("delete_confirmation").into());
}
