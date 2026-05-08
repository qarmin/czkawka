use gtk4::Builder;

#[derive(Clone)]
pub struct GuiSelectPopovers {
    #[expect(unused)]
    pub buttons_popover_right_click_open_file: gtk4::Button,
    #[expect(unused)]
    pub buttons_popover_right_click_open_folder: gtk4::Button,
    #[expect(unused)]
    pub popover_right_click: gtk4::Popover,
}

impl GuiSelectPopovers {
    pub(crate) fn create_from_builder() -> Self {
        let glade_src = include_str!("../../ui/popover_right_click.ui").to_string();
        let builder = Builder::from_string(glade_src.as_str());

        let buttons_popover_right_click_open_file: gtk4::Button = builder.object("buttons_popover_right_click_open_file").expect("Cambalache");
        let buttons_popover_right_click_open_folder: gtk4::Button = builder.object("buttons_popover_right_click_open_folder").expect("Cambalache");
        let popover_right_click: gtk4::Popover = builder.object("popover_right_click").expect("Cambalache");

        Self {
            buttons_popover_right_click_open_file,
            buttons_popover_right_click_open_folder,
            popover_right_click,
        }
    }

    pub(crate) fn update_language(&self) {}
}
