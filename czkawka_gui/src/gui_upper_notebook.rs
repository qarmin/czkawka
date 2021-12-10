use crate::fl;
use crate::help_functions::get_custom_label_from_label_with_image;
use crate::notebook_enums::NotebookUpperEnum;
use gtk::prelude::*;
use gtk::{Bin, EventControllerKey, TreeView};

#[derive(Clone)]
pub struct GuiUpperNotebook {
    pub notebook_upper: gtk::Notebook,

    pub scrolled_window_included_directories: gtk::ScrolledWindow,
    pub scrolled_window_excluded_directories: gtk::ScrolledWindow,

    pub tree_view_included_directories: gtk::TreeView,
    pub tree_view_excluded_directories: gtk::TreeView,

    pub evk_tree_view_included_directories: gtk::EventControllerKey,
    pub evk_tree_view_excluded_directories: gtk::EventControllerKey,

    pub entry_excluded_items: gtk::Entry,
    pub entry_allowed_extensions: gtk::Entry,

    pub check_button_recursive: gtk::CheckButton,

    pub buttons_manual_add_included_directory: gtk::Button,
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

        let evk_tree_view_included_directories: gtk::EventControllerKey = EventControllerKey::new(&tree_view_included_directories);
        let evk_tree_view_excluded_directories: gtk::EventControllerKey = EventControllerKey::new(&tree_view_excluded_directories);

        let entry_allowed_extensions: gtk::Entry = builder.object("entry_allowed_extensions").unwrap();
        let entry_excluded_items: gtk::Entry = builder.object("entry_excluded_items").unwrap();

        let check_button_recursive: gtk::CheckButton = builder.object("check_button_recursive").unwrap();

        let buttons_manual_add_included_directory: gtk::Button = builder.object("buttons_manual_add_included_directory").unwrap();
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
            evk_tree_view_included_directories,
            evk_tree_view_excluded_directories,
            entry_excluded_items,
            entry_allowed_extensions,
            check_button_recursive,
            buttons_manual_add_included_directory,
            buttons_add_included_directory,
            buttons_remove_included_directory,
            buttons_manual_add_excluded_directory,
            buttons_add_excluded_directory,
            buttons_remove_excluded_directory,
        }
    }
    pub fn update_language(&self) {
        self.check_button_recursive.set_label(&fl!("upper_recursive_button"));
        self.check_button_recursive.set_tooltip_text(Some(&fl!("upper_recursive_button_tooltip")));

        get_custom_label_from_label_with_image(&self.buttons_manual_add_included_directory.clone().upcast::<Bin>()).set_text(&fl!("upper_manual_add_included_button"));
        get_custom_label_from_label_with_image(&self.buttons_add_included_directory.clone().upcast::<Bin>()).set_text(&fl!("upper_add_included_button"));
        get_custom_label_from_label_with_image(&self.buttons_remove_included_directory.clone().upcast::<Bin>()).set_text(&fl!("upper_remove_included_button"));
        get_custom_label_from_label_with_image(&self.buttons_manual_add_excluded_directory.clone().upcast::<Bin>()).set_text(&fl!("upper_manual_add_excluded_button"));
        get_custom_label_from_label_with_image(&self.buttons_add_excluded_directory.clone().upcast::<Bin>()).set_text(&fl!("upper_add_excluded_button"));
        get_custom_label_from_label_with_image(&self.buttons_remove_excluded_directory.clone().upcast::<Bin>()).set_text(&fl!("upper_remove_excluded_button"));

        self.buttons_manual_add_included_directory.set_tooltip_text(Some(&fl!("upper_manual_add_included_button_tooltip")));
        self.buttons_add_included_directory.set_tooltip_text(Some(&fl!("upper_add_included_button_tooltip")));
        self.buttons_remove_included_directory.set_tooltip_text(Some(&fl!("upper_remove_included_button_tooltip")));
        self.buttons_manual_add_excluded_directory.set_tooltip_text(Some(&fl!("upper_manual_add_excluded_button_tooltip")));
        self.buttons_add_excluded_directory.set_tooltip_text(Some(&fl!("upper_add_excluded_button_tooltip")));
        self.buttons_remove_excluded_directory.set_tooltip_text(Some(&fl!("upper_remove_excluded_button_tooltip")));

        let vec_children: Vec<gtk::Widget> = self.notebook_upper.children().into_iter().map(|e| e).collect();

        for (upper_enum, fl_thing) in [
            (NotebookUpperEnum::AllowedExtensions as usize, fl!("upper_notebook_allowed_extension")),
            (NotebookUpperEnum::ExcludedItems as usize, fl!("upper_notebook_excluded_items")),
            (NotebookUpperEnum::ExcludedDirectories as usize, fl!("upper_notebook_excluded_directories")),
            (NotebookUpperEnum::IncludedDirectories as usize, fl!("upper_notebook_included_directories")),
        ] {
            self.notebook_upper.tab_label(&vec_children[upper_enum]).unwrap().downcast::<gtk::Label>().unwrap().set_text(&fl_thing);
        }
    }
}
