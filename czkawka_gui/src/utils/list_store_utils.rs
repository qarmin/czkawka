use std::path::PathBuf;

use gtk4::prelude::*;
use gtk4::{ListStore, TreeView};

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

pub fn clean_invalid_headers(model: &ListStore, column_header: i32, column_path: i32) {
    if let Some(first_iter) = model.iter_first() {
        let mut vec_tree_path_to_delete: Vec<gtk4::TreePath> = Vec::new();
        let mut current_iter = first_iter;
        assert!(model.get::<bool>(&current_iter, column_header), "First deleted element, should be a header");
        let mut next_iter;
        let mut next_next_iter;
        if model.get::<String>(&current_iter, column_path).is_empty() {
            'main: loop {
                assert!(model.get::<bool>(&current_iter, column_header), "First deleted element, should be a header");
                next_iter = current_iter;
                if !model.iter_next(&next_iter) {
                    vec_tree_path_to_delete.push(model.path(&current_iter));
                    break 'main;
                }
                if model.get::<bool>(&next_iter, column_header) {
                    vec_tree_path_to_delete.push(model.path(&current_iter));
                    current_iter = next_iter;
                    continue 'main;
                }
                next_next_iter = next_iter;
                if !model.iter_next(&next_next_iter) {
                    vec_tree_path_to_delete.push(model.path(&current_iter));
                    vec_tree_path_to_delete.push(model.path(&next_iter));
                    break 'main;
                }
                if model.get::<bool>(&next_next_iter, column_header) {
                    vec_tree_path_to_delete.push(model.path(&current_iter));
                    vec_tree_path_to_delete.push(model.path(&next_iter));
                    current_iter = next_next_iter;
                    continue 'main;
                }
                loop {
                    if !model.iter_next(&next_next_iter) {
                        break 'main;
                    }
                    if model.get::<bool>(&next_next_iter, column_header) {
                        current_iter = next_next_iter;
                        continue 'main;
                    }
                }
            }
        } else {
            'reference: loop {
                assert!(model.get::<bool>(&current_iter, column_header), "First deleted element, should be a header");
                next_iter = current_iter;
                if !model.iter_next(&next_iter) {
                    vec_tree_path_to_delete.push(model.path(&current_iter));
                    break 'reference;
                }
                if model.get::<bool>(&next_iter, column_header) {
                    vec_tree_path_to_delete.push(model.path(&current_iter));
                    current_iter = next_iter;
                    continue 'reference;
                }
                next_next_iter = next_iter;
                if !model.iter_next(&next_next_iter) {
                    break 'reference;
                }
                if model.get::<bool>(&next_next_iter, column_header) {
                    current_iter = next_next_iter;
                    continue 'reference;
                }
                loop {
                    if !model.iter_next(&next_next_iter) {
                        break 'reference;
                    }
                    if model.get::<bool>(&next_next_iter, column_header) {
                        current_iter = next_next_iter;
                        continue 'reference;
                    }
                }
            }
        }
        for tree_path in vec_tree_path_to_delete.iter().rev() {
            model.remove(&model.iter(tree_path).expect("Using invalid tree_path"));
        }
    }
    if let Some(iter) = model.iter_first() {
        if !model.iter_next(&iter) {
            model.clear();
        }
    }
}

pub fn check_how_much_elements_is_selected(tree_view: &TreeView, column_header: Option<i32>, column_selection: i32) -> (u64, u64) {
    let mut number_of_selected_items: u64 = 0;
    let mut number_of_selected_groups: u64 = 0;
    let model = get_list_store(tree_view);
    let mut is_item_currently_selected_in_group: bool = false;
    if let Some(iter) = model.iter_first() {
        if let Some(column_header) = column_header {
            assert!(model.get::<bool>(&iter, column_header));
            loop {
                if !model.iter_next(&iter) {
                    break;
                }
                if model.get::<bool>(&iter, column_header) {
                    is_item_currently_selected_in_group = false;
                } else {
                    if model.get::<bool>(&iter, column_selection) {
                        number_of_selected_items += 1;
                        if !is_item_currently_selected_in_group {
                            number_of_selected_groups += 1;
                        }
                        is_item_currently_selected_in_group = true;
                    }
                }
            }
        } else {
            if model.get::<bool>(&iter, column_selection) {
                number_of_selected_items += 1;
            }
            loop {
                if !model.iter_next(&iter) {
                    break;
                }
                if model.get::<bool>(&iter, column_selection) {
                    number_of_selected_items += 1;
                }
            }
        }
    }
    (number_of_selected_items, number_of_selected_groups)
}

