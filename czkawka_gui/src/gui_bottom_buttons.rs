use gtk::prelude::*;
use gtk::{Bin, Widget};

use crate::fl;
use crate::help_functions::get_custom_label_from_button_with_image;

#[derive(Clone)]
pub struct GuiBottomButtons {
    pub buttons_search: gtk::Button,
    pub buttons_select: gtk::MenuButton,
    pub buttons_delete: gtk::Button,
    pub buttons_save: gtk::Button,
    pub buttons_symlink: gtk::Button,
    pub buttons_hardlink: gtk::Button,
    pub buttons_move: gtk::Button,
    pub buttons_show_errors: gtk::Button,
    pub buttons_show_upper_notebook: gtk::Button,
    pub buttons_names: [String; 7],
    pub buttons_array: [Widget; 7],
}

impl GuiBottomButtons {
    pub fn create_from_builder(builder: &gtk::Builder, popover_select: &gtk::Popover) -> Self {
        let buttons_search: gtk::Button = builder.object("buttons_search").unwrap();
        let buttons_select: gtk::MenuButton = builder.object("buttons_select").unwrap();
        let buttons_delete: gtk::Button = builder.object("buttons_delete").unwrap();
        let buttons_save: gtk::Button = builder.object("buttons_save").unwrap();
        let buttons_symlink: gtk::Button = builder.object("buttons_symlink").unwrap();
        let buttons_hardlink: gtk::Button = builder.object("buttons_hardlink").unwrap();
        let buttons_move: gtk::Button = builder.object("buttons_move").unwrap();

        let buttons_show_errors: gtk::Button = builder.object("buttons_show_errors").unwrap();
        let buttons_show_upper_notebook: gtk::Button = builder.object("buttons_show_upper_notebook").unwrap();

        let buttons_names = [
            "search".to_string(),
            "select".to_string(),
            "delete".to_string(),
            "save".to_string(),
            "symlink".to_string(),
            "hardlink".to_string(),
            "move".to_string(),
        ];
        let buttons_array: [Widget; 7] = [
            buttons_search.clone().upcast::<Widget>(),
            buttons_select.clone().upcast::<Widget>(),
            buttons_delete.clone().upcast::<Widget>(),
            buttons_save.clone().upcast::<Widget>(),
            buttons_symlink.clone().upcast::<Widget>(),
            buttons_hardlink.clone().upcast::<Widget>(),
            buttons_move.clone().upcast::<Widget>(),
        ];

        buttons_select.set_popover(Some(popover_select));

        Self {
            buttons_search,
            buttons_select,
            buttons_delete,
            buttons_save,
            buttons_symlink,
            buttons_hardlink,
            buttons_move,
            buttons_show_errors,
            buttons_show_upper_notebook,
            buttons_names,
            buttons_array,
        }
    }
    pub fn update_language(&self) {
        get_custom_label_from_button_with_image(&self.buttons_search.clone().upcast::<Bin>()).set_text(&fl!("bottom_search_button"));
        get_custom_label_from_button_with_image(&self.buttons_select.clone().upcast::<Bin>()).set_text(&fl!("bottom_select_button"));
        get_custom_label_from_button_with_image(&self.buttons_delete.clone().upcast::<Bin>()).set_text(&fl!("bottom_delete_button"));
        get_custom_label_from_button_with_image(&self.buttons_save.clone().upcast::<Bin>()).set_text(&fl!("bottom_save_button"));
        get_custom_label_from_button_with_image(&self.buttons_symlink.clone().upcast::<Bin>()).set_text(&fl!("bottom_symlink_button"));
        get_custom_label_from_button_with_image(&self.buttons_hardlink.clone().upcast::<Bin>()).set_text(&fl!("bottom_hardlink_button"));
        get_custom_label_from_button_with_image(&self.buttons_move.clone().upcast::<Bin>()).set_text(&fl!("bottom_move_button"));

        // get_custom_label_from_button_with_image(&self.buttons_search.clone()).set_text(&fl!("bottom_search_button"));
        // get_custom_label_from_button_with_image(&self.buttons_select.clone()).set_text(&fl!("bottom_select_button"));
        // get_custom_label_from_button_with_image(&self.buttons_delete.clone()).set_text(&fl!("bottom_delete_button"));
        // get_custom_label_from_button_with_image(&self.buttons_save.clone()).set_text(&fl!("bottom_save_button"));
        // get_custom_label_from_button_with_image(&self.buttons_symlink.clone()).set_text(&fl!("bottom_symlink_button"));
        // get_custom_label_from_button_with_image(&self.buttons_hardlink.clone()).set_text(&fl!("bottom_hardlink_button"));
        // get_custom_label_from_button_with_image(&self.buttons_move.clone()).set_text(&fl!("bottom_move_button"));

        self.buttons_search.set_tooltip_text(Some(&fl!("bottom_search_button_tooltip")));
        self.buttons_select.set_tooltip_text(Some(&fl!("bottom_select_button_tooltip")));
        self.buttons_delete.set_tooltip_text(Some(&fl!("bottom_delete_button_tooltip")));
        self.buttons_save.set_tooltip_text(Some(&fl!("bottom_save_button_tooltip")));
        self.buttons_symlink.set_tooltip_text(Some(&fl!("bottom_symlink_button_tooltip")));
        self.buttons_hardlink.set_tooltip_text(Some(&fl!("bottom_hardlink_button_tooltip")));
        self.buttons_move.set_tooltip_text(Some(&fl!("bottom_move_button_tooltip")));

        self.buttons_show_errors.set_tooltip_text(Some(&fl!("bottom_show_errors_tooltip")));
        self.buttons_show_upper_notebook.set_tooltip_text(Some(&fl!("bottom_show_upper_notebook_tooltip")));
    }
}
