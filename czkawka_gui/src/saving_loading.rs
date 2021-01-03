use crate::gui_data::*;
use crate::help_functions::*;
use directories_next::ProjectDirs;
use gtk::prelude::*;
use gtk::{EntryExt, GtkListStoreExt, ToggleButtonExt};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::{env, fs};

// TODO add more settings, maybe to different dialog window

const SAVE_FILE_NAME: &str = "czkawka_gui_config.txt";

pub fn save_configuration(gui_data: &GuiData, manual_execution: bool) {
    let check_button_settings_save_at_exit = gui_data.check_button_settings_save_at_exit.clone();
    let text_view_errors = gui_data.text_view_errors.clone();

    reset_text_view(&text_view_errors);

    if !manual_execution && !check_button_settings_save_at_exit.get_active() {
        // When check button is deselected, not save configuration at exit
        return;
    }
    if let Some(proj_dirs) = ProjectDirs::from("pl", "Qarmin", "Czkawka") {
        // Lin: /home/alice/.config/barapp
        // Win: C:\Users\Alice\AppData\Roaming\Foo Corp\Bar App\config
        // Mac: /Users/Alice/Library/Application Support/com.Foo-Corp.Bar-App

        let config_dir = proj_dirs.config_dir();
        if config_dir.exists() {
            if !config_dir.is_dir() {
                add_text_to_text_view(&text_view_errors, format!("Cannot create save file inside {} because this isn't a folder.", config_dir.display()).as_str());
                return;
            }
        } else if fs::create_dir_all(config_dir).is_err() {
            add_text_to_text_view(&text_view_errors, format!("Failed configuration to create configuration folder {}", config_dir.display()).as_str());
            return;
        }

        let mut data_to_save: Vec<String> = Vec::new();

        //// Included Directories
        data_to_save.push("--included_directories:".to_string());
        let scrolled_window_included_directories = gui_data.scrolled_window_included_directories.clone();
        let list_store = get_list_store(&scrolled_window_included_directories);
        if let Some(iter) = list_store.get_iter_first() {
            loop {
                data_to_save.push(list_store.get_value(&iter, ColumnsDirectory::Path as i32).get::<String>().unwrap().unwrap());
                if !list_store.iter_next(&iter) {
                    break;
                }
            }
        }

        //// Excluded Directories
        data_to_save.push("--excluded_directories:".to_string());
        let scrolled_window_excluded_directories = gui_data.scrolled_window_excluded_directories.clone();
        let list_store = get_list_store(&scrolled_window_excluded_directories);
        if let Some(iter) = list_store.get_iter_first() {
            loop {
                data_to_save.push(list_store.get_value(&iter, ColumnsDirectory::Path as i32).get::<String>().unwrap().unwrap());
                if !list_store.iter_next(&iter) {
                    break;
                }
            }
        }

        {
            //// Excluded Items
            data_to_save.push("--excluded_items:".to_string());
            let entry_excluded_items = gui_data.entry_excluded_items.clone();
            for item in entry_excluded_items.get_text().split(',') {
                if item.trim().is_empty() {
                    continue;
                }
                data_to_save.push(item.to_string());
            }

            //// Allowed extensions
            data_to_save.push("--allowed_extensions:".to_string());
            let entry_allowed_extensions = gui_data.entry_allowed_extensions.clone();
            for extension in entry_allowed_extensions.get_text().split(',') {
                if extension.trim().is_empty() {
                    continue;
                }
                data_to_save.push(extension.to_string());
            }

            //// Save at exit
            data_to_save.push("--save_at_exit:".to_string());
            let check_button_settings_save_at_exit = gui_data.check_button_settings_save_at_exit.clone();
            data_to_save.push(check_button_settings_save_at_exit.get_active().to_string());

            //// Load at start
            data_to_save.push("--load_at_start:".to_string());
            let check_button_settings_load_at_start = gui_data.check_button_settings_load_at_start.clone();
            data_to_save.push(check_button_settings_load_at_start.get_active().to_string());

            //// Confirm deletion of files
            data_to_save.push("--confirm_deletion:".to_string());
            let check_button_settings_confirm_deletion = gui_data.check_button_settings_confirm_deletion.clone();
            data_to_save.push(check_button_settings_confirm_deletion.get_active().to_string());

            //// Show image previews in similar images
            data_to_save.push("--show_previews:".to_string());
            let check_button_settings_show_preview_similar_images = gui_data.check_button_settings_show_preview_similar_images.clone();
            data_to_save.push(check_button_settings_show_preview_similar_images.get_active().to_string());

            //// Show bottom text panel with errors
            data_to_save.push("--bottom_text_panel:".to_string());
            let check_button_settings_show_text_view = gui_data.check_button_settings_show_text_view.clone();
            data_to_save.push(check_button_settings_show_text_view.get_active().to_string());
        }

        // Creating/Opening config file

        let config_file = config_dir.join(Path::new(SAVE_FILE_NAME));

        let mut config_file_handler = match File::create(&config_file) {
            Ok(t) => t,
            Err(_) => {
                add_text_to_text_view(&text_view_errors, format!("Failed to create config file {}", config_dir.display()).as_str());
                return;
            }
        };

        let mut data_saved: bool = false;
        for data in data_to_save {
            match writeln!(config_file_handler, "{}", data) {
                Ok(_) => {
                    data_saved = true;
                }
                Err(_) => {
                    data_saved = false;
                    break;
                }
            }
        }
        if data_saved {
            add_text_to_text_view(&text_view_errors, format!("Saved configuration to file {}", config_dir.display()).as_str());
        } else {
            add_text_to_text_view(&text_view_errors, format!("Failed to save configuration data to file {}", config_dir.display()).as_str());
        }
    } else {
        add_text_to_text_view(&text_view_errors, "Failed to get home directory, so can't save file.");
    }
}