pub fn count_number_of_groups(tree_view: &TreeView, column_header: i32) -> u32 {
    let mut number_of_selected_groups = 0;
    let model = get_list_store(tree_view);
    if let Some(iter) = model.iter_first() {
        assert!(model.get::<bool>(&iter, column_header));
        number_of_selected_groups += 1;
        loop {
            if !model.iter_next(&iter) {
                break;
            }
            if model.get::<bool>(&iter, column_header) {
                number_of_selected_groups += 1;
            }
        }
    }
    number_of_selected_groups
}

pub fn check_if_value_is_in_list_store(list_store: &ListStore, column: i32, value: &str) -> bool {
    if let Some(iter) = list_store.iter_first() {
        loop {
            let list_store_value: String = list_store.get::<String>(&iter, column);
            if value == list_store_value {
                return true;
            }
            if !list_store.iter_next(&iter) {
                break;
            }
        }
    }
    false
}

pub fn check_if_list_store_column_have_all_same_values(list_store: &ListStore, column: i32, value: bool) -> bool {
    if let Some(iter) = list_store.iter_first() {
        loop {
            let list_store_value: bool = list_store.get::<bool>(&iter, column);
            if value != list_store_value {
                return false;
            }
            if !list_store.iter_next(&iter) {
                break;
            }
        }
        return true;
    }
    false
}

#[cfg(test)]
mod test {
    use glib::Value;
    use glib::types::Type;
    use gtk4::TreeView;
    use gtk4::prelude::*;

    use super::*;

    #[gtk4::test]
    fn test_get_string_from_list_store() {
        let columns_types: &[Type] = &[Type::STRING];
        let list_store = gtk4::ListStore::new(columns_types);
        let tree_view = TreeView::with_model(&list_store);
        let values_to_add: &[(u32, &dyn ToValue)] = &[(0, &"test"), (0, &"test2"), (0, &"test3")];
        for i in values_to_add {
            list_store.set(&list_store.append(), &[*i]);
        }
        assert_eq!(
            get_string_from_list_store(&tree_view, 0, None),
            vec!["test".to_string(), "test2".to_string(), "test3".to_string()]
        );
        let columns_types: &[Type] = &[Type::BOOL, Type::STRING];
        let list_store = gtk4::ListStore::new(columns_types);
        let tree_view = TreeView::with_model(&list_store);
        let values_to_add: &[&[(u32, &dyn ToValue)]] = &[
            &[(0, &Into::<Value>::into(true)), (1, &Into::<Value>::into("test"))],
            &[(0, &Into::<Value>::into(true)), (1, &Into::<Value>::into("test2"))],
            &[(0, &Into::<Value>::into(false)), (1, &Into::<Value>::into("test3"))],
        ];
        for i in values_to_add {
            list_store.set(&list_store.append(), i);
        }
        assert_eq!(get_string_from_list_store(&tree_view, 1, Some(0)), vec!["test".to_string(), "test2".to_string()]);
    }

