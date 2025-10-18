use std::path::PathBuf;

use gtk4::prelude::*;

pub fn extract_paths_from_file_chooser(file_chooser: &gtk4::FileChooserNative) -> Vec<PathBuf> {
    let mut folders: Vec<PathBuf> = Vec::new();
    let g_files = file_chooser.files();
    for index in 0..g_files.n_items() {
        if let Some(file) = g_files.item(index) {
            let ss = file.clone().downcast::<gtk4::gio::File>().expect("Failed to downcast to File");
            if let Some(path_buf) = ss.path() {
                folders.push(path_buf);
            }
        }
    }
    folders
}
