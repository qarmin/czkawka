use gtk::prelude::*;
use gtk::TreeView;

#[derive(Clone)]
pub struct GUIUpperNotebook {
    pub notebook_upper: gtk::Notebook,

    pub scrolled_window_included_directories: gtk::ScrolledWindow,
    pub scrolled_window_excluded_directories: gtk::ScrolledWindow,

    pub tree_view_included_directories: gtk::TreeView,
    pub tree_view_excluded_directories: gtk::TreeView,

    pub entry_excluded_items: gtk::Entry,
    pub entry_allowed_extensions: gtk::Entry,

    //// Settings
    pub check_button_settings_save_at_exit: gtk::CheckButton,
    pub check_button_settings_load_at_start: gtk::CheckButton,
    pub check_button_settings_confirm_deletion: gtk::CheckButton,
    pub check_button_settings_show_preview_similar_images: gtk::CheckButton,
    pub check_button_settings_show_text_view: gtk::CheckButton,

    pub button_settings_save_configuration: gtk::Button,
    pub button_settings_load_configuration: gtk::Button,
    pub button_settings_reset_configuration: gtk::Button,

    pub check_button_recursive: gtk::CheckButton,

    pub buttons_manual_add_directory: gtk::Button,
    pub buttons_add_included_directory: gtk::Button,
    pub buttons_remove_included_directory: gtk::Button,
    pub buttons_manual_add_excluded_directory: gtk::Button,
    pub buttons_add_excluded_directory: gtk::Button,
    pub buttons_remove_excluded_directory: gtk::Button,
}

impl GUIUpperNotebook {
    pub fn create_from_builder(builder: &gtk::Builder) -> Self {
        let notebook_upper: gtk::Notebook = builder.get_object("notebook_upper").unwrap();

        let scrolled_window_included_directories: gtk::ScrolledWindow = builder.get_object("scrolled_window_included_directories").unwrap();
        let scrolled_window_excluded_directories: gtk::ScrolledWindow = builder.get_object("scrolled_window_excluded_directories").unwrap();

        let tree_view_included_directories: gtk::TreeView = TreeView::new();
        let tree_view_excluded_directories: gtk::TreeView = TreeView::new();

        let entry_allowed_extensions: gtk::Entry = builder.get_object("entry_allowed_extensions").unwrap();
        let entry_excluded_items: gtk::Entry = builder.get_object("entry_excluded_items").unwrap();

        //// Settings
        let check_button_settings_save_at_exit: gtk::CheckButton = builder.get_object("check_button_settings_save_at_exit").unwrap();
        let check_button_settings_load_at_start: gtk::CheckButton = builder.get_object("check_button_settings_load_at_start").unwrap();
        let check_button_settings_confirm_deletion: gtk::CheckButton = builder.get_object("check_button_settings_confirm_deletion").unwrap();
        let check_button_settings_show_preview_similar_images: gtk::CheckButton = builder.get_object("check_button_settings_show_preview_similar_images").unwrap();
        let check_button_settings_show_text_view: gtk::CheckButton = builder.get_object("check_button_settings_show_text_view").unwrap();

        let button_settings_save_configuration: gtk::Button = builder.get_object("button_settings_save_configuration").unwrap();
        let button_settings_load_configuration: gtk::Button = builder.get_object("button_settings_load_configuration").unwrap();
        let button_settings_reset_configuration: gtk::Button = builder.get_object("button_settings_reset_configuration").unwrap();

        let check_button_recursive: gtk::CheckButton = builder.get_object("check_button_recursive").unwrap();

        let buttons_manual_add_directory: gtk::Button = builder.get_object("buttons_manual_add_directory").unwrap();
        let buttons_add_included_directory: gtk::Button = builder.get_object("buttons_add_included_directory").unwrap();
        let buttons_remove_included_directory: gtk::Button = builder.get_object("buttons_remove_included_directory").unwrap();
        let buttons_manual_add_excluded_directory: gtk::Button = builder.get_object("buttons_manual_add_excluded_directory").unwrap();
        let buttons_add_excluded_directory: gtk::Button = builder.get_object("buttons_add_excluded_directory").unwrap();
        let buttons_remove_excluded_directory: gtk::Button = builder.get_object("buttons_remove_excluded_directory").unwrap();

        Self {
            notebook_upper,
            scrolled_window_included_directories,
            scrolled_window_excluded_directories,
            tree_view_included_directories,
            tree_view_excluded_directories,
            entry_excluded_items,
            entry_allowed_extensions,
            check_button_settings_save_at_exit,
            check_button_settings_load_at_start,
            check_button_settings_confirm_deletion,
            check_button_settings_show_preview_similar_images,
            check_button_settings_show_text_view,
            button_settings_save_configuration,
            button_settings_load_configuration,
            button_settings_reset_configuration,
            check_button_recursive,
            buttons_manual_add_directory,
            buttons_add_included_directory,
            buttons_remove_included_directory,
            buttons_manual_add_excluded_directory,
            buttons_add_excluded_directory,
            buttons_remove_excluded_directory,
        }
    }
}
