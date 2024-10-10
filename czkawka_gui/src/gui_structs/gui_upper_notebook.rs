use gtk4::prelude::*;
use gtk4::{EventControllerKey, GestureClick, TreeView};

use crate::help_functions::{get_all_direct_children, get_custom_label_from_widget, set_icon_of_button};
use crate::notebook_enums::NotebookUpperEnum;
use crate::{flg, CZK_ICON_ADD, CZK_ICON_DELETE, CZK_ICON_MANUAL_ADD};

#[derive(Clone)]
pub struct GuiUpperNotebook {
    pub notebook_upper: gtk4::Notebook,

    pub scrolled_window_included_directories: gtk4::ScrolledWindow,
    pub scrolled_window_excluded_directories: gtk4::ScrolledWindow,

    pub tree_view_included_directories: TreeView,
    pub tree_view_excluded_directories: TreeView,

    pub evk_tree_view_included_directories: EventControllerKey,
    pub evk_tree_view_excluded_directories: EventControllerKey,

    pub gc_tree_view_included_directories: GestureClick,
    pub gc_tree_view_excluded_directories: GestureClick,

    pub entry_excluded_items: gtk4::Entry,
    pub entry_allowed_extensions: gtk4::Entry,
    pub entry_excluded_extensions: gtk4::Entry,

    pub check_button_recursive: gtk4::CheckButton,

    pub buttons_manual_add_included_directory: gtk4::Button,
    pub buttons_add_included_directory: gtk4::Button,
    pub buttons_remove_included_directory: gtk4::Button,
    pub buttons_manual_add_excluded_directory: gtk4::Button,
    pub buttons_add_excluded_directory: gtk4::Button,
    pub buttons_remove_excluded_directory: gtk4::Button,

    pub label_excluded_items: gtk4::Label,
    pub label_allowed_extensions: gtk4::Label,
    pub label_excluded_extensions: gtk4::Label,

    pub entry_general_minimal_size: gtk4::Entry,
    pub entry_general_maximal_size: gtk4::Entry,
    pub label_general_size_bytes: gtk4::Label,
    pub label_general_min_size: gtk4::Label,
    pub label_general_max_size: gtk4::Label,
}

