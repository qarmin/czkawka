use std::cell::RefCell;
use std::collections::HashMap;
use std::path::PathBuf;
use std::rc::Rc;

use chrono::NaiveDateTime;
use glib::Receiver;
use gtk::prelude::*;
use humansize::{file_size_opts as options, FileSize};

use crate::fl;
use czkawka_core::duplicate::CheckingMethod;
use czkawka_core::same_music::MusicSimilarity;
use czkawka_core::similar_images;

use crate::gui_data::GuiData;
use crate::help_functions::*;
use crate::notebook_enums::*;

pub fn connect_compute_results(gui_data: &GuiData, glib_stop_receiver: Receiver<Message>) {
    let buttons_search = gui_data.bottom_buttons.buttons_search.clone();
    let notebook_main = gui_data.main_notebook.notebook_main.clone();
    let entry_info = gui_data.entry_info.clone();
    let tree_view_empty_folder_finder = gui_data.main_notebook.tree_view_empty_folder_finder.clone();
    let tree_view_empty_files_finder = gui_data.main_notebook.tree_view_empty_files_finder.clone();
    let tree_view_duplicate_finder = gui_data.main_notebook.tree_view_duplicate_finder.clone();
    let tree_view_similar_images_finder = gui_data.main_notebook.tree_view_similar_images_finder.clone();
    let tree_view_similar_videos_finder = gui_data.main_notebook.tree_view_similar_videos_finder.clone();
    let buttons_array = gui_data.bottom_buttons.buttons_array.clone();
    let text_view_errors = gui_data.text_view_errors.clone();
    let shared_duplication_state = gui_data.shared_duplication_state.clone();
    let shared_buttons = gui_data.shared_buttons.clone();
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
    let shared_similar_videos_state = gui_data.shared_similar_videos_state.clone();
    let tree_view_same_music_finder = gui_data.main_notebook.tree_view_same_music_finder.clone();
    let shared_same_music_state = gui_data.shared_same_music_state.clone();
    let buttons_names = gui_data.bottom_buttons.buttons_names.clone();
    let window_progress = gui_data.progress_window.window_progress.clone();
    let taskbar_state = gui_data.taskbar_state.clone();
    let radio_button_similar_hash_size_8 = gui_data.main_notebook.radio_button_similar_hash_size_8.clone();
    let radio_button_similar_hash_size_16 = gui_data.main_notebook.radio_button_similar_hash_size_16.clone();
    let radio_button_similar_hash_size_32 = gui_data.main_notebook.radio_button_similar_hash_size_32.clone();
    let radio_button_similar_hash_size_64 = gui_data.main_notebook.radio_button_similar_hash_size_64.clone();
    let notebook_upper = gui_data.upper_notebook.notebook_upper.clone();
    let button_settings = gui_data.header.button_settings.clone();
    let button_app_info = gui_data.header.button_app_info.clone();

    let main_context = glib::MainContext::default();
    let _guard = main_context.acquire().unwrap();

    glib_stop_receiver.attach(None, move |msg| {
        buttons_search.show();

        notebook_main.set_sensitive(true);
        notebook_upper.set_sensitive(true);
        button_settings.set_sensitive(true);
        button_app_info.set_sensitive(true);

        window_progress.hide();

        taskbar_state.borrow().hide();

        let hash_size;
        if radio_button_similar_hash_size_8.is_active() {
            hash_size = 8;
        } else if radio_button_similar_hash_size_16.is_active() {
            hash_size = 16;
        } else if radio_button_similar_hash_size_32.is_active() {
            hash_size = 32;
        } else if radio_button_similar_hash_size_64.is_active() {
            hash_size = 64;
        } else {
            panic!("No radio button is pressed");
        }

        match msg {
            Message::Duplicates(df) => {
                if df.get_stopped_search() {
                    entry_info.set_text(&fl!("compute_stopped_by_user"));
                } else {
                    let information = df.get_information();
                    let text_messages = df.get_text_messages();

                    let duplicates_number: usize;
                    let duplicates_size: u64;
                    let duplicates_group: usize;

                    let fl_found = fl!("compute_found");
                    let fl_groups = fl!("compute_groups");
                    let fl_groups_which_took = fl!("compute_groups_which_took");
                    let fl_duplicated_files_in = fl!("compute_duplicated_files_in");

                    match df.get_check_method() {
                        CheckingMethod::Name => {
                            duplicates_number = information.number_of_duplicated_files_by_name;
                            // duplicates_size = 0;
                            duplicates_group = information.number_of_groups_by_name;
                            entry_info.set_text(format!("{} {} {} {} {}", fl_found, duplicates_number, fl_duplicated_files_in, duplicates_group, fl_groups).as_str());
                        }
                        CheckingMethod::Hash => {
                            duplicates_number = information.number_of_duplicated_files_by_hash;
                            duplicates_size = information.lost_space_by_hash;
                            duplicates_group = information.number_of_groups_by_hash;
                            entry_info.set_text(
                                format!(
                                    "{} {} {} {} {} {}.",
                                    fl_found,
                                    duplicates_number,
                                    fl_duplicated_files_in,
                                    duplicates_group,
                                    fl_groups_which_took,
                                    duplicates_size.file_size(options::BINARY).unwrap()
                                )
                                .as_str(),
                            );
                        }
                        CheckingMethod::Size => {
                            duplicates_number = information.number_of_duplicated_files_by_size;
                            duplicates_size = information.lost_space_by_size;
                            duplicates_group = information.number_of_groups_by_size;
                            entry_info.set_text(
                                format!(
                                    "{} {} {} {} {} {}.",
                                    fl_found,
                                    duplicates_number,
                                    fl_duplicated_files_in,
                                    duplicates_group,
                                    fl_groups_which_took,
                                    duplicates_size.file_size(options::BINARY).unwrap()
                                )
                                .as_str(),
                            );
                        }
                        CheckingMethod::None => {
                            panic!();
                        }
                    }

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
                                        (ColumnsDuplicates::ActivatableSelectButton as u32, &false),
                                        (ColumnsDuplicates::SelectionButton as u32, &false),
                                        (ColumnsDuplicates::Name as u32, &name),
                                        (ColumnsDuplicates::Path as u32, (&(format!("{} results", vector.len())))),
                                        (ColumnsDuplicates::Modification as u32, (&"".to_string())), // No text in 3 column
                                        (ColumnsDuplicates::ModificationAsSecs as u32, (&(0))),      // Not used here
                                        (ColumnsDuplicates::Color as u32, &(HEADER_ROW_COLOR.to_string())),
                                        (ColumnsDuplicates::TextColor as u32, &(TEXT_COLOR.to_string())),
                                    ];

                                    list_store.set(&list_store.append(), &values);
                                    for entry in vector {
                                        let (directory, file) = split_path(&entry.path);
                                        let values: [(u32, &dyn ToValue); 8] = [
                                            (ColumnsDuplicates::ActivatableSelectButton as u32, &true),
                                            (ColumnsDuplicates::SelectionButton as u32, &false),
                                            (ColumnsDuplicates::Name as u32, &file),
                                            (ColumnsDuplicates::Path as u32, &directory),
                                            (
                                                ColumnsDuplicates::Modification as u32,
                                                &(format!("{} - ({})", NaiveDateTime::from_timestamp(entry.modified_date as i64, 0), entry.size.file_size(options::BINARY).unwrap())),
                                            ),
                                            (ColumnsDuplicates::ModificationAsSecs as u32, &(entry.modified_date)),
                                            (ColumnsDuplicates::Color as u32, &(MAIN_ROW_COLOR.to_string())),
                                            (ColumnsDuplicates::TextColor as u32, &(TEXT_COLOR.to_string())),
                                        ];
                                        list_store.set(&list_store.append(), &values);
                                    }
                                }
                            }
                            CheckingMethod::Hash => {
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
                                            (ColumnsDuplicates::ActivatableSelectButton as u32, &false),
                                            (ColumnsDuplicates::SelectionButton as u32, &false),
                                            (ColumnsDuplicates::Name as u32, &(format!("{} x {} ({} {})", vector.len(), size.file_size(options::BINARY).unwrap(), size, fl!("general_bytes")))),
                                            (
                                                ColumnsDuplicates::Path as u32,
                                                &(format!(
                                                    "{} ({} {}) {}",
                                                    ((vector.len() - 1) as u64 * *size as u64).file_size(options::BINARY).unwrap(),
                                                    (vector.len() - 1) as u64 * *size as u64,
                                                    fl!("general_bytes"),
                                                    fl!("general_lost")
                                                )),
                                            ),
                                            (ColumnsDuplicates::Modification as u32, &"".to_string()), // No text in 3 column
                                            (ColumnsDuplicates::ModificationAsSecs as u32, &(0)),
                                            (ColumnsDuplicates::Color as u32, &(HEADER_ROW_COLOR.to_string())),
                                            (ColumnsDuplicates::TextColor as u32, &(TEXT_COLOR.to_string())),
                                        ];

                                        list_store.set(&list_store.append(), &values);
                                        for entry in vector {
                                            let (directory, file) = split_path(&entry.path);

                                            let values: [(u32, &dyn ToValue); 8] = [
                                                (ColumnsDuplicates::ActivatableSelectButton as u32, &true),
                                                (ColumnsDuplicates::SelectionButton as u32, &false),
                                                (ColumnsDuplicates::Name as u32, &file),
                                                (ColumnsDuplicates::Path as u32, &directory),
                                                (ColumnsDuplicates::Modification as u32, &(NaiveDateTime::from_timestamp(entry.modified_date as i64, 0).to_string())),
                                                (ColumnsDuplicates::ModificationAsSecs as u32, &(entry.modified_date)),
                                                (ColumnsDuplicates::Color as u32, &(MAIN_ROW_COLOR.to_string())),
                                                (ColumnsDuplicates::TextColor as u32, &(TEXT_COLOR.to_string())),
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
                                        (ColumnsDuplicates::ActivatableSelectButton as u32, &false),
                                        (ColumnsDuplicates::SelectionButton as u32, &false),
                                        (ColumnsDuplicates::Name as u32, &(format!("{} x {} ({} {})", vector.len(), size.file_size(options::BINARY).unwrap(), size, fl!("general_bytes")))),
                                        (
                                            ColumnsDuplicates::Path as u32,
                                            &(format!(
                                                "{} ({} {}) {}",
                                                ((vector.len() - 1) as u64 * *size as u64).file_size(options::BINARY).unwrap(),
                                                (vector.len() - 1) as u64 * *size as u64,
                                                fl!("general_bytes"),
                                                fl!("general_lost")
                                            )),
                                        ),
                                        (ColumnsDuplicates::Modification as u32, &"".to_string()), // No text in 3 column
                                        (ColumnsDuplicates::ModificationAsSecs as u32, &(0)),      // Not used here
                                        (ColumnsDuplicates::Color as u32, &(HEADER_ROW_COLOR.to_string())),
                                        (ColumnsDuplicates::TextColor as u32, &(TEXT_COLOR.to_string())),
                                    ];

                                    list_store.set(&list_store.append(), &values);
                                    for entry in vector {
                                        let (directory, file) = split_path(&entry.path);
                                        let values: [(u32, &dyn ToValue); 8] = [
                                            (ColumnsDuplicates::ActivatableSelectButton as u32, &true),
                                            (ColumnsDuplicates::SelectionButton as u32, &false),
                                            (ColumnsDuplicates::Name as u32, &file),
                                            (ColumnsDuplicates::Path as u32, &directory),
                                            (ColumnsDuplicates::Modification as u32, &(NaiveDateTime::from_timestamp(entry.modified_date as i64, 0).to_string())),
                                            (ColumnsDuplicates::ModificationAsSecs as u32, &(entry.modified_date)),
                                            (ColumnsDuplicates::Color as u32, &(MAIN_ROW_COLOR.to_string())),
                                            (ColumnsDuplicates::TextColor as u32, &(TEXT_COLOR.to_string())),
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

                        set_specific_buttons_as_active(&shared_buttons, &NotebookMainEnum::Duplicate, &["save", "delete", "select", "symlink", "hardlink", "move"], duplicates_number > 0);

                        set_buttons(&mut *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::Duplicate).unwrap(), &buttons_array, &buttons_names);
                    }
                }
            }
            Message::EmptyFolders(ef) => {
                if ef.get_stopped_search() {
                    entry_info.set_text(&fl!("compute_stopped_by_user"));
                } else {
                    let information = ef.get_information();
                    let text_messages = ef.get_text_messages();

                    let empty_folder_number: usize = information.number_of_empty_folders;

                    entry_info.set_text(format!("{} {} {}.", fl!("compute_found"), empty_folder_number, fl!("compute_empty_folders")).as_str());

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
                            let values: [(u32, &dyn ToValue); 5] = [
                                (ColumnsEmptyFolders::SelectionButton as u32, &false),
                                (ColumnsEmptyFolders::Name as u32, &file),
                                (ColumnsEmptyFolders::Path as u32, &directory),
                                (ColumnsEmptyFolders::Modification as u32, &(NaiveDateTime::from_timestamp(hashmap.get(&path).unwrap().modified_date as i64, 0).to_string())),
                                (ColumnsEmptyFolders::ModificationAsSecs as u32, &(hashmap.get(&path).unwrap().modified_date as u64)),
                            ];
                            list_store.set(&list_store.append(), &values);
                        }
                        print_text_messages_to_text_view(text_messages, &text_view_errors);
                    }

                    // Set state
                    {
                        *shared_empty_folders_state.borrow_mut() = ef;

                        set_specific_buttons_as_active(&shared_buttons, &NotebookMainEnum::EmptyDirectories, &["save", "delete", "select", "move"], empty_folder_number > 0);

                        set_buttons(&mut *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::EmptyDirectories).unwrap(), &buttons_array, &buttons_names);
                    }
                }
            }
            Message::EmptyFiles(vf) => {
                if vf.get_stopped_search() {
                    entry_info.set_text(&fl!("compute_stopped_by_user"));
                } else {
                    let information = vf.get_information();
                    let text_messages = vf.get_text_messages();

                    let empty_files_number: usize = information.number_of_empty_files;

                    entry_info.set_text(format!("{} {} {}.", fl!("compute_found"), empty_files_number, fl!("compute_empty_files")).as_str());

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
                            let values: [(u32, &dyn ToValue); 5] = [
                                (ColumnsEmptyFiles::SelectionButton as u32, &false),
                                (ColumnsEmptyFiles::Name as u32, &file),
                                (ColumnsEmptyFiles::Path as u32, &directory),
                                (ColumnsEmptyFiles::Modification as u32, &(NaiveDateTime::from_timestamp(file_entry.modified_date as i64, 0).to_string())),
                                (ColumnsEmptyFiles::ModificationAsSecs as u32, &(file_entry.modified_date as i64)),
                            ];
                            list_store.set(&list_store.append(), &values);
                        }
                        print_text_messages_to_text_view(text_messages, &text_view_errors);
                    }

                    // Set state
                    {
                        *shared_empty_files_state.borrow_mut() = vf;

                        set_specific_buttons_as_active(&shared_buttons, &NotebookMainEnum::EmptyFiles, &["save", "delete", "select", "move"], empty_files_number > 0);

                        set_buttons(&mut *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::EmptyFiles).unwrap(), &buttons_array, &buttons_names);
                    }
                }
            }
            Message::BigFiles(bf) => {
                if bf.get_stopped_search() {
                    entry_info.set_text(&fl!("compute_stopped_by_user"));
                } else {
                    let information = bf.get_information();
                    let text_messages = bf.get_text_messages();

                    let biggest_files_number: usize = information.number_of_real_files;

                    entry_info.set_text(format!("{} {} {}.", fl!("compute_found"), biggest_files_number, fl!("compute_biggest_files")).as_str());

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
                                let values: [(u32, &dyn ToValue); 7] = [
                                    (ColumnsBigFiles::SelectionButton as u32, &false),
                                    (ColumnsBigFiles::Size as u32, &(format!("{} ({} {})", size.file_size(options::BINARY).unwrap(), size, fl!("general_bytes")))),
                                    (ColumnsBigFiles::Name as u32, &file),
                                    (ColumnsBigFiles::Path as u32, &directory),
                                    (ColumnsBigFiles::Modification as u32, &(NaiveDateTime::from_timestamp(file_entry.modified_date as i64, 0).to_string())),
                                    (ColumnsBigFiles::ModificationAsSecs as u32, &(file_entry.modified_date as i64)),
                                    (ColumnsBigFiles::SizeAsBytes as u32, &(size)),
                                ];
                                list_store.set(&list_store.append(), &values);
                            }
                        }
                        print_text_messages_to_text_view(text_messages, &text_view_errors);
                    }

                    // Set state
                    {
                        *shared_big_files_state.borrow_mut() = bf;

                        set_specific_buttons_as_active(&shared_buttons, &NotebookMainEnum::BigFiles, &["save", "delete", "select", "move"], biggest_files_number > 0);

                        set_buttons(&mut *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::BigFiles).unwrap(), &buttons_array, &buttons_names);
                    }
                }
            }
            Message::Temporary(tf) => {
                if tf.get_stopped_search() {
                    entry_info.set_text(&fl!("compute_stopped_by_user"));
                } else {
                    let information = tf.get_information();
                    let text_messages = tf.get_text_messages();

                    let temporary_files_number: usize = information.number_of_temporary_files;
                    entry_info.set_text(format!("{} {} {}.", fl!("compute_found"), temporary_files_number, fl!("compute_temporary_files")).as_str());

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
                            let values: [(u32, &dyn ToValue); 5] = [
                                (ColumnsTemporaryFiles::SelectionButton as u32, &false),
                                (ColumnsTemporaryFiles::Name as u32, &file),
                                (ColumnsTemporaryFiles::Path as u32, &directory),
                                (ColumnsTemporaryFiles::Modification as u32, &(NaiveDateTime::from_timestamp(file_entry.modified_date as i64, 0).to_string())),
                                (ColumnsTemporaryFiles::ModificationAsSecs as u32, &(file_entry.modified_date as i64)),
                            ];
                            list_store.set(&list_store.append(), &values);
                        }
                        print_text_messages_to_text_view(text_messages, &text_view_errors);
                    }

                    // Set state
                    {
                        *shared_temporary_files_state.borrow_mut() = tf;

                        set_specific_buttons_as_active(&shared_buttons, &NotebookMainEnum::Temporary, &["save", "delete", "select", "move"], temporary_files_number > 0);

                        set_buttons(&mut *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::Temporary).unwrap(), &buttons_array, &buttons_names);
                    }
                }
            }
            Message::SimilarImages(sf) => {
                if sf.get_stopped_search() {
                    entry_info.set_text(&fl!("compute_stopped_by_user"));
                } else {
                    //let information = sf.get_information();
                    let text_messages = sf.get_text_messages();

                    let base_images_size = sf.get_similar_images().len();

                    entry_info.set_text(format!("{} {} {} {}.", fl!("compute_found"), fl!("compute_duplicates_for"), base_images_size, fl!("compute_similar_image")).as_str());

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
                                (ColumnsSimilarImages::ActivatableSelectButton as u32, &false),
                                (ColumnsSimilarImages::SelectionButton as u32, &false),
                                (ColumnsSimilarImages::Similarity as u32, &"".to_string()),
                                (ColumnsSimilarImages::Size as u32, &"".to_string()),
                                (ColumnsSimilarImages::SizeAsBytes as u32, &(0)),
                                (ColumnsSimilarImages::Dimensions as u32, &"".to_string()),
                                (ColumnsSimilarImages::Name as u32, &"".to_string()),
                                (ColumnsSimilarImages::Path as u32, &"".to_string()),
                                (ColumnsSimilarImages::Modification as u32, &"".to_string()),
                                (ColumnsSimilarImages::ModificationAsSecs as u32, &(0)),
                                (ColumnsSimilarImages::Color as u32, &(HEADER_ROW_COLOR.to_string())),
                                (ColumnsSimilarImages::TextColor as u32, &(TEXT_COLOR.to_string())),
                            ];
                            list_store.set(&list_store.append(), &values);

                            // Meat
                            for file_entry in vec_file_entry.iter() {
                                let (directory, file) = split_path(&file_entry.path);
                                let values: [(u32, &dyn ToValue); 12] = [
                                    (ColumnsSimilarImages::ActivatableSelectButton as u32, &true),
                                    (ColumnsSimilarImages::SelectionButton as u32, &false),
                                    (ColumnsSimilarImages::Similarity as u32, &(similar_images::get_string_from_similarity(&file_entry.similarity, hash_size).to_string())),
                                    (ColumnsSimilarImages::Size as u32, &file_entry.size.file_size(options::BINARY).unwrap()),
                                    (ColumnsSimilarImages::SizeAsBytes as u32, &file_entry.size),
                                    (ColumnsSimilarImages::Dimensions as u32, &file_entry.dimensions),
                                    (ColumnsSimilarImages::Name as u32, &file),
                                    (ColumnsSimilarImages::Path as u32, &directory),
                                    (ColumnsSimilarImages::Modification as u32, &(NaiveDateTime::from_timestamp(file_entry.modified_date as i64, 0).to_string())),
                                    (ColumnsSimilarImages::ModificationAsSecs as u32, &(file_entry.modified_date)),
                                    (ColumnsSimilarImages::Color as u32, &(MAIN_ROW_COLOR.to_string())),
                                    (ColumnsSimilarImages::TextColor as u32, &(TEXT_COLOR.to_string())),
                                ];
                                list_store.set(&list_store.append(), &values);
                            }
                        }

                        print_text_messages_to_text_view(text_messages, &text_view_errors);
                    }

                    // Set state
                    {
                        *shared_similar_images_state.borrow_mut() = sf;

                        set_specific_buttons_as_active(&shared_buttons, &NotebookMainEnum::SimilarImages, &["save", "delete", "select", "symlink", "hardlink", "move"], base_images_size > 0);

                        set_buttons(&mut *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::SimilarImages).unwrap(), &buttons_array, &buttons_names);
                    }
                }
            }
            Message::SimilarVideos(ff) => {
                if ff.get_stopped_search() {
                    entry_info.set_text(&fl!("compute_stopped_by_user"));
                } else {
                    //let information = ff.get_information();
                    let text_messages = ff.get_text_messages();

                    let base_videos_size = ff.get_similar_videos().len();

                    entry_info.set_text(format!("{} {} {} {}.", fl!("compute_found"), fl!("compute_duplicates_for"), base_videos_size, fl!("compute_similar_videos")).as_str());

                    // Create GUI
                    {
                        let list_store = get_list_store(&tree_view_similar_videos_finder);

                        let vec_struct_similar = ff.get_similar_videos();

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
                            let values: [(u32, &dyn ToValue); 10] = [
                                (ColumnsSimilarVideos::ActivatableSelectButton as u32, &false),
                                (ColumnsSimilarVideos::SelectionButton as u32, &false),
                                (ColumnsSimilarVideos::Size as u32, &"".to_string()),
                                (ColumnsSimilarVideos::SizeAsBytes as u32, &(0)),
                                (ColumnsSimilarVideos::Name as u32, &"".to_string()),
                                (ColumnsSimilarVideos::Path as u32, &"".to_string()),
                                (ColumnsSimilarVideos::Modification as u32, &"".to_string()),
                                (ColumnsSimilarVideos::ModificationAsSecs as u32, &(0)),
                                (ColumnsSimilarVideos::Color as u32, &(HEADER_ROW_COLOR.to_string())),
                                (ColumnsSimilarVideos::TextColor as u32, &(TEXT_COLOR.to_string())),
                            ];
                            list_store.set(&list_store.append(), &values);

                            // Meat
                            for file_entry in vec_file_entry.iter() {
                                let (directory, file) = split_path(&file_entry.path);
                                let values: [(u32, &dyn ToValue); 10] = [
                                    (ColumnsSimilarVideos::ActivatableSelectButton as u32, &true),
                                    (ColumnsSimilarVideos::SelectionButton as u32, &false),
                                    (ColumnsSimilarVideos::Size as u32, &file_entry.size.file_size(options::BINARY).unwrap()),
                                    (ColumnsSimilarVideos::SizeAsBytes as u32, &file_entry.size),
                                    (ColumnsSimilarVideos::Name as u32, &file),
                                    (ColumnsSimilarVideos::Path as u32, &directory),
                                    (ColumnsSimilarVideos::Modification as u32, &(NaiveDateTime::from_timestamp(file_entry.modified_date as i64, 0).to_string())),
                                    (ColumnsSimilarVideos::ModificationAsSecs as u32, &(file_entry.modified_date)),
                                    (ColumnsSimilarVideos::Color as u32, &(MAIN_ROW_COLOR.to_string())),
                                    (ColumnsSimilarVideos::TextColor as u32, &(TEXT_COLOR.to_string())),
                                ];
                                list_store.set(&list_store.append(), &values);
                            }
                        }

                        print_text_messages_to_text_view(text_messages, &text_view_errors);
                    }

                    // Set state
                    {
                        *shared_similar_videos_state.borrow_mut() = ff;

                        set_specific_buttons_as_active(&shared_buttons, &NotebookMainEnum::SimilarVideos, &["save", "delete", "select", "symlink", "hardlink", "move"], base_videos_size > 0);

                        set_buttons(&mut *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::SimilarVideos).unwrap(), &buttons_array, &buttons_names);
                    }
                }
            }
            Message::SameMusic(mf) => {
                if mf.get_stopped_search() {
                    entry_info.set_text(&fl!("compute_stopped_by_user"));
                } else {
                    let information = mf.get_information();
                    let text_messages = mf.get_text_messages();

                    let same_music_number: usize = information.number_of_duplicates_music_files;

                    entry_info.set_text(format!("{} {} {}.", fl!("compute_found"), same_music_number, fl!("compute_music_files")).as_str());

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
                                (ColumnsSameMusic::ActivatableSelectButton as u32, &false),
                                (ColumnsSameMusic::SelectionButton as u32, &false),
                                (ColumnsSameMusic::Size as u32, &"".to_string()),
                                (ColumnsSameMusic::SizeAsBytes as u32, &(0)),
                                (ColumnsSameMusic::Name as u32, &"".to_string()),
                                (ColumnsSameMusic::Path as u32, &"".to_string()),
                                (
                                    ColumnsSameMusic::Title as u32,
                                    &(match is_title {
                                        true => text.clone(),
                                        false => "".to_string(),
                                    }),
                                ),
                                (
                                    ColumnsSameMusic::Artist as u32,
                                    &(match is_artist {
                                        true => text.clone(),
                                        false => "".to_string(),
                                    }),
                                ),
                                (
                                    ColumnsSameMusic::AlbumTitle as u32,
                                    &(match is_album_title {
                                        true => text.clone(),
                                        false => "".to_string(),
                                    }),
                                ),
                                (
                                    ColumnsSameMusic::AlbumArtist as u32,
                                    &(match is_album_artist {
                                        true => text.clone(),
                                        false => "".to_string(),
                                    }),
                                ),
                                (
                                    ColumnsSameMusic::Year as u32,
                                    &(match is_year {
                                        true => text.clone(),
                                        false => "".to_string(),
                                    }),
                                ),
                                (ColumnsSameMusic::Modification as u32, &"".to_string()),
                                (ColumnsSameMusic::ModificationAsSecs as u32, &(0)),
                                (ColumnsSameMusic::Color as u32, &(HEADER_ROW_COLOR.to_string())),
                                (ColumnsSameMusic::TextColor as u32, &(TEXT_COLOR.to_string())),
                            ];
                            list_store.set(&list_store.append(), &values);
                            for file_entry in vec_file_entry {
                                let (directory, file) = split_path(&file_entry.path);
                                let values: [(u32, &dyn ToValue); 15] = [
                                    (ColumnsSameMusic::ActivatableSelectButton as u32, &true),
                                    (ColumnsSameMusic::SelectionButton as u32, &false),
                                    (ColumnsSameMusic::Size as u32, &file_entry.size.file_size(options::BINARY).unwrap()),
                                    (ColumnsSameMusic::SizeAsBytes as u32, &file_entry.size),
                                    (ColumnsSameMusic::Name as u32, &file),
                                    (ColumnsSameMusic::Path as u32, &directory),
                                    (ColumnsSameMusic::Title as u32, &file_entry.title),
                                    (ColumnsSameMusic::Artist as u32, &file_entry.artist),
                                    (ColumnsSameMusic::AlbumTitle as u32, &file_entry.album_title),
                                    (ColumnsSameMusic::AlbumArtist as u32, &file_entry.album_artist),
                                    (ColumnsSameMusic::Year as u32, &file_entry.year.to_string()),
                                    (ColumnsSameMusic::Modification as u32, &(NaiveDateTime::from_timestamp(file_entry.modified_date as i64, 0).to_string())),
                                    (ColumnsSameMusic::ModificationAsSecs as u32, &(file_entry.modified_date)),
                                    (ColumnsSameMusic::Color as u32, &(MAIN_ROW_COLOR.to_string())),
                                    (ColumnsSameMusic::TextColor as u32, &(TEXT_COLOR.to_string())),
                                ];
                                list_store.set(&list_store.append(), &values);
                            }
                        }
                        print_text_messages_to_text_view(text_messages, &text_view_errors);
                    }

                    // Set state
                    {
                        *shared_same_music_state.borrow_mut() = mf;

                        set_specific_buttons_as_active(&shared_buttons, &NotebookMainEnum::SameMusic, &["save", "delete", "select", "symlink", "hardlink", "move"], same_music_number > 0);

                        set_buttons(&mut *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::SameMusic).unwrap(), &buttons_array, &buttons_names);
                    }
                }
            }
            Message::InvalidSymlinks(ifs) => {
                if ifs.get_stopped_search() {
                    entry_info.set_text(&fl!("compute_stopped_by_user"));
                } else {
                    let information = ifs.get_information();
                    let text_messages = ifs.get_text_messages();

                    let invalid_symlinks: usize = information.number_of_invalid_symlinks;

                    entry_info.set_text(format!("{} {} {}.", fl!("compute_found"), invalid_symlinks, fl!("compute_symlinks")).as_str());

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
                            let values: [(u32, &dyn ToValue); 7] = [
                                (ColumnsInvalidSymlinks::SelectionButton as u32, &false),
                                (ColumnsInvalidSymlinks::Name as u32, &file),
                                (ColumnsInvalidSymlinks::Path as u32, &directory),
                                (ColumnsInvalidSymlinks::DestinationPath as u32, &file_entry.destination_path.to_string_lossy().to_string()),
                                (ColumnsInvalidSymlinks::TypeOfError as u32, &get_text_from_invalid_symlink_cause(&file_entry.type_of_error)),
                                (ColumnsInvalidSymlinks::Modification as u32, &(NaiveDateTime::from_timestamp(file_entry.modified_date as i64, 0).to_string())),
                                (ColumnsInvalidSymlinks::ModificationAsSecs as u32, &(file_entry.modified_date as i64)),
                            ];
                            list_store.set(&list_store.append(), &values);
                        }
                        print_text_messages_to_text_view(text_messages, &text_view_errors);
                    }

                    // Set state
                    {
                        *shared_same_invalid_symlinks.borrow_mut() = ifs;

                        set_specific_buttons_as_active(&shared_buttons, &NotebookMainEnum::Symlinks, &["save", "delete", "select", "move"], invalid_symlinks > 0);

                        set_buttons(&mut *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::Symlinks).unwrap(), &buttons_array, &buttons_names);
                    }
                }
            }
            Message::BrokenFiles(br) => {
                if br.get_stopped_search() {
                    entry_info.set_text(&fl!("compute_stopped_by_user"));
                } else {
                    let information = br.get_information();
                    let text_messages = br.get_text_messages();

                    let broken_files_number: usize = information.number_of_broken_files;

                    entry_info.set_text(format!("{} {} {}.", fl!("compute_found"), broken_files_number, fl!("compute_broken_files")).as_str());

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
                            let values: [(u32, &dyn ToValue); 6] = [
                                (ColumnsBrokenFiles::SelectionButton as u32, &false),
                                (ColumnsBrokenFiles::Name as u32, &file),
                                (ColumnsBrokenFiles::Path as u32, &directory),
                                (ColumnsBrokenFiles::ErrorType as u32, &file_entry.error_string),
                                (ColumnsBrokenFiles::Modification as u32, &(NaiveDateTime::from_timestamp(file_entry.modified_date as i64, 0).to_string())),
                                (ColumnsBrokenFiles::ModificationAsSecs as u32, &(file_entry.modified_date as i64)),
                            ];
                            list_store.set(&list_store.append(), &values);
                        }
                        print_text_messages_to_text_view(text_messages, &text_view_errors);
                    }

                    // Set state
                    {
                        *shared_broken_files_state.borrow_mut() = br;

                        set_specific_buttons_as_active(&shared_buttons, &NotebookMainEnum::BrokenFiles, &["save", "delete", "select", "move"], broken_files_number > 0);

                        set_buttons(&mut *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::BrokenFiles).unwrap(), &buttons_array, &buttons_names);
                    }
                }
            }
        }
        // Returning false here would close the receiver and have senders fail
        glib::Continue(true)
    });
}

fn set_specific_buttons_as_active(buttons_array: &Rc<RefCell<HashMap<NotebookMainEnum, HashMap<String, bool>>>>, notebook_enum: &NotebookMainEnum, buttons: &[&str], value_to_set: bool) {
    for i in buttons {
        *buttons_array.borrow_mut().get_mut(notebook_enum).unwrap().get_mut(*i).unwrap() = value_to_set;
    }
}
