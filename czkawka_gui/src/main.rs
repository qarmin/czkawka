#[allow(unused_imports)]
use czkawka_core::{duplicate, empty_folder};
use humansize::{file_size_opts as options, FileSize};

extern crate gtk;
use duplicate::DuplicateFinder;
use gtk::prelude::*;
use gtk::{Builder, TreeView, TreeViewColumn};
use std::collections::HashMap;

fn main() {
    gtk::init().expect("Failed to initialize GTK.");

    // Loading glade file content and build with it help UI
    let glade_src = include_str!("../czkawka.glade");
    let builder = Builder::from_string(glade_src);

    // Windows
    let main_window: gtk::Window = builder.get_object("main_window").unwrap();
    main_window.show_all();

    // Buttons State

    // let shared_buttons: Rc<RefCell<_>> = Rc::new(RefCell::new( HashMap::<&str, bool>::new()));

    let mut hashmap_buttons: HashMap<&str, bool> = Default::default();
    for i in ["duplicate", "empty_folder"].iter() {
        hashmap_buttons.insert(i, false);
    }

    // GUI Notepad Buttons

    // GUI Buttons
    let buttons_search: gtk::Button = builder.get_object("buttons_search").unwrap();
    let buttons_stop: gtk::Button = builder.get_object("buttons_stop").unwrap();
    let buttons_resume: gtk::Button = builder.get_object("buttons_resume").unwrap();
    let buttons_pause: gtk::Button = builder.get_object("buttons_pause").unwrap();
    let buttons_select: gtk::Button = builder.get_object("buttons_select").unwrap();
    let buttons_delete: gtk::Button = builder.get_object("buttons_delete").unwrap();
    let buttons_save: gtk::Button = builder.get_object("buttons_save").unwrap();

    // Notebooks
    let notebook_chooser_tool: gtk::Notebook = builder.get_object("notebook_chooser_tool").unwrap();
    let mut notebook_chooser_tool_children_names: Vec<String> = Vec::new();

    for i in notebook_chooser_tool.get_children() {
        notebook_chooser_tool_children_names.push(i.get_buildable_name().unwrap().to_string());
    }

    // Entry
    let info_entry: gtk::Entry = builder.get_object("info_entry").unwrap(); // To show default

    // Scrolled window
    let scrolled_window_duplicate_finder: gtk::ScrolledWindow = builder.get_object("scrolled_window_duplicate_finder").unwrap();

    {
        // Set starting intro
        // Duplicate Finder

        let name_column: gtk::TreeViewColumn = TreeViewColumn::new();
        name_column.set_title("File Name");
        name_column.set_resizable(true);
        name_column.set_min_width(50);

        let path_column: gtk::TreeViewColumn = TreeViewColumn::new();
        path_column.set_title("Path");
        path_column.set_resizable(true);
        path_column.set_min_width(100);

        let modification_date_column: gtk::TreeViewColumn = TreeViewColumn::new();
        modification_date_column.set_title("Modification Date");
        modification_date_column.set_resizable(true);
        modification_date_column.set_min_width(100);

        let col_types: [glib::types::Type; 3] = [glib::types::Type::String, glib::types::Type::String, glib::types::Type::String];
        let list_store: gtk::ListStore = gtk::ListStore::new(&col_types);

        let tree_view_duplicate_finder: gtk::TreeView = TreeView::with_model(&list_store);

        tree_view_duplicate_finder.append_column(&name_column);
        tree_view_duplicate_finder.append_column(&path_column);
        tree_view_duplicate_finder.append_column(&modification_date_column);

        scrolled_window_duplicate_finder.add(&tree_view_duplicate_finder);
        scrolled_window_duplicate_finder.show_all();

        info_entry.set_text("Duplicated Files");

        // // Disable all unused buttons
        buttons_search.show();
        buttons_stop.hide();
        buttons_resume.hide();
        buttons_pause.hide();
        buttons_select.show();
        buttons_delete.hide();
        buttons_save.hide();
    }
    {
        // Connect Buttons

        let buttons_search_clone = buttons_search.clone();

        buttons_search.connect_clicked(move |_| {
            assert!(notebook_chooser_tool_children_names.contains(&"notebook_duplicate_finder_label".to_string()));
            assert!(notebook_chooser_tool_children_names.contains(&"notebook_empty_folders_label".to_string()));
            match notebook_chooser_tool_children_names.get(notebook_chooser_tool.get_current_page().unwrap() as usize).unwrap().as_str() {
                "notebook_duplicate_finder_label" => {
                    // TODO Change to proper value
                    let mut df = DuplicateFinder::new();
                    let check_method = duplicate::CheckingMethod::HASH;
                    df.set_include_directory("/home/rafal/Pulpit/AAA".to_owned());
                    df.set_exclude_directory("/rafa/".to_owned());
                    df.set_excluded_items("".to_owned());
                    df.set_allowed_extensions("".to_owned());
                    df.set_min_file_size(1000);
                    df.set_check_method(check_method.clone());
                    df.set_delete_method(duplicate::DeleteMethod::None);
                    df.find_duplicates();
                    let information = df.get_information();

                    let duplicates_number: usize;
                    let duplicates_size: u64;
                    let duplicates_group: usize;

                    if check_method == duplicate::CheckingMethod::HASH {
                        duplicates_number = information.number_of_duplicated_files_by_hash;
                        duplicates_size = information.lost_space_by_hash;
                        duplicates_group = information.number_of_groups_by_hash;
                    } else {
                        duplicates_number = information.number_of_duplicated_files_by_size;
                        duplicates_size = information.lost_space_by_size;
                        duplicates_group = information.number_of_groups_by_size;
                    }

                    info_entry.set_text(format!("Found {} duplicates files in {} groups which took {}.", duplicates_number, duplicates_group, duplicates_size.file_size(options::BINARY).unwrap()).as_str());

                    // Buttons
                    buttons_select.show();
                    buttons_delete.show();
                    // TODO Add buttons
                    // if *hashmap_buttons.get("duplicate").unwrap() {
                    //     buttons_select.show();
                    //     buttons_delete.show();
                    // }
                    // else{
                    //     buttons_select.hide();
                    //     buttons_delete.hide();
                    // }

                    buttons_search_clone.show();
                    buttons_stop.hide();
                    buttons_resume.hide();
                    buttons_pause.hide();
                    buttons_save.hide();
                }
                "notebook_empty_folders_label" => {
                    // let mut ef = empty_folder::EmptyFolder::new();
                    // let mut delete_folders: bool = false;
                    //
                    //     ef.set_include_directory("/home/rafal/Pulpit".to_string());
                    //
                    // ef.find_empty_folders(false);
                    //
                    //
                    // info_entry.set_text(format!("Found {} empty folders.",duplicates_number).as_str());
                    //
                    //
                    // buttons_select.show();
                    // buttons_delete.show();
                    // buttons_search_clone.show();
                    // buttons_stop.hide();
                    // buttons_resume.hide();
                    // buttons_pause.hide();
                    // buttons_save.hide();
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