impl GuiUpperNotebook {
    pub fn create_from_builder(builder: &gtk4::Builder) -> Self {
        let notebook_upper: gtk4::Notebook = builder.object("notebook_upper").expect("Cambalache");

        let scrolled_window_included_directories: gtk4::ScrolledWindow = builder.object("scrolled_window_included_directories").expect("Cambalache");
        let scrolled_window_excluded_directories: gtk4::ScrolledWindow = builder.object("scrolled_window_excluded_directories").expect("Cambalache");

        let tree_view_included_directories: TreeView = TreeView::new();
        let tree_view_excluded_directories: TreeView = TreeView::new();

        let evk_tree_view_included_directories: EventControllerKey = EventControllerKey::new();
        tree_view_included_directories.add_controller(evk_tree_view_included_directories.clone());
        let evk_tree_view_excluded_directories: EventControllerKey = EventControllerKey::new();
        tree_view_excluded_directories.add_controller(evk_tree_view_excluded_directories.clone());

        let gc_tree_view_included_directories: GestureClick = GestureClick::new();
        tree_view_included_directories.add_controller(gc_tree_view_included_directories.clone());
        let gc_tree_view_excluded_directories: GestureClick = GestureClick::new();
        tree_view_excluded_directories.add_controller(gc_tree_view_excluded_directories.clone());

        let entry_allowed_extensions: gtk4::Entry = builder.object("entry_allowed_extensions").expect("Cambalache");
        let entry_excluded_extensions: gtk4::Entry = builder.object("entry_excluded_extensions").expect("Cambalache");
        let entry_excluded_items: gtk4::Entry = builder.object("entry_excluded_items").expect("Cambalache");

        let check_button_recursive: gtk4::CheckButton = builder.object("check_button_recursive").expect("Cambalache");

        let buttons_manual_add_included_directory: gtk4::Button = builder.object("buttons_manual_add_included_directory").expect("Cambalache");
        let buttons_add_included_directory: gtk4::Button = builder.object("buttons_add_included_directory").expect("Cambalache");
        let buttons_remove_included_directory: gtk4::Button = builder.object("buttons_remove_included_directory").expect("Cambalache");
        let buttons_manual_add_excluded_directory: gtk4::Button = builder.object("buttons_manual_add_excluded_directory").expect("Cambalache");
        let buttons_add_excluded_directory: gtk4::Button = builder.object("buttons_add_excluded_directory").expect("Cambalache");
        let buttons_remove_excluded_directory: gtk4::Button = builder.object("buttons_remove_excluded_directory").expect("Cambalache");

        let label_excluded_items: gtk4::Label = builder.object("label_excluded_items").expect("Cambalache");
        let label_allowed_extensions: gtk4::Label = builder.object("label_allowed_extensions").expect("Cambalache");
        let label_excluded_extensions: gtk4::Label = builder.object("label_excluded_extensions").expect("Cambalache");

        let entry_general_minimal_size: gtk4::Entry = builder.object("entry_general_minimal_size").expect("Cambalache");
        let entry_general_maximal_size: gtk4::Entry = builder.object("entry_general_maximal_size").expect("Cambalache");
        let label_general_size_bytes: gtk4::Label = builder.object("label_general_size_bytes").expect("Cambalache");
        let label_general_min_size: gtk4::Label = builder.object("label_general_min_size").expect("Cambalache");
        let label_general_max_size: gtk4::Label = builder.object("label_general_max_size").expect("Cambalache");

        set_icon_of_button(&buttons_add_included_directory, CZK_ICON_ADD);
        set_icon_of_button(&buttons_manual_add_included_directory, CZK_ICON_MANUAL_ADD);
        set_icon_of_button(&buttons_remove_included_directory, CZK_ICON_DELETE);
        set_icon_of_button(&buttons_add_excluded_directory, CZK_ICON_ADD);
        set_icon_of_button(&buttons_manual_add_excluded_directory, CZK_ICON_MANUAL_ADD);
        set_icon_of_button(&buttons_remove_excluded_directory, CZK_ICON_DELETE);

        Self {
            notebook_upper,
            scrolled_window_included_directories,
            scrolled_window_excluded_directories,
            tree_view_included_directories,
            tree_view_excluded_directories,
            evk_tree_view_included_directories,
            evk_tree_view_excluded_directories,
            gc_tree_view_included_directories,
            gc_tree_view_excluded_directories,
            entry_excluded_items,
            entry_allowed_extensions,
            entry_excluded_extensions,
            check_button_recursive,
            buttons_manual_add_included_directory,
            buttons_add_included_directory,
            buttons_remove_included_directory,
            buttons_manual_add_excluded_directory,
            buttons_add_excluded_directory,
            buttons_remove_excluded_directory,
            label_excluded_items,
            label_allowed_extensions,
            label_excluded_extensions,
            entry_general_minimal_size,
            entry_general_maximal_size,
            label_general_size_bytes,
            label_general_min_size,
            label_general_max_size,
        }
    }
    pub fn update_language(&self) {
        self.check_button_recursive.set_label(Some(&flg!("upper_recursive_button")));
        self.check_button_recursive.set_tooltip_text(Some(&flg!("upper_recursive_button_tooltip")));

        get_custom_label_from_widget(&self.buttons_manual_add_included_directory.clone()).set_text(&flg!("upper_manual_add_included_button"));
        get_custom_label_from_widget(&self.buttons_add_included_directory.clone()).set_text(&flg!("upper_add_included_button"));
        get_custom_label_from_widget(&self.buttons_remove_included_directory.clone()).set_text(&flg!("upper_remove_included_button"));
        get_custom_label_from_widget(&self.buttons_manual_add_excluded_directory.clone()).set_text(&flg!("upper_manual_add_excluded_button"));
        get_custom_label_from_widget(&self.buttons_add_excluded_directory.clone()).set_text(&flg!("upper_add_excluded_button"));
        get_custom_label_from_widget(&self.buttons_remove_excluded_directory.clone()).set_text(&flg!("upper_remove_excluded_button"));

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
        self.label_excluded_extensions.set_tooltip_text(Some(&flg!("upper_excluded_extensions_tooltip")));
        self.entry_excluded_extensions.set_tooltip_text(Some(&flg!("upper_excluded_extensions_tooltip")));
        self.label_excluded_items.set_tooltip_text(Some(&flg!("upper_excluded_items_tooltip")));
        self.entry_excluded_items.set_tooltip_text(Some(&flg!("upper_excluded_items_tooltip")));

        self.label_excluded_items.set_label(&flg!("upper_excluded_items"));
        self.label_allowed_extensions.set_label(&flg!("upper_allowed_extensions"));
        self.label_excluded_extensions.set_label(&flg!("upper_excluded_extensions"));

        self.label_general_size_bytes.set_label(&flg!("main_label_size_bytes"));
        self.label_general_min_size.set_label(&flg!("main_label_min_size"));
        self.label_general_max_size.set_label(&flg!("main_label_max_size"));

        self.label_general_size_bytes.set_tooltip_text(Some(&flg!("main_label_size_bytes_tooltip")));
        self.label_general_min_size.set_tooltip_text(Some(&flg!("main_label_size_bytes_tooltip")));
        self.label_general_max_size.set_tooltip_text(Some(&flg!("main_label_size_bytes_tooltip")));
        self.entry_general_minimal_size.set_tooltip_text(Some(&flg!("main_label_size_bytes_tooltip")));
        self.entry_general_maximal_size.set_tooltip_text(Some(&flg!("main_label_size_bytes_tooltip")));

        let vec_children: Vec<gtk4::Widget> = get_all_direct_children(&self.notebook_upper);
        let vec_children: Vec<gtk4::Widget> = get_all_direct_children(&vec_children[1]); // This is quite safe in GTK 4, because tab label is always second child

        // Change name of upper notebook tabs
        for (upper_enum, fl_thing) in [
            (NotebookUpperEnum::ItemsConfiguration as usize, flg!("upper_notebook_items_configuration")),
            (NotebookUpperEnum::ExcludedDirectories as usize, flg!("upper_notebook_excluded_directories")),
            (NotebookUpperEnum::IncludedDirectories as usize, flg!("upper_notebook_included_directories")),
        ] {
            self.notebook_upper
                .tab_label(&vec_children[upper_enum])
                .expect("Failed to get tab label")
                .downcast::<gtk4::Label>()
                .expect("Failed to downcast to label")
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
