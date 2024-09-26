use gtk4::prelude::*;
use gtk4::{GestureClick, Widget};

use crate::gui_structs::gui_data::CZK_ICON_SORT;
use crate::help_functions::{get_custom_label_from_widget, set_icon_of_button, BottomButtonsEnum};
use crate::{
    flg, CZK_ICON_COMPARE, CZK_ICON_HARDLINK, CZK_ICON_HIDE_DOWN, CZK_ICON_HIDE_UP, CZK_ICON_MOVE, CZK_ICON_SAVE, CZK_ICON_SEARCH, CZK_ICON_SELECT, CZK_ICON_SYMLINK,
    CZK_ICON_TRASH,
};

#[derive(Clone)]
pub struct GuiBottomButtons {
    pub buttons_search: gtk4::Button,
    pub buttons_select: gtk4::MenuButton,
    pub buttons_delete: gtk4::Button,
    pub buttons_save: gtk4::Button,
    pub buttons_symlink: gtk4::Button,
    pub buttons_hardlink: gtk4::Button,
    pub buttons_move: gtk4::Button,
    pub buttons_compare: gtk4::Button,
    pub buttons_sort: gtk4::MenuButton,
    pub buttons_show_errors: gtk4::Button,
    pub buttons_show_upper_notebook: gtk4::Button,

    pub label_buttons_select: gtk4::Label,
    pub label_buttons_sort: gtk4::Label,

    pub buttons_names: [BottomButtonsEnum; 9],
    pub buttons_array: [Widget; 9],

    pub gc_buttons_select: GestureClick,
    pub gc_buttons_sort: GestureClick,
}

