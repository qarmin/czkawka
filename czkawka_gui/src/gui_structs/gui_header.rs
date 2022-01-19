use gtk::prelude::*;

use crate::flg;

#[derive(Clone)]
pub struct GuiHeader {
    pub button_settings: gtk::Button,
    pub button_app_info: gtk::Button,
}

impl GuiHeader {
    pub fn create_from_builder(builder: &gtk::Builder) -> Self {
        let button_settings: gtk::Button = builder.object("button_settings").unwrap();
        let button_app_info: gtk::Button = builder.object("button_app_info").unwrap();

        Self { button_settings, button_app_info }
    }

    pub fn update_language(&self) {
        self.button_settings.set_tooltip_text(Some(&flg!("header_setting_button_tooltip")));
        self.button_app_info.set_tooltip_text(Some(&flg!("header_about_button_tooltip")));
    }
}
