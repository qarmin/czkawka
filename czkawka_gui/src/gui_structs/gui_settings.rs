use gtk4::prelude::*;
use gtk4::{Builder, Window};

use crate::flg;
use crate::help_functions::get_all_direct_children;

#[derive(Clone)]
pub struct GuiSettings {
    pub window_settings: Window,

    pub notebook_settings: gtk4::Notebook,

    // General
    pub check_button_settings_save_at_exit: gtk4::CheckButton,
    pub check_button_settings_load_at_start: gtk4::CheckButton,
    pub check_button_settings_confirm_deletion: gtk4::CheckButton,
    pub check_button_settings_confirm_link: gtk4::CheckButton,
    pub check_button_settings_confirm_group_deletion: gtk4::CheckButton,
    pub check_button_settings_show_text_view: gtk4::CheckButton,
    pub check_button_settings_use_cache: gtk4::CheckButton,
    pub check_button_settings_save_also_json: gtk4::CheckButton,
    pub check_button_settings_use_trash: gtk4::CheckButton,
    pub label_settings_general_language: gtk4::Label,
    pub combo_box_settings_language: gtk4::ComboBoxText,
    pub check_button_settings_one_filesystem: gtk4::CheckButton,
    pub label_settings_number_of_threads: gtk4::Label,
    pub scale_settings_number_of_threads: gtk4::Scale,
    pub label_restart_needed: gtk4::Label,

    // Duplicates
    pub check_button_settings_hide_hard_links: gtk4::CheckButton,
    pub entry_settings_cache_file_minimal_size: gtk4::Entry,
    pub entry_settings_prehash_cache_file_minimal_size: gtk4::Entry,
    pub check_button_duplicates_use_prehash_cache: gtk4::CheckButton,
    pub check_button_settings_show_preview_duplicates: gtk4::CheckButton,
    pub check_button_settings_duplicates_delete_outdated_cache: gtk4::CheckButton,
    pub button_settings_duplicates_clear_cache: gtk4::Button,
    pub label_settings_duplicate_minimal_size_cache: gtk4::Label,
    pub label_settings_duplicate_minimal_size_cache_prehash: gtk4::Label,

    // Similar Images
    pub check_button_settings_show_preview_similar_images: gtk4::CheckButton,
    pub check_button_settings_similar_images_delete_outdated_cache: gtk4::CheckButton,
    pub button_settings_similar_images_clear_cache: gtk4::Button,

    // Similar Videos
    pub check_button_settings_similar_videos_delete_outdated_cache: gtk4::CheckButton,
    pub button_settings_similar_videos_clear_cache: gtk4::Button,

    // Buttons
    pub button_settings_save_configuration: gtk4::Button,
    pub button_settings_load_configuration: gtk4::Button,
    pub button_settings_reset_configuration: gtk4::Button,

    pub button_settings_open_cache_folder: gtk4::Button,
    pub button_settings_open_settings_folder: gtk4::Button,
}

