mod connect_delete;
mod connect_scan;

use std::borrow::BorrowMut;
use std::path::Path;
use std::rc::Rc;

use crate::connect_delete::connect_delete_button;
use crate::connect_scan::connect_scan_button;

use czkawka_core::common_tool::CommonData;
use czkawka_core::empty_folder::EmptyFolder;
use slint::{Model, ModelRc, SharedString, VecModel};

slint::include_modules!();
fn main() {
    let app = MainWindow::new().unwrap(); //.run().unwrap();
    let row_data: Rc<VecModel<(bool, bool, bool, ModelRc<SharedString>)>> = Rc::new(VecModel::default());

    for r in 0..1000 {
        let items = VecModel::default();

        for c in 0..3 {
            items.push(slint::format!("Item {r}.{c}").into());
        }

        row_data.push((r % 2 == 0, false, true, ModelRc::new(items)));
    }
    app.set_empty_folder_model(row_data.into());

    connect_delete_button(&app);
    connect_scan_button(&app);

    app.run().unwrap();
}

pub fn split_path(path: &Path) -> (String, String) {
    match (path.parent(), path.file_name()) {
        (Some(dir), Some(file)) => (dir.display().to_string(), file.to_string_lossy().into_owned()),
        (Some(dir), None) => (dir.display().to_string(), String::new()),
        (None, _) => (String::new(), String::new()),
    }
}
