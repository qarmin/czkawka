use slint::{Model, ModelRc, SharedString, VecModel};

use crate::FileEntry;
use crate::scan_runner::FileItem;

pub fn make_file_model(items: Vec<FileItem>) -> ModelRc<FileEntry> {
    let entries: Vec<FileEntry> = items
        .into_iter()
        .map(|item| FileEntry {
            checked: false,
            is_header: item.is_header,
            name: SharedString::from(item.name),
            path: SharedString::from(item.path),
            size: SharedString::from(item.size),
            modified: SharedString::default(),
            extra: SharedString::from(item.extra),
        })
        .collect();

    ModelRc::new(VecModel::from(entries))
}

pub fn toggle_row(model: &ModelRc<FileEntry>, index: usize) {
    if let Some(mut entry) = model.row_data(index) {
        if !entry.is_header {
            entry.checked = !entry.checked;
            model.set_row_data(index, entry);
        }
    }
}

pub fn count_checked(model: &ModelRc<FileEntry>) -> i32 {
    (0..model.row_count()).filter(|&i| model.row_data(i).map(|e| e.checked).unwrap_or(false)).count() as i32
}
