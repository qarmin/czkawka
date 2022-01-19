use crate::flg;
use gtk::prelude::*;
use gtk::{Builder, TreePath};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct GuiCompareImages {
    pub window_compare: gtk::Window,

    pub label_group_info: gtk::Label,

    pub button_go_previous_compare_group: gtk::Button,
    pub button_go_next_compare_group: gtk::Button,

    pub check_button_left_preview_text: gtk::CheckButton,
    pub check_button_right_preview_text: gtk::CheckButton,

    pub image_compare_left: gtk::Image,
    pub image_compare_right: gtk::Image,

    pub scrolled_window_compare_choose_images: gtk::ScrolledWindow,

    pub shared_numbers_of_groups: Rc<RefCell<u32>>,
    pub shared_current_of_groups: Rc<RefCell<u32>>,
    pub shared_current_path: Rc<RefCell<Option<TreePath>>>,
    pub shared_image_cache: Rc<RefCell<Vec<(String, String, gtk::Image, gtk::Image, gtk::TreePath)>>>,
    pub shared_using_for_preview: Rc<RefCell<(Option<gtk::TreePath>, Option<gtk::TreePath>)>>,
}

impl GuiCompareImages {
    pub fn create_from_builder(window_main: &gtk::Window) -> Self {
        let glade_src = include_str!("../../ui/compare_images.glade").to_string();
        let builder = Builder::from_string(glade_src.as_str());

        let window_compare: gtk::Window = builder.object("window_compare").unwrap();
        window_compare.set_title(&flg!("window_compare_images"));
        window_compare.set_modal(true);
        window_compare.set_transient_for(Some(window_main));

        let label_group_info: gtk::Label = builder.object("label_group_info").unwrap();

        let button_go_previous_compare_group: gtk::Button = builder.object("button_go_previous_compare_group").unwrap();
        let button_go_next_compare_group: gtk::Button = builder.object("button_go_next_compare_group").unwrap();

        let check_button_left_preview_text: gtk::CheckButton = builder.object("check_button_left_preview_text").unwrap();
        let check_button_right_preview_text: gtk::CheckButton = builder.object("check_button_right_preview_text").unwrap();

        let image_compare_left: gtk::Image = builder.object("image_compare_left").unwrap();
        let image_compare_right: gtk::Image = builder.object("image_compare_right").unwrap();

        let scrolled_window_compare_choose_images: gtk::ScrolledWindow = builder.object("scrolled_window_compare_choose_images").unwrap();

        let shared_numbers_of_groups = Rc::new(RefCell::new(0));
        let shared_current_of_groups = Rc::new(RefCell::new(0));
        let shared_current_path = Rc::new(RefCell::new(None));
        let shared_image_cache = Rc::new(RefCell::new(Vec::new()));
        let shared_using_for_preview = Rc::new(RefCell::new((None, None)));

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
        self.window_compare.set_title(&flg!("window_compare_images"));
    }
}