impl GuiBottomButtons {
    pub fn create_from_builder(builder: &gtk4::Builder, popover_select: &gtk4::Popover, popover_sort: &gtk4::Popover) -> Self {
        let buttons_search: gtk4::Button = builder.object("buttons_search").expect("Cambalache");
        let buttons_select: gtk4::MenuButton = builder.object("buttons_select").expect("Cambalache");
        let buttons_delete: gtk4::Button = builder.object("buttons_delete").expect("Cambalache");
        let buttons_save: gtk4::Button = builder.object("buttons_save").expect("Cambalache");
        let buttons_symlink: gtk4::Button = builder.object("buttons_symlink").expect("Cambalache");
        let buttons_hardlink: gtk4::Button = builder.object("buttons_hardlink").expect("Cambalache");
        let buttons_move: gtk4::Button = builder.object("buttons_move").expect("Cambalache");
        let buttons_compare: gtk4::Button = builder.object("buttons_compare").expect("Cambalache");
        let buttons_sort: gtk4::MenuButton = builder.object("buttons_sort").expect("Cambalache");

        let buttons_show_errors: gtk4::Button = builder.object("buttons_show_errors").expect("Cambalache");
        let buttons_show_upper_notebook: gtk4::Button = builder.object("buttons_show_upper_notebook").expect("Cambalache");

        let label_buttons_select: gtk4::Label = builder.object("label_buttons_select").expect("Cambalache");
        let label_buttons_sort: gtk4::Label = builder.object("label_buttons_sort").expect("Cambalache");

        let gc_buttons_select: GestureClick = GestureClick::new();
        let gc_buttons_sort: GestureClick = GestureClick::new();

        buttons_select.add_controller(gc_buttons_select.clone());
        buttons_sort.add_controller(gc_buttons_sort.clone());

        set_icon_of_button(&buttons_search, CZK_ICON_SEARCH);
        set_icon_of_button(&buttons_select, CZK_ICON_SELECT);
        set_icon_of_button(&buttons_delete, CZK_ICON_TRASH);
        set_icon_of_button(&buttons_save, CZK_ICON_SAVE);
        set_icon_of_button(&buttons_symlink, CZK_ICON_SYMLINK);
        set_icon_of_button(&buttons_hardlink, CZK_ICON_HARDLINK);
        set_icon_of_button(&buttons_move, CZK_ICON_MOVE);
        set_icon_of_button(&buttons_compare, CZK_ICON_COMPARE);
        set_icon_of_button(&buttons_sort, CZK_ICON_SORT);
        set_icon_of_button(&buttons_show_errors, CZK_ICON_HIDE_DOWN);
        set_icon_of_button(&buttons_show_upper_notebook, CZK_ICON_HIDE_UP);

        let buttons_names = [
            BottomButtonsEnum::Search,
            BottomButtonsEnum::Select,
            BottomButtonsEnum::Delete,
            BottomButtonsEnum::Save,
            BottomButtonsEnum::Symlink,
            BottomButtonsEnum::Hardlink,
            BottomButtonsEnum::Move,
            BottomButtonsEnum::Compare,
            BottomButtonsEnum::Sort,
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
            buttons_sort.clone().upcast::<Widget>(),
        ];

        buttons_select.set_popover(Some(popover_select));
        buttons_sort.set_popover(Some(popover_sort));

        #[cfg(target_family = "windows")]
        buttons_hardlink.set_sensitive(test_hardlinks());

        Self {
            buttons_search,
            buttons_select,
            buttons_delete,
            buttons_save,
            buttons_symlink,
            buttons_hardlink,
            buttons_move,
            buttons_compare,
            buttons_sort,
            buttons_show_errors,
            buttons_show_upper_notebook,
            label_buttons_select,
            label_buttons_sort,
            buttons_names,
            buttons_array,
            gc_buttons_select,
            gc_buttons_sort,
        }
    }
    pub fn update_language(&self) {
        get_custom_label_from_widget(&self.buttons_search.clone()).set_text(&flg!("bottom_search_button"));
        self.label_buttons_select.set_text(&flg!("bottom_select_button"));
        get_custom_label_from_widget(&self.buttons_delete.clone()).set_text(&flg!("bottom_delete_button"));
        get_custom_label_from_widget(&self.buttons_save.clone()).set_text(&flg!("bottom_save_button"));
        get_custom_label_from_widget(&self.buttons_symlink.clone()).set_text(&flg!("bottom_symlink_button"));
        get_custom_label_from_widget(&self.buttons_move.clone()).set_text(&flg!("bottom_move_button"));
        get_custom_label_from_widget(&self.buttons_hardlink.clone()).set_text(&flg!("bottom_hardlink_button"));
        self.label_buttons_sort.set_text(&flg!("bottom_sort_button"));

        self.buttons_search.set_tooltip_text(Some(&flg!("bottom_search_button_tooltip")));
        self.buttons_select.set_tooltip_text(Some(&flg!("bottom_select_button_tooltip")));
        self.buttons_delete.set_tooltip_text(Some(&flg!("bottom_delete_button_tooltip")));
        self.buttons_save.set_tooltip_text(Some(&flg!("bottom_save_button_tooltip")));
        self.buttons_symlink.set_tooltip_text(Some(&flg!("bottom_symlink_button_tooltip")));
        self.buttons_move.set_tooltip_text(Some(&flg!("bottom_move_button_tooltip")));
        self.buttons_sort.set_tooltip_text(Some(&flg!("bottom_sort_button_tooltip")));
        if self.buttons_hardlink.is_sensitive() {
            self.buttons_hardlink.set_tooltip_text(Some(&flg!("bottom_hardlink_button_tooltip")));
        } else {
            self.buttons_hardlink.set_tooltip_text(Some(&flg!("bottom_hardlink_button_not_available_tooltip")));
        }

        self.buttons_show_errors.set_tooltip_text(Some(&flg!("bottom_show_errors_tooltip")));
        self.buttons_show_upper_notebook.set_tooltip_text(Some(&flg!("bottom_show_upper_notebook_tooltip")));
    }
}

#[cfg(target_family = "windows")]
fn test_hardlinks() -> bool {
    use directories_next::ProjectDirs;
    use std::fs;
    use std::io::Write;
    use std::path::Path;

    let mut hardlinked = false;

    if let Some(proj_dirs) = ProjectDirs::from("pl", "Qarmin", "Czkawka") {
        let cache_dir = proj_dirs.cache_dir();
        let cache_file = cache_dir.join(Path::new("GILLES_FROM_NOSE.temp"));
        let cache_file_second = cache_dir.join(Path::new("ROBERCIG_LOWANDOWSKI.temp"));

        if cache_file.exists() {
            let _ = fs::remove_file(&cache_file);
        }
        if cache_file_second.exists() {
            let _ = fs::remove_file(&cache_file_second);
        }
        if !cache_file.exists() && !cache_file_second.exists() {
            if let Ok(mut file_handler) = fs::OpenOptions::new().write(true).create(true).open(&cache_file) {
                let _ = writeln!(file_handler, "GURKA");
            }
            let _ = fs::hard_link(&cache_file, &cache_file_second);

            if cache_file.exists() && cache_file_second.exists() {
                hardlinked = true;
            }
        }
        let _ = fs::remove_file(&cache_file);
        let _ = fs::remove_file(&cache_file_second);
    }

    hardlinked
}
