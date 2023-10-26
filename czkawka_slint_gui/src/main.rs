mod connect_delete;
mod connect_open;
mod connect_progress_receiver;
mod connect_scan;
mod connect_stop;

use crossbeam_channel::{unbounded, Receiver, Sender};
use std::path::Path;
use std::rc::Rc;

use crate::connect_delete::connect_delete_button;
use crate::connect_open::connect_open_items;
use crate::connect_scan::connect_scan_button;

use crate::connect_progress_receiver::connect_progress_gathering;
use crate::connect_stop::connect_stop_button;
use czkawka_core::common_dir_traversal::ProgressData;
use slint::{ModelRc, SharedString, VecModel};

slint::include_modules!();
fn main() {
    let app = MainWindow::new().unwrap(); //.run().unwrap();

    let (progress_sender, progress_receiver): (Sender<ProgressData>, Receiver<ProgressData>) = unbounded();
    let (stop_sender, stop_receiver): (Sender<()>, Receiver<()>) = unbounded();
    // Fills model at start, don't really needed too much after testing
    to_remove_debug(&app);

    connect_delete_button(&app);
    connect_scan_button(&app, progress_sender, stop_receiver);
    connect_stop_button(&app, stop_sender);
    connect_open_items(&app);
    connect_progress_gathering(&app, progress_receiver);

    app.run().unwrap();
}

type ModelType = VecModel<(bool, bool, bool, ModelRc<SharedString>)>;
// TODO remove this after trying
pub fn to_remove_debug(app: &MainWindow) {
    let row_data: Rc<ModelType> = Rc::new(VecModel::default());

    for r in 0..20_000_000 {
        let items = VecModel::default();

        for c in 0..3 {
            items.push(slint::format!("Item {r}.{c}"));
        }

        row_data.push((r % 2 == 0, r % 3 == 0, false, ModelRc::new(items)));
    }
    app.set_empty_folder_model(row_data.into());
}

pub fn split_path(path: &Path) -> (String, String) {
    match (path.parent(), path.file_name()) {
        (Some(dir), Some(file)) => (dir.display().to_string(), file.to_string_lossy().into_owned()),
        (Some(dir), None) => (dir.display().to_string(), String::new()),
        (None, _) => (String::new(), String::new()),
    }
}
