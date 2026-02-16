use gtk4::Label;
use gtk4::prelude::*;

use crate::gtk_traits::WidgetTraits;
use crate::gui_structs::common_upper_tree_view::{CommonUpperTreeViews, UpperSubView, UpperTreeViewEnum};
use crate::helpers::image_operations::set_icon_of_button;
use crate::notebook_enums::NotebookUpperEnum;
use crate::{CZK_ICON_ADD, CZK_ICON_DELETE, CZK_ICON_MANUAL_ADD, flg};

#[derive(Clone)]
pub struct GuiUpperNotebook {
    pub notebook_upper: gtk4::Notebook,

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

    pub common_upper_tree_views: CommonUpperTreeViews,
}

impl GuiUpperNotebook {
    pub(crate) fn create_from_builder(builder: &gtk4::Builder) -> Self {
        let notebook_upper: gtk4::Notebook = builder.object("notebook_upper").expect("Cambalache");

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

        let common_upper_tree_views = CommonUpperTreeViews {
            subviews: vec![
                UpperSubView::new(
                    builder,
                    "scrolled_window_included_directories",
                    NotebookUpperEnum::IncludedDirectories,
                    UpperTreeViewEnum::IncludedDirectories,
                    "tree_view_upper_included_directories",
                ),
                UpperSubView::new(
                    builder,
                    "scrolled_window_excluded_directories",
                    NotebookUpperEnum::ExcludedDirectories,
                    UpperTreeViewEnum::ExcludedDirectories,
                    "tree_view_upper_excluded_directories",
                ),
            ],
        };

        Self {
            notebook_upper,
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
            common_upper_tree_views,
        }
    }

    pub(crate) fn setup(&self) {
        self.common_upper_tree_views.setup();
    }

    pub(crate) fn update_language(&self) {
        self.check_button_recursive.set_label(Some(&flg!("upper_recursive_button")));
        self.check_button_recursive.set_tooltip_text(Some(&flg!("upper_recursive_button_tooltip")));

        self.buttons_manual_add_included_directory
            .get_widget_of_type::<Label>(true)
            .set_text(&flg!("upper_manual_add_included_button"));
        self.buttons_add_included_directory
            .get_widget_of_type::<Label>(true)
            .set_text(&flg!("upper_add_included_button"));
        self.buttons_remove_included_directory
            .get_widget_of_type::<Label>(true)
            .set_text(&flg!("upper_remove_included_button"));
        self.buttons_manual_add_excluded_directory
            .get_widget_of_type::<Label>(true)
            .set_text(&flg!("upper_manual_add_excluded_button"));
        self.buttons_add_excluded_directory
            .get_widget_of_type::<Label>(true)
            .set_text(&flg!("upper_add_excluded_button"));
        self.buttons_remove_excluded_directory
            .get_widget_of_type::<Label>(true)
            .set_text(&flg!("upper_remove_excluded_button"));

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

        let vec_children: Vec<gtk4::Widget> = self.notebook_upper.get_all_direct_children();
        let vec_children: Vec<gtk4::Widget> = vec_children[1].get_all_direct_children(); // This is quite safe in GTK 4, because tab label is always second child

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
               // TODO - missing Excluded folders?
        ];

        for (notebook_index, tree_view) in std::iter::once(self.common_upper_tree_views.get_tree_view(UpperTreeViewEnum::IncludedDirectories)).enumerate() {
            for (column_index, column) in tree_view.columns().iter().enumerate() {
                column.set_title(&names_of_columns[notebook_index][column_index]);
            }
        }
    }
}