impl GuiSettings {
    pub fn create_from_builder(window_main: &Window) -> Self {
        let glade_src = include_str!("../../ui/settings.ui").to_string();
        let builder = Builder::from_string(glade_src.as_str());

        let window_settings: Window = builder.object("window_settings").expect("Cambalache");
        window_settings.set_title(Some(&flg!("window_settings_title")));
        window_settings.set_modal(true);
        window_settings.set_transient_for(Some(window_main));

        let notebook_settings: gtk4::Notebook = builder.object("notebook_settings").expect("Cambalache");

        // General
        let check_button_settings_one_filesystem: gtk4::CheckButton = builder.object("check_button_settings_one_filesystem").expect("Cambalache");
        let check_button_settings_save_at_exit: gtk4::CheckButton = builder.object("check_button_settings_save_at_exit").expect("Cambalache");
        let check_button_settings_load_at_start: gtk4::CheckButton = builder.object("check_button_settings_load_at_start").expect("Cambalache");
        let check_button_settings_confirm_deletion: gtk4::CheckButton = builder.object("check_button_settings_confirm_deletion").expect("Cambalache");
        let check_button_settings_confirm_link: gtk4::CheckButton = builder.object("check_button_settings_confirm_link").expect("Cambalache");
        let check_button_settings_confirm_group_deletion: gtk4::CheckButton = builder.object("check_button_settings_confirm_group_deletion").expect("Cambalache");
        let check_button_settings_show_text_view: gtk4::CheckButton = builder.object("check_button_settings_show_text_view").expect("Cambalache");
        let check_button_settings_use_cache: gtk4::CheckButton = builder.object("check_button_settings_use_cache").expect("Cambalache");
        let check_button_settings_save_also_json: gtk4::CheckButton = builder.object("check_button_settings_save_also_json").expect("Cambalache");
        let check_button_settings_use_trash: gtk4::CheckButton = builder.object("check_button_settings_use_trash").expect("Cambalache");
        let label_settings_general_language: gtk4::Label = builder.object("label_settings_general_language").expect("Cambalache");
        let combo_box_settings_language: gtk4::ComboBoxText = builder.object("combo_box_settings_language").expect("Cambalache");
        let label_settings_number_of_threads: gtk4::Label = builder.object("label_settings_number_of_threads").expect("Cambalache");
        let scale_settings_number_of_threads: gtk4::Scale = builder.object("scale_settings_number_of_threads").expect("Cambalache");
        let label_restart_needed: gtk4::Label = builder.object("label_restart_needed").expect("Cambalache");

        // Duplicates
        let check_button_settings_hide_hard_links: gtk4::CheckButton = builder.object("check_button_settings_hide_hard_links").expect("Cambalache");
        let entry_settings_cache_file_minimal_size: gtk4::Entry = builder.object("entry_settings_cache_file_minimal_size").expect("Cambalache");
        let check_button_settings_show_preview_duplicates: gtk4::CheckButton = builder.object("check_button_settings_show_preview_duplicates").expect("Cambalache");
        let check_button_settings_duplicates_delete_outdated_cache: gtk4::CheckButton =
            builder.object("check_button_settings_duplicates_delete_outdated_cache").expect("Cambalache");
        let button_settings_duplicates_clear_cache: gtk4::Button = builder.object("button_settings_duplicates_clear_cache").expect("Cambalache");
        let check_button_duplicates_use_prehash_cache: gtk4::CheckButton = builder.object("check_button_duplicates_use_prehash_cache").expect("Cambalache");
        let entry_settings_prehash_cache_file_minimal_size: gtk4::Entry = builder.object("entry_settings_prehash_cache_file_minimal_size").expect("Cambalache");
        let label_settings_duplicate_minimal_size_cache: gtk4::Label = builder.object("label_settings_duplicate_minimal_size_cache").expect("Cambalache");
        let label_settings_duplicate_minimal_size_cache_prehash: gtk4::Label = builder.object("label_settings_duplicate_minimal_size_cache_prehash").expect("Cambalache");

        // Similar Images
        let check_button_settings_show_preview_similar_images: gtk4::CheckButton = builder.object("check_button_settings_show_preview_similar_images").expect("Cambalache");
        let check_button_settings_similar_images_delete_outdated_cache: gtk4::CheckButton =
            builder.object("check_button_settings_similar_images_delete_outdated_cache").expect("Cambalache");
        let button_settings_similar_images_clear_cache: gtk4::Button = builder.object("button_settings_similar_images_clear_cache").expect("Cambalache");

        // Similar Videos
        let check_button_settings_similar_videos_delete_outdated_cache: gtk4::CheckButton =
            builder.object("check_button_settings_similar_videos_delete_outdated_cache").expect("Cambalache");
        let button_settings_similar_videos_clear_cache: gtk4::Button = builder.object("button_settings_similar_videos_clear_cache").expect("Cambalache");

        // Saving/Loading/Resetting configuration
        let button_settings_save_configuration: gtk4::Button = builder.object("button_settings_save_configuration").expect("Cambalache");
        let button_settings_load_configuration: gtk4::Button = builder.object("button_settings_load_configuration").expect("Cambalache");
        let button_settings_reset_configuration: gtk4::Button = builder.object("button_settings_reset_configuration").expect("Cambalache");

        let button_settings_open_cache_folder: gtk4::Button = builder.object("button_settings_open_cache_folder").expect("Cambalache");
        let button_settings_open_settings_folder: gtk4::Button = builder.object("button_settings_open_settings_folder").expect("Cambalache");

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
            check_button_settings_one_filesystem,
            label_settings_number_of_threads,
            scale_settings_number_of_threads,
            label_restart_needed,
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
        self.window_settings.set_title(Some(&flg!("window_settings_title")));

        if !self.label_restart_needed.label().is_empty() {
            self.label_restart_needed.set_label(&flg!("settings_label_restart"));
        }

        self.check_button_settings_save_at_exit.set_label(Some(&flg!("settings_save_at_exit_button")));
        self.check_button_settings_load_at_start.set_label(Some(&flg!("settings_load_at_start_button")));
        self.check_button_settings_confirm_deletion.set_label(Some(&flg!("settings_confirm_deletion_button")));
        self.check_button_settings_confirm_link.set_label(Some(&flg!("settings_confirm_link_button")));
        self.check_button_settings_confirm_group_deletion
            .set_label(Some(&flg!("settings_confirm_group_deletion_button")));
        self.check_button_settings_show_text_view.set_label(Some(&flg!("settings_show_text_view_button")));
        self.check_button_settings_use_cache.set_label(Some(&flg!("settings_use_cache_button")));
        self.check_button_settings_save_also_json.set_label(Some(&flg!("settings_save_also_as_json_button")));
        self.check_button_settings_use_trash.set_label(Some(&flg!("settings_use_trash_button")));
        self.label_settings_general_language.set_label(&flg!("settings_language_label"));
        self.check_button_settings_one_filesystem.set_label(Some(&flg!("settings_ignore_other_filesystems")));
        self.label_settings_number_of_threads.set_label(&flg!("settings_number_of_threads"));

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
        self.check_button_settings_one_filesystem
            .set_tooltip_text(Some(&flg!("settings_ignore_other_filesystems_tooltip")));
        self.scale_settings_number_of_threads.set_tooltip_text(Some(&flg!("settings_number_of_threads_tooltip")));

        self.check_button_settings_hide_hard_links
            .set_label(Some(&flg!("settings_duplicates_hide_hard_link_button")));
        self.check_button_settings_show_preview_duplicates
            .set_label(Some(&flg!("settings_multiple_image_preview_checkbutton")));
        self.check_button_settings_duplicates_delete_outdated_cache
            .set_label(Some(&flg!("settings_multiple_delete_outdated_cache_checkbutton")));
        self.button_settings_duplicates_clear_cache.set_label(&flg!("settings_multiple_clear_cache_button"));
        self.check_button_duplicates_use_prehash_cache
            .set_label(Some(&flg!("settings_duplicates_prehash_checkbutton")));
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
            .set_label(Some(&flg!("settings_multiple_image_preview_checkbutton")));
        self.check_button_settings_similar_images_delete_outdated_cache
            .set_label(Some(&flg!("settings_multiple_delete_outdated_cache_checkbutton")));
        self.button_settings_similar_images_clear_cache.set_label(&flg!("settings_multiple_clear_cache_button"));

        self.check_button_settings_show_preview_similar_images
            .set_tooltip_text(Some(&flg!("settings_multiple_image_preview_checkbutton_tooltip")));
        self.check_button_settings_similar_images_delete_outdated_cache
            .set_tooltip_text(Some(&flg!("settings_multiple_delete_outdated_cache_checkbutton_tooltip")));
        self.button_settings_similar_images_clear_cache
            .set_tooltip_text(Some(&flg!("settings_multiple_clear_cache_button_tooltip")));

        self.check_button_settings_similar_videos_delete_outdated_cache
            .set_label(Some(&flg!("settings_multiple_delete_outdated_cache_checkbutton")));
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

        let vec_children: Vec<gtk4::Widget> = get_all_direct_children(&self.notebook_settings);
        let vec_children: Vec<gtk4::Widget> = get_all_direct_children(&vec_children[1]);

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
                .expect("Couldn't get tab label")
                .downcast::<gtk4::Label>()
                .expect("Couldn't downcast to Label")
                .set_text(fl_thing);
        }
    }
}
