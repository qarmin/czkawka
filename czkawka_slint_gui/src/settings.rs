use crate::MainWindow;
use std::env;

use crate::Settings;
use slint::{ComponentHandle, ModelRc, SharedString, StandardListViewItem, VecModel};
pub fn reset_settings(app: &MainWindow) {
    let settings = app.global::<Settings>();

    // app.width(1000);
    app.invoke_set_console_text(SharedString::from(""));

    // Get current folder where executed binary is
    let current_folder = env::current_dir();
    let mut included_directories = vec![];
    if let Ok(current_dir) = current_folder {
        included_directories.push(current_dir.to_string_lossy().to_string());
    };

    let included_items = VecModel::default();
    for i in included_directories {
        let mut element = StandardListViewItem::default();
        element.text = SharedString::from(i);
        included_items.push(element);
    }
    settings.set_included_directories(ModelRc::new(included_items));
}
