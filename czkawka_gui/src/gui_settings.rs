use gtk::prelude::*;

#[derive(Clone)]
pub struct GUISettings {
    pub window_settings: gtk::Window,

    // General
    pub check_button_settings_save_at_exit: gtk::CheckButton,
    pub check_button_settings_load_at_start: gtk::CheckButton,
    pub check_button_settings_confirm_deletion: gtk::CheckButton,
    pub check_button_settings_confirm_group_deletion: gtk::CheckButton,
    pub check_button_settings_show_text_view: gtk::CheckButton,
    pub check_button_settings_use_cache: gtk::CheckButton,
    pub check_button_settings_use_trash: gtk::CheckButton,

    // Duplicates
    pub check_button_settings_hide_hard_links: gtk::CheckButton,

    // Similar Images
    pub check_button_settings_show_preview_similar_images: gtk::CheckButton,

    // Buttons
    pub button_settings_save_configuration: gtk::Button,
    pub button_settings_load_configuration: gtk::Button,
    pub button_settings_reset_configuration: gtk::Button,
}

impl GUISettings {
    pub fn create_from_builder(builder: &gtk::Builder) -> Self {
        let window_settings: gtk::Window = builder.get_object("window_settings").unwrap();

        // General
        let check_button_settings_save_at_exit: gtk::CheckButton = builder.get_object("check_button_settings_save_at_exit").unwrap();
        let check_button_settings_load_at_start: gtk::CheckButton = builder.get_object("check_button_settings_load_at_start").unwrap();
        let check_button_settings_confirm_deletion: gtk::CheckButton = builder.get_object("check_button_settings_confirm_deletion").unwrap();
        let check_button_settings_confirm_group_deletion: gtk::CheckButton = builder.get_object("check_button_settings_confirm_group_deletion").unwrap();
        let check_button_settings_show_text_view: gtk::CheckButton = builder.get_object("check_button_settings_show_text_view").unwrap();
        let check_button_settings_use_cache: gtk::CheckButton = builder.get_object("check_button_settings_use_cache").unwrap();
        let check_button_settings_use_trash: gtk::CheckButton = builder.get_object("check_button_settings_use_trash").unwrap();

        // Duplicates
        let check_button_settings_hide_hard_links: gtk::CheckButton = builder.get_object("check_button_settings_hide_hard_links").unwrap();

        // Similar Images
        let check_button_settings_show_preview_similar_images: gtk::CheckButton = builder.get_object("check_button_settings_show_preview_similar_images").unwrap();

        // Saving/Loading/Resetting configuration
        let button_settings_save_configuration: gtk::Button = builder.get_object("button_settings_save_configuration").unwrap();
        let button_settings_load_configuration: gtk::Button = builder.get_object("button_settings_load_configuration").unwrap();
        let button_settings_reset_configuration: gtk::Button = builder.get_object("button_settings_reset_configuration").unwrap();

        Self {
            window_settings,
            check_button_settings_save_at_exit,
            check_button_settings_load_at_start,
            check_button_settings_confirm_deletion,
            check_button_settings_confirm_group_deletion,
            check_button_settings_show_text_view,
            button_settings_save_configuration,
            button_settings_load_configuration,
            button_settings_reset_configuration,
            check_button_settings_show_preview_similar_images,
            check_button_settings_hide_hard_links,
            check_button_settings_use_cache,
            check_button_settings_use_trash,
        }
    }
}
