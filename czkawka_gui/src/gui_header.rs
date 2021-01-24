use gtk::prelude::*;

#[derive(Clone)]
pub struct GUIHeader {
    pub button_settings: gtk::Button,
    pub button_app_info: gtk::Button,
}

impl GUIHeader {
    pub fn create_from_builder(builder: &gtk::Builder) -> Self {
        let button_settings: gtk::Button = builder.get_object("button_settings").unwrap();
        let button_app_info: gtk::Button = builder.get_object("button_app_info").unwrap();
        Self { button_settings, button_app_info }
    }
}
