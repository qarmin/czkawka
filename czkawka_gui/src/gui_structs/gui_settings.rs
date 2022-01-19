use gtk::prelude::*;
use gtk::{Builder, Window};

use crate::flg;

#[derive(Clone)]
pub struct GuiSettings {
    pub window_settings: gtk::Window,

    pub notebook_settings: gtk::Notebook,

    // General
    pub check_button_settings_save_at_exit: gtk::CheckButton,
    pub check_button_settings_load_at_start: gtk::CheckButton,
    pub check_button_settings_confirm_deletion: gtk::CheckButton,
    pub check_button_settings_confirm_link: gtk::CheckButton,
    pub check_button_settings_confirm_group_deletion: gtk::CheckButton,
    pub check_button_settings_show_text_view: gtk::CheckButton,
    pub check_button_settings_use_cache: gtk::CheckButton,
    pub check_button_settings_save_also_json: gtk::CheckButton,
    pub check_button_settings_use_trash: gtk::CheckButton,
    pub label_settings_general_language: gtk::Label,
    pub combo_box_settings_language: gtk::ComboBoxText,

    // Duplicates
    pub check_button_settings_hide_hard_links: gtk::CheckButton,
    pub entry_settings_cache_file_minimal_size: gtk::Entry,
    pub entry_settings_prehash_cache_file_minimal_size: gtk::Entry,
    pub check_button_duplicates_use_prehash_cache: gtk::CheckButton,
    pub check_button_settings_show_preview_duplicates: gtk::CheckButton,
    pub check_button_settings_duplicates_delete_outdated_cache: gtk::CheckButton,
    pub button_settings_duplicates_clear_cache: gtk::Button,
    pub label_settings_duplicate_minimal_size_cache: gtk::Label,
    pub label_settings_duplicate_minimal_size_cache_prehash: gtk::Label,

    // Similar Images
    pub check_button_settings_show_preview_similar_images: gtk::CheckButton,
    pub check_button_settings_similar_images_delete_outdated_cache: gtk::CheckButton,
    pub button_settings_similar_images_clear_cache: gtk::Button,

    // Similar Videos
    pub check_button_settings_similar_videos_delete_outdated_cache: gtk::CheckButton,
    pub button_settings_similar_videos_clear_cache: gtk::Button,

    // Buttons
    pub button_settings_save_configuration: gtk::Button,
    pub button_settings_load_configuration: gtk::Button,
    pub button_settings_reset_configuration: gtk::Button,

    pub button_settings_open_cache_folder: gtk::Button,
    pub button_settings_open_settings_folder: gtk::Button,
}