enum TypeOfLoadedData {
    None,
    IncludedDirectories,
    ExcludedDirectories,
    ExcludedItems,
    AllowedExtensions,
    LoadingAtStart,
    SavingAtExit,
    ConfirmDeletion,
    ShowPreviews,
    BottomTextPanel,
}

pub fn load_configuration(gui_data: &GuiData, manual_execution: bool) {
    let text_view_errors = gui_data.text_view_errors.clone();

    reset_text_view(&text_view_errors);

    if let Some(proj_dirs) = ProjectDirs::from("pl", "Qarmin", "Czkawka") {
        // Lin: /home/alice/.config/barapp
        // Win: C:\Users\Alice\AppData\Roaming\Foo Corp\Bar App\config
        // Mac: /Users/Alice/Library/Application Support/com.Foo-Corp.Bar-App

        let config_dir = proj_dirs.config_dir();
        let config_file = config_dir.join(Path::new(SAVE_FILE_NAME));
        if !config_file.exists() || !config_file.is_file() {
            if manual_execution {
                // Don't show errors when there is no configuration file when starting app
                add_text_to_text_view(&text_view_errors, format!("Cannot load configuration from file {:?}.", config_file.display()).as_str());
            }
            return;
        }

        // Loading Data
        let loaded_data: String = match fs::read_to_string(&config_file) {
            Ok(t) => t,
            Err(_) => {
                add_text_to_text_view(&text_view_errors, format!("Failed to read data from file {:?}.", config_file).as_str());
                return;
            }
        };

        // Parsing Data

        let mut included_directories: Vec<String> = Vec::new();
        let mut excluded_directories: Vec<String> = Vec::new();
        let mut excluded_items: Vec<String> = Vec::new();
        let mut allowed_extensions: Vec<String> = Vec::new();
        let mut loading_at_start: bool = true;
        let mut saving_at_exit: bool = true;
        let mut confirm_deletion: bool = true;
        let mut show_previews: bool = true;
        let mut bottom_text_panel: bool = true;

        let mut current_type = TypeOfLoadedData::None;
        for (line_number, line) in loaded_data.replace("\r\n", "\n").split('\n').enumerate() {
            let line: String = line.trim().to_string();
            if line.is_empty() {
                continue; // Empty line, so we just skip it
            }
            if line.starts_with("--included_directories") {
                current_type = TypeOfLoadedData::IncludedDirectories;
            } else if line.starts_with("--excluded_directories") {
                current_type = TypeOfLoadedData::ExcludedDirectories;
            } else if line.starts_with("--excluded_items") {
                current_type = TypeOfLoadedData::ExcludedItems;
            } else if line.starts_with("--allowed_extensions") {
                current_type = TypeOfLoadedData::AllowedExtensions;
            } else if line.starts_with("--load_at_start") {
                current_type = TypeOfLoadedData::LoadingAtStart;
            } else if line.starts_with("--save_at_exit") {
                current_type = TypeOfLoadedData::SavingAtExit;
            } else if line.starts_with("--confirm_deletion") {
                current_type = TypeOfLoadedData::ConfirmDeletion;
            } else if line.starts_with("--show_previews") {
                current_type = TypeOfLoadedData::ShowPreviews;
            } else if line.starts_with("--bottom_text_panel") {
                current_type = TypeOfLoadedData::BottomTextPanel;
            } else if line.starts_with("--") {
                current_type = TypeOfLoadedData::None;
                add_text_to_text_view(
                    &text_view_errors,
                    format!("Found invalid header in line {} \"{}\" when loading file {:?} (save file may be from different Czkawka version)", line_number, line, config_file).as_str(),
                );
            } else {
                match current_type {
                    TypeOfLoadedData::None => {
                        add_text_to_text_view(
                            &text_view_errors,
                            format!("Found orphan data in line {} \"{}\" when loading file {:?} (save file may be from different Czkawka version)", line_number, line, config_file).as_str(),
                        );
                    }
                    TypeOfLoadedData::IncludedDirectories => {
                        included_directories.push(line);
                    }
                    TypeOfLoadedData::ExcludedDirectories => {
                        excluded_directories.push(line);
                    }
                    TypeOfLoadedData::ExcludedItems => {
                        excluded_items.push(line);
                    }
                    TypeOfLoadedData::AllowedExtensions => {
                        allowed_extensions.push(line);
                    }
                    TypeOfLoadedData::LoadingAtStart => {
                        let line = line.to_lowercase();
                        if line == "1" || line == "true" {
                            loading_at_start = true;
                        } else if line == "0" || line == "false" {
                            loading_at_start = false;
                        } else {
                            add_text_to_text_view(
                                &text_view_errors,
                                format!("Found invalid data in line {} \"{}\" isn't proper value(0/1/true/false) when loading file {:?}", line_number, line, config_file).as_str(),
                            );
                        }
                    }
                    TypeOfLoadedData::SavingAtExit => {
                        let line = line.to_lowercase();
                        if line == "1" || line == "true" {
                            saving_at_exit = true;
                        } else if line == "0" || line == "false" {
                            saving_at_exit = false;
                        } else {
                            add_text_to_text_view(
                                &text_view_errors,
                                format!("Found invalid data in line {} \"{}\" isn't proper value(0/1/true/false) when loading file {:?}", line_number, line, config_file).as_str(),
                            );
                        }
                    }
                    TypeOfLoadedData::ConfirmDeletion => {
                        let line = line.to_lowercase();
                        if line == "1" || line == "true" {
                            confirm_deletion = true;
                        } else if line == "0" || line == "false" {
                            confirm_deletion = false;
                        } else {
                            add_text_to_text_view(
                                &text_view_errors,
                                format!("Found invalid data in line {} \"{}\" isn't proper value(0/1/true/false) when loading file {:?}", line_number, line, config_file).as_str(),
                            );
                        }
                    }
                    TypeOfLoadedData::ShowPreviews => {
                        let line = line.to_lowercase();
                        if line == "1" || line == "true" {
                            show_previews = true;
                        } else if line == "0" || line == "false" {
                            show_previews = false;
                        } else {
                            add_text_to_text_view(
                                &text_view_errors,
                                format!("Found invalid data in line {} \"{}\" isn't proper value(0/1/true/false) when loading file {:?}", line_number, line, config_file).as_str(),
                            );
                        }
                    }
                    TypeOfLoadedData::BottomTextPanel => {
                        let line = line.to_lowercase();
                        if line == "1" || line == "true" {
                            bottom_text_panel = true;
                        } else if line == "0" || line == "false" {
                            bottom_text_panel = false;
                        } else {
                            add_text_to_text_view(
                                &text_view_errors,
                                format!("Found invalid data in line {} \"{}\" isn't proper value(0/1/true/false) when loading file {:?}", line_number, line, config_file).as_str(),
                            );
                        }
                    }
                }
            }
        }

        // Setting data
        if manual_execution || loading_at_start {
            //// Included Directories
            let scrolled_window_included_directories = gui_data.scrolled_window_included_directories.clone();
            let list_store = get_list_store(&scrolled_window_included_directories);
            list_store.clear();

            let col_indices = [0];

            for directory in included_directories {
                let values: [&dyn ToValue; 1] = [&directory];
                list_store.set(&list_store.append(), &col_indices, &values);
            }

            //// Exclude Directories
            let scrolled_window_excluded_directories = gui_data.scrolled_window_excluded_directories.clone();
            let list_store = get_list_store(&scrolled_window_excluded_directories);
            list_store.clear();

            let col_indices = [0];

            for directory in excluded_directories {
                let values: [&dyn ToValue; 1] = [&directory];
                list_store.set(&list_store.append(), &col_indices, &values);
            }

            //// Excluded Items
            let entry_excluded_items = gui_data.entry_excluded_items.clone();
            entry_excluded_items.set_text(excluded_items.iter().map(|e| e.to_string() + ",").collect::<String>().as_str());

            //// Allowed extensions
            let entry_allowed_extensions = gui_data.entry_allowed_extensions.clone();
            entry_allowed_extensions.set_text(allowed_extensions.iter().map(|e| e.to_string() + ",").collect::<String>().as_str());

            //// Buttons
            gui_data.check_button_settings_load_at_start.set_active(loading_at_start);
            gui_data.check_button_settings_save_at_exit.set_active(saving_at_exit);
            gui_data.check_button_settings_confirm_deletion.set_active(confirm_deletion);
            gui_data.check_button_settings_show_preview_similar_images.set_active(show_previews);

            gui_data.check_button_settings_show_text_view.set_active(bottom_text_panel);
            if !bottom_text_panel {
                gui_data.scrolled_window_errors.hide();
            } else {
                gui_data.scrolled_window_errors.show();
            }
        } else {
            gui_data.check_button_settings_load_at_start.set_active(false);
        }

        if manual_execution {
            add_text_to_text_view(&text_view_errors, format!("Properly loaded configuration from file {:?}", config_file).as_str());
        }
    } else {
        add_text_to_text_view(&text_view_errors, "Failed to get home directory, so can't load file.");
    }
}

