mod help_functions;

use czkawka_core::*;
use humansize::{file_size_opts as options, FileSize};

extern crate gtk;
use crate::help_functions::*;
use chrono::NaiveDateTime;
use czkawka_core::big_file::BigFile;
use czkawka_core::common_traits::SaveResults;
use czkawka_core::duplicate::CheckingMethod;
use czkawka_core::empty_files::EmptyFiles;
use czkawka_core::empty_folder::EmptyFolder;
use czkawka_core::temporary::Temporary;
use duplicate::DuplicateFinder;
use gtk::prelude::*;
use gtk::{Builder, SelectionMode, TreeIter, TreeView};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::{env, fs, process};

fn main() {
    // Printing version
    {
        let all_arguments: Vec<String> = env::args().skip(1).collect(); // Not need to check program name

        for i in all_arguments {
            if i == "--v" || i == "--version" {
                println!("Czkawka GUI {}", CZKAWKA_VERSION);
                process::exit(0);
            }
        }
    }

    gtk::init().expect("Failed to initialize GTK.");

    //// Loading glade file content and build with it help UI
    let glade_src = include_str!("../czkawka.glade");
    let builder = Builder::from_string(glade_src);

    //// Windows
    let main_window: gtk::Window = builder.get_object("main_window").unwrap();
    main_window.show_all();
    main_window.set_title("Czkawka");

    ////////////////////////////////////////////////////////////////////////////////////////////////
    //// States

    // Buttons State - to remember existence of different buttons on pages

    let shared_buttons: Rc<RefCell<_>> = Rc::new(RefCell::new(HashMap::<String, HashMap<String, bool>>::new()));
    shared_buttons.borrow_mut().clear();

    // Show by default only search button
    for i in ["duplicate", "empty_folder", "empty_file", "temporary_file", "big_file"].iter() {
        let mut temp_hashmap: HashMap<String, bool> = Default::default();
        for j in ["search", "stop", "resume", "pause", "select", "delete", "save"].iter() {
            if *j == "search" {
                temp_hashmap.insert(j.to_string(), true);
            } else {
                temp_hashmap.insert(j.to_string(), false);
            }
        }
        shared_buttons.borrow_mut().insert(i.to_string(), temp_hashmap);
    }

    // State of search results - probably are not necessary due

    let shared_duplication_state: Rc<RefCell<_>> = Rc::new(RefCell::new(DuplicateFinder::new()));
    let shared_empty_folders_state: Rc<RefCell<_>> = Rc::new(RefCell::new(EmptyFolder::new()));
    let shared_empty_files_state: Rc<RefCell<_>> = Rc::new(RefCell::new(EmptyFiles::new()));
    let shared_temporary_files_state: Rc<RefCell<_>> = Rc::new(RefCell::new(Temporary::new()));
    let shared_big_files_state: Rc<RefCell<_>> = Rc::new(RefCell::new(BigFile::new()));

    // State of confirmation dialogs
    let shared_confirmation_dialog_delete_dialog_showing_state: Rc<RefCell<_>> = Rc::new(RefCell::new(true));

    ////////////////////////////////////////////////////////////////////////////////////////////////

    //// GUI Entry
    let entry_duplicate_minimal_size: gtk::Entry = builder.get_object("entry_duplicate_minimal_size").unwrap();
    let entry_allowed_extensions: gtk::Entry = builder.get_object("entry_allowed_extensions").unwrap();
    let entry_excluded_items: gtk::Entry = builder.get_object("entry_excluded_items").unwrap();
    let entry_big_files_number: gtk::Entry = builder.get_object("entry_big_files_number").unwrap();

    //// GUI Buttons
    let buttons_search: gtk::Button = builder.get_object("buttons_search").unwrap();
    let buttons_stop: gtk::Button = builder.get_object("buttons_stop").unwrap();
    let buttons_resume: gtk::Button = builder.get_object("buttons_resume").unwrap();
    let buttons_pause: gtk::Button = builder.get_object("buttons_pause").unwrap();
    let buttons_select: gtk::Button = builder.get_object("buttons_select").unwrap();
    let buttons_delete: gtk::Button = builder.get_object("buttons_delete").unwrap();
    let buttons_save: gtk::Button = builder.get_object("buttons_save").unwrap();

    let buttons_add_included_directory: gtk::Button = builder.get_object("buttons_add_included_directory").unwrap();
    let buttons_remove_included_directory: gtk::Button = builder.get_object("buttons_remove_included_directory").unwrap();
    let buttons_add_excluded_directory: gtk::Button = builder.get_object("buttons_add_excluded_directory").unwrap();
    let buttons_remove_excluded_directory: gtk::Button = builder.get_object("buttons_remove_excluded_directory").unwrap();

    // Buttons search popover buttons
    let buttons_popover_select_all: gtk::Button = builder.get_object("buttons_popover_select_all").unwrap();
    let buttons_popover_unselect_all: gtk::Button = builder.get_object("buttons_popover_unselect_all").unwrap();
    let buttons_popover_reverse: gtk::Button = builder.get_object("buttons_popover_reverse").unwrap();
    let buttons_popover_select_all_except_oldest: gtk::Button = builder.get_object("buttons_popover_select_all_except_oldest").unwrap();
    let buttons_popover_select_all_except_newest: gtk::Button = builder.get_object("buttons_popover_select_all_except_newest").unwrap();
    let buttons_popover_select_one_oldest: gtk::Button = builder.get_object("buttons_popover_select_one_oldest").unwrap();
    let buttons_popover_select_one_newest: gtk::Button = builder.get_object("buttons_popover_select_one_newest").unwrap();

    //// Popovers
    let popover_select: gtk::Popover = builder.get_object("popover_select").unwrap();

    //// Check Buttons
    let check_button_recursive: gtk::CheckButton = builder.get_object("check_button_recursive").unwrap();

    //// Radio Buttons
    let radio_button_size: gtk::RadioButton = builder.get_object("radio_button_size").unwrap();
    let radio_button_hashmb: gtk::RadioButton = builder.get_object("radio_button_hashmb").unwrap();
    let radio_button_hash: gtk::RadioButton = builder.get_object("radio_button_hash").unwrap();

    //// Notebooks
    let notebook_chooser_tool: gtk::Notebook = builder.get_object("notebook_chooser_tool").unwrap();
    let mut notebook_chooser_tool_children_names: Vec<String> = Vec::new();

    for i in notebook_chooser_tool.get_children() {
        notebook_chooser_tool_children_names.push(i.get_buildable_name().unwrap().to_string());
    }

    //// Entry
    let entry_info: gtk::Entry = builder.get_object("entry_info").unwrap(); // To show default

    //// Text View
    let text_view_errors: gtk::TextView = builder.get_object("text_view_errors").unwrap();

    //// Scrolled windows

    // Main notebook
    let scrolled_window_duplicate_finder: gtk::ScrolledWindow = builder.get_object("scrolled_window_duplicate_finder").unwrap();
    let scrolled_window_empty_folder_finder: gtk::ScrolledWindow = builder.get_object("scrolled_window_empty_folder_finder").unwrap();
    let scrolled_window_empty_files_finder: gtk::ScrolledWindow = builder.get_object("scrolled_window_empty_files_finder").unwrap();
    let scrolled_window_temporary_files_finder: gtk::ScrolledWindow = builder.get_object("scrolled_window_temporary_files_finder").unwrap();
    let scrolled_window_big_files_finder: gtk::ScrolledWindow = builder.get_object("scrolled_window_big_files_finder").unwrap();

    // Upper notebook
    let scrolled_window_included_directories: gtk::ScrolledWindow = builder.get_object("scrolled_window_included_directories").unwrap();
    let scrolled_window_excluded_directories: gtk::ScrolledWindow = builder.get_object("scrolled_window_excluded_directories").unwrap();

    //// Set starting information in bottom panel
    {
        entry_info.set_text("Duplicated Files");

        // Disable and show buttons
        buttons_search.show();
        buttons_save.hide();
        buttons_delete.hide();
        buttons_stop.hide();
        buttons_resume.hide();
        buttons_pause.hide();
        buttons_select.hide();

        // Set Main ScrolledWindow Treeviews
        {
            // Duplicate Files
            {
                let col_types: [glib::types::Type; 6] = [
                    glib::types::Type::String,
                    glib::types::Type::String,
                    glib::types::Type::String,
                    glib::types::Type::U64,
                    glib::types::Type::String,
                    glib::types::Type::String,
                ];
                let list_store: gtk::ListStore = gtk::ListStore::new(&col_types);

                let mut tree_view: gtk::TreeView = TreeView::with_model(&list_store);

                tree_view.get_selection().set_mode(SelectionMode::Multiple);
                tree_view.get_selection().set_select_function(Some(Box::new(select_function_3column)));

                create_tree_view_duplicates(&mut tree_view);

                scrolled_window_duplicate_finder.add(&tree_view);
                scrolled_window_duplicate_finder.show_all();
            }
            // Empty Folders
            {
                let col_types: [glib::types::Type; 3] = [glib::types::Type::String, glib::types::Type::String, glib::types::Type::String];
                let list_store: gtk::ListStore = gtk::ListStore::new(&col_types);

                let mut tree_view: gtk::TreeView = TreeView::with_model(&list_store);

                tree_view.get_selection().set_mode(SelectionMode::Multiple);

                create_tree_view_empty_folders(&mut tree_view);

                scrolled_window_empty_folder_finder.add(&tree_view);
                scrolled_window_empty_folder_finder.show_all();
            }
            // Empty Files
            {
                let col_types: [glib::types::Type; 3] = [glib::types::Type::String, glib::types::Type::String, glib::types::Type::String];
                let list_store: gtk::ListStore = gtk::ListStore::new(&col_types);

                let mut tree_view: gtk::TreeView = TreeView::with_model(&list_store);

                tree_view.get_selection().set_mode(SelectionMode::Multiple);

                create_tree_view_empty_files(&mut tree_view);

                scrolled_window_empty_files_finder.add(&tree_view);
                scrolled_window_empty_files_finder.show_all();
            }
            // Temporary Files
            {
                let col_types: [glib::types::Type; 3] = [glib::types::Type::String, glib::types::Type::String, glib::types::Type::String];
                let list_store: gtk::ListStore = gtk::ListStore::new(&col_types);

                let mut tree_view: gtk::TreeView = TreeView::with_model(&list_store);

                tree_view.get_selection().set_mode(SelectionMode::Multiple);

                create_tree_view_temporary_files(&mut tree_view);

                scrolled_window_temporary_files_finder.add(&tree_view);
                scrolled_window_temporary_files_finder.show_all();
            }
            // Big Files
            {
                let col_types: [glib::types::Type; 4] = [glib::types::Type::String, glib::types::Type::String, glib::types::Type::String, glib::types::Type::String];
                let list_store: gtk::ListStore = gtk::ListStore::new(&col_types);

                let mut tree_view: gtk::TreeView = TreeView::with_model(&list_store);

                tree_view.get_selection().set_mode(SelectionMode::Multiple);

                create_tree_view_big_files(&mut tree_view);

                scrolled_window_big_files_finder.add(&tree_view);
                scrolled_window_big_files_finder.show_all();
            }
        }

        // Set Included Directory
        {
            let col_types: [glib::types::Type; 2] = [glib::types::Type::String, glib::types::Type::String];
            let list_store: gtk::ListStore = gtk::ListStore::new(&col_types);

            let mut tree_view_included_directory: gtk::TreeView = TreeView::with_model(&list_store);

            tree_view_included_directory.get_selection().set_mode(SelectionMode::Single);

            create_tree_view_directories(&mut tree_view_included_directory);

            let col_indices = [0, 1];

            let current_dir: String = match env::current_dir() {
                Ok(t) => t.to_str().unwrap().to_string(),
                Err(_) => {
                    println!("Failed to read current directory, setting /home instead");
                    "/home".to_string()
                }
            };

            let values: [&dyn ToValue; 2] = [&current_dir, &(MAIN_ROW_COLOR.to_string())];
            list_store.set(&list_store.append(), &col_indices, &values);

            scrolled_window_included_directories.add(&tree_view_included_directory);
            scrolled_window_included_directories.show_all();
        }
        // Set Excluded Directory
        {
            let col_types: [glib::types::Type; 2] = [glib::types::Type::String, glib::types::Type::String];
            let list_store: gtk::ListStore = gtk::ListStore::new(&col_types);

            let mut tree_view_excluded_directory: gtk::TreeView = TreeView::with_model(&list_store);

            tree_view_excluded_directory.get_selection().set_mode(SelectionMode::Single);

            create_tree_view_directories(&mut tree_view_excluded_directory);

            let col_indices = [0, 1];

            for i in ["/proc", "/dev", "/sys"].iter() {
                let values: [&dyn ToValue; 2] = [&i, &(MAIN_ROW_COLOR.to_string())];
                list_store.set(&list_store.append(), &col_indices, &values);
            }

            scrolled_window_excluded_directories.add(&tree_view_excluded_directory);
            scrolled_window_excluded_directories.show_all();
        }
    }

    // Connecting events
    {
        // Connect Notebook Tabs
        {
            let shared_buttons = shared_buttons.clone();

            #[allow(clippy::redundant_clone)]
            let buttons_search = buttons_search.clone();
            #[allow(clippy::redundant_clone)]
            let buttons_stop = buttons_stop.clone();
            #[allow(clippy::redundant_clone)]
            let buttons_resume = buttons_resume.clone();
            #[allow(clippy::redundant_clone)]
            let buttons_pause = buttons_pause.clone();
            #[allow(clippy::redundant_clone)]
            let buttons_select = buttons_select.clone();
            #[allow(clippy::redundant_clone)]
            let buttons_delete = buttons_delete.clone();
            #[allow(clippy::redundant_clone)]
            let buttons_save = buttons_save.clone();

            let notebook_chooser_tool_children_names = notebook_chooser_tool_children_names.clone();

            notebook_chooser_tool.connect_switch_page(move |_, _, number| {
                let page: &str;
                match notebook_chooser_tool_children_names.get(number as usize).unwrap().as_str() {
                    "notebook_duplicate_finder_label" => {
                        page = "duplicate";
                    }
                    "scrolled_window_empty_folder_finder" => {
                        page = "empty_folder";
                    }
                    "scrolled_window_empty_files_finder" => page = "empty_file",
                    "scrolled_window_temporary_files_finder" => page = "temporary_file",
                    "notebook_big_file_finder" => page = "big_file",
                    e => {
                        panic!("Not existent page {}", e);
                    }
                };

                if *shared_buttons.borrow_mut().get_mut(page).unwrap().get_mut("search").unwrap() {
                    buttons_search.show();
                } else {
                    buttons_search.hide();
                }
                if *shared_buttons.borrow_mut().get_mut(page).unwrap().get_mut("stop").unwrap() {
                    buttons_stop.show();
                } else {
                    buttons_stop.hide();
                }
                if *shared_buttons.borrow_mut().get_mut(page).unwrap().get_mut("resume").unwrap() {
                    buttons_resume.show();
                } else {
                    buttons_resume.hide();
                }
                if *shared_buttons.borrow_mut().get_mut(page).unwrap().get_mut("pause").unwrap() {
                    buttons_pause.show();
                } else {
                    buttons_pause.hide();
                }
                if *shared_buttons.borrow_mut().get_mut(page).unwrap().get_mut("select").unwrap() {
                    buttons_select.show();
                } else {
                    buttons_select.hide();
                }
                if *shared_buttons.borrow_mut().get_mut(page).unwrap().get_mut("delete").unwrap() {
                    buttons_delete.show();
                } else {
                    buttons_delete.hide();
                }
                if *shared_buttons.borrow_mut().get_mut(page).unwrap().get_mut("save").unwrap() {
                    buttons_save.show();
                } else {
                    buttons_save.hide();
                }
            });
        }

        //// Connect Buttons

        // Down notepad
        {
            assert!(notebook_chooser_tool_children_names.contains(&"notebook_duplicate_finder_label".to_string()));
            assert!(notebook_chooser_tool_children_names.contains(&"scrolled_window_empty_folder_finder".to_string()));
            assert!(notebook_chooser_tool_children_names.contains(&"scrolled_window_empty_files_finder".to_string()));
            assert!(notebook_chooser_tool_children_names.contains(&"scrolled_window_temporary_files_finder".to_string()));
            assert!(notebook_chooser_tool_children_names.contains(&"notebook_big_file_finder".to_string()));
            // Search button
            {
                let buttons_delete = buttons_delete.clone();
                let buttons_save = buttons_save.clone();
                let buttons_select = buttons_select.clone();
                let entry_info = entry_info.clone();
                let notebook_chooser_tool_children_names = notebook_chooser_tool_children_names.clone();
                let notebook_chooser_tool = notebook_chooser_tool.clone();
                let scrolled_window_duplicate_finder = scrolled_window_duplicate_finder.clone();
                let scrolled_window_empty_folder_finder = scrolled_window_empty_folder_finder.clone();
                let scrolled_window_empty_files_finder = scrolled_window_empty_files_finder.clone();
                let scrolled_window_temporary_files_finder = scrolled_window_temporary_files_finder.clone();
                let scrolled_window_big_files_finder = scrolled_window_big_files_finder.clone();
                let scrolled_window_included_directories = scrolled_window_included_directories.clone();
                let scrolled_window_excluded_directories = scrolled_window_excluded_directories.clone();
                let text_view_errors = text_view_errors.clone();
                let shared_duplication_state = shared_duplication_state.clone();
                let shared_empty_folders_state = shared_empty_folders_state.clone();
                let shared_empty_files_state = shared_empty_files_state.clone();
                let shared_temporary_files_state = shared_temporary_files_state.clone();
                let shared_big_files_state = shared_big_files_state.clone();
                let shared_buttons = shared_buttons.clone();
                buttons_search.connect_clicked(move |_| {
                    match notebook_chooser_tool_children_names.get(notebook_chooser_tool.get_current_page().unwrap() as usize).unwrap().as_str() {
                        "notebook_duplicate_finder_label" => {
                            // Find duplicates

                            let mut df = DuplicateFinder::new();
                            let check_method;
                            if radio_button_size.get_active() {
                                check_method = duplicate::CheckingMethod::Size;
                            } else if radio_button_hashmb.get_active() {
                                check_method = duplicate::CheckingMethod::HashMB;
                            } else if radio_button_hash.get_active() {
                                check_method = duplicate::CheckingMethod::Hash;
                            } else {
                                panic!("No radio button is pressed");
                            }
                            {
                                df.set_included_directory(get_string_from_list_store(&scrolled_window_included_directories));
                                df.set_excluded_directory(get_string_from_list_store(&scrolled_window_excluded_directories));
                                df.set_recursive_search(check_button_recursive.get_active());
                                df.set_excluded_items(entry_excluded_items.get_text().as_str().to_string());
                                df.set_allowed_extensions(entry_allowed_extensions.get_text().as_str().to_string());
                                df.set_minimal_file_size(match entry_duplicate_minimal_size.get_text().as_str().parse::<u64>() {
                                    Ok(t) => t,
                                    Err(_) => 1024, // By default
                                });
                                df.set_check_method(check_method.clone());
                                df.set_delete_method(duplicate::DeleteMethod::None);
                                df.find_duplicates();
                            }
                            let information = df.get_information();
                            let text_messages = df.get_text_messages();

                            let duplicates_number: usize;
                            let duplicates_size: u64;
                            let duplicates_group: usize;

                            match check_method {
                                CheckingMethod::Hash | CheckingMethod::HashMB => {
                                    duplicates_number = information.number_of_duplicated_files_by_hash;
                                    duplicates_size = information.lost_space_by_hash;
                                    duplicates_group = information.number_of_groups_by_hash;
                                }
                                CheckingMethod::Size => {
                                    duplicates_number = information.number_of_duplicated_files_by_size;
                                    duplicates_size = information.lost_space_by_size;
                                    duplicates_group = information.number_of_groups_by_size;
                                }
                                CheckingMethod::None => {
                                    panic!();
                                }
                            }

                            entry_info.set_text(format!("Found {} duplicates files in {} groups which took {}.", duplicates_number, duplicates_group, duplicates_size.file_size(options::BINARY).unwrap()).as_str());

                            // Create GUI
                            {
                                let list_store = scrolled_window_duplicate_finder
                                    .get_children()
                                    .get(0)
                                    .unwrap()
                                    .clone()
                                    .downcast::<gtk::TreeView>()
                                    .unwrap()
                                    .get_model()
                                    .unwrap()
                                    .downcast::<gtk::ListStore>()
                                    .unwrap();
                                list_store.clear();

                                let col_indices = [0, 1, 2, 3, 4, 5];

                                match check_method {
                                    CheckingMethod::Hash | CheckingMethod::HashMB => {
                                        let btreemap = df.get_files_sorted_by_hash();

                                        for (size, vectors_vector) in btreemap.iter().rev() {
                                            for vector in vectors_vector {
                                                let values: [&dyn ToValue; 6] = [
                                                    &(vector.len().to_string() + " x " + size.to_string().as_str()),
                                                    &(format!("{} ({} bytes) lost", ((vector.len() - 1) as u64 * *size as u64).file_size(options::BINARY).unwrap(), (vector.len() - 1) as u64 * *size as u64)),
                                                    &"".to_string(), // No text in 3 column
                                                    &(0),            // Not used here
                                                    &(HEADER_ROW_COLOR.to_string()),
                                                    &(TEXT_COLOR.to_string()),
                                                ];
                                                list_store.set(&list_store.append(), &col_indices, &values);
                                                for entry in vector {
                                                    let path = &entry.path;
                                                    let index = path.rfind('/').unwrap();

                                                    let values: [&dyn ToValue; 6] = [
                                                        &(path[index + 1..].to_string()),
                                                        &(path[..index].to_string()),
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
                                                &(vector.len().to_string() + " x " + size.to_string().as_str()),
                                                &(format!("{} ({} bytes) lost", ((vector.len() - 1) as u64 * *size as u64).file_size(options::BINARY).unwrap(), (vector.len() - 1) as u64 * *size as u64)),
                                                &"".to_string(), // No text in 3 column
                                                &(0),            // Not used here
                                                &(HEADER_ROW_COLOR.to_string()),
                                                &(TEXT_COLOR.to_string()),
                                            ];
                                            list_store.set(&list_store.append(), &col_indices, &values);
                                            for entry in vector {
                                                let path = &entry.path;
                                                let index = path.rfind('/').unwrap();

                                                let values: [&dyn ToValue; 6] = [
                                                    &(path[index + 1..].to_string()),
                                                    &(path[..index].to_string()),
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

                                print_text_messages_to_text_view(&text_messages, &text_view_errors);
                            }

                            // Set state
                            {
                                *shared_duplication_state.borrow_mut() = df;

                                if duplicates_size > 0 {
                                    buttons_save.show();
                                    buttons_delete.show();
                                    buttons_select.show();
                                    *shared_buttons.borrow_mut().get_mut("duplicate").unwrap().get_mut("save").unwrap() = true;
                                    *shared_buttons.borrow_mut().get_mut("duplicate").unwrap().get_mut("delete").unwrap() = true;
                                    *shared_buttons.borrow_mut().get_mut("duplicate").unwrap().get_mut("select").unwrap() = true;
                                } else {
                                    buttons_save.hide();
                                    buttons_delete.hide();
                                    buttons_select.hide();
                                    *shared_buttons.borrow_mut().get_mut("duplicate").unwrap().get_mut("save").unwrap() = false;
                                    *shared_buttons.borrow_mut().get_mut("duplicate").unwrap().get_mut("delete").unwrap() = false;
                                    *shared_buttons.borrow_mut().get_mut("duplicate").unwrap().get_mut("select").unwrap() = false;
                                }
                            }
                        }
                        "scrolled_window_empty_folder_finder" => {
                            // Find empty folders
                            let mut ef = EmptyFolder::new();

                            ef.set_included_directory(get_string_from_list_store(&scrolled_window_included_directories));
                            ef.set_delete_folder(false);
                            ef.find_empty_folders();

                            let information = ef.get_information();
                            let text_messages = ef.get_text_messages();

                            let empty_folder_number: usize = information.number_of_empty_folders;

                            entry_info.set_text(format!("Found {} empty folders.", empty_folder_number).as_str());

                            // Create GUI
                            {
                                let list_store = scrolled_window_empty_folder_finder
                                    .get_children()
                                    .get(0)
                                    .unwrap()
                                    .clone()
                                    .downcast::<gtk::TreeView>()
                                    .unwrap()
                                    .get_model()
                                    .unwrap()
                                    .downcast::<gtk::ListStore>()
                                    .unwrap();
                                list_store.clear();

                                let col_indices = [0, 1, 2];

                                let hashmap = ef.get_empty_folder_list();

                                for (name, entry) in hashmap {
                                    let name: String = name[..(name.len() - 1)].to_string();
                                    let index = name.rfind('/').unwrap();
                                    let values: [&dyn ToValue; 3] = [&(name[index + 1..].to_string()), &(name[..index].to_string()), &(NaiveDateTime::from_timestamp(entry.modified_date as i64, 0).to_string())];
                                    list_store.set(&list_store.append(), &col_indices, &values);
                                }
                                print_text_messages_to_text_view(&text_messages, &text_view_errors);
                            }

                            // Set state
                            {
                                *shared_empty_folders_state.borrow_mut() = ef;

                                if empty_folder_number > 0 {
                                    buttons_save.show();
                                    buttons_delete.show();
                                    *shared_buttons.borrow_mut().get_mut("empty_folder").unwrap().get_mut("save").unwrap() = true;
                                    *shared_buttons.borrow_mut().get_mut("empty_folder").unwrap().get_mut("delete").unwrap() = true;
                                } else {
                                    buttons_save.hide();
                                    buttons_delete.hide();
                                    *shared_buttons.borrow_mut().get_mut("empty_folder").unwrap().get_mut("save").unwrap() = false;
                                    *shared_buttons.borrow_mut().get_mut("empty_folder").unwrap().get_mut("delete").unwrap() = false;
                                }
                            }
                        }
                        "scrolled_window_empty_files_finder" => {
                            // Find empty files
                            let mut vf = EmptyFiles::new();

                            vf.set_included_directory(get_string_from_list_store(&scrolled_window_included_directories));
                            vf.set_excluded_directory(get_string_from_list_store(&scrolled_window_excluded_directories));
                            vf.set_recursive_search(check_button_recursive.get_active());
                            vf.set_excluded_items(entry_excluded_items.get_text().as_str().to_string());
                            vf.set_allowed_extensions(entry_allowed_extensions.get_text().as_str().to_string());
                            vf.find_empty_files();

                            let information = vf.get_information();
                            let text_messages = vf.get_text_messages();

                            let empty_files_number: usize = information.number_of_empty_files;

                            entry_info.set_text(format!("Found {} empty files.", empty_files_number).as_str());

                            // Create GUI
                            {
                                let list_store = scrolled_window_empty_files_finder
                                    .get_children()
                                    .get(0)
                                    .unwrap()
                                    .clone()
                                    .downcast::<gtk::TreeView>()
                                    .unwrap()
                                    .get_model()
                                    .unwrap()
                                    .downcast::<gtk::ListStore>()
                                    .unwrap();
                                list_store.clear();

                                let col_indices = [0, 1, 2];

                                let vector = vf.get_empty_files();

                                for file_entry in vector {
                                    let name: String = file_entry.path.to_string();
                                    let index = name.rfind('/').unwrap();
                                    let values: [&dyn ToValue; 3] = [&(name[index + 1..].to_string()), &(name[..index].to_string()), &(NaiveDateTime::from_timestamp(file_entry.modified_date as i64, 0).to_string())];
                                    list_store.set(&list_store.append(), &col_indices, &values);
                                }
                                print_text_messages_to_text_view(&text_messages, &text_view_errors);
                            }

                            // Set state
                            {
                                *shared_empty_files_state.borrow_mut() = vf;

                                if empty_files_number > 0 {
                                    buttons_save.show();
                                    buttons_delete.show();
                                    *shared_buttons.borrow_mut().get_mut("empty_file").unwrap().get_mut("save").unwrap() = true;
                                    *shared_buttons.borrow_mut().get_mut("empty_file").unwrap().get_mut("delete").unwrap() = true;
                                } else {
                                    buttons_save.hide();
                                    buttons_delete.hide();
                                    *shared_buttons.borrow_mut().get_mut("empty_file").unwrap().get_mut("save").unwrap() = false;
                                    *shared_buttons.borrow_mut().get_mut("empty_file").unwrap().get_mut("delete").unwrap() = false;
                                }
                            }
                        }
                        "scrolled_window_temporary_files_finder" => {
                            // Find temporary files
                            let mut tf = Temporary::new();

                            tf.set_included_directory(get_string_from_list_store(&scrolled_window_included_directories));
                            tf.set_excluded_directory(get_string_from_list_store(&scrolled_window_excluded_directories));
                            tf.set_recursive_search(check_button_recursive.get_active());
                            tf.set_excluded_items(entry_excluded_items.get_text().as_str().to_string());
                            tf.find_temporary_files();

                            let information = tf.get_information();
                            let text_messages = tf.get_text_messages();

                            let temporary_files_number: usize = information.number_of_temporary_files;

                            entry_info.set_text(format!("Found {} temporary files.", temporary_files_number).as_str());

                            // Create GUI
                            {
                                let list_store = scrolled_window_temporary_files_finder
                                    .get_children()
                                    .get(0)
                                    .unwrap()
                                    .clone()
                                    .downcast::<gtk::TreeView>()
                                    .unwrap()
                                    .get_model()
                                    .unwrap()
                                    .downcast::<gtk::ListStore>()
                                    .unwrap();
                                list_store.clear();

                                let col_indices = [0, 1, 2];

                                let vector = tf.get_temporary_files();

                                for file_entry in vector {
                                    let name: String = file_entry.path.to_string();
                                    let index = name.rfind('/').unwrap();
                                    let values: [&dyn ToValue; 3] = [&(name[index + 1..].to_string()), &(name[..index].to_string()), &(NaiveDateTime::from_timestamp(file_entry.modified_date as i64, 0).to_string())];
                                    list_store.set(&list_store.append(), &col_indices, &values);
                                }
                                print_text_messages_to_text_view(&text_messages, &text_view_errors);
                            }

                            // Set state
                            {
                                *shared_temporary_files_state.borrow_mut() = tf;

                                if temporary_files_number > 0 {
                                    buttons_save.show();
                                    buttons_delete.show();
                                    *shared_buttons.borrow_mut().get_mut("temporary_file").unwrap().get_mut("save").unwrap() = true;
                                    *shared_buttons.borrow_mut().get_mut("temporary_file").unwrap().get_mut("delete").unwrap() = true;
                                } else {
                                    buttons_save.hide();
                                    buttons_delete.hide();
                                    *shared_buttons.borrow_mut().get_mut("temporary_file").unwrap().get_mut("save").unwrap() = false;
                                    *shared_buttons.borrow_mut().get_mut("temporary_file").unwrap().get_mut("delete").unwrap() = false;
                                }
                            }
                        }
                        "notebook_big_file_finder" => {
                            // Find temporary files
                            let mut bf = BigFile::new();

                            bf.set_included_directory(get_string_from_list_store(&scrolled_window_included_directories));
                            bf.set_excluded_directory(get_string_from_list_store(&scrolled_window_excluded_directories));
                            bf.set_recursive_search(check_button_recursive.get_active());
                            bf.set_excluded_items(entry_excluded_items.get_text().as_str().to_string());
                            bf.set_number_of_files_to_check(match entry_big_files_number.get_text().as_str().parse::<usize>() {
                                Ok(t) => t,
                                Err(_) => 50, // By default
                            });
                            bf.find_big_files();

                            let information = bf.get_information();
                            let text_messages = bf.get_text_messages();

                            let biggest_files_number: usize = information.number_of_real_files;

                            entry_info.set_text(format!("Found {} biggest files.", biggest_files_number).as_str());

                            // Create GUI
                            {
                                let list_store = scrolled_window_big_files_finder
                                    .get_children()
                                    .get(0)
                                    .unwrap()
                                    .clone()
                                    .downcast::<gtk::TreeView>()
                                    .unwrap()
                                    .get_model()
                                    .unwrap()
                                    .downcast::<gtk::ListStore>()
                                    .unwrap();
                                list_store.clear();

                                let col_indices = [0, 1, 2, 3];

                                let btreemap = bf.get_big_files();

                                for (size, vector) in btreemap.iter().rev() {
                                    for file_entry in vector {
                                        let name: String = file_entry.path.to_string();
                                        let index = name.rfind('/').unwrap();
                                        let values: [&dyn ToValue; 4] = [
                                            &(format!("{} ({} bytes)", size.file_size(options::BINARY).unwrap(), size)),
                                            &(name[index + 1..].to_string()),
                                            &(name[..index].to_string()),
                                            &(NaiveDateTime::from_timestamp(file_entry.modified_date as i64, 0).to_string()),
                                        ];
                                        list_store.set(&list_store.append(), &col_indices, &values);
                                    }
                                }
                                print_text_messages_to_text_view(&text_messages, &text_view_errors);
                            }

                            // Set state
                            {
                                *shared_big_files_state.borrow_mut() = bf;

                                if biggest_files_number > 0 {
                                    buttons_save.show();
                                    buttons_delete.show();
                                    *shared_buttons.borrow_mut().get_mut("big_file").unwrap().get_mut("save").unwrap() = true;
                                    *shared_buttons.borrow_mut().get_mut("big_file").unwrap().get_mut("delete").unwrap() = true;
                                } else {
                                    buttons_save.hide();
                                    buttons_delete.hide();
                                    *shared_buttons.borrow_mut().get_mut("big_file").unwrap().get_mut("save").unwrap() = false;
                                    *shared_buttons.borrow_mut().get_mut("big_file").unwrap().get_mut("delete").unwrap() = false;
                                }
                            }
                        }
                        e => panic!("Not existent {}", e),
                    }
                });
            }
            // Delete button
            {
                let scrolled_window_duplicate_finder = scrolled_window_duplicate_finder.clone();
                let notebook_chooser_tool_children_names = notebook_chooser_tool_children_names.clone();
                let notebook_chooser_tool = notebook_chooser_tool.clone();
                let main_window = main_window.clone();

                buttons_delete.connect_clicked(move |_| {
                    if *shared_confirmation_dialog_delete_dialog_showing_state.borrow_mut() {
                        let confirmation_dialog_delete = gtk::Dialog::with_buttons(
                            Option::from("Delete confirmation"),
                            Option::from(&main_window),
                            gtk::DialogFlags::MODAL,
                            &[("Ok", gtk::ResponseType::Ok), ("Close", gtk::ResponseType::Cancel)],
                        );
                        let label: gtk::Label = gtk::Label::new(Some("Are you sure that you want to delete files?"));
                        let check_button: gtk::CheckButton = gtk::CheckButton::with_label("Ask in future");
                        check_button.set_active(true);

                        for widgets in confirmation_dialog_delete.get_children() {
                            // By default GtkBox is child of dialog, so we can easily add other things to it
                            widgets.clone().downcast::<gtk::Box>().unwrap().add(&label);
                            widgets.downcast::<gtk::Box>().unwrap().add(&check_button);
                        }

                        confirmation_dialog_delete.show_all();

                        let response_type = confirmation_dialog_delete.run();
                        if response_type == gtk::ResponseType::Ok {
                            if !check_button.get_active() {
                                *shared_confirmation_dialog_delete_dialog_showing_state.borrow_mut() = false;
                            }
                        } else {
                            confirmation_dialog_delete.close();
                            return;
                        }
                        confirmation_dialog_delete.close();
                    }

                    match notebook_chooser_tool_children_names.get(notebook_chooser_tool.get_current_page().unwrap() as usize).unwrap().as_str() {
                        "notebook_duplicate_finder_label" => {
                            let tree_view = scrolled_window_duplicate_finder.get_children().get(0).unwrap().clone().downcast::<gtk::TreeView>().unwrap();
                            let selection = tree_view.get_selection();

                            let (selection_rows, tree_model) = selection.get_selected_rows();
                            let list_store = tree_model.clone().downcast::<gtk::ListStore>().unwrap();

                            // let new_tree_model = TreeModel::new(); // TODO - maybe create new model when inserting a new data, because this seems to be not optimal when using thousands of rows

                            let mut messages: String = "".to_string();

                            // Must be deleted from end to start, because when deleting entries, TreePath(and also TreeIter) will points to invalid data
                            for tree_path in selection_rows.iter().rev() {
                                let name = tree_model.get_value(&tree_model.get_iter(&tree_path).unwrap(), ColumnsDuplicates::Name as i32).get::<String>().unwrap().unwrap();
                                let path = tree_model.get_value(&tree_model.get_iter(&tree_path).unwrap(), ColumnsDuplicates::Path as i32).get::<String>().unwrap().unwrap();

                                match fs::remove_file(format!("{}/{}", path, name)) {
                                    Ok(_) => {
                                        list_store.remove(&list_store.get_iter(&tree_path).unwrap());
                                    }
                                    Err(_) => messages += format!("Failed to remove file {}/{} because file doesn't exists or you don't have permissions.\n", path, name).as_str(),
                                }
                            }

                            text_view_errors.get_buffer().unwrap().set_text(messages.as_str());
                            selection.unselect_all();
                        }
                        "scrolled_window_empty_folder_finder" => {
                            let tree_view = scrolled_window_empty_folder_finder.get_children().get(0).unwrap().clone().downcast::<gtk::TreeView>().unwrap();
                            let selection = tree_view.get_selection();

                            let (selection_rows, tree_model) = selection.get_selected_rows();
                            let list_store = tree_model.clone().downcast::<gtk::ListStore>().unwrap();

                            // let new_tree_model = TreeModel::new(); // TODO - maybe create new model when inserting a new data, because this seems to be not optimal when using thousands of rows

                            let mut messages: String = "".to_string();

                            // Must be deleted from end to start, because when deleting entries, TreePath(and also TreeIter) will points to invalid data
                            for tree_path in selection_rows.iter().rev() {
                                let name = tree_model.get_value(&tree_model.get_iter(&tree_path).unwrap(), ColumnsEmptyFolders::Name as i32).get::<String>().unwrap().unwrap();
                                let path = tree_model.get_value(&tree_model.get_iter(&tree_path).unwrap(), ColumnsEmptyFolders::Path as i32).get::<String>().unwrap().unwrap();

                                match fs::remove_dir(format!("{}/{}", path, name)) {
                                    Ok(_) => {
                                        list_store.remove(&list_store.get_iter(&tree_path).unwrap());
                                    }
                                    Err(_) => messages += format!("Failed to remove folder {}/{} because folder doesn't exists, you don't have permissions or isn't empty.\n", path, name).as_str(),
                                }
                            }

                            text_view_errors.get_buffer().unwrap().set_text(messages.as_str());
                            selection.unselect_all();
                        }
                        "scrolled_window_empty_files_finder" => {
                            let tree_view = scrolled_window_empty_files_finder.get_children().get(0).unwrap().clone().downcast::<gtk::TreeView>().unwrap();
                            let selection = tree_view.get_selection();

                            let (selection_rows, tree_model) = selection.get_selected_rows();
                            let list_store = tree_model.clone().downcast::<gtk::ListStore>().unwrap();

                            // let new_tree_model = TreeModel::new(); // TODO - maybe create new model when inserting a new data, because this seems to be not optimal when using thousands of rows

                            let mut messages: String = "".to_string();

                            // Must be deleted from end to start, because when deleting entries, TreePath(and also TreeIter) will points to invalid data
                            for tree_path in selection_rows.iter().rev() {
                                let name = tree_model.get_value(&tree_model.get_iter(&tree_path).unwrap(), ColumnsEmptyFiles::Name as i32).get::<String>().unwrap().unwrap();
                                let path = tree_model.get_value(&tree_model.get_iter(&tree_path).unwrap(), ColumnsEmptyFiles::Path as i32).get::<String>().unwrap().unwrap();

                                match fs::remove_file(format!("{}/{}", path, name)) {
                                    Ok(_) => {
                                        list_store.remove(&list_store.get_iter(&tree_path).unwrap());
                                    }
                                    Err(_) => messages += format!("Failed to remove file {}/{} because file doesn't exists or you don't have permissions.\n", path, name).as_str(),
                                }
                            }

                            text_view_errors.get_buffer().unwrap().set_text(messages.as_str());
                            selection.unselect_all();
                        }
                        "scrolled_window_temporary_files_finder" => {
                            let tree_view = scrolled_window_temporary_files_finder.get_children().get(0).unwrap().clone().downcast::<gtk::TreeView>().unwrap();
                            let selection = tree_view.get_selection();

                            let (selection_rows, tree_model) = selection.get_selected_rows();
                            let list_store = tree_model.clone().downcast::<gtk::ListStore>().unwrap();

                            // let new_tree_model = TreeModel::new(); // TODO - maybe create new model when inserting a new data, because this seems to be not optimal when using thousands of rows

                            let mut messages: String = "".to_string();

                            // Must be deleted from end to start, because when deleting entries, TreePath(and also TreeIter) will points to invalid data
                            for tree_path in selection_rows.iter().rev() {
                                let name = tree_model.get_value(&tree_model.get_iter(&tree_path).unwrap(), ColumnsTemporaryFiles::Name as i32).get::<String>().unwrap().unwrap();
                                let path = tree_model.get_value(&tree_model.get_iter(&tree_path).unwrap(), ColumnsTemporaryFiles::Path as i32).get::<String>().unwrap().unwrap();

                                match fs::remove_file(format!("{}/{}", path, name)) {
                                    Ok(_) => {
                                        list_store.remove(&list_store.get_iter(&tree_path).unwrap());
                                    }
                                    Err(_) => messages += format!("Failed to remove file {}/{} because file doesn't exists or you don't have permissions.\n", path, name).as_str(),
                                }
                            }

                            text_view_errors.get_buffer().unwrap().set_text(messages.as_str());
                            selection.unselect_all();
                        }
                        "notebook_big_file_finder" => {
                            let tree_view = scrolled_window_big_files_finder.get_children().get(0).unwrap().clone().downcast::<gtk::TreeView>().unwrap();
                            let selection = tree_view.get_selection();

                            let (selection_rows, tree_model) = selection.get_selected_rows();
                            let list_store = tree_model.clone().downcast::<gtk::ListStore>().unwrap();

                            // let new_tree_model = TreeModel::new(); // TODO - maybe create new model when inserting a new data, because this seems to be not optimal when using thousands of rows

                            let mut messages: String = "".to_string();

                            // Must be deleted from end to start, because when deleting entries, TreePath(and also TreeIter) will points to invalid data
                            for tree_path in selection_rows.iter().rev() {
                                let name = tree_model.get_value(&tree_model.get_iter(&tree_path).unwrap(), ColumnsBigFiles::Name as i32).get::<String>().unwrap().unwrap();
                                let path = tree_model.get_value(&tree_model.get_iter(&tree_path).unwrap(), ColumnsBigFiles::Path as i32).get::<String>().unwrap().unwrap();

                                match fs::remove_file(format!("{}/{}", path, name)) {
                                    Ok(_) => {
                                        list_store.remove(&list_store.get_iter(&tree_path).unwrap());
                                    }
                                    Err(_) => messages += format!("Failed to remove file {}/{} because file doesn't exists or you don't have permissions.\n", path, name).as_str(),
                                }
                            }

                            text_view_errors.get_buffer().unwrap().set_text(messages.as_str());
                            selection.unselect_all();
                        }
                        e => panic!("Not existent {}", e),
                    }
                });
            }
            // Select button
            {
                let notebook_chooser_tool_children_names = notebook_chooser_tool_children_names.clone();
                let notebook_chooser_tool = notebook_chooser_tool.clone();
                let buttons_select_clone = buttons_select.clone();
                let popover_select = popover_select.clone();
                buttons_select_clone.connect_clicked(move |_| match notebook_chooser_tool_children_names.get(notebook_chooser_tool.get_current_page().unwrap() as usize).unwrap().as_str() {
                    "notebook_duplicate_finder_label" => {
                        // Only popup popup
                        popover_select.set_relative_to(Some(&buttons_select));
                        popover_select.popup();
                    }
                    "scrolled_window_empty_folder_finder" => {
                        // Do nothing
                    }
                    "scrolled_window_empty_files_finder" => {
                        // Do nothing
                    }
                    "scrolled_window_temporary_files_finder" => {
                        // Do nothing
                    }
                    "notebook_big_file_finder" => {
                        // Do nothing
                    }
                    e => panic!("Not existent {}", e),
                });
            }
            // Save button
            {
                let buttons_save_clone = buttons_save.clone();
                buttons_save_clone.connect_clicked(move |_| match notebook_chooser_tool_children_names.get(notebook_chooser_tool.get_current_page().unwrap() as usize).unwrap().as_str() {
                    "notebook_duplicate_finder_label" => {
                        let file_name = "results_duplicates.txt";

                        let mut df = shared_duplication_state.borrow_mut();
                        df.save_results_to_file(file_name);

                        entry_info.set_text(format!("Saved results to file {}", file_name).as_str());
                        // Set state
                        {
                            buttons_save.hide();
                            *shared_buttons.borrow_mut().get_mut("duplicate").unwrap().get_mut("save").unwrap() = false;
                        }
                    }
                    "scrolled_window_empty_folder_finder" => {
                        let file_name = "results_empty_folder.txt";

                        let mut ef = shared_empty_folders_state.borrow_mut();
                        ef.save_results_to_file(file_name);

                        entry_info.set_text(format!("Saved results to file {}", file_name).as_str());
                        // Set state
                        {
                            buttons_save.hide();
                            *shared_buttons.borrow_mut().get_mut("empty_folder").unwrap().get_mut("save").unwrap() = false;
                        }
                    }
                    "scrolled_window_empty_files_finder" => {
                        let file_name = "results_empty_files.txt";

                        let mut df = shared_empty_files_state.borrow_mut();
                        df.save_results_to_file(file_name);

                        entry_info.set_text(format!("Saved results to file {}", file_name).as_str());
                        // Set state
                        {
                            buttons_save.hide();
                            *shared_buttons.borrow_mut().get_mut("empty_file").unwrap().get_mut("save").unwrap() = false;
                        }
                    }
                    "scrolled_window_temporary_files_finder" => {
                        let file_name = "results_temporary_files.txt";

                        let mut df = shared_temporary_files_state.borrow_mut();
                        df.save_results_to_file(file_name);

                        entry_info.set_text(format!("Saved results to file {}", file_name).as_str());
                        // Set state
                        {
                            buttons_save.hide();
                            *shared_buttons.borrow_mut().get_mut("temporary_file").unwrap().get_mut("save").unwrap() = false;
                        }
                    }
                    "notebook_big_file_finder" => {
                        let file_name = "results_big_files.txt";

                        let mut df = shared_big_files_state.borrow_mut();
                        df.save_results_to_file(file_name);

                        entry_info.set_text(format!("Saved results to file {}", file_name).as_str());
                        // Set state
                        {
                            buttons_save.hide();
                            *shared_buttons.borrow_mut().get_mut("big_file").unwrap().get_mut("save").unwrap() = false;
                        }
                    }
                    e => panic!("Not existent {}", e),
                });
            }
        }
        // Popover Buttons
        {
            // Select all button
            {
                let scrolled_window_duplicate_finder = scrolled_window_duplicate_finder.clone();
                let popover_select = popover_select.clone();
                buttons_popover_select_all.connect_clicked(move |_| {
                    let tree_view = scrolled_window_duplicate_finder.get_children().get(0).unwrap().clone().downcast::<gtk::TreeView>().unwrap();
                    let selection = tree_view.get_selection();

                    selection.select_all();
                    popover_select.popdown();
                });
            }

            // Unselect all button
            {
                let scrolled_window_duplicate_finder = scrolled_window_duplicate_finder.clone();
                let popover_select = popover_select.clone();
                buttons_popover_unselect_all.connect_clicked(move |_| {
                    let tree_view = scrolled_window_duplicate_finder.get_children().get(0).unwrap().clone().downcast::<gtk::TreeView>().unwrap();
                    let selection = tree_view.get_selection();

                    selection.unselect_all();
                    popover_select.popdown();
                });
            }

            // Reverse selection
            {
                let scrolled_window_duplicate_finder = scrolled_window_duplicate_finder.clone();
                let popover_select = popover_select.clone();
                buttons_popover_reverse.connect_clicked(move |_| {
                    let tree_view = scrolled_window_duplicate_finder.get_children().get(0).unwrap().clone().downcast::<gtk::TreeView>().unwrap();
                    let selection = tree_view.get_selection();

                    let (vector_tree_path, tree_model) = selection.get_selected_rows();

                    if vector_tree_path.is_empty() {
                        selection.select_all();
                    } else {
                        let tree_iter_all = tree_model.get_iter_first().unwrap(); // Never should be available button where there is no available records

                        let mut current_path_index = 0;
                        let mut tree_iter_selected: TreeIter;
                        loop {
                            if current_path_index >= vector_tree_path.len() {
                                selection.select_iter(&tree_iter_all);
                            } else {
                                tree_iter_selected = tree_model.get_iter(vector_tree_path.get(current_path_index).unwrap()).unwrap();
                                if tree_model.get_path(&tree_iter_all).unwrap() == tree_model.get_path(&tree_iter_selected).unwrap() {
                                    selection.unselect_iter(&tree_iter_selected);
                                    current_path_index += 1;
                                } else {
                                    selection.select_iter(&tree_iter_all);
                                }
                            }
                            if !tree_model.iter_next(&tree_iter_all) {
                                break;
                            }
                        }
                    }

                    popover_select.popdown();
                });
            }

            // All except oldest
            {
                let scrolled_window_duplicate_finder = scrolled_window_duplicate_finder.clone();
                let popover_select = popover_select.clone();
                buttons_popover_select_all_except_oldest.connect_clicked(move |_| {
                    let tree_view = scrolled_window_duplicate_finder.get_children().get(0).unwrap().clone().downcast::<gtk::TreeView>().unwrap();
                    let selection = tree_view.get_selection();
                    let tree_model = tree_view.get_model().unwrap();

                    let tree_iter_all = tree_model.get_iter_first().unwrap(); // Never should be available button where there is no available records

                    let mut end: bool = false;

                    loop {
                        let mut tree_iter_array: Vec<TreeIter> = Vec::new();
                        let mut oldest_index: Option<usize> = None;
                        let mut current_index: usize = 0;
                        let mut oldest_modification_time: u64 = u64::max_value();

                        loop {
                            let color = tree_model.get_value(&tree_iter_all, ColumnsDuplicates::Color as i32).get::<String>().unwrap().unwrap();
                            if color == HEADER_ROW_COLOR {
                                if !tree_model.iter_next(&tree_iter_all) {
                                    end = true;
                                }
                                break;
                            }
                            tree_iter_array.push(tree_iter_all.clone());
                            let modification = tree_model.get_value(&tree_iter_all, ColumnsDuplicates::ModificationAsSecs as i32).get::<u64>().unwrap().unwrap();
                            if modification < oldest_modification_time {
                                oldest_modification_time = modification;
                                oldest_index = Some(current_index);
                            }

                            current_index += 1;

                            if !tree_model.iter_next(&tree_iter_all) {
                                end = true;
                                break;
                            }
                        }
                        if oldest_index == None {
                            continue;
                        }
                        for (index, tree_iter) in tree_iter_array.iter().enumerate() {
                            if index != oldest_index.unwrap() {
                                selection.select_iter(tree_iter);
                            } else {
                                selection.unselect_iter(tree_iter);
                            }
                        }

                        if end {
                            break;
                        }
                    }

                    popover_select.popdown();
                });
            }

            // All except newest
            {
                let scrolled_window_duplicate_finder = scrolled_window_duplicate_finder.clone();
                let popover_select = popover_select.clone();
                buttons_popover_select_all_except_newest.connect_clicked(move |_| {
                    let tree_view = scrolled_window_duplicate_finder.get_children().get(0).unwrap().clone().downcast::<gtk::TreeView>().unwrap();
                    let selection = tree_view.get_selection();
                    let tree_model = tree_view.get_model().unwrap();

                    let tree_iter_all = tree_model.get_iter_first().unwrap(); // Never should be available button where there is no available records

                    let mut end: bool = false;

                    loop {
                        let mut tree_iter_array: Vec<TreeIter> = Vec::new();
                        let mut newest_index: Option<usize> = None;
                        let mut current_index: usize = 0;
                        let mut newest_modification_time: u64 = 0;

                        loop {
                            let color = tree_model.get_value(&tree_iter_all, ColumnsDuplicates::Color as i32).get::<String>().unwrap().unwrap();
                            if color == HEADER_ROW_COLOR {
                                if !tree_model.iter_next(&tree_iter_all) {
                                    end = true;
                                }
                                break;
                            }
                            tree_iter_array.push(tree_iter_all.clone());
                            let modification = tree_model.get_value(&tree_iter_all, ColumnsDuplicates::ModificationAsSecs as i32).get::<u64>().unwrap().unwrap();
                            if modification > newest_modification_time {
                                newest_modification_time = modification;
                                newest_index = Some(current_index);
                            }

                            current_index += 1;

                            if !tree_model.iter_next(&tree_iter_all) {
                                end = true;
                                break;
                            }
                        }
                        if newest_index == None {
                            continue;
                        }
                        for (index, tree_iter) in tree_iter_array.iter().enumerate() {
                            if index != newest_index.unwrap() {
                                selection.select_iter(tree_iter);
                            } else {
                                selection.unselect_iter(tree_iter);
                            }
                        }

                        if end {
                            break;
                        }
                    }

                    popover_select.popdown();
                });
            }

            // All one oldest
            {
                let scrolled_window_duplicate_finder = scrolled_window_duplicate_finder.clone();
                let popover_select = popover_select.clone();
                buttons_popover_select_one_oldest.connect_clicked(move |_| {
                    let tree_view = scrolled_window_duplicate_finder.get_children().get(0).unwrap().clone().downcast::<gtk::TreeView>().unwrap();
                    let selection = tree_view.get_selection();
                    let tree_model = tree_view.get_model().unwrap();

                    let tree_iter_all = tree_model.get_iter_first().unwrap(); // Never should be available button where there is no available records

                    let mut end: bool = false;

                    loop {
                        let mut tree_iter_array: Vec<TreeIter> = Vec::new();
                        let mut oldest_index: Option<usize> = None;
                        let mut current_index: usize = 0;
                        let mut oldest_modification_time: u64 = u64::max_value();

                        loop {
                            let color = tree_model.get_value(&tree_iter_all, ColumnsDuplicates::Color as i32).get::<String>().unwrap().unwrap();
                            if color == HEADER_ROW_COLOR {
                                if !tree_model.iter_next(&tree_iter_all) {
                                    end = true;
                                }
                                break;
                            }
                            tree_iter_array.push(tree_iter_all.clone());
                            let modification = tree_model.get_value(&tree_iter_all, ColumnsDuplicates::ModificationAsSecs as i32).get::<u64>().unwrap().unwrap();
                            if modification < oldest_modification_time {
                                oldest_modification_time = modification;
                                oldest_index = Some(current_index);
                            }

                            current_index += 1;

                            if !tree_model.iter_next(&tree_iter_all) {
                                end = true;
                                break;
                            }
                        }
                        if oldest_index == None {
                            continue;
                        }
                        for (index, tree_iter) in tree_iter_array.iter().enumerate() {
                            if index == oldest_index.unwrap() {
                                selection.select_iter(tree_iter);
                            } else {
                                selection.unselect_iter(tree_iter);
                            }
                        }

                        if end {
                            break;
                        }
                    }

                    popover_select.popdown();
                });
            }
            // All one newest
            {
                // let scrolled_window_duplicate_finder = scrolled_window_duplicate_finder.clone();
                // let popover_select = popover_select.clone();
                buttons_popover_select_one_newest.connect_clicked(move |_| {
                    let tree_view = scrolled_window_duplicate_finder.get_children().get(0).unwrap().clone().downcast::<gtk::TreeView>().unwrap();
                    let selection = tree_view.get_selection();
                    let tree_model = tree_view.get_model().unwrap();

                    let tree_iter_all = tree_model.get_iter_first().unwrap(); // Never should be available button where there is no available records

                    let mut end: bool = false;

                    loop {
                        let mut tree_iter_array: Vec<TreeIter> = Vec::new();
                        let mut newest_index: Option<usize> = None;
                        let mut current_index: usize = 0;
                        let mut newest_modification_time: u64 = 0;

                        loop {
                            let color = tree_model.get_value(&tree_iter_all, ColumnsDuplicates::Color as i32).get::<String>().unwrap().unwrap();
                            if color == HEADER_ROW_COLOR {
                                if !tree_model.iter_next(&tree_iter_all) {
                                    end = true;
                                }
                                break;
                            }
                            tree_iter_array.push(tree_iter_all.clone());
                            let modification = tree_model.get_value(&tree_iter_all, ColumnsDuplicates::ModificationAsSecs as i32).get::<u64>().unwrap().unwrap();
                            if modification > newest_modification_time {
                                newest_modification_time = modification;
                                newest_index = Some(current_index);
                            }

                            current_index += 1;

                            if !tree_model.iter_next(&tree_iter_all) {
                                end = true;
                                break;
                            }
                        }
                        if newest_index == None {
                            continue;
                        }
                        for (index, tree_iter) in tree_iter_array.iter().enumerate() {
                            if index == newest_index.unwrap() {
                                selection.select_iter(tree_iter);
                            } else {
                                selection.unselect_iter(tree_iter);
                            }
                        }

                        if end {
                            break;
                        }
                    }

                    popover_select.popdown();
                });
            }
        }
        // Upper Notepad
        {
            // Add included directory
            {
                let scrolled_window_included_directories = scrolled_window_included_directories.clone();
                let main_window = main_window.clone();
                buttons_add_included_directory.connect_clicked(move |_| {
                    let chooser = gtk::FileChooserDialog::with_buttons(
                        Option::from("Folders to include"),
                        Option::from(&main_window),
                        gtk::FileChooserAction::SelectFolder,
                        &[("Ok", gtk::ResponseType::Ok), ("Close", gtk::ResponseType::Cancel)],
                    );
                    chooser.show_all();
                    let response_type = chooser.run();
                    if response_type == gtk::ResponseType::Ok {
                        let folder = chooser.get_filename().unwrap().to_str().unwrap().to_string();

                        let tree_view = scrolled_window_included_directories.get_children().get(0).unwrap().clone().downcast::<gtk::TreeView>().unwrap();
                        let list_store = tree_view.get_model().unwrap().downcast::<gtk::ListStore>().unwrap();

                        let col_indices = [0];

                        let values: [&dyn ToValue; 1] = [&folder];
                        list_store.set(&list_store.append(), &col_indices, &values);
                    }
                    chooser.close();
                });
            }
            // Add excluded directory
            {
                let scrolled_window_excluded_directories = scrolled_window_excluded_directories.clone();
                let main_window = main_window.clone();
                buttons_add_excluded_directory.connect_clicked(move |_| {
                    let chooser = gtk::FileChooserDialog::with_buttons(
                        Option::from("Folders to exclude"),
                        Option::from(&main_window),
                        gtk::FileChooserAction::SelectFolder,
                        &[("Ok", gtk::ResponseType::Ok), ("Close", gtk::ResponseType::Cancel)],
                    );
                    chooser.show_all();
                    let response_type = chooser.run();
                    if response_type == gtk::ResponseType::Ok {
                        let folder = chooser.get_filename().unwrap().to_str().unwrap().to_string();

                        let tree_view = scrolled_window_excluded_directories.get_children().get(0).unwrap().clone().downcast::<gtk::TreeView>().unwrap();
                        let list_store = tree_view.get_model().unwrap().downcast::<gtk::ListStore>().unwrap();

                        let col_indices = [0];

                        let values: [&dyn ToValue; 1] = [&folder];
                        list_store.set(&list_store.append(), &col_indices, &values);
                    }
                    chooser.close();
                });
            }
            // Remove Excluded Folder
            {
                //let scrolled_window_excluded_directories = scrolled_window_excluded_directories.clone();
                buttons_remove_excluded_directory.connect_clicked(move |_| {
                    let tree_view = scrolled_window_excluded_directories.get_children().get(0).unwrap().clone().downcast::<gtk::TreeView>().unwrap();
                    let list_store = tree_view.get_model().unwrap().downcast::<gtk::ListStore>().unwrap();
                    let selection = tree_view.get_selection();

                    let (_, tree_iter) = match selection.get_selected() {
                        Some(t) => t,
                        None => {
                            return;
                        }
                    };
                    list_store.remove(&tree_iter);
                });
            }
            // Remove Included Folder
            {
                //let scrolled_window_included_directories = scrolled_window_included_directories.clone();
                buttons_remove_included_directory.connect_clicked(move |_| {
                    let tree_view = scrolled_window_included_directories.get_children().get(0).unwrap().clone().downcast::<gtk::TreeView>().unwrap();
                    let list_store = tree_view.get_model().unwrap().downcast::<gtk::ListStore>().unwrap();
                    let selection = tree_view.get_selection();

                    let (_, tree_iter) = match selection.get_selected() {
                        Some(t) => t,
                        None => {
                            return;
                        }
                    };
                    list_store.remove(&tree_iter);
                });
            }
        }
    }

    // Quit the program when X in main window was clicked
    main_window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    // We start the gtk main loop.
    gtk::main();
}
