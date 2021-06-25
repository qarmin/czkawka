use gtk::prelude::*;
use gtk::TreeView;

#[derive(Clone)]
pub struct GuiUpperNotebook {
    pub notebook_upper: gtk::Notebook,

    pub scrolled_window_included_directories: gtk::ScrolledWindow,
    pub scrolled_window_excluded_directories: gtk::ScrolledWindow,

    pub tree_view_included_directories: gtk::TreeView,
    pub tree_view_excluded_directories: gtk::TreeView,

    pub entry_excluded_items: gtk::Entry,
    pub entry_allowed_extensions: gtk::Entry,

    pub check_button_recursive: gtk::CheckButton,

    pub buttons_manual_add_directory: gtk::Button,
    pub buttons_add_included_directory: gtk::Button,
    pub buttons_remove_included_directory: gtk::Button,
    pub buttons_manual_add_excluded_directory: gtk::Button,
    pub buttons_add_excluded_directory: gtk::Button,
    pub buttons_remove_excluded_directory: gtk::Button,
}

impl GuiUpperNotebook {
    pub fn create_from_builder(builder: &gtk::Builder) -> Self {
        let notebook_upper: gtk::Notebook = builder.object("notebook_upper").unwrap();

        let scrolled_window_included_directories: gtk::ScrolledWindow = builder.object("scrolled_window_included_directories").unwrap();
        let scrolled_window_excluded_directories: gtk::ScrolledWindow = builder.object("scrolled_window_excluded_directories").unwrap();

        let tree_view_included_directories: gtk::TreeView = TreeView::new();
        let tree_view_excluded_directories: gtk::TreeView = TreeView::new();

        let entry_allowed_extensions: gtk::Entry = builder.object("entry_allowed_extensions").unwrap();
        let entry_excluded_items: gtk::Entry = builder.object("entry_excluded_items").unwrap();

        let check_button_recursive: gtk::CheckButton = builder.object("check_button_recursive").unwrap();

        let buttons_manual_add_directory: gtk::Button = builder.object("buttons_manual_add_directory").unwrap();
        let buttons_add_included_directory: gtk::Button = builder.object("buttons_add_included_directory").unwrap();
        let buttons_remove_included_directory: gtk::Button = builder.object("buttons_remove_included_directory").unwrap();
        let buttons_manual_add_excluded_directory: gtk::Button = builder.object("buttons_manual_add_excluded_directory").unwrap();
        let buttons_add_excluded_directory: gtk::Button = builder.object("buttons_add_excluded_directory").unwrap();
        let buttons_remove_excluded_directory: gtk::Button = builder.object("buttons_remove_excluded_directory").unwrap();

        Self {
            notebook_upper,
            scrolled_window_included_directories,
            scrolled_window_excluded_directories,
            tree_view_included_directories,
            tree_view_excluded_directories,
            entry_excluded_items,
            entry_allowed_extensions,
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
