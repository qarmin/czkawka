use czkawka_core::*;
use humansize::{file_size_opts as options, FileSize};

extern crate gtk;
use chrono::NaiveDateTime;
use czkawka_core::common_messages::Messages;
use czkawka_core::duplicate::CheckingMethod;
use czkawka_core::empty_folder::EmptyFolder;
use duplicate::DuplicateFinder;
use gtk::prelude::*;
use gtk::{Builder, SelectionMode, TreeView, TreeViewColumn};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::time::UNIX_EPOCH;
use std::{env, process};

enum ColumnsDefault {
    Name = 0,
    Path,
    Modification,
    Color,
}
enum ColumnsDirectory {
    Path = 0,
    Color,
}

const MAIN_ROW_COLOR: &str = "#343434";
const HEADER_ROW_COLOR: &str = "#272727";
// const MAIN_ROW_COLOR: &str = "#f4f434"; // TEST
// const HEADER_ROW_COLOR: &str = "#010101"; // TEST

fn main() {
    // Printing version
    {
        let all_arguments: Vec<String> = env::args().skip(1).collect(); // Not need to check program name

        for i in all_arguments {
            if i == "--v" || i == "--version" {
                println!("Czkawka CLI {}", CZKAWKA_VERSION);
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
    main_window.set_title("Czkawka GTK GUI");

    ////////////////////////////////////////////////////////////////////////////////////////////////
    //// States

    // Buttons State - to remember existence of different buttons on pages

    let shared_buttons: Rc<RefCell<_>> = Rc::new(RefCell::new(HashMap::<String, HashMap<String, bool>>::new()));
    shared_buttons.borrow_mut().clear();

    // Show by default only search button
    for i in ["duplicate", "empty_folder"].iter() {
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

    ////////////////////////////////////////////////////////////////////////////////////////////////

    //// GUI Entry
    let entry_duplicate_minimal_size: gtk::Entry = builder.get_object("entry_duplicate_minimal_size").unwrap();
    let entry_allowed_extensions: gtk::Entry = builder.get_object("entry_allowed_extensions").unwrap();
    let entry_excluded_items: gtk::Entry = builder.get_object("entry_excluded_items").unwrap();

    //// GUI Buttons
    let buttons_search: gtk::Button = builder.get_object("buttons_search").unwrap();
    let buttons_stop: gtk::Button = builder.get_object("buttons_stop").unwrap();
    let buttons_resume: gtk::Button = builder.get_object("buttons_resume").unwrap();
    let buttons_pause: gtk::Button = builder.get_object("buttons_pause").unwrap();
    let buttons_select: gtk::Button = builder.get_object("buttons_select").unwrap();
    let buttons_delete: gtk::Button = builder.get_object("buttons_delete").unwrap();
    let buttons_save: gtk::Button = builder.get_object("buttons_save").unwrap();

    // Not used buttons for now
    buttons_stop.hide();
    buttons_resume.hide();
    buttons_pause.hide();
    buttons_select.hide();

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

        // Set Included Directory
        {
            let col_types: [glib::types::Type; 2] = [glib::types::Type::String, glib::types::Type::String];
            let list_store: gtk::ListStore = gtk::ListStore::new(&col_types);

            let mut tree_view_included_directory: gtk::TreeView = TreeView::with_model(&list_store);

            tree_view_included_directory.get_selection().set_mode(SelectionMode::Single);

            create_tree_view_directories(&mut tree_view_included_directory);

            let col_indices = [0, 1];

            let values: [&dyn ToValue; 2] = [&("/home/rafal/Pulpit"), &(MAIN_ROW_COLOR.to_string())];
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

        // Connect Buttons

        assert!(notebook_chooser_tool_children_names.contains(&"notebook_duplicate_finder_label".to_string()));
        assert!(notebook_chooser_tool_children_names.contains(&"scrolled_window_empty_folder_finder".to_string()));
        buttons_search.connect_clicked(move |_| {
            match notebook_chooser_tool_children_names.get(notebook_chooser_tool.get_current_page().unwrap() as usize).unwrap().as_str() {
                "notebook_duplicate_finder_label" => {
                    // Find duplicates
                    // TODO Change to proper value

                    let mut df = DuplicateFinder::new();
                    let check_method = duplicate::CheckingMethod::Hash; // TODO
                    {
                        df.set_included_directory(get_string_from_list_store(&scrolled_window_included_directories));
                        df.set_excluded_directory(get_string_from_list_store(&scrolled_window_excluded_directories));
                        df.set_excluded_items(entry_excluded_items.get_text().as_str().to_string());
                        df.set_allowed_extensions(entry_allowed_extensions.get_text().as_str().to_string());
                        df.set_min_file_size(match entry_duplicate_minimal_size.get_text().as_str().parse::<u64>() {
                            Ok(t) => t,
                            Err(_) => 1024, // By default
                        });
                        df.set_check_method(check_method.clone()); // TODO
                        df.set_delete_method(duplicate::DeleteMethod::None);
                        df.find_duplicates();
                    }
                    let information = df.get_information();
                    let text_messages = df.get_text_messages();

                    let duplicates_number: usize;
                    let duplicates_size: u64;
                    let duplicates_group: usize;

                    match check_method {
                        CheckingMethod::Hash => {
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
                        // Remove scrolled window from before - BUG - when doing it when view is scrolled, then scroll button disappears
                        for i in &scrolled_window_duplicate_finder.get_children() {
                            scrolled_window_duplicate_finder.remove(i);
                        }

                        let col_types: [glib::types::Type; 4] = [glib::types::Type::String, glib::types::Type::String, glib::types::Type::String, glib::types::Type::String];
                        let list_store: gtk::ListStore = gtk::ListStore::new(&col_types);

                        let mut tree_view_duplicate_finder: gtk::TreeView = TreeView::with_model(&list_store);

                        tree_view_duplicate_finder.get_selection().set_mode(SelectionMode::Multiple);

                        create_tree_view_duplicates(&mut tree_view_duplicate_finder);

                        let col_indices = [0, 1, 2, 3];

                        match check_method {
                            CheckingMethod::Hash => {
                                let btreemap = df.get_files_sorted_by_hash();

                                for (size, vectors_vector) in btreemap.iter().rev() {
                                    for vector in vectors_vector {
                                        let values: [&dyn ToValue; 4] = [
                                            &(vector.len().to_string() + " x " + size.to_string().as_str()),
                                            &("(".to_string() + ((vector.len() - 1) as u64 * *size as u64).to_string().as_str() + ")"),
                                            &"Bytes lost".to_string(),
                                            &(HEADER_ROW_COLOR.to_string()),
                                        ];
                                        list_store.set(&list_store.append(), &col_indices, &values);
                                        for entry in vector {
                                            let path = &entry.path;
                                            let index = path.rfind('/').unwrap();

                                            let values: [&dyn ToValue; 4] = [
                                                &(path[index + 1..].to_string()),
                                                &(path[..index].to_string()),
                                                &(NaiveDateTime::from_timestamp(entry.modified_date.duration_since(UNIX_EPOCH).expect("Invalid file date").as_secs() as i64, 0).to_string()),
                                                &(MAIN_ROW_COLOR.to_string()),
                                            ];
                                            list_store.set(&list_store.append(), &col_indices, &values);
                                        }
                                    }
                                }
                            }
                            CheckingMethod::Size => {
                                let btreemap = df.get_files_sorted_by_size();

                                for (size, vector) in btreemap.iter().rev() {
                                    let values: [&dyn ToValue; 3] = [
                                        &(vector.len().to_string() + " x " + size.to_string().as_str()),
                                        &("(".to_string() + ((vector.len() - 1) as u64 * *size as u64).to_string().as_str() + ")"),
                                        &"Bytes lost".to_string(),
                                    ];
                                    list_store.set(&list_store.append(), &col_indices, &values);
                                    for entry in vector {
                                        let path = &entry.path;
                                        let index = path.rfind('/').unwrap();

                                        let values: [&dyn ToValue; 3] = [
                                            &(path[index + 1..].to_string()),
                                            &(path[..index].to_string()),
                                            &(NaiveDateTime::from_timestamp(entry.modified_date.duration_since(UNIX_EPOCH).expect("Invalid file date").as_secs() as i64, 0).to_string()),
                                        ];
                                        list_store.set(&list_store.append(), &col_indices, &values);
                                    }
                                }
                            }
                            CheckingMethod::None => {
                                panic!();
                            }
                        }

                        let tree_selection = tree_view_duplicate_finder.get_selection();
                        tree_selection.set_select_function(Some(Box::new(select_function)));

                        scrolled_window_duplicate_finder.add(&tree_view_duplicate_finder);
                        scrolled_window_duplicate_finder.show_all();

                        print_text_messages_to_text_view(&text_messages, &text_view_errors);
                    }

                    // Set state
                    {
                        *shared_duplication_state.borrow_mut() = df;

                        if duplicates_size > 0 {
                            buttons_save.show();
                            buttons_delete.show();
                            *shared_buttons.borrow_mut().get_mut("duplicate").unwrap().get_mut("save").unwrap() = true;
                            *shared_buttons.borrow_mut().get_mut("duplicate").unwrap().get_mut("delete").unwrap() = true;
                        } else {
                            buttons_save.hide();
                            buttons_delete.hide();
                            *shared_buttons.borrow_mut().get_mut("duplicate").unwrap().get_mut("save").unwrap() = false;
                            *shared_buttons.borrow_mut().get_mut("duplicate").unwrap().get_mut("delete").unwrap() = false;
                        }
                    }
                }
                "scrolled_window_empty_folder_finder" => {
                    // Find empty folders
                    // TODO Change to proper value
                    let mut ef = EmptyFolder::new();

                    ef.set_included_directory("/home/rafal/Pulpit".to_string());
                    ef.set_delete_folder(false);
                    ef.find_empty_folders();

                    let information = ef.get_information();
                    let text_messages = ef.get_text_messages();

                    let empty_folder_number: usize = information.number_of_empty_folders;

                    entry_info.set_text(format!("Found {} empty folders.", empty_folder_number).as_str());

                    // Create GUI
                    {
                        // Remove scrolled window from before - BUG - when doing it when view is scrolled, then scroll button disappears
                        for i in &scrolled_window_empty_folder_finder.get_children() {
                            scrolled_window_empty_folder_finder.remove(i);
                        }

                        let col_types: [glib::types::Type; 4] = [glib::types::Type::String, glib::types::Type::String, glib::types::Type::String, glib::types::Type::String];
                        let list_store: gtk::ListStore = gtk::ListStore::new(&col_types);

                        let mut tree_view_empty_folder_finder: gtk::TreeView = TreeView::with_model(&list_store);

                        tree_view_empty_folder_finder.get_selection().set_mode(SelectionMode::Multiple);

                        create_tree_view_empty_folders(&mut tree_view_empty_folder_finder);

                        let col_indices = [0, 1, 2, 3];

                        let hashmap = ef.get_empty_folder_list();

                        for (name, entry) in hashmap {
                            let name: String = name[..(name.len() - 1)].to_string();
                            let index = name.rfind('/').unwrap();
                            let values: [&dyn ToValue; 4] = [
                                &(name[index + 1..].to_string()),
                                &(name[..index].to_string()),
                                &(NaiveDateTime::from_timestamp(entry.modified_date.duration_since(UNIX_EPOCH).expect("Invalid file date").as_secs() as i64, 0).to_string()),
                                &(MAIN_ROW_COLOR.to_string()),
                            ];
                            list_store.set(&list_store.append(), &col_indices, &values);
                        }

                        scrolled_window_empty_folder_finder.add(&tree_view_empty_folder_finder);
                        scrolled_window_empty_folder_finder.show_all();

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
                e => panic!("Not existent {}", e),
            }
        });
    }

    // Quit the program when X in main window was clicked
    main_window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    // We start the gtk main loop.
    gtk::main();
}
pub fn create_tree_view_duplicates(tree_view_duplicate_finder: &mut gtk::TreeView) {
    let renderer = gtk::CellRendererText::new();
    let name_column: gtk::TreeViewColumn = TreeViewColumn::new();
    name_column.pack_start(&renderer, true);
    name_column.set_title("File Name");
    name_column.set_resizable(true);
    name_column.set_min_width(50);
    name_column.add_attribute(&renderer, "text", ColumnsDefault::Name as i32);
    name_column.add_attribute(&renderer, "background", ColumnsDefault::Color as i32);
    tree_view_duplicate_finder.append_column(&name_column);

    let renderer = gtk::CellRendererText::new();
    let path_column: gtk::TreeViewColumn = TreeViewColumn::new();
    path_column.pack_start(&renderer, true);
    path_column.set_title("Path");
    path_column.set_resizable(true);
    path_column.set_min_width(100);
    path_column.add_attribute(&renderer, "text", ColumnsDefault::Path as i32);
    path_column.add_attribute(&renderer, "background", ColumnsDefault::Color as i32);
    tree_view_duplicate_finder.append_column(&path_column);

    let renderer = gtk::CellRendererText::new();
    let modification_date_column: gtk::TreeViewColumn = TreeViewColumn::new();
    modification_date_column.pack_start(&renderer, true);
    modification_date_column.set_title("Modification Date");
    modification_date_column.set_resizable(true);
    modification_date_column.set_min_width(100);
    modification_date_column.add_attribute(&renderer, "text", ColumnsDefault::Modification as i32);
    modification_date_column.add_attribute(&renderer, "background", ColumnsDefault::Color as i32);
    tree_view_duplicate_finder.append_column(&modification_date_column);

    tree_view_duplicate_finder.set_vexpand(true);
}

pub fn create_tree_view_empty_folders(tree_view_empty_folder_finder: &mut gtk::TreeView) {
    let renderer = gtk::CellRendererText::new();
    let name_column: gtk::TreeViewColumn = TreeViewColumn::new();
    name_column.pack_start(&renderer, true);
    name_column.set_title("Folder Name");
    name_column.set_resizable(true);
    name_column.set_min_width(50);
    name_column.add_attribute(&renderer, "text", ColumnsDefault::Name as i32);
    name_column.add_attribute(&renderer, "background", ColumnsDefault::Color as i32);
    tree_view_empty_folder_finder.append_column(&name_column);

    let renderer = gtk::CellRendererText::new();
    let path_column: gtk::TreeViewColumn = TreeViewColumn::new();
    path_column.pack_start(&renderer, true);
    path_column.set_title("Path");
    path_column.set_resizable(true);
    path_column.set_min_width(100);
    path_column.add_attribute(&renderer, "text", ColumnsDefault::Path as i32);
    path_column.add_attribute(&renderer, "background", ColumnsDefault::Color as i32);
    tree_view_empty_folder_finder.append_column(&path_column);

    let renderer = gtk::CellRendererText::new();
    let modification_date_column: gtk::TreeViewColumn = TreeViewColumn::new();
    modification_date_column.pack_start(&renderer, true);
    modification_date_column.set_title("Modification Date");
    modification_date_column.set_resizable(true);
    modification_date_column.set_min_width(100);
    modification_date_column.add_attribute(&renderer, "text", ColumnsDefault::Modification as i32);
    modification_date_column.add_attribute(&renderer, "background", ColumnsDefault::Color as i32);
    tree_view_empty_folder_finder.append_column(&modification_date_column);

    tree_view_empty_folder_finder.set_vexpand(true);
}

pub fn create_tree_view_directories(tree_view_directories: &mut gtk::TreeView) {
    let renderer = gtk::CellRendererText::new();
    let name_column: gtk::TreeViewColumn = TreeViewColumn::new();
    name_column.pack_start(&renderer, true);
    name_column.add_attribute(&renderer, "text", ColumnsDirectory::Path as i32);
    name_column.add_attribute(&renderer, "background", ColumnsDirectory::Color as i32);
    tree_view_directories.append_column(&name_column);

    tree_view_directories.set_headers_visible(false);
}

pub fn get_string_from_list_store(scrolled_window: &gtk::ScrolledWindow) -> String {
    let tree_view: gtk::TreeView = scrolled_window.get_children().get(0).unwrap().clone().downcast::<gtk::TreeView>().unwrap();
    let list_store: gtk::ListStore = tree_view.get_model().unwrap().downcast::<gtk::ListStore>().unwrap();

    let tree_iter = match list_store.get_iter_first() {
        Some(t) => t,
        None => return "".to_string(),
    };

    list_store.get_value(&tree_iter, 0).get::<String>().unwrap().unwrap()
}
pub fn print_text_messages_to_text_view(text_messages: &Messages, text_view: &gtk::TextView) {
    let mut messages: String = String::from("");
    if !text_messages.messages.is_empty() {
        messages += "############### MESSAGES ###############\n";
    }
    for text in &text_messages.messages {
        messages += text.as_str();
        messages += "\n";
    }
    if !text_messages.messages.is_empty() {
        messages += "\n";
    }
    if !text_messages.warnings.is_empty() {
        messages += "############### WARNINGS ###############\n";
    }
    for text in &text_messages.warnings {
        messages += text.as_str();
        messages += "\n";
    }
    if !text_messages.warnings.is_empty() {
        messages += "\n";
    }
    if !text_messages.errors.is_empty() {
        messages += "############### ERRORS ###############\n";
    }
    for text in &text_messages.errors {
        messages += text.as_str();
        messages += "\n";
    }
    if !text_messages.errors.is_empty() {
        messages += "\n";
    }

    text_view.get_buffer().unwrap().set_text(messages.as_str());
}

fn select_function(_ts: &gtk::TreeSelection, _tm: &gtk::TreeModel, _tp: &gtk::TreePath, _b: bool) -> bool {
    // TODO Create proper function to disable selecting header rows
    false
}
