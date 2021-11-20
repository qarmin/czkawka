use humansize::{file_size_opts as options, FileSize};

use crate::gui_data::GuiData;
use crate::help_functions::*;
use crate::notebook_enums::*;
use chrono::NaiveDateTime;
use czkawka_core::duplicate::CheckingMethod;
use czkawka_core::same_music::MusicSimilarity;
use czkawka_core::similar_images;
use glib::Receiver;
use gtk::prelude::*;
use std::path::PathBuf;

pub fn connect_compute_results(gui_data: &GuiData, glib_stop_receiver: Receiver<Message>) {
    let buttons_search = gui_data.bottom_buttons.buttons_search.clone();
    let notebook_main = gui_data.main_notebook.notebook_main.clone();
    let entry_info = gui_data.entry_info.clone();
    let tree_view_empty_folder_finder = gui_data.main_notebook.tree_view_empty_folder_finder.clone();
    let tree_view_empty_files_finder = gui_data.main_notebook.tree_view_empty_files_finder.clone();
    let tree_view_duplicate_finder = gui_data.main_notebook.tree_view_duplicate_finder.clone();
    let tree_view_similar_images_finder = gui_data.main_notebook.tree_view_similar_images_finder.clone();
    let buttons_array = gui_data.bottom_buttons.buttons_array.clone();
    let text_view_errors = gui_data.text_view_errors.clone();
    let shared_duplication_state = gui_data.shared_duplication_state.clone();
    let shared_buttons = gui_data.shared_buttons.clone();
    let tree_view_zeroed_files_finder = gui_data.main_notebook.tree_view_zeroed_files_finder.clone();
    let shared_empty_folders_state = gui_data.shared_empty_folders_state.clone();
    let shared_empty_files_state = gui_data.shared_empty_files_state.clone();
    let shared_broken_files_state = gui_data.shared_broken_files_state.clone();
    let tree_view_big_files_finder = gui_data.main_notebook.tree_view_big_files_finder.clone();
    let tree_view_broken_files = gui_data.main_notebook.tree_view_broken_files.clone();
    let tree_view_invalid_symlinks = gui_data.main_notebook.tree_view_invalid_symlinks.clone();
    let shared_big_files_state = gui_data.shared_big_files_state.clone();
    let shared_same_invalid_symlinks = gui_data.shared_same_invalid_symlinks.clone();
    let tree_view_temporary_files_finder = gui_data.main_notebook.tree_view_temporary_files_finder.clone();
    let shared_temporary_files_state = gui_data.shared_temporary_files_state.clone();
    let shared_similar_images_state = gui_data.shared_similar_images_state.clone();
    let shared_zeroed_files_state = gui_data.shared_zeroed_files_state.clone();
    let tree_view_same_music_finder = gui_data.main_notebook.tree_view_same_music_finder.clone();
    let shared_same_music_state = gui_data.shared_same_music_state.clone();
    let buttons_names = gui_data.bottom_buttons.buttons_names.clone();
    let window_progress = gui_data.progress_window.window_progress.clone();
    let taskbar_state = gui_data.taskbar_state.clone();
    let radio_button_similar_hash_size_4 = gui_data.main_notebook.radio_button_similar_hash_size_4.clone();
    let radio_button_similar_hash_size_8 = gui_data.main_notebook.radio_button_similar_hash_size_8.clone();
    let radio_button_similar_hash_size_16 = gui_data.main_notebook.radio_button_similar_hash_size_16.clone();

    let main_context = glib::MainContext::default();
    let _guard = main_context.acquire().unwrap();

    glib_stop_receiver.attach(None, move |msg| {
        buttons_search.show();

        window_progress.hide();

        taskbar_state.borrow().hide();

        // Restore clickability to main notebook
        notebook_main.set_sensitive(true);

        let hash_size;
        if radio_button_similar_hash_size_4.is_active() {
            hash_size = 4;
        } else if radio_button_similar_hash_size_8.is_active() {
            hash_size = 8;
        } else if radio_button_similar_hash_size_16.is_active() {
            hash_size = 16;
        } else {
            panic!("No radio button is pressed");
        }

        match msg {
            Message::Duplicates(df) => {
                if df.get_stopped_search() {
                    entry_info.set_text("Searching for duplicates was stopped by user");
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
                        CheckingMethod::Hash | CheckingMethod::HashMb => {
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
                        let list_store = get_list_store(&tree_view_duplicate_finder);

                        match df.get_check_method() {
                            CheckingMethod::Name => {
                                let btreemap = df.get_files_sorted_by_names();

                                for (name, vector) in btreemap.iter().rev() {
                                    // Sort
                                    let vector = if vector.len() >= 2 {
                                        let mut vector = vector.clone();
                                        vector.sort_by_key(|e| {
                                            let t = split_path(e.path.as_path());
                                            (t.0, t.1)
                                        });
                                        vector
                                    } else {
                                        vector.clone()
                                    };

                                    let values: [(u32, &dyn ToValue); 8] = [
                                        (0, &false),
                                        (1, &false),
                                        (2, &name),
                                        (3, (&(format!("{} results", vector.len())))),
                                        (4, (&"".to_string())), // No text in 3 column
                                        (5, (&(0))),            // Not used here
                                        (6, &(HEADER_ROW_COLOR.to_string())),
                                        (7, &(TEXT_COLOR.to_string())),
                                    ];

                                    list_store.set(&list_store.append(), &values);
                                    for entry in vector {
                                        let (directory, file) = split_path(&entry.path);
                                        let values: [(u32, &dyn ToValue); 8] = [
                                            (0, &true),
                                            (1, &false),
                                            (2, &file),
                                            (3, &directory),
                                            (4, &(format!("{} - ({})", NaiveDateTime::from_timestamp(entry.modified_date as i64, 0).to_string(), entry.size.file_size(options::BINARY).unwrap()))),
                                            (5, &(entry.modified_date)),
                                            (6, &(MAIN_ROW_COLOR.to_string())),
                                            (7, &(TEXT_COLOR.to_string())),
                                        ];
                                        list_store.set(&list_store.append(), &values);
                                    }
                                }
                            }
                            CheckingMethod::Hash | CheckingMethod::HashMb => {
                                let btreemap = df.get_files_sorted_by_hash();

                                for (size, vectors_vector) in btreemap.iter().rev() {
                                    for vector in vectors_vector {
                                        // Sort
                                        let vector = if vector.len() >= 2 {
                                            let mut vector = vector.clone();
                                            vector.sort_by_key(|e| {
                                                let t = split_path(e.path.as_path());
                                                (t.0, t.1)
                                            });
                                            vector
                                        } else {
                                            vector.clone()
                                        };

                                        let values: [(u32, &dyn ToValue); 8] = [
                                            (0, &false),
                                            (1, &false),
                                            (2, &(format!("{} x {} ({} bytes)", vector.len(), size.file_size(options::BINARY).unwrap(), size))),
                                            (
                                                3,
                                                &(format!("{} ({} bytes) lost", ((vector.len() - 1) as u64 * *size as u64).file_size(options::BINARY).unwrap(), (vector.len() - 1) as u64 * *size as u64)),
                                            ),
                                            (4, &"".to_string()), // No text in 3 column
                                            (5, &(0)),
                                            (6, &(HEADER_ROW_COLOR.to_string())),
                                            (7, &(TEXT_COLOR.to_string())),
                                        ];

                                        list_store.set(&list_store.append(), &values);
                                        for entry in vector {
                                            let (directory, file) = split_path(&entry.path);

                                            let values: [(u32, &dyn ToValue); 8] = [
                                                (0, &true),
                                                (1, &false),
                                                (2, &file),
                                                (3, &directory),
                                                (4, &(NaiveDateTime::from_timestamp(entry.modified_date as i64, 0).to_string())),
                                                (5, &(entry.modified_date)),
                                                (6, &(MAIN_ROW_COLOR.to_string())),
                                                (7, &(TEXT_COLOR.to_string())),
                                            ];

                                            list_store.set(&list_store.append(), &values);
                                        }
                                    }
                                }
                            }
                            CheckingMethod::Size => {
                                let btreemap = df.get_files_sorted_by_size();

                                for (size, vector) in btreemap.iter().rev() {
                                    // Sort
                                    let vector = if vector.len() >= 2 {
                                        let mut vector = vector.clone();
                                        vector.sort_by_key(|e| {
                                            let t = split_path(e.path.as_path());
                                            (t.0, t.1)
                                        });
                                        vector
                                    } else {
                                        vector.clone()
                                    };
                                    let values: [(u32, &dyn ToValue); 8] = [
                                        (0, &false),
                                        (1, &false),
                                        (2, &(format!("{} x {} ({} bytes)", vector.len(), size.file_size(options::BINARY).unwrap(), size))),
                                        (
                                            3,
                                            &(format!("{} ({} bytes) lost", ((vector.len() - 1) as u64 * *size as u64).file_size(options::BINARY).unwrap(), (vector.len() - 1) as u64 * *size as u64)),
                                        ),
                                        (4, &"".to_string()), // No text in 3 column
                                        (5, &(0)),            // Not used here
                                        (6, &(HEADER_ROW_COLOR.to_string())),
                                        (7, &(TEXT_COLOR.to_string())),
                                    ];

                                    list_store.set(&list_store.append(), &values);
                                    for entry in vector {
                                        let (directory, file) = split_path(&entry.path);
                                        let values: [(u32, &dyn ToValue); 8] = [
                                            (0, &true),
                                            (1, &false),
                                            (2, &file),
                                            (3, &directory),
                                            (4, &(NaiveDateTime::from_timestamp(entry.modified_date as i64, 0).to_string())),
                                            (5, &(entry.modified_date)),
                                            (6, &(MAIN_ROW_COLOR.to_string())),
                                            (7, &(TEXT_COLOR.to_string())),
                                        ];
                                        list_store.set(&list_store.append(), &values);
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
                            *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::Duplicate).unwrap().get_mut("save").unwrap() = true;
                            *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::Duplicate).unwrap().get_mut("delete").unwrap() = true;
                            *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::Duplicate).unwrap().get_mut("select").unwrap() = true;
                            *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::Duplicate).unwrap().get_mut("symlink").unwrap() = true;
                            *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::Duplicate).unwrap().get_mut("hardlink").unwrap() = true;
                            *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::Duplicate).unwrap().get_mut("move").unwrap() = true;
                        } else {
                            *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::Duplicate).unwrap().get_mut("save").unwrap() = false;
                            *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::Duplicate).unwrap().get_mut("delete").unwrap() = false;
                            *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::Duplicate).unwrap().get_mut("select").unwrap() = false;
                            *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::Duplicate).unwrap().get_mut("symlink").unwrap() = false;
                            *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::Duplicate).unwrap().get_mut("hardlink").unwrap() = false;
                            *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::Duplicate).unwrap().get_mut("move").unwrap() = false;
                        }
                        set_buttons(&mut *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::Duplicate).unwrap(), &buttons_array, &buttons_names);
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
                        let list_store = get_list_store(&tree_view_empty_folder_finder);

                        let hashmap = ef.get_empty_folder_list();
                        let mut vector = hashmap.keys().cloned().collect::<Vec<PathBuf>>();

                        vector.sort_by_key(|e| {
                            let t = split_path(e.as_path());
                            (t.0, t.1)
                        });

                        for path in vector {
                            let (directory, file) = split_path(&path);
                            let values: [(u32, &dyn ToValue); 4] = [(0, &false), (1, &file), (2, &directory), (3, &(NaiveDateTime::from_timestamp(hashmap.get(&path).unwrap().modified_date as i64, 0).to_string()))];
                            list_store.set(&list_store.append(), &values);
                        }
                        print_text_messages_to_text_view(text_messages, &text_view_errors);
                    }

                    // Set state
                    {
                        *shared_empty_folders_state.borrow_mut() = ef;

                        if empty_folder_number > 0 {
                            *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::EmptyDirectories).unwrap().get_mut("save").unwrap() = true;
                            *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::EmptyDirectories).unwrap().get_mut("delete").unwrap() = true;
                            *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::EmptyDirectories).unwrap().get_mut("select").unwrap() = true;
                            *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::EmptyDirectories).unwrap().get_mut("move").unwrap() = true;
                        } else {
                            *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::EmptyDirectories).unwrap().get_mut("save").unwrap() = false;
                            *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::EmptyDirectories).unwrap().get_mut("delete").unwrap() = false;
                            *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::EmptyDirectories).unwrap().get_mut("select").unwrap() = false;
                            *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::EmptyDirectories).unwrap().get_mut("move").unwrap() = false;
                        }
                        set_buttons(&mut *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::EmptyDirectories).unwrap(), &buttons_array, &buttons_names);
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
                        let list_store = get_list_store(&tree_view_empty_files_finder);

                        let vector = vf.get_empty_files();

                        // Sort
                        let mut vector = vector.clone();
                        vector.sort_by_key(|e| {
                            let t = split_path(e.path.as_path());
                            (t.0, t.1)
                        });

                        for file_entry in vector {
                            let (directory, file) = split_path(&file_entry.path);
                            let values: [(u32, &dyn ToValue); 4] = [(0, &false), (1, &file), (2, &directory), (3, &(NaiveDateTime::from_timestamp(file_entry.modified_date as i64, 0).to_string()))];
                            list_store.set(&list_store.append(), &values);
                        }
                        print_text_messages_to_text_view(text_messages, &text_view_errors);
                    }

                    // Set state
                    {
                        *shared_empty_files_state.borrow_mut() = vf;

                        if empty_files_number > 0 {
                            *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::EmptyFiles).unwrap().get_mut("save").unwrap() = true;
                            *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::EmptyFiles).unwrap().get_mut("delete").unwrap() = true;
                            *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::EmptyFiles).unwrap().get_mut("select").unwrap() = true;
                            *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::EmptyFiles).unwrap().get_mut("move").unwrap() = true;
                        } else {
                            *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::EmptyFiles).unwrap().get_mut("save").unwrap() = false;
                            *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::EmptyFiles).unwrap().get_mut("delete").unwrap() = false;
                            *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::EmptyFiles).unwrap().get_mut("select").unwrap() = false;
                            *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::EmptyFiles).unwrap().get_mut("move").unwrap() = false;
                        }
                        set_buttons(&mut *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::EmptyFiles).unwrap(), &buttons_array, &buttons_names);
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
                        let list_store = get_list_store(&tree_view_big_files_finder);

                        let btreemap = bf.get_big_files();

                        for (size, vector) in btreemap.iter().rev() {
                            let mut vector = vector.clone();
                            vector.sort_by_key(|e| {
                                let t = split_path(e.path.as_path());
                                (t.0, t.1)
                            });
                            for file_entry in vector {
                                let (directory, file) = split_path(&file_entry.path);
                                let values: [(u32, &dyn ToValue); 5] = [
                                    (0, &false),
                                    (1, &(format!("{} ({} bytes)", size.file_size(options::BINARY).unwrap(), size))),
                                    (2, &file),
                                    (3, &directory),
                                    (4, &(NaiveDateTime::from_timestamp(file_entry.modified_date as i64, 0).to_string())),
                                ];
                                list_store.set(&list_store.append(), &values);
                            }
                        }
                        print_text_messages_to_text_view(text_messages, &text_view_errors);
                    }

                    // Set state
                    {
                        *shared_big_files_state.borrow_mut() = bf;

                        if biggest_files_number > 0 {
                            *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::BigFiles).unwrap().get_mut("save").unwrap() = true;
                            *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::BigFiles).unwrap().get_mut("delete").unwrap() = true;
                            *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::BigFiles).unwrap().get_mut("select").unwrap() = true;
                            *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::BigFiles).unwrap().get_mut("move").unwrap() = true;
                        } else {
                            *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::BigFiles).unwrap().get_mut("save").unwrap() = false;
                            *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::BigFiles).unwrap().get_mut("delete").unwrap() = false;
                            *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::BigFiles).unwrap().get_mut("select").unwrap() = false;
                            *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::BigFiles).unwrap().get_mut("move").unwrap() = false;
                        }
                        set_buttons(&mut *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::BigFiles).unwrap(), &buttons_array, &buttons_names);
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
                        let list_store = get_list_store(&tree_view_temporary_files_finder);

                        let vector = tf.get_temporary_files();

                        // Sort
                        let mut vector = vector.clone();
                        vector.sort_by_key(|e| {
                            let t = split_path(e.path.as_path());
                            (t.0, t.1)
                        });

                        for file_entry in vector {
                            let (directory, file) = split_path(&file_entry.path);
                            let values: [(u32, &dyn ToValue); 4] = [(0, &false), (1, &file), (2, &directory), (3, &(NaiveDateTime::from_timestamp(file_entry.modified_date as i64, 0).to_string()))];
                            list_store.set(&list_store.append(), &values);
                        }
                        print_text_messages_to_text_view(text_messages, &text_view_errors);
                    }

                    // Set state
                    {
                        *shared_temporary_files_state.borrow_mut() = tf;

                        if temporary_files_number > 0 {
                            *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::Temporary).unwrap().get_mut("save").unwrap() = true;
                            *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::Temporary).unwrap().get_mut("delete").unwrap() = true;
                            *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::Temporary).unwrap().get_mut("select").unwrap() = true;
                            *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::Temporary).unwrap().get_mut("move").unwrap() = true;
                        } else {
                            *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::Temporary).unwrap().get_mut("save").unwrap() = false;
                            *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::Temporary).unwrap().get_mut("delete").unwrap() = false;
                            *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::Temporary).unwrap().get_mut("select").unwrap() = false;
                            *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::Temporary).unwrap().get_mut("move").unwrap() = false;
                        }
                        set_buttons(&mut *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::Temporary).unwrap(), &buttons_array, &buttons_names);
                    }
                }
            }
            Message::SimilarImages(sf) => {
                if sf.get_stopped_search() {
                    entry_info.set_text("Searching for similar images was stopped by user");
                } else {
                    //let information = sf.get_information();
                    let text_messages = sf.get_text_messages();

                    let base_images_size = sf.get_similar_images().len();

                    entry_info.set_text(format!("Found similar pictures for {} images.", base_images_size).as_str());

                    // Create GUI
                    {
                        let list_store = get_list_store(&tree_view_similar_images_finder);

                        let vec_struct_similar = sf.get_similar_images();

                        for vec_file_entry in vec_struct_similar.iter() {
                            // Sort
                            let vec_file_entry = if vec_file_entry.len() >= 2 {
                                let mut vec_file_entry = vec_file_entry.clone();
                                vec_file_entry.sort_by_key(|e| {
                                    let t = split_path(e.path.as_path());
                                    (t.0, t.1)
                                });
                                vec_file_entry
                            } else {
                                vec_file_entry.clone()
                            };

                            // Header
                            let values: [(u32, &dyn ToValue); 12] = [
                                (0, &false),
                                (1, &false),
                                (2, &"".to_string()),
                                (3, &"".to_string()),
                                (4, &(0)),
                                (5, &"".to_string()),
                                (6, &"".to_string()),
                                (7, &"".to_string()),
                                (8, &"".to_string()),
                                (9, &(0)),
                                (10, &(HEADER_ROW_COLOR.to_string())),
                                (11, &(TEXT_COLOR.to_string())),
                            ];
                            list_store.set(&list_store.append(), &values);

                            // Meat
                            for file_entry in vec_file_entry.iter() {
                                let (directory, file) = split_path(&file_entry.path);
                                let values: [(u32, &dyn ToValue); 12] = [
                                    (0, &true),
                                    (1, &false),
                                    (2, &(similar_images::get_string_from_similarity(&file_entry.similarity, hash_size).to_string())),
                                    (3, &file_entry.size.file_size(options::BINARY).unwrap()),
                                    (4, &file_entry.size),
                                    (5, &file_entry.dimensions),
                                    (6, &file),
                                    (7, &directory),
                                    (8, &(NaiveDateTime::from_timestamp(file_entry.modified_date as i64, 0).to_string())),
                                    (9, &(file_entry.modified_date)),
                                    (10, &(MAIN_ROW_COLOR.to_string())),
                                    (11, &(TEXT_COLOR.to_string())),
                                ];
                                list_store.set(&list_store.append(), &values);
                            }
                        }

                        print_text_messages_to_text_view(text_messages, &text_view_errors);
                    }

                    // Set state
                    {
                        *shared_similar_images_state.borrow_mut() = sf;

                        if base_images_size > 0 {
                            *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::SimilarImages).unwrap().get_mut("save").unwrap() = true;
                            *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::SimilarImages).unwrap().get_mut("delete").unwrap() = true;
                            *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::SimilarImages).unwrap().get_mut("select").unwrap() = true;
                            *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::SimilarImages).unwrap().get_mut("symlink").unwrap() = true;
                            *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::SimilarImages).unwrap().get_mut("hardlink").unwrap() = true;
                            *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::SimilarImages).unwrap().get_mut("move").unwrap() = true;
                        } else {
                            *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::SimilarImages).unwrap().get_mut("save").unwrap() = false;
                            *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::SimilarImages).unwrap().get_mut("delete").unwrap() = false;
                            *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::SimilarImages).unwrap().get_mut("select").unwrap() = false;
                            *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::SimilarImages).unwrap().get_mut("symlink").unwrap() = false;
                            *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::SimilarImages).unwrap().get_mut("hardlink").unwrap() = false;
                            *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::SimilarImages).unwrap().get_mut("move").unwrap() = false;
                        }
                        set_buttons(&mut *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::SimilarImages).unwrap(), &buttons_array, &buttons_names);
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
                        let list_store = get_list_store(&tree_view_zeroed_files_finder);

                        let vector = zf.get_zeroed_files();

                        // Sort
                        let mut vector = vector.clone();
                        vector.sort_by_key(|e| {
                            let t = split_path(e.path.as_path());
                            (t.0, t.1)
                        });

                        for file_entry in vector {
                            let (directory, file) = split_path(&file_entry.path);
                            let values: [(u32, &dyn ToValue); 6] = [
                                (0, &false),
                                (1, &(file_entry.size.file_size(options::BINARY).unwrap())),
                                (2, &(file_entry.size)),
                                (3, &file),
                                (4, &directory),
                                (5, &(NaiveDateTime::from_timestamp(file_entry.modified_date as i64, 0).to_string())),
                            ];
                            list_store.set(&list_store.append(), &values);
                        }
                        print_text_messages_to_text_view(text_messages, &text_view_errors);
                    }

                    // Set state
                    {
                        *shared_zeroed_files_state.borrow_mut() = zf;

                        if zeroed_files_number > 0 {
                            *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::Zeroed).unwrap().get_mut("save").unwrap() = true;
                            *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::Zeroed).unwrap().get_mut("delete").unwrap() = true;
                            *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::Zeroed).unwrap().get_mut("select").unwrap() = true;
                            *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::Zeroed).unwrap().get_mut("move").unwrap() = true;
                        } else {
                            *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::Zeroed).unwrap().get_mut("save").unwrap() = false;
                            *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::Zeroed).unwrap().get_mut("delete").unwrap() = false;
                            *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::Zeroed).unwrap().get_mut("select").unwrap() = false;
                            *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::Zeroed).unwrap().get_mut("move").unwrap() = false;
                        }
                        set_buttons(&mut *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::Zeroed).unwrap(), &buttons_array, &buttons_names);
                    }
                }
            }
            Message::SameMusic(mf) => {
                if mf.get_stopped_search() {
                    entry_info.set_text("Searching for same music was stopped by user");
                } else {
                    let information = mf.get_information();
                    let text_messages = mf.get_text_messages();

                    let same_music_number: usize = information.number_of_duplicates_music_files;

                    entry_info.set_text(format!("Found {} duplicated music files.", same_music_number).as_str());

                    // Create GUI
                    {
                        let list_store = get_list_store(&tree_view_same_music_finder);

                        let vector = mf.get_duplicated_music_entries();

                        let music_similarity = *mf.get_music_similarity();

                        let is_title = (MusicSimilarity::TITLE & music_similarity) != MusicSimilarity::NONE;
                        let is_artist = (MusicSimilarity::ARTIST & music_similarity) != MusicSimilarity::NONE;
                        let is_album_title = (MusicSimilarity::ALBUM_TITLE & music_similarity) != MusicSimilarity::NONE;
                        let is_album_artist = (MusicSimilarity::ALBUM_ARTIST & music_similarity) != MusicSimilarity::NONE;
                        let is_year = (MusicSimilarity::YEAR & music_similarity) != MusicSimilarity::NONE;

                        let text: String = "-----".to_string();

                        for vec_file_entry in vector {
                            // Sort
                            let vec_file_entry = if vec_file_entry.len() >= 2 {
                                let mut vec_file_entry = vec_file_entry.clone();
                                vec_file_entry.sort_by_key(|e| {
                                    let t = split_path(e.path.as_path());
                                    (t.0, t.1)
                                });
                                vec_file_entry
                            } else {
                                vec_file_entry.clone()
                            };

                            let values: [(u32, &dyn ToValue); 15] = [
                                (0, &false),
                                (1, &false),
                                (2, &"".to_string()),
                                (3, &(0)),
                                (4, &"".to_string()),
                                (5, &"".to_string()),
                                (
                                    6,
                                    &(match is_title {
                                        true => text.clone(),
                                        false => "".to_string(),
                                    }),
                                ),
                                (
                                    7,
                                    &(match is_artist {
                                        true => text.clone(),
                                        false => "".to_string(),
                                    }),
                                ),
                                (
                                    8,
                                    &(match is_album_title {
                                        true => text.clone(),
                                        false => "".to_string(),
                                    }),
                                ),
                                (
                                    9,
                                    &(match is_album_artist {
                                        true => text.clone(),
                                        false => "".to_string(),
                                    }),
                                ),
                                (
                                    10,
                                    &(match is_year {
                                        true => text.clone(),
                                        false => "".to_string(),
                                    }),
                                ),
                                (11, &"".to_string()),
                                (12, &(0)),
                                (13, &(HEADER_ROW_COLOR.to_string())),
                                (14, &(TEXT_COLOR.to_string())),
                            ];
                            list_store.set(&list_store.append(), &values);
                            for file_entry in vec_file_entry {
                                let (directory, file) = split_path(&file_entry.path);
                                let values: [(u32, &dyn ToValue); 15] = [
                                    (0, &true),
                                    (1, &false),
                                    (2, &file_entry.size.file_size(options::BINARY).unwrap()),
                                    (3, &file_entry.size),
                                    (4, &file),
                                    (5, &directory),
                                    (6, &file_entry.title),
                                    (7, &file_entry.artist),
                                    (8, &file_entry.album_title),
                                    (9, &file_entry.album_artist),
                                    (10, &file_entry.year.to_string()),
                                    (11, &(NaiveDateTime::from_timestamp(file_entry.modified_date as i64, 0).to_string())),
                                    (12, &(file_entry.modified_date)),
                                    (13, &(MAIN_ROW_COLOR.to_string())),
                                    (14, &(TEXT_COLOR.to_string())),
                                ];
                                list_store.set(&list_store.append(), &values);
                            }
                        }
                        print_text_messages_to_text_view(text_messages, &text_view_errors);
                    }

                    // Set state
                    {
                        *shared_same_music_state.borrow_mut() = mf;

                        if same_music_number > 0 {
                            *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::SameMusic).unwrap().get_mut("save").unwrap() = true;
                            *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::SameMusic).unwrap().get_mut("delete").unwrap() = true;
                            *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::SameMusic).unwrap().get_mut("select").unwrap() = true;
                            *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::SameMusic).unwrap().get_mut("symlink").unwrap() = true;
                            *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::SameMusic).unwrap().get_mut("hardlink").unwrap() = true;
                            *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::SameMusic).unwrap().get_mut("move").unwrap() = true;
                        } else {
                            *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::SameMusic).unwrap().get_mut("save").unwrap() = false;
                            *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::SameMusic).unwrap().get_mut("delete").unwrap() = false;
                            *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::SameMusic).unwrap().get_mut("select").unwrap() = false;
                            *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::SameMusic).unwrap().get_mut("symlink").unwrap() = false;
                            *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::SameMusic).unwrap().get_mut("hardlink").unwrap() = false;
                            *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::SameMusic).unwrap().get_mut("move").unwrap() = false;
                        }
                        set_buttons(&mut *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::SameMusic).unwrap(), &buttons_array, &buttons_names);
                    }
                }
            }
            Message::InvalidSymlinks(ifs) => {
                if ifs.get_stopped_search() {
                    entry_info.set_text("Searching for invalid symlink was stopped by user");
                } else {
                    let information = ifs.get_information();
                    let text_messages = ifs.get_text_messages();

                    let invalid_symlinks: usize = information.number_of_invalid_symlinks;

                    entry_info.set_text(format!("Found {} invalid symlinks.", invalid_symlinks).as_str());

                    // Create GUI
                    {
                        let list_store = get_list_store(&tree_view_invalid_symlinks);

                        let vector = ifs.get_invalid_symlinks();

                        // Sort
                        let mut vector = vector.clone();

                        vector.sort_by_key(|e| {
                            let t = split_path(e.symlink_path.as_path());
                            (t.0, t.1)
                        });

                        for file_entry in vector {
                            let (directory, file) = split_path(&file_entry.symlink_path);
                            let values: [(u32, &dyn ToValue); 6] = [
                                (0, &false),
                                (1, &file),
                                (2, &directory),
                                (3, &file_entry.destination_path.to_string_lossy().to_string()),
                                (4, &get_text_from_invalid_symlink_cause(&file_entry.type_of_error)),
                                (5, &(NaiveDateTime::from_timestamp(file_entry.modified_date as i64, 0).to_string())),
                            ];
                            list_store.set(&list_store.append(), &values);
                        }
                        print_text_messages_to_text_view(text_messages, &text_view_errors);
                    }

                    // Set state
                    {
                        *shared_same_invalid_symlinks.borrow_mut() = ifs;

                        if invalid_symlinks > 0 {
                            *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::Symlinks).unwrap().get_mut("save").unwrap() = true;
                            *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::Symlinks).unwrap().get_mut("delete").unwrap() = true;
                            *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::Symlinks).unwrap().get_mut("select").unwrap() = true;
                            *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::Symlinks).unwrap().get_mut("move").unwrap() = true;
                        } else {
                            *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::Symlinks).unwrap().get_mut("save").unwrap() = false;
                            *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::Symlinks).unwrap().get_mut("delete").unwrap() = false;
                            *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::Symlinks).unwrap().get_mut("select").unwrap() = false;
                            *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::Symlinks).unwrap().get_mut("move").unwrap() = false;
                        }
                        set_buttons(&mut *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::Symlinks).unwrap(), &buttons_array, &buttons_names);
                    }
                }
            }
            Message::BrokenFiles(br) => {
                if br.get_stopped_search() {
                    entry_info.set_text("Searching for broken files was stopped by user");
                } else {
                    let information = br.get_information();
                    let text_messages = br.get_text_messages();

                    let broken_files_number: usize = information.number_of_broken_files;

                    entry_info.set_text(format!("Found {} broken files.", broken_files_number).as_str());

                    // Create GUI
                    {
                        let list_store = get_list_store(&tree_view_broken_files);

                        let vector = br.get_broken_files();

                        // Sort
                        let mut vector = vector.clone();
                        vector.sort_by_key(|e| {
                            let t = split_path(e.path.as_path());
                            (t.0, t.1)
                        });

                        for file_entry in vector {
                            let (directory, file) = split_path(&file_entry.path);
                            let values: [(u32, &dyn ToValue); 5] = [
                                (0, &false),
                                (1, &file),
                                (2, &directory),
                                (3, &file_entry.error_string),
                                (4, &(NaiveDateTime::from_timestamp(file_entry.modified_date as i64, 0).to_string())),
                            ];
                            list_store.set(&list_store.append(), &values);
                        }
                        print_text_messages_to_text_view(text_messages, &text_view_errors);
                    }

                    // Set state
                    {
                        *shared_broken_files_state.borrow_mut() = br;

                        if broken_files_number > 0 {
                            *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::BrokenFiles).unwrap().get_mut("save").unwrap() = true;
                            *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::BrokenFiles).unwrap().get_mut("delete").unwrap() = true;
                            *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::BrokenFiles).unwrap().get_mut("select").unwrap() = true;
                            *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::BrokenFiles).unwrap().get_mut("move").unwrap() = true;
                        } else {
                            *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::BrokenFiles).unwrap().get_mut("save").unwrap() = false;
                            *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::BrokenFiles).unwrap().get_mut("delete").unwrap() = false;
                            *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::BrokenFiles).unwrap().get_mut("select").unwrap() = false;
                            *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::BrokenFiles).unwrap().get_mut("move").unwrap() = false;
                        }
                        set_buttons(&mut *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::BrokenFiles).unwrap(), &buttons_array, &buttons_names);
                    }
                }
            }
        }
        // Returning false here would close the receiver and have senders fail
        glib::Continue(true)
    });
}
