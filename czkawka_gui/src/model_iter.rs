use gtk4::prelude::*;
use gtk4::{ListStore, TreeIter};

pub fn iter_list_with_break_init<G, F>(model: &ListStore, init: G, mut f: F)
where
    G: Fn(&ListStore, &TreeIter) -> bool,
    F: FnMut(&ListStore, &TreeIter),
{
    if let Some(iter) = model.iter_first() {
        if !init(model, &iter) {
            return;
        }
        loop {
            f(model, &iter);

            if !model.iter_next(&iter) {
                break;
            }
        }
    }
}
pub fn iter_list_break_with_init<G, F>(model: &ListStore, init: G, mut f: F)
where
    G: Fn(&ListStore, &TreeIter),
    F: FnMut(&ListStore, &TreeIter) -> bool,
{
    if let Some(iter) = model.iter_first() {
        init(model, &iter);
        loop {
            if !f(model, &iter) {
                break;
            }

            if !model.iter_next(&iter) {
                break;
            }
        }
    }
}

pub fn iter_list<F>(model: &ListStore, mut f: F)
where
    F: FnMut(&ListStore, &TreeIter),
{
    if let Some(iter) = model.iter_first() {
        loop {
            f(model, &iter);

            if !model.iter_next(&iter) {
                break;
            }
        }
    }
}
