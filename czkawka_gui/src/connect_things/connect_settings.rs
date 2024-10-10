use std::collections::BTreeMap;
use std::default::Default;

use czkawka_core::common_cache::{
    get_duplicate_cache_file, get_similar_images_cache_file, get_similar_videos_cache_file, load_cache_from_file_generalized_by_path, load_cache_from_file_generalized_by_size,
    save_cache_to_file_generalized,
};
use czkawka_core::common_messages::Messages;
use czkawka_core::duplicate::HashType;
use directories_next::ProjectDirs;
use gtk4::prelude::*;
use gtk4::{Label, ResponseType, Window};
use image::imageops::FilterType;
use image_hasher::HashAlg;

use crate::flg;
use crate::gui_structs::gui_data::GuiData;
use crate::help_functions::get_dialog_box_child;
use crate::saving_loading::{load_configuration, reset_configuration, save_configuration};

pub fn connect_settings(gui_data: &GuiData) {
    // Connect scale
    {
        let label_restart_needed = gui_data.settings.label_restart_needed.clone();
        gui_data.settings.scale_settings_number_of_threads.connect_value_changed(move |_| {
            if label_restart_needed.label().is_empty() {
                label_restart_needed.set_label(&flg!("settings_label_restart"));
            }
        });
    }
    // Connect button settings
    {
        let button_settings = gui_data.header.button_settings.clone();
        let window_settings = gui_data.settings.window_settings.clone();
        button_settings.connect_clicked(move |_| {
            window_settings.show();
        });

        let window_settings = gui_data.settings.window_settings.clone();

        window_settings.connect_close_request(move |window| {
            window.hide();
            glib::Propagation::Stop
        });
    }

    // Connect save configuration button
    {
        let upper_notebook = gui_data.upper_notebook.clone();
        let settings = gui_data.settings.clone();
        let main_notebook = gui_data.main_notebook.clone();
        let text_view_errors = gui_data.text_view_errors.clone();
        let button_settings_save_configuration = gui_data.settings.button_settings_save_configuration.clone();
        button_settings_save_configuration.connect_clicked(move |_| {
            save_configuration(true, &upper_notebook, &main_notebook, &settings, &text_view_errors);
        });
    }
    // Connect load configuration button
    {
        let upper_notebook = gui_data.upper_notebook.clone();
        let settings = gui_data.settings.clone();
        let main_notebook = gui_data.main_notebook.clone();
        let text_view_errors = gui_data.text_view_errors.clone();
        let button_settings_load_configuration = gui_data.settings.button_settings_load_configuration.clone();
        let scrolled_window_errors = gui_data.scrolled_window_errors.clone();
        button_settings_load_configuration.connect_clicked(move |_| {
            load_configuration(true, &upper_notebook, &main_notebook, &settings, &text_view_errors, &scrolled_window_errors, &Vec::new());
        });
    }
    // Connect reset configuration button
    {
        let upper_notebook = gui_data.upper_notebook.clone();
        let settings = gui_data.settings.clone();
        let main_notebook = gui_data.main_notebook.clone();
        let text_view_errors = gui_data.text_view_errors.clone();
        let button_settings_reset_configuration = gui_data.settings.button_settings_reset_configuration.clone();
        button_settings_reset_configuration.connect_clicked(move |_| {
            reset_configuration(true, &upper_notebook, &main_notebook, &settings, &text_view_errors);
        });
    }
    // Connect button for opening cache
    {
        let button_settings_open_cache_folder = gui_data.settings.button_settings_open_cache_folder.clone();
        button_settings_open_cache_folder.connect_clicked(move |_| {
            if let Some(proj_dirs) = ProjectDirs::from("pl", "Qarmin", "Czkawka") {
                let cache_dir = proj_dirs.cache_dir();

                if let Err(e) = open::that(cache_dir) {
                    println!("Failed to open config folder {cache_dir:?}, reason {e}");
                };
            }
        });
    }
    // Connect button for opening settings
    {
        let button_settings_open_settings_folder = gui_data.settings.button_settings_open_settings_folder.clone();
        button_settings_open_settings_folder.connect_clicked(move |_| {
            if let Some(proj_dirs) = ProjectDirs::from("pl", "Qarmin", "Czkawka") {
                let config_dir = proj_dirs.config_dir();

                if let Err(e) = open::that(config_dir) {
                    println!("Failed to open config folder {config_dir:?}, reason {e}");
                };
            }
        });
    }
    // Connect clear cache methods
    {
        {
            let button_settings_duplicates_clear_cache = gui_data.settings.button_settings_duplicates_clear_cache.clone();
            let settings_window = gui_data.settings.window_settings.clone();
            let text_view_errors = gui_data.text_view_errors.clone();
            let entry_settings_cache_file_minimal_size = gui_data.settings.entry_settings_cache_file_minimal_size.clone();

            button_settings_duplicates_clear_cache.connect_clicked(move |_| {
                let dialog = create_clear_cache_dialog(&flg!("cache_clear_duplicates_title"), &settings_window);
                dialog.show();

                let text_view_errors = text_view_errors.clone();
                let entry_settings_cache_file_minimal_size = entry_settings_cache_file_minimal_size.clone();

                dialog.connect_response(move |dialog, response_type| {
                    if response_type == ResponseType::Ok {
                        let mut messages: Messages = Messages::new();
                        for use_prehash in [true, false] {
                            for type_of_hash in &[HashType::Xxh3, HashType::Blake3, HashType::Crc32] {
                                let (mut messages, loaded_items) = load_cache_from_file_generalized_by_size::<czkawka_core::common_dir_traversal::FileEntry>(
                                    &get_duplicate_cache_file(type_of_hash, use_prehash),
                                    true,
                                    &Default::default(),
                                );

                                if let Some(cache_entries) = loaded_items {
                                    let mut hashmap_to_save: BTreeMap<String, czkawka_core::common_dir_traversal::FileEntry> = Default::default();
                                    for (_, vec_file_entry) in cache_entries {
                                        for file_entry in vec_file_entry {
                                            hashmap_to_save.insert(file_entry.path.to_string_lossy().to_string(), file_entry);
                                        }
                                    }

                                    let minimal_cache_size = entry_settings_cache_file_minimal_size.text().as_str().parse::<u64>().unwrap_or(2 * 1024 * 1024);

                                    let save_messages =
                                        save_cache_to_file_generalized(&get_duplicate_cache_file(type_of_hash, use_prehash), &hashmap_to_save, false, minimal_cache_size);
                                    messages.extend_with_another_messages(save_messages);
                                }
                            }

                            messages.messages.push(flg!("cache_properly_cleared"));
                            text_view_errors.buffer().set_text(messages.create_messages_text().as_str());
                        }
                    }
                    dialog.close();
                });
            });
        }
        {
            let button_settings_similar_images_clear_cache = gui_data.settings.button_settings_similar_images_clear_cache.clone();
            let settings_window = gui_data.settings.window_settings.clone();
            let text_view_errors = gui_data.text_view_errors.clone();

            button_settings_similar_images_clear_cache.connect_clicked(move |_| {
                let dialog = create_clear_cache_dialog(&flg!("cache_clear_similar_images_title"), &settings_window);
                dialog.show();

                let text_view_errors = text_view_errors.clone();

                dialog.connect_response(move |dialog, response_type| {
                    if response_type == ResponseType::Ok {
                        let mut messages: Messages = Messages::new();
                        for hash_size in &[8, 16, 32, 64] {
                            for image_filter in &[
                                FilterType::Lanczos3,
                                FilterType::CatmullRom,
                                FilterType::Gaussian,
                                FilterType::Nearest,
                                FilterType::Triangle,
                            ] {
                                for hash_alg in &[
                                    HashAlg::Blockhash,
                                    HashAlg::Gradient,
                                    HashAlg::DoubleGradient,
                                    HashAlg::VertGradient,
                                    HashAlg::Mean,
                                    HashAlg::Median,
                                ] {
                                    let (mut messages, loaded_items) = load_cache_from_file_generalized_by_path::<czkawka_core::similar_images::ImagesEntry>(
                                        &get_similar_images_cache_file(hash_size, hash_alg, image_filter),
                                        true,
                                        &Default::default(),
                                    );

                                    if let Some(cache_entries) = loaded_items {
                                        let save_messages =
                                            save_cache_to_file_generalized(&get_similar_images_cache_file(hash_size, hash_alg, image_filter), &cache_entries, false, 0);
                                        messages.extend_with_another_messages(save_messages);
                                    }
                                }
                            }
                        }

                        messages.messages.push(flg!("cache_properly_cleared"));
                        text_view_errors.buffer().set_text(messages.create_messages_text().as_str());
                    }
                    dialog.close();
                });
            });
        }
        {
            let button_settings_similar_videos_clear_cache = gui_data.settings.button_settings_similar_videos_clear_cache.clone();
            let settings_window = gui_data.settings.window_settings.clone();
            let text_view_errors = gui_data.text_view_errors.clone();

            button_settings_similar_videos_clear_cache.connect_clicked(move |_| {
                let dialog = create_clear_cache_dialog(&flg!("cache_clear_similar_videos_title"), &settings_window);
                dialog.show();

                let text_view_errors = text_view_errors.clone();

                dialog.connect_response(move |dialog, response_type| {
                    if response_type == ResponseType::Ok {
                        let (mut messages, loaded_items) =
                            load_cache_from_file_generalized_by_path::<czkawka_core::similar_videos::VideosEntry>(&get_similar_videos_cache_file(), true, &Default::default());

                        if let Some(cache_entries) = loaded_items {
                            let save_messages = save_cache_to_file_generalized(&get_similar_videos_cache_file(), &cache_entries, false, 0);
                            messages.extend_with_another_messages(save_messages);
                        }

                        messages.messages.push(flg!("cache_properly_cleared"));
                        text_view_errors.buffer().set_text(messages.create_messages_text().as_str());
                    }
                    dialog.close();
                });
            });
        }
    }
}

fn create_clear_cache_dialog(title_str: &str, window_settings: &Window) -> gtk4::Dialog {
    let dialog = gtk4::Dialog::builder().title(title_str).modal(true).transient_for(window_settings).build();
    dialog.add_button(&flg!("general_ok_button"), ResponseType::Ok);
    dialog.add_button(&flg!("general_close_button"), ResponseType::Cancel);

    let label = Label::builder().label(flg!("cache_clear_message_label_1")).build();
    let label2 = Label::builder().label(flg!("cache_clear_message_label_2")).build();
    let label3 = Label::builder().label(flg!("cache_clear_message_label_3")).build();
    let label4 = Label::builder().label(flg!("cache_clear_message_label_4")).build();

    let internal_box = get_dialog_box_child(&dialog);
    internal_box.append(&label);
    internal_box.append(&label2);
    internal_box.append(&label3);
    internal_box.append(&label4);
    dialog
}
