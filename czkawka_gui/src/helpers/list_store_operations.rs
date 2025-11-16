use gtk4::prelude::*;
use gtk4::{ListStore, TreeView};

use crate::gui_structs::common_tree_view::{SubView, TreeViewListStoreTrait};
use crate::helpers::model_iter::{iter_list, iter_list_with_break, iter_list_with_break_init};

pub(crate) fn get_string_from_list_store(tree_view: &TreeView, column_full_path: i32, column_selection: Option<i32>) -> Vec<String> {
    let list_store: ListStore = tree_view.get_model();

    let mut string_vector: Vec<String> = Vec::new();

    match column_selection {
        Some(column_selection) => {
            iter_list(&list_store, |m, i| {
                if m.get::<bool>(i, column_selection) {
                    string_vector.push(m.get::<String>(i, column_full_path));
                }
            });
        }
        None => {
            iter_list(&list_store, |m, i| {
                string_vector.push(m.get::<String>(i, column_full_path));
            });
        }
    }

    string_vector
}

pub(crate) fn get_from_list_store_fnc<T>(tree_view: &TreeView, fnc: &dyn Fn(&ListStore, &gtk4::TreeIter, &mut Vec<T>)) -> Vec<T> {
    let list_store: ListStore = tree_view.get_model();

    let mut result_vector: Vec<T> = Vec::new();

    iter_list(&list_store, |m, i| {
        fnc(m, i, &mut result_vector);
    });

    result_vector
}

