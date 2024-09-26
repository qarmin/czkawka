use std::fmt::Debug;

use gtk4::prelude::*;
use gtk4::{ListStore, TreeIter};

use crate::gui_structs::gui_data::GuiData;
use crate::help_functions::*;
use crate::notebook_info::NOTEBOOKS_INFO;

fn popover_sort_general<T>(popover: &gtk4::Popover, tree_view: &gtk4::TreeView, column_sort: i32, column_header: i32)
where
    T: Ord + for<'b> glib::value::FromValue<'b> + 'static + Debug,
{
    let model = get_list_store(tree_view);

    if let Some(curr_iter) = model.iter_first() {
        assert!(model.get::<bool>(&curr_iter, column_header)); // First item should be header
        assert!(model.iter_next(&curr_iter)); // Must be at least two items
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

pub fn connect_popover_sort(gui_data: &GuiData) {
    let popover_sort = gui_data.popovers_sort.popover_sort.clone();
    let buttons_popover_file_name = gui_data.popovers_sort.buttons_popover_sort_file_name.clone();
    let notebook_main = gui_data.main_notebook.notebook_main.clone();
    let main_tree_views = gui_data.main_notebook.get_main_tree_views();

    buttons_popover_file_name.connect_clicked(move |_| {
        let nb_number = notebook_main.current_page().expect("Current page not set");
        let tree_view = &main_tree_views[nb_number as usize];
        let nb_object = &NOTEBOOKS_INFO[nb_number as usize];

        popover_sort_general::<String>(
            &popover_sort,
            tree_view,
            nb_object.column_name,
            nb_object.column_header.expect("Failed to get header column"),
        );
    });

    let popover_sort = gui_data.popovers_sort.popover_sort.clone();
    let buttons_popover_sort_folder_name = gui_data.popovers_sort.buttons_popover_sort_folder_name.clone();
    let notebook_main = gui_data.main_notebook.notebook_main.clone();
    let main_tree_views = gui_data.main_notebook.get_main_tree_views();

    buttons_popover_sort_folder_name.connect_clicked(move |_| {
        let nb_number = notebook_main.current_page().expect("Current page not set");
        let tree_view = &main_tree_views[nb_number as usize];
        let nb_object = &NOTEBOOKS_INFO[nb_number as usize];

        popover_sort_general::<String>(
            &popover_sort,
            tree_view,
            nb_object.column_path,
            nb_object.column_header.expect("Failed to get header column"),
        );
    });

    let popover_sort = gui_data.popovers_sort.popover_sort.clone();
    let buttons_popover_sort_selection = gui_data.popovers_sort.buttons_popover_sort_selection.clone();
    let notebook_main = gui_data.main_notebook.notebook_main.clone();
    let main_tree_views = gui_data.main_notebook.get_main_tree_views();

    buttons_popover_sort_selection.connect_clicked(move |_| {
        let nb_number = notebook_main.current_page().expect("Current page not set");
        let tree_view = &main_tree_views[nb_number as usize];
        let nb_object = &NOTEBOOKS_INFO[nb_number as usize];

        popover_sort_general::<bool>(
            &popover_sort,
            tree_view,
            nb_object.column_selection,
            nb_object.column_header.expect("Failed to get header column"),
        );
    });

    let popover_sort = gui_data.popovers_sort.popover_sort.clone();
    let buttons_popover_sort_size = gui_data.popovers_sort.buttons_popover_sort_size.clone();
    let notebook_main = gui_data.main_notebook.notebook_main.clone();
    let main_tree_views = gui_data.main_notebook.get_main_tree_views();

    buttons_popover_sort_size.connect_clicked(move |_| {
        let nb_number = notebook_main.current_page().expect("Current page not set");
        let tree_view = &main_tree_views[nb_number as usize];
        let nb_object = &NOTEBOOKS_INFO[nb_number as usize];

        popover_sort_general::<u64>(
            &popover_sort,
            tree_view,
            nb_object.column_size_as_bytes.expect("Failed to get size as bytes column"),
            nb_object.column_header.expect("Failed to get header column"),
        );
    });
}

#[cfg(test)]
mod test {
    use glib::types::Type;
    use gtk4::prelude::*;
    use gtk4::{Popover, TreeView};

    use crate::connect_things::connect_popovers_sort::{popover_sort_general, sort_iters};

    #[gtk4::test]
    fn test_sort_iters() {
        let columns_types: &[Type] = &[Type::U32, Type::STRING];
        let list_store = gtk4::ListStore::new(columns_types);

        let values_to_add: &[&[(u32, &dyn ToValue)]] = &[&[(0, &2), (1, &"AAA")], &[(0, &3), (1, &"CCC")], &[(0, &1), (1, &"BBB")]];
        for i in values_to_add {
            list_store.set(&list_store.append(), i);
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
    pub fn test_popover_sort_general_simple() {
        let columns_types: &[Type] = &[Type::BOOL, Type::STRING];
        let list_store = gtk4::ListStore::new(columns_types);
        let tree_view = TreeView::builder().model(&list_store).build();
        let popover = Popover::new();

        let values_to_add: &[&[(u32, &dyn ToValue)]] = &[&[(0, &true), (1, &"DDD")], &[(0, &false), (1, &"CCC")], &[(0, &false), (1, &"BBB")]];
        for i in values_to_add {
            list_store.set(&list_store.append(), i);
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
    pub fn test_popover_sort_general() {
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
            list_store.set(&list_store.append(), i);
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
}
