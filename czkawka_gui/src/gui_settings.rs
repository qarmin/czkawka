use gtk::prelude::*;
use gtk::{Builder, Window};

#[derive(Clone)]
pub struct GuiSettings {
    pub window_settings: gtk::Window,

    // General
    pub check_button_settings_save_at_exit: gtk::CheckButton,
    pub check_button_settings_load_at_start: gtk::CheckButton,
    pub check_button_settings_confirm_deletion: gtk::CheckButton,
    pub check_button_settings_confirm_group_deletion: gtk::CheckButton,
    pub check_button_settings_show_text_view: gtk::CheckButton,
    pub check_button_settings_use_cache: gtk::CheckButton,
    pub check_button_settings_use_trash: gtk::CheckButton,

    // Duplicates
    pub check_button_settings_hide_hard_links: gtk::CheckButton,
    pub entry_settings_cache_file_minimal_size: gtk::Entry,
    pub check_button_settings_show_preview_duplicates: gtk::CheckButton,
    pub check_button_settings_duplicates_delete_outdated_cache: gtk::CheckButton,

    // Similar Images
    pub check_button_settings_show_preview_similar_images: gtk::CheckButton,
    pub check_button_settings_similar_images_delete_outdated_cache: gtk::CheckButton,

    // Similar Videos
    pub check_button_settings_similar_videos_delete_outdated_cache: gtk::CheckButton,

    // Buttons
    pub button_settings_save_configuration: gtk::Button,
    pub button_settings_load_configuration: gtk::Button,
    pub button_settings_reset_configuration: gtk::Button,

    pub button_settings_open_cache_folder: gtk::Button,
    pub button_settings_open_settings_folder: gtk::Button,
}

