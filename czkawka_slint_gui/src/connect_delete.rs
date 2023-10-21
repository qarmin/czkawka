use crate::MainWindow;
use slint::{ComponentHandle, Model, ModelRc, VecModel};
use std::borrow::BorrowMut;

pub fn connect_delete_button(app: &MainWindow) {
    let a = app.as_weak();
    app.on_deleted(move || {
        let app = a.upgrade().unwrap();

        let mut r = app.get_empty_folder_model();
        let m = r.borrow_mut();
        let length_before = m.iter().count();
        let (entries_to_delete, entries_left): (Vec<_>, Vec<_>) = m.iter().partition(|(checked, _selected_row, _header_row, _data)| *checked);
        let mut s: Vec<_> = m.iter().filter(|(checked, _selected_row, _header_row, _data)| !*checked).collect();

        entries_to_delete.into_iter().for_each(|(_checked, _selected_row, _header_row, _data)| {
            // TODO delete in parallel items, consider to add progress bar
        });

        let length_after = s.len();
        if length_before != length_after {
            dbg!(format!("Items to remove {}", length_before - length_after));
            s.iter_mut().for_each(|(_checked, selected_row, _header_row, _data)| {
                *selected_row = false;
            });
            let r = ModelRc::new(VecModel::from(s));
            app.set_empty_folder_model(r.into());
        }
    });
}
