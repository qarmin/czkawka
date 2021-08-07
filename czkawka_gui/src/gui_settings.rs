use gtk::prelude::*;
use gtk::{Builder, WindowPosition};

#[derive(Clone)]
pub struct GuiSettings {
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
    pub entry_settings_cache_file_minimal_size: gtk::Entry,
    pub check_button_settings_show_preview_duplicates: gtk::CheckButton,

    // Similar Images
    pub check_button_settings_show_preview_similar_images: gtk::CheckButton,

    // Buttons
    pub button_settings_save_configuration: gtk::Button,
    pub button_settings_load_configuration: gtk::Button,
    pub button_settings_reset_configuration: gtk::Button,
}

impl GuiSettings {
    pub fn create_from_builder() -> Self {
        let glade_src = include_str!("../ui/settings.glade").to_string();
        let builder = Builder::from_string(glade_src.as_str());

        let window_settings: gtk::Window = builder.object("window_settings").unwrap();
        window_settings.set_position(WindowPosition::Center);

        // General
        let check_button_settings_save_at_exit: gtk::CheckButton = builder.object("check_button_settings_save_at_exit").unwrap();
        let check_button_settings_load_at_start: gtk::CheckButton = builder.object("check_button_settings_load_at_start").unwrap();
        let check_button_settings_confirm_deletion: gtk::CheckButton = builder.object("check_button_settings_confirm_deletion").unwrap();
        let check_button_settings_confirm_group_deletion: gtk::CheckButton = builder.object("check_button_settings_confirm_group_deletion").unwrap();
        let check_button_settings_show_text_view: gtk::CheckButton = builder.object("check_button_settings_show_text_view").unwrap();
        let check_button_settings_use_cache: gtk::CheckButton = builder.object("check_button_settings_use_cache").unwrap();
        let check_button_settings_use_trash: gtk::CheckButton = builder.object("check_button_settings_use_trash").unwrap();

        // Duplicates
        let check_button_settings_hide_hard_links: gtk::CheckButton = builder.object("check_button_settings_hide_hard_links").unwrap();
        let entry_settings_cache_file_minimal_size: gtk::Entry = builder.object("entry_settings_cache_file_minimal_size").unwrap();
        let check_button_settings_show_preview_duplicates: gtk::CheckButton = builder.object("check_button_settings_show_preview_duplicates").unwrap();

        // Similar Images
        let check_button_settings_show_preview_similar_images: gtk::CheckButton = builder.object("check_button_settings_show_preview_similar_images").unwrap();

        // Saving/Loading/Resetting configuration
        let button_settings_save_configuration: gtk::Button = builder.object("button_settings_save_configuration").unwrap();
        let button_settings_load_configuration: gtk::Button = builder.object("button_settings_load_configuration").unwrap();
        let button_settings_reset_configuration: gtk::Button = builder.object("button_settings_reset_configuration").unwrap();

        Self {
            window_settings,
            check_button_settings_save_at_exit,
            check_button_settings_load_at_start,
            check_button_settings_confirm_deletion,
            check_button_settings_confirm_group_deletion,
            check_button_settings_show_text_view,
            check_button_settings_use_cache,
            check_button_settings_use_trash,
            check_button_settings_hide_hard_links,
            entry_settings_cache_file_minimal_size,
            check_button_settings_show_preview_duplicates,
            check_button_settings_show_preview_similar_images,
            button_settings_save_configuration,
            button_settings_load_configuration,
            button_settings_reset_configuration,
        }
    }
}
