#[allow(unused_imports)]
use czkawka_core::{duplicate, empty_folder};

extern crate gtk;
use gtk::prelude::*;
use gtk::{Builder, TreeView, TreeViewColumn};
use duplicate::DuplicateFinder;
use czkawka_core::duplicate::{CheckingMethod, DeleteMethod};
// use gtk::{ButtonsType, DialogFlags, MessageDialog, MessageType, Window};

fn main() {
    gtk::init().expect("Failed to initialize GTK.");

    // Loading glade file content
    let glade_src = include_str!("../czkawka.glade");
    // Build UI from glade file
    let builder = gtk::Builder::from_string(glade_src);

    // Show first window
    let main_window: gtk::Window = builder.get_object("main_window").unwrap();
    main_window.show_all();

    prepare_buttons_at_start(builder);


    // We start the gtk main loop.
    gtk::main();
}
fn prepare_buttons_at_start(builder : Builder){
    let buttons_search : gtk::Button = builder.get_object("buttons_search").unwrap();
    buttons_search.connect_clicked(search_for_duplicates);
    let buttons_stop : gtk::Button = builder.get_object("buttons_stop").unwrap();
    buttons_stop.hide();
    let buttons_resume : gtk::Button = builder.get_object("buttons_resume").unwrap();
    buttons_resume.hide();
    let buttons_pause : gtk::Button = builder.get_object("buttons_pause").unwrap();
    buttons_pause.hide();


    let buttons_select : gtk::Button = builder.get_object("buttons_select").unwrap();
    buttons_select.hide();
    let buttons_delete : gtk::Button = builder.get_object("buttons_delete").unwrap();
    buttons_delete.hide();
    let buttons_save : gtk::Button = builder.get_object("buttons_save").unwrap();
    buttons_save.hide();

    search_for_duplicates_layout(builder);
}
fn search_for_duplicates_layout(builder:Builder){

    let scrolled_window_duplicate_finder : gtk::ScrolledWindow = builder.get_object("scrolled_window_duplicate_finder").unwrap();


    let name_column : gtk::TreeViewColumn = TreeViewColumn::new();
    name_column.set_title("File Name");
    name_column.set_resizable(true);
    name_column.set_min_width(50);

    let path_column : gtk::TreeViewColumn = TreeViewColumn::new();
    path_column.set_title("Path");
    path_column.set_resizable(true);
    path_column.set_min_width(50);

    let modification_date_column : gtk::TreeViewColumn = TreeViewColumn::new();
    modification_date_column.set_title("Modification Date");
    modification_date_column.set_resizable(true);
    modification_date_column.set_min_width(50);

    let col_types: [glib::types::Type; 2] = [
        glib::types::Type::String,
        glib::types::Type::String,
    ];
    let list_store : gtk::ListStore = gtk::ListStore::new(&col_types);

    let tree_view_duplicate_finder : gtk::TreeView = TreeView::with_model(&list_store);

    tree_view_duplicate_finder.append_column(&name_column);
    tree_view_duplicate_finder.append_column(&path_column);
    tree_view_duplicate_finder.append_column(&modification_date_column);

    scrolled_window_duplicate_finder.add(&tree_view_duplicate_finder);
    scrolled_window_duplicate_finder.show_all();
}
fn search_for_duplicates(button : &gtk::Button){// : gtk::Button){
    println!("Szukam");
    button.hide();
    let mut df  = DuplicateFinder::new();
    df.set_include_directory("/home/rafal/Pulpit".to_owned());
    df.set_exclude_directory("/rafa/".to_owned());
    df.set_allowed_extensions("".to_owned());
    df.set_min_file_size(1000);
    df.find_duplicates(&CheckingMethod::HASH, &DeleteMethod::None);
    button.show();
}