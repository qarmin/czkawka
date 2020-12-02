use humansize::{file_size_opts as options, FileSize};

extern crate gtk;
use crate::gui_data::GuiData;
use crate::help_functions::*;
use chrono::NaiveDateTime;
use czkawka_core::duplicate::CheckingMethod;
use czkawka_core::same_music::MusicSimilarity;
use glib::Receiver;
use gtk::prelude::*;

pub fn connect_compute_results(gui_data: &GuiData, glib_stop_receiver: Receiver<Message>) {
    let buttons_search = gui_data.buttons_search.clone();
    let notebook_main = gui_data.notebook_main.clone();
    let entry_info = gui_data.entry_info.clone();
    let scrolled_window_main_empty_folder_finder = gui_data.scrolled_window_main_empty_folder_finder.clone();
    let scrolled_window_main_empty_files_finder = gui_data.scrolled_window_main_empty_files_finder.clone();
    let scrolled_window_duplicate_finder = gui_data.scrolled_window_duplicate_finder.clone();
    let scrolled_window_similar_images_finder = gui_data.scrolled_window_similar_images_finder.clone();
    let buttons_array = gui_data.buttons_array.clone();
    let text_view_errors = gui_data.text_view_errors.clone();
    let shared_duplication_state = gui_data.shared_duplication_state.clone();
    let shared_buttons = gui_data.shared_buttons.clone();
    let scrolled_window_zeroed_files_finder = gui_data.scrolled_window_zeroed_files_finder.clone();
    let shared_empty_folders_state = gui_data.shared_empty_folders_state.clone();
    let shared_empty_files_state = gui_data.shared_empty_files_state.clone();
    let scrolled_window_big_files_finder = gui_data.scrolled_window_big_files_finder.clone();
    let shared_big_files_state = gui_data.shared_big_files_state.clone();
    let scrolled_window_main_temporary_files_finder = gui_data.scrolled_window_main_temporary_files_finder.clone();
    let shared_temporary_files_state = gui_data.shared_temporary_files_state.clone();
    let shared_similar_images_state = gui_data.shared_similar_images_state.clone();
    let shared_zeroed_files_state = gui_data.shared_zeroed_files_state.clone();
    let scrolled_window_same_music_finder = gui_data.scrolled_window_same_music_finder.clone();
    let shared_same_music_state = gui_data.shared_same_music_state.clone();
    let buttons_names = gui_data.buttons_names.clone();
    let dialog_progress = gui_data.dialog_progress.clone();

    glib_stop_receiver.attach(None, move |msg| {
        buttons_search.show();

        dialog_progress.hide();

        // Restore clickability to main notebook
        notebook_main.set_sensitive(true);

        match msg {
            Message::Duplicates(df) => {
                if df.get_stopped_search() {
                    entry_info.set_text("Searching for duplicated was stopped by user");
                } else {
                    let information = df.get_information();
                    let text_messages = df.get_text_messages();

                    let duplicates_number: usize;
                    let duplicates_size: u64;
                    let duplicates_group: usize;

                    match df.get_check_method() {
                        CheckingMethod::Name => {
                            duplicates_number = information.number_of_duplicated_files_by_name;
                            duplicates_size = 0;
                            duplicates_group = information.number_of_groups_by_name;
                            entry_info.set_text(format!("Found {} files in {} groups which have same names.", duplicates_number, duplicates_group).as_str());
                        }
                        CheckingMethod::Hash | CheckingMethod::HashMB => {
                            duplicates_number = information.number_of_duplicated_files_by_hash;
                            duplicates_size = information.lost_space_by_hash;
                            duplicates_group = information.number_of_groups_by_hash;
                            entry_info.set_text(format!("Found {} duplicates files in {} groups which took {}.", duplicates_number, duplicates_group, duplicates_size.file_size(options::BINARY).unwrap()).as_str());
                        }
                        CheckingMethod::Size => {
                            duplicates_number = information.number_of_duplicated_files_by_size;
                            duplicates_size = information.lost_space_by_size;
                            duplicates_group = information.number_of_groups_by_size;
                            entry_info.set_text(format!("Found {} duplicates files in {} groups which took {}.", duplicates_number, duplicates_group, duplicates_size.file_size(options::BINARY).unwrap()).as_str());
                        }
                        CheckingMethod::None => {
                            panic!();
                        }
                    }

                    entry_info.set_text(format!("Found {} duplicates files in {} groups which took {}.", duplicates_number, duplicates_group, duplicates_size.file_size(options::BINARY).unwrap()).as_str());

                    // Create GUI
                    {
                        let list_store = get_list_store(&scrolled_window_duplicate_finder);

                        let col_indices = [0, 1, 2, 3, 4, 5];

                        match df.get_check_method() {
                            CheckingMethod::Name => {
                                let btreemap = df.get_files_sorted_by_names();

                                for (name, vector) in btreemap.iter().rev() {
                                    let values: [&dyn ToValue; 6] = [
                                        &name,
                                        &(format!("{} results", vector.len())),
                                        &"".to_string(), // No text in 3 column
                                        &(0),            // Not used here
                                        &(HEADER_ROW_COLOR.to_string()),
                                        &(TEXT_COLOR.to_string()),
                                    ];
                                    list_store.set(&list_store.append(), &col_indices, &values);
                                    for entry in vector {
                                        let (directory, file) = split_path(&entry.path);
                                        let values: [&dyn ToValue; 6] = [
                                            &file,
                                            &directory,
                                            &(format!("{} - ({})", NaiveDateTime::from_timestamp(entry.modified_date as i64, 0).to_string(), entry.size.file_size(options::BINARY).unwrap())),
                                            &(entry.modified_date),
                                            &(MAIN_ROW_COLOR.to_string()),
                                            &(TEXT_COLOR.to_string()),
                                        ];
                                        list_store.set(&list_store.append(), &col_indices, &values);
                                    }
                                }
                            }
                            CheckingMethod::Hash | CheckingMethod::HashMB => {
                                let btreemap = df.get_files_sorted_by_hash();

                                for (size, vectors_vector) in btreemap.iter().rev() {
                                    for vector in vectors_vector {
                                        let values: [&dyn ToValue; 6] = [
                                            &(format!("{} x {} ({} bytes)", vector.len(), size.file_size(options::BINARY).unwrap(), size)),
                                            &(format!("{} ({} bytes) lost", ((vector.len() - 1) as u64 * *size as u64).file_size(options::BINARY).unwrap(), (vector.len() - 1) as u64 * *size as u64)),
                                            &"".to_string(), // No text in 3 column
                                            &(0),            // Not used here
                                            &(HEADER_ROW_COLOR.to_string()),
                                            &(TEXT_COLOR.to_string()),
                                        ];
                                        list_store.set(&list_store.append(), &col_indices, &values);
                                        for entry in vector {
                                            let (directory, file) = split_path(&entry.path);
                                            let values: [&dyn ToValue; 6] = [
                                                &file,
                                                &directory,
                                                &(NaiveDateTime::from_timestamp(entry.modified_date as i64, 0).to_string()),
                                                &(entry.modified_date),
                                                &(MAIN_ROW_COLOR.to_string()),
                                                &(TEXT_COLOR.to_string()),
                                            ];
                                            list_store.set(&list_store.append(), &col_indices, &values);
                                        }
                                    }
                                }
                            }
                            CheckingMethod::Size => {
                                let btreemap = df.get_files_sorted_by_size();

                                for (size, vector) in btreemap.iter().rev() {
                                    let values: [&dyn ToValue; 6] = [
                                        &(format!("{} x {} ({} bytes)", vector.len(), size.file_size(options::BINARY).unwrap(), size)),
                                        &(format!("{} ({} bytes) lost", ((vector.len() - 1) as u64 * *size as u64).file_size(options::BINARY).unwrap(), (vector.len() - 1) as u64 * *size as u64)),
                                        &"".to_string(), // No text in 3 column
                                        &(0),            // Not used here
                                        &(HEADER_ROW_COLOR.to_string()),
                                        &(TEXT_COLOR.to_string()),
                                    ];
                                    list_store.set(&list_store.append(), &col_indices, &values);
                                    for entry in vector {
                                        let (directory, file) = split_path(&entry.path);
                                        let values: [&dyn ToValue; 6] = [
                                            &file,
                                            &directory,
                                            &(NaiveDateTime::from_timestamp(entry.modified_date as i64, 0).to_string()),
                                            &(entry.modified_date),
                                            &(MAIN_ROW_COLOR.to_string()),
                                            &(TEXT_COLOR.to_string()),
                                        ];
                                        list_store.set(&list_store.append(), &col_indices, &values);
                                    }
                                }
                            }
                            CheckingMethod::None => {
                                panic!();
                            }
                        }

                        print_text_messages_to_text_view(text_messages, &text_view_errors);
                    }

                    // Set state
                    {
                        *shared_duplication_state.borrow_mut() = df;

                        if duplicates_number > 0 {
                            *shared_buttons.borrow_mut().get_mut("duplicate").unwrap().get_mut("save").unwrap() = true;
                            *shared_buttons.borrow_mut().get_mut("duplicate").unwrap().get_mut("delete").unwrap() = true;
                            *shared_buttons.borrow_mut().get_mut("duplicate").unwrap().get_mut("select").unwrap() = true;
                        } else {
                            *shared_buttons.borrow_mut().get_mut("duplicate").unwrap().get_mut("save").unwrap() = false;
                            *shared_buttons.borrow_mut().get_mut("duplicate").unwrap().get_mut("delete").unwrap() = false;
                            *shared_buttons.borrow_mut().get_mut("duplicate").unwrap().get_mut("select").unwrap() = false;
                        }
                        set_buttons(&mut *shared_buttons.borrow_mut().get_mut("duplicate").unwrap(), &buttons_array, &buttons_names);
                    }
                }
            }
            Message::EmptyFolders(ef) => {
                if ef.get_stopped_search() {
                    entry_info.set_text("Searching for empty folders was stopped by user");
                } else {
                    let information = ef.get_information();
                    let text_messages = ef.get_text_messages();

                    let empty_folder_number: usize = information.number_of_empty_folders;

                    entry_info.set_text(format!("Found {} empty folders.", empty_folder_number).as_str());

                    // Create GUI
                    {
                        let list_store = get_list_store(&scrolled_window_main_empty_folder_finder);

                        let col_indices = [0, 1, 2];

                        let hashmap = ef.get_empty_folder_list();

                        for (path, entry) in hashmap {
                            let (directory, file) = split_path(path);
                            let values: [&dyn ToValue; 3] = [&file, &directory, &(NaiveDateTime::from_timestamp(entry.modified_date as i64, 0).to_string())];
                            list_store.set(&list_store.append(), &col_indices, &values);
                        }
                        print_text_messages_to_text_view(text_messages, &text_view_errors);
                    }

                    // Set state
                    {
                        *shared_empty_folders_state.borrow_mut() = ef;

                        if empty_folder_number > 0 {
                            *shared_buttons.borrow_mut().get_mut("empty_folder").unwrap().get_mut("save").unwrap() = true;
                            *shared_buttons.borrow_mut().get_mut("empty_folder").unwrap().get_mut("delete").unwrap() = true;
                            *shared_buttons.borrow_mut().get_mut("empty_folder").unwrap().get_mut("select").unwrap() = true;
                        } else {
                            *shared_buttons.borrow_mut().get_mut("empty_folder").unwrap().get_mut("save").unwrap() = false;
                            *shared_buttons.borrow_mut().get_mut("empty_folder").unwrap().get_mut("delete").unwrap() = false;
                            *shared_buttons.borrow_mut().get_mut("empty_folder").unwrap().get_mut("select").unwrap() = false;
                        }
                        set_buttons(&mut *shared_buttons.borrow_mut().get_mut("empty_folder").unwrap(), &buttons_array, &buttons_names);
                    }
                }
            }
            Message::EmptyFiles(vf) => {
                if vf.get_stopped_search() {
                    entry_info.set_text("Searching for empty files was stopped by user");
                } else {
                    let information = vf.get_information();
                    let text_messages = vf.get_text_messages();

                    let empty_files_number: usize = information.number_of_empty_files;

                    entry_info.set_text(format!("Found {} empty files.", empty_files_number).as_str());

                    // Create GUI
                    {
                        let list_store = get_list_store(&scrolled_window_main_empty_files_finder);

                        let col_indices = [0, 1, 2];

                        let vector = vf.get_empty_files();

                        for file_entry in vector {
                            let (directory, file) = split_path(&file_entry.path);
                            let values: [&dyn ToValue; 3] = [&file, &directory, &(NaiveDateTime::from_timestamp(file_entry.modified_date as i64, 0).to_string())];
                            list_store.set(&list_store.append(), &col_indices, &values);
                        }
                        print_text_messages_to_text_view(text_messages, &text_view_errors);
                    }

                    // Set state
                    {
                        *shared_empty_files_state.borrow_mut() = vf;

                        if empty_files_number > 0 {
                            *shared_buttons.borrow_mut().get_mut("empty_file").unwrap().get_mut("save").unwrap() = true;
                            *shared_buttons.borrow_mut().get_mut("empty_file").unwrap().get_mut("delete").unwrap() = true;
                            *shared_buttons.borrow_mut().get_mut("empty_file").unwrap().get_mut("select").unwrap() = true;
                        } else {
                            *shared_buttons.borrow_mut().get_mut("empty_file").unwrap().get_mut("save").unwrap() = false;
                            *shared_buttons.borrow_mut().get_mut("empty_file").unwrap().get_mut("delete").unwrap() = false;
                            *shared_buttons.borrow_mut().get_mut("empty_file").unwrap().get_mut("select").unwrap() = false;
                        }
                        set_buttons(&mut *shared_buttons.borrow_mut().get_mut("empty_file").unwrap(), &buttons_array, &buttons_names);
                    }
                }
            }
            Message::BigFiles(bf) => {
                if bf.get_stopped_search() {
                    entry_info.set_text("Searching for big files was stopped by user");
                } else {
                    let information = bf.get_information();
                    let text_messages = bf.get_text_messages();

                    let biggest_files_number: usize = information.number_of_real_files;

                    entry_info.set_text(format!("Found {} biggest files.", biggest_files_number).as_str());

                    // Create GUI
                    {
                        let list_store = get_list_store(&scrolled_window_big_files_finder);

                        let col_indices = [0, 1, 2, 3];

                        let btreemap = bf.get_big_files();

                        for (size, vector) in btreemap.iter().rev() {
                            for file_entry in vector {
                                let (directory, file) = split_path(&file_entry.path);
                                let values: [&dyn ToValue; 4] = [
                                    &(format!("{} ({} bytes)", size.file_size(options::BINARY).unwrap(), size)),
                                    &file,
                                    &directory,
                                    &(NaiveDateTime::from_timestamp(file_entry.modified_date as i64, 0).to_string()),
                                ];
                                list_store.set(&list_store.append(), &col_indices, &values);
                            }
                        }
                        print_text_messages_to_text_view(text_messages, &text_view_errors);
                    }

                    // Set state
                    {
                        *shared_big_files_state.borrow_mut() = bf;

                        if biggest_files_number > 0 {
                            *shared_buttons.borrow_mut().get_mut("big_file").unwrap().get_mut("save").unwrap() = true;
                            *shared_buttons.borrow_mut().get_mut("big_file").unwrap().get_mut("delete").unwrap() = true;
                            *shared_buttons.borrow_mut().get_mut("big_file").unwrap().get_mut("select").unwrap() = true;
                        } else {
                            *shared_buttons.borrow_mut().get_mut("big_file").unwrap().get_mut("save").unwrap() = false;
                            *shared_buttons.borrow_mut().get_mut("big_file").unwrap().get_mut("delete").unwrap() = false;
                            *shared_buttons.borrow_mut().get_mut("big_file").unwrap().get_mut("select").unwrap() = false;
                        }
                        set_buttons(&mut *shared_buttons.borrow_mut().get_mut("big_file").unwrap(), &buttons_array, &buttons_names);
                    }
                }
            }
            Message::Temporary(tf) => {
                if tf.get_stopped_search() {
                    entry_info.set_text("Searching for temporary files was stopped by user");
                } else {
                    let information = tf.get_information();
                    let text_messages = tf.get_text_messages();

                    let temporary_files_number: usize = information.number_of_temporary_files;

                    entry_info.set_text(format!("Found {} temporary files.", temporary_files_number).as_str());

                    // Create GUI
                    {
                        let list_store = get_list_store(&scrolled_window_main_temporary_files_finder);

                        let col_indices = [0, 1, 2];

                        let vector = tf.get_temporary_files();

                        for file_entry in vector {
                            let (directory, file) = split_path(&file_entry.path);
                            let values: [&dyn ToValue; 3] = [&file, &directory, &(NaiveDateTime::from_timestamp(file_entry.modified_date as i64, 0).to_string())];
                            list_store.set(&list_store.append(), &col_indices, &values);
                        }
                        print_text_messages_to_text_view(text_messages, &text_view_errors);
                    }

                    // Set state
                    {
                        *shared_temporary_files_state.borrow_mut() = tf;

                        if temporary_files_number > 0 {
                            *shared_buttons.borrow_mut().get_mut("temporary_file").unwrap().get_mut("save").unwrap() = true;
                            *shared_buttons.borrow_mut().get_mut("temporary_file").unwrap().get_mut("delete").unwrap() = true;
                            *shared_buttons.borrow_mut().get_mut("temporary_file").unwrap().get_mut("select").unwrap() = true;
                        } else {
                            *shared_buttons.borrow_mut().get_mut("temporary_file").unwrap().get_mut("save").unwrap() = false;
                            *shared_buttons.borrow_mut().get_mut("temporary_file").unwrap().get_mut("delete").unwrap() = false;
                            *shared_buttons.borrow_mut().get_mut("temporary_file").unwrap().get_mut("select").unwrap() = false;
                        }
                        set_buttons(&mut *shared_buttons.borrow_mut().get_mut("temporary_file").unwrap(), &buttons_array, &buttons_names);
                    }
                }
            }
            Message::SimilarImages(sf) => {
                if sf.get_stopped_search() {
                    entry_info.set_text("Searching for duplicated was stopped by user");
                } else {
                    //let information = sf.get_information();
                    let text_messages = sf.get_text_messages();

                    let base_images_size = sf.get_similar_images().len();

                    entry_info.set_text(format!("Found similar pictures for {} images.", base_images_size).as_str());

                    // Create GUI
                    {
                        let list_store = get_list_store(&scrolled_window_similar_images_finder);

                        let col_indices = [0, 1, 2, 3, 4, 5, 6, 7, 8];

                        let vec_struct_similar = sf.get_similar_images();

                        for vec_file_entry in vec_struct_similar.iter() {
                            // Header
                            let values: [&dyn ToValue; 9] = [
                                &"".to_string(),
                                &"".to_string(),
                                &"".to_string(),
                                &"".to_string(),
                                &"".to_string(),
                                &"".to_string(),
                                &(0),
                                &(HEADER_ROW_COLOR.to_string()),
                                &(TEXT_COLOR.to_string()),
                            ];
                            list_store.set(&list_store.append(), &col_indices, &values);

                            // Meat
                            for file_entry in vec_file_entry.iter() {
                                let (directory, file) = split_path(&file_entry.path);
                                let values: [&dyn ToValue; 9] = [
                                    &(get_text_from_similarity(&file_entry.similarity).to_string()),
                                    &file_entry.size.file_size(options::BINARY).unwrap(),
                                    &file_entry.dimensions,
                                    &file,
                                    &directory,
                                    &(NaiveDateTime::from_timestamp(file_entry.modified_date as i64, 0).to_string()),
                                    &(file_entry.modified_date),
                                    &(MAIN_ROW_COLOR.to_string()),
                                    &(TEXT_COLOR.to_string()),
                                ];
                                list_store.set(&list_store.append(), &col_indices, &values);
                            }
                        }

                        print_text_messages_to_text_view(text_messages, &text_view_errors);
                    }

                    // Set state
                    {
                        *shared_similar_images_state.borrow_mut() = sf;

                        if base_images_size > 0 {
                            *shared_buttons.borrow_mut().get_mut("similar_images").unwrap().get_mut("save").unwrap() = true;
                            *shared_buttons.borrow_mut().get_mut("similar_images").unwrap().get_mut("delete").unwrap() = true;
                            *shared_buttons.borrow_mut().get_mut("similar_images").unwrap().get_mut("select").unwrap() = true;
                        } else {
                            *shared_buttons.borrow_mut().get_mut("similar_images").unwrap().get_mut("save").unwrap() = false;
                            *shared_buttons.borrow_mut().get_mut("similar_images").unwrap().get_mut("delete").unwrap() = false;
                            *shared_buttons.borrow_mut().get_mut("similar_images").unwrap().get_mut("select").unwrap() = false;
                        }
                        set_buttons(&mut *shared_buttons.borrow_mut().get_mut("similar_images").unwrap(), &buttons_array, &buttons_names);
                    }
                }
            }
            Message::ZeroedFiles(zf) => {
                if zf.get_stopped_search() {
                    entry_info.set_text("Searching for zeroed files was stopped by user");
                } else {
                    let information = zf.get_information();
                    let text_messages = zf.get_text_messages();

                    let zeroed_files_number: usize = information.number_of_zeroed_files;

                    entry_info.set_text(format!("Found {} zeroed files.", zeroed_files_number).as_str());

                    // Create GUI
                    {
                        let list_store = get_list_store(&scrolled_window_zeroed_files_finder);

                        let col_indices = [0, 1, 2, 3];

                        let vector = zf.get_zeroed_files();

                        for file_entry in vector {
                            let (directory, file) = split_path(&file_entry.path);
                            let values: [&dyn ToValue; 4] = [
                                &(file_entry.size.file_size(options::BINARY).unwrap()),
                                &file,
                                &directory,
                                &(NaiveDateTime::from_timestamp(file_entry.modified_date as i64, 0).to_string()),
                            ];
                            list_store.set(&list_store.append(), &col_indices, &values);
                        }
                        print_text_messages_to_text_view(text_messages, &text_view_errors);
                    }

                    // Set state
                    {
                        *shared_zeroed_files_state.borrow_mut() = zf;

                        if zeroed_files_number > 0 {
                            *shared_buttons.borrow_mut().get_mut("zeroed_files").unwrap().get_mut("save").unwrap() = true;
                            *shared_buttons.borrow_mut().get_mut("zeroed_files").unwrap().get_mut("delete").unwrap() = true;
                            *shared_buttons.borrow_mut().get_mut("zeroed_files").unwrap().get_mut("select").unwrap() = true;
                        } else {
                            *shared_buttons.borrow_mut().get_mut("zeroed_files").unwrap().get_mut("save").unwrap() = false;
                            *shared_buttons.borrow_mut().get_mut("zeroed_files").unwrap().get_mut("delete").unwrap() = false;
                            *shared_buttons.borrow_mut().get_mut("zeroed_files").unwrap().get_mut("select").unwrap() = false;
                        }
                        set_buttons(&mut *shared_buttons.borrow_mut().get_mut("zeroed_files").unwrap(), &buttons_array, &buttons_names);
                    }
                }
            }
            Message::SameMusic(mf) => {
                if mf.get_stopped_search() {
                    entry_info.set_text("Searching for empty files was stopped by user");
                } else {
                    let information = mf.get_information();
                    let text_messages = mf.get_text_messages();

                    let same_music_number: usize = information.number_of_duplicates_music_files;

                    entry_info.set_text(format!("Found {} duplicated music files.", same_music_number).as_str());

                    // Create GUI
                    {
                        let list_store = get_list_store(&scrolled_window_same_music_finder);

                        let col_indices = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];

                        let vector = mf.get_duplicated_music_entries();

                        let music_similarity = *mf.get_music_similarity();

                        let is_title = (MusicSimilarity::TITLE & music_similarity) != MusicSimilarity::NONE;
                        let is_artist = (MusicSimilarity::ARTIST & music_similarity) != MusicSimilarity::NONE;
                        let is_album_title = (MusicSimilarity::ALBUM_TITLE & music_similarity) != MusicSimilarity::NONE;
                        let is_album_artist = (MusicSimilarity::ALBUM_ARTIST & music_similarity) != MusicSimilarity::NONE;
                        let is_year = (MusicSimilarity::YEAR & music_similarity) != MusicSimilarity::NONE;

                        let text: String = "-----".to_string();

                        for vec_file_entry in vector {
                            let values: [&dyn ToValue; 12] = [
                                &"".to_string(),
                                &"".to_string(),
                                &"".to_string(),
                                &(match is_title {
                                    true => text.clone(),
                                    false => "".to_string(),
                                }),
                                &(match is_artist {
                                    true => text.clone(),
                                    false => "".to_string(),
                                }),
                                &(match is_album_title {
                                    true => text.clone(),
                                    false => "".to_string(),
                                }),
                                &(match is_album_artist {
                                    true => text.clone(),
                                    false => "".to_string(),
                                }),
                                &(match is_year {
                                    true => text.clone(),
                                    false => "".to_string(),
                                }),
                                &"".to_string(),
                                &(0),
                                &(HEADER_ROW_COLOR.to_string()),
                                &(TEXT_COLOR.to_string()),
                            ];
                            list_store.set(&list_store.append(), &col_indices, &values);
                            for file_entry in vec_file_entry {
                                let (directory, file) = split_path(&file_entry.path);
                                let values: [&dyn ToValue; 12] = [
                                    &file_entry.size.file_size(options::BINARY).unwrap(),
                                    &file,
                                    &directory,
                                    &file_entry.title,
                                    &file_entry.artist,
                                    &file_entry.album_title,
                                    &file_entry.album_artist,
                                    &file_entry.year.to_string(),
                                    &(NaiveDateTime::from_timestamp(file_entry.modified_date as i64, 0).to_string()),
                                    &(file_entry.modified_date),
                                    &(MAIN_ROW_COLOR.to_string()),
                                    &(TEXT_COLOR.to_string()),
                                ];
                                list_store.set(&list_store.append(), &col_indices, &values);
                            }
                        }
                        print_text_messages_to_text_view(text_messages, &text_view_errors);
                    }

                    // Set state
                    {
                        *shared_same_music_state.borrow_mut() = mf;

                        if same_music_number > 0 {
                            *shared_buttons.borrow_mut().get_mut("same_music").unwrap().get_mut("save").unwrap() = true;
                            *shared_buttons.borrow_mut().get_mut("same_music").unwrap().get_mut("delete").unwrap() = true;
                            *shared_buttons.borrow_mut().get_mut("same_music").unwrap().get_mut("select").unwrap() = true;
                        } else {
                            *shared_buttons.borrow_mut().get_mut("same_music").unwrap().get_mut("save").unwrap() = false;
                            *shared_buttons.borrow_mut().get_mut("same_music").unwrap().get_mut("delete").unwrap() = false;
                            *shared_buttons.borrow_mut().get_mut("same_music").unwrap().get_mut("select").unwrap() = false;
                        }
                        set_buttons(&mut *shared_buttons.borrow_mut().get_mut("same_music").unwrap(), &buttons_array, &buttons_names);
                    }
                }
            }
        }
        // Returning false here would close the receiver and have senders fail
        glib::Continue(true)
    });
}