pub fn reset_configuration(gui_data: &GuiData, manual_clearing: bool) {
    // TODO Maybe add popup dialog to confirm resetting
    let text_view_errors = gui_data.text_view_errors.clone();

    reset_text_view(&text_view_errors);

    // Resetting included directories
    {
        let col_indices = [0];
        let scrolled_window_included_directories = gui_data.scrolled_window_included_directories.clone();
        let list_store = get_list_store(&scrolled_window_included_directories);
        list_store.clear();

        let current_dir: String = match env::current_dir() {
            Ok(t) => t.to_str().unwrap().to_string(),
            Err(_) => {
                if cfg!(target_family = "unix") {
                    add_text_to_text_view(&text_view_errors, "Failed to read current directory, setting /home instead");
                    "/home".to_string()
                } else if cfg!(target_family = "windows") {
                    add_text_to_text_view(&text_view_errors, "Failed to read current directory, setting C:\\ instead");
                    "C:\\".to_string()
                } else {
                    "".to_string()
                }
            }
        };

        let values: [&dyn ToValue; 1] = [&current_dir];
        list_store.set(&list_store.append(), &col_indices, &values);
    }
    // Resetting excluded directories
    {
        let col_indices = [0];
        let scrolled_window_excluded_directories = gui_data.scrolled_window_excluded_directories.clone();
        let list_store = get_list_store(&scrolled_window_excluded_directories);
        list_store.clear();
        if cfg!(target_family = "unix") {
            for i in ["/proc", "/dev", "/sys", "/run", "/snap"].iter() {
                let values: [&dyn ToValue; 1] = [&i];
                list_store.set(&list_store.append(), &col_indices, &values);
            }
        }
    }
    // Resetting excluded items
    {
        let entry_excluded_items = gui_data.entry_excluded_items.clone();
        if cfg!(target_family = "unix") {
            entry_excluded_items.set_text("*/.git/*,*/node_modules/*,*/lost+found/*,*/Trash/*,*/.Trash-*/*,*/snap/*,/home/*/.cache/*");
        }
        if cfg!(target_family = "windows") {
            entry_excluded_items.set_text("*/.git/*,*/node_modules/*,*/lost+found/*,*:/windows/*");
        }
    }
    // Resetting allowed extensions
    {
        let entry_allowed_extensions = gui_data.entry_allowed_extensions.clone();
        entry_allowed_extensions.set_text("");
    }

    // Set settings
    {
        gui_data.check_button_settings_save_at_exit.set_active(true);
        gui_data.check_button_settings_load_at_start.set_active(true);
        gui_data.check_button_settings_confirm_deletion.set_active(true);
        gui_data.check_button_settings_show_preview_similar_images.set_active(true);
        gui_data.check_button_settings_show_text_view.set_active(true);
    }
    if manual_clearing {
        add_text_to_text_view(&text_view_errors, "Current configuration was cleared.");
    }
}
