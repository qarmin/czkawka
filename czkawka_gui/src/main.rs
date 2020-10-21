mod create_tree_view;
mod double_click_opening;
mod help_functions;

use czkawka_core::*;
use humansize::{file_size_opts as options, FileSize};

// TODO split it across multiple files

extern crate gtk;
use crate::create_tree_view::*;
use crate::double_click_opening::*;
use crate::help_functions::*;
use chrono::NaiveDateTime;
use crossbeam_channel::unbounded;
use czkawka_core::big_file::BigFile;
use czkawka_core::common_traits::SaveResults;
use czkawka_core::duplicate::{CheckingMethod, DuplicateFinder};
use czkawka_core::empty_files::EmptyFiles;
use czkawka_core::empty_folder::EmptyFolder;
use czkawka_core::similar_files::SimilarImages;
use czkawka_core::temporary::Temporary;
use gtk::prelude::*;
use gtk::{Builder, SelectionMode, TreeIter, TreeView};
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::Metadata;
use std::path::Path;
use std::rc::Rc;
use std::{env, fs, process, thread};

fn split_path(path: &Path) -> (String, String) {
    match (path.parent(), path.file_name()) {
        (Some(dir), Some(file)) => (dir.display().to_string(), file.to_string_lossy().into_owned()),
        (Some(dir), None) => (dir.display().to_string(), String::new()),
        (None, _) => (String::new(), String::new()),
    }
}

