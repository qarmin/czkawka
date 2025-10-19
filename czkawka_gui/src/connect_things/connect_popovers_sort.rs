use std::fmt::Debug;

use gtk4::prelude::*;
use gtk4::{ListStore, TreeIter};

use crate::gui_structs::common_tree_view::SubView;
use crate::gui_structs::gui_data::GuiData;
use crate::help_functions::get_list_store;
use crate::model_iter::iter_list;

fn popover_sort_general_abs<T>(popover: &gtk4::Popover, sv: &SubView)
where
    T: Ord + for<'b> glib::value::FromValue<'b> + 'static + Debug,
{
    popover_sort_general::<T>(
        popover,
        &sv.tree_view,
        sv.nb_object.column_size_as_bytes.expect("Failed to get size as bytes column"),
        sv.nb_object.column_header.expect("Failed to get header column"),
    );
}

fn popover_sort_general<T>(popover: &gtk4::Popover, tree_view: &gtk4::TreeView, column_sort: i32, column_header: i32)
where
    T: Ord + for<'b> glib::value::FromValue<'b> + 'static + Debug,
{
    let model = get_list_store(tree_view);

    if let Some(mut curr_iter) = model.iter_first() {
        assert!(model.get::<bool>(&curr_iter, column_header));
        assert!(model.iter_next(&curr_iter));
        loop {
            let mut iters = Vec::new();
            let mut all_have = false;
            let local_iter = curr_iter;
            loop {
                if model.get::<bool>(&local_iter, column_header) {
                    if !model.iter_next(&local_iter) {
                        all_have = true;
                    }
                    break;
                }
                iters.push(local_iter);
                if !model.iter_next(&local_iter) {
                    all_have = true;
                    break;
                }
            }
            if iters.len() == 1 {
                curr_iter = local_iter;
                if all_have {
                    break;
                }
                continue;
            }
            sort_iters::<T>(&model, iters, column_sort);
            curr_iter = local_iter;
            if all_have {
                break;
            }
        }
    }
    popover.popdown();
}

fn sort_iters<T>(model: &ListStore, mut iters: Vec<TreeIter>, column_sort: i32)
where
    T: Ord + for<'b> glib::value::FromValue<'b> + 'static + Debug,
{
    assert!(iters.len() >= 2);
    loop {
        let mut changed_item = false;
        for idx in 0..(iters.len() - 1) {
            if model.get::<T>(&iters[idx], column_sort) > model.get::<T>(&iters[idx + 1], column_sort) {
                model.swap(&iters[idx], &iters[idx + 1]);
                iters.swap(idx, idx + 1);
                changed_item = true;
            }
        }
        if !changed_item {
            return;
        }
    }
}

pub(crate) fn connect_popover_sort(gui_data: &GuiData) {
    let popover_sort = gui_data.popovers_sort.popover_sort.clone();
    let buttons_popover_file_name = gui_data.popovers_sort.buttons_popover_sort_file_name.clone();
    let common_tree_views = gui_data.main_notebook.common_tree_views.clone();

    buttons_popover_file_name.connect_clicked(move |_| {
        popover_sort_general_abs::<String>(&popover_sort, common_tree_views.get_current_subview());
    });

    let popover_sort = gui_data.popovers_sort.popover_sort.clone();
    let buttons_popover_sort_folder_name = gui_data.popovers_sort.buttons_popover_sort_folder_name.clone();
    let common_tree_views = gui_data.main_notebook.common_tree_views.clone();

    buttons_popover_sort_folder_name.connect_clicked(move |_| {
        popover_sort_general_abs::<String>(&popover_sort, common_tree_views.get_current_subview());
    });

    let popover_sort = gui_data.popovers_sort.popover_sort.clone();
    let buttons_popover_sort_selection = gui_data.popovers_sort.buttons_popover_sort_selection.clone();
    let common_tree_views = gui_data.main_notebook.common_tree_views.clone();

    buttons_popover_sort_selection.connect_clicked(move |_| {
        popover_sort_general_abs::<bool>(&popover_sort, common_tree_views.get_current_subview());
    });

    let popover_sort = gui_data.popovers_sort.popover_sort.clone();
    let buttons_popover_sort_size = gui_data.popovers_sort.buttons_popover_sort_size.clone();
    let common_tree_views = gui_data.main_notebook.common_tree_views.clone();
    buttons_popover_sort_size.connect_clicked(move |_| {
        popover_sort_general_abs::<u64>(&popover_sort, common_tree_views.get_current_subview());
    });
}

#[cfg(test)]
mod test {
    use glib::types::Type;
    use gtk4::prelude::*;
    use gtk4::{Popover, TreeView};
    use rand::random;

    use crate::connect_things::connect_popovers_sort::{popover_sort_general, sort_iters};
    use crate::help_functions::append_row_to_list_store;

