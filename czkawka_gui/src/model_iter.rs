use gtk4::prelude::*;
use gtk4::{ListStore, TreeIter};


pub fn iter_list<F, G>(model: &ListStore, f: F, init_asserts: G)
where
    F: Fn(&ListStore, &TreeIter) -> bool,
    G: Fn(&ListStore, &TreeIter) -> (),

{
    if let Some(iter) = model.iter_first() {
        init_asserts(model, &iter);
        loop {
            f(model, &iter);

            if !model.iter_next(&iter) {
                break;
            }
        }
    }
}

