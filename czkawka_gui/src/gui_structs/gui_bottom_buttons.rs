use gtk::prelude::*;
use gtk::{Bin, Widget};

use crate::flg;
use crate::help_functions::{get_custom_label_from_button_with_image, BottomButtonsEnum};

#[derive(Clone)]
pub struct GuiBottomButtons {
    pub buttons_search: gtk::Button,
    pub buttons_select: gtk::MenuButton,
    pub buttons_delete: gtk::Button,
    pub buttons_save: gtk::Button,
    pub buttons_symlink: gtk::Button,
    pub buttons_hardlink: gtk::Button,
    pub buttons_move: gtk::Button,
    pub buttons_compare: gtk::Button,
    pub buttons_show_errors: gtk::Button,
    pub buttons_show_upper_notebook: gtk::Button,
    pub buttons_names: [BottomButtonsEnum; 8],
    pub buttons_array: [Widget; 8],
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
        let buttons_compare: gtk::Button = builder.object("buttons_compare").unwrap();

        let buttons_show_errors: gtk::Button = builder.object("buttons_show_errors").unwrap();
        let buttons_show_upper_notebook: gtk::Button = builder.object("buttons_show_upper_notebook").unwrap();

        let buttons_names = [
            BottomButtonsEnum::Search,
            BottomButtonsEnum::Select,
            BottomButtonsEnum::Delete,
            BottomButtonsEnum::Save,
            BottomButtonsEnum::Symlink,
            BottomButtonsEnum::Hardlink,
            BottomButtonsEnum::Move,
            BottomButtonsEnum::Compare,
        ];
        let buttons_array = [
            buttons_search.clone().upcast::<Widget>(),
            buttons_select.clone().upcast::<Widget>(),
            buttons_delete.clone().upcast::<Widget>(),
            buttons_save.clone().upcast::<Widget>(),
            buttons_symlink.clone().upcast::<Widget>(),
            buttons_hardlink.clone().upcast::<Widget>(),
            buttons_move.clone().upcast::<Widget>(),
            buttons_compare.clone().upcast::<Widget>(),
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
            buttons_compare,
            buttons_show_errors,
            buttons_show_upper_notebook,
            buttons_names,
            buttons_array,
        }
    }
    pub fn update_language(&self) {
        get_custom_label_from_button_with_image(&self.buttons_search.clone().upcast::<Bin>()).set_text(&flg!("bottom_search_button"));
        get_custom_label_from_button_with_image(&self.buttons_select.clone().upcast::<Bin>()).set_text(&flg!("bottom_select_button"));
        get_custom_label_from_button_with_image(&self.buttons_delete.clone().upcast::<Bin>()).set_text(&flg!("bottom_delete_button"));
        get_custom_label_from_button_with_image(&self.buttons_save.clone().upcast::<Bin>()).set_text(&flg!("bottom_save_button"));
        get_custom_label_from_button_with_image(&self.buttons_symlink.clone().upcast::<Bin>()).set_text(&flg!("bottom_symlink_button"));
        get_custom_label_from_button_with_image(&self.buttons_hardlink.clone().upcast::<Bin>()).set_text(&flg!("bottom_hardlink_button"));
        get_custom_label_from_button_with_image(&self.buttons_move.clone().upcast::<Bin>()).set_text(&flg!("bottom_move_button"));

        // get_custom_label_from_button_with_image(&self.buttons_search.clone()).set_text(&flg!("bottom_search_button"));
        // get_custom_label_from_button_with_image(&self.buttons_select.clone()).set_text(&flg!("bottom_select_button"));
        // get_custom_label_from_button_with_image(&self.buttons_delete.clone()).set_text(&flg!("bottom_delete_button"));
        // get_custom_label_from_button_with_image(&self.buttons_save.clone()).set_text(&flg!("bottom_save_button"));
        // get_custom_label_from_button_with_image(&self.buttons_symlink.clone()).set_text(&flg!("bottom_symlink_button"));
        // get_custom_label_from_button_with_image(&self.buttons_hardlink.clone()).set_text(&flg!("bottom_hardlink_button"));
        // get_custom_label_from_button_with_image(&self.buttons_move.clone()).set_text(&flg!("bottom_move_button"));

        self.buttons_search.set_tooltip_text(Some(&flg!("bottom_search_button_tooltip")));
        self.buttons_select.set_tooltip_text(Some(&flg!("bottom_select_button_tooltip")));
        self.buttons_delete.set_tooltip_text(Some(&flg!("bottom_delete_button_tooltip")));
        self.buttons_save.set_tooltip_text(Some(&flg!("bottom_save_button_tooltip")));
        self.buttons_symlink.set_tooltip_text(Some(&flg!("bottom_symlink_button_tooltip")));
        self.buttons_hardlink.set_tooltip_text(Some(&flg!("bottom_hardlink_button_tooltip")));
        self.buttons_move.set_tooltip_text(Some(&flg!("bottom_move_button_tooltip")));

        self.buttons_show_errors.set_tooltip_text(Some(&flg!("bottom_show_errors_tooltip")));
        self.buttons_show_upper_notebook.set_tooltip_text(Some(&flg!("bottom_show_upper_notebook_tooltip")));
    }
}