    #[gtk4::test]
    fn test_sort_iters() {
        let columns_types: &[Type] = &[Type::U32, Type::STRING];
        let list_store = gtk4::ListStore::new(columns_types);

        let values_to_add: &[&[(u32, &dyn ToValue)]] = &[&[(0, &2), (1, &"AAA")], &[(0, &3), (1, &"CCC")], &[(0, &1), (1, &"BBB")]];
        for i in values_to_add {
            append_row_to_list_store(&list_store, i);
        }
        let mut iters = Vec::new();
        let iter = list_store.iter_first().expect("Failed to get first iter");
        iters.push(iter);
        list_store.iter_next(&iter);
        iters.push(iter);
        list_store.iter_next(&iter);
        iters.push(iter);

        sort_iters::<String>(&list_store, iters, 1);

        let expected = [(2, "AAA"), (1, "BBB"), (3, "CCC")];
        let curr_iter = list_store.iter_first().expect("Failed to get first iter");
        for exp in expected {
            let real_0 = list_store.get::<u32>(&curr_iter, 0);
            assert_eq!(real_0, exp.0);
            let real_1 = list_store.get::<String>(&curr_iter, 1);
            assert_eq!(real_1, exp.1);
            list_store.iter_next(&curr_iter);
        }
    }

    #[gtk4::test]
    pub(crate) fn test_popover_sort_general_simple() {
        let columns_types: &[Type] = &[Type::BOOL, Type::STRING];
        let list_store = gtk4::ListStore::new(columns_types);
        let tree_view = TreeView::builder().model(&list_store).build();
        let popover = Popover::new();

        let values_to_add: &[&[(u32, &dyn ToValue)]] = &[&[(0, &true), (1, &"DDD")], &[(0, &false), (1, &"CCC")], &[(0, &false), (1, &"BBB")]];
        for i in values_to_add {
            append_row_to_list_store(&list_store, i);
        }

        popover_sort_general::<String>(&popover, &tree_view, 1, 0);

        let expected = ["DDD", "BBB", "CCC"];
        let curr_iter = list_store.iter_first().expect("Failed to get first iter");
        for exp in expected {
            let real = list_store.get::<String>(&curr_iter, 1);
            assert_eq!(real, exp);
            list_store.iter_next(&curr_iter);
        }
    }

    #[gtk4::test]
    pub(crate) fn test_popover_sort_general() {
        let columns_types: &[Type] = &[Type::BOOL, Type::STRING];
        let list_store = gtk4::ListStore::new(columns_types);
        let tree_view = TreeView::builder().model(&list_store).build();
        let popover = Popover::new();

        let values_to_add: &[&[(u32, &dyn ToValue)]] = &[
            &[(0, &true), (1, &"AAA")],
            &[(0, &false), (1, &"CCC")],
            &[(0, &false), (1, &"BBB")],
            &[(0, &true), (1, &"TTT")],
            &[(0, &false), (1, &"PPP")],
            &[(0, &false), (1, &"AAA")],
            &[(0, &true), (1, &"RRR")],
            &[(0, &false), (1, &"WWW")],
            &[(0, &false), (1, &"ZZZ")],
        ];
        for i in values_to_add {
            append_row_to_list_store(&list_store, i);
        }

        popover_sort_general::<String>(&popover, &tree_view, 1, 0);

        let expected = ["AAA", "BBB", "CCC", "TTT", "AAA", "PPP", "RRR", "WWW", "ZZZ"];
        let curr_iter = list_store.iter_first().expect("Failed to get first iter");
        for exp in expected {
            let real = list_store.get::<String>(&curr_iter, 1);
            assert_eq!(real, exp);
            list_store.iter_next(&curr_iter);
        }
    }

    #[gtk4::test]
    pub(crate) fn fuzzer_test() {
        for _ in 0..1000 {
            let columns_types: &[Type] = &[Type::BOOL, Type::STRING];
            let list_store = gtk4::ListStore::new(columns_types);
            let tree_view = TreeView::builder().model(&list_store).build();
            let popover = Popover::new();

            // Always start with a header
            let first_row: &[(u32, &dyn ToValue)] = &[(0, &true), (1, &"AAA")];
            append_row_to_list_store(&list_store, first_row);

            let mut since_last_header = 0;
            let mut need_header = false;
            let num_rows = (random::<u32>() % 10 + 5) as usize;
            let mut i = 0;
            while i < num_rows {
                if need_header {
                    // Insert a header only if last was not a header
                    let a: Vec<(u32, &dyn ToValue)> = vec![(0, &true), (1, &"HEADER")];
                    append_row_to_list_store(&list_store, &a);
                    since_last_header = 0;
                    need_header = false;
                    i += 1;
                    continue;
                }
                // Insert a non-header row
                let string_val = rand::random::<u32>().to_string();
                let a: Vec<(u32, &dyn ToValue)> = vec![(0, &false), (1, &string_val)];
                append_row_to_list_store(&list_store, &a);
                since_last_header += 1;
                // After at least 2 non-header rows, randomly decide to insert a header next
                if since_last_header >= 2 && random::<u8>().is_multiple_of(3) {
                    need_header = true;
                }
                i += 1;
            }

            // Ensure at least one non-header after the last header
            let last_iter = list_store.iter_first().expect("TEST");
            let mut last_is_header;
            loop {
                last_is_header = list_store.get::<bool>(&last_iter, 0);
                if !list_store.iter_next(&last_iter) {
                    break;
                }
            }
            if last_is_header {
                let a: Vec<(u32, &dyn ToValue)> = vec![(0, &false), (1, &"FINALROW")];
                append_row_to_list_store(&list_store, &a);
            }

            popover_sort_general::<String>(&popover, &tree_view, 1, 0);
        }
    }
}
