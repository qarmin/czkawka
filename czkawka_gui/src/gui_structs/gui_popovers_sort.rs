use gtk4::prelude::*;
use gtk4::Builder;

use crate::flg;

#[derive(Clone)]
pub struct GuiSortPopovers {
    pub buttons_popover_sort_file_name: gtk4::Button,
    pub buttons_popover_sort_folder_name: gtk4::Button,
    pub buttons_popover_sort_full_name: gtk4::Button,
    pub buttons_popover_sort_size: gtk4::Button,
    pub buttons_popover_sort_selection: gtk4::Button,

    pub popover_sort: gtk4::Popover,
}

impl GuiSortPopovers {
    pub fn create_from_builder() -> Self {
        let glade_src = include_str!("../../ui/popover_sort.ui").to_string();
        let builder = Builder::from_string(glade_src.as_str());

        let buttons_popover_sort_file_name: gtk4::Button = builder.object("buttons_popover_sort_file_name").expect("Cambalache");
        let buttons_popover_sort_folder_name: gtk4::Button = builder.object("buttons_popover_sort_folder_name").expect("Cambalache");
        let buttons_popover_sort_full_name: gtk4::Button = builder.object("buttons_popover_sort_full_name").expect("Cambalache");
        let buttons_popover_sort_size: gtk4::Button = builder.object("buttons_popover_sort_size").expect("Cambalache");
        let buttons_popover_sort_selection: gtk4::Button = builder.object("buttons_popover_sort_selection").expect("Cambalache");

        let popover_sort: gtk4::Popover = builder.object("popover_sort").expect("Cambalache");

        Self {
            buttons_popover_sort_file_name,
            buttons_popover_sort_folder_name,
            buttons_popover_sort_full_name,
            buttons_popover_sort_size,
            buttons_popover_sort_selection,
            popover_sort,
        }
    }
    pub fn update_language(&self) {
        self.buttons_popover_sort_file_name.set_label(&flg!("popover_sort_file_name"));
        self.buttons_popover_sort_folder_name.set_label(&flg!("popover_sort_folder_name"));
        self.buttons_popover_sort_full_name.set_label(&flg!("popover_sort_full_name"));
        self.buttons_popover_sort_size.set_label(&flg!("popover_sort_size"));
        self.buttons_popover_sort_selection.set_label(&flg!("popover_sort_selection"));
    }
}
