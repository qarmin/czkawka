use gtk::prelude::*;
use gtk::Button;

#[derive(Clone)]
pub struct GuiBottomButtons {
    pub buttons_search: gtk::Button,
    pub buttons_select: gtk::Button,
    pub buttons_delete: gtk::Button,
    pub buttons_save: gtk::Button,
    pub buttons_symlink: gtk::Button,
    pub buttons_hardlink: gtk::Button,
    pub buttons_move: gtk::Button,
    pub buttons_show_errors: gtk::Button,
    pub buttons_names: [String; 7],
    pub buttons_array: [Button; 7],
}

impl GuiBottomButtons {
    pub fn create_from_builder(builder: &gtk::Builder) -> Self {
        let buttons_search: gtk::Button = builder.object("buttons_search").unwrap();
        let buttons_select: gtk::Button = builder.object("buttons_select").unwrap();
        let buttons_delete: gtk::Button = builder.object("buttons_delete").unwrap();
        let buttons_save: gtk::Button = builder.object("buttons_save").unwrap();
        let buttons_symlink: gtk::Button = builder.object("buttons_symlink").unwrap();
        let buttons_hardlink: gtk::Button = builder.object("buttons_hardlink").unwrap();
        let buttons_move: gtk::Button = builder.object("buttons_move").unwrap();

        let buttons_show_errors: gtk::Button = builder.object("buttons_show_errors").unwrap();

        let buttons_names = [
            "search".to_string(),
            "select".to_string(),
            "delete".to_string(),
            "save".to_string(),
            "symlink".to_string(),
            "hardlink".to_string(),
            "move".to_string(),
        ];
        let buttons_array = [
            buttons_search.clone(),
            buttons_select.clone(),
            buttons_delete.clone(),
            buttons_save.clone(),
            buttons_symlink.clone(),
            buttons_hardlink.clone(),
            buttons_move.clone(),
        ];
        Self {
            buttons_search,
            buttons_select,
            buttons_delete,
            buttons_save,
            buttons_symlink,
            buttons_hardlink,
            buttons_move,
            buttons_show_errors,
            buttons_names,
            buttons_array,
        }
    }
}
