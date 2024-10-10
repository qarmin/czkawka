use std::cell::RefCell;
use std::rc::Rc;

use gtk4::prelude::*;
use gtk4::{Builder, TreePath};

use crate::help_functions::set_icon_of_button;
use crate::{flg, CZK_ICON_LEFT, CZK_ICON_RIGHT};

#[derive(Clone)]
pub struct GuiCompareImages {
    pub window_compare: gtk4::Window,

    pub label_group_info: gtk4::Label,

    pub button_go_previous_compare_group: gtk4::Button,
    pub button_go_next_compare_group: gtk4::Button,

    pub check_button_left_preview_text: gtk4::CheckButton,
    pub check_button_right_preview_text: gtk4::CheckButton,

    pub image_compare_left: gtk4::Image,
    pub image_compare_right: gtk4::Image,

    pub scrolled_window_compare_choose_images: gtk4::ScrolledWindow,

    pub shared_numbers_of_groups: Rc<RefCell<u32>>,
    pub shared_current_of_groups: Rc<RefCell<u32>>,
    pub shared_current_path: Rc<RefCell<Option<TreePath>>>,
    pub shared_image_cache: Rc<RefCell<Vec<(String, String, gtk4::Image, gtk4::Image, TreePath)>>>,
    pub shared_using_for_preview: Rc<RefCell<(Option<TreePath>, Option<TreePath>)>>,
}

impl GuiCompareImages {
    pub fn create_from_builder(window_main: &gtk4::Window) -> Self {
        let glade_src = include_str!("../../ui/compare_images.ui").to_string();
        let builder = Builder::from_string(glade_src.as_str());

        let window_compare: gtk4::Window = builder.object("window_compare").expect("Cambalache");
        window_compare.set_title(Some(&flg!("window_compare_images")));
        window_compare.set_modal(true);
        window_compare.set_transient_for(Some(window_main));

        let label_group_info: gtk4::Label = builder.object("label_group_info").expect("Cambalache");

        let button_go_previous_compare_group: gtk4::Button = builder.object("button_go_previous_compare_group").expect("Cambalache");
        let button_go_next_compare_group: gtk4::Button = builder.object("button_go_next_compare_group").expect("Cambalache");

        let check_button_left_preview_text: gtk4::CheckButton = builder.object("check_button_left_preview_text").expect("Cambalache");
        let check_button_right_preview_text: gtk4::CheckButton = builder.object("check_button_right_preview_text").expect("Cambalache");

        let image_compare_left: gtk4::Image = builder.object("image_compare_left").expect("Cambalache");
        let image_compare_right: gtk4::Image = builder.object("image_compare_right").expect("Cambalache");

        let scrolled_window_compare_choose_images: gtk4::ScrolledWindow = builder.object("scrolled_window_compare_choose_images").expect("Cambalache");

        let shared_numbers_of_groups = Rc::new(RefCell::new(0));
        let shared_current_of_groups = Rc::new(RefCell::new(0));
        let shared_current_path = Rc::new(RefCell::new(None));
        let shared_image_cache = Rc::new(RefCell::new(Vec::new()));
        let shared_using_for_preview = Rc::new(RefCell::new((None, None)));

        set_icon_of_button(&button_go_previous_compare_group, CZK_ICON_LEFT);
        set_icon_of_button(&button_go_next_compare_group, CZK_ICON_RIGHT);

        Self {
            window_compare,
            label_group_info,
            button_go_previous_compare_group,
            button_go_next_compare_group,
            check_button_left_preview_text,
            check_button_right_preview_text,
            image_compare_left,
            image_compare_right,
            scrolled_window_compare_choose_images,
            shared_numbers_of_groups,
            shared_current_of_groups,
            shared_current_path,
            shared_image_cache,
            shared_using_for_preview,
        }
    }
    pub fn update_language(&self) {
        self.window_compare.set_title(Some(&flg!("window_compare_images")));
    }
}
