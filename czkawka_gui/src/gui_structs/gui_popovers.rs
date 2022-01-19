use gtk::prelude::*;
use gtk::Builder;

use crate::flg;

#[derive(Clone)]
pub struct GuiPopovers {
    pub buttons_popover_select_all: gtk::Button,
    pub buttons_popover_unselect_all: gtk::Button,
    pub buttons_popover_reverse: gtk::Button,
    pub buttons_popover_select_all_except_oldest: gtk::Button,
    pub buttons_popover_select_all_except_newest: gtk::Button,
    pub buttons_popover_select_one_oldest: gtk::Button,
    pub buttons_popover_select_one_newest: gtk::Button,
    pub buttons_popover_select_custom: gtk::Button,
    pub buttons_popover_unselect_custom: gtk::Button,
    pub buttons_popover_select_all_images_except_biggest: gtk::Button,
    pub buttons_popover_select_all_images_except_smallest: gtk::Button,

    pub separator_select_image_size: gtk::Separator,
    pub separator_select_reverse: gtk::Separator,
    pub separator_select_date: gtk::Separator,
    pub separator_select_custom: gtk::Separator,

    pub buttons_popover_right_click_open_file: gtk::Button,
    pub buttons_popover_right_click_open_folder: gtk::Button,

    pub popover_select: gtk::Popover,
    pub popover_right_click: gtk::Popover,
}

impl GuiPopovers {
    pub fn create_from_builder() -> Self {
        let glade_src = include_str!("../../ui/popover_select.glade").to_string();
        let builder = Builder::from_string(glade_src.as_str());

        let buttons_popover_select_all: gtk::Button = builder.object("buttons_popover_select_all").unwrap();
        let buttons_popover_unselect_all: gtk::Button = builder.object("buttons_popover_unselect_all").unwrap();
        let buttons_popover_reverse: gtk::Button = builder.object("buttons_popover_reverse").unwrap();
        let buttons_popover_select_all_except_oldest: gtk::Button = builder.object("buttons_popover_select_all_except_oldest").unwrap();
        let buttons_popover_select_all_except_newest: gtk::Button = builder.object("buttons_popover_select_all_except_newest").unwrap();
        let buttons_popover_select_one_oldest: gtk::Button = builder.object("buttons_popover_select_one_oldest").unwrap();
        let buttons_popover_select_one_newest: gtk::Button = builder.object("buttons_popover_select_one_newest").unwrap();
        let buttons_popover_select_custom: gtk::Button = builder.object("buttons_popover_select_custom").unwrap();
        let buttons_popover_unselect_custom: gtk::Button = builder.object("buttons_popover_unselect_custom").unwrap();
        let buttons_popover_select_all_images_except_biggest: gtk::Button = builder.object("buttons_popover_select_all_images_except_biggest").unwrap();
        let buttons_popover_select_all_images_except_smallest: gtk::Button = builder.object("buttons_popover_select_all_images_except_smallest").unwrap();

        let separator_select_image_size: gtk::Separator = builder.object("separator_select_image_size").unwrap();
        let separator_select_reverse: gtk::Separator = builder.object("separator_select_reverse").unwrap();
        let separator_select_date: gtk::Separator = builder.object("separator_select_date").unwrap();
        let separator_select_custom: gtk::Separator = builder.object("separator_select_custom").unwrap();

        let popover_select: gtk::Popover = builder.object("popover_select").unwrap();

        // Popover right click(not implemented for now)
        let glade_src = include_str!("../../ui/popover_right_click.glade").to_string();
        let builder = Builder::from_string(glade_src.as_str());

        let buttons_popover_right_click_open_file: gtk::Button = builder.object("buttons_popover_right_click_open_file").unwrap();
        let buttons_popover_right_click_open_folder: gtk::Button = builder.object("buttons_popover_right_click_open_folder").unwrap();

        let popover_right_click: gtk::Popover = builder.object("popover_right_click").unwrap();

        Self {
            buttons_popover_select_all,
            buttons_popover_unselect_all,
            buttons_popover_reverse,
            buttons_popover_select_all_except_oldest,
            buttons_popover_select_all_except_newest,
            buttons_popover_select_one_oldest,
            buttons_popover_select_one_newest,
            buttons_popover_select_custom,
            buttons_popover_unselect_custom,
            buttons_popover_select_all_images_except_biggest,
            buttons_popover_select_all_images_except_smallest,
            separator_select_image_size,
            separator_select_reverse,
            separator_select_date,
            separator_select_custom,
            buttons_popover_right_click_open_file,
            buttons_popover_right_click_open_folder,
            popover_select,
            popover_right_click,
        }
    }
    pub fn update_language(&self) {
        self.buttons_popover_select_all.set_label(&flg!("popover_select_all"));
        self.buttons_popover_unselect_all.set_label(&flg!("popover_unselect_all"));
        self.buttons_popover_reverse.set_label(&flg!("popover_reverse"));
        self.buttons_popover_select_all_except_oldest.set_label(&flg!("popover_select_all_except_oldest"));
        self.buttons_popover_select_all_except_newest.set_label(&flg!("popover_select_all_except_newest"));
        self.buttons_popover_select_one_oldest.set_label(&flg!("popover_select_one_oldest"));
        self.buttons_popover_select_one_newest.set_label(&flg!("popover_select_one_newest"));
        self.buttons_popover_select_custom.set_label(&flg!("popover_select_custom"));
        self.buttons_popover_unselect_custom.set_label(&flg!("popover_unselect_custom"));
        self.buttons_popover_select_all_images_except_biggest
            .set_label(&flg!("popover_select_all_images_except_biggest"));
        self.buttons_popover_select_all_images_except_smallest
            .set_label(&flg!("popover_select_all_images_except_smallest"));
    }
}
