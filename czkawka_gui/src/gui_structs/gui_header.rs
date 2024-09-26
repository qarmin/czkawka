use gtk4::prelude::*;

use crate::help_functions::set_icon_of_button;
use crate::{flg, CZK_ICON_INFO, CZK_ICON_SETTINGS};

#[derive(Clone)]
pub struct GuiHeader {
    pub button_settings: gtk4::Button,
    pub button_app_info: gtk4::Button,
}

impl GuiHeader {
    pub fn create_from_builder(builder: &gtk4::Builder) -> Self {
        let button_settings: gtk4::Button = builder.object("button_settings").expect("Cambalache");
        let button_app_info: gtk4::Button = builder.object("button_app_info").expect("Cambalache");

        set_icon_of_button(&button_settings, CZK_ICON_SETTINGS);
        set_icon_of_button(&button_app_info, CZK_ICON_INFO);

        Self { button_settings, button_app_info }
    }

    pub fn update_language(&self) {
        self.button_settings.set_tooltip_text(Some(&flg!("header_setting_button_tooltip")));
        self.button_app_info.set_tooltip_text(Some(&flg!("header_about_button_tooltip")));
    }
}
