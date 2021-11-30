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

        buttons_search.set_tooltip_text(Some("Start to search for files/folders"));
        buttons_select.set_tooltip_text(Some("Selects records\nOnly selected files/folders can be later processed."));
        buttons_delete.set_tooltip_text(Some("Delete selected files/folders"));
        buttons_save.set_tooltip_text(Some("Save data about search to file"));
        buttons_symlink.set_tooltip_text(Some(
            "Creates symbolic links\nOnly works when at least 2 results in group are selected\nFirst is unchanged and second and later are symlinked to first",
        ));
        buttons_hardlink.set_tooltip_text(Some("Creates hardlinks\nOnly works when at least 2 results in group are selected\nFirst is unchanged and second and later are hardlinked to first"));
        buttons_move.set_tooltip_text(Some(
            "Moves files to chosen folder\nIt copy all files to folder without preserving directory tree\nWhen trying to move 2 files with identical name to folder, second will fail and show error",
        ));

        let buttons_show_errors: gtk::Button = builder.object("buttons_show_errors").unwrap();
        buttons_show_errors.set_tooltip_text(Some("Show/Hide bottom error panel"));

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