// After e.g. deleting files, header may become orphan or have one child, so should be deleted in this case
pub(crate) fn clean_invalid_headers(model: &ListStore, column_header: i32, column_path: i32) {
    // Remove only child from header
    if let Some(first_iter) = model.iter_first() {
        let mut vec_tree_path_to_delete: Vec<gtk4::TreePath> = Vec::new();
        let mut current_iter = first_iter;
        // First element should be header
        assert!(model.get::<bool>(&current_iter, column_header), "First deleted element, should be a header");

        let mut next_iter;
        let mut next_next_iter;

        // Empty means default check type
        if model.get::<String>(&current_iter, column_path).is_empty() {
            'main: loop {
                // First element should be header
                assert!(model.get::<bool>(&current_iter, column_header), "First deleted element, should be a header");

                next_iter = current_iter;
                if !model.iter_next(&next_iter) {
                    // There is only single header left (H1 -> END) -> (NOTHING)
                    vec_tree_path_to_delete.push(model.path(&current_iter));
                    break 'main;
                }

                if model.get::<bool>(&next_iter, column_header) {
                    // There are two headers each others(we remove just first) -> (H1 -> H2) -> (H2)
                    vec_tree_path_to_delete.push(model.path(&current_iter));
                    current_iter = next_iter;
                    continue 'main;
                }

                next_next_iter = next_iter;
                if !model.iter_next(&next_next_iter) {
                    // There is only one child of header left, so we remove it with header (H1 -> C1 -> END) -> (NOTHING)
                    vec_tree_path_to_delete.push(model.path(&current_iter));
                    vec_tree_path_to_delete.push(model.path(&next_iter));
                    break 'main;
                }

                if model.get::<bool>(&next_next_iter, column_header) {
                    // One child between two headers, we can remove them  (H1 -> C1 -> H2) -> (H2)
                    vec_tree_path_to_delete.push(model.path(&current_iter));
                    vec_tree_path_to_delete.push(model.path(&next_iter));
                    current_iter = next_next_iter;
                    continue 'main;
                }

                loop {
                    // (H1 -> C1 -> C2 -> Cn -> END) -> (NO CHANGE, BECAUSE IS GOOD)
                    if !model.iter_next(&next_next_iter) {
                        break 'main;
                    }
                    // Move to next header
                    if model.get::<bool>(&next_next_iter, column_header) {
                        current_iter = next_next_iter;
                        continue 'main;
                    }
                }
            }
        }
        // Non empty means that header points at reference folder
        else {
            'reference: loop {
                // First element should be header
                assert!(model.get::<bool>(&current_iter, column_header), "First deleted element, should be a header");

                next_iter = current_iter;
                if !model.iter_next(&next_iter) {
                    // There is only single header left (H1 -> END) -> (NOTHING)
                    vec_tree_path_to_delete.push(model.path(&current_iter));
                    break 'reference;
                }

                if model.get::<bool>(&next_iter, column_header) {
                    // There are two headers each others(we remove just first) -> (H1 -> H2) -> (H2)
                    vec_tree_path_to_delete.push(model.path(&current_iter));
                    current_iter = next_iter;
                    continue 'reference;
                }

                next_next_iter = next_iter;
                if !model.iter_next(&next_next_iter) {
                    // There is only one child of header left, so we remove it with header (H1 -> C1 -> END) -> (NOTHING)
                    break 'reference;
                }

                if model.get::<bool>(&next_next_iter, column_header) {
                    // One child between two headers, we can remove them  (H1 -> C1 -> H2) -> (H2)
                    current_iter = next_next_iter;
                    continue 'reference;
                }

                loop {
                    // (H1 -> C1 -> C2 -> Cn -> END) -> (NO CHANGE, BECAUSE IS GOOD)
                    if !model.iter_next(&next_next_iter) {
                        break 'reference;
                    }
                    // Move to next header
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

    // Last step, remove orphan header if exists
    if let Some(iter) = model.iter_first()
        && !model.iter_next(&iter)
    {
        model.clear();
    }
}

pub(crate) fn check_how_much_elements_is_selected(sv: &SubView) -> (u64, u64) {
    let mut number_of_selected_items: u64 = 0;
    let mut number_of_selected_groups: u64 = 0;

    let model = sv.get_model();

    let mut is_item_currently_selected_in_group: bool = false;

    if let Some(column_header) = sv.nb_object.column_header {
        iter_list_with_break_init(
            &model,
            |m, i| {
                assert!(m.get::<bool>(i, column_header)); // First element should be header
                m.iter_next(i)
            },
            |m, i| {
                if m.get::<bool>(i, column_header) {
                    is_item_currently_selected_in_group = false;
                } else if m.get::<bool>(i, sv.nb_object.column_selection) {
                    number_of_selected_items += 1;

                    if !is_item_currently_selected_in_group {
                        number_of_selected_groups += 1;
                    }
                    is_item_currently_selected_in_group = true;
                }
            },
        );
    } else {
        iter_list(&model, |m, i| {
            if m.get::<bool>(i, sv.nb_object.column_selection) {
                number_of_selected_items += 1;
            }
        });
    }

    (number_of_selected_items, number_of_selected_groups)
}

pub(crate) fn count_number_of_groups(sv: &SubView) -> u32 {
    let mut number_of_selected_groups = 0;
    let column_header = sv.nb_object.column_header.expect("Column header should be present to count number of groups");

    let model = sv.get_model();

    iter_list_with_break_init(
        &model,
        |_m, i| {
            assert!(model.get::<bool>(i, column_header)); // First element should be header
            true
        },
        |m, i| {
            if m.get::<bool>(i, column_header) {
                number_of_selected_groups += 1;
            }
        },
    );
    number_of_selected_groups
}

pub(crate) fn check_if_value_is_in_list_store(model: &ListStore, column: i32, value: &str) -> bool {
    let mut is_in_store = false;
    iter_list_with_break(model, |m, i| {
        if m.get::<String>(i, column) == value {
            is_in_store = true;
            return false;
        }
        true
    });

    is_in_store
}

pub(crate) fn check_if_list_store_column_have_all_same_values(model: &ListStore, column: i32, value: bool) -> bool {
    let mut all_are_same = false;
    iter_list_with_break(model, |m, i| {
        all_are_same = true;
        if m.get::<bool>(i, column) != value {
            all_are_same = false;
            return false;
        }

        true
    });

    all_are_same
}

pub(crate) fn append_row_to_list_store(list_store: &ListStore, values: &[(u32, &dyn ToValue)]) {
    list_store.set(&list_store.append(), values);
}

#[cfg(test)]
mod test {
    use glib::Value;
    use glib::types::Type;
    use gtk4::TreeView;
    use gtk4::prelude::*;

    use super::*;
    use crate::notebook_enums::NotebookMainEnum;
    use crate::notebook_info::NOTEBOOKS_INFO;

    // Helper to create a minimal SubView for Duplicate notebook along with its ListStore
    fn get_test_sv_duplicate() -> (crate::gui_structs::common_tree_view::SubView, gtk4::ListStore) {
        use std::cell::RefCell;
        use std::rc::Rc;

        use czkawka_core::tools::duplicate::DuplicateFinder;

        use crate::gui_structs::common_tree_view::SharedModelEnum;

        let nb_object = NOTEBOOKS_INFO[NotebookMainEnum::Duplicate as usize].clone();

        let list_store = gtk4::ListStore::new(nb_object.columns_types);
        let tree_view = gtk4::TreeView::new();
        tree_view.set_model(Some(&list_store));

        let scrolled_window = gtk4::ScrolledWindow::new();
        let gesture_click = gtk4::GestureClick::new();
        let event_controller_key = gtk4::EventControllerKey::new();
        tree_view.add_controller(event_controller_key.clone());
        tree_view.add_controller(gesture_click.clone());

        let sv = crate::gui_structs::common_tree_view::SubView {
            scrolled_window,
            tree_view,
            gesture_click,
            event_controller_key,
            nb_object,
            enum_value: NotebookMainEnum::Duplicate,
            preview_struct: None,
            shared_model_enum: SharedModelEnum::Duplicates(Rc::new(RefCell::new(None::<DuplicateFinder>))),
        };

        (sv, list_store)
    }

    #[gtk4::test]
    fn test_get_string_from_list_store() {
        let columns_types: &[Type] = &[Type::STRING];
        let list_store = gtk4::ListStore::new(columns_types);
        let tree_view = TreeView::with_model(&list_store);

        let values_to_add: &[(u32, &dyn ToValue)] = &[(0, &"test"), (0, &"test2"), (0, &"test3")];
        for i in values_to_add {
            append_row_to_list_store(&list_store, &[*i]);
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
            append_row_to_list_store(&list_store, i);
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
            append_row_to_list_store(&list_store, &[*i]);
        }
        assert!(!check_if_list_store_column_have_all_same_values(&list_store, 0, true));
        assert!(!check_if_list_store_column_have_all_same_values(&list_store, 0, false));

        list_store.clear();
        let values_to_add: &[(u32, &dyn ToValue)] = &[(0, &true), (0, &true), (0, &true)];
        for i in values_to_add {
            append_row_to_list_store(&list_store, &[*i]);
        }
        assert!(check_if_list_store_column_have_all_same_values(&list_store, 0, true));
        assert!(!check_if_list_store_column_have_all_same_values(&list_store, 0, false));

        list_store.clear();
        let values_to_add: &[(u32, &dyn ToValue)] = &[(0, &false)];
        for i in values_to_add {
            append_row_to_list_store(&list_store, &[*i]);
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
            append_row_to_list_store(&list_store, &[*i]);
        }
        assert!(check_if_value_is_in_list_store(&list_store, 0, "Koczkodan"));
        assert!(check_if_value_is_in_list_store(&list_store, 0, "Kachir"));
        assert!(!check_if_value_is_in_list_store(&list_store, 0, "Koczkodan2"));

        let columns_types: &[Type] = &[Type::STRING, Type::STRING];
        let list_store = gtk4::ListStore::new(columns_types);
        let values_to_add: &[&[(u32, &dyn ToValue)]] = &[&[(0, &"Koczkodan"), (1, &"Krakus")], &[(0, &"Kachir"), (1, &"Wodnica")]];
        for i in values_to_add {
            append_row_to_list_store(&list_store, i);
        }
        assert!(check_if_value_is_in_list_store(&list_store, 0, "Koczkodan"));
        assert!(check_if_value_is_in_list_store(&list_store, 1, "Krakus"));
        assert!(check_if_value_is_in_list_store(&list_store, 0, "Kachir"));
        assert!(check_if_value_is_in_list_store(&list_store, 1, "Wodnica"));
        assert!(!check_if_value_is_in_list_store(&list_store, 0, "Krakus"));
        assert!(!check_if_value_is_in_list_store(&list_store, 1, "Kachir"));
    }

    #[gtk4::test]
    fn test_count_number_of_groups() {
        // Use helper that builds SubView + ListStore
        let (sv, list_store) = get_test_sv_duplicate();

        let column_header = sv.nb_object.column_header.expect("Duplicate NB must have header column");

        // Build rows: H, C, H, C -> expected 2 groups
        append_row_to_list_store(&list_store, &[(column_header as u32, &Into::<Value>::into(true))]);
        append_row_to_list_store(&list_store, &[(column_header as u32, &Into::<Value>::into(false))]);
        append_row_to_list_store(&list_store, &[(column_header as u32, &Into::<Value>::into(true))]);
        append_row_to_list_store(&list_store, &[(column_header as u32, &Into::<Value>::into(false))]);

        assert_eq!(crate::helpers::list_store_operations::count_number_of_groups(&sv), 2);
    }

    #[gtk4::test]
    fn test_check_how_much_elements_is_selected() {
        let (sv, list_store) = get_test_sv_duplicate();

        let column_header = sv.nb_object.column_header.expect("Duplicate NB must have header column");
        let column_selection = sv.nb_object.column_selection;

        // Build rows: H, C(selected), C(not selected), H, C(selected) => 2 selected items in 2 groups
        append_row_to_list_store(
            &list_store,
            &[(column_header as u32, &Into::<Value>::into(true)), (column_selection as u32, &Into::<Value>::into(false))],
        );
        append_row_to_list_store(
            &list_store,
            &[(column_header as u32, &Into::<Value>::into(false)), (column_selection as u32, &Into::<Value>::into(true))],
        );
        append_row_to_list_store(
            &list_store,
            &[(column_header as u32, &Into::<Value>::into(false)), (column_selection as u32, &Into::<Value>::into(false))],
        );
        append_row_to_list_store(
            &list_store,
            &[(column_header as u32, &Into::<Value>::into(true)), (column_selection as u32, &Into::<Value>::into(false))],
        );
        append_row_to_list_store(
            &list_store,
            &[(column_header as u32, &Into::<Value>::into(false)), (column_selection as u32, &Into::<Value>::into(true))],
        );

        let res = check_how_much_elements_is_selected(&sv);
        assert_eq!(res, (2, 2));
    }

    #[gtk4::test]
    fn test_get_from_list_store_fnc() {
        let columns_types: &[Type] = &[Type::STRING];
        let list_store = gtk4::ListStore::new(columns_types);
        let tree_view = TreeView::with_model(&list_store);

        // Append literals directly to avoid lifetime/coercion issues
        append_row_to_list_store(&list_store, &[(0, &"a")]);
        append_row_to_list_store(&list_store, &[(0, &"b")]);
        append_row_to_list_store(&list_store, &[(0, &"c")]);

        let collected: Vec<String> = get_from_list_store_fnc(&tree_view, &|m, i, vec: &mut Vec<String>| {
            vec.push(m.get::<String>(i, 0));
        });

        assert_eq!(collected, vec!["a".to_string(), "b".to_string(), "c".to_string()]);
    }
}
