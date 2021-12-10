use gtk::prelude::*;

#[derive(Clone)]
pub struct GuiHeader {
    pub button_settings: gtk::Button,
    pub button_app_info: gtk::Button,
    pub check_button_language: gtk::CheckButton,
}

impl GuiHeader {
    pub fn create_from_builder(builder: &gtk::Builder) -> Self {
        let button_settings: gtk::Button = builder.object("button_settings").unwrap();
        let button_app_info: gtk::Button = builder.object("button_app_info").unwrap();
        let check_button_language: gtk::CheckButton = builder.object("check_button_language").unwrap();

        button_settings.set_tooltip_text(Some("Opens settings dialog"));
        button_app_info.set_tooltip_text(Some("Opens dialog with info about app"));
        check_button_language.set_tooltip_text(Some("Use Polish or English language in runtime."));

        Self {
            button_settings,
            button_app_info,
            check_button_language,
        }
    }
}