impl GuiSettings {
    pub fn create_from_builder(window_main: &Window) -> Self {
        let glade_src = include_str!("../../ui/settings.glade").to_string();
        let builder = Builder::from_string(glade_src.as_str());

        let window_settings: gtk::Window = builder.object("window_settings").unwrap();
        window_settings.set_title(&flg!("window_settings_title"));
        window_settings.set_modal(true);
        window_settings.set_transient_for(Some(window_main));

        let notebook_settings: gtk::Notebook = builder.object("notebook_settings").unwrap();

        // General
        let check_button_settings_save_at_exit: gtk::CheckButton = builder.object("check_button_settings_save_at_exit").unwrap();
        let check_button_settings_load_at_start: gtk::CheckButton = builder.object("check_button_settings_load_at_start").unwrap();
        let check_button_settings_confirm_deletion: gtk::CheckButton = builder.object("check_button_settings_confirm_deletion").unwrap();
        let check_button_settings_confirm_link: gtk::CheckButton = builder.object("check_button_settings_confirm_link").unwrap();
        let check_button_settings_confirm_group_deletion: gtk::CheckButton = builder.object("check_button_settings_confirm_group_deletion").unwrap();
        let check_button_settings_show_text_view: gtk::CheckButton = builder.object("check_button_settings_show_text_view").unwrap();
        let check_button_settings_use_cache: gtk::CheckButton = builder.object("check_button_settings_use_cache").unwrap();
        let check_button_settings_save_also_json: gtk::CheckButton = builder.object("check_button_settings_save_also_json").unwrap();
        let check_button_settings_use_trash: gtk::CheckButton = builder.object("check_button_settings_use_trash").unwrap();
        let label_settings_general_language: gtk::Label = builder.object("label_settings_general_language").unwrap();
        let combo_box_settings_language: gtk::ComboBoxText = builder.object("combo_box_settings_language").unwrap();

        // Duplicates
        let check_button_settings_hide_hard_links: gtk::CheckButton = builder.object("check_button_settings_hide_hard_links").unwrap();
        let entry_settings_cache_file_minimal_size: gtk::Entry = builder.object("entry_settings_cache_file_minimal_size").unwrap();
        let check_button_settings_show_preview_duplicates: gtk::CheckButton = builder.object("check_button_settings_show_preview_duplicates").unwrap();
        let check_button_settings_duplicates_delete_outdated_cache: gtk::CheckButton = builder.object("check_button_settings_duplicates_delete_outdated_cache").unwrap();
        let button_settings_duplicates_clear_cache: gtk::Button = builder.object("button_settings_duplicates_clear_cache").unwrap();
        let check_button_duplicates_use_prehash_cache: gtk::CheckButton = builder.object("check_button_duplicates_use_prehash_cache").unwrap();
        let entry_settings_prehash_cache_file_minimal_size: gtk::Entry = builder.object("entry_settings_prehash_cache_file_minimal_size").unwrap();
        let label_settings_duplicate_minimal_size_cache: gtk::Label = builder.object("label_settings_duplicate_minimal_size_cache").unwrap();
        let label_settings_duplicate_minimal_size_cache_prehash: gtk::Label = builder.object("label_settings_duplicate_minimal_size_cache_prehash").unwrap();

        // Similar Images
        let check_button_settings_show_preview_similar_images: gtk::CheckButton = builder.object("check_button_settings_show_preview_similar_images").unwrap();
        let check_button_settings_similar_images_delete_outdated_cache: gtk::CheckButton = builder.object("check_button_settings_similar_images_delete_outdated_cache").unwrap();
        let button_settings_similar_images_clear_cache: gtk::Button = builder.object("button_settings_similar_images_clear_cache").unwrap();

        // Similar Videos
        let check_button_settings_similar_videos_delete_outdated_cache: gtk::CheckButton = builder.object("check_button_settings_similar_videos_delete_outdated_cache").unwrap();
        let button_settings_similar_videos_clear_cache: gtk::Button = builder.object("button_settings_similar_videos_clear_cache").unwrap();

        // Saving/Loading/Resetting configuration
        let button_settings_save_configuration: gtk::Button = builder.object("button_settings_save_configuration").unwrap();
        let button_settings_load_configuration: gtk::Button = builder.object("button_settings_load_configuration").unwrap();
        let button_settings_reset_configuration: gtk::Button = builder.object("button_settings_reset_configuration").unwrap();

        let button_settings_open_cache_folder: gtk::Button = builder.object("button_settings_open_cache_folder").unwrap();
        let button_settings_open_settings_folder: gtk::Button = builder.object("button_settings_open_settings_folder").unwrap();

        Self {
            window_settings,
            notebook_settings,
            check_button_settings_save_at_exit,
            check_button_settings_load_at_start,
            check_button_settings_confirm_deletion,
            check_button_settings_confirm_link,
            check_button_settings_confirm_group_deletion,
            check_button_settings_show_text_view,
            check_button_settings_use_cache,
            check_button_settings_save_also_json,
            check_button_settings_use_trash,
            label_settings_general_language,
            combo_box_settings_language,
            check_button_settings_hide_hard_links,
            entry_settings_cache_file_minimal_size,
            entry_settings_prehash_cache_file_minimal_size,
            check_button_duplicates_use_prehash_cache,
            check_button_settings_show_preview_duplicates,
            check_button_settings_duplicates_delete_outdated_cache,
            button_settings_duplicates_clear_cache,
            label_settings_duplicate_minimal_size_cache,
            label_settings_duplicate_minimal_size_cache_prehash,
            check_button_settings_show_preview_similar_images,
            check_button_settings_similar_images_delete_outdated_cache,
            button_settings_similar_images_clear_cache,
            check_button_settings_similar_videos_delete_outdated_cache,
            button_settings_similar_videos_clear_cache,
            button_settings_save_configuration,
            button_settings_load_configuration,
            button_settings_reset_configuration,
            button_settings_open_cache_folder,
            button_settings_open_settings_folder,
        }
    }