impl GuiSettings {
    pub fn create_from_builder(window_main: &Window) -> Self {
        let glade_src = include_str!("../ui/settings.glade").to_string();
        let builder = Builder::from_string(glade_src.as_str());

        let window_settings: gtk::Window = builder.object("window_settings").unwrap();
        window_settings.set_modal(true);
        window_settings.set_transient_for(Some(window_main));

        // General
        let check_button_settings_save_at_exit: gtk::CheckButton = builder.object("check_button_settings_save_at_exit").unwrap();
        let check_button_settings_load_at_start: gtk::CheckButton = builder.object("check_button_settings_load_at_start").unwrap();
        let check_button_settings_confirm_deletion: gtk::CheckButton = builder.object("check_button_settings_confirm_deletion").unwrap();
        let check_button_settings_confirm_group_deletion: gtk::CheckButton = builder.object("check_button_settings_confirm_group_deletion").unwrap();
        let check_button_settings_show_text_view: gtk::CheckButton = builder.object("check_button_settings_show_text_view").unwrap();
        let check_button_settings_use_cache: gtk::CheckButton = builder.object("check_button_settings_use_cache").unwrap();
        let check_button_settings_use_trash: gtk::CheckButton = builder.object("check_button_settings_use_trash").unwrap();

        check_button_settings_save_at_exit.set_tooltip_text(Some("Saves configuration to file when closing app."));
        check_button_settings_load_at_start.set_tooltip_text(Some("Loading at start configuration from file.\n\nNot selecting this option will load default settings."));
        check_button_settings_confirm_deletion.set_tooltip_text(Some("Shows confirmation dialog when clicking at delete button."));
        check_button_settings_confirm_group_deletion.set_tooltip_text(Some("Shows dialog when trying to remove all records from group."));
        check_button_settings_show_text_view.set_tooltip_text(Some("Shows error panel at bottom."));
        check_button_settings_use_cache.set_tooltip_text(Some("Option to which allows to not use cache feature."));
        check_button_settings_use_trash.set_tooltip_text(Some("When enabled it moves files to trash instead deleting them permanently."));

        // Duplicates
        let check_button_settings_hide_hard_links: gtk::CheckButton = builder.object("check_button_settings_hide_hard_links").unwrap();
        let entry_settings_cache_file_minimal_size: gtk::Entry = builder.object("entry_settings_cache_file_minimal_size").unwrap();
        let check_button_settings_show_preview_duplicates: gtk::CheckButton = builder.object("check_button_settings_show_preview_duplicates").unwrap();
        let check_button_settings_duplicates_delete_outdated_cache: gtk::CheckButton = builder.object("check_button_settings_duplicates_delete_outdated_cache").unwrap();

        check_button_settings_hide_hard_links.set_tooltip_text(Some(
            "Hides all files except one, if are points to same data(are hardlinked).\n\nE.g. in case where on disk there is 7 files which are hardlinked to specific data and one different file with same data but different inode, then in duplicate finder will be visible only one unique file and one file from hardlinked ones.",
        ));
        entry_settings_cache_file_minimal_size.set_tooltip_text(Some(
            "Allows to set minimal size of file, which will be cached.\n\nChoosing smaller value, will generate more records which will speedup search, but slowdown cache loading/saving.",
        ));
        check_button_settings_show_preview_duplicates.set_tooltip_text(Some("Shows preview at right side, when selecting image file."));
        check_button_settings_duplicates_delete_outdated_cache.set_tooltip_text(Some("Allows to delete outdated cache results which points to non-existent files.\n\nWhen enabled, app make sure when loading records, that all points to valid files and ignore broken ones.\n\nDisabling this option, will help to scan files on external drives, so cache entries about them will not be purged in next scan.\n\nIn case of having hundred of thousands records in cache, it is suggested to enable this option, to speedup cache loading and saving at start and end of scan."));

        // Similar Images
        let check_button_settings_show_preview_similar_images: gtk::CheckButton = builder.object("check_button_settings_show_preview_similar_images").unwrap();
        let check_button_settings_similar_images_delete_outdated_cache: gtk::CheckButton = builder.object("check_button_settings_similar_images_delete_outdated_cache").unwrap();

        check_button_settings_show_preview_similar_images.set_tooltip_text(Some("Shows preview at right side, when selecting image file."));
        check_button_settings_similar_images_delete_outdated_cache.set_tooltip_text(Some("Allows to delete outdated cache results which points to non-existent files.\n\nWhen enabled, app make sure when loading records, that all points to valid files and ignore broken ones.\n\nDisabling this option, will help to scan files on external drives, so cache entries about them will not be purged in next scan.\n\nIn case of having hundred of thousands records in cache, it is suggested to enable this option, to speedup cache loading and saving at start and end of scan."));

        // Similar Videos
        let check_button_settings_similar_videos_delete_outdated_cache: gtk::CheckButton = builder.object("check_button_settings_similar_videos_delete_outdated_cache").unwrap();
        check_button_settings_similar_videos_delete_outdated_cache.set_tooltip_text(Some("Allows to delete outdated cache results which points to non-existent files.\n\nWhen enabled, app make sure when loading records, that all points to valid files and ignore broken ones.\n\nDisabling this option, will help to scan files on external drives, so cache entries about them will not be purged in next scan.\n\nIn case of having hundred of thousands records in cache, it is suggested to enable this option, to speedup cache loading and saving at start and end of scan."));

        // Saving/Loading/Resetting configuration
        let button_settings_save_configuration: gtk::Button = builder.object("button_settings_save_configuration").unwrap();
        let button_settings_load_configuration: gtk::Button = builder.object("button_settings_load_configuration").unwrap();
        let button_settings_reset_configuration: gtk::Button = builder.object("button_settings_reset_configuration").unwrap();

        button_settings_save_configuration.set_tooltip_text(Some("Save current settings configuration to file."));
        button_settings_load_configuration.set_tooltip_text(Some("Load settings from file and replace current configuration with them."));
        button_settings_reset_configuration.set_tooltip_text(Some("Reset current configuration to default one."));

        let button_settings_open_cache_folder: gtk::Button = builder.object("button_settings_open_cache_folder").unwrap();
        let button_settings_open_settings_folder: gtk::Button = builder.object("button_settings_open_settings_folder").unwrap();

        button_settings_open_cache_folder.set_tooltip_text(Some(
            "Opens folder where are stored txt files with cache.\n\nModifying them may cause to show invalid results but also modifying e.g. path may save time when moving big amount of files to different place.\n\nYou can copy this files between computers to save time on scanning again for files(of course if they have similar directory structure).\n\nIn case of problems with cache, this files can be removed, so app will automatically regenerate them.",
        ));
        button_settings_open_settings_folder.set_tooltip_text(Some("Opens folder where Czkawka config are stored.\n\nModifying them, may cause to show."));

        Self {
            window_settings,
            check_button_settings_save_at_exit,
            check_button_settings_load_at_start,
            check_button_settings_confirm_deletion,
            check_button_settings_confirm_group_deletion,
            check_button_settings_show_text_view,
            check_button_settings_use_cache,
            check_button_settings_use_trash,
            check_button_settings_hide_hard_links,
            entry_settings_cache_file_minimal_size,
            check_button_settings_show_preview_duplicates,
            check_button_settings_duplicates_delete_outdated_cache,
            check_button_settings_show_preview_similar_images,
            check_button_settings_similar_images_delete_outdated_cache,
            check_button_settings_similar_videos_delete_outdated_cache,
            button_settings_save_configuration,
            button_settings_load_configuration,
            button_settings_reset_configuration,
            button_settings_open_cache_folder,
            button_settings_open_settings_folder,
        }
    }
}
