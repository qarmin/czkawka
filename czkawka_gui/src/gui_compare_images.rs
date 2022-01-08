use gtk::prelude::*;
use gtk::Builder;

#[derive(Clone)]
pub struct GuiCompareImages {
    pub window_compare: gtk::Window,

    pub label_group_info: gtk::Label,

    pub button_go_previous_compare_group: gtk::Button,
    pub button_go_next_compare_group: gtk::Button,

    pub label_compare_left_index: gtk::Label,
    pub check_button_compare_select_left: gtk::CheckButton,
    pub label_compare_right_index: gtk::Label,
    pub check_button_compare_select_right: gtk::CheckButton,

    pub image_compare_left: gtk::Image,
    pub image_compare_right: gtk::Image,

    pub scrolled_window_compare_choose_images: gtk::ScrolledWindow,
}

impl GuiCompareImages {
    pub fn create_from_builder(window_main : &gtk::Window) -> Self {
        let glade_src = include_str!("../ui/compare_images.glade").to_string();
        let builder = Builder::from_string(glade_src.as_str());

        let window_compare: gtk::Window = builder.object("window_compare").unwrap();
        window_compare.set_modal(true);
        window_compare.set_transient_for(Some(window_main));

        let label_group_info: gtk::Label = builder.object("label_group_info").unwrap();

        let button_go_previous_compare_group: gtk::Button = builder.object("button_go_previous_compare_group").unwrap();
        let button_go_next_compare_group: gtk::Button = builder.object("button_go_next_compare_group").unwrap();

        let label_compare_left_index: gtk::Label = builder.object("label_compare_left_index").unwrap();
        let check_button_compare_select_left: gtk::CheckButton = builder.object("check_button_compare_select_left").unwrap();
        let label_compare_right_index: gtk::Label = builder.object("label_compare_right_index").unwrap();
        let check_button_compare_select_right: gtk::CheckButton = builder.object("check_button_compare_select_right").unwrap();

        let image_compare_left: gtk::Image = builder.object("image_compare_left").unwrap();
        let image_compare_right: gtk::Image = builder.object("image_compare_right").unwrap();

        let scrolled_window_compare_choose_images: gtk::ScrolledWindow = builder.object("scrolled_window_compare_choose_images").unwrap();

        Self {
            window_compare,
            label_group_info,
            button_go_previous_compare_group,
            button_go_next_compare_group,
            label_compare_left_index,
            check_button_compare_select_left,
            label_compare_right_index,
            check_button_compare_select_right,
            image_compare_left,
            image_compare_right,
            scrolled_window_compare_choose_images
        }
    }
    pub fn update_language(&self) {
    }
}