    #[gtk4::test]
    fn test_check_if_list_store_column_have_all_same_values() {
        let columns_types: &[Type] = &[Type::BOOL];
        let list_store = gtk4::ListStore::new(columns_types);
        list_store.clear();
        let values_to_add: &[(u32, &dyn ToValue)] = &[(0, &true), (0, &true), (0, &false)];
        for i in values_to_add {
            list_store.set(&list_store.append(), &[*i]);
        }
        assert!(!check_if_list_store_column_have_all_same_values(&list_store, 0, true));
        assert!(!check_if_list_store_column_have_all_same_values(&list_store, 0, false));
        list_store.clear();
        let values_to_add: &[(u32, &dyn ToValue)] = &[(0, &true), (0, &true), (0, &true)];
        for i in values_to_add {
            list_store.set(&list_store.append(), &[*i]);
        }
        assert!(check_if_list_store_column_have_all_same_values(&list_store, 0, true));
        assert!(!check_if_list_store_column_have_all_same_values(&list_store, 0, false));
        list_store.clear();
        let values_to_add: &[(u32, &dyn ToValue)] = &[(0, &false)];
        for i in values_to_add {
            list_store.set(&list_store.append(), &[*i]);
        }
        assert!(!check_if_list_store_column_have_all_same_values(&list_store, 0, true));
        assert!(check_if_list_store_column_have_all_same_values(&list_store, 0, false));
        list_store.clear();
        assert!(!check_if_list_store_column_have_all_same_values(&list_store, 0, true));
        assert!(!check_if_list_store_column_have_all_same_values(&list_store, 0, false));
    }

    #[gtk4::test]
    fn test_check_if_value_is_in_list_store() {
        let columns_types: &[Type] = &[Type::STRING];
        let list_store = gtk4::ListStore::new(columns_types);
        let values_to_add: &[(u32, &dyn ToValue)] = &[(0, &"Koczkodan"), (0, &"Kachir")];
        for i in values_to_add {
            list_store.set(&list_store.append(), &[*i]);
        }
        assert!(check_if_value_is_in_list_store(&list_store, 0, "Koczkodan"));
        assert!(check_if_value_is_in_list_store(&list_store, 0, "Kachir"));
        assert!(!check_if_value_is_in_list_store(&list_store, 0, "NotPresent"));
        list_store.clear();
        assert!(!check_if_value_is_in_list_store(&list_store, 0, "Koczkodan"));
    }

    #[gtk4::test]
    fn test_count_number_of_groups() {
        let columns_types: &[Type] = &[Type::BOOL];
        let list_store = gtk4::ListStore::new(columns_types);
        let tree_view = TreeView::with_model(&list_store);
        // Add 3 groups (headers)
        for _ in 0..3 {
            let iter = list_store.append();
            list_store.set(&iter, &[(0, &true)]);
        }
        assert_eq!(count_number_of_groups(&tree_view, 0), 3);
        list_store.clear();
        assert_eq!(count_number_of_groups(&tree_view, 0), 0);
    }

    #[gtk4::test]
    fn test_check_how_much_elements_is_selected() {
        let columns_types: &[Type] = &[Type::BOOL, Type::BOOL]; // header, selection
        let list_store = gtk4::ListStore::new(columns_types);
        let tree_view = TreeView::with_model(&list_store);
        // Group 1 (header + 2 selected)
        let header1 = list_store.append();
        list_store.set(&header1, &[(0, &true), (1, &false)]);
        let item1 = list_store.append();
        list_store.set(&item1, &[(0, &false), (1, &true)]);
        let item2 = list_store.append();
        list_store.set(&item2, &[(0, &false), (1, &true)]);
        // Group 2 (header + 1 selected)
        let header2 = list_store.append();
        list_store.set(&header2, &[(0, &true), (1, &false)]);
        let item3 = list_store.append();
        list_store.set(&item3, &[(0, &false), (1, &true)]);
        // Group 3 (header + 0 selected)
        let header3 = list_store.append();
        list_store.set(&header3, &[(0, &true), (1, &false)]);
        let (items, groups) = check_how_much_elements_is_selected(&tree_view, Some(0), 1);
        assert_eq!(items, 3);
        assert_eq!(groups, 2); // Only 2 groups have selected items
    }

    #[gtk4::test]
    fn test_clean_invalid_headers() {
        let columns_types: &[Type] = &[Type::BOOL, Type::STRING]; // header, path
        let list_store = gtk4::ListStore::new(columns_types);
        // Add a header with empty path (should be removed)
        let header1 = list_store.append();
        list_store.set(&header1, &[(0, &true), (1, &"")]);
        // Add a header with non-empty path (should be removed)
        let header2 = list_store.append();
        list_store.set(&header2, &[(0, &true), (1, &"/valid")]);
        clean_invalid_headers(&list_store, 0, 1);
        assert!(list_store.iter_first().is_none());
    }
}
