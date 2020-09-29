use czkawka_core::common_messages::Messages;
use gtk::prelude::*;
use gtk::TreeViewColumn;

pub enum Columns3Default {
    // Columns for duplicate and empty folder treeview
    Name = 0,
    Path,
    Modification,
    Color,
}
pub enum ColumnsDirectory {
    // Columns for Included and Excluded Directories in upper Notebook
    Path = 0,
    Color,
}

pub const MAIN_ROW_COLOR: &str = "#343434";
pub const HEADER_ROW_COLOR: &str = "#272727";
//pub const MAIN_ROW_COLOR: &str = "#f4f434"; // TEST
//pub const HEADER_ROW_COLOR: &str = "#010101"; // TEST

pub fn create_tree_view_duplicates(tree_view_duplicate_finder: &mut gtk::TreeView) {
    let renderer = gtk::CellRendererText::new();
    let name_column: gtk::TreeViewColumn = TreeViewColumn::new();
    name_column.pack_start(&renderer, true);
    name_column.set_title("File Name");
    name_column.set_resizable(true);
    name_column.set_min_width(50);
    name_column.add_attribute(&renderer, "text", Columns3Default::Name as i32);
    name_column.add_attribute(&renderer, "background", Columns3Default::Color as i32);
    tree_view_duplicate_finder.append_column(&name_column);

    let renderer = gtk::CellRendererText::new();
    let path_column: gtk::TreeViewColumn = TreeViewColumn::new();
    path_column.pack_start(&renderer, true);
    path_column.set_title("Path");
    path_column.set_resizable(true);
    path_column.set_min_width(100);
    path_column.add_attribute(&renderer, "text", Columns3Default::Path as i32);
    path_column.add_attribute(&renderer, "background", Columns3Default::Color as i32);
    tree_view_duplicate_finder.append_column(&path_column);

    let renderer = gtk::CellRendererText::new();
    let modification_date_column: gtk::TreeViewColumn = TreeViewColumn::new();
    modification_date_column.pack_start(&renderer, true);
    modification_date_column.set_title("Modification Date");
    modification_date_column.set_resizable(true);
    modification_date_column.set_min_width(100);
    modification_date_column.add_attribute(&renderer, "text", Columns3Default::Modification as i32);
    modification_date_column.add_attribute(&renderer, "background", Columns3Default::Color as i32);
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
    name_column.add_attribute(&renderer, "text", Columns3Default::Name as i32);
    name_column.add_attribute(&renderer, "background", Columns3Default::Color as i32);
    tree_view_empty_folder_finder.append_column(&name_column);

    let renderer = gtk::CellRendererText::new();
    let path_column: gtk::TreeViewColumn = TreeViewColumn::new();
    path_column.pack_start(&renderer, true);
    path_column.set_title("Path");
    path_column.set_resizable(true);
    path_column.set_min_width(100);
    path_column.add_attribute(&renderer, "text", Columns3Default::Path as i32);
    path_column.add_attribute(&renderer, "background", Columns3Default::Color as i32);
    tree_view_empty_folder_finder.append_column(&path_column);

    let renderer = gtk::CellRendererText::new();
    let modification_date_column: gtk::TreeViewColumn = TreeViewColumn::new();
    modification_date_column.pack_start(&renderer, true);
    modification_date_column.set_title("Modification Date");
    modification_date_column.set_resizable(true);
    modification_date_column.set_min_width(100);
    modification_date_column.add_attribute(&renderer, "text", Columns3Default::Modification as i32);
    modification_date_column.add_attribute(&renderer, "background", Columns3Default::Color as i32);
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
    let mut first: bool = true;

    let mut return_string: String = "".to_string();
    let tree_iter = match list_store.get_iter_first() {
        Some(t) => t,
        None => return return_string,
    };
    loop {
        if !first {
            return_string += ",";
        } else {
            first = false;
        }
        return_string += list_store.get_value(&tree_iter, 0).get::<String>().unwrap().unwrap().as_str();
        if !list_store.iter_next(&tree_iter) {
            return return_string;
        }
    }
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

pub fn select_function_3column(_tree_selection: &gtk::TreeSelection, tree_model: &gtk::TreeModel, tree_path: &gtk::TreePath, _is_path_currently_selected: bool) -> bool {
    // let name = tree_model.get_value(&tree_model.get_iter(tree_path).unwrap(),Columns3Default::Name as i32).get::<String>().unwrap().unwrap();
    // let path = tree_model.get_value(&tree_model.get_iter(tree_path).unwrap(), Columns3Default::Path as i32).get::<String>().unwrap().unwrap();
    // let modification = tree_model.get_value(&tree_model.get_iter(tree_path).unwrap(),Columns3Default::Modification as i32).get::<String>().unwrap().unwrap();
    let color = tree_model.get_value(&tree_model.get_iter(tree_path).unwrap(), Columns3Default::Color as i32).get::<String>().unwrap().unwrap();

    if color == HEADER_ROW_COLOR {
        return false;
    }

    true
}
