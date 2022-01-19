use gtk::prelude::*;
use gtk::{Bin, EventControllerKey, TreeView};

use crate::flg;
use crate::help_functions::get_custom_label_from_button_with_image;
use crate::notebook_enums::NotebookUpperEnum;

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

    pub label_excluded_items: gtk::Label,
    pub label_allowed_extensions: gtk::Label,

    pub entry_general_minimal_size: gtk::Entry,
    pub entry_general_maximal_size: gtk::Entry,
    pub label_general_size_bytes: gtk::Label,
    pub label_general_min_size: gtk::Label,
    pub label_general_max_size: gtk::Label,
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

        let label_excluded_items: gtk::Label = builder.object("label_excluded_items").unwrap();
        let label_allowed_extensions: gtk::Label = builder.object("label_allowed_extensions").unwrap();

        let entry_general_minimal_size: gtk::Entry = builder.object("entry_general_minimal_size").unwrap();
        let entry_general_maximal_size: gtk::Entry = builder.object("entry_general_maximal_size").unwrap();
        let label_general_size_bytes: gtk::Label = builder.object("label_general_size_bytes").unwrap();
        let label_general_min_size: gtk::Label = builder.object("label_general_min_size").unwrap();
        let label_general_max_size: gtk::Label = builder.object("label_general_max_size").unwrap();

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
            label_excluded_items,
            label_allowed_extensions,
            entry_general_minimal_size,
            entry_general_maximal_size,
            label_general_size_bytes,
            label_general_min_size,
            label_general_max_size,
        }
    }
    pub fn update_language(&self) {
        self.check_button_recursive.set_label(&flg!("upper_recursive_button"));
        self.check_button_recursive.set_tooltip_text(Some(&flg!("upper_recursive_button_tooltip")));

        get_custom_label_from_button_with_image(&self.buttons_manual_add_included_directory.clone().upcast::<Bin>()).set_text(&flg!("upper_manual_add_included_button"));
        get_custom_label_from_button_with_image(&self.buttons_add_included_directory.clone().upcast::<Bin>()).set_text(&flg!("upper_add_included_button"));
        get_custom_label_from_button_with_image(&self.buttons_remove_included_directory.clone().upcast::<Bin>()).set_text(&flg!("upper_remove_included_button"));
        get_custom_label_from_button_with_image(&self.buttons_manual_add_excluded_directory.clone().upcast::<Bin>()).set_text(&flg!("upper_manual_add_excluded_button"));
        get_custom_label_from_button_with_image(&self.buttons_add_excluded_directory.clone().upcast::<Bin>()).set_text(&flg!("upper_add_excluded_button"));
        get_custom_label_from_button_with_image(&self.buttons_remove_excluded_directory.clone().upcast::<Bin>()).set_text(&flg!("upper_remove_excluded_button"));

        // GTK 4
        // get_custom_label_from_label_with_image(&self.buttons_manual_add_included_directory.clone()).set_text(&flg!("upper_manual_add_included_button"));
        // get_custom_label_from_label_with_image(&self.buttons_add_included_directory.clone()).set_text(&flg!("upper_add_included_button"));
        // get_custom_label_from_label_with_image(&self.buttons_remove_included_directory.clone()).set_text(&flg!("upper_remove_included_button"));
        // get_custom_label_from_label_with_image(&self.buttons_manual_add_excluded_directory.clone()).set_text(&flg!("upper_manual_add_excluded_button"));
        // get_custom_label_from_label_with_image(&self.buttons_add_excluded_directory.clone()).set_text(&flg!("upper_add_excluded_button"));
        // get_custom_label_from_label_with_image(&self.buttons_remove_excluded_directory.clone()).set_text(&flg!("upper_remove_excluded_button"));

        self.buttons_manual_add_included_directory
            .set_tooltip_text(Some(&flg!("upper_manual_add_included_button_tooltip")));
        self.buttons_add_included_directory.set_tooltip_text(Some(&flg!("upper_add_included_button_tooltip")));
        self.buttons_remove_included_directory.set_tooltip_text(Some(&flg!("upper_remove_included_button_tooltip")));
        self.buttons_manual_add_excluded_directory
            .set_tooltip_text(Some(&flg!("upper_manual_add_excluded_button_tooltip")));
        self.buttons_add_excluded_directory.set_tooltip_text(Some(&flg!("upper_add_excluded_button_tooltip")));
        self.buttons_remove_excluded_directory.set_tooltip_text(Some(&flg!("upper_remove_excluded_button_tooltip")));

        self.label_allowed_extensions.set_tooltip_text(Some(&flg!("upper_allowed_extensions_tooltip")));
        self.entry_allowed_extensions.set_tooltip_text(Some(&flg!("upper_allowed_extensions_tooltip")));
        self.label_excluded_items.set_tooltip_text(Some(&flg!("upper_excluded_items_tooltip")));
        self.entry_excluded_items.set_tooltip_text(Some(&flg!("upper_excluded_items_tooltip")));

        self.label_excluded_items.set_label(&flg!("upper_excluded_items"));
        self.label_allowed_extensions.set_label(&flg!("upper_allowed_extensions"));

        self.label_general_size_bytes.set_label(&flg!("main_label_size_bytes"));
        self.label_general_min_size.set_label(&flg!("main_label_min_size"));
        self.label_general_max_size.set_label(&flg!("main_label_max_size"));

        self.label_general_size_bytes.set_tooltip_text(Some(&flg!("main_label_size_bytes_tooltip")));
        self.label_general_min_size.set_tooltip_text(Some(&flg!("main_label_size_bytes_tooltip")));
        self.label_general_max_size.set_tooltip_text(Some(&flg!("main_label_size_bytes_tooltip")));
        self.entry_general_minimal_size.set_tooltip_text(Some(&flg!("main_label_size_bytes_tooltip")));
        self.entry_general_maximal_size.set_tooltip_text(Some(&flg!("main_label_size_bytes_tooltip")));

        let vec_children: Vec<gtk::Widget> = self.notebook_upper.children();

        // let vec_children: Vec<gtk::Widget> = get_all_children(&self.notebook_upper);
        // let vec_children: Vec<gtk::Widget> = get_all_children(&vec_children[1]);

        // Change name of upper notebook tabs
        for (upper_enum, fl_thing) in [
            (NotebookUpperEnum::ItemsConfiguration as usize, flg!("upper_notebook_items_configuration")),
            (NotebookUpperEnum::ExcludedDirectories as usize, flg!("upper_notebook_excluded_directories")),
            (NotebookUpperEnum::IncludedDirectories as usize, flg!("upper_notebook_included_directories")),
        ] {
            self.notebook_upper
                .tab_label(&vec_children[upper_enum])
                .unwrap()
                .downcast::<gtk::Label>()
                .unwrap()
                .set_text(&fl_thing);
        }

        let names_of_columns = [
            vec![
                flg!("upper_tree_view_included_folder_column_title"),
                flg!("upper_tree_view_included_reference_column_title"),
            ], // Included folders
        ];

        for (notebook_index, tree_view) in [self.tree_view_included_directories.clone()].iter().enumerate() {
            for (column_index, column) in tree_view.columns().iter().enumerate() {
                column.set_title(&names_of_columns[notebook_index][column_index]);
            }
        }
    }
}
