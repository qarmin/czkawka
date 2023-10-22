use crate::MainWindow;
use slint::{ComponentHandle, Model, ModelRc, VecModel};
use std::borrow::BorrowMut;

pub fn connect_delete_button(app: &MainWindow) {
    let a = app.as_weak();
    app.on_deleted(move || {
        let app = a.upgrade().unwrap();

        let mut r = app.get_empty_folder_model();
        let m = r.borrow_mut();
        let (entries_to_delete, mut entries_left): (Vec<_>, Vec<_>) = m.iter().partition(|(checked, _header_row, _selected_row, _data)| *checked);

        if !entries_to_delete.is_empty() {
            dbg!(format!("Items to remove {}", entries_to_delete.len()));
            entries_to_delete.into_iter().for_each(|(_checked, _header_row, _selected_row, _data)| {
                // TODO delete in parallel items, consider to add progress bar
            });
            entries_left.iter_mut().for_each(|(_checked, _header_row, selected_row, _data)| {
                *selected_row = false;
            });
            let r = ModelRc::new(VecModel::from(entries_left));
            app.set_empty_folder_model(r);
        }
    });
}
