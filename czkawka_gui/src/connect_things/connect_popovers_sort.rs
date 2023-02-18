use gtk4::prelude::*;
use gtk4::{ListStore, TreeIter};

use crate::gui_structs::gui_data::GuiData;
use crate::help_functions::*;
use crate::notebook_info::NOTEBOOKS_INFO;

fn popover_sort_general<T>(popover: &gtk4::Popover, tree_view: &gtk4::TreeView, column_sort: i32, column_header: i32)
where
    T: Ord + for<'b> glib::value::FromValue<'b> + 'static + std::fmt::Debug,
{
    let model = get_list_store(tree_view);

    if let Some(curr_iter) = model.iter_first() {
        assert!(model.get::<bool>(&curr_iter, column_header));
        assert!(model.iter_next(&curr_iter)); // Must be at least one item
        loop {
            let mut iters = Vec::new();
            let mut all_have = false;
            loop {
                if model.get::<bool>(&curr_iter, column_header) {
                    assert!(model.iter_next(&curr_iter), "Empty header, this should not happens");
                    break;
                }
                iters.push(curr_iter);
                if !model.iter_next(&curr_iter) {
                    all_have = true;
                    break;
                }
            }
            if iters.len() == 1 {
                continue; // Can be equal 1 in reference folders
            }

            sort_iters::<T>(&model, iters, column_sort);
            if all_have {
                break;
            }
        }
    }
    popover.popdown();
}

fn sort_iters<T>(model: &ListStore, mut iters: Vec<TreeIter>, column_sort: i32)
where
    T: Ord + for<'b> glib::value::FromValue<'b> + 'static,
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

pub fn connect_popover_sort(gui_data: &GuiData) {
    let popover_sort = gui_data.popovers_sort.popover_sort.clone();
    let buttons_popover_file_name = gui_data.popovers_sort.buttons_popover_sort_file_name.clone();
    let notebook_main = gui_data.main_notebook.notebook_main.clone();
    let main_tree_views = gui_data.main_notebook.get_main_tree_views();

    buttons_popover_file_name.connect_clicked(move |_| {
        let nb_number = notebook_main.current_page().unwrap();
        let tree_view = &main_tree_views[nb_number as usize];
        let nb_object = &NOTEBOOKS_INFO[nb_number as usize];

        popover_sort_general::<String>(&popover_sort, tree_view, nb_object.column_name, nb_object.column_header.unwrap());
    });
}

#[cfg(test)]
mod test {
    use crate::connect_things::connect_popovers_sort::{popover_sort_general, sort_iters};
    use gtk4::prelude::*;
    use gtk4::{Popover, TreeView};

    #[gtk4::test]
    fn test_sort_iters() {
        let columns_types: &[glib::types::Type] = &[glib::types::Type::U32, glib::types::Type::STRING];
        let list_store = gtk4::ListStore::new(columns_types);

        let values_to_add: &[&[(u32, &dyn ToValue)]] = &[&[(0, &2), (1, &"AAA")], &[(0, &3), (1, &"CCC")], &[(0, &1), (1, &"BBB")]];
        for i in values_to_add {
            list_store.set(&list_store.append(), i);
        }
        let mut iters = Vec::new();
        let iter = list_store.iter_first().unwrap();
        iters.push(iter.clone());
        list_store.iter_next(&iter);
        iters.push(iter.clone());
        list_store.iter_next(&iter);
        iters.push(iter.clone());

        sort_iters::<String>(&list_store, iters, 1);

        let first = list_store.iter_first().unwrap();
        let second = first.clone();
        list_store.iter_next(&second);
        let third = second.clone();
        list_store.iter_next(&third);

        assert_eq!(list_store.get::<String>(&first, 1), "AAA");
        assert_eq!(list_store.get::<String>(&second, 1), "BBB");
        assert_eq!(list_store.get::<String>(&third, 1), "CCC");

        assert_eq!(list_store.get::<u32>(&first, 0), 2);
        assert_eq!(list_store.get::<u32>(&second, 0), 1);
        assert_eq!(list_store.get::<u32>(&third, 0), 3);
    }

    #[gtk4::test]
    pub fn test_popover_sort_general_simple() {
        let columns_types: &[glib::types::Type] = &[glib::types::Type::BOOL, glib::types::Type::STRING];
        let list_store = gtk4::ListStore::new(columns_types);
        let tree_view = TreeView::builder().model(&list_store).build();
        let popover = Popover::new();

        let values_to_add: &[&[(u32, &dyn ToValue)]] = &[&[(0, &true), (1, &"DDD")], &[(0, &false), (1, &"CCC")], &[(0, &false), (1, &"BBB")]];
        for i in values_to_add {
            list_store.set(&list_store.append(), i);
        }

        popover_sort_general::<String>(&popover, &tree_view, 1, 0);

        let expected = ["DDD", "BBB", "CCC"];
        let curr_iter = list_store.iter_first().unwrap();
        for exp in expected {
            let real = list_store.get::<String>(&curr_iter, 1);
            assert_eq!(real, exp);
            list_store.iter_next(&curr_iter);
        }
    }

    #[gtk4::test]
    pub fn test_popover_sort_general() {
        let columns_types: &[glib::types::Type] = &[glib::types::Type::BOOL, glib::types::Type::STRING];
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
            list_store.set(&list_store.append(), i);
        }

        popover_sort_general::<String>(&popover, &tree_view, 1, 0);

        let expected = ["AAA", "BBB", "CCC", "TTT", "AAA", "PPP", "RRR", "WWW", "ZZZ"];
        let curr_iter = list_store.iter_first().unwrap();
        for exp in expected {
            let real = list_store.get::<String>(&curr_iter, 1);
            assert_eq!(real, exp);
            list_store.iter_next(&curr_iter);
        }
    }
}