    pub fn update_language(&self) {
        self.window_settings.set_title(&flg!("window_settings_title"));

        self.check_button_settings_save_at_exit.set_label(&flg!("settings_save_at_exit_button"));
        self.check_button_settings_load_at_start.set_label(&flg!("settings_load_at_start_button"));
        self.check_button_settings_confirm_deletion.set_label(&flg!("settings_confirm_deletion_button"));
        self.check_button_settings_confirm_link.set_label(&flg!("settings_confirm_link_button"));
        self.check_button_settings_confirm_group_deletion.set_label(&flg!("settings_confirm_group_deletion_button"));
        self.check_button_settings_show_text_view.set_label(&flg!("settings_show_text_view_button"));
        self.check_button_settings_use_cache.set_label(&flg!("settings_use_cache_button"));
        self.check_button_settings_save_also_json.set_label(&flg!("settings_save_also_as_json_button"));
        self.check_button_settings_use_trash.set_label(&flg!("settings_use_trash_button"));
        self.label_settings_general_language.set_label(&flg!("settings_language_label"));

        self.check_button_settings_save_at_exit
            .set_tooltip_text(Some(&flg!("settings_save_at_exit_button_tooltip")));
        self.check_button_settings_load_at_start
            .set_tooltip_text(Some(&flg!("settings_load_at_start_button_tooltip")));
        self.check_button_settings_confirm_deletion
            .set_tooltip_text(Some(&flg!("settings_confirm_deletion_button_tooltip")));
        self.check_button_settings_confirm_link
            .set_tooltip_text(Some(&flg!("settings_confirm_link_button_tooltip")));
        self.check_button_settings_confirm_group_deletion
            .set_tooltip_text(Some(&flg!("settings_confirm_group_deletion_button_tooltip")));
        self.check_button_settings_show_text_view
            .set_tooltip_text(Some(&flg!("settings_show_text_view_button_tooltip")));
        self.check_button_settings_save_also_json
            .set_tooltip_text(Some(&flg!("settings_save_also_as_json_button_tooltip")));
        self.check_button_settings_use_cache.set_tooltip_text(Some(&flg!("settings_use_cache_button_tooltip")));
        self.check_button_settings_use_trash.set_tooltip_text(Some(&flg!("settings_use_trash_button_tooltip")));
        self.label_settings_general_language.set_tooltip_text(Some(&flg!("settings_language_label_tooltip")));

        self.check_button_settings_hide_hard_links.set_label(&flg!("settings_duplicates_hide_hard_link_button"));
        self.check_button_settings_show_preview_duplicates
            .set_label(&flg!("settings_multiple_image_preview_checkbutton"));
        self.check_button_settings_duplicates_delete_outdated_cache
            .set_label(&flg!("settings_multiple_delete_outdated_cache_checkbutton"));
        self.button_settings_duplicates_clear_cache.set_label(&flg!("settings_multiple_clear_cache_button"));
        self.check_button_duplicates_use_prehash_cache.set_label(&flg!("settings_duplicates_prehash_checkbutton"));
        self.label_settings_duplicate_minimal_size_cache
            .set_label(&flg!("settings_duplicates_minimal_size_cache_label"));
        self.label_settings_duplicate_minimal_size_cache_prehash
            .set_label(&flg!("settings_duplicates_minimal_size_cache_prehash_label"));

        self.check_button_settings_hide_hard_links
            .set_tooltip_text(Some(&flg!("settings_duplicates_hide_hard_link_button_tooltip")));
        self.entry_settings_cache_file_minimal_size
            .set_tooltip_text(Some(&flg!("settings_duplicates_minimal_size_entry_tooltip")));
        self.check_button_settings_show_preview_duplicates
            .set_tooltip_text(Some(&flg!("settings_multiple_image_preview_checkbutton_tooltip")));
        self.check_button_settings_duplicates_delete_outdated_cache
            .set_tooltip_text(Some(&flg!("settings_multiple_delete_outdated_cache_checkbutton_tooltip")));
        self.button_settings_duplicates_clear_cache
            .set_tooltip_text(Some(&flg!("settings_multiple_clear_cache_button_tooltip")));
        self.check_button_duplicates_use_prehash_cache
            .set_tooltip_text(Some(&flg!("settings_duplicates_prehash_checkbutton_tooltip")));
        self.entry_settings_prehash_cache_file_minimal_size
            .set_tooltip_text(Some(&flg!("settings_duplicates_prehash_minimal_entry_tooltip")));

        self.check_button_settings_show_preview_similar_images
            .set_label(&flg!("settings_multiple_image_preview_checkbutton"));
        self.check_button_settings_similar_images_delete_outdated_cache
            .set_label(&flg!("settings_multiple_delete_outdated_cache_checkbutton"));
        self.button_settings_similar_images_clear_cache.set_label(&flg!("settings_multiple_clear_cache_button"));

        self.check_button_settings_show_preview_similar_images
            .set_tooltip_text(Some(&flg!("settings_multiple_image_preview_checkbutton_tooltip")));
        self.check_button_settings_similar_images_delete_outdated_cache
            .set_tooltip_text(Some(&flg!("settings_multiple_delete_outdated_cache_checkbutton_tooltip")));
        self.button_settings_similar_images_clear_cache
            .set_tooltip_text(Some(&flg!("settings_multiple_clear_cache_button_tooltip")));

        self.check_button_settings_similar_videos_delete_outdated_cache
            .set_label(&flg!("settings_multiple_delete_outdated_cache_checkbutton"));
        self.button_settings_similar_videos_clear_cache.set_label(&flg!("settings_multiple_clear_cache_button"));

        self.check_button_settings_similar_videos_delete_outdated_cache
            .set_tooltip_text(Some(&flg!("settings_multiple_delete_outdated_cache_checkbutton_tooltip")));
        self.button_settings_similar_videos_clear_cache
            .set_tooltip_text(Some(&flg!("settings_multiple_clear_cache_button_tooltip")));

        self.button_settings_save_configuration.set_label(&flg!("settings_saving_button"));
        self.button_settings_load_configuration.set_label(&flg!("settings_loading_button"));
        self.button_settings_reset_configuration.set_label(&flg!("settings_reset_button"));

        self.button_settings_save_configuration.set_tooltip_text(Some(&flg!("settings_saving_button_tooltip")));
        self.button_settings_load_configuration.set_tooltip_text(Some(&flg!("settings_loading_button_tooltip")));
        self.button_settings_reset_configuration.set_tooltip_text(Some(&flg!("settings_reset_button_tooltip")));

        self.button_settings_open_cache_folder.set_label(&flg!("settings_folder_cache_open"));
        self.button_settings_open_settings_folder.set_label(&flg!("settings_folder_settings_open"));

        self.button_settings_open_cache_folder.set_tooltip_text(Some(&flg!("settings_folder_cache_open_tooltip")));
        self.button_settings_open_settings_folder
            .set_tooltip_text(Some(&flg!("settings_folder_settings_open_tooltip")));

        let vec_children: Vec<gtk::Widget> = self.notebook_settings.children();

        // let vec_children: Vec<gtk::Widget> = get_all_children(&self.notebook_settings);
        // let vec_children: Vec<gtk::Widget> = get_all_children(&vec_children[1]);

        // Change name of main notebook tabs
        let names: [String; 4] = [
            flg!("settings_notebook_general"),
            flg!("settings_notebook_duplicates"),
            flg!("settings_notebook_images"),
            flg!("settings_notebook_videos"),
        ];
        for (index, fl_thing) in names.iter().enumerate() {
            self.notebook_settings
                .tab_label(&vec_children[index])
                .unwrap()
                .downcast::<gtk::Label>()
                .unwrap()
                .set_text(fl_thing);
        }
    }
}
