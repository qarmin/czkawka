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
pub fn iter_list_with_break<F>(model: &ListStore, mut f: F)
where
    F: FnMut(&ListStore, &TreeIter) -> bool,
{
    if let Some(iter) = model.iter_first() {
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

#[cfg(test)]
mod tests {
    use glib::Value;
    use glib::types::Type;

    use super::*;

    #[gtk4::test]
    fn test_iter_list_collects_items() {
        let types: &[Type] = &[Type::STRING];
        let list_store = ListStore::new(types);

        list_store.set(&list_store.append(), &[(0u32, &Into::<Value>::into("a"))]);
        list_store.set(&list_store.append(), &[(0u32, &Into::<Value>::into("b"))]);
        list_store.set(&list_store.append(), &[(0u32, &Into::<Value>::into("c"))]);

        let mut collected = Vec::new();
        iter_list(&list_store, |m, i| {
            collected.push(m.get::<String>(i, 0));
        });

        assert_eq!(collected, vec!["a".to_string(), "b".to_string(), "c".to_string()]);
    }

    #[gtk4::test]
    fn test_iter_list_with_break_stops() {
        let types: &[Type] = &[Type::STRING];
        let list_store = ListStore::new(types);
        list_store.set(&list_store.append(), &[(0u32, &Into::<Value>::into("a"))]);
        list_store.set(&list_store.append(), &[(0u32, &Into::<Value>::into("b"))]);
        list_store.set(&list_store.append(), &[(0u32, &Into::<Value>::into("c"))]);

        let mut collected = Vec::new();
        iter_list_with_break(&list_store, |m, i| {
            collected.push(m.get::<String>(i, 0));
            false
        });

        assert_eq!(collected, vec!["a".to_string()]);
    }

    #[gtk4::test]
    fn test_iter_list_with_break_init_runs_init_and_iterates() {
        let types: &[Type] = &[Type::STRING];
        let list_store = ListStore::new(types);
        list_store.set(&list_store.append(), &[(0u32, &Into::<Value>::into("a"))]);
        list_store.set(&list_store.append(), &[(0u32, &Into::<Value>::into("b"))]);

        let mut collected = Vec::new();
        iter_list_with_break_init(
            &list_store,
            |m, i| {
                m.iter_next(i)
            },
            |m, i| {
                collected.push(m.get::<String>(i, 0));
            },
        );

        assert_eq!(collected, vec!["b".to_string()]);
    }

    #[gtk4::test]
    fn test_iter_list_break_with_init_runs_init_and_can_break() {
        let types: &[Type] = &[Type::STRING];
        let list_store = ListStore::new(types);
        list_store.set(&list_store.append(), &[(0u32, &Into::<Value>::into("a"))]);
        list_store.set(&list_store.append(), &[(0u32, &Into::<Value>::into("b"))]);

        let mut collected = Vec::new();
        iter_list_break_with_init(
            &list_store,
            |_m, _i| {
            },
            |m, i| {
                collected.push(m.get::<String>(i, 0));
                false
            },
        );

        assert_eq!(collected, vec!["a".to_string()]);
    }
}
