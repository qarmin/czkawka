use std::collections::HashMap;

use slint::{ComponentHandle, Model};

use crate::localizer_krokiet::LANGUAGE_LOADER_GUI;
use crate::{Callabler, MainWindow};

pub fn connect_translations(app: &MainWindow) {
    app.global::<Callabler>().on_translate(move |text_to_translate, args| {
        let text_to_translate = text_to_translate.to_string();

        let mut arguments = HashMap::new();
        args.iter().for_each(|(key, value)| {
            arguments.insert(key.to_string(), value.to_string());
        });

        if arguments.is_empty() {
            LANGUAGE_LOADER_GUI.get(&text_to_translate)
        } else {
            LANGUAGE_LOADER_GUI.get_args(&text_to_translate, arguments)
        }
        .into()
    });
}