fn main() {
    let mut exit_program_after_initialization: bool = false;
    // Printing version
    {
        let all_arguments: Vec<String> = env::args().skip(1).collect(); // Not need to check program name

        for i in all_arguments {
            if i == "-v" || i == "--version" {
                println!("Czkawka GUI {}", CZKAWKA_VERSION);
                process::exit(0);
            }
            if i == "-q" || i == "--quit" {
                exit_program_after_initialization = true;
            }
        }
    }

    gtk::init().expect("Failed to initialize GTK.");

    //// Loading glade file content and build with it help UI
    let glade_src = include_str!("../czkawka.glade");
    let builder = Builder::from_string(glade_src);

    //// Windows
    let window_main: gtk::Window = builder.get_object("window_main").unwrap();
    window_main.show_all();
    window_main.set_title("Czkawka");

    ////////////////////////////////////////////////////////////////////////////////////////////////
    //// States
    let main_notebooks_labels = ["duplicate", "empty_folder", "empty_file", "temporary_file", "big_file", "similar_images"];
    let upper_notebooks_labels = [/*"general",*/ "included_directories", "excluded_directories", "excluded_items", "allowed_extensions"];
    let buttons_labels = ["search", "stop", "resume", "pause", "select", "delete", "save"];

    // Buttons State - to remember existence of different buttons on pages

    let shared_buttons: Rc<RefCell<_>> = Rc::new(RefCell::new(HashMap::<String, HashMap<String, bool>>::new()));
    shared_buttons.borrow_mut().clear();

    // Show by default only search button
    for i in main_notebooks_labels.iter() {
        let mut temp_hashmap: HashMap<String, bool> = Default::default();
        for j in buttons_labels.iter() {
            if *j == "search" {
                temp_hashmap.insert(j.to_string(), true);
            } else {
                temp_hashmap.insert(j.to_string(), false);
            }
        }
        shared_buttons.borrow_mut().insert(i.to_string(), temp_hashmap);
    }

    // Upper Notebook state
    let shared_upper_notebooks: Rc<RefCell<_>> = Rc::new(RefCell::new(HashMap::<String, HashMap<String, bool>>::new()));

    for i in main_notebooks_labels.iter() {
        let mut temp_hashmap: HashMap<String, bool> = Default::default();
        for j in upper_notebooks_labels.iter() {
            temp_hashmap.insert(j.to_string(), true);
        }
        shared_upper_notebooks.borrow_mut().insert(i.to_string(), temp_hashmap);
    }
    // Some upper notebook tabs are disabled
    *shared_upper_notebooks.borrow_mut().get_mut("empty_file").unwrap().get_mut("allowed_extensions").unwrap() = false;
    *shared_upper_notebooks.borrow_mut().get_mut("temporary_file").unwrap().get_mut("allowed_extensions").unwrap() = false;

    // State of search results

    let shared_duplication_state: Rc<RefCell<_>> = Rc::new(RefCell::new(DuplicateFinder::new()));
    let shared_empty_folders_state: Rc<RefCell<_>> = Rc::new(RefCell::new(EmptyFolder::new()));
    let shared_empty_files_state: Rc<RefCell<_>> = Rc::new(RefCell::new(EmptyFiles::new()));
    let shared_temporary_files_state: Rc<RefCell<_>> = Rc::new(RefCell::new(Temporary::new()));
    let shared_big_files_state: Rc<RefCell<_>> = Rc::new(RefCell::new(BigFile::new()));
    let shared_similar_images_state: Rc<RefCell<_>> = Rc::new(RefCell::new(SimilarImages::new()));

    // State of confirmation dialogs
    let shared_confirmation_dialog_delete_dialog_showing_state: Rc<RefCell<_>> = Rc::new(RefCell::new(true));

    ////////////////////////////////////////////////////////////////////////////////////////////////

    //// GUI Entry
    let entry_similar_images_minimal_size: gtk::Entry = builder.get_object("entry_similar_images_minimal_size").unwrap();
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

    let buttons_names = ["search", "stop", "resume", "pause", "select", "delete", "save"];
    let buttons_array = [
        buttons_search.clone(),
        buttons_stop.clone(),
        buttons_resume.clone(),
        buttons_pause.clone(),
        buttons_select.clone(),
        buttons_delete.clone(),
        buttons_save.clone(),
    ];

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
    let notebook_main: gtk::Notebook = builder.get_object("notebook_main").unwrap();
    let notebook_upper: gtk::Notebook = builder.get_object("notebook_upper").unwrap();

    let mut notebook_main_children_names: Vec<String> = Vec::new();
    let mut notebook_upper_children_names: Vec<String> = Vec::new();

    for i in notebook_main.get_children() {
        notebook_main_children_names.push(i.get_buildable_name().unwrap().to_string());
    }
    for i in notebook_upper.get_children() {
        notebook_upper_children_names.push(i.get_buildable_name().unwrap().to_string());
    }

    //// Entry
    let entry_info: gtk::Entry = builder.get_object("entry_info").unwrap(); // To show default

    //// Text View
    let text_view_errors: gtk::TextView = builder.get_object("text_view_errors").unwrap();

    //// Scrolled windows
    // Main notebook
    let scrolled_window_duplicate_finder: gtk::ScrolledWindow = builder.get_object("scrolled_window_duplicate_finder").unwrap();
    let scrolled_window_main_empty_folder_finder: gtk::ScrolledWindow = builder.get_object("scrolled_window_main_empty_folder_finder").unwrap();
    let scrolled_window_main_empty_files_finder: gtk::ScrolledWindow = builder.get_object("scrolled_window_main_empty_files_finder").unwrap();
    let scrolled_window_main_temporary_files_finder: gtk::ScrolledWindow = builder.get_object("scrolled_window_main_temporary_files_finder").unwrap();
    let scrolled_window_big_files_finder: gtk::ScrolledWindow = builder.get_object("scrolled_window_big_files_finder").unwrap();
    let scrolled_window_similar_images_finder: gtk::ScrolledWindow = builder.get_object("scrolled_window_similar_images_finder").unwrap();

    // Upper notebook
    let scrolled_window_included_directories: gtk::ScrolledWindow = builder.get_object("scrolled_window_included_directories").unwrap();
    let scrolled_window_excluded_directories: gtk::ScrolledWindow = builder.get_object("scrolled_window_excluded_directories").unwrap();

    //// Threads
    // Types of messages to send to main thread where gui can be draw.
    enum Message {
        Duplicates(DuplicateFinder),
        EmptyFolders(EmptyFolder),
        EmptyFiles(EmptyFiles),
        BigFiles(BigFile),
        Temporary(Temporary),
        SimilarImages(SimilarImages),
    }

    // Used for getting data from thread
    let (sender, receiver) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

    // Used for sending stop signal to thread
    let (sx, rx): (crossbeam_channel::Sender<()>, crossbeam_channel::Receiver<()>) = unbounded();

    //// Setup default look(duplicate finder)
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

        // Set Main Scrolled Window Treeviews
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
                tree_view.get_selection().set_select_function(Some(Box::new(select_function_duplicates)));

                create_tree_view_duplicates(&mut tree_view);

                tree_view.connect_button_press_event(opening_double_click_function_duplicates);

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

                tree_view.connect_button_press_event(opening_double_click_function_empty_folders);

                scrolled_window_main_empty_folder_finder.add(&tree_view);
                scrolled_window_main_empty_folder_finder.show_all();
            }
            // Empty Files
            {
                let col_types: [glib::types::Type; 3] = [glib::types::Type::String, glib::types::Type::String, glib::types::Type::String];
                let list_store: gtk::ListStore = gtk::ListStore::new(&col_types);

                let mut tree_view: gtk::TreeView = TreeView::with_model(&list_store);

                tree_view.get_selection().set_mode(SelectionMode::Multiple);

                create_tree_view_empty_files(&mut tree_view);

                tree_view.connect_button_press_event(opening_double_click_function_empty_files);

                scrolled_window_main_empty_files_finder.add(&tree_view);
                scrolled_window_main_empty_files_finder.show_all();
            }
            // Temporary Files
            {
                let col_types: [glib::types::Type; 3] = [glib::types::Type::String, glib::types::Type::String, glib::types::Type::String];
                let list_store: gtk::ListStore = gtk::ListStore::new(&col_types);

                let mut tree_view: gtk::TreeView = TreeView::with_model(&list_store);

                tree_view.get_selection().set_mode(SelectionMode::Multiple);

                create_tree_view_temporary_files(&mut tree_view);

                tree_view.connect_button_press_event(opening_double_click_function_temporary_files);

                scrolled_window_main_temporary_files_finder.add(&tree_view);
                scrolled_window_main_temporary_files_finder.show_all();
            }
            // Big Files
            {
                let col_types: [glib::types::Type; 4] = [glib::types::Type::String, glib::types::Type::String, glib::types::Type::String, glib::types::Type::String];
                let list_store: gtk::ListStore = gtk::ListStore::new(&col_types);

                let mut tree_view: gtk::TreeView = TreeView::with_model(&list_store);

                tree_view.get_selection().set_mode(SelectionMode::Multiple);

                create_tree_view_big_files(&mut tree_view);

                tree_view.connect_button_press_event(opening_double_click_function_big_files);

                scrolled_window_big_files_finder.add(&tree_view);
                scrolled_window_big_files_finder.show_all();
            }
            // Similar Images
            {
                // TODO create maybe open button to support opening multiple files at once
                let col_types: [glib::types::Type; 8] = [
                    glib::types::Type::String,
                    glib::types::Type::String,
                    glib::types::Type::String,
                    glib::types::Type::String,
                    glib::types::Type::String,
                    glib::types::Type::String,
                    glib::types::Type::String,
                    glib::types::Type::String,
                ];
                let list_store: gtk::ListStore = gtk::ListStore::new(&col_types);

                let mut tree_view: gtk::TreeView = TreeView::with_model(&list_store);

                tree_view.get_selection().set_mode(SelectionMode::Multiple);

                create_tree_view_similar_images(&mut tree_view);

                tree_view.connect_button_press_event(opening_double_click_function_similar_images);

                scrolled_window_similar_images_finder.add(&tree_view);
                scrolled_window_similar_images_finder.show_all();
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
                    if cfg!(target_family = "unix") {
                        println!("Failed to read current directory, setting /home instead");
                        "/home".to_string()
                    } else if cfg!(target_family = "windows") {
                        println!("Failed to read current directory, setting C:\\ instead");
                        "C:\\".to_string()
                    } else {
                        "".to_string()
                    }
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

            for i in ["/proc", "/dev", "/sys", "/run", "/snap"].iter() {
                let values: [&dyn ToValue; 2] = [&i, &(MAIN_ROW_COLOR.to_string())];
                list_store.set(&list_store.append(), &col_indices, &values);
            }

            scrolled_window_excluded_directories.add(&tree_view_excluded_directory);
            scrolled_window_excluded_directories.show_all();
        }
        // Set Excluded Items
        {
            if cfg!(target_family = "unix") {
                entry_excluded_items.set_text("*/.git/*,*/node_modules/*,*/lost+found/*");
            }
            if cfg!(target_family = "windows") {
                entry_excluded_items.set_text("*/.git/*,*/node_modules/*,*/lost+found/*,*:/windows/*");
            }
        }
    }

    // Connecting events
    {
        // Connect Notebook Tabs
        {
            let shared_buttons = shared_buttons.clone();
            let buttons_array = buttons_array.clone();
            let notebook_main_children_names = notebook_main_children_names.clone();
            let notebook_main_clone = notebook_main.clone();

            notebook_main_clone.connect_switch_page(move |_, _, number| {
                let page: &str;
                match notebook_main_children_names.get(number as usize).unwrap().as_str() {
                    "notebook_main_duplicate_finder_label" => {
                        page = "duplicate";
                    }
                    "scrolled_window_main_empty_folder_finder" => {
                        page = "empty_folder";
                    }
                    "scrolled_window_main_empty_files_finder" => page = "empty_file",
                    "scrolled_window_main_temporary_files_finder" => page = "temporary_file",
                    "notebook_big_main_file_finder" => page = "big_file",
                    "notebook_main_similar_images_finder_label" => page = "similar_images",
                    e => {
                        panic!("Not existent page {}", e);
                    }
                };
                // Buttons
                set_buttons(&mut *shared_buttons.borrow_mut().get_mut(page).unwrap(), &buttons_array, &buttons_names);
                // Upper notebook
                {
                    //let upper_notebooks_labels = [/*"general",*/"included_directories","excluded_directories","excluded_items","allowed_extensions"];
                    let mut hashmap: HashMap<&str, &str> = Default::default();
                    //hashmap.insert("notebook_upper_general","general");
                    hashmap.insert("notebook_upper_included_directories", "included_directories");
                    hashmap.insert("notebook_upper_excluded_directories", "excluded_directories");
                    hashmap.insert("notebook_upper_excluded_items", "excluded_items");
                    hashmap.insert("notebook_upper_allowed_extensions", "allowed_extensions");

                    for tab in &notebook_upper_children_names {
                        let name = hashmap.get(tab.as_str()).unwrap().to_string();
                        let index = upper_notebooks_labels.iter().position(|&x| x == name).unwrap();
                        if *shared_upper_notebooks.borrow_mut().get_mut(page).unwrap().get_mut(&name).unwrap() {
                            notebook_upper.get_children().get(index).unwrap().show();
                        } else {
                            notebook_upper.get_children().get(index).unwrap().hide();
                        }
                    }
                }
            });
        }

        // Connect double click

        //// Connect Buttons

        // Main buttons
        {
            // Search button
            {
                let entry_info = entry_info.clone();
                let notebook_main_children_names = notebook_main_children_names.clone();
                let notebook_main = notebook_main.clone();
                let scrolled_window_included_directories = scrolled_window_included_directories.clone();
                let scrolled_window_excluded_directories = scrolled_window_excluded_directories.clone();
                let buttons_search_clone = buttons_search.clone();
                let buttons_array = buttons_array.clone();
                buttons_search_clone.connect_clicked(move |_| {
                    let included_directories = get_string_from_list_store(&scrolled_window_included_directories);
                    let excluded_directories = get_string_from_list_store(&scrolled_window_excluded_directories);
                    let recursive_search = check_button_recursive.get_active();
                    let excluded_items = entry_excluded_items.get_text().as_str().to_string();
                    let allowed_extensions = entry_allowed_extensions.get_text().as_str().to_string();

                    hide_all_buttons_except("stop", &buttons_array, &buttons_names);

                    // Disable main notebook from any iteraction until search will end
                    notebook_main.set_sensitive(false);

                    entry_info.set_text("Searching data, it may take a while, please wait...");

                    match notebook_main_children_names.get(notebook_main.get_current_page().unwrap() as usize).unwrap().as_str() {
                        "notebook_main_duplicate_finder_label" => {
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
                            let minimal_file_size = match entry_duplicate_minimal_size.get_text().as_str().parse::<u64>() {
                                Ok(t) => t,
                                Err(_) => 1024, // By default
                            };
                            let delete_method = duplicate::DeleteMethod::None;

                            let sender = sender.clone();
                            let receiver_stop = rx.clone();
                            // Find duplicates
                            thread::spawn(move || {
                                let mut df = DuplicateFinder::new();
                                df.set_included_directory(included_directories);
                                df.set_excluded_directory(excluded_directories);
                                df.set_recursive_search(recursive_search);
                                df.set_excluded_items(excluded_items);
                                df.set_allowed_extensions(allowed_extensions);
                                df.set_minimal_file_size(minimal_file_size);
                                df.set_check_method(check_method);
                                df.set_delete_method(delete_method);
                                df.find_duplicates(Option::from(&receiver_stop)); //&rc_stop_signal.borrow().1);
                                let _ = sender.send(Message::Duplicates(df));
                            });
                        }
                        "scrolled_window_main_empty_folder_finder" => {
                            let sender = sender.clone();
                            let receiver_stop = rx.clone();

                            // Find empty folders
                            thread::spawn(move || {
                                let mut ef = EmptyFolder::new();
                                ef.set_included_directory(included_directories);
                                ef.set_delete_folder(false);
                                ef.find_empty_folders(Option::from(&receiver_stop));
                                let _ = sender.send(Message::EmptyFolders(ef));
                            });
                        }
                        "scrolled_window_main_empty_files_finder" => {
                            let sender = sender.clone();
                            let receiver_stop = rx.clone();

                            // Find empty files
                            thread::spawn(move || {
                                let mut vf = EmptyFiles::new();

                                vf.set_included_directory(included_directories);
                                vf.set_excluded_directory(excluded_directories);
                                vf.set_recursive_search(recursive_search);
                                vf.set_excluded_items(excluded_items);
                                vf.set_allowed_extensions(allowed_extensions);
                                vf.find_empty_files(Option::from(&receiver_stop));
                                let _ = sender.send(Message::EmptyFiles(vf));
                            });
                        }
                        "scrolled_window_main_temporary_files_finder" => {
                            let sender = sender.clone();
                            let receiver_stop = rx.clone();

                            // Find temporary files
                            thread::spawn(move || {
                                let mut tf = Temporary::new();

                                tf.set_included_directory(included_directories);
                                tf.set_excluded_directory(excluded_directories);
                                tf.set_recursive_search(recursive_search);
                                tf.set_excluded_items(excluded_items);
                                tf.find_temporary_files(Option::from(&receiver_stop));
                                let _ = sender.send(Message::Temporary(tf));
                            });
                        }
                        "notebook_big_main_file_finder" => {
                            let numbers_of_files_to_check = match entry_big_files_number.get_text().as_str().parse::<usize>() {
                                Ok(t) => t,
                                Err(_) => 50, // By default
                            };

                            let sender = sender.clone();
                            let receiver_stop = rx.clone();

                            // Find big files
                            thread::spawn(move || {
                                let mut bf = BigFile::new();

                                bf.set_included_directory(included_directories);
                                bf.set_excluded_directory(excluded_directories);
                                bf.set_recursive_search(recursive_search);
                                bf.set_excluded_items(excluded_items);
                                bf.set_number_of_files_to_check(numbers_of_files_to_check);
                                bf.find_big_files(Option::from(&receiver_stop));
                                let _ = sender.send(Message::BigFiles(bf));
                            });
                        }

                        "notebook_main_similar_images_finder_label" => {
                            let sender = sender.clone();
                            let receiver_stop = rx.clone();

                            let minimal_file_size = match entry_similar_images_minimal_size.get_text().as_str().parse::<u64>() {
                                Ok(t) => t,
                                Err(_) => 1024 * 16, // By default
                            };

                            // Find similar images
                            thread::spawn(move || {
                                let mut sf = SimilarImages::new();

                                sf.set_included_directory(included_directories);
                                sf.set_excluded_directory(excluded_directories);
                                sf.set_recursive_search(recursive_search);
                                sf.set_excluded_items(excluded_items);
                                sf.set_minimal_file_size(minimal_file_size);
                                sf.find_similar_images(Option::from(&receiver_stop));
                                let _ = sender.send(Message::SimilarImages(sf));
                            });
                        }
                        e => panic!("Not existent {}", e),
                    }
                });
            }
            // Delete button
            {
                let scrolled_window_duplicate_finder = scrolled_window_duplicate_finder.clone();
                let text_view_errors = text_view_errors.clone();
                let notebook_main_children_names = notebook_main_children_names.clone();
                let notebook_main = notebook_main.clone();
                let window_main = window_main.clone();
                let scrolled_window_main_empty_folder_finder = scrolled_window_main_empty_folder_finder.clone();
                let scrolled_window_big_files_finder = scrolled_window_big_files_finder.clone();
                let scrolled_window_main_empty_files_finder = scrolled_window_main_empty_files_finder.clone();
                let scrolled_window_main_temporary_files_finder = scrolled_window_main_temporary_files_finder.clone();
                let scrolled_window_similar_images_finder = scrolled_window_similar_images_finder.clone();

                buttons_delete.connect_clicked(move |_| {
                    if *shared_confirmation_dialog_delete_dialog_showing_state.borrow_mut() {
                        let confirmation_dialog_delete = gtk::Dialog::with_buttons(
                            Option::from("Delete confirmation"),
                            Option::from(&window_main),
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

                    match notebook_main_children_names.get(notebook_main.get_current_page().unwrap() as usize).unwrap().as_str() {
                        "notebook_main_duplicate_finder_label" => {
                            let tree_view = scrolled_window_duplicate_finder.get_children().get(0).unwrap().clone().downcast::<gtk::TreeView>().unwrap();
                            let selection = tree_view.get_selection();

                            let (selection_rows, tree_model) = selection.get_selected_rows();
                            if selection_rows.is_empty() {
                                return;
                            }
                            let list_store = tree_model.clone().downcast::<gtk::ListStore>().unwrap();

                            // let new_tree_model = TreeModel::new(); // TODO - maybe create new model when inserting a new data, because this seems to be not optimal when using thousands of rows

                            let mut messages: String = "".to_string();

                            // Must be deleted from end to start, because when deleting entries, TreePath(and also TreeIter) will points to invalid data
                            for tree_path in selection_rows.iter().rev() {
                                let name = tree_model.get_value(&tree_model.get_iter(tree_path).unwrap(), ColumnsDuplicates::Name as i32).get::<String>().unwrap().unwrap();
                                let path = tree_model.get_value(&tree_model.get_iter(tree_path).unwrap(), ColumnsDuplicates::Path as i32).get::<String>().unwrap().unwrap();

                                match fs::remove_file(format!("{}/{}", path, name)) {
                                    Ok(_) => {
                                        list_store.remove(&list_store.get_iter(tree_path).unwrap());
                                    }
                                    Err(_) => messages += format!("Failed to remove file {}/{} because file doesn't exists or you don't have permissions.\n", path, name).as_str(),
                                }
                            }

                            text_view_errors.get_buffer().unwrap().set_text(messages.as_str());
                            selection.unselect_all();
                        }
                        "scrolled_window_main_empty_folder_finder" => {
                            let tree_view = scrolled_window_main_empty_folder_finder.get_children().get(0).unwrap().clone().downcast::<gtk::TreeView>().unwrap();
                            let selection = tree_view.get_selection();

                            let (selection_rows, tree_model) = selection.get_selected_rows();
                            if selection_rows.is_empty() {
                                return;
                            }
                            let list_store = tree_model.clone().downcast::<gtk::ListStore>().unwrap();

                            // let new_tree_model = TreeModel::new(); // TODO - maybe create new model when inserting a new data, because this seems to be not optimal when using thousands of rows

                            let mut messages: String = "".to_string();

                            // Must be deleted from end to start, because when deleting entries, TreePath(and also TreeIter) will points to invalid data
                            for tree_path in selection_rows.iter().rev() {
                                let name = tree_model.get_value(&tree_model.get_iter(tree_path).unwrap(), ColumnsEmptyFolders::Name as i32).get::<String>().unwrap().unwrap();
                                let path = tree_model.get_value(&tree_model.get_iter(tree_path).unwrap(), ColumnsEmptyFolders::Path as i32).get::<String>().unwrap().unwrap();

                                // We must check if folder is really empty or contains only other empty folders
                                let mut error_happened = false;
                                let mut folders_to_check: Vec<String> = vec![format!("{}/{}", path, name)];
                                let mut current_folder: String;
                                let mut next_folder: String;
                                'dir: while !folders_to_check.is_empty() {
                                    current_folder = folders_to_check.pop().unwrap();
                                    let read_dir = match fs::read_dir(&current_folder) {
                                        Ok(t) => t,
                                        Err(_) => {
                                            error_happened = true;
                                            break 'dir;
                                        }
                                    };

                                    for entry in read_dir {
                                        let entry_data = match entry {
                                            Ok(t) => t,
                                            Err(_) => {
                                                error_happened = true;
                                                break 'dir;
                                            }
                                        };
                                        let metadata: Metadata = match entry_data.metadata() {
                                            Ok(t) => t,
                                            Err(_) => {
                                                error_happened = true;
                                                break 'dir;
                                            }
                                        };
                                        if metadata.is_dir() {
                                            next_folder = "".to_owned()
                                                + &current_folder
                                                + "/"
                                                + match &entry_data.file_name().into_string() {
                                                    Ok(t) => t,
                                                    Err(_) => {
                                                        error_happened = true;
                                                        break 'dir;
                                                    }
                                                };
                                            folders_to_check.push(next_folder.clone());
                                        } else {
                                            error_happened = true;
                                        }
                                    }
                                }

                                if !error_happened {
                                    match fs::remove_dir_all(format!("{}/{}", path, name)) {
                                        Ok(_) => {
                                            list_store.remove(&list_store.get_iter(tree_path).unwrap());
                                        }
                                        Err(_) => error_happened = true,
                                    }
                                }
                                if error_happened {
                                    messages += format!("Failed to remove folder {}/{} because folder doesn't exists, you don't have permissions or isn't empty.\n", path, name).as_str()
                                }
                            }

                            text_view_errors.get_buffer().unwrap().set_text(messages.as_str());
                            selection.unselect_all();
                        }
                        "scrolled_window_main_empty_files_finder" => {
                            let tree_view = scrolled_window_main_empty_files_finder.get_children().get(0).unwrap().clone().downcast::<gtk::TreeView>().unwrap();
                            let selection = tree_view.get_selection();

                            let (selection_rows, tree_model) = selection.get_selected_rows();
                            if selection_rows.is_empty() {
                                return;
                            }
                            let list_store = tree_model.clone().downcast::<gtk::ListStore>().unwrap();

                            // let new_tree_model = TreeModel::new(); // TODO - maybe create new model when inserting a new data, because this seems to be not optimal when using thousands of rows

                            let mut messages: String = "".to_string();

                            // Must be deleted from end to start, because when deleting entries, TreePath(and also TreeIter) will points to invalid data
                            for tree_path in selection_rows.iter().rev() {
                                let name = tree_model.get_value(&tree_model.get_iter(tree_path).unwrap(), ColumnsEmptyFiles::Name as i32).get::<String>().unwrap().unwrap();
                                let path = tree_model.get_value(&tree_model.get_iter(tree_path).unwrap(), ColumnsEmptyFiles::Path as i32).get::<String>().unwrap().unwrap();

                                match fs::remove_file(format!("{}/{}", path, name)) {
                                    Ok(_) => {
                                        list_store.remove(&list_store.get_iter(tree_path).unwrap());
                                    }
                                    Err(_) => messages += format!("Failed to remove file {}/{} because file doesn't exists or you don't have permissions.\n", path, name).as_str(),
                                }
                            }

                            text_view_errors.get_buffer().unwrap().set_text(messages.as_str());
                            selection.unselect_all();
                        }
                        "scrolled_window_main_temporary_files_finder" => {
                            let tree_view = scrolled_window_main_temporary_files_finder.get_children().get(0).unwrap().clone().downcast::<gtk::TreeView>().unwrap();
                            let selection = tree_view.get_selection();

                            let (selection_rows, tree_model) = selection.get_selected_rows();
                            if selection_rows.is_empty() {
                                return;
                            }
                            let list_store = tree_model.clone().downcast::<gtk::ListStore>().unwrap();

                            // let new_tree_model = TreeModel::new(); // TODO - maybe create new model when inserting a new data, because this seems to be not optimal when using thousands of rows

                            let mut messages: String = "".to_string();

                            // Must be deleted from end to start, because when deleting entries, TreePath(and also TreeIter) will points to invalid data
                            for tree_path in selection_rows.iter().rev() {
                                let name = tree_model.get_value(&tree_model.get_iter(tree_path).unwrap(), ColumnsTemporaryFiles::Name as i32).get::<String>().unwrap().unwrap();
                                let path = tree_model.get_value(&tree_model.get_iter(tree_path).unwrap(), ColumnsTemporaryFiles::Path as i32).get::<String>().unwrap().unwrap();

                                match fs::remove_file(format!("{}/{}", path, name)) {
                                    Ok(_) => {
                                        list_store.remove(&list_store.get_iter(tree_path).unwrap());
                                    }
                                    Err(_) => messages += format!("Failed to remove file {}/{} because file doesn't exists or you don't have permissions.\n", path, name).as_str(),
                                }
                            }

                            text_view_errors.get_buffer().unwrap().set_text(messages.as_str());
                            selection.unselect_all();
                        }
                        "notebook_big_main_file_finder" => {
                            let tree_view = scrolled_window_big_files_finder.get_children().get(0).unwrap().clone().downcast::<gtk::TreeView>().unwrap();
                            let selection = tree_view.get_selection();

                            let (selection_rows, tree_model) = selection.get_selected_rows();
                            if selection_rows.is_empty() {
                                return;
                            }
                            let list_store = tree_model.clone().downcast::<gtk::ListStore>().unwrap();

                            // let new_tree_model = TreeModel::new(); // TODO - maybe create new model when inserting a new data, because this seems to be not optimal when using thousands of rows

                            let mut messages: String = "".to_string();

                            // Must be deleted from end to start, because when deleting entries, TreePath(and also TreeIter) will points to invalid data
                            for tree_path in selection_rows.iter().rev() {
                                let name = tree_model.get_value(&tree_model.get_iter(tree_path).unwrap(), ColumnsBigFiles::Name as i32).get::<String>().unwrap().unwrap();
                                let path = tree_model.get_value(&tree_model.get_iter(tree_path).unwrap(), ColumnsBigFiles::Path as i32).get::<String>().unwrap().unwrap();

                                match fs::remove_file(format!("{}/{}", path, name)) {
                                    Ok(_) => {
                                        list_store.remove(&list_store.get_iter(tree_path).unwrap());
                                    }
                                    Err(_) => messages += format!("Failed to remove file {}/{} because file doesn't exists or you don't have permissions.\n", path, name).as_str(),
                                }
                            }

                            text_view_errors.get_buffer().unwrap().set_text(messages.as_str());
                            selection.unselect_all();
                        }
                        "notebook_main_similar_images_finder_label" => {
                            let tree_view = scrolled_window_similar_images_finder.get_children().get(0).unwrap().clone().downcast::<gtk::TreeView>().unwrap();
                            let selection = tree_view.get_selection();

                            let (selection_rows, tree_model) = selection.get_selected_rows();
                            if selection_rows.is_empty() {
                                return;
                            }
                            let list_store = tree_model.clone().downcast::<gtk::ListStore>().unwrap();

                            // let new_tree_model = TreeModel::new(); // TODO - maybe create new model when inserting a new data, because this seems to be not optimal when using thousands of rows

                            let mut messages: String = "".to_string();

                            let mut vec_path_to_delete: Vec<(String, String)> = Vec::new();

                            // Just remove file, later must be deleted list entry with all occurencies
                            for tree_path in selection_rows.iter().rev() {
                                let name = tree_model.get_value(&tree_model.get_iter(tree_path).unwrap(), ColumnsSimilarImages::Name as i32).get::<String>().unwrap().unwrap();
                                let path = tree_model.get_value(&tree_model.get_iter(tree_path).unwrap(), ColumnsSimilarImages::Path as i32).get::<String>().unwrap().unwrap();

                                if fs::remove_file(format!("{}/{}", path, name)).is_err() {
                                    messages += format!(
                                        "Failed to remove file {}/{}. It is possible that you already deleted it, because similar images shows all possible file doesn't exists or you don't have permissions.\n",
                                        path, name
                                    )
                                    .as_str()
                                }
                                vec_path_to_delete.push((path, name));
                            }
                            // Must be deleted from end to start, because when deleting entries, TreePath(and also TreeIter) will points to invalid data
                            for path_to_delete in vec_path_to_delete {
                                let mut vec_tree_path_to_delete: Vec<gtk::TreePath> = Vec::new();

                                let iter = match list_store.get_iter_first() {
                                    Some(t) => t,
                                    None => break,
                                };
                                let mut take_child_mode = false; // When original image is searched one, we must remove all occurences of its children
                                let mut prepared_for_delete;
                                loop {
                                    prepared_for_delete = false;
                                    if take_child_mode {
                                        let color = tree_model.get_value(&iter, ColumnsSimilarImages::Color as i32).get::<String>().unwrap().unwrap();
                                        if color == HEADER_ROW_COLOR {
                                            take_child_mode = false;
                                        } else {
                                            prepared_for_delete = true;
                                        }
                                    } else {
                                        let path = tree_model.get_value(&iter, ColumnsSimilarImages::Path as i32).get::<String>().unwrap().unwrap();
                                        if path == path_to_delete.0 {
                                            let name = tree_model.get_value(&iter, ColumnsSimilarImages::Name as i32).get::<String>().unwrap().unwrap();
                                            if name == path_to_delete.1 {
                                                let color = tree_model.get_value(&iter, ColumnsSimilarImages::Color as i32).get::<String>().unwrap().unwrap();
                                                if color == HEADER_ROW_COLOR {
                                                    take_child_mode = true;
                                                }
                                                prepared_for_delete = true;
                                            }
                                        }
                                    }

                                    if prepared_for_delete {
                                        vec_tree_path_to_delete.push(list_store.get_path(&iter).unwrap());
                                    }

                                    if !list_store.iter_next(&iter) {
                                        break;
                                    }
                                }

                                for tree_path in vec_tree_path_to_delete.iter().rev() {
                                    list_store.remove(&list_store.get_iter(&tree_path).unwrap());
                                }
                            }
                            // End run to remove single header rows(without duplicates)
                            if let Some(next_iter) = list_store.get_iter_first() {
                                let mut header_was_before = false;
                                let mut vec_tree_path_to_delete: Vec<gtk::TreePath> = Vec::new();
                                let mut current_iter = next_iter.clone();
                                loop {
                                    let color = tree_model.get_value(&next_iter, ColumnsSimilarImages::Color as i32).get::<String>().unwrap().unwrap();
                                    if color == HEADER_ROW_COLOR {
                                        if header_was_before {
                                            vec_tree_path_to_delete.push(list_store.get_path(&current_iter).unwrap());
                                        } else {
                                            header_was_before = true;
                                        }
                                    } else {
                                        header_was_before = false;
                                    }

                                    current_iter = next_iter.clone();
                                    if !list_store.iter_next(&next_iter) {
                                        break;
                                    }
                                }
                            }

                            // Last step, remove orphan header if exists
                            if let Some(iter) = list_store.get_iter_first() {
                                if !list_store.iter_next(&iter) {
                                    list_store.clear();
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
                let notebook_main_children_names = notebook_main_children_names.clone();
                let notebook_main = notebook_main.clone();
                let buttons_select_clone = buttons_select.clone();
                let popover_select = popover_select.clone();
                buttons_select_clone.connect_clicked(move |_| match notebook_main_children_names.get(notebook_main.get_current_page().unwrap() as usize).unwrap().as_str() {
                    "notebook_main_duplicate_finder_label" => {
                        // Only popup popup
                        popover_select.set_relative_to(Some(&buttons_select));
                        popover_select.popup();
                    }
                    e => panic!("Not existent {}", e),
                });
            }
            // Save button
            {
                let shared_buttons = shared_buttons.clone();
                let buttons_save_clone = buttons_save.clone();
                let entry_info = entry_info.clone();
                let shared_duplication_state = shared_duplication_state.clone();
                let shared_empty_folders_state = shared_empty_folders_state.clone();
                let shared_big_files_state = shared_big_files_state.clone();
                let shared_temporary_files_state = shared_temporary_files_state.clone();
                let shared_empty_files_state = shared_empty_files_state.clone();
                let shared_similar_images_state = shared_similar_images_state.clone();
                let notebook_main = notebook_main.clone();
                buttons_save_clone.connect_clicked(move |_| match notebook_main_children_names.get(notebook_main.get_current_page().unwrap() as usize).unwrap().as_str() {
                    "notebook_main_duplicate_finder_label" => {
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
                    "scrolled_window_main_empty_folder_finder" => {
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
                    "scrolled_window_main_empty_files_finder" => {
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
                    "scrolled_window_main_temporary_files_finder" => {
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
                    "notebook_big_main_file_finder" => {
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
                    "notebook_main_similar_images_finder_label" => {
                        let file_name = "results_similar_images.txt";

                        let mut df = shared_similar_images_state.borrow_mut();
                        df.save_results_to_file(file_name);

                        entry_info.set_text(format!("Saved results to file {}", file_name).as_str());
                        // Set state
                        {
                            buttons_save.hide();
                            *shared_buttons.borrow_mut().get_mut("similar_images").unwrap().get_mut("save").unwrap() = false;
                        }
                    }
                    e => panic!("Not existent {}", e),
                });
            }
            // Stop button
            {
                buttons_stop.connect_clicked(move |_| {
                    sx.send(()).unwrap();
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
                let scrolled_window_duplicate_finder = scrolled_window_duplicate_finder.clone();
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
        // Upper Notebook
        {
            // Add included directory
            {
                let scrolled_window_included_directories = scrolled_window_included_directories.clone();
                let window_main = window_main.clone();
                buttons_add_included_directory.connect_clicked(move |_| {
                    let chooser = gtk::FileChooserDialog::with_buttons(
                        Option::from("Folders to include"),
                        Option::from(&window_main),
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
                let window_main = window_main.clone();
                buttons_add_excluded_directory.connect_clicked(move |_| {
                    let chooser = gtk::FileChooserDialog::with_buttons(
                        Option::from("Folders to exclude"),
                        Option::from(&window_main),
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

    // Wait for ending of search:
    // Unblock left notebook bar
    // Show proper buttons
    receiver.attach(None, move |msg| {
        buttons_search.show();
        buttons_stop.hide();

        // Restore clickability to main notebook
        notebook_main.set_sensitive(true);

        match msg {
            Message::Duplicates(df) => {
                if df.get_stopped_search() {
                    entry_info.set_text("Searching for duplicated was stopped by user");

                    //Also clear list
                    scrolled_window_duplicate_finder
                        .get_children()
                        .get(0)
                        .unwrap()
                        .clone()
                        .downcast::<gtk::TreeView>()
                        .unwrap()
                        .get_model()
                        .unwrap()
                        .downcast::<gtk::ListStore>()
                        .unwrap()
                        .clear();
                } else {
                    let information = df.get_information();
                    let text_messages = df.get_text_messages();

                    let duplicates_number: usize;
                    let duplicates_size: u64;
                    let duplicates_group: usize;

                    match df.get_check_method() {
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

                        match df.get_check_method() {
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
                                        &(vector.len().to_string() + " x " + size.to_string().as_str()),
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

                        if duplicates_size > 0 {
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

                    //Also clear list
                    scrolled_window_main_empty_folder_finder
                        .get_children()
                        .get(0)
                        .unwrap()
                        .clone()
                        .downcast::<gtk::TreeView>()
                        .unwrap()
                        .get_model()
                        .unwrap()
                        .downcast::<gtk::ListStore>()
                        .unwrap()
                        .clear();
                } else {
                    let information = ef.get_information();
                    let text_messages = ef.get_text_messages();

                    let empty_folder_number: usize = information.number_of_empty_folders;

                    entry_info.set_text(format!("Found {} empty folders.", empty_folder_number).as_str());

                    // Create GUI
                    {
                        let list_store = scrolled_window_main_empty_folder_finder
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
                        } else {
                            *shared_buttons.borrow_mut().get_mut("empty_folder").unwrap().get_mut("save").unwrap() = false;
                            *shared_buttons.borrow_mut().get_mut("empty_folder").unwrap().get_mut("delete").unwrap() = false;
                        }
                        set_buttons(&mut *shared_buttons.borrow_mut().get_mut("empty_folder").unwrap(), &buttons_array, &buttons_names);
                    }
                }
            }
            Message::EmptyFiles(vf) => {
                if vf.get_stopped_search() {
                    entry_info.set_text("Searching for empty files was stopped by user");

                    //Also clear list
                    scrolled_window_main_empty_files_finder
                        .get_children()
                        .get(0)
                        .unwrap()
                        .clone()
                        .downcast::<gtk::TreeView>()
                        .unwrap()
                        .get_model()
                        .unwrap()
                        .downcast::<gtk::ListStore>()
                        .unwrap()
                        .clear();
                } else {
                    let information = vf.get_information();
                    let text_messages = vf.get_text_messages();

                    let empty_files_number: usize = information.number_of_empty_files;

                    entry_info.set_text(format!("Found {} empty files.", empty_files_number).as_str());

                    // Create GUI
                    {
                        let list_store = scrolled_window_main_empty_files_finder
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
                        } else {
                            *shared_buttons.borrow_mut().get_mut("empty_file").unwrap().get_mut("save").unwrap() = false;
                            *shared_buttons.borrow_mut().get_mut("empty_file").unwrap().get_mut("delete").unwrap() = false;
                        }
                        set_buttons(&mut *shared_buttons.borrow_mut().get_mut("empty_file").unwrap(), &buttons_array, &buttons_names);
                    }
                }
            }
            Message::BigFiles(bf) => {
                if bf.get_stopped_search() {
                    entry_info.set_text("Searching for big files was stopped by user");

                    //Also clear list
                    scrolled_window_duplicate_finder
                        .get_children()
                        .get(0)
                        .unwrap()
                        .clone()
                        .downcast::<gtk::TreeView>()
                        .unwrap()
                        .get_model()
                        .unwrap()
                        .downcast::<gtk::ListStore>()
                        .unwrap()
                        .clear();
                } else {
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
                        } else {
                            *shared_buttons.borrow_mut().get_mut("big_file").unwrap().get_mut("save").unwrap() = false;
                            *shared_buttons.borrow_mut().get_mut("big_file").unwrap().get_mut("delete").unwrap() = false;
                        }
                        set_buttons(&mut *shared_buttons.borrow_mut().get_mut("big_file").unwrap(), &buttons_array, &buttons_names);
                    }
                }
            }
            Message::Temporary(tf) => {
                if tf.get_stopped_search() {
                    entry_info.set_text("Searching for temporary files was stopped by user");

                    //Also clear list
                    scrolled_window_duplicate_finder
                        .get_children()
                        .get(0)
                        .unwrap()
                        .clone()
                        .downcast::<gtk::TreeView>()
                        .unwrap()
                        .get_model()
                        .unwrap()
                        .downcast::<gtk::ListStore>()
                        .unwrap()
                        .clear();
                } else {
                    let information = tf.get_information();
                    let text_messages = tf.get_text_messages();

                    let temporary_files_number: usize = information.number_of_temporary_files;

                    entry_info.set_text(format!("Found {} temporary files.", temporary_files_number).as_str());

                    // Create GUI
                    {
                        let list_store = scrolled_window_main_temporary_files_finder
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
                        } else {
                            *shared_buttons.borrow_mut().get_mut("temporary_file").unwrap().get_mut("save").unwrap() = false;
                            *shared_buttons.borrow_mut().get_mut("temporary_file").unwrap().get_mut("delete").unwrap() = false;
                        }
                        set_buttons(&mut *shared_buttons.borrow_mut().get_mut("temporary_file").unwrap(), &buttons_array, &buttons_names);
                    }
                }
            }
            Message::SimilarImages(sf) => {
                if sf.get_stopped_search() {
                    entry_info.set_text("Searching for duplicated was stopped by user");

                    //Also clear list
                    scrolled_window_duplicate_finder
                        .get_children()
                        .get(0)
                        .unwrap()
                        .clone()
                        .downcast::<gtk::TreeView>()
                        .unwrap()
                        .get_model()
                        .unwrap()
                        .downcast::<gtk::ListStore>()
                        .unwrap()
                        .clear();
                } else {
                    //let information = sf.get_information();
                    let text_messages = sf.get_text_messages();

                    let base_images_size = sf.get_similar_images().len();

                    entry_info.set_text(format!("Found similar pictures for {} images.", base_images_size).as_str());

                    // Create GUI
                    {
                        let list_store = scrolled_window_similar_images_finder
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

                        let col_indices = [0, 1, 2, 3, 4, 5, 6, 7];

                        let vec_struct_similar = sf.get_similar_images();

                        for struct_similar in vec_struct_similar.iter() {
                            // Header
                            let (directory, file) = split_path(&struct_similar.base_image.path);
                            let values: [&dyn ToValue; 8] = [
                                &(get_text_from_similarity(&struct_similar.base_image.similarity).to_string()),
                                &struct_similar.base_image.size.file_size(options::BINARY).unwrap(),
                                &struct_similar.base_image.dimensions,
                                &file,
                                &directory,
                                &(NaiveDateTime::from_timestamp(struct_similar.base_image.modified_date as i64, 0).to_string()),
                                &(HEADER_ROW_COLOR.to_string()),
                                &(TEXT_COLOR.to_string()),
                            ];
                            list_store.set(&list_store.append(), &col_indices, &values);

                            // Meat
                            for similar_images in &struct_similar.similar_images {
                                let (directory, file) = split_path(&similar_images.path);
                                let values: [&dyn ToValue; 8] = [
                                    &(get_text_from_similarity(&similar_images.similarity).to_string()),
                                    &similar_images.size.file_size(options::BINARY).unwrap(),
                                    &similar_images.dimensions,
                                    &file,
                                    &directory,
                                    &(NaiveDateTime::from_timestamp(similar_images.modified_date as i64, 0).to_string()),
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
                        } else {
                            *shared_buttons.borrow_mut().get_mut("similar_images").unwrap().get_mut("save").unwrap() = false;
                            *shared_buttons.borrow_mut().get_mut("similar_images").unwrap().get_mut("delete").unwrap() = false;
                        }
                        set_buttons(&mut *shared_buttons.borrow_mut().get_mut("similar_images").unwrap(), &buttons_array, &buttons_names);
                    }
                }
            }
        }
        // Returning false here would close the receiver and have senders fail
        glib::Continue(true)
    });

    // Quit the program when X in main window was clicked
    window_main.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    // We start the gtk main loop.
    gtk::main();

    // Quiting if quit flag was provided
    if exit_program_after_initialization {
        gtk::main_quit();
    }
}
