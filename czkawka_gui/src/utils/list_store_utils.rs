use gtk4::{ListStore, TreeView};
use std::path::PathBuf;

pub fn get_string_from_list_store(tree_view: &TreeView, column_full_path: i32, column_selection: Option<i32>) -> Vec<String> {
    let list_store: ListStore = get_list_store(tree_view);
    let mut string_vector: Vec<String> = Vec::new();
    let Some(tree_iter) = list_store.iter_first() else { return string_vector; };
    match column_selection {
        Some(column_selection) => loop {
            if list_store.get::<bool>(&tree_iter, column_selection) {
                string_vector.push(list_store.get::<String>(&tree_iter, column_full_path));
            }
            if !list_store.iter_next(&tree_iter) { return string_vector; }
        },
        None => loop {
            string_vector.push(list_store.get::<String>(&tree_iter, column_full_path));
            if !list_store.iter_next(&tree_iter) { return string_vector; }
        },
    }
}

pub fn get_from_list_store_fnc<T>(tree_view: &TreeView, fnc: &dyn Fn(&ListStore, &gtk4::TreeIter, &mut Vec<T>)) -> Vec<T> {
    let list_store: ListStore = get_list_store(tree_view);
    let mut result_vector: Vec<T> = Vec::new();
    let Some(tree_iter) = list_store.iter_first() else { return result_vector; };
    loop {
        fnc(&list_store, &tree_iter, &mut result_vector);
        if !list_store.iter_next(&tree_iter) { return result_vector; }
    }
}

pub fn get_path_buf_from_vector_of_strings(vec_string: &[String]) -> Vec<PathBuf> {
    vec_string.iter().map(PathBuf::from).collect()
}

pub fn get_list_store(tree_view: &TreeView) -> ListStore {
    tree_view.model().expect("Tree view have no model").downcast::<ListStore>().expect("Model is not ListStore")
}

pub fn clean_invalid_headers(model: &ListStore, column_header: i32, column_path: i32) {
    // ...existing code from help_functions.rs...
}

pub fn check_how_much_elements_is_selected(tree_view: &TreeView, column_header: Option<i32>, column_selection: i32) -> (u64, u64) {
    // ...existing code from help_functions.rs...
}

pub fn count_number_of_groups(tree_view: &TreeView, column_header: i32) -> u32 {
    // ...existing code from help_functions.rs...
}

pub fn check_if_value_is_in_list_store(list_store: &ListStore, column: i32, value: &str) -> bool {
    // ...existing code from help_functions.rs...
}

pub fn check_if_list_store_column_have_all_same_values(list_store: &ListStore, column: i32, value: bool) -> bool {
    // ...existing code from help_functions.rs...
}

