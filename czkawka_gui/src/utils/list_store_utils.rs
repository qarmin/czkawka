use std::path::PathBuf;

use gtk4::{ListStore, TreeView};
use gtk4::prelude::*;

pub fn get_string_from_list_store(tree_view: &TreeView, column_full_path: i32, column_selection: Option<i32>) -> Vec<String> {
    let list_store: ListStore = get_list_store(tree_view);
    let mut string_vector: Vec<String> = Vec::new();
    let Some(tree_iter) = list_store.iter_first() else {
        return string_vector;
    };
    match column_selection {
        Some(column_selection) => loop {
            if list_store.get::<bool>(&tree_iter, column_selection) {
                string_vector.push(list_store.get::<String>(&tree_iter, column_full_path));
            }
            if !list_store.iter_next(&tree_iter) {
                return string_vector;
            }
        },
        None => loop {
            string_vector.push(list_store.get::<String>(&tree_iter, column_full_path));
            if !list_store.iter_next(&tree_iter) {
                return string_vector;
            }
        },
    }
}

pub fn get_from_list_store_fnc<T>(tree_view: &TreeView, fnc: &dyn Fn(&ListStore, &gtk4::TreeIter, &mut Vec<T>)) -> Vec<T> {
    let list_store: ListStore = get_list_store(tree_view);
    let mut result_vector: Vec<T> = Vec::new();
    let Some(tree_iter) = list_store.iter_first() else {
        return result_vector;
    };
    loop {
        fnc(&list_store, &tree_iter, &mut result_vector);
        if !list_store.iter_next(&tree_iter) {
            return result_vector;
        }
    }
}

pub fn get_path_buf_from_vector_of_strings(vec_string: &[String]) -> Vec<PathBuf> {
    vec_string.iter().map(PathBuf::from).collect()
}

pub fn get_list_store(tree_view: &TreeView) -> ListStore {
    tree_view.model().expect("Tree view have no model").downcast::<ListStore>().expect("Model is not ListStore")
}

