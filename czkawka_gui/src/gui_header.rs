use crate::fl;
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

        Self {
            button_settings,
            button_app_info,
            check_button_language,
        }
    }

    pub fn update_language(&self) {
        self.button_settings.set_tooltip_text(Some(&fl!("header_setting_button_tooltip")));
        self.button_app_info.set_tooltip_text(Some(&fl!("header_about_button_tooltip")));
        self.check_button_language.set_tooltip_text(Some(&fl!("header_language_button_tooltip")));
    }
}
