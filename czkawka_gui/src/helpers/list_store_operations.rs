use fun_time::fun_time;
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
#[fun_time(message = "clean_invalid_headers", level = "debug")]
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
                if !model.iter_next(&mut next_iter) {
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
                if !model.iter_next(&mut next_next_iter) {
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
                    if !model.iter_next(&mut next_next_iter) {
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
                if !model.iter_next(&mut next_iter) {
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
                if !model.iter_next(&mut next_next_iter) {
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
                    if !model.iter_next(&mut next_next_iter) {
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
    if let Some(mut iter) = model.iter_first()
        && !model.iter_next(&mut iter)
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
    fn returns_empty_vector_when_list_store_is_empty() {
        let columns_types: &[Type] = &[Type::STRING];
        let list_store = gtk4::ListStore::new(columns_types);
        let tree_view = TreeView::with_model(&list_store);

        assert_eq!(get_string_from_list_store(&tree_view, 0, None), Vec::<String>::new());
    }

    #[gtk4::test]
    fn filters_by_boolean_column_when_selection_specified() {
        let columns_types: &[Type] = &[Type::BOOL, Type::STRING];
        let list_store = gtk4::ListStore::new(columns_types);
        let tree_view = TreeView::with_model(&list_store);

        let values_to_add: &[&[(u32, &dyn ToValue)]] = &[
            &[(0, &Into::<Value>::into(true)), (1, &Into::<Value>::into("selected1"))],
            &[(0, &Into::<Value>::into(false)), (1, &Into::<Value>::into("not_selected"))],
            &[(0, &Into::<Value>::into(true)), (1, &Into::<Value>::into("selected2"))],
        ];
        for i in values_to_add {
            append_row_to_list_store(&list_store, i);
        }

        assert_eq!(get_string_from_list_store(&tree_view, 1, Some(0)), vec!["selected1".to_string(), "selected2".to_string()]);
    }

    #[gtk4::test]
    fn applies_custom_function_to_all_rows() {
        let columns_types: &[Type] = &[Type::STRING, Type::I32];
        let list_store = gtk4::ListStore::new(columns_types);
        let tree_view = TreeView::with_model(&list_store);

        append_row_to_list_store(&list_store, &[(0, &"row1"), (1, &10)]);
        append_row_to_list_store(&list_store, &[(0, &"row2"), (1, &20)]);
        append_row_to_list_store(&list_store, &[(0, &"row3"), (1, &30)]);

        let collected: Vec<(String, i32)> = get_from_list_store_fnc(&tree_view, &|m, i, vec: &mut Vec<(String, i32)>| {
            vec.push((m.get::<String>(i, 0), m.get::<i32>(i, 1)));
        });

        assert_eq!(collected, vec![("row1".to_string(), 10), ("row2".to_string(), 20), ("row3".to_string(), 30)]);
    }

    #[gtk4::test]
    fn removes_single_orphan_header_with_empty_path() {
        let columns_types: &[Type] = &[Type::BOOL, Type::STRING];
        let list_store = gtk4::ListStore::new(columns_types);

        append_row_to_list_store(&list_store, &[(0, &true), (1, &"")]);

        clean_invalid_headers(&list_store, 0, 1);

        assert!(list_store.iter_first().is_none());
    }

    #[gtk4::test]
    fn cleans_invalid_headers_properly() {
        let columns_types: &[Type] = &[Type::BOOL, Type::STRING];
        let list_store = gtk4::ListStore::new(columns_types);

        // Scenario: H1 -> C1 -> H2 -> C2 -> C3
        // After cleaning: H2 -> C2 -> C3 (H1 and C1 removed as H1 has only one child)
        append_row_to_list_store(&list_store, &[(0, &true), (1, &"")]);
        append_row_to_list_store(&list_store, &[(0, &false), (1, &"/path/file1.txt")]);
        append_row_to_list_store(&list_store, &[(0, &true), (1, &"")]);
        append_row_to_list_store(&list_store, &[(0, &false), (1, &"/path/file2.txt")]);
        append_row_to_list_store(&list_store, &[(0, &false), (1, &"/path/file3.txt")]);

        clean_invalid_headers(&list_store, 0, 1);

        let count = list_store.iter_n_children(None);
        assert_eq!(count, 3, "Should keep header with 2 children");
    }

    #[gtk4::test]
    fn keeps_header_with_multiple_children_when_path_empty() {
        let columns_types: &[Type] = &[Type::BOOL, Type::STRING];
        let list_store = gtk4::ListStore::new(columns_types);

        append_row_to_list_store(&list_store, &[(0, &true), (1, &"")]);
        append_row_to_list_store(&list_store, &[(0, &false), (1, &"/path/file1.txt")]);
        append_row_to_list_store(&list_store, &[(0, &false), (1, &"/path/file2.txt")]);

        clean_invalid_headers(&list_store, 0, 1);

        let mut count = 0;
        iter_list(&list_store, |_, _| count += 1);
        assert_eq!(count, 3);
    }

    #[gtk4::test]
    fn keeps_reference_folder_header_with_single_child() {
        let columns_types: &[Type] = &[Type::BOOL, Type::STRING];
        let list_store = gtk4::ListStore::new(columns_types);

        append_row_to_list_store(&list_store, &[(0, &true), (1, &"/reference/path")]);
        append_row_to_list_store(&list_store, &[(0, &false), (1, &"/some/child")]);

        clean_invalid_headers(&list_store, 0, 1);

        let mut count = 0;
        iter_list(&list_store, |_, _| count += 1);
        assert_eq!(count, 2);
    }

    #[gtk4::test]
    fn counts_items_and_groups_correctly() {
        let (sv, list_store) = get_test_sv_duplicate();

        let column_header = sv.nb_object.column_header.expect("Duplicate NB must have header column");
        let column_selection = sv.nb_object.column_selection;

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
            &[(column_header as u32, &Into::<Value>::into(false)), (column_selection as u32, &Into::<Value>::into(true))],
        );
        append_row_to_list_store(
            &list_store,
            &[(column_header as u32, &Into::<Value>::into(true)), (column_selection as u32, &Into::<Value>::into(false))],
        );
        append_row_to_list_store(
            &list_store,
            &[(column_header as u32, &Into::<Value>::into(false)), (column_selection as u32, &Into::<Value>::into(false))],
        );
        append_row_to_list_store(
            &list_store,
            &[(column_header as u32, &Into::<Value>::into(false)), (column_selection as u32, &Into::<Value>::into(true))],
        );

        let (items, groups) = check_how_much_elements_is_selected(&sv);
        assert_eq!(items, 3);
        assert_eq!(groups, 2);
    }

    #[gtk4::test]
    fn returns_zero_when_nothing_selected() {
        let (sv, list_store) = get_test_sv_duplicate();

        let column_header = sv.nb_object.column_header.expect("Duplicate NB must have header column");
        let column_selection = sv.nb_object.column_selection;

        append_row_to_list_store(
            &list_store,
            &[(column_header as u32, &Into::<Value>::into(true)), (column_selection as u32, &Into::<Value>::into(false))],
        );
        append_row_to_list_store(
            &list_store,
            &[(column_header as u32, &Into::<Value>::into(false)), (column_selection as u32, &Into::<Value>::into(false))],
        );

        let (items, groups) = check_how_much_elements_is_selected(&sv);
        assert_eq!(items, 0);
        assert_eq!(groups, 0);
    }

    #[gtk4::test]
    fn returns_correct_count_of_groups() {
        let (sv, list_store) = get_test_sv_duplicate();

        let column_header = sv.nb_object.column_header.expect("Duplicate NB must have header column");

        append_row_to_list_store(&list_store, &[(column_header as u32, &Into::<Value>::into(true))]);
        append_row_to_list_store(&list_store, &[(column_header as u32, &Into::<Value>::into(false))]);
        append_row_to_list_store(&list_store, &[(column_header as u32, &Into::<Value>::into(true))]);
        append_row_to_list_store(&list_store, &[(column_header as u32, &Into::<Value>::into(false))]);
        append_row_to_list_store(&list_store, &[(column_header as u32, &Into::<Value>::into(true))]);

        assert_eq!(count_number_of_groups(&sv), 3);
    }

    #[gtk4::test]
    fn finds_existing_values_in_different_columns() {
        let columns_types: &[Type] = &[Type::STRING, Type::STRING];
        let list_store = gtk4::ListStore::new(columns_types);

        let values_to_add: &[&[(u32, &dyn ToValue)]] = &[&[(0, &"key1"), (1, &"value1")], &[(0, &"key2"), (1, &"value2")], &[(0, &"key3"), (1, &"value3")]];
        for i in values_to_add {
            append_row_to_list_store(&list_store, i);
        }

        assert!(check_if_value_is_in_list_store(&list_store, 0, "key2"));
        assert!(check_if_value_is_in_list_store(&list_store, 1, "value3"));
        assert!(!check_if_value_is_in_list_store(&list_store, 0, "nonexistent"));
        assert!(!check_if_value_is_in_list_store(&list_store, 1, "key1"));
    }

    #[gtk4::test]
    fn detects_uniform_values_in_column() {
        let columns_types: &[Type] = &[Type::BOOL];
        let list_store = gtk4::ListStore::new(columns_types);

        append_row_to_list_store(&list_store, &[(0, &true)]);
        append_row_to_list_store(&list_store, &[(0, &true)]);
        append_row_to_list_store(&list_store, &[(0, &true)]);

        assert!(check_if_list_store_column_have_all_same_values(&list_store, 0, true));
        assert!(!check_if_list_store_column_have_all_same_values(&list_store, 0, false));
    }

    #[gtk4::test]
    fn detects_mixed_values_in_column() {
        let columns_types: &[Type] = &[Type::BOOL];
        let list_store = gtk4::ListStore::new(columns_types);

        append_row_to_list_store(&list_store, &[(0, &true)]);
        append_row_to_list_store(&list_store, &[(0, &false)]);
        append_row_to_list_store(&list_store, &[(0, &true)]);

        assert!(!check_if_list_store_column_have_all_same_values(&list_store, 0, true));
        assert!(!check_if_list_store_column_have_all_same_values(&list_store, 0, false));
    }

    #[gtk4::test]
    fn adds_row_with_multiple_columns() {
        let columns_types: &[Type] = &[Type::STRING, Type::I32, Type::BOOL];
        let list_store = gtk4::ListStore::new(columns_types);

        append_row_to_list_store(&list_store, &[(0, &"test"), (1, &42), (2, &true)]);

        let iter = list_store.iter_first().expect("Should have a row");
        assert_eq!(list_store.get::<String>(&iter, 0), "test");
        assert_eq!(list_store.get::<i32>(&iter, 1), 42);
        assert!(list_store.get::<bool>(&iter, 2));
    }
}
